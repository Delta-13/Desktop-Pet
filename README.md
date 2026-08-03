# Desktop Pet / 桌面宠物 / デスクトップペット

**English** · A local, Codex-compatible animated pet for an image-capable
terminal or a focused desktop preview.

**中文** · 一个可在支持图片的终端或本地桌面预览中运行的、兼容 Codex 的动态宠物。

**日本語** · 画像対応ターミナルまたはローカルのデスクトッププレビューで動作する、
Codex 互換のアニメーションペットです。

> This is an independent project and is not affiliated with OpenAI.
>
> 本项目为独立项目，与 OpenAI 没有隶属关系。
>
> このプロジェクトは独立したものであり、OpenAI とは提携していません。

## Features / 功能 / 機能

- **English** — Load a local Codex pet package, preview its animation states,
  and render it in a compatible terminal.
- **中文** — 加载本地 Codex 宠物包，预览不同动画状态，并在兼容的终端中渲染。
- **日本語** — ローカルの Codex ペットパッケージを読み込み、アニメーションを
  プレビューし、対応ターミナルに描画します。

### Desktop GUI / 桌面 GUI / デスクトップ GUI

- Choose a local pet folder containing `pet.json` and its spritesheet.
- Switch among the animation states declared by the pet package.
- Drag the pet anywhere within the preview canvas.
- Copy a terminal command without launching a terminal or subprocess.

- 选择包含 `pet.json` 与精灵图的本地宠物文件夹。
- 切换宠物包声明的动画状态。
- 可用鼠标在预览画布中拖动精灵。
- 仅复制终端命令，不会启动终端或子进程。

- `pet.json` とスプライトシートを含むローカルのペットフォルダーを選択できます。
- ペットパッケージに定義されたアニメーション状態を切り替えられます。
- プレビューキャンバス内でペットをマウスでドラッグできます。
- ターミナルや子プロセスを起動せず、コマンドだけをコピーできます。

## Quick start / 快速开始 / クイックスタート

### 1. Install Rust / 安装 Rust / Rust をインストール

Install the stable Rust toolchain from [rustup](https://rustup.rs/).

请从 [rustup](https://rustup.rs/) 安装稳定版 Rust 工具链。

[rustup](https://rustup.rs/) から安定版 Rust ツールチェーンをインストールします。

### 2. Build and open the GUI / 构建并打开 GUI / ビルドして GUI を開く

```powershell
git clone https://github.com/Delta-13/Desktop-Pet.git
Set-Location Desktop-Pet
cargo run -- --gui
```

Running `cargo run` with no arguments also opens the GUI.

不带参数运行 `cargo run` 也会打开 GUI。

引数なしで `cargo run` を実行しても GUI が開きます。

### 3. Use a pet package / 使用宠物包 / ペットパッケージを使う

A package must contain `pet.json` plus the spritesheet it references, normally
`spritesheet.webp`. Choose its folder in the GUI, or render it in a terminal:

宠物包必须包含 `pet.json` 和它引用的精灵图，通常为 `spritesheet.webp`。
可在 GUI 中选择该文件夹，也可在终端中渲染：

ペットパッケージには `pet.json` と、そのファイルが参照するスプライトシート
（通常は `spritesheet.webp`）が必要です。GUI でフォルダーを選択するか、
ターミナルで描画します：

```powershell
cargo run -- "$env:USERPROFILE\.codex\pets\anpan" --state running
```

## Terminal support / 终端支持 / ターミナル対応

| Platform / 平台 / プラットフォーム | Supported terminal protocol / 支持的协议 / 対応プロトコル |
| --- | --- |
| Windows | Windows Terminal or another Sixel-capable terminal |
| macOS | iTerm2 3.6+, Kitty, Ghostty, or WezTerm |

The terminal renderer accepts both 8×9 (v1) and 8×11 (v2) Codex atlas layouts.
Use `--protocol kitty` or `--protocol sixel` to override auto detection.

终端渲染器兼容 8×9（v1）与 8×11（v2）两种 Codex 图集布局。可使用
`--protocol kitty` 或 `--protocol sixel` 覆盖自动检测。

ターミナルレンダラーは 8×9（v1）と 8×11（v2）の Codex アトラスレイアウトに
対応しています。自動検出を上書きするには `--protocol kitty` または
`--protocol sixel` を指定してください。

## Development / 开发 / 開発

```powershell
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

All pet data stays local. The app does not require an account or cloud sync.

所有宠物数据都保留在本地；程序不需要账户或云同步。

すべてのペットデータはローカルに保持され、アカウントやクラウド同期は不要です。

## License / 许可证 / ライセンス

Licensed under [Apache-2.0](LICENSE). See
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for upstream attribution.

本项目使用 [Apache-2.0](LICENSE) 许可证；上游归属请参见
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。

[Apache-2.0](LICENSE) ライセンスで提供されます。上流プロジェクトの帰属表示は
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) を参照してください。
