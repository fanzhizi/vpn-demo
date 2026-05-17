# 第五章：SOCKS5 转发与数据中继

---

## 5.1 SOCKS5 协议简介

SOCKS5 是一个通用的网络代理协议（RFC 1928），工作在会话层（OSI 第 5 层）。与 HTTP 代理不同，SOCKS5 可以代理**任意 TCP/UDP 流量**，不局限于 HTTP。

### 为什么选择 SOCKS5

| 特性 | HTTP 代理 | SOCKS5 代理 |
|---|---|---|
| 支持协议 | 仅 HTTP/HTTPS | 任意 TCP/UDP |
| 工作层级 | 应用层 | 会话层 |
| 加密支持 | CONNECT 隧道 | 不加密（需搭配其他协议） |
| 实现复杂度 | 中等 | 简单 |
| 生态 | 广泛 | 广泛（Shadowsocks、V2Ray 均支持） |

本项目中，tunnel-core 将 TUN 设备截获的 TCP 流量通过 SOCKS5 协议转发到远端代理服务器。

### SOCKS5 协议三个阶段

SOCKS5 连接分为三个阶段：**握手（Handshake）**、**请求（Connect Request）**、**数据中继（Data Relay）**。

#### 阶段一：握手（认证协商）

客户端告诉服务器自己支持哪些认证方式，服务器选择一种：

```
客户端 → 服务器（握手请求）:
+-----+----------+----------+
| VER | NMETHODS | METHODS  |
+-----+----------+----------+
|  1  |    1     | 1~255    |
+-----+----------+----------+
| 0x05|   0x01   |   0x00   |   ← 版本5, 1种方法, 无认证(0x00)
+-----+----------+----------+

服务器 → 客户端（握手响应）:
+-----+--------+
| VER | METHOD |
+-----+--------+
|  1  |   1    |
+-----+--------+
| 0x05|  0x00  |   ← 服务器同意使用无认证
+-----+--------+
```

常见的认证方式：
- `0x00`：无认证（本项目使用）
- `0x02`：用户名/密码认证
- `0xFF`：不可接受的方法

#### 阶段二：连接请求

握手成功后，客户端发送实际的连接请求：

```
客户端 → 服务器（CONNECT 请求）:
+-----+-----+------+------+----------+----------+
| VER | CMD | RSV  | ATYP | DST.ADDR | DST.PORT |
+-----+-----+------+------+----------+----------+
|  1  |  1  | 0x00 |  1   | Variable |    2     |
+-----+-----+------+------+----------+----------+
| 0x05| 0x01| 0x00 | 0x01 | 4 bytes  | 2 bytes  |  ← IPv4 地址
| 0x05| 0x01| 0x00 | 0x03 | 域名     | 2 bytes  |  ← 域名（首字节是长度）
| 0x05| 0x01| 0x00 | 0x04 | 16 bytes | 2 bytes  |  ← IPv6 地址
+-----+-----+------+------+----------+----------+

CMD 命令类型:
  0x01 = CONNECT（TCP 连接，本项目使用）
  0x02 = BIND（TCP 服务端监听）
  0x03 = UDP ASSOCIATE（UDP 转发）

ATYP 地址类型:
  0x01 = IPv4（4 字节）
  0x03 = 域名（首字节为长度，后跟域名字符串）
  0x04 = IPv6（16 字节）
```

**举例**：连接 `93.184.216.34:80`（example.com）的请求字节：

```
05 01 00 01 5D B8 D8 22 00 50
│  │  │  │  ├──────────┤ ├──┤
│  │  │  │  │ IP地址    │ │端口80
│  │  │  │  │93.184.    │
│  │  │  │  │216.34     │
│  │  │  └─ IPv4
│  │  └─── 保留字段
│  └────── CONNECT
└──────── SOCKS5
```

服务器响应：

```
服务器 → 客户端（CONNECT 响应）:
+-----+-----+------+------+----------+----------+
| VER | REP | RSV  | ATYP | BND.ADDR | BND.PORT |
+-----+-----+------+------+----------+----------+
|  1  |  1  | 0x00 |  1   | Variable |    2     |
+-----+-----+------+------+----------+----------+
| 0x05| 0x00| 0x00 | 0x01 | 4 bytes  | 2 bytes  |

REP 响应状态:
  0x00 = 成功
  0x01 = 一般 SOCKS 服务器错误
  0x02 = 连接不被规则允许
  0x03 = 网络不可达
  0x04 = 主机不可达
  0x05 = 连接被拒绝
  0x06 = TTL 过期
  0x07 = 不支持的命令
  0x08 = 不支持的地址类型
```

#### 阶段三：数据中继

当服务器返回 `REP=0x00`（成功）后，SOCKS5 连接变成一个**透明的 TCP 双向管道**。客户端和目标服务器之间的数据直接通过这个管道传输，SOCKS5 服务器只做透传，不再添加任何协议头。

```
阶段一+二（握手+请求）完成后:

客户端 ←────────────── 透明管道 ──────────────→ 目标服务器
        (直接传输 HTTP/TLS/任何 TCP 数据)
```

---

## 5.2 fast-socks5 库的使用

本项目使用 `fast-socks5` crate 作为 SOCKS5 客户端实现，它基于 tokio 异步运行时，API 简洁高效。

### 依赖配置

在 `tunnel-core/Cargo.toml` 中：

```toml
[dependencies]
fast-socks5 = "1"
tokio = { version = "1", features = ["rt-multi-thread", "net", "io-util", "sync", "time", "macros"] }
```

### Socks5Stream::connect

`Socks5Stream::connect` 是最核心的 API，它完成上述三个阶段（握手 + 请求），返回一个可以像普通 TCP 流一样读写的流对象：

```rust
use fast_socks5::client::{Config as Socks5Config, Socks5Stream};

// 参数说明:
//   socks5_addr: SOCKS5 服务器地址，如 "192.168.31.209:1080"
//   target_host: 目标主机（IP 或域名），如 "93.184.216.34" 或 "google.com"
//   target_port: 目标端口，如 80 或 443
//   config: 配置对象（认证方式、超时等）
let mut socks5_stream = Socks5Stream::connect(
    socks5_addr.to_string(),       // SOCKS5 代理地址
    dst_addr.ip().to_string(),     // 目标 IP
    dst_addr.port(),               // 目标端口
    Socks5Config::default(),       // 默认配置（无认证）
)
.await
.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
```

### Socks5Config 配置

`Socks5Config::default()` 创建一个无认证的默认配置。如果需要用户名密码认证，可以这样：

```rust
let mut config = Socks5Config::default();
// fast-socks5 支持通过 connect_with_password 进行认证:
let stream = Socks5Stream::connect_with_password(
    socks5_addr,
    target_host,
    target_port,
    "username".to_string(),
    "password".to_string(),
    Socks5Config::default(),
).await?;
```

### 错误处理

`Socks5Stream::connect` 返回的错误类型是 `fast_socks5::SocksError`，不是标准的 `std::io::Error`。因此需要做转换：

```rust
.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
```

常见的错误场景：
- 无法连接到 SOCKS5 服务器（网络不通）
- SOCKS5 握手失败（认证方式不匹配）
- 目标地址不可达（SOCKS5 服务器连不上目标）

---

## 5.3 TCP 双向中继

SOCKS5 连接建立后，最关键的一步是**双向数据中继**——把来自虚拟网卡的 TCP 数据和来自 SOCKS5 隧道的数据互相转发。

### handle_tcp_via_socks5 完整代码解析

```rust
async fn handle_tcp_via_socks5(
    mut local_stream: netstack_smoltcp::TcpStream,  // 来自虚拟网卡的 TCP 流
    dst_addr: SocketAddr,                            // 原始目标地址
    socks5_addr: SocketAddr,                         // SOCKS5 服务器地址
) -> std::io::Result<()> {
    use fast_socks5::client::{Config as Socks5Config, Socks5Stream};

    info!("Connecting via SOCKS5 {} to {}", socks5_addr, dst_addr);

    // 第一步：通过 SOCKS5 服务器建立到目标地址的隧道
    // 这一步完成了 SOCKS5 握手和 CONNECT 请求
    let mut socks5_stream = Socks5Stream::connect(
        socks5_addr.to_string(),
        dst_addr.ip().to_string(),
        dst_addr.port(),
        Socks5Config::default(),
    )
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    info!("SOCKS5 tunnel established: -> {}", dst_addr);

    // 第二步：双向中继 — 这是整个隧道的核心！
    match io::copy_bidirectional(&mut local_stream, &mut socks5_stream).await {
        Ok((up, down)) => {
            // up: 从 local_stream 复制到 socks5_stream 的字节数（上行）
            // down: 从 socks5_stream 复制到 local_stream 的字节数（下行）
            info!("Connection {} closed: up={} down={}", dst_addr, up, down);
        }
        Err(e) => {
            warn!("copy_bidirectional error {}: {}", dst_addr, e);
        }
    }

    Ok(())
}
```

### tokio::io::copy_bidirectional 详解

`copy_bidirectional` 是 tokio 提供的工具函数，它同时在两个方向上复制数据：

```
                  copy_bidirectional
                  ┌──────────────┐
local_stream ────►│              │────► socks5_stream   (上行: 设备→代理)
                  │              │
local_stream ◄────│              │◄──── socks5_stream   (下行: 代理→设备)
                  └──────────────┘
```

它的等价实现大致如下（简化版）：

```rust
// copy_bidirectional 内部做了类似这样的事情:
async fn copy_bidirectional<A, B>(a: &mut A, b: &mut B) -> io::Result<(u64, u64)>
where
    A: AsyncRead + AsyncWrite + Unpin,
    B: AsyncRead + AsyncWrite + Unpin,
{
    let (a_reader, a_writer) = tokio::io::split(a);
    let (b_reader, b_writer) = tokio::io::split(b);

    // 同时运行两个方向的复制
    let a_to_b = tokio::io::copy(a_reader, b_writer);  // A 读 → B 写
    let b_to_a = tokio::io::copy(b_reader, a_writer);  // B 读 → A 写

    // 等待两个方向都完成（任一端关闭连接时结束）
    tokio::try_join!(a_to_b, b_to_a)
}
```

### 为什么用 copy_bidirectional 而不是手动实现

1. **效率**：内部使用了优化的缓冲区管理，避免不必要的内存拷贝
2. **正确性**：正确处理了半关闭（half-close）状态——当一端关闭写入时，另一方向仍然可以继续传输
3. **简洁**：一行代码替代几十行手动实现

### 数据流的完整路径

从设备上的 App 发起 HTTPS 请求到收到响应的完整路径：

```
Safari 请求 google.com:443
    │
    ▼
iOS 内核路由到 TUN 网卡
    │
    ▼
packetFlow.readPackets() 读取 IP 包
    │
    ▼
tunnel_feed_packet() → Rust inbound channel
    │
    ▼
netstack-smoltcp 解析 IP→TCP，重组为 TCP 流
    │
    ▼
tcp_listener.next() 获得 TcpStream (local_stream)
    │
    ▼
handle_tcp_via_socks5:
  ├─ Socks5Stream::connect()  ← 与 SOCKS5 服务器建立隧道
  └─ copy_bidirectional()     ← 双向转发开始
       │
       ├─ 上行: local_stream → socks5_stream → 代理服务器 → google.com
       │
       └─ 下行: google.com → 代理服务器 → socks5_stream → local_stream
                                                              │
                                                              ▼
                                                netstack-smoltcp 组装为 IP 包
                                                              │
                                                              ▼
                                                from_stack → outbound channel
                                                              │
                                                              ▼
                                                tunnel_read_packet() → Swift
                                                              │
                                                              ▼
                                                packetFlow.writePackets()
                                                              │
                                                              ▼
                                                TUN → iOS 内核 → Safari 收到响应
```

---

## 5.4 UDP/DNS 直接转发

### 为什么 DNS 必须工作

DNS 是互联网的基础——几乎所有网络请求都以 DNS 查询开始。当 VPN 隧道启用后，DNS 查询也会被路由到 TUN 网卡。如果 DNS 不工作，**所有基于域名的请求都会失败**，即使 TCP 转发逻辑完全正确。

在本项目中，我们通过 `NEDNSSettings` 指定了 DNS 服务器：

```swift
settings.dnsSettings = NEDNSSettings(servers: ["8.8.8.8", "8.8.4.4"])
```

这意味着 iOS 会将 DNS 查询发送到 `8.8.8.8:53`，这个 UDP 包会经过 TUN → netstack-smoltcp → UDP socket → `forward_udp` 函数。

### forward_udp 函数详解

```rust
/// Forward a UDP packet directly to destination and return the response.
/// 直接将 UDP 包转发到目标地址并返回响应。
/// 注意：不经过 SOCKS5 代理！
async fn forward_udp(
    payload: &[u8],          // UDP 载荷（对 DNS 来说就是 DNS 查询报文）
    _src_addr: SocketAddr,   // 源地址（TUN 网卡的虚拟 IP，未使用）
    dst_addr: SocketAddr,    // 目标地址（如 8.8.8.8:53）
) -> std::io::Result<Vec<u8>> {
    // 根据目标地址是 IPv4 还是 IPv6，绑定对应的本地地址
    // "0.0.0.0:0" 表示让操作系统自动分配端口
    let local_addr: SocketAddr = if dst_addr.is_ipv4() {
        "0.0.0.0:0".parse().unwrap()
    } else {
        "[::]:0".parse().unwrap()
    };

    // 创建一个新的 UDP socket
    let socket = TokioUdpSocket::bind(local_addr).await?;

    // 发送 UDP 载荷到目标地址
    socket.send_to(payload, dst_addr).await?;

    // 等待响应，最多等 5 秒
    let mut buf = vec![0u8; 4096];
    let (len, _) = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        socket.recv_from(&mut buf),
    )
    .await
    .map_err(|_| std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "UDP timeout"
    ))??;   // 注意两个 ?：外层解 timeout Result，内层解 recv_from Result

    buf.truncate(len);
    Ok(buf)
}
```

### 为什么 UDP 不经过 SOCKS5

SOCKS5 协议虽然支持 UDP ASSOCIATE 命令，但实现复杂度远高于 TCP CONNECT：

1. 需要先发送 UDP ASSOCIATE 请求获取中继地址
2. 每个 UDP 包需要添加 SOCKS5 UDP 包头
3. 很多 SOCKS5 服务器不支持 UDP

对于 DNS 查询这种场景，直接转发更简单可靠。DNS 查询通常发往公共 DNS 服务器（8.8.8.8），这些服务器全球可达，不需要通过代理。

### 超时处理的必要性

UDP 是无连接协议，如果目标不响应，`recv_from` 会永远阻塞。5 秒超时确保：

1. 不会因为一个 DNS 查询失败而永久占用资源
2. iOS 系统可以在超时后重试或使用备用 DNS 服务器

```rust
// tokio::time::timeout 的返回类型是 Result<Result<T, E>, Elapsed>
// 外层 Result: Ok = 没超时, Err = 超时
// 内层 Result: recv_from 的结果
tokio::time::timeout(Duration::from_secs(5), socket.recv_from(&mut buf))
    .await           // Result<Result<(usize, SocketAddr), io::Error>, Elapsed>
    .map_err(...)    // 超时 → io::Error(TimedOut)
    ?                // 解开超时 Result
    ?                // 解开 recv_from Result
```

### UDP 转发的完整架构

```rust
// run_stack 中的 UDP 处理部分:

// 1. 从 netstack-smoltcp 获取 UDP socket 并拆分为读写两半
let (mut udp_rx, mut udp_tx) = udp_socket.split();

// 2. 创建一个 channel 用于 UDP 回复
let (udp_reply_tx, mut udp_reply_rx) = mpsc::channel::<(Vec<u8>, SocketAddr, SocketAddr)>(256);

// 3. 回复写入任务：从 channel 读取回复，写入 netstack 的 UDP 发送端
tokio::spawn(async move {
    while let Some(msg) = udp_reply_rx.recv().await {
        // msg = (响应数据, 源地址, 目标地址)
        if let Err(e) = udp_tx.send(msg).await {
            warn!("UDP reply write error: {}", e);
        }
    }
});

// 4. 转发任务：从 netstack 读取 UDP 包，转发并等待回复
tokio::spawn(async move {
    while let Some((payload, src_addr, dst_addr)) = udp_rx.next().await {
        let reply_tx = udp_reply_tx.clone();
        // 每个 UDP 包在独立的 task 中处理，互不阻塞
        tokio::spawn(async move {
            match forward_udp(&payload, src_addr, dst_addr).await {
                Ok(response) => {
                    // 注意：回复时交换了 src 和 dst！
                    // 原来是 src→dst，回复是 dst→src
                    reply_tx.send((response, dst_addr, src_addr)).await.ok();
                }
                Err(e) => warn!("UDP forward error: {}", e),
            }
        });
    }
});
```

关键设计点：
- **每个 UDP 包独立 spawn**：DNS 查询之间互不阻塞，一个查询超时不影响其他查询
- **地址交换**：回复时 `(response, dst_addr, src_addr)`，源和目标对调，因为响应应该从 DNS 服务器"发回"给发起查询的应用
- **channel 解耦**：读取和写入通过 `mpsc::channel` 解耦，避免共享 `udp_tx` 的锁竞争

---

## 5.5 路由回环问题与解决

### 什么是路由回环

当 VPN 隧道启用后，`NEIPv4Route.default()`（即 `0.0.0.0/0`）会把**所有流量**路由到 TUN 网卡。这包括我们自己要连接 SOCKS5 服务器的流量！

```
App 发起请求
    ↓
TUN 截获 → tunnel-core 处理
    ↓
需要连接 SOCKS5 服务器 192.168.31.209:1080
    ↓
这个连接也被路由到 TUN ← 回环！
    ↓
又要通过 SOCKS5 转发...
    ↓
无限循环 → 网络死锁
```

### 解决方案：excludedRoutes

在 `PacketTunnelProvider.swift` 中，我们将 SOCKS5 服务器的 IP 从 VPN 路由中排除：

```swift
let ipv4 = NEIPv4Settings(addresses: ["10.0.0.2"], subnetMasks: ["255.255.255.0"])
ipv4.includedRoutes = [NEIPv4Route.default()]  // 所有流量走 TUN

// 关键：排除 SOCKS5 服务器 IP
if let host = socks5Address.split(separator: ":").first {
    let excludeRoute = NEIPv4Route(
        destinationAddress: String(host),      // "192.168.31.209"
        subnetMask: "255.255.255.255"          // /32 — 精确匹配单个 IP
    )
    ipv4.excludedRoutes = [excludeRoute]
    os_log("Excluding route: %{public}@", log: self.log, type: .info, String(host))
}
```

排除后的路由行为：

```
App 发起请求 google.com
    ↓ (匹配 0.0.0.0/0)
TUN → tunnel-core → SOCKS5 客户端
    ↓ (连接 192.168.31.209)
    ↓ (匹配 excludedRoute 192.168.31.209/32)
直接走物理网卡 → SOCKS5 服务器 ✓  不回环了！
```

### 子网掩码 255.255.255.255 的含义

`/32` 子网掩码表示精确匹配一个 IP 地址。这是最精确的排除方式，只排除 SOCKS5 服务器这一个 IP，其他流量仍然走 VPN。

### 需要排除的其他场景

在更复杂的场景中，可能还需要排除：

| 场景 | 需要排除的地址 |
|---|---|
| SOCKS5 服务器 | 服务器 IP/32 |
| 局域网设备 | 192.168.0.0/16（如需局域网直连） |
| 多个代理服务器 | 每个服务器 IP/32 |
| IPv6 代理 | 需要在 `NEIPv6Settings` 中单独配置 |

---

## 5.6 扩展：替换为其他协议

SOCKS5 只是最简单的代理协议。实际的翻墙工具通常使用加密协议来对抗审查。本项目的架构设计使得替换代理协议非常简单。

### 架构的解耦点

```
TUN → netstack-smoltcp → TcpStream
                              │
                              ▼
                    handle_tcp_via_socks5()   ← 只需替换这个函数！
                              │
                              ▼
                    copy_bidirectional()
```

`handle_tcp_via_socks5` 函数接收一个 `TcpStream`（来自虚拟网卡），只要替换其中建立远端连接的部分，就能支持不同的代理协议。

### 替换为 Shadowsocks

[Shadowsocks](https://shadowsocks.org/) 是一个加密代理协议。使用 `shadowsocks-rust` crate：

```rust
// 概念代码，展示替换思路
async fn handle_tcp_via_shadowsocks(
    mut local_stream: netstack_smoltcp::TcpStream,
    dst_addr: SocketAddr,
    ss_config: &ServerConfig,
) -> std::io::Result<()> {
    // 使用 shadowsocks-rust 库建立连接
    let mut ss_stream = ProxyClientStream::connect(
        context,
        ss_config,
        dst_addr,
    ).await?;

    // 同样使用 copy_bidirectional 进行双向中继
    // 这部分代码完全不变！
    io::copy_bidirectional(&mut local_stream, &mut ss_stream).await?;
    Ok(())
}
```

### 替换为 VMess (V2Ray)

VMess 是 V2Ray 项目的核心协议，提供更强的抗检测能力：

```rust
// 概念代码
async fn handle_tcp_via_vmess(
    mut local_stream: netstack_smoltcp::TcpStream,
    dst_addr: SocketAddr,
    vmess_config: &VMessConfig,
) -> std::io::Result<()> {
    // 建立 VMess 连接（需要第三方 VMess 客户端库）
    let mut vmess_stream = VMessStream::connect(
        vmess_config.server_addr,
        vmess_config.uuid,
        dst_addr,
    ).await?;

    // 双向中继 — 同样不变
    io::copy_bidirectional(&mut local_stream, &mut vmess_stream).await?;
    Ok(())
}
```

### 替换协议的通用模式

不管使用什么代理协议，模式都是相同的：

```rust
async fn handle_tcp_via_xxx(
    mut local_stream: netstack_smoltcp::TcpStream,  // 来自虚拟网卡
    dst_addr: SocketAddr,                            // 原始目标
    config: &XxxConfig,                              // 协议配置
) -> std::io::Result<()> {
    // 1. 使用特定协议建立远端连接
    let mut remote_stream = XxxStream::connect(config, dst_addr).await?;

    // 2. 双向中继（永远不变）
    io::copy_bidirectional(&mut local_stream, &mut remote_stream).await?;

    Ok(())
}
```

关键约束：`remote_stream` 必须实现 `AsyncRead + AsyncWrite`（tokio 的异步读写 trait），这是 `copy_bidirectional` 的要求。绝大多数 Rust 代理库都满足这个条件。

### UDP 转发的协议替换

UDP 转发更复杂。SOCKS5 的 UDP ASSOCIATE、Shadowsocks 的 UDP relay、VMess 的 UDP over TCP 各有不同。但基本思路类似——替换 `forward_udp` 函数即可。

对于简单场景（如仅 DNS），当前的直接转发方案已经足够。只有当需要通过代理转发所有 UDP 流量时（如视频通话、游戏），才需要实现完整的 UDP 代理转发。
