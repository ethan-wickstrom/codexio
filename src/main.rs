use codexio::input::parse_config;
use codexio::output::{
    copy_to_clipboard, print_json_output, print_to_console, print_token_info, write_to_file,
};
use codexio::processing::process_codebase;
use anyhow::Result;
use colored::Colorize;
use codexio::path::label;

fn main() -> Result<()> {
    env_logger::init();

    // Parse Configuration
    let config = parse_config()?;

    // Process Codebase
    let (rendered, token_count, model_info, paths) = process_codebase(&config)?;

    // Output Handling
    if config.json {
        print_json_output(
            &rendered,
            &label(&config.path),
            token_count,
            &model_info,
            paths,
        )?;
    } else {
        if config.tokens {
            print_token_info(token_count, &model_info);
        }

        if !config.no_clipboard {
            if let Err(e) = copy_to_clipboard(&rendered) {
                eprintln!(
                    "{}{}{} {}",
                    "[".bold().white(),
                    "!".bold().red(),
                    "]".bold().white(),
                    format!("Failed to copy to clipboard: {}", e).red()
                );
                print_to_console(&rendered);
            }
        }

        if let Some(output_path) = &config.output {
            write_to_file(output_path, &rendered)?;
        }
    }

    Ok(())
}