import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

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
}

const state: AppState = {
  currentPath: null,
  svgContent: null,
  originalContent: null,
  isModified: false,
  selectedElement: null,
};

// DOM Elements
const btnOpen = document.getElementById("btn-open") as HTMLButtonElement;
const btnSave = document.getElementById("btn-save") as HTMLButtonElement;
const btnSaveAs = document.getElementById("btn-save-as") as HTMLButtonElement;
const btnOptimize = document.getElementById("btn-optimize") as HTMLButtonElement;
const canvas = document.getElementById("canvas") as HTMLDivElement;
const panelContent = document.getElementById("panel-content") as HTMLDivElement;
const statusMessage = document.getElementById("status-message") as HTMLSpanElement;
const statusFile = document.getElementById("status-file") as HTMLSpanElement;
const statusSize = document.getElementById("status-size") as HTMLSpanElement;

// Initialize application
document.addEventListener("DOMContentLoaded", () => {
  setupEventListeners();
  updateUI();
});

function setupEventListeners() {
  btnOpen.addEventListener("click", handleOpen);
  btnSave.addEventListener("click", handleSave);
  btnSaveAs.addEventListener("click", handleSaveAs);
  btnOptimize.addEventListener("click", handleOptimize);
}

// File operations
async function handleOpen() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: "SVG Files", extensions: ["svg"] }],
    });

    if (selected) {
      await loadSvg(selected as string);
    }
  } catch (error) {
    showError(`Failed to open file: ${error}`);
  }
}

async function loadSvg(path: string) {
  try {
    setStatus("Loading...");
    const response = await invoke<SvgResponse>("read_svg", { path });

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
    showError(`Failed to load SVG: ${error}`);
  }
}

async function handleSave() {
  if (!state.currentPath || !state.svgContent) return;

  try {
    setStatus("Saving...");
    await invoke("write_svg", {
      path: state.currentPath,
      content: state.svgContent,
    });

    state.originalContent = state.svgContent;
    state.isModified = false;
    updateUI();
    setStatus("Saved");
  } catch (error) {
    showError(`Failed to save: ${error}`);
  }
}

async function handleSaveAs() {
  if (!state.svgContent) return;

  try {
    const path = await save({
      filters: [{ name: "SVG Files", extensions: ["svg"] }],
      defaultPath: state.currentPath || "untitled.svg",
    });

    if (path) {
      setStatus("Saving...");
      await invoke("write_svg", { path, content: state.svgContent });

      state.currentPath = path;
      state.originalContent = state.svgContent;
      state.isModified = false;
      updateUI();
      statusFile.textContent = getFileName(path);
      setStatus("Saved");
    }
  } catch (error) {
    showError(`Failed to save: ${error}`);
  }
}

async function handleOptimize() {
  if (!state.svgContent) return;

  try {
    setStatus("Optimizing...");
    const response = await invoke<OptimizeResponse>("optimize_svg", {
      content: state.svgContent,
    });

    state.svgContent = response.content;
    state.isModified = true;

    renderSvg(response.content);
    updateUI();
    statusSize.textContent = response.size_formatted;
    setStatus(
      `Optimized: ${response.reduction_percent.toFixed(1)}% reduction`
    );
  } catch (error) {
    showError(`Failed to optimize: ${error}`);
  }
}

// SVG rendering
function renderSvg(content: string) {
  // Clear previous content
  canvas.innerHTML = "";

  // Create wrapper for the SVG
  const wrapper = document.createElement("div");
  wrapper.className = "svg-wrapper";
  wrapper.innerHTML = content;

  // Get the SVG element
  const svg = wrapper.querySelector("svg");
  if (svg) {
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
  if (!state.svgContent) return;

  try {
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

    // Reselect if there was a selection
    deselectElement();
  } catch (error) {
    showError(`Failed to apply colors: ${error}`);
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

function showError(message: string) {
  console.error(message);
  setStatus(`Error: ${message}`);
  statusMessage.style.color = "var(--error)";
  setTimeout(() => {
    statusMessage.style.color = "";
  }, 3000);
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
