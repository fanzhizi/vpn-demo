# 第四章：Socket 绑定机制（路由绕过）

---

## 4.1 核心问题：路由死循环

### 问题描述

当 TUN 添加了默认路由后，**所有**流量都走 TUN，包括 SOCKS5 代理连接本身：

```
没有路由绕过时的死循环：

  浏览器请求 google.com
       │
       ▼
  TUN 捕获 → netstack 解析为 TCP → 需要通过 SOCKS5 转发
       │
       ▼
  创建到 SOCKS5 代理的 TCP socket
  connect(socks5_addr)
       │
       ▼
  这个 connect 的 SYN 包也被路由到 TUN！（因为 0.0.0.0/1 路由）
       │
       ▼
  TUN 捕获 → netstack 解析 → 又要通过 SOCKS5 转发...
       │
       ▼
  无限循环！程序卡死
```

---

## 4.2 为什么不用 route 命令排除 IP

### 传统方案

```bash
# 添加排除路由：SOCKS5 服务器 IP 走网关而非 TUN
route add 1.2.3.4 mask 255.255.255.255 192.168.1.1
```

### 传统方案的问题

| 问题 | 说明 |
|------|------|
| 路由残留 | 程序崩溃时 `route delete` 不执行，路由残留在系统中 |
| IP 变化 | SOCKS5 服务器 IP 可能变化，需要动态更新路由 |
| 多客户端冲突 | 多个 VPN 客户端添加的排除路由可能冲突 |
| 网关获取 | 需要先获取当前网关 IP，逻辑更复杂 |
| DNS 泄露 | 排除 IP 的方式容易遗漏 DNS 查询包 |

---

## 4.3 socket bind 原理

### 核心思想

不修改路由表来排除特定 IP，而是在创建 socket 时**绑定源地址**：

```
socket bind 方案：

  正常流量：
    浏览器 → TUN → netstack → handle_tcp()
                                   │
                                   ▼
                            socket.bind(192.168.1.100:0)  ← 物理网卡 IP
                            socket.connect(socks5_addr)
                                   │
                                   ▼
                            内核：源 IP 是 192.168.1.100
                            → 属于物理网卡
                            → 直接从物理网卡出去
                            → 不经过 TUN！
                                   │
                                   ▼
                            SOCKS5 代理 → 目标服务器
```

### 内核路由选择的逻辑

```
没有 bind 时：
  connect(socks5_addr)
  → 内核查路由表 → 匹配 0.0.0.0/1 → 从 TUN 出去 → 回环！

有 bind 时：
  bind(192.168.1.100:0)     ← 源地址是物理网卡的 IP
  connect(socks5_addr)
  → 内核查路由表
  → 但源地址已指定为 192.168.1.100
  → 192.168.1.100 属于物理网卡
  → 从物理网卡出去 → 正确！
```

### 为什么 bind 了物理 IP 就不走 TUN

Windows 内核的路由选择规则（简化）：

1. 如果 socket 有绑定的源地址，优先使用源地址所在的接口
2. 源地址 192.168.1.100 → 属于物理网卡接口
3. 即使路由表指向 TUN，内核也不会用 TUN 发送源地址为物理网卡 IP 的包

---

## 4.4 三平台路由绕过对比

| 方面 | iOS | Android | Windows |
|------|-----|---------|---------|
| 机制 | 进程隔离 | protect(fd) | socket bind |
| 原理 | Extension 进程的流量不经过 TUN | 标记 socket 不走 VPN 路由 | 绑定源 IP 到物理网卡 |
| 实现复杂度 | 零（系统自动） | 中（JNI 回调） | 中（socket2 操作） |
| 开发者工作 | 无 | 每个 socket 调用 protect | 每个 socket 调用 bind |
| 需要知道 | 无 | VpnService 引用 | 物理网卡 IP |
| 适用范围 | Extension 进程所有流量 | 单个 fd | 单个 socket |

### iOS：进程隔离（最优雅）

```
  ┌─────────────────────┐     ┌─────────────────────┐
  │   主 App 进程        │     │   Extension 进程     │
  │   流量走 TUN         │     │   流量不走 TUN       │  ← 系统保证！
  └─────────────────────┘     └─────────────────────┘
                                       │
                                SOCKS5 连接直接走物理网卡
```

### Android：protect(fd)

```java
// Kotlin/Java 端
vpnService.protect(socket.fd)  // 标记此 fd 不走 VPN
```

```rust
// Rust 端通过 JNI 回调 Java
let fd = socket.as_raw_fd();
env.call_method(vpn_service, "protect", "(I)Z", &[fd.into()])?;
```

### Windows：socket bind（本项目方案）

```rust
let socket = socket2::Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
socket.bind(&SockAddr::from("192.168.1.100:0".parse()?))?;  // 绑定物理 IP
socket.connect(&SockAddr::from(socks5_addr))?;               // 连接 SOCKS5
```

---

## 4.5 socket2 分步操作详解

### 为什么不能用 TcpStream::connect

```rust
// 标准库 TcpStream::connect 一步完成，无法在 connect 前 bind
let stream = TcpStream::connect(socks5_addr)?;  // 没有机会 bind！

// 必须用 socket2 分步操作
let socket = Socket::new(...)?;    // 1. 创建
socket.bind(physical_ip)?;        // 2. 绑定源地址（关键！）
socket.connect(socks5_addr)?;     // 3. 连接
```

### 完整的 TCP 流程

```rust
// ===== 第 1 步：创建原始 socket =====
let socket = socket2::Socket::new(
    socket2::Domain::IPV4,           // IPv4
    socket2::Type::STREAM,           // TCP
    Some(socket2::Protocol::TCP),
)?;

// ===== 第 2 步：绑定到物理网卡 IP =====
// 端口 0 = 让系统自动分配一个临时端口
let bind_addr: SocketAddr = format!("{}:0", physical_ip).parse().unwrap();
socket.bind(&socket2::SockAddr::from(bind_addr))?;

// ===== 第 3 步：连接到 SOCKS5 代理 =====
// 此时 SYN 包从物理网卡出去，不走 TUN
socket.connect(&socket2::SockAddr::from(socks5_addr))?;

// ===== 第 4 步：转为非阻塞模式 =====
// 因为后续要交给 tokio 的 async 运行时管理
socket.set_nonblocking(true)?;

// ===== 第 5 步：转为 tokio TcpStream =====
let std_stream: std::net::TcpStream = socket.into();
let tokio_stream = tokio::net::TcpStream::from_std(std_stream)?;
```

---

## 4.6 Socks5Stream::use_stream 手动握手

### 为什么不能用 Socks5Stream::connect

```rust
// Socks5Stream::connect 内部会创建新的 TCP 连接
// 它不知道要 bind 到哪个 IP，所以连接会走 TUN
let stream = Socks5Stream::connect(addr, host, port, config).await?;

// 解决：先用 socket2 创建已绑定的连接，再在上面运行 SOCKS5 协议
```

### use_stream + request 两步握手

```rust
// 第一步：use_stream - 在已有连接上初始化 SOCKS5
// 发送 SOCKS5 认证协商（版本号 + 支持的认证方法）
let mut socks5_stream = Socks5Stream::use_stream(
    tokio_stream,     // 已经绑定物理 IP 并连接到 SOCKS5 代理的 TCP 流
    None,             // 不使用用户名/密码认证
    Config::default(),
).await?;

// 第二步：request - 发送 SOCKS5 CONNECT 命令
// 告诉代理连接到实际的目标服务器
let target = TargetAddr::Ip(dst);   // 目标地址（如 google.com:443）
socks5_stream.request(Socks5Command::TCPConnect, target).await?;

// 此时 socks5_stream 已经是一个到目标服务器的透明隧道
// 可以直接 read/write，数据由 SOCKS5 代理中转
```

### SOCKS5 协议交互

```
客户端 (我们)                          SOCKS5 代理
     │                                     │
     │─── use_stream 阶段 ──────────────▶ │
     │  [05 01 00]                        │  ← 版本5, 1种方法, 无认证
     │  ◀────────────────────────────────  │
     │  [05 00]                           │  ← 选择无认证
     │                                     │
     │─── request 阶段 ─────────────────▶ │
     │  [05 01 00 01 ...dst_ip... port]   │  ← CONNECT 命令
     │  ◀────────────────────────────────  │
     │  [05 00 00 01 ...bind_ip... port]  │  ← 连接成功
     │                                     │
     │  此后双向透明转发                     │
     │ ◀────────────────────────────────▶  │
```

---

## 4.7 UDP 同样 bind 物理 IP

UDP 转发不经过 SOCKS5，但同样需要 bind 物理 IP 防止走 TUN：

```rust
let sock = socket2::Socket::new(
    socket2::Domain::IPV4,
    socket2::Type::DGRAM,           // UDP
    Some(socket2::Protocol::UDP),
)?;
sock.bind(&socket2::SockAddr::from(bind_addr))?;  // 绑定物理 IP
sock.set_nonblocking(true)?;

let std_socket: std::net::UdpSocket = sock.into();
let socket = tokio::net::UdpSocket::from_std(std_socket)?;

socket.send_to(payload, dst).await?;   // 从物理网卡出去
```

### UDP 为什么不走 SOCKS5

1. SOCKS5 的 UDP ASSOCIATE 实现复杂，且不是所有代理都支持
2. UDP 主要是 DNS 查询（53 端口），直接转发即可
3. 其他 UDP 流量（如游戏、视频）直接转发延迟更低
4. 如果需要 UDP 也走代理，可以用 SOCKS5 UDP ASSOCIATE 或 tun2socks 方案

---

## 4.8 物理 IP 的全局共享

```
tun.rs                                 main.rs
  │                                      │
  │  get_physical_adapter_ip()           │
  │  → GetBestRoute2 查询                │
  │  → 返回 192.168.1.100               │
  │                                      │
  │  BIND_ADDRESS.write() = Some(...)    │
  │  ★ 写入全局 RwLock ★                │
  │                                      │
  │                                      │  handle_tcp():
  │                                      │  get_bind_address()
  │                                      │  ★ 从 RwLock 读取 ★
  │                                      │  → Some(192.168.1.100)
  │                                      │  → socket.bind(192.168.1.100:0)
  │                                      │
  │                                      │  forward_udp():
  │                                      │  get_bind_address()
  │                                      │  ★ 从 RwLock 读取 ★
  │                                      │  → Some(192.168.1.100)
  │                                      │  → sock.bind(192.168.1.100:0)
```

RwLock 选型理由：
- 写入只发生一次（start_tun 初始化时）
- 读取频繁（每个 TCP/UDP 连接都要读）
- RwLock 允许多个并发读者，不会成为瓶颈
