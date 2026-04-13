# driftwm-launcher

Minimal Wayland application launcher for driftwm compositor.

![CI](https://github.com/wwmaxik/driftwm-launcher/workflows/CI/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![GTK](https://img.shields.io/badge/GTK-4-green)
![License](https://img.shields.io/badge/license-GPLv3-blue)

## Features

- **Wayland-native** - uses gtk4-layer-shell for overlay window
- **Fuzzy search** - find apps quickly without exact matches
- **Fast and lightweight** - minimal dependencies
- **Application icons** - shows icons from .desktop files
- **Keyboard-driven** - Enter to launch, Escape to close
- **Terminal support** - automatically detects terminal apps

## Installation

### Requirements

- Rust 1.85+ (edition 2024)
- GTK4 development libraries
- gtk4-layer-shell

### Debian/Ubuntu

```bash
sudo apt install libgtk-4-dev libgtk4-layer-shell-dev build-essential
```

### Fedora

```bash
sudo dnf install gtk4-devel gtk4-layer-shell-devel rust cargo
```

### Build from source

```bash
git clone https://github.com/wwmaxik/driftwm-launcher.git
cd driftwm-launcher
cargo build --release
```

### Install

```bash
sudo make install
```

This installs the binary to `/usr/local/bin/driftwm-launcher`.

### Uninstall

```bash
sudo make uninstall
```

## Usage

Launch from terminal:
```bash
driftwm-launcher
```

Or bind to a key in driftwm config:
```toml
[bindings.key]
"Mod+d" = "spawn driftwm-launcher"
```

### Keyboard shortcuts

- **Type** - search applications
- **Enter** - launch selected app
- **Escape** - close launcher
- **Arrow keys** - navigate results

## Configuration

Config file: `~/.config/driftwm-launcher/config.toml`

```toml
width = 600
height = 400
position = "top"  # top, center, or bottom
margin_top = 100
```

## How it works

- Scans `.desktop` files from standard locations
- Uses fuzzy matching for search (powered by skim-matcher)
- Displays as overlay window on top of all other windows
- Automatically closes after launching an app

## Comparison with bemenu

| Feature | driftwm-launcher | bemenu |
|---------|------------------|--------|
| UI | GTK4 with icons | Text-only |
| Search | Fuzzy matching | Substring |
| Wayland | Native (layer-shell) | Yes |
| Configuration | TOML file | CLI args |
| Dependencies | GTK4 | Minimal |

## License

GPLv3

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

See also: [Code of Conduct](CODE_OF_CONDUCT.md)

## Credits

Built for [driftwm](https://github.com/malbiruk/driftwm) compositor.
