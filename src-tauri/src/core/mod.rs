pub mod color;
pub mod optimizer;
pub mod parser;
pub mod rasterizer;

// Re-export commonly used types
pub use color::{normalize_color, parse_color, ColorError, ParsedColor};
pub use optimizer::{calculate_reduction, format_size, optimize, OptimizeConfig, OptimizeError};
pub use parser::{ParseError, SvgDocument};
pub use rasterizer::{rasterize, RasterizeError};
