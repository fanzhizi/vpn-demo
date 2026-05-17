# 第五章：完整数据流追踪

---

## 5.1 从用户点击到网页加载

以用户在浏览器中访问 `https://google.com` 为例，追踪完整的数据流路径。

### 完整时序图

```
┌─────────┐  ┌──────┐  ┌──────────────┐  ┌─────────────┐  ┌─────────────┐  ┌───────────┐
│ 浏览器   │  │ 内核  │  │TunnelVpnSvc │  │ tunnel-core │  │ SOCKS5 代理  │  │ google.com│
└────┬────┘  └──┬───┘  └──────┬───────┘  └──────┬──────┘  └──────┬──────┘  └─────┬─────┘
     │          │              │                 │                │               │
     │ DNS查询  │              │                 │                │               │
     │ 8.8.8.8 │              │                 │                │               │
     │─────────▶│              │                 │                │               │
     │          │ 路由到 tun0  │                 │                │               │
     │          │─────────────▶│                 │                │               │
     │          │              │ read() UDP 包   │                │               │
     │          │              │────────────────▶│                │               │
     │          │              │ tunnelFeedPacket │                │               │
     │          │              │                 │                │               │
     │          │              │                 │ netstack 解析   │               │
     │          │              │                 │ UDP dst=8.8.8.8│               │
     │          │              │                 │                │               │
     │          │              │                 │ forward_udp()  │               │
     │          │              │                 │ protect + send │               │
     │          │              │                 │───────────────▶│               │
     │          │              │                 │                │──DNS查询──────▶│
     │          │              │                 │                │◀──DNS响应──────│
     │          │              │                 │◀───────────────│               │
     │          │              │                 │                │               │
     │          │              │ tunnelReadPacket │                │               │
     │          │              │◀────────────────│                │               │
     │          │              │ write() UDP 响应 │                │               │
     │          │◀─────────────│                 │                │               │
     │ DNS响应  │              │                 │                │               │
     │◀─────────│              │                 │                │               │
     │          │              │                 │                │               │
     │ TCP SYN  │              │                 │                │               │
     │ google IP│              │                 │                │               │
     │─────────▶│ 路由到 tun0  │                 │                │               │
     │          │─────────────▶│ read()          │                │               │
     │          │              │────────────────▶│                │               │
     │          │              │                 │ netstack:      │               │
     │          │              │                 │ 新 TCP 连接     │               │
     │          │              │                 │                │               │
     │          │              │                 │ 创建 socket     │               │
     │          │              │                 │ protect(fd)    │               │
     │          │              │                 │ connect(socks5)│               │
     │          │              │                 │───────────────▶│               │
     │          │              │                 │ SOCKS5 握手    │               │
     │          │              │                 │ CONNECT google │               │
     │          │              │                 │───────────────▶│               │
     │          │              │                 │                │──TCP连接──────▶│
     │          │              │                 │ 连接建立       │               │
     │          │              │                 │◀───────────────│               │
     │          │              │                 │                │               │
     │          │              │                 │ netstack:      │               │
     │          │              │                 │ TCP SYN-ACK    │               │
     │          │              │◀────────────────│               │               │
     │          │◀─────────────│ write()         │                │               │
     │ SYN-ACK  │              │                 │                │               │
     │◀─────────│              │                 │                │               │
     │          │              │                 │                │               │
     │ ...双向数据传输 (copy_bidirectional)...   │               │               │
     │          │              │                 │                │               │
```

---

## 5.2 TUN 读写线程模型

```
┌─────────────────────────────────────────────────────────────────────┐
│                    TunnelVpnService 进程                              │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                    Java 层 (线程)                             │    │
│  │                                                             │    │
│  │  ┌──────────────┐                    ┌──────────────┐       │    │
│  │  │ tun-read 线程 │                    │ tun-write 线程│       │    │
│  │  │              │                    │              │       │    │
│  │  │ while(true): │                    │ while(true): │       │    │
│  │  │  pkt=read()  │                    │  n=readPkt() │       │    │
│  │  │  feedPkt(pkt)│                    │  if n>0:     │       │    │
│  │  │              │                    │   write(buf) │       │    │
│  │  └──────┬───────┘                    └──────▲───────┘       │    │
│  │         │ JNI                               │ JNI           │    │
│  └─────────┼───────────────────────────────────┼───────────────┘    │
│            │                                   │                    │
│  ┌─────────▼───────────────────────────────────┼───────────────┐    │
│  │                 Rust 层 (tokio runtime)      │               │    │
│  │                                             │               │    │
│  │  inbound_tx ──▶ [channel 1024] ──▶ inbound_rx              │    │
│  │                                       │                     │    │
│  │                                       ▼                     │    │
│  │                              ┌─────────────────┐            │    │
│  │                              │  netstack-smoltcp│            │    │
│  │                              │  (TCP/IP 协议栈) │            │    │
│  │                              └────────┬────────┘            │    │
│  │                                       │                     │    │
│  │  outbound_rx ◀── [channel 1024] ◀── outbound_tx            │    │
│  │       │                                                     │    │
│  │       │ try_recv()                                          │    │
│  │       └─────────────────────────────────────────────────────│────┘
│  │                                                             │
│  │  ┌──────────────────────────────────────────────────┐      │
│  │  │           tokio 任务们                            │      │
│  │  │                                                  │      │
│  │  │  ┌─────────────┐  ┌──────────────┐  ┌────────┐ │      │
│  │  │  │TCP listener │  │UDP forwarder │  │ Stack  │ │      │
│  │  │  │→ SOCKS5转发  │  │→ 直接转发    │  │ driver │ │      │
│  │  │  └─────────────┘  └──────────────┘  └────────┘ │      │
│  │  └──────────────────────────────────────────────────┘      │
│  └────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
```

### 线程模型总结

| 线程 | 类型 | 职责 | 阻塞方式 |
|------|------|------|----------|
| tun-read | Java Thread | 从 TUN 读包 → 送入 Rust | FileInputStream.read() 阻塞 |
| tun-write | Java Thread | 从 Rust 取包 → 写入 TUN | try_recv + sleep(1ms) 轮询 |
| tokio workers | Rust Thread (多个) | 运行异步任务 | tokio event loop |
| main (UI) | Android UI Thread | 用户交互 | Android Looper |

### 为什么 tun-write 使用轮询而非阻塞

```
理想方案：tunnelReadPacket() 阻塞直到有数据
问题：
  1. JNI 不支持 Rust 的 async/.await
  2. 如果用条件变量阻塞，需要 Rust 侧通知 Java 侧（复杂）
  3. 停止时需要唤醒阻塞的线程（更复杂）

妥协方案：try_recv() + sleep(1ms)
  - 简单可靠
  - 1ms 延迟对用户无感知
  - CPU 开销极小（1000 次/秒的空循环 vs 数万包/秒的处理能力）
```

---

## 5.3 Rust tunnel-core 内部数据路径

### netstack-smoltcp 的角色

netstack-smoltcp 是一个用户态 TCP/IP 协议栈，它：
1. 接收原始 IP 数据包（从 TUN 读到的）
2. 解析 TCP/UDP/ICMP 头部
3. 管理 TCP 状态机（握手、重传、拥塞控制）
4. 向上层暴露 TcpStream/UdpSocket 接口

```
IP 数据包进入 netstack：

  ┌────────────────────────────────────────┐
  │            IP 数据包                    │
  │  ┌──────┬─────────────┬─────────────┐ │
  │  │IP Hdr│  TCP/UDP Hdr│   Payload   │ │
  │  │dst IP│  dst port   │   数据      │ │
  │  └──────┴─────────────┴─────────────┘ │
  └────────────────────┬───────────────────┘
                       │
                       ▼
  ┌────────────────────────────────────────┐
  │          netstack-smoltcp               │
  │                                        │
  │  TCP? ──▶ tcp_listener.next()          │
  │           返回 (TcpStream, src, dst)    │
  │                                        │
  │  UDP? ──▶ udp_rx.next()               │
  │           返回 (payload, src, dst)      │
  │                                        │
  │  ICMP? ──▶ runner 内部处理             │
  └────────────────────────────────────────┘
```

### Stack 通道模型

```rust
// ffi.rs 中的 Stack 通道设计

// 外部通道：连接 Java TUN 线程和 Rust
let (inbound_tx, inbound_rx) = mpsc::channel::<Vec<u8>>(1024);   // TUN → Rust
let (outbound_tx, outbound_rx) = mpsc::channel::<Vec<u8>>(1024); // Rust → TUN

// 内部通道：连接异步任务和 Stack 对象
let (to_stack_tx, to_stack_rx) = mpsc::channel::<Vec<u8>>(1024);   // → Stack
let (from_stack_tx, from_stack_rx) = mpsc::channel::<Vec<u8>>(1024); // Stack →

// 为什么需要两层通道：
// 1. 外部通道：跨 JNI 边界，需要 Mutex 包装（非 async 代码访问）
// 2. 内部通道：纯 async 环境，性能更好
```

---

## 5.4 TCP 通过 SOCKS5 转发

### 转发流程

```
netstack TCP listener 接收到新连接:
  tcp_listener.next() → (tcp_stream, src="10.0.0.2:54321", dst="142.250.x.x:443")

handle_tcp_via_socks5(tcp_stream, dst, socks5_addr):

  1. 创建到 SOCKS5 代理的 TCP 连接（已 protect）
     ┌───────────────────────────────────┐
     │ new socket → protect → connect    │
     └───────────────────────────────────┘

  2. SOCKS5 协议握手
     ┌───────────────────────────────────┐
     │ Client → Proxy: 05 01 00         │  (版本5, 1个方法, 无认证)
     │ Proxy → Client: 05 00            │  (版本5, 选择无认证)
     │                                   │
     │ Client → Proxy: 05 01 00 01      │  (CONNECT 命令)
     │                  [4B IP] [2B Port]│  (目标地址)
     │ Proxy → Client: 05 00 00 01      │  (成功)
     │                  [4B IP] [2B Port]│  (绑定地址)
     └───────────────────────────────────┘

  3. 双向数据转发
     ┌───────────────────────────────────────────────────┐
     │                                                   │
     │  netstack TcpStream ←─ copy_bidirectional ─→ SOCKS5 Stream  │
     │                                                   │
     │  浏览器数据 → TUN → netstack → SOCKS5 → 代理 → google.com  │
     │  google响应 ← TUN ← netstack ← SOCKS5 ← 代理 ← google.com  │
     │                                                   │
     └───────────────────────────────────────────────────┘
```

### copy_bidirectional 工作原理

```rust
// tokio::io::copy_bidirectional 同时双向复制数据
// 当任一方向 EOF 或错误时停止

match io::copy_bidirectional(&mut local_stream, &mut socks5_stream).await {
    Ok((up, down)) => {
        // up: 上行字节数（App → 代理 → 目标）
        // down: 下行字节数（目标 → 代理 → App）
        info!("Connection closed: up={} down={}", up, down);
    }
    Err(e) => { /* 连接异常断开 */ }
}
```

---

## 5.5 UDP/DNS 直接转发

### 为什么 UDP 不走 SOCKS5

```
原因：
1. SOCKS5 的 UDP ASSOCIATE 支持率低（很多代理不实现）
2. DNS 查询要求低延迟（SOCKS5 额外一次握手太慢）
3. DNS 包很小（通常 <512 字节），直接转发开销小

本项目策略：
  TCP → 通过 SOCKS5 代理（隐藏真实 IP）
  UDP → 直接转发到目标（主要是 DNS 查询）
  
  注意：这意味着 DNS 查询会暴露用户真实 IP
  生产环境应该：使用 DoH/DoT 或 SOCKS5 UDP ASSOCIATE
```

### UDP 转发流程

```
App DNS 查询 (UDP dst=8.8.8.8:53):

  1. netstack 识别为 UDP 包
     udp_rx.next() → (payload="DNS查询", src="10.0.0.2:12345", dst="8.8.8.8:53")

  2. forward_udp(payload, src, dst):
     - 创建新 UDP socket（已 protect）
     - socket.send_to(payload, "8.8.8.8:53")   → DNS 查询直接发出
     - socket.recv_from() 等待响应              → 最多 5 秒超时

  3. 收到 DNS 响应:
     - reply_tx.send((response, dst="8.8.8.8:53", src="10.0.0.2:12345"))
     - 注意 src/dst 互换！（响应的方向相反）

  4. udp_tx.send() → 写回 netstack → 构造 IP 包 → outbound channel → TUN → App
```

---

## 5.6 protect 在数据流中的位置

```
                    哪些 socket 需要 protect？
                    
  ┌─────────────────────────────────────────────────────────┐
  │                                                         │
  │  App 流量 ──▶ TUN ──▶ netstack                         │
  │                            │                            │
  │                    ┌───────┴────────┐                   │
  │                    │                │                   │
  │                    ▼                ▼                   │
  │            TCP 连接请求        UDP 数据包               │
  │                    │                │                   │
  │                    ▼                ▼                   │
  │         ┌──────────────────┐  ┌────────────────┐       │
  │         │ SOCKS5 TCP socket│  │ UDP 转发 socket │       │
  │         │                  │  │                │       │
  │         │  ★ 需要 protect  │  │  ★ 需要 protect│       │
  │         │                  │  │                │       │
  │         └────────┬─────────┘  └───────┬────────┘       │
  │                  │                    │                 │
  │                  └────────┬───────────┘                 │
  │                           │                             │
  │                           ▼                             │
  │                    物理网卡 (wlan0)                      │
  │                    绕过 TUN 路由                         │
  │                           │                             │
  └───────────────────────────┼─────────────────────────────┘
                              │
                              ▼
                       互联网 / SOCKS5 代理
```

### 总结：需要 protect 的 socket

| Socket 类型 | 目的 | 如果不 protect |
|-------------|------|----------------|
| SOCKS5 TCP socket | 连接 SOCKS5 代理服务器 | 死循环（SYN → TUN → SOCKS5 → TUN...） |
| UDP 转发 socket | DNS 查询、UDP 数据转发 | 死循环（UDP → TUN → forward → TUN...） |

### 不需要 protect 的 socket

| Socket 类型 | 原因 |
|-------------|------|
| App 的 socket | 就是要被 TUN 捕获的流量 |
| netstack 内部 | 不是真实 socket，是用户态模拟 |
| localhost 连接 | 127.0.0.0/8 通常不走 VPN 路由 |
