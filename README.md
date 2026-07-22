# mouse-cursor-mover

A lightweight **macOS (Apple Silicon + Intel)** menu-bar utility that keeps your Mac awake by smoothly moving the mouse cursor every 5 seconds.

## Features

- Lives entirely in the macOS menu bar — no Dock icon
- Two tray-icon states embedded at build time:
  - **Running icon**
  - **Stopped icon**
- Menu with three items: **Start/Stop** toggle, **Support project**, and **Quit**
- Global toggle shortcut: **Cmd+Option+S** (also shown natively in the menu)
- Cursor moves to a random screen position every 5 seconds while active
- Zero external runtime dependencies; uses CoreGraphics directly for mouse control

## Requirements

- macOS on Apple Silicon (aarch64) or Intel (x86_64)
- Xcode Command Line Tools: `xcode-select --install`
- Rust toolchain: <https://rustup.rs>

## Build

```sh
# Debug build
cargo build

# Optimised release build (recommended)
cargo build --release
```

The compiled binary is at `target/release/mouse-cursor-mover`.

## Run

```sh
cargo run --release
```

Or copy the binary anywhere on your `$PATH` and launch it directly:

```sh
cp target/release/mouse-cursor-mover /usr/local/bin/
mouse-cursor-mover &
```

## Prebuilt Downloads

GitHub Releases publish prebuilt downloads for both macOS architectures:

- Apple Silicon: `aarch64-apple-darwin`
- Intel: `x86_64-apple-darwin`

Available artifact types:

- `*.dmg` (recommended): drag-and-drop installer image with `.app` + `Applications` shortcut
- `*.app.zip`: zipped `.app` bundle
- `*.tar.gz`: standalone CLI binary archive

Download from the repository's **Releases** page and choose the asset matching your machine architecture.

### Drag-and-drop install (Applications)

1. Download the `*.dmg` for your architecture.
2. Open it and drag `Mouse Cursor Mover.app` into `Applications`.
3. Launch from `Applications` (or Spotlight).

### Running from `*.tar.gz`

1. Extract the archive.
2. Run the binary directly.

## Usage

After launch, the tray icon appears in the macOS menu bar.

| Action | Effect |
|--------|--------|
| Click icon → **Start** / **Stop** (`Cmd+Option+S`) | Toggles between running and stopped |
| Click icon → **Support project** | Opens `https://buymeacoffee.com/roomjs` |
| Click icon → **Quit** | Exits the app cleanly |

## Notes

- The app requests no special permissions unless the system prompts for *Accessibility* access (required by CoreGraphics to synthesise mouse events). Grant it in **System Settings → Privacy & Security → Accessibility**.
- The cursor is moved using `CGEventPost`, which is the same low-level API used by macOS itself, so it reliably prevents display sleep and screen-saver activation.
