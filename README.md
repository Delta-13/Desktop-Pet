<div align="center">

# 🐾 Desktop Pet

**A Codex-compatible animated pet for image-capable terminals and a focused desktop preview.**

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-5B73D0?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Desktop GUI](https://img.shields.io/badge/interface-desktop%20GUI-8B84FF?style=flat-square)](#desktop-gui)

</div>

---

## Overview

Desktop Pet loads a local Codex pet package, previews its animations in a
Codex-inspired desktop controller, and can render the same pet in compatible
terminals. It is fully local: no account, cloud sync, or network connection is
required.

> This is an independent project and is not affiliated with OpenAI.

## Table of contents

- [Features](#features)
- [Quick start](#quick-start)
- [Desktop GUI](#desktop-gui)
- [Terminal support](#terminal-support)
- [Development](#development)
- [License](#license)

## Features

- Load any local pet folder containing `pet.json` and its spritesheet.
- Preview each animation state defined by the pet package.
- Drag the pet anywhere within the GUI preview canvas.
- Copy a terminal command without launching a terminal or subprocess.
- Render Codex v1 (8×9) and v2 (8×11) sprite atlases in compatible terminals.

## Quick start

Install the stable Rust toolchain from [rustup](https://rustup.rs/), then:

```powershell
git clone https://github.com/Delta-13/Desktop-Pet.git
Set-Location Desktop-Pet
cargo run -- --gui
```

Running `cargo run` with no arguments also opens the GUI.

## Desktop GUI

The local controller lets you choose a pet package, switch between its
animation states, and drag the animated sprite in the preview canvas. The
**Copy** button only places a terminal command on the clipboard; it never
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
