//! This module contains the functions for traversing the directory and processing the files.

use crate::filter::should_include_file;
use anyhow::Result;
use ignore::WalkBuilder;
use log::debug;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use termtree::Tree;

/// Traverses the directory and returns the string representation of the tree and the vector of JSON file representations.
///
/// # Arguments
///
/// * `root_path` - The path to the root directory.
/// * `include` - The patterns of files to include.
/// * `exclude` - The patterns of files to exclude.
/// * `include_priority` - Whether to give priority to include patterns.
/// * `line_number` - Whether to add line numbers to the code.
/// * `relative_paths` - Whether to use relative paths.
/// * `exclude_from_tree` - Whether to exclude files/folders from the tree based on exclude patterns.
/// * `no_codeblock` - Whether to disable wrapping code inside markdown code blocks.
///
/// # Returns
///
/// A tuple containing the string representation of the directory tree and a vector of JSON representations of the files.
pub fn traverse_directory(
    root_path: &Path,
    include: &[String],
    exclude: &[String],
    include_priority: bool,
    line_number: bool,
    relative_paths: bool,
    exclude_from_tree: bool,
    no_codeblock: bool,
) -> Result<(String, Vec<serde_json::Value>)> {
    // ~~~ Initialization ~~~
    let mut files = Vec::new();
    let canonical_root_path = root_path.canonicalize()?;

    // ~~~ Build the Tree ~~~
    let tree = build_directory_tree(
        &canonical_root_path,
        include,
        exclude,
        include_priority,
        exclude_from_tree,
    )?;

    // ~~~ Process the files ~~~
    process_files(
        &canonical_root_path,
        &mut files,
        include,
        exclude,
        include_priority,
        line_number,
        relative_paths,
        no_codeblock,
    )?;

    Ok((tree.to_string(), files))
}

/// Builds the directory tree structure.
///
/// # Arguments
///
/// * `canonical_root_path` - The canonicalized path to the root directory.
/// * `include` - The patterns of files to include.
/// * `exclude` - The patterns of files to exclude.
/// * `include_priority` - Whether to give priority to include patterns.
/// * `exclude_from_tree` - Whether to exclude files/folders from the tree based on exclude patterns.
///
/// # Returns
///
/// * `Result<Tree<String>>` - The directory tree structure.
fn build_directory_tree(
    canonical_root_path: &PathBuf,
    include: &[String],
    exclude: &[String],
    include_priority: bool,
    exclude_from_tree: bool,
) -> Result<Tree<String>> {
    let parent_directory = label(canonical_root_path);
    let tree = WalkBuilder::new(canonical_root_path)
        .git_ignore(true)
        .build()
        .filter_map(|e| e.ok())
        .fold(
            Tree::new(parent_directory.to_owned()),
            |mut root, entry| {
                let path = entry.path();
                if let Ok(relative_path) = path.strip_prefix(canonical_root_path) {
                    let mut current_tree = &mut root;
                    for component in relative_path.components() {
                        let component_str =
                            component.as_os_str().to_string_lossy().to_string();

                        // Check if the current component should be excluded from the tree
                        if exclude_from_tree
                            && !should_include_file(
                            path,
                            include,
                            exclude,
                            include_priority,
                        )
                        {
                            break;
                        }

                        current_tree = if let Some(pos) = current_tree
                            .leaves
                            .iter_mut()
                            .position(|child| child.root == component_str)
                        {
                            &mut current_tree.leaves[pos]
                        } else {
                            let new_tree = Tree::new(component_str.clone());
                            current_tree.leaves.push(new_tree);
                            current_tree.leaves.last_mut().unwrap()
                        };
                    }
                }
                root
            },
        );
    Ok(tree)
}

/// Processes the files in the directory, adding their JSON representations to the `files` vector.
///
/// # Arguments
///
/// * `canonical_root_path` - The canonicalized path to the root directory.
/// * `files` - The vector of JSON file representations.
/// * `include` - The patterns of files to include.
/// * `exclude` - The patterns of files to exclude.
/// * `include_priority` - Whether to give priority to include patterns.
/// * `line_number` - Whether to add line numbers to the code.
/// * `relative_paths` - Whether to use relative paths.
/// * `no_codeblock` - Whether to disable wrapping code inside markdown code blocks.
///
/// # Returns
///
/// * `Result<()>` - An empty result indicating success or an error.
fn process_files(
    canonical_root_path: &PathBuf,
    files: &mut Vec<serde_json::Value>,
    include: &[String],
    exclude: &[String],
    include_priority: bool,
    line_number: bool,
    relative_paths: bool,
    no_codeblock: bool,
) -> Result<()> {
    let parent_directory = label(canonical_root_path);
    for entry in WalkBuilder::new(canonical_root_path)
        .git_ignore(true)
        .build()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file()
            && should_include_file(path, include, exclude, include_priority)
        {
            process_file(
                path,
                files,
                &parent_directory,
                relative_paths,
                line_number,
                no_codeblock,
            )?;
        }
    }
    Ok(())
}

/// Processes a single file, adding its JSON representation to the `files` vector.
///
/// # Arguments
///
/// * `path` - The path to the file.
/// * `files` - The vector of JSON file representations.
/// * `parent_directory` - The name of the parent directory.
/// * `relative_paths` - Whether to use relative paths.
/// * `line_number` - Whether to add line numbers to the code.
/// * `no_codeblock` - Whether to disable wrapping code inside markdown code blocks.
///
/// # Returns
///
/// * `Result<()>` - An empty result indicating success or an error.
fn process_file(
    path: &Path,
    files: &mut Vec<serde_json::Value>,
    parent_directory: &str,
    relative_paths: bool,
    line_number: bool,
    no_codeblock: bool,
) -> Result<()> {
    if let Ok(code_bytes) = fs::read(path) {
        let code = String::from_utf8_lossy(&code_bytes);

        let code_block = wrap_code_block(
            &code,
            path.extension().and_then(|ext| ext.to_str()).unwrap_or(""),
            line_number,
            no_codeblock,
        );

        if !code.trim().is_empty() && !code.contains(char::REPLACEMENT_CHARACTER) {
            let file_path = if relative_paths {
                format!("{}/{}", parent_directory, path.strip_prefix(parent_directory).unwrap().display())
            } else {
                path.display().to_string()
            };

            files.push(json!({
                "path": file_path,
                "extension": path.extension().and_then(|ext| ext.to_str()).unwrap_or(""),
                "code": code_block,
            }));
            debug!(target: "included_files", "Included file: {}", file_path);
        } else {
            debug!("Excluded file (empty or invalid UTF-8): {}", path.display());
        }
    } else {
        debug!("Failed to read file: {}", path.display());
    }
    Ok(())
}

/// Returns the file name or the string representation of the path.
///
/// # Arguments
///
/// * `p` - The path to label.
///
/// # Returns
///
/// * `String` - The file name or string representation of the path.
pub fn label<P: AsRef<Path>>(p: P) -> String {
    let path = p.as_ref();
    if path.file_name().is_none() {
        let current_dir = std::env::current_dir().unwrap();
        current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(".")
            .to_owned()
    } else {
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_owned()
    }
}

/// Wraps the code block with a delimiter and adds line numbers if required.
///
/// # Arguments
///
/// * `code` - The code block to wrap.
/// * `extension` - The file extension of the code block.
/// * `line_numbers` - Whether to add line numbers to the code.
/// * `no_codeblock` - Whether to not wrap the code block with a delimiter.
///
/// # Returns
///
/// * `String` - The wrapped code block.
pub fn wrap_code_block(code: &str, extension: &str, line_numbers: bool, no_codeblock: bool) -> String {
    let delimiter = "`".repeat(3);
    let mut code_with_line_numbers = String::new();

    if line_numbers {
        for (line_number, line) in code.lines().enumerate() {
            code_with_line_numbers.push_str(&format!("{:4} | {}\n", line_number + 1, line));
        }
    } else {
        code_with_line_numbers = code.to_string();
    }

    if no_codeblock {
        code_with_line_numbers
    } else {
        format!(
            "{}{}\n{}\n{}",
            delimiter, extension, code_with_line_numbers, delimiter
        )
    }
}