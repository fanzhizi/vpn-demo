# 第四章：netstack-smoltcp 与数据包流转

---

## 4.1 为什么不直接用 smoltcp

### smoltcp 是什么

[smoltcp](https://github.com/smoltcp-rs/smoltcp) 是一个用 Rust 编写的用户态 TCP/IP 协议栈。它实现了 IP、TCP、UDP、ICMP 等协议，可以在没有操作系统网络栈的环境中处理网络数据包——比如嵌入式设备、虚拟网卡场景。

### 直接用 smoltcp 的困难

我们最初尝试直接使用 smoltcp，但很快发现它的 API 设计面向的是**嵌入式系统**，而非**透明代理**场景：

| 需求 | smoltcp 提供的 | 我们需要的 |
|---|---|---|
| 接口模型 | 轮询式（poll-based） | 异步（async/await） |
| TCP 连接 | 需要手动管理 socket 句柄、缓冲区分配 | 类似 `TcpListener::accept()` 的高层 API |
| 事件驱动 | 需要自己实现事件循环、计算下次轮询时间 | 集成到 tokio 运行时 |
| 包收发 | Device trait（需要自己实现） | 简单的 `send()`/`recv()` |
| 缓冲区管理 | 需要为每个 socket 预分配固定缓冲区 | 自动管理 |

smoltcp 的典型使用方式：

```rust
// smoltcp 原生 API（伪代码）——非常底层
let mut iface = Interface::new(config, &mut device);
let tcp_handle = iface.add_socket(tcp::Socket::new(rx_buf, tx_buf));

loop {
    let timestamp = Instant::now();
    iface.poll(timestamp, &mut device, &mut sockets);

    let socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
    if socket.may_recv() {
        let data = socket.recv(|buf| (buf.len(), buf.to_vec()));
        // 处理数据...
    }

    // 需要自己计算下次轮询时间
    let wait = iface.poll_delay(timestamp, &sockets);
    thread::sleep(wait.unwrap_or(Duration::from_millis(10)));
}
```

这种模型和 tokio 的异步生态完全不兼容。我们需要将 IP 包"喂入"协议栈，然后以 `AsyncRead`/`AsyncWrite` 的方式读写 TCP 流，这样才能与 `fast-socks5`、`tokio::io::copy_bidirectional` 等异步工具库配合。

### netstack-smoltcp 的解决方案

[netstack-smoltcp](https://crates.io/crates/netstack-smoltcp) 在 smoltcp 之上封装了一层**异步友好**的 API，完美解决了上述问题：

```rust
// netstack-smoltcp 的 API——简洁、异步、与 tokio 兼容
let (stack, runner, udp_socket, tcp_listener) = StackBuilder::default()
    .enable_tcp(true)
    .enable_udp(true)
    .build()?;

// IP 包通过 Stack 的 Sink/Stream 接口收发
stack.send(ip_packet).await?;       // 喂入 IP 包
let response = stack.next().await;   // 取出响应包

// TCP 连接自动呈现为异步流
while let Some((stream, src, dst)) = tcp_listener.next().await {
    // stream 实现了 AsyncRead + AsyncWrite
    // 可以直接用 tokio::io::copy_bidirectional
}
```

一行 `build()` 替代了几十行 smoltcp 配置代码。

---

## 4.2 netstack-smoltcp 的核心抽象

`StackBuilder::build()` 返回四个组件，每个都有明确的职责：

```rust
let (stack, runner, udp_socket, tcp_listener) = StackBuilder::default()
    .enable_tcp(true)
    .enable_udp(true)
    .enable_icmp(true)
    .tcp_buffer_size(65535)
    .udp_buffer_size(65535)
    .stack_buffer_size(1024)
    .build()?;
```

### 四大组件

```
                    ┌─────────────────────────────┐
                    │          StackBuilder        │
                    │                             │
                    │  enable_tcp(true)            │
                    │  enable_udp(true)            │
                    │  enable_icmp(true)           │
                    │  tcp_buffer_size(65535)       │
                    │  udp_buffer_size(65535)       │
                    │  stack_buffer_size(1024)      │
                    └─────────────┬───────────────┘
                                  │ build()
                                  ▼
          ┌──────────────────────────────────────────┐
          │                                          │
    ┌─────┴─────┐  ┌────────┐  ┌──────────┐  ┌──────┴──────┐
    │   Stack    │  │ Runner │  │UdpSocket │  │TcpListener  │
    │            │  │        │  │          │  │             │
    │ Sink+Stream│  │ Future │  │split() → │  │ Stream of   │
    │ for IP pkts│  │ (must  │  │ ReadHalf │  │ TcpStream   │
    │            │  │  spawn)│  │ WriteHalf│  │             │
    └────────────┘  └────────┘  └──────────┘  └─────────────┘
```

| 组件 | 类型 | 职责 |
|---|---|---|
| **Stack** | `impl Sink<Vec<u8>> + Stream<Item=Result<Vec<u8>>>` | IP 数据包的入口和出口 |
| **Runner** | `Option<impl Future>` | 驱动 smoltcp 内部状态机（轮询、定时器、ICMP） |
| **UdpSocket** | `Option<UdpSocket>` | 从 IP 包中提取出的 UDP 数据报 |
| **TcpListener** | `Option<TcpListener>` | 从 IP 包中提取出的 TCP 连接 |

每个 `Option` 组件是否为 `Some` 取决于对应的 `enable_xxx(true)` 是否被调用。

---

## 4.3 Stack：Sink + Stream 双向通道

`Stack` 是整个系统的核心枢纽。它实现了两个 trait：

### Sink：向协议栈喂入 IP 数据包

```rust
use futures::SinkExt;

// 将一个原始 IP 数据包喂入协议栈
// 这个包可能是 TCP SYN、UDP 数据报、ICMP echo request 等
stack.send(ip_packet_bytes).await?;
```

`Sink<Vec<u8>>` 意味着你可以把 `Vec<u8>`（原始 IP 数据包）"倒入"这个水槽。smoltcp 会解析这个包，更新内部的 TCP 状态机、UDP 队列等。

### Stream：从协议栈取出 IP 数据包

```rust
use futures::StreamExt;

// 从协议栈取出一个需要发回 TUN 的 IP 数据包
// 这可能是 TCP SYN-ACK、TCP 数据响应、DNS 应答等
if let Some(Ok(response_packet)) = stack.next().await {
    // 把 response_packet 写回 TUN 网卡
}
```

`Stream<Item = Result<Vec<u8>>>` 产出协议栈生成的响应包。比如收到一个 TCP SYN 后，smoltcp 的 TCP 状态机会产出一个 SYN-ACK 包；收到 TCP 数据后，会产出 ACK 包。

### 双向数据流示意

```
          原始 IP 数据包
    （来自 TUN 网卡 / Swift）
              │
              │  stack.send(pkt)
              ▼
    ┌─────────────────────┐
    │                     │
    │   Stack (smoltcp)   │
    │                     │
    │  ┌───────────────┐  │
    │  │ TCP 状态机      │  │──→ TcpListener (新连接通知)
    │  │ 窗口管理       │  │──→ TcpStream (数据读写)
    │  │ 重传计时器     │  │
    │  └───────────────┘  │
    │  ┌───────────────┐  │
    │  │ UDP 队列       │  │──→ UdpSocket (数据报读写)
    │  └───────────────┘  │
    │  ┌───────────────┐  │
    │  │ ICMP 处理      │  │──→ 自动回复 ping (通过 Runner)
    │  └───────────────┘  │
    │                     │
    └─────────┬───────────┘
              │
              │  stack.next()
              ▼
          响应 IP 数据包
    （发回 TUN 网卡 / Swift）
```

### 为什么 Stack 不能直接在 tokio::select! 中使用

`Stack` 同时是 `Sink` 和 `Stream`，但 Rust 的借用规则不允许同时对一个对象进行可变借用两次：

```rust
// ❌ 编译错误！不能同时 &mut stack 两次
tokio::select! {
    _ = stack.send(pkt) => {}     // 第一次 &mut stack
    result = stack.next() => {}   // 第二次 &mut stack — 冲突！
}
```

这是本项目中一个关键的架构挑战，在 4.8 节详细讨论解决方案。

---

## 4.4 TcpListener：从 IP 包到 TCP 流

`TcpListener` 是 netstack-smoltcp 最"神奇"的抽象之一：它把底层的 TCP 三次握手、窗口管理、重传等细节全部隐藏，呈现为一个简单的连接流。

### 工作原理

```
    原始 IP 包 (TCP SYN)
         │
         │ stack.send()
         ▼
    smoltcp 内部 TCP 状态机
         │
         │ SYN → SYN-ACK → ACK (三次握手)
         │ (自动完成，不需要我们的代码参与)
         │
         ▼
    TcpListener.next()
         │
         │ 返回 (TcpStream, src_addr, dst_addr)
         ▼
    TcpStream: AsyncRead + AsyncWrite
    （看起来就像一个普通的 TCP 连接！）
```

### 在代码中的使用

```rust
let mut tcp_listener = tcp_listener.expect("TCP listener should be enabled");

// tcp_listener 实现了 Stream trait
// 每次 next() 返回一个新的 TCP 连接
while let Some((tcp_stream, src_addr, dst_addr)) = tcp_listener.next().await {
    info!("New TCP: {} -> {}", src_addr, dst_addr);

    // tcp_stream 实现了 AsyncRead + AsyncWrite
    // 可以直接传给 SOCKS5 客户端做双向中继
    tokio::spawn(async move {
        handle_tcp_via_socks5(tcp_stream, dst_addr, socks5_addr).await
    });
}
```

关键点：
- `src_addr`：发起连接的"应用"的地址（实际上是 smoltcp 内部分配的虚拟地址）
- `dst_addr`：应用想要连接的目标地址（如 `142.250.80.46:443` — Google）
- `tcp_stream`：一个完全异步的 TCP 流，可以像真正的 `TcpStream` 一样使用

### TcpStream 与 SOCKS5 中继

```rust
async fn handle_tcp_via_socks5(
    mut local_stream: netstack_smoltcp::TcpStream,  // 来自 netstack 的流
    dst_addr: SocketAddr,
    socks5_addr: SocketAddr,
) -> std::io::Result<()> {
    // 通过 SOCKS5 代理连接到目标地址
    let mut socks5_stream = Socks5Stream::connect(
        socks5_addr.to_string(),
        dst_addr.ip().to_string(),
        dst_addr.port(),
        Socks5Config::default(),
    ).await?;

    // 双向复制数据：local_stream ↔ socks5_stream
    // 这一行就完成了所有的数据中继工作！
    io::copy_bidirectional(&mut local_stream, &mut socks5_stream).await?;

    Ok(())
}
```

`io::copy_bidirectional` 能工作的前提是两端都实现了 `AsyncRead + AsyncWrite`。netstack-smoltcp 的 `TcpStream` 正好满足这个要求——这就是它的封装价值所在。

---

## 4.5 UdpSocket：UDP 数据报处理

### split 为读写两半

```rust
let udp_socket = udp_socket.expect("UDP socket should be enabled");
let (mut udp_rx, mut udp_tx) = udp_socket.split();
```

`split()` 将 `UdpSocket` 拆分为：
- `udp_rx`（ReadHalf）：实现了 `Stream`，产出 `(payload, src_addr, dst_addr)` 三元组
- `udp_tx`（WriteHalf）：实现了 `Sink`，接受 `(payload, src_addr, dst_addr)` 三元组

拆分的好处：读和写可以在不同的异步任务中进行，不需要 `Arc<Mutex<...>>` 包装。

### 读取 UDP 数据报

```rust
// udp_rx 是一个 Stream，每次 next() 返回一个 UDP 数据报
while let Some((payload, src_addr, dst_addr)) = udp_rx.next().await {
    info!("UDP: {} -> {} ({} bytes)", src_addr, dst_addr, payload.len());
    // payload: 纯 UDP 载荷（不含 IP/UDP 头）
    // src_addr: 发送方地址（如 10.0.0.2:12345）
    // dst_addr: 目标地址（如 8.8.8.8:53 — DNS 查询）
}
```

### 写回 UDP 响应

```rust
// udp_tx 是一个 Sink，接受三元组 (payload, src_addr, dst_addr)
// 注意：写回时 src 和 dst 要交换！
// 原始请求：app(src) → dns(dst)
// 响应写回：dns(src) → app(dst)  ← 从 DNS 服务器的视角
udp_tx.send((response, dst_addr, src_addr)).await?;
```

### UDP 转发的架构

本项目中 UDP 不走 SOCKS5 代理（SOCKS5 UDP 支持复杂且不可靠），而是直接转发：

```rust
// 创建一个 channel 用于将 UDP 响应从转发任务发回给 udp_tx
let (udp_reply_tx, mut udp_reply_rx) =
    mpsc::channel::<(Vec<u8>, SocketAddr, SocketAddr)>(256);

// 子任务：从 reply channel 读取响应，写入 udp_tx
tokio::spawn(async move {
    while let Some(msg) = udp_reply_rx.recv().await {
        if let Err(e) = udp_tx.send(msg).await {
            warn!("UDP reply write error: {}", e);
        }
    }
});

// 主 UDP 任务：读取数据报，为每个请求 spawn 转发任务
tokio::spawn(async move {
    while let Some((payload, src_addr, dst_addr)) = udp_rx.next().await {
        let reply_tx = udp_reply_tx.clone();
        // 每个 UDP 请求 spawn 一个独立任务
        // 这样多个 DNS 查询可以并发处理
        tokio::spawn(async move {
            match forward_udp(&payload, src_addr, dst_addr).await {
                Ok(response) => {
                    // 发回响应：注意 src/dst 交换
                    reply_tx.send((response, dst_addr, src_addr)).await.ok();
                }
                Err(e) => warn!("UDP forward error: {}", e),
            }
        });
    }
});
```

为什么需要 `udp_reply_tx` channel？因为 `udp_tx` 不是 `Clone` 的，不能在多个 `spawn` 任务中共享。通过一个 channel 中转，所有转发任务把响应发到 channel，由一个专门的任务负责写入 `udp_tx`。

```
                      ┌──────────────────┐
  udp_rx.next() ───►  │  UDP 转发主任务    │
                      └──────┬───────────┘
                             │ spawn per request
                     ┌───────┼───────┐
                     ▼       ▼       ▼
               ┌─────────┐ ┌──────┐ ┌──────┐
               │forward 1│ │fwd 2 │ │fwd 3 │  (并发 DNS 查询)
               └────┬────┘ └──┬───┘ └──┬───┘
                    │         │        │
                    └────┬────┘────────┘
                         │ reply_tx.send()
                         ▼
                 ┌───────────────┐
                 │  reply channel │
                 └───────┬───────┘
                         │ reply_rx.recv()
                         ▼
                 ┌───────────────┐
                 │ udp_tx.send() │  (写回 Stack)
                 └───────────────┘
```

---

## 4.6 Runner：内部事件驱动

### 为什么需要 Runner

smoltcp 内部需要周期性轮询来处理：
- **TCP 重传定时器**：超时未收到 ACK 的数据需要重传
- **TCP TIME_WAIT**：关闭连接后的等待状态
- **ICMP echo reply**：自动回复 ping 请求
- **ARP/NDP**：（在某些配置下）地址解析

如果没有 Runner 驱动这些内部逻辑，TCP 连接会"卡住"——比如丢包后永远不会重传。

### 启动 Runner

```rust
if let Some(runner) = runner {
    tokio::spawn(async move {
        if let Err(e) = runner.await {
            error!("Runner error: {}", e);
        }
    });
}
```

Runner 是一个 `Future`，必须被 spawn 到 tokio 运行时中。它会一直运行直到 Stack 被 drop。关键点：

- Runner 是 **必须 spawn 的**，如果不 spawn，smoltcp 的内部状态机不会推进
- Runner 和 Stack 是协作的——Runner 处理内部事件，Stack 处理外部的包收发
- Runner 是一个无限循环（直到 Stack drop），所以用 `tokio::spawn` 放到后台

### Runner 与其他组件的关系

```
                 ┌────────────────────────┐
                 │       tokio Runtime     │
                 │                        │
                 │  ┌──────┐  ┌────────┐ │
    send() ─────►│  │ Stack │◄►│ Runner │ │
                 │  └──┬───┘  └────────┘ │
    next() ◄────│     │                  │
                 │     ├──► TcpListener   │
                 │     └──► UdpSocket     │
                 │                        │
                 └────────────────────────┘
```

Stack、Runner、TcpListener、UdpSocket 内部通过共享状态通信（由 netstack-smoltcp 管理），我们不需要关心这些细节。

---

## 4.7 完整的数据包流转路径

让我们追踪一个完整的 HTTP 请求，从 Safari 发出到收到响应。

### 场景：Safari 访问 https://google.com

#### 第 1 步：DNS 查询（UDP）

```
Safari: "我要访问 google.com，先查 DNS"
    │
    │ 系统生成 DNS 查询的 IP 数据包
    │ (src: 10.0.0.2:53421, dst: 8.8.8.8:53, proto: UDP)
    ▼
┌─────────────┐
│  TUN 虚拟网卡 │  iOS 内核将数据包路由到 TUN
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────────────────┐
│  Swift: packetFlow.readPackets()                     │
│         获取原始 IP 数据包                              │
│         调用 tunnel_feed_packet(data, len)            │
└──────┬──────────────────────────────────────────────┘
       │
       │  try_send(packet) → inbound mpsc channel
       ▼
┌─────────────────────────────────────────────────────┐
│  Rust Task 2 (inbound 转发器):                       │
│     inbound_rx.recv() → to_stack_tx.send(pkt)       │
└──────┬──────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────┐
│  Rust Task 1 (Stack 驱动器):                         │
│     to_stack_rx.recv() → stack.send(pkt)            │
│                                                      │
│     smoltcp 解析 IP 包：                              │
│     "这是 UDP 包，目标端口 53"                         │
│     将 UDP 载荷推入 UdpSocket 的队列                   │
└──────┬──────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────┐
│  Rust Task 5 (UDP 转发器):                           │
│     udp_rx.next() → (dns_query, src, 8.8.8.8:53)   │
│                                                      │
│     spawn forward_udp():                             │
│       - 创建真实的 UdpSocket                          │
│       - 向 8.8.8.8:53 发送 DNS 查询                  │
│       - 等待响应（5 秒超时）                            │
│       - 收到 DNS 响应                                 │
│       - reply_tx.send((response, 8.8.8.8:53, src))  │
└──────┬──────────────────────────────────────────────┘
       │
       │  reply_rx.recv() → udp_tx.send(...)
       │  smoltcp 生成 UDP 响应的 IP 数据包
       ▼
┌─────────────────────────────────────────────────────┐
│  Rust Task 1 (Stack 驱动器):                         │
│     stack.next() → 响应 IP 数据包                     │
│     from_stack_tx.send(pkt)                          │
└──────┬──────────────────────────────────────────────┘
       │
       │  Rust Task 3: from_stack_rx.recv() → outbound_tx.send(pkt)
       │  outbound mpsc channel
       ▼
┌─────────────────────────────────────────────────────┐
│  Swift: tunnel_read_packet(buf, len)                 │
│         try_recv() 取出响应包                          │
│         packetFlow.writePackets() 写回 TUN            │
└──────┬──────────────────────────────────────────────┘
       │
       ▼
┌─────────────┐
│  TUN 虚拟网卡 │ → iOS 内核 → Safari 收到 DNS 响应
└─────────────┘
    Safari: "google.com 的 IP 是 142.250.80.46"
```

#### 第 2 步：TCP 连接（经 SOCKS5）

```
Safari: "连接 142.250.80.46:443"
    │
    │ 系统生成 TCP SYN 的 IP 数据包
    ▼
  [同样经过 TUN → Swift → tunnel_feed_packet → Stack]
       │
       ▼
┌─────────────────────────────────────────────────────┐
│  Stack (smoltcp TCP 状态机):                         │
│                                                      │
│  收到 SYN → 内部创建 TCP socket → 发出 SYN-ACK       │
│  (SYN-ACK 通过 stack.next() 写回 TUN)               │
│                                                      │
│  收到 ACK → 三次握手完成 → 通知 TcpListener          │
└──────┬──────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────┐
│  Rust 主循环 (TCP 监听):                              │
│     tcp_listener.next().await                        │
│     → Some((tcp_stream, src, 142.250.80.46:443))    │
│                                                      │
│     spawn handle_tcp_via_socks5():                   │
│       1. 连接 SOCKS5 代理 (192.168.1.100:1080)       │
│       2. SOCKS5 握手，请求连接 142.250.80.46:443      │
│       3. SOCKS5 代理连接目标成功                       │
│       4. copy_bidirectional(tcp_stream, socks5)      │
│          ↕ 双向复制所有数据                             │
│          ↕ TLS 握手、HTTP 请求、响应...                │
│          ↕ 全部透明转发                                │
└─────────────────────────────────────────────────────┘
```

#### 第 3 步：数据传输

```
Safari 发出 TLS ClientHello:

  Safari → TUN → Swift → tunnel_feed_packet
    → Stack.send() → smoltcp 解包 → TcpStream.read()
    → copy_bidirectional → socks5_stream.write()
    → SOCKS5 代理 → 142.250.80.46:443 (Google)

Google 返回 TLS ServerHello:

  Google → SOCKS5 代理 → socks5_stream.read()
    → copy_bidirectional → TcpStream.write()
    → smoltcp 封装为 IP 包 → stack.next()
    → outbound channel → tunnel_read_packet → Swift
    → packetFlow.writePackets() → TUN → Safari
```

### 完整任务拓扑图

```
┌────────────────────────────────────────────────────────────────┐
│                    tokio Runtime                                │
│                                                                │
│  ┌──────────┐   to_stack_tx   ┌──────────────┐                │
│  │  Task 2   │ ──────────────► │   Task 1     │                │
│  │ inbound   │                 │ Stack 驱动器  │                │
│  │ 转发器     │   from_stack_tx │              │                │
│  └──────────┘ ◄─────────────── │ send + next  │                │
│       ▲                        └──────────────┘                │
│       │ inbound_rx                   │                         │
│       │                         ┌────┴────┐                    │
│  ┌────┴────────┐           ┌────┴───┐ ┌───┴────┐              │
│  │  Swift 线程  │           │  Task 6 │ │ Runner │              │
│  │             │           │TcpListen│ │ Task   │              │
│  │feed_packet──┤           │ er 主循环│ │        │              │
│  │             │           └────┬───┘ └────────┘              │
│  │read_packet──┤                │                              │
│  └────┬────────┘          ┌─────┴────────┐                    │
│       │ outbound_rx       │ Task per conn │ ← handle_tcp_     │
│       │                   │ SOCKS5 中继    │    via_socks5()   │
│  ┌────┴────┐              └──────────────┘                    │
│  │  Task 3  │                                                  │
│  │ outbound │    ┌───────────┐   ┌─────────────┐              │
│  │ 转发器    │    │  Task 5    │   │ Task per pkt │              │
│  └─────────┘    │ UDP 转发器  │──►│ forward_udp  │              │
│                 └───────────┘   └──────┬──────┘              │
│                      ▲                  │                      │
│                      │   reply channel  │                      │
│                      └──────────────────┘                      │
└────────────────────────────────────────────────────────────────┘
```

---

## 4.8 tokio::select! 中同时驱动 Stack 的技巧

### 问题：Sink 和 Stream 的借用冲突

如 4.3 节所述，`Stack` 同时实现了 `Sink` 和 `Stream`，但我们需要在一个循环中同时做两件事：

1. 将收到的 IP 包喂入 Stack（`stack.send()`）
2. 从 Stack 取出响应包（`stack.next()`）

如果用 `tokio::select!` 同时等待两个操作，会因为双重可变借用而编译失败。

### 解决方案：Channel 分离模式

本项目的解决方案是将 Stack 独占在一个专门的任务中，通过 channel 与其他任务通信：

```rust
// 创建两个 channel，将 Stack 的读写"导出"到 channel
let (to_stack_tx, mut to_stack_rx) = mpsc::channel::<Vec<u8>>(1024);
let (from_stack_tx, mut from_stack_rx) = mpsc::channel::<Vec<u8>>(1024);

// 专门的任务：独占 Stack，同时处理读和写
tokio::spawn(async move {
    let mut stack = stack;
    loop {
        tokio::select! {
            biased;  // 优先处理先列出的分支

            // 从 channel 接收数据 → 喂入 Stack
            Some(pkt) = to_stack_rx.recv() => {
                if let Err(e) = stack.send(pkt).await {
                    warn!("Stack send error: {}", e);
                }
            }

            // 从 Stack 取出数据 → 发送到 channel
            result = stack.next() => {
                match result {
                    Some(Ok(pkt)) => {
                        if from_stack_tx.send(pkt).await.is_err() {
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        warn!("Stack next error: {}", e);
                    }
                    None => break,
                }
            }
        }
    }
    info!("Stack driver loop ended");
});
```

### 为什么这样可以编译

关键在于 `tokio::select!` 的工作方式。`select!` 不是同时执行两个分支——它**轮询**所有分支，哪个先就绪就执行哪个。在每次循环迭代中：

1. 先检查 `to_stack_rx.recv()` 是否有数据
2. 再检查 `stack.next()` 是否有数据
3. 执行先就绪的那个分支

由于每个分支内部是**顺序执行**的（先 `recv` 再 `send`，或先 `next` 再处理结果），不存在对 `stack` 的同时可变借用。`select!` 宏在编译时会正确处理借用的交替使用。

### `biased` 关键字

```rust
tokio::select! {
    biased;  // ← 关键！
    // ...
}
```

默认情况下，`tokio::select!` 会随机选择先检查哪个分支（防止饥饿）。`biased` 改为按声明顺序检查——总是先检查 `to_stack_rx`，再检查 `stack.next()`。

为什么要 `biased`？因为我们希望**优先处理入站数据包**。如果大量 IP 包涌入，优先将它们喂入 Stack，Stack 才能产生响应包供 `next()` 读取。如果反过来优先 `next()`，而 Stack 内部没有数据要产出，就会浪费轮询周期。

### 整体通信拓扑

```
Swift 线程                  tokio 任务
                                   
tunnel_feed_packet         Task 2 (inbound 转发):
    │                      inbound_rx.recv()
    │ try_send                │
    ▼                         │ to_stack_tx.send()
inbound channel               │
    │                         ▼
    └───────────────►    Task 1 (Stack 驱动器):
                         ┌─ to_stack_rx.recv()
                         │    → stack.send(pkt)
                         │
                         └─ stack.next()
                              → from_stack_tx.send(pkt)
                                   │
                         Task 3 (outbound 转发):
                         from_stack_rx.recv()    
                              │                  
                              │ outbound_tx.send()
                              ▼                  
                         outbound channel
    ┌───────────────◄         │
    │ try_recv                │
    ▼                         
tunnel_read_packet     Swift 线程
```

这种模式的代价是增加了两个 channel（`to_stack` 和 `from_stack`），增加了一些延迟（每个包要多经过一次 channel）。但换来的好处是：
1. **编译通过**：避免了 Sink/Stream 的借用冲突
2. **关注点分离**：Stack 驱动逻辑独立于 Swift 交互逻辑
3. **背压传播**：如果 Swift 读取慢，`from_stack_tx.send()` 会背压到 Stack，Stack 自然减速
4. **可测试性**：可以独立测试 Stack 驱动器，不需要真实的 Swift 环境

### 替代方案对比

| 方案 | 优点 | 缺点 |
|---|---|---|
| **Channel 分离（本项目）** | 简单、正确、易理解 | 额外的 channel 开销 |
| `Pin<Box<Stack>>` + `StreamExt::split()` | 零额外开销 | `Sink::split` 内部也用了 channel/锁 |
| 单线程轮询 | 无并发问题 | 不适合高吞吐场景 |
| `Arc<Mutex<Stack>>` | 直观 | 锁竞争、不支持 async Mutex 下的 Stream |

Channel 分离是最务实的选择——在 VPN 场景中，channel 带来的微秒级延迟完全可以忽略不计。
