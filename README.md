<div align="center">

# 🐾 Desktop Pet

**A Codex-compatible animated pet for image-capable terminals and a draggable desktop companion.**

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-5B73D0?style=flat-square)](LICENSE)
[![Release](https://img.shields.io/github/v/release/Delta-13/Desktop-Pet?style=flat-square&color=8B84FF)](https://github.com/Delta-13/Desktop-Pet/releases/latest)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Desktop GUI](https://img.shields.io/badge/interface-desktop%20GUI-8B84FF?style=flat-square)](#desktop-gui)

</div>

---

## Overview

Desktop Pet loads a local Codex pet package, previews its animations in a
Codex-inspired controller, and can promote it to a transparent, draggable,
always-on-top desktop pet. It is fully local: no account, cloud sync, or
network connection is required.

> This is an independent project and is not affiliated with OpenAI.

## Table of contents

- [Features](#features)
- [Download v0.1.0](#download-v010)
- [Quick start](#quick-start)
- [Desktop GUI](#desktop-gui)
- [Terminal support](#terminal-support)
- [Development](#development)
- [License](#license)

## Features

- Load any local pet folder containing `pet.json` and its spritesheet.
- Switch the complete interface between English, Simplified Chinese, and Japanese.
- Preview each animation state defined by the pet package.
- Drag the pet anywhere within the GUI preview canvas.
- Turn a loaded pet into a borderless desktop companion that you can drag around
  the desktop. Click it to play or wave, drag it to walk in that direction, and
  right-click it to return to settings.
- Adjust the pet size and optionally let it take short, random walks while idle.
- Copy a terminal command without launching a terminal or subprocess.
- Render Codex v1 (8×9) and v2 (8×11) sprite atlases in compatible terminals.

## Download v0.1.0

| Platform | Download |
| --- | --- |
| Windows 10/11 · x86_64 | [Desktop-Pet-v0.1.0-windows-x86_64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-windows-x86_64.zip) |
| macOS 11+ · Intel | [Desktop-Pet-v0.1.0-macos-x86_64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-macos-x86_64.zip) |
| macOS 11+ · Apple Silicon | [Desktop-Pet-v0.1.0-macos-aarch64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-macos-aarch64.zip) |

Extract the archive and launch `terminal-sprite-pet.exe` on Windows or
`Desktop Pet.app` on macOS. These first-release binaries are not commercially
code-signed. Windows SmartScreen may show a warning; on macOS, Control-click
the app and choose **Open** the first time.

## Quick start

Install the stable Rust toolchain from [rustup](https://rustup.rs/), then:

```powershell
git clone https://github.com/Delta-13/Desktop-Pet.git
Set-Location Desktop-Pet
cargo run -- --gui
```

Running `cargo run` with no arguments also opens the GUI.

Launch a pet directly as a desktop companion:

```powershell
cargo run -- --desktop .\pets\anpan
```

## Desktop GUI

The local controller provides an English / 简体中文 / 日本語 selector, pet-package
loading, animation states, a draggable preview, a size slider, and a random-walk
switch. Choose **Desktop pet mode** to make the loaded pet a borderless,
transparent, always-on-top companion with no visible controls. Click the pet to
randomly play or wave; drag it to move its window and walk in the drag direction.
The walk direction follows the latest horizontal pointer movement, including a
left/right reversal without releasing the mouse. Pausing or releasing stops the
running animation immediately.
After it is idle for a while it rests. Right-click the pet to return to the
controller and change its settings.

The **Copy** button only places a terminal command on the clipboard; it never
launches a terminal or process.

## Terminal support

| Platform | Supported terminal protocol |
| --- | --- |
| Windows | Windows Terminal or another Sixel-capable terminal |
| macOS | iTerm2 3.6+, Kitty, Ghostty, or WezTerm |

Render a pet package from the terminal:

```powershell
cargo run -- "$env:USERPROFILE\.codex\pets\anpan" --state running
```

Use `--protocol kitty` or `--protocol sixel` to override auto detection.

## Development

```powershell
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

## License

Licensed under [Apache-2.0](LICENSE). See
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for upstream attribution.
