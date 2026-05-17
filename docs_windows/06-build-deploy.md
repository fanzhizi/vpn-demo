# 第六章：构建与部署

---

## 6.1 环境准备

### 必要工具

| 工具 | 用途 | 安装方式 |
|------|------|---------|
| Rust | 编译器 | `rustup` |
| Visual Studio Build Tools | MSVC 链接器 | VS Installer (C++ 工具) |
| wintun.dll | TUN 驱动 | [wintun.net](https://www.wintun.net) 下载 |
| NSIS (可选) | 安装包打包 | `choco install nsis` |
| mingw-w64 (可选) | 交叉编译链接器 | `brew install mingw-w64` |

### wintun.dll 获取

```bash
# 下载 wintun 发行包
curl -L https://www.wintun.net/builds/wintun-0.14.1.zip -o wintun.zip
unzip wintun.zip

# 选择 amd64 版本
cp wintun/bin/amd64/wintun.dll target/release/

# wintun.dll 必须和 exe 在同一目录！
# 或者在系统 PATH 中
```

---

## 6.2 本地构建

### Windows 原生构建（推荐）

```powershell
# 确保在 vpn-demo 根目录
cd vpn-demo

# 编译 Release 版本
cargo build --package vpn-demo-windows --release

# 编译产物
# target\release\vpn-demo-windows.exe

# 运行（需要管理员权限）
# 右键 → 以管理员身份运行，或在管理员终端中运行
target\release\vpn-demo-windows.exe
```

### macOS/Linux 交叉编译

```bash
# 安装 Windows 交叉编译目标
rustup target add x86_64-pc-windows-gnu

# 安装 mingw-w64 链接器
# macOS:
brew install mingw-w64
# Ubuntu:
sudo apt install gcc-mingw-w64-x86-64

# 编译
cargo build --package vpn-demo-windows --target x86_64-pc-windows-gnu --release

# 产物
# target/x86_64-pc-windows-gnu/release/vpn-demo-windows.exe
```

### Debug vs Release 构建

| 配置 | 命令 | 大小（约） | 用途 |
|------|------|-----------|------|
| Debug | `cargo build` | ~30MB | 开发调试 |
| Release | `cargo build --release` | ~8MB | 发布 |
| Release + strip | `cargo build --release` + `strip` | ~5MB | 最小化 |

---

## 6.3 GitHub Actions CI 配置

### build-windows.yml 详解

```yaml
name: Build Windows Installer

on:
  push:
    branches: [main]
    paths:                              # 只在相关文件变更时触发
      - 'tunnel-core/**'
      - 'windows-app/**'
      - 'vpn-app/ui/**'
      - '.github/workflows/build-windows.yml'
  workflow_dispatch:                    # 支持手动触发

jobs:
  build:
    runs-on: windows-latest            # Windows Server 2022 runner
    steps:
      # 1. 检出代码
      - uses: actions/checkout@v4

      # 2. 安装 Rust 工具链
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      # 3. 编译 Windows 应用
      - name: Build Windows app
        run: cargo build --package vpn-demo-windows --release

      # 4. 下载 wintun.dll（不包含在代码仓库中）
      - name: Download wintun.dll
        run: |
          Invoke-WebRequest -Uri "https://www.wintun.net/builds/wintun-0.14.1.zip" \
            -OutFile wintun.zip
          Expand-Archive wintun.zip -DestinationPath wintun-extracted
          Copy-Item "wintun-extracted/wintun/bin/amd64/wintun.dll" "target/release/"

      # 5. 安装 NSIS 打包工具
      - name: Install NSIS
        run: choco install nsis -y

      # 6. 构建安装包
      - name: Build installer
        run: |
          Copy-Item "target/release/vpn-demo-windows.exe" "windows-app/installer/"
          Copy-Item "target/release/wintun.dll" "windows-app/installer/"
          & "C:\Program Files (x86)\NSIS\makensis.exe" \
            "windows-app/installer/installer.nsi"

      # 7. 上传安装包和便携版
      - name: Upload installer
        uses: actions/upload-artifact@v4
        with:
          name: VPNDemo-Windows-Installer
          path: windows-app/installer/VPNDemo-Setup.exe

      - name: Upload portable
        uses: actions/upload-artifact@v4
        with:
          name: VPNDemo-Windows-Portable
          path: |
            target/release/vpn-demo-windows.exe
            target/release/wintun.dll
```

### CI 流程图

```
git push main
     │
     ▼
GitHub Actions 触发
     │
     ├─ checkout 代码
     ├─ 安装 Rust
     ├─ cargo build --release
     ├─ 下载 wintun.dll
     ├─ 安装 NSIS
     ├─ 打包 installer
     │
     ▼
产物上传：
  ├─ VPNDemo-Setup.exe     (NSIS 安装包)
  └─ VPNDemo-Portable/     (便携版)
      ├─ vpn-demo-windows.exe
      └─ wintun.dll
```

---

## 6.4 NSIS 安装包制作

### installer.nsi 详解

NSIS（Nullsoft Scriptable Install System）是开源的 Windows 安装包制作工具。

```nsis
; ===== 基本信息 =====
!define APP_NAME "VPN Demo"
!define APP_VERSION "1.0.0"
!define APP_EXE "vpn-demo-windows.exe"
!define WINTUN_DLL "wintun.dll"

; 安装目录
InstallDir "$PROGRAMFILES64\${APP_NAME}"    ; C:\Program Files\VPN Demo

; 请求管理员权限（关键！wintun 需要管理员才能运行）
RequestExecutionLevel admin
```

### 安装段：做了什么

```
安装过程：

1. 复制文件到安装目录
   C:\Program Files\VPN Demo\
   ├── vpn-demo-windows.exe      ← 主程序
   └── wintun.dll                ← TUN 驱动

2. 写注册表（用于"添加/删除程序"）
   HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\VPN Demo
   ├── DisplayName = "VPN Demo"
   ├── UninstallString = "...\Uninstall.exe"
   ├── DisplayVersion = "1.0.0"
   └── Publisher = "VPN Demo Project"

3. 创建快捷方式
   开始菜单: VPN Demo → VPN Demo.lnk
   桌面: VPN Demo.lnk

4. 创建卸载程序
   Uninstall.exe
```

### 卸载段：做了什么

```
卸载过程：

1. 删除安装目录中的所有文件
2. 删除开始菜单和桌面快捷方式
3. 删除注册表项
4. 删除安装目录
```

---

## 6.5 UAC Manifest 嵌入

### 为什么需要 UAC

```
不嵌入 manifest 时：
  双击 exe → 普通权限运行 → wintun::Adapter::create() 失败
  → 用户需要右键"以管理员身份运行"（体验差）

嵌入 manifest 后：
  双击 exe → Windows 自动弹出 UAC 对话框 → 用户点击"是"
  → 管理员权限运行 → 一切正常
```

### app.manifest

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel
          level="requireAdministrator"    ← 要求管理员权限
          uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
```

### build.rs 嵌入过程

```rust
#[cfg(target_os = "windows")]
{
    let mut res = winres::WindowsResource::new();
    res.set_manifest_file("app.manifest");     // 嵌入 manifest
    res.set("FileDescription", "VPN Demo");    // exe 属性：文件描述
    res.set("ProductName", "VPN Demo");        // exe 属性：产品名称
    res.compile().unwrap();
}
```

winres 的工作原理：
1. 读取 `app.manifest` 文件内容
2. 生成 Windows 资源脚本 (.rc)
3. 编译为资源对象 (.res)
4. 链接到最终的 exe 文件中

---

## 6.6 发布清单

### 便携版（Portable）

```
VPNDemo-Portable/
├── vpn-demo-windows.exe    # 主程序
└── wintun.dll              # TUN 驱动 DLL
                            # 两个文件必须在同一目录
```

使用方式：右键 exe → 以管理员身份运行

### 安装版（Installer）

```
VPNDemo-Setup.exe           # NSIS 安装包
                            # 双击运行 → UAC 提示 → 安装向导
                            # 安装后在开始菜单和桌面创建快捷方式
```

### 分发注意事项

| 事项 | 说明 |
|------|------|
| wintun.dll 必须包含 | exe 启动时 LoadLibrary("wintun.dll")，找不到则报错 |
| 管理员权限 | manifest 嵌入后自动请求，无需手动操作 |
| 杀毒误报 | wintun.dll 操作内核设备，可能触发杀毒软件误报 |
| Windows 版本 | 最低 Windows 7（wintun 要求） |
| 架构 | x86_64 only（wintun.dll 分 x86/amd64/arm64） |
