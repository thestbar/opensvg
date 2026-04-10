use resvg::usvg::{Options, Tree};
use resvg::tiny_skia::{Pixmap, Transform};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RasterizeError {
    #[error("Failed to parse SVG: {0}")]
    Parse(String),
    #[error("Failed to allocate pixel buffer (dimensions too large?)")]
    PixmapCreate,
    #[error("Failed to encode PNG: {0}")]
    EncodePng(String),
    #[error("Failed to save JPEG: {0}")]
    SaveJpeg(String),
    #[error("Unsupported output format '{0}'. Use .png, .jpg, or .jpeg")]
    UnsupportedFormat(String),
}

/// Render an SVG string to a raster image file.
///
/// The output format is inferred from the file extension (.png, .jpg, .jpeg).
/// Returns the final pixel dimensions (width, height).
pub fn rasterize(svg_content: &str, output: &Path, scale: f32) -> Result<(u32, u32), RasterizeError> {
    let opt = Options::default();
    // Strip UTF-8 BOM if present — some SVG editors emit it and usvg rejects it
    let svg_content = svg_content.trim_start_matches('\u{FEFF}');
    let tree = Tree::from_str(svg_content, &opt)
        .map_err(|e| RasterizeError::Parse(e.to_string()))?;

    let svg_size = tree.size();
    let width = ((svg_size.width() * scale) as u32).max(1);
    let height = ((svg_size.height() * scale) as u32).max(1);

    let mut pixmap = Pixmap::new(width, height)
        .ok_or(RasterizeError::PixmapCreate)?;

    resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap.as_mut());

    let ext = output
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match ext.as_deref() {
        Some("png") => {
            let data = pixmap
                .encode_png()
                .map_err(|e| RasterizeError::EncodePng(e.to_string()))?;
            std::fs::write(output, data)
                .map_err(|e| RasterizeError::EncodePng(e.to_string()))?;
        }
        Some("jpg") | Some("jpeg") => {
            // Composite premultiplied RGBA over a white background and save as JPEG
            let rgb_data: Vec<u8> = pixmap
                .pixels()
                .iter()
                .flat_map(|p| {
                    // Premultiplied composite over white:
                    // final = premul_channel + (255 - alpha)
                    let r = p.red().saturating_add(255 - p.alpha());
                    let g = p.green().saturating_add(255 - p.alpha());
                    let b = p.blue().saturating_add(255 - p.alpha());
                    [r, g, b]
                })
                .collect();

            use image::{ImageBuffer, Rgb};
            let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
                ImageBuffer::from_raw(width, height, rgb_data)
                    .ok_or(RasterizeError::PixmapCreate)?;
            img.save(output)
                .map_err(|e| RasterizeError::SaveJpeg(e.to_string()))?;
        }
        Some(other) => return Err(RasterizeError::UnsupportedFormat(other.to_string())),
        None => return Err(RasterizeError::UnsupportedFormat("(none)".to_string())),
    }

    Ok((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50">
        <rect width="100" height="50" fill="red"/>
    </svg>"#;

    #[test]
    fn test_rasterize_png() {
        let dir = std::env::temp_dir();
        let out = dir.join("opensvg_test_output.png");
        let (w, h) = rasterize(SIMPLE_SVG, &out, 1.0).unwrap();
        assert_eq!(w, 100);
        assert_eq!(h, 50);
        assert!(out.exists());
        std::fs::remove_file(out).ok();
    }

    #[test]
    fn test_rasterize_png_with_scale() {
        let dir = std::env::temp_dir();
        let out = dir.join("opensvg_test_scaled.png");
        let (w, h) = rasterize(SIMPLE_SVG, &out, 2.0).unwrap();
        assert_eq!(w, 200);
        assert_eq!(h, 100);
        assert!(out.exists());
        std::fs::remove_file(out).ok();
    }

    #[test]
    fn test_rasterize_jpeg() {
        let dir = std::env::temp_dir();
        let out = dir.join("opensvg_test_output.jpg");
        let (w, h) = rasterize(SIMPLE_SVG, &out, 1.0).unwrap();
        assert_eq!(w, 100);
        assert_eq!(h, 50);
        assert!(out.exists());
        std::fs::remove_file(out).ok();
    }

    #[test]
    fn test_unsupported_format() {
        let dir = std::env::temp_dir();
        let out = dir.join("opensvg_test.bmp");
        assert!(rasterize(SIMPLE_SVG, &out, 1.0).is_err());
    }

    #[test]
    fn test_invalid_svg() {
        let dir = std::env::temp_dir();
        let out = dir.join("opensvg_test_bad.png");
        assert!(rasterize("not svg at all", &out, 1.0).is_err());
    }
}
