use std::sync::OnceLock;

use regex::Regex;
use smallvec::SmallVec;
use util::debug_panic;

static START_MARKER: OnceLock<Regex> = OnceLock::new();
static END_MARKER: OnceLock<Regex> = OnceLock::new();

#[derive(Debug)]
pub enum CreateFileParserEvent {
    NewTextChunk { chunk: String },
}

#[derive(Debug)]
pub struct CreateFileParser {
    state: ParserState,
    buffer: String,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    Pending,
    WithinText,
    Finishing,
    Finished,
}

impl CreateFileParser {
    pub fn new() -> Self {
        CreateFileParser {
            state: ParserState::Pending,
            buffer: String::new(),
        }
    }

    pub fn push(&mut self, chunk: Option<&str>) -> SmallVec<[CreateFileParserEvent; 1]> {
        if chunk.is_none() {
            self.state = ParserState::Finishing;
        }

        let chunk = chunk.unwrap_or_default();

        self.buffer.push_str(chunk);

        let mut edit_events = SmallVec::new();
        let start_marker_regex = START_MARKER.get_or_init(|| Regex::new(r"\n?```\S*\n").unwrap());
        let end_marker_regex = END_MARKER.get_or_init(|| Regex::new(r"(^|\n)```\s*$").unwrap());
        loop {
            match &mut self.state {
                ParserState::Pending => {
                    if let Some(m) = start_marker_regex.find(&self.buffer) {
                        self.buffer.drain(..m.end());
                        self.state = ParserState::WithinText;
                    } else {
                        break;
                    }
                }
                ParserState::WithinText => {
                    let text = self.buffer.trim_end_matches(&['`', '\n', ' ']);
                    let text_len = text.len();

                    if text_len > 0 {
                        edit_events.push(CreateFileParserEvent::NewTextChunk {
                            chunk: self.buffer.drain(..text_len).collect(),
                        });
                    }
                    break;
                }
                ParserState::Finishing => {
                    if let Some(m) = end_marker_regex.find(&self.buffer) {
                        self.buffer.drain(m.start()..);
                    }
                    if !self.buffer.is_empty() {
                        if !self.buffer.ends_with('\n') {
                            self.buffer.push('\n');
                        }
                        edit_events.push(CreateFileParserEvent::NewTextChunk {
                            chunk: self.buffer.drain(..).collect(),
                        });
                    }
                    self.state = ParserState::Finished;
                    break;
                }
                ParserState::Finished => debug_panic!("Can't call parser after finishing"),
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

    #[gpui::test(iterations = 100)]
    fn test_happy_path(mut rng: StdRng) {
        let mut parser = CreateFileParser::new();
        assert_eq!(
            parse_random_chunks("```\nHello world\n```", &mut parser, &mut rng),
            "Hello world".to_string()
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_cut_prefix(mut rng: StdRng) {
        let mut parser = CreateFileParser::new();
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    Let me write this file for you:

                    ```
                    Hello world
                    ```

                "},
                &mut parser,
                &mut rng
            ),
            "Hello world".to_string()
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_language_name_on_fences(mut rng: StdRng) {
        let mut parser = CreateFileParser::new();
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    ```rust
                    Hello world
                    ```

                "},
                &mut parser,
                &mut rng
            ),
            "Hello world".to_string()
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_leave_suffix(mut rng: StdRng) {
        let mut parser = CreateFileParser::new();
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    Let me write this file for you:

                    ```
                    Hello world
                    ```

                    The end
                "},
                &mut parser,
                &mut rng
            ),
            // This output is marlformed, so we're doing our best effort
            "Hello world\n```\n\nThe end\n".to_string()
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_inner_fences(mut rng: StdRng) {
        let mut parser = CreateFileParser::new();
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    Let me write this file for you:

                    ```
                    ```
                    Hello world
                    ```
                    ```
                "},
                &mut parser,
                &mut rng
            ),
            // This output is marlformed, so we're doing our best effort
            "```\nHello world\n```\n".to_string()
        );
    }

    #[gpui::test(iterations = 10)]
    fn test_empty_file(mut rng: StdRng) {
        let mut parser = CreateFileParser::new();
        assert_eq!(
            parse_random_chunks(
                indoc! {"
                    ```
                    ```
                "},
                &mut parser,
                &mut rng
            ),
            "".to_string()
        );
    }

    fn parse_random_chunks(input: &str, parser: &mut CreateFileParser, rng: &mut StdRng) -> String {
        let chunk_count = rng.gen_range(1..=cmp::min(input.len(), 50));
        let mut chunk_indices = (0..input.len()).choose_multiple(rng, chunk_count);
        chunk_indices.sort();
        chunk_indices.push(input.len());

        let chunk_indices = chunk_indices
            .into_iter()
            .map(Some)
            .chain(vec![None])
            .collect::<Vec<Option<usize>>>();

        let mut edit = String::default();
        let mut last_ix = 0;
        for chunk_ix in chunk_indices {
            let mut chunk = None;
            if let Some(chunk_ix) = chunk_ix {
                chunk = Some(&input[last_ix..chunk_ix]);
                last_ix = chunk_ix;
            }

            for event in parser.push(chunk) {
                match event {
                    CreateFileParserEvent::NewTextChunk { chunk } => {
                        edit.push_str(&chunk);
                    }
                }
            }
        }
        edit
    }
}
