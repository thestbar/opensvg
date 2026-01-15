use csscolorparser::Color;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Invalid color format: {0}")]
    InvalidFormat(String),
}

/// Parse a color string and return a validated color
/// Supports: #rgb, #rrggbb, #rrggbbaa, rgb(), rgba(), named colors
pub fn parse_color(input: &str) -> Result<ParsedColor, ColorError> {
    let color = input
        .parse::<Color>()
        .map_err(|_| ColorError::InvalidFormat(input.to_string()))?;

    Ok(ParsedColor {
        r: (color.r * 255.0) as u8,
        g: (color.g * 255.0) as u8,
        b: (color.b * 255.0) as u8,
        a: color.a as f32,
    })
}

/// Represents a parsed color with RGBA components
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32, // 0.0 to 1.0
}

impl ParsedColor {
    /// Create a new color from RGBA values
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to a CSS hex string (#rrggbb or #rrggbbaa if alpha < 1)
    pub fn to_hex(&self) -> String {
        if (self.a - 1.0).abs() < f32::EPSILON {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            let alpha = (self.a * 255.0) as u8;
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, alpha)
        }
    }

    /// Convert to CSS rgba() string
    pub fn to_rgba(&self) -> String {
        if (self.a - 1.0).abs() < f32::EPSILON {
            format!("rgb({}, {}, {})", self.r, self.g, self.b)
        } else {
            format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
        }
    }

    /// Check if color has transparency
    pub fn has_alpha(&self) -> bool {
        self.a < 1.0
    }
}

/// Normalize a color string to a consistent format for SVG
/// Returns #rrggbb for opaque colors, #rrggbbaa for transparent
pub fn normalize_color(input: &str) -> Result<String, ColorError> {
    let color = parse_color(input)?;
    Ok(color.to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_6() {
        let color = parse_color("#ff0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert!((color.a - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_parse_hex_3() {
        let color = parse_color("#f00").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_parse_hex_8_with_alpha() {
        let color = parse_color("#ff000080").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        // Alpha 0x80 = 128, which is ~0.502
        assert!((color.a - 0.502).abs() < 0.01);
    }

    #[test]
    fn test_parse_rgba() {
        let color = parse_color("rgba(255, 128, 0, 0.5)").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert!((color.a - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_parse_named_color() {
        let color = parse_color("red").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_to_hex() {
        let color = ParsedColor::new(255, 128, 0, 1.0);
        assert_eq!(color.to_hex(), "#ff8000");
    }

    #[test]
    fn test_to_hex_with_alpha() {
        let color = ParsedColor::new(255, 128, 0, 0.5);
        assert_eq!(color.to_hex(), "#ff80007f");
    }

    #[test]
    fn test_normalize_color() {
        assert_eq!(normalize_color("#f00").unwrap(), "#ff0000");
        assert_eq!(normalize_color("red").unwrap(), "#ff0000");
        assert_eq!(normalize_color("rgb(0, 255, 0)").unwrap(), "#00ff00");
    }

    #[test]
    fn test_invalid_color() {
        assert!(parse_color("notacolor").is_err());
        assert!(parse_color("#gggggg").is_err());
    }
}
