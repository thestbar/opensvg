use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("XML parsing error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("Invalid SVG: {0}")]
    InvalidSvg(String),
    #[error("UTF-8 encoding error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Represents an SVG document that can be manipulated
#[derive(Debug, Clone)]
pub struct SvgDocument {
    content: String,
}

impl SvgDocument {
    /// Parse SVG content from a string
    pub fn parse(content: &str) -> Result<Self, ParseError> {
        // Validate it's valid XML and contains an SVG element
        let mut reader = Reader::from_str(content);
        reader.config_mut().trim_text(true);

        let mut found_svg = false;
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() == b"svg" {
                        found_svg = true;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(ParseError::XmlError(e)),
                _ => {}
            }
        }

        if !found_svg {
            return Err(ParseError::InvalidSvg(
                "No <svg> element found".to_string(),
            ));
        }

        Ok(Self {
            content: content.to_string(),
        })
    }

    /// Get the raw SVG content as a string
    pub fn to_string(&self) -> String {
        self.content.clone()
    }

    /// Get the size of the SVG content in bytes
    pub fn size(&self) -> usize {
        self.content.len()
    }

    /// Set an attribute on all elements of a given type
    pub fn set_attribute_on_elements(
        &mut self,
        element_names: &[&str],
        attr_name: &str,
        attr_value: &str,
    ) -> Result<(), ParseError> {
        let mut reader = Reader::from_str(&self.content);
        reader.config_mut().trim_text(true);

        let mut writer = Writer::new(Cursor::new(Vec::new()));

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name_bytes = e.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    if element_names.contains(&name) {
                        let new_elem = set_or_add_attribute(e, attr_name, attr_value)?;
                        writer.write_event(Event::Start(new_elem))?;
                    } else {
                        writer.write_event(Event::Start(e.clone()))?;
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name_bytes = e.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    if element_names.contains(&name) {
                        let new_elem = set_or_add_attribute(e, attr_name, attr_value)?;
                        writer.write_event(Event::Empty(new_elem))?;
                    } else {
                        writer.write_event(Event::Empty(e.clone()))?;
                    }
                }
                Ok(Event::Eof) => break,
                Ok(e) => writer.write_event(e)?,
                Err(e) => return Err(ParseError::XmlError(e)),
            }
        }

        let result = writer.into_inner().into_inner();
        self.content =
            String::from_utf8(result).map_err(|e| ParseError::Utf8Error(e.utf8_error()))?;
        Ok(())
    }

    /// Set fill color on all shape elements
    pub fn set_fill(&mut self, color: &str) -> Result<(), ParseError> {
        let shape_elements = [
            "path", "rect", "circle", "ellipse", "line", "polyline", "polygon", "text", "g",
        ];
        self.set_attribute_on_elements(&shape_elements, "fill", color)
    }

    /// Set stroke color on all shape elements
    pub fn set_stroke(&mut self, color: &str) -> Result<(), ParseError> {
        let shape_elements = [
            "path", "rect", "circle", "ellipse", "line", "polyline", "polygon", "text", "g",
        ];
        self.set_attribute_on_elements(&shape_elements, "stroke", color)
    }
}

/// Helper function to set or add an attribute to an element
fn set_or_add_attribute(
    element: &BytesStart,
    attr_name: &str,
    attr_value: &str,
) -> Result<BytesStart<'static>, ParseError> {
    let name_bytes = element.name();
    let name_str = std::str::from_utf8(name_bytes.as_ref())?;
    let mut new_elem = BytesStart::new(name_str.to_string());

    let mut found = false;
    for attr in element.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        let key = std::str::from_utf8(attr.key.as_ref())?;
        if key == attr_name {
            new_elem.push_attribute((attr_name, attr_value));
            found = true;
        } else {
            let value = std::str::from_utf8(&attr.value)?;
            new_elem.push_attribute((key, value));
        }
    }

    if !found {
        new_elem.push_attribute((attr_name, attr_value));
    }

    Ok(new_elem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_svg() {
        let svg =
            r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100"/></svg>"#;
        let doc = SvgDocument::parse(svg);
        assert!(doc.is_ok());
    }

    #[test]
    fn test_parse_invalid_svg() {
        let svg = r#"<html><body>Not an SVG</body></html>"#;
        let doc = SvgDocument::parse(svg);
        assert!(doc.is_err());
    }

    #[test]
    fn test_set_fill() {
        let svg =
            r#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100"/></svg>"#;
        let mut doc = SvgDocument::parse(svg).unwrap();
        doc.set_fill("#ff0000").unwrap();
        assert!(doc.to_string().contains("fill=\"#ff0000\""));
    }

    #[test]
    fn test_set_stroke() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M0 0 L10 10"/></svg>"#;
        let mut doc = SvgDocument::parse(svg).unwrap();
        doc.set_stroke("#00ff00").unwrap();
        assert!(doc.to_string().contains("stroke=\"#00ff00\""));
    }
}
