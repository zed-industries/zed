use smallvec::SmallVec;

use super::{Edit, PartialEdit};

/// Events emitted by `StreamingParser` for edit-mode input.
#[derive(Debug, PartialEq, Eq)]
pub enum EditEvent {
    /// A chunk of `old_text` for an edit operation.
    OldTextChunk {
        edit_index: usize,
        chunk: String,
        done: bool,
    },
    /// A chunk of `new_text` for an edit operation.
    NewTextChunk {
        edit_index: usize,
        chunk: String,
        done: bool,
    },
}

/// Events emitted by `StreamingParser` for write-mode input.
#[derive(Debug, PartialEq, Eq)]
pub enum WriteEvent {
    /// A chunk of content for write/overwrite mode.
    ContentChunk { chunk: String },
}

/// Tracks the streaming state of a single edit to detect deltas.
#[derive(Default, Debug)]
struct EditStreamState {
    old_text_emitted_len: usize,
    old_text_done: bool,
    new_text_emitted_len: usize,
    new_text_done: bool,
    hold_until_complete: bool,
    buffer_new_text_until_old_text_done: bool,
}

/// Converts incrementally-growing tool call JSON into a stream of chunk events.
///
/// The tool call streaming infrastructure delivers partial JSON objects where
/// string fields grow over time. This parser compares consecutive partials,
/// computes the deltas, and emits `EditEvent`s or `WriteEvent`s that downstream
/// pipeline stages (`StreamingFuzzyMatcher` for old_text, `StreamingDiff` for
/// new_text) can consume incrementally.
///
/// Because partial JSON comes through a fixer (`partial-json-fixer`) that
/// closes incomplete escape sequences, a string can temporarily contain wrong
/// trailing characters (e.g. a literal `\` instead of `\n`).  We handle this
/// by holding back trailing backslash characters in non-finalized chunks: if
/// a partial string ends with `\` (0x5C), that byte is not emitted until the
/// next partial confirms or corrects it.  This avoids feeding corrupted bytes
/// to downstream consumers.
#[derive(Default, Debug)]
pub struct StreamingParser {
    edit_states: Vec<EditStreamState>,
    content_emitted_len: usize,
}

impl StreamingParser {
    /// Push a new set of partial edits (from edit mode) and return any events.
    ///
    /// Each call should pass the *entire current* edits array as seen in the
    /// latest partial input. The parser will diff it against its internal state
    /// to produce only the new events.
    pub fn push_edits(&mut self, edits: &[PartialEdit]) -> SmallVec<[EditEvent; 4]> {
        let mut events = SmallVec::new();

        for (index, partial) in edits.iter().enumerate() {
            if index >= self.edit_states.len() {
                // A new edit appeared — finalize the previous one if there was one.
                if let Some(previous) = self.finalize_previous_edit(
                    index,
                    edits
                        .get(index.saturating_sub(1))
                        .and_then(|edit| edit.old_text.as_deref()),
                    edits
                        .get(index.saturating_sub(1))
                        .and_then(|edit| edit.new_text.as_deref()),
                ) {
                    events.extend(previous);
                }
                self.edit_states.push(EditStreamState::default());
            }

            let state = &mut self.edit_states[index];

            if state.old_text_emitted_len == 0
                && state.new_text_emitted_len == 0
                && !state.old_text_done
                && partial.new_text.is_some()
                && !state.buffer_new_text_until_old_text_done
            {
                if partial
                    .old_text
                    .as_ref()
                    .is_some_and(|old_text| !old_text.is_empty())
                {
                    state.hold_until_complete = true;
                } else {
                    state.buffer_new_text_until_old_text_done = true;
                }
            }

            if state.hold_until_complete {
                continue;
            }

            // Process old_text changes.
            if let Some(old_text) = &partial.old_text
                && !state.old_text_done
            {
                if partial.new_text.is_some() && !state.buffer_new_text_until_old_text_done {
                    // new_text appeared after old_text, so old_text is done — emit everything.
                    let start = find_char_boundary(old_text, state.old_text_emitted_len);
                    let chunk = old_text[start..].to_string();
                    state.old_text_done = true;
                    state.old_text_emitted_len = old_text.len();
                    events.push(EditEvent::OldTextChunk {
                        edit_index: index,
                        chunk,
                        done: true,
                    });
                } else {
                    let safe_end = safe_emit_end_for_edit_text(old_text);
                    let safe_start = find_char_boundary(old_text, state.old_text_emitted_len);

                    if safe_end > safe_start {
                        let chunk = old_text[safe_start..safe_end].to_string();
                        state.old_text_emitted_len = safe_end;
                        events.push(EditEvent::OldTextChunk {
                            edit_index: index,
                            chunk,
                            done: false,
                        });
                    }
                }
            }

            // Process new_text changes.
            if let Some(new_text) = &partial.new_text
                && state.old_text_done
                && !state.new_text_done
            {
                let safe_end = safe_emit_end_for_edit_text(new_text);
                let safe_start = find_char_boundary(new_text, state.new_text_emitted_len);

                if safe_end > safe_start {
                    let chunk = new_text[safe_start..safe_end].to_string();
                    state.new_text_emitted_len = safe_end;
                    events.push(EditEvent::NewTextChunk {
                        edit_index: index,
                        chunk,
                        done: false,
                    });
                }
            }
        }

        events
    }

    /// Push new content and return any events.
    ///
    /// Each call should pass the *entire current* content string. The parser
    /// will diff it against its internal state to emit only the new chunk.
    pub fn push_content(&mut self, content: &str) -> SmallVec<[WriteEvent; 1]> {
        let mut events = SmallVec::new();

        let safe_end = safe_emit_end(content);
        let safe_start = find_char_boundary(content, self.content_emitted_len);
        if safe_end > safe_start {
            let chunk = content[safe_start..safe_end].to_string();
            self.content_emitted_len = safe_end;
            events.push(WriteEvent::ContentChunk { chunk });
        }

        events
    }

    /// Finalize all edits with the complete input. This emits `done: true`
    /// events for any in-progress old_text or new_text that hasn't been
    /// finalized yet.
    ///
    /// `final_edits` should be the fully deserialized final edits array. The
    /// parser compares against its tracked state and emits any remaining deltas
    /// with `done: true`.
    pub fn finalize_edits(&mut self, edits: &[Edit]) -> SmallVec<[EditEvent; 4]> {
        let mut events = SmallVec::new();

        for (index, edit) in edits.iter().enumerate() {
            if index >= self.edit_states.len() {
                // This edit was never seen in partials — emit it fully.
                if let Some(previous) = self.finalize_previous_edit(
                    index,
                    edits
                        .get(index.saturating_sub(1))
                        .map(|edit| edit.old_text.as_str()),
                    edits
                        .get(index.saturating_sub(1))
                        .map(|edit| edit.new_text.as_str()),
                ) {
                    events.extend(previous);
                }
                self.edit_states.push(EditStreamState::default());
            }

            let state = &mut self.edit_states[index];

            if state.hold_until_complete {
                state.old_text_done = true;
                state.old_text_emitted_len = edit.old_text.len();
                state.new_text_done = true;
                state.new_text_emitted_len = edit.new_text.len();
                state.hold_until_complete = false;
                state.buffer_new_text_until_old_text_done = false;
                events.push(EditEvent::OldTextChunk {
                    edit_index: index,
                    chunk: edit.old_text.clone(),
                    done: true,
                });
                events.push(EditEvent::NewTextChunk {
                    edit_index: index,
                    chunk: edit.new_text.clone(),
                    done: true,
                });
                continue;
            }

            if !state.old_text_done {
                let start = find_char_boundary(&edit.old_text, state.old_text_emitted_len);
                let chunk = edit.old_text[start..].to_string();
                state.old_text_done = true;
                state.old_text_emitted_len = edit.old_text.len();
                events.push(EditEvent::OldTextChunk {
                    edit_index: index,
                    chunk,
                    done: true,
                });
            }

            if !state.new_text_done {
                let start = find_char_boundary(&edit.new_text, state.new_text_emitted_len);
                let chunk = edit.new_text[start..].to_string();
                state.new_text_done = true;
                state.new_text_emitted_len = edit.new_text.len();
                events.push(EditEvent::NewTextChunk {
                    edit_index: index,
                    chunk,
                    done: true,
                });
            }
        }

        events
    }

    /// Finalize content with the complete input.
    pub fn finalize_content(&mut self, content: &str) -> SmallVec<[WriteEvent; 1]> {
        let mut events = SmallVec::new();

        let start = find_char_boundary(content, self.content_emitted_len);
        if content.len() > start {
            let chunk = content[start..].to_string();
            self.content_emitted_len = content.len();
            events.push(WriteEvent::ContentChunk { chunk });
        }

        events
    }

    /// When a new edit appears at `index`, finalize the edit at `index - 1`
    /// by emitting a `NewTextChunk { done: true }` if it hasn't been finalized.
    fn finalize_previous_edit(
        &mut self,
        new_index: usize,
        old_text: Option<&str>,
        new_text: Option<&str>,
    ) -> Option<SmallVec<[EditEvent; 2]>> {
        if new_index == 0 || self.edit_states.is_empty() {
            return None;
        }

        let previous_index = new_index - 1;
        if previous_index >= self.edit_states.len() {
            return None;
        }

        let state = &mut self.edit_states[previous_index];
        let mut events = SmallVec::new();

        if state.hold_until_complete {
            let old_text = old_text.unwrap_or_default();
            let new_text = new_text.unwrap_or_default();
            state.old_text_done = true;
            state.old_text_emitted_len = old_text.len();
            state.new_text_done = true;
            state.new_text_emitted_len = new_text.len();
            state.hold_until_complete = false;
            state.buffer_new_text_until_old_text_done = false;
            events.push(EditEvent::OldTextChunk {
                edit_index: previous_index,
                chunk: old_text.to_string(),
                done: true,
            });
            events.push(EditEvent::NewTextChunk {
                edit_index: previous_index,
                chunk: new_text.to_string(),
                done: true,
            });
            return Some(events);
        }

        if !state.old_text_done {
            let old_text = old_text.unwrap_or_default();
            let start = find_char_boundary(old_text, state.old_text_emitted_len);
            state.old_text_done = true;
            state.old_text_emitted_len = old_text.len();
            events.push(EditEvent::OldTextChunk {
                edit_index: previous_index,
                chunk: old_text[start..].to_string(),
                done: true,
            });
        }

        if !state.new_text_done {
            let new_text = new_text.unwrap_or_default();
            let start = find_char_boundary(new_text, state.new_text_emitted_len);
            state.new_text_done = true;
            state.new_text_emitted_len = new_text.len();
            state.buffer_new_text_until_old_text_done = false;
            events.push(EditEvent::NewTextChunk {
                edit_index: previous_index,
                chunk: new_text[start..].to_string(),
                done: true,
            });
        }

        Some(events)
    }
}

/// Returns the byte position up to which it is safe to emit from a partial
/// string.  If the string ends with a backslash (`\`, 0x5C), that byte is
/// held back because it may be an artifact of the partial JSON fixer closing
/// an incomplete escape sequence (e.g. turning a half-received `\n` into `\\`).
/// The next partial will reveal the correct character.
///
/// The returned position is always a valid UTF-8 character boundary.
fn safe_emit_end(text: &str) -> usize {
    if text.ends_with('\\') {
        text.len() - 1
    } else {
        text.len()
    }
}

fn safe_emit_end_for_edit_text(text: &str) -> usize {
    let safe_end = safe_emit_end(text);
    // Use string slicing to check the last character, ensuring we respect UTF-8 boundaries.
    if safe_end > 0 && text[..safe_end].ends_with('\n') {
        safe_end - 1
    } else {
        safe_end
    }
}

/// Finds a valid UTF-8 character boundary at or before the target position.
///
/// When streaming partial JSON, the text structure can change between updates
/// (e.g., an escape sequence being completed). This means a byte position that
/// was valid in one partial may land inside a multi-byte character in the next.
/// This function finds the nearest valid boundary at or before the target.
fn find_char_boundary(text: &str, target: usize) -> usize {
    if target >= text.len() {
        return text.len();
    }
    if text.is_char_boundary(target) {
        return target;
    }
    // Walk backwards to find a valid boundary.
    let mut pos = target;
    while pos > 0 && !text.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn emitted_len_inside_multibyte_char() -> impl Strategy<Value = (String, String)> {
        (1usize..8, prop::sample::select(&["。", "—", "é", "🦀"])).prop_map(
            |(emitted_len, multibyte_char)| {
                let first = "a".repeat(emitted_len);
                let second = format!("{}{}", "a".repeat(emitted_len - 1), multibyte_char);
                (first, second)
            },
        )
    }

    fn boundary_sensitive_text() -> impl Strategy<Value = String> {
        prop_oneof![
            emitted_len_inside_multibyte_char().prop_map(|(first, _)| first),
            emitted_len_inside_multibyte_char().prop_map(|(_, second)| second),
            prop::sample::select(&[
                "",
                "a",
                "ab",
                "ab\\",
                "a。",
                "a—",
                "hello,\\",
                "hello,\n",
                "hello,\nworld",
            ])
            .prop_map(ToString::to_string),
        ]
    }

    fn partial_edit() -> impl Strategy<Value = PartialEdit> {
        (
            prop::option::of(boundary_sensitive_text()),
            prop::option::of(boundary_sensitive_text()),
        )
            .prop_map(|(old_text, new_text)| PartialEdit { old_text, new_text })
    }

    #[test]
    fn test_first_edit_with_new_text_in_first_chunk_is_held_until_finalize() {
        let mut parser = StreamingParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("old".into()),
            new_text: Some("new".into()),
        }]);
        assert!(events.is_empty());

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("old text".into()),
            new_text: Some("new text".into()),
        }]);
        assert!(events.is_empty());

        let events = parser.finalize_edits(&[Edit {
            old_text: "old text".into(),
            new_text: "new text".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "old text".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "new text".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_single_edit_streamed_incrementally() {
        let mut parser = StreamingParser::default();

        // old_text arrives in chunks: "hell" → "hello w" → "hello world"
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hell".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "hell".into(),
                done: false,
            }]
        );

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello w".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "o w".into(),
                done: false,
            }]
        );

        // new_text appears → old_text finalizes
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello world".into()),
            new_text: Some("good".into()),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "orld".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "good".into(),
                    done: false,
                },
            ]
        );

        // new_text grows
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello world".into()),
            new_text: Some("goodbye world".into()),
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::NewTextChunk {
                edit_index: 0,
                chunk: "bye world".into(),
                done: false,
            }]
        );

        // Finalize
        let events = parser.finalize_edits(&[Edit {
            old_text: "hello world".into(),
            new_text: "goodbye world".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::NewTextChunk {
                edit_index: 0,
                chunk: "".into(),
                done: true,
            }]
        );
    }

    #[test]
    fn test_done_chunks_preserve_trailing_newlines() {
        let mut parser = StreamingParser::default();

        let events = parser.finalize_edits(&[Edit {
            old_text: "before\n".into(),
            new_text: "after\n".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "before\n".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "after\n".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_partial_edit_preserves_trailing_newlines() {
        let mut parser = StreamingParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("before\n".into()),
            new_text: Some("after\n".into()),
        }]);
        assert!(events.is_empty());

        let events = parser.finalize_edits(&[Edit {
            old_text: "before\n".into(),
            new_text: "after\n".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "before\n".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "after\n".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_multiple_edits_sequential() {
        let mut parser = StreamingParser::default();

        // First edit streams in
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("first old".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "first old".into(),
                done: false,
            }]
        );

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("first old".into()),
            new_text: Some("first new".into()),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "first new".into(),
                    done: false,
                },
            ]
        );

        // Second edit appears → first edit's new_text is finalized
        let events = parser.push_edits(&[
            PartialEdit {
                old_text: Some("first old".into()),
                new_text: Some("first new".into()),
            },
            PartialEdit {
                old_text: Some("second".into()),
                new_text: None,
            },
        ]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "".into(),
                    done: true,
                },
                EditEvent::OldTextChunk {
                    edit_index: 1,
                    chunk: "second".into(),
                    done: false,
                },
            ]
        );

        // Finalize everything
        let events = parser.finalize_edits(&[
            Edit {
                old_text: "first old".into(),
                new_text: "first new".into(),
            },
            Edit {
                old_text: "second old".into(),
                new_text: "second new".into(),
            },
        ]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 1,
                    chunk: " old".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 1,
                    chunk: "second new".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_content_streamed_incrementally() {
        let mut parser = StreamingParser::default();

        let events = parser.push_content("hello");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk {
                chunk: "hello".into(),
            }]
        );

        let events = parser.push_content("hello world");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk {
                chunk: " world".into(),
            }]
        );

        // No change
        let events = parser.push_content("hello world");
        assert!(events.is_empty());

        let events = parser.push_content("hello world!");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk { chunk: "!".into() }]
        );

        // Finalize with no additional content
        let events = parser.finalize_content("hello world!");
        assert!(events.is_empty());
    }

    #[test]
    fn test_finalize_content_with_remaining() {
        let mut parser = StreamingParser::default();

        parser.push_content("partial");
        let events = parser.finalize_content("partial content here");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk {
                chunk: " content here".into(),
            }]
        );
    }

    #[test]
    fn test_content_trailing_backslash_held_back() {
        let mut parser = StreamingParser::default();

        // Partial JSON fixer turns incomplete \n into \\ (literal backslash).
        // The trailing backslash is held back.
        let events = parser.push_content("hello,\\");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk {
                chunk: "hello,".into(),
            }]
        );

        // Next partial corrects the escape to an actual newline.
        // The held-back byte was wrong; the correct newline is emitted.
        let events = parser.push_content("hello,\n");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk { chunk: "\n".into() }]
        );

        // Normal growth.
        let events = parser.push_content("hello,\nworld");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk {
                chunk: "world".into(),
            }]
        );
    }

    #[test]
    fn test_content_finalize_with_trailing_backslash() {
        let mut parser = StreamingParser::default();

        // Stream a partial with a fixer-corrupted trailing backslash.
        // The backslash is held back.
        parser.push_content("abc\\");

        // Finalize reveals the correct character.
        let events = parser.finalize_content("abc\n");
        assert_eq!(
            events.as_slice(),
            &[WriteEvent::ContentChunk { chunk: "\n".into() }]
        );
    }

    proptest! {
        #[test]
        fn test_content_finalize_does_not_panic_when_emitted_len_lands_inside_multibyte_char(
            pair in emitted_len_inside_multibyte_char()
        ) {
            let (first, second) = pair;
            let mut parser = StreamingParser::default();

            parser.push_content(&first);
            parser.finalize_content(&second);
        }

        #[test]
        fn test_push_edits_does_not_panic_on_boundary_sensitive_sequences(
            partials in prop::collection::vec(prop::collection::vec(partial_edit(), 0..4), 1..12)
        ) {
            let mut parser = StreamingParser::default();

            for edits in partials {
                parser.push_edits(&edits);
            }
        }
    }

    #[test]
    fn test_no_partials_direct_finalize() {
        let mut parser = StreamingParser::default();

        let events = parser.finalize_edits(&[Edit {
            old_text: "old".into(),
            new_text: "new".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "old".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "new".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_no_partials_direct_finalize_multiple() {
        let mut parser = StreamingParser::default();

        let events = parser.finalize_edits(&[
            Edit {
                old_text: "first old".into(),
                new_text: "first new".into(),
            },
            Edit {
                old_text: "second old".into(),
                new_text: "second new".into(),
            },
        ]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "first old".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "first new".into(),
                    done: true,
                },
                EditEvent::OldTextChunk {
                    edit_index: 1,
                    chunk: "second old".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 1,
                    chunk: "second new".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_old_text_no_growth() {
        let mut parser = StreamingParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("same".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "same".into(),
                done: false,
            }]
        );

        // Same old_text, no new_text → no events
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("same".into()),
            new_text: None,
        }]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_old_text_none_then_appears() {
        let mut parser = StreamingParser::default();

        // Edit exists but old_text is None (field hasn't arrived yet)
        let events = parser.push_edits(&[PartialEdit {
            old_text: None,
            new_text: None,
        }]);
        assert!(events.is_empty());

        // old_text appears
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("text".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "text".into(),
                done: false,
            }]
        );
    }

    #[test]
    fn test_new_text_before_old_text_buffers_new_text_but_streams_old_text() {
        let mut parser = StreamingParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: None,
            new_text: Some("new".into()),
        }]);
        assert!(events.is_empty());

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("old".into()),
            new_text: Some("new".into()),
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "old".into(),
                done: false,
            }]
        );

        let events = parser.finalize_edits(&[Edit {
            old_text: "old".into(),
            new_text: "new".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "new".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_three_edits_streamed() {
        let mut parser = StreamingParser::default();

        // Stream first edit
        parser.push_edits(&[PartialEdit {
            old_text: Some("a".into()),
            new_text: Some("A".into()),
        }]);

        // Second edit appears
        parser.push_edits(&[
            PartialEdit {
                old_text: Some("a".into()),
                new_text: Some("A".into()),
            },
            PartialEdit {
                old_text: Some("b".into()),
                new_text: Some("B".into()),
            },
        ]);

        // Third edit appears
        let events = parser.push_edits(&[
            PartialEdit {
                old_text: Some("a".into()),
                new_text: Some("A".into()),
            },
            PartialEdit {
                old_text: Some("b".into()),
                new_text: Some("B".into()),
            },
            PartialEdit {
                old_text: Some("c".into()),
                new_text: None,
            },
        ]);

        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 1,
                    chunk: "b".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 1,
                    chunk: "B".into(),
                    done: true,
                },
                EditEvent::OldTextChunk {
                    edit_index: 2,
                    chunk: "c".into(),
                    done: false,
                },
            ]
        );

        // Finalize
        let events = parser.finalize_edits(&[
            Edit {
                old_text: "a".into(),
                new_text: "A".into(),
            },
            Edit {
                old_text: "b".into(),
                new_text: "B".into(),
            },
            Edit {
                old_text: "c".into(),
                new_text: "C".into(),
            },
        ]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 2,
                    chunk: "".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 2,
                    chunk: "C".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_finalize_with_unseen_old_text() {
        let mut parser = StreamingParser::default();

        // Only saw partial old_text, never saw new_text in partials
        parser.push_edits(&[PartialEdit {
            old_text: Some("partial".into()),
            new_text: None,
        }]);

        let events = parser.finalize_edits(&[Edit {
            old_text: "partial old text".into(),
            new_text: "replacement".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: " old text".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "replacement".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_repeated_pushes_with_no_change() {
        let mut parser = StreamingParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("stable".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "stable".into(),
                done: false,
            }]
        );

        // Push the exact same data again
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("stable".into()),
            new_text: None,
        }]);
        assert!(events.is_empty());

        // And again
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("stable".into()),
            new_text: None,
        }]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_old_text_trailing_backslash_held_back() {
        let mut parser = StreamingParser::default();

        // Partial-json-fixer produces a literal backslash when the JSON stream
        // cuts in the middle of an escape sequence like \n. The parser holds
        // back the trailing backslash instead of emitting it.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello,\\".into()), // fixer closed incomplete \n as \\
            new_text: None,
        }]);
        // The trailing `\` is held back — only "hello," is emitted.
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "hello,".into(),
                done: false,
            }]
        );

        // Next partial: the fixer corrects the escape to \n.
        // Because edit text also holds back a trailing newline, nothing new
        // is emitted yet.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello,\n".into()),
            new_text: None,
        }]);
        assert!(events.is_empty());

        // Continue normally. The held-back newline is emitted together with the
        // next content once it is no longer trailing.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello,\nworld".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "\nworld".into(),
                done: false,
            }]
        );
    }

    #[test]
    fn test_multiline_old_and_new_text() {
        let mut parser = StreamingParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("line1\nline2".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "line1\nline2".into(),
                done: false,
            }]
        );

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("line1\nline2\nline3".into()),
            new_text: Some("LINE1\n".into()),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                EditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "\nline3".into(),
                    done: true,
                },
                EditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "LINE1".into(),
                    done: false,
                },
            ]
        );

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("line1\nline2\nline3".into()),
            new_text: Some("LINE1\nLINE2\nLINE3".into()),
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::NewTextChunk {
                edit_index: 0,
                chunk: "\nLINE2\nLINE3".into(),
                done: false,
            }]
        );
    }

    #[test]
    fn test_multibyte_char_with_trailing_backslash() {
        // Reproduces a panic where the stored `old_text_emitted_len` from a previous
        // partial lands inside a multi-byte UTF-8 character in the current partial.
        //
        // Scenario: The JSON fixer produces a literal backslash when the stream cuts
        // mid-escape. If the *next* partial replaces that backslash with a multi-byte
        // character (e.g., em-dash '—'), the stored byte position is no longer valid.
        let mut parser = StreamingParser::default();

        // First partial: text ends with backslash (held back by safe_emit_end).
        // "abc" = 3 bytes, backslash held back, so emitted_len = 3.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("abc\\".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "abc".into(),
                done: false,
            }]
        );

        // Second partial: the backslash is replaced by em-dash '—' (3 bytes: E2 80 94).
        // "ab—" = 2 + 3 = 5 bytes total, with em-dash at bytes 2..5.
        // The stored emitted_len (3) is inside the em-dash!
        // This should NOT panic.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("ab—".into()),
            new_text: None,
        }]);
        // The parser should handle this gracefully.
        let _ = events;
    }

    #[test]
    fn test_emitted_len_inside_multibyte_char_boundary() {
        // More direct reproduction: emitted_len points inside a multi-byte character.
        //
        // This can happen when:
        // 1. First partial has text where byte N is a valid boundary
        // 2. Second partial has *different* text where byte N is inside a multi-byte char
        let mut parser = StreamingParser::default();

        // First partial: "ab" (2 bytes), backslash held back.
        // After processing: emitted_len = 2
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("ab\\".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[EditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "ab".into(),
                done: false,
            }]
        );

        // Second partial: "a—" where em-dash starts at byte 1 and spans bytes 1-3.
        // Stored emitted_len = 2, but byte 2 is inside the em-dash!
        // This should NOT panic.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("a—".into()),
            new_text: None,
        }]);
        // The parser should handle this gracefully.
        // We don't care exactly what it emits, just that it doesn't panic.
        let _ = events;
    }
}
