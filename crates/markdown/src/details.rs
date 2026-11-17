use std::ops::Range;

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::parser::PARSE_OPTIONS;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParsedDetails {
    pub range: Range<usize>,
    pub summary: Range<usize>,
    pub body: Range<usize>,
    pub open: bool,
}

// Fragments remove container prefixes and HTML indentation, but every emitted
// event must still address the original buffer for selection and preview sync.
struct MappedSource {
    text: String,
    original_offsets: Vec<usize>,
    end: usize,
}

impl MappedSource {
    fn original_range(&self, range: Range<usize>) -> Range<usize> {
        let start = self
            .original_offsets
            .get(range.start)
            .copied()
            .unwrap_or(self.end);
        if range.is_empty() {
            return start..start;
        }
        let end = range
            .end
            .checked_sub(1)
            .and_then(|end| self.original_offsets.get(end))
            .map_or(start, |end| end + 1);
        start..end
    }

    fn fragment(&self, range: Range<usize>, prefix: &str, indentation: usize) -> Self {
        let mut text = String::new();
        let mut original_offsets = Vec::new();
        let mut start = range.start;
        for line in self.text[range.clone()].split_inclusive('\n') {
            let mut content = line;
            if start == 0 || self.text.as_bytes().get(start - 1) == Some(&b'\n') {
                let mut remaining = content;
                for character in prefix.chars() {
                    if let Some(first) = remaining.chars().next()
                        && ((character == '>' && first == '>')
                            || (character != '>' && first.is_ascii_whitespace() && first != '\n'))
                    {
                        remaining = &remaining[first.len_utf8()..];
                    }
                }
                content = remaining;
                let spaces = content
                    .bytes()
                    .take(indentation)
                    .take_while(|byte| *byte == b' ')
                    .count();
                content = &content[spaces..];
            }
            let content_start = start + line.len() - content.len();
            text.push_str(content);
            original_offsets
                .extend_from_slice(&self.original_offsets[content_start..start + line.len()]);
            start += line.len();
        }
        Self {
            text,
            original_offsets,
            end: self.original_range(range).end,
        }
    }
}

struct HtmlTag<'a> {
    name: &'a str,
    closing: bool,
    range: Range<usize>,
}

fn next_tag(text: &str, mut offset: usize, protect_code: bool) -> Option<HtmlTag<'_>> {
    let code_ranges: Vec<_> = if protect_code {
        Parser::new_ext(&text[offset..], PARSE_OPTIONS)
            .into_offset_iter()
            .filter_map(|(event, range)| {
                matches!(event, Event::Code(_) | Event::Start(Tag::CodeBlock(_)))
                    .then_some(range.start + offset..range.end + offset)
            })
            .collect()
    } else {
        Vec::new()
    };
    let bytes = text.as_bytes();
    while offset < text.len() {
        if let Some(range) = code_ranges.iter().find(|range| range.contains(&offset)) {
            offset = range.end;
            continue;
        }
        let byte = *bytes.get(offset)?;
        if byte != b'<' {
            offset += 1;
            continue;
        }
        if text[offset..].starts_with("<!--") {
            offset += text[offset..].find("-->")? + 3;
            continue;
        }
        let start = offset;
        offset += 1;
        let closing = bytes.get(offset) == Some(&b'/');
        if closing {
            offset += 1;
        }
        let name_start = offset;
        while bytes.get(offset).is_some_and(u8::is_ascii_alphanumeric) {
            offset += 1;
        }
        if offset == name_start
            || !bytes
                .get(offset)
                .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>' || *byte == b'/')
        {
            continue;
        }
        let name = &text[name_start..offset];
        let mut quote = None;
        while let Some(byte) = bytes.get(offset).copied() {
            offset += 1;
            match (quote, byte) {
                (Some(delimiter), byte) if delimiter == byte => quote = None,
                (None, b'\'' | b'"') => quote = Some(byte),
                (None, b'>') => {
                    return Some(HtmlTag {
                        name,
                        closing,
                        range: start..offset,
                    });
                }
                (None, b'<') => {
                    offset -= 1;
                    break;
                }
                _ => {}
            }
        }
    }
    None
}

fn open_attribute(tag: &str) -> bool {
    use html5ever::tendril::TendrilSink;
    fn visit(node: &markup5ever_rcdom::Node) -> bool {
        if let markup5ever_rcdom::NodeData::Element { name, attrs, .. } = &node.data
            && name.local.as_ref() == "details"
        {
            return attrs
                .borrow()
                .iter()
                .any(|attribute| attribute.name.local.as_ref() == "open");
        }
        node.children.borrow().iter().any(|child| visit(child))
    }
    let dom =
        html5ever::parse_document(markup5ever_rcdom::RcDom::default(), Default::default()).one(tag);
    visit(&dom.document)
}

struct DetailsContent {
    body: Range<usize>,
    summary: Option<Range<usize>>,
    summary_tags: Option<Range<usize>>,
    end: usize,
}

fn find_details_content(text: &str, opening: &HtmlTag<'_>) -> DetailsContent {
    let mut depth = 1;
    let mut cursor = opening.range.end;
    let mut closing_start = text.len();
    let mut end = text.len();
    let mut summary_start = None;
    let mut summary = None;
    let mut summary_tags = None;
    let mut raw_tag: Option<String> = None;
    while let Some(tag) = next_tag(text, cursor, raw_tag.is_none()) {
        cursor = tag.range.end;
        if let Some(raw) = raw_tag.as_ref() {
            if tag.closing && tag.name.eq_ignore_ascii_case(raw) {
                raw_tag = None;
            }
            continue;
        }
        if !tag.closing
            && ["pre", "code", "script", "style", "textarea"]
                .iter()
                .any(|raw| tag.name.eq_ignore_ascii_case(raw))
        {
            raw_tag = Some(tag.name.to_string());
            continue;
        }
        if tag.name.eq_ignore_ascii_case("details") {
            if tag.closing {
                depth -= 1;
            } else {
                depth += 1;
            }
            if depth == 0 {
                closing_start = tag.range.start;
                end = tag.range.end;
                break;
            }
        } else if depth == 1 && tag.name.eq_ignore_ascii_case("summary") && summary.is_none() {
            if tag.closing {
                if let Some(start) = summary_start.take() {
                    summary = Some(start..tag.range.start);
                    summary_tags = summary_tags.map(|tags: Range<usize>| tags.start..tag.range.end);
                }
            } else if summary_start.is_none() {
                summary_start = Some(tag.range.end);
                summary_tags = Some(tag.range.clone());
            }
        }
    }
    DetailsContent {
        body: opening.range.end..closing_start,
        summary,
        summary_tags,
        end,
    }
}

pub(crate) fn expand_details(
    text: &str,
    start: usize,
    details: &mut Vec<ParsedDetails>,
    html_sources: &mut std::collections::BTreeMap<usize, String>,
) -> Vec<(Event<'static>, Range<usize>)> {
    if !text
        .as_bytes()
        .windows(b"<details".len())
        .any(|window| window.eq_ignore_ascii_case(b"<details"))
    {
        return Parser::new_ext(text, PARSE_OPTIONS)
            .into_offset_iter()
            .map(|(event, range)| (event.into_static(), range.start + start..range.end + start))
            .collect();
    }
    let source = MappedSource {
        text: text.to_string(),
        original_offsets: (start..start + text.len()).collect(),
        end: start + text.len(),
    };
    let parser = Parser::new_ext(text, PARSE_OPTIONS);
    expand(
        &source,
        details,
        html_sources,
        parser.reference_definitions(),
    )
}

#[stacksafe::stacksafe]
fn expand(
    source: &MappedSource,
    details: &mut Vec<ParsedDetails>,
    html_sources: &mut std::collections::BTreeMap<usize, String>,
    definitions: &pulldown_cmark::RefDefs<'_>,
) -> Vec<(Event<'static>, Range<usize>)> {
    let mut output = Vec::new();
    let mut consumed = 0..0;
    let mut resolve_reference = |link: pulldown_cmark::BrokenLink<'_>| {
        definitions.get(&link.reference).map(|definition| {
            (
                definition.dest.clone().into_static(),
                definition.title.clone().unwrap_or("".into()).into_static(),
            )
        })
    };
    for (event, range) in Parser::new_with_broken_link_callback(
        &source.text,
        PARSE_OPTIONS,
        Some(&mut resolve_reference),
    )
    .into_offset_iter()
    {
        if consumed.contains(&range.start) {
            continue;
        }
        if matches!(event, Event::Start(Tag::HtmlBlock))
            && let Some(opening) = next_tag(&source.text, range.start, false)
            && opening.range.start == range.start
            && opening.name.eq_ignore_ascii_case("details")
            && !opening.closing
        {
            let content = find_details_content(&source.text, &opening);
            let line_start = source.text[..opening.range.start]
                .rfind('\n')
                .map_or(0, |start| start + 1);
            let prefix: String = source.text[line_start..opening.range.start]
                .chars()
                .map(|character| if character == '>' { '>' } else { ' ' })
                .collect();
            let inner = source.fragment(content.body.clone(), &prefix, 0);
            let indentation = inner
                .text
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.bytes().take_while(|byte| *byte == b' ').count())
                .min()
                .unwrap_or(0);
            let summary_range = content.summary.clone().unwrap_or(opening.range.clone());
            let summary_range = source.original_range(summary_range);
            details.push(ParsedDetails {
                range: source.original_range(opening.range.start..content.end),
                summary: summary_range.clone(),
                body: source.original_range(content.body.clone()),
                open: open_attribute(&source.text[opening.range.clone()]),
            });
            output.push((Event::Start(Tag::Paragraph), summary_range.clone()));
            if let Some(summary) = content.summary {
                let summary = source.fragment(summary, &prefix, indentation);
                let events = expand(&summary, details, html_sources, definitions);
                let count = events.len();
                output.extend(events.into_iter().enumerate().filter_map(
                    |(index, (event, range))| match event {
                        Event::Start(Tag::Paragraph) => None,
                        Event::End(TagEnd::Paragraph) if index + 1 == count => None,
                        Event::End(TagEnd::Paragraph) => Some((Event::SoftBreak, range)),
                        event => Some((event, range)),
                    },
                ));
            } else {
                output.push((Event::Text("Details".into()), summary_range.clone()));
            }
            output.push((Event::End(TagEnd::Paragraph), summary_range));
            let body_ranges = if let Some(summary_tags) = content.summary_tags {
                vec![
                    opening.range.end..summary_tags.start,
                    summary_tags.end..content.body.end,
                ]
            } else {
                vec![content.body.clone()]
            };
            for body in body_ranges {
                if body.start <= body.end {
                    let body = source.fragment(body, &prefix, indentation);
                    output.extend(expand(&body, details, html_sources, definitions));
                }
            }
            if content.end < range.end {
                let remainder = source.fragment(content.end..range.end, &prefix, 0);
                output.extend(expand(&remainder, details, html_sources, definitions));
            }
            consumed = opening.range.start..content.end.max(range.end);
            continue;
        }
        if matches!(event, Event::Start(Tag::HtmlBlock)) {
            html_sources.insert(
                source.original_range(range.clone()).start,
                source.text[range.clone()].to_string(),
            );
        }
        output.push((event.into_static(), source.original_range(range)));
    }
    output
}
