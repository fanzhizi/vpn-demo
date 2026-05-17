# 第三章：Win32 API 网络管理

---

## 3.1 概述

Windows 没有像 iOS/Android 那样的高层 VPN 框架，需要直接调用 Win32 API 管理网络配置。本章详解项目中使用的每个 API。

```
Win32 API 使用全景：

┌──────────────┐   ┌──────────────────────────┐   ┌──────────────┐
│ GetBestRoute2│   │ CreateUnicastIpAddress... │   │CreateIpFwd...│
│ (查物理 IP)  │   │ (配置适配器 IP)           │   │(添加路由)    │
└──────┬───────┘   └──────────┬───────────────┘   └──────┬───────┘
       │                      │                          │
       ▼                      ▼                          ▼
  物理网卡 IP            TUN IP: 10.0.0.2          0.0.0.0/1 → TUN
  用于 socket bind       子网: /24                 128.0.0.0/1 → TUN

┌──────────────────────────┐   ┌──────────────────────────┐
│ ConvertInterfaceLuidToGuid│   │ 注册表 NameServer        │
│ (LUID → GUID 转换)       │   │ (配置 DNS)               │
└──────────┬───────────────┘   └──────────┬───────────────┘
           │                              │
           └──────────────────────────────┘
                     用于定位注册表路径
```

---

## 3.2 GetBestRoute2：查询物理网卡 IP

### 函数签名

```c
NETIOAPI_API GetBestRoute2(
    NET_LUID    *InterfaceLuid,      // [可选] 限定接口
    NET_IFINDEX  InterfaceIndex,     // [可选] 限定接口索引
    SOCKADDR_INET *SourceAddress,    // [可选] 限定源地址
    SOCKADDR_INET *DestinationAddress, // [必须] 目标地址
    ULONG        AddressSortOptions, // 排序选项
    MIB_IPFORWARD_ROW2 *BestRoute,  // [输出] 最佳路由条目
    SOCKADDR_INET *BestSourceAddress // [输出] 源地址（就是我们要的）
);
```

### 调用原理

```
输入：目标地址 = 8.8.8.8 (Google DNS)
       │
       ▼
系统路由表查找最佳匹配路由
       │
       ├─── 匹配路由: 0.0.0.0/0 via 192.168.1.1 (默认网关)
       │    出口接口: 物理网卡 (Wi-Fi/Ethernet)
       │
       ▼
输出：
  BestRoute   → 匹配的路由条目
  BestSource  → 192.168.1.100 (物理网卡 IP)  ← 这就是我们需要的！
```

### 为什么查询 8.8.8.8

1. **8.8.8.8 是 Google DNS**，几乎所有网络环境都能路由到它
2. 查询它的最佳路由 → 系统返回「连接互联网用的物理网卡 IP」
3. 这个 IP 就是后续 socket bind 的目标

### 调用时机至关重要

```
正确顺序：

  get_physical_adapter_ip()    ← 此时路由表正常
  create TUN adapter           ← 创建后添加 TUN 路由
  add default route via TUN    ← 路由表被修改

错误顺序：

  create TUN adapter
  add default route via TUN    ← 路由表已修改
  get_physical_adapter_ip()    ← 可能返回 TUN 的 IP！
```

### 代码实现

```rust
fn get_physical_adapter_ip() -> Option<Ipv4Addr> {
    unsafe {
        let dest_addr = SOCKADDR_INET {
            Ipv4: SOCKADDR_IN {
                sin_family: AF_INET,
                sin_addr: in_addr_from_ipv4(Ipv4Addr::new(8, 8, 8, 8)),
                sin_port: 0,
                sin_zero: [0; 8],
            },
        };

        let mut best_route: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
        let mut best_source: SOCKADDR_INET = std::mem::zeroed();

        let result = GetBestRoute2(
            None,           // InterfaceLuid: 不限定接口
            0,              // InterfaceIndex: 不限定
            None,           // SourceAddress: 不限定
            &dest_addr,     // 目标: 8.8.8.8
            0,              // 无标志
            &mut best_route,
            &mut best_source,  // 输出：物理网卡 IP
        );
        // ...
    }
}
```

---

## 3.3 CreateUnicastIpAddressEntry：配置适配器 IP

### 功能

为 TUN 适配器分配 IP 地址和子网掩码。

### MIB_UNICASTIPADDRESS_ROW 结构体

```
MIB_UNICASTIPADDRESS_ROW
├── InterfaceLuid    → 目标接口（TUN 的 LUID）
├── Address          → IP 地址（SOCKADDR_INET 格式）
│   ├── si_family = AF_INET
│   └── Ipv4.sin_addr = 10.0.0.2
├── OnLinkPrefixLength → 子网前缀长度（24 = 255.255.255.0）
└── DadState         → 重复地址检测状态
                       IpDadStatePreferred = 直接使用，跳过 DAD
```

### 调用流程

```rust
let mut row: MIB_UNICASTIPADDRESS_ROW = std::mem::zeroed();
InitializeUnicastIpAddressEntry(&mut row);  // 必须先初始化！

row.InterfaceLuid.Value = luid;             // TUN 接口
row.Address.si_family = AF_INET;
row.Address.Ipv4.sin_addr = in_addr_from_ipv4(ip);
row.OnLinkPrefixLength = 24;               // /24 子网
row.DadState = IpDadStatePreferred;         // 跳过 DAD

CreateUnicastIpAddressEntry(&row);
```

### 与其他方式的对比

| 方式 | 命令/API | 优缺点 |
|------|---------|--------|
| netsh | `netsh interface ip set address ...` | 需要解析命令输出，不够可靠 |
| API | `CreateUnicastIpAddressEntry` | 程序化、可靠、推荐 |
| WMI | Win32_NetworkAdapterConfiguration | 太重量级 |

---

## 3.4 CreateIpForwardEntry2：路由管理

### 功能

添加路由条目到系统路由表。

### MIB_IPFORWARD_ROW2 结构体

```
MIB_IPFORWARD_ROW2
├── InterfaceLuid        → 出口接口（TUN 的 LUID）
│                          ★ 这是自动清理的关键 ★
├── DestinationPrefix    → 目标网段
│   ├── Prefix           → 网络地址 (如 0.0.0.0)
│   └── PrefixLength     → 前缀长度 (如 1)
├── NextHop              → 下一跳地址 (如 10.0.0.2)
├── Metric               → 优先级 (5 = 高优先级)
└── Protocol             → 路由来源 (3 = PROTO_IP_NETMGMT)
```

### 路由策略：0.0.0.0/1 + 128.0.0.0/1

```
为什么不用 0.0.0.0/0 (默认路由)？

  系统原有:  0.0.0.0/0  via 192.168.1.1  (网关)
  如果添加:  0.0.0.0/0  via 10.0.0.2    (TUN)
  → 冲突！可能覆盖系统默认路由，断开时网络异常

改用两条更具体的路由:
  0.0.0.0/1    via 10.0.0.2  → 覆盖 0.0.0.0 ~ 127.255.255.255
  128.0.0.0/1  via 10.0.0.2  → 覆盖 128.0.0.0 ~ 255.255.255.255

为什么 /1 优先于 /0？
  路由匹配规则: 最长前缀匹配 (Longest Prefix Match)
  /1 (1 bit) 比 /0 (0 bit) 更具体 → 优先匹配
  两条 /1 路由完整覆盖所有 IPv4 地址
  但系统的 /0 默认路由不被覆盖，作为回退
```

### LUID 绑定的自动清理机制

```
创建路由时:
  row.InterfaceLuid.Value = tun_luid;
  CreateIpForwardEntry2(&row);
  → 路由绑定到 TUN 接口的 LUID

适配器销毁时:
  drop(adapter)
  → wintun.dll 通知内核销毁 TUN 接口
  → Windows 内核遍历路由表
  → 删除所有 InterfaceLuid == tun_luid 的路由条目
  → 无需手动 DeleteIpForwardEntry2！

对比 route add 命令:
  route add 0.0.0.0 mask 128.0.0.0 10.0.0.2
  → 路由不绑定到特定接口
  → 程序崩溃后路由残留
  → 必须手动 route delete 清理
```

---

## 3.5 ConvertInterfaceLuidToGuid + 注册表配置 DNS

### 为什么 DNS 没有直接 API

Windows 的 IP Helper API 提供了：
- IP 地址管理：`CreateUnicastIpAddressEntry` / `DeleteUnicastIpAddressEntry`
- 路由管理：`CreateIpForwardEntry2` / `DeleteIpForwardEntry2`
- 接口查询：`GetBestRoute2`、`GetAdaptersAddresses`

但**没有**：
- DNS 设置 API（只有 DNS 查询 API `DnsQuery`）

DNS 配置存储在注册表中，必须直接操作注册表。

### 注册表路径

```
HKEY_LOCAL_MACHINE
└── SYSTEM
    └── CurrentControlSet
        └── Services
            └── Tcpip
                └── Parameters
                    └── Interfaces
                        └── {A1B2C3D4-E5F6-7890-ABCD-EF1234567890}  ← 接口 GUID
                            ├── NameServer = "8.8.8.8,8.8.4.4"      ← 静态 DNS
                            ├── DhcpNameServer = "..."               ← DHCP DNS
                            └── IPAddress = "..."                    ← IP 地址
```

### LUID → GUID 转换

```
为什么需要转换？

  wintun API 返回: LUID (u64 数值, 内存中唯一)
  注册表键名使用: GUID (128-bit UUID, 如 {A1B2C3...})

  所以需要 ConvertInterfaceLuidToGuid 转换
```

### 代码流程

```rust
// 1. LUID → GUID
let mut luid_val: NET_LUID_LH = std::mem::zeroed();
luid_val.Value = luid;
let mut guid = GUID::zeroed();
ConvertInterfaceLuidToGuid(&luid_val, &mut guid);

// 2. GUID 格式化为字符串
let guid_str = format!("{{{:08X}-{:04X}-...}}", guid.data1, ...);

// 3. 构造注册表路径
let reg_path = format!(
    "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\\{}",
    guid_str
);

// 4. 写入 NameServer
let hkey = RegKey::predef(HKEY_LOCAL_MACHINE);
let subkey = hkey.open_subkey_with_flags(&reg_path, KEY_WRITE)?;
subkey.set_value("NameServer", &"8.8.8.8,8.8.4.4")?;
```

### 回退方案：netsh

如果注册表写入失败（权限问题），回退到 netsh 命令：

```bash
netsh interface ip set dns "VPNDemo" static 8.8.8.8
netsh interface ip add dns "VPNDemo" 8.8.4.4 index=2
```

---

## 3.6 核心数据结构速查

| 结构体 | 用途 | 关键字段 |
|--------|------|---------|
| `SOCKADDR_INET` | 通用网络地址（IPv4/IPv6 联合体） | `si_family`, `Ipv4.sin_addr` |
| `SOCKADDR_IN` | IPv4 地址 | `sin_family`, `sin_addr`, `sin_port` |
| `IN_ADDR` | IPv4 地址（4字节） | `S_un.S_addr` (u32, 网络字节序) |
| `NET_LUID_LH` | 网络接口本地唯一标识符 | `Value` (u64) |
| `MIB_IPFORWARD_ROW2` | 路由表条目 | `InterfaceLuid`, `DestinationPrefix`, `NextHop`, `Metric` |
| `MIB_UNICASTIPADDRESS_ROW` | IP 地址配置 | `InterfaceLuid`, `Address`, `OnLinkPrefixLength`, `DadState` |

### IN_ADDR 字节序

```rust
// Ipv4Addr 的 octets() 返回网络字节序（大端）
// IN_ADDR.S_un.S_addr 也是网络字节序
// 所以用 from_ne_bytes 直接转换（不做字节序变换）
fn in_addr_from_ipv4(ip: Ipv4Addr) -> IN_ADDR {
    IN_ADDR {
        S_un: IN_ADDR_0 {
            S_addr: u32::from_ne_bytes(ip.octets()),
        },
    }
}

// 例如 192.168.1.1:
//   octets = [192, 168, 1, 1]
//   from_ne_bytes → 内存布局 [192, 168, 1, 1] → 正确的网络字节序
```
