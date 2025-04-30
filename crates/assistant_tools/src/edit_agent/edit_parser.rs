use smallvec::SmallVec;
use std::mem;

#[derive(Debug)]
pub enum EditParserEvent {
    OldText(String),
    NewTextChunk { chunk: String, done: bool },
}

#[derive(Debug)]
pub struct EditParser {
    state: EditParserState,
    buffer: String,
}

#[derive(Debug, PartialEq)]
enum EditParserState {
    Pending,
    WithinOldText,
    AfterOldText,
    WithinNewText { start: bool },
}

impl EditParser {
    pub fn new() -> Self {
        EditParser {
            state: EditParserState::Pending,
            buffer: String::new(),
        }
    }

    pub fn push(&mut self, chunk: &str) -> SmallVec<[EditParserEvent; 1]> {
        self.buffer.push_str(chunk);

        let mut edit_events = SmallVec::new();
        loop {
            match &mut self.state {
                EditParserState::Pending => {
                    if let Some(start) = self.buffer.find("<old_text>") {
                        self.buffer.drain(..start + "<old_text>".len());
                        self.state = EditParserState::WithinOldText;
                    } else {
                        break;
                    }
                }
                EditParserState::WithinOldText => {
                    if let Some(end) = self.buffer.find("</old_text>") {
                        let mut start = 0;
                        if self.buffer.starts_with('\n') {
                            start = 1;
                        }
                        let mut old_text = self.buffer[start..end].to_string();
                        if old_text.ends_with('\n') {
                            old_text.pop();
                        }

                        self.buffer.drain(..end + "</old_text>".len());
                        self.state = EditParserState::AfterOldText;
                        edit_events.push(EditParserEvent::OldText(old_text));
                    } else {
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
                    const NEW_TEXT_END_TAG: &str = "</new_text>";

                    if !self.buffer.is_empty() {
                        if *start && self.buffer.starts_with('\n') {
                            self.buffer.remove(0);
                        }
                        *start = false;
                    }

                    if let Some(end) = self.buffer.find(NEW_TEXT_END_TAG) {
                        let mut chunk = self.buffer[..end].to_string();
                        if chunk.ends_with('\n') {
                            chunk.pop();
                        }

                        edit_events.push(EditParserEvent::NewTextChunk { chunk, done: true });
                        self.buffer.drain(..end + NEW_TEXT_END_TAG.len());
                        self.state = EditParserState::Pending;
                    } else {
                        let mut end_prefixes = (1..NEW_TEXT_END_TAG.len())
                            .map(|i| &NEW_TEXT_END_TAG[..i])
                            .chain(["\n"]);
                        if end_prefixes.all(|prefix| !self.buffer.ends_with(&prefix)) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rand::prelude::*;
    use std::cmp;

    #[gpui::test(iterations = 1000)]
    fn test_single_edit(mut rng: StdRng) {
        assert_eq!(
            parse(
                "<old_text>original</old_text><new_text>updated</new_text>",
                &mut rng
            ),
            vec![Edit {
                old_text: "original".to_string(),
                new_text: "updated".to_string(),
            }]
        )
    }

    #[gpui::test(iterations = 1000)]
    fn test_multiple_edits(mut rng: StdRng) {
        assert_eq!(
            parse(
                indoc! {"
                    <old_text>
                    first old
                    </old_text><new_text>first new</new_text>
                    <old_text>second old</old_text><new_text>
                    second new
                    </new_text>
                "},
                &mut rng
            ),
            vec![
                Edit {
                    old_text: "first old".to_string(),
                    new_text: "first new".to_string(),
                },
                Edit {
                    old_text: "second old".to_string(),
                    new_text: "second new".to_string(),
                },
            ]
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_edits_with_extra_text(mut rng: StdRng) {
        assert_eq!(
            parse(
                indoc! {"
                    ignore this <old_text>
                    content</old_text>extra stuff<new_text>updated content</new_text>trailing data
                    more text <old_text>second item
                    </old_text>middle text<new_text>modified second item</new_text>end
                    <old_text>third case</old_text><new_text>improved third case</new_text> with trailing text
                "},
                &mut rng
            ),
            vec![
                Edit {
                    old_text: "content".to_string(),
                    new_text: "updated content".to_string(),
                },
                Edit {
                    old_text: "second item".to_string(),
                    new_text: "modified second item".to_string(),
                },
                Edit {
                    old_text: "third case".to_string(),
                    new_text: "improved third case".to_string(),
                },
            ]
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_nested_tags(mut rng: StdRng) {
        assert_eq!(
            parse(
                "<old_text>code with <tag>nested</tag> elements</old_text><new_text>new <code>content</code></new_text>",
                &mut rng
            ),
            vec![Edit {
                old_text: "code with <tag>nested</tag> elements".to_string(),
                new_text: "new <code>content</code>".to_string(),
            }]
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_empty_old_and_new_text(mut rng: StdRng) {
        assert_eq!(
            parse("<old_text></old_text><new_text></new_text>", &mut rng),
            vec![Edit {
                old_text: "".to_string(),
                new_text: "".to_string(),
            }]
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_with_special_characters(mut rng: StdRng) {
        assert_eq!(
            parse(
                "<old_text>function(x) { return x * 2; }</old_text><new_text>function(x) { return x ** 2; }</new_text>",
                &mut rng
            ),
            vec![Edit {
                old_text: "function(x) { return x * 2; }".to_string(),
                new_text: "function(x) { return x ** 2; }".to_string(),
            }]
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_multiline_content(mut rng: StdRng) {
        assert_eq!(
            parse(
                "<old_text>line1\nline2\nline3</old_text><new_text>line1\nmodified line2\nline3</new_text>",
                &mut rng
            ),
            vec![Edit {
                old_text: "line1\nline2\nline3".to_string(),
                new_text: "line1\nmodified line2\nline3".to_string(),
            }]
        );
    }

    #[gpui::test(iterations = 1000)]
    fn test_unmatched_tags(mut rng: StdRng) {
        assert_eq!(
            parse(
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
                "},
                &mut rng
            ),
            vec![Edit {
                old_text: "a\nb\nc".to_string(),
                new_text: "a\nB\nc".to_string(),
            }]
        );
    }

    #[derive(Default, Debug, PartialEq, Eq)]
    struct Edit {
        old_text: String,
        new_text: String,
    }

    fn parse(input: &str, rng: &mut StdRng) -> Vec<Edit> {
        let mut parser = EditParser::new();
        let chunk_count = rng.gen_range(1..=cmp::min(input.len(), 50));
        let mut chunk_indices = (0..input.len()).choose_multiple(rng, chunk_count);
        chunk_indices.sort();
        chunk_indices.push(input.len());

        let mut pending_edit = Edit::default();
        let mut edits = Vec::new();
        let mut last_ix = 0;
        for chunk_ix in chunk_indices {
            for event in parser.push(&input[last_ix..chunk_ix]) {
                match event {
                    EditParserEvent::OldText(old_text) => {
                        assert!(!old_text.contains("old_text"));
                        assert!(!old_text.contains("new_text"));
                        pending_edit.old_text = old_text;
                    }
                    EditParserEvent::NewTextChunk { chunk, done } => {
                        pending_edit.new_text.push_str(&chunk);
                        assert!(!pending_edit.new_text.contains("old_text"));
                        assert!(!pending_edit.new_text.contains("new_text"));

                        if done {
                            edits.push(pending_edit);
                            pending_edit = Edit::default();
                        }
                    }
                }
            }
            last_ix = chunk_ix;
        }
        edits
    }
}
