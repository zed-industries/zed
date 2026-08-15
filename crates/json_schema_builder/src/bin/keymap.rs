use std::{collections::BTreeSet, env, fs, path::Path, time::Instant};

use json_schema_builder::*;

const SCHEMA_PATH: &str = "schemas/keymap.schema.json";

fn main() {
    let root = env::args().nth(1).unwrap_or_else(|| ".".into());
    if let Err(err) = generate_schema(Path::new(&root), Path::new(SCHEMA_PATH)) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn generate_schema(root: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Scanning {}...", root.display());
    let start = Instant::now();
    let files = find_action_files(root.to_str().unwrap());
    let scan_elapsed = start.elapsed();
    println!("Found {} action files ({:.2?})", files.len(), scan_elapsed);
    let start = Instant::now();
    let mut commands = BTreeSet::new();
    for (index, path) in files.iter().enumerate() {
        eprintln!("  [{}/{}] {}", index + 1, files.len(), path.display());
        collect_file(path, &mut commands);
    }
    println!(
        "Found {} unique actions ({:.2?})",
        commands.len(),
        start.elapsed()
    );

    let source = fs::read_to_string(output)
        .map_err(|err| format!("failed to read {}: {err}", output.display()))?;

    let mut schema: serde_json::Value = serde_json::from_str(&source)
        .map_err(|err| format!("failed to parse {}: {err}", output.display()))?;

    schema["$defs"]["action"]["oneOf"][0]["enum"] = commands
        .iter()
        .cloned()
        .map(serde_json::Value::String)
        .collect::<Vec<_>>()
        .into();

    let formatted = serde_json::to_string_pretty(&schema)?;

    fs::write(output, formatted + "\n")
        .map_err(|err| format!("failed to write {}: {err}", output.display()))?;

    println!("Updated {}", output.display());

    Ok(())
}
