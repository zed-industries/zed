use anyhow::bail;
use derive_more::{Add, AddAssign};
use language_model::LanguageModel;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{mem, ops::Range, str::FromStr, sync::Arc};

const OLD_TEXT_END_TAG: &str = "</old_text>";
const NEW_TEXT_END_TAG: &str = "</new_text>";
const EDITS_END_TAG: &str = "</edits>";
const SEARCH_MARKER: &str = "<<<<<<< SEARCH";
const SEPARATOR_MARKER: &str = "=======";
const REPLACE_MARKER: &str = ">>>>>>> REPLACE";
const SONNET_PARAMETER_INVOKE_1: &str = "</parameter>\n</invoke>";
const SONNET_PARAMETER_INVOKE_2: &str = "</parameter></invoke>";
const SONNET_PARAMETER_INVOKE_3: &str = "</parameter>";
const END_TAGS: [&str; 6] = [
    OLD_TEXT_END_TAG,
    NEW_TEXT_END_TAG,
    EDITS_END_TAG,
    SONNET_PARAMETER_INVOKE_1, // Remove these after switching to streaming tool call
    SONNET_PARAMETER_INVOKE_2,
    SONNET_PARAMETER_INVOKE_3,
];

#[derive(Debug)]
pub enum EditParserEvent {
    OldTextChunk {
        chunk: String,
        done: bool,
        line_hint: Option<u32>,
    },
    NewTextChunk {
        chunk: String,
        done: bool,
    },
}

#[derive(
    Clone, Debug, Default, PartialEq, Eq, Add, AddAssign, Serialize, Deserialize, JsonSchema,
)]
pub struct EditParserMetrics {
    pub tags: usize,
    pub mismatched_tags: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditFormat {
    /// XML-like tags:
    /// <old_text>...</old_text>
    /// <new_text>...</new_text>
    XmlTags,
    /// Diff-fenced format, in which:
    /// - Text before the SEARCH marker is ignored
    /// - Fences are optional
    /// - Line hint is optional.
    ///
    /// Example:
    ///
    /// ```diff
    /// <<<<<<< SEARCH line=42
    /// ...
    /// =======
    /// ...
    /// >>>>>>> REPLACE
    /// ```
    DiffFenced,
}

impl FromStr for EditFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "xml_tags" | "xml" => Ok(EditFormat::XmlTags),
            "diff_fenced" | "diff-fenced" | "diff" => Ok(EditFormat::DiffFenced),
            _ => bail!("Unknown EditFormat: {}", s),
        }
    }
}

impl EditFormat {
    /// Return an optimal edit format for the language model
    pub fn from_model(model: Arc<dyn LanguageModel>) -> anyhow::Result<Self> {
        if model.provider_id().0 == "google" || model.id().0.to_lowercase().contains("gemini") {
            Ok(EditFormat::DiffFenced)
        } else {
            Ok(EditFormat::XmlTags)
        }
    }

    /// Return an optimal edit format for the language model,
    /// with the ability to override it by setting the
    /// `ZED_EDIT_FORMAT` environment variable
    #[allow(dead_code)]
    pub fn from_env(model: Arc<dyn LanguageModel>) -> anyhow::Result<Self> {
        let default = EditFormat::from_model(model)?;
        std::env::var("ZED_EDIT_FORMAT").map_or(Ok(default), |s| EditFormat::from_str(&s))
    }
}

pub trait EditFormatParser: Send + std::fmt::Debug {
    fn push(&mut self, chunk: &str) -> SmallVec<[EditParserEvent; 1]>;
    fn take_metrics(&mut self) -> EditParserMetrics;
}

#[derive(Debug)]
pub struct XmlEditParser {
    state: XmlParserState,
    buffer: String,
    metrics: EditParserMetrics,
}

#[derive(Debug, PartialEq)]
enum XmlParserState {
    Pending,
    WithinOldText { start: bool, line_hint: Option<u32> },
    AfterOldText,
    WithinNewText { start: bool },
}

#[derive(Debug)]
pub struct DiffFencedEditParser {
    state: DiffParserState,
    buffer: String,
    metrics: EditParserMetrics,
}

#[derive(Debug, PartialEq)]
enum DiffParserState {
    Pending,
    WithinSearch { start: bool, line_hint: Option<u32> },
    WithinReplace { start: bool },
}

/// Main parser that delegates to format-specific parsers
pub struct EditParser {
    parser: Box<dyn EditFormatParser>,
}

impl XmlEditParser {
    pub fn new() -> Self {
        XmlEditParser {
            state: XmlParserState::Pending,
            buffer: String::new(),
            metrics: EditParserMetrics::default(),
        }
    }

    fn find_end_tag(&self) -> Option<Range<usize>> {
        let (tag, start_ix) = END_TAGS
            .iter()
            .flat_map(|tag| Some((tag, self.buffer.find(tag)?)))
            .min_by_key(|(_, ix)| *ix)?;
        Some(start_ix..start_ix + tag.len())
    }

    fn ends_with_tag_prefix(&self) -> bool {
        let mut end_prefixes = END_TAGS
            .iter()
            .flat_map(|tag| (1..tag.len()).map(move |i| &tag[..i]))
            .chain(["\n"]);
        end_prefixes.any(|prefix| self.buffer.ends_with(&prefix))
    }

    fn parse_line_hint(&self, tag: &str) -> Option<u32> {
        use std::sync::LazyLock;
        static LINE_HINT_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"line=(?:"?)(\d+)"#).unwrap());

        LINE_HINT_REGEX
            .captures(tag)
            .and_then(|caps| caps.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok())
    }
}

impl EditFormatParser for XmlEditParser {
    fn push(&mut self, chunk: &str) -> SmallVec<[EditParserEvent; 1]> {
        self.buffer.push_str(chunk);

        let mut edit_events = SmallVec::new();
        loop {
            match &mut self.state {
                XmlParserState::Pending => {
                    if let Some(start) = self.buffer.find("<old_text") {
                        if let Some(tag_end) = self.buffer[start..].find('>') {
                            let tag_end = start + tag_end + 1;
                            let tag = &self.buffer[start..tag_end];
                            let line_hint = self.parse_line_hint(tag);
                            self.buffer.drain(..tag_end);
                            self.state = XmlParserState::WithinOldText {
                                start: true,
                                line_hint,
                            };
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                XmlParserState::WithinOldText { start, line_hint } => {
                    if !self.buffer.is_empty() {
                        if *start && self.buffer.starts_with('\n') {
                            self.buffer.remove(0);
                        }
                        *start = false;
                    }

                    let line_hint = *line_hint;
                    if let Some(tag_range) = self.find_end_tag() {
                        let mut chunk = self.buffer[..tag_range.start].to_string();
                        if chunk.ends_with('\n') {
                            chunk.pop();
                        }

                        self.metrics.tags += 1;
                        if &self.buffer[tag_range.clone()] != OLD_TEXT_END_TAG {
                            self.metrics.mismatched_tags += 1;
                        }

                        self.buffer.drain(..tag_range.end);
                        self.state = XmlParserState::AfterOldText;
                        edit_events.push(EditParserEvent::OldTextChunk {
                            chunk,
                            done: true,
                            line_hint,
                        });
                    } else {
                        if !self.ends_with_tag_prefix() {
                            edit_events.push(EditParserEvent::OldTextChunk {
                                chunk: mem::take(&mut self.buffer),
                                done: false,
                                line_hint,
                            });
                        }
                        break;
                    }
                }
                XmlParserState::AfterOldText => {
                    if let Some(start) = self.buffer.find("<new_text>") {
                        self.buffer.drain(..start + "<new_text>".len());
                        self.state = XmlParserState::WithinNewText { start: true };
                    } else {
                        break;
                    }
                }
                XmlParserState::WithinNewText { start } => {
                    if !self.buffer.is_empty() {
                        if *start && self.buffer.starts_with('\n') {
                            self.buffer.remove(0);
                        }
                        *start = false;
                    }

                    if let Some(tag_range) = self.find_end_tag() {
                        let mut chunk = self.buffer[..tag_range.start].to_string();
                        if chunk.ends_with('\n') {
                            chunk.pop();
                        }

                        self.metrics.tags += 1;
                        if &self.buffer[tag_range.clone()] != NEW_TEXT_END_TAG {
                            self.metrics.mismatched_tags += 1;
                        }

                        self.buffer.drain(..tag_range.end);
                        self.state = XmlParserState::Pending;
                        edit_events.push(EditParserEvent::NewTextChunk { chunk, done: true });
                    } else {
                        if !self.ends_with_tag_prefix() {
                            edit_events.push(EditParserEvent::NewTextChunk {
                                chunk: mem::take(&mut self.buffer),
                                done: false,
                            });
                        }
                        break;
                    }
                }
            }
        }
        edit_events
    }

    fn take_metrics(&mut self) -> EditParserMetrics {
        std::mem::take(&mut self.metrics)
    }
}

impl DiffFencedEditParser {
    pub fn new() -> Self {
        DiffFencedEditParser {
            state: DiffParserState::Pending,
            buffer: String::new(),
            metrics: EditParserMetrics::default(),
        }
    }

    fn ends_with_diff_marker_prefix(&self) -> bool {
        let diff_markers = [SEPARATOR_MARKER, REPLACE_MARKER];
        let mut diff_prefixes = diff_markers
            .iter()
            .flat_map(|marker| (1..marker.len()).map(move |i| &marker[..i]))
            .chain(["\n"]);
        diff_prefixes.any(|prefix| self.buffer.ends_with(&prefix))
    }

    fn parse_line_hint(&self, search_line: &str) -> Option<u32> {
        use regex::Regex;
        use std::sync::LazyLock;
        static LINE_HINT_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"line=(?:"?)(\d+)"#).unwrap());

        LINE_HINT_REGEX
            .captures(search_line)
            .and_then(|caps| caps.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok())
    }
}

impl EditFormatParser for DiffFencedEditParser {
    fn push(&mut self, chunk: &str) -> SmallVec<[EditParserEvent; 1]> {
        self.buffer.push_str(chunk);

        let mut edit_events = SmallVec::new();
        loop {
            match &mut self.state {
                DiffParserState::Pending => {
                    if let Some(diff) = self.buffer.find(SEARCH_MARKER) {
                        let search_end = diff + SEARCH_MARKER.len();
                        if let Some(newline_pos) = self.buffer[search_end..].find('\n') {
                            let search_line = &self.buffer[diff..search_end + newline_pos];
                            let line_hint = self.parse_line_hint(search_line);
                            self.buffer.drain(..search_end + newline_pos + 1);
                            self.state = DiffParserState::WithinSearch {
                                start: true,
                                line_hint,
                            };
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                DiffParserState::WithinSearch { start, line_hint } => {
                    if !self.buffer.is_empty() {
                        if *start && self.buffer.starts_with('\n') {
                            self.buffer.remove(0);
                        }
                        *start = false;
                    }

                    let line_hint = *line_hint;
                    if let Some(separator_pos) = self.buffer.find(SEPARATOR_MARKER) {
                        let mut chunk = self.buffer[..separator_pos].to_string();
                        if chunk.ends_with('\n') {
                            chunk.pop();
                        }

                        let separator_end = separator_pos + SEPARATOR_MARKER.len();
                        if let Some(newline_pos) = self.buffer[separator_end..].find('\n') {
                            self.buffer.drain(..separator_end + newline_pos + 1);
                            self.state = DiffParserState::WithinReplace { start: true };
                            edit_events.push(EditParserEvent::OldTextChunk {
                                chunk,
                                done: true,
                                line_hint,
                            });
                        } else {
                            break;
                        }
                    } else {
                        if !self.ends_with_diff_marker_prefix() {
                            edit_events.push(EditParserEvent::OldTextChunk {
                                chunk: mem::take(&mut self.buffer),
                                done: false,
                                line_hint,
                            });
                        }
                        break;
                    }
                }
                DiffParserState::WithinReplace { start } => {
                    if !self.buffer.is_empty() {
                        if *start && self.buffer.starts_with('\n') {
                            self.buffer.remove(0);
                        }
                        *start = false;
                    }

                    if let Some(replace_pos) = self.buffer.find(REPLACE_MARKER) {
                        let mut chunk = self.buffer[..replace_pos].to_string();
                        if chunk.ends_with('\n') {
                            chunk.pop();
                        }

                        self.buffer.drain(..replace_pos + REPLACE_MARKER.len());
                        if let Some(newline_pos) = self.buffer.find('\n') {
                            self.buffer.drain(..newline_pos + 1);
                        } else {
                            self.buffer.clear();
                        }

                        self.state = DiffParserState::Pending;
                        edit_events.push(EditParserEvent::NewTextChunk { chunk, done: true });
                    } else {
                        if !self.ends_with_diff_marker_prefix() {
                            edit_events.push(EditParserEvent::NewTextChunk {
                                chunk: mem::take(&mut self.buffer),
                                done: false,
                            });
                        }
                        break;
                    }
                }
            }
        }
        edit_events
    }

    fn take_metrics(&mut self) -> EditParserMetrics {
        std::mem::take(&mut self.metrics)
    }
}

impl EditParser {
    pub fn new(format: EditFormat) -> Self {
        let parser: Box<dyn EditFormatParser> = match format {
            EditFormat::XmlTags => Box::new(XmlEditParser::new()),
            EditFormat::DiffFenced => Box::new(DiffFencedEditParser::new()),
        };
        EditParser { parser }
    }

    pub fn push(&mut self, chunk: &str) -> SmallVec<[EditParserEvent; 1]> {
        self.parser.push(chunk)
    }

    pub fn finish(mut self) -> EditParserMetrics {
        self.parser.take_metrics()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rand::prelude::*;
    use std::cmp;

    #[gpui::test(iterations = 1000)]
    fn test_xml_single_edit(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                "<old_text>original</old_text><new_text>updated</new_text>",
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "original".to_string(),
                new_text: "updated".to_string(),
                line_hint: None,
            }]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 2,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_xml_multiple_edits(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    <old_text>
                    first old
                    </old_text><new_text>first new</new_text>
                    <old_text>second old</old_text><new_text>
                    second new
                    </new_text>
                "},
                &mut parser,
                &mut rng
            ),
            vec![
                Edit {
                    old_text: "first old".to_string(),
                    new_text: "first new".to_string(),
                    line_hint: None,
                },
                Edit {
                    old_text: "second old".to_string(),
                    new_text: "second new".to_string(),
                    line_hint: None,
                },
            ]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 4,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_xml_edits_with_extra_text(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    ignore this <old_text>
                    content</old_text>extra stuff<new_text>updated content</new_text>trailing data
                    more text <old_text>second item
                    </old_text>middle text<new_text>modified second item</new_text>end
                    <old_text>third case</old_text><new_text>improved third case</new_text> with trailing text
                "},
                &mut parser,
                &mut rng
            ),
            vec![
                Edit {
                    old_text: "content".to_string(),
                    new_text: "updated content".to_string(),
                    line_hint: None,
                },
                Edit {
                    old_text: "second item".to_string(),
                    new_text: "modified second item".to_string(),
                    line_hint: None,
                },
                Edit {
                    old_text: "third case".to_string(),
                    new_text: "improved third case".to_string(),
                    line_hint: None,
                },
            ]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 6,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_xml_edits_with_closing_parameter_invoke(mut rng: StdRng) {
        // This case is a regression with Claude Sonnet 4.5.
        // Sometimes Sonnet thinks that it's doing a tool call
        // and closes its response with '</parameter></invoke>'
        // instead of properly closing </new_text>

        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    <old_text>some text</old_text><new_text>updated text</parameter></invoke>
                    <old_text>more text</old_text><new_text>upd</parameter></new_text>
                "},
                &mut parser,
                &mut rng
            ),
            vec![
                Edit {
                    old_text: "some text".to_string(),
                    new_text: "updated text".to_string(),
                    line_hint: None,
                },
                Edit {
                    old_text: "more text".to_string(),
                    new_text: "upd".to_string(),
                    line_hint: None,
                },
            ]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 4,
                mismatched_tags: 2
            }
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_xml_nested_tags(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                "<old_text>code with <tag>nested</tag> elements</old_text><new_text>new <code>content</code></new_text>",
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "code with <tag>nested</tag> elements".to_string(),
                new_text: "new <code>content</code>".to_string(),
                line_hint: None,
            }]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 2,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_xml_empty_old_and_new_text(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                "<old_text></old_text><new_text></new_text>",
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "".to_string(),
                new_text: "".to_string(),
                line_hint: None,
            }]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 2,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_xml_multiline_content(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                "<old_text>line1\nline2\nline3</old_text><new_text>line1\nmodified line2\nline3</new_text>",
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "line1\nline2\nline3".to_string(),
                new_text: "line1\nmodified line2\nline3".to_string(),
                line_hint: None,
            }]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 2,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_xml_mismatched_tags(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                // Reduced from an actual Sonnet 3.7 output
                indoc! {"
                    <old_text>
                    a
                    b
                    c
                    </new_text>
                    <new_text>
                    a
                    B
                    c
                    </old_text>
                    <old_text>
                    d
                    e
                    f
                    </new_text>
                    <new_text>
                    D
                    e
                    F
                    </old_text>
                "},
                &mut parser,
                &mut rng
            ),
            vec![
                Edit {
                    old_text: "a\nb\nc".to_string(),
                    new_text: "a\nB\nc".to_string(),
                    line_hint: None,
                },
                Edit {
                    old_text: "d\ne\nf".to_string(),
                    new_text: "D\ne\nF".to_string(),
                    line_hint: None,
                }
            ]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 4,
                mismatched_tags: 4
            }
        );

        let mut parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                // Reduced from an actual Opus 4 output
                indoc! {"
                    <edits>
                    <old_text>
                    Lorem
                    </old_text>
                    <new_text>
                    LOREM
                    </edits>
                "},
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "Lorem".to_string(),
                new_text: "LOREM".to_string(),
                line_hint: None,
            },]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 2,
                mismatched_tags: 1
            }
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_diff_fenced_single_edit(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::DiffFenced);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    <<<<<<< SEARCH
                    original text
                    =======
                    updated text
                    >>>>>>> REPLACE
                "},
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "original text".to_string(),
                new_text: "updated text".to_string(),
                line_hint: None,
            }]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 0,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_diff_fenced_with_markdown_fences(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::DiffFenced);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    ```diff
                    <<<<<<< SEARCH
                    from flask import Flask
                    =======
                    import math
                    from flask import Flask
                    >>>>>>> REPLACE
                    ```
                "},
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "from flask import Flask".to_string(),
                new_text: "import math\nfrom flask import Flask".to_string(),
                line_hint: None,
            }]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 0,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_diff_fenced_multiple_edits(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::DiffFenced);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    <<<<<<< SEARCH
                    first old
                    =======
                    first new
                    >>>>>>> REPLACE

                    <<<<<<< SEARCH
                    second old
                    =======
                    second new
                    >>>>>>> REPLACE
                "},
                &mut parser,
                &mut rng
            ),
            vec![
                Edit {
                    old_text: "first old".to_string(),
                    new_text: "first new".to_string(),
                    line_hint: None,
                },
                Edit {
                    old_text: "second old".to_string(),
                    new_text: "second new".to_string(),
                    line_hint: None,
                },
            ]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 0,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_mixed_formats(mut rng: StdRng) {
        // Test XML format parser only parses XML tags
        let mut xml_parser = EditParser::new(EditFormat::XmlTags);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    <old_text>xml style old</old_text><new_text>xml style new</new_text>

                    <<<<<<< SEARCH
                    diff style old
                    =======
                    diff style new
                    >>>>>>> REPLACE
                "},
                &mut xml_parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "xml style old".to_string(),
                new_text: "xml style new".to_string(),
                line_hint: None,
            },]
        );
        assert_eq!(
            xml_parser.finish(),
            EditParserMetrics {
                tags: 2,
                mismatched_tags: 0
            }
        );

        // Test diff-fenced format parser only parses diff markers
        let mut diff_parser = EditParser::new(EditFormat::DiffFenced);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    <old_text>xml style old</old_text><new_text>xml style new</new_text>

                    <<<<<<< SEARCH
                    diff style old
                    =======
                    diff style new
                    >>>>>>> REPLACE
                "},
                &mut diff_parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "diff style old".to_string(),
                new_text: "diff style new".to_string(),
                line_hint: None,
            },]
        );
        assert_eq!(
            diff_parser.finish(),
            EditParserMetrics {
                tags: 0,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_diff_fenced_empty_sections(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::DiffFenced);
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                <<<<<<< SEARCH
                =======
                >>>>>>> REPLACE
            "},
                &mut parser,
                &mut rng
            ),
            vec![Edit {
                old_text: "".to_string(),
                new_text: "".to_string(),
                line_hint: None,
            }]
        );
        assert_eq!(
            parser.finish(),
            EditParserMetrics {
                tags: 0,
                mismatched_tags: 0
            }
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_diff_fenced_with_line_hint(mut rng: StdRng) {
        let mut parser = EditParser::new(EditFormat::DiffFenced);
        let edits = parse_random_chunks(
            indoc! {"
                <<<<<<< SEARCH line=42
                original text
                =======
                updated text
                >>>>>>> REPLACE
            "},
            &mut parser,
            &mut rng,
        );
        assert_eq!(
            edits,
            vec![Edit {
                old_text: "original text".to_string(),
                line_hint: Some(42),
                new_text: "updated text".to_string(),
            }]
        );
    }
    #[gpui::test(iterations = 100)]
    fn test_xml_line_hints(mut rng: StdRng) {
        // Line hint is a single quoted line number
        let mut parser = EditParser::new(EditFormat::XmlTags);

        let edits = parse_random_chunks(
            r#"
                    <old_text line="23">original code</old_text>
                    <new_text>updated code</new_text>"#,
            &mut parser,
            &mut rng,
        );

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].old_text, "original code");
        assert_eq!(edits[0].line_hint, Some(23));
        assert_eq!(edits[0].new_text, "updated code");

        // Line hint is a single unquoted line number
        let mut parser = EditParser::new(EditFormat::XmlTags);

        let edits = parse_random_chunks(
            r#"
                    <old_text line=45>original code</old_text>
                    <new_text>updated code</new_text>"#,
            &mut parser,
            &mut rng,
        );

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].old_text, "original code");
        assert_eq!(edits[0].line_hint, Some(45));
        assert_eq!(edits[0].new_text, "updated code");

        // Line hint is a range
        let mut parser = EditParser::new(EditFormat::XmlTags);

        let edits = parse_random_chunks(
            r#"
            <old_text line="23:50">original code</old_text>
            <new_text>updated code</new_text>"#,
            &mut parser,
            &mut rng,
        );

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].old_text, "original code");
        assert_eq!(edits[0].line_hint, Some(23));
        assert_eq!(edits[0].new_text, "updated code");

        // No line hint
        let mut parser = EditParser::new(EditFormat::XmlTags);
        let edits = parse_random_chunks(
            r#"
            <old_text>old</old_text>
            <new_text>new</new_text>"#,
            &mut parser,
            &mut rng,
        );

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].old_text, "old");
        assert_eq!(edits[0].line_hint, None);
        assert_eq!(edits[0].new_text, "new");
    }

    #[derive(Default, Debug, PartialEq, Eq)]
    struct Edit {
        old_text: String,
        new_text: String,
        line_hint: Option<u32>,
    }

    fn parse_random_chunks(input: &str, parser: &mut EditParser, rng: &mut StdRng) -> Vec<Edit> {
        let chunk_count = rng.random_range(1..=cmp::min(input.len(), 50));
        let mut chunk_indices = (0..input.len()).choose_multiple(rng, chunk_count);
        chunk_indices.sort();
        chunk_indices.push(input.len());

        let mut old_text = Some(String::new());
        let mut new_text = None;
        let mut pending_edit = Edit::default();
        let mut edits = Vec::new();
        let mut last_ix = 0;
        for chunk_ix in chunk_indices {
            for event in parser.push(&input[last_ix..chunk_ix]) {
                match event {
                    EditParserEvent::OldTextChunk {
                        chunk,
                        done,
                        line_hint,
                    } => {
                        old_text.as_mut().unwrap().push_str(&chunk);
                        if done {
                            pending_edit.old_text = old_text.take().unwrap();
                            pending_edit.line_hint = line_hint;
                            new_text = Some(String::new());
                        }
                    }
                    EditParserEvent::NewTextChunk { chunk, done } => {
                        new_text.as_mut().unwrap().push_str(&chunk);
                        if done {
                            pending_edit.new_text = new_text.take().unwrap();
                            edits.push(pending_edit);
                            pending_edit = Edit::default();
                            old_text = Some(String::new());
                        }
                    }
                }
            }
            last_ix = chunk_ix;
        }

        if new_text.is_some() {
            pending_edit.new_text = new_text.take().unwrap();
            edits.push(pending_edit);
        }

        edits
    }
}
