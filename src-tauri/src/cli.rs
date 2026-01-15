use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::core::{
    calculate_reduction, format_size, normalize_color, optimize, OptimizeConfig, SvgDocument,
};

#[derive(Parser)]
#[command(name = "opensvg")]
#[command(author, version, about = "A simple SVG editor and optimizer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Optimize/minify an SVG file
    #[command(alias = "opt")]
    Optimize {
        /// Input SVG file
        file: PathBuf,

        /// Output file (default: overwrite input)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Print result to stdout instead of file
        #[arg(short, long)]
        stdout: bool,

        /// Suppress status messages
        #[arg(short, long)]
        quiet: bool,
    },

    /// Change fill color of all elements
    Fill {
        /// Input SVG file
        file: PathBuf,

        /// Color (hex: #rgb, #rrggbb, #rrggbbaa, or named colors)
        color: String,

        /// Output file (default: overwrite input)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Suppress status messages
        #[arg(short, long)]
        quiet: bool,
    },

    /// Change stroke color of all elements
    Stroke {
        /// Input SVG file
        file: PathBuf,

        /// Color (hex: #rgb, #rrggbb, #rrggbbaa, or named colors)
        color: String,

        /// Output file (default: overwrite input)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Suppress status messages
        #[arg(short, long)]
        quiet: bool,
    },
}

/// Run the CLI with parsed arguments
pub fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Commands::Optimize {
            file,
            output,
            stdout,
            quiet,
        } => cmd_optimize(file, output, stdout, quiet),

        Commands::Fill {
            file,
            color,
            output,
            quiet,
        } => cmd_fill(file, color, output, quiet),

        Commands::Stroke {
            file,
            color,
            output,
            quiet,
        } => cmd_stroke(file, color, output, quiet),
    }
}

/// Optimize/minify an SVG file
fn cmd_optimize(
    file: PathBuf,
    output: Option<PathBuf>,
    to_stdout: bool,
    quiet: bool,
) -> Result<(), String> {
    // Read input file
    let content = std::fs::read_to_string(&file)
        .map_err(|e| format!("Failed to read '{}': {}", file.display(), e))?;

    let original_size = content.len();

    // Optimize
    let config = OptimizeConfig::default();
    let optimized = optimize(&content, &config)
        .map_err(|e| format!("Failed to optimize SVG: {}", e))?;

    let new_size = optimized.len();

    // Output
    if to_stdout {
        print!("{}", optimized);
    } else {
        let out_path = output.unwrap_or(file.clone());
        std::fs::write(&out_path, &optimized)
            .map_err(|e| format!("Failed to write '{}': {}", out_path.display(), e))?;

        if !quiet {
            let reduction = calculate_reduction(original_size, new_size);
            eprintln!(
                "Optimized: {} → {} ({:.1}% reduction)",
                format_size(original_size),
                format_size(new_size),
                reduction
            );
        }
    }

    Ok(())
}

/// Change fill color of all elements
fn cmd_fill(
    file: PathBuf,
    color: String,
    output: Option<PathBuf>,
    quiet: bool,
) -> Result<(), String> {
    // Validate and normalize color
    let normalized = normalize_color(&color)
        .map_err(|_| format!("Invalid color format: '{}'", color))?;

    // Read input file
    let content = std::fs::read_to_string(&file)
        .map_err(|e| format!("Failed to read '{}': {}", file.display(), e))?;

    // Parse and modify
    let mut doc = SvgDocument::parse(&content)
        .map_err(|e| format!("Failed to parse SVG: {}", e))?;

    doc.set_fill(&normalized)
        .map_err(|e| format!("Failed to set fill color: {}", e))?;

    // Write output
    let out_path = output.unwrap_or(file.clone());
    std::fs::write(&out_path, doc.to_string())
        .map_err(|e| format!("Failed to write '{}': {}", out_path.display(), e))?;

    if !quiet {
        eprintln!("Updated fill color to {} in '{}'", normalized, out_path.display());
    }

    Ok(())
}

/// Change stroke color of all elements
fn cmd_stroke(
    file: PathBuf,
    color: String,
    output: Option<PathBuf>,
    quiet: bool,
) -> Result<(), String> {
    // Validate and normalize color
    let normalized = normalize_color(&color)
        .map_err(|_| format!("Invalid color format: '{}'", color))?;

    // Read input file
    let content = std::fs::read_to_string(&file)
        .map_err(|e| format!("Failed to read '{}': {}", file.display(), e))?;

    // Parse and modify
    let mut doc = SvgDocument::parse(&content)
        .map_err(|e| format!("Failed to parse SVG: {}", e))?;

    doc.set_stroke(&normalized)
        .map_err(|e| format!("Failed to set stroke color: {}", e))?;

    // Write output
    let out_path = output.unwrap_or(file.clone());
    std::fs::write(&out_path, doc.to_string())
        .map_err(|e| format!("Failed to write '{}': {}", out_path.display(), e))?;

    if !quiet {
        eprintln!("Updated stroke color to {} in '{}'", normalized, out_path.display());
    }

    Ok(())
}
