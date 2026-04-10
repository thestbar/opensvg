pub mod cli;
pub mod core;

use crate::core::{
    calculate_reduction, format_size, normalize_color, optimize, rasterize, OptimizeConfig,
    SvgDocument,
};

/// Response for SVG operations with size info
#[derive(serde::Serialize)]
pub struct SvgResponse {
    pub content: String,
    pub size: usize,
    pub size_formatted: String,
}

/// Response for optimization with reduction info
#[derive(serde::Serialize)]
pub struct OptimizeResponse {
    pub content: String,
    pub original_size: usize,
    pub new_size: usize,
    pub reduction_percent: f64,
    pub size_formatted: String,
}

/// Read an SVG file and return its content
#[tauri::command]
fn read_svg(path: String) -> Result<SvgResponse, String> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Validate it's a valid SVG
    SvgDocument::parse(&content).map_err(|e| format!("Invalid SVG: {}", e))?;

    let size = content.len();
    Ok(SvgResponse {
        content,
        size,
        size_formatted: format_size(size),
    })
}

/// Write SVG content to a file
#[tauri::command]
fn write_svg(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, &content).map_err(|e| format!("Failed to write file: {}", e))
}

/// Optimize/minify SVG content
#[tauri::command]
fn optimize_svg(content: String) -> Result<OptimizeResponse, String> {
    let original_size = content.len();

    let config = OptimizeConfig::default();
    let optimized =
        optimize(&content, &config).map_err(|e| format!("Failed to optimize: {}", e))?;

    let new_size = optimized.len();
    let reduction = calculate_reduction(original_size, new_size);

    Ok(OptimizeResponse {
        content: optimized,
        original_size,
        new_size,
        reduction_percent: reduction,
        size_formatted: format_size(new_size),
    })
}

/// Set fill color on SVG content
#[tauri::command]
fn set_fill_color(content: String, color: String) -> Result<SvgResponse, String> {
    let normalized =
        normalize_color(&color).map_err(|_| format!("Invalid color: '{}'", color))?;

    let mut doc = SvgDocument::parse(&content).map_err(|e| format!("Invalid SVG: {}", e))?;

    doc.set_fill(&normalized)
        .map_err(|e| format!("Failed to set fill: {}", e))?;

    let result = doc.to_string();
    let size = result.len();

    Ok(SvgResponse {
        content: result,
        size,
        size_formatted: format_size(size),
    })
}

/// Set stroke color on SVG content
#[tauri::command]
fn set_stroke_color(content: String, color: String) -> Result<SvgResponse, String> {
    let normalized =
        normalize_color(&color).map_err(|_| format!("Invalid color: '{}'", color))?;

    let mut doc = SvgDocument::parse(&content).map_err(|e| format!("Invalid SVG: {}", e))?;

    doc.set_stroke(&normalized)
        .map_err(|e| format!("Failed to set stroke: {}", e))?;

    let result = doc.to_string();
    let size = result.len();

    Ok(SvgResponse {
        content: result,
        size,
        size_formatted: format_size(size),
    })
}

/// Validate a color string
#[tauri::command]
fn validate_color(color: String) -> Result<String, String> {
    normalize_color(&color).map_err(|_| format!("Invalid color: '{}'", color))
}

/// Convert SVG content to a raster image file (PNG or JPEG)
#[tauri::command]
fn convert_svg(content: String, output_path: String, scale: f32) -> Result<(), String> {
    rasterize(&content, std::path::Path::new(&output_path), scale)
        .map(|_| ())
        .map_err(|e| format!("{}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            read_svg,
            write_svg,
            optimize_svg,
            set_fill_color,
            set_stroke_color,
            validate_color,
            convert_svg
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
