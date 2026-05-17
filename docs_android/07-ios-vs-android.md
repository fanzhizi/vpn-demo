# 第七章：iOS 与 Android 实现对比

---

## 7.1 架构差异总结

```
┌─────────────────────────────────┐   ┌─────────────────────────────────┐
│          iOS 架构                │   │         Android 架构             │
│                                 │   │                                 │
│  ┌───────────┐  ┌───────────┐  │   │  ┌───────────────────────────┐  │
│  │  主 App    │  │ Extension │  │   │  │        同一进程            │  │
│  │  (进程 A)  │  │ (进程 B)  │  │   │  │                           │  │
│  │           │  │           │  │   │  │  MainActivity             │  │
│  │  UI 控制   │  │ Tunnel    │  │   │  │  TunnelVpnService        │  │
│  │  开关 VPN  │  │ Provider  │  │   │  │  Rust tunnel-core        │  │
│  │           │  │ Rust core │  │   │  │                           │  │
│  └───────────┘  └───────────┘  │   │  └───────────────────────────┘  │
│        │              │         │   │                                 │
│        │  IPC (XPC)   │         │   │  （无 IPC，直接函数调用）        │
│        └──────────────┘         │   │                                 │
└─────────────────────────────────┘   └─────────────────────────────────┘
```

---

## 7.2 VPN API 对比

| 功能 | iOS (NEPacketTunnelProvider) | Android (VpnService) |
|------|-----|---------|
| **启动** | `startTunnel(options:completionHandler:)` | `onStartCommand(intent, flags, startId)` |
| **停止** | `stopTunnel(with:completionHandler:)` | 手动 `stopSelf()` |
| **TUN 配置** | `setTunnelNetworkSettings()` | `Builder.establish()` |
| **IP 地址** | `NEIPv4Settings(addresses:subnetMasks:)` | `Builder.addAddress(addr, prefix)` |
| **路由** | `NEIPv4Route.default()` | `Builder.addRoute("0.0.0.0", 0)` |
| **DNS** | `NEDNSSettings(servers:)` | `Builder.addDnsServer(addr)` |
| **MTU** | `tunnelNetworkSettings.mtu` | `Builder.setMtu(mtu)` |
| **读包** | `packetFlow.readPackets(completionHandler:)` | `FileInputStream.read(buf)` |
| **写包** | `packetFlow.writePackets(packets, protocols:)` | `FileOutputStream.write(buf)` |
| **完成通知** | `completionHandler(nil)` | 无（startCommand 返回即可） |

### 配置代码对比

```swift
// iOS - 异步回调风格
let settings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "10.0.0.1")
settings.ipv4Settings = NEIPv4Settings(addresses: ["10.0.0.2"], subnetMasks: ["255.255.255.0"])
settings.ipv4Settings?.includedRoutes = [NEIPv4Route.default()]
settings.dnsSettings = NEDNSSettings(servers: ["8.8.8.8"])
settings.mtu = 1500

setTunnelNetworkSettings(settings) { error in
    completionHandler(error)
}
```

```java
// Android - Builder 模式
Builder builder = new Builder();
builder.addAddress("10.0.0.2", 24)
       .addRoute("0.0.0.0", 0)
       .addDnsServer("8.8.8.8")
       .setMtu(1500);

ParcelFileDescriptor tunFd = builder.establish();
// 返回 fd 表示成功，null 表示失败
```

---

## 7.3 TUN 读写方式对比

### iOS: 回调 + 批量处理

```swift
// iOS 读取：回调式，一次可能返回多个包
func readLoop() {
    packetFlow.readPackets { [weak self] packets, protocols in
        for (i, packet) in packets.enumerated() {
            // packet 是 Data 类型（IP 数据包）
            tunnel_feed_packet(packet.bytes, packet.count)
        }
        self?.readLoop()  // 递归继续读取
    }
}

// iOS 写入：批量写入
let data = Data(bytes: buf, count: Int(n))
packetFlow.writePackets([data], withProtocols: [AF_INET as NSNumber])
```

### Android: 阻塞 I/O + 独立线程

```java
// Android 读取：阻塞式，一次一个包
readThread = new Thread(() -> {
    FileInputStream in = new FileInputStream(tunFd.getFileDescriptor());
    byte[] buf = new byte[65535];
    while (running) {
        int n = in.read(buf);  // 阻塞！直到有包可读
        if (n > 0) {
            byte[] packet = new byte[n];
            System.arraycopy(buf, 0, packet, 0, n);
            TunnelCore.tunnelFeedPacket(packet, n);
        }
    }
});

// Android 写入：轮询式
writeThread = new Thread(() -> {
    FileOutputStream out = new FileOutputStream(tunFd.getFileDescriptor());
    byte[] buf = new byte[65535];
    while (running) {
        int n = TunnelCore.tunnelReadPacket(buf, buf.length);
        if (n > 0) {
            out.write(buf, 0, n);
        } else {
            Thread.sleep(1);  // 无数据时短暂等待
        }
    }
});
```

### 对比

| 维度 | iOS | Android |
|------|-----|---------|
| 读取方式 | 回调 (readPackets) | 阻塞 read() |
| 批量处理 | 支持（一次多个包） | 不支持（一次一个） |
| 线程模型 | GCD 自动管理 | 手动创建线程 |
| 写入方式 | 批量 writePackets | 逐个 write() |
| 协议标识 | 需要传 AF_INET/AF_INET6 | 不需要 |

---

## 7.4 Socket 保护机制对比

### iOS: 无需处理

```
iOS Extension 进程的网络流量天然不经过 TUN。
这是 iOS 内核的设计保证，开发者无需任何额外代码。

Rust 代码中的体现：
  #[cfg(not(target_os = "android"))]
  pub fn protect_socket(_fd: i32) -> bool {
      true  // 直接返回 true，什么都不做
  }
```

### Android: 必须手动处理

```
Android VpnService 与 App 同进程，所有 socket 默认走 TUN。
必须显式调用 protect(fd) 来豁免代理连接。

涉及的代码量：
  - android_protect 模块：~80 行 Rust
  - JNI_OnLoad：~5 行
  - setProtectSocketCallback JNI 函数：~15 行
  - 每个 TCP/UDP 连接的 protect 调用：~10 行
  - Java 层 protectSocket 方法：~3 行
  总计：~110 行额外代码
```

### 代码量对比

| 文件 | iOS 版本 | Android 额外代码 |
|------|---------|-----------------|
| ffi.rs | 基础逻辑 | +80 行 (android_protect 模块) |
| ffi.rs | TCP/UDP 处理 | +30 行 (#[cfg] socket2 保护流程) |
| jni_ffi.rs | 不存在 | +130 行 (整个文件) |
| TunnelVpnService.java | 不存在 | +170 行 |
| TunnelCore.java | 不存在 | +22 行 |

---

## 7.5 Rust 集成方式对比

### .a 静态库 (iOS)

```
编译：
  cargo build --target aarch64-apple-ios --release
  → libtunnel_core.a

集成：
  1. 将 .a 文件添加到 Xcode 项目
  2. 创建 Bridging Header 声明 C 函数
  3. Swift 直接调用 C 函数（无运行时开销）

调用方式：
  Swift → C Bridging Header → Rust extern "C" fn
  （编译时链接，零开销）
```

```
┌──────────────────────────────┐
│  iOS Extension 二进制文件     │
│                              │
│  ┌────────────┐  ┌────────┐ │
│  │ Swift 代码  │  │ .a 静态│ │
│  │            │  │ 库代码  │ │
│  │ 直接调用 ──│──│──▶ fn   │ │
│  └────────────┘  └────────┘ │
│                              │
│  （单一可执行文件）           │
└──────────────────────────────┘
```

### .so 动态库 (Android)

```
编译：
  cargo ndk -t arm64-v8a build --release
  → libtunnel_core.so

集成：
  1. 将 .so 放入 jniLibs/arm64-v8a/
  2. Java 声明 native 方法
  3. Rust 实现 JNI 函数（命名规则匹配）

调用方式：
  Java → JNI → dlsym 查找符号 → Rust extern "system" fn
  （运行时链接，有 JNI 开销）
```

```
┌──────────────────────────────────────────┐
│            Android 进程                   │
│                                          │
│  ┌────────────┐       ┌────────────────┐ │
│  │ Java 代码   │       │ .so 动态库     │ │
│  │            │  JNI  │               │ │
│  │ native ────│───────│──▶ fn         │ │
│  │ method     │       │               │ │
│  └────────────┘       └────────────────┘ │
│        │                    │            │
│        │ dalvik/ART         │ mmap       │
│        │ 虚拟机             │ 加载       │
│        ▼                    ▼            │
│  ┌──────────────────────────────────┐    │
│  │          进程地址空间             │    │
│  └──────────────────────────────────┘    │
└──────────────────────────────────────────┘
```

### 对比表

| 维度 | iOS (.a) | Android (.so) |
|------|----------|--------------|
| 链接时机 | 编译时 | 运行时 (dlopen) |
| 调用开销 | 零（直接函数指针） | 小（JNI ~100ns） |
| 类型安全 | Bridging Header 提供 | 手动管理类型转换 |
| 内存管理 | 共享进程堆 | 跨 JVM/Native 边界 |
| 调试 | Xcode 完整支持 | 需要 ndk-gdb/lldb |
| 文件大小 | 包含在二进制中 | 独立 .so 文件 |
| 更新 | 必须重新编译 App | 理论上可热替换 |

---

## 7.6 共享代码 vs 平台特定代码

### 代码共享架构

```
tunnel-core/src/
├── lib.rs          ← 通用：TunnelConfig 定义
├── ffi.rs          ← 90% 通用 + 10% 平台特定 (#[cfg])
└── jni_ffi.rs      ← Android 专用（iOS 不编译）

通用代码（两平台共享）：
  - netstack-smoltcp 初始化和驱动
  - TCP SOCKS5 转发逻辑（除了 socket 创建部分）
  - UDP 转发逻辑（除了 socket 创建部分）
  - channel 管理和数据流转
  - tunnel_start/stop/feed/read_packet C FFI

平台特定代码 (#[cfg] 条件编译)：
  - socket 创建和 protect（Android 需要 socket2 + protect）
  - 日志初始化（oslog vs android_logger）
  - JNI_OnLoad + protect 回调机制（Android only）
  - jni_ffi.rs 整个文件（Android only）
```

### 条件编译使用统计

```rust
// ffi.rs 中的 #[cfg] 使用

#[cfg(target_os = "android")]    // 出现 6 次
  - android_protect 模块定义
  - protect_socket 函数（Android 版本）
  - android_logger 初始化
  - forward_udp 中的 socket2 创建
  - handle_tcp_via_socks5 中的 socket2 创建

#[cfg(not(target_os = "android"))]  // 出现 3 次
  - protect_socket 空实现
  - forward_udp 中的标准 UdpSocket
  - handle_tcp_via_socks5 中的标准 Socks5Stream::connect

#[cfg(target_os = "ios")]        // 出现 1 次
  - oslog 初始化
```

### 共享比例

```
代码行数（估算）：

  ┌────────────────────────────────────────────────────┐
  │                 ffi.rs (~630 行)                    │
  │                                                    │
  │  ┌──────────────────────────────────────────────┐  │
  │  │         通用代码 (~450 行, 71%)               │  │
  │  │                                              │  │
  │  │  run_stack, channel 管理, copy_bidirectional  │  │
  │  │  tunnel_start/stop/feed/read_packet          │  │
  │  │  test_socks5_impl                            │  │
  │  └──────────────────────────────────────────────┘  │
  │                                                    │
  │  ┌──────────────────┐  ┌─────────────────────┐    │
  │  │ Android (100行)  │  │ iOS 特定 (5行)      │    │
  │  │ 16%              │  │ 1%                  │    │
  │  │ protect 模块      │  │ oslog init         │    │
  │  │ socket2 创建      │  │                    │    │
  │  └──────────────────┘  └─────────────────────┘    │
  │                                                    │
  │  ┌──────────────────┐                             │
  │  │ 桌面 (5行) 1%    │                             │
  │  │ env_logger       │                             │
  │  └──────────────────┘                             │
  └────────────────────────────────────────────────────┘

  jni_ffi.rs: 130 行（100% Android 专用）
```

---

## 7.7 开发体验对比

| 维度 | iOS | Android |
|------|-----|---------|
| IDE | Xcode (必须 macOS) | Android Studio (跨平台) |
| 编译速度 | 中等（含 Swift） | 快（Java 增量快，Rust 只需编一次） |
| 调试 TUN 数据 | 较难（Extension 进程限制） | 容易（logcat 实时查看） |
| 模拟器测试 | 不支持（Network Extension） | 有限支持（需要 x86_64 .so） |
| 真机要求 | 必须 Apple Developer 会员 | 开启 USB 调试即可 |
| 热重载 | 不支持 | Java 修改可即时生效（Rust 不行） |
| 证书 | 复杂（Provisioning Profile） | 简单（debug.keystore 自动） |
| 上架 | App Store 审核严格 | Google Play 相对宽松 |

---

## 7.8 总结：选择建议

```
如果你正在做跨平台 VPN 项目，建议架构：

  ┌─────────────────────────────────────────────────────────────┐
  │                  Rust tunnel-core (共享 70%+)                │
  │                                                             │
  │  ┌─────────────┐   ┌──────────────┐   ┌─────────────────┐  │
  │  │ netstack    │   │ SOCKS5 转发   │   │ 协议处理逻辑     │  │
  │  └─────────────┘   └──────────────┘   └─────────────────┘  │
  │                                                             │
  │  ┌─────────────────────┐   ┌───────────────────────────┐   │
  │  │ #[cfg(android)]     │   │ #[cfg(ios)]               │   │
  │  │ JNI + protect       │   │ （几乎为空）               │   │
  │  └─────────────────────┘   └───────────────────────────┘   │
  └─────────────────────────────────────────────────────────────┘
                │                              │
                ▼                              ▼
  ┌──────────────────────┐      ┌──────────────────────────┐
  │ Android 壳            │      │ iOS 壳                    │
  │                      │      │                          │
  │ Java/Kotlin:         │      │ Swift:                   │
  │ - VpnService         │      │ - NEPacketTunnelProvider │
  │ - TUN 读写线程        │      │ - packetFlow 读写        │
  │ - JNI 声明           │      │ - Bridging Header       │
  │ (~300 行)            │      │ (~200 行)               │
  └──────────────────────┘      └──────────────────────────┘
```

关键原则：
1. **核心逻辑全部用 Rust**：协议栈、转发逻辑、SOCKS5 处理
2. **平台层尽可能薄**：只做 TUN 读写和系统 API 桥接
3. **用 `#[cfg]` 处理差异**：socket 保护、日志、库类型
4. **Android 额外工作主要在 protect**：这是两平台最大的差异点
