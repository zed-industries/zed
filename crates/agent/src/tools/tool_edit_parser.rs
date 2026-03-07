use smallvec::SmallVec;

use crate::{Edit, PartialEdit};

/// Events emitted by `ToolEditParser` as tool call input streams in.
#[derive(Debug, PartialEq, Eq)]
pub enum ToolEditEvent {
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
}

/// Converts incrementally-growing tool call JSON into a stream of chunk events.
///
/// The tool call streaming infrastructure delivers partial JSON objects where
/// string fields grow over time. This parser compares consecutive partials,
/// computes the deltas, and emits `ToolEditEvent`s that downstream pipeline
/// stages (`StreamingFuzzyMatcher` for old_text, `StreamingDiff` for new_text)
/// can consume incrementally.
///
/// Because partial JSON comes through a fixer (`partial-json-fixer`) that
/// closes incomplete escape sequences, a string can temporarily contain wrong
/// trailing characters (e.g. a literal `\` instead of `\n`).  We handle this
/// by holding back trailing backslash characters in non-finalized chunks: if
/// a partial string ends with `\` (0x5C), that byte is not emitted until the
/// next partial confirms or corrects it.  This avoids feeding corrupted bytes
/// to downstream consumers.
#[derive(Default, Debug)]
pub struct ToolEditParser {
    edit_states: Vec<EditStreamState>,
    content_emitted_len: usize,
}

impl ToolEditParser {
    /// Push a new set of partial edits (from edit mode) and return any events.
    ///
    /// Each call should pass the *entire current* edits array as seen in the
    /// latest partial input. The parser will diff it against its internal state
    /// to produce only the new events.
    pub fn push_edits(&mut self, edits: &[PartialEdit]) -> SmallVec<[ToolEditEvent; 4]> {
        let mut events = SmallVec::new();

        for (index, partial) in edits.iter().enumerate() {
            if index >= self.edit_states.len() {
                // A new edit appeared — finalize the previous one if there was one.
                if let Some(previous) = self.finalize_previous_edit(index) {
                    events.extend(previous);
                }
                self.edit_states.push(EditStreamState::default());
            }

            let state = &mut self.edit_states[index];

            // Process old_text changes.
            if let Some(old_text) = &partial.old_text
                && !state.old_text_done
            {
                if partial.new_text.is_some() {
                    // new_text appeared, so old_text is done — emit everything.
                    let start = state.old_text_emitted_len.min(old_text.len());
                    let chunk = old_text[start..].to_string();
                    state.old_text_done = true;
                    state.old_text_emitted_len = old_text.len();
                    events.push(ToolEditEvent::OldTextChunk {
                        edit_index: index,
                        chunk,
                        done: true,
                    });
                } else {
                    let safe_end = safe_emit_end(old_text);
                    if safe_end > state.old_text_emitted_len {
                        let chunk = old_text[state.old_text_emitted_len..safe_end].to_string();
                        state.old_text_emitted_len = safe_end;
                        events.push(ToolEditEvent::OldTextChunk {
                            edit_index: index,
                            chunk,
                            done: false,
                        });
                    }
                }
            }

            // Process new_text changes.
            if let Some(new_text) = &partial.new_text
                && !state.new_text_done
            {
                let safe_end = safe_emit_end(new_text);
                if safe_end > state.new_text_emitted_len {
                    let chunk = new_text[state.new_text_emitted_len..safe_end].to_string();
                    state.new_text_emitted_len = safe_end;
                    events.push(ToolEditEvent::NewTextChunk {
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
    pub fn push_content(&mut self, content: &str) -> SmallVec<[ToolEditEvent; 1]> {
        let mut events = SmallVec::new();

        let safe_end = safe_emit_end(content);
        if safe_end > self.content_emitted_len {
            let chunk = content[self.content_emitted_len..safe_end].to_string();
            self.content_emitted_len = safe_end;
            events.push(ToolEditEvent::ContentChunk { chunk });
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
    pub fn finalize_edits(&mut self, edits: &[Edit]) -> SmallVec<[ToolEditEvent; 4]> {
        let mut events = SmallVec::new();

        for (index, edit) in edits.iter().enumerate() {
            if index >= self.edit_states.len() {
                // This edit was never seen in partials — emit it fully.
                if let Some(previous) = self.finalize_previous_edit(index) {
                    events.extend(previous);
                }
                self.edit_states.push(EditStreamState::default());
            }

            let state = &mut self.edit_states[index];

            if !state.old_text_done {
                let start = state.old_text_emitted_len.min(edit.old_text.len());
                let chunk = edit.old_text[start..].to_string();
                state.old_text_done = true;
                state.old_text_emitted_len = edit.old_text.len();
                events.push(ToolEditEvent::OldTextChunk {
                    edit_index: index,
                    chunk,
                    done: true,
                });
            }

            if !state.new_text_done {
                let start = state.new_text_emitted_len.min(edit.new_text.len());
                let chunk = edit.new_text[start..].to_string();
                state.new_text_done = true;
                state.new_text_emitted_len = edit.new_text.len();
                events.push(ToolEditEvent::NewTextChunk {
                    edit_index: index,
                    chunk,
                    done: true,
                });
            }
        }

        events
    }

    /// Finalize content with the complete input.
    pub fn finalize_content(&mut self, content: &str) -> SmallVec<[ToolEditEvent; 1]> {
        let mut events = SmallVec::new();

        let start = self.content_emitted_len.min(content.len());
        if content.len() > start {
            let chunk = content[start..].to_string();
            self.content_emitted_len = content.len();
            events.push(ToolEditEvent::ContentChunk { chunk });
        }

        events
    }

    /// When a new edit appears at `index`, finalize the edit at `index - 1`
    /// by emitting a `NewTextChunk { done: true }` if it hasn't been finalized.
    fn finalize_previous_edit(&mut self, new_index: usize) -> Option<SmallVec<[ToolEditEvent; 2]>> {
        if new_index == 0 || self.edit_states.is_empty() {
            return None;
        }

        let previous_index = new_index - 1;
        if previous_index >= self.edit_states.len() {
            return None;
        }

        let state = &mut self.edit_states[previous_index];
        let mut events = SmallVec::new();

        // If old_text was never finalized, finalize it now with an empty done chunk.
        if !state.old_text_done {
            state.old_text_done = true;
            events.push(ToolEditEvent::OldTextChunk {
                edit_index: previous_index,
                chunk: String::new(),
                done: true,
            });
        }

        // Emit a done event for new_text if not already finalized.
        if !state.new_text_done {
            state.new_text_done = true;
            events.push(ToolEditEvent::NewTextChunk {
                edit_index: previous_index,
                chunk: String::new(),
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
fn safe_emit_end(text: &str) -> usize {
    if text.as_bytes().last() == Some(&b'\\') {
        text.len() - 1
    } else {
        text.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_edit_streamed_incrementally() {
        let mut parser = ToolEditParser::default();

        // old_text arrives in chunks: "hell" → "hello w" → "hello world"
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hell".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::OldTextChunk {
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
            &[ToolEditEvent::OldTextChunk {
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
                ToolEditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "orld".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
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
            &[ToolEditEvent::NewTextChunk {
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
            &[ToolEditEvent::NewTextChunk {
                edit_index: 0,
                chunk: "".into(),
                done: true,
            }]
        );
    }

    #[test]
    fn test_multiple_edits_sequential() {
        let mut parser = ToolEditParser::default();

        // First edit streams in
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("first old".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::OldTextChunk {
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
                ToolEditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
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
                ToolEditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "".into(),
                    done: true,
                },
                ToolEditEvent::OldTextChunk {
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
                ToolEditEvent::OldTextChunk {
                    edit_index: 1,
                    chunk: " old".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 1,
                    chunk: "second new".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_content_streamed_incrementally() {
        let mut parser = ToolEditParser::default();

        let events = parser.push_content("hello");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk {
                chunk: "hello".into(),
            }]
        );

        let events = parser.push_content("hello world");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk {
                chunk: " world".into(),
            }]
        );

        // No change
        let events = parser.push_content("hello world");
        assert!(events.is_empty());

        let events = parser.push_content("hello world!");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk { chunk: "!".into() }]
        );

        // Finalize with no additional content
        let events = parser.finalize_content("hello world!");
        assert!(events.is_empty());
    }

    #[test]
    fn test_finalize_content_with_remaining() {
        let mut parser = ToolEditParser::default();

        parser.push_content("partial");
        let events = parser.finalize_content("partial content here");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk {
                chunk: " content here".into(),
            }]
        );
    }

    #[test]
    fn test_content_trailing_backslash_held_back() {
        let mut parser = ToolEditParser::default();

        // Partial JSON fixer turns incomplete \n into \\ (literal backslash).
        // The trailing backslash is held back.
        let events = parser.push_content("hello,\\");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk {
                chunk: "hello,".into(),
            }]
        );

        // Next partial corrects the escape to an actual newline.
        // The held-back byte was wrong; the correct newline is emitted.
        let events = parser.push_content("hello,\n");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk { chunk: "\n".into() }]
        );

        // Normal growth.
        let events = parser.push_content("hello,\nworld");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk {
                chunk: "world".into(),
            }]
        );
    }

    #[test]
    fn test_content_finalize_with_trailing_backslash() {
        let mut parser = ToolEditParser::default();

        // Stream a partial with a fixer-corrupted trailing backslash.
        // The backslash is held back.
        parser.push_content("abc\\");

        // Finalize reveals the correct character.
        let events = parser.finalize_content("abc\n");
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::ContentChunk { chunk: "\n".into() }]
        );
    }

    #[test]
    fn test_no_partials_direct_finalize() {
        let mut parser = ToolEditParser::default();

        let events = parser.finalize_edits(&[Edit {
            old_text: "old".into(),
            new_text: "new".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                ToolEditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "old".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "new".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_no_partials_direct_finalize_multiple() {
        let mut parser = ToolEditParser::default();

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
                ToolEditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "first old".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "first new".into(),
                    done: true,
                },
                ToolEditEvent::OldTextChunk {
                    edit_index: 1,
                    chunk: "second old".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 1,
                    chunk: "second new".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_old_text_no_growth() {
        let mut parser = ToolEditParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("same".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::OldTextChunk {
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
        let mut parser = ToolEditParser::default();

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
            &[ToolEditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "text".into(),
                done: false,
            }]
        );
    }

    #[test]
    fn test_empty_old_text_with_new_text() {
        let mut parser = ToolEditParser::default();

        // old_text is empty, new_text appears immediately
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("".into()),
            new_text: Some("inserted".into()),
        }]);
        assert_eq!(
            events.as_slice(),
            &[
                ToolEditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "inserted".into(),
                    done: false,
                },
            ]
        );
    }

    #[test]
    fn test_three_edits_streamed() {
        let mut parser = ToolEditParser::default();

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

        // Should finalize edit 1 (index=1) and start edit 2 (index=2)
        assert_eq!(
            events.as_slice(),
            &[
                ToolEditEvent::NewTextChunk {
                    edit_index: 1,
                    chunk: "".into(),
                    done: true,
                },
                ToolEditEvent::OldTextChunk {
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
                ToolEditEvent::OldTextChunk {
                    edit_index: 2,
                    chunk: "".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 2,
                    chunk: "C".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_finalize_with_unseen_old_text() {
        let mut parser = ToolEditParser::default();

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
                ToolEditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: " old text".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "replacement".into(),
                    done: true,
                },
            ]
        );
    }

    #[test]
    fn test_finalize_with_partially_seen_new_text() {
        let mut parser = ToolEditParser::default();

        parser.push_edits(&[PartialEdit {
            old_text: Some("old".into()),
            new_text: Some("partial".into()),
        }]);

        let events = parser.finalize_edits(&[Edit {
            old_text: "old".into(),
            new_text: "partial new text".into(),
        }]);
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::NewTextChunk {
                edit_index: 0,
                chunk: " new text".into(),
                done: true,
            }]
        );
    }

    #[test]
    fn test_repeated_pushes_with_no_change() {
        let mut parser = ToolEditParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("stable".into()),
            new_text: Some("also stable".into()),
        }]);
        assert_eq!(events.len(), 2); // old done + new chunk

        // Push the exact same data again
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("stable".into()),
            new_text: Some("also stable".into()),
        }]);
        assert!(events.is_empty());

        // And again
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("stable".into()),
            new_text: Some("also stable".into()),
        }]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_old_text_trailing_backslash_held_back() {
        let mut parser = ToolEditParser::default();

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
            &[ToolEditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "hello,".into(),
                done: false,
            }]
        );

        // Next partial: the fixer corrects the escape to \n.
        // The held-back byte was wrong, but we never emitted it. Now the
        // correct newline at that position is emitted normally.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello,\n".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "\n".into(),
                done: false,
            }]
        );

        // Continue normally.
        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("hello,\nworld".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::OldTextChunk {
                edit_index: 0,
                chunk: "world".into(),
                done: false,
            }]
        );
    }

    #[test]
    fn test_multiline_old_and_new_text() {
        let mut parser = ToolEditParser::default();

        let events = parser.push_edits(&[PartialEdit {
            old_text: Some("line1\nline2".into()),
            new_text: None,
        }]);
        assert_eq!(
            events.as_slice(),
            &[ToolEditEvent::OldTextChunk {
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
                ToolEditEvent::OldTextChunk {
                    edit_index: 0,
                    chunk: "\nline3".into(),
                    done: true,
                },
                ToolEditEvent::NewTextChunk {
                    edit_index: 0,
                    chunk: "LINE1\n".into(),
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
            &[ToolEditEvent::NewTextChunk {
                edit_index: 0,
                chunk: "LINE2\nLINE3".into(),
                done: false,
            }]
        );
    }
}
