# Changelog

All notable changes to OpenSVG will be documented in this file.

## [0.1.0] - 2026-01-16

### Initial Release

OpenSVG is a lightweight desktop application for viewing, editing colors, and optimizing SVG files.

### Features

#### GUI Application
- **SVG Viewer** - Open and preview SVG files with a clean, dark-themed interface
- **Element Selection** - Click on any shape to select it and view its properties
- **Color Editing** - Change fill and stroke colors using a color picker or hex input
- **Apply to All** - Apply color changes to all elements in the SVG
- **SVG Optimization** - Minify SVGs by removing comments, metadata, and whitespace
- **File Operations** - Open, Save, and Save As with native file dialogs
- **Drag & Drop** - Drop SVG files directly onto the canvas to open them
- **Unsaved Changes Warning** - Confirmation dialog when closing with unsaved changes
- **Loading States** - Visual feedback during file operations
- **Toast Notifications** - Success, error, and warning messages
- **Status Bar** - Shows current file name, size, and status messages

#### Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| `Cmd+O` / `Ctrl+O` | Open file |
| `Cmd+S` / `Ctrl+S` | Save |
| `Cmd+Shift+S` / `Ctrl+Shift+S` | Save As |

#### CLI Tool
- **`opensvg optimize <file>`** - Minify SVG files
  - `-o, --output <file>` - Save to a different file
  - `--stdout` - Output to stdout
  - `-q, --quiet` - Suppress status messages
- **`opensvg fill <file> <color>`** - Set fill color on all elements
- **`opensvg stroke <file> <color>`** - Set stroke color on all elements

#### Color Support
- Hex formats: `#rgb`, `#rrggbb`, `#rrggbbaa`
- RGB/RGBA: `rgb(255, 0, 0)`, `rgba(255, 0, 0, 0.5)`
- Named colors: `red`, `blue`, `rebeccapurple`, etc.

### Technical Details

- **Frontend**: TypeScript, Vite 7.3.1
- **Backend**: Rust, Tauri 2.0
- **SVG Parsing**: quick-xml 0.37
- **Color Handling**: csscolorparser 0.7
- **CLI**: clap 4

### System Requirements

- macOS 10.15 (Catalina) or later
- Apple Silicon (aarch64) or Intel (x86_64)

### Known Limitations

- Linux support planned for future release
- Windows support planned for future release
- Large SVG files (>5MB) may have reduced performance

---

[0.1.0]: https://github.com/yourusername/opensvg/releases/tag/v0.1.0
