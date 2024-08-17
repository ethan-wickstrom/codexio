//! This module handles the output operations, including printing to the console,
//! copying to the clipboard, and writing to a file.

use anyhow::{Context, Result};
use arboard::Clipboard;
use colored::*;
use std::fs::File;
use std::io::Write;
use serde_json::json;

/// Prints the rendered template to the console.
///
/// # Arguments
///
/// * `rendered` - The rendered template string.
pub fn print_to_console(rendered: &str) {
    println!("{}", rendered);
}

/// Copies the rendered template to the clipboard.
///
/// # Arguments
///
/// * `rendered` - The rendered template string.
///
/// # Returns
///
/// * `Result<()>` - An empty result indicating success or an error.
pub fn copy_to_clipboard(rendered: &str) -> Result<()> {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            clipboard
                .set_text(rendered.to_string())
                .context("Failed to copy to clipboard")?;
            println!(
                "{}{}{} {}",
                "[".bold().white(),
                "✓".bold().green(),
                "]".bold().white(),
                "Copied to clipboard successfully.".green()
            );
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!("Failed to initialize clipboard: {}", e)),
    }
}

/// Writes the rendered template to a specified output file.
///
/// # Arguments
///
/// * `output_path` - The path to the output file.
/// * `rendered` - The rendered template string.
///
/// # Returns
///
/// * `Result<()>` - An empty result indicating success or an error.
pub fn write_to_file(output_path: &str, rendered: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = std::io::BufWriter::new(file);
    write!(writer, "{}", rendered)?;
    println!(
        "{}{}{} {}",
        "[".bold().white(),
        "✓".bold().green(),
        "]".bold().white(),
        format!("Prompt written to file: {}", output_path).green()
    );
    Ok(())
}

/// Prints the token count and model information to the console.
///
/// # Arguments
///
/// * `token_count` - The number of tokens in the rendered template.
/// * `model_info` - The model information string.
pub fn print_token_info(token_count: usize, model_info: &str) {
    println!(
        "{}{}{} Token count: {}, Model info: {}",
        "[".bold().white(),
        "i".bold().blue(),
        "]".bold().white(),
        token_count.to_string().bold().yellow(),
        model_info
    );
}

/// Prints the output in JSON format.
///
/// # Arguments
///
/// * `rendered` - The rendered template string.
/// * `directory_name` - The name of the directory.
/// * `token_count` - The number of tokens in the rendered template.
/// * `model_info` - The model information string.
/// * `files` - A vector of file paths.
///
/// # Returns
///
/// * `Result<()>` - An empty result indicating success or an error.
pub fn print_json_output(
    rendered: &str,
    directory_name: &str,
    token_count: usize,
    model_info: &str,
    files: Vec<String>,
) -> Result<()> {
    let json_output = json!({
        "prompt": rendered,
        "directory_name": directory_name,
        "token_count": token_count,
        "model_info": model_info,
        "files": files,
    });
    println!("{}", serde_json::to_string_pretty(&json_output)?);
    Ok(())
}