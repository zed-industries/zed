use anyhow::Result;
use html_to_markdown::markdown::{
    CodeHandler, HeadingHandler, ListHandler, ParagraphHandler, StyledTextHandler, TableHandler,
    WebpageChromeRemover,
};
use html_to_markdown::{TagHandler, convert_html_to_markdown};
use std::cell::RefCell;
use std::rc::Rc;

/// Convert HTML to Markdown for rendering in the REPL.
pub fn html_to_markdown(html: &str) -> Result<String> {
    let mut handlers: Vec<TagHandler> = vec![
        // WebpageChromeRemover must come first to skip style, script, head, nav tags
        Rc::new(RefCell::new(WebpageChromeRemover)),
        Rc::new(RefCell::new(ParagraphHandler)),
        Rc::new(RefCell::new(HeadingHandler)),
        Rc::new(RefCell::new(ListHandler)),
        Rc::new(RefCell::new(TableHandler::new())),
        Rc::new(RefCell::new(StyledTextHandler)),
        Rc::new(RefCell::new(CodeHandler)),
    ];

    let markdown = convert_html_to_markdown(html.as_bytes(), &mut handlers)?;
    Ok(clean_markdown_tables(&markdown))
}

/// Clean up markdown table formatting and ensure tables have separator rows.
fn clean_markdown_tables(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut result: Vec<String> = Vec::new();
    let mut in_table = false;
    let mut has_separator = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with('|') {
            let normalized = normalize_table_row(trimmed);

            if !in_table {
                // Starting a new table
                in_table = true;
                has_separator = false;
            }

            // Check if this line is a separator row
            if trimmed.contains("---") {
                has_separator = true;
            }

            result.push(normalized.clone());

            // If this is the first row and no separator exists yet,
            // check if next row is a table row (not separator) and add one
            if !has_separator {
                let next_is_table_row = i + 1 < lines.len()
                    && lines[i + 1].trim().starts_with('|')
                    && !lines[i + 1].contains("---");

                if next_is_table_row {
                    // Insert separator after first row
                    let col_count = normalized.matches('|').count().saturating_sub(1);
                    if col_count > 0 {
                        let separator = (0..col_count)
                            .map(|_| "---")
                            .collect::<Vec<_>>()
                            .join(" | ");
                        result.push(format!("| {} |", separator));
                        has_separator = true;
                    }
                }
            }
        } else {
            // Not a table row
            if !trimmed.is_empty() {
                result.push(trimmed.to_string());
            }
            in_table = false;
            has_separator = false;
        }
    }

    result.join("\n")
}

/// Normalize a table row by trimming cells and ensuring consistent spacing.
fn normalize_table_row(row: &str) -> String {
    let parts: Vec<&str> = row.split('|').collect();
    let normalized: Vec<String> = parts.iter().map(|cell| cell.trim().to_string()).collect();
    normalized.join(" | ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_table_to_markdown() {
        let html = r#"<table>
            <thead><tr><th>A</th><th>B</th></tr></thead>
            <tbody><tr><td>1</td><td>x</td></tr></tbody>
        </table>"#;

        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("|"));
        assert!(md.contains("---"));
    }

    #[test]
    fn test_html_with_headings() {
        let html = "<h1>Title</h1><p>Content</p>";
        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("# Title"));
    }

    #[test]
    fn test_pandas_dataframe_html() {
        let html = r#"<table border="1" class="dataframe">
            <thead><tr><th></th><th>A</th><th>B</th></tr></thead>
            <tbody>
                <tr><th>0</th><td>1</td><td>x</td></tr>
                <tr><th>1</th><td>2</td><td>y</td></tr>
            </tbody>
        </table>"#;

        let md = html_to_markdown(html).unwrap();
        assert!(md.contains("|"));
        // Verify table rows are properly formatted (start with |)
        for line in md.lines() {
            if line.contains("|") {
                assert!(
                    line.starts_with("|"),
                    "Table line should start with |: {:?}",
                    line
                );
            }
        }
    }

    #[test]
    fn test_table_format_normalized() {
        let html = r#"<table>
  <thead>
    <tr><th>Name</th><th>Age</th></tr>
  </thead>
  <tbody>
    <tr><td>Alice</td><td>25</td></tr>
  </tbody>
</table>"#;

        let md = html_to_markdown(html).unwrap();

        // Should have clean table format
        assert!(md.contains("| Name | Age |"));
        assert!(md.contains("| --- | --- |"));
        assert!(md.contains("| Alice | 25 |"));
    }

    #[test]
    fn test_style_tags_are_filtered() {
        let html = r#"<style>
            .dataframe { border: 1px solid; }
        </style>
        <table>
            <thead><tr><th>A</th></tr></thead>
            <tbody><tr><td>1</td></tr></tbody>
        </table>"#;

        let md = html_to_markdown(html).unwrap();

        // Style content should not appear in output
        assert!(!md.contains("dataframe"));
        assert!(!md.contains("border"));
        // Table should still be present
        assert!(md.contains("| A |"));
    }

    #[test]
    fn test_table_without_thead() {
        // Tables without <thead> should still get a separator row
        let html = r#"<table>
            <tr><th>Feature</th><th>Supported</th></tr>
            <tr><td>Tables</td><td>✓</td></tr>
            <tr><td>Lists</td><td>✓</td></tr>
        </table>"#;

        let md = html_to_markdown(html).unwrap();

        // Should have separator row inserted after first row
        assert!(
            md.contains("| --- | --- |"),
            "Missing separator row: {}",
            md
        );
        assert!(md.contains("| Feature | Supported |"));
        assert!(md.contains("| Tables | ✓ |"));
    }
}
