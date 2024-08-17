<div align="center">

<h1>📋 Codexio</h1> 

<h4>A lightning-fast Rust-based CLI tool that distills any codebase into a powerful LLM prompt, no matter the size.</h3>

<img src=".assets/logo.webp" alt="Codexio Demo" width="600">

</div>

<hr />

## Table of Contents
1. [Introduction](#introduction)
2. [Features](#features)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Configuration](#configuration)
6. [Contributing](#contributing)
7. [Testing](#testing)
8. [License](#license)
9. [Acknowledgements](#acknowledgements)

## Introduction

Codexio is a powerful command-line interface (CLI) tool designed to bridge the gap between complex codebases and large language models (LLMs). By efficiently processing and summarizing codebases of any size, Codexio creates optimized prompts that enable LLMs to provide more accurate and context-aware responses.

### Key Benefits:

- **Instant Code Understanding**: Quickly generate comprehensive summaries of large codebases.
- **10x LLM Interactions**: Create context-rich prompts for more accurate AI-assisted coding and analysis.
- **Time-Saving**: Automate the process of extracting relevant code information for LLM inputs.
- **Flexible Integration**: Works with various LLM providers and can be customized for specific use cases.

Whether you're a startup founder rapidly prototyping ideas, a corporate developer streamlining code review processes, or an AI researcher pushing the boundaries of code generation, Codexio empowers you to leverage LLMs more effectively in your development workflow.

## Features

- **Rapid Codebase Analysis**: Quickly traverse and analyze entire codebases, regardless of size or complexity.
- **Intelligent File Filtering**: Include or exclude specific files or directories using flexible glob patterns.
- **Git Integration**: Generate diffs and retrieve logs between branches to focus on recent changes.
- **Customizable Output**: Tailor the generated prompt using Handlebars templates for different LLM models or specific use cases.
- **Token Optimization**: Count and optimize tokens for various LLM models to ensure efficient use of model capacity.
- **Clipboard Integration**: Automatically copy generated prompts to the clipboard for quick use in LLM interfaces.
- **Progress Visualization**: Real-time progress indicators for long-running operations.
- **Flexible Output Options**: Write results to files or display them in the console, with optional JSON formatting.

## Installation

### Prerequisites

- Rust (version 1.56 or later)
- Git (for version control related features)

### Steps

1. Clone the repository:
   ```
   git clone https://github.com/ethan-wickstrom/codexio.git
   ```

2. Navigate to the project directory:
   ```
   cd codexio
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. (Optional) Add the binary to your PATH:
   ```
   cp target/release/codexio /usr/local/bin/
   ```

## Usage

### Basic Usage

Generate a prompt from a codebase:

```
codexio /path/to/your/codebase
```

This will analyze the codebase and output a formatted prompt suitable for use with an LLM.

### Advanced Options

- Include specific files or patterns:
  ```
  codexio /path/to/codebase --include="*.rs,*.toml"
  ```

- Exclude files or directories:
  ```
  codexio /path/to/codebase --exclude="target/*,*.log"
  ```

- Generate a git diff:
  ```
  codexio /path/to/codebase --diff
  ```

- Count tokens for a specific model:
  ```
  codexio /path/to/codebase --tokens --encoding=cl100k
  ```

- Use a custom template:
  ```
  codexio /path/to/codebase --template=/path/to/custom/template.hbs
  ```

- Output to a file:
  ```
  codexio /path/to/codebase --output=output.txt
  ```

### Example Workflow

1. Navigate to your project directory:
   ```
   cd /path/to/your/project
   ```

2. Generate a prompt focusing on recent changes:
   ```
   codexio . --diff --include="src/**/*.rs" --exclude="tests/*" --tokens
   ```

3. The generated prompt will be copied to your clipboard and can be pasted directly into your preferred LLM interface.

## Configuration

### Command-line Options

Here's a comprehensive list of available command-line options:

| Option               | Description                                             | Example                            |
|----------------------|---------------------------------------------------------|------------------------------------|
| `--include`          | Patterns to include in the analysis (comma-separated)   | `--include="*.rs,*.toml"`          |
| `--exclude`          | Patterns to exclude from the analysis (comma-separated) | `--exclude="tests/*,*.log"`        |
| `--include-priority` | Prioritize include patterns over exclude patterns       | `--include-priority`               |
| `--tokens`           | Display token count for the generated prompt            | `--tokens`                         |
| `--encoding`         | Specify the tokenizer to use                            | `--encoding=cl100k`                |
| `--output`           | Specify an output file for the generated prompt         | `--output=output.txt`              |
| `--diff`             | Include git diff in the output                          | `--diff`                           |
| `--git-diff-branch`  | Generate diff between two specified branches            | `--git-diff-branch="main,feature"` |
| `--git-log-branch`   | Retrieve git log between two specified branches         | `--git-log-branch="main,feature"`  |
| `--line-number`      | Add line numbers to source code in the output           | `--line-number`                    |
| `--no-codeblock`     | Disable wrapping code inside markdown code blocks       | `--no-codeblock`                   |
| `--relative-paths`   | Use relative paths instead of absolute paths            | `--relative-paths`                 |
| `--no-clipboard`     | Disable automatic copying to clipboard                  | `--no-clipboard`                   |
| `--template`         | Specify a custom Handlebars template file               | `--template=custom.hbs`            |
| `--json`             | Output results in JSON format                           | `--json`                           |

For a full list of options with detailed descriptions, run:
```bash
codexio --help
```

## Customization

### Custom Templates

Codexio uses Handlebars templates to generate prompts. You can create custom templates to tailor the output to your specific needs.

Here's an example of a custom template:

    # Project: {{absolute_code_path}}
    
    ## Structure
    
    ```plaintext
    {{source_tree}}
    ```
    
    ## Files
    
    {{#each files}}
    
    ### `{{path}}`
    
    ```{{extension}}
    {{code}}
    ```
    
    {{/each}}
    
    {{#if git_diff}}
    ## Recent Changes
    ```diff
    {{git_diff}}
    ```
    {{/if}}

Save this template as `custom_template.hbs` and use it with Codexio:

`codexio /path/to/your/codebase --template=custom_template.hbs`

For more information on Handlebars templates, refer to the [official documentation](https://handlebarsjs.com/guide/).

## Contributing

We welcome contributions to Codexio! Here's how you can help:

1. Fork the repository on GitHub.
2. Create a new branch for your feature or bug fix.
3. Write your code, ensuring it follows the project's coding standards.
4. Add or update tests as necessary.
5. Commit your changes and push to your fork.
6. Submit a pull request with a clear description of your changes.

Please ensure your code adheres to the following guidelines:
- Follow the Rust style guide.
- Write clear, concise commit messages.
- Include unit tests for new features or bug fixes.
- Update documentation as necessary.

## Testing

Codexio uses Rust's built-in testing framework. To run the tests:

```
cargo test
```

This will run both unit tests and integration tests. For more verbose output:

```
cargo test -- --nocapture
```

## License

Codexio is distributed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

Codexio makes use of several open-source libraries, including:
- [clap](https://github.com/clap-rs/clap) for parsing command-line arguments
- [handlebars](https://github.com/sunng87/handlebars-rust) for template rendering
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) for token counting
- [git2](https://github.com/rust-lang/git2-rs) for Git operations

We're grateful to anyone who has contributed to these projects, including the maintainers and contributors.

### Special Thanks

- [code2prompt](https://github.com/mufeedvh/code2prompt) for the inspiration and motivation to create Codexio.
- [Mufeed VH](https://github.com/mufeedvh) for creating the [code2prompt](https://github.com/mufeedvh/code2prompt) project.

---

For the latest updates and more information, visit the [Codexio GitHub repository](https://github.com/ethan-wickstrom/codexio).
