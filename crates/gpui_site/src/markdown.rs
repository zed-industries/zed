use anyhow::Result;
use html_escape;
use pulldown_cmark::{html, Options, Parser};
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

/// Process markdown content to HTML with code highlighting
pub fn markdown_to_html(content: &str) -> Result<String> {
    // Setup parser with CommonMark options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);

    // Transform to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Process code blocks for syntax highlighting
    let html_with_highlighted_code = highlight_code_blocks(&html_output)?;

    Ok(html_with_highlighted_code)
}

/// Highlight code blocks in HTML
fn highlight_code_blocks(html: &str) -> Result<String> {
    // This is a simplified version - a real implementation would need to parse HTML
    // and replace code blocks with highlighted versions

    // Load syntax highlighting resources
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["base16-ocean.dark"];

    // For this simple example, we'll just look for Rust code blocks
    // A real implementation would need proper HTML parsing
    let mut result = html.to_string();

    // Replace code blocks with syntax highlighted versions
    // This is a very naive implementation for demonstration
    if let Some(start) = result.find("<code class=\"language-rust\">") {
        if let Some(end) = result[start..].find("</code>") {
            let code_start = start + "<code class=\"language-rust\">".len();
            let code_end = start + end;
            let code = &result[code_start..code_end];

            // Unescape HTML entities in the code
            let unescaped_code = html_escape::decode_html_entities(code);

            // Highlight the code
            let highlighted = highlighted_html_for_string(
                &unescaped_code,
                &syntax_set,
                syntax_set.find_syntax_by_extension("rs").unwrap(),
                theme,
            )?;

            // Replace the original code block with the highlighted version
            result.replace_range(code_start..code_end, &highlighted);
        }
    }

    Ok(result)
}

/// Read a markdown file and convert it to HTML
pub fn read_markdown_file(path: &Path) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    markdown_to_html(&content)
}
