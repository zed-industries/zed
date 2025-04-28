use crate::examples::{collect_examples, read_example_file};
use crate::markdown::read_markdown_file;
use crate::templates::{DocInfo, SiteContent, TemplateEngine, create_template_stubs};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Generate the complete gpui site
pub fn generate_site(gpui_dir: &Path, output_dir: &Path) -> Result<()> {
    // Create necessary directories
    fs::create_dir_all(output_dir.join("examples"))?;
    fs::create_dir_all(output_dir.join("docs"))?;
    fs::create_dir_all(output_dir.join("css"))?;
    fs::create_dir_all(output_dir.join("js"))?;

    // Create template stubs
    create_template_stubs(output_dir)?;

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
    let css_content = r#"/* Base styles */
:root {
    --bg-color: #ffffff;
    --text-color: #333333;
    --primary-color: #3060b8;
    --secondary-color: #5a80d0;
    --accent-color: #ff6b6b;
    --border-color: #e0e0e0;
    --code-bg: #f5f5f5;
    --header-bg: #1a1a2e;
    --header-text: #ffffff;
}

@media (prefers-color-scheme: dark) {
    :root {
        --bg-color: #1a1a2e;
        --text-color: #e0e0e0;
        --primary-color: #5a80d0;
        --secondary-color: #3060b8;
        --accent-color: #ff6b6b;
        --border-color: #444;
        --code-bg: #282c34;
        --header-bg: #0f0f1a;
        --header-text: #ffffff;
    }
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background: var(--bg-color);
}

.container {
    width: 100%;
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 20px;
}

/* Header */
header {
    background: var(--header-bg);
    color: var(--header-text);
    padding: 1rem 0;
}

header .container {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.logo {
    font-size: 1.5rem;
    font-weight: bold;
    color: var(--header-text);
    text-decoration: none;
}

nav ul {
    display: flex;
    list-style: none;
}

nav ul li {
    margin-left: 1.5rem;
    position: relative;
}

nav ul li a, nav ul li span {
    color: var(--header-text);
    text-decoration: none;
    font-weight: 500;
    cursor: pointer;
}

nav ul li a:hover {
    text-decoration: underline;
}

nav ul li ul {
    display: none;
    position: absolute;
    background: var(--header-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 0.5rem 0;
    min-width: 150px;
    flex-direction: column;
    z-index: 100;
}

nav ul li:hover ul {
    display: flex;
}

nav ul li ul li {
    margin: 0;
    padding: 0.5rem 1rem;
}

/* Hero section */
.hero {
    text-align: center;
    padding: 4rem 0;
}

.hero h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
}

.tagline {
    font-size: 1.5rem;
    color: var(--secondary-color);
    margin-bottom: 2rem;
}

.cta-buttons {
    display: flex;
    justify-content: center;
    gap: 1rem;
}

.button {
    display: inline-block;
    padding: 0.8rem 1.5rem;
    border-radius: 4px;
    font-weight: 500;
    text-decoration: none;
    transition: all 0.3s ease;
}

.button.primary {
    background: var(--primary-color);
    color: white;
}

.button.secondary {
    background: transparent;
    color: var(--primary-color);
    border: 1px solid var(--primary-color);
}

.button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}

/* Content section */
.content {
    padding: 3rem 0;
}

.content h2 {
    font-size: 2rem;
    margin: 2rem 0 1rem;
}

.content p {
    margin-bottom: 1.5rem;
}

.content code {
    background: var(--code-bg);
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
}

.content pre {
    background: var(--code-bg);
    padding: 1rem;
    border-radius: 5px;
    overflow-x: auto;
    margin: 1.5rem 0;
}

.content pre code {
    padding: 0;
    background: transparent;
}

/* Examples grid */
.examples-grid {
    padding: 3rem 0;
}

.examples-grid h2 {
    font-size: 2rem;
    margin-bottom: 2rem;
    text-align: center;
}

.grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 2rem;
}

.example-card {
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    text-decoration: none;
    color: var(--text-color);
    transition: all 0.3s ease;
}

.example-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 16px rgba(0,0,0,0.1);
}

.example-card h3 {
    color: var(--primary-color);
    margin-bottom: 0.5rem;
}

/* Example page */
.example {
    padding: 2rem 0;
}

.example h1 {
    margin-bottom: 1rem;
}

.code-container {
    margin: 2rem 0;
    border-radius: 8px;
    overflow: hidden;
}

.example-info {
    background: var(--code-bg);
    padding: 1.5rem;
    border-radius: 8px;
    margin-top: 2rem;
}

/* Documentation page */
.documentation {
    padding: 2rem 0;
}

.documentation h1 {
    margin-bottom: 2rem;
}

.documentation .content h2 {
    margin-top: 3rem;
}

.documentation .content h3 {
    margin-top: 2rem;
    margin-bottom: 1rem;
}

.documentation .content ul,
.documentation .content ol {
    margin-left: 2rem;
    margin-bottom: 1.5rem;
}

/* Footer */
footer {
    background: var(--header-bg);
    color: var(--header-text);
    padding: 2rem 0;
    margin-top: 3rem;
    text-align: center;
}

footer a {
    color: var(--primary-color);
}

/* Responsive */
@media (max-width: 768px) {
    header .container {
        flex-direction: column;
        gap: 1rem;
    }

    nav ul {
        flex-wrap: wrap;
        justify-content: center;
    }

    nav ul li {
        margin: 0.5rem;
    }

    .hero h1 {
        font-size: 2.5rem;
    }

    .tagline {
        font-size: 1.2rem;
    }

    .grid {
        grid-template-columns: 1fr;
    }
}"#;

    std::fs::write(output_dir.join("css").join("styles.css"), css_content)?;

    Ok(())
}

/// Generate JS files
fn generate_js(output_dir: &Path) -> Result<()> {
    // Try to read from our assets directory first
    let js_content = std::fs::read_to_string("assets/js/main.js")
        .unwrap_or_else(|_| {
            // Fallback content if file can't be read
            r#"// Basic JavaScript functionality for the gpui site

document.addEventListener('DOMContentLoaded', function() {
    // Add clipboard functionality to code blocks
    document.querySelectorAll('pre code').forEach((block) => {
        const copyButton = document.createElement('button');
        copyButton.className = 'copy-button';
        copyButton.textContent = 'Copy';
        
        const pre = block.parentNode;
        pre.style.position = 'relative';
        pre.appendChild(copyButton);
        
        copyButton.addEventListener('click', () => {
            navigator.clipboard.writeText(block.textContent).then(() => {
                copyButton.textContent = 'Copied!';
                setTimeout(() => {
                    copyButton.textContent = 'Copy';
                }, 2000);
            });
        });
    });
});
"#.to_string()
        });
    
    std::fs::write(output_dir.join("js").join("main.js"), js_content)?;
    
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
