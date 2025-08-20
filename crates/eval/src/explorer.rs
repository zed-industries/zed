use anyhow::{Context as _, Result};
use clap::Parser;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(about = "Generate HTML explorer from JSON thread files")]
struct Args {
    /// Paths to JSON files or directories. If a directory is provided,
    /// it will be searched for 'last.messages.json' files up to 2 levels deep.
    #[clap(long, required = true, num_args = 1..)]
    input: Vec<PathBuf>,

    /// Path where the output HTML file will be written
    #[clap(long)]
    output: PathBuf,
}

/// Recursively finds files with `target_filename` in `dir_path` up to `max_depth`.
#[allow(dead_code)]
fn find_target_files_recursive(
    dir_path: &Path,
    target_filename: &str,
    current_depth: u8,
    max_depth: u8,
    found_files: &mut Vec<PathBuf>,
) -> Result<()> {
    if current_depth > max_depth {
        return Ok(());
    }

    for entry_result in fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read directory: {}", dir_path.display()))?
    {
        let entry = entry_result.with_context(|| {
            format!("Failed to read directory entry in: {}", dir_path.display())
        })?;
        let path = entry.path();

        if path.is_dir() {
            find_target_files_recursive(
                &path,
                target_filename,
                current_depth + 1,
                max_depth,
                found_files,
            )?;
        } else if path.is_file()
            && let Some(filename_osstr) = path.file_name()
            && let Some(filename_str) = filename_osstr.to_str()
            && filename_str == target_filename
        {
            found_files.push(path);
        }
    }
    Ok(())
}

pub fn generate_explorer_html(input_paths: &[PathBuf], output_path: &PathBuf) -> Result<String> {
    if let Some(parent) = output_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).context(format!(
            "Failed to create output directory: {}",
            parent.display()
        ))?;
    }

    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/explorer.html");
    let template_content = fs::read_to_string(&template_path).context(format!(
        "Template file not found or couldn't be read: {}",
        template_path.display()
    ))?;

    if input_paths.is_empty() {
        println!(
            "No input JSON files found to process. Explorer will be generated with template defaults or empty data."
        );
    }

    let threads = input_paths
        .iter()
        .map(|input_path| {
            let file_content = fs::read_to_string(input_path)
                .context(format!("Failed to read file: {}", input_path.display()))?;
            let mut thread_data: Value = file_content
                .parse::<Value>()
                .context(format!("Failed to parse JSON from file: {}", input_path.display()))?;

            if let Some(obj) = thread_data.as_object_mut() {
                obj.insert("filename".to_string(), json!(input_path.display().to_string()));
            } else {
                eprintln!("Warning: JSON data in {} is not a root object. Wrapping it to include filename.", input_path.display());
                thread_data = json!({
                    "original_data": thread_data,
                    "filename": input_path.display().to_string()
                });
            }
            Ok(thread_data)
        })
        .collect::<Result<Vec<_>>>()?;

    let all_threads_data = json!({ "threads": threads });
    let html_content = inject_thread_data(template_content, all_threads_data)?;
    fs::write(&output_path, &html_content)
        .context(format!("Failed to write output: {}", output_path.display()))?;

    println!(
        "Saved data from {} resolved file(s) ({} threads) to {}",
        input_paths.len(),
        threads.len(),
        output_path.display()
    );
    Ok(html_content)
}

fn inject_thread_data(template: String, threads_data: Value) -> Result<String> {
    let injection_marker = "let threadsData = window.threadsData || { threads: [dummyThread] };";
    if !template.contains(injection_marker) {
        anyhow::bail!(
            "Could not find the thread injection point in the template. Expected: '{}'",
            injection_marker
        );
    }

    let threads_json_string = serde_json::to_string_pretty(&threads_data)
        .context("Failed to serialize threads data to JSON")?
        .replace("</script>", r"<\/script>");

    let script_injection = format!("let threadsData = {};", threads_json_string);
    let final_html = template.replacen(injection_marker, &script_injection, 1);

    Ok(final_html)
}

#[cfg(not(any(test, doctest)))]
#[allow(dead_code)]
fn main() -> Result<()> {
    let args = Args::parse();

    const DEFAULT_FILENAME: &str = "last.messages.json";
    const MAX_SEARCH_DEPTH: u8 = 2;

    let mut resolved_input_files: Vec<PathBuf> = Vec::new();

    for input_path_arg in &args.input {
        if !input_path_arg.exists() {
            eprintln!(
                "Warning: Input path {} does not exist. Skipping.",
                input_path_arg.display()
            );
            continue;
        }

        if input_path_arg.is_dir() {
            find_target_files_recursive(
                input_path_arg,
                DEFAULT_FILENAME,
                0, // starting depth
                MAX_SEARCH_DEPTH,
                &mut resolved_input_files,
            )
            .with_context(|| {
                format!(
                    "Error searching for '{}' files in directory: {}",
                    DEFAULT_FILENAME,
                    input_path_arg.display()
                )
            })?;
        } else if input_path_arg.is_file() {
            resolved_input_files.push(input_path_arg.clone());
        }
    }

    resolved_input_files.sort_unstable();
    resolved_input_files.dedup();

    println!("No input paths provided/found.");

    generate_explorer_html(&resolved_input_files, &args.output).map(|_| ())
}
