# 第三章：Rust FFI 桥接层

---

## 3.1 FFI 设计原则

FFI（Foreign Function Interface）是 Rust 与其他语言互操作的机制。本项目中，Swift 代码需要调用 Rust 编写的隧道逻辑，中间通过 C ABI 桥接。

### 为什么必须走 C ABI

Swift 不能直接调用 Rust 函数。两者的 ABI（Application Binary Interface）完全不同——函数名修饰规则、参数传递方式、返回值处理都不兼容。但 Swift 和 Rust **都能**与 C 互操作：

```
Swift ←──Objective-C Bridging──→ C ABI ←──extern "C"──→ Rust
```

所以我们的策略是：Rust 函数以 C 的方式导出，Swift 通过 Bridging Header 看到 C 函数声明，然后直接调用。

### 三个关键注解

```rust
#[no_mangle]                    // ① 禁止 Rust 编译器修改函数名
pub unsafe extern "C" fn        // ② 使用 C 调用约定  ③ 标记为 unsafe
tunnel_start(socks5_addr: *const c_char) -> bool {
    // ...
}
```

#### ① `#[no_mangle]`

Rust 编译器默认会对函数名做 **name mangling**（名称修饰），将 `tunnel_start` 变成类似 `_ZN11tunnel_core3ffi12tunnel_start17h8a3b4c5d6e7f8g9hE` 这样的符号。这对链接器来说是不可识别的。

`#[no_mangle]` 告诉编译器保持原始函数名，这样链接器能在 `.a` 静态库中找到 `tunnel_start` 符号，Swift 也能通过 C 头文件的声明正确调用它。

#### ② `extern "C"`

指定使用 **C 调用约定**（cdecl）：
- 参数如何通过寄存器/栈传递
- 返回值如何传递
- 调用者/被调用者谁负责清理栈

如果不写 `extern "C"`，Rust 使用自己的调用约定，Swift 调用时参数会错位，导致崩溃或数据损坏。

#### ③ `unsafe`

FFI 函数本质上是不安全的，因为：
- Rust 无法验证 C/Swift 传入的指针是否有效
- Rust 无法保证调用者遵守函数的使用约束
- 原始指针（`*const c_char`）没有 Rust 的借用检查保护

将函数标记为 `unsafe` 是诚实地告诉使用者："你需要自己确保参数合法。"

### FFI 数据类型映射

| Rust 类型 | C 类型 | Swift 类型 | 用途 |
|---|---|---|---|
| `*const c_char` | `const char *` | `UnsafePointer<CChar>` | 字符串传入 |
| `*const u8` | `const unsigned char *` | `UnsafePointer<UInt8>` | 字节数组传入 |
| `*mut u8` | `unsigned char *` | `UnsafeMutablePointer<UInt8>` | 字节数组传出 |
| `usize` | `size_t` | `Int` | 长度/大小 |
| `bool` | `bool` | `Bool` | 布尔值 |
| `i32` | `int32_t` | `Int32` | 返回码 |

---

## 3.2 C 头文件定义

`tunnel_core.h` 是 Swift 和 Rust 之间的"契约"。让我们逐行分析：

```c
#ifndef TUNNEL_CORE_H          // 头文件保护宏，防止重复包含
#define TUNNEL_CORE_H

#include <stdbool.h>           // bool 类型（C99 引入）
#include <stddef.h>            // size_t 类型
#include <stdint.h>            // int32_t 等定宽整数类型

// 启动隧道。传入 SOCKS5 代理地址（如 "192.168.1.100:1080"）
// 返回 true 表示成功
bool tunnel_start(const char *socks5_addr);

// 停止隧道。设置 running 标志为 false
void tunnel_stop(void);

// 从 Swift 向 Rust 喂入一个 IP 数据包（TUN 读到的包）
// data: 数据包指针, len: 数据包长度
// 返回 true 表示成功入队
bool tunnel_feed_packet(const unsigned char *data, size_t len);

// 从 Rust 读取一个待写回 TUN 的 IP 数据包
// buf: 缓冲区指针, buf_len: 缓冲区大小
// 返回实际写入的字节数，0 表示没有数据
size_t tunnel_read_packet(unsigned char *buf, size_t buf_len);

// 检查隧道是否正在运行
bool tunnel_is_running(void);

/// 测试 SOCKS5 连通性。返回 1 成功，0 失败。
/// 结果信息写入 out_buf（以 null 终止）。
int32_t tunnel_test_socks5(const char *socks5_addr,
                           unsigned char *out_buf,
                           size_t out_len);

#endif /* TUNNEL_CORE_H */
```

### 设计要点

**最小化接口面**：只暴露 6 个函数，覆盖隧道的完整生命周期：

```
tunnel_start()         → 创建
tunnel_feed_packet()   → 写入数据
tunnel_read_packet()   → 读取数据
tunnel_is_running()    → 查询状态
tunnel_stop()          → 销毁
tunnel_test_socks5()   → 诊断工具
```

**不传递复杂结构体**：所有参数都是基本类型或原始指针。这避免了跨语言的内存布局兼容问题。如果需要传配置，用字符串（如 JSON 或简单的 `"ip:port"` 格式），而不是定义跨语言结构体。

**返回值语义清晰**：`bool` 表示成功/失败，`size_t` 表示实际长度（0 表示无数据），`int32_t` 表示状态码。

---

## 3.3 tunnel_start：初始化运行时和网络栈

`tunnel_start` 是整个隧道的启动入口。让我们逐段分析：

### 全局状态定义

```rust
// 隧道运行时所需的所有状态，打包成一个结构体
struct TunnelRuntime {
    runtime: Runtime,                              // tokio 异步运行时
    running: Arc<AtomicBool>,                      // 运行标志（原子布尔，线程安全）
    inbound_tx: mpsc::Sender<Vec<u8>>,             // Swift → Rust 的数据通道（发送端）
    outbound_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,  // Rust → Swift 的数据通道（接收端）
}

// 全局单例。OnceCell 确保只能初始化一次。
static TUNNEL: OnceCell<TunnelRuntime> = OnceCell::new();
```

为什么用 `OnceCell` 而不是 `static mut`？

- `static mut` 在 Rust 中被认为是 **unsound**（不安全的），每次访问都需要 `unsafe`
- `OnceCell` 保证只能写入一次（`set` 只有第一次调用成功），之后只读
- 读取（`get`）是无锁的，性能极好

### 函数入口：参数校验

```rust
#[no_mangle]
pub unsafe extern "C" fn tunnel_start(socks5_addr: *const c_char) -> bool {
    // 初始化 iOS 系统日志。传入 subsystem 标识符，
    // 在 Console.app 中可以用这个标识符过滤日志。
    let _ = oslog::OsLogger::new("com.vpndemo.app.tunnel.rust")
        .level_filter(log::LevelFilter::Debug)
        .init();

    info!("tunnel_start called");

    // 空指针检查——C 世界没有 Option<T>，
    // 调用者可能传入 NULL
    if socks5_addr.is_null() {
        error!("socks5_addr is null");
        return false;
    }

    // 将 C 字符串转换为 Rust &str
    // CStr::from_ptr 从原始指针创建 CStr（找到 null 终止符）
    // to_str() 检查是否是合法的 UTF-8
    let addr_str = match CStr::from_ptr(socks5_addr).to_str() {
        Ok(s) => s,
        Err(_) => return false,    // 非 UTF-8 字符串，返回失败
    };

    // 解析为 SocketAddr（如 "192.168.1.100:1080"）
    let socket_addr: SocketAddr = match addr_str.parse() {
        Ok(a) => a,
        Err(_) => return false,    // 格式不对，返回失败
    };

    let config = TunnelConfig {
        socks5_addr: socket_addr,
    };
```

注意防御式编程风格——每一步都可能失败，都返回 `false` 而不是 panic。**FFI 函数绝不能 panic**，因为 panic 跨越 FFI 边界是未定义行为（Undefined Behavior）。

### 创建 tokio Runtime

```rust
    // 创建一个多线程的 tokio 运行时
    // Runtime::new() 默认创建与 CPU 核心数相同的工作线程
    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return false,
    };
```

为什么在 FFI 函数中**创建** Runtime，而不是用 `#[tokio::main]`？

- `#[tokio::main]` 只能用在 `async fn main()` 上
- FFI 函数是同步的 `extern "C" fn`，不能是 async
- 我们需要手动控制 Runtime 的生命周期（创建、存储、后续 spawn 任务）

### 创建通信通道

```rust
    let running = Arc::new(AtomicBool::new(true));

    // Swift → Rust 方向：Swift 调用 tunnel_feed_packet 写入，Rust 内部消费
    let (inbound_tx, inbound_rx) = mpsc::channel::<Vec<u8>>(1024);

    // Rust → Swift 方向：Rust 内部产出，Swift 调用 tunnel_read_packet 读取
    let (outbound_tx, outbound_rx) = mpsc::channel::<Vec<u8>>(1024);
```

通道容量为 1024：这意味着最多缓存 1024 个数据包。如果消费者跟不上生产者，新的包会被丢弃（`try_send`）或阻塞（`send`）——在网络应用中丢包是可接受的。

### 启动异步任务

```rust
    let running_clone = running.clone();

    // 在 tokio Runtime 中 spawn 一个异步任务
    // 注意：spawn 是非阻塞的，它只是将任务提交给运行时
    runtime.spawn(async move {
        if let Err(e) = run_stack(config, inbound_rx, outbound_tx, running_clone).await {
            error!("Stack error: {}", e);
        }
    });
```

`runtime.spawn()` 而不是 `runtime.block_on()`——这是关键设计决策。`block_on()` 会阻塞当前线程直到 Future 完成，而 `tunnel_start` 需要**立即返回**给 Swift。网络栈在后台异步运行。

### 存储全局状态

```rust
    // 将所有状态存入全局 OnceCell
    let _ = TUNNEL.set(TunnelRuntime {
        runtime,
        running,
        inbound_tx,
        outbound_rx: Arc::new(Mutex::new(outbound_rx)),
    });

    true   // 返回成功
}
```

`TUNNEL.set()` 的返回值被忽略（`let _ =`）。如果 `TUNNEL` 已经被设置过（比如重复调用 `tunnel_start`），`set` 会返回 `Err`，但我们选择忽略它。当前设计中，`tunnel_start` 只应被调用一次。

### run_stack：核心异步逻辑

`run_stack` 是 `tunnel_start` 内 spawn 的异步函数，负责构建网络栈并启动所有转发任务：

```rust
async fn run_stack(
    config: TunnelConfig,
    mut inbound_rx: mpsc::Receiver<Vec<u8>>,  // 从 Swift 接收数据包
    outbound_tx: mpsc::Sender<Vec<u8>>,        // 向 Swift 发送数据包
    running: Arc<AtomicBool>,                   // 运行控制标志
) -> std::io::Result<()> {
    info!("Building netstack, SOCKS5 proxy: {}", config.socks5_addr);

    // 使用 Builder 模式创建 netstack-smoltcp 网络栈
    let (stack, runner, udp_socket, tcp_listener) = StackBuilder::default()
        .enable_tcp(true)       // 启用 TCP 处理
        .enable_udp(true)       // 启用 UDP 处理
        .enable_icmp(true)      // 启用 ICMP（ping 等）
        .tcp_buffer_size(65535) // TCP 收发缓冲区 64KB
        .udp_buffer_size(65535) // UDP 收发缓冲区 64KB
        .stack_buffer_size(1024) // 内部 IP 包队列长度
        .build()?;              // 构建，失败返回 io::Error
```

`build()` 返回四个组件：
1. **`stack`**：核心的 IP 包收发通道（实现了 `Sink` + `Stream`）
2. **`runner`**：内部事件驱动器，必须被 spawn
3. **`udp_socket`**：UDP 数据报接口
4. **`tcp_listener`**：TCP 连接监听器

这四个组件的协作关系将在第四章详细分析。

---

## 3.4 tunnel_feed_packet / tunnel_read_packet：数据包交换机制

这两个函数是 Swift 与 Rust 之间交换 IP 数据包的核心接口。它们被 Swift 端高频调用——每收到一个 IP 包就调用一次 `tunnel_feed_packet`，每个 Timer tick 就调用一次 `tunnel_read_packet`。

### tunnel_feed_packet：Swift → Rust

```rust
#[no_mangle]
pub unsafe extern "C" fn tunnel_feed_packet(data: *const u8, len: usize) -> bool {
    // 防御性检查：空指针或零长度
    if data.is_null() || len == 0 {
        return false;
    }

    // 从原始指针创建一个字节切片，然后复制为 Vec<u8>
    // 注意：to_vec() 会分配新内存并复制数据
    // 这是必要的——data 指针的生命周期由 Swift 控制，
    // 我们不能在异步任务中持有它
    let packet = std::slice::from_raw_parts(data, len).to_vec();

    if let Some(tunnel) = TUNNEL.get() {
        // try_send 是非阻塞的：
        // - 通道未满 → 发送成功，返回 Ok
        // - 通道已满 → 立即返回 Err（不等待）
        tunnel.inbound_tx.try_send(packet).is_ok()
    } else {
        false   // TUNNEL 未初始化
    }
}
```

#### 为什么用 `try_send` 而不是 `send`

`send` 是异步方法（`async fn`），会在通道满时等待。但 `tunnel_feed_packet` 是同步的 `extern "C"` 函数，**不能** `.await`。

`try_send` 是同步方法，立即返回结果：
- 通道未满：数据入队，返回 `Ok(())`
- 通道已满：数据丢弃，返回 `Err(TrySendError::Full(...))`

在 VPN 场景中，如果处理速度跟不上来包速度，丢弃一些包是完全可接受的——TCP 有重传机制，UDP 本身就是不可靠的。

#### 数据复制的必要性

```rust
let packet = std::slice::from_raw_parts(data, len).to_vec();
```

这行代码执行了一次内存复制。为什么不能直接传指针？

1. `data` 指针指向 Swift 管理的内存，Swift 随时可能释放它
2. 数据会被发送到 mpsc channel，由另一个线程的异步任务消费
3. Rust 的所有权系统要求 `Vec<u8>` 拥有自己的数据

### tunnel_read_packet：Rust → Swift

```rust
#[no_mangle]
pub unsafe extern "C" fn tunnel_read_packet(buf: *mut u8, buf_len: usize) -> usize {
    // 防御性检查
    if buf.is_null() || buf_len == 0 {
        return 0;
    }

    if let Some(tunnel) = TUNNEL.get() {
        // 加锁获取 Receiver 的独占访问权
        // parking_lot::Mutex 比 std::sync::Mutex 更轻量
        let mut rx = tunnel.outbound_rx.lock();

        // try_recv 是非阻塞的：
        // - 有数据 → 返回 Ok(packet)
        // - 无数据 → 返回 Err(TryRecvError::Empty)
        // - 通道关闭 → 返回 Err(TryRecvError::Disconnected)
        match rx.try_recv() {
            Ok(packet) => {
                // 取实际包长和缓冲区大小的较小值，防止越界写入
                let copy_len = packet.len().min(buf_len);
                // 将数据从 Rust 的 Vec 复制到 Swift 提供的缓冲区
                std::ptr::copy_nonoverlapping(packet.as_ptr(), buf, copy_len);
                copy_len   // 返回实际写入的字节数
            }
            Err(_) => 0,   // 没有数据包可读
        }
    } else {
        0   // TUNNEL 未初始化
    }
}
```

#### 为什么 outbound_rx 需要 Mutex

`mpsc::Receiver` 不是 `Sync` 的——它不能被多个线程同时访问。虽然在当前设计中，`tunnel_read_packet` 可能只在一个线程（Swift 的 Timer 回调线程）中被调用，但 Rust 的类型系统要求 `static` 变量中的所有字段必须是 `Send + Sync` 的。

`Arc<Mutex<Receiver>>` 满足了 `Send + Sync` 约束，同时保证了即使从多个线程调用也是安全的。

#### 为什么用 `try_recv` 而不是 `recv`

和 `tunnel_feed_packet` 中用 `try_send` 的原因一样——这是同步 FFI 函数，不能阻塞等待。Swift 端通过 Timer 轮询调用 `tunnel_read_packet`，如果没有数据就返回 0，Swift 下一次 tick 再试。

### 数据流向总结

```
Swift (PacketTunnelProvider)              Rust (tunnel-core)

packetFlow.readPackets()
    │ 收到 IP 数据包
    ▼
tunnel_feed_packet(data, len)  ──►  try_send(packet) ──►  mpsc channel
                                                              │
                                                     inbound_rx.recv()
                                                              │
                                                     Stack.send(pkt)
                                                              │
                                                    [netstack 处理]
                                                              │
                                                     Stack.next()
                                                              │
                                                     outbound_tx.send()
                                                              │
                                                         mpsc channel
                                                              │
tunnel_read_packet(buf, len)   ◄──  try_recv()      ◄────────┘
    │
    ▼
packetFlow.writePackets()
```

---

## 3.5 tunnel_stop 和 tunnel_is_running

### tunnel_stop：优雅关闭

```rust
#[no_mangle]
pub extern "C" fn tunnel_stop() {
    if let Some(tunnel) = TUNNEL.get() {
        // 将 running 标志设为 false
        // SeqCst（Sequentially Consistent）是最严格的内存序
        // 确保所有线程立即看到这个写入
        tunnel.running.store(false, Ordering::SeqCst);
        info!("tunnel_stop: signaled stop");
    }
}
```

注意 `tunnel_stop` 的特点：

1. **没有 `unsafe`**：它不接收任何原始指针，所有操作都是安全的
2. **非阻塞**：只设置一个标志位，不等待实际停止
3. **信号式关闭**：各个异步任务在自己的循环中检查 `running` 标志，发现为 `false` 后自行退出

这种模式叫做 **cooperative cancellation**（协作式取消）。比起强制终止线程，它让每个任务有机会清理自己的资源。

### tunnel_is_running：状态查询

```rust
#[no_mangle]
pub extern "C" fn tunnel_is_running() -> bool {
    TUNNEL.get()
        .map(|t| t.running.load(Ordering::SeqCst))  // 读取原子布尔值
        .unwrap_or(false)                             // TUNNEL 未初始化则返回 false
}
```

Swift 端可以用这个函数轮询隧道状态，决定 UI 显示。

---

## 3.6 内存安全与线程模型

FFI 是 Rust 安全模型的"裂缝"——跨越这个边界后，Rust 的借用检查器无法保护我们。本项目通过精心的设计来最大限度地减少 unsafe 代码的范围和风险。

### OnceCell：安全的全局状态

```rust
static TUNNEL: OnceCell<TunnelRuntime> = OnceCell::new();
```

`OnceCell` 的安全保证：
- **只能写入一次**：`set()` 在已初始化后返回 `Err`，不会覆盖
- **读取无锁**：`get()` 返回 `Option<&T>`，没有 Mutex 开销
- **线程安全**：`sync::OnceCell` 实现了 `Send + Sync`
- **内存顺序正确**：内部使用适当的原子操作确保跨线程可见性

相比其他方案：

| 方案 | 安全性 | 性能 | 问题 |
|---|---|---|---|
| `static mut` | 不安全 | 快 | 每次访问都需 unsafe，数据竞争 |
| `lazy_static! + Mutex` | 安全 | 有锁开销 | 每次读取都要加锁 |
| `OnceCell` | 安全 | 无锁读取 | 只能写入一次（正好符合需求） |

### Arc<AtomicBool>：跨线程的运行标志

```rust
running: Arc<AtomicBool>
```

`running` 需要被多个地方访问：
1. `tunnel_stop()` —— 写入 `false`（在 Swift 调用线程）
2. `tunnel_is_running()` —— 读取（在 Swift 调用线程）
3. `run_stack` 中的各个 `tokio::spawn` 任务 —— 读取（在 tokio 工作线程）

`Arc`（Atomic Reference Counting）允许多个所有者共享同一份数据。`AtomicBool` 保证读写操作在多线程环境下是原子的，不需要额外加锁。

```rust
// 写入（tunnel_stop）
tunnel.running.store(false, Ordering::SeqCst);

// 读取（各异步任务的循环条件）
while running.load(Ordering::SeqCst) {
    // ...
}
```

### Mutex 的使用范围

整个项目只有一处使用了 `Mutex`：

```rust
outbound_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>
```

使用 `parking_lot::Mutex` 而不是 `std::sync::Mutex`，原因：

| | `std::sync::Mutex` | `parking_lot::Mutex` |
|---|---|---|
| 锁中毒（poison） | 会（panic 后锁不可用） | 不会 |
| API 便利性 | `lock().unwrap()` | `lock()`（直接返回 Guard） |
| 性能 | 较好 | 更好（尤其在无竞争场景） |
| 大小 | 较大（含 poison 标志） | 更小 |

`parking_lot::Mutex` 在 FFI 场景中更合适——不用担心 panic 导致锁中毒的问题。

### 为什么 tunnel_start 中不用 block_on

```rust
// ✅ 正确做法：spawn 后立即返回
runtime.spawn(async move {
    run_stack(config, inbound_rx, outbound_tx, running_clone).await
});
return true;

// ❌ 错误做法：block_on 会阻塞调用线程
runtime.block_on(async move {
    run_stack(...).await    // 永远不会返回，因为 run_stack 是无限循环
});
return true;               // 永远执行不到这里
```

`block_on` 会阻塞当前线程直到 Future 完成。而 `run_stack` 内部是个无限循环（持续监听 TCP 连接），永远不会完成。如果用 `block_on`，`tunnel_start` 就永远不会返回，Swift 的 `startTunnel` 回调会超时。

### unsafe 代码的边界控制

项目中 `unsafe` 代码的范围被严格限制：

```rust
// tunnel_feed_packet 中的 unsafe 操作：
let packet = std::slice::from_raw_parts(data, len).to_vec();
//           ^^^^^^^^^^^^^^^^^^^^^^^^^^
//           唯一的 unsafe 操作：从原始指针创建切片
//           to_vec() 之后，数据就完全由 Rust 管理了

// tunnel_read_packet 中的 unsafe 操作：
std::ptr::copy_nonoverlapping(packet.as_ptr(), buf, copy_len);
//         ^^^^^^^^^^^^^^^^^^^^
//         唯一的 unsafe 操作：写入 Swift 提供的缓冲区

// 其余所有代码（channel 操作、网络处理、SOCKS5 转发）都是安全的 Rust
```

原则：**尽快将 unsafe 数据转换为安全的 Rust 类型**。`from_raw_parts().to_vec()` 这个模式将原始指针立即转为拥有所有权的 `Vec<u8>`，之后就回到了 Rust 安全世界。

### 线程模型总览

```
┌────────────────────┐
│  Swift 调用线程      │  tunnel_start / tunnel_stop / tunnel_is_running
│  (iOS 主线程或       │  tunnel_feed_packet / tunnel_read_packet
│   NE 回调线程)       │
└────────┬───────────┘
         │ mpsc channel（跨线程通信）
         ▼
┌─────────────────────────────────────────────┐
│  tokio Runtime（多线程工作池）                  │
│                                              │
│  Task 1: Stack 驱动器（send + next 循环）      │
│  Task 2: inbound 转发（inbound_rx → stack）   │
│  Task 3: outbound 转发（stack → outbound_tx） │
│  Task 4: TCP 监听 + 每连接 spawn SOCKS5 中继  │
│  Task 5: UDP 转发器                           │
│  Task 6: Runner（smoltcp 内部事件循环）         │
└─────────────────────────────────────────────┘
```

Swift 线程和 tokio 线程之间唯一的通信方式是 mpsc channel——没有共享的可变状态（除了 `AtomicBool` 标志），不存在死锁的可能。

---

## 3.7 tunnel_test_socks5：测试函数设计

`tunnel_test_socks5` 是一个独立的诊断工具，不依赖隧道的运行状态，用于在主 App 中直接测试 SOCKS5 代理是否可用。

### 函数签名

```rust
#[no_mangle]
pub unsafe extern "C" fn tunnel_test_socks5(
    socks5_addr: *const c_char,   // SOCKS5 地址（C 字符串）
    out_buf: *mut u8,              // 输出缓冲区（用于返回结果消息）
    out_len: usize,                // 缓冲区大小
) -> i32 {                         // 1=成功, 0=失败
```

这个设计体现了 FFI 中返回字符串的经典模式：**调用者分配缓冲区，被调用者写入内容**。Swift 端预先分配一块内存，传入指针和长度，Rust 将结果字符串写入其中。

### 为什么这里可以用 block_on

```rust
    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => return write_result(out_buf, out_len, &format!("runtime error: {}", e)),
    };

    // 这里用 block_on 是正确的！
    let result = rt.block_on(async {
        test_socks5_impl(addr_str).await
    });
```

与 `tunnel_start` 不同，`tunnel_test_socks5` 是一个**一次性**操作：发起测试、等待结果、返回。`block_on` 会阻塞直到测试完成，这正是我们需要的行为。测试函数创建自己的临时 Runtime，用完即弃。

### 三步测试逻辑

```rust
async fn test_socks5_impl(socks5_addr: &str) -> Result<String, String> {
    // 第 1 步：测试 TCP 连通性（能不能连上 SOCKS5 服务器）
    let tcp_result = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        tokio::net::TcpStream::connect(socks5_addr),
    ).await;
    // ...

    // 第 2 步：测试 SOCKS5 握手（代理协议是否正常工作）
    let stream_result = tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        Socks5Stream::connect(
            socks5_addr.to_string(),
            "httpbin.org".to_string(),
            80,
            Socks5Config::default(),
        ),
    ).await;
    // ...

    // 第 3 步：通过 SOCKS5 发送 HTTP 请求（端到端验证）
    let req = "GET /ip HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
    stream.write_all(req.as_bytes()).await;
    // ...
}
```

每一步都有超时保护（5~10 秒），避免网络问题导致函数永远不返回。

### write_result：安全写入 C 缓冲区

```rust
unsafe fn write_result(buf: *mut u8, len: usize, msg: &str) -> i32 {
    if buf.is_null() || len == 0 {
        return 0;
    }
    let bytes = msg.as_bytes();
    let copy_len = bytes.len().min(len - 1);  // 预留 1 字节给 null 终止符
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, copy_len);
    *buf.add(copy_len) = 0;  // 写入 null 终止符，使其成为合法的 C 字符串
    0
}
```

关键细节：
- `len - 1`：总是给 null 终止符留位置
- `bytes.len().min(len - 1)`：防止消息过长导致缓冲区溢出
- `*buf.add(copy_len) = 0`：手动添加 C 字符串的 null 终止符

这是 C 世界中"写入调用者缓冲区"的标准模式，在 Rust FFI 中经常会用到。
