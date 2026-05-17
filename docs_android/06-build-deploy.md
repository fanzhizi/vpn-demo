# 第六章：构建、调试与部署

---

## 6.1 环境准备

### 必需工具

| 工具 | 版本要求 | 用途 |
|------|---------|------|
| Android Studio | 2023+ | IDE + SDK Manager |
| Android SDK | API 34 | 编译 Java 代码 |
| Android NDK | r25+ | 提供交叉编译工具链 |
| Rust (rustup) | 1.70+ | 编译 tunnel-core |
| cargo-ndk | 3.0+ | 简化 Android 交叉编译 |
| adb | SDK 自带 | 安装和调试 |

### 安装步骤

```bash
# 1. 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 添加 Android 目标架构
rustup target add aarch64-linux-android    # arm64 (主要)
rustup target add armv7-linux-androideabi  # arm32 (可选)
rustup target add x86_64-linux-android     # 模拟器 (可选)

# 3. 安装 cargo-ndk
cargo install cargo-ndk

# 4. 设置 NDK 路径 (macOS 示例)
export ANDROID_NDK_HOME=$HOME/Library/Android/sdk/ndk/25.2.9519653
# 或者在 ~/.cargo/config.toml 中配置：
# [env]
# ANDROID_NDK_HOME = "/Users/xxx/Library/Android/sdk/ndk/25.2.9519653"
```

### NDK 安装

通过 Android Studio 的 SDK Manager 安装：
- Settings → Languages & Frameworks → Android SDK → SDK Tools
- 勾选 "NDK (Side by side)" 安装

---

## 6.2 完整构建步骤

### 构建 Rust .so 库

```bash
# 进入 tunnel-core 目录
cd /path/to/vpn-demo/tunnel-core

# 编译 arm64 版本（Release 模式，体积更小）
cargo ndk -t arm64-v8a build --release

# 编译产物位置：
# target/aarch64-linux-android/release/libtunnel_core.so

# 复制到 Android 项目的 jniLibs 目录
cp target/aarch64-linux-android/release/libtunnel_core.so \
   ../android/app/src/main/jniLibs/arm64-v8a/
```

### 构建 APK

```bash
# 进入 Android 项目目录
cd /path/to/vpn-demo/android

# 构建 Debug APK
./gradlew assembleDebug

# APK 位置：
# app/build/outputs/apk/debug/app-debug.apk

# 或者构建 Release APK
./gradlew assembleRelease
```

### 一键构建脚本

```bash
#!/bin/bash
# build-android.sh

set -e

echo "=== Building Rust library ==="
cd tunnel-core
cargo ndk -t arm64-v8a build --release
cp target/aarch64-linux-android/release/libtunnel_core.so \
   ../android/app/src/main/jniLibs/arm64-v8a/

echo "=== Building APK ==="
cd ../android
./gradlew assembleDebug

echo "=== Done ==="
echo "APK: android/app/build/outputs/apk/debug/app-debug.apk"
```

---

## 6.3 安装与调试

### adb 安装

```bash
# 连接设备（USB 或 WiFi）
adb devices

# 安装 APK
adb install -r app/build/outputs/apk/debug/app-debug.apk

# 如果已安装旧版本，用 -r 覆盖安装
# 如果签名不同，先卸载：adb uninstall com.vpndemo.app
```

### logcat 日志查看

```bash
# 查看所有 VPN 相关日志
adb logcat -s TunnelVpnService tunnel-core VPNDemo

# 只看 Rust 层日志
adb logcat -s tunnel-core

# 只看 protect 相关
adb logcat | grep -i protect

# 清除旧日志后查看
adb logcat -c && adb logcat -s tunnel-core TunnelVpnService

# 保存日志到文件
adb logcat -s tunnel-core > vpn-debug.log
```

### 典型成功日志

```
I/TunnelVpnService: Starting VPN, SOCKS5: 192.168.31.209:1080
I/TunnelVpnService: Protect callback registered
I/tunnel-core: tunnel_start called
I/tunnel-core: Building netstack, SOCKS5 proxy: 192.168.31.209:1080
I/TunnelVpnService: Rust tunnel started
I/TunnelVpnService: TUN device established, fd=47
I/TunnelVpnService: VPN started successfully
D/TunnelVpnService: TUN read #1: 60 bytes
I/tunnel-core: UDP: 10.0.0.2:12345 -> 8.8.8.8:53 (45 bytes)
I/tunnel-core: protect_socket(fd=52) = true
I/tunnel-core: UDP reply from 8.8.8.8:53: 76 bytes
D/TunnelVpnService: TUN write #1: 96 bytes
I/tunnel-core: New TCP: 10.0.0.2:54321 -> 142.250.80.46:443
I/tunnel-core: protect_socket(fd=55) = true
I/tunnel-core: Connecting via SOCKS5 192.168.31.209:1080 to 142.250.80.46:443
I/tunnel-core: SOCKS5 tunnel established: -> 142.250.80.46:443
```

---

## 6.4 常见错误与解决

### 编译错误

| 错误 | 原因 | 解决 |
|------|------|------|
| `linker not found` | NDK 路径未设置 | 设置 ANDROID_NDK_HOME |
| `cannot find -llog` | NDK 版本不对 | 使用 r25+ |
| `undefined reference to __android_log_print` | 缺少链接标志 | cargo-ndk 会自动处理 |

### 运行时错误

| 错误 | 日志特征 | 解决 |
|------|---------|------|
| .so 加载失败 | `UnsatisfiedLinkError: dlopen failed` | 检查 jniLibs 目录和 ABI |
| native 方法找不到 | `No implementation found for...` | 检查函数命名是否匹配 |
| TUN 建立失败 | `Failed to establish TUN` | 用户拒绝了 VPN 权限 |
| SOCKS5 连接超时 | `TCP connect timeout` | 代理地址不可达 |
| 死循环/卡死 | 无日志输出 | protect 失败，检查回调注册顺序 |

### 调试死循环

```
症状：点击 Connect 后界面卡住，logcat 无新日志
原因：SOCKS5 连接未 protect，流量在 TUN 循环

排查步骤：
1. 检查 logcat 是否有 "protect_socket" 日志
   - 没有 → setProtectSocketCallback 未被调用
   - 有但返回 false → JVM/callback 未正确设置

2. 检查 tunnelStart 是否在 setProtectSocketCallback 之后
   （Java 层 startVpn() 中的调用顺序）

3. 临时添加日志：在 protect_socket() 入口打印
   adb logcat -s tunnel-core | grep protect
```

---

## 6.5 Debug vs Release 大小

### .so 文件大小对比

| 模式 | 大小 | 说明 |
|------|------|------|
| Debug | ~15 MB | 包含调试符号、未优化 |
| Release | ~4 MB | 优化后、strip 前 |
| Release + strip | ~2.5 MB | 移除符号表 |

### 减小体积的方法

```toml
# Cargo.toml 中添加 Release 优化配置
[profile.release]
opt-level = "z"        # 最小体积优化（比 "3" 更小）
lto = true             # 链接时优化（跨 crate 优化）
codegen-units = 1      # 单 codegen unit（更好的优化机会）
strip = true           # 自动 strip 符号表
panic = "abort"        # 不需要 panic 展开（减少代码）
```

```bash
# 手动 strip（如果 Cargo.toml 没配置）
$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-strip \
    target/aarch64-linux-android/release/libtunnel_core.so
```

### APK 最终大小估算

```
APK 构成：
  classes.dex (Java 代码)       ~50 KB
  libtunnel_core.so            ~2.5 MB (Release + strip)
  res/ (布局、资源)             ~10 KB
  AndroidManifest.xml           ~2 KB
  META-INF/ (签名)              ~5 KB
  ─────────────────────────────────────
  总计                          ~2.6 MB
```

---

## 6.6 持续开发工作流

### 快速迭代流程

```bash
# 修改 Rust 代码后：
cargo ndk -t arm64-v8a build --release && \
cp target/aarch64-linux-android/release/libtunnel_core.so \
   ../android/app/src/main/jniLibs/arm64-v8a/ && \
cd ../android && \
./gradlew installDebug && \
adb logcat -c && \
adb logcat -s tunnel-core TunnelVpnService

# 只修改 Java 代码后：
cd android && \
./gradlew installDebug

# 实时查看日志
adb logcat -s tunnel-core TunnelVpnService --color=always
```

### 使用 Android Studio

1. 用 Android Studio 打开 `android/` 目录
2. 修改 Java 代码后直接 Run（自动安装到设备）
3. Logcat 面板实时查看日志
4. 注意：修改 Rust 代码需要手动重新编译 .so

### WiFi 调试（无需 USB 线）

```bash
# Android 11+ 无线调试
# 1. 设备设置 → 开发者选项 → 无线调试 → 开启
# 2. 点击"使用配对码配对设备"
adb pair 192.168.31.100:37123  # 输入配对码
adb connect 192.168.31.100:42567

# 之后所有 adb 命令都走 WiFi
```
