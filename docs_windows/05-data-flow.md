# 第五章：完整数据流

---

## 5.1 从点击 Connect 到网页加载

### 时序图

```
用户                 Slint UI          main.rs           tun.rs              系统
 │                     │                  │                 │                  │
 │  点击 Connect       │                  │                 │                  │
 │────────────────────▶│                  │                 │                  │
 │                     │ on_connect_clicked│                 │                  │
 │                     │─────────────────▶│                 │                  │
 │                     │                  │ spawn thread    │                  │
 │                     │                  │────────────────▶│                  │
 │                     │                  │                 │ GetBestRoute2    │
 │                     │                  │                 │────────────────▶│
 │                     │                  │                 │ physical IP ◀───│
 │                     │                  │                 │                  │
 │                     │                  │                 │ load wintun.dll  │
 │                     │                  │                 │────────────────▶│
 │                     │                  │                 │                  │
 │                     │                  │                 │ create adapter   │
 │                     │                  │                 │────────────────▶│
 │                     │                  │                 │ set IP/DNS/route │
 │                     │                  │                 │────────────────▶│
 │                     │                  │                 │                  │
 │                     │                  │                 │ start session    │
 │                     │                  │                 │ 启动读写线程      │
 │                     │                  │                 │                  │
 │                     │                  │ run_netstack    │                  │
 │                     │                  │ (事件循环)       │                  │
 │                     │                  │                 │                  │
 │                     │  Timer 轮询      │                 │                  │
 │                     │  running=true    │                 │                  │
 │ ◀── UI 显示 Connected                  │                 │                  │
```

---

## 5.2 TCP 请求的完整数据流

以浏览器访问 `https://google.com` 为例：

```
浏览器                                                       SOCKS5 代理
  │                                                             │
  │ DNS 查询 google.com (UDP)                                   │
  │───────────────────────┐                                     │
  │                       ▼                                     │
  │               ┌──────────────┐                              │
  │               │   TUN 设备    │                              │
  │               └──────┬───────┘                              │
  │                      │ receive_blocking()                   │
  │                      ▼                                      │
  │               ┌──────────────┐                              │
  │               │  TUN 读线程   │                              │
  │               └──────┬───────┘                              │
  │                      │ inbound_tx.send()                    │
  │                      ▼                                      │
  │               ┌──────────────┐                              │
  │               │  to_stack_tx  │                              │
  │               └──────┬───────┘                              │
  │                      │ stack.send()                         │
  │                      ▼                                      │
  │            ┌────────────────────┐                           │
  │            │   netstack-smoltcp  │                           │
  │            │   (TCP/IP 协议栈)    │                           │
  │            └────────┬───────────┘                           │
  │                     │                                       │
  │          ┌──────────┴──────────┐                            │
  │          ▼                     ▼                            │
  │   ┌──────────┐         ┌──────────┐                        │
  │   │ UDP 数据报│         │ TCP 连接  │                        │
  │   │ (DNS等)  │         │ (HTTP等)  │                        │
  │   └────┬─────┘         └────┬─────┘                        │
  │        │                    │                               │
  │        ▼                    ▼                               │
  │  forward_udp()       handle_tcp()                           │
  │        │                    │                               │
  │        │                    │ socket2::Socket::new()        │
  │        │                    │ socket.bind(192.168.1.100:0)  │
  │   sock.bind(...)           │ socket.connect(socks5)        │
  │   socket.send_to(dst)      │                               │
  │        │                    │ Socks5Stream::use_stream()    │
  │        │                    │ socks5_stream.request()       │
  │        │                    │                               │
  │        │ (物理网卡)         │ (物理网卡)                     │
  │        │                    │                               │
  │        ▼                    ▼                               │
  │   DNS 服务器           SOCKS5 代理 ──────────────────────▶ 目标
  │   8.8.8.8                   │                            google.com
  │        │                    │                               │
  │   DNS 响应              TCP 响应                            │
  │        │                    │ ◀─────────────────────────── 响应
  │        ▼                    ▼                               │
  │   udp_reply_tx         copy_bidirectional                   │
  │        │                    │                               │
  │        └────────┬───────────┘                               │
  │                 ▼                                           │
  │          ┌──────────────┐                                   │
  │          │  netstack     │                                   │
  │          └──────┬───────┘                                   │
  │                 │ from_stack_tx                              │
  │                 ▼                                           │
  │          ┌──────────────┐                                   │
  │          │  TUN 写线程   │                                   │
  │          └──────┬───────┘                                   │
  │                 │ session.send_packet()                      │
  │                 ▼                                           │
  │          ┌──────────────┐                                   │
  │          │   TUN 设备    │                                   │
  │          └──────┬───────┘                                   │
  │                 │                                           │
  │ ◀───────────────┘                                           │
  │ 收到 google.com 的网页内容                                    │
```

---

## 5.3 wintun 读写线程模型

### 为什么用同步线程而非 async

```
wintun API 是同步阻塞的：
  session.receive_blocking()   ← 阻塞等待数据包
  session.send_packet()        ← 同步写入

tokio runtime 是异步的：
  tun_rx.recv().await          ← 异步等待
  stack.send(pkt).await        ← 异步发送

桥接方案：
  同步线程 ←──mpsc channel──→ 异步 tokio 任务

  读线程 (std::thread):
    loop { receive_blocking() → inbound_tx.blocking_send() }

  写线程 (std::thread):
    loop { outbound_rx.blocking_recv() → send_packet() }

  注意：用 blocking_send/blocking_recv 而非 .await，
  因为同步线程中不能使用 await。
```

### 线程全景图

```
┌─────────────────────────────────────────────────────────────┐
│ 主线程 (UI)                                                  │
│  window.run() ← Slint 事件循环                               │
│  Timer 500ms ← 轮询 running 状态                             │
└─────────────────────────────────────────────────────────────┘

┌──────────────────────┐   ┌──────────────────────┐
│ TUN 读线程 (std)      │   │ TUN 写线程 (std)      │
│ receive_blocking()   │   │ blocking_recv()      │
│ → inbound_tx         │   │ → send_packet()      │
└──────────┬───────────┘   └──────────▲───────────┘
           │ mpsc                     │ mpsc
           ▼                          │
┌─────────────────────────────────────────────────────────────┐
│ tokio runtime (子线程中的 block_on)                           │
│                                                             │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │ Stack 驱动   │  │ TUN→Stack    │  │ Stack→TUN        │   │
│  │ (select!)   │  │ 桥接任务      │  │ 桥接任务          │   │
│  └─────────────┘  └──────────────┘  └──────────────────┘   │
│                                                             │
│  ┌─────────────┐  ┌──────────────┐                         │
│  │ TCP listener│  │ UDP 转发      │                         │
│  │ → handle_tcp│  │ → forward_udp│                         │
│  └─────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘

┌──────────────────────┐
│ adapter 保活线程 (std) │
│ sleep(500ms) 循环      │
│ running=false → drop   │
└──────────────────────┘
```

---

## 5.4 netstack 内部路径

### TCP 连接处理

```
原始 IP 包 (TUN)
     │
     ▼
netstack-smoltcp
     │
     ├─ 解析 IP 头 → 源/目标 IP
     ├─ 解析 TCP 头 → 源/目标端口
     ├─ TCP 状态机 → SYN/ACK/FIN 处理
     │
     ▼
tcp_listener.next()
     → (TcpStream, src_addr, dst_addr)
     │
     ▼
handle_tcp(stream, dst, socks5)
     → socket bind + SOCKS5 转发
     → copy_bidirectional 双向复制
```

### UDP 数据报处理

```
原始 IP 包 (TUN)
     │
     ▼
netstack-smoltcp
     │
     ├─ 解析 IP 头
     ├─ 解析 UDP 头
     │
     ▼
udp_rx.next()
     → (payload, src_addr, dst_addr)
     │
     ▼
forward_udp(payload, dst)
     → socket bind + send_to
     → recv_from (5秒超时)
     │
     ▼
udp_reply_tx.send((resp, dst, src))
     → udp_tx.send() → netstack → TUN
```

---

## 5.5 socket bind 在数据流中的位置

```
关键节点标注：

TUN ──read──▶ netstack ──parse──▶ TCP/UDP
                                    │
                                    ▼
                            ┌──────────────┐
                            │ socket bind  │  ← ★ 路由绕过在此发生 ★
                            │ 物理网卡 IP   │
                            └──────┬───────┘
                                   │
                    ┌──────────────┼──────────────┐
                    ▼              ▼              ▼
              ┌─────────┐  ┌─────────────┐ ┌──────────┐
              │ TCP SOCKS5│  │ UDP direct │ │ 物理网卡  │
              │ 转发      │  │ 转发        │ │ (出口)   │
              └─────────┘  └─────────────┘ └──────────┘
                    │              │
                    ▼              ▼
              SOCKS5 代理    DNS/其他 UDP
                    │         目标服务器
                    ▼
              目标网站

响应的反向路径：

目标 → SOCKS5/DNS → 物理网卡 → socket → netstack → TUN → 浏览器
```

---

## 5.6 断开连接的清理流程

```
用户点击 Disconnect
       │
       ▼
running.store(false)
       │
       ├───▶ TUN 读线程：receive_blocking() 循环退出
       │
       ├───▶ TUN 写线程：blocking_recv() 返回 None
       │
       ├───▶ tokio 任务：检测 running=false，退出
       │
       └───▶ adapter 保活线程：
              while running → 退出
              drop(adapter)
                    │
                    ▼
              wintun 销毁 TUN 适配器
                    │
                    ▼
              Windows 内核自动删除：
              - 0.0.0.0/1 via TUN 路由
              - 128.0.0.0/1 via TUN 路由
              - TUN 网络接口
              - 注册表中的 DNS 配置
                    │
                    ▼
              系统恢复到 VPN 启动前的状态
              网络正常（无残留）
```
