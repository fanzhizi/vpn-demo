# 第一章：Android VPN 架构基础

---

## 1.1 Android VpnService 概述

Android 从 API 14 (Android 4.0) 开始提供 `VpnService` API，允许第三方应用创建 VPN 连接。与 iOS 的 `NEPacketTunnelProvider` 类似，它提供了对网络流量的完全控制能力。

### 核心 API 对比

| 特性 | Android VpnService | iOS NEPacketTunnelProvider |
|------|-------------------|---------------------------|
| 运行方式 | 同进程 Service | 独立 Extension 进程 |
| 权限获取 | 运行时弹窗确认 | 系统设置中配置 |
| TUN 设备操作 | FileInputStream/FileOutputStream | NEPacketTunnelFlow |
| 路由配置 | Builder.addRoute() | NEIPv4Settings.includedRoutes |
| Socket 保护 | VpnService.protect(fd) | 不需要（进程隔离） |
| 并发 VPN 数量 | 同一时间只能有一个 | 同一时间只能有一个 |

### Android VPN 的安全限制

```
用户必须通过系统弹窗确认才能启动 VPN
        │
        ▼
┌───────────────────────────────────────────┐
│  "VPN Demo" 想要设置 VPN 连接             │
│  以监控网络流量。                          │
│                                           │
│  仅在您信任该应用时才应允许。               │
│                                           │
│        [取消]        [确定]               │
└───────────────────────────────────────────┘
```

这个弹窗由 `VpnService.prepare(activity)` 触发，是 Android 的安全机制：
- 每次 App 安装后首次使用需要确认
- 同一时间只允许一个 VPN 活跃（新 VPN 会断开旧的）
- 系统通知栏会显示 VPN 图标，用户随时知道 VPN 在运行

---

## 1.2 VpnService 生命周期

```
┌────────────────────────────────────────────────────────────────────┐
│                        完整生命周期                                  │
└────────────────────────────────────────────────────────────────────┘

MainActivity.toggleVpn()
        │
        ├─── VpnService.prepare(this) ──→ 返回 Intent? ──→ 弹窗确认
        │                                                      │
        │    返回 null（已授权）◀────────── 用户点确定 ──────────┘
        │
        ▼
startService(Intent)
        │
        ▼
┌───────────────────────────────────────┐
│  TunnelVpnService.onStartCommand()    │
│                                       │
│  1. setProtectSocketCallback()        │  ← 注册 protect 回调
│  2. TunnelCore.tunnelStart()          │  ← 启动 Rust 核心
│  3. Builder 配置 TUN 设备             │  ← 声明 IP、路由、DNS
│  4. builder.establish()               │  ← 创建 TUN 设备
│  5. 启动 readThread / writeThread     │  ← 数据转发线程
└───────────────┬───────────────────────┘
                │
                ▼
┌───────────────────────────────────────┐
│            VPN 运行中                  │
│                                       │
│  readThread:  TUN → Rust (持续运行)    │
│  writeThread: Rust → TUN (持续运行)    │
│                                       │
│  所有网络流量通过 TUN → Rust → SOCKS5  │
└───────────────┬───────────────────────┘
                │
                │  用户点击 Disconnect / 系统杀进程
                ▼
┌───────────────────────────────────────┐
│  stopVpn()                            │
│                                       │
│  1. running = false                   │  ← 线程退出循环
│  2. TunnelCore.tunnelStop()           │  ← 停止 Rust 核心
│  3. interrupt 线程                    │  ← 唤醒阻塞中的线程
│  4. tunFd.close()                     │  ← 关闭 TUN 设备
│  5. stopSelf()                        │  ← 停止 Service
└───────────────────────────────────────┘
```

---

## 1.3 TUN 设备工作原理

### 什么是 TUN 设备

TUN (network TUNnel) 是 Linux 内核提供的虚拟网络设备，工作在 IP 层（L3）。它创建一个虚拟网卡，用户态程序可以通过文件读写来收发 IP 数据包。

```
┌────────────────────────────────────────────────────────────┐
│                     Android 系统                            │
│                                                            │
│  ┌─────────┐     ┌─────────┐     ┌─────────┐             │
│  │  浏览器  │     │  微信   │     │  其他App │             │
│  └────┬────┘     └────┬────┘     └────┬────┘             │
│       │               │               │                   │
│       └───────────────┼───────────────┘                   │
│                       │ socket API                        │
│                       ▼                                    │
│  ┌─────────────────────────────────────────────────────┐  │
│  │              Linux 内核网络栈                         │  │
│  │                                                     │  │
│  │  路由表: 0.0.0.0/0 → tun0 (VPN 生效时)              │  │
│  │                                                     │  │
│  └──────────┬────────────────────────────┬─────────────┘  │
│             │                            │                │
│     ┌───────▼───────┐           ┌────────▼────────┐      │
│     │   tun0 (虚拟)  │           │  wlan0 (物理)    │      │
│     │  10.0.0.2/24  │           │ 192.168.31.x/24 │      │
│     └───────┬───────┘           └────────▲────────┘      │
│             │                            │                │
└─────────────┼────────────────────────────┼────────────────┘
              │ 文件 I/O                    │ 物理网络
              ▼                            │
┌─────────────────────────┐                │
│   TunnelVpnService      │                │
│                         │                │
│  read(tun_fd) → IP包    │                │
│  → Rust netstack 解析    │                │
│  → SOCKS5 代理转发 ──────┼────────────────┘
│                         │   (protect 过的 socket
│  write(tun_fd) ← IP包   │    走物理网卡)
│  ← Rust netstack 封装    │
└─────────────────────────┘
```

### ParcelFileDescriptor

Android 使用 `ParcelFileDescriptor` 封装 TUN 设备的文件描述符：

```java
// Builder.establish() 返回 ParcelFileDescriptor
ParcelFileDescriptor tunFd = builder.establish();

// 获取底层的 FileDescriptor（Linux fd 的 Java 封装）
FileDescriptor fd = tunFd.getFileDescriptor();

// 用 FileInputStream/FileOutputStream 进行读写
// 每次 read() 返回一个完整的 IP 数据包
// 每次 write() 注入一个完整的 IP 数据包
FileInputStream in = new FileInputStream(fd);   // 读取 App 发出的包
FileOutputStream out = new FileOutputStream(fd); // 注入返回给 App 的包
```

### 为什么选择 setBlocking(true)

```java
builder.setBlocking(true);
```

阻塞模式下，`read()` 会阻塞直到有数据包可读。这简化了线程模型：

| 模式 | 优点 | 缺点 |
|------|------|------|
| 阻塞 (true) | 代码简单，无需轮询 | 需要 interrupt 来终止线程 |
| 非阻塞 (false) | 可以检查 running 标志 | 需要 sleep/poll 避免空转 |

本项目选择阻塞模式，通过 `tunFd.close()` 使阻塞中的 `read()` 抛出 IOException 来终止线程。

---

## 1.4 VPN 权限请求

```java
// VpnService.prepare() 是静态方法，检查 VPN 权限状态
Intent vpnIntent = VpnService.prepare(this);

if (vpnIntent != null) {
    // 需要用户确认 → 启动系统确认对话框
    // 对话框由系统绘制，应用无法绕过或修改
    startActivityForResult(vpnIntent, VPN_REQUEST_CODE);
} else {
    // 已有权限，之前确认过且未被撤销
    startVpnService();
}
```

权限被撤销的情况：
- 用户在系统设置中手动撤销
- 另一个 VPN 应用被激活（Android 同时只允许一个 VPN）
- 应用被卸载重装

---

## 1.5 Builder 配置详解

`VpnService.Builder` 用于声明式配置 TUN 设备的网络参数：

```java
Builder builder = new Builder();

builder.setSession("VPN Demo")      // 会话名称，显示在系统设置中
       .addAddress("10.0.0.2", 24)  // TUN 设备 IP 和子网
       .addDnsServer("8.8.8.8")     // DNS 服务器（流量也走 TUN）
       .addDnsServer("8.8.4.4")     // 备用 DNS
       .setMtu(1500)                // 最大传输单元
       .setBlocking(true)           // 阻塞式读取
       .addRoute("0.0.0.0", 0);    // 路由所有 IPv4 流量
```

### 各参数详解

| 参数 | 值 | 说明 |
|------|-----|------|
| `addAddress` | "10.0.0.2", 24 | TUN 设备的 IP 地址。选用私有地址段避免与物理网络冲突 |
| `addRoute` | "0.0.0.0", 0 | 默认路由，匹配所有目的地。效果：所有流量走 TUN |
| `addDnsServer` | "8.8.8.8" | DNS 查询也走 TUN（重要：否则 DNS 泄露） |
| `setMtu` | 1500 | 标准以太网 MTU。设太小影响性能，设太大可能被分片 |
| `setBlocking` | true | read() 阻塞等待。配合独立线程使用 |

### 路由策略

```
addRoute("0.0.0.0", 0)  →  全局 VPN（所有流量走代理）

// 如果只想代理特定网段：
addRoute("192.168.0.0", 16)  →  只代理 192.168.x.x

// 如果想排除某些 IP（比如局域网）：
// Android API 33+ 支持 excludeRoute()
// 之前只能通过 addRoute 多次添加覆盖
```

---

## 1.6 与 iOS NEPacketTunnelProvider 的架构对比

### 进程模型

```
                Android                           iOS
        ┌────────────────────┐          ┌────────────────────┐
        │     App 进程        │          │     App 进程        │
        │                    │          │                    │
        │  MainActivity      │          │  主 App (UI)       │
        │  TunnelVpnService  │          │                    │
        │  (同一进程!)        │          └────────────────────┘
        └────────────────────┘                    │
                                          IPC (XPC)
                                                  │
                                         ┌────────▼───────────┐
                                         │  Extension 进程     │
                                         │                    │
                                         │  PacketTunnel-     │
                                         │  Provider          │
                                         └────────────────────┘
```

### 关键差异

| 维度 | Android | iOS |
|------|---------|-----|
| **进程隔离** | VpnService 与 App 同进程 | Extension 独立进程 |
| **数据包读取** | FileInputStream.read() 阻塞 | packetFlow.readPackets() 回调 |
| **数据包写入** | FileOutputStream.write() | packetFlow.writePackets() |
| **Socket 保护** | 必须手动 protect(fd) | 系统自动隔离（Extension 进程流量不走 TUN） |
| **内存限制** | 与主 App 共享 | 独立限制（通常 15MB） |
| **Rust 集成** | JNI + .so 动态库 | C Bridging Header + .a 静态库 |
| **配置方式** | Builder 对象 | NEPacketTunnelNetworkSettings |
| **启动方式** | startService(Intent) | NETunnelProviderManager.startVPNTunnel() |

### 为什么 iOS 不需要 protect

iOS 的关键设计：Extension 运行在**独立进程**中，该进程的网络流量**不经过 TUN 设备**。这是操作系统层面的保证，开发者无需做任何额外处理。

而 Android 的 VpnService 与 App 在**同一进程**，该进程创建的 socket 默认也会被路由到 TUN。因此必须显式调用 `protect(fd)` 来标记哪些 socket 应该绕过 VPN。

这个架构差异是本项目中 Android 适配最复杂的部分，详见[第四章](04-socket-protect.md)。
