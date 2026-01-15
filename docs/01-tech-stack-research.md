# Tech Stack Research: OpenSVG Desktop Application

## Project Requirements

- **Primary Platform**: macOS
- **Future Platform**: Linux
- **Core Features**:
  1. Open/view SVG files
  2. Change vector colors
  3. Minify/optimize SVG files
  4. Save modified SVGs

## Desktop Framework Options

### Option 1: Electron

**Pros:**
- Mature ecosystem with extensive documentation
- Full Node.js integration
- Consistent UI across all platforms (bundles Chromium)
- Used by VS Code, Slack, Discord

**Cons:**
- Large bundle size (~100MB+)
- High memory usage (~200-400MB on startup)
- Slower startup time (1-2 seconds)
- Overkill for simple applications

### Option 2: Tauri 2.0 (Recommended)

**Pros:**
- Extremely small bundle size (~2-10MB)
- Low memory footprint (~20-40MB)
- Fast startup (<0.5 seconds)
- Uses system webview (no bundled browser)
- Built on Rust (memory safety, performance)
- Native macOS and Linux support
- Active development (2.0 released late 2024)
- No Rust knowledge required for simple apps

**Cons:**
- Smaller ecosystem than Electron
- Potential webview inconsistencies across platforms
- Less mature than Electron

### Decision: **Tauri 2.0**

For a simple SVG editor, Tauri is the ideal choice. The app doesn't need Node.js packages, and the significant reduction in bundle size and memory usage aligns perfectly with the "keep it simple" philosophy.

---

## Frontend Framework Options

### Option 1: Vanilla HTML/CSS/TypeScript

**Pros:**
- Zero framework overhead
- Simple to understand and maintain
- No build complexity
- Perfect for small applications

**Cons:**
- Manual DOM manipulation
- No component reusability patterns

### Option 2: Svelte

**Pros:**
- Compiles to vanilla JS (small bundle)
- Simple, intuitive syntax
- Built-in reactivity
- Great developer experience

**Cons:**
- Additional build step
- Learning curve (though minimal)

### Option 3: React/Vue

**Pros:**
- Large ecosystem
- Component-based architecture

**Cons:**
- Overkill for this simple app
- Larger bundle size
- More complexity than needed

### Decision: **Vanilla TypeScript**

Given the simplicity of the application (essentially 3 features), vanilla TypeScript provides the cleanest solution without unnecessary abstraction. This also means fewer dependencies and easier maintenance.

---

## SVG Manipulation Libraries

### SVG.js (Recommended for manipulation)

- **Size**: ~16KB gzipped
- **Purpose**: Lightweight SVG manipulation and animation
- **Features**: Create, manipulate, animate SVG elements
- **License**: MIT
- **Weekly Downloads**: High adoption

Ideal for:
- Parsing SVG files
- Selecting elements by type/attribute
- Changing fill/stroke colors
- Modifying attributes

### SVGO (Recommended for optimization)

- **Size**: Moderate
- **Purpose**: SVG optimization/minification
- **Features**: Remove metadata, optimize paths, minify
- **License**: MIT
- **Weekly Downloads**: ~19 million/week
- **Current Version**: 4.0.0

Industry standard for SVG optimization. Removes:
- Editor metadata
- Comments
- Hidden elements
- Redundant attributes
- Unnecessary whitespace

---

## Final Tech Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Desktop Framework | Tauri 2.0 | Small, fast, cross-platform |
| Frontend | Vanilla TypeScript | Simple, no framework overhead |
| Styling | CSS (with CSS Variables) | Native, themeable |
| SVG Manipulation | SVG.js | Lightweight, full-featured |
| SVG Optimization | SVGO | Industry standard |
| Build Tool | Vite | Fast, modern, Tauri-compatible |
| Package Manager | npm/pnpm | Standard Node.js tooling |

---

## References

- [Electron vs Tauri Comparison - DoltHub](https://www.dolthub.com/blog/2025-11-13-electron-vs-tauri/)
- [Tauri vs Electron - RaftLabs](https://www.raftlabs.com/blog/tauri-vs-electron-pros-cons/)
- [Tauri 2.0 Official Documentation](https://v2.tauri.app/)
- [SVG.js GitHub](https://github.com/svgdotjs/svg.js)
- [SVGO GitHub](https://github.com/svg/svgo)
- [SVGO npm](https://www.npmjs.com/package/svgo)
