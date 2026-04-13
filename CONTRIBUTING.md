# Contributing to driftwm-launcher

Thank you for your interest in contributing to driftwm-launcher!

## Getting Started

### Prerequisites

- Rust 1.85+ (edition 2024)
- GTK4 development libraries
- gtk4-layer-shell
- Git

### Setting Up Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/driftwm-launcher.git
   cd driftwm-launcher
   ```

3. Install dependencies:
   
   **Debian/Ubuntu:**
   ```bash
   sudo apt install libgtk-4-dev libgtk4-layer-shell-dev build-essential
   ```
   
   **Fedora:**
   ```bash
   sudo dnf install gtk4-devel gtk4-layer-shell-devel rust cargo
   ```
   
   **Arch Linux:**
   ```bash
   sudo pacman -S rust gtk4 gtk4-layer-shell base-devel
   ```

4. Build and run:
   ```bash
   cargo build
   cargo run
   ```

## Development Workflow

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy before committing: `cargo clippy`
- Write self-documenting code with clear names
- Keep functions small and focused

### Project Structure

```
driftwm-launcher/
├── src/
│   ├── main.rs        # UI and main application logic
│   ├── app_entry.rs   # .desktop file parsing
│   ├── launcher.rs    # Search and launch logic
│   └── config.rs      # Configuration handling
├── style.css          # GTK CSS styling
└── config.toml.example
```

### Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the code style

3. Test your changes:
   ```bash
   cargo build
   cargo clippy
   cargo fmt
   cargo run
   ```

4. Commit with a clear message:
   ```bash
   git commit -m "Add feature: description"
   ```

5. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

6. Open a Pull Request on GitHub

### Commit Message Guidelines

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- First line should be 50 characters or less
- Reference issues when relevant

Examples:
```
Add history tracking for launched apps
Fix crash when .desktop file is malformed
Update README with new keybindings
```

## Feature Ideas

- **History tracking** - remember recently launched apps
- **Favorites** - pin frequently used apps to top
- **Custom keybindings** - configurable shortcuts
- **Themes** - light/dark theme support
- **Categories** - filter apps by category
- **Plugins** - extensibility system

## Testing

Currently manual testing:

1. Build and run the launcher
2. Test search functionality
3. Test launching different types of apps
4. Test keyboard navigation
5. Test configuration loading

## Continuous Integration

All pull requests are automatically tested via GitHub Actions:

- Code formatting check (`cargo fmt`)
- Linting with clippy (`cargo clippy`)
- Build on Ubuntu
- Run tests (`cargo test`)

Make sure your code passes all checks before submitting a PR.

## Reporting Issues

When reporting issues, please include:

- Operating system and version
- GTK4 version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages

## Questions?

Feel free to open an issue for questions or discussions about contributing.

## License

By contributing to driftwm-launcher, you agree that your contributions will be licensed under the GPLv3 license.
