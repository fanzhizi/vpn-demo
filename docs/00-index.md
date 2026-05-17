# iOS TUN2SOCKS VPN 开发实战

> 使用 Rust + Swift + netstack-smoltcp 从零实现 iOS 翻墙客户端

---

## 目录

### [第一章：iOS Network Extension 基础](./01-network-extension.md)
- 1.1 什么是 Network Extension
- 1.2 NEPacketTunnelProvider 生命周期
- 1.3 TUN 虚拟网卡的工作原理
- 1.4 iOS 如何路由流量到 TUN
- 1.5 Extension 与主 App 的进程隔离

### [第二章：项目结构与构建系统](./02-project-structure.md)
- 2.1 整体架构概览
- 2.2 Cargo Workspace 结构
- 2.3 Xcode 项目配置
- 2.4 Rust 静态库如何链接到 Swift
- 2.5 构建脚本与交叉编译

### [第三章：Rust FFI 桥接层](./03-rust-ffi.md)
- 3.1 FFI 设计原则
- 3.2 C 头文件定义
- 3.3 tunnel_start：初始化运行时和网络栈
- 3.4 tunnel_feed_packet / tunnel_read_packet：数据包交换
- 3.5 内存安全与线程模型

### [第四章：netstack-smoltcp 与数据包流转](./04-netstack.md)
- 4.1 为什么不直接用 smoltcp
- 4.2 netstack-smoltcp 的核心抽象
- 4.3 Stack：Sink + Stream 双向通道
- 4.4 TcpListener：从 IP 包到 TCP 流
- 4.5 UdpSocket：UDP 数据报处理
- 4.6 完整的数据包流转路径

### [第五章：SOCKS5 转发与数据中继](./05-socks5-relay.md)
- 5.1 SOCKS5 协议简介
- 5.2 TCP 双向中继（copy_bidirectional）
- 5.3 UDP/DNS 直接转发
- 5.4 路由回环问题与解决
- 5.5 扩展：替换为其他协议

### [第六章：Swift 端 — UI 与 VPN 管理](./06-swift-side.md)
- 6.1 VPNManager：配置与控制 VPN 隧道
- 6.2 ViewController：用户界面
- 6.3 PacketTunnelProvider：Extension 核心逻辑
- 6.4 数据包读写循环
- 6.5 Swift 与 Rust 的调用时序

### [第七章：构建、调试与部署](./07-build-deploy.md)
- 7.1 环境准备
- 7.2 从零开始的完整构建流程
- 7.3 常见编译错误与解决
- 7.4 真机调试技巧
- 7.5 签名与 Entitlements
- 7.6 Release 构建与优化

---

## 本书适用读者

- 想了解 iOS VPN/代理客户端实现原理的开发者
- 有 Rust 基础，想学习 iOS Network Extension 开发的程序员
- 有 iOS 开发经验，想了解如何集成 Rust 原生库的开发者

## 技术栈

| 技术 | 用途 |
|------|------|
| Rust | 核心网络逻辑（协议栈、SOCKS5、数据中继） |
| Swift | iOS UI、VPN 配置管理、PacketTunnelProvider |
| netstack-smoltcp | 用户态 TCP/IP 协议栈（IP 包 → TCP 流） |
| fast-socks5 | SOCKS5 客户端库 |
| tokio | Rust 异步运行时 |
| NetworkExtension.framework | iOS VPN API |

## 项目完整数据流

```
┌──────────────────────────────────────────────────────────────┐
│                        iPhone                                 │
│                                                               │
│  Safari/App 发起网络请求                                        │
│       │                                                       │
│       ▼                                                       │
│  ┌─────────┐    iOS 内核将所有 IP 包                            │
│  │   TUN   │    路由到虚拟网卡                                   │
│  │  虚拟网卡 │                                                  │
│  └────┬────┘                                                  │
│       │ 原始 IP 数据包                                          │
│       ▼                                                       │
│  ┌─────────────────────────────────────────────┐              │
│  │     PacketTunnelProvider (Swift)             │              │
│  │     packetFlow.readPackets()                 │              │
│  │          │                                   │              │
│  │          │ tunnel_feed_packet() ← FFI 调用    │              │
│  │          ▼                                   │              │
│  │  ┌──────────────────────────────────┐        │              │
│  │  │      tunnel-core (Rust)          │        │              │
│  │  │                                  │        │              │
│  │  │  mpsc::channel → Stack.send()    │        │              │
│  │  │          │                       │        │              │
│  │  │          ▼                       │        │              │
│  │  │  ┌────────────────┐              │        │              │
│  │  │  │ netstack-smoltcp│              │        │              │
│  │  │  │                │              │        │              │
│  │  │  │ IP包 → TCP流    │              │        │              │
│  │  │  │ IP包 → UDP报文  │              │        │              │
│  │  │  └───┬───────┬───┘              │        │              │
│  │  │      │       │                  │        │              │
│  │  │   TCP流    UDP报文               │        │              │
│  │  │      │       │                  │        │              │
│  │  │      ▼       ▼                  │        │              │
│  │  │  SOCKS5   直接转发               │        │              │
│  │  │  代理连接  (DNS等)               │        │              │
│  │  └──────┼───────┼──────────────────┘        │              │
│  │         │       │                           │              │
│  │         ▼       ▼                           │              │
│  │    Stack.next() → tunnel_read_packet()      │              │
│  │         │                                   │              │
│  │         ▼                                   │              │
│  │    packetFlow.writePackets()                 │              │
│  └─────────────────────────────────────────────┘              │
│       │                                                       │
│       ▼                                                       │
│  ┌─────────┐                                                  │
│  │   TUN   │ 响应包写回 TUN                                     │
│  └────┬────┘                                                  │
│       │                                                       │
│       ▼                                                       │
│  Safari/App 收到响应                                            │
└──────────────────────────────────────────────────────────────┘
         ║                              ║
         ║  TCP (经 SOCKS5 代理)          ║  UDP (直连)
         ▼                              ▼
   ┌───────────┐                  ┌───────────┐
   │  SOCKS5   │                  │  8.8.8.8  │
   │  代理服务器 │                  │  DNS 服务器 │
   └─────┬─────┘                  └───────────┘
         │
         ▼
   ┌───────────┐
   │  目标网站   │
   │  (Google等) │
   └───────────┘
```
