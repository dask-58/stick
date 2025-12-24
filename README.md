# Stick

Minimal static site generator in Rust. Zero dependencies, single binary.

## Install

### Homebrew (macOS)

```bash
brew tap dask-58/stick https://github.com/dask-58/stick
brew install stick
```

### Build from Source

```bash
git clone https://github.com/dask-58/stick.git
cd stick
cargo build --release
sudo cp target/release/stick /usr/local/bin/
```

### Cargo

```bash
cargo install --path .
```

## Usage

```bash
stick                      # content/ -> dist/
stick ./docs               # docs/ -> dist/
stick -i src -o public     # src/ -> public/
```

## Options

```
-i, --input <DIR>     Input directory (default: content)
-o, --output <DIR>    Output directory (default: dist)
-h, --help            Print help
-V, --version         Print version
```

## Example

Create markdown files in `content/`:

```markdown
# My Page

This is **bold** and *italic* text.

## Lists

- Item one
- Item two

| Column A | Column B |
|----------|----------|
| Data 1   | Data 2   |
```

Run `stick` and open `dist/my-page.html`.

## Markdown Support

| Syntax | Output |
|--------|--------|
| `#` - `####` | Headings |
| `**text**` | Bold |
| `*text*` | Italic |
| `` `text` `` | Inline code |
| `[text](url)` | Link |
| `- item` | Unordered list |
| `1. item` | Ordered list |
| `> text` | Blockquote |
| `---` | Horizontal rule |
| ` ``` ` | Code block |
| `\| a \| b \|` | Table |

## Troubleshooting

### Update to latest version

```bash
brew untap dask-58/stick
brew tap dask-58/stick https://github.com/dask-58/stick
brew install stick
```

### Tap not found

Always include the full URL:
```bash
brew tap dask-58/stick https://github.com/dask-58/stick
```

### Reinstall

```bash
brew uninstall stick
brew install stick
```

### Check version

```bash
stick --version
```

## License

MIT
