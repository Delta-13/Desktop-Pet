<div align="center">

# 🐾 Desktop Pet

**一个可在支持图片的终端或桌面上运行的、兼容 Codex 的可拖动动态桌宠。**

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

[![许可证：Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-5B73D0?style=flat-square)](LICENSE)
[![最新版本](https://img.shields.io/github/v/release/Delta-13/Desktop-Pet?style=flat-square&color=8B84FF)](https://github.com/Delta-13/Desktop-Pet/releases/latest)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![桌面 GUI](https://img.shields.io/badge/interface-桌面%20GUI-8B84FF?style=flat-square)](#桌面-gui)

</div>

---

## 简介

Desktop Pet 可加载本地 Codex 宠物包，在具有 Codex 风格的桌面控制器中预览动画，
并将其切换为透明、置顶、可拖动的桌面宠物，也能在兼容终端中渲染同一只宠物。所有数据仅保留在本地，
不需要账户、云同步或网络连接。

> 本项目为独立项目，与 OpenAI 没有隶属关系。

## 目录

- [功能](#功能)
- [下载 v0.1.0](#下载-v010)
- [快速开始](#快速开始)
- [桌面 GUI](#桌面-gui)
- [终端支持](#终端支持)
- [开发](#开发)
- [许可证](#许可证)

## 功能

- 加载任何包含 `pet.json` 与精灵图的本地宠物文件夹。
- 完整界面支持 English、简体中文与日本語切换。
- 预览宠物包中定义的每个动画状态。
- 可用鼠标在 GUI 预览画布内拖动精灵。
- 可将已载入宠物变为无边框、透明、始终置顶的桌宠：单击会玩耍或挥手，拖动时会按拖动方向行走，右键可返回设置。
- 可调整宠物大小，并选择让它在空闲时随机进行短距离散步。
- 仅复制终端命令，不会启动终端或子进程。
- 可在兼容终端中渲染 Codex v1（8×9）与 v2（8×11）精灵图集。

## 下载 v0.1.0

| 平台 | 下载 |
| --- | --- |
| Windows 10/11 · x86_64 | [Desktop-Pet-v0.1.0-windows-x86_64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-windows-x86_64.zip) |
| macOS 11+ · Intel | [Desktop-Pet-v0.1.0-macos-x86_64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-macos-x86_64.zip) |
| macOS 11+ · Apple 芯片 | [Desktop-Pet-v0.1.0-macos-aarch64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-macos-aarch64.zip) |

解压后，在 Windows 上运行 `terminal-sprite-pet.exe`，或在 macOS 上打开
`Desktop Pet.app`。首版文件尚未进行商业代码签名，因此 Windows SmartScreen
可能显示警告；macOS 首次启动时请按住 Control 点击应用并选择**打开**。

## 快速开始

请先通过 [rustup](https://rustup.rs/) 安装稳定版 Rust 工具链，然后执行：

```powershell
git clone https://github.com/Delta-13/Desktop-Pet.git
Set-Location Desktop-Pet
cargo run -- --gui
```

不带参数运行 `cargo run` 也会打开 GUI。

直接以桌宠模式启动本地宠物：

```powershell
cargo run -- --desktop .\pets\anpan
```

## 桌面 GUI

本地控制器提供 English / 简体中文 / 日本語 切换、宠物包选择、动画状态切换和预览画布拖动。
选择**进入桌宠模式**后，已载入宠物会成为无边框、透明且始终置顶的桌面伙伴，且桌面上只显示宠物本身。
单击宠物会随机玩耍或挥手；拖动宠物会移动整个桌宠窗口，并按鼠标拖动方向行走。长时间没有操作时会休息；
在同一次连续拖动中，桌宠会实时跟随最近的鼠标水平移动方向转身；鼠标暂停或松开后会立即停止跑动。
右键点击宠物即可返回控制台。主页设置还可调整宠物大小，并开启空闲时的随机走动。
**复制**按钮只会把终端命令写入剪贴板，不会启动终端或任何进程。

## 终端支持

| 平台 | 支持的终端协议 |
| --- | --- |
| Windows | Windows Terminal 或其他支持 Sixel 的终端 |
| macOS | iTerm2 3.6+、Kitty、Ghostty 或 WezTerm |

在终端中渲染宠物包：

```powershell
cargo run -- "$env:USERPROFILE\.codex\pets\anpan" --state running
```

可使用 `--protocol kitty` 或 `--protocol sixel` 覆盖自动检测。

## 开发

```powershell
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

## 许可证

本项目使用 [Apache-2.0](LICENSE) 许可证；上游归属请参见
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。
