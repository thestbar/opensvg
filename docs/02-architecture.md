# OpenSVG Architecture Design

## Overview

OpenSVG is a minimal SVG editor with two interfaces:
1. **Desktop GUI** - Visual editor for interactive use
2. **CLI** - Command-line tool for scripting and automation

Both interfaces share the same core logic and are built with Tauri 2.0.

## Application Modes

```
                     OpenSVG
                        │
           ┌────────────┴────────────┐
           │                         │
      GUI Mode                   CLI Mode
    (Desktop App)            (Terminal Tool)
           │                         │
           └────────────┬────────────┘
                        │
               Shared Core Logic
              (SVG Operations)
```

---

## CLI Interface

### Commands

```bash
# Optimize/Minify an SVG file
opensvg optimize <file> [options]
opensvg opt <file>                    # short alias

# Change fill color of all elements
opensvg fill <file> <color> [options]

# Change stroke color of all elements
opensvg stroke <file> <color> [options]
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--output <path>` | `-o` | Output file (default: overwrite input) |
| `--stdout` | `-s` | Print result to stdout instead of file |
| `--quiet` | `-q` | Suppress status messages |
| `--help` | `-h` | Show help |
| `--version` | `-v` | Show version |

### Color Format

Supports standard CSS color formats:
- `#rgb` - 3-digit hex (e.g., `#fff`)
- `#rrggbb` - 6-digit hex (e.g., `#ffffff`)
- `#rrggbbaa` - 8-digit hex with alpha (e.g., `#ffffff80` = 50% opacity)
- `rgb(r,g,b)` - RGB values
- `rgba(r,g,b,a)` - RGBA with alpha (0-1)

### Usage Examples

```bash
# Minify an SVG (overwrites original)
opensvg optimize icon.svg

# Minify and save to new file
opensvg opt icon.svg -o icon.min.svg

# Change all fills to red
opensvg fill logo.svg "#ff0000"

# Change fills with 50% opacity, output to new file
opensvg fill logo.svg "#ff000080" -o logo-red.svg

# Change stroke color
opensvg stroke diagram.svg "#333333"

# Pipe optimized SVG to another tool
opensvg opt icon.svg --stdout | other-tool

# Batch process multiple files
for f in icons/*.svg; do
  opensvg opt "$f" -o "dist/$(basename $f)"
done
```

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      OpenSVG Application                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────┐     ┌─────────────────────────────┐    │
│  │      CLI Mode       │     │         GUI Mode            │    │
│  │                     │     │                             │    │
│  │  ┌───────────────┐  │     │  ┌───────────────────────┐  │    │
│  │  │ Arg Parser    │  │     │  │  Frontend (WebView)   │  │    │
│  │  │ (clap)        │  │     │  │                       │  │    │
│  │  └───────────────┘  │     │  │  - Toolbar            │  │    │
│  │         │           │     │  │  - Canvas             │  │    │
│  │         ▼           │     │  │  - Color Panel        │  │    │
│  │  ┌───────────────┐  │     │  │  - SVG Manager (TS)   │  │    │
│  │  │ CLI Commands  │  │     │  │                       │  │    │
│  │  │ - optimize    │  │     │  └───────────────────────┘  │    │
│  │  │ - fill        │  │     │             │               │    │
│  │  │ - stroke      │  │     │      Tauri IPC Bridge       │    │
│  │  └───────────────┘  │     │             │               │    │
│  └──────────┬──────────┘     └─────────────┬───────────────┘    │
│             │                              │                     │
│             └──────────────┬───────────────┘                     │
│                            │                                     │
│  ┌─────────────────────────▼─────────────────────────────────┐  │
│  │                  Core Library (Rust)                       │  │
│  │                                                            │  │
│  │  ┌────────────────┐  ┌────────────────┐  ┌──────────────┐ │  │
│  │  │ SVG Parser     │  │ SVG Optimizer  │  │ Color Utils  │ │  │
│  │  │ (quick-xml)    │  │ (usvg/resvg or │  │ (parse/apply)│ │  │
│  │  │                │  │  custom)       │  │              │ │  │
│  │  └────────────────┘  └────────────────┘  └──────────────┘ │  │
│  │                                                            │  │
│  │  ┌────────────────┐  ┌────────────────┐                   │  │
│  │  │ File I/O       │  │ Dialog         │                   │  │
│  │  │ (std::fs)      │  │ (Tauri plugin) │                   │  │
│  │  └────────────────┘  └────────────────┘                   │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Details

### 1. CLI Layer (Rust)

#### Argument Parser
Using `clap` crate for argument parsing:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "opensvg")]
#[command(about = "A simple SVG editor and optimizer")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Optimize/minify an SVG file
    #[command(alias = "opt")]
    Optimize {
        /// Input SVG file
        file: PathBuf,
        /// Output file (default: overwrite input)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Print to stdout
        #[arg(short, long)]
        stdout: bool,
    },
    /// Change fill color of all elements
    Fill {
        /// Input SVG file
        file: PathBuf,
        /// Color (hex: #rgb, #rrggbb, #rrggbbaa)
        color: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Change stroke color of all elements
    Stroke {
        /// Input SVG file
        file: PathBuf,
        /// Color (hex: #rgb, #rrggbb, #rrggbbaa)
        color: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
```

### 2. Core Library (Rust)

Shared between CLI and GUI modes:

```rust
pub mod svg_core {
    /// Parse and validate SVG content
    pub fn parse_svg(content: &str) -> Result<SvgDocument, Error>;

    /// Optimize/minify SVG
    pub fn optimize_svg(doc: &SvgDocument) -> Result<String, Error>;

    /// Change fill color of all elements
    pub fn set_fill_color(doc: &mut SvgDocument, color: &str) -> Result<(), Error>;

    /// Change stroke color of all elements
    pub fn set_stroke_color(doc: &mut SvgDocument, color: &str) -> Result<(), Error>;

    /// Export SVG to string
    pub fn to_string(doc: &SvgDocument) -> String;
}

pub mod color {
    /// Parse color string (supports #rgb, #rrggbb, #rrggbbaa, rgba())
    pub fn parse_color(input: &str) -> Result<Color, Error>;

    /// Convert color to SVG-compatible string
    pub fn to_svg_color(color: &Color) -> String;
}
```

### 3. GUI Frontend (TypeScript)

#### Components

**Toolbar**
- Open, Save, Save As buttons
- Optimize button (with size reduction display)

**Canvas**
- SVG preview and rendering
- Click-to-select elements
- Selection highlight

**Color Panel**
- Selected element info
- Fill color picker
- Stroke color picker
- Apply/Reset buttons

#### SVG Manager (TypeScript)

```typescript
interface SVGManager {
  loadSVG(content: string): void;
  selectElement(element: SVGElement): void;
  getSelectedElement(): SVGElement | null;
  getFillColor(element: SVGElement): string;
  getStrokeColor(element: SVGElement): string;
  setFillColor(element: SVGElement, color: string): void;
  setStrokeColor(element: SVGElement, color: string): void;
  getSVGString(): string;
  optimize(): Promise<string>;
}
```

---

## Data Flow

### CLI: Optimize Command
```
$ opensvg optimize icon.svg -o icon.min.svg
    │
    ▼
Parse arguments (clap)
    │
    ▼
Read file (std::fs::read_to_string)
    │
    ▼
Parse SVG (svg_core::parse_svg)
    │
    ▼
Optimize (svg_core::optimize_svg)
    │
    ▼
Write output (std::fs::write)
    │
    ▼
Print status: "Optimized: 4.2KB → 1.8KB (57% reduction)"
```

### CLI: Fill Command
```
$ opensvg fill logo.svg "#ff0000"
    │
    ▼
Parse arguments + validate color
    │
    ▼
Read file
    │
    ▼
Parse SVG
    │
    ▼
Apply fill color to all elements
    │
    ▼
Write output (overwrite original)
    │
    ▼
Print status: "Updated fill color to #ff0000"
```

### GUI: Interactive Color Change
```
User clicks element in canvas
    │
    ▼
JavaScript identifies clicked element
    │
    ▼
Color Panel shows current fill/stroke
    │
    ▼
User picks new color
    │
    ▼
User clicks "Apply"
    │
    ▼
SVG Manager updates element
    │
    ▼
Canvas re-renders
```

---

## File Structure

```
opensvg/
├── docs/
│   ├── 01-tech-stack-research.md
│   ├── 02-architecture.md
│   └── 03-implementation-tasks.md
│
├── src/                          # Frontend source (GUI)
│   ├── index.html
│   ├── main.ts
│   ├── styles/
│   │   └── main.css
│   ├── components/
│   │   ├── toolbar.ts
│   │   ├── canvas.ts
│   │   └── color-panel.ts
│   └── lib/
│       └── svg-manager.ts
│
├── src-tauri/                    # Rust source (CLI + Core)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   └── src/
│       ├── main.rs               # Entry point (CLI or GUI)
│       ├── lib.rs                # Tauri commands
│       ├── cli.rs                # CLI argument handling
│       └── core/
│           ├── mod.rs
│           ├── parser.rs         # SVG parsing
│           ├── optimizer.rs      # SVG optimization
│           └── color.rs          # Color utilities
│
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

---

## UI Layout (GUI Mode)

```
┌────────────────────────────────────────────────────────────────┐
│  [Open]  [Save]  [Save As]  [Optimize]        OpenSVG v1.0     │
├────────────────────────────────────────────────┬───────────────┤
│                                                │               │
│                                                │  Selected:    │
│                                                │  <path>       │
│                                                │               │
│                                                │  ┌─────────┐  │
│              SVG PREVIEW CANVAS                │  │  Fill   │  │
│                                                │  │ [color] │  │
│            (click to select elements)          │  └─────────┘  │
│                                                │               │
│                                                │  ┌─────────┐  │
│                                                │  │ Stroke  │  │
│                                                │  │ [color] │  │
│                                                │  └─────────┘  │
│                                                │               │
│                                                │  [Apply]      │
│                                                │  [Reset]      │
├────────────────────────────────────────────────┴───────────────┤
│  Status: Ready                          Size: 2.4 KB           │
└────────────────────────────────────────────────────────────────┘
```

---

## Rust Crate Dependencies

```toml
[dependencies]
tauri = { version = "2", features = ["dialog"] }
clap = { version = "4", features = ["derive"] }
quick-xml = "0.31"              # XML/SVG parsing
oxvg_optimiser = "0.1"          # SVG optimization (or custom)
csscolorparser = "0.6"          # Color parsing with alpha support
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"                 # Error handling
```

---

## Security Considerations

1. **File Access**: CLI only operates on specified files
2. **Path Validation**: Prevent directory traversal attacks
3. **SVG Sanitization**: Validate SVG content before processing
4. **No Network**: App works entirely offline
5. **Minimal Permissions**: Tauri capabilities restricted to necessary APIs

---

## Future Extensibility

- Batch mode for processing multiple files
- Additional CLI commands (e.g., `opensvg info`, `opensvg resize`)
- Export to PNG
- Dark mode theme
- Plugin system for custom operations
