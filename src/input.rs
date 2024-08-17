//! This module handles the input operations, including parsing command-line arguments,
//! reading files, and interacting with the user for undefined variables.

use crate::config::Config;
use crate::path::label;
use crate::template::extract_undefined_variables;
use anyhow::{Context, Result};
use inquire::Text;
use log::debug;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::PathBuf;
use clap::Parser;

/// Parses the command-line arguments and returns a `Config` struct.
///
/// # Returns
///
/// * `Result<Config>` - The parsed configuration options.
pub fn parse_config() -> Result<Config> {
    let config = Config::parse();
    debug!("Parsed config: {:?}", config);
    Ok(config)
}

/// Reads the content of a file at the given path.
///
/// # Arguments
///
/// * `path` - The path to the file.
///
/// # Returns
///
/// * `Result<String>` - The content of the file as a string.
pub fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).context("Failed to read file")
}

/// Prompts the user for values of undefined variables in the template.
///
/// # Arguments
///
/// * `data` - The JSON data object to be populated with user-defined variables.
/// * `template_content` - The template content string.
///
/// # Returns
///
/// * `Result<()>` - An empty result indicating success or an error.
pub fn prompt_for_undefined_variables(data: &mut Value, template_content: &str) -> Result<()> {
    let undefined_variables = extract_undefined_variables(template_content);
    let mut user_defined_vars = Map::new();

    for var in undefined_variables {
        if !data.as_object().unwrap().contains_key(&var) {
            let prompt = format!("Enter value for '{}': ", var);
            let answer = Text::new(&prompt)
                .with_help_message("Fill user defined variable in template")
                .prompt()?;
            user_defined_vars.insert(var, Value::String(answer));
        }
    }

    if let Some(obj) = data.as_object_mut() {
        for (key, value) in user_defined_vars {
            obj.insert(key, value);
        }
    }

    Ok(())
}

/// Creates the initial JSON data object based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The application configuration.
/// * `tree` - The string representation of the directory tree.
/// * `files` - The vector of JSON file representations.
/// * `git_diff` - The git diff string.
/// * `git_diff_branch` - The git diff between branches string.
/// * `git_log_branch` - The git log between branches string.
///
/// # Returns
///
/// * `Value` - The initial JSON data object.
pub fn create_initial_data(
    config: &Config,
    tree: String,
    files: Vec<Value>,
    git_diff: String,
    git_diff_branch: String,
    git_log_branch: String,
) -> Value {
    json!({
        "absolute_code_path": label(&config.path),
        "source_tree": tree,
        "files": files,
        "git_diff": git_diff,
        "git_diff_branch": git_diff_branch,
        "git_log_branch": git_log_branch
    })
}

/// Parses comma-separated patterns into a vector of strings.
///
/// # Arguments
///
/// * `patterns` - An optional string containing comma-separated patterns.
///
/// # Returns
///
/// * `Result<Vec<String>>` - A vector of parsed patterns, or an error if parsing fails.
pub fn parse_patterns(patterns: &Option<String>) -> Result<Vec<String>> {
    match patterns {
        Some(patterns) if !patterns.is_empty() => {
            Ok(patterns
                .split(',')
                .map(|s| s.trim().to_string())
                .collect())
        }
        _ => Ok(vec![]),
    }
}