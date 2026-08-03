<div align="center">

# 🐾 Desktop Pet

**一个可在支持图片的终端或本地桌面预览中运行的、兼容 Codex 的动态宠物。**

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

[![许可证：Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-5B73D0?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![桌面 GUI](https://img.shields.io/badge/interface-桌面%20GUI-8B84FF?style=flat-square)](#桌面-gui)

</div>

---

## 简介

Desktop Pet 可加载本地 Codex 宠物包，在具有 Codex 风格的桌面控制器中预览动画，
也能在兼容终端中渲染同一只宠物。所有数据仅保留在本地，不需要账户、云同步或网络连接。

> 本项目为独立项目，与 OpenAI 没有隶属关系。

## 目录

- [功能](#功能)
- [快速开始](#快速开始)
- [桌面 GUI](#桌面-gui)
- [终端支持](#终端支持)
- [开发](#开发)
- [许可证](#许可证)

## 功能

- 加载任何包含 `pet.json` 与精灵图的本地宠物文件夹。
- 预览宠物包中定义的每个动画状态。
- 可用鼠标在 GUI 预览画布内拖动精灵。
- 仅复制终端命令，不会启动终端或子进程。
- 可在兼容终端中渲染 Codex v1（8×9）与 v2（8×11）精灵图集。

## 快速开始

请先通过 [rustup](https://rustup.rs/) 安装稳定版 Rust 工具链，然后执行：

```powershell
git clone https://github.com/Delta-13/Desktop-Pet.git
Set-Location Desktop-Pet
cargo run -- --gui
```

不带参数运行 `cargo run` 也会打开 GUI。

## 桌面 GUI

本地控制器可让你选择宠物包、切换动画状态，并在预览画布中拖动动态精灵。
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
