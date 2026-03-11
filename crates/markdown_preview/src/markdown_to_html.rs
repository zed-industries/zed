// /tmp/zed-workers/markdown_to_html.rs
//
// HTML generator for a parsed markdown document, intended for clipboard operations.
//
// Notes:
// - This file is intentionally self-contained for easy vendoring.
// - If your codebase already defines `ParsedMarkdown`, `Block`, `Inline`, etc.,
//   delete the AST here and wire `MarkdownHtmlRenderer` to your existing types.
#![allow(dead_code)]

use std::fmt::Write;
use std::ops::Range;

/// A parsed markdown document.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsedMarkdown {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Paragraph(Vec<Inline>),
    Heading {
        level: u8,
        content: Vec<Inline>,
    },

    /// A list is rendered as `<ol>` when ordered, otherwise `<ul>`.
    List {
        ordered: bool,
        start: Option<u64>,
        items: Vec<ListItem>,
    },

    CodeBlock {
        language: Option<String>,
        code: String,
    },
    BlockQuote(Vec<Block>),
    Table {
        header: Vec<Vec<Inline>>,
        align: Vec<TableAlign>,
        rows: Vec<Vec<Vec<Inline>>>,
    },
    Image {
        alt: String,
        url: String,
        title: Option<String>,
    },
    HorizontalRule,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    /// `None` => normal bullet/ordered item. `Some(true/false)` => task item.
    pub task: Option<bool>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableAlign {
    Default,
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(String),
    Code(String),
    Emph(Vec<Inline>),
    Strong(Vec<Inline>),
    Link {
        text: Vec<Inline>,
        url: String,
        title: Option<String>,
    },
    SoftBreak,
    HardBreak,
}

pub struct MarkdownHtmlRenderer {
    /// Wrap output in a `.markdown-body` container and include inline CSS.
    pub include_css: bool,
}

impl Default for MarkdownHtmlRenderer {
    fn default() -> Self {
        Self { include_css: true }
    }
}

impl MarkdownHtmlRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render a complete HTML document for clipboard consumption.
    pub fn render(&self, doc: &ParsedMarkdown) -> String {
        let mut out = String::new();

        out.push_str("<!doctype html><html><head><meta charset=\"utf-8\">");
        if self.include_css {
            out.push_str("<style>");
            out.push_str(GITHUBISH_CSS);
            out.push_str("</style>");
        }
        out.push_str("</head><body>");
        out.push_str("<article class=\"markdown-body\">");

        for block in &doc.blocks {
            self.render_block(&mut out, block);
        }

        out.push_str("</article></body></html>");
        out
    }

    pub fn render_block(&self, out: &mut String, block: &Block) {
        match block {
            Block::Paragraph(inlines) => {
                out.push_str("<p>");
                self.render_inlines(out, inlines);
                out.push_str("</p>");
            }
            Block::Heading { level, content } => {
                let lvl = (*level).clamp(1, 6);
                let _ = write!(out, "<h{lvl}>");
                self.render_inlines(out, content);
                let _ = write!(out, "</h{lvl}>");
            }
            Block::List {
                ordered,
                start,
                items,
            } => {
                if *ordered {
                    if let Some(s) = start {
                        let _ = write!(out, "<ol start=\"{}\">", escape_attr(&s.to_string()));
                    } else {
                        out.push_str("<ol>");
                    }
                } else {
                    out.push_str("<ul>");
                }

                for item in items {
                    self.render_list_item(out, item);
                }

                if *ordered {
                    out.push_str("</ol>");
                } else {
                    out.push_str("</ul>");
                }
            }
            Block::CodeBlock { language, code } => {
                out.push_str("<pre><code");
                if let Some(lang) = language.as_ref().filter(|s| !s.trim().is_empty()) {
                    let _ = write!(out, " class=\"language-{}\"", escape_attr(lang.trim()));
                }
                out.push_str(">");
                out.push_str(&escape_html(code));
                out.push_str("</code></pre>");
            }
            Block::BlockQuote(children) => {
                out.push_str("<blockquote>");
                for b in children {
                    self.render_block(out, b);
                }
                out.push_str("</blockquote>");
            }
            Block::Table {
                header,
                align,
                rows,
            } => {
                out.push_str("<table><thead><tr>");
                for (i, cell) in header.iter().enumerate() {
                    out.push_str("<th");
                    if let Some(a) = align.get(i).copied() {
                        if let Some(css) = table_align_css(a) {
                            let _ = write!(out, " style=\"text-align:{}\"", css);
                        }
                    }
                    out.push_str(">");
                    self.render_inlines(out, cell);
                    out.push_str("</th>");
                }
                out.push_str("</tr></thead><tbody>");

                for row in rows {
                    out.push_str("<tr>");
                    for (i, cell) in row.iter().enumerate() {
                        out.push_str("<td");
                        if let Some(a) = align.get(i).copied() {
                            if let Some(css) = table_align_css(a) {
                                let _ = write!(out, " style=\"text-align:{}\"", css);
                            }
                        }
                        out.push_str(">");
                        self.render_inlines(out, cell);
                        out.push_str("</td>");
                    }
                    out.push_str("</tr>");
                }

                out.push_str("</tbody></table>");
            }
            Block::Image { alt, url, title } => {
                out.push_str("<p><img");
                let _ = write!(out, " alt=\"{}\"", escape_attr(alt));
                let _ = write!(out, " src=\"{}\"", escape_attr(url));
                if let Some(t) = title.as_ref().filter(|s| !s.is_empty()) {
                    let _ = write!(out, " title=\"{}\"", escape_attr(t));
                }
                out.push_str("></p>");
            }
            Block::HorizontalRule => out.push_str("<hr>"),
        }
    }

    fn render_list_item(&self, out: &mut String, item: &ListItem) {
        out.push_str("<li>");

        if let Some(checked) = item.task {
            // Matches GitHub’s task list markup well enough for clipboard use.
            out.push_str("<input type=\"checkbox\" disabled");
            if checked {
                out.push_str(" checked");
            }
            out.push_str("> ");
        }

        // If the item is a single paragraph, keep it compact; otherwise render blocks.
        match item.blocks.as_slice() {
            [Block::Paragraph(inlines)] => self.render_inlines(out, inlines),
            _ => {
                for b in &item.blocks {
                    self.render_block(out, b);
                }
            }
        }

        out.push_str("</li>");
    }

    fn render_inlines(&self, out: &mut String, inlines: &[Inline]) {
        for inline in inlines {
            match inline {
                Inline::Text(s) => out.push_str(&escape_html(s)),
                Inline::Code(s) => {
                    out.push_str("<code>");
                    out.push_str(&escape_html(s));
                    out.push_str("</code>");
                }
                Inline::Emph(children) => {
                    out.push_str("<em>");
                    self.render_inlines(out, children);
                    out.push_str("</em>");
                }
                Inline::Strong(children) => {
                    out.push_str("<strong>");
                    self.render_inlines(out, children);
                    out.push_str("</strong>");
                }
                Inline::Link { text, url, title } => {
                    out.push_str("<a");
                    let _ = write!(out, " href=\"{}\"", escape_attr(url));
                    if let Some(t) = title.as_ref().filter(|s| !s.is_empty()) {
                        let _ = write!(out, " title=\"{}\"", escape_attr(t));
                    }
                    out.push_str(">");
                    self.render_inlines(out, text);
                    out.push_str("</a>");
                }
                Inline::SoftBreak => out.push('\n'),
                Inline::HardBreak => out.push_str("<br>"),
            }
        }
    }
}

fn table_align_css(a: TableAlign) -> Option<&'static str> {
    match a {
        TableAlign::Default => None,
        TableAlign::Left => Some("left"),
        TableAlign::Center => Some("center"),
        TableAlign::Right => Some("right"),
    }
}

pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

pub fn escape_attr(input: &str) -> String {
    // Attribute escaping is identical for our use-case; kept separate for clarity.
    escape_html(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionExport {
    pub plain_text: String,
    pub html: String,
}

pub fn selectable_text_len(doc: &crate::markdown_elements::ParsedMarkdown) -> usize {
    doc.children.iter().map(selectable_block_len).sum()
}

pub fn selectable_text(doc: &crate::markdown_elements::ParsedMarkdown) -> String {
    let mut text = String::with_capacity(selectable_text_len(doc));
    append_selectable_text(&doc.children, &mut text);
    text
}

pub fn export_selection(
    doc: &crate::markdown_elements::ParsedMarkdown,
    selection: Range<usize>,
) -> Option<SelectionExport> {
    let selection = clamp_selection(selection, selectable_text_len(doc))?;
    let mut exporter = SelectionExporter::new(selection);
    let blocks = export_blocks(&doc.children, &mut exporter);
    if blocks.plain_text.is_empty() || blocks.html.is_empty() {
        return None;
    }

    Some(SelectionExport {
        plain_text: blocks.plain_text,
        html: wrap_html_document(&blocks.html),
    })
}

fn clamp_selection(selection: Range<usize>, max_len: usize) -> Option<Range<usize>> {
    let start = selection.start.min(max_len);
    let end = selection.end.min(max_len);
    (start < end).then_some(start..end)
}

fn wrap_html_document(body: &str) -> String {
    let mut out = String::new();
    out.push_str("<!doctype html><html><head><meta charset=\"utf-8\">");
    out.push_str("<style>");
    out.push_str(GITHUBISH_CSS);
    out.push_str("</style>");
    out.push_str("</head><body><article class=\"markdown-body\">");
    out.push_str(body);
    out.push_str("</article></body></html>");
    out
}

fn append_selectable_text(
    blocks: &[crate::markdown_elements::ParsedMarkdownElement],
    out: &mut String,
) {
    use crate::markdown_elements::{MarkdownParagraphChunk, ParsedMarkdownElement};

    for block in blocks {
        match block {
            ParsedMarkdownElement::Paragraph(chunks) => {
                for chunk in chunks {
                    if let MarkdownParagraphChunk::Text(text) = chunk {
                        out.push_str(text.contents.as_ref());
                    }
                }
            }
            ParsedMarkdownElement::Heading(heading) => {
                for chunk in &heading.contents {
                    if let MarkdownParagraphChunk::Text(text) = chunk {
                        out.push_str(text.contents.as_ref());
                    }
                }
            }
            ParsedMarkdownElement::ListItem(item) => append_selectable_text(&item.content, out),
            ParsedMarkdownElement::Table(table) => {
                if let Some(caption) = &table.caption {
                    for chunk in caption {
                        if let MarkdownParagraphChunk::Text(text) = chunk {
                            out.push_str(text.contents.as_ref());
                        }
                    }
                }
                for row in &table.header {
                    for column in &row.columns {
                        for chunk in &column.children {
                            if let MarkdownParagraphChunk::Text(text) = chunk {
                                out.push_str(text.contents.as_ref());
                            }
                        }
                    }
                }
                for row in &table.body {
                    for column in &row.columns {
                        for chunk in &column.children {
                            if let MarkdownParagraphChunk::Text(text) = chunk {
                                out.push_str(text.contents.as_ref());
                            }
                        }
                    }
                }
            }
            ParsedMarkdownElement::BlockQuote(block_quote) => {
                append_selectable_text(&block_quote.children, out)
            }
            ParsedMarkdownElement::CodeBlock(code_block) => {
                out.push_str(code_block.contents.as_ref())
            }
            ParsedMarkdownElement::HorizontalRule(_) | ParsedMarkdownElement::Image(_) => {}
        }
    }
}

fn selectable_block_len(block: &crate::markdown_elements::ParsedMarkdownElement) -> usize {
    use crate::markdown_elements::{MarkdownParagraphChunk, ParsedMarkdownElement};

    match block {
        ParsedMarkdownElement::Paragraph(chunks) => chunks
            .iter()
            .map(|chunk| match chunk {
                MarkdownParagraphChunk::Text(text) => text.contents.len(),
                MarkdownParagraphChunk::Image(_) => 0,
            })
            .sum(),
        ParsedMarkdownElement::Heading(heading) => heading
            .contents
            .iter()
            .map(|chunk| match chunk {
                MarkdownParagraphChunk::Text(text) => text.contents.len(),
                MarkdownParagraphChunk::Image(_) => 0,
            })
            .sum(),
        ParsedMarkdownElement::ListItem(item) => {
            item.content.iter().map(selectable_block_len).sum()
        }
        ParsedMarkdownElement::Table(table) => {
            let caption_len = table
                .caption
                .as_ref()
                .map(|caption| {
                    caption
                        .iter()
                        .map(|chunk| match chunk {
                            MarkdownParagraphChunk::Text(text) => text.contents.len(),
                            MarkdownParagraphChunk::Image(_) => 0,
                        })
                        .sum::<usize>()
                })
                .unwrap_or_default();
            let rows_len = table
                .header
                .iter()
                .chain(table.body.iter())
                .flat_map(|row| row.columns.iter())
                .flat_map(|column| column.children.iter())
                .map(|chunk| match chunk {
                    MarkdownParagraphChunk::Text(text) => text.contents.len(),
                    MarkdownParagraphChunk::Image(_) => 0,
                })
                .sum::<usize>();
            caption_len + rows_len
        }
        ParsedMarkdownElement::BlockQuote(block_quote) => {
            block_quote.children.iter().map(selectable_block_len).sum()
        }
        ParsedMarkdownElement::CodeBlock(code_block) => code_block.contents.len(),
        ParsedMarkdownElement::HorizontalRule(_) | ParsedMarkdownElement::Image(_) => 0,
    }
}

#[derive(Debug, Default)]
struct SelectionExporter {
    selection: Range<usize>,
    cursor: usize,
}

#[derive(Debug)]
struct BlockSelection {
    plain_text: String,
    html: String,
}

impl SelectionExporter {
    fn new(selection: Range<usize>) -> Self {
        Self {
            selection,
            cursor: 0,
        }
    }

    fn selected_range(&mut self, len: usize) -> Option<Range<usize>> {
        let start = self.cursor;
        let end = start + len;
        if self.selection.end <= start || self.selection.start >= end {
            self.cursor = end;
            return None;
        }

        let overlap = self.selection.start.max(start)..self.selection.end.min(end);
        self.cursor = end;
        Some((overlap.start - start)..(overlap.end - start))
    }
}

fn export_blocks(
    blocks: &[crate::markdown_elements::ParsedMarkdownElement],
    exporter: &mut SelectionExporter,
) -> BlockSelection {
    let mut plain_text = String::new();
    let mut html = String::new();
    let mut first = true;

    for block in blocks {
        let Some(selection) = export_block(block, exporter) else {
            continue;
        };

        if !first {
            plain_text.push('\n');
        }
        plain_text.push_str(&selection.plain_text);
        html.push_str(&selection.html);
        first = false;
    }

    BlockSelection { plain_text, html }
}

fn export_block(
    block: &crate::markdown_elements::ParsedMarkdownElement,
    exporter: &mut SelectionExporter,
) -> Option<BlockSelection> {
    use crate::markdown_elements::ParsedMarkdownElement;

    match block {
        ParsedMarkdownElement::Paragraph(chunks) => {
            let inline = export_paragraph(chunks, exporter)?;
            Some(BlockSelection {
                plain_text: inline.plain_text,
                html: format!("<p>{}</p>", inline.html),
            })
        }
        ParsedMarkdownElement::Heading(heading) => {
            let inline = export_paragraph(&heading.contents, exporter)?;
            let level = match heading.level {
                crate::markdown_elements::HeadingLevel::H1 => 1,
                crate::markdown_elements::HeadingLevel::H2 => 2,
                crate::markdown_elements::HeadingLevel::H3 => 3,
                crate::markdown_elements::HeadingLevel::H4 => 4,
                crate::markdown_elements::HeadingLevel::H5 => 5,
                crate::markdown_elements::HeadingLevel::H6 => 6,
            };
            Some(BlockSelection {
                plain_text: inline.plain_text,
                html: format!("<h{level}>{}</h{level}>", inline.html),
            })
        }
        ParsedMarkdownElement::ListItem(item) => {
            let selection = export_blocks(&item.content, exporter);
            if selection.html.is_empty() {
                return None;
            }
            Some(BlockSelection {
                plain_text: selection.plain_text,
                html: format!("<li>{}</li>", selection.html),
            })
        }
        ParsedMarkdownElement::Table(table) => export_table(table, exporter),
        ParsedMarkdownElement::BlockQuote(block_quote) => {
            let selection = export_blocks(&block_quote.children, exporter);
            if selection.html.is_empty() {
                return None;
            }
            Some(BlockSelection {
                plain_text: selection.plain_text,
                html: format!("<blockquote>{}</blockquote>", selection.html),
            })
        }
        ParsedMarkdownElement::CodeBlock(code_block) => {
            let selected = exporter.selected_range(code_block.contents.len())?;
            let plain_text = code_block.contents[selected.clone()].to_string();
            let mut html = String::new();
            html.push_str("<pre><code");
            if let Some(language) = code_block
                .language
                .as_ref()
                .filter(|language| !language.is_empty())
            {
                let _ = write!(html, " class=\"language-{}\"", escape_attr(language));
            }
            html.push('>');
            html.push_str(&escape_html(&plain_text));
            html.push_str("</code></pre>");
            Some(BlockSelection { plain_text, html })
        }
        ParsedMarkdownElement::HorizontalRule(_) | ParsedMarkdownElement::Image(_) => None,
    }
}

fn export_table(
    table: &crate::markdown_elements::ParsedMarkdownTable,
    exporter: &mut SelectionExporter,
) -> Option<BlockSelection> {
    let caption = table
        .caption
        .as_ref()
        .and_then(|caption| export_paragraph(caption, exporter));

    let header_rows = table
        .header
        .iter()
        .filter_map(|row| export_table_row(row, exporter))
        .collect::<Vec<_>>();
    let body_rows = table
        .body
        .iter()
        .filter_map(|row| export_table_row(row, exporter))
        .collect::<Vec<_>>();

    if caption.is_none() && header_rows.is_empty() && body_rows.is_empty() {
        return None;
    }

    let mut plain_parts = Vec::new();
    let mut html = String::new();

    if let Some(caption) = caption {
        plain_parts.push(caption.plain_text);
        html.push_str("<p>");
        html.push_str(&caption.html);
        html.push_str("</p>");
    }

    html.push_str("<table>");
    if !header_rows.is_empty() {
        html.push_str("<thead>");
        for row in &header_rows {
            plain_parts.push(row.plain_text.clone());
            html.push_str(&row.html);
        }
        html.push_str("</thead>");
    }
    if !body_rows.is_empty() {
        html.push_str("<tbody>");
        for row in &body_rows {
            plain_parts.push(row.plain_text.clone());
            html.push_str(&row.html);
        }
        html.push_str("</tbody>");
    }
    html.push_str("</table>");

    Some(BlockSelection {
        plain_text: plain_parts.join("\n"),
        html,
    })
}

fn export_table_row(
    row: &crate::markdown_elements::ParsedMarkdownTableRow,
    exporter: &mut SelectionExporter,
) -> Option<BlockSelection> {
    let mut plain_cells = Vec::new();
    let mut html = String::new();
    let mut has_selected_cell = false;

    html.push_str("<tr>");
    for column in &row.columns {
        let exported = export_paragraph(&column.children, exporter);
        if exported.is_some() {
            has_selected_cell = true;
        }

        let tag = if column.is_header { "th" } else { "td" };
        html.push('<');
        html.push_str(tag);
        if let Some(alignment) = table_alignment_css(column.alignment) {
            let _ = write!(html, " style=\"text-align:{}\"", alignment);
        }
        html.push('>');
        if let Some(exported) = exported {
            plain_cells.push(exported.plain_text);
            html.push_str(&exported.html);
        }
        html.push_str("</");
        html.push_str(tag);
        html.push('>');
    }
    html.push_str("</tr>");

    has_selected_cell.then_some(BlockSelection {
        plain_text: plain_cells.join("\t"),
        html,
    })
}

fn export_paragraph(
    chunks: &[crate::markdown_elements::MarkdownParagraphChunk],
    exporter: &mut SelectionExporter,
) -> Option<BlockSelection> {
    use crate::markdown_elements::MarkdownParagraphChunk;

    let mut plain_text = String::new();
    let mut html = String::new();

    for chunk in chunks {
        match chunk {
            MarkdownParagraphChunk::Text(text) => {
                let Some(selected) = exporter.selected_range(text.contents.len()) else {
                    continue;
                };
                plain_text.push_str(&text.contents[selected.clone()]);
                html.push_str(&render_selected_text(text, selected));
            }
            MarkdownParagraphChunk::Image(_) => {}
        }
    }

    (!plain_text.is_empty()).then_some(BlockSelection { plain_text, html })
}

fn render_selected_text(
    text: &crate::markdown_elements::ParsedMarkdownText,
    selection: Range<usize>,
) -> String {
    let mut boundaries = vec![selection.start, selection.end];

    for (range, _) in &text.highlights {
        if let Some(overlap) = intersect(&selection, range) {
            boundaries.push(overlap.start);
            boundaries.push(overlap.end);
        }
    }

    for (range, _) in &text.regions {
        if let Some(overlap) = intersect(&selection, range) {
            boundaries.push(overlap.start);
            boundaries.push(overlap.end);
        }
    }

    boundaries.sort_unstable();
    boundaries.dedup();

    let mut html = String::new();
    for window in boundaries.windows(2) {
        let range = window[0]..window[1];
        if range.is_empty() {
            continue;
        }

        let style = segment_style(text, &range);
        let mut segment = escape_html(&text.contents[range.clone()]);

        if style.code {
            segment = format!("<code>{}</code>", segment);
        }
        if style.strikethrough {
            segment = format!("<del>{}</del>", segment);
        }
        if style.underline {
            segment = format!("<ins>{}</ins>", segment);
        }
        if style.italic {
            segment = format!("<em>{}</em>", segment);
        }
        if style.bold {
            segment = format!("<strong>{}</strong>", segment);
        }
        if let Some(href) = style.link_href {
            segment = format!("<a href=\"{}\">{}</a>", escape_attr(&href), segment);
        }

        html.push_str(&segment);
    }

    html
}

#[derive(Default)]
struct SegmentStyle {
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    code: bool,
    link_href: Option<String>,
}

fn segment_style(
    text: &crate::markdown_elements::ParsedMarkdownText,
    range: &Range<usize>,
) -> SegmentStyle {
    let mut style = SegmentStyle::default();

    for (highlight_range, highlight) in &text.highlights {
        if !highlight_range.contains(&range.start) {
            continue;
        }

        match highlight {
            crate::markdown_elements::MarkdownHighlight::Style(markdown_style) => {
                style.bold |= markdown_style.weight != gpui::FontWeight::default();
                style.italic |= markdown_style.italic || markdown_style.oblique;
                style.underline |= markdown_style.underline;
                style.strikethrough |= markdown_style.strikethrough;
            }
            crate::markdown_elements::MarkdownHighlight::Code(_) => {
                style.code = true;
            }
        }
    }

    for (region_range, region) in &text.regions {
        if !region_range.contains(&range.start) {
            continue;
        }

        style.code |= region.code;
        if style.link_href.is_none() {
            style.link_href = region.link.as_ref().map(link_href);
        }
    }

    style
}

fn link_href(link: &crate::markdown_elements::Link) -> String {
    match link {
        crate::markdown_elements::Link::Web { url } => url.clone(),
        crate::markdown_elements::Link::Path { display_path, .. } => {
            display_path.to_string_lossy().to_string()
        }
    }
}

fn intersect(left: &Range<usize>, right: &Range<usize>) -> Option<Range<usize>> {
    let start = left.start.max(right.start);
    let end = left.end.min(right.end);
    (start < end).then_some(start..end)
}

fn table_alignment_css(
    alignment: crate::markdown_elements::ParsedMarkdownTableAlignment,
) -> Option<&'static str> {
    match alignment {
        crate::markdown_elements::ParsedMarkdownTableAlignment::None => None,
        crate::markdown_elements::ParsedMarkdownTableAlignment::Left => Some("left"),
        crate::markdown_elements::ParsedMarkdownTableAlignment::Center => Some("center"),
        crate::markdown_elements::ParsedMarkdownTableAlignment::Right => Some("right"),
    }
}

// Minimal GitHub-flavored markdown look, designed for paste targets.
// Not a verbatim copy of GitHub’s stylesheet.
const GITHUBISH_CSS: &str = r#"
  :root {
    --fg: #1f2328;
    --muted: #59636e;
    --border: #d0d7de;
    --bg: #ffffff;
    --code-bg: #f6f8fa;
    --link: #0969da;
  }
  body { margin: 0; background: var(--bg); color: var(--fg); }
  .markdown-body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 1.5;
    padding: 16px;
    word-wrap: break-word;
  }
  .markdown-body p { margin: 0 0 16px; }
  .markdown-body h1, .markdown-body h2, .markdown-body h3,
  .markdown-body h4, .markdown-body h5, .markdown-body h6 {
    margin: 24px 0 16px;
    font-weight: 600;
    line-height: 1.25;
  }
  .markdown-body h1 { font-size: 2em; padding-bottom: 0.3em; border-bottom: 1px solid var(--border); }
  .markdown-body h2 { font-size: 1.5em; padding-bottom: 0.3em; border-bottom: 1px solid var(--border); }
  .markdown-body h3 { font-size: 1.25em; }
  .markdown-body h4 { font-size: 1em; }
  .markdown-body h5 { font-size: 0.875em; }
  .markdown-body h6 { font-size: 0.85em; color: var(--muted); }
  .markdown-body a { color: var(--link); text-decoration: none; }
  .markdown-body a:hover { text-decoration: underline; }
  .markdown-body ul, .markdown-body ol { margin: 0 0 16px; padding-left: 2em; }
  .markdown-body li { margin: 0.25em 0; }
  .markdown-body code {
    font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, "Liberation Mono", monospace;
    font-size: 0.95em;
    background: var(--code-bg);
    padding: 0.15em 0.35em;
    border-radius: 6px;
  }
  .markdown-body pre {
    margin: 0 0 16px;
    padding: 16px;
    overflow: auto;
    background: var(--code-bg);
    border-radius: 6px;
  }
  .markdown-body pre code { background: transparent; padding: 0; }
  .markdown-body blockquote {
    margin: 0 0 16px;
    padding: 0 1em;
    color: var(--muted);
    border-left: 0.25em solid var(--border);
  }
  .markdown-body hr {
    height: 0.25em;
    padding: 0;
    margin: 24px 0;
    background-color: var(--border);
    border: 0;
  }
  .markdown-body table {
    border-spacing: 0;
    border-collapse: collapse;
    margin: 0 0 16px;
    width: 100%;
  }
  .markdown-body th, .markdown-body td {
    border: 1px solid var(--border);
    padding: 6px 13px;
  }
  .markdown-body th { font-weight: 600; background: #f6f8fa; }
  .markdown-body img { max-width: 100%; }
"#;

// Conversion from markdown_elements types to markdown_to_html types
impl From<&crate::markdown_elements::ParsedMarkdown> for ParsedMarkdown {
    fn from(doc: &crate::markdown_elements::ParsedMarkdown) -> Self {
        Self {
            blocks: doc.children.iter().map(|child| child.into()).collect(),
        }
    }
}

impl From<&crate::markdown_elements::ParsedMarkdownElement> for Block {
    fn from(elem: &crate::markdown_elements::ParsedMarkdownElement) -> Self {
        use crate::markdown_elements::ParsedMarkdownElement;
        match elem {
            ParsedMarkdownElement::Paragraph(chunks) => {
                let inlines: Vec<Inline> = chunks.iter().map(|c| c.into()).collect();
                Block::Paragraph(inlines)
            }
            _ => Block::Paragraph(vec![]),
        }
    }
}

impl From<&crate::markdown_elements::MarkdownParagraphChunk> for Inline {
    fn from(chunk: &crate::markdown_elements::MarkdownParagraphChunk) -> Self {
        use crate::markdown_elements::MarkdownParagraphChunk;
        match chunk {
            MarkdownParagraphChunk::Text(text) => Inline::Text(text.contents.to_string()),
            MarkdownParagraphChunk::Image(image) => {
                let url = match &image.link {
                    crate::markdown_elements::Link::Web { url } => url.clone(),
                    crate::markdown_elements::Link::Path { display_path, .. } => {
                        display_path.to_string_lossy().to_string()
                    }
                };
                Inline::Text(format!("[Image: {}]", url))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown_parser::parse_markdown;

    async fn parse(input: &str) -> crate::markdown_elements::ParsedMarkdown {
        parse_markdown(input, None, None).await
    }

    #[gpui::test]
    async fn exports_partial_bold_selection_as_rich_html() {
        let parsed = parse("Hello **world**").await;

        let selection = export_selection(&parsed, 6..11).expect("selection export");

        assert_eq!(selection.plain_text, "world");
        assert!(selection.html.contains("<strong>world</strong>"));
        assert!(!selection.html.contains("Hello "));
    }

    #[gpui::test]
    async fn exports_selection_across_inline_styles() {
        let parsed = parse("Hello **world** and `code`").await;

        let selection = export_selection(&parsed, 6..20).expect("selection export");

        assert_eq!(selection.plain_text, "world and code");
        assert!(selection.html.contains("<strong>world</strong>"));
        assert!(selection.html.contains("<code>code</code>"));
        assert!(!selection.html.contains("Hello "));
    }
}
