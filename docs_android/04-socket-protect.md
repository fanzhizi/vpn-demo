# 第四章：Socket 保护机制

---

## 4.1 为什么 Android 需要 protect 而 iOS 不需要

这是 Android VPN 开发中最关键也最容易出错的部分。

### 核心问题：路由死循环

```
没有 protect 时的死循环：

  App 请求 google.com
       │
       ▼
  TUN 捕获 → netstack 解析为 TCP → 需要通过 SOCKS5 转发
       │
       ▼
  创建到 SOCKS5 代理的 TCP socket
  connect(socks5_addr)
       │
       ▼
  这个 connect 的 SYN 包也被路由到 TUN！（因为 0.0.0.0/0 路由）
       │
       ▼
  TUN 捕获 → netstack 解析 → 又要通过 SOCKS5 转发...
       │
       ▼
  无限循环！进程卡死或 OOM
```

### iOS 为什么不需要

```
iOS 架构：

  ┌──────────────────────┐     ┌──────────────────────┐
  │    主 App 进程         │     │   Extension 进程      │
  │                      │     │                      │
  │  UI 代码             │     │  PacketTunnelProvider │
  │  NETunnelProvider-   │     │  tunnel-core          │
  │  Manager (控制)      │     │  SOCKS5 连接          │
  └──────────────────────┘     └──────────────────────┘
                                         │
                                         │ Extension 进程的流量
                                         │ 不经过 TUN！（系统保证）
                                         │
                                         ▼
                                    物理网卡 → SOCKS5 代理
```

```
Android 架构：

  ┌──────────────────────────────────────────┐
  │              同一个进程                    │
  │                                          │
  │  MainActivity                            │
  │  TunnelVpnService (含 TUN)               │
  │  Rust tunnel-core                        │
  │  SOCKS5 连接 ← 也在这个进程里！           │
  │                     │                    │
  │                     │ 默认也走 TUN 路由    │
  │                     ▼                    │
  │              必须 protect(fd) !           │
  └──────────────────────────────────────────┘
```

### 根本原因对比

| | Android | iOS |
|--|---------|-----|
| 隔离方式 | 同进程，路由隔离 | 不同进程，内核隔离 |
| 代理流量 | 默认走 TUN | 默认走物理网卡 |
| 保护方式 | 手动 protect(fd) | 不需要 |
| 忘记保护的后果 | 死循环/崩溃 | 不存在此问题 |

---

## 4.2 VpnService.protect(fd) 原理

### Linux 内核层面

```java
// Java 层
public boolean protectSocket(int fd) {
    return protect(fd);  // VpnService 的 protected 方法
}
```

`VpnService.protect(int fd)` 在内核层面做了什么：

```
protect(fd)
    │
    ▼
setsockopt(fd, SOL_SOCKET, SO_MARK, vpn_mark)
    │
    ▼
内核路由决策时检查 SO_MARK：
    if (socket 有 VPN mark) {
        使用原始路由表（物理网卡）
    } else {
        使用 VPN 路由表（TUN 设备）
    }
```

### 时机要求

```
正确顺序：
  socket() → protect(fd) → connect()
  创建       标记绕过VPN    连接代理
     ✓          ✓            ✓

错误顺序 1：
  socket() → connect() → protect(fd)
                 ↑
          SYN 已经走了 TUN！protect 太晚了！
          
错误顺序 2：
  connect(socks5)    ← 内部隐式创建 socket + connect
      ↑
  拿不到 fd，无法 protect！
  所以不能用高级 API 直接 connect
```

---

## 4.3 完整实现流程

### 全链路时序图

```
┌────────────┐  ┌────────────┐  ┌─────────────┐  ┌──────────────┐
│ Java 层    │  │ JNI 层     │  │ Rust ffi.rs │  │ Linux Kernel │
└─────┬──────┘  └─────┬──────┘  └──────┬──────┘  └──────┬───────┘
      │               │                │                 │
      │ System.load   │                │                 │
      │ Library()     │                │                 │
      │──────────────▶│                │                 │
      │               │  JNI_OnLoad    │                 │
      │               │───────────────▶│                 │
      │               │  set_jvm(vm)   │                 │
      │               │                │                 │
      │ setProtect-   │                │                 │
      │ SocketCallback│                │                 │
      │──────────────▶│                │                 │
      │               │ new_global_ref │                 │
      │               │ set_protect_cb │                 │
      │               │───────────────▶│                 │
      │               │                │ 保存到 RwLock   │
      │               │                │                 │
      │               │                │                 │
      │  ... 后来，Rust 需要连接 SOCKS5 代理 ...        │
      │               │                │                 │
      │               │                │ socket2::new()  │
      │               │                │────────────────▶│
      │               │                │ 返回 fd         │
      │               │                │◀────────────────│
      │               │                │                 │
      │               │                │ protect_socket  │
      │               │  attach_thread │ (fd)            │
      │               │◀───────────────│                 │
      │ protectSocket │                │                 │
      │ (fd)          │                │                 │
      │◀──────────────│                │                 │
      │               │                │                 │
      │ VpnService    │                │                 │
      │ .protect(fd)  │                │                 │
      │───────────────┼────────────────┼────────────────▶│
      │               │                │                 │ SO_MARK
      │  true         │                │                 │
      │──────────────▶│                │                 │
      │               │  true          │                 │
      │               │───────────────▶│                 │
      │               │                │                 │
      │               │                │ sock.connect()  │
      │               │                │────────────────▶│
      │               │                │                 │ 走物理网卡
      │               │                │  connected      │ (绕过 TUN)
      │               │                │◀────────────────│
```

---

## 4.4 Rust 侧实现详解

### 第一步：JNI_OnLoad 缓存 JVM

```rust
// 静态变量：存储 JVM 引用（进程全局唯一）
static JVM: RwLock<Option<JavaVM>> = RwLock::new(None);

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: jni::JavaVM, _: *mut c_void) -> jint {
    // 保存 JVM 引用，后续任意线程都能通过它获取 JNIEnv
    *JVM.write().unwrap() = Some(vm);
    JNI_VERSION_1_6
}
```

### 第二步：setProtectSocketCallback 保存回调信息

```rust
static PROTECT_CALLBACK: RwLock<Option<ProtectCallback>> = RwLock::new(None);

struct ProtectCallback {
    class: GlobalRef,   // VpnService 实例（GlobalRef 防止 GC）
    method: String,     // "protectSocket"
}

pub fn set_protect_callback(class: GlobalRef, method: String) {
    *PROTECT_CALLBACK.write().unwrap() = Some(ProtectCallback { class, method });
}
```

### 第三步：protect_socket 通过 JNI 回调

```rust
pub fn protect_socket(fd: RawFd) -> bool {
    // 1. 获取 JVM 引用
    let jvm_guard = JVM.read().unwrap();
    let vm = jvm_guard.as_ref()?;
    
    // 2. 获取回调信息
    let cb_guard = PROTECT_CALLBACK.read().unwrap();
    let cb = cb_guard.as_ref()?;
    
    // 3. 将当前线程附加到 JVM（获取 JNIEnv）
    let mut env = vm.attach_current_thread_permanently()?;
    
    // 4. 通过 JNI 调用 Java 方法
    //    签名 "(I)Z"：参数 int, 返回 boolean
    env.call_method(&cb.class, &cb.method, "(I)Z", &[JValue::Int(fd as i32)])
       .ok()?.z().ok()
}
```

### 第四步：socket2 分步创建连接

```rust
// TCP 连接 SOCKS5 代理的完整流程（Android 专用）

// 步骤 1: 创建原始 socket（还未连接）
let sock = socket2::Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

// 步骤 2: protect！必须在 connect 之前！
let fd = sock.as_raw_fd();
protect_socket(fd);  // → JNI → VpnService.protect(fd) → SO_MARK

// 步骤 3: 连接到 SOCKS5 代理（此时 SYN 走物理网卡）
sock.connect(&SockAddr::from(socks5_addr))?;

// 步骤 4: 设为非阻塞（tokio 要求）
sock.set_nonblocking(true)?;

// 步骤 5: 转换为 tokio TcpStream
let std_stream: std::net::TcpStream = sock.into();
let tokio_stream = tokio::net::TcpStream::from_std(std_stream)?;

// 步骤 6: 在已有 TCP 连接上执行 SOCKS5 握手
let socks5_stream = Socks5Stream::use_stream(tokio_stream, None, config).await?;
```

---

## 4.5 attach_current_thread_permanently 原理

### 为什么需要 attach

```
JNI 规则：只有附加到 JVM 的线程才能调用 Java 方法。

Java 线程（如 main、tun-read）→ 自动附加
Rust/tokio 线程 → 未附加，必须手动 attach
```

### permanently vs 普通 attach

| | `attach_current_thread()` | `attach_current_thread_permanently()` |
|--|--------------------------|--------------------------------------|
| 行为 | 返回的 guard drop 时自动 detach | 永不 detach |
| 性能 | 每次 attach/detach 有开销 | 只有首次 attach 有开销 |
| 适用场景 | 偶尔调用一次 | 频繁调用（如每个 TCP 连接都要 protect） |
| 在 tokio 中 | 不可用（guard 可能跨 .await 移动） | 安全（无 guard 需要管理） |

### 为什么选择 permanently

```
场景：每个 TCP 连接都需要 protect

使用普通 attach：
  连接 1: attach → protect → detach → attach → protect → detach ...
  开销：N 个连接 × 2 次 attach/detach

使用 permanently：
  线程首次: attach（一次性）
  连接 1: protect（直接调用，无 attach 开销）
  连接 2: protect（直接调用）
  ...
  开销：1 次 attach + N 次直接调用

而且 tokio 的 worker 线程会处理多个连接，permanently 更合适。
已经 attach 的线程再次调用 attach_permanently 是空操作（安全）。
```

---

## 4.6 对比 leaf 的实现

本项目的 protect 机制参考了 [leaf](https://github.com/eycorsican/leaf) (一个 Rust 编写的代理工具) 的设计。

### 相同点

| 方面 | 本项目 | leaf |
|------|--------|------|
| JVM 存储 | `RwLock<Option<JavaVM>>` | `RwLock<Option<JavaVM>>` |
| 回调存储 | `RwLock<Option<ProtectCallback>>` | 类似的 RwLock 封装 |
| 线程附加 | `attach_current_thread_permanently` | `attach_current_thread_permanently` |
| protect 时机 | socket 创建后、connect 前 | socket 创建后、connect 前 |

### 不同点

| 方面 | 本项目 | leaf |
|------|--------|------|
| 回调方式 | GlobalRef + 方法名反射调用 | 类似但封装更复杂 |
| 错误处理 | 简单 warn 日志 | 更完善的错误传播 |
| 架构复杂度 | 单一 SOCKS5 代理 | 支持多种协议和路由规则 |
| 代码量 | ~80 行 | ~200 行（含更多边界处理） |

### leaf 中值得学习的点

1. **protect 所有出站 socket**：不仅 TCP，UDP 也需要 protect
2. **错误恢复**：protect 失败时重试或降级
3. **日志完善**：记录 protect 的 fd 和结果，方便调试

---

## 4.7 常见错误与调试

### 错误 1：忘记 protect UDP socket

```
症状：DNS 查询超时，网页打不开
原因：UDP socket（用于 DNS）没有被 protect，DNS 查询走了 TUN 形成循环
解决：forward_udp() 中也需要 protect

// 本项目中已正确处理：
let sock = socket2::Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
let fd = sock.as_raw_fd();
protect_socket(fd);  // UDP 也要 protect！
```

### 错误 2：protect 在 connect 之后

```
症状：首次连接卡死，logcat 无明显错误
原因：SYN 包已经走了 TUN
解决：严格保证 new → protect → connect 顺序
```

### 错误 3：JVM/Callback 未初始化

```
症状：logcat 显示 "protect_socket: JVM not set" 或 "callback not set"
原因：tunnelStart 在 setProtectSocketCallback 之前调用
解决：Java 层确保调用顺序

// TunnelVpnService.startVpn() 中：
TunnelCore.setProtectSocketCallback(this, "protectSocket");  // 先注册
TunnelCore.tunnelStart(socks5Address);                       // 后启动
```

### 错误 4：ProGuard 混淆方法名

```
症状：protect 回调失败，NoSuchMethodError
原因：ProGuard 把 "protectSocket" 重命名了
解决：添加 keep 规则或关闭混淆
```

### 调试技巧

```bash
# 查看 protect 相关日志
adb logcat -s tunnel-core | grep protect

# 查看路由表（需要 root）
adb shell ip rule list
adb shell ip route show table all

# 检查 socket mark
adb shell cat /proc/net/tcp  # 查看 TCP socket 状态
```
