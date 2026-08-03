<div align="center">

# 🐾 Desktop Pet

**画像対応ターミナルまたはデスクトップで動く、Codex 互換のドラッグ可能なアニメーションペットです。**

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

[![ライセンス：Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-5B73D0?style=flat-square)](LICENSE)
[![リリース](https://img.shields.io/github/v/release/Delta-13/Desktop-Pet?style=flat-square&color=8B84FF)](https://github.com/Delta-13/Desktop-Pet/releases/latest)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![デスクトップ GUI](https://img.shields.io/badge/interface-デスクトップ%20GUI-8B84FF?style=flat-square)](#デスクトップ-gui)

</div>

---

## 概要

Desktop Pet はローカルの Codex ペットパッケージを読み込み、Codex 風の
デスクトップコントローラーでアニメーションをプレビューし、透明で常に手前に
表示されるドラッグ可能なデスクトップペットにもできます。対応ターミナルにも同じペットを描画でき、
すべてのデータはローカルに保持されます。アカウント、クラウド同期、ネットワーク接続は不要です。

> このプロジェクトは独立したものであり、OpenAI とは提携していません。

## 目次

- [機能](#機能)
- [v0.1.0 をダウンロード](#v010-をダウンロード)
- [クイックスタート](#クイックスタート)
- [デスクトップ GUI](#デスクトップ-gui)
- [ターミナル対応](#ターミナル対応)
- [開発](#開発)
- [ライセンス](#ライセンス)

## 機能

- `pet.json` とスプライトシートを含むローカルのペットフォルダーを読み込みます。
- インターフェース全体を English、简体中文、日本語で切り替えられます。
- ペットパッケージで定義された各アニメーション状態をプレビューできます。
- GUI のプレビューキャンバス内でペットをマウスでドラッグできます。
- 読み込んだペットを枠なし・透明・常に手前のデスクトップ相棒にできます。クリックで遊ぶ／手を振る、ドラッグ方向に歩く、右クリックで設定へ戻る操作に対応します。
- ペットのサイズを調整でき、待機中のランダムな短い散歩も有効にできます。
- ターミナルや子プロセスを起動せず、ターミナルコマンドだけをコピーできます。
- 対応ターミナルで Codex v1（8×9）および v2（8×11）のスプライトアトラスを描画します。

## v0.1.0 をダウンロード

| プラットフォーム | ダウンロード |
| --- | --- |
| Windows 10/11 · x86_64 | [Desktop-Pet-v0.1.0-windows-x86_64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-windows-x86_64.zip) |
| macOS 11+ · Intel | [Desktop-Pet-v0.1.0-macos-x86_64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-macos-x86_64.zip) |
| macOS 11+ · Apple Silicon | [Desktop-Pet-v0.1.0-macos-aarch64.zip](https://github.com/Delta-13/Desktop-Pet/releases/download/v0.1.0/Desktop-Pet-v0.1.0-macos-aarch64.zip) |

アーカイブを展開し、Windows では `terminal-sprite-pet.exe`、macOS では
`Desktop Pet.app` を起動してください。初回リリースのバイナリには商用コード署名がないため、
Windows SmartScreen の警告が表示される場合があります。macOS では初回のみ Control キーを
押しながらアプリをクリックし、**開く**を選択してください。

## クイックスタート

[rustup](https://rustup.rs/) から安定版 Rust ツールチェーンをインストールしてから、
次を実行します：

```powershell
git clone https://github.com/Delta-13/Desktop-Pet.git
Set-Location Desktop-Pet
cargo run -- --gui
```

引数なしで `cargo run` を実行しても GUI が開きます。

ローカルペットをデスクトップペットとして直接起動します：

```powershell
cargo run -- --desktop .\pets\anpan
```

## デスクトップ GUI

ローカルコントローラーでは English / 简体中文 / 日本語の切替、ペットパッケージの選択、
アニメーション状態の切替、プレビューキャンバス内でのドラッグができます。
**デスクトップペットにする**を選ぶと、読み込んだペットは枠なし・透明・常に手前の相棒になり、デスクトップにはペット本体だけが表示されます。
ペットをクリックすると遊ぶ／手を振る動きがランダムに選ばれ、ドラッグするとウィンドウを移動しながらその方向へ歩きます。
同じドラッグ操作中に左右を反転しても、最新のマウス移動方向へすぐに向き直ります。マウスを止めるかボタンを離すと、走るアニメーションもすぐに停止します。
しばらく操作しなければ休み、右クリックでコントローラーへ戻れます。コントローラーではサイズと待機中のランダム散歩を設定できます。
**Copy** ボタンはターミナルコマンドをクリップボードにコピーするだけで、
ターミナルやプロセスを起動しません。

## ターミナル対応

| プラットフォーム | 対応ターミナルプロトコル |
| --- | --- |
| Windows | Windows Terminal または Sixel 対応ターミナル |
| macOS | iTerm2 3.6+、Kitty、Ghostty、または WezTerm |

ターミナルでペットパッケージを描画します：

```powershell
cargo run -- "$env:USERPROFILE\.codex\pets\anpan" --state running
```

自動検出を上書きするには `--protocol kitty` または `--protocol sixel` を指定します。

## 開発

```powershell
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

## ライセンス

[Apache-2.0](LICENSE) ライセンスで提供されます。上流プロジェクトの帰属表示は
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) を参照してください。
