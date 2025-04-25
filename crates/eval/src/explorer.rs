use anyhow::{Context, Result, anyhow};
use clap::Parser;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(about = "Generate HTML explorer from JSON thread files")]
struct Args {
    /// Paths to JSON files containing thread data
    #[clap(long, required = true, num_args = 1..)]
    input: Vec<PathBuf>,

    /// Path where the HTML explorer file will be written
    #[clap(long)]
    output: PathBuf,
}

pub fn generate_explorer_html(inputs: &[PathBuf], output: &PathBuf) -> Result<String> {
    if let Some(parent) = output.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context(format!(
                "Failed to create output directory: {}",
                parent.display()
            ))?;
        }
    }

    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/explorer.html");
    let template = fs::read_to_string(&template_path).context(format!(
        "Template file not found or couldn't be read: {}",
        template_path.display()
    ))?;

    let threads = inputs
        .iter()
        .map(|input_path| {
            let mut thread_data: Value = fs::read_to_string(input_path)
                .context(format!("Failed to read file: {}", input_path.display()))?
                .parse::<Value>()
                .context(format!("Failed to parse JSON: {}", input_path.display()))?;
            thread_data["filename"] = json!(input_path); // This will be shown in a thread heading
            Ok(thread_data)
        })
        .collect::<Result<Vec<_>>>()?;

    let all_threads = json!({ "threads": threads });
    let html_content = inject_thread_data(template, all_threads)?;
    fs::write(&output, &html_content)
        .context(format!("Failed to write output: {}", output.display()))?;

    println!("Saved {} thread(s) to {}", threads.len(), output.display());
    Ok(html_content)
}

fn inject_thread_data(template: String, threads_data: Value) -> Result<String> {
    let injection_marker = "let threadsData = window.threadsData || { threads: [dummyThread] };";
    template
        .find(injection_marker)
        .ok_or_else(|| anyhow!("Could not find the thread injection point in the template"))?;

    let threads_json = serde_json::to_string_pretty(&threads_data)
        .context("Failed to serialize threads data to JSON")?;
    let script_injection = format!("let threadsData = {};", threads_json);
    let final_html = template.replacen(injection_marker, &script_injection, 1);

    Ok(final_html)
}

#[cfg(not(any(test, doctest)))]
#[allow(dead_code)]
fn main() -> Result<()> {
    let args = Args::parse();
    generate_explorer_html(&args.input, &args.output).map(|_| ())
}
