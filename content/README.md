# Stick

A minimal static site generator in Rust. Zero dependencies, single binary.

## Usage

```bash
# Add markdown files to content/
echo "# Hello World" > content/hello.md

# Build the site
cargo run

# Output in dist/
open dist/hello.html
```

## Markdown Support

| Syntax | Output |
|--------|--------|
| `# Heading` | `<h1>` |
| `## Heading` | `<h2>` |
| `**bold**` | `<strong>` |
| `*italic*` | `<em>` |
| `` `code` `` | `<code>` |

Paragraphs are separated by blank lines.

## Project Structure

```
src/
├── main.rs      # Entry point
├── markdown.rs  # Markdown → HTML
├── template.rs  # HTML template
└── site.rs      # Build orchestration
```

## Build

```bash
cargo build --release
```

## License

MIT
