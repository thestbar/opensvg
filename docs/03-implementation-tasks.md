# OpenSVG Implementation Tasks

This document breaks down the implementation into discrete, manageable tasks. Each task is designed to be completed independently while building toward the full application.

---

## Phase 1: Project Setup

### Task 1.1: Initialize Tauri Project
- [ ] Install Tauri CLI and prerequisites
- [ ] Create new Tauri project with Vite + TypeScript template
- [ ] Verify project builds and runs on macOS
- [ ] Configure basic project metadata (name, version, author)

### Task 1.2: Configure Development Environment
- [ ] Set up TypeScript configuration
- [ ] Configure Vite for development
- [ ] Set up basic CSS structure
- [ ] Configure Tauri capabilities (file system, dialog)

### Task 1.3: Add Rust Dependencies
- [ ] Add `clap` for CLI argument parsing
- [ ] Add `quick-xml` for SVG parsing
- [ ] Add `csscolorparser` for color handling
- [ ] Add `thiserror` for error handling
- [ ] Verify all crates compile

---

## Phase 2: Core Library (Rust)

### Task 2.1: SVG Parser Module
- [ ] Create `src-tauri/src/core/mod.rs`
- [ ] Create `src-tauri/src/core/parser.rs`
- [ ] Implement `parse_svg()` function to load SVG content
- [ ] Implement `to_string()` to export SVG
- [ ] Add basic SVG validation
- [ ] Write unit tests for parser

### Task 2.2: Color Utilities Module
- [ ] Create `src-tauri/src/core/color.rs`
- [ ] Implement `parse_color()` supporting:
  - `#rgb` format
  - `#rrggbb` format
  - `#rrggbbaa` format (with alpha)
  - `rgba(r,g,b,a)` format
- [ ] Implement `to_svg_color()` for output
- [ ] Write unit tests for color parsing

### Task 2.3: SVG Color Modification
- [ ] Implement `set_fill_color()` function
- [ ] Implement `set_stroke_color()` function
- [ ] Handle elements with existing fill/stroke
- [ ] Handle elements without fill/stroke (add attribute)
- [ ] Handle groups and nested elements
- [ ] Write unit tests

### Task 2.4: SVG Optimizer Module
- [ ] Create `src-tauri/src/core/optimizer.rs`
- [ ] Implement `optimize_svg()` function
- [ ] Remove unnecessary whitespace
- [ ] Remove comments
- [ ] Remove metadata elements
- [ ] Simplify path data (optional)
- [ ] Write unit tests with before/after comparisons

---

## Phase 3: CLI Implementation

### Task 3.1: CLI Argument Structure
- [ ] Create `src-tauri/src/cli.rs`
- [ ] Define CLI struct with clap derive macros
- [ ] Define `Optimize` subcommand with options
- [ ] Define `Fill` subcommand with options
- [ ] Define `Stroke` subcommand with options
- [ ] Add command aliases (`opt` for `optimize`)

### Task 3.2: CLI Entry Point
- [ ] Modify `main.rs` to detect CLI vs GUI mode
- [ ] Implement CLI dispatch logic
- [ ] Handle `--help` and `--version`
- [ ] Implement proper exit codes

### Task 3.3: Optimize Command Implementation
- [ ] Read input file
- [ ] Parse SVG
- [ ] Run optimization
- [ ] Calculate size reduction percentage
- [ ] Write output (file or stdout)
- [ ] Print status message (unless quiet mode)
- [ ] Handle errors gracefully

### Task 3.4: Fill Command Implementation
- [ ] Read input file
- [ ] Validate color argument
- [ ] Parse SVG
- [ ] Apply fill color to all elements
- [ ] Write output
- [ ] Print status message
- [ ] Handle errors gracefully

### Task 3.5: Stroke Command Implementation
- [ ] Read input file
- [ ] Validate color argument
- [ ] Parse SVG
- [ ] Apply stroke color to all elements
- [ ] Write output
- [ ] Print status message
- [ ] Handle errors gracefully

### Task 3.6: CLI Testing
- [ ] Test `optimize` on various SVG files
- [ ] Test `fill` with different color formats
- [ ] Test `stroke` with different color formats
- [ ] Test `--output` option
- [ ] Test `--stdout` option
- [ ] Test error handling (invalid file, bad color)

---

## Phase 4: GUI Frontend

### Task 4.1: Basic Layout
- [ ] Create HTML structure in `index.html`
- [ ] Create `src/styles/main.css`
- [ ] Implement responsive layout grid
- [ ] Style toolbar area
- [ ] Style canvas area
- [ ] Style side panel area
- [ ] Style status bar

### Task 4.2: Toolbar Component
- [ ] Create `src/components/toolbar.ts`
- [ ] Add "Open" button with icon
- [ ] Add "Save" button
- [ ] Add "Save As" button
- [ ] Add "Optimize" button
- [ ] Style buttons with hover states
- [ ] Wire up event listeners (placeholders)

### Task 4.3: Canvas Component
- [ ] Create `src/components/canvas.ts`
- [ ] Create SVG container element
- [ ] Implement `renderSVG()` function
- [ ] Implement click-to-select functionality
- [ ] Add visual selection highlight (outline/border)
- [ ] Handle empty state (no file loaded)

### Task 4.4: Color Panel Component
- [ ] Create `src/components/color-panel.ts`
- [ ] Display selected element type
- [ ] Add fill color input (native color picker)
- [ ] Add stroke color input
- [ ] Add "Apply" button
- [ ] Add "Reset" button
- [ ] Handle no selection state

### Task 4.5: SVG Manager
- [ ] Create `src/lib/svg-manager.ts`
- [ ] Implement `loadSVG()` to parse and store SVG
- [ ] Implement `selectElement()` with state tracking
- [ ] Implement `getFillColor()` / `getStrokeColor()`
- [ ] Implement `setFillColor()` / `setStrokeColor()`
- [ ] Implement `getSVGString()` for export
- [ ] Implement `optimize()` (calls Rust backend)

### Task 4.6: Status Bar
- [ ] Create status bar element
- [ ] Show current file name
- [ ] Show file size
- [ ] Show status messages (Ready, Saved, Optimized)
- [ ] Show size reduction after optimization

---

## Phase 5: Tauri Integration

### Task 5.1: File Dialog Integration
- [ ] Configure Tauri dialog plugin
- [ ] Implement native "Open File" dialog (filter: .svg)
- [ ] Implement native "Save As" dialog
- [ ] Handle dialog cancellation

### Task 5.2: Tauri Commands
- [ ] Create `read_svg_file` command
- [ ] Create `write_svg_file` command
- [ ] Create `optimize_svg` command (calls core library)
- [ ] Register commands in Tauri

### Task 5.3: Frontend-Backend Communication
- [ ] Set up Tauri invoke calls in TypeScript
- [ ] Wire up Open button → file dialog → load file
- [ ] Wire up Save button → write file
- [ ] Wire up Save As → save dialog → write file
- [ ] Wire up Optimize button → optimize → refresh canvas

### Task 5.4: State Management
- [ ] Track current file path
- [ ] Track modified state (unsaved changes)
- [ ] Prompt before closing with unsaved changes
- [ ] Update window title with filename

---

## Phase 6: Polish & Testing

### Task 6.1: Error Handling
- [ ] Handle invalid SVG files gracefully
- [ ] Show user-friendly error messages
- [ ] Handle file permission errors
- [ ] Handle disk full scenarios

### Task 6.2: UI Polish
- [ ] Add loading indicators
- [ ] Add transition animations (subtle)
- [ ] Ensure consistent spacing
- [ ] Test with various SVG complexities
- [ ] Verify color picker works correctly

### Task 6.3: Application Packaging
- [ ] Configure Tauri build settings
- [ ] Set application icon
- [ ] Build macOS .app bundle
- [ ] Build macOS .dmg installer
- [ ] Test installation on clean system

### Task 6.4: CLI Binary Distribution
- [ ] Configure CLI-only build target
- [ ] Document installation via Homebrew (future)
- [ ] Test CLI on macOS
- [ ] Test CLI on Linux (if available)

---

## Phase 7: Documentation

### Task 7.1: User Documentation
- [ ] Write README.md with:
  - Installation instructions
  - CLI usage examples
  - GUI overview
  - Screenshots
- [ ] Add CHANGELOG.md

### Task 7.2: Developer Documentation
- [ ] Document build instructions
- [ ] Document project structure
- [ ] Add inline code comments where complex

---

## Task Dependency Graph

```
Phase 1 (Setup)
    │
    ▼
Phase 2 (Core Library) ──────────┐
    │                            │
    ▼                            ▼
Phase 3 (CLI)              Phase 4 (GUI Frontend)
    │                            │
    │                            ▼
    │                      Phase 5 (Tauri Integration)
    │                            │
    └────────────┬───────────────┘
                 │
                 ▼
           Phase 6 (Polish)
                 │
                 ▼
           Phase 7 (Docs)
```

---

## Estimated Task Sizes

| Phase | Tasks | Complexity |
|-------|-------|------------|
| Phase 1: Setup | 3 | Low |
| Phase 2: Core Library | 4 | Medium |
| Phase 3: CLI | 6 | Medium |
| Phase 4: GUI Frontend | 6 | Medium |
| Phase 5: Tauri Integration | 4 | Medium |
| Phase 6: Polish | 4 | Low-Medium |
| Phase 7: Documentation | 2 | Low |

**Total: 29 tasks across 7 phases**

---

## Suggested Starting Order

1. **Start with Phase 1** - Get project scaffolded and building
2. **Move to Phase 2** - Build core library in isolation (testable)
3. **Phase 3 next** - CLI provides quick testing of core functionality
4. **Then Phase 4 + 5** - GUI can be built in parallel once core exists
5. **Finish with Phase 6 + 7** - Polish and document

This order allows early validation of core functionality via CLI before investing in GUI work.
