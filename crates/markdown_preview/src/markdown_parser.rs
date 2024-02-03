use crate::markdown_elements::*;
use gpui::FontWeight;
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag};
use std::{collections::HashMap, ops::Range};

pub fn parse_markdown(markdown_input: &str) -> ParsedMarkdown {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options);
    let parser = MarkdownParser::new(parser.into_offset_iter().collect());
    let renderer = parser.parse_document();
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
}

impl<'a> MarkdownParser<'a> {
    fn new(tokens: Vec<(Event<'a>, Range<usize>)>) -> Self {
        Self {
            tokens,
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

    fn parse_document(mut self) -> Self {
        while !self.eof() {
            if let Some(block) = self.parse_block() {
                self.parsed.push(block);
            }
        }
        self
    }

    fn parse_block(&mut self) -> Option<ParsedMarkdownElement> {
        let (current, _source_range) = self.current().unwrap();
        match current {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    self.cursor += 1;
                    let text = self.parse_text(false);
                    Some(ParsedMarkdownElement::Paragraph(text))
                }
                Tag::Heading(level, _, _) => {
                    let level = level.clone();
                    self.cursor += 1;
                    let heading = self.parse_heading(level);
                    Some(ParsedMarkdownElement::Heading(heading))
                }
                Tag::Table(_) => {
                    self.cursor += 1;
                    let table = self.parse_table();
                    Some(ParsedMarkdownElement::Table(table))
                }
                Tag::List(order) => {
                    let order = order.clone();
                    self.cursor += 1;
                    let list = self.parse_list(order);
                    Some(ParsedMarkdownElement::List(list))
                }
                Tag::BlockQuote => {
                    self.cursor += 1;
                    let block_quote = self.parse_block_quote();
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

                    let code_block = self.parse_code_block(language);
                    Some(ParsedMarkdownElement::CodeBlock(code_block))
                }
                _ => {
                    self.cursor += 1;
                    None
                }
            },
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
        let mut link_url = None;
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
                    break;
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

                    if let Some(link) = link_url.clone().and_then(|u| Link::identify(u)) {
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

                    let link = link_url.clone().and_then(|u| Link::identify(u));
                    if link.is_some() {
                        highlights.push((
                            prev_len..text.len(),
                            MarkdownHighlight::Style(MarkdownHighlightStyle {
                                underline: true,
                                ..Default::default()
                            }),
                        ));
                    }
                    regions.push(ParsedRegion { code: true, link });
                }

                Event::Start(tag) => {
                    match tag {
                        Tag::Emphasis => italic_depth += 1,
                        Tag::Strong => bold_depth += 1,
                        Tag::Link(_type, url, _title) => {
                            link_url = Some(url.to_string());
                        }
                        Tag::Strikethrough => {
                            // TODO: Confirm that gpui currently doesn't support strikethroughs
                        }
                        _ => {
                            break;
                        }
                    }
                }

                Event::End(tag) => match tag {
                    Tag::Emphasis => {
                        italic_depth -= 1;
                    }
                    Tag::Strong => {
                        bold_depth -= 1;
                    }
                    Tag::Link(_, _, _) => {
                        link_url = None;
                    }
                    Tag::Strikethrough => {
                        // TODO: Confirm that gpui currently doesn't support strikethroughs
                    }
                    Tag::Paragraph => {
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

    fn parse_table(&mut self) -> ParsedMarkdownTable {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut header = ParsedMarkdownTableRow::new();
        let mut body = vec![];
        let mut current_row = vec![];
        let mut in_header = true;
        let mut alignment: Vec<ParsedMarkdownTableAlignment> = vec![];

        // Expect at least a header row
        // Expect zero or more rows for the body
        loop {
            if self.eof() {
                break;
            }

            let (current, _source_range) = self.current().unwrap();
            match current {
                Event::Start(Tag::TableHead)
                | Event::Start(Tag::TableRow)
                | Event::End(Tag::TableCell) => {
                    self.cursor += 1;
                }
                Event::Start(Tag::TableCell) => {
                    self.cursor += 1;
                    let cell_contents = self.parse_text(false);
                    current_row.push(cell_contents);
                }
                Event::End(Tag::TableHead) | Event::End(Tag::TableRow) => {
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
                Event::End(Tag::Table(table_alignment)) => {
                    alignment = table_alignment
                        .iter()
                        .map(|a| Self::convert_alignment(a))
                        .collect();
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
            column_alignments: alignment,
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

    fn parse_list(&mut self, order: Option<u64>) -> ParsedMarkdownList {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut orders_by_depth: HashMap<u16, Option<u64>> = HashMap::new();
        let mut depth: u16 = 1;
        let mut children = vec![];

        orders_by_depth.insert(1, order);

        loop {
            if self.eof() {
                break;
            }

            let (current, _source_range) = self.current().unwrap();
            match current {
                Event::Start(Tag::List(order)) => {
                    depth += 1;
                    orders_by_depth.insert(depth, order.clone());

                    self.cursor += 1;
                }
                Event::End(Tag::List(_)) => {
                    self.cursor += 1;

                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Event::Start(Tag::Item) => {
                    self.cursor += 1;

                    let order = orders_by_depth.get(&depth).unwrap_or(&None).clone();
                    match order {
                        Some(order) => {
                            orders_by_depth.insert(depth, Some(order + 1));
                        }
                        _ => {}
                    };

                    let contents = self.parse_text(false);
                    children.push(ParsedMarkdownListItem {
                        order,
                        depth,
                        contents,
                    });
                }
                Event::End(Tag::Item) => {
                    self.cursor += 1;
                }
                _ => {
                    break;
                }
            }
        }

        ParsedMarkdownList {
            source_range,
            children,
        }
    }

    fn parse_block_quote(&mut self) -> ParsedMarkdownBlockQuote {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut nested_depth = 1;

        let mut children: Vec<Box<ParsedMarkdownElement>> = vec![];

        while !self.eof() {
            let block = self.parse_block();

            if let Some(block) = block {
                children.push(Box::new(block));
            } else {
                // TODO: Needed?
                break;
            }

            if self.eof() {
                break;
            }

            let (current, _source_range) = self.current().unwrap();
            match current {
                // This is a nested block quote.
                // Record that we're in a nested block quote and continue parsing.
                Event::Start(Tag::BlockQuote) => {
                    nested_depth += 1;
                }
                Event::End(Tag::BlockQuote) => {
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

    fn parse_code_block(&mut self, language: Option<String>) -> ParsedMarkdownCodeBlock {
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
                Event::End(Tag::CodeBlock(_)) => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        ParsedMarkdownCodeBlock {
            source_range,
            contents: code.trim().to_string().into(),
            language,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use ParsedMarkdownElement::*;

    #[test]
    fn test_headings() {
        let parsed = parse_markdown("# Heading one\n## Heading two\n### Heading three");

        assert_eq!(
            parsed.children,
            vec![
                h1(text("Heading one", 0..14), 0..14),
                h2(text("Heading two", 14..29), 14..29),
                h3(text("Heading three", 29..46), 29..46),
            ]
        );
    }

    #[test]
    fn test_newlines_dont_new_paragraphs() {
        let parsed = parse_markdown("Some text **that is bolded**\n and *italicized*");

        assert_eq!(
            parsed.children,
            vec![p("Some text that is bolded and italicized", 0..46)]
        );
    }

    #[test]
    fn test_heading_with_paragraph() {
        let parsed = parse_markdown("# Zed\nThe editor");

        assert_eq!(
            parsed.children,
            vec![h1(text("Zed", 0..6), 0..6), p("The editor", 6..16),]
        );
    }

    #[test]
    fn test_double_newlines_do_new_paragraphs() {
        let parsed = parse_markdown("Some text **that is bolded**\n\n and *italicized*");

        assert_eq!(
            parsed.children,
            vec![
                p("Some text that is bolded", 0..29),
                p("and italicized", 31..47),
            ]
        );
    }

    #[test]
    fn test_bold_italic_text() {
        let parsed = parse_markdown("Some text **that is bolded** and *italicized*");

        assert_eq!(
            parsed.children,
            vec![p("Some text that is bolded and italicized", 0..45)]
        );
    }

    #[test]
    fn test_header_only_table() {
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
            parse_markdown(markdown).children[0],
            ParsedMarkdownElement::Table(expected_table)
        );
    }

    #[test]
    fn test_basic_table() {
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
            parse_markdown(markdown).children[0],
            ParsedMarkdownElement::Table(expected_table)
        );
    }

    #[test]
    fn test_list() {
        let parsed = parse_markdown(
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
        );

        assert_eq!(
            parsed.children,
            vec![
                list(
                    vec![
                        list_item(1, None, text("Item 1", 0..9)),
                        list_item(1, None, text("Item 2", 9..18)),
                        list_item(1, None, text("Item 3", 18..28)),
                    ],
                    0..28
                ),
                list(
                    vec![
                        list_item(1, Some(1), text("Hello", 28..37)),
                        list_item(1, Some(2), text("Two", 37..56)),
                        list_item(2, Some(1), text("Three", 47..56)),
                        list_item(1, Some(3), text("Four", 56..64)),
                        list_item(1, Some(4), text("Five", 64..73)),
                    ],
                    28..73
                ),
                list(
                    vec![
                        list_item(1, None, text("First", 73..155)),
                        list_item(2, Some(1), text("Hello", 83..141)),
                        list_item(3, Some(1), text("Goodbyte", 97..141)),
                        list_item(4, None, text("Inner", 117..125)),
                        list_item(4, None, text("Inner", 133..141)),
                        list_item(2, Some(2), text("Goodbyte", 143..155)),
                        list_item(1, None, text("Last", 155..162)),
                    ],
                    73..162
                ),
            ]
        );
    }

    #[test]
    fn test_simple_block_quote() {
        let parsed = parse_markdown("> Simple block quote with **styled text**");

        assert_eq!(
            parsed.children,
            vec![block_quote(
                vec![p("Simple block quote with styled text", 2..41)],
                0..41
            )]
        );
    }

    #[test]
    fn test_simple_block_quote_with_multiple_lines() {
        let parsed = parse_markdown(
            "\
> # Heading
> More
> text
>
> More text
",
        );

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

    #[test]
    fn test_nested_block_quote() {
        let parsed = parse_markdown(
            "\
> A
>
> > # B
>
> C

More text
",
        );

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

    #[test]
    fn test_code_block() {
        let parsed = parse_markdown(
            "\
```
fn main() {
    return 0;
}
```
",
        );

        assert_eq!(
            parsed.children,
            vec![code_block(None, "fn main() {\n    return 0;\n}", 0..35)]
        );
    }

    #[test]
    fn test_code_block_with_language() {
        let parsed = parse_markdown(
            "\
```rust
fn main() {
    return 0;
}
```
",
        );

        assert_eq!(
            parsed.children,
            vec![code_block(
                Some("rust".into()),
                "fn main() {\n    return 0;\n}",
                0..39
            )]
        );
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
    ) -> ParsedMarkdownElement {
        ParsedMarkdownElement::CodeBlock(ParsedMarkdownCodeBlock {
            source_range,
            language,
            contents: code.to_string().into(),
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
        order: Option<u64>,
        contents: ParsedMarkdownText,
    ) -> ParsedMarkdownListItem {
        ParsedMarkdownListItem {
            order,
            depth,
            contents,
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
