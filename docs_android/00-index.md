# Android VPN 开发实战

---

## 目录

本文档系列详细讲解如何在 Android 平台上基于 `VpnService` + Rust `tunnel-core` 实现一个完整的 VPN 客户端。

| 章节 | 标题 | 核心内容 |
|------|------|----------|
| [第一章](01-android-vpn-architecture.md) | Android VPN 架构基础 | VpnService 生命周期、TUN 设备、Builder 配置 |
| [第二章](02-project-structure.md) | Android 项目结构 | Gradle 配置、jniLibs、cargo-ndk 交叉编译 |
| [第三章](03-jni-bridge.md) | JNI 桥接层 | System.loadLibrary、JNI_OnLoad、类型转换 |
| [第四章](04-socket-protect.md) | Socket 保护机制 | protect 原理、JNI 回调、与 iOS 的差异 |
| [第五章](05-data-flow.md) | 完整数据流追踪 | 从点击到网页加载的全链路分析 |
| [第六章](06-build-deploy.md) | 构建、调试与部署 | 环境准备、编译步骤、常见错误 |
| [第七章](07-ios-vs-android.md) | iOS 与 Android 实现对比 | 架构差异、API 对比、代码共享 |

---

## 项目概述

```
┌─────────────────────────────────────────────────────────────────┐
│                        Android App                               │
│                                                                  │
│  ┌──────────────┐    ┌──────────────────┐    ┌───────────────┐  │
│  │ MainActivity │───▶│ TunnelVpnService │───▶│  TunnelCore   │  │
│  │   (UI 层)    │    │  (VPN Service)   │    │  (JNI 桥接)   │  │
│  └──────────────┘    └───────┬──────────┘    └───────┬───────┘  │
│                              │                       │           │
│                         TUN 设备                JNI 调用          │
│                              │                       │           │
└──────────────────────────────┼───────────────────────┼───────────┘
                               │                       │
                               ▼                       ▼
┌──────────────────────────────────────────────────────────────────┐
│                    Rust tunnel-core (.so)                         │
│                                                                  │
│  ┌───────────┐    ┌──────────────┐    ┌────────────────────┐    │
│  │ jni_ffi   │───▶│   ffi.rs     │───▶│  netstack-smoltcp  │    │
│  │ (JNI 入口)│    │ (核心逻辑)   │    │  (TCP/IP 协议栈)   │    │
│  └───────────┘    └──────┬───────┘    └────────────────────┘    │
│                          │                                       │
│                          ▼                                       │
│                   ┌──────────────┐                               │
│                   │ fast-socks5  │                               │
│                   │ (SOCKS5 转发)│                               │
│                   └──────────────┘                               │
└──────────────────────────────────────────────────────────────────┘
```

---

## 适合读者

- 想了解 Android VPN 实现原理的开发者
- 正在用 Rust 开发跨平台网络工具的开发者
- 想对比 iOS 和 Android VPN 架构差异的开发者
- 想学习 JNI (Java Native Interface) 实际应用的开发者

---

## 前置知识

- 基本的 Android 开发知识（Activity、Service、Intent）
- 基本的网络知识（TCP/IP、DNS、SOCKS5 协议）
- 基本的 Rust 语法（不需要精通）
- 了解什么是 VPN 和代理
