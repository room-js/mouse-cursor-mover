# mouse-cursor-mover

A lightweight **macOS (Apple Silicon)** menu-bar utility that keeps your Mac awake by smoothly moving the mouse cursor every 5 seconds.

## Features

- Lives entirely in the macOS menu bar — no Dock icon
- Two tray-icon states loaded from assets:
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

## Usage

After launch, your configured tray icon appears in the macOS menu bar.

| Action | Effect |
|--------|--------|
| Click icon → **Start** / **Stop** (`Cmd+Option+S`) | Toggles between running and stopped |
| Click icon → **Support project** | Opens `https://buymeacoffee.com/roomjs` |
| Click icon → **Quit** | Exits the app cleanly |

## Custom tray icons

You can provide your own icons for the two states by placing PNG files here:

- `assets/icon-running.png`
- `assets/icon-stopped.png`

Optional Retina variants (preferred when present):

- `assets/icon-running@2x.png`
- `assets/icon-stopped@2x.png`

Behavior:

- Icon files are required.
- If `@2x` and non-`@2x` both exist, `@2x` is used.
- If a required icon file is missing or cannot be decoded, the app fails fast at startup with an error.
- Running and stopped state switch the tray icon between the two configured assets.

Tips:

- Use square PNGs.
- For best Retina quality use 36x36 for `@2x` (logical menu-bar icon is ~18pt high).
- Non-`@2x` can be 18x18.
- Keep transparent background for menu-bar clarity.
- Icons are rendered as macOS template images (system-tinted). Use a single-color glyph with transparency, and avoid baked black backgrounds.

## Notes

- The app requests no special permissions unless the system prompts for *Accessibility* access (required by CoreGraphics to synthesise mouse events). Grant it in **System Settings → Privacy & Security → Accessibility**.
- The cursor is moved using `CGEventPost`, which is the same low-level API used by macOS itself, so it reliably prevents display sleep and screen-saver activation.
