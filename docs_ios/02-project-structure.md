# 第二章：项目结构与构建系统

---

## 2.1 整体架构概览

项目由两个构建系统协作：**Cargo**（Rust）和 **Xcode**（Swift/iOS）。

```
vpn-demo/
├── Cargo.toml                 # Rust workspace 根配置
├── tunnel-core/               # Rust 核心库（编译为 .a 静态库）
│   ├── Cargo.toml
│   ├── include/
│   │   └── tunnel_core.h     # C FFI 头文件（Swift 通过此文件调用 Rust）
│   └── src/
│       ├── lib.rs             # 库入口
│       └── ffi.rs             # FFI 导出函数
├── vpn-app/                   # Slint UI（桌面端预览用，iOS 不用）
├── ios/
│   └── VPNDemo/               # Xcode 项目根目录
│       ├── VPNDemo.xcodeproj/ # Xcode 项目文件
│       ├── VPNDemo/           # 主 App target 源码
│       │   ├── AppDelegate.swift
│       │   ├── SceneDelegate.swift
│       │   ├── ViewController.swift    # UI + 测试按钮
│       │   ├── VPNManager.swift        # VPN 配置管理
│       │   ├── BridgingHeader.h        # Swift-C 桥接头文件
│       │   └── VPNDemo.entitlements    # 权限声明
│       └── PacketTunnel/      # Network Extension target 源码
│           ├── PacketTunnelProvider.swift  # NE 核心逻辑
│           ├── BridgingHeader.h           # Swift-C 桥接头文件
│           ├── PacketTunnel.entitlements   # 权限声明
│           └── Info.plist                 # Extension 配置
├── build-ios.sh               # iOS 编译脚本
└── docs/                      # 本文档
```

### 构建产物关系

```
Cargo (Rust)                          Xcode (Swift)
    │                                      │
    ▼                                      ▼
libtunnel_core.a  ──链接到──►  PacketTunnel.appex (NE)
(静态库, arm64)                            │
    │                                      │
    └──────────链接到──►  VPNDemo.app (主 App)
                                           │
                                           ▼
                                    VPNDemo.app/
                                    ├── VPNDemo (主二进制)
                                    └── PlugIns/
                                        └── PacketTunnel.appex/
                                            └── PacketTunnel (NE 二进制)
```

---

## 2.2 Cargo Workspace 结构

### 根 Cargo.toml

```toml
[workspace]
members = ["tunnel-core", "vpn-app"]
resolver = "2"
```

`resolver = "2"` 使用 Cargo 新版依赖解析器，避免 feature 冲突。

### tunnel-core/Cargo.toml

```toml
[package]
name = "tunnel-core"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib", "lib"]
# "staticlib" → 编译为 .a 静态库（给 Xcode 链接用）
# "lib"       → 普通 Rust 库（给 vpn-app 等 Rust 项目用）

[dependencies]
netstack-smoltcp = "0.2"    # IP包 → TCP流/UDP报文 转换
tokio = { version = "1", features = [
    "rt-multi-thread",       # 多线程异步运行时
    "net",                   # TCP/UDP 网络操作
    "io-util",               # copy_bidirectional 等 IO 工具
    "sync",                  # mpsc channel 等同步原语
    "time",                  # timeout 等时间操作
    "macros",                # tokio::spawn 等宏
] }
futures = "0.3"             # Stream/Sink trait（netstack-smoltcp 需要）
fast-socks5 = "1"           # SOCKS5 客户端
log = "0.4"                 # 日志门面
oslog = "0.2"               # iOS 系统日志后端（os_log）
once_cell = "1"             # 全局静态初始化
parking_lot = "0.12"        # 高效互斥锁
```

### 为什么用 `staticlib`

iOS 不支持动态加载第三方 `.dylib`（出于安全策略），所以 Rust 代码必须编译为**静态库** `.a`，然后链接到 Swift 二进制中。编译后所有 Rust 代码、tokio 运行时、smoltcp 协议栈都被嵌入到最终的二进制里。

---

## 2.3 Xcode 项目配置

Xcode 项目包含两个 target：

### Target 1：VPNDemo（主 App）

| 配置项 | 值 | 说明 |
|---|---|---|
| Bundle ID | com.vpndemo.app.VPNDemo | 主 App 标识 |
| Frameworks | libtunnel_core.a | Rust 静态库 |
| Bridging Header | VPNDemo/BridgingHeader.h | 让 Swift 看到 C 函数 |
| Header Search Paths | ../../tunnel-core/include | 找到 tunnel_core.h |
| Library Search Paths | ../../target/aarch64-apple-ios/release | 找到 .a 文件 |
| Other Linker Flags | -lresolv | tokio DNS 解析需要 |
| Entitlements | packet-tunnel-provider | NE 权限 |

### Target 2：PacketTunnel（Network Extension）

| 配置项 | 值 | 说明 |
|---|---|---|
| Bundle ID | com.vpndemo.app.VPNDemo.tunnel | **必须以主 App ID 为前缀** |
| Product Type | com.apple.product-type.app-extension | Extension 类型 |
| Frameworks | NetworkExtension.framework + libtunnel_core.a | 系统框架 + Rust 库 |
| Bridging Header | PacketTunnel/BridgingHeader.h | 让 Swift 看到 C 函数 |
| NSExtensionPrincipalClass | PacketTunnelProvider | Extension 入口类 |
| NSExtensionPointIdentifier | com.apple.networkextension.packet-tunnel | 声明是 Packet Tunnel 类型 |

### Bundle ID 前缀规则

Extension 的 Bundle ID **必须**以主 App 的 Bundle ID 为前缀：

```
主 App:     com.vpndemo.app.VPNDemo
Extension:  com.vpndemo.app.VPNDemo.tunnel  ← 前缀匹配 ✓
Extension:  com.vpndemo.app.tunnel          ← 前缀不匹配 ✗（会报错）
```

这是 iOS 的安全策略，确保 Extension 只能属于对应的主 App。

---

## 2.4 Rust 静态库如何链接到 Swift

整个链接过程分为 4 步：

### 步骤 1：Rust 编译为静态库

```bash
cargo build --package tunnel-core --release --target aarch64-apple-ios
```

产物：`target/aarch64-apple-ios/release/libtunnel_core.a`

这个 `.a` 文件是一个 **ar 归档**，里面包含了所有 Rust 代码编译出的 `.o` 目标文件。

### 步骤 2：C 头文件声明接口

```c
// tunnel-core/include/tunnel_core.h
bool tunnel_start(const char *socks5_addr);
void tunnel_stop(void);
bool tunnel_feed_packet(const unsigned char *data, size_t len);
size_t tunnel_read_packet(unsigned char *buf, size_t buf_len);
```

这些函数在 Rust 中通过 `#[no_mangle] pub extern "C"` 导出，符号名不会被 Rust 编译器修改。

### 步骤 3：Bridging Header 让 Swift 看到 C 函数

```c
// PacketTunnel/BridgingHeader.h
#import "tunnel_core.h"
```

Xcode 设置中指定 `SWIFT_OBJC_BRIDGING_HEADER = PacketTunnel/BridgingHeader.h`，Swift 编译器会自动将 C 函数声明导入为 Swift 可调用的全局函数。

### 步骤 4：链接器将 .a 链接进最终二进制

在 Xcode 的 Build Phases → Link Binary With Libraries 中添加 `libtunnel_core.a`，链接器会：
1. 扫描 `.a` 中的符号（`tunnel_start`, `tunnel_stop` 等）
2. 将被引用的 `.o` 文件提取并链接到最终二进制
3. 未被引用的代码会被 dead code stripping 删除

### 调用链总结

```
Swift 代码                    C 接口                      Rust 代码
tunnel_start("addr")  ──►  tunnel_start(*char)  ──►  pub extern "C" fn tunnel_start(...)
    (自动桥接)              (Bridging Header)          (#[no_mangle])
```

---

## 2.5 构建脚本与交叉编译

### build-ios.sh

```bash
#!/bin/bash
# 编译 Rust 静态库给 iOS 真机 (arm64)
cargo build --package tunnel-core --release --target aarch64-apple-ios

# 编译给 iOS 模拟器 (arm64, Apple Silicon Mac)
cargo build --package tunnel-core --release --target aarch64-apple-ios-sim
```

### 交叉编译目标

| Target Triple | 平台 |
|---|---|
| `aarch64-apple-ios` | iOS 真机 (arm64) |
| `aarch64-apple-ios-sim` | iOS 模拟器 (Apple Silicon Mac) |
| `x86_64-apple-ios` | iOS 模拟器 (Intel Mac, 已罕见) |

安装编译目标：

```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
```

### 完整构建流程

```bash
# 1. 编译 Rust 静态库
./build-ios.sh

# 2. 编译 Xcode 项目（自动链接 .a）
xcodebuild -project ios/VPNDemo/VPNDemo.xcodeproj \
    -scheme VPNDemo \
    -configuration Release \
    -destination 'generic/platform=iOS' \
    -allowProvisioningUpdates \
    build

# 3. 安装到设备
xcrun devicectl device install app \
    --device <DEVICE_UUID> \
    /path/to/VPNDemo.app
```

### Debug vs Release 构建

| | Debug | Release |
|---|---|---|
| Rust 优化 | `-Onone` | `-O3`（`--release`） |
| Swift 优化 | `-Onone` | `-O` + whole module |
| 符号信息 | 完整 DWARF | 分离的 dSYM |
| 包大小 | ~7 MB | ~3 MB |
| NE 二进制 | ~3.6 MB | ~1.7 MB |
| 运行速度 | 较慢 | 正常 |
