// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use clap::Parser;
use opensvg_lib::cli::Cli;

fn main() {
    // Check if we have CLI arguments (subcommands)
    let args: Vec<String> = std::env::args().collect();

    // If we have more than just the program name and it looks like a subcommand, run CLI
    if args.len() > 1 && is_cli_command(&args[1]) {
        run_cli();
    } else {
        // Otherwise, launch GUI
        opensvg_lib::run();
    }
}

/// Check if the argument looks like a CLI subcommand
fn is_cli_command(arg: &str) -> bool {
    matches!(
        arg,
        "optimize" | "opt" | "fill" | "stroke" | "help" | "--help" | "-h" | "--version" | "-V"
    )
}

/// Run in CLI mode
fn run_cli() {
    let cli = Cli::parse();

    if let Err(e) = opensvg_lib::cli::run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
