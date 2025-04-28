use crate::examples::{collect_examples, read_example_file};
use crate::markdown::read_markdown_file;
use crate::templates::{DocInfo, SiteContent, TemplateEngine};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Generate the complete gpui site
pub fn generate_site(gpui_dir: &Path, output_dir: &Path) -> Result<()> {
    // Create necessary directories
    fs::create_dir_all(output_dir.join("examples"))?;
    fs::create_dir_all(output_dir.join("docs"))?;
    fs::create_dir_all(output_dir.join("css"))?;
    fs::create_dir_all(output_dir.join("js"))?;

    // Collect examples
    let examples = collect_examples(gpui_dir)?;
    println!("Found {} examples", examples.len());

    // Collect docs
    let docs = collect_docs(gpui_dir)?;
    println!("Found {} docs", docs.len());

    // Create site content
    let site_content = SiteContent {
        title: "gpui".to_string(),
        content: process_readme(gpui_dir)?,
        examples: examples.clone(),
        docs: docs.clone(),
    };

    // Create template engine
    let templates_dir = output_dir.join("templates");
    let template_engine = TemplateEngine::new(&templates_dir)?;

    // Generate index page
    let index_html = template_engine.render_index(&site_content)?;
    fs::write(output_dir.join("index.html"), index_html)?;

    // Generate example pages
    for example in &examples {
        let code = read_example_file(gpui_dir, &example.name)?;
        let html = template_engine.render_example(example, &code, &site_content)?;
        fs::write(output_dir.join("examples").join(&example.path), html)?;
    }

    // Generate doc pages
    for doc in &docs {
        let doc_path = get_doc_path(gpui_dir, &doc.name)?;
        let content = read_markdown_file(&doc_path)?;
        let html = template_engine.render_doc(doc, &content, &site_content)?;
        fs::write(output_dir.join("docs").join(&doc.path), html)?;
    }

    // Copy assets
    generate_css(output_dir)?;
    generate_js(output_dir)?;

    Ok(())
}

/// Process the README for the index page
fn process_readme(gpui_dir: &Path) -> Result<String> {
    let readme_path = gpui_dir.join("README.md");
    read_markdown_file(&readme_path)
}

/// Collect documentation files
fn collect_docs(gpui_dir: &Path) -> Result<Vec<DocInfo>> {
    let docs_dir = gpui_dir.join("docs");
    let mut docs = Vec::new();

    // Check if docs directory exists
    if !docs_dir.exists() {
        return Ok(docs);
    }

    // Add README.md as intro document
    docs.push(DocInfo {
        name: "README.md".to_string(),
        title: "Introduction".to_string(),
        path: "intro.html".to_string(),
    });

    // Walk docs directory
    for entry in walkdir::WalkDir::new(&docs_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let file_name = path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Failed to get file name for {}", path.display()))?
                .to_string_lossy();

            let file_stem = path
                .file_stem()
                .ok_or_else(|| anyhow::anyhow!("Failed to get file stem for {}", path.display()))?
                .to_string_lossy();

            // Extract title from filename
            let title = title_case(&file_stem);

            docs.push(DocInfo {
                name: file_name.to_string(),
                title,
                path: format!("{}.html", file_stem),
            });
        }
    }

    // Sort docs by name, but keep README.md as first item
    let intro = docs.remove(0);
    docs.sort_by(|a, b| a.name.cmp(&b.name));
    docs.insert(0, intro);

    Ok(docs)
}

/// Get the path to a doc file
fn get_doc_path(gpui_dir: &Path, doc_name: &str) -> Result<PathBuf> {
    if doc_name == "README.md" {
        Ok(gpui_dir.join("README.md"))
    } else {
        Ok(gpui_dir.join("docs").join(doc_name))
    }
}

/// Generate CSS files
fn generate_css(output_dir: &Path) -> Result<()> {
    // Create the css directory if it doesn't exist
    let css_dir = output_dir.join("css");
    fs::create_dir_all(&css_dir)?;

    // Read the CSS file from our assets directory
    let css_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/css/styles.css");
    let css_content = fs::read_to_string(&css_path)
        .with_context(|| format!("Failed to read CSS file from {}", css_path.display()))?;

    // Write to the output directory
    fs::write(css_dir.join("styles.css"), css_content)?;

    Ok(())
}

/// Generate JS files
fn generate_js(output_dir: &Path) -> Result<()> {
    // Create the js directory if it doesn't exist
    let js_dir = output_dir.join("js");
    fs::create_dir_all(&js_dir)?;

    // Read the JS file from our assets directory
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/js/main.js");
    let js_content = fs::read_to_string(&js_path)
        .with_context(|| format!("Failed to read JS file from {}", js_path.display()))?;

    // Write to the output directory
    fs::write(js_dir.join("main.js"), js_content)?;

    Ok(())
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
