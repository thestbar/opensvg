# OpenSVG

A lightweight desktop application for viewing, editing colors, and optimizing SVG files. Includes both a graphical interface and a command-line tool.

## Features

- **View SVGs** - Open and preview SVG files with a clean interface
- **Edit Colors** - Change fill and stroke colors on SVG elements
- **Optimize** - Minify SVGs by removing comments, metadata, and unnecessary whitespace
- **Dual Interface** - Use the GUI app or CLI tool based on your workflow
- **Drag & Drop** - Drop SVG files directly onto the canvas
- **Keyboard Shortcuts** - Quick access with Cmd+O, Cmd+S, Cmd+Shift+S

## Installation

### macOS

Download the latest `.dmg` from the releases page, open it, and drag OpenSVG to your Applications folder.

If MacOS blocks the app from opening, go to System Preferences > Security & Privacy and click "Open Anyway".

If this option is not available and instead MacOS states “OpenSVG is damaged and can’t be opened. You should move it to the Bin.”, open Terminal and run:

```bash
bashxattr -cr /Applications/OpenSVG.app # Adjust the path if you placed it elsewhere
```

Then try opening the app again.

### Build from Source

Requirements:
- Node.js 24.x or later
- Rust 1.70 or later
- macOS 10.15 or later

```bash
# Clone the repository
git clone https://github.com/yourusername/opensvg.git
cd opensvg

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## GUI Usage

1. **Open a file** - Click "Open" or press `Cmd+O` to select an SVG file
2. **Select elements** - Click on any shape in the canvas to select it
3. **Edit colors** - Use the color panel on the right to change fill and stroke colors
4. **Optimize** - Click "Optimize" to minify the SVG
5. **Save** - Click "Save" (`Cmd+S`) or "Save As" (`Cmd+Shift+S`)

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+O` | Open file |
| `Cmd+S` | Save |
| `Cmd+Shift+S` | Save As |

## CLI Usage

The CLI tool is available when running from the built binary or development mode.

```bash
cd src-tauri
cargo run -- <command>
```

### Commands

#### Optimize SVG

Minify an SVG file by removing unnecessary content.

```bash
# Optimize in place
opensvg optimize icon.svg

# Save to new file
opensvg optimize icon.svg -o icon.min.svg

# Output to stdout
opensvg optimize icon.svg --stdout

# Quiet mode (no status messages)
opensvg optimize icon.svg -q
```

#### Set Fill Color

Change the fill color of all elements in an SVG.

```bash
# Set fill to red
opensvg fill icon.svg "#ff0000"

# With alpha channel
opensvg fill icon.svg "#ff000080"

# Save to new file
opensvg fill icon.svg "#ff0000" -o icon-red.svg

# Using named colors
opensvg fill icon.svg "rebeccapurple"
```

#### Set Stroke Color

Change the stroke color of all elements in an SVG.

```bash
# Set stroke to blue
opensvg stroke icon.svg "#0000ff"

# Save to new file
opensvg stroke icon.svg "#0000ff" -o icon-outlined.svg
```

### Color Formats

The following color formats are supported:

- Hex: `#rgb`, `#rrggbb`, `#rrggbbaa`
- RGB/RGBA: `rgb(255, 0, 0)`, `rgba(255, 0, 0, 0.5)`
- Named colors: `red`, `blue`, `rebeccapurple`, etc.

## Project Structure

```
opensvg/
├── src/                    # Frontend (TypeScript)
│   ├── main.ts            # Application entry point
│   └── styles/            # CSS styles
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Entry point (CLI/GUI detection)
│   │   ├── lib.rs         # Tauri commands
│   │   ├── cli.rs         # CLI implementation
│   │   └── core/          # Core SVG processing
│   │       ├── parser.rs  # SVG parsing
│   │       ├── color.rs   # Color utilities
│   │       └── optimizer.rs # SVG optimization
│   └── Cargo.toml
└── package.json
```

## Tech Stack

- **Frontend**: TypeScript, Vite
- **Backend**: Rust, Tauri 2.0
- **SVG Parsing**: quick-xml
- **Color Handling**: csscolorparser
- **CLI**: clap

## License

MIT

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
