# Sky Language Support for Zed

[![Zed Extension](https://img.shields.io/badge/Zed-Extension-blue)](https://zed.dev)
[![Sky Language](https://img.shields.io/badge/Sky-Language-orange)](https://github.com/SkyDevSoft/Sky)

This extension provides comprehensive language support for the **Sky** programming language in the [Zed Editor](https://zed.dev).

## Features

- 🎨 **Syntax Highlighting** for `.sky` and `.skyi` files
- 🧠 **Language Server Protocol (LSP)** support (requires `sky` binary with LSP)
- ✂️ **Code Snippets** for common patterns
- 🔧 **Auto-formatting** and indentation support
- 📝 **Bracket auto-closing** for `{}`, `[]`, `()`, and `""`
- 🔍 **Outline view** and symbol navigation

## Installation

This extension requires building from source. There are two ways to install it:

### Option 1: Manual Installation

#### Prerequisites

- [Rust](https://rustup.rs/) with `wasm32-wasip1` target
- [Node.js](https://nodejs.org/) and npm (for building the grammar)
- [Tree-sitter CLI](https://github.com/tree-sitter/tree-sitter/blob/master/cli/README.md)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-wasip1

# Install tree-sitter CLI
npm install -g tree-sitter-cli
```

#### Build from Source

```bash
# Clone the repository
git clone https://github.com/TheGB0077/sky-zed.git
cd sky-zed

# Build the tree-sitter grammar
git clone https://github.com/anzellai/tree-sitter-sky.git /tmp/tree-sitter-sky
cd /tmp/tree-sitter-sky
git checkout 15209fbff9675d618f6e89ed1a58944d053ba82e
tree-sitter build --wasm
cd -

# Copy the grammar to the extension
mkdir -p grammars
cp /tmp/tree-sitter-sky/tree-sitter-sky.wasm grammars/sky.wasm

# Build the extension WASM
cargo build --release --target wasm32-wasip1

# Copy the extension WASM
cp target/wasm32-wasip1/release/sky.wasm sky.wasm

# Clean up build artifacts
rm -rf target src Cargo.toml Cargo.lock
```

#### Install to Zed

**macOS:**
```bash
# Copy to Zed extensions directory
cp -r . ~/Library/Application\ Support/Zed/extensions/installed/sky
```

**Linux:**
```bash
# Copy to Zed extensions directory
cp -r ~/.local/share/zed/extensions/installed/sky
```

Then restart Zed or run `Reload Window` from the Command Palette.

### Option 2: Development Mode

To develop or modify the extension:

```bash
# Clone the repository
git clone https://github.com/TheGB0077/sky-zed.git
cd sky-zed

# Build the grammar (see Option 1 above)
# ...

# Copy to Zed dev extensions
mkdir -p ~/.config/zed/extensions
cp -r . ~/.config/zed/extensions/sky
```

Then restart Zed and enable "Developer Mode" in extensions settings.

## Configuration

### Language Server

To enable LSP features (autocomplete, go-to-definition, etc.), you need the `sky` binary with LSP support in your PATH:

```bash
# Verify sky is installed
which sky

# Verify LSP is available
sky lsp --help
```

The extension will automatically detect and start the language server.

### File Associations

The extension automatically activates for files with these extensions:
- `.sky` - Sky source files
- `.skyi` - Sky interface files

## Project Structure

```
sky-zed/
├── extension.toml          # Extension manifest
├── sky.wasm               # Compiled extension (WASM)
├── grammars/
│   └── sky.wasm           # Tree-sitter grammar (WASM)
├── languages/
│   └── sky/               # Language configuration
│       ├── config.toml    # Language settings
│       ├── highlights.scm # Syntax highlighting queries
│       ├── indents.scm    # Indentation rules
│       ├── locals.scm     # Local variable tracking
│       ├── outline.scm    # Document outline
│       ├── overrides.scm  # Override rules
│       └── tags.scm       # Tag definitions
└── snippets/
    └── sky.json           # Code snippets
```

## Development

### Building the Extension

```bash
# Build grammar
cd /tmp
git clone https://github.com/anzellai/tree-sitter-sky.git
cd tree-sitter-sky
git checkout 15209fbff9675d618f6e89ed1a58944d053ba82e
tree-sitter build --wasm

# Build extension
cd ~/path/to/sky-zed
cargo build --release --target wasm32-wasip1
```

### Testing Changes

1. Build the extension
2. Copy to Zed extensions directory
3. Restart Zed or run `Reload Window`
4. Open a `.sky` file to test

## Troubleshooting

### Extension shows as "Unknown"

1. Ensure both WASM files exist:
   - `sky.wasm` (extension)
   - `grammars/sky.wasm` (grammar)

2. Verify file permissions:
   ```bash
   ls -la *.wasm grammars/*.wasm
   ```

3. Restart Zed completely

### LSP not working

1. Verify `sky` binary is in PATH:
   ```bash
   which sky
   ```

2. Check LSP is available:
   ```bash
   sky lsp --help
   ```

3. Check Zed logs for errors:
   - Open Command Palette → `zed: open log`

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

This extension is open source. See the repository for license details.

## Credits

- **Extension Author:** Gabriel Lima (@TheGB0077)
- **Tree-sitter Grammar:** [anzellai/tree-sitter-sky](https://github.com/anzellai/tree-sitter-sky)
- **Sky Language:** [SkyDevSoft/Sky](https://github.com/SkyDevSoft/Sky)

## Support

- 🐛 **Bug Reports:** [Open an Issue](https://github.com/TheGB0077/sky-zed/issues)
- 💡 **Feature Requests:** [Open an Issue](https://github.com/TheGB0077/sky-zed/issues)
- 📖 **Sky Language:** [Sky Documentation](https://github.com/SkyDevSoft/Sky)
