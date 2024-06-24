use anyhow::Result;
use rustdoc::{convert_rustdoc_to_markdown, RustdocItem};
use std::path::{Path, PathBuf};

fn fetch_and_convert_docs(crate_docs_path: &Path) -> Result<(String, Vec<RustdocItem>)> {
    if !crate_docs_path.exists() {
        anyhow::bail!("File not found at {:?}", crate_docs_path);
    }

    let html_content = std::fs::read_to_string(crate_docs_path)?;
    let (markdown, items) = convert_rustdoc_to_markdown(html_content.as_bytes())?;

    Ok((markdown, items))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path_to_crate_docs>", args[0]);
        std::process::exit(1);
    }

    let crate_docs_path = PathBuf::from(&args[1]);

    let (markdown, items) = fetch_and_convert_docs(&crate_docs_path.join("index.html"))?;

    println!("Converted Markdown:\n{}", markdown);

    println!("\nLinked item contents:");
    for item in &items {
        println!("- {}", item.href);
        let item_path = crate_docs_path.join(item.href.as_ref());
        match fetch_and_convert_docs(&item_path) {
            Ok((item_markdown, _)) => {
                println!("{}\n", item_markdown);
            }
            Err(e) => {
                eprintln!(
                    "Failed to fetch and convert item: {} for path {:?}",
                    e, item.href
                );
            }
        }
    }

    Ok(())
}
