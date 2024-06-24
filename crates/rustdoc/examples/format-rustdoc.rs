use anyhow::Result;
use fs::Fs;
use rustdoc::{convert_rustdoc_to_markdown, RustdocItem};
use std::{path::PathBuf, sync::Arc};

fn fetch_and_convert_docs(crate_docs_path: PathBuf) -> Result<(String, Vec<RustdocItem>)> {
    let index_path = crate_docs_path.join("index.html");
    if !index_path.exists() {
        anyhow::bail!("Index file not found at {:?}", index_path);
    }

    let html_content = std::fs::read_to_string(&index_path)?;
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

    let (markdown, items) = fetch_and_convert_docs(crate_docs_path)?;

    println!("Converted Markdown:\n{}", markdown);

    // println!("\nExtracted Items:");
    // for item in items {
    //     println!("- {:?}", item);
    // }

    Ok(())
}
