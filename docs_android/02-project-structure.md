# 第二章：Android 项目结构

---

## 2.1 目录结构总览

```
android/
├── app/
│   ├── build.gradle                          ← Gradle 构建配置
│   ├── src/main/
│   │   ├── AndroidManifest.xml               ← 应用清单（权限、组件声明）
│   │   ├── java/com/vpndemo/app/
│   │   │   ├── MainActivity.java             ← UI 界面
│   │   │   ├── TunnelVpnService.java         ← VPN Service 核心
│   │   │   └── TunnelCore.java               ← JNI 桥接类
│   │   ├── res/layout/
│   │   │   └── activity_main.xml             ← 布局文件
│   │   └── jniLibs/
│   │       └── arm64-v8a/
│   │           └── libtunnel_core.so         ← Rust 编译产物
│   └── ...
├── build.gradle                              ← 项目级 Gradle 配置
├── settings.gradle
└── gradle/
    └── wrapper/
        └── gradle-wrapper.properties

tunnel-core/                                   ← Rust 核心库（跨平台共享）
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── ffi.rs                                ← C FFI（iOS/Android 共用）
│   └── jni_ffi.rs                            ← JNI 包装（Android 专用）
```

---

## 2.2 Gradle 配置详解

### app/build.gradle

```groovy
apply plugin: 'com.android.application'

android {
    namespace 'com.vpndemo.app'   // 包名，用于 R 类和 AndroidManifest 合并
    compileSdk 34                  // 编译时使用的 SDK 版本

    defaultConfig {
        applicationId "com.vpndemo.app"  // APK 的唯一标识
        minSdk 24                         // 最低支持 Android 7.0
        targetSdk 34                      // 目标 SDK 版本
        versionCode 1
        versionName "1.0"

        ndk {
            // 只编译 arm64 架构（现代手机都是 arm64）
            // 如需支持模拟器，添加 'x86_64'
            abiFilters 'arm64-v8a'
        }
    }

    buildTypes {
        release {
            minifyEnabled false  // 不混淆（JNI 方法名不能被混淆！）
        }
    }

    compileOptions {
        sourceCompatibility JavaVersion.VERSION_1_8
        targetCompatibility JavaVersion.VERSION_1_8
    }
}

dependencies {
    // 本项目无外部 Java 依赖
    // 所有网络逻辑在 Rust 侧实现
}
```

### 关键配置说明

| 配置项 | 值 | 为什么 |
|--------|-----|--------|
| `minSdk 24` | Android 7.0 | VpnService 在 API 14 就有，但 24 是合理的最低线 |
| `abiFilters 'arm64-v8a'` | 只支持 64 位 ARM | 减小 APK 体积，现代设备都是 arm64 |
| `minifyEnabled false` | 不开 ProGuard/R8 | JNI native 方法名必须精确匹配，混淆会破坏 |

### 为什么不用 minifyEnabled

```
Java 侧声明:  native boolean tunnelStart(String socks5Addr);
               ↓ JNI 命名规则
Rust 侧导出:  Java_com_vpndemo_app_TunnelCore_tunnelStart

如果开启 ProGuard 混淆:
  TunnelCore → 被重命名为 a
  tunnelStart → 被重命名为 b
  
Rust 侧仍然导出 Java_com_vpndemo_app_TunnelCore_tunnelStart
系统找不到匹配 → UnsatisfiedLinkError!
```

如果必须开启混淆，需要在 `proguard-rules.pro` 中添加：

```
-keep class com.vpndemo.app.TunnelCore { *; }
-keep class com.vpndemo.app.TunnelVpnService { *; }
```

---

## 2.3 AndroidManifest.xml 解析

```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <!-- 网络权限：SOCKS5 代理连接需要 -->
    <uses-permission android:name="android.permission.INTERNET" />
    <!-- 前台服务权限：Android 9+ VPN Service 需要 -->
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE" />

    <application
        android:allowBackup="false"
        android:label="VPN Demo"
        android:theme="@android:style/Theme.Light.NoTitleBar"
        android:supportsRtl="true">

        <!-- 主界面 Activity -->
        <activity
            android:name=".MainActivity"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>

        <!-- VPN Service 声明 -->
        <service
            android:name=".TunnelVpnService"
            <!--
              BIND_VPN_SERVICE 权限：只有系统能绑定此 Service
              这防止其他应用恶意控制 VPN
            -->
            android:permission="android.permission.BIND_VPN_SERVICE"
            android:exported="false">
            <intent-filter>
                <!-- 这个 action 让系统识别这是一个 VPN Service -->
                <action android:name="android.net.VpnService" />
            </intent-filter>
        </service>

    </application>
</manifest>
```

### Service 声明的关键点

| 属性 | 值 | 作用 |
|------|-----|------|
| `android:permission` | `BIND_VPN_SERVICE` | 系统级保护，防止非系统组件绑定 |
| `android:exported` | `false` | 不暴露给其他应用 |
| `intent-filter action` | `android.net.VpnService` | 系统识别标记（必须有） |

---

## 2.4 jniLibs 目录

```
app/src/main/jniLibs/
└── arm64-v8a/
    └── libtunnel_core.so    ← Rust 编译产物
```

### 命名规则

| Java 代码 | 文件名 | 说明 |
|-----------|--------|------|
| `System.loadLibrary("tunnel_core")` | `libtunnel_core.so` | 系统自动加前缀 `lib` 和后缀 `.so` |

### ABI 架构说明

| ABI | 设备 | 说明 |
|-----|------|------|
| `arm64-v8a` | 现代手机/平板 | 64 位 ARM，必须支持 |
| `armeabi-v7a` | 旧手机 | 32 位 ARM，可选 |
| `x86_64` | 模拟器 | 开发调试用 |
| `x86` | 旧模拟器 | 基本不需要 |

本项目只编译 `arm64-v8a`，因为：
- 2024 年起 Google Play 强制要求 64 位
- 减少编译时间和 APK 体积
- 开发时可用真机调试

---

## 2.5 cargo-ndk 交叉编译

### 为什么需要交叉编译

开发机通常是 x86_64 (macOS/Linux)，而 Android 手机是 ARM64。Rust 代码需要编译为目标架构的机器码。

### 工具链

```
┌─────────────────────────────────────────────────────────────────┐
│                        编译工具链                                 │
│                                                                  │
│  Rust 源码 (.rs)                                                 │
│       │                                                          │
│       ▼                                                          │
│  rustc + LLVM (交叉编译器)                                        │
│       │                                                          │
│       │  target: aarch64-linux-android                           │
│       │  linker: NDK 中的 aarch64-linux-android-clang            │
│       ▼                                                          │
│  libtunnel_core.so (ARM64 ELF 格式)                              │
│       │                                                          │
│       ▼                                                          │
│  复制到 app/src/main/jniLibs/arm64-v8a/                          │
└─────────────────────────────────────────────────────────────────┘
```

### Cargo.toml 中的关键配置

```toml
[lib]
crate-type = ["staticlib", "cdylib", "lib"]
#              │            │         │
#              │            │         └── Rust 库（用于测试）
#              │            └── 动态库 (.so)（Android 使用）
#              └── 静态库 (.a)（iOS 使用）
```

| crate-type | 产物 | 用途 |
|------------|------|------|
| `staticlib` | `libtunnel_core.a` | iOS：链接到 Extension 二进制 |
| `cdylib` | `libtunnel_core.so` | Android：运行时动态加载 |
| `lib` | `libtunnel_core.rlib` | Rust 内部依赖、测试 |

### 平台特定依赖

```toml
# iOS 专用
[target.'cfg(target_os = "ios")'.dependencies]
oslog = "0.2"          # iOS 系统日志

# Android 专用
[target.'cfg(target_os = "android")'.dependencies]
android_logger = "0.14"  # logcat 日志输出
jni = "0.21"             # JNI 绑定
socket2 = "0.5"          # 底层 socket 操作（用于 protect）

# 桌面专用（开发调试）
[target.'cfg(not(any(target_os = "ios", target_os = "android")))'.dependencies]
env_logger = "0.11"      # 终端日志
```

---

## 2.6 .so 动态库 vs iOS .a 静态库

### 对比

| 特性 | .so (Android) | .a (iOS) |
|------|---------------|----------|
| 类型 | 动态链接库 | 静态链接库 |
| 加载时机 | 运行时 (System.loadLibrary) | 编译时链接进二进制 |
| 文件位置 | APK 内 jniLibs/ 目录 | 编译产物，不在 App Bundle 中 |
| 符号可见性 | 需要 `#[no_mangle]` + `pub extern` | 需要 `#[no_mangle]` + `pub extern "C"` |
| 大小影响 | 独立文件，可按需加载 | 链接后成为可执行文件的一部分 |
| 更新方式 | 替换 .so 文件 | 必须重新编译整个 Extension |

### 为什么 Android 用动态库

```
Android 加载流程：
  1. System.loadLibrary("tunnel_core")
  2. 系统在 APK 的 lib/arm64-v8a/ 中查找 libtunnel_core.so
  3. dlopen() 加载到进程地址空间
  4. 调用 JNI_OnLoad()（如果存在）
  5. native 方法可用

iOS 加载流程：
  1. .a 在编译时已经链接到 Extension 二进制
  2. Extension 启动时，代码已经在内存中
  3. Swift 通过 Bridging Header 直接调用 C 函数
```

Android 使用动态库的原因：
- JNI 机制要求通过 `System.loadLibrary()` 加载
- APK 支持包含多个 ABI 的 .so，系统自动选择匹配的
- 动态库可以独立更新（热修复场景）
