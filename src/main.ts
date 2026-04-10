import { invoke } from "@tauri-apps/api/core";
import { open, save, confirm } from "@tauri-apps/plugin-dialog";
import { getCurrentWebview } from "@tauri-apps/api/webview";

// Types for Tauri responses
interface SvgResponse {
  content: string;
  size: number;
  size_formatted: string;
}

interface OptimizeResponse {
  content: string;
  original_size: number;
  new_size: number;
  reduction_percent: number;
  size_formatted: string;
}

// Application state
interface AppState {
  currentPath: string | null;
  svgContent: string | null;
  originalContent: string | null;
  isModified: boolean;
  selectedElement: SVGElement | null;
  isLoading: boolean;
}

const state: AppState = {
  currentPath: null,
  svgContent: null,
  originalContent: null,
  isModified: false,
  selectedElement: null,
  isLoading: false,
};

// DOM Elements
const btnOpen = document.getElementById("btn-open") as HTMLButtonElement;
const btnSave = document.getElementById("btn-save") as HTMLButtonElement;
const btnSaveAs = document.getElementById("btn-save-as") as HTMLButtonElement;
const btnOptimize = document.getElementById("btn-optimize") as HTMLButtonElement;
const btnConvert = document.getElementById("btn-convert") as HTMLButtonElement;
const canvas = document.getElementById("canvas") as HTMLDivElement;
const panelContent = document.getElementById("panel-content") as HTMLDivElement;
const statusMessage = document.getElementById("status-message") as HTMLSpanElement;
const statusFile = document.getElementById("status-file") as HTMLSpanElement;
const statusSize = document.getElementById("status-size") as HTMLSpanElement;
const convertModal = document.getElementById("convert-modal") as HTMLDivElement;

// Initialize application
document.addEventListener("DOMContentLoaded", async () => {
  setupEventListeners();
  setupKeyboardShortcuts();
  await setupDragAndDrop();
  updateUI();
});

function setupEventListeners() {
  btnOpen.addEventListener("click", handleOpen);
  btnSave.addEventListener("click", handleSave);
  btnSaveAs.addEventListener("click", handleSaveAs);
  btnOptimize.addEventListener("click", handleOptimize);
  btnConvert.addEventListener("click", openConvertPane);

  // Convert modal controls
  document.getElementById("btn-export-cancel")!.addEventListener("click", closeConvertPane);
  document.getElementById("btn-export-confirm")!.addEventListener("click", handleExport);

  // Format toggle
  convertModal.querySelectorAll<HTMLButtonElement>(".format-btn").forEach((btn) => {
    btn.addEventListener("click", () => {
      convertModal.querySelectorAll(".format-btn").forEach((b) => b.classList.remove("active"));
      btn.classList.add("active");
    });
  });

  // Scale → update dimensions preview
  document.getElementById("export-scale")!.addEventListener("input", updateDimensionsPreview);

  // Close on backdrop click
  convertModal.addEventListener("click", (e) => {
    if (e.target === convertModal) closeConvertPane();
  });
}

function setupKeyboardShortcuts() {
  document.addEventListener("keydown", async (e) => {
    const isMod = e.metaKey || e.ctrlKey;

    if (isMod && e.key === "o") {
      e.preventDefault();
      await handleOpen();
    } else if (isMod && e.key === "s") {
      e.preventDefault();
      if (e.shiftKey) {
        await handleSaveAs();
      } else {
        await handleSave();
      }
    }
  });
}

async function setupDragAndDrop() {
  const webview = getCurrentWebview();

  await webview.onDragDropEvent(async (event) => {
    if (event.payload.type === "over") {
      canvas.classList.add("drag-over");
    } else if (event.payload.type === "leave" || event.payload.type === "drop") {
      canvas.classList.remove("drag-over");
    }

    if (event.payload.type === "drop") {
      const paths = event.payload.paths;
      if (paths && paths.length > 0) {
        const filePath = paths[0];
        if (filePath.toLowerCase().endsWith(".svg")) {
          // Check for unsaved changes first
          if (state.isModified) {
            const shouldContinue = await confirmUnsavedChanges();
            if (!shouldContinue) return;
          }

          // Load the dropped SVG file using our existing function
          await loadSvg(filePath);
        } else {
          showToast("Please drop an SVG file", "warning");
        }
      }
    }
  });
}

async function confirmUnsavedChanges(): Promise<boolean> {
  return await confirm(
    "You have unsaved changes. Do you want to continue without saving?",
    { title: "Unsaved Changes", kind: "warning" }
  );
}

function setLoading(loading: boolean) {
  state.isLoading = loading;
  document.body.classList.toggle("loading", loading);
  btnOpen.disabled = loading;
  btnSave.disabled = loading || !state.svgContent || !state.isModified;
  btnSaveAs.disabled = loading || !state.svgContent;
  btnOptimize.disabled = loading || !state.svgContent;
  btnConvert.disabled = loading || !state.svgContent;
}

// File operations
async function handleOpen() {
  if (state.isLoading) return;

  // Check for unsaved changes
  if (state.isModified) {
    const shouldContinue = await confirmUnsavedChanges();
    if (!shouldContinue) return;
  }

  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: "SVG Files", extensions: ["svg"] }],
    });

    if (selected) {
      await loadSvg(selected as string);
    }
  } catch (error) {
    showToast(`Failed to open file: ${error}`, "error");
  }
}

const LARGE_FILE_THRESHOLD = 5 * 1024 * 1024; // 5MB

async function loadSvg(path: string) {
  try {
    setLoading(true);
    setStatus("Loading...");
    const response = await invoke<SvgResponse>("read_svg", { path });

    // Warn about large files
    if (response.size > LARGE_FILE_THRESHOLD) {
      showToast("Large file detected. Performance may be affected.", "warning");
    }

    state.currentPath = path;
    state.svgContent = response.content;
    state.originalContent = response.content;
    state.isModified = false;

    renderSvg(response.content);
    updateUI();
    setStatus("Ready");
    statusFile.textContent = getFileName(path);
    statusSize.textContent = response.size_formatted;
  } catch (error) {
    showToast(`Failed to load SVG: ${error}`, "error");
  } finally {
    setLoading(false);
  }
}

async function handleSave() {
  if (!state.currentPath || !state.svgContent || state.isLoading) return;

  try {
    setLoading(true);
    setStatus("Saving...");
    await invoke("write_svg", {
      path: state.currentPath,
      content: state.svgContent,
    });

    state.originalContent = state.svgContent;
    state.isModified = false;
    updateUI();
    showToast("File saved successfully", "success");
    setStatus("Saved");
  } catch (error) {
    showToast(`Failed to save: ${error}`, "error");
  } finally {
    setLoading(false);
  }
}

async function handleSaveAs() {
  if (!state.svgContent || state.isLoading) return;

  try {
    const path = await save({
      filters: [{ name: "SVG Files", extensions: ["svg"] }],
      defaultPath: state.currentPath || "untitled.svg",
    });

    if (path) {
      setLoading(true);
      setStatus("Saving...");
      await invoke("write_svg", { path, content: state.svgContent });

      state.currentPath = path;
      state.originalContent = state.svgContent;
      state.isModified = false;
      updateUI();
      statusFile.textContent = getFileName(path);
      showToast("File saved successfully", "success");
      setStatus("Saved");
    }
  } catch (error) {
    showToast(`Failed to save: ${error}`, "error");
  } finally {
    setLoading(false);
  }
}

async function handleOptimize() {
  if (!state.svgContent || state.isLoading) return;

  try {
    setLoading(true);
    setStatus("Optimizing...");
    const response = await invoke<OptimizeResponse>("optimize_svg", {
      content: state.svgContent,
    });

    state.svgContent = response.content;
    state.isModified = true;

    renderSvg(response.content);
    updateUI();
    statusSize.textContent = response.size_formatted;
    const msg = `Optimized: ${response.reduction_percent.toFixed(1)}% reduction`;
    setStatus(msg);
    showToast(msg, "success");
  } catch (error) {
    showToast(`Failed to optimize: ${error}`, "error");
  } finally {
    setLoading(false);
  }
}

function showEmptyState(message: string) {
  canvas.innerHTML = `
    <div class="canvas-empty">
      <p>${message}</p>
      <p class="hint">Open an SVG file to get started</p>
    </div>
  `;
}

// SVG rendering
function renderSvg(content: string) {
  // Clear previous content
  canvas.innerHTML = "";

  // Check for empty or minimal content
  if (!content || content.trim().length === 0) {
    showEmptyState("Empty SVG file");
    return;
  }

  // Create wrapper for the SVG
  const wrapper = document.createElement("div");
  wrapper.className = "svg-wrapper";
  wrapper.innerHTML = content;

  // Get the SVG element
  const svg = wrapper.querySelector("svg");
  if (!svg) {
    showEmptyState("No valid SVG element found");
    return;
  }

  // Check for SVG with no visual content (just warn, still show the SVG)
  const visualElements = svg.querySelectorAll(
    "path, rect, circle, ellipse, line, polyline, polygon, text, image, use"
  );
  if (visualElements.length === 0) {
    showToast("SVG has no visual elements", "warning");
  }

  // SVG exists and is valid
  {
    // Make SVG responsive
    svg.style.maxWidth = "100%";
    svg.style.maxHeight = "100%";
    svg.style.width = "auto";
    svg.style.height = "auto";

    // Add click handlers to selectable elements
    const selectableElements = svg.querySelectorAll(
      "path, rect, circle, ellipse, line, polyline, polygon, text, g"
    );

    selectableElements.forEach((el) => {
      el.addEventListener("click", (e) => {
        e.stopPropagation();
        selectElement(el as SVGElement);
      });
      (el as HTMLElement).style.cursor = "pointer";
    });

    // Click on canvas to deselect
    wrapper.addEventListener("click", () => {
      deselectElement();
    });
  }

  canvas.appendChild(wrapper);
}

// Element selection
function selectElement(element: SVGElement) {
  // Deselect previous
  if (state.selectedElement) {
    state.selectedElement.classList.remove("selected");
  }

  // Select new
  state.selectedElement = element;
  element.classList.add("selected");

  // Update color panel
  updateColorPanel(element);
}

function deselectElement() {
  if (state.selectedElement) {
    state.selectedElement.classList.remove("selected");
    state.selectedElement = null;
  }
  resetColorPanel();
}

// Color panel
function updateColorPanel(element: SVGElement) {
  const tagName = element.tagName.toLowerCase();
  const fill = getComputedStyleProp(element, "fill") || "#000000";
  const stroke = getComputedStyleProp(element, "stroke") || "none";

  panelContent.innerHTML = `
    <div class="panel-info">
      <label>Element</label>
      <span class="element-tag">&lt;${tagName}&gt;</span>
    </div>

    <div class="panel-group">
      <label>Fill Color</label>
      <div class="color-input">
        <input type="color" id="fill-color" value="${normalizeToHex(fill)}" />
        <input type="text" id="fill-text" value="${fill}" />
      </div>
    </div>

    <div class="panel-group">
      <label>Stroke Color</label>
      <div class="color-input">
        <input type="color" id="stroke-color" value="${normalizeToHex(stroke)}" />
        <input type="text" id="stroke-text" value="${stroke}" />
      </div>
    </div>

    <div class="panel-buttons">
      <button class="btn-apply" id="btn-apply-colors">Apply to All</button>
    </div>
  `;

  // Setup color input sync
  setupColorInputs();
}

function setupColorInputs() {
  const fillColor = document.getElementById("fill-color") as HTMLInputElement;
  const fillText = document.getElementById("fill-text") as HTMLInputElement;
  const strokeColor = document.getElementById("stroke-color") as HTMLInputElement;
  const strokeText = document.getElementById("stroke-text") as HTMLInputElement;
  const btnApply = document.getElementById("btn-apply-colors") as HTMLButtonElement;

  // Sync color picker with text input
  fillColor?.addEventListener("input", () => {
    fillText.value = fillColor.value;
  });

  fillText?.addEventListener("change", () => {
    const hex = normalizeToHex(fillText.value);
    if (hex !== "#000000" || fillText.value.toLowerCase() === "black") {
      fillColor.value = hex;
    }
  });

  strokeColor?.addEventListener("input", () => {
    strokeText.value = strokeColor.value;
  });

  strokeText?.addEventListener("change", () => {
    const hex = normalizeToHex(strokeText.value);
    if (hex !== "#000000" || strokeText.value.toLowerCase() === "black") {
      strokeColor.value = hex;
    }
  });

  // Apply colors button
  btnApply?.addEventListener("click", async () => {
    await applyColors(fillText.value, strokeText.value);
  });
}

async function applyColors(fill: string, stroke: string) {
  if (!state.svgContent || state.isLoading) return;

  try {
    setLoading(true);
    setStatus("Applying colors...");

    // Apply fill
    let response = await invoke<SvgResponse>("set_fill_color", {
      content: state.svgContent,
      color: fill,
    });

    // Apply stroke
    response = await invoke<SvgResponse>("set_stroke_color", {
      content: response.content,
      color: stroke,
    });

    state.svgContent = response.content;
    state.isModified = true;

    renderSvg(response.content);
    updateUI();
    statusSize.textContent = response.size_formatted;
    setStatus("Colors applied");
    showToast("Colors applied successfully", "success");

    // Reselect if there was a selection
    deselectElement();
  } catch (error) {
    showToast(`Failed to apply colors: ${error}`, "error");
  } finally {
    setLoading(false);
  }
}

function resetColorPanel() {
  panelContent.innerHTML = `
    <p class="panel-empty">Select an element to edit its colors</p>
  `;
}

// UI updates
function updateUI() {
  const hasFile = state.svgContent !== null;
  const isModified = state.isModified;

  btnSave.disabled = !hasFile || !isModified;
  btnSaveAs.disabled = !hasFile;
  btnOptimize.disabled = !hasFile;
  btnConvert.disabled = !hasFile;

  // Update window title indicator
  if (state.currentPath) {
    const fileName = getFileName(state.currentPath);
    document.title = isModified ? `${fileName} * - OpenSVG` : `${fileName} - OpenSVG`;
  } else {
    document.title = "OpenSVG";
  }
}

function setStatus(message: string) {
  statusMessage.textContent = message;
}

function showToast(message: string, type: "success" | "error" | "warning" = "success") {
  // Remove existing toast if any
  const existingToast = document.querySelector(".toast");
  if (existingToast) {
    existingToast.remove();
  }

  const toast = document.createElement("div");
  toast.className = `toast toast-${type}`;
  toast.textContent = message;
  document.body.appendChild(toast);

  // Trigger animation
  requestAnimationFrame(() => {
    toast.classList.add("show");
  });

  // Auto-remove after 3 seconds
  setTimeout(() => {
    toast.classList.remove("show");
    setTimeout(() => toast.remove(), 300);
  }, 3000);

  // Also update status bar for errors
  if (type === "error") {
    console.error(message);
    setStatus(`Error: ${message}`);
  }
}

// Convert / Export pane
function getSvgNaturalSize(svgContent: string): { width: number; height: number } | null {
  const parser = new DOMParser();
  const doc = parser.parseFromString(svgContent, "image/svg+xml");
  const svg = doc.querySelector("svg");
  if (!svg) return null;

  const w = parseFloat(svg.getAttribute("width") || "0");
  const h = parseFloat(svg.getAttribute("height") || "0");
  if (w > 0 && h > 0) return { width: w, height: h };

  const vb = svg.getAttribute("viewBox");
  if (vb) {
    const parts = vb.trim().split(/[\s,]+/);
    if (parts.length >= 4) {
      const vw = parseFloat(parts[2]);
      const vh = parseFloat(parts[3]);
      if (vw > 0 && vh > 0) return { width: vw, height: vh };
    }
  }

  return null;
}

function updateDimensionsPreview() {
  const scaleInput = document.getElementById("export-scale") as HTMLInputElement;
  const preview = document.getElementById("export-dimensions") as HTMLSpanElement;
  if (!state.svgContent || !scaleInput || !preview) return;

  const size = getSvgNaturalSize(state.svgContent);
  if (!size) {
    preview.textContent = "unknown";
    return;
  }

  const scale = parseFloat(scaleInput.value) || 1;
  const w = Math.round(size.width * scale);
  const h = Math.round(size.height * scale);
  preview.textContent = `${w} × ${h} px`;
}

function openConvertPane() {
  if (!state.svgContent) return;

  // Reset to defaults
  const scaleInput = document.getElementById("export-scale") as HTMLInputElement;
  scaleInput.value = "1";
  convertModal.querySelectorAll(".format-btn").forEach((b, i) => {
    b.classList.toggle("active", i === 0); // default PNG
  });

  updateDimensionsPreview();
  convertModal.removeAttribute("hidden");
}

function closeConvertPane() {
  convertModal.setAttribute("hidden", "");
}

async function handleExport() {
  if (!state.svgContent) return;

  const activeFormat = convertModal.querySelector<HTMLButtonElement>(".format-btn.active");
  const format = activeFormat?.dataset.format ?? "png";
  const scaleInput = document.getElementById("export-scale") as HTMLInputElement;
  const scale = parseFloat(scaleInput.value) || 1;

  if (scale <= 0) {
    showToast("Scale must be greater than 0", "warning");
    return;
  }

  const baseName = state.currentPath
    ? getFileName(state.currentPath).replace(/\.svg$/i, "")
    : "export";

  const outputPath = await save({
    filters: [
      format === "png"
        ? { name: "PNG Image", extensions: ["png"] }
        : { name: "JPEG Image", extensions: ["jpg", "jpeg"] },
    ],
    defaultPath: `${baseName}.${format === "jpeg" ? "jpg" : "png"}`,
  });

  if (!outputPath) return;

  closeConvertPane();

  try {
    setLoading(true);
    setStatus("Exporting...");
    await invoke("convert_svg", {
      content: state.svgContent,
      outputPath,
      scale,
    });
    showToast(`Exported to ${getFileName(outputPath)}`, "success");
    setStatus("Ready");
  } catch (error) {
    showToast(`Export failed: ${error}`, "error");
  } finally {
    setLoading(false);
  }
}

// Utility functions
function getFileName(path: string): string {
  return path.split("/").pop() || path.split("\\").pop() || path;
}

function getComputedStyleProp(element: SVGElement, prop: string): string {
  // First check inline style
  const inline = element.getAttribute(prop);
  if (inline) return inline;

  // Then check computed style
  const computed = window.getComputedStyle(element);
  return computed.getPropertyValue(prop);
}

function normalizeToHex(color: string): string {
  if (!color || color === "none") return "#000000";

  // Already hex
  if (color.startsWith("#")) {
    // Convert 3-digit to 6-digit
    if (color.length === 4) {
      return `#${color[1]}${color[1]}${color[2]}${color[2]}${color[3]}${color[3]}`;
    }
    return color.substring(0, 7); // Remove alpha if present
  }

  // RGB/RGBA
  const rgbMatch = color.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
  if (rgbMatch) {
    const r = parseInt(rgbMatch[1]).toString(16).padStart(2, "0");
    const g = parseInt(rgbMatch[2]).toString(16).padStart(2, "0");
    const b = parseInt(rgbMatch[3]).toString(16).padStart(2, "0");
    return `#${r}${g}${b}`;
  }

  // Named colors - create temp element to convert
  const temp = document.createElement("div");
  temp.style.color = color;
  document.body.appendChild(temp);
  const computed = window.getComputedStyle(temp).color;
  document.body.removeChild(temp);

  const match = computed.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
  if (match) {
    const r = parseInt(match[1]).toString(16).padStart(2, "0");
    const g = parseInt(match[2]).toString(16).padStart(2, "0");
    const b = parseInt(match[3]).toString(16).padStart(2, "0");
    return `#${r}${g}${b}`;
  }

  return "#000000";
}
