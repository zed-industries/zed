use std::{cell::RefCell, collections::HashMap, mem, ops::Range};

use gpui::{DefiniteLength, FontWeight, SharedString, TextAlign, px, relative};
use html5ever::{
    Attribute, LocalName, ParseOpts, local_name, parse_document, tendril::TendrilSink,
};
use markup5ever_rcdom::{Node, NodeData, RcDom};
use pulldown_cmark::{Alignment, HeadingLevel};
use stacksafe::stacksafe;

use crate::html::html_minifier::{Minifier, MinifierOptions};

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlBlock {
    pub source_range: Range<usize>,
    pub children: Vec<ParsedHtmlElement>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum ParsedHtmlElement {
    Heading(ParsedHtmlHeading),
    List(ParsedHtmlList),
    Table(ParsedHtmlTable),
    BlockQuote(ParsedHtmlBlockQuote),
    Paragraph(ParsedHtmlParagraph),
    Image(HtmlImage),
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlParagraph {
    pub text_align: Option<TextAlign>,
    pub contents: HtmlParagraph,
}

impl ParsedHtmlElement {
    pub fn source_range(&self) -> Option<Range<usize>> {
        Some(match self {
            Self::Heading(heading) => heading.source_range.clone(),
            Self::List(list) => list.source_range.clone(),
            Self::Table(table) => table.source_range.clone(),
            Self::BlockQuote(block_quote) => block_quote.source_range.clone(),
            Self::Paragraph(paragraph) => match paragraph.contents.first()? {
                HtmlParagraphChunk::Text(text) => text.source_range.clone(),
                HtmlParagraphChunk::Image(image) => image.source_range.clone(),
            },
            Self::Image(image) => image.source_range.clone(),
        })
    }
}

pub(crate) type HtmlParagraph = Vec<HtmlParagraphChunk>;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum HtmlParagraphChunk {
    Text(ParsedHtmlText),
    Image(HtmlImage),
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlList {
    pub source_range: Range<usize>,
    pub depth: u16,
    pub ordered: bool,
    pub items: Vec<ParsedHtmlListItem>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlListItem {
    pub source_range: Range<usize>,
    pub item_type: ParsedHtmlListItemType,
    pub content: Vec<ParsedHtmlElement>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum ParsedHtmlListItemType {
    Ordered(u64),
    Unordered,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlHeading {
    pub source_range: Range<usize>,
    pub level: HeadingLevel,
    pub contents: HtmlParagraph,
    pub text_align: Option<TextAlign>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlTable {
    pub source_range: Range<usize>,
    pub header: Vec<ParsedHtmlTableRow>,
    pub body: Vec<ParsedHtmlTableRow>,
    pub caption: Option<HtmlParagraph>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlTableColumn {
    pub col_span: usize,
    pub row_span: usize,
    pub is_header: bool,
    pub children: HtmlParagraph,
    pub alignment: Alignment,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlTableRow {
    pub columns: Vec<ParsedHtmlTableColumn>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlBlockQuote {
    pub source_range: Range<usize>,
    pub children: Vec<ParsedHtmlElement>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedHtmlText {
    pub source_range: Range<usize>,
    pub contents: SharedString,
    pub highlights: Vec<(Range<usize>, HtmlHighlightStyle)>,
    pub links: Vec<(Range<usize>, SharedString)>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct HtmlHighlightStyle {
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub weight: FontWeight,
    pub link: bool,
    pub oblique: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct HtmlImage {
    pub dest_url: SharedString,
    pub source_range: Range<usize>,
    pub alt_text: Option<SharedString>,
    pub width: Option<DefiniteLength>,
    pub height: Option<DefiniteLength>,
}

impl HtmlImage {
    fn new(dest_url: String, source_range: Range<usize>) -> Self {
        Self {
            dest_url: dest_url.into(),
            source_range,
            alt_text: None,
            width: None,
            height: None,
        }
    }

    fn set_alt_text(&mut self, alt_text: SharedString) {
        self.alt_text = Some(alt_text);
    }

    fn set_width(&mut self, width: DefiniteLength) {
        self.width = Some(width);
    }

    fn set_height(&mut self, height: DefiniteLength) {
        self.height = Some(height);
    }
}

#[derive(Debug)]
struct ParseHtmlNodeContext {
    list_item_depth: u16,
}

impl Default for ParseHtmlNodeContext {
    fn default() -> Self {
        Self { list_item_depth: 1 }
    }
}

pub(crate) fn parse_html_block(
    source: &str,
    source_range: Range<usize>,
) -> Option<ParsedHtmlBlock> {
    let bytes = cleanup_html(source);
    let mut cursor = std::io::Cursor::new(bytes);
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut cursor)
        .ok()?;

    let mut children = Vec::new();
    parse_html_node(
        source_range.clone(),
        &dom.document,
        &mut children,
        &ParseHtmlNodeContext::default(),
    );

    Some(ParsedHtmlBlock {
        source_range,
        children,
    })
}

fn cleanup_html(source: &str) -> Vec<u8> {
    let mut writer = std::io::Cursor::new(Vec::new());
    let mut reader = std::io::Cursor::new(source);
    let mut minify = Minifier::new(
        &mut writer,
        MinifierOptions {
            omit_doctype: true,
            collapse_whitespace: true,
            ..Default::default()
        },
    );
    if let Ok(()) = minify.minify(&mut reader) {
        writer.into_inner()
    } else {
        source.bytes().collect()
    }
}

#[stacksafe]
fn parse_html_node(
    source_range: Range<usize>,
    node: &Node,
    elements: &mut Vec<ParsedHtmlElement>,
    context: &ParseHtmlNodeContext,
) {
    match &node.data {
        NodeData::Document => {
            consume_children(source_range, node, elements, context);
        }
        NodeData::Text { contents } => {
            elements.push(ParsedHtmlElement::Paragraph(ParsedHtmlParagraph {
                text_align: None,
                contents: vec![HtmlParagraphChunk::Text(ParsedHtmlText {
                    source_range,
                    highlights: Vec::default(),
                    links: Vec::default(),
                    contents: contents.borrow().to_string().into(),
                })],
            }));
        }
        NodeData::Comment { .. } => {}
        NodeData::Element { name, attrs, .. } => {
            let styles_map = extract_styles_from_attributes(attrs);
            let text_align = text_align_from_attributes(attrs, &styles_map);
            let mut styles = if let Some(styles) = html_style_from_html_styles(styles_map) {
                vec![styles]
            } else {
                Vec::default()
            };

            if name.local == local_name!("img") {
                if let Some(image) = extract_image(source_range, attrs) {
                    elements.push(ParsedHtmlElement::Image(image));
                }
            } else if name.local == local_name!("p") {
                let mut paragraph = HtmlParagraph::new();
                parse_paragraph(
                    source_range,
                    node,
                    &mut paragraph,
                    &mut styles,
                    &mut Vec::new(),
                );

                if !paragraph.is_empty() {
                    elements.push(ParsedHtmlElement::Paragraph(ParsedHtmlParagraph {
                        text_align,
                        contents: paragraph,
                    }));
                }
            } else if matches!(
                name.local,
                local_name!("h1")
                    | local_name!("h2")
                    | local_name!("h3")
                    | local_name!("h4")
                    | local_name!("h5")
                    | local_name!("h6")
            ) {
                let mut paragraph = HtmlParagraph::new();
                consume_paragraph(
                    source_range.clone(),
                    node,
                    &mut paragraph,
                    &mut styles,
                    &mut Vec::new(),
                );

                if !paragraph.is_empty() {
                    elements.push(ParsedHtmlElement::Heading(ParsedHtmlHeading {
                        source_range,
                        level: match name.local {
                            local_name!("h1") => HeadingLevel::H1,
                            local_name!("h2") => HeadingLevel::H2,
                            local_name!("h3") => HeadingLevel::H3,
                            local_name!("h4") => HeadingLevel::H4,
                            local_name!("h5") => HeadingLevel::H5,
                            local_name!("h6") => HeadingLevel::H6,
                            _ => unreachable!(),
                        },
                        contents: paragraph,
                        text_align,
                    }));
                }
            } else if name.local == local_name!("ul") || name.local == local_name!("ol") {
                if let Some(list) = extract_html_list(
                    node,
                    name.local == local_name!("ol"),
                    context.list_item_depth,
                    source_range,
                ) {
                    elements.push(ParsedHtmlElement::List(list));
                }
            } else if name.local == local_name!("blockquote") {
                if let Some(blockquote) = extract_html_blockquote(node, source_range) {
                    elements.push(ParsedHtmlElement::BlockQuote(blockquote));
                }
            } else if name.local == local_name!("table") {
                if let Some(table) = extract_html_table(node, source_range) {
                    elements.push(ParsedHtmlElement::Table(table));
                }
            } else {
                consume_children(source_range, node, elements, context);
            }
        }
        _ => {}
    }
}

#[stacksafe]
fn parse_paragraph(
    source_range: Range<usize>,
    node: &Node,
    paragraph: &mut HtmlParagraph,
    highlights: &mut Vec<HtmlHighlightStyle>,
    links: &mut Vec<SharedString>,
) {
    fn items_with_range<T>(
        range: Range<usize>,
        items: impl IntoIterator<Item = T>,
    ) -> Vec<(Range<usize>, T)> {
        items
            .into_iter()
            .map(|item| (range.clone(), item))
            .collect()
    }

    match &node.data {
        NodeData::Text { contents } => {
            if let Some(text) =
                paragraph
                    .iter_mut()
                    .last()
                    .and_then(|paragraph_chunk| match paragraph_chunk {
                        HtmlParagraphChunk::Text(text) => Some(text),
                        _ => None,
                    })
            {
                let mut new_text = text.contents.to_string();
                new_text.push_str(&contents.borrow());

                text.highlights.extend(items_with_range(
                    text.contents.len()..new_text.len(),
                    mem::take(highlights),
                ));
                text.links.extend(items_with_range(
                    text.contents.len()..new_text.len(),
                    mem::take(links),
                ));
                text.contents = SharedString::from(new_text);
            } else {
                let contents = contents.borrow().to_string();
                paragraph.push(HtmlParagraphChunk::Text(ParsedHtmlText {
                    source_range,
                    highlights: items_with_range(0..contents.len(), mem::take(highlights)),
                    links: items_with_range(0..contents.len(), mem::take(links)),
                    contents: contents.into(),
                }));
            }
        }
        NodeData::Element { name, attrs, .. } => {
            if name.local == local_name!("img") {
                if let Some(image) = extract_image(source_range, attrs) {
                    paragraph.push(HtmlParagraphChunk::Image(image));
                }
            } else if name.local == local_name!("b") || name.local == local_name!("strong") {
                highlights.push(HtmlHighlightStyle {
                    weight: FontWeight::BOLD,
                    ..Default::default()
                });
                consume_paragraph(source_range, node, paragraph, highlights, links);
            } else if name.local == local_name!("i") {
                highlights.push(HtmlHighlightStyle {
                    italic: true,
                    ..Default::default()
                });
                consume_paragraph(source_range, node, paragraph, highlights, links);
            } else if name.local == local_name!("em") {
                highlights.push(HtmlHighlightStyle {
                    oblique: true,
                    ..Default::default()
                });
                consume_paragraph(source_range, node, paragraph, highlights, links);
            } else if name.local == local_name!("del") {
                highlights.push(HtmlHighlightStyle {
                    strikethrough: true,
                    ..Default::default()
                });
                consume_paragraph(source_range, node, paragraph, highlights, links);
            } else if name.local == local_name!("ins") {
                highlights.push(HtmlHighlightStyle {
                    underline: true,
                    ..Default::default()
                });
                consume_paragraph(source_range, node, paragraph, highlights, links);
            } else if name.local == local_name!("a") {
                if let Some(url) = attr_value(attrs, local_name!("href")) {
                    highlights.push(HtmlHighlightStyle {
                        link: true,
                        ..Default::default()
                    });
                    links.push(url.into());
                }
                consume_paragraph(source_range, node, paragraph, highlights, links);
            } else {
                consume_paragraph(source_range, node, paragraph, highlights, links);
            }
        }
        _ => {}
    }
}

fn consume_paragraph(
    source_range: Range<usize>,
    node: &Node,
    paragraph: &mut HtmlParagraph,
    highlights: &mut Vec<HtmlHighlightStyle>,
    links: &mut Vec<SharedString>,
) {
    for child in node.children.borrow().iter() {
        parse_paragraph(source_range.clone(), child, paragraph, highlights, links);
    }
}

fn parse_table_row(source_range: Range<usize>, node: &Node) -> Option<ParsedHtmlTableRow> {
    let mut columns = Vec::new();

    if let NodeData::Element { name, .. } = &node.data {
        if name.local != local_name!("tr") {
            return None;
        }

        for child in node.children.borrow().iter() {
            if let Some(column) = parse_table_column(source_range.clone(), child) {
                columns.push(column);
            }
        }
    }

    if columns.is_empty() {
        None
    } else {
        Some(ParsedHtmlTableRow { columns })
    }
}

fn parse_table_column(source_range: Range<usize>, node: &Node) -> Option<ParsedHtmlTableColumn> {
    match &node.data {
        NodeData::Element { name, attrs, .. } => {
            if !matches!(name.local, local_name!("th") | local_name!("td")) {
                return None;
            }

            let mut children = HtmlParagraph::new();
            consume_paragraph(
                source_range,
                node,
                &mut children,
                &mut Vec::new(),
                &mut Vec::new(),
            );

            let is_header = name.local == local_name!("th");

            Some(ParsedHtmlTableColumn {
                col_span: std::cmp::max(
                    attr_value(attrs, local_name!("colspan"))
                        .and_then(|span| span.parse().ok())
                        .unwrap_or(1),
                    1,
                ),
                row_span: std::cmp::max(
                    attr_value(attrs, local_name!("rowspan"))
                        .and_then(|span| span.parse().ok())
                        .unwrap_or(1),
                    1,
                ),
                is_header,
                children,
                alignment: attr_value(attrs, local_name!("align"))
                    .and_then(|align| match align.as_str() {
                        "left" => Some(Alignment::Left),
                        "center" => Some(Alignment::Center),
                        "right" => Some(Alignment::Right),
                        _ => None,
                    })
                    .unwrap_or(if is_header {
                        Alignment::Center
                    } else {
                        Alignment::None
                    }),
            })
        }
        _ => None,
    }
}

fn consume_children(
    source_range: Range<usize>,
    node: &Node,
    elements: &mut Vec<ParsedHtmlElement>,
    context: &ParseHtmlNodeContext,
) {
    for child in node.children.borrow().iter() {
        parse_html_node(source_range.clone(), child, elements, context);
    }
}

fn attr_value(attrs: &RefCell<Vec<Attribute>>, name: LocalName) -> Option<String> {
    attrs.borrow().iter().find_map(|attr| {
        if attr.name.local == name {
            Some(attr.value.to_string())
        } else {
            None
        }
    })
}

fn html_style_from_html_styles(styles: HashMap<String, String>) -> Option<HtmlHighlightStyle> {
    let mut html_style = HtmlHighlightStyle::default();

    if let Some(text_decoration) = styles.get("text-decoration") {
        match text_decoration.to_lowercase().as_str() {
            "underline" => {
                html_style.underline = true;
            }
            "line-through" => {
                html_style.strikethrough = true;
            }
            _ => {}
        }
    }

    if let Some(font_style) = styles.get("font-style") {
        match font_style.to_lowercase().as_str() {
            "italic" => {
                html_style.italic = true;
            }
            "oblique" => {
                html_style.oblique = true;
            }
            _ => {}
        }
    }

    if let Some(font_weight) = styles.get("font-weight") {
        match font_weight.to_lowercase().as_str() {
            "bold" => {
                html_style.weight = FontWeight::BOLD;
            }
            "lighter" => {
                html_style.weight = FontWeight::THIN;
            }
            _ => {
                if let Ok(weight) = font_weight.parse::<f32>() {
                    html_style.weight = FontWeight(weight);
                }
            }
        }
    }

    if html_style != HtmlHighlightStyle::default() {
        Some(html_style)
    } else {
        None
    }
}

fn parse_text_align(value: &str) -> Option<TextAlign> {
    match value.trim().to_ascii_lowercase().as_str() {
        "left" => Some(TextAlign::Left),
        "center" => Some(TextAlign::Center),
        "right" => Some(TextAlign::Right),
        _ => None,
    }
}

fn text_align_from_styles(styles: &HashMap<String, String>) -> Option<TextAlign> {
    styles
        .get("text-align")
        .and_then(|value| parse_text_align(value))
}

fn text_align_from_attributes(
    attrs: &RefCell<Vec<Attribute>>,
    styles: &HashMap<String, String>,
) -> Option<TextAlign> {
    text_align_from_styles(styles).or_else(|| {
        attr_value(attrs, local_name!("align")).and_then(|value| parse_text_align(&value))
    })
}

fn extract_styles_from_attributes(attrs: &RefCell<Vec<Attribute>>) -> HashMap<String, String> {
    let mut styles = HashMap::new();

    if let Some(style) = attr_value(attrs, local_name!("style")) {
        for declaration in style.split(';') {
            let mut parts = declaration.splitn(2, ':');
            if let Some((key, value)) = parts.next().zip(parts.next()) {
                styles.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }
    }

    styles
}

fn extract_image(source_range: Range<usize>, attrs: &RefCell<Vec<Attribute>>) -> Option<HtmlImage> {
    let src = attr_value(attrs, local_name!("src"))?;

    let mut image = HtmlImage::new(src, source_range);

    if let Some(alt) = attr_value(attrs, local_name!("alt")) {
        image.set_alt_text(alt.into());
    }

    let styles = extract_styles_from_attributes(attrs);

    if let Some(width) = attr_value(attrs, local_name!("width"))
        .or_else(|| styles.get("width").cloned())
        .and_then(|width| parse_html_element_dimension(&width))
    {
        image.set_width(width);
    }

    if let Some(height) = attr_value(attrs, local_name!("height"))
        .or_else(|| styles.get("height").cloned())
        .and_then(|height| parse_html_element_dimension(&height))
    {
        image.set_height(height);
    }

    Some(image)
}

fn extract_html_list(
    node: &Node,
    ordered: bool,
    depth: u16,
    source_range: Range<usize>,
) -> Option<ParsedHtmlList> {
    let mut items = Vec::with_capacity(node.children.borrow().len());

    for (index, child) in node.children.borrow().iter().enumerate() {
        if let NodeData::Element { name, .. } = &child.data {
            if name.local != local_name!("li") {
                continue;
            }

            let mut content = Vec::new();
            consume_children(
                source_range.clone(),
                child,
                &mut content,
                &ParseHtmlNodeContext {
                    list_item_depth: depth + 1,
                },
            );

            if !content.is_empty() {
                items.push(ParsedHtmlListItem {
                    source_range: source_range.clone(),
                    item_type: if ordered {
                        ParsedHtmlListItemType::Ordered(index as u64 + 1)
                    } else {
                        ParsedHtmlListItemType::Unordered
                    },
                    content,
                });
            }
        }
    }

    if items.is_empty() {
        None
    } else {
        Some(ParsedHtmlList {
            source_range,
            depth,
            ordered,
            items,
        })
    }
}

fn parse_html_element_dimension(value: &str) -> Option<DefiniteLength> {
    if value.ends_with('%') {
        value
            .trim_end_matches('%')
            .parse::<f32>()
            .ok()
            .map(|value| relative(value / 100.))
    } else {
        value
            .trim_end_matches("px")
            .parse()
            .ok()
            .map(|value| px(value).into())
    }
}

fn extract_html_blockquote(
    node: &Node,
    source_range: Range<usize>,
) -> Option<ParsedHtmlBlockQuote> {
    let mut children = Vec::new();
    consume_children(
        source_range.clone(),
        node,
        &mut children,
        &ParseHtmlNodeContext::default(),
    );

    if children.is_empty() {
        None
    } else {
        Some(ParsedHtmlBlockQuote {
            children,
            source_range,
        })
    }
}

fn extract_html_table(node: &Node, source_range: Range<usize>) -> Option<ParsedHtmlTable> {
    let mut header_rows = Vec::new();
    let mut body_rows = Vec::new();
    let mut caption = None;

    for child in node.children.borrow().iter() {
        if let NodeData::Element { name, .. } = &child.data {
            if name.local == local_name!("caption") {
                let mut paragraph = HtmlParagraph::new();
                parse_paragraph(
                    source_range.clone(),
                    child,
                    &mut paragraph,
                    &mut Vec::new(),
                    &mut Vec::new(),
                );
                caption = Some(paragraph);
            }

            if name.local == local_name!("thead") {
                for row in child.children.borrow().iter() {
                    if let Some(row) = parse_table_row(source_range.clone(), row) {
                        header_rows.push(row);
                    }
                }
            } else if name.local == local_name!("tbody") {
                for row in child.children.borrow().iter() {
                    if let Some(row) = parse_table_row(source_range.clone(), row) {
                        body_rows.push(row);
                    }
                }
            }
        }
    }

    if !header_rows.is_empty() || !body_rows.is_empty() {
        Some(ParsedHtmlTable {
            source_range,
            body: body_rows,
            header: header_rows,
            caption,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TextAlign;

    #[test]
    fn parses_html_styled_text() {
        let parsed = parse_html_block(
            "<p>Some text <strong>strong</strong> <a href=\"https://example.com\">link</a></p>",
            0..79,
        )
        .unwrap();

        assert_eq!(parsed.children.len(), 1);
        let ParsedHtmlElement::Paragraph(paragraph) = &parsed.children[0] else {
            panic!("expected paragraph");
        };
        let HtmlParagraphChunk::Text(text) = &paragraph.contents[0] else {
            panic!("expected text chunk");
        };

        assert_eq!(text.contents.as_ref(), "Some text strong link");
        assert_eq!(
            text.highlights,
            vec![
                (
                    10..16,
                    HtmlHighlightStyle {
                        weight: FontWeight::BOLD,
                        ..Default::default()
                    }
                ),
                (
                    17..21,
                    HtmlHighlightStyle {
                        link: true,
                        ..Default::default()
                    }
                )
            ]
        );
        assert_eq!(
            text.links,
            vec![(17..21, SharedString::from("https://example.com"))]
        );
    }

    #[test]
    fn parses_html_table_spans() {
        let parsed = parse_html_block(
            "<table><tbody><tr><td colspan=\"2\">a</td></tr><tr><td>b</td><td>c</td></tr></tbody></table>",
            0..91,
        )
        .unwrap();

        let ParsedHtmlElement::Table(table) = &parsed.children[0] else {
            panic!("expected table");
        };
        assert_eq!(table.body.len(), 2);
        assert_eq!(table.body[0].columns[0].col_span, 2);
        assert_eq!(table.body[1].columns.len(), 2);
    }

    #[test]
    fn parses_html_list_as_explicit_list_node() {
        let parsed = parse_html_block(
            "<ul><li>parent<ul><li>child</li></ul></li><li>sibling</li></ul>",
            0..64,
        )
        .unwrap();

        assert_eq!(parsed.children.len(), 1);

        let ParsedHtmlElement::List(list) = &parsed.children[0] else {
            panic!("expected list");
        };

        assert!(!list.ordered);
        assert_eq!(list.depth, 1);
        assert_eq!(list.items.len(), 2);

        let first_item = &list.items[0];
        let ParsedHtmlElement::Paragraph(paragraph) = &first_item.content[0] else {
            panic!("expected first item paragraph");
        };
        let HtmlParagraphChunk::Text(text) = &paragraph.contents[0] else {
            panic!("expected first item text");
        };
        assert_eq!(text.contents.as_ref(), "parent");

        let ParsedHtmlElement::List(nested_list) = &first_item.content[1] else {
            panic!("expected nested list");
        };
        assert_eq!(nested_list.depth, 2);
        assert_eq!(nested_list.items.len(), 1);

        let ParsedHtmlElement::Paragraph(nested_paragraph) = &nested_list.items[0].content[0]
        else {
            panic!("expected nested item paragraph");
        };
        let HtmlParagraphChunk::Text(nested_text) = &nested_paragraph.contents[0] else {
            panic!("expected nested item text");
        };
        assert_eq!(nested_text.contents.as_ref(), "child");

        let second_item = &list.items[1];
        let ParsedHtmlElement::Paragraph(second_paragraph) = &second_item.content[0] else {
            panic!("expected second item paragraph");
        };
        let HtmlParagraphChunk::Text(second_text) = &second_paragraph.contents[0] else {
            panic!("expected second item text");
        };
        assert_eq!(second_text.contents.as_ref(), "sibling");
    }

    #[test]
    fn parses_paragraph_text_align_from_style() {
        let parsed = parse_html_block("<p style=\"text-align: center\">x</p>", 0..40).unwrap();
        let ParsedHtmlElement::Paragraph(paragraph) = &parsed.children[0] else {
            panic!("expected paragraph");
        };
        assert_eq!(paragraph.text_align, Some(TextAlign::Center));
    }

    #[test]
    fn parses_heading_text_align_from_style() {
        let parsed = parse_html_block("<h2 style=\"text-align: right\">Title</h2>", 0..45).unwrap();
        let ParsedHtmlElement::Heading(heading) = &parsed.children[0] else {
            panic!("expected heading");
        };
        assert_eq!(heading.text_align, Some(TextAlign::Right));
    }

    #[test]
    fn parses_paragraph_text_align_from_align_attribute() {
        let parsed = parse_html_block("<p align=\"center\">x</p>", 0..24).unwrap();
        let ParsedHtmlElement::Paragraph(paragraph) = &parsed.children[0] else {
            panic!("expected paragraph");
        };
        assert_eq!(paragraph.text_align, Some(TextAlign::Center));
    }

    #[test]
    fn parses_heading_text_align_from_align_attribute() {
        let parsed = parse_html_block("<h2 align=\"right\">Title</h2>", 0..30).unwrap();
        let ParsedHtmlElement::Heading(heading) = &parsed.children[0] else {
            panic!("expected heading");
        };
        assert_eq!(heading.text_align, Some(TextAlign::Right));
    }

    #[test]
    fn prefers_style_text_align_over_align_attribute() {
        let parsed = parse_html_block(
            "<p align=\"left\" style=\"text-align: center\">x</p>",
            0..50,
        )
        .unwrap();
        let ParsedHtmlElement::Paragraph(paragraph) = &parsed.children[0] else {
            panic!("expected paragraph");
        };
        assert_eq!(paragraph.text_align, Some(TextAlign::Center));
    }
}
