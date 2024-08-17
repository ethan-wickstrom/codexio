//! This module handles the core processing logic of the application, including directory traversal,
//! git operations, template rendering, and token counting.

use std::fs;
use crate::git::{get_git_diff, get_git_diff_between_branches, get_git_log};
use crate::input::{create_initial_data, parse_patterns};
use crate::path::traverse_directory;
use crate::template::{handlebars_setup, render_template};
use crate::token::{get_model_info, get_tokenizer};
use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};
use crate::config::Config;

/// Constants
const DEFAULT_TEMPLATE_NAME: &str = "default";
const CUSTOM_TEMPLATE_NAME: &str = "custom";

/// Processes the codebase based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The application configuration.
///
/// # Returns
///
/// * `Result<(String, usize, String, Vec<String>)>` - A tuple containing the rendered template, token count, model info, and file paths.
pub fn process_codebase(config: &Config) -> Result<(String, usize, String, Vec<String>)> {
    // Handlebars Template Setup
    let (template_content, template_name) = get_template(config)?;
    let handlebars = handlebars_setup(&template_content, template_name)?;

    // Progress Bar Setup
    let spinner = setup_spinner("Traversing directory and building tree...");

    // Parse Patterns
    let include_patterns = parse_patterns(&config.include)?;
    let exclude_patterns = parse_patterns(&config.exclude)?;

    // Traverse the directory
    let (tree, files) = traverse_directory(
        &config.path,
        &include_patterns,
        &exclude_patterns,
        config.include_priority,
        config.line_number,
        config.relative_paths,
        config.exclude_from_tree,
        config.no_codeblock,
    )?;

    // Git Diff
    let git_diff = if config.diff {
        spinner.set_message("Generating git diff...");
        get_git_diff(&config.path).unwrap_or_default()
    } else {
        String::new()
    };

    // Git Diff Between Branches
    let git_diff_branch = get_git_diff_between_branches_with_spinner(config, &spinner)?;

    // Git Log Between Branches
    let git_log_branch = get_git_log_between_branches_with_spinner(config, &spinner)?;

    spinner.finish_with_message("Done!".green().to_string());

    // Prepare JSON Data
    let mut data = create_initial_data(
        config,
        tree,
        files.clone(),
        git_diff,
        git_diff_branch,
        git_log_branch,
    );

    debug!(
        "JSON Data: {}",
        serde_json::to_string_pretty(&data).unwrap()
    );

    // Handle undefined variables
    crate::input::prompt_for_undefined_variables(&mut data, &template_content)?;

    // Render the template
    let rendered = render_template(&handlebars, template_name, &data)?;

    // Token Count
    let token_count = if config.tokens {
        let bpe = get_tokenizer(&config.encoding);
        bpe.encode_with_special_tokens(&rendered).len()
    } else {
        0
    };

    let paths: Vec<String> = files
        .iter()
        .filter_map(|file| file.get("path").and_then(|p| p.as_str()).map(String::from))
        .collect();

    let model_info = get_model_info(&config.encoding);

    Ok((rendered, token_count, model_info.parse()?, paths))
}

/// Generates the git diff between two branches, updating the progress spinner.
///
/// # Arguments
///
/// * `config` - The application configuration.
/// * `spinner` - The progress spinner.
///
/// # Returns
///
/// * `Result<String>` - The git diff between branches string.
fn get_git_diff_between_branches_with_spinner(
    config: &Config,
    spinner: &ProgressBar,
) -> Result<String> {
    if let Some(branches) = &config.git_diff_branch {
        spinner.set_message("Generating git diff between two branches...");
        let branches = parse_patterns(&Some(branches.to_string()))?;

        if branches.len() != 2 {
            error!("Please provide exactly two branches separated by a comma.");
            std::process::exit(1);
        }

        Ok(get_git_diff_between_branches(&config.path, &branches[0], &branches[1]).unwrap_or_default())
    } else {
        Ok(String::new())
    }
}

/// Retrieves the git log between two branches, updating the progress spinner.
///
/// # Arguments
///
/// * `config` - The application configuration.
/// * `spinner` - The progress spinner.
///
/// # Returns
///
/// * `Result<String>` - The git log between branches string.
fn get_git_log_between_branches_with_spinner(
    config: &Config,
    spinner: &ProgressBar,
) -> Result<String> {
    if let Some(branches) = &config.git_log_branch {
        spinner.set_message("Generating git log between two branches...");
        let branches = parse_patterns(&Some(branches.to_string()))?;

        if branches.len() != 2 {
            error!("Please provide exactly two branches separated by a comma.");
            std::process::exit(1);
        }

        Ok(get_git_log(&config.path, &branches[0], &branches[1]).unwrap_or_default())
    } else {
        Ok(String::new())
    }
}

/// Sets up a progress spinner with a given message.
///
/// # Arguments
///
/// * `message` - A message to display with the spinner.
///
/// # Returns
///
/// * `ProgressBar` - The configured progress spinner.
fn setup_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(120));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"])
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner
}

/// Retrieves the template content and name based on the CLI arguments.
///
/// # Arguments
///
/// * `config` - The parsed CLI arguments.
///
/// # Returns
///
/// * `Result<(String, &str)>` - A tuple containing the template content and name.
fn get_template(config: &Config) -> Result<(String, &str)> {
    if let Some(template_path) = &config.template {
        let content = fs::read_to_string(template_path)
            .context("Failed to read custom template file")?;
        Ok((content, CUSTOM_TEMPLATE_NAME))
    } else {
        Ok((
            include_str!("default_template.hbs").to_string(),
            DEFAULT_TEMPLATE_NAME,
        ))
    }
}