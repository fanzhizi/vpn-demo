# 第一章：iOS Network Extension 基础

---

## 1.1 什么是 Network Extension

Network Extension 是 Apple 提供的一组框架，允许第三方 app 自定义 iOS/macOS 的网络行为。它包含多种 Provider 类型：

| Provider 类型 | 用途 |
|---|---|
| **NEPacketTunnelProvider** | 创建 VPN 隧道，拦截所有 IP 层流量（本项目使用） |
| NEAppProxyProvider | 应用层代理（类似 HTTP 代理） |
| NEFilterDataProvider | 内容过滤（网络防火墙） |
| NEDNSProxyProvider | 自定义 DNS 解析 |

### 为什么选择 NEPacketTunnelProvider

翻墙客户端需要拦截设备的**所有网络流量**（不仅仅是 HTTP），所以必须工作在 IP 层。`NEPacketTunnelProvider` 创建一个虚拟 TUN 网卡，iOS 内核将所有网络流量路由到这个虚拟网卡，我们的代码就能读取和处理每一个 IP 数据包。

### 特殊权限要求

使用 Network Extension 需要：
1. **Apple Developer Program 付费会员**（99 美元/年）
2. 在 Apple Developer Portal 申请 `com.apple.developer.networking.networkextension` entitlement
3. 在 Xcode 中配置对应的 Capability

这是 Apple 对 VPN 类应用的安全管控——不是任何人都能开发 VPN app。

---

## 1.2 NEPacketTunnelProvider 生命周期

`NEPacketTunnelProvider` 是一个继承自 `NEProvider` 的类，运行在**独立的进程**中（不是主 App 进程）。它的生命周期由 iOS 系统管理：

```
用户点击"连接 VPN"
        │
        ▼
┌───────────────────────────────────┐
│  iOS 系统启动 Extension 进程        │
│  创建 PacketTunnelProvider 实例     │
└───────────────┬───────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│  startTunnel(options:completion:)  │  ← 你的代码在这里初始化
│                                    │
│  1. 读取配置（SOCKS5 地址等）         │
│  2. 启动 Rust tunnel-core          │
│  3. 配置 TUN 网卡参数               │
│  4. 调用 completionHandler(nil)    │
│     告诉系统隧道已建立               │
└───────────────┬───────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│  隧道运行中                         │
│                                    │
│  packetFlow.readPackets() 持续读包  │
│  tunnel_read_packet() 持续写包      │
│  （这两个循环驱动所有数据流转）         │
└───────────────┬───────────────────┘
                │ 用户点击"断开"
                ▼
┌───────────────────────────────────┐
│  stopTunnel(with:completion:)      │
│                                    │
│  1. 停止 Rust tunnel-core          │
│  2. 清理资源                        │
│  3. 调用 completionHandler()       │
└───────────────┬───────────────────┘
                │
                ▼
        Extension 进程被系统回收
```

### 关键回调方法

```swift
class PacketTunnelProvider: NEPacketTunnelProvider {

    // 系统要求启动隧道时调用
    // options: 从主 App 通过 startVPNTunnel(options:) 传入的参数
    // completionHandler: 必须调用，传 nil 表示成功，传 Error 表示失败
    override func startTunnel(
        options: [String: NSObject]?,
        completionHandler: @escaping (Error?) -> Void
    )

    // 系统要求停止隧道时调用
    // reason: 停止原因（用户主动断开、配置变更、系统需要等）
    override func stopTunnel(
        with reason: NEProviderStopReason,
        completionHandler: @escaping () -> Void
    )
}
```

### completionHandler 的重要性

`startTunnel` 中的 `completionHandler` 必须被调用：
- 传 `nil`：系统认为隧道建立成功，VPN 状态变为 Connected
- 传 `Error`：系统认为隧道建立失败，VPN 状态变为 Disconnected
- **不调用**：系统会一直等待，最终超时报错

---

## 1.3 TUN 虚拟网卡的工作原理

### 什么是 TUN

TUN（network TUNnel）是操作系统内核提供的虚拟网络设备。与物理网卡不同，TUN 没有真实的硬件——它的数据收发由用户空间程序控制。

```
┌──────────────────────────────────────┐
│              iOS 内核                 │
│                                      │
│  ┌──────────┐    ┌──────────┐       │
│  │  物理网卡  │    │  TUN 网卡 │       │
│  │  (Wi-Fi)  │    │  (utun3) │       │
│  └─────┬────┘    └─────┬────┘       │
│        │               │            │
│   路由表决定数据包走哪个网卡              │
│                                      │
└────────┼───────────────┼─────────────┘
         │               │
         ▼               ▼
    物理网络         用户空间程序
   (真实发送)     (我们的 Extension)
```

### TUN vs TAP

| | TUN | TAP |
|---|---|---|
| 工作层级 | IP 层（第 3 层） | 以太网层（第 2 层） |
| 数据格式 | 原始 IP 数据包 | 以太网帧 |
| iOS 支持 | 是（通过 NE） | 否 |

iOS 的 `NEPacketTunnelProvider` 使用的是 TUN 模式，我们收到的是**纯 IP 数据包**（没有以太网帧头）。

### packetFlow：TUN 的读写接口

`NEPacketTunnelProvider` 继承自 `NETunnelProvider`，提供了 `packetFlow` 属性，类型是 `NEPacketTunnelFlow`：

```swift
// 从 TUN 读取数据包（设备发出的流量）
// packets: [Data] — 每个 Data 是一个完整的 IP 数据包
// protocols: [NSNumber] — 每个包的协议族（AF_INET=2 表示 IPv4, AF_INET6=30 表示 IPv6）
packetFlow.readPackets { packets, protocols in
    // 处理数据包...
    // 必须在回调中再次调用 readPackets 形成循环
}

// 向 TUN 写入数据包（发给设备的流量）
packetFlow.writePackets([responseData], withProtocols: [NSNumber(value: AF_INET)])
```

**注意**：`readPackets` 不是流式 API，它是回调式的。每次回调可能返回多个包。处理完后必须再次调用 `readPackets` 以继续接收，形成一个递归循环。

---

## 1.4 iOS 如何路由流量到 TUN

当 VPN 隧道建立后，我们通过 `setTunnelNetworkSettings()` 告诉 iOS 如何配置 TUN 网卡：

```swift
let settings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "10.0.0.1")

// IPv4 配置
let ipv4 = NEIPv4Settings(
    addresses: ["10.0.0.2"],        // TUN 网卡的 IP 地址
    subnetMasks: ["255.255.255.0"]   // 子网掩码
)

// includedRoutes: 哪些流量走 TUN
ipv4.includedRoutes = [NEIPv4Route.default()]  // default() = 0.0.0.0/0 = 所有流量

// 注意：不需要 excludedRoutes 来排除代理服务器 IP！（详见下文）

settings.ipv4Settings = ipv4

// DNS 配置 — 告诉系统使用哪个 DNS 服务器
settings.dnsSettings = NEDNSSettings(servers: ["8.8.8.8", "8.8.4.4"])

settings.mtu = 1500  // 最大传输单元

// 应用配置
setTunnelNetworkSettings(settings) { error in
    // 配置生效后，iOS 内核开始将流量路由到 TUN
}
```

### excludedRoutes 的用途

`excludedRoutes` **不是**用来防止代理服务器连接回环的（那是 Android 的思路）。在 iOS 上它用于 **split tunneling**（分流）：

```swift
// 仅在需要分流时使用，例如让内网地址绕过 VPN
ipv4.excludedRoutes = [
    NEIPv4Route(destinationAddress: "192.168.0.0", subnetMask: "255.255.0.0"),  // 内网直连
    NEIPv4Route(destinationAddress: "10.0.0.0", subnetMask: "255.0.0.0"),       // 内网直连
]
```

### iOS 不存在路由回环问题

这是 iOS 和 Android 在 VPN 架构上的**本质区别**。在 Android 上，VPN 代码如果不调用 `protect(fd)`，自身的出站连接会被 TUN 捕获，形成回环。但 iOS 上这个问题**天然不存在**：

```
iOS NE Extension 进程发起的网络连接
    → iOS 内核检测到：这是 Tunnel Provider 进程自身的流量
    → 自动走物理网卡，不经过 TUN
    → 不会回环 ✓
```

**原因**：`NEPacketTunnelProvider` 运行在一个独立的系统扩展进程中，iOS 内核对这个进程做了特殊处理——它发起的标准网络连接（BSD socket / `NWConnection` / `URLSession` 等）默认走物理网卡，不会被自己创建的 TUN 虚拟网卡捕获。

这意味着我们在 Extension 进程中用 Rust 的 `tokio::net::TcpStream::connect()` 连接 SOCKS5 服务器，这个连接**自动绕过 VPN**，无需任何额外处理。

### 与 Android 的对比

| 需求 | Android | iOS |
|---|---|---|
| VPN 自身的出站 socket 不走 TUN | 需要 `VpnService.protect(fd)` | **系统自动**，NE 进程默认走物理网卡 |
| 按目标 IP 分流（split tunneling） | `addRoute()` 路由配置 | `excludedRoutes` / `includedRoutes` |
| 按 App 分流 | `addDisallowedApplication()` | Per-App VPN（仅 MDM 管理场景） |
| 代码复杂度 | ~80 行（JNI + socket2 + protect） | 零（什么都不用做） |

### Extension 进程中的网络 API

在 `NEPacketTunnelProvider` 中建立网络连接，有两类 API：

```swift
// ❶ 走物理网卡（默认行为）— 用于连接代理服务器
let connection = NWConnection(host: "proxy.example.com", port: 1080, using: .tcp)
// 或者直接用 BSD socket / Rust 的 TcpStream::connect()
// 这些连接不会被 TUN 捕获

// ❷ 走 TUN 虚拟网卡（显式指定）— 极少使用
let tunnelConn = createTCPConnectionThroughTunnel(to: endpoint, enableTLS: false, ...)
// 这个连接会被路由回 TUN，通常不是你想要的
```

> **leaf 项目的做法**：iOS 端没有任何 `protect_socket` 代码（`#[cfg(target_os = "android")]` only），完全依赖 NE 进程的系统级隔离。这是所有 iOS VPN 客户端（Surge、Shadowrocket、Clash for iOS 等）的标准做法。

---

## 1.5 Extension 与主 App 的进程隔离

这是 iOS Network Extension 开发中最需要理解的概念，也是 **iOS 不需要 `protect()` 的根本原因**：

```
┌─────────────────────┐    ┌─────────────────────────────────┐
│     主 App 进程       │    │   Extension 进程 (系统特殊对待)    │
│                     │    │                                  │
│  ViewController     │    │  PacketTunnelProvider            │
│  VPNManager         │    │  tunnel-core (Rust)              │
│                     │    │  netstack-smoltcp                │
│  没有网络处理能力      │    │  SOCKS5 转发                     │
│  只负责 UI 和配置     │    │                                  │
│                     │    │  ⚡ 此进程发起的 socket 连接       │
│                     │    │     自动走物理网卡，不经过 TUN      │
└──────────┬──────────┘    └──────────┬───────────────────────┘
           │                          │
           │    NETunnelProvider       │
           │    Manager (IPC)          │
           └──────────────────────────┘
```

### 关键区别

| | 主 App | Extension |
|---|---|---|
| 进程 | 独立进程 | 独立进程（与主 App 不同！） |
| 生命周期 | 用户打开/关闭 | iOS 系统管理 |
| 内存限制 | 宽松 | **严格（约 15-50 MB）** |
| 数据共享 | 直接访问 | 需通过 App Group |
| 职责 | UI、配置 VPN | 实际的网络数据处理 |
| 出站网络 | 走 TUN（被 VPN 捕获） | **走物理网卡**（系统级隔离） |

### 进程隔离带来的 socket 保护

这个进程隔离设计意味着：
- 主 App 进程的网络流量 → 被 TUN 捕获 → 由 Extension 处理
- Extension 进程的网络流量 → **直接走物理网卡** → 不被 TUN 捕获

这就是为什么 iOS 不需要 Android 的 `protect(fd)` 机制。Rust tunnel-core 在 Extension 进程中通过 `TcpStream::connect()` 连接 SOCKS5 服务器，iOS 内核自动让这个连接绕过 TUN。

对比 Android：VpnService 运行在 App 主进程中，所有 socket 都会被自己的 TUN 捕获，所以必须对代理相关的 socket 调用 `protect(fd)` 来豁免。

### 通信方式

主 App 和 Extension 之间的通信非常有限：

1. **启动参数**：`startVPNTunnel(options:)` 可以传一个字典
2. **协议配置**：`NETunnelProviderProtocol.providerConfiguration` 字典
3. **App Group**：共享 UserDefaults 或文件目录
4. **IPC 消息**：`sendProviderMessage()` / `handleAppMessage()` （实时通信）

本项目中，SOCKS5 地址通过 `options` 和 `providerConfiguration` 两种方式传递给 Extension，确保不管哪种启动方式都能获取到。

### 为什么 Rust 代码运行在 Extension 中

因为实际的网络数据处理（读 TUN 包、解析 TCP、转发 SOCKS5）都发生在 Extension 进程里。所以 `libtunnel_core.a` 必须链接到 **PacketTunnel** target（Extension），而不是主 App target。

主 App 中也链接了 `libtunnel_core.a`，但仅用于测试功能（`tunnel_test_socks5`），不涉及实际的隧道数据处理。
