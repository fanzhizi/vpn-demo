# iOS VPN Demo (Rust + Slint + smoltcp)

一个用于验证 iOS 翻墙原理的 demo 项目。通过 NEPacketTunnelProvider 获取系统网络数据包，
使用 smoltcp 用户态 TCP/IP 协议栈解析，然后通过 SOCKS5 代理转发。

## 架构

```
┌─────────────────────────────────────────────────┐
│                   iOS App                        │
│   ┌─────────────────────────────────────────┐   │
│   │         Slint UI (Rust)                  │   │
│   │   [SOCKS5 Address Input] [Connect Btn]  │   │
│   └─────────────────────────────────────────┘   │
│                      │                           │
│         NETunnelProviderManager                  │
└─────────────────────────────────────────────────┘
                       │ (IPC)
┌─────────────────────────────────────────────────┐
│          Network Extension Process               │
│   ┌─────────────────────────────────────────┐   │
│   │   NEPacketTunnelProvider (Swift)         │   │
│   │     ┌───────────────────────────────┐   │   │
│   │     │    tunnel-core (Rust FFI)      │   │   │
│   │     │                               │   │   │
│   │     │  ┌─────────┐   ┌──────────┐  │   │   │
│   │     │  │ smoltcp  │──▶│  SOCKS5  │  │   │   │
│   │     │  │ (TCP/IP) │   │ Forward  │  │   │   │
│   │     │  └─────────┘   └──────────┘  │   │   │
│   │     └───────────────────────────────┘   │   │
│   └─────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
                       │
                       ▼
              SOCKS5 Proxy Server
```

## 工作流程

1. 用户在 UI 中输入 SOCKS5 代理服务器地址并点击连接
2. App 通过 `NETunnelProviderManager` 配置并启动 VPN
3. iOS 系统创建虚拟 TUN 网卡，所有网络流量通过此网卡
4. `NEPacketTunnelProvider` 从 TUN 设备读取原始 IP 数据包
5. 数据包通过 FFI 传入 Rust `tunnel-core`
6. `smoltcp` 解析 IP 包，重建 TCP 连接
7. TCP 数据通过 SOCKS5 协议转发到代理服务器
8. 代理服务器返回的数据经过 smoltcp 重新封装为 IP 包
9. 封装后的 IP 包写回 TUN 设备，完成代理

## 构建

### 前置条件

```bash
# 安装 iOS 编译目标
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim

# 确保有 Xcode 和 Apple Developer 账号（需要 Network Extension entitlement）
```

### 编译 Rust 库

```bash
./build-ios.sh
```

### Xcode 项目配置

1. 创建新的 Xcode 项目，添加两个 target：
   - **VPNDemo** (iOS App)
   - **PacketTunnel** (Network Extension - Packet Tunnel)

2. 将 `target/aarch64-apple-ios/release/libtunnel_core.a` 添加到 PacketTunnel target

3. 配置 Build Settings：
   - Header Search Paths: 添加 `$(PROJECT_DIR)/../tunnel-core/include`
   - Bridging Header: `PacketTunnel/BridgingHeader.h`
   - Other Linker Flags: `-lresolv`（tokio 需要）

4. 配置 Capabilities：
   - 两个 target 都需要 "Network Extensions" → "Packet Tunnel"
   - App Groups（用于主 app 和 extension 通信）

5. 需要 Apple Developer 账号申请 Network Extension entitlement

## 项目结构

```
vpn-demo/
├── Cargo.toml              # Workspace
├── tunnel-core/            # Rust 核心库 (smoltcp + SOCKS5 + FFI)
│   ├── Cargo.toml
│   ├── include/
│   │   └── tunnel_core.h  # C FFI 头文件
│   └── src/
│       ├── lib.rs          # 库入口
│       ├── stack.rs        # smoltcp 网络栈
│       ├── socks5.rs       # SOCKS5 连接器
│       └── ffi.rs          # C FFI 导出
├── vpn-app/                # Slint UI 应用
│   ├── Cargo.toml
│   ├── ui/
│   │   └── main.slint     # UI 定义
│   └── src/
│       └── main.rs
├── ios/                    # iOS 原生代码
│   ├── VPNDemo/            # 主 App target
│   │   ├── AppDelegate.swift
│   │   ├── ViewController.swift
│   │   ├── VPNManager.swift
│   │   └── Info.plist
│   └── PacketTunnel/       # Network Extension target
│       ├── PacketTunnelProvider.swift
│       ├── BridgingHeader.h
│       ├── Info.plist
│       └── PacketTunnel.entitlements
├── build-ios.sh            # iOS 构建脚本
└── README.md
```

## 注意事项

- Network Extension 需要 Apple Developer Program 会员资格
- 需要向 Apple 申请 `com.apple.developer.networking.networkextension` entitlement
- 模拟器上无法测试 VPN 功能，必须使用真机
- SOCKS5 服务器地址不能是 localhost（因为网络走的是 tunnel）
- 首次连接时系统会弹出 VPN 权限确认弹窗
