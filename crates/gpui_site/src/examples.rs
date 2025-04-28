use crate::templates::ExampleInfo;
use anyhow::{Context, Result};
use std::path::Path
use syntect::highlighting::ThemeSet;
use syntect::html::{highlighted_html_for_string, IncludeBackground};
use syntect::parsing::SyntaxSet;

/// Collect information about examples in the gpui crate
pub fn collect_examples(gpui_dir: &Path) -> Result<Vec<ExampleInfo>> {
    let examples_dir = gpui_dir.join("examples");
    let mut examples = Vec::new();

    // Check if the examples directory exists
    if !examples_dir.exists() {
        return Ok(examples);
    }

    // Read the Cargo.toml to get example information
    let cargo_toml_path = gpui_dir.join("Cargo.toml");
    let cargo_toml = std::fs::read_to_string(cargo_toml_path).with_context(|| {
        format!(
            "Failed to read Cargo.toml from {}",
            cargo_toml_path.display()
    )
    })?;

    let cargo_data: toml::Value =
        toml::from_str(&cargo_toml).with_context(|| "Failed to parse Cargo.toml")?;

    // Extract example definitions from Cargo.toml
    if let Some(example_array) = cargo_data.get("example").and_then(|v| v.as_array()) {
        for example in example_array {
            if let (Some(name), Some(path)) = (
                example.get("name").and_then(|v| v.as_str()),
                example.get("path").and_then(|v| v.as_str()),
            ) {
                let example_path = Path::new(path);
                let title = title_case(name);

                // Read the first comment block to extract description, if any
                let mut description = format!("Example demonstrating {}", title.to_lowercase());
                if let Ok(content) = std::fs::read_to_string(gpui_dir.join(path)) {
                    if let Some(comment) = extract_first_comment(&content) {
                        description = comment;
                    }
                }

                examples.push(ExampleInfo {
                    name: name.to_string(),
                    title,
                    description,
                    path: format!("{}.html", name),
                });
            }
        }
    } else {
        // If no example definitions in Cargo.toml, scan the examples directory
        for entry in walkdir::WalkDir::new(&examples_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let file_stem = path
                    .file_stem()
                    .ok_or_else(|| {
                        anyhow::anyhow!("Failed to get file stem for {}", path.display())
                    })?
                    .to_string_lossy();

                // Skip mod.rs or lib.rs files
                if file_stem == "mod" || file_stem == "lib" {
                    continue;
                }

                let name = file_stem.to_string();
                let title = title_case(&name);

                // Read the first comment block to extract description, if any
                let mut description = format!("Example demonstrating {}", title.to_lowercase());
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Some(comment) = extract_first_comment(&content) {
                        description = comment;
                    }
                }

                examples.push(ExampleInfo {
                    name: name.clone(),
                    title,
                    description,
                    path: format!("{}.html", name),
                });
            }
        }
    }

    // Sort examples by name
    examples.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(examples)
}

/// Read an example file and return its highlighted HTML content
pub fn read_example_file(gpui_dir: &Path, example_name: &str) -> Result<String> {
    // First try to find the example in Cargo.toml
    let cargo_toml_path = gpui_dir.join("Cargo.toml");
    let cargo_toml = std::fs::read_to_string(cargo_toml_path)?;

    let cargo_data: toml::Value = toml::from_str(&cargo_toml)?;

    let mut example_path = None;

    // Look for example in Cargo.toml
    if let Some(example_array) = cargo_data.get("example").and_then(|v| v.as_array()) {
        for example in example_array {
            if let (Some(name), Some(path)) = (
                example.get("name").and_then(|v| v.as_str()),
                example.get("path").and_then(|v| v.as_str()),
            ) {
                if name == example_name {
                    example_path = Some(gpui_dir.join(path));
                    break;
                }
            }
        }
    }

    // If not found in Cargo.toml, look in examples directory
    if example_path.is_none() {
        let examples_dir = gpui_dir.join("examples");
        let rs_file = examples_dir.join(format!("{}.rs", example_name));

        if rs_file.exists() {
            example_path = Some(rs_file);
        } else {
            // Check if it's in a subdirectory
            let dir_file = examples_dir
                .join(example_name)
                .join(format!("{}.rs", example_name));
            if dir_file.exists() {
                example_path = Some(dir_file);
            }
        }
    }

    // Read and highlight the code
    if let Some(path) = example_path {
        let code = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read example file: {}", path.display()))?;

        // Syntax highlight the code
        highlight_rust_code(&code)
    } else {
        Err(anyhow::anyhow!("Example file not found: {}", example_name))
    }
}

/// Highlight Rust code
fn highlight_rust_code(code: &str) -> Result<String> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["base16-ocean.dark"];

    let syntax = syntax_set
        .find_syntax_by_extension("rs")
        .ok_or_else(|| anyhow::anyhow!("Could not find Rust syntax"))?;

    let highlighted =
        highlighted_html_for_string(code, &syntax_set, syntax, theme, IncludeBackground::Yes)?;

    Ok(highlighted)
}

/// Extract the first comment block from a Rust file
fn extract_first_comment(content: &str) -> Option<String> {
    let mut in_comment = false;
    let mut comment_lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("//") {
            // Single line comment
            let comment_text = trimmed.trim_start_matches("//").trim();
            comment_lines.push(comment_text.to_string());
        } else if trimmed.starts_with("/*") {
            // Start of multi-line comment
            in_comment = true;
            let comment_text = trimmed.trim_start_matches("/*").trim();
            if !comment_text.is_empty() {
                comment_lines.push(comment_text.to_string());
            }
        } else if in_comment && trimmed.contains("*/") {
            // End of multi-line comment
            in_comment = false;
            let comment_text = trimmed.split("*/").next().unwrap_or("").trim();
            if !comment_text.is_empty() {
                comment_lines.push(comment_text.to_string());
            }
            break;
        } else if in_comment {
            // Middle of multi-line comment
            comment_lines.push(trimmed.to_string());
        } else if !trimmed.is_empty() && !comment_lines.is_empty() {
            // We've hit non-comment code, stop looking
            break;
        }
    }

    if comment_lines.is_empty() {
        None
    } else {
        Some(comment_lines.join(" "))
    }
}

/// Convert a snake_case string to Title Case
fn title_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

