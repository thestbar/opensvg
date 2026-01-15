use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptimizeError {
    #[error("XML parsing error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("UTF-8 encoding error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("UTF-8 string error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Configuration for SVG optimization
#[derive(Debug, Clone)]
pub struct OptimizeConfig {
    /// Remove XML comments
    pub remove_comments: bool,
    /// Remove metadata elements (metadata, title, desc)
    pub remove_metadata: bool,
    /// Remove empty elements
    pub remove_empty_attrs: bool,
    /// Minify output (no pretty printing)
    pub minify: bool,
}

impl Default for OptimizeConfig {
    fn default() -> Self {
        Self {
            remove_comments: true,
            remove_metadata: true,
            remove_empty_attrs: true,
            minify: true,
        }
    }
}

/// Elements to skip during optimization (metadata-like)
const SKIP_ELEMENTS: &[&str] = &["metadata", "title", "desc"];

/// Attributes to remove if empty
const SKIP_EMPTY_ATTRS: &[&str] = &["id", "class", "style"];

/// Optimize an SVG string
pub fn optimize(content: &str, config: &OptimizeConfig) -> Result<String, OptimizeError> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(config.minify);

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut skip_depth = 0;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name_bytes = e.name();
                let name_ref = name_bytes.as_ref();
                let name = std::str::from_utf8(name_ref)?;

                // Skip metadata elements
                if config.remove_metadata && SKIP_ELEMENTS.contains(&name) {
                    skip_depth += 1;
                    continue;
                }

                if skip_depth > 0 {
                    skip_depth += 1;
                    continue;
                }

                // Process and write the element
                let processed = process_element(e, config)?;
                writer.write_event(Event::Start(processed))?;
            }
            Ok(Event::End(ref e)) => {
                let name_bytes = e.name();
                let name_ref = name_bytes.as_ref();
                let name = std::str::from_utf8(name_ref)?;

                if skip_depth > 0 {
                    skip_depth -= 1;
                    continue;
                }

                if config.remove_metadata && SKIP_ELEMENTS.contains(&name) {
                    continue;
                }

                writer.write_event(Event::End(e.clone()))?;
            }
            Ok(Event::Empty(ref e)) => {
                if skip_depth > 0 {
                    continue;
                }

                let name_bytes = e.name();
                let name_ref = name_bytes.as_ref();
                let name = std::str::from_utf8(name_ref)?;
                if config.remove_metadata && SKIP_ELEMENTS.contains(&name) {
                    continue;
                }

                let processed = process_element(e, config)?;
                writer.write_event(Event::Empty(processed))?;
            }
            Ok(Event::Comment(_)) => {
                if !config.remove_comments {
                    // Keep comments if not removing them
                    // Note: we're skipping them by default
                }
            }
            Ok(Event::Text(ref e)) => {
                if skip_depth > 0 {
                    continue;
                }
                // Skip whitespace-only text in minify mode
                if config.minify {
                    let text = e.unescape()?;
                    if text.trim().is_empty() {
                        continue;
                    }
                }
                writer.write_event(Event::Text(e.clone()))?;
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                if skip_depth == 0 {
                    writer.write_event(e)?;
                }
            }
            Err(e) => return Err(OptimizeError::XmlError(e)),
        }
    }

    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result)?)
}

/// Process an element's attributes
fn process_element(
    element: &BytesStart,
    config: &OptimizeConfig,
) -> Result<BytesStart<'static>, OptimizeError> {
    let name_bytes = element.name();
    let name = std::str::from_utf8(name_bytes.as_ref())?.to_string();
    let mut new_elem = BytesStart::new(name);

    for attr in element.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        let key = std::str::from_utf8(attr.key.as_ref())?;
        let value = std::str::from_utf8(&attr.value)?;

        // Skip empty attributes if configured
        if config.remove_empty_attrs && value.is_empty() && SKIP_EMPTY_ATTRS.contains(&key) {
            continue;
        }

        // Skip editor-specific attributes
        if key.starts_with("inkscape:")
            || key.starts_with("sodipodi:")
            || key.starts_with("xmlns:inkscape")
            || key.starts_with("xmlns:sodipodi")
        {
            continue;
        }

        new_elem.push_attribute((key, value));
    }

    Ok(new_elem)
}

/// Calculate size reduction percentage
pub fn calculate_reduction(original: usize, optimized: usize) -> f64 {
    if original == 0 {
        return 0.0;
    }
    ((original - optimized) as f64 / original as f64) * 100.0
}

/// Format bytes size in human-readable format
pub fn format_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comments() {
        let svg = r#"<svg><!-- comment --><rect/></svg>"#;
        let config = OptimizeConfig::default();
        let result = optimize(svg, &config).unwrap();
        assert!(!result.contains("comment"));
    }

    #[test]
    fn test_remove_metadata() {
        let svg = r#"<svg><metadata>some data</metadata><rect/></svg>"#;
        let config = OptimizeConfig::default();
        let result = optimize(svg, &config).unwrap();
        assert!(!result.contains("metadata"));
        assert!(result.contains("rect"));
    }

    #[test]
    fn test_remove_inkscape_attrs() {
        let svg =
            r#"<svg inkscape:version="1.0" xmlns:inkscape="http://inkscape.org"><rect/></svg>"#;
        let config = OptimizeConfig::default();
        let result = optimize(svg, &config).unwrap();
        assert!(!result.contains("inkscape"));
    }

    #[test]
    fn test_preserve_content() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100" fill="red"/></svg>"#;
        let config = OptimizeConfig::default();
        let result = optimize(svg, &config).unwrap();
        assert!(result.contains("rect"));
        assert!(result.contains("width"));
        assert!(result.contains("fill"));
    }

    #[test]
    fn test_calculate_reduction() {
        assert!((calculate_reduction(100, 50) - 50.0).abs() < 0.01);
        assert!((calculate_reduction(100, 75) - 25.0).abs() < 0.01);
        assert!((calculate_reduction(0, 0) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1500), "1.5 KB");
        assert_eq!(format_size(1500000), "1.4 MB");
    }
}
