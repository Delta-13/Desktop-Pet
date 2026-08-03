<div align="center">

# 🐾 Desktop Pet

**画像対応ターミナルまたはローカルのデスクトッププレビューで動作する、Codex 互換のアニメーションペットです。**

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

[![ライセンス：Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-5B73D0?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![デスクトップ GUI](https://img.shields.io/badge/interface-デスクトップ%20GUI-8B84FF?style=flat-square)](#デスクトップ-gui)

</div>

---

## 概要

Desktop Pet はローカルの Codex ペットパッケージを読み込み、Codex 風の
デスクトップコントローラーでアニメーションをプレビューし、対応ターミナルにも
同じペットを描画できます。すべてのデータはローカルに保持され、アカウント、
クラウド同期、ネットワーク接続は不要です。

> このプロジェクトは独立したものであり、OpenAI とは提携していません。

## 目次

- [機能](#機能)
- [クイックスタート](#クイックスタート)
- [デスクトップ GUI](#デスクトップ-gui)
- [ターミナル対応](#ターミナル対応)
- [開発](#開発)
- [ライセンス](#ライセンス)

## 機能

- `pet.json` とスプライトシートを含むローカルのペットフォルダーを読み込みます。
- ペットパッケージで定義された各アニメーション状態をプレビューできます。
- GUI のプレビューキャンバス内でペットをマウスでドラッグできます。
- ターミナルや子プロセスを起動せず、ターミナルコマンドだけをコピーできます。
- 対応ターミナルで Codex v1（8×9）および v2（8×11）のスプライトアトラスを描画します。

## クイックスタート

[rustup](https://rustup.rs/) から安定版 Rust ツールチェーンをインストールしてから、
次を実行します：

```powershell
git clone https://github.com/Delta-13/Desktop-Pet.git
Set-Location Desktop-Pet
cargo run -- --gui
```

引数なしで `cargo run` を実行しても GUI が開きます。

## デスクトップ GUI

ローカルコントローラーでは、ペットパッケージの選択、アニメーション状態の切替、
プレビューキャンバス内でのアニメーションスプライトのドラッグができます。
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
