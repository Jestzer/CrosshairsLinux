# CrosshairsLinux
**Please disable any anti-cheat software before using this, if the software allows you to do so. Using this software can get you banned.**

Linux equivalent of [Crosshairs](https://github.com/Jestzer/Crosshairs), built for Wayland.

- Puts an adjustable crosshair overlay in the center of your screen.
- The overlay is transparent and click-through — it won't interfere with your mouse input.
- Use the interface window's arrow buttons or your keyboard arrow keys (hold to repeat) to reposition the crosshair.
- Position is automatically saved when you close the interface window and restored on next launch.
- Works on Wayland compositors that support `zwlr_layer_shell_v1` (KDE Plasma 6, Sway, Hyprland, etc.). Does **not** work on GNOME/Mutter.

## Building

Requires Rust and GTK4/gtk4-layer-shell development libraries.

### Fedora
```bash
sudo dnf install gtk4-devel gtk4-layer-shell-devel
cargo build --release
```

### Arch
```bash
sudo pacman -S gtk4 gtk4-layer-shell
cargo build --release
```

The compiled binary will be at `target/release/crosshairs-linux`.

## Usage

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/crosshairs-linux
```
