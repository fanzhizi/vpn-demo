# 第七章：三平台对比总结

---

## 7.1 整体架构对比

### iOS

```
┌──────────────────┐     ┌──────────────────────────┐
│  主 App (Swift)   │     │  Network Extension       │
│  UI + 控制        │     │  PacketTunnelProvider    │
│                  │     │  ├── tunnel-core (.a)     │
│  NEVPNManager    │────▶│  ├── netstack-smoltcp     │
│  .startVPNTunnel │     │  └── fast-socks5          │
└──────────────────┘     └──────────────────────────┘
  独立进程                   独立进程（流量不走 TUN）
```

### Android

```
┌──────────────────────────────────────────┐
│              单一 App 进程                 │
│  ┌─────────────┐   ┌──────────────────┐  │
│  │ MainActivity│   │ TunnelVpnService  │  │
│  │ (Kotlin UI) │──▶│ (VpnService)     │  │
│  └─────────────┘   │ ├── JNI 桥接      │  │
│                    │ ├── tunnel-core.so│  │
│                    │ └── protect(fd)   │  │
│                    └──────────────────┘  │
└──────────────────────────────────────────┘
```

### Windows

```
┌──────────────────────────────────────────┐
│              单一 exe 进程                 │
│  ┌─────────────┐   ┌──────────────────┐  │
│  │ Slint UI    │   │ tun.rs           │  │
│  │ (主线程)    │──▶│ (wintun + Win32) │  │
│  └─────────────┘   │ socket bind      │  │
│                    │ netstack-smoltcp  │  │
│                    └──────────────────┘  │
└──────────────────────────────────────────┘
```

---

## 7.2 VPN API 对比

| 方面 | iOS NEPacketTunnel | Android VpnService | Windows wintun |
|------|-------------------|-------------------|----------------|
| **提供者** | Apple 系统框架 | Android 系统框架 | WireGuard 第三方 |
| **API 级别** | 高层抽象 | 高层抽象 | 底层 C API |
| **TUN 创建** | 系统自动创建 | Builder.establish() | Adapter::create() |
| **IP 配置** | NEPacketTunnelNetwork... | Builder.addAddress() | CreateUnicastIpAddress... |
| **路由配置** | NEIPv4Settings.includedRoutes | Builder.addRoute() | CreateIpForwardEntry2 |
| **DNS 配置** | NEDNSSettings | Builder.addDnsServer() | 注册表 (无 API) |
| **数据格式** | NSData (IP 包) | FileDescriptor | 环形缓冲区 |
| **权限** | entitlement + 用户确认 | BIND_VPN_SERVICE | UAC 管理员 |
| **后台运行** | Extension 进程独立 | 前台 Service | 无保活机制 |
| **系统集成** | 状态栏 VPN 图标 | 通知栏 VPN 图标 | 无系统集成 |

---

## 7.3 路由绕过对比

这是三个平台差异最大的部分。

### 问题本质

```
所有平台都面临同一个问题：

  SOCKS5 连接本身的流量不能走 TUN，否则路由回环。

三个平台的解决思路完全不同。
```

### 对比表

| 方面 | iOS | Android | Windows |
|------|-----|---------|---------|
| **机制** | 进程隔离 | protect(fd) | socket bind |
| **原理** | Extension 进程流量不走 TUN | 系统标记 fd 绕过 VPN | 绑定物理网卡 IP |
| **开发者工作** | 无（零代码） | 每个 socket 调用 protect | 每个 socket 调用 bind |
| **实现位置** | 系统内核 | VpnService 框架 | 应用代码 |
| **粒度** | 进程级别 | 单个 fd | 单个 socket |
| **需要获取** | 无 | VpnService 引用 | 物理网卡 IP |
| **跨语言** | 不需要 | JNI 回调 Java | 不需要（纯 Rust） |
| **崩溃安全** | 安全 | 安全 | 安全（bind 不影响路由表） |

### 代码对比

**iOS：零代码**
```swift
// 不需要任何处理！
// Extension 进程的流量天然不走 TUN
let connection = try await URLSession.shared.data(from: url)
```

**Android：protect(fd)**
```rust
// Rust 端：创建 socket 后通过 JNI 调用 Java protect
let fd = socket.as_raw_fd();
env.call_method(vpn_service, "protect", "(I)Z", &[fd.into()])?;
```

**Windows：socket bind**
```rust
// Rust 端：socket 创建后 bind 物理网卡 IP
let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
socket.bind(&SockAddr::from("192.168.1.100:0".parse()?))?;
socket.connect(&SockAddr::from(socks5_addr))?;
```

---

## 7.4 Rust 集成方式

| 方面 | iOS | Android | Windows |
|------|-----|---------|---------|
| **产物格式** | `.a` 静态库 | `.so` 动态库 | `.exe` 直接链接 |
| **调用方式** | C FFI (Swift → Rust) | JNI (Kotlin → Rust) | 直接 (Rust → Rust) |
| **桥接层** | `tunnel-core/src/ffi.rs` | `tunnel-core/src/jni_ffi.rs` | 无（直接 `mod tun`） |
| **头文件** | `tunnel_core.h` (C 头) | JNI 自动生成 | 不需要 |
| **编译工具** | `cargo lipo` | `cargo-ndk` | `cargo build` |
| **目标平台** | `aarch64-apple-ios` | `aarch64-linux-android` | `x86_64-pc-windows-msvc` |
| **交叉编译** | 从 macOS | 从 macOS/Linux | 从 macOS/Linux (mingw) |
| **复杂度** | 高（FFI + Xcode） | 高（JNI + Gradle） | 低（纯 Rust） |

### 集成复杂度排名

```
Windows < iOS < Android

Windows: 最简单
  - 纯 Rust 项目，mod tun 直接引用
  - 不需要 FFI/JNI 桥接
  - cargo build 即可

iOS: 中等
  - 需要定义 C FFI 接口
  - 需要 cbindgen 生成头文件
  - 需要 Xcode 项目配置
  - 需要处理内存管理（手动 free）

Android: 最复杂
  - 需要 JNI 桥接层
  - 需要处理 Java/Rust 类型转换
  - 需要 JNI_OnLoad 缓存 JVM 引用
  - 需要 Gradle + cargo-ndk 集成
  - 需要多 ABI 支持 (arm64, x86_64)
```

---

## 7.5 TUN 读写方式对比

| 方面 | iOS | Android | Windows |
|------|-----|---------|---------|
| **读取** | `packetFlow.readPackets()` | `read(tunFd, buf)` | `session.receive_blocking()` |
| **写入** | `packetFlow.writePackets()` | `write(tunFd, buf)` | `session.send_packet()` |
| **数据格式** | `[NSData]` + 协议族 | 原始 IP 包 | 原始 IP 包 |
| **阻塞模型** | completion handler | 阻塞 read | 阻塞 receive |
| **线程模型** | GCD 队列 | 独立线程 | 独立线程 |
| **缓冲区** | 系统管理 | 手动 buf | 环形缓冲区 (4MB) |
| **批量操作** | readPackets 批量读 | 单包 | 单包 |

### 读取代码对比

**iOS (Swift)**
```swift
packetFlow.readPackets { packets, protocols in
    for (i, packet) in packets.enumerated() {
        tunnel_feed_packet(context, packet.bytes, packet.count)
    }
}
```

**Android (Rust via fd)**
```rust
let n = unsafe { libc::read(tun_fd, buf.as_mut_ptr() as _, buf.len()) };
```

**Windows (Rust via wintun)**
```rust
let packet = session.receive_blocking()?;
let data = packet.bytes().to_vec();
```

---

## 7.6 共享代码 vs 平台特定代码

### 共享组件

| 组件 | 代码位置 | 三平台共用 |
|------|---------|-----------|
| netstack-smoltcp | 第三方 crate | 完全共用 |
| fast-socks5 | 第三方 crate | 完全共用 |
| Slint UI | `vpn-app/ui/main.slint` | Windows + (可选 iOS/Android) |
| SOCKS5 转发逻辑 | 各平台 main.rs | 逻辑相同，代码略有差异 |

### 平台特定组件

| 组件 | iOS | Android | Windows |
|------|-----|---------|---------|
| TUN 管理 | Swift (PacketTunnel) | Kotlin (VpnService) | Rust (tun.rs) |
| 路由绕过 | 无 | JNI protect | socket bind |
| UI 框架 | SwiftUI | Jetpack Compose | Slint |
| 构建系统 | Xcode + cargo | Gradle + cargo-ndk | cargo |
| 桥接层 | ffi.rs + .h | jni_ffi.rs | 无 |

### 代码量对比（估算）

```
共享代码（所有平台都使用的逻辑）：
  netstack 集成 + SOCKS5 转发 ≈ 200 行 Rust

平台特定代码：
  iOS:
    Swift (VPN Service + UI)        ≈ 300 行
    Rust FFI (ffi.rs)               ≈ 150 行
    Xcode 项目配置                   ≈ 大量 XML

  Android:
    Kotlin (Service + UI)           ≈ 400 行
    Rust JNI (jni_ffi.rs)           ≈ 200 行
    Gradle 配置                      ≈ 100 行

  Windows:
    Rust (main.rs + tun.rs)         ≈ 350 行  ← 全部 Rust！
    NSIS 安装脚本                    ≈ 100 行
```

---

## 7.7 各平台包大小对比

| 平台 | 组件 | 大小（约） |
|------|------|-----------|
| **iOS** | tunnel-core.a (静态库) | ~5MB |
| | App + Extension (IPA) | ~15MB |
| **Android** | libtunnel_core.so (arm64) | ~4MB |
| | APK (单 ABI) | ~8MB |
| | APK (多 ABI) | ~15MB |
| **Windows** | vpn-demo-windows.exe | ~8MB |
| | wintun.dll | ~400KB |
| | NSIS 安装包 | ~5MB |
| | 便携版 (exe + dll) | ~8.4MB |

---

## 7.8 开发体验对比

| 方面 | iOS | Android | Windows |
|------|-----|---------|---------|
| **调试难度** | 高（Extension 进程） | 中（JNI + logcat） | 低（直接运行） |
| **编译速度** | 慢（Xcode + lipo） | 慢（Gradle + ndk） | 快（cargo） |
| **热重载** | 不支持 | 不支持 | 不支持 |
| **日志查看** | Console.app | adb logcat | 终端 stdout |
| **权限获取** | 繁琐（证书 + entitlement） | 简单（manifest 声明） | 简单（manifest 嵌入） |
| **发布流程** | App Store 审核 | Play Store 审核 | 直接分发 |
| **最难的部分** | Xcode 项目配置 | JNI 类型转换 | Win32 API 参数 |

---

## 7.9 总结

### 三平台选型建议

```
如果只做一个平台：
  → Windows：最简单，纯 Rust，没有 FFI/JNI 开销

如果做移动端：
  → iOS 先：进程隔离最优雅，不需要处理路由绕过
  → Android 后：JNI 桥接较复杂，但 protect() 也可靠

如果三平台都做：
  → 先实现核心逻辑（netstack + SOCKS5 转发）
  → 再分别实现平台适配层
  → Slint UI 可以跨平台共享（但移动端体验不如原生 UI）
```

### 关键差异速记

```
路由绕过：  iOS 进程隔离 / Android protect / Windows socket bind
TUN 创建：  iOS 系统创建 / Android Builder / Windows wintun
Rust 集成： iOS FFI(.a) / Android JNI(.so) / Windows 直接(.exe)
权限获取：  iOS entitlement / Android manifest / Windows UAC
构建系统：  iOS Xcode / Android Gradle / Windows cargo
```
