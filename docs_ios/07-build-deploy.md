# 第七章：构建、调试与部署

---

## 7.1 环境准备

### 必需软件

| 软件 | 版本要求 | 用途 |
|---|---|---|
| **Rust** | stable 最新版 | 编译 tunnel-core |
| **Xcode** | 15.0+ | 编译 iOS App 和 Extension |
| **Apple Developer Account** | 付费会员 (99$/年) | Network Extension 权限 |
| **iPhone** | iOS 15.0+ | 真机调试（模拟器不支持 NE） |
| **USB 线** 或 **Wi-Fi 调试** | — | 连接 iPhone 到 Mac |

### Rust 环境安装

```bash
# 安装 Rust（如果未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 添加 iOS 编译目标
rustup target add aarch64-apple-ios         # 真机 (iPhone arm64)
rustup target add aarch64-apple-ios-sim     # 模拟器 (Apple Silicon Mac)

# 验证安装
rustup target list --installed
# 应该看到:
#   aarch64-apple-darwin (default)
#   aarch64-apple-ios
#   aarch64-apple-ios-sim
```

### Apple Developer 账号配置

1. 注册 [Apple Developer Program](https://developer.apple.com/programs/)（99 美元/年）
2. 在 Xcode 中登录：Xcode → Settings → Accounts → 添加 Apple ID
3. 申请 Network Extension 权限（见 7.5 节）

> **重要**：免费的 Apple ID 无法使用 Network Extension。必须是付费的 Developer Program 会员。

---

## 7.2 从零开始的完整构建流程

### 第一步：克隆项目

```bash
git clone <项目地址>
cd vpn-demo
```

### 第二步：编译 Rust 静态库

```bash
# 使用项目提供的脚本
chmod +x build-ios.sh
./build-ios.sh
```

脚本内容如下：

```bash
#!/bin/bash
set -e

echo "=== Building tunnel-core for iOS (arm64) ==="
cargo build --package tunnel-core --release --target aarch64-apple-ios

echo "=== Building tunnel-core for iOS Simulator (arm64) ==="
cargo build --package tunnel-core --release --target aarch64-apple-ios-sim

echo ""
echo "=== Build complete ==="
echo "Static libraries:"
echo "  Device:    target/aarch64-apple-ios/release/libtunnel_core.a"
echo "  Simulator: target/aarch64-apple-ios-sim/release/libtunnel_core.a"
```

编译完成后的产物：

```
vpn-demo/
├── target/
│   ├── aarch64-apple-ios/release/
│   │   └── libtunnel_core.a          ← 真机用静态库 (~15MB release)
│   └── aarch64-apple-ios-sim/release/
│       └── libtunnel_core.a          ← 模拟器用静态库
└── tunnel-core/
    └── include/
        └── tunnel_core.h             ← C 头文件（手动编写）
```

### 第三步：打开 Xcode 项目

```bash
open ios/VPNDemo/VPNDemo.xcodeproj
```

### 第四步：配置签名

1. 在 Xcode 左侧导航器中选择项目（蓝色图标）
2. 分别选择 **VPNDemo** target 和 **PacketTunnel** target
3. 在 "Signing & Capabilities" 标签中：
   - 勾选 "Automatically manage signing"
   - 选择你的 Team（付费开发者账号）
   - 确保 Bundle Identifier 唯一

### 第五步：连接 iPhone 并构建

1. 用 USB 线连接 iPhone 到 Mac
2. 在 Xcode 顶部选择你的 iPhone 作为目标设备
3. 点击 Run（Cmd+R）或 Product → Build（Cmd+B）

---

## 7.3 Xcode 项目创建详细步骤

如果需要从零创建 Xcode 项目（而不是使用仓库中已有的），按以下步骤操作：

### 步骤一：创建主 App 项目

1. Xcode → File → New → Project
2. 选择 "iOS" → "App"
3. 填写信息：
   - Product Name: `VPNDemo`
   - Team: 你的开发者账号
   - Organization Identifier: `com.vpndemo.app`（最终 Bundle ID 为 `com.vpndemo.app.VPNDemo`）
   - Interface: **Storyboard**（本项目用纯代码布局，但 Storyboard 模板更简单）
   - Language: **Swift**
4. 选择保存位置，点击 Create

### 步骤二：添加 Network Extension target

1. File → New → Target
2. 选择 "iOS" → "Network Extension"
3. 填写信息：
   - Product Name: `PacketTunnel`
   - Team: 同上
   - Bundle Identifier: `com.vpndemo.app.VPNDemo.tunnel`（**必须是主 App Bundle ID 的子标识符**）
   - Provider Type: **Packet Tunnel Provider**
   - Language: **Swift**
4. 点击 Finish
5. 弹出 "Activate scheme?" 选择 **Cancel**（保持主 App 为活动 scheme）

Xcode 会自动生成 `PacketTunnelProvider.swift` 文件，用项目中的版本替换它。

### 步骤三：添加 Rust 静态库

需要将 `libtunnel_core.a` 添加到**两个** target：

**添加到 PacketTunnel target（Extension）：**

1. 选择项目 → PacketTunnel target → "General" 标签
2. 滚动到 "Frameworks and Libraries"
3. 点击 "+" → "Add Other..." → "Add Files..."
4. 导航到 `vpn-demo/target/aarch64-apple-ios/release/libtunnel_core.a`
5. 确保 "Add to targets" 勾选了 PacketTunnel

**添加到 VPNDemo target（主 App，用于测试功能）：**

1. 同样的步骤，但选择 VPNDemo target

### 步骤四：配置 Header Search Paths

让 Swift 通过 Bridging Header 找到 `tunnel_core.h`：

1. 选择项目 → PacketTunnel target → "Build Settings" 标签
2. 搜索 "Header Search Paths"
3. 添加：`$(SRCROOT)/../../tunnel-core/include`（相对于 Xcode 项目目录的路径）
4. 对 VPNDemo target 执行相同操作

> 路径说明：Xcode 项目在 `ios/VPNDemo/` 中，`tunnel-core/include/` 在项目根目录，所以需要 `../../` 向上两级。

### 步骤五：创建 Bridging Header

Swift 不能直接调用 C 函数，需要通过 Bridging Header 引入 C 头文件。

**为 PacketTunnel target 创建：**

1. 在 `ios/VPNDemo/PacketTunnel/` 目录下创建文件 `BridgingHeader.h`
2. 内容：

```c
#import "tunnel_core.h"
```

3. 配置 Build Settings：
   - 选择 PacketTunnel target → Build Settings
   - 搜索 "Objective-C Bridging Header"
   - 设置为：`PacketTunnel/BridgingHeader.h`

**为 VPNDemo target 创建：**

1. 在 `ios/VPNDemo/VPNDemo/` 目录下创建文件 `BridgingHeader.h`
2. 内容相同：`#import "tunnel_core.h"`
3. 配置 Build Settings：
   - 选择 VPNDemo target → Build Settings
   - 搜索 "Objective-C Bridging Header"
   - 设置为：`VPNDemo/BridgingHeader.h`

### 步骤六：添加 Network Extension Capability

1. 选择项目 → VPNDemo target → "Signing & Capabilities" 标签
2. 点击 "+ Capability"
3. 搜索并添加 "Network Extensions"
4. 勾选 "Packet Tunnel"

对 PacketTunnel target 执行相同操作。

### 步骤七：配置 App Group（可选但推荐）

如果主 App 和 Extension 需要共享数据：

1. 选择项目 → 两个 target 都添加 "App Groups" capability
2. 创建 group：`group.com.vpndemo.app.VPNDemo`

### 步骤八：添加源文件

将以下文件添加到对应 target：

| 文件 | Target |
|---|---|
| `ViewController.swift` | VPNDemo |
| `VPNManager.swift` | VPNDemo |
| `PacketTunnelProvider.swift` | PacketTunnel |

---

## 7.4 常见编译错误与解决

### 错误 1：Bridging Header not found

```
<unknown>:0: error: bridging header 'xxx/BridgingHeader.h' does not exist
```

**原因**：Build Settings 中 "Objective-C Bridging Header" 的路径不正确。

**解决**：
1. 确认 BridgingHeader.h 文件确实存在于指定路径
2. 路径是相对于 `$(SRCROOT)`（即 .xcodeproj 所在目录）的
3. 对于本项目：
   - VPNDemo target: `VPNDemo/BridgingHeader.h`
   - PacketTunnel target: `PacketTunnel/BridgingHeader.h`

### 错误 2：Undefined symbols（链接错误）

```
Undefined symbols for architecture arm64:
  "_tunnel_start", referenced from: ...
  "_tunnel_stop", referenced from: ...
  "_tunnel_feed_packet", referenced from: ...
```

**原因**：`libtunnel_core.a` 未链接到对应 target，或者链接了错误架构的库。

**解决**：
1. 确认 `libtunnel_core.a` 已添加到 target 的 "Frameworks and Libraries"
2. 确认使用了正确架构的库：
   - 真机：`target/aarch64-apple-ios/release/libtunnel_core.a`
   - 模拟器：`target/aarch64-apple-ios-sim/release/libtunnel_core.a`
3. 如果仍然报错，可能需要在 Build Settings 中手动添加 "Library Search Paths"：
   ```
   $(SRCROOT)/../../target/aarch64-apple-ios/release
   ```

### 错误 3：Undefined symbols for system libraries

```
Undefined symbols:
  "_SecRandomCopyBytes", referenced from: ...
  "_CFRelease", referenced from: ...
```

**原因**：Rust 依赖了系统框架（如 Security, CoreFoundation），但 Xcode 没有自动链接。

**解决**：在 target 的 "Frameworks and Libraries" 中添加：
- `Security.framework`
- `libresolv.tbd`（如果有 DNS 相关符号缺失）

### 错误 4：Bundle ID 前缀不匹配

```
Provisioning profile "xxx" doesn't match the bundle identifier "com.xxx.PacketTunnel"
```

**原因**：Extension 的 Bundle ID 必须是主 App Bundle ID 的子标识符。

**解决**：
- 主 App Bundle ID: `com.vpndemo.app.VPNDemo`
- Extension Bundle ID: `com.vpndemo.app.VPNDemo.tunnel`（必须以主 App ID 开头）

错误示例：
- `com.vpndemo.tunnel` — 不是 `com.vpndemo.app.VPNDemo` 的子标识符

### 错误 5：MinimumOSVersion 不匹配

```
The XXX target's iOS deployment target (16.0) should be equal to or less than
the main target's deployment target (15.0)
```

**原因**：Extension 的最低部署版本高于主 App。

**解决**：确保两个 target 的 Deployment Target 一致：
1. 选择项目 → 分别选择两个 target
2. "General" → "Minimum Deployments" → 设置相同的 iOS 版本（如 iOS 15.0）

### 错误 6：UDP/DNS 不工作

**现象**：VPN 连接成功，但网页打不开（DNS 解析失败）。

**原因及解决**：

1. **netstack-smoltcp 没有启用 UDP**：确保构建 stack 时启用了 UDP：
   ```rust
   StackBuilder::default()
       .enable_tcp(true)
       .enable_udp(true)    // 必须启用！
       .enable_icmp(true)
       .build()?;
   ```

2. **DNS 设置未生效**：确保 `setTunnelNetworkSettings` 中配置了 DNS：
   ```swift
   settings.dnsSettings = NEDNSSettings(servers: ["8.8.8.8", "8.8.4.4"])
   ```

3. **UDP 转发函数超时**：检查 `forward_udp` 是否能连接到 DNS 服务器。如果 DNS 服务器的 IP 被路由到 TUN（回环），需要在 `excludedRoutes` 中排除。

### 错误 7：providerBundleIdentifier 不匹配

**现象**：VPN 状态卡在 "Connecting"，然后变为 "Disconnected"。

**原因**：`NETunnelProviderProtocol.providerBundleIdentifier` 与 Extension target 的 Bundle Identifier 不一致。

**解决**：

```swift
// VPNManager.swift 中
proto.providerBundleIdentifier = "com.vpndemo.app.VPNDemo.tunnel"
// 必须与 PacketTunnel target 的 Bundle Identifier 完全一致！
```

在 Xcode 中检查：选择 PacketTunnel target → General → Bundle Identifier。

---

## 7.5 签名与 Entitlements

### 什么是 Entitlements

Entitlements（权限声明）是嵌入在 App 签名中的键值对，告诉 iOS 系统这个 App 有权使用哪些受限功能。Network Extension 是一个需要特殊权限的功能。

### 主 App 的 Entitlements

文件：`ios/VPNDemo/VPNDemo/VPNDemo.entitlements`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.developer.networking.networkextension</key>
    <array>
        <string>packet-tunnel-provider</string>
    </array>
</dict>
</plist>
```

### Extension 的 Entitlements

文件：`ios/VPNDemo/PacketTunnel/PacketTunnel.entitlements`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <!-- Network Extension 权限 -->
    <key>com.apple.developer.networking.networkextension</key>
    <array>
        <string>packet-tunnel-provider</string>
    </array>

    <!-- App Group：用于主 App 和 Extension 之间共享数据 -->
    <key>com.apple.security.application-groups</key>
    <array>
        <string>group.com.vpndemo.app.VPNDemo</string>
    </array>
</dict>
</plist>
```

### 如何向 Apple 申请 Network Extension 权限

Network Extension 不是随便就能用的。你需要：

1. **登录 [Apple Developer Portal](https://developer.apple.com/account)**

2. **提交 NE 权限申请**：
   - 访问 https://developer.apple.com/contact/request/network-extension
   - 填写表单，说明你的 App 使用 NE 的目的
   - 通常 1-3 个工作日内会收到审批邮件

3. **审批通过后配置 Provisioning Profile**：
   - 在 Developer Portal → Certificates, Identifiers & Profiles
   - 编辑你的 App ID，启用 "Network Extensions" capability
   - 重新生成 Provisioning Profile
   - 在 Xcode 中刷新：Xcode → Settings → Accounts → 选择账号 → 下载手动配置的 Profile

4. **自动签名的情况**：
   - 如果使用 "Automatically manage signing"，Xcode 会自动处理 Profile
   - 但前提是 Apple 已经审批了你的 NE 权限申请

> **注意**：如果权限未审批就尝试构建，会遇到签名错误：
> `Provisioning profile doesn't include the com.apple.developer.networking.networkextension entitlement`

---

## 7.6 真机调试技巧

### Network Extension 无法在模拟器上运行

这是 iOS 开发中必须面对的限制：**NEPacketTunnelProvider 只能在真机上运行**。模拟器没有真实的网络栈，无法创建 TUN 设备。

### 查看 Extension 日志

Extension 运行在独立进程中，Xcode 默认不会显示其日志。以下是几种查看方法：

#### 方法一：Xcode 附加调试器

最方便的方法，但需要 VPN 隧道已经启动：

1. 在 Xcode 中运行主 App
2. 点击 "Connect VPN" 启动隧道
3. Xcode 菜单 → Debug → Attach to Process → 选择 `PacketTunnel`
4. 附加成功后，Extension 的 `os_log` 输出会出现在 Xcode 控制台
5. 可以在 Extension 代码中设置断点

#### 方法二：idevicesyslog

实时查看设备所有日志，适合快速定位问题：

```bash
# 安装
brew install libimobiledevice

# 查看所有日志（信息量巨大，建议过滤）
idevicesyslog

# 过滤只看我们的日志
idevicesyslog | grep -E "tunnel|vpndemo|PacketTunnel"

# 更精确的过滤（只看 Rust 输出）
idevicesyslog | grep "com.vpndemo.app.tunnel.rust"
```

#### 方法三：xcrun devicectl（iOS 17+）

Apple 新的设备管理工具：

```bash
# 查看设备列表
xcrun devicectl list devices

# 实时日志流
xcrun devicectl device info log --device <设备UDID>

# 带过滤的日志
xcrun devicectl device info log --device <设备UDID> \
    --predicate 'subsystem == "com.vpndemo.app.tunnel"'
```

#### 方法四：macOS Console.app

图形化日志查看器：

1. 打开 "应用程序 → 实用工具 → 控制台"
2. 左侧栏选择你的 iPhone（需要 USB 连接）
3. 右上角搜索框输入过滤条件：`subsystem:com.vpndemo.app.tunnel`
4. 开始捕获日志

### 调试 Extension 启动失败

如果 VPN 状态卡在 "Connecting" 然后变回 "Disconnected"，通常是 Extension 启动失败。调试步骤：

1. **查看系统日志**：用上述方法查看 Extension 的日志，看是否有错误输出
2. **检查 `startTunnel` 中的每一步**：
   - SOCKS5 地址是否正确获取？
   - `tunnel_start()` 是否返回 `true`？
   - `setTunnelNetworkSettings` 是否成功？
   - `completionHandler` 是否被调用？
3. **检查 Entitlements**：确保权限配置正确
4. **检查 providerBundleIdentifier**：必须与 Extension 的 Bundle ID 完全匹配

### 调试数据包流转

当 VPN 连接成功但网络不通时：

```swift
// 在 PacketTunnelProvider 中添加更多日志
// 检查包是否进入了 readPackets 回调
os_log("readPackets called with %d packets", log: log, type: .info, packets.count)

// 检查 tunnel_feed_packet 是否成功
let ok = tunnel_feed_packet(baseAddr, ptr.count)
os_log("feed_packet: %d bytes, ok=%d", log: log, type: .info, ptr.count, ok ? 1 : 0)

// 检查是否有输出包
if len > 0 {
    os_log("read_packet: %d bytes", log: log, type: .info, len)
}
```

如果入站包有但出站包为零，问题在 Rust 侧（netstack 或 SOCKS5 连接）。
如果入站包为零，问题在路由配置（流量没有被路由到 TUN）。

---

## 7.7 Release 构建与大小优化

### Debug vs Release 对比

```bash
# Debug 构建（默认）
cargo build --package tunnel-core --target aarch64-apple-ios
# 产物大小: ~50-80 MB（包含调试符号）

# Release 构建
cargo build --package tunnel-core --release --target aarch64-apple-ios
# 产物大小: ~10-15 MB
```

| 指标 | Debug | Release |
|---|---|---|
| 编译速度 | 快（增量编译） | 慢（全量优化） |
| 库文件大小 | 50-80 MB | 10-15 MB |
| 运行性能 | 较慢 | 快（优化后） |
| 调试信息 | 完整 | 无（可配置保留） |
| 适用场景 | 开发调试 | 发布 |

### 进一步优化静态库大小

在 `tunnel-core/Cargo.toml` 中添加 Release profile 配置：

```toml
[profile.release]
opt-level = "z"      # 优化目标：最小体积（而非最快速度）
lto = true           # Link-Time Optimization：跨 crate 优化
codegen-units = 1    # 单编译单元：更好的优化机会（但编译更慢）
panic = "abort"      # panic 时直接 abort（省去 unwinding 代码）
strip = true         # 移除符号表
```

优化效果估计：

| 配置 | 大小 |
|---|---|
| Release 默认 | ~15 MB |
| + opt-level="z" | ~12 MB |
| + lto=true | ~8 MB |
| + strip=true | ~6 MB |
| 全部启用 | ~5 MB |

### Xcode 侧的优化

在 Xcode 的 Build Settings 中：

1. **Build Configuration**：
   - Xcode 顶部 scheme 选择 "Release"
   - 或 Product → Archive（自动使用 Release）

2. **Strip Debug Symbols**：
   - Build Settings → Strip Debug Symbols During Copy → Yes

3. **Bitcode**（已废弃）：
   - Xcode 14+ 不再需要 Bitcode

### 完整的 Release 构建命令

```bash
# 1. 编译 Rust Release 版本
./build-ios.sh   # 脚本已使用 --release 参数

# 2. 在 Xcode 中 Archive
# Product → Archive（或 Cmd+Shift+B 选择 Release scheme）

# 3. 导出 IPA
# Xcode Organizer → Distribute App
```

### App 体积分析

最终 App 的体积组成：

```
VPNDemo.app/
├── VPNDemo (主 App 二进制)           ~1 MB
├── PlugIns/
│   └── PacketTunnel.appex/
│       └── PacketTunnel (Extension 二进制)  ~5-15 MB (主要是 Rust 库)
├── Frameworks/                        ~0 MB (无动态库)
└── 资源文件                            ~1 MB
                                    ─────────
                                    总计 ~7-17 MB
```

Rust 静态库是体积的主要来源。如果对体积有严格要求，可以：
1. 使用上述 Cargo profile 优化
2. 审查依赖，移除不必要的 crate
3. 考虑使用 `cargo-bloat` 分析哪些函数占用最多空间：
   ```bash
   cargo install cargo-bloat
   cargo bloat --release --target aarch64-apple-ios -n 20
   ```

---

## 7.8 Mac Catalyst 支持

Mac Catalyst 允许 iOS app 直接在 macOS 上运行，复用同一套 Swift 代码和 NE Extension。但有几个关键差异需要处理。

### 为什么同一套代码不能直接在 Mac 上跑

iOS app 在 Mac Catalyst 模式下运行时，macOS 对 App Sandbox 有**强制要求**，而 iOS 上没有这个要求：

| 差异 | iOS | Mac Catalyst |
|---|---|---|
| App Sandbox | 不需要（iOS 自带沙盒） | **必须显式声明** |
| 网络权限 | 默认允许 | **必须声明 `network.client`** |
| Rust 编译目标 | `aarch64-apple-ios` | `aarch64-apple-ios-macabi`（专用 ABI） |
| 库格式 | `.a` (staticlib) | `.a` (staticlib)，但 ABI 不同 |

### Entitlements 差异

iOS 上只需要 Network Extension entitlement：

```xml
<!-- iOS 上只需这些 -->
<key>com.apple.developer.networking.networkextension</key>
<array>
    <string>packet-tunnel-provider</string>
</array>
```

Mac Catalyst 上还需要额外的 App Sandbox 权限，否则 PacketTunnel Extension **静默失败**（不崩溃、无日志，只显示 "internal error"）：

```xml
<!-- Mac Catalyst 必须添加 -->
<key>com.apple.security.app-sandbox</key>
<true/>
<key>com.apple.security.network.client</key>
<true/>
<key>com.apple.security.network.server</key>
<true/>

<!-- 原有的 NE 权限 -->
<key>com.apple.developer.networking.networkextension</key>
<array>
    <string>packet-tunnel-provider</string>
</array>
```

**这些 entitlements 在 iOS 上是无害的**（iOS 忽略 sandbox 相关的 key），所以可以直接添加到两个 target 的 `.entitlements` 文件中，不需要条件编译。

### Rust 编译目标

Mac Catalyst 有自己的 target triple，不能用普通 macOS 或 iOS 的库：

```
aarch64-apple-ios        → iOS 真机
aarch64-apple-ios-sim    → iOS 模拟器
aarch64-apple-darwin     → 原生 macOS（不能用于 Catalyst！）
aarch64-apple-ios-macabi → Mac Catalyst ← 必须用这个
```

如果错误地链接了 `aarch64-apple-darwin` 的库，链接器会报错：

```
ld: building for 'macCatalyst', but linking in object file built for 'macOS'
```

如果错误地链接了 `.dylib`（因为 `crate-type` 包含 `cdylib`），运行时会崩溃：

```
Library not loaded: libtunnel_core.dylib
```

### 解决方案：per-SDK Library Search Paths

在 Xcode 的 Build Settings 中，使用条件化的 `LIBRARY_SEARCH_PATHS`：

```
LIBRARY_SEARCH_PATHS[sdk=iphoneos*]         = .../target/aarch64-apple-ios/release
LIBRARY_SEARCH_PATHS[sdk=macosx*]           = .../target/aarch64-apple-ios-macabi/release
LIBRARY_SEARCH_PATHS[sdk=iphonesimulator*]  = .../target/aarch64-apple-ios-sim/release
```

### 完整构建命令

```bash
# 1. 编译所有 target 的 Rust 库
rustup target add aarch64-apple-ios-macabi
cargo build --package tunnel-core --release --target aarch64-apple-ios          # iOS
cargo build --package tunnel-core --release --target aarch64-apple-ios-macabi   # Mac Catalyst
cargo build --package tunnel-core --release --target aarch64-apple-ios-sim      # 模拟器

# 2. 删除 dylib（防止 Xcode 优先链接动态库导致崩溃）
rm -f target/aarch64-apple-ios-macabi/release/libtunnel_core.dylib

# 3. 构建 Mac Catalyst 版
xcodebuild -scheme VPNDemo \
    -destination 'platform=macOS,variant=Mac Catalyst,arch=arm64' \
    -allowProvisioningUpdates build
```

### 设备注册

Mac Catalyst 构建需要在 Apple Developer Portal 注册 Mac 设备：
1. 获取 Hardware UUID：`ioreg -d2 -c IOPlatformExpertDevice | awk -F\" '/IOPlatformUUID/{print $(NF-1)}'`
2. 登录 https://developer.apple.com/account/resources/devices/list
3. 添加设备：Platform = macOS，Device ID = Hardware UUID

### 调试技巧

Mac Catalyst 的 NE Extension 日志可以通过 macOS 的 `log` 命令查看：

```bash
# 实时查看 NE 相关日志
/usr/bin/log stream --predicate 'eventMessage CONTAINS "VPN Demo" OR process CONTAINS "PacketTunnel"' --info --debug

# 查看最近 5 分钟的日志
/usr/bin/log show --last 5m --predicate 'eventMessage CONTAINS "VPN Demo"' --info --debug
```

常见错误信息及原因：

| 错误 | 原因 |
|---|---|
| `Plugin failed` | Extension 加载失败，通常是 entitlements 问题 |
| `internal error occurred` | 缺少 App Sandbox entitlements |
| `building for macCatalyst, but linking for macOS` | 链接了错误 target 的 `.a` |
| `Library not loaded: libtunnel_core.dylib` | `cdylib` 生成了 `.dylib`，需要删除 |
