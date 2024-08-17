//! This module defines the configuration options for the application.

use clap::Parser;
use std::path::PathBuf;

/// Configuration options for the application.
#[derive(Parser, Debug)]
#[clap(name = "codexio", version = "2.0.0", author = "Mufeed VH")]
pub struct Config {
    /// Path to the codebase directory.
    #[arg()]
    pub path: PathBuf,

    /// Patterns to include.
    #[clap(long)]
    pub include: Option<String>,

    /// Patterns to exclude.
    #[clap(long)]
    pub exclude: Option<String>,

    /// Include files in case of conflict between include and exclude patterns.
    #[clap(long)]
    pub include_priority: bool,

    /// Exclude files/folders from the source tree based on exclude patterns.
    #[clap(long)]
    pub exclude_from_tree: bool,

    /// Display the token count of the generated prompt.
    #[clap(long)]
    pub tokens: bool,

    /// Optional tokenizer to use for token count.
    ///
    /// Supported tokenizers: cl100k (default), p50k, p50k_edit, r50k, gpt2.
    #[clap(short = 'c', long)]
    pub encoding: Option<String>,

    /// Optional output file path.
    #[clap(short, long)]
    pub output: Option<String>,

    /// Include git diff.
    #[clap(short, long)]
    pub diff: bool,

    /// Generate git diff between two branches.
    #[clap(long, value_name = "BRANCHES")]
    pub git_diff_branch: Option<String>,

    /// Retrieve git log between two branches.
    #[clap(long, value_name = "BRANCHES")]
    pub git_log_branch: Option<String>,

    /// Add line numbers to the source code.
    #[clap(short, long)]
    pub line_number: bool,

    /// Disable wrapping code inside markdown code blocks.
    #[clap(long)]
    pub no_codeblock: bool,

    /// Use relative paths instead of absolute paths, including the parent directory.
    #[clap(long)]
    pub relative_paths: bool,

    /// Optional Disable copying to clipboard.
    #[clap(long)]
    pub no_clipboard: bool,

    /// Optional Path to a custom Handlebars template.
    #[clap(short, long)]
    pub template: Option<PathBuf>,

    /// Print output as JSON.
    #[clap(long)]
    pub json: bool,
}