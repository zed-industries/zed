use derive_more::{Add, AddAssign};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{mem, ops::Range};

const OLD_TEXT_END_TAG: &str = "</old_text>";
const NEW_TEXT_END_TAG: &str = "</new_text>";
const EDITS_END_TAG: &str = "</edits>";
const END_TAGS: [&str; 3] = [OLD_TEXT_END_TAG, NEW_TEXT_END_TAG, EDITS_END_TAG];

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

#[derive(Debug)]
pub struct EditParser {
    state: EditParserState,
    buffer: String,
    metrics: EditParserMetrics,
}

#[derive(Debug, PartialEq)]
enum EditParserState {
    Pending,
    WithinOldText { start: bool, line_hint: Option<u32> },
    AfterOldText,
    WithinNewText { start: bool },
}

impl EditParser {
    pub fn new() -> Self {
        EditParser {
            state: EditParserState::Pending,
            buffer: String::new(),
            metrics: EditParserMetrics::default(),
        }
    }

    pub fn push(&mut self, chunk: &str) -> SmallVec<[EditParserEvent; 1]> {
        self.buffer.push_str(chunk);

        let mut edit_events = SmallVec::new();
        loop {
            match &mut self.state {
                EditParserState::Pending => {
                    if let Some(start) = self.buffer.find("<old_text") {
                        if let Some(tag_end) = self.buffer[start..].find('>') {
                            let tag_end = start + tag_end + 1;
                            let tag = &self.buffer[start..tag_end];
                            let line_hint = self.parse_line_hint(tag);
                            self.buffer.drain(..tag_end);
                            self.state = EditParserState::WithinOldText {
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
                EditParserState::WithinOldText { start, line_hint } => {
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
                        self.state = EditParserState::AfterOldText;
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
                EditParserState::AfterOldText => {
                    if let Some(start) = self.buffer.find("<new_text>") {
                        self.buffer.drain(..start + "<new_text>".len());
                        self.state = EditParserState::WithinNewText { start: true };
                    } else {
                        break;
                    }
                }
                EditParserState::WithinNewText { start } => {
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
                        self.state = EditParserState::Pending;
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
        static LINE_HINT_REGEX: std::sync::LazyLock<Regex> =
            std::sync::LazyLock::new(|| Regex::new(r#"line=(?:"|)(\d+)"#).unwrap());

        LINE_HINT_REGEX
            .captures(tag)
            .and_then(|caps| caps.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok())
    }

    pub fn finish(self) -> EditParserMetrics {
        self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rand::prelude::*;
    use std::cmp;

    #[gpui::test(iterations = 1000)]
    fn test_single_edit(mut rng: StdRng) {
        let mut parser = EditParser::new();
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
    fn test_multiple_edits(mut rng: StdRng) {
        let mut parser = EditParser::new();
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
    fn test_edits_with_extra_text(mut rng: StdRng) {
        let mut parser = EditParser::new();
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
    fn test_nested_tags(mut rng: StdRng) {
        let mut parser = EditParser::new();
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
    fn test_empty_old_and_new_text(mut rng: StdRng) {
        let mut parser = EditParser::new();
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
    fn test_multiline_content(mut rng: StdRng) {
        let mut parser = EditParser::new();
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
    fn test_mismatched_tags(mut rng: StdRng) {
        let mut parser = EditParser::new();
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

        let mut parser = EditParser::new();
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

    #[gpui::test(iterations = 100)]
    fn test_line_hints(mut rng: StdRng) {
        // Line hint provided, and it's a range
        let mut parser = EditParser::new();

        let edits = parse_random_chunks(
            r#"
            <old_text line_hint="23:50">original code</old_text>
            <new_text>updated code</new_text>"#,
            &mut parser,
            &mut rng,
        );

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].old_text, "original code");
        assert_eq!(edits[0].line_hint, Some(23));
        assert_eq!(edits[0].new_text, "updated code");

        // Line hint provided, and it's a single number (line number)
        let mut parser = EditParser::new();

        let edits = parse_random_chunks(
            r#"
            <old_text line_hint="23">original code</old_text>
            <new_text>updated code</new_text>"#,
            &mut parser,
            &mut rng,
        );

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].old_text, "original code");
        assert_eq!(edits[0].line_hint, Some(23));
        assert_eq!(edits[0].new_text, "updated code");

        // No line hint
        let mut parser = EditParser::new();
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
        let chunk_count = rng.gen_range(1..=cmp::min(input.len(), 50));
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

        edits
    }
}
