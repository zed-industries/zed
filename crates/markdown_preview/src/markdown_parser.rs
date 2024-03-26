use crate::markdown_elements::*;
use async_recursion::async_recursion;
use gpui::FontWeight;
use language::LanguageRegistry;
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag, TagEnd};
use std::{ops::Range, path::PathBuf, sync::Arc};

pub async fn parse_markdown(
    markdown_input: &str,
    file_location_directory: Option<PathBuf>,
    language_registry: Option<Arc<LanguageRegistry>>,
) -> ParsedMarkdown {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options);
    let parser = MarkdownParser::new(
        parser.into_offset_iter().collect(),
        file_location_directory,
        language_registry,
    );
    let renderer = parser.parse_document().await;
    ParsedMarkdown {
        children: renderer.parsed,
    }
}

struct MarkdownParser<'a> {
    tokens: Vec<(Event<'a>, Range<usize>)>,
    /// The current index in the tokens array
    cursor: usize,
    /// The blocks that we have successfully parsed so far
    parsed: Vec<ParsedMarkdownElement>,
    file_location_directory: Option<PathBuf>,
    language_registry: Option<Arc<LanguageRegistry>>,
}

impl<'a> MarkdownParser<'a> {
    fn new(
        tokens: Vec<(Event<'a>, Range<usize>)>,
        file_location_directory: Option<PathBuf>,
        language_registry: Option<Arc<LanguageRegistry>>,
    ) -> Self {
        Self {
            tokens,
            file_location_directory,
            language_registry,
            cursor: 0,
            parsed: vec![],
        }
    }

    fn eof(&self) -> bool {
        if self.tokens.is_empty() {
            return true;
        }
        self.cursor >= self.tokens.len() - 1
    }

    fn peek(&self, steps: usize) -> Option<&(Event, Range<usize>)> {
        if self.eof() || (steps + self.cursor) >= self.tokens.len() {
            return self.tokens.last();
        }
        return self.tokens.get(self.cursor + steps);
    }

    fn previous(&self) -> Option<&(Event, Range<usize>)> {
        if self.cursor == 0 || self.cursor > self.tokens.len() {
            return None;
        }
        return self.tokens.get(self.cursor - 1);
    }

    fn current(&self) -> Option<&(Event, Range<usize>)> {
        return self.peek(0);
    }

    fn current_event(&self) -> Option<&Event> {
        return self.current().map(|(event, _)| event);
    }

    fn is_text_like(event: &Event) -> bool {
        match event {
            Event::Text(_)
            // Represent an inline code block
            | Event::Code(_)
            | Event::Html(_)
            | Event::FootnoteReference(_)
            | Event::Start(Tag::Link { link_type: _, dest_url: _, title: _, id: _ })
            | Event::Start(Tag::Emphasis)
            | Event::Start(Tag::Strong)
            | Event::Start(Tag::Strikethrough)
            | Event::Start(Tag::Image { link_type: _, dest_url: _, title: _, id: _ }) => {
                return true;
            }
            _ => return false,
        }
    }

    async fn parse_document(mut self) -> Self {
        while !self.eof() {
            if let Some(block) = self.parse_block().await {
                self.parsed.push(block);
            }
        }
        self
    }

    async fn parse_block(&mut self) -> Option<ParsedMarkdownElement> {
        let (current, source_range) = self.current().unwrap();
        match current {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    self.cursor += 1;
                    let text = self.parse_text(false);
                    Some(ParsedMarkdownElement::Paragraph(text))
                }
                Tag::Heading {
                    level,
                    id: _,
                    classes: _,
                    attrs: _,
                } => {
                    let level = *level;
                    self.cursor += 1;
                    let heading = self.parse_heading(level);
                    Some(ParsedMarkdownElement::Heading(heading))
                }
                Tag::Table(alignment) => {
                    let alignment = alignment.clone();
                    self.cursor += 1;
                    let table = self.parse_table(alignment);
                    Some(ParsedMarkdownElement::Table(table))
                }
                Tag::List(order) => {
                    let order = *order;
                    self.cursor += 1;
                    let list = self.parse_list(1, order).await;
                    Some(ParsedMarkdownElement::List(list))
                }
                Tag::BlockQuote => {
                    self.cursor += 1;
                    let block_quote = self.parse_block_quote().await;
                    Some(ParsedMarkdownElement::BlockQuote(block_quote))
                }
                Tag::CodeBlock(kind) => {
                    let language = match kind {
                        pulldown_cmark::CodeBlockKind::Indented => None,
                        pulldown_cmark::CodeBlockKind::Fenced(language) => {
                            if language.is_empty() {
                                None
                            } else {
                                Some(language.to_string())
                            }
                        }
                    };

                    self.cursor += 1;

                    let code_block = self.parse_code_block(language).await;
                    Some(ParsedMarkdownElement::CodeBlock(code_block))
                }
                _ => {
                    self.cursor += 1;
                    None
                }
            },
            Event::Rule => {
                let source_range = source_range.clone();
                self.cursor += 1;
                Some(ParsedMarkdownElement::HorizontalRule(source_range))
            }
            _ => {
                self.cursor += 1;
                None
            }
        }
    }

    fn parse_text(&mut self, should_complete_on_soft_break: bool) -> ParsedMarkdownText {
        let (_current, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();

        let mut text = String::new();
        let mut bold_depth = 0;
        let mut italic_depth = 0;
        let mut strikethrough_depth = 0;
        let mut link: Option<Link> = None;
        let mut region_ranges: Vec<Range<usize>> = vec![];
        let mut regions: Vec<ParsedRegion> = vec![];
        let mut highlights: Vec<(Range<usize>, MarkdownHighlight)> = vec![];

        loop {
            if self.eof() {
                break;
            }

            let (current, _source_range) = self.current().unwrap();
            let prev_len = text.len();
            match current {
                Event::SoftBreak => {
                    if should_complete_on_soft_break {
                        break;
                    }

                    // `Some text\nSome more text` should be treated as a single line.
                    text.push(' ');
                }

                Event::HardBreak => {
                    text.push('\n');
                }

                Event::Text(t) => {
                    text.push_str(t.as_ref());

                    let mut style = MarkdownHighlightStyle::default();

                    if bold_depth > 0 {
                        style.weight = FontWeight::BOLD;
                    }

                    if italic_depth > 0 {
                        style.italic = true;
                    }

                    if strikethrough_depth > 0 {
                        style.strikethrough = true;
                    }

                    if let Some(link) = link.clone() {
                        region_ranges.push(prev_len..text.len());
                        regions.push(ParsedRegion {
                            code: false,
                            link: Some(link),
                        });
                        style.underline = true;
                    }

                    if style != MarkdownHighlightStyle::default() {
                        let mut new_highlight = true;
                        if let Some((last_range, MarkdownHighlight::Style(last_style))) =
                            highlights.last_mut()
                        {
                            if last_range.end == prev_len && last_style == &style {
                                last_range.end = text.len();
                                new_highlight = false;
                            }
                        }
                        if new_highlight {
                            let range = prev_len..text.len();
                            highlights.push((range, MarkdownHighlight::Style(style)));
                        }
                    }
                }

                // Note: This event means "inline code" and not "code block"
                Event::Code(t) => {
                    text.push_str(t.as_ref());
                    region_ranges.push(prev_len..text.len());

                    if link.is_some() {
                        highlights.push((
                            prev_len..text.len(),
                            MarkdownHighlight::Style(MarkdownHighlightStyle {
                                underline: true,
                                ..Default::default()
                            }),
                        ));
                    }

                    regions.push(ParsedRegion {
                        code: true,
                        link: link.clone(),
                    });
                }

                Event::Start(tag) => match tag {
                    Tag::Emphasis => italic_depth += 1,
                    Tag::Strong => bold_depth += 1,
                    Tag::Strikethrough => strikethrough_depth += 1,
                    Tag::Link {
                        link_type: _,
                        dest_url,
                        title: _,
                        id: _,
                    } => {
                        link = Link::identify(
                            self.file_location_directory.clone(),
                            dest_url.to_string(),
                        );
                    }
                    _ => {
                        break;
                    }
                },

                Event::End(tag) => match tag {
                    TagEnd::Emphasis => {
                        italic_depth -= 1;
                    }
                    TagEnd::Strong => {
                        bold_depth -= 1;
                    }
                    TagEnd::Strikethrough => {
                        strikethrough_depth -= 1;
                    }
                    TagEnd::Link => {
                        link = None;
                    }
                    TagEnd::Paragraph => {
                        self.cursor += 1;
                        break;
                    }
                    _ => {
                        break;
                    }
                },

                _ => {
                    break;
                }
            }

            self.cursor += 1;
        }

        ParsedMarkdownText {
            source_range,
            contents: text,
            highlights,
            regions,
            region_ranges,
        }
    }

    fn parse_heading(&mut self, level: pulldown_cmark::HeadingLevel) -> ParsedMarkdownHeading {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let text = self.parse_text(true);

        // Advance past the heading end tag
        self.cursor += 1;

        ParsedMarkdownHeading {
            source_range: source_range.clone(),
            level: match level {
                pulldown_cmark::HeadingLevel::H1 => HeadingLevel::H1,
                pulldown_cmark::HeadingLevel::H2 => HeadingLevel::H2,
                pulldown_cmark::HeadingLevel::H3 => HeadingLevel::H3,
                pulldown_cmark::HeadingLevel::H4 => HeadingLevel::H4,
                pulldown_cmark::HeadingLevel::H5 => HeadingLevel::H5,
                pulldown_cmark::HeadingLevel::H6 => HeadingLevel::H6,
            },
            contents: text,
        }
    }

    fn parse_table(&mut self, alignment: Vec<Alignment>) -> ParsedMarkdownTable {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut header = ParsedMarkdownTableRow::new();
        let mut body = vec![];
        let mut current_row = vec![];
        let mut in_header = true;
        let column_alignments = alignment
            .iter()
            .map(|a| Self::convert_alignment(a))
            .collect();

        loop {
            if self.eof() {
                break;
            }

            let (current, _source_range) = self.current().unwrap();
            match current {
                Event::Start(Tag::TableHead)
                | Event::Start(Tag::TableRow)
                | Event::End(TagEnd::TableCell) => {
                    self.cursor += 1;
                }
                Event::Start(Tag::TableCell) => {
                    self.cursor += 1;
                    let cell_contents = self.parse_text(false);
                    current_row.push(cell_contents);
                }
                Event::End(TagEnd::TableHead) | Event::End(TagEnd::TableRow) => {
                    self.cursor += 1;
                    let new_row = std::mem::replace(&mut current_row, vec![]);
                    if in_header {
                        header.children = new_row;
                        in_header = false;
                    } else {
                        let row = ParsedMarkdownTableRow::with_children(new_row);
                        body.push(row);
                    }
                }
                Event::End(TagEnd::Table) => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        ParsedMarkdownTable {
            source_range,
            header,
            body,
            column_alignments,
        }
    }

    fn convert_alignment(alignment: &Alignment) -> ParsedMarkdownTableAlignment {
        match alignment {
            Alignment::None => ParsedMarkdownTableAlignment::None,
            Alignment::Left => ParsedMarkdownTableAlignment::Left,
            Alignment::Center => ParsedMarkdownTableAlignment::Center,
            Alignment::Right => ParsedMarkdownTableAlignment::Right,
        }
    }

    #[async_recursion]
    async fn parse_list(&mut self, depth: u16, order: Option<u64>) -> ParsedMarkdownList {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut children = vec![];
        let mut inside_list_item = false;
        let mut order = order;
        let mut task_item = None;

        let mut current_list_items: Vec<Box<ParsedMarkdownElement>> = vec![];

        while !self.eof() {
            let (current, _source_range) = self.current().unwrap();
            match current {
                Event::Start(Tag::List(order)) => {
                    let order = *order;
                    self.cursor += 1;

                    let inner_list = self.parse_list(depth + 1, order).await;
                    let block = ParsedMarkdownElement::List(inner_list);
                    current_list_items.push(Box::new(block));
                }
                Event::End(TagEnd::List(_)) => {
                    self.cursor += 1;
                    break;
                }
                Event::Start(Tag::Item) => {
                    self.cursor += 1;
                    inside_list_item = true;

                    // Check for task list marker (`- [ ]` or `- [x]`)
                    if let Some(event) = self.current_event() {
                        // If there is a linebreak in between two list items the task list marker will actually be the first element of the paragraph
                        if event == &Event::Start(Tag::Paragraph) {
                            self.cursor += 1;
                        }

                        if let Some(Event::TaskListMarker(checked)) = self.current_event() {
                            task_item = Some(*checked);
                            self.cursor += 1;
                        }
                    }

                    if let Some(event) = self.current_event() {
                        // This is a plain list item.
                        // For example `- some text` or `1. [Docs](./docs.md)`
                        if MarkdownParser::is_text_like(event) {
                            let text = self.parse_text(false);
                            let block = ParsedMarkdownElement::Paragraph(text);
                            current_list_items.push(Box::new(block));
                        } else {
                            let block = self.parse_block().await;
                            if let Some(block) = block {
                                current_list_items.push(Box::new(block));
                            }
                        }
                    }

                    // If there is a linebreak in between two list items the task list marker will actually be the first element of the paragraph
                    if self.current_event() == Some(&Event::End(TagEnd::Paragraph)) {
                        self.cursor += 1;
                    }
                }
                Event::End(TagEnd::Item) => {
                    self.cursor += 1;

                    let item_type = if let Some(checked) = task_item {
                        ParsedMarkdownListItemType::Task(checked)
                    } else if let Some(order) = order {
                        ParsedMarkdownListItemType::Ordered(order)
                    } else {
                        ParsedMarkdownListItemType::Unordered
                    };

                    if let Some(current) = order {
                        order = Some(current + 1);
                    }

                    let contents = std::mem::replace(&mut current_list_items, vec![]);

                    children.push(ParsedMarkdownListItem {
                        contents,
                        depth,
                        item_type,
                    });

                    inside_list_item = false;
                    task_item = None;
                }
                _ => {
                    if !inside_list_item {
                        break;
                    }

                    let block = self.parse_block().await;
                    if let Some(block) = block {
                        current_list_items.push(Box::new(block));
                    }
                }
            }
        }

        ParsedMarkdownList {
            source_range,
            children,
        }
    }

    #[async_recursion]
    async fn parse_block_quote(&mut self) -> ParsedMarkdownBlockQuote {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut nested_depth = 1;

        let mut children: Vec<Box<ParsedMarkdownElement>> = vec![];

        while !self.eof() {
            let block = self.parse_block().await;

            if let Some(block) = block {
                children.push(Box::new(block));
            } else {
                break;
            }

            if self.eof() {
                break;
            }

            let (current, _source_range) = self.current().unwrap();
            match current {
                // This is a nested block quote.
                // Record that we're in a nested block quote and continue parsing.
                // We don't need to advance the cursor since the next
                // call to `parse_block` will handle it.
                Event::Start(Tag::BlockQuote) => {
                    nested_depth += 1;
                }
                Event::End(TagEnd::BlockQuote) => {
                    nested_depth -= 1;
                    if nested_depth == 0 {
                        self.cursor += 1;
                        break;
                    }
                }
                _ => {}
            };
        }

        ParsedMarkdownBlockQuote {
            source_range,
            children,
        }
    }

    async fn parse_code_block(&mut self, language: Option<String>) -> ParsedMarkdownCodeBlock {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut code = String::new();

        while !self.eof() {
            let (current, _source_range) = self.current().unwrap();
            match current {
                Event::Text(text) => {
                    code.push_str(&text);
                    self.cursor += 1;
                }
                Event::End(TagEnd::CodeBlock) => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        let highlights = if let Some(language) = &language {
            if let Some(registry) = &self.language_registry {
                let rope: language::Rope = code.as_str().into();
                registry
                    .language_for_name_or_extension(language)
                    .await
                    .map(|l| l.highlight_text(&rope, 0..code.len()))
                    .ok()
            } else {
                None
            }
        } else {
            None
        };

        ParsedMarkdownCodeBlock {
            source_range,
            contents: code.trim().to_string().into(),
            language,
            highlights,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use gpui::BackgroundExecutor;
    use language::{tree_sitter_rust, HighlightId, Language, LanguageConfig, LanguageMatcher};
    use pretty_assertions::assert_eq;

    use ParsedMarkdownElement::*;
    use ParsedMarkdownListItemType::*;

    async fn parse(input: &str) -> ParsedMarkdown {
        parse_markdown(input, None, None).await
    }

    #[gpui::test]
    async fn test_headings() {
        let parsed = parse("# Heading one\n## Heading two\n### Heading three").await;

        assert_eq!(
            parsed.children,
            vec![
                h1(text("Heading one", 0..14), 0..14),
                h2(text("Heading two", 14..29), 14..29),
                h3(text("Heading three", 29..46), 29..46),
            ]
        );
    }

    #[gpui::test]
    async fn test_newlines_dont_new_paragraphs() {
        let parsed = parse("Some text **that is bolded**\n and *italicized*").await;

        assert_eq!(
            parsed.children,
            vec![p("Some text that is bolded and italicized", 0..46)]
        );
    }

    #[gpui::test]
    async fn test_heading_with_paragraph() {
        let parsed = parse("# Zed\nThe editor").await;

        assert_eq!(
            parsed.children,
            vec![h1(text("Zed", 0..6), 0..6), p("The editor", 6..16),]
        );
    }

    #[gpui::test]
    async fn test_double_newlines_do_new_paragraphs() {
        let parsed = parse("Some text **that is bolded**\n\n and *italicized*").await;

        assert_eq!(
            parsed.children,
            vec![
                p("Some text that is bolded", 0..29),
                p("and italicized", 31..47),
            ]
        );
    }

    #[gpui::test]
    async fn test_bold_italic_text() {
        let parsed = parse("Some text **that is bolded** and *italicized*").await;

        assert_eq!(
            parsed.children,
            vec![p("Some text that is bolded and italicized", 0..45)]
        );
    }

    #[gpui::test]
    async fn test_nested_bold_strikethrough_text() {
        let parsed = parse("Some **bo~~strikethrough~~ld** text").await;

        assert_eq!(parsed.children.len(), 1);
        assert_eq!(
            parsed.children[0],
            ParsedMarkdownElement::Paragraph(ParsedMarkdownText {
                source_range: 0..35,
                contents: "Some bostrikethroughld text".to_string(),
                highlights: Vec::new(),
                region_ranges: Vec::new(),
                regions: Vec::new(),
            })
        );

        let paragraph = if let ParsedMarkdownElement::Paragraph(text) = &parsed.children[0] {
            text
        } else {
            panic!("Expected a paragraph");
        };
        assert_eq!(
            paragraph.highlights,
            vec![
                (
                    5..7,
                    MarkdownHighlight::Style(MarkdownHighlightStyle {
                        weight: FontWeight::BOLD,
                        ..Default::default()
                    }),
                ),
                (
                    7..20,
                    MarkdownHighlight::Style(MarkdownHighlightStyle {
                        weight: FontWeight::BOLD,
                        strikethrough: true,
                        ..Default::default()
                    }),
                ),
                (
                    20..22,
                    MarkdownHighlight::Style(MarkdownHighlightStyle {
                        weight: FontWeight::BOLD,
                        ..Default::default()
                    }),
                ),
            ]
        );
    }

    #[gpui::test]
    async fn test_header_only_table() {
        let markdown = "\
| Header 1 | Header 2 |
|----------|----------|

Some other content
";

        let expected_table = table(
            0..48,
            row(vec![text("Header 1", 1..11), text("Header 2", 12..22)]),
            vec![],
        );

        assert_eq!(
            parse(markdown).await.children[0],
            ParsedMarkdownElement::Table(expected_table)
        );
    }

    #[gpui::test]
    async fn test_basic_table() {
        let markdown = "\
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |";

        let expected_table = table(
            0..95,
            row(vec![text("Header 1", 1..11), text("Header 2", 12..22)]),
            vec![
                row(vec![text("Cell 1", 49..59), text("Cell 2", 60..70)]),
                row(vec![text("Cell 3", 73..83), text("Cell 4", 84..94)]),
            ],
        );

        assert_eq!(
            parse(markdown).await.children[0],
            ParsedMarkdownElement::Table(expected_table)
        );
    }

    #[gpui::test]
    async fn test_list_basic() {
        let parsed = parse(
            "\
* Item 1
* Item 2
* Item 3
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![list(
                vec![
                    list_item(1, Unordered, vec![p("Item 1", 0..9)]),
                    list_item(1, Unordered, vec![p("Item 2", 9..18)]),
                    list_item(1, Unordered, vec![p("Item 3", 18..27)]),
                ],
                0..27
            ),]
        );
    }

    #[gpui::test]
    async fn test_list_with_tasks() {
        let parsed = parse(
            "\
- [ ] TODO
- [x] Checked
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![list(
                vec![
                    list_item(1, Task(false), vec![p("TODO", 2..5)]),
                    list_item(1, Task(true), vec![p("Checked", 13..16)]),
                ],
                0..25
            ),]
        );
    }

    #[gpui::test]
    async fn test_list_with_linebreak_is_handled_correctly() {
        let parsed = parse(
            "\
- [ ] Task 1

- [x] Task 2
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![list(
                vec![
                    list_item(1, Task(false), vec![p("Task 1", 2..5)]),
                    list_item(1, Task(true), vec![p("Task 2", 16..19)]),
                ],
                0..27
            ),]
        );
    }

    #[gpui::test]
    async fn test_list_nested() {
        let parsed = parse(
            "\
* Item 1
* Item 2
* Item 3

1. Hello
1. Two
   1. Three
2. Four
3. Five

* First
  1. Hello
     1. Goodbyte
        - Inner
        - Inner
  2. Goodbyte
* Last
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![
                list(
                    vec![
                        list_item(1, Unordered, vec![p("Item 1", 0..9)]),
                        list_item(1, Unordered, vec![p("Item 2", 9..18)]),
                        list_item(1, Unordered, vec![p("Item 3", 18..28)]),
                    ],
                    0..28
                ),
                list(
                    vec![
                        list_item(1, Ordered(1), vec![p("Hello", 28..37)]),
                        list_item(
                            1,
                            Ordered(2),
                            vec![
                                p("Two", 37..56),
                                list(
                                    vec![list_item(2, Ordered(1), vec![p("Three", 47..56)]),],
                                    47..56
                                ),
                            ]
                        ),
                        list_item(1, Ordered(3), vec![p("Four", 56..64)]),
                        list_item(1, Ordered(4), vec![p("Five", 64..73)]),
                    ],
                    28..73
                ),
                list(
                    vec![
                        list_item(
                            1,
                            Unordered,
                            vec![
                                p("First", 73..155),
                                list(
                                    vec![
                                        list_item(
                                            2,
                                            Ordered(1),
                                            vec![
                                                p("Hello", 83..141),
                                                list(
                                                    vec![list_item(
                                                        3,
                                                        Ordered(1),
                                                        vec![
                                                            p("Goodbyte", 97..141),
                                                            list(
                                                                vec![
                                                                    list_item(
                                                                        4,
                                                                        Unordered,
                                                                        vec![p("Inner", 117..125)]
                                                                    ),
                                                                    list_item(
                                                                        4,
                                                                        Unordered,
                                                                        vec![p("Inner", 133..141)]
                                                                    ),
                                                                ],
                                                                117..141
                                                            )
                                                        ]
                                                    ),],
                                                    97..141
                                                )
                                            ]
                                        ),
                                        list_item(2, Ordered(2), vec![p("Goodbyte", 143..155)]),
                                    ],
                                    83..155
                                )
                            ]
                        ),
                        list_item(1, Unordered, vec![p("Last", 155..162)]),
                    ],
                    73..162
                ),
            ]
        );
    }

    #[gpui::test]
    async fn test_list_with_nested_content() {
        let parsed = parse(
            "\
*   This is a list item with two paragraphs.

    This is the second paragraph in the list item.",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![list(
                vec![list_item(
                    1,
                    Unordered,
                    vec![
                        p("This is a list item with two paragraphs.", 4..45),
                        p("This is the second paragraph in the list item.", 50..96)
                    ],
                ),],
                0..96,
            ),]
        );
    }

    #[gpui::test]
    async fn test_list_with_leading_text() {
        let parsed = parse(
            "\
* `code`
* **bold**
* [link](https://example.com)
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![list(
                vec![
                    list_item(1, Unordered, vec![p("code", 0..9)],),
                    list_item(1, Unordered, vec![p("bold", 9..20)]),
                    list_item(1, Unordered, vec![p("link", 20..50)],)
                ],
                0..50,
            ),]
        );
    }

    #[gpui::test]
    async fn test_simple_block_quote() {
        let parsed = parse("> Simple block quote with **styled text**").await;

        assert_eq!(
            parsed.children,
            vec![block_quote(
                vec![p("Simple block quote with styled text", 2..41)],
                0..41
            )]
        );
    }

    #[gpui::test]
    async fn test_simple_block_quote_with_multiple_lines() {
        let parsed = parse(
            "\
> # Heading
> More
> text
>
> More text
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![block_quote(
                vec![
                    h1(text("Heading", 2..12), 2..12),
                    p("More text", 14..26),
                    p("More text", 30..40)
                ],
                0..40
            )]
        );
    }

    #[gpui::test]
    async fn test_nested_block_quote() {
        let parsed = parse(
            "\
> A
>
> > # B
>
> C

More text
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![
                block_quote(
                    vec![
                        p("A", 2..4),
                        block_quote(vec![h1(text("B", 10..14), 10..14)], 8..14),
                        p("C", 18..20)
                    ],
                    0..20
                ),
                p("More text", 21..31)
            ]
        );
    }

    #[gpui::test]
    async fn test_code_block() {
        let parsed = parse(
            "\
```
fn main() {
    return 0;
}
```
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![code_block(
                None,
                "fn main() {\n    return 0;\n}",
                0..35,
                None
            )]
        );
    }

    #[gpui::test]
    async fn test_code_block_with_language(executor: BackgroundExecutor) {
        let language_registry = Arc::new(LanguageRegistry::test(executor.clone()));
        language_registry.add(rust_lang());

        let parsed = parse_markdown(
            "\
```rust
fn main() {
    return 0;
}
```
",
            None,
            Some(language_registry),
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![code_block(
                Some("rust".to_string()),
                "fn main() {\n    return 0;\n}",
                0..39,
                Some(vec![])
            )]
        );
    }

    fn rust_lang() -> Arc<Language> {
        Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".into()],
                    ..Default::default()
                },
                collapsed_placeholder: " /* ... */ ".to_string(),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ))
    }

    fn h1(contents: ParsedMarkdownText, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Heading(ParsedMarkdownHeading {
            source_range,
            level: HeadingLevel::H1,
            contents,
        })
    }

    fn h2(contents: ParsedMarkdownText, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Heading(ParsedMarkdownHeading {
            source_range,
            level: HeadingLevel::H2,
            contents,
        })
    }

    fn h3(contents: ParsedMarkdownText, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Heading(ParsedMarkdownHeading {
            source_range,
            level: HeadingLevel::H3,
            contents,
        })
    }

    fn p(contents: &str, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Paragraph(text(contents, source_range))
    }

    fn text(contents: &str, source_range: Range<usize>) -> ParsedMarkdownText {
        ParsedMarkdownText {
            highlights: Vec::new(),
            region_ranges: Vec::new(),
            regions: Vec::new(),
            source_range,
            contents: contents.to_string(),
        }
    }

    fn block_quote(
        children: Vec<ParsedMarkdownElement>,
        source_range: Range<usize>,
    ) -> ParsedMarkdownElement {
        ParsedMarkdownElement::BlockQuote(ParsedMarkdownBlockQuote {
            source_range,
            children: children.into_iter().map(Box::new).collect(),
        })
    }

    fn code_block(
        language: Option<String>,
        code: &str,
        source_range: Range<usize>,
        highlights: Option<Vec<(Range<usize>, HighlightId)>>,
    ) -> ParsedMarkdownElement {
        ParsedMarkdownElement::CodeBlock(ParsedMarkdownCodeBlock {
            source_range,
            language,
            contents: code.to_string().into(),
            highlights,
        })
    }

    fn list(
        children: Vec<ParsedMarkdownListItem>,
        source_range: Range<usize>,
    ) -> ParsedMarkdownElement {
        List(ParsedMarkdownList {
            source_range,
            children,
        })
    }

    fn list_item(
        depth: u16,
        item_type: ParsedMarkdownListItemType,
        contents: Vec<ParsedMarkdownElement>,
    ) -> ParsedMarkdownListItem {
        ParsedMarkdownListItem {
            item_type,
            depth,
            contents: contents.into_iter().map(Box::new).collect(),
        }
    }

    fn table(
        source_range: Range<usize>,
        header: ParsedMarkdownTableRow,
        body: Vec<ParsedMarkdownTableRow>,
    ) -> ParsedMarkdownTable {
        ParsedMarkdownTable {
            column_alignments: Vec::new(),
            source_range,
            header,
            body,
        }
    }

    fn row(children: Vec<ParsedMarkdownText>) -> ParsedMarkdownTableRow {
        ParsedMarkdownTableRow { children }
    }

    impl PartialEq for ParsedMarkdownTable {
        fn eq(&self, other: &Self) -> bool {
            self.source_range == other.source_range
                && self.header == other.header
                && self.body == other.body
        }
    }

    impl PartialEq for ParsedMarkdownText {
        fn eq(&self, other: &Self) -> bool {
            self.source_range == other.source_range && self.contents == other.contents
        }
    }
}
