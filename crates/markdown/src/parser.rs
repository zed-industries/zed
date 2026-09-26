use collections::{BTreeMap, HashMap, HashSet};
use gpui::SharedString;
use linkify::LinkFinder;
pub use pulldown_cmark::TagEnd as MarkdownTagEnd;
use pulldown_cmark::{
    Alignment, CowStr, HeadingLevel, LinkType, MetadataBlockKind, Options, Parser,
};
use std::{ops::Range, sync::Arc};
use util::markdown::generate_heading_slug;

use crate::{html, path_range::PathWithRange};

pub const PARSE_OPTIONS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_FOOTNOTES)
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_TASKLISTS)
    .union(Options::ENABLE_SMART_PUNCTUATION)
    .union(Options::ENABLE_HEADING_ATTRIBUTES)
    .union(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS)
    .union(Options::ENABLE_OLD_FOOTNOTES)
    .union(Options::ENABLE_GFM)
    .union(Options::ENABLE_SUPERSCRIPT)
    .union(Options::ENABLE_SUBSCRIPT)
    .union(Options::ENABLE_MATH);

#[derive(Default)]
struct ParseState {
    events: Vec<(Range<usize>, MarkdownEvent)>,
    root_block_starts: Vec<usize>,
    depth: usize,
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct ParsedMarkdownData {
    pub events: Vec<(Range<usize>, MarkdownEvent)>,
    pub language_names: HashSet<SharedString>,
    pub language_paths: HashSet<Arc<str>>,
    pub root_block_starts: Vec<usize>,
    pub html_blocks: BTreeMap<usize, html::html_parser::ParsedHtmlBlock>,
    pub metadata_blocks: BTreeMap<usize, ParsedMetadataBlock>,
    pub heading_slugs: HashMap<SharedString, usize>,
    pub footnote_definitions: HashMap<SharedString, usize>,
    /// Source spans of link reference definitions (`[id]: https://example.com`), which are
    /// consumed by the parser and never appear in any event range.
    pub link_definition_spans: Vec<Range<usize>>,
    pub has_untagged_code_block: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParsedMetadataBlock {
    pub content_range: Range<usize>,
    pub rows: Option<Vec<MetadataRow>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MetadataRow {
    pub key: Range<usize>,
    pub value: Range<usize>,
}

impl ParseState {
    fn push_event(&mut self, range: Range<usize>, event: MarkdownEvent) {
        match &event {
            MarkdownEvent::Start(_) => {
                if self.depth == 0 {
                    self.root_block_starts.push(range.start);
                    self.events.push((range.clone(), MarkdownEvent::RootStart));
                }
                self.depth += 1;
                self.events.push((range, event));
            }
            MarkdownEvent::End(_) => {
                self.events.push((range.clone(), event));
                if self.depth > 0 {
                    self.depth -= 1;
                    if self.depth == 0 {
                        let root_block_index = self.root_block_starts.len() - 1;
                        self.events
                            .push((range, MarkdownEvent::RootEnd(root_block_index)));
                    }
                }
            }
            MarkdownEvent::Rule => {
                if self.depth == 0 && !range.is_empty() {
                    self.root_block_starts.push(range.start);
                    let root_block_index = self.root_block_starts.len() - 1;
                    self.events.push((range.clone(), MarkdownEvent::RootStart));
                    self.events.push((range.clone(), event));
                    self.events
                        .push((range, MarkdownEvent::RootEnd(root_block_index)));
                } else {
                    self.events.push((range, event));
                }
            }
            _ => {
                self.events.push((range, event));
            }
        }
    }
}

const MAX_DUPLICATE_HEADING_SLUGS: usize = 128;

fn build_heading_slugs(
    source: &str,
    events: &[(Range<usize>, MarkdownEvent)],
) -> HashMap<SharedString, usize> {
    let mut slugs = HashMap::default();
    let mut slug_counts: HashMap<String, usize> = HashMap::default();
    let mut inside_heading = false;
    let mut heading_text = String::new();
    let mut heading_source_start: Option<usize> = None;

    for (range, event) in events {
        match event {
            MarkdownEvent::Start(MarkdownTag::Heading { .. }) => {
                inside_heading = true;
                heading_text.clear();
                heading_source_start = None;
            }
            MarkdownEvent::End(MarkdownTagEnd::Heading(_)) => {
                if inside_heading {
                    let source_offset = heading_source_start.unwrap_or(range.start);
                    let base_slug = generate_heading_slug(&heading_text);
                    let count = slug_counts.entry(base_slug.clone()).or_insert(0);
                    let mut slug = if *count == 0 {
                        base_slug.clone()
                    } else {
                        format!("{base_slug}-{count}")
                    };
                    *count += 1;
                    while slugs.contains_key(slug.as_str()) {
                        let Some(count) = slug_counts.get_mut(&base_slug) else {
                            slug.clear();
                            break;
                        };
                        if *count >= MAX_DUPLICATE_HEADING_SLUGS {
                            slug.clear();
                            break;
                        }
                        slug = format!("{base_slug}-{count}");
                        *count += 1;
                    }
                    if !slug.is_empty() {
                        slugs.insert(SharedString::from(slug), source_offset);
                    }
                    inside_heading = false;
                }
            }
            MarkdownEvent::Text | MarkdownEvent::Code if inside_heading => {
                if heading_source_start.is_none() {
                    heading_source_start = Some(range.start);
                }
                heading_text.push_str(&source[range.clone()]);
            }
            MarkdownEvent::SubstitutedCode(substituted) if inside_heading => {
                if heading_source_start.is_none() {
                    heading_source_start = Some(range.start);
                }
                heading_text.push_str(substituted);
            }
            MarkdownEvent::SubstitutedText(substituted) if inside_heading => {
                if heading_source_start.is_none() {
                    heading_source_start = Some(range.start);
                }
                heading_text.push_str(substituted);
            }
            _ => {}
        }
    }

    slugs
}

fn parse_metadata_table_rows(source: &str, source_range: Range<usize>) -> Option<Vec<MetadataRow>> {
    let mut rows = Vec::new();
    let mut line_start = source_range.start;

    for line in source[source_range].split_inclusive('\n') {
        let line_end = line_start + line.len();
        let content_end = line_start + line.trim_end_matches(['\r', '\n']).len();
        let content_range = line_start..content_end;
        let line_text = &source[content_range.clone()];

        if line_text.is_empty()
            || line_text
                .chars()
                .next()
                .is_some_and(|character| character.is_whitespace())
        {
            return None;
        }

        let delimiter = line_text.find(':')?;
        let key = trim_metadata_range(source, content_range.start..content_range.start + delimiter);
        let value = trim_metadata_range(
            source,
            content_range.start + delimiter + 1..content_range.end,
        );
        if key.is_empty() || value.is_empty() {
            return None;
        }

        rows.push(MetadataRow { key, value });
        line_start = line_end;
    }

    if rows.is_empty() { None } else { Some(rows) }
}

fn trim_metadata_range(source: &str, range: Range<usize>) -> Range<usize> {
    let text = &source[range.clone()];
    let start_offset = text.len() - text.trim_start().len();
    let end_offset = text.trim_end().len();
    let start = range.start + start_offset;
    let end = (range.start + end_offset).max(start);
    start..end
}

fn is_br_tag(html: &str) -> bool {
    let Some(inner) = html
        .trim()
        .strip_prefix('<')
        .and_then(|s| s.strip_suffix('>'))
    else {
        return false;
    };
    let inner = inner.strip_suffix('/').unwrap_or(inner);
    inner
        .split_ascii_whitespace()
        .next()
        .is_some_and(|name| name.eq_ignore_ascii_case("br"))
}

/// Math is inline to pulldown-cmark, so a block-level line inside a `$$` span
/// ends the paragraph and drops the formula. Rewrites are byte-for-byte, which
/// keeps the ranges pulldown-cmark reports valid against the original source.
fn normalize_math_for_parsing(text: &str) -> Option<String> {
    if !text.contains('$') {
        return None;
    }
    let bytes = text.as_bytes();
    let mut flattened: Option<Vec<u8>> = None;
    let mut open_fence: Option<(u8, usize)> = None;
    let mut padding_swaps = Vec::new();
    let mut line_start = 0;
    while line_start < bytes.len() {
        let line_end = line_end_at(bytes, line_start);
        let indent = indent_width(&bytes[line_start..line_end]);
        let line = &bytes[line_start + indent..line_end];
        // Four spaces in is an indented code block.
        let is_block_start = indent <= 3;

        if let Some((fence_char, fence_len)) = open_fence {
            if is_block_start && closes_fence(line, fence_char, fence_len) {
                open_fence = None;
            }
        } else if is_block_start && let Some(fence) = opening_fence(line) {
            open_fence = Some(fence);
        } else if is_block_start && line.starts_with(b"$$") {
            if let Some(close_end) = find_display_math_close(bytes, line_start + indent + 2) {
                let span = line_start + indent..close_end;
                if bytes[span.clone()].contains(&b'\n')
                    && !contains_unescaped_percent(&bytes[span.clone()])
                {
                    let target = flattened.get_or_insert_with(|| bytes.to_vec());
                    for byte in &mut target[span] {
                        if *byte == b'\n' || *byte == b'\r' {
                            *byte = b' ';
                        }
                    }
                }
                line_start = line_end_at(bytes, close_end) + 1;
                continue;
            }
            inline_math_padding_swaps(bytes, line_start..line_end, &mut padding_swaps);
        } else {
            inline_math_padding_swaps(bytes, line_start..line_end, &mut padding_swaps);
        }
        line_start = line_end + 1;
    }

    if !padding_swaps.is_empty() {
        let target = flattened.get_or_insert_with(|| bytes.to_vec());
        for pair_start in padding_swaps {
            target.swap(pair_start, pair_start + 1);
        }
    }

    let flattened = flattened?;
    match String::from_utf8(flattened) {
        Ok(flattened) => Some(flattened),
        Err(error) => {
            // Unreachable: only ASCII was rewritten, and only into ASCII.
            log::error!("markdown: display math flattening broke UTF-8: {error}");
            None
        }
    }
}

fn line_end_at(bytes: &[u8], from: usize) -> usize {
    bytes[from..]
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(bytes.len(), |index| from + index)
}

fn indent_width(line: &[u8]) -> usize {
    line.iter().take_while(|byte| **byte == b' ').count()
}

fn opening_fence(line: &[u8]) -> Option<(u8, usize)> {
    let fence_char = *line.first()?;
    if fence_char != b'`' && fence_char != b'~' {
        return None;
    }
    let fence_len = line.iter().take_while(|byte| **byte == fence_char).count();
    if fence_len < 3 {
        return None;
    }
    // A backtick info string may not itself contain a backtick.
    if fence_char == b'`' && line[fence_len..].contains(&b'`') {
        return None;
    }
    Some((fence_char, fence_len))
}

fn closes_fence(line: &[u8], fence_char: u8, fence_len: usize) -> bool {
    let run = line.iter().take_while(|byte| **byte == fence_char).count();
    run >= fence_len && line[run..].iter().all(u8::is_ascii_whitespace)
}

/// Bails on an intervening code fence, so a fenced `$$` cannot close a span.
fn find_display_math_close(bytes: &[u8], search_start: usize) -> Option<usize> {
    let mut line_start = search_start;
    let mut cursor = search_start;
    loop {
        let line_end = line_end_at(bytes, line_start);
        if line_start > search_start {
            let indent = indent_width(&bytes[line_start..line_end]);
            if indent <= 3 && opening_fence(&bytes[line_start + indent..line_end]).is_some() {
                return None;
            }
        }
        while cursor + 1 < line_end {
            if bytes[cursor] == b'$' && bytes[cursor + 1] == b'$' && !is_escaped(bytes, cursor) {
                return Some(cursor + 2);
            }
            cursor += 1;
        }
        if line_end >= bytes.len() {
            return None;
        }
        line_start = line_end + 1;
        cursor = line_start;
    }
}

/// A LaTeX comment runs to end of line, so joining lines would comment out the
/// rest of the formula.
fn contains_unescaped_percent(span: &[u8]) -> bool {
    span.iter()
        .enumerate()
        .any(|(index, byte)| *byte == b'%' && !is_escaped(span, index))
}

/// LaTeX's `\\` row separator leaves an even count, so it escapes nothing.
fn is_escaped(bytes: &[u8], index: usize) -> bool {
    let mut backslashes = 0;
    let mut cursor = index;
    while cursor > 0 && bytes[cursor - 1] == b'\\' {
        backslashes += 1;
        cursor -= 1;
    }
    backslashes % 2 == 1
}

/// pulldown-cmark rejects padded inline math, so `$ \geq 0$` rendered as text.
/// Moving the padding outside the delimiters is byte-for-byte.
fn inline_math_padding_swaps(bytes: &[u8], line: Range<usize>, swaps: &mut Vec<usize>) {
    let mut index = line.start;
    while index < line.end {
        match bytes[index] {
            b'`' => index = skip_code_span(bytes, index, line.end),
            b'$' if !is_escaped(bytes, index) => {
                if bytes.get(index + 1) == Some(&b'$') {
                    index += 2;
                    continue;
                }
                let content_start = index + 1;
                match inline_math_close(bytes, content_start, line.end) {
                    Some(close) => {
                        if looks_like_formula(&bytes[content_start..close]) {
                            if bytes[content_start] == b' ' {
                                swaps.push(index);
                            }
                            if bytes[close - 1] == b' ' {
                                swaps.push(close - 1);
                            }
                        }
                        index = close + 1;
                    }
                    None => index += 1,
                }
            }
            _ => index += 1,
        }
    }
}

// Tells `$C(x_i) = $` apart from prose like `$ 5 and $ 10`.
const FORMULA_HINTS: &[u8] = br"\^_{}=+-<>()[]|/*";

fn looks_like_formula(content: &[u8]) -> bool {
    let Some(start) = content.iter().position(|byte| !byte.is_ascii_whitespace()) else {
        return false;
    };
    let end = content
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(start);
    let trimmed = &content[start..=end];
    trimmed.iter().any(|byte| FORMULA_HINTS.contains(byte))
        || !trimmed.iter().any(|byte| byte.is_ascii_whitespace())
}

/// Code spans are verbatim, so a byte moved inside one would be visible.
fn skip_code_span(bytes: &[u8], start: usize, end: usize) -> usize {
    let run = backtick_run(bytes, start, end);
    let mut cursor = start + run;
    while cursor < end {
        if bytes[cursor] == b'`' {
            let closing = backtick_run(bytes, cursor, end);
            if closing == run {
                return cursor + closing;
            }
            cursor += closing;
        } else {
            cursor += 1;
        }
    }
    end
}

fn backtick_run(bytes: &[u8], start: usize, end: usize) -> usize {
    bytes[start..end]
        .iter()
        .take_while(|byte| **byte == b'`')
        .count()
}

fn inline_math_close(bytes: &[u8], content_start: usize, end: usize) -> Option<usize> {
    let mut index = content_start;
    while index < end {
        if bytes[index] == b'$'
            && !is_escaped(bytes, index)
            && index > content_start
            && bytes.get(index + 1) != Some(&b'$')
        {
            return Some(index);
        }
        index += 1;
    }
    None
}

pub(crate) fn parse_markdown_with_options(
    text: &str,
    parse_html: bool,
    parse_heading_slugs: bool,
    parse_metadata_blocks: bool,
) -> ParsedMarkdownData {
    let mut state = ParseState::default();
    let mut language_names = HashSet::default();
    let mut language_paths = HashSet::default();
    let mut has_untagged_code_block = false;
    let mut html_blocks = BTreeMap::default();
    let mut metadata_blocks = BTreeMap::default();
    let mut within_link = false;
    let mut within_code_block = false;
    let mut within_metadata = false;
    let mut within_table = false;
    let mut current_metadata_block_start = None;
    let mut metadata_block_content_range: Option<Range<usize>> = None;
    let parse_options = if parse_metadata_blocks {
        PARSE_OPTIONS.union(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS)
    } else {
        PARSE_OPTIONS
    };
    let flattened_math = normalize_math_for_parsing(text);
    let parser = Parser::new_ext(flattened_math.as_deref().unwrap_or(text), parse_options);
    let mut link_definition_spans = parser
        .reference_definitions()
        .iter()
        .map(|(_, definition)| definition.span.clone())
        .collect::<Vec<_>>();
    link_definition_spans.sort_by_key(|span| span.start);
    let mut parser = parser.into_offset_iter().peekable();
    while let Some((pulldown_event, range)) = parser.next() {
        if within_metadata && !parse_metadata_blocks {
            if let pulldown_cmark::Event::End(pulldown_cmark::TagEnd::MetadataBlock(_)) =
                pulldown_event
            {
                within_metadata = false;
                current_metadata_block_start = None;
                metadata_block_content_range = None;
            }
            continue;
        }
        match pulldown_event {
            pulldown_cmark::Event::Start(tag) => {
                if let pulldown_cmark::Tag::HtmlBlock = &tag {
                    state.push_event(range.clone(), MarkdownEvent::Start(MarkdownTag::HtmlBlock));

                    if parse_html {
                        if let Some(block) =
                            html::html_parser::parse_html_block(&text[range.clone()], range.clone())
                        {
                            html_blocks.insert(range.start, block);

                            while let Some((event, end_range)) = parser.next() {
                                if let pulldown_cmark::Event::End(
                                    pulldown_cmark::TagEnd::HtmlBlock,
                                ) = event
                                {
                                    state.push_event(
                                        end_range,
                                        MarkdownEvent::End(MarkdownTagEnd::HtmlBlock),
                                    );
                                    break;
                                }
                            }
                        }
                    }
                    continue;
                }

                let tag = match tag {
                    pulldown_cmark::Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => {
                        within_link = true;
                        MarkdownTag::Link {
                            link_type,
                            dest_url: SharedString::from(dest_url.into_string()),
                            title: SharedString::from(title.into_string()),
                            id: SharedString::from(id.into_string()),
                        }
                    }
                    pulldown_cmark::Tag::MetadataBlock(kind) => {
                        within_metadata = true;
                        current_metadata_block_start = Some(range.start);
                        metadata_block_content_range = None;
                        if !parse_metadata_blocks {
                            continue;
                        }
                        MarkdownTag::MetadataBlock(kind)
                    }
                    pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Indented) => {
                        within_code_block = true;
                        MarkdownTag::CodeBlock {
                            kind: CodeBlockKind::Indented,
                            metadata: CodeBlockMetadata {
                                content_range: range.clone(),
                                line_count: 1,
                                is_fenced_closed: false,
                            },
                        }
                    }
                    pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(
                        ref info,
                    )) => {
                        within_code_block = true;
                        let code_block_source = &text[range.clone()];
                        let content_range = extract_code_block_content_range(code_block_source);
                        let is_fenced_closed = content_range.end < code_block_source.len();
                        let content_range =
                            content_range.start + range.start..content_range.end + range.start;

                        // Valid to use bytes since multi-byte UTF-8 doesn't use ASCII chars.
                        let line_count = text[content_range.clone()]
                            .bytes()
                            .filter(|c| *c == b'\n')
                            .count();

                        let metadata = CodeBlockMetadata {
                            content_range,
                            line_count,
                            is_fenced_closed,
                        };

                        let info = info.trim();
                        let kind = if info.is_empty() {
                            has_untagged_code_block = true;
                            CodeBlockKind::Fenced
                            // Languages should never contain a slash, and PathRanges always should.
                            // (Models are told to specify them relative to a workspace root.)
                        } else if info.contains('/') {
                            let path_range = PathWithRange::new(info);
                            language_paths.insert(path_range.path.clone());
                            CodeBlockKind::FencedSrc(path_range)
                        } else {
                            let language = SharedString::from(info.to_string());
                            language_names.insert(language.clone());
                            CodeBlockKind::FencedLang(language)
                        };

                        MarkdownTag::CodeBlock { kind, metadata }
                    }
                    pulldown_cmark::Tag::Paragraph => MarkdownTag::Paragraph,
                    pulldown_cmark::Tag::Heading {
                        level,
                        id,
                        classes,
                        attrs,
                    } => {
                        let id = id.map(|id| SharedString::from(id.into_string()));
                        let classes = classes
                            .into_iter()
                            .map(|c| SharedString::from(c.into_string()))
                            .collect();
                        let attrs = attrs
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    SharedString::from(key.into_string()),
                                    value.map(|v| SharedString::from(v.into_string())),
                                )
                            })
                            .collect();
                        MarkdownTag::Heading {
                            level,
                            id,
                            classes,
                            attrs,
                        }
                    }
                    pulldown_cmark::Tag::BlockQuote(kind) => MarkdownTag::BlockQuote(kind),
                    pulldown_cmark::Tag::List(start_number) => MarkdownTag::List(start_number),
                    pulldown_cmark::Tag::Item => MarkdownTag::Item,
                    pulldown_cmark::Tag::FootnoteDefinition(label) => {
                        MarkdownTag::FootnoteDefinition(SharedString::from(label.to_string()))
                    }
                    pulldown_cmark::Tag::Table(alignments) => {
                        within_table = true;
                        MarkdownTag::Table(alignments)
                    }
                    pulldown_cmark::Tag::TableHead => MarkdownTag::TableHead,
                    pulldown_cmark::Tag::TableRow => MarkdownTag::TableRow,
                    pulldown_cmark::Tag::TableCell => MarkdownTag::TableCell,
                    pulldown_cmark::Tag::Emphasis => MarkdownTag::Emphasis,
                    pulldown_cmark::Tag::Strong => MarkdownTag::Strong,
                    pulldown_cmark::Tag::Strikethrough => MarkdownTag::Strikethrough,
                    pulldown_cmark::Tag::Superscript => MarkdownTag::Superscript,
                    pulldown_cmark::Tag::Subscript => MarkdownTag::Subscript,
                    pulldown_cmark::Tag::Image {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => MarkdownTag::Image {
                        link_type,
                        dest_url: SharedString::from(dest_url.into_string()),
                        title: SharedString::from(title.into_string()),
                        id: SharedString::from(id.into_string()),
                    },
                    pulldown_cmark::Tag::HtmlBlock => MarkdownTag::HtmlBlock, // this is handled above separately
                    pulldown_cmark::Tag::DefinitionList => MarkdownTag::DefinitionList,
                    pulldown_cmark::Tag::DefinitionListTitle => MarkdownTag::DefinitionListTitle,
                    pulldown_cmark::Tag::DefinitionListDefinition => {
                        MarkdownTag::DefinitionListDefinition
                    }
                };
                state.push_event(range, MarkdownEvent::Start(tag))
            }
            pulldown_cmark::Event::End(tag) => {
                if let pulldown_cmark::TagEnd::Link = tag {
                    within_link = false;
                } else if let pulldown_cmark::TagEnd::CodeBlock = tag {
                    within_code_block = false;
                } else if let pulldown_cmark::TagEnd::MetadataBlock(_) = tag {
                    within_metadata = false;
                    let block_start = current_metadata_block_start.take();
                    let content_range = metadata_block_content_range.take();
                    if parse_metadata_blocks
                        && let (Some(block_start), Some(content_range)) =
                            (block_start, content_range)
                    {
                        metadata_blocks.insert(
                            block_start,
                            ParsedMetadataBlock {
                                rows: parse_metadata_table_rows(text, content_range.clone()),
                                content_range,
                            },
                        );
                    }
                    if !parse_metadata_blocks {
                        continue;
                    }
                } else if let pulldown_cmark::TagEnd::Table = tag {
                    within_table = false;
                }
                state.push_event(range, MarkdownEvent::End(tag));
            }
            pulldown_cmark::Event::Text(parsed) => {
                fn event_for(
                    text: &str,
                    range: Range<usize>,
                    str: &str,
                ) -> (Range<usize>, MarkdownEvent) {
                    if str == &text[range.clone()] {
                        (range, MarkdownEvent::Text)
                    } else {
                        (range, MarkdownEvent::SubstitutedText(str.to_owned()))
                    }
                }

                if within_metadata {
                    match &mut metadata_block_content_range {
                        Some(content_range) => {
                            content_range.start = content_range.start.min(range.start);
                            content_range.end = content_range.end.max(range.end);
                        }
                        None => metadata_block_content_range = Some(range.clone()),
                    }
                    state.push_event(range, MarkdownEvent::Text);
                    continue;
                }

                if within_code_block {
                    let (range, event) = event_for(text, range, &parsed);
                    state.push_event(range, event);
                    continue;
                }

                #[derive(Debug)]
                struct TextRange<'a> {
                    source_range: Range<usize>,
                    merged_range: Range<usize>,
                    parsed: CowStr<'a>,
                }

                let mut last_len = parsed.len();
                let mut ranges = vec![TextRange {
                    source_range: range.clone(),
                    merged_range: 0..last_len,
                    parsed,
                }];

                while match parser.peek() {
                    Some((pulldown_cmark::Event::Text(_), _)) => true,
                    Some((pulldown_cmark::Event::InlineHtml(html), _)) => {
                        parse_html && !is_br_tag(html)
                    }
                    _ => false,
                } {
                    let Some((next_event, next_range)) = parser.next() else {
                        unreachable!()
                    };
                    let next_text = match next_event {
                        pulldown_cmark::Event::Text(next_event) => next_event,
                        pulldown_cmark::Event::InlineHtml(_) => CowStr::Borrowed(""),
                        _ => unreachable!(),
                    };
                    let next_len = last_len + next_text.len();
                    ranges.push(TextRange {
                        source_range: next_range.clone(),
                        merged_range: last_len..next_len,
                        parsed: next_text,
                    });
                    last_len = next_len;
                }

                let mut merged_text =
                    String::with_capacity(ranges.last().unwrap().merged_range.end);
                for range in &ranges {
                    merged_text.push_str(&range.parsed);
                }

                let mut ranges = ranges.into_iter().peekable();

                if !within_link && !within_code_block {
                    let mut finder = LinkFinder::new();
                    finder.kinds(&[linkify::LinkKind::Url]);

                    // Find links in the merged text
                    for link in finder.links(&merged_text) {
                        let link_start_in_merged = link.start();
                        let link_end_in_merged = link.end();

                        while ranges
                            .peek()
                            .is_some_and(|range| range.merged_range.end <= link_start_in_merged)
                        {
                            let range = ranges.next().unwrap();
                            let (range, event) = event_for(text, range.source_range, &range.parsed);
                            state.push_event(range, event);
                        }

                        let Some(range) = ranges.peek_mut() else {
                            continue;
                        };
                        let prefix_len = link_start_in_merged - range.merged_range.start;
                        if prefix_len > 0 {
                            let (head, tail) = range.parsed.split_at(prefix_len);
                            let (event_range, event) = event_for(
                                text,
                                range.source_range.start..range.source_range.start + prefix_len,
                                head,
                            );
                            state.push_event(event_range, event);
                            range.parsed = CowStr::Boxed(tail.into());
                            range.merged_range.start += prefix_len;
                            range.source_range.start += prefix_len;
                        }

                        let link_start_in_source = range.source_range.start;
                        let mut link_end_in_source = range.source_range.end;
                        let mut link_events = Vec::new();

                        while ranges
                            .peek()
                            .is_some_and(|range| range.merged_range.end <= link_end_in_merged)
                        {
                            let range = ranges.next().unwrap();
                            link_end_in_source = range.source_range.end;
                            link_events.push(event_for(text, range.source_range, &range.parsed));
                        }

                        if let Some(range) = ranges.peek_mut() {
                            let prefix_len = link_end_in_merged - range.merged_range.start;
                            if prefix_len > 0 {
                                let (head, tail) = range.parsed.split_at(prefix_len);
                                link_events.push(event_for(
                                    text,
                                    range.source_range.start..range.source_range.start + prefix_len,
                                    head,
                                ));
                                range.parsed = CowStr::Boxed(tail.into());
                                range.merged_range.start += prefix_len;
                                range.source_range.start += prefix_len;
                                link_end_in_source = range.source_range.start;
                            }
                        }
                        let link_range = link_start_in_source..link_end_in_source;

                        state.push_event(
                            link_range.clone(),
                            MarkdownEvent::Start(MarkdownTag::Link {
                                link_type: LinkType::Autolink,
                                dest_url: SharedString::from(link.as_str().to_string()),
                                title: SharedString::default(),
                                id: SharedString::default(),
                            }),
                        );
                        for (range, event) in link_events {
                            state.push_event(range, event);
                        }
                        state.push_event(
                            link_range.clone(),
                            MarkdownEvent::End(MarkdownTagEnd::Link),
                        );
                    }
                }

                for range in ranges {
                    let (range, event) = event_for(text, range.source_range, &range.parsed);
                    state.push_event(range, event);
                }
            }
            pulldown_cmark::Event::Code(parsed) => {
                let content_range = extract_code_content_range(&text[range.clone()]);
                let content_range =
                    content_range.start + range.start..content_range.end + range.start;
                let source = &text[content_range.clone()];
                let event = if within_table && source.contains(r"\|") {
                    MarkdownEvent::SubstitutedCode(parsed.to_string())
                } else {
                    MarkdownEvent::Code
                };
                state.push_event(content_range, event)
            }
            pulldown_cmark::Event::Html(_) => state.push_event(range, MarkdownEvent::Html),
            pulldown_cmark::Event::InlineHtml(html) => {
                if parse_html && is_br_tag(&html) {
                    state.push_event(range, MarkdownEvent::HardBreak)
                } else {
                    state.push_event(range, MarkdownEvent::InlineHtml)
                }
            }
            pulldown_cmark::Event::FootnoteReference(label) => state.push_event(
                range,
                MarkdownEvent::FootnoteReference(SharedString::from(label.to_string())),
            ),
            pulldown_cmark::Event::SoftBreak => state.push_event(range, MarkdownEvent::SoftBreak),
            pulldown_cmark::Event::HardBreak => state.push_event(range, MarkdownEvent::HardBreak),
            pulldown_cmark::Event::Rule => state.push_event(range, MarkdownEvent::Rule),
            pulldown_cmark::Event::TaskListMarker(checked) => {
                state.push_event(range, MarkdownEvent::TaskListMarker(checked))
            }
            pulldown_cmark::Event::InlineMath(content) => state.push_event(
                range,
                MarkdownEvent::Math {
                    display: false,
                    content: SharedString::from(content.into_string()),
                },
            ),
            pulldown_cmark::Event::DisplayMath(content) => state.push_event(
                range,
                MarkdownEvent::Math {
                    display: true,
                    content: SharedString::from(content.into_string()),
                },
            ),
        }
    }

    let heading_slugs = if parse_heading_slugs {
        build_heading_slugs(text, &state.events)
    } else {
        HashMap::default()
    };
    let footnote_definitions = build_footnote_definitions(&state.events);

    ParsedMarkdownData {
        events: state.events,
        language_names,
        language_paths,
        root_block_starts: state.root_block_starts,
        html_blocks,
        metadata_blocks,
        heading_slugs,
        footnote_definitions,
        link_definition_spans,
        has_untagged_code_block,
    }
}

fn build_footnote_definitions(
    events: &[(Range<usize>, MarkdownEvent)],
) -> HashMap<SharedString, usize> {
    let mut definitions = HashMap::default();
    let mut current_label: Option<SharedString> = None;

    for (range, event) in events {
        match event {
            MarkdownEvent::Start(MarkdownTag::FootnoteDefinition(label)) => {
                current_label = Some(label.clone());
            }
            MarkdownEvent::End(MarkdownTagEnd::FootnoteDefinition) => {
                current_label = None;
            }
            MarkdownEvent::Text if current_label.is_some() => {
                if let Some(label) = current_label.take() {
                    definitions.entry(label).or_insert(range.start);
                }
            }
            _ => {}
        }
    }

    definitions
}

pub fn parse_links_only(text: &str) -> Vec<(Range<usize>, MarkdownEvent)> {
    let mut events = Vec::new();
    let mut finder = LinkFinder::new();
    finder.kinds(&[linkify::LinkKind::Url]);
    let mut text_range = Range {
        start: 0,
        end: text.len(),
    };
    for link in finder.links(text) {
        let link_range = link.start()..link.end();

        if link_range.start > text_range.start {
            events.push((text_range.start..link_range.start, MarkdownEvent::Text));
        }

        events.push((
            link_range.clone(),
            MarkdownEvent::Start(MarkdownTag::Link {
                link_type: LinkType::Autolink,
                dest_url: SharedString::from(link.as_str().to_string()),
                title: SharedString::default(),
                id: SharedString::default(),
            }),
        ));
        events.push((link_range.clone(), MarkdownEvent::Text));
        events.push((link_range.clone(), MarkdownEvent::End(MarkdownTagEnd::Link)));

        text_range.start = link_range.end;
    }

    if text_range.end > text_range.start {
        events.push((text_range, MarkdownEvent::Text));
    }

    events
}

/// A static-lifetime equivalent of pulldown_cmark::Event so we can cache the
/// parse result for rendering without resorting to unsafe lifetime coercion.
#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownEvent {
    /// Start of a tagged element. Events that are yielded after this event
    /// and before its corresponding `End` event are inside this element.
    /// Start and end events are guaranteed to be balanced.
    Start(MarkdownTag),
    /// End of a tagged element.
    End(MarkdownTagEnd),
    /// Text that uses the associated range from the markdown source.
    Text,
    /// Text that differs from the markdown source - typically due to substitution of HTML entities
    /// and smart punctuation.
    SubstitutedText(String),
    /// An inline code node.
    Code,
    /// An inline code node that differs from the markdown source due to escape decoding.
    SubstitutedCode(String),
    /// An HTML node.
    Html,
    /// An inline HTML node.
    InlineHtml,
    /// A reference to a footnote with given label, which may or may not be defined
    /// by an event with a `Tag::FootnoteDefinition` tag. Definitions and references to them may
    /// occur in any order.
    FootnoteReference(SharedString),
    /// A soft line break.
    SoftBreak,
    /// A hard line break.
    HardBreak,
    /// A horizontal ruler.
    Rule,
    /// A task list marker, rendered as a checkbox in HTML. Contains a true when it is checked.
    TaskListMarker(bool),
    /// A LaTeX math expression. `display: true` indicates block-level (`$$...$$`) math;
    /// `display: false` indicates inline (`$...$`) math. `content` is the formula text
    /// with delimiters stripped (as yielded by pulldown_cmark).
    Math {
        display: bool,
        content: SharedString,
    },
    /// Start of a root-level block (a top-level structural element like a paragraph, heading, list, etc.).
    RootStart,
    /// End of a root-level block. Contains the root block index.
    RootEnd(usize),
}

/// Tags for elements that can contain other elements.
#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownTag {
    /// A paragraph of text and other inline elements.
    Paragraph,

    /// A heading, with optional identifier, classes and custom attributes.
    /// The identifier is prefixed with `#` and the last one in the attributes
    /// list is chosen, classes are prefixed with `.` and custom attributes
    /// have no prefix and can optionally have a value (`myattr` o `myattr=myvalue`).
    Heading {
        level: HeadingLevel,
        id: Option<SharedString>,
        classes: Vec<SharedString>,
        /// The first item of the tuple is the attr and second one the value.
        attrs: Vec<(SharedString, Option<SharedString>)>,
    },

    BlockQuote(Option<pulldown_cmark::BlockQuoteKind>),

    /// A code block.
    CodeBlock {
        kind: CodeBlockKind,
        metadata: CodeBlockMetadata,
    },

    /// A HTML block.
    HtmlBlock,

    /// A list. If the list is ordered the field indicates the number of the first item.
    /// Contains only list items.
    List(Option<u64>), // TODO: add delim and tight for ast (not needed for html)

    /// A list item.
    Item,

    /// A footnote definition. The value contained is the footnote's label by which it can
    /// be referred to.
    FootnoteDefinition(SharedString),

    /// A table. Contains a vector describing the text-alignment for each of its columns.
    Table(Vec<Alignment>),

    /// A table header. Contains only `TableCell`s. Note that the table body starts immediately
    /// after the closure of the `TableHead` tag. There is no `TableBody` tag.
    TableHead,

    /// A table row. Is used both for header rows as body rows. Contains only `TableCell`s.
    TableRow,
    TableCell,

    // span-level tags
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,

    /// A link.
    Link {
        link_type: LinkType,
        dest_url: SharedString,
        title: SharedString,
        /// Identifier of reference links, e.g. `world` in the link `[hello][world]`.
        id: SharedString,
    },

    /// An image. The first field is the link type, the second the destination URL and the third is a title,
    /// the fourth is the link identifier.
    Image {
        link_type: LinkType,
        dest_url: SharedString,
        title: SharedString,
        /// Identifier of reference links, e.g. `world` in the link `[hello][world]`.
        id: SharedString,
    },

    /// A metadata block.
    MetadataBlock(MetadataBlockKind),

    DefinitionList,
    DefinitionListTitle,
    DefinitionListDefinition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CodeBlockKind {
    Indented,
    /// "Fenced" means "surrounded by triple backticks."
    /// There can optionally be either a language after the backticks (like in traditional Markdown)
    /// or, if an agent is specifying a path for a source location in the project, it can be a PathRange,
    /// e.g. ```path/to/foo.rs#L123-456 instead of ```rust
    Fenced,
    FencedLang(SharedString),
    FencedSrc(PathWithRange),
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct CodeBlockMetadata {
    pub content_range: Range<usize>,
    pub line_count: usize,
    pub is_fenced_closed: bool,
}

fn extract_code_content_range(text: &str) -> Range<usize> {
    let text_len = text.len();
    if text_len == 0 {
        return 0..0;
    }

    let start_ticks = text.chars().take_while(|&c| c == '`').count();

    if start_ticks == 0 || start_ticks > text_len {
        return 0..text_len;
    }

    let end_ticks = text.chars().rev().take_while(|&c| c == '`').count();

    if end_ticks != start_ticks || text_len < start_ticks + end_ticks {
        return 0..text_len;
    }

    start_ticks..text_len - end_ticks
}

pub(crate) fn extract_code_block_content_range(text: &str) -> Range<usize> {
    let mut range = 0..text.len();
    let Some(fence_character) = text
        .as_bytes()
        .first()
        .copied()
        .filter(|character| matches!(character, b'`' | b'~'))
    else {
        return range;
    };
    let opening_fence_len = text
        .bytes()
        .take_while(|character| *character == fence_character)
        .count();
    if opening_fence_len < 3 {
        return range;
    }

    range.start += opening_fence_len;
    if let Some(newline_ix) = text[range.clone()].find('\n') {
        range.start += newline_ix + 1;
    }

    let text_without_line_ending =
        text.trim_end_matches(|character| matches!(character, '\r' | '\n'));
    let closing_line_start = text_without_line_ending
        .rfind('\n')
        .map_or(0, |newline_ix| newline_ix + 1);
    if closing_line_start >= range.start {
        let closing_line = &text_without_line_ending[closing_line_start..];
        let closing_fence = closing_line.trim_start_matches(' ');
        let indentation_len = closing_line.len() - closing_fence.len();
        let closing_fence_len = closing_fence
            .bytes()
            .take_while(|character| *character == fence_character)
            .count();
        let trailing_characters = &closing_fence[closing_fence_len..];
        if indentation_len <= 3
            && closing_fence_len >= opening_fence_len
            && trailing_characters
                .bytes()
                .all(|character| matches!(character, b' ' | b'\t'))
        {
            range.end = closing_line_start;
        }
    }

    if range.start > range.end {
        range.end = range.start;
    }
    range
}

#[cfg(test)]
mod tests {
    use super::MarkdownEvent::*;
    use super::MarkdownTag::*;
    use super::*;

    const CONDITIONAL_OPTIONS: Options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;
    const UNWANTED_OPTIONS: Options = Options::empty()
        .union(Options::ENABLE_DEFINITION_LIST)
        .union(Options::ENABLE_WIKILINKS);

    #[test]
    fn all_options_considered() {
        // The purpose of this is to fail when new options are added to pulldown_cmark, so that they
        // can be evaluated for inclusion.
        assert_eq!(
            PARSE_OPTIONS
                .union(CONDITIONAL_OPTIONS)
                .union(UNWANTED_OPTIONS),
            Options::all()
        );
    }

    #[test]
    fn wanted_and_unwanted_options_disjoint() {
        assert_eq!(
            PARSE_OPTIONS
                .union(CONDITIONAL_OPTIONS)
                .intersection(UNWANTED_OPTIONS),
            Options::empty()
        );
    }

    #[test]
    fn test_yaml_style_metadata_block() {
        assert_eq!(
            parse_markdown_with_options("---\ntitle: Post\n---\n# Heading", false, false, true),
            ParsedMarkdownData {
                events: vec![
                    (0..19, RootStart),
                    (0..19, Start(MetadataBlock(MetadataBlockKind::YamlStyle))),
                    (4..16, Text),
                    (
                        0..19,
                        End(MarkdownTagEnd::MetadataBlock(MetadataBlockKind::YamlStyle))
                    ),
                    (0..19, RootEnd(0)),
                    (20..29, RootStart),
                    (
                        20..29,
                        Start(Heading {
                            level: HeadingLevel::H1,
                            id: None,
                            classes: Vec::new(),
                            attrs: Vec::new(),
                        })
                    ),
                    (22..29, Text),
                    (20..29, End(MarkdownTagEnd::Heading(HeadingLevel::H1))),
                    (20..29, RootEnd(1)),
                ],
                root_block_starts: vec![0, 20],
                metadata_blocks: BTreeMap::from_iter([(
                    0,
                    ParsedMetadataBlock {
                        content_range: 4..16,
                        rows: Some(vec![MetadataRow {
                            key: 4..9,
                            value: 11..15,
                        }]),
                    },
                )]),
                ..Default::default()
            }
        )
    }

    #[test]
    fn test_metadata_block_text_is_verbatim() {
        let parsed =
            parse_markdown_with_options("---\nurl: https://zed.dev\n---\nBody", false, false, true);
        assert!(
            parsed
                .events
                .iter()
                .all(|(_, event)| !matches!(event, Start(Link { .. })))
        );
    }

    #[test]
    fn test_metadata_blocks_store_table_rows() {
        let parsed = parse_markdown_with_options(
            "---\ntitle: Post\nauthor: Zed\n---\nBody",
            false,
            false,
            true,
        );

        assert_eq!(
            parsed.metadata_blocks,
            BTreeMap::from_iter([(
                0,
                ParsedMetadataBlock {
                    content_range: 4..28,
                    rows: Some(vec![
                        MetadataRow {
                            key: 4..9,
                            value: 11..15,
                        },
                        MetadataRow {
                            key: 16..22,
                            value: 24..27,
                        },
                    ]),
                },
            )])
        );
    }

    #[test]
    fn test_metadata_blocks_store_fallback_for_nested_yaml() {
        let parsed =
            parse_markdown_with_options("---\ntags:\n  - zed\n---\nBody", false, false, true);

        assert_eq!(
            parsed.metadata_blocks,
            BTreeMap::from_iter([(
                0,
                ParsedMetadataBlock {
                    content_range: 4..18,
                    rows: None,
                },
            )])
        );
    }

    #[test]
    fn test_metadata_table_rows_parse_simple_colon_pairs() {
        let source = "title: Post\nauthor: Zed\n";
        let Some(rows) = parse_metadata_table_rows(source, 0..source.len()) else {
            panic!("expected metadata rows");
        };
        let pairs = rows
            .into_iter()
            .map(|row| (&source[row.key], &source[row.value]))
            .collect::<Vec<_>>();

        assert_eq!(pairs, vec![("title", "Post"), ("author", "Zed")]);
    }

    #[test]
    fn test_metadata_table_rows_reject_non_simple_colon_pairs() {
        for source in [
            "tags:\n  - zed\n",
            "title = Post\n",
            "title:\n",
            "title:   \n",
            ": Post\n",
            " title: Post\n",
            "\n",
        ] {
            assert!(parse_metadata_table_rows(source, 0..source.len()).is_none());
        }
    }

    #[test]
    fn test_trim_metadata_range_returns_valid_empty_range() {
        let source = "key:   \n";
        let trimmed = trim_metadata_range(source, 4..7);

        assert_eq!(trimmed, 7..7);
        assert!(source[trimmed].is_empty());
    }

    #[test]
    fn test_html_comments() {
        assert_eq!(
            parse_markdown_with_options(
                "  <!--\nrdoc-file=string.c\n-->\nReturns",
                false,
                false,
                false
            ),
            ParsedMarkdownData {
                events: vec![
                    (2..30, RootStart),
                    (2..30, Start(HtmlBlock)),
                    (2..2, SubstitutedText("  ".into())),
                    (2..7, Html),
                    (7..26, Html),
                    (26..30, Html),
                    (2..30, End(MarkdownTagEnd::HtmlBlock)),
                    (2..30, RootEnd(0)),
                    (30..37, RootStart),
                    (30..37, Start(Paragraph)),
                    (30..37, Text),
                    (30..37, End(MarkdownTagEnd::Paragraph)),
                    (30..37, RootEnd(1)),
                ],
                root_block_starts: vec![2, 30],
                ..Default::default()
            }
        )
    }

    #[test]
    fn test_plain_urls_and_escaped_text() {
        assert_eq!(
            parse_markdown_with_options(
                "&nbsp;&nbsp; https://some.url some \\`&#9658;\\` text",
                false,
                false,
                false,
            ),
            ParsedMarkdownData {
                events: vec![
                    (0..51, RootStart),
                    (0..51, Start(Paragraph)),
                    (0..6, SubstitutedText("\u{a0}".into())),
                    (6..12, SubstitutedText("\u{a0}".into())),
                    (12..13, Text),
                    (
                        13..29,
                        Start(Link {
                            link_type: LinkType::Autolink,
                            dest_url: "https://some.url".into(),
                            title: "".into(),
                            id: "".into(),
                        })
                    ),
                    (13..29, Text),
                    (13..29, End(MarkdownTagEnd::Link)),
                    (29..35, Text),
                    (36..37, Text), // Escaped backtick
                    (37..44, SubstitutedText("►".into())),
                    (45..46, Text), // Escaped backtick
                    (46..51, Text),
                    (0..51, End(MarkdownTagEnd::Paragraph)),
                    (0..51, RootEnd(0)),
                ],
                root_block_starts: vec![0],
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_incomplete_link() {
        assert_eq!(
            parse_markdown_with_options(
                "You can use the [GitHub Search API](https://docs.github.com/en",
                false,
                false,
                false,
            )
            .events,
            vec![
                (0..62, RootStart),
                (0..62, Start(Paragraph)),
                (0..16, Text),
                (16..17, Text),
                (17..34, Text),
                (34..35, Text),
                (35..36, Text),
                (
                    36..62,
                    Start(Link {
                        link_type: LinkType::Autolink,
                        dest_url: "https://docs.github.com/en".into(),
                        title: "".into(),
                        id: "".into()
                    })
                ),
                (36..62, Text),
                (36..62, End(MarkdownTagEnd::Link)),
                (0..62, End(MarkdownTagEnd::Paragraph)),
                (0..62, RootEnd(0)),
            ],
        );
    }

    #[test]
    fn test_smart_punctuation() {
        assert_eq!(
            parse_markdown_with_options(
                "-- --- ... \"double quoted\" 'single quoted' ----------",
                false,
                false,
                false,
            ),
            ParsedMarkdownData {
                events: vec![
                    (0..53, RootStart),
                    (0..53, Start(Paragraph)),
                    (0..2, SubstitutedText("–".into())),
                    (2..3, Text),
                    (3..6, SubstitutedText("—".into())),
                    (6..7, Text),
                    (7..10, SubstitutedText("…".into())),
                    (10..11, Text),
                    (11..12, SubstitutedText("\u{201c}".into())),
                    (12..25, Text),
                    (25..26, SubstitutedText("\u{201d}".into())),
                    (26..27, Text),
                    (27..28, SubstitutedText("\u{2018}".into())),
                    (28..41, Text),
                    (41..42, SubstitutedText("\u{2019}".into())),
                    (42..43, Text),
                    (43..53, SubstitutedText("–––––".into())),
                    (0..53, End(MarkdownTagEnd::Paragraph)),
                    (0..53, RootEnd(0)),
                ],
                root_block_starts: vec![0],
                ..Default::default()
            }
        )
    }

    #[test]
    fn test_code_block_metadata() {
        assert_eq!(
            parse_markdown_with_options(
                "```rust\nfn main() {\n let a = 1;\n}\n```",
                false,
                false,
                false
            ),
            ParsedMarkdownData {
                events: vec![
                    (0..37, RootStart),
                    (
                        0..37,
                        Start(CodeBlock {
                            kind: CodeBlockKind::FencedLang("rust".into()),
                            metadata: CodeBlockMetadata {
                                content_range: 8..34,
                                line_count: 3,
                                is_fenced_closed: true,
                            }
                        })
                    ),
                    (8..34, Text),
                    (0..37, End(MarkdownTagEnd::CodeBlock)),
                    (0..37, RootEnd(0)),
                ],
                language_names: {
                    let mut h = HashSet::default();
                    h.insert("rust".into());
                    h
                },
                root_block_starts: vec![0],
                ..Default::default()
            }
        );
        assert_eq!(
            parse_markdown_with_options("    fn main() {}", false, false, false),
            ParsedMarkdownData {
                events: vec![
                    (4..16, RootStart),
                    (
                        4..16,
                        Start(CodeBlock {
                            kind: CodeBlockKind::Indented,
                            metadata: CodeBlockMetadata {
                                content_range: 4..16,
                                line_count: 1,
                                is_fenced_closed: false,
                            }
                        })
                    ),
                    (4..16, Text),
                    (4..16, End(MarkdownTagEnd::CodeBlock)),
                    (4..16, RootEnd(0)),
                ],
                root_block_starts: vec![4],
                ..Default::default()
            }
        );

        for markdown in ["```mermaid\ngraph TD;\n~~~", "~~~~mermaid\ngraph TD;\n~~~"] {
            let parsed = parse_markdown_with_options(markdown, false, false, false);
            let metadata = parsed.events.iter().find_map(|(_, event)| match event {
                Start(CodeBlock { metadata, .. }) => Some(metadata),
                _ => None,
            });
            assert_eq!(
                metadata.map(|metadata| metadata.is_fenced_closed),
                Some(false)
            );
        }
    }

    fn assert_code_block_does_not_emit_links(markdown: &str) {
        let parsed = parse_markdown_with_options(markdown, false, false, false);
        let mut code_block_depth = 0;
        let mut code_block_count = 0;
        let mut saw_text_inside_code_block = false;

        for (_, event) in &parsed.events {
            match event {
                Start(CodeBlock { .. }) => {
                    code_block_depth += 1;
                    code_block_count += 1;
                }
                End(MarkdownTagEnd::CodeBlock) => {
                    assert!(
                        code_block_depth > 0,
                        "encountered a code block end without a matching start"
                    );
                    code_block_depth -= 1;
                }
                Start(Link { .. }) | End(MarkdownTagEnd::Link) => {
                    assert_eq!(
                        code_block_depth, 0,
                        "code blocks should not emit link events"
                    );
                }
                Text | SubstitutedText(_) if code_block_depth > 0 => {
                    saw_text_inside_code_block = true;
                }
                _ => {}
            }
        }

        assert_eq!(code_block_count, 1, "expected exactly one code block");
        assert_eq!(code_block_depth, 0, "unterminated code block");
        assert!(
            saw_text_inside_code_block,
            "expected text inside the code block"
        );
    }

    #[test]
    fn test_code_blocks_do_not_autolink_urls() {
        assert_code_block_does_not_emit_links("```txt\nhttps://example.com\n```");
        assert_code_block_does_not_emit_links("    https://example.com");
        assert_code_block_does_not_emit_links(
            "```txt\r\nhttps:/\\/example.com\r\nhttps://example&#46;com\r\n```",
        );
        assert_code_block_does_not_emit_links(
            "    https:/\\/example.com\r\n    https://example&#46;com",
        );
    }

    #[test]
    fn test_metadata_blocks_are_root_blocks() {
        assert_eq!(
            parse_markdown_with_options(
                "+++\ntitle = \"Example\"\n+++\n\nParagraph",
                false,
                false,
                true
            ),
            ParsedMarkdownData {
                events: vec![
                    (0..25, RootStart),
                    (0..25, Start(MetadataBlock(MetadataBlockKind::PlusesStyle))),
                    (4..22, Text),
                    (
                        0..25,
                        End(MarkdownTagEnd::MetadataBlock(
                            MetadataBlockKind::PlusesStyle
                        ))
                    ),
                    (0..25, RootEnd(0)),
                    (27..36, RootStart),
                    (27..36, Start(Paragraph)),
                    (27..36, Text),
                    (27..36, End(MarkdownTagEnd::Paragraph)),
                    (27..36, RootEnd(1)),
                ],
                root_block_starts: vec![0, 27],
                metadata_blocks: BTreeMap::from_iter([(
                    0,
                    ParsedMetadataBlock {
                        content_range: 4..22,
                        rows: None,
                    },
                )]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_metadata_blocks_are_omitted_by_default() {
        assert_eq!(
            parse_markdown_with_options(
                "+++\ntitle = \"Example\"\n+++\n\nParagraph",
                false,
                false,
                false
            ),
            ParsedMarkdownData {
                events: vec![
                    (27..36, RootStart),
                    (27..36, Start(Paragraph)),
                    (27..36, Text),
                    (27..36, End(MarkdownTagEnd::Paragraph)),
                    (27..36, RootEnd(0)),
                ],
                root_block_starts: vec![27],
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_table_checkboxes_remain_text_in_cells() {
        let markdown = "\
| Done | Task    |
|------|---------|
| [x]  | Fix bug |
| [ ]  | Add feature |";
        let parsed = parse_markdown_with_options(markdown, false, false, false);

        let mut in_table = false;
        let mut saw_task_list_marker = false;
        let mut cell_texts = Vec::new();
        let mut current_cell = String::new();

        for (range, event) in &parsed.events {
            match event {
                Start(Table(_)) => in_table = true,
                End(MarkdownTagEnd::Table) => in_table = false,
                Start(TableCell) => current_cell.clear(),
                End(MarkdownTagEnd::TableCell) => {
                    if in_table {
                        cell_texts.push(current_cell.clone());
                    }
                }
                Text if in_table => current_cell.push_str(&markdown[range.clone()]),
                TaskListMarker(_) if in_table => saw_task_list_marker = true,
                _ => {}
            }
        }

        let checkbox_cells: Vec<&str> = cell_texts
            .iter()
            .map(|cell| cell.trim())
            .filter(|cell| *cell == "[x]" || *cell == "[X]" || *cell == "[ ]")
            .collect();

        assert!(
            !saw_task_list_marker,
            "Table checkboxes should remain text, not task-list markers"
        );
        assert_eq!(checkbox_cells, vec!["[x]", "[ ]"]);
    }

    #[test]
    fn test_extract_code_content_range() {
        let input = "```let x = 5;```";
        assert_eq!(extract_code_content_range(input), 3..13);

        let input = "``let x = 5;``";
        assert_eq!(extract_code_content_range(input), 2..12);

        let input = "`let x = 5;`";
        assert_eq!(extract_code_content_range(input), 1..11);

        let input = "plain text";
        assert_eq!(extract_code_content_range(input), 0..10);

        let input = "``let x = 5;`";
        assert_eq!(extract_code_content_range(input), 0..13);
    }

    #[test]
    fn test_inline_code_substitutes_escaped_pipes() {
        let markdown = r"| Pattern |
| --- |
| `a\|b` |";
        let parsed = parse_markdown_with_options(markdown, false, false, false);
        let code_range = {
            let start = markdown.find(r"a\|b").expect("inline code source");
            start..start + r"a\|b".len()
        };

        assert!(
            parsed
                .events
                .iter()
                .any(|(range, event)| range == &code_range
                    && event == &SubstitutedCode("a|b".into())),
            "expected escaped pipe in table inline code to render as decoded inline code: {:?}",
            parsed.events
        );
    }

    #[test]
    fn test_inline_code_keeps_escaped_pipes_outside_tables() {
        let markdown = r"`a\|b`";
        let parsed = parse_markdown_with_options(markdown, false, false, false);

        assert!(
            parsed
                .events
                .iter()
                .any(|(range, event)| range == &(1..5) && event == &Code),
            "expected escaped pipe outside a table to remain normal inline code: {:?}",
            parsed.events
        );
    }

    #[test]
    fn test_extract_code_block_content_range() {
        let input = "```rust\nlet x = 5;\n```";
        assert_eq!(extract_code_block_content_range(input), 8..19);

        let input = "plain text";
        assert_eq!(extract_code_block_content_range(input), 0..10);

        let input = "```python\nprint('hello')\nprint('world')\n```";
        assert_eq!(extract_code_block_content_range(input), 10..40);

        let input = "~~~~mermaid\ngraph TD;\n~~~~";
        let content_range = extract_code_block_content_range(input);
        assert_eq!(&input[content_range], "graph TD;\n");

        let input = "~~~mermaid\ngraph TD;\n    ~~~~";
        let content_range = extract_code_block_content_range(input);
        assert_eq!(&input[content_range], "graph TD;\n    ~~~~");

        let input = "~~~mermaid\ngraph TD;\n   ~~~~ \t";
        let content_range = extract_code_block_content_range(input);
        assert_eq!(&input[content_range], "graph TD;\n");

        // Malformed input
        let input = "`````";
        assert_eq!(extract_code_block_content_range(input), 5..5);
    }

    #[test]
    fn test_footnotes() {
        let parsed = parse_markdown_with_options(
            "Text with a footnote[^1] and some more text.\n\n[^1]: This is the footnote content.",
            false,
            false,
            false,
        );
        assert_eq!(
            parsed.events,
            vec![
                (0..45, RootStart),
                (0..45, Start(Paragraph)),
                (0..20, Text),
                (20..24, FootnoteReference("1".into())),
                (24..44, Text),
                (0..45, End(MarkdownTagEnd::Paragraph)),
                (0..45, RootEnd(0)),
                (46..81, RootStart),
                (46..81, Start(FootnoteDefinition("1".into()))),
                (52..81, Start(Paragraph)),
                (52..81, Text),
                (52..81, End(MarkdownTagEnd::Paragraph)),
                (46..81, End(MarkdownTagEnd::FootnoteDefinition)),
                (46..81, RootEnd(1)),
            ]
        );
        assert_eq!(parsed.footnote_definitions.len(), 1);
        assert_eq!(parsed.footnote_definitions.get("1").copied(), Some(52));
    }

    #[test]
    fn test_footnote_definitions_multiple() {
        let parsed = parse_markdown_with_options(
            "Text[^a] and[^b].\n\n[^a]: First.\n\n[^b]: Second.",
            false,
            false,
            false,
        );
        assert_eq!(parsed.footnote_definitions.len(), 2);
        assert!(parsed.footnote_definitions.contains_key("a"));
        assert!(parsed.footnote_definitions.contains_key("b"));
    }

    #[test]
    fn test_links_split_across_fragments() {
        // This test verifies that links split across multiple text fragments due to escaping or other issues
        // are correctly detected and processed
        // Note: In real usage, pulldown_cmark creates separate text events for the escaped character
        // We're verifying our parser can handle this correctly
        assert_eq!(
            parse_markdown_with_options(
                "https:/\\/example.com is equivalent to https://example&#46;com!",
                false,
                false,
                false,
            )
            .events,
            vec![
                (0..62, RootStart),
                (0..62, Start(Paragraph)),
                (
                    0..20,
                    Start(Link {
                        link_type: LinkType::Autolink,
                        dest_url: "https://example.com".into(),
                        title: "".into(),
                        id: "".into()
                    })
                ),
                (0..7, Text),
                (8..20, Text),
                (0..20, End(MarkdownTagEnd::Link)),
                (20..38, Text),
                (
                    38..61,
                    Start(Link {
                        link_type: LinkType::Autolink,
                        dest_url: "https://example.com".into(),
                        title: "".into(),
                        id: "".into()
                    })
                ),
                (38..53, Text),
                (53..58, SubstitutedText(".".into())),
                (58..61, Text),
                (38..61, End(MarkdownTagEnd::Link)),
                (61..62, Text),
                (0..62, End(MarkdownTagEnd::Paragraph)),
                (0..62, RootEnd(0)),
            ],
        );

        assert_eq!(
            parse_markdown_with_options(
                "Visit https://example.com/cat\\/é&#8205;☕ for coffee!",
                false,
                false,
                false,
            )
            .events,
            [
                (0..55, RootStart),
                (0..55, Start(Paragraph)),
                (0..6, Text),
                (
                    6..43,
                    Start(Link {
                        link_type: LinkType::Autolink,
                        dest_url: "https://example.com/cat/é\u{200d}☕".into(),
                        title: "".into(),
                        id: "".into()
                    })
                ),
                (6..29, Text),
                (30..33, Text),
                (33..40, SubstitutedText("\u{200d}".into())),
                (40..43, Text),
                (6..43, End(MarkdownTagEnd::Link)),
                (43..55, Text),
                (0..55, End(MarkdownTagEnd::Paragraph)),
                (0..55, RootEnd(0)),
            ]
        );
    }

    #[test]
    fn test_heading_slugs() {
        let parsed = parse_markdown_with_options(
            "# Hello World\n\n## Code `block`\n\n### Third Level\n\n#### Fourth Level\n\n## Hello World",
            false,
            true,
            false,
        );
        assert_eq!(parsed.heading_slugs.len(), 5);
        assert!(parsed.heading_slugs.contains_key("hello-world"));
        assert!(parsed.heading_slugs.contains_key("code-block"));
        assert!(parsed.heading_slugs.contains_key("third-level"));
        assert!(parsed.heading_slugs.contains_key("fourth-level"));
        assert!(parsed.heading_slugs.contains_key("hello-world-1"));
    }

    #[test]
    fn test_heading_source_index_for_slug() {
        let parsed = parse_markdown_with_options(
            "# Duplicate\n\nText\n\n## Duplicate\n\nMore text",
            false,
            true,
            false,
        );
        let first = parsed.heading_slugs.get("duplicate").copied();
        let second = parsed.heading_slugs.get("duplicate-1").copied();
        assert!(first.is_some());
        assert!(second.is_some());
        assert!(first.expect("first slug missing") < second.expect("second slug missing"));
    }

    #[test]
    fn test_heading_slug_collision_with_dedup_suffix() {
        let parsed = parse_markdown_with_options("# Foo\n\n## Foo\n\n## Foo 1", false, true, false);
        assert_eq!(parsed.heading_slugs.len(), 3);
        assert!(parsed.heading_slugs.contains_key("foo"));
        assert!(parsed.heading_slugs.contains_key("foo-1"));
        assert!(parsed.heading_slugs.contains_key("foo-1-1"));
    }

    #[test]
    fn test_gfm_alert_block_quote_kinds() {
        use pulldown_cmark::BlockQuoteKind;

        let markdown = "\n> [!NOTE]\n> A note.\n\n> [!TIP]\n> A tip.\n\n> [!IMPORTANT]\n> Important.\n\n> [!WARNING]\n> A warning.\n\n> [!CAUTION]\n> A caution.\n\n> Plain quote.\n";
        let parsed = parse_markdown_with_options(markdown, false, false, false);

        let block_quote_kinds: Vec<_> = parsed
            .events
            .iter()
            .filter_map(|(_, event)| match event {
                Start(BlockQuote(kind)) => Some(*kind),
                _ => None,
            })
            .collect();

        assert_eq!(
            block_quote_kinds,
            vec![
                Some(BlockQuoteKind::Note),
                Some(BlockQuoteKind::Tip),
                Some(BlockQuoteKind::Important),
                Some(BlockQuoteKind::Warning),
                Some(BlockQuoteKind::Caution),
                None,
            ]
        );
    }

    #[test]
    fn test_br_tag_emits_hard_break() {
        for input in [
            "hello<br>world",
            "hello<br/>world",
            "hello<br />world",
            "hello<br >world",
            "hello<BR>world",
            "hello<br class=\"x\">world",
            "hello<br class=\"x\"/>world",
        ] {
            let parsed = parse_markdown_with_options(input, true, false, false);
            let has_hard_break = parsed
                .events
                .iter()
                .any(|(_, event)| matches!(event, MarkdownEvent::HardBreak));
            let has_empty_substituted_text = parsed.events.iter().any(|(_, event)| {
                matches!(event, MarkdownEvent::SubstitutedText(text) if text.is_empty())
            });
            assert!(has_hard_break, "<br> in \"{input}\" should emit HardBreak");
            assert!(
                !has_empty_substituted_text,
                "<br> in \"{input}\" should not produce empty SubstitutedText"
            );
        }
    }

    #[test]
    fn test_br_tag_not_a_hard_break_without_parse_html() {
        for input in ["hello<br>world", "hello<br/>world", "hello<br />world"] {
            let parsed = parse_markdown_with_options(input, false, false, false);
            let has_hard_break = parsed
                .events
                .iter()
                .any(|(_, event)| matches!(event, MarkdownEvent::HardBreak));
            let has_inline_html = parsed
                .events
                .iter()
                .any(|(_, event)| matches!(event, MarkdownEvent::InlineHtml));
            assert!(
                !has_hard_break,
                "<br> in \"{input}\" should not emit HardBreak when parse_html is disabled"
            );
            assert!(
                has_inline_html,
                "<br> in \"{input}\" should be preserved as InlineHtml when parse_html is disabled"
            );
        }
    }

    #[test]
    fn test_br_prefixed_tag_is_not_a_hard_break() {
        for input in ["a<break>b", "a<brick>b", "a<b>bold</b>c"] {
            let parsed = parse_markdown_with_options(input, true, false, false);
            let has_hard_break = parsed
                .events
                .iter()
                .any(|(_, event)| matches!(event, MarkdownEvent::HardBreak));
            assert!(
                !has_hard_break,
                "\"{input}\" should not be treated as a <br> hard break"
            );
        }
    }

    #[test]
    fn test_unrecognized_inline_html_preserved_as_inline_html() {
        for input in ["a<span>b</span>c", "a<em>b</em>c", "a<strong>b</strong>c"] {
            let parsed = parse_markdown_with_options(input, false, false, false);
            let has_inline_html = parsed
                .events
                .iter()
                .any(|(_, event)| matches!(event, MarkdownEvent::InlineHtml));
            let has_hard_break = parsed
                .events
                .iter()
                .any(|(_, event)| matches!(event, MarkdownEvent::HardBreak));
            assert!(
                has_inline_html,
                "unrecognized inline HTML \"{input}\" should emit InlineHtml"
            );
            assert!(
                !has_hard_break,
                "unrecognized inline HTML \"{input}\" should not emit HardBreak"
            );
        }
    }

    fn display_math_contents(text: &str) -> Vec<String> {
        parse_markdown_with_options(text, false, false, false)
            .events
            .into_iter()
            .filter_map(|(_, event)| match event {
                MarkdownEvent::Math {
                    display: true,
                    content,
                } => Some(content.to_string()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn display_math_survives_setext_underline_lines() {
        // A lone `=` line is a setext underline, which used to split the span.
        let matrix_product = "\
$$
\\begin{pmatrix}
a & b \\\\
c & d
\\end{pmatrix}
=
\\begin{pmatrix}
ax + by \\\\
cx + dy
\\end{pmatrix}
$$";
        let contents = display_math_contents(matrix_product);
        assert_eq!(contents.len(), 1, "expected one display formula");
        assert!(contents[0].contains('='), "{:?}", contents[0]);
        assert!(contents[0].contains("ax + by"), "{:?}", contents[0]);
        assert!(!contents[0].contains('\n'), "{:?}", contents[0]);
    }

    #[test]
    fn display_math_survives_thematic_break_and_blank_lines() {
        for interior in ["---", "", "- a", "# h", "> q"] {
            let source = format!("$$\nx = 1\n{interior}\ny = 2\n$$");
            let contents = display_math_contents(&source);
            assert_eq!(
                contents.len(),
                1,
                "interior line {interior:?} broke the display math span"
            );
            assert!(contents[0].contains("y = 2"), "{:?}", contents[0]);
        }
    }

    #[test]
    fn display_math_ranges_still_index_the_original_source() {
        let source = "intro\n\n$$\na\n=\nb\n$$\n\ntail\n";
        let parsed = parse_markdown_with_options(source, false, false, false);
        let (range, _) = parsed
            .events
            .iter()
            .find(|(_, event)| matches!(event, MarkdownEvent::Math { display: true, .. }))
            .expect("display math event");
        assert_eq!(&source[range.clone()], "$$\na\n=\nb\n$$");
    }

    #[test]
    fn dollar_pairs_inside_fenced_code_are_left_alone() {
        let source = "```\n$$\na\n=\nb\n$$\n```\n";
        assert!(normalize_math_for_parsing(source).is_none());
        assert!(display_math_contents(source).is_empty());
    }

    #[test]
    fn single_line_display_math_needs_no_rewrite() {
        assert!(normalize_math_for_parsing("$$x^2 + y^2 = z^2$$\n").is_none());
        assert!(normalize_math_for_parsing("no math here\n").is_none());
    }

    #[test]
    fn escaped_dollars_do_not_close_a_display_span() {
        // `\$` escapes, but `\\` is a row separator and does not.
        let source = "$$\na \\$\\$ b\n=\nc\n$$\n";
        let contents = display_math_contents(source);
        assert_eq!(contents.len(), 1, "{contents:?}");
        assert!(contents[0].contains("c"), "{:?}", contents[0]);
    }

    #[test]
    fn latex_comments_block_the_line_join() {
        // Joining these lines would put `= b` behind the comment.
        assert!(normalize_math_for_parsing("$$\na % note\n= b\n$$\n").is_none());
        // An escaped percent is a literal character, not a comment.
        assert!(normalize_math_for_parsing("$$\n50\\% \\\\\n= x\n$$\n").is_some());
    }

    fn inline_math_contents(text: &str) -> Vec<String> {
        parse_markdown_with_options(text, false, false, false)
            .events
            .into_iter()
            .filter_map(|(_, event)| match event {
                MarkdownEvent::Math {
                    display: false,
                    content,
                } => Some(content.to_string()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn inline_math_tolerates_a_leading_space() {
        // pulldown-cmark alone drops this span.
        assert_eq!(
            inline_math_contents(r"Therefore it is $ \geq 0$ and done."),
            vec![r"\geq 0".to_string()]
        );
    }

    #[test]
    fn dollar_amounts_do_not_become_math() {
        // The padding swap must not turn these into formulas.
        assert!(inline_math_contents("it cost $ 5 and $ 10 total").is_empty());
        assert!(inline_math_contents("it cost $5 and $10 total").is_empty());
    }

    #[test]
    fn inline_math_padding_leaves_code_spans_alone() {
        // Code spans are verbatim.
        assert!(normalize_math_for_parsing("use `$ 5` here\n").is_none());
    }

    #[test]
    fn inline_math_tolerates_a_trailing_space() {
        assert_eq!(
            inline_math_contents(r"Consider $C(x_i) = $ binary representation"),
            vec!["C(x_i) =".to_string()]
        );
    }

    #[test]
    fn inline_math_tolerates_padding_on_both_sides() {
        assert_eq!(
            inline_math_contents(r"a $ \geq 0 $ b"),
            vec![r"\geq 0".to_string()]
        );
    }

    #[test]
    fn all_whitespace_spans_are_left_alone() {
        assert!(inline_math_contents("a $ $ b").is_empty());
    }

    #[test]
    fn unterminated_display_math_is_left_alone() {
        assert!(normalize_math_for_parsing("$$\na = b\n").is_none());
    }
}
