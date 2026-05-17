# 第三章：JNI 桥接层

---

## 3.1 JNI 概述

JNI (Java Native Interface) 是 Java 与本地代码（C/C++/Rust）之间的标准互操作接口。在本项目中，JNI 连接 Java 层的 `TunnelCore` 类和 Rust 层的 `jni_ffi.rs`。

```
┌─────────────────────────────────────────────────────────────┐
│                      调用链路                                 │
└─────────────────────────────────────────────────────────────┘

Java 层                    JNI 层                     Rust 层
─────────                  ──────                     ──────
TunnelCore.java            jni_ffi.rs                 ffi.rs
                                                      
tunnelStart(addr)  ──→  Java_..._tunnelStart()  ──→  tunnel_start()
  │                        │                           │
  │ String                 │ JString → CString         │ *const c_char
  │                        │                           │
  │ boolean               ←│ jboolean                 ←│ bool
  └────────────────────────┴───────────────────────────┘
```

---

## 3.2 System.loadLibrary 流程

```java
public class TunnelCore {
    static {
        System.loadLibrary("tunnel_core");
    }
    // ...
}
```

### 加载时序

```
类首次被引用（如 TunnelCore.tunnelStart()）
        │
        ▼
JVM 执行 static {} 初始化块
        │
        ▼
System.loadLibrary("tunnel_core")
        │
        ├── 搜索路径：APK/lib/arm64-v8a/libtunnel_core.so
        │
        ▼
dlopen("libtunnel_core.so")
        │
        ├── 加载 .so 到进程地址空间
        ├── 解析符号表
        │
        ▼
查找并调用 JNI_OnLoad 函数（如果存在）
        │
        ├── 保存 JavaVM 引用到全局静态变量
        ├── 返回 JNI_VERSION_1_6
        │
        ▼
加载完成，native 方法可用
```

### 常见错误

| 错误 | 原因 | 解决方案 |
|------|------|----------|
| `UnsatisfiedLinkError: dlopen failed` | .so 文件不存在 | 检查 jniLibs 路径和 ABI |
| `UnsatisfiedLinkError: No implementation found` | 函数名不匹配 | 检查 JNI 命名规则 |
| `SIGABRT` 在加载时 | JNI_OnLoad panic | 检查 Rust 代码初始化逻辑 |

---

## 3.3 JNI_OnLoad 缓存 JVM

```rust
/// JNI_OnLoad - 动态库加载时由 JVM 自动调用
#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    vm: jni::JavaVM,       // JVM 实例（整个进程唯一）
    _: *mut std::ffi::c_void,
) -> jni::sys::jint {
    set_jvm(vm);           // 保存到 RwLock<Option<JavaVM>> 静态变量
    jni::sys::JNI_VERSION_1_6  // 告诉 JVM 需要的 JNI 版本
}
```

### 为什么要缓存 JVM

```
问题场景：
  Rust 的 tokio 线程需要回调 Java 的 protect 方法
  但 tokio 线程不是 Java 线程，没有 JNIEnv

解决方案：
  1. JNI_OnLoad 时保存 JavaVM（全局唯一）
  2. 任意 Rust 线程通过 JavaVM.attach_current_thread() 获取 JNIEnv
  3. 有了 JNIEnv 就能调用 Java 方法

时序：
  System.loadLibrary()
       │
       ▼
  JNI_OnLoad(vm) → 保存 vm 到 static JVM
       │
       ...（之后的某个时刻）...
       │
  tokio 线程需要 protect
       │
       ▼
  JVM.read() → vm.attach_current_thread_permanently() → JNIEnv
       │
       ▼
  env.call_method(vpnService, "protectSocket", fd)
```

---

## 3.4 Native Method 声明到 Rust 函数的映射规则

### 命名规则

```
Java:   package com.vpndemo.app; class TunnelCore { native boolean tunnelStart(String addr); }
                │          │          │                     │              │
                ▼          ▼          ▼                     ▼              ▼
Rust:   Java_com_vpndemo_app_TunnelCore_tunnelStart(env, class, addr) -> jboolean
        ^^^^                                        ^^^^^^^^^^^^^^^^^^^
        前缀    包名(dot→underscore)  类名   方法名   JNI 标准参数 + Java 参数
```

### 完整映射表

| Java 声明 | Rust 函数名 |
|-----------|------------|
| `boolean tunnelStart(String)` | `Java_com_vpndemo_app_TunnelCore_tunnelStart` |
| `void tunnelStop()` | `Java_com_vpndemo_app_TunnelCore_tunnelStop` |
| `boolean tunnelFeedPacket(byte[], int)` | `Java_com_vpndemo_app_TunnelCore_tunnelFeedPacket` |
| `int tunnelReadPacket(byte[], int)` | `Java_com_vpndemo_app_TunnelCore_tunnelReadPacket` |
| `boolean tunnelIsRunning()` | `Java_com_vpndemo_app_TunnelCore_tunnelIsRunning` |
| `void setProtectSocketCallback(Object, String)` | `Java_com_vpndemo_app_TunnelCore_setProtectSocketCallback` |

### Rust 函数签名要求

```rust
#[no_mangle]                    // 禁止 Rust 名称修饰（否则链接器找不到）
pub extern "system" fn          // 使用系统调用约定
Java_com_vpndemo_app_TunnelCore_tunnelStart(
    mut env: JNIEnv,            // 第 1 个参数：JNI 环境（类型转换用）
    _class: JClass,             // 第 2 个参数：调用者 Class（静态方法时）
    socks5_addr: JString,       // 第 3+ 个参数：Java 方法的参数
) -> jboolean {                 // 返回值：对应 Java 的 boolean
    // ...
}
```

### `#[no_mangle]` 的必要性

```
没有 #[no_mangle]:
  Rust 编译后符号名: _ZN11tunnel_core7jni_ffi7android...（Rust name mangling）
  JNI 查找: Java_com_vpndemo_app_TunnelCore_tunnelStart
  结果: 找不到 → UnsatisfiedLinkError

有 #[no_mangle]:
  编译后符号名: Java_com_vpndemo_app_TunnelCore_tunnelStart（原样保留）
  JNI 查找: Java_com_vpndemo_app_TunnelCore_tunnelStart
  结果: 匹配成功 ✓
```

---

## 3.5 JNI 类型转换

### 基本类型映射

| Java 类型 | JNI 类型 | Rust 类型 | 大小 |
|-----------|----------|-----------|------|
| `boolean` | `jboolean` | `u8` (JNI_TRUE=1, JNI_FALSE=0) | 1 byte |
| `int` | `jint` | `i32` | 4 bytes |
| `long` | `jlong` | `i64` | 8 bytes |
| `byte[]` | `JByteArray` | 需要 `convert_byte_array()` | 变长 |
| `String` | `JString` | 需要 `get_string()` | 变长 |
| `Object` | `JObject` | 不透明句柄 | 指针 |

### String 转换

```rust
// Java String → Rust String
// JNI 字符串是 Modified UTF-8 编码，get_string() 处理编码转换
let addr: String = env.get_string(&socks5_addr)?.into();

// Rust String → C 字符串（如果需要传给 C FFI）
let c_addr = CString::new(addr)?;  // 添加 null 结尾
```

### byte[] 转换

```rust
// Java byte[] → Rust Vec<u8>（读取方向）
// 这会从 Java 堆复制数据到 Rust 堆
let bytes: Vec<u8> = env.convert_byte_array(&data)?;

// Rust 数据 → Java byte[]（写入方向）
// set_byte_array_region 直接写入 Java 堆内存，避免额外分配
env.set_byte_array_region(&buf, 0, unsafe {
    std::slice::from_raw_parts(native_buf.as_ptr() as *const i8, n)
})?;
```

### 为什么用 `as *const i8`

```
Java byte 是 signed（-128 到 127）
Rust u8 是 unsigned（0 到 255）
底层内存表示完全相同（都是 8 位），只是解释方式不同
所以直接 cast 指针类型是安全的
```

---

## 3.6 GlobalRef 防止 GC 回收

### 问题场景

```
时刻 T1: setProtectSocketCallback(vpnService, "protectSocket")
         Rust 保存了 vpnService 的引用
         
时刻 T2: Java GC 运行
         vpnService 没有 Java 层的强引用了（只有 Rust 持有）
         GC 回收了 vpnService 对象！

时刻 T3: Rust 调用 protect_socket(fd)
         尝试通过已回收的引用调用方法
         → SIGSEGV 崩溃！
```

### 解决方案：GlobalRef

```rust
// 创建 GlobalRef：告诉 JVM "我还在用这个对象，别回收"
let global_ref = env.new_global_ref(vpn_service)?;

// GlobalRef 的生命周期：
// - 创建后，GC 不会回收该对象
// - 直到 GlobalRef 被 drop（Rust 析构）
// - 或者手动调用 env.delete_global_ref()

// 在本项目中：GlobalRef 存在 static RwLock 中，永远不会被 drop
// → VpnService 实例在整个进程生命周期内不会被 GC
```

### 引用类型对比

| 类型 | 生命周期 | 用途 |
|------|---------|------|
| Local Reference | 当前 JNI 函数调用期间 | 临时使用 |
| Global Reference | 手动管理（直到 delete） | 跨函数/跨线程保持对象 |
| Weak Global Reference | 可能被 GC 回收 | 缓存（需检查有效性） |

```
JNI 函数调用:
  ┌─────────────────────────────────────┐
  │  env, class, vpn_service            │  ← 这些都是 Local Reference
  │  函数返回后自动释放                  │
  └─────────────────────────────────────┘
  
  如果需要在函数返回后继续使用 vpn_service：
  → env.new_global_ref(vpn_service) → GlobalRef（不会被自动释放）
```

---

## 3.7 完整 JNI 调用示例

以 `tunnelFeedPacket` 为例，展示一个完整的 JNI 调用过程：

```
Java 层:
  byte[] packet = new byte[n];
  System.arraycopy(buf, 0, packet, 0, n);
  TunnelCore.tunnelFeedPacket(packet, n);
       │
       │  JNI 框架自动处理：
       │  1. 查找 native 方法对应的 C 符号
       │  2. 准备 JNIEnv
       │  3. 将 Java 参数转为 JNI 类型
       │
       ▼
Rust JNI 层 (jni_ffi.rs):
  fn Java_..._tunnelFeedPacket(env, _class, data: JByteArray, len: jint) {
      // data 是 Java byte[] 的 JNI 句柄（不是实际数据！）
      let bytes = env.convert_byte_array(&data)?;
      //          └── 从 Java 堆复制字节到 Rust Vec<u8>
      
      unsafe { ffi::tunnel_feed_packet(bytes.as_ptr(), len as usize) }
      //       └── 调用 C FFI 函数
  }
       │
       ▼
Rust FFI 层 (ffi.rs):
  pub unsafe extern "C" fn tunnel_feed_packet(data: *const u8, len: usize) -> bool {
      let packet = std::slice::from_raw_parts(data, len).to_vec();
      //          └── 创建 Vec<u8>（又一次拷贝，但这次是 Rust 内部的）
      
      TUNNEL.get()?.inbound_tx.try_send(packet).is_ok()
      //                       └── 送入 mpsc channel
  }
       │
       ▼
Rust netstack 消费 channel 中的包，进行 TCP/IP 解析...
```

### 性能考虑

每次 `tunnelFeedPacket` 调用涉及：
1. JNI 调用开销（约 100ns）
2. Java → Rust 内存拷贝（`convert_byte_array`）
3. 指针到 Vec 的拷贝（`to_vec()`）
4. Channel 发送

对于 VPN 流量（通常 <10000 包/秒），这个开销完全可接受。
