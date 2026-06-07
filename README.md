# markdown-preview

A lightweight, zero-dependency markdown previewer for Linux. Render any `.md` file as a polished HTML page and open it in your browser — from the command line **or** your file manager's right-click menu.

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT">
  <img src="https://img.shields.io/badge/rust-2024-orange.svg" alt="Rust 2024">
  <img src="https://img.shields.io/badge/platform-Linux-lightgrey.svg" alt="Platform: Linux">
</p>

## Features

- **GitHub-flavored markdown** — tables, strikethrough, task lists, footnotes, heading attributes
- **Dark mode** — automatic via `prefers-color-scheme`
- **GitHub-style CSS** — clean, familiar rendering that matches GitHub's look
- **File manager integration** — right-click any `.md` file to preview
- **Zero runtime dependencies** — single static binary
- **CLI + GUI** — use from terminal or desktop

## Supported File Managers

| File Manager | Desktop Environment | Integration |
|---|---|---|
| **Nautilus** | GNOME | Python extension |
| **Dolphin** | KDE Plasma | Service menu |
| **Nemo** | Cinnamon | `.nemo_action` |
| **Thunar** | XFCE | Custom action (UCA) |

The installer auto-detects which file managers are installed and configures only the relevant ones.

## Installation

### Quick install

```bash
git clone https://github.com/danvincent/markdown-preview.git
cd markdown-preview
./install.sh
```

The installer will:

1. Build the binary in release mode
2. Install it to `~/.local/bin/`
3. Register a `.desktop` entry for MIME associations
4. Add right-click context menus for detected file managers

### Manual install

```bash
cargo build --release
cp target/release/markdown-preview ~/.local/bin/
```

### Requirements

- **Rust** toolchain (`cargo`) — install via [rustup](https://rustup.rs/)
- **Linux** desktop environment
- A web browser (any, opened via `xdg-open`)

## Usage

### Command line

```bash
markdown-preview README.md
```

Generates a styled HTML file in `/tmp/` and opens it in your default browser.

### File manager

Right-click any `.md`, `.markdown`, `.mdown`, or `.mkd` file and select **"Open in Markdown Preview"**.

## Library Usage

`markdown-preview` also exposes a small public API for embedding markdown rendering in other Rust projects:

```rust
use md_viewer::{render_markdown, markdown_to_html_fragment, build_html};

// Full document
let html = render_markdown("# Hello\n\n**World**", "My Doc");

// Just the HTML fragment
let fragment = markdown_to_html_fragment("# Hello");

// Custom HTML template
let html = build_html("Custom Title", &fragment);
```

## Project Structure

```
├── src/
│   ├── lib.rs      # Core rendering logic + unit tests
│   └── main.rs     # CLI entry point
├── install.sh      # Multi-DE installer script
├── Cargo.toml
└── LICENSE
```

## Contributing

Pull requests welcome! For bugs or feature requests, please open an issue.

## License

[MIT](LICENSE)
