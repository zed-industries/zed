use crate::markdown_elements::*;
use async_recursion::async_recursion;
use collections::FxHashMap;
use gpui::FontWeight;
use language::LanguageRegistry;
use pulldown_cmark::{Alignment, Event, Options, Parser, Tag, TagEnd};
use std::{ops::Range, path::PathBuf, sync::Arc, vec};

pub async fn parse_markdown(
    markdown_input: &str,
    file_location_directory: Option<PathBuf>,
    language_registry: Option<Arc<LanguageRegistry>>,
) -> ParsedMarkdown {
    let mut options = Options::all();
    options.remove(pulldown_cmark::Options::ENABLE_DEFINITION_LIST);

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

struct MarkdownListItem {
    content: Vec<ParsedMarkdownElement>,
    item_type: ParsedMarkdownListItemType,
}

impl Default for MarkdownListItem {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            item_type: ParsedMarkdownListItemType::Unordered,
        }
    }
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

    fn peek(&self, steps: usize) -> Option<&(Event<'_>, Range<usize>)> {
        if self.eof() || (steps + self.cursor) >= self.tokens.len() {
            return self.tokens.last();
        }
        self.tokens.get(self.cursor + steps)
    }

    fn previous(&self) -> Option<&(Event<'_>, Range<usize>)> {
        if self.cursor == 0 || self.cursor > self.tokens.len() {
            return None;
        }
        self.tokens.get(self.cursor - 1)
    }

    fn current(&self) -> Option<&(Event<'_>, Range<usize>)> {
        self.peek(0)
    }

    fn current_event(&self) -> Option<&Event<'_>> {
        self.current().map(|(event, _)| event)
    }

    fn is_text_like(event: &Event) -> bool {
        match event {
            Event::Text(_)
            // Represent an inline code block
            | Event::Code(_)
            | Event::Html(_)
            | Event::InlineHtml(_)
            | Event::FootnoteReference(_)
            | Event::Start(Tag::Link { .. })
            | Event::Start(Tag::Emphasis)
            | Event::Start(Tag::Strong)
            | Event::Start(Tag::Strikethrough)
            | Event::Start(Tag::Image { .. }) => {
                true
            }
            _ => false,
        }
    }

    async fn parse_document(mut self) -> Self {
        while !self.eof() {
            if let Some(block) = self.parse_block().await {
                self.parsed.extend(block);
            } else {
                self.cursor += 1;
            }
        }
        self
    }

    #[async_recursion]
    async fn parse_block(&mut self) -> Option<Vec<ParsedMarkdownElement>> {
        let (current, source_range) = self.current().unwrap();
        let source_range = source_range.clone();
        match current {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    self.cursor += 1;
                    let text = self.parse_text(false, Some(source_range));
                    Some(vec![ParsedMarkdownElement::Paragraph(text)])
                }
                Tag::Heading { level, .. } => {
                    let level = *level;
                    self.cursor += 1;
                    let heading = self.parse_heading(level);
                    Some(vec![ParsedMarkdownElement::Heading(heading)])
                }
                Tag::Table(alignment) => {
                    let alignment = alignment.clone();
                    self.cursor += 1;
                    let table = self.parse_table(alignment);
                    Some(vec![ParsedMarkdownElement::Table(table)])
                }
                Tag::List(order) => {
                    let order = *order;
                    self.cursor += 1;
                    let list = self.parse_list(order).await;
                    Some(list)
                }
                Tag::BlockQuote(_kind) => {
                    self.cursor += 1;
                    let block_quote = self.parse_block_quote().await;
                    Some(vec![ParsedMarkdownElement::BlockQuote(block_quote)])
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
                    Some(vec![ParsedMarkdownElement::CodeBlock(code_block)])
                }
                _ => None,
            },
            Event::Rule => {
                self.cursor += 1;
                Some(vec![ParsedMarkdownElement::HorizontalRule(source_range)])
            }
            _ => None,
        }
    }

    fn parse_text(
        &mut self,
        should_complete_on_soft_break: bool,
        source_range: Option<Range<usize>>,
    ) -> MarkdownParagraph {
        let source_range = source_range.unwrap_or_else(|| {
            self.current()
                .map(|(_, range)| range.clone())
                .unwrap_or_default()
        });

        let mut markdown_text_like = Vec::new();
        let mut text = String::new();
        let mut bold_depth = 0;
        let mut italic_depth = 0;
        let mut strikethrough_depth = 0;
        let mut link: Option<Link> = None;
        let mut image: Option<Image> = None;
        let mut region_ranges: Vec<Range<usize>> = vec![];
        let mut regions: Vec<ParsedRegion> = vec![];
        let mut highlights: Vec<(Range<usize>, MarkdownHighlight)> = vec![];
        let mut link_urls: Vec<String> = vec![];
        let mut link_ranges: Vec<Range<usize>> = vec![];

        loop {
            if self.eof() {
                break;
            }

            let (current, _) = self.current().unwrap();
            let prev_len = text.len();
            match current {
                Event::SoftBreak => {
                    if should_complete_on_soft_break {
                        break;
                    }
                    text.push(' ');
                }

                Event::HardBreak => {
                    text.push('\n');
                }

                // We want to ignore any inline HTML tags in the text but keep
                // the text between them
                Event::InlineHtml(_) => {}

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

                    let last_run_len = if let Some(link) = link.clone() {
                        region_ranges.push(prev_len..text.len());
                        regions.push(ParsedRegion {
                            code: false,
                            link: Some(link),
                        });
                        style.underline = true;
                        prev_len
                    } else {
                        // Manually scan for links
                        let mut finder = linkify::LinkFinder::new();
                        finder.kinds(&[linkify::LinkKind::Url]);
                        let mut last_link_len = prev_len;
                        for link in finder.links(t) {
                            let start = link.start();
                            let end = link.end();
                            let range = (prev_len + start)..(prev_len + end);
                            link_ranges.push(range.clone());
                            link_urls.push(link.as_str().to_string());

                            // If there is a style before we match a link, we have to add this to the highlighted ranges
                            if style != MarkdownHighlightStyle::default()
                                && last_link_len < link.start()
                            {
                                highlights.push((
                                    last_link_len..link.start(),
                                    MarkdownHighlight::Style(style.clone()),
                                ));
                            }

                            highlights.push((
                                range.clone(),
                                MarkdownHighlight::Style(MarkdownHighlightStyle {
                                    underline: true,
                                    ..style
                                }),
                            ));
                            region_ranges.push(range.clone());
                            regions.push(ParsedRegion {
                                code: false,
                                link: Some(Link::Web {
                                    url: link.as_str().to_string(),
                                }),
                            });
                            last_link_len = end;
                        }
                        last_link_len
                    };

                    if style != MarkdownHighlightStyle::default() && last_run_len < text.len() {
                        let mut new_highlight = true;
                        if let Some((last_range, last_style)) = highlights.last_mut()
                            && last_range.end == last_run_len
                            && last_style == &MarkdownHighlight::Style(style.clone())
                        {
                            last_range.end = text.len();
                            new_highlight = false;
                        }
                        if new_highlight {
                            highlights.push((
                                last_run_len..text.len(),
                                MarkdownHighlight::Style(style.clone()),
                            ));
                        }
                    }
                }
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
                    Tag::Link { dest_url, .. } => {
                        link = Link::identify(
                            self.file_location_directory.clone(),
                            dest_url.to_string(),
                        );
                    }
                    Tag::Image { dest_url, .. } => {
                        if !text.is_empty() {
                            let parsed_regions = MarkdownParagraphChunk::Text(ParsedMarkdownText {
                                source_range: source_range.clone(),
                                contents: text.clone(),
                                highlights: highlights.clone(),
                                region_ranges: region_ranges.clone(),
                                regions: regions.clone(),
                            });
                            text = String::new();
                            highlights = vec![];
                            region_ranges = vec![];
                            regions = vec![];
                            markdown_text_like.push(parsed_regions);
                        }
                        image = Image::identify(
                            dest_url.to_string(),
                            source_range.clone(),
                            self.file_location_directory.clone(),
                        );
                    }
                    _ => {
                        break;
                    }
                },

                Event::End(tag) => match tag {
                    TagEnd::Emphasis => italic_depth -= 1,
                    TagEnd::Strong => bold_depth -= 1,
                    TagEnd::Strikethrough => strikethrough_depth -= 1,
                    TagEnd::Link => {
                        link = None;
                    }
                    TagEnd::Image => {
                        if let Some(mut image) = image.take() {
                            if !text.is_empty() {
                                image.alt_text = Some(std::mem::take(&mut text).into());
                            }
                            markdown_text_like.push(MarkdownParagraphChunk::Image(image));
                        }
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
        if !text.is_empty() {
            markdown_text_like.push(MarkdownParagraphChunk::Text(ParsedMarkdownText {
                source_range,
                contents: text,
                highlights,
                regions,
                region_ranges,
            }));
        }
        markdown_text_like
    }

    fn parse_heading(&mut self, level: pulldown_cmark::HeadingLevel) -> ParsedMarkdownHeading {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let text = self.parse_text(true, None);

        // Advance past the heading end tag
        self.cursor += 1;

        ParsedMarkdownHeading {
            source_range,
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
        let column_alignments = alignment.iter().map(Self::convert_alignment).collect();

        loop {
            if self.eof() {
                break;
            }

            let (current, source_range) = self.current().unwrap();
            let source_range = source_range.clone();
            match current {
                Event::Start(Tag::TableHead)
                | Event::Start(Tag::TableRow)
                | Event::End(TagEnd::TableCell) => {
                    self.cursor += 1;
                }
                Event::Start(Tag::TableCell) => {
                    self.cursor += 1;
                    let cell_contents = self.parse_text(false, Some(source_range));
                    current_row.push(cell_contents);
                }
                Event::End(TagEnd::TableHead) | Event::End(TagEnd::TableRow) => {
                    self.cursor += 1;
                    let new_row = std::mem::take(&mut current_row);
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

    async fn parse_list(&mut self, order: Option<u64>) -> Vec<ParsedMarkdownElement> {
        let (_, list_source_range) = self.previous().unwrap();

        let mut items = Vec::new();
        let mut items_stack = vec![MarkdownListItem::default()];
        let mut depth = 1;
        let mut order = order;
        let mut order_stack = Vec::new();

        let mut insertion_indices = FxHashMap::default();
        let mut source_ranges = FxHashMap::default();
        let mut start_item_range = list_source_range.clone();

        while !self.eof() {
            let (current, source_range) = self.current().unwrap();
            match current {
                Event::Start(Tag::List(new_order)) => {
                    if items_stack.last().is_some() && !insertion_indices.contains_key(&depth) {
                        insertion_indices.insert(depth, items.len());
                    }

                    // We will use the start of the nested list as the end for the current item's range,
                    // because we don't care about the hierarchy of list items
                    if let collections::hash_map::Entry::Vacant(e) = source_ranges.entry(depth) {
                        e.insert(start_item_range.start..source_range.start);
                    }

                    order_stack.push(order);
                    order = *new_order;
                    self.cursor += 1;
                    depth += 1;
                }
                Event::End(TagEnd::List(_)) => {
                    order = order_stack.pop().flatten();
                    self.cursor += 1;
                    depth -= 1;

                    if depth == 0 {
                        break;
                    }
                }
                Event::Start(Tag::Item) => {
                    start_item_range = source_range.clone();

                    self.cursor += 1;
                    items_stack.push(MarkdownListItem::default());

                    let mut task_list = None;
                    // Check for task list marker (`- [ ]` or `- [x]`)
                    if let Some(event) = self.current_event() {
                        // If there is a linebreak in between two list items the task list marker will actually be the first element of the paragraph
                        if event == &Event::Start(Tag::Paragraph) {
                            self.cursor += 1;
                        }

                        if let Some((Event::TaskListMarker(checked), range)) = self.current() {
                            task_list = Some((*checked, range.clone()));
                            self.cursor += 1;
                        }
                    }

                    if let Some((event, range)) = self.current() {
                        // This is a plain list item.
                        // For example `- some text` or `1. [Docs](./docs.md)`
                        if MarkdownParser::is_text_like(event) {
                            let text = self.parse_text(false, Some(range.clone()));
                            let block = ParsedMarkdownElement::Paragraph(text);
                            if let Some(content) = items_stack.last_mut() {
                                let item_type = if let Some((checked, range)) = task_list {
                                    ParsedMarkdownListItemType::Task(checked, range)
                                } else if let Some(order) = order {
                                    ParsedMarkdownListItemType::Ordered(order)
                                } else {
                                    ParsedMarkdownListItemType::Unordered
                                };
                                content.item_type = item_type;
                                content.content.push(block);
                            }
                        } else {
                            let block = self.parse_block().await;
                            if let Some(block) = block
                                && let Some(list_item) = items_stack.last_mut()
                            {
                                list_item.content.extend(block);
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

                    if let Some(current) = order {
                        order = Some(current + 1);
                    }

                    if let Some(list_item) = items_stack.pop() {
                        let source_range = source_ranges
                            .remove(&depth)
                            .unwrap_or(start_item_range.clone());

                        // We need to remove the last character of the source range, because it includes the newline character
                        let source_range = source_range.start..source_range.end - 1;
                        let item = ParsedMarkdownElement::ListItem(ParsedMarkdownListItem {
                            source_range,
                            content: list_item.content,
                            depth,
                            item_type: list_item.item_type,
                        });

                        if let Some(index) = insertion_indices.get(&depth) {
                            items.insert(*index, item);
                            insertion_indices.remove(&depth);
                        } else {
                            items.push(item);
                        }
                    }
                }
                _ => {
                    if depth == 0 {
                        break;
                    }
                    // This can only happen if a list item starts with more then one paragraph,
                    // or the list item contains blocks that should be rendered after the nested list items
                    let block = self.parse_block().await;
                    if let Some(block) = block {
                        if let Some(list_item) = items_stack.last_mut() {
                            // If we did not insert any nested items yet (in this case insertion index is set), we can append the block to the current list item
                            if !insertion_indices.contains_key(&depth) {
                                list_item.content.extend(block);
                                continue;
                            }
                        }

                        // Otherwise we need to insert the block after all the nested items
                        // that have been parsed so far
                        items.extend(block);
                    } else {
                        self.cursor += 1;
                    }
                }
            }
        }

        items
    }

    #[async_recursion]
    async fn parse_block_quote(&mut self) -> ParsedMarkdownBlockQuote {
        let (_event, source_range) = self.previous().unwrap();
        let source_range = source_range.clone();
        let mut nested_depth = 1;

        let mut children: Vec<ParsedMarkdownElement> = vec![];

        while !self.eof() {
            let block = self.parse_block().await;

            if let Some(block) = block {
                children.extend(block);
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
                Event::Start(Tag::BlockQuote(_kind)) => {
                    nested_depth += 1;
                }
                Event::End(TagEnd::BlockQuote(_kind)) => {
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
                    code.push_str(text);
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

        code = code.strip_suffix('\n').unwrap_or(&code).to_string();

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
            contents: code.into(),
            language,
            highlights,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    use ParsedMarkdownListItemType::*;
    use gpui::BackgroundExecutor;
    use language::{
        HighlightId, Language, LanguageConfig, LanguageMatcher, LanguageRegistry, tree_sitter_rust,
    };
    use pretty_assertions::assert_eq;

    async fn parse(input: &str) -> ParsedMarkdown {
        parse_markdown(input, None, None).await
    }

    #[gpui::test]
    async fn test_headings() {
        let parsed = parse("# Heading one\n## Heading two\n### Heading three").await;

        assert_eq!(
            parsed.children,
            vec![
                h1(text("Heading one", 2..13), 0..14),
                h2(text("Heading two", 17..28), 14..29),
                h3(text("Heading three", 33..46), 29..46),
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
            vec![h1(text("Zed", 2..5), 0..6), p("The editor", 6..16),]
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
            ParsedMarkdownElement::Paragraph(vec![MarkdownParagraphChunk::Text(
                ParsedMarkdownText {
                    source_range: 0..35,
                    contents: "Some bostrikethroughld text".to_string(),
                    highlights: Vec::new(),
                    region_ranges: Vec::new(),
                    regions: Vec::new(),
                }
            )])
        );

        let new_text = if let ParsedMarkdownElement::Paragraph(text) = &parsed.children[0] {
            text
        } else {
            panic!("Expected a paragraph");
        };

        let paragraph = if let MarkdownParagraphChunk::Text(text) = &new_text[0] {
            text
        } else {
            panic!("Expected a text");
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
    async fn test_text_with_inline_html() {
        let parsed = parse("This is a paragraph with an inline HTML <sometag>tag</sometag>.").await;

        assert_eq!(
            parsed.children,
            vec![p("This is a paragraph with an inline HTML tag.", 0..63),],
        );
    }

    #[gpui::test]
    async fn test_raw_links_detection() {
        let parsed = parse("Checkout this https://zed.dev link").await;

        assert_eq!(
            parsed.children,
            vec![p("Checkout this https://zed.dev link", 0..34)]
        );
    }

    #[gpui::test]
    async fn test_empty_image() {
        let parsed = parse("![]()").await;

        let paragraph = if let ParsedMarkdownElement::Paragraph(text) = &parsed.children[0] {
            text
        } else {
            panic!("Expected a paragraph");
        };
        assert_eq!(paragraph.len(), 0);
    }

    #[gpui::test]
    async fn test_image_links_detection() {
        let parsed = parse("![test](https://blog.logrocket.com/wp-content/uploads/2024/04/exploring-zed-open-source-code-editor-rust-2.png)").await;

        let paragraph = if let ParsedMarkdownElement::Paragraph(text) = &parsed.children[0] {
            text
        } else {
            panic!("Expected a paragraph");
        };
        assert_eq!(
            paragraph[0],
            MarkdownParagraphChunk::Image(Image {
                source_range: 0..111,
                link: Link::Web {
                    url: "https://blog.logrocket.com/wp-content/uploads/2024/04/exploring-zed-open-source-code-editor-rust-2.png".to_string(),
                },
                alt_text: Some("test".into()),
            },)
        );
    }

    #[gpui::test]
    async fn test_image_without_alt_text() {
        let parsed = parse("![](http://example.com/foo.png)").await;

        let paragraph = if let ParsedMarkdownElement::Paragraph(text) = &parsed.children[0] {
            text
        } else {
            panic!("Expected a paragraph");
        };
        assert_eq!(
            paragraph[0],
            MarkdownParagraphChunk::Image(Image {
                source_range: 0..31,
                link: Link::Web {
                    url: "http://example.com/foo.png".to_string(),
                },
                alt_text: None,
            },)
        );
    }

    #[gpui::test]
    async fn test_image_with_alt_text_containing_formatting() {
        let parsed = parse("![foo *bar* baz](http://example.com/foo.png)").await;

        let ParsedMarkdownElement::Paragraph(chunks) = &parsed.children[0] else {
            panic!("Expected a paragraph");
        };
        assert_eq!(
            chunks,
            &[MarkdownParagraphChunk::Image(Image {
                source_range: 0..44,
                link: Link::Web {
                    url: "http://example.com/foo.png".to_string(),
                },
                alt_text: Some("foo bar baz".into()),
            }),],
        );
    }

    #[gpui::test]
    async fn test_images_with_text_in_between() {
        let parsed = parse(
            "![foo](http://example.com/foo.png)\nLorem Ipsum\n![bar](http://example.com/bar.png)",
        )
        .await;

        let chunks = if let ParsedMarkdownElement::Paragraph(text) = &parsed.children[0] {
            text
        } else {
            panic!("Expected a paragraph");
        };
        assert_eq!(
            chunks,
            &vec![
                MarkdownParagraphChunk::Image(Image {
                    source_range: 0..81,
                    link: Link::Web {
                        url: "http://example.com/foo.png".to_string(),
                    },
                    alt_text: Some("foo".into()),
                }),
                MarkdownParagraphChunk::Text(ParsedMarkdownText {
                    source_range: 0..81,
                    contents: " Lorem Ipsum ".to_string(),
                    highlights: Vec::new(),
                    region_ranges: Vec::new(),
                    regions: Vec::new(),
                }),
                MarkdownParagraphChunk::Image(Image {
                    source_range: 0..81,
                    link: Link::Web {
                        url: "http://example.com/bar.png".to_string(),
                    },
                    alt_text: Some("bar".into()),
                })
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
            vec![
                list_item(0..8, 1, Unordered, vec![p("Item 1", 2..8)]),
                list_item(9..17, 1, Unordered, vec![p("Item 2", 11..17)]),
                list_item(18..26, 1, Unordered, vec![p("Item 3", 20..26)]),
            ],
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
            vec![
                list_item(0..10, 1, Task(false, 2..5), vec![p("TODO", 6..10)]),
                list_item(11..24, 1, Task(true, 13..16), vec![p("Checked", 17..24)]),
            ],
        );
    }

    #[gpui::test]
    async fn test_list_with_indented_task() {
        let parsed = parse(
            "\
- [ ] TODO
  - [x] Checked
  - Unordered
  1. Number 1
  1. Number 2
1. Number A
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![
                list_item(0..12, 1, Task(false, 2..5), vec![p("TODO", 6..10)]),
                list_item(13..26, 2, Task(true, 15..18), vec![p("Checked", 19..26)]),
                list_item(29..40, 2, Unordered, vec![p("Unordered", 31..40)]),
                list_item(43..54, 2, Ordered(1), vec![p("Number 1", 46..54)]),
                list_item(57..68, 2, Ordered(2), vec![p("Number 2", 60..68)]),
                list_item(69..80, 1, Ordered(1), vec![p("Number A", 72..80)]),
            ],
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
            vec![
                list_item(0..13, 1, Task(false, 2..5), vec![p("Task 1", 6..12)]),
                list_item(14..26, 1, Task(true, 16..19), vec![p("Task 2", 20..26)]),
            ],
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
        - Next item empty
        -
* Last
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![
                list_item(0..8, 1, Unordered, vec![p("Item 1", 2..8)]),
                list_item(9..17, 1, Unordered, vec![p("Item 2", 11..17)]),
                list_item(18..27, 1, Unordered, vec![p("Item 3", 20..26)]),
                list_item(28..36, 1, Ordered(1), vec![p("Hello", 31..36)]),
                list_item(37..46, 1, Ordered(2), vec![p("Two", 40..43),]),
                list_item(47..55, 2, Ordered(1), vec![p("Three", 50..55)]),
                list_item(56..63, 1, Ordered(3), vec![p("Four", 59..63)]),
                list_item(64..72, 1, Ordered(4), vec![p("Five", 67..71)]),
                list_item(73..82, 1, Unordered, vec![p("First", 75..80)]),
                list_item(83..96, 2, Ordered(1), vec![p("Hello", 86..91)]),
                list_item(97..116, 3, Ordered(1), vec![p("Goodbyte", 100..108)]),
                list_item(117..124, 4, Unordered, vec![p("Inner", 119..124)]),
                list_item(133..140, 4, Unordered, vec![p("Inner", 135..140)]),
                list_item(143..159, 2, Ordered(2), vec![p("Goodbyte", 146..154)]),
                list_item(160..180, 3, Unordered, vec![p("Next item empty", 165..180)]),
                list_item(186..190, 3, Unordered, vec![]),
                list_item(191..197, 1, Unordered, vec![p("Last", 193..197)]),
            ]
        );
    }

    #[gpui::test]
    async fn test_list_with_nested_content() {
        let parsed = parse(
            "\
*   This is a list item with two paragraphs.

    This is the second paragraph in the list item.
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![list_item(
                0..96,
                1,
                Unordered,
                vec![
                    p("This is a list item with two paragraphs.", 4..44),
                    p("This is the second paragraph in the list item.", 50..97)
                ],
            ),],
        );
    }

    #[gpui::test]
    async fn test_list_item_with_inline_html() {
        let parsed = parse(
            "\
*   This is a list item with an inline HTML <sometag>tag</sometag>.
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![list_item(
                0..67,
                1,
                Unordered,
                vec![p("This is a list item with an inline HTML tag.", 4..44),],
            ),],
        );
    }

    #[gpui::test]
    async fn test_nested_list_with_paragraph_inside() {
        let parsed = parse(
            "\
1. a
    1. b
        1. c

    text

    1. d
",
        )
        .await;

        assert_eq!(
            parsed.children,
            vec![
                list_item(0..7, 1, Ordered(1), vec![p("a", 3..4)],),
                list_item(8..20, 2, Ordered(1), vec![p("b", 12..13),],),
                list_item(21..27, 3, Ordered(1), vec![p("c", 25..26),],),
                p("text", 32..37),
                list_item(41..46, 2, Ordered(1), vec![p("d", 45..46),],),
            ],
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
            vec![
                list_item(0..8, 1, Unordered, vec![p("code", 2..8)]),
                list_item(9..19, 1, Unordered, vec![p("bold", 11..19)]),
                list_item(20..49, 1, Unordered, vec![p("link", 22..49)],),
            ],
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
                    h1(text("Heading", 4..11), 2..12),
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
                        block_quote(vec![h1(text("B", 12..13), 10..14)], 8..14),
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
            Some(tree_sitter_rust::LANGUAGE.into()),
        ))
    }

    fn h1(contents: MarkdownParagraph, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Heading(ParsedMarkdownHeading {
            source_range,
            level: HeadingLevel::H1,
            contents,
        })
    }

    fn h2(contents: MarkdownParagraph, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Heading(ParsedMarkdownHeading {
            source_range,
            level: HeadingLevel::H2,
            contents,
        })
    }

    fn h3(contents: MarkdownParagraph, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Heading(ParsedMarkdownHeading {
            source_range,
            level: HeadingLevel::H3,
            contents,
        })
    }

    fn p(contents: &str, source_range: Range<usize>) -> ParsedMarkdownElement {
        ParsedMarkdownElement::Paragraph(text(contents, source_range))
    }

    fn text(contents: &str, source_range: Range<usize>) -> MarkdownParagraph {
        vec![MarkdownParagraphChunk::Text(ParsedMarkdownText {
            highlights: Vec::new(),
            region_ranges: Vec::new(),
            regions: Vec::new(),
            source_range,
            contents: contents.to_string(),
        })]
    }

    fn block_quote(
        children: Vec<ParsedMarkdownElement>,
        source_range: Range<usize>,
    ) -> ParsedMarkdownElement {
        ParsedMarkdownElement::BlockQuote(ParsedMarkdownBlockQuote {
            source_range,
            children,
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

    fn list_item(
        source_range: Range<usize>,
        depth: u16,
        item_type: ParsedMarkdownListItemType,
        content: Vec<ParsedMarkdownElement>,
    ) -> ParsedMarkdownElement {
        ParsedMarkdownElement::ListItem(ParsedMarkdownListItem {
            source_range,
            item_type,
            depth,
            content,
        })
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

    fn row(children: Vec<MarkdownParagraph>) -> ParsedMarkdownTableRow {
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
