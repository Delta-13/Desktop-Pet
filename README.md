# Terminal Sprite Pet

Run a Codex-compatible animated pet in an image-capable terminal. This is the
small, standalone terminal-rendering portion of Codex: it loads a local
`pet.json` plus spritesheet, slices the animation frames in memory, and draws
them with Kitty graphics or Sixel.

It deliberately does not include Codex, account/config integration, built-in
asset downloads, or a desktop overlay. The optional GUI is a local controller
for choosing a pet and previewing its animation states.

## Desktop GUI

Launch the app with no arguments (or pass `--gui`) to open a local desktop
controller. It lets you choose a pet folder, switch its animation states,
drag the preview within the canvas, and preview the sprite in a
Codex-inspired dark interface. The **Copy** control only copies a terminal
command; it never launches a terminal or process.

```powershell
terminal-sprite-pet --gui
```

## Install

```powershell
cargo install --path .
```

## Run

Point it at a directory containing a Codex custom pet package:

```powershell
terminal-sprite-pet "$env:USERPROFILE\.codex\pets\anpan"
terminal-sprite-pet "$env:USERPROFILE\.codex\pets\anpan" --state running
terminal-sprite-pet "$env:USERPROFILE\.codex\pets\anpan" --state review --duration 10
```

The package must contain `pet.json` and its relative spritesheet (normally
`spritesheet.webp`). Both 8x9 (v1) and 8x11 (v2) Codex atlas layouts are
accepted. Press `Ctrl+C` to stop a pet with no `--duration`.

## Platform support

| Platform | Auto-selected terminal protocol |
| --- | --- |
| Windows | Windows Terminal / a Sixel-capable terminal |
| macOS | iTerm2 3.6+, Kitty, Ghostty, or WezTerm |

Use `--protocol kitty` or `--protocol sixel` to override detection. The tool
refuses tmux and Zellij in auto mode because terminal images are not reliably
pane-local there.

## Development

```powershell
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

The GitHub Actions workflow builds and tests Windows and macOS on every push.

## License and origin

Apache-2.0. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for the
upstream Codex attribution. This project is not affiliated with OpenAI.
