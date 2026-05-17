# 第二章：项目结构

---

## 2.1 文件结构

```
vpn-demo/
├── windows-app/                    # Windows 平台代码
│   ├── Cargo.toml                  # 依赖配置（条件编译）
│   ├── build.rs                    # 构建脚本（Slint + winres）
│   ├── app.manifest                # UAC 管理员权限声明
│   ├── src/
│   │   ├── main.rs                 # 入口 + netstack + SOCKS5
│   │   └── tun.rs                  # TUN 设备管理 + Win32 API
│   └── installer/
│       └── installer.nsi           # NSIS 安装包脚本
│
├── vpn-app/
│   └── ui/
│       └── main.slint              # 共享 UI 定义
│
├── .github/workflows/
│   └── build-windows.yml           # Windows CI 配置
│
└── docs_windows/                   # 本文档
```

---

## 2.2 Cargo.toml 依赖分析

### 跨平台依赖

```toml
[dependencies]
slint = "1.9"                       # 跨平台 GUI（Slint 框架）
netstack-smoltcp = "0.2"            # 用户态 TCP/IP 协议栈
tokio = { version = "1", features = ["rt-multi-thread", "net", ...] }
futures = "0.3"                     # Stream/Sink trait
fast-socks5 = "1"                   # SOCKS5 客户端
log = "0.4"                         # 日志门面
env_logger = "0.11"                 # 日志实现
```

### Windows 条件依赖

```toml
[target.'cfg(windows)'.dependencies]
wintun = "0.5"                      # wintun.dll 的 Rust 封装
socket2 = "0.6"                     # 底层 socket 操作（bind 物理 IP）
winreg = "0.56"                     # 注册表操作（配置 DNS）
windows = { version = "0.62", features = [
    "Win32_NetworkManagement_IpHelper",   # GetBestRoute2, CreateIpForwardEntry2 等
    "Win32_NetworkManagement_Ndis",       # NET_LUID_LH
    "Win32_Networking_WinSock",           # SOCKADDR_INET, IN_ADDR 等
] }
```

### 条件构建依赖

```toml
[build-dependencies]
slint-build = "1.9"                 # 编译 .slint → Rust 代码

[target.'cfg(windows)'.build-dependencies]
winres = "0.1"                      # 嵌入 UAC manifest 到 exe
```

### cfg(windows) 的作用

```
cfg(windows) 依赖只在 Windows 编译时参与：

  ┌─────────────────────────────────────────────┐
  │  cargo build (在 Windows 上)                 │
  │                                             │
  │  编译: slint + netstack + tokio + ...       │
  │  编译: wintun + socket2 + winreg + windows  │  ← cfg(windows) 生效
  └─────────────────────────────────────────────┘

  ┌─────────────────────────────────────────────┐
  │  cargo build (在 macOS/Linux 上)             │
  │                                             │
  │  编译: slint + netstack + tokio + ...       │
  │  跳过: wintun + socket2 + winreg + windows  │  ← cfg(windows) 不生效
  │                                             │
  │  → 代码中 #[cfg(not(windows))] 分支生效     │
  │  → mock 模式运行（可测试 UI 和 netstack）    │
  └─────────────────────────────────────────────┘
```

---

## 2.3 共享 Slint UI

### build.rs 引用共享 UI

```rust
fn main() {
    // 引用上层目录的 Slint 文件
    slint_build::compile("../vpn-app/ui/main.slint").unwrap();
    // ...
}
```

`main.slint` 定义了跨平台共享的界面：
- SOCKS5 地址输入框
- 连接/断开按钮
- 状态指示灯
- 测试按钮

### UI 架构

```
vpn-app/ui/main.slint          ← 单一 UI 定义
       │
       ├─── iOS (Swift 调用)
       ├─── Android (Kotlin 调用)  
       └─── Windows (Rust 直接 include)
              │
              └── slint::include_modules!()
                  → MainWindow::new()
                  → window.on_connect_clicked(...)
                  → window.set_connected(...)
```

### Slint 的线程模型

```
主线程 (UI):
  window.run()                  ← 事件循环（阻塞）
  Timer 回调                    ← 每 500ms 轮询状态
  on_xxx_clicked 回调           ← 按钮点击

子线程 (VPN):
  start_vpn()                   ← VPN 核心逻辑
  tokio runtime                 ← async 任务

通信方式:
  Arc<AtomicBool> running       ← 跨线程状态同步
  Timer 轮询                    ← UI 更新（不能跨线程直接操作 Slint 组件）
```

---

## 2.4 交叉编译

### 从 macOS/Linux 编译 Windows 目标

```bash
# 安装交叉编译工具链
rustup target add x86_64-pc-windows-gnu
# macOS
brew install mingw-w64
# Ubuntu
sudo apt install gcc-mingw-w64-x86-64

# 编译
cargo build --target x86_64-pc-windows-gnu --release
```

### .cargo/config.toml 配置（如需要）

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

### 交叉编译 vs 原生编译

| 方面 | 交叉编译 (macOS → Windows) | 原生编译 (Windows) |
|------|--------------------------|-------------------|
| 工具链 | x86_64-pc-windows-gnu | x86_64-pc-windows-msvc |
| 链接器 | mingw-w64 (GNU) | MSVC link.exe |
| C 运行时 | msvcrt.dll (旧版) | UCRT (现代) |
| 调试 | 不方便 | Visual Studio 调试 |
| CI | 可在 Linux runner 编译 | 需要 Windows runner |
| 推荐场景 | CI 自动构建 | 本地开发调试 |

### GitHub Actions 使用原生编译

本项目的 CI 使用 `windows-latest` runner 原生编译：

```yaml
jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --package vpn-demo-windows --release
```

---

## 2.5 构建产物

```
target/release/
├── vpn-demo-windows.exe        # 主程序（~8MB Release）
└── (需要手动放置)
    └── wintun.dll              # wintun 驱动 DLL（~400KB）

两个文件必须在同一目录，exe 启动时 LoadLibrary("wintun.dll")
```
