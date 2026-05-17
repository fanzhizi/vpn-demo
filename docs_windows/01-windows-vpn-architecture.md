# 第一章：Windows VPN 架构基础

---

## 1.1 wintun.dll 介绍

### 什么是 wintun

wintun 是由 WireGuard 项目开发的**用户态 TUN 驱动**，专为 Windows 设计。

```
传统方案（TAP）:
  应用程序 → TAP 驱动 (需要安装, .sys 文件) → 虚拟网卡

wintun 方案:
  应用程序 → wintun.dll (无需安装, 单个 DLL) → 虚拟网卡
```

### 为什么选择 wintun

| 特性 | TAP (OpenVPN) | wintun (WireGuard) |
|------|---------------|-------------------|
| 安装方式 | 需要安装驱动 (.sys) | 只需 DLL 文件 |
| 数据层 | L2 (以太网帧) | L3 (IP 包) |
| 性能 | 一般 | 高（零拷贝环形缓冲区） |
| 接口复杂度 | 复杂 | 简单（C API） |
| 签名要求 | 需要内核驱动签名 | DLL 已签名 |
| Rust 支持 | 无官方 crate | wintun crate |
| 最低 Windows | Windows Vista | Windows 7 |

### wintun 的工作原理

```
┌──────────────────────────────────────────────────┐
│                    用户态                          │
│                                                  │
│  ┌───────────┐    ┌──────────────────────────┐   │
│  │ 我们的 App │◄──▶│       wintun.dll          │   │
│  │           │    │  (用户态接口 + 内核通信)   │   │
│  └───────────┘    └────────────┬─────────────┘   │
│                                │                  │
├────────────────────────────────┼──────────────────┤
│                    内核态       │                  │
│                                ▼                  │
│              ┌────────────────────────────┐       │
│              │     wintun.sys (内核驱动)    │       │
│              │   创建虚拟网络接口 (TUN)      │       │
│              └───────────────┬────────────┘       │
│                              │                    │
│              ┌───────────────▼────────────┐       │
│              │    Windows TCP/IP 协议栈     │       │
│              │    (路由、DNS 解析等)        │       │
│              └────────────────────────────┘       │
└──────────────────────────────────────────────────┘
```

wintun.dll 负责与内核驱动通信，应用程序只需调用 DLL 导出的函数。

---

## 1.2 TUN 设备生命周期

### 完整生命周期

```
load wintun.dll
    │
    ▼
Adapter::create("VPNDemo", ...)     ← 创建虚拟网络接口
    │                                   (出现在「网络连接」中)
    ▼
set_adapter_ip(10.0.0.2/24)         ← 配置 IP 地址
set_adapter_dns(8.8.8.8)            ← 配置 DNS
add_route(0.0.0.0/1, via TUN)       ← 添加路由（绑定 LUID）
    │
    ▼
adapter.start_session(4MB)          ← 创建读写 session
    │                                   (分配环形缓冲区)
    ├──────────────────────┐
    ▼                      ▼
读线程:                   写线程:
session.receive()         session.allocate_send_packet()
    │                     session.send_packet()
    ▼                      ▼
TUN → channel → netstack → channel → TUN
    │
    ▼
running = false
    │
    ▼
drop(adapter)                       ← 销毁虚拟网络接口
    │                                   系统自动清理:
    │                                   - 删除绑定到 LUID 的路由
    │                                   - 移除网络接口
    ▼
完毕（无残留）
```

### 关键 API 调用

```rust
// 1. 加载 DLL
let wintun = unsafe { wintun::load()? };

// 2. 创建适配器（需要管理员权限）
let adapter = wintun::Adapter::create(&wintun, "VPNDemo", "Tunnel", None)?;

// 3. 获取 LUID（后续 API 调用需要）
let luid = adapter.get_luid();

// 4. 启动 session（设置环形缓冲区大小）
let session = adapter.start_session(0x400000)?; // 4MB

// 5. 读取数据包（阻塞）
let packet = session.receive_blocking()?;
let data = packet.bytes(); // 原始 IP 包（无以太网头）

// 6. 写入数据包
let mut send_pkt = session.allocate_send_packet(data.len() as u16)?;
send_pkt.bytes_mut().copy_from_slice(&data);
session.send_packet(send_pkt);

// 7. 清理（drop 即可）
drop(adapter); // 自动销毁，路由自动清理
```

---

## 1.3 与 iOS/Android VPN API 的对比

| 特性 | iOS (NEPacketTunnel) | Android (VpnService) | Windows (wintun) |
|------|---------------------|---------------------|------------------|
| API 层级 | 系统框架 | 系统 Service | 第三方 DLL |
| 权限获取 | entitlement + 用户确认 | BIND_VPN_SERVICE | 管理员权限 (UAC) |
| TUN 创建 | 系统创建，Extension 获取 | Builder.establish() | Adapter::create() |
| IP 配置 | NEPacketTunnelNetwork... | Builder.addAddress() | CreateUnicastIpAddress... |
| 路由配置 | NEIPv4Settings | Builder.addRoute() | CreateIpForwardEntry2 |
| DNS 配置 | NEDNSSettings | Builder.addDnsServer() | 注册表 (无 API) |
| 数据包格式 | NSData (IP 包) | FileDescriptor (IP 包) | 环形缓冲区 (IP 包) |
| 路由绕过 | 进程隔离（自动） | protect(fd) | socket bind |
| 清理机制 | Extension 退出即清理 | Service 停止即清理 | adapter drop 即清理 |
| 后台运行 | Extension 独立进程 | 前台 Service | 普通进程（无后台保活） |

---

## 1.4 管理员权限要求

### 为什么需要管理员

wintun 创建虚拟网络接口需要内核操作，必须以管理员身份运行：
- `Adapter::create()` → 需要创建网络设备
- `CreateIpForwardEntry2()` → 需要修改路由表
- 注册表写入 DNS → 需要 HKLM 写权限

### UAC Manifest

通过 `app.manifest` 声明需要管理员权限：

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
```

在 `build.rs` 中通过 `winres` crate 嵌入 manifest：

```rust
#[cfg(target_os = "windows")]
{
    let mut res = winres::WindowsResource::new();
    res.set_manifest_file("app.manifest");
    res.compile().unwrap();
}
```

效果：双击 exe 时 Windows 自动弹出 UAC 提示，用户确认后以管理员身份运行。

### 与其他平台的权限对比

| 平台 | 权限机制 | 用户体验 |
|------|---------|---------|
| iOS | Entitlement + 系统弹窗 | 首次连接弹窗「允许 VPN」 |
| Android | BIND_VPN_SERVICE + 系统弹窗 | 首次连接弹窗「建立 VPN 连接」 |
| Windows | UAC manifest | 每次启动弹窗「是否允许更改」 |
