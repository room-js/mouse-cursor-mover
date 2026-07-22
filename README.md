# mouse-cursor-mover

A lightweight **macOS (Apple Silicon)** menu-bar utility that keeps your Mac awake by randomly moving the mouse cursor every 5 seconds.

## Features

- Lives entirely in the macOS menu bar — no Dock icon
- Two distinct tray icons:
  - **● (green filled circle)** — running
  - **○ (gray ring)** — stopped
- Menu with three items: **Start**, **Stop**, **Quit**
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

## Usage

After launch a small circle icon appears in the macOS menu bar.

| Action | Effect |
|--------|--------|
| Click icon → **Start** | Begins moving the cursor every 5 s; icon turns green |
| Click icon → **Stop**  | Pauses cursor movement; icon turns gray |
| Click icon → **Quit**  | Exits the application |

## Notes

- The app requests no special permissions unless the system prompts for *Accessibility* access (required by CoreGraphics to synthesise mouse events). Grant it in **System Settings → Privacy & Security → Accessibility**.
- The cursor is moved using `CGEventPost`, which is the same low-level API used by macOS itself, so it reliably prevents display sleep and screen-saver activation.
