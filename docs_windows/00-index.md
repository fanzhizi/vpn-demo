# Windows VPN 开发实战

> 使用 Rust + wintun.dll + netstack-smoltcp + SOCKS5 从零实现 Windows VPN 客户端

---

## 目录

| 章节 | 标题 | 核心内容 |
|------|------|----------|
| [第一章](01-windows-vpn-architecture.md) | Windows VPN 架构基础 | wintun.dll 介绍、TUN 生命周期、与 iOS/Android 对比 |
| [第二章](02-project-structure.md) | 项目结构 | Cargo.toml 依赖、共享 UI、交叉编译配置 |
| [第三章](03-win32-api.md) | Win32 API 网络管理 | GetBestRoute2、IP 配置、路由管理、DNS 注册表 |
| [第四章](04-socket-bind.md) | Socket 绑定机制 | 路由绕过原理、socket bind、与 Android/iOS 对比 |
| [第五章](05-data-flow.md) | 完整数据流 | 从点击到网页加载的全链路追踪 |
| [第六章](06-build-deploy.md) | 构建与部署 | 环境准备、CI 配置、NSIS 安装包 |
| [第七章](07-platform-comparison.md) | 三平台对比总结 | iOS/Android/Windows 架构、API、代码共享对比 |

---

## 项目概述

```
┌─────────────────────────────────────────────────────────────────┐
│                     Windows VPN Demo                            │
│                                                                 │
│  ┌──────────────┐    ┌──────────────────┐    ┌──────────────┐  │
│  │   Slint UI   │───▶│     main.rs      │───▶│   tun.rs     │  │
│  │  (共享界面)   │    │   (核心逻辑)      │    │ (TUN + Win32)│  │
│  └──────────────┘    └───────┬──────────┘    └──────┬───────┘  │
│                              │                      │           │
│                         netstack                 wintun.dll     │
│                       (TCP/IP 栈)              (用户态 TUN)      │
│                              │                      │           │
│                              ▼                      ▼           │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                   tokio async runtime                     │   │
│  │                                                           │   │
│  │   ┌─────────────┐  ┌─────────────┐  ┌────────────────┐  │   │
│  │   │ TCP → SOCKS5│  │ UDP 直接转发 │  │ TUN 读写线程    │  │   │
│  │   │(socket bind)│  │(socket bind)│  │(同步阻塞)      │  │   │
│  │   └─────────────┘  └─────────────┘  └────────────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │  SOCKS5 代理服务器│
                    │  (远程 VPS 等)   │
                    └─────────────────┘
```

---

## 核心设计决策

### 1. wintun 而非 TAP

wintun 是 WireGuard 项目的 TUN 驱动，相比传统 TAP 适配器：
- 不需要安装驱动（只需一个 DLL 文件）
- 性能更高（零拷贝环形缓冲区）
- 接口更简单（纯 C API，Rust 有 wintun crate 封装）

### 2. socket bind 而非 route 命令

所有 SOCKS5/UDP 连接通过 `socket.bind(physical_ip)` 绑定到物理网卡，而非用 `route add` 排除特定 IP：
- 程序崩溃不残留路由
- 不需要知道 SOCKS5 服务器 IP（bind 到物理 IP 即可）
- 路由通过 LUID 绑定到 TUN 适配器，适配器销毁时自动清理

### 3. 共享 Slint UI

界面定义在 `vpn-app/ui/main.slint`，iOS/Android/Windows 三平台共用同一套 UI 代码。

---

## 技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| UI | Slint | 跨平台 GUI 框架，声明式语法 |
| TUN 驱动 | wintun 0.14 | WireGuard 的用户态 TUN 驱动 |
| TCP/IP 栈 | netstack-smoltcp | 用户态协议栈，解析原始 IP 包 |
| SOCKS5 | fast-socks5 | 异步 SOCKS5 客户端 |
| 异步运行时 | tokio | 多线程 async runtime |
| Win32 API | windows crate | IP Helper API 绑定 |
| Socket | socket2 | 底层 socket 操作（bind 物理 IP） |
| 注册表 | winreg | DNS 配置 |
| 安装包 | NSIS | Windows 安装程序 |
