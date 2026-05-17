# 第六章：Swift 端 — UI 与 VPN 管理

---

## 6.1 VPNManager: NETunnelProviderManager 的封装

`VPNManager` 是对 Apple 的 `NETunnelProviderManager` 的封装，负责 VPN 配置的加载、保存、启动和停止。它使用单例模式，因为整个 App 只需要一个 VPN 管理器。

### 完整代码逐行解析

```swift
import Foundation
import NetworkExtension

class VPNManager {

    // 单例模式：全局只有一个 VPNManager 实例
    static let shared = VPNManager()

    // NETunnelProviderManager 是 Apple 提供的 VPN 配置管理器
    // 它管理 VPN 的配置（协议、服务器地址等）和连接状态
    private var manager: NETunnelProviderManager?

    // private init 防止外部创建实例
    private init() {}
```

### loadConfiguration：加载 VPN 配置

```swift
    func loadConfiguration(completion: @escaping (Error?) -> Void) {
        // loadAllFromPreferences 从系统偏好设置中加载所有已保存的 VPN 配置
        // 第一次运行时，managers 为空数组
        NETunnelProviderManager.loadAllFromPreferences { [weak self] managers, error in
            if let error = error {
                completion(error)
                return
            }
            // 如果已有配置，使用第一个；否则创建新的 manager
            self?.manager = managers?.first ?? NETunnelProviderManager()
            completion(nil)
        }
    }
```

**为什么要先 load 再用？**

`NETunnelProviderManager` 必须从系统偏好设置中加载。iOS 持久化保存 VPN 配置——即使 App 重启，之前保存的配置仍然存在。直接创建新的 `NETunnelProviderManager()` 而不 load 会导致保存失败。

### configureVPN：配置 VPN 参数

```swift
    func configureVPN(socks5Address: String, completion: @escaping (Error?) -> Void) {
        guard let manager = manager else {
            completion(NSError(domain: "VPNManager", code: -1,
                             userInfo: [NSLocalizedDescriptionKey: "Manager not loaded"]))
            return
        }

        // NETunnelProviderProtocol 描述了 VPN 隧道的协议信息
        let proto = NETunnelProviderProtocol()

        // providerBundleIdentifier: 指定哪个 Extension 来处理这个 VPN 隧道
        // 必须与 PacketTunnel target 的 Bundle Identifier 完全匹配！
        proto.providerBundleIdentifier = "com.vpndemo.app.VPNDemo.tunnel"

        // serverAddress: iOS 系统设置界面中显示的"服务器"标签
        // 这只是一个显示标签，不影响实际连接
        // 但这个字段是必填的，不设置会导致保存失败
        proto.serverAddress = "TUN2SOCKS Local Tunnel"

        // providerConfiguration: 传给 Extension 的自定义配置字典
        // 这是主 App 与 Extension 之间传递配置的方式之一
        proto.providerConfiguration = [
            "socks5_address": socks5Address   // 如 "192.168.31.209:1080"
        ]

        manager.protocolConfiguration = proto
        manager.localizedDescription = "VPN Demo"  // 系统 VPN 列表中显示的名称
        manager.isEnabled = true                    // 启用这个 VPN 配置

        // saveToPreferences: 保存配置到系统偏好设置
        // 第一次保存时，iOS 会弹出系统确认框："xxx 想要添加 VPN 配置"
        manager.saveToPreferences { error in
            if let error = error {
                completion(error)
                return
            }
            // 保存后必须重新 load，否则后续操作可能失败
            // 这是 Apple 文档明确要求的
            manager.loadFromPreferences { error in
                completion(error)
            }
        }
    }
```

**saveToPreferences + loadFromPreferences 的必要性**

这是 Apple API 的一个"坑"：`saveToPreferences` 后必须 `loadFromPreferences`，否则 manager 的内部状态与系统偏好不同步，后续调用 `startVPNTunnel()` 可能会失败。

### startVPN：启动 VPN 隧道

```swift
    func startVPN(socks5Address: String) throws {
        guard let manager = manager else {
            throw NSError(domain: "VPNManager", code: -1,
                         userInfo: [NSLocalizedDescriptionKey: "Manager not loaded"])
        }

        // connection 属性代表当前的 VPN 连接
        // 对于 NETunnelProviderManager，它的类型是 NETunnelProviderSession
        let session = manager.connection as! NETunnelProviderSession

        // startVPNTunnel(options:) 启动 VPN 隧道
        // options 字典会传递到 Extension 的 startTunnel(options:) 方法
        // 这是传递实时参数的方式（与 providerConfiguration 的区别见下文）
        try session.startVPNTunnel(options: [
            "socks5_address": socks5Address as NSObject
        ])
    }
```

**options vs providerConfiguration**

| | options | providerConfiguration |
|---|---|---|
| 传递时机 | 每次启动时 | 保存配置时 |
| 是否持久化 | 否（仅本次启动有效） | 是（保存在系统偏好中） |
| 适用场景 | 动态参数 | 固定配置 |
| Extension 获取方式 | `startTunnel(options:)` 参数 | `protocolConfiguration.providerConfiguration` |

本项目中两种方式都传递了 `socks5_address`，这是一种防御性编程——确保 Extension 不管通过哪种方式启动都能获取到地址。

### stopVPN：停止 VPN 隧道

```swift
    func stopVPN() {
        // stopVPNTunnel() 通知系统停止 VPN
        // 系统会调用 Extension 的 stopTunnel(with:completionHandler:)
        manager?.connection.stopVPNTunnel()
    }

    // 当前 VPN 连接状态
    var status: NEVPNStatus {
        return manager?.connection.status ?? .invalid
    }
```

`NEVPNStatus` 的所有状态：

| 状态 | 含义 |
|---|---|
| `.invalid` | 未初始化 |
| `.disconnected` | 已断开 |
| `.connecting` | 正在连接 |
| `.connected` | 已连接 |
| `.reasserting` | 正在重新连接 |
| `.disconnecting` | 正在断开 |

---

## 6.2 ViewController: 用户界面

`ViewController` 是 App 的主界面，提供 SOCKS5 地址输入、VPN 连接/断开控制、状态显示和日志输出。

### UI 布局

界面使用 `UIStackView` 垂直排列组件，纯代码布局（无 Storyboard）：

```
┌─────────────────────────────┐
│                             │
│       iOS VPN Demo          │  ← titleLabel (24pt 加粗)
│                             │
│  ┌───────────────────────┐  │
│  │ 192.168.31.209:1080   │  │  ← addressField (SOCKS5 地址输入)
│  └───────────────────────┘  │
│                             │
│  🔴 Disconnected            │  ← statusDot + statusLabel
│                             │
│      Connect VPN            │  ← connectButton
│                             │
│      Test SOCKS5            │  ← testButton
│                             │
│  Loading VPN config...      │  ← logLabel (最多显示5行日志)
│  VPN config loaded          │
│                             │
└─────────────────────────────┘
```

### UI 组件创建

```swift
private let addressField = UITextField()
private let connectButton = UIButton(type: .system)
private let statusLabel = UILabel()
private let statusDot = UIView()     // 一个 12x12 的圆形 View，用颜色表示状态
private let logLabel = UILabel()

private func setupUI() {
    view.backgroundColor = .systemBackground

    // 标题
    let titleLabel = UILabel()
    titleLabel.text = "iOS VPN Demo"
    titleLabel.font = .systemFont(ofSize: 24, weight: .bold)

    // SOCKS5 地址输入框
    addressField.placeholder = "SOCKS5 address (e.g. 1.2.3.4:1080)"
    addressField.text = "192.168.31.209:1080"     // 默认值，方便调试
    addressField.borderStyle = .roundedRect
    addressField.keyboardType = .numbersAndPunctuation

    // 状态指示器：12x12 的圆形 View
    statusDot.backgroundColor = .systemRed         // 初始红色=断开
    statusDot.layer.cornerRadius = 6               // 宽高12/2=6，变成圆形

    // 连接按钮
    connectButton.setTitle("Connect VPN", for: .normal)
    connectButton.addTarget(self, action: #selector(toggleVPN), for: .touchUpInside)

    // 测试按钮：直接测试 SOCKS5 连通性（不需要启动 VPN）
    let testButton = UIButton(type: .system)
    testButton.setTitle("Test SOCKS5", for: .normal)
    testButton.addTarget(self, action: #selector(testSocks5), for: .touchUpInside)

    // 日志标签：等宽字体，最多5行
    logLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
    logLabel.numberOfLines = 0

    // 使用 UIStackView 垂直排列
    let stack = UIStackView(arrangedSubviews: [
        titleLabel, addressField, statusStack, connectButton, testButton, logLabel
    ])
    stack.axis = .vertical
    stack.spacing = 16
    stack.alignment = .center
    // ... Auto Layout 约束
}
```

### VPN 状态观察：NEVPNStatusDidChange

iOS 通过 `NotificationCenter` 广播 VPN 状态变化。这是标准的观察者模式：

```swift
private func observeVPNStatus() {
    // 注册通知观察者
    // NEVPNStatusDidChange 在 VPN 状态变化时触发
    NotificationCenter.default.addObserver(
        self,
        selector: #selector(vpnStatusChanged),
        name: .NEVPNStatusDidChange,   // 系统预定义的通知名
        object: nil                     // nil = 接收所有 VPN 连接的状态变化
    )
}

@objc private func vpnStatusChanged() {
    DispatchQueue.main.async { [weak self] in
        guard let self = self else { return }
        let status = VPNManager.shared.status

        switch status {
        case .connected:
            self.statusDot.backgroundColor = .systemGreen   // 绿色 = 已连接
            self.statusLabel.text = "Connected"
            self.connectButton.setTitle("Disconnect", for: .normal)
            self.addressField.isEnabled = false              // 连接中禁用输入

        case .connecting:
            self.statusDot.backgroundColor = .systemOrange   // 橙色 = 连接中
            self.statusLabel.text = "Connecting..."

        case .disconnecting:
            self.statusDot.backgroundColor = .systemOrange   // 橙色 = 断开中
            self.statusLabel.text = "Disconnecting..."

        default:  // .disconnected, .invalid, .reasserting
            self.statusDot.backgroundColor = .systemRed      // 红色 = 断开
            self.statusLabel.text = "Disconnected"
            self.connectButton.setTitle("Connect VPN", for: .normal)
            self.addressField.isEnabled = true
        }
    }
}
```

**注意 `DispatchQueue.main.async`**：`NEVPNStatusDidChange` 通知可能在后台线程触发，但 UI 更新必须在主线程。

### toggleVPN：连接/断开的完整流程

```swift
@objc private func toggleVPN() {
    if isConnected {
        // 已连接 → 断开
        appendLog("Stopping VPN...")
        VPNManager.shared.stopVPN()
    } else {
        // 未连接 → 连接
        let address = addressField.text ?? "192.168.31.209:1080"
        appendLog("Configuring VPN with \(address)...")

        // 第一步：配置并保存 VPN（异步）
        VPNManager.shared.configureVPN(socks5Address: address) { [weak self] error in
            if let error = error {
                self?.appendLog("Config error: \(error.localizedDescription)")
                return
            }
            // 第二步：保存成功后，启动 VPN 隧道
            self?.appendLog("Config saved, starting tunnel...")
            do {
                try VPNManager.shared.startVPN(socks5Address: address)
                self?.appendLog("startVPNTunnel called OK")
            } catch {
                self?.appendLog("Start error: \(error.localizedDescription)")
            }
        }
    }
}
```

连接流程是嵌套回调式的：`configureVPN` 完成后才能 `startVPN`。这是因为 `saveToPreferences` 和 `loadFromPreferences` 都是异步的。

### testSocks5：直接测试 SOCKS5 连通性

```swift
@objc private func testSocks5() {
    let address = addressField.text ?? ""
    appendLog("Testing SOCKS5: \(address)...")

    // 在后台线程调用 Rust FFI（因为 tunnel_test_socks5 内部会阻塞等待网络响应）
    DispatchQueue.global(qos: .userInitiated).async { [weak self] in
        var buf = [UInt8](repeating: 0, count: 2048)
        let result = address.withCString { ptr in
            // 调用 Rust FFI 函数
            // ptr: SOCKS5 地址的 C 字符串指针
            // &buf: 输出缓冲区
            // buf.count: 缓冲区大小
            // 返回值: 1=成功, 0=失败
            tunnel_test_socks5(ptr, &buf, buf.count)
        }
        let msg = String(cString: buf.map { CChar(bitPattern: $0) })

        // 回到主线程更新 UI
        DispatchQueue.main.async {
            self?.appendLog("Test result(\(result)): \(msg)")
        }
    }
}
```

这个测试功能不需要启动 VPN 隧道，直接在主 App 进程中通过 Rust FFI 测试 SOCKS5 服务器的连通性。这对调试非常有用——可以先确认 SOCKS5 服务器可达，再启动完整的 VPN 隧道。

### 日志系统

```swift
private func appendLog(_ msg: String) {
    DispatchQueue.main.async {
        let current = self.logLabel.text ?? ""
        // 只保留最近5行日志，避免界面过长
        let lines = current.split(separator: "\n").suffix(5)
        self.logLabel.text = (lines + [Substring(msg)]).joined(separator: "\n")
    }
    // 同时输出到 Xcode 控制台
    print("[VPNDemo] \(msg)")
}
```

---

## 6.3 PacketTunnelProvider: Extension 核心逻辑

`PacketTunnelProvider` 是整个 VPN 的核心，运行在独立的 Extension 进程中。它负责：
1. 启动 Rust tunnel-core
2. 配置 TUN 网卡
3. 驱动数据包的读写循环

### startTunnel 完整流程

```swift
class PacketTunnelProvider: NEPacketTunnelProvider {

    private let log = OSLog(subsystem: "com.vpndemo.app.tunnel", category: "tunnel")
    private var readTimer: DispatchSourceTimer?   // 用于轮询 Rust 输出包的定时器
    private var packetCount: UInt64 = 0           // 入站包计数（调试用）
    private var outPacketCount: UInt64 = 0        // 出站包计数（调试用）

    override func startTunnel(
        options: [String: NSObject]?,
        completionHandler: @escaping (Error?) -> Void
    ) {
        os_log("=== PacketTunnelProvider startTunnel ===", log: log, type: .info)

        // === 第一步：获取 SOCKS5 地址 ===
        // 优先从 options 获取（startVPNTunnel 传入的实时参数）
        // 其次从 protocolConfiguration 获取（持久化的配置）
        let socks5Address: String
        if let addr = options?["socks5_address"] as? String {
            socks5Address = addr
            os_log("SOCKS5 from options: %{public}@", log: log, type: .info, socks5Address)
        } else if let proto = protocolConfiguration as? NETunnelProviderProtocol,
                  let addr = proto.providerConfiguration?["socks5_address"] as? String {
            socks5Address = addr
            os_log("SOCKS5 from config: %{public}@", log: log, type: .info, socks5Address)
        } else {
            os_log("ERROR: No SOCKS5 address found!", log: log, type: .error)
            completionHandler(NSError(domain: "PacketTunnel", code: -1,
                                     userInfo: [NSLocalizedDescriptionKey: "No SOCKS5 address"]))
            return  // 没有地址，隧道无法启动
        }
```

为什么两种方式都要尝试？

- 用户手动启动：`options` 中有地址
- iOS 系统自动重连（如网络切换后）：`options` 可能为 `nil`，需要从 `protocolConfiguration` 读取

```swift
        // === 第二步：启动 Rust tunnel-core ===
        os_log("Calling tunnel_start...", log: log, type: .info)
        let started = socks5Address.withCString { ptr in
            tunnel_start(ptr)   // 调用 Rust FFI: 创建 tokio runtime, 启动 netstack
        }

        if !started {
            os_log("ERROR: tunnel_start returned false!", log: log, type: .error)
            completionHandler(NSError(domain: "PacketTunnel", code: -2,
                                     userInfo: [NSLocalizedDescriptionKey: "Failed to start Rust tunnel"]))
            return
        }
        os_log("tunnel_start succeeded", log: log, type: .info)
```

`tunnel_start` 是同步调用，但内部创建了 tokio runtime 和多个异步 task。返回 `true` 表示初始化成功，此后 Rust 侧的异步 task 在后台持续运行。

```swift
        // === 第三步：配置 TUN 网卡参数 ===
        let settings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "10.0.0.1")

        // IPv4 配置：TUN 网卡的 IP 地址
        let ipv4 = NEIPv4Settings(
            addresses: ["10.0.0.2"],          // TUN 网卡 IP
            subnetMasks: ["255.255.255.0"]     // /24 子网
        )
        ipv4.includedRoutes = [NEIPv4Route.default()]  // 0.0.0.0/0 = 所有流量走 TUN

        // 排除 SOCKS5 服务器 IP，避免路由回环
        if let host = socks5Address.split(separator: ":").first {
            let excludeRoute = NEIPv4Route(
                destinationAddress: String(host),
                subnetMask: "255.255.255.255"  // /32 精确匹配
            )
            ipv4.excludedRoutes = [excludeRoute]
        }
        settings.ipv4Settings = ipv4

        // DNS 配置：使用 Google Public DNS
        settings.dnsSettings = NEDNSSettings(servers: ["8.8.8.8", "8.8.4.4"])
        settings.mtu = 1500   // 标准 MTU

        // === 第四步：应用网卡配置 ===
        os_log("Applying tunnel network settings...", log: log, type: .info)
        setTunnelNetworkSettings(settings) { [weak self] error in
            guard let self = self else { return }
            if let error = error {
                os_log("ERROR setting tunnel: %{public}@",
                       log: self.log, type: .error, error.localizedDescription)
                completionHandler(error)
                return
            }

            os_log("Tunnel settings applied OK, starting packet flow",
                   log: self.log, type: .info)

            // === 第五步：启动数据包读写循环 ===
            self.startReadingPackets()    // 从 TUN 读包 → 喂给 Rust
            self.startWritingPackets()    // 从 Rust 读包 → 写入 TUN

            // 告诉系统隧道建立成功
            completionHandler(nil)
        }
    }
```

### stopTunnel

```swift
    override func stopTunnel(
        with reason: NEProviderStopReason,
        completionHandler: @escaping () -> Void
    ) {
        os_log("stopTunnel reason: %d", log: log, type: .info, reason.rawValue)

        // 停止轮询定时器
        readTimer?.cancel()
        readTimer = nil

        // 通知 Rust 停止
        tunnel_stop()   // 设置 running = false，各 task 逐步退出

        completionHandler()
    }
```

---

## 6.4 数据包读写循环

数据包的读写是 VPN 运行时的核心循环，驱动所有流量在 Swift 和 Rust 之间流转。

### startReadingPackets：TUN → Rust（递归回调模式）

```swift
private func startReadingPackets() {
    // readPackets 是异步回调式 API
    // 当 TUN 网卡有数据包可读时，回调被调用
    packetFlow.readPackets { [weak self] packets, protocols in
        guard let self = self else { return }

        // packets: [Data] — 可能一次返回多个包
        // protocols: [NSNumber] — 每个包的协议族 (AF_INET=2, AF_INET6=30)
        for (i, packet) in packets.enumerated() {
            self.packetCount += 1

            // 仅前5个包和每100个包打印日志（避免日志洪水）
            if self.packetCount <= 5 || self.packetCount % 100 == 0 {
                os_log("IN packet #%llu: %lu bytes, proto=%{public}@",
                       log: self.log, type: .debug,
                       self.packetCount, packet.count, protocols[i].stringValue)
            }

            // 将 IP 数据包喂给 Rust tunnel-core
            packet.withUnsafeBytes { ptr in
                guard let baseAddr = ptr.baseAddress else { return }
                let ok = tunnel_feed_packet(
                    baseAddr.assumingMemoryBound(to: UInt8.self),
                    ptr.count
                )
                if !ok && self.packetCount <= 5 {
                    os_log("tunnel_feed_packet FAILED for packet #%llu",
                           log: self.log, type: .error, self.packetCount)
                }
            }
        }

        // 关键：递归调用自己，形成持续的读取循环
        // 这不是真正的递归（不会栈溢出），因为 readPackets 是异步的
        // 当前回调结束后，下一次回调在新的事件循环中触发
        self.startReadingPackets()
    }
}
```

**为什么是递归模式而不是循环？**

`packetFlow.readPackets` 是基于回调的异步 API，不是 `while` 循环可以驱动的。Apple 选择这种设计是因为 Network Extension 框架需要与 iOS 内核的数据包队列配合——当没有数据包时，回调不会触发，避免忙等待。

```
readPackets ──回调──► 处理数据包 ──► readPackets ──回调──► 处理数据包 ──► ...
            (等待)                              (等待)
```

### startWritingPackets：Rust → TUN（定时器轮询模式）

```swift
private func startWritingPackets() {
    // 使用 GCD 定时器，每 1 毫秒轮询一次
    let timer = DispatchSource.makeTimerSource(
        queue: DispatchQueue.global(qos: .userInteractive)  // 最高优先级队列
    )
    timer.schedule(deadline: .now(), repeating: .milliseconds(1))

    timer.setEventHandler { [weak self] in
        guard let self = self else { return }

        // 分配 64KB 缓冲区（最大 IP 包大小）
        var buf = [UInt8](repeating: 0, count: 65535)

        // tunnel_read_packet: 非阻塞调用
        // 有包时返回包大小 (>0)，无包时返回 0
        let len = tunnel_read_packet(&buf, buf.count)

        if len > 0 {
            self.outPacketCount += 1
            if self.outPacketCount <= 5 || self.outPacketCount % 100 == 0 {
                os_log("OUT packet #%llu: %lu bytes",
                       log: self.log, type: .debug, self.outPacketCount, len)
            }

            let packetData = Data(bytes: buf, count: len)
            // 写入 TUN 网卡，iOS 内核会将这个包路由给目标 App
            // AF_INET (=2) 表示 IPv4 包
            self.packetFlow.writePackets(
                [packetData],
                withProtocols: [NSNumber(value: AF_INET)]
            )
        }
    }

    timer.resume()
    readTimer = timer   // 保存引用，stopTunnel 时取消
}
```

**为什么用 1ms 定时器轮询？**

这是一个工程上的权衡：

| 方案 | 优点 | 缺点 |
|---|---|---|
| 1ms 定时器轮询 | 实现简单，延迟低 | 空闲时浪费 CPU |
| 更长间隔（10ms+） | 省 CPU | 延迟高，影响网速 |
| 回调/通知机制 | 最优 | FFI 跨语言回调复杂 |

1ms 是一个实用的折中：延迟足够低（人类感知不到），CPU 消耗在可接受范围内。在真正的生产项目中，可以考虑用更复杂的 FFI 回调机制替代轮询。

### 读写循环的整体数据流

```
┌─────────────────────── Swift (Extension 进程) ───────────────────────┐
│                                                                       │
│  packetFlow.readPackets()                                            │
│       │ (iOS 发出的 IP 包)                                             │
│       ▼                                                               │
│  tunnel_feed_packet() ──────► Rust inbound channel                    │
│                                     │                                 │
│                                     ▼                                 │
│                              netstack-smoltcp                         │
│                              (IP→TCP/UDP 拆包)                        │
│                                     │                                 │
│                              ┌──────┴──────┐                          │
│                              │             │                          │
│                              ▼             ▼                          │
│                          TCP 流        UDP 包                         │
│                              │             │                          │
│                              ▼             ▼                          │
│                      handle_tcp_       forward_udp()                  │
│                      via_socks5()          │                          │
│                              │             │                          │
│                              ▼             ▼                          │
│                      SOCKS5 代理     直接转发到                        │
│                       服务器          目标地址                          │
│                              │             │                          │
│                              ▼             ▼                          │
│                         响应数据       响应数据                         │
│                              │             │                          │
│                              ▼             ▼                          │
│                              netstack-smoltcp                         │
│                              (TCP/UDP→IP 封包)                        │
│                                     │                                 │
│                                     ▼                                 │
│  tunnel_read_packet() ◄───── Rust outbound channel                    │
│       │ (响应 IP 包)                                                   │
│       ▼                                                               │
│  packetFlow.writePackets()                                            │
│       │                                                               │
│       ▼                                                               │
│  TUN 网卡 → iOS 内核 → App 收到响应                                    │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

---

## 6.5 Swift 与 Rust 的调用时序

从用户点击"Connect VPN"到数据开始流转的完整时序：

```
用户界面              主 App 进程               iOS 系统              Extension 进程            Rust tunnel-core
  │                     │                        │                      │                        │
  │  点击 Connect       │                        │                      │                        │
  ├────────────────────►│                        │                      │                        │
  │                     │                        │                      │                        │
  │                     │ configureVPN()         │                      │                        │
  │                     ├───────────────────────►│                      │                        │
  │                     │  saveToPreferences     │                      │                        │
  │                     │                        │ (首次弹出确认框)      │                        │
  │                     │◄───────────────────────┤                      │                        │
  │                     │  loadFromPreferences   │                      │                        │
  │                     ├───────────────────────►│                      │                        │
  │                     │◄───────────────────────┤                      │                        │
  │                     │                        │                      │                        │
  │                     │ startVPNTunnel()       │                      │                        │
  │                     ├───────────────────────►│                      │                        │
  │                     │                        │  启动 Extension 进程  │                        │
  │                     │                        ├─────────────────────►│                        │
  │                     │                        │                      │                        │
  │                     │                        │ startTunnel(options)  │                        │
  │                     │                        ├─────────────────────►│                        │
  │                     │                        │                      │                        │
  │                     │                        │                      │  tunnel_start()        │
  │                     │                        │                      ├───────────────────────►│
  │                     │                        │                      │  创建 tokio runtime     │
  │                     │                        │                      │  启动 netstack          │
  │                     │                        │                      │  启动 TCP/UDP 监听      │
  │                     │                        │                      │◄───────────────────────┤
  │                     │                        │                      │  return true            │
  │                     │                        │                      │                        │
  │                     │                        │                      │ setTunnelNetworkSettings│
  │                     │                        │                      ├───────────────────────►│
  │                     │                        │  配置 TUN 网卡        │                (系统)  │
  │                     │                        │◄─────────────────────┤                        │
  │                     │                        │                      │                        │
  │                     │                        │                      │ startReadingPackets()  │
  │                     │                        │                      │ startWritingPackets()  │
  │                     │                        │                      │                        │
  │                     │                        │                      │ completionHandler(nil) │
  │                     │                        │                      ├───────────────────────►│
  │                     │                        │                      │                 (系统)  │
  │                     │                        │ 状态变为 .connected    │                        │
  │                     │  NEVPNStatusDidChange  │                      │                        │
  │                     │◄───────────────────────┤                      │                        │
  │  UI 更新为绿色      │                        │                      │                        │
  │◄────────────────────┤                        │                      │                        │
  │                     │                        │                      │                        │
  │  ═══════════════════ 隧道运行中，数据开始流转 ═══════════════════════│                        │
  │                     │                        │                      │                        │
  │  App 发起 HTTP 请求  │                        │                      │                        │
  │                     │                        │  IP 包路由到 TUN      │                        │
  │                     │                        ├─────────────────────►│                        │
  │                     │                        │                      │  readPackets 回调      │
  │                     │                        │                      │  tunnel_feed_packet()  │
  │                     │                        │                      ├───────────────────────►│
  │                     │                        │                      │    netstack 解析       │
  │                     │                        │                      │    SOCKS5 转发         │
  │                     │                        │                      │    收到响应             │
  │                     │                        │                      │◄───────────────────────┤
  │                     │                        │                      │  tunnel_read_packet()  │
  │                     │                        │                      │  writePackets          │
  │                     │                        │  IP 包写入 TUN        │                        │
  │                     │                        │◄─────────────────────┤                        │
  │  App 收到响应        │                        │                      │                        │
  │                     │                        │                      │                        │
```

---

## 6.6 日志系统

调试 Network Extension 是 iOS 开发中最困难的部分之一，因为 Extension 运行在独立进程中，Xcode 的控制台默认不显示 Extension 的输出。良好的日志系统至关重要。

### Swift 端：os_log

Apple 推荐使用 `os_log`（统一日志系统），而不是 `print()` 或 `NSLog()`。

```swift
import os.log

class PacketTunnelProvider: NEPacketTunnelProvider {
    // 创建日志对象
    // subsystem: 通常使用 Bundle Identifier
    // category: 用于过滤的分类标签
    private let log = OSLog(subsystem: "com.vpndemo.app.tunnel", category: "tunnel")

    func someMethod() {
        // 基本用法
        os_log("Simple message", log: log, type: .info)

        // 带参数（%{public}@ 表示公开显示，默认是 <private>）
        os_log("SOCKS5 address: %{public}@", log: log, type: .info, address)

        // 日志级别
        os_log("Debug info", log: log, type: .debug)    // 调试（默认不持久化）
        os_log("Normal info", log: log, type: .info)     // 普通信息
        os_log("Error!", log: log, type: .error)         // 错误（始终持久化）
    }
}
```

`os_log` 的优势：
- **性能极高**：编译时优化，比 `NSLog` 快 10 倍以上
- **隐私保护**：默认对参数脱敏，需要 `%{public}` 才显示明文
- **系统集成**：可通过 Console.app 或 `log` 命令查看
- **持久化**：error 级别的日志会写入磁盘，重启后仍可查看

### Rust 端：oslog crate

Rust 端使用 `oslog` crate 桥接到 Apple 的统一日志系统：

```rust
// 在 tunnel_start 中初始化
let _ = oslog::OsLogger::new("com.vpndemo.app.tunnel.rust")
    .level_filter(log::LevelFilter::Debug)
    .init();

// 之后通过标准的 log crate 宏输出
use log::{info, warn, error};

info!("tunnel_start called");                          // 普通信息
warn!("Stack send error: {}", e);                      // 警告
error!("socks5_addr is null");                         // 错误
```

`Cargo.toml` 中的依赖：

```toml
log = "0.4"      # Rust 标准日志接口（facade）
oslog = "0.2"    # Apple os_log 的 Rust 绑定（backend）
```

`log` crate 是 Rust 的标准日志接口，`oslog` 是它的后端实现。这种分离意味着你的代码使用 `info!`/`warn!`/`error!` 宏，不需要关心日志输出到哪里——在 iOS 上输出到 os_log，在 Linux 上可以换成 `env_logger` 输出到终端。

### 查看 Extension 日志的方法

由于 Extension 运行在独立进程中，有以下几种查看日志的方式：

#### 方法一：macOS Console.app

1. 将 iPhone 连接到 Mac
2. 打开 Console.app（应用程序 → 实用工具 → 控制台）
3. 左侧选择你的设备
4. 搜索栏输入 `com.vpndemo.app.tunnel` 过滤

#### 方法二：命令行 idevicesyslog

```bash
# 安装 libimobiledevice
brew install libimobiledevice

# 查看实时日志（过滤关键字）
idevicesyslog | grep -E "tunnel|vpndemo|PacketTunnel"
```

#### 方法三：xcrun devicectl

```bash
# iOS 17+ 新工具
xcrun devicectl device info log --device <UDID> \
    --predicate 'subsystem == "com.vpndemo.app.tunnel"'
```

#### 方法四：Xcode 附加到 Extension 进程

1. 启动 VPN 隧道
2. Xcode 菜单 → Debug → Attach to Process
3. 选择 `PacketTunnel`（或你的 Extension target 名）
4. 此后 Extension 的 `os_log` 输出会出现在 Xcode 控制台

### 日志的最佳实践

```swift
// 好的做法：条件性日志，避免高频输出
if self.packetCount <= 5 || self.packetCount % 100 == 0 {
    os_log("IN packet #%llu: %lu bytes", log: self.log, type: .debug,
           self.packetCount, packet.count)
}

// 坏的做法：每个包都打印（会严重影响性能）
// os_log("IN packet: %lu bytes", log: self.log, type: .debug, packet.count)
```

在数据包处理路径上，日志是有性能开销的。本项目中使用了"前 5 个包 + 每 100 个包"的策略，在调试初期能看到足够信息，同时不影响稳态性能。
