use crate::{Supermaven, SupermavenCompletionStateId};
use anyhow::Result;
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use futures::StreamExt as _;
use gpui::{App, Context, Entity, EntityId, Task};
use language::{Anchor, Buffer, BufferSnapshot};
use project::Project;
use std::{
    ops::{AddAssign, Range},
    path::Path,
    time::Duration,
};
use text::{ToOffset, ToPoint};
use unicode_segmentation::UnicodeSegmentation;

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub struct SupermavenCompletionProvider {
    supermaven: Entity<Supermaven>,
    buffer_id: Option<EntityId>,
    completion_id: Option<SupermavenCompletionStateId>,
    completion_text: Option<String>,
    file_extension: Option<String>,
    pending_refresh: Option<Task<Result<()>>>,
    completion_position: Option<language::Anchor>,
}

impl SupermavenCompletionProvider {
    pub fn new(supermaven: Entity<Supermaven>) -> Self {
        Self {
            supermaven,
            buffer_id: None,
            completion_id: None,
            completion_text: None,
            file_extension: None,
            pending_refresh: None,
            completion_position: None,
        }
    }
}

// Computes the edit prediction from the difference between the completion text.
// This is defined by greedily matching the buffer text against the completion text.
// Inlays are inserted for parts of the completion text that are not present in the buffer text.
// For example, given the completion text "axbyc" and the buffer text "xy", the rendered output in the editor would be "[a]x[b]y[c]".
// The parts in brackets are the inlays.
fn completion_from_diff(
    snapshot: BufferSnapshot,
    completion_text: &str,
    position: Anchor,
    delete_range: Range<Anchor>,
) -> EditPrediction {
    let buffer_text = snapshot.text_for_range(delete_range).collect::<String>();

    let mut edits: Vec<(Range<language::Anchor>, String)> = Vec::new();

    let completion_graphemes: Vec<&str> = completion_text.graphemes(true).collect();
    let buffer_graphemes: Vec<&str> = buffer_text.graphemes(true).collect();

    let mut offset = position.to_offset(&snapshot);

    let mut i = 0;
    let mut j = 0;
    while i < completion_graphemes.len() && j < buffer_graphemes.len() {
        // find the next instance of the buffer text in the completion text.
        let k = completion_graphemes[i..]
            .iter()
            .position(|c| *c == buffer_graphemes[j]);
        match k {
            Some(k) => {
                if k != 0 {
                    let offset = snapshot.anchor_after(offset);
                    // the range from the current position to item is an inlay.
                    let edit = (offset..offset, completion_graphemes[i..i + k].join(""));
                    edits.push(edit);
                }
                i += k + 1;
                j += 1;
                offset.add_assign(buffer_graphemes[j - 1].len());
            }
            None => {
                // there are no more matching completions, so drop the remaining
                // completion text as an inlay.
                break;
            }
        }
    }

    if j == buffer_graphemes.len() && i < completion_graphemes.len() {
        let offset = snapshot.anchor_after(offset);
        // there is leftover completion text, so drop it as an inlay.
        let edit_range = offset..offset;
        let edit_text = completion_graphemes[i..].join("");
        edits.push((edit_range, edit_text));
    }

    EditPrediction {
        id: None,
        edits,
        edit_preview: None,
    }
}

impl EditPredictionProvider for SupermavenCompletionProvider {
    fn name() -> &'static str {
        "supermaven"
    }

    fn display_name() -> &'static str {
        "Supermaven"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn supports_jump_to_edit() -> bool {
        false
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        self.supermaven.read(cx).is_enabled()
    }

    fn is_refreshing(&self) -> bool {
        self.pending_refresh.is_some() && self.completion_id.is_none()
    }

    fn refresh(
        &mut self,
        _project: Option<Entity<Project>>,
        buffer_handle: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        // Only make new completion requests when debounce is true (i.e., when text is typed)
        // When debounce is false (i.e., cursor movement), we should not make new requests
        if !debounce {
            return;
        }

        reset_completion_cache(self, cx);

        let Some(mut completion) = self.supermaven.update(cx, |supermaven, cx| {
            supermaven.complete(&buffer_handle, cursor_position, cx)
        }) else {
            return;
        };

        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            if debounce {
                cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
            }

            while let Some(()) = completion.updates.next().await {
                this.update(cx, |this, cx| {
                    // Get the completion text and cache it
                    if let Some(text) =
                        this.supermaven
                            .read(cx)
                            .completion(&buffer_handle, cursor_position, cx)
                    {
                        this.completion_text = Some(text.to_string());

                        this.completion_position = Some(cursor_position);
                    }

                    this.completion_id = Some(completion.id);
                    this.buffer_id = Some(buffer_handle.entity_id());
                    this.file_extension = buffer_handle.read(cx).file().and_then(|file| {
                        Some(
                            Path::new(file.file_name(cx))
                                .extension()?
                                .to_str()?
                                .to_string(),
                        )
                    });
                    cx.notify();
                })?;
            }
            Ok(())
        }));
    }

    fn cycle(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        reset_completion_cache(self, _cx);
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        reset_completion_cache(self, _cx);
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        if self.buffer_id != Some(buffer.entity_id()) {
            return None;
        }

        if self.completion_id.is_none() {
            return None;
        }

        let completion_text = if let Some(cached_text) = &self.completion_text {
            cached_text.as_str()
        } else {
            let text = self
                .supermaven
                .read(cx)
                .completion(buffer, cursor_position, cx)?;
            self.completion_text = Some(text.to_string());
            text
        };

        // Check if the cursor is still at the same position as the completion request
        // If we don't have a completion position stored, don't show the completion
        if let Some(completion_position) = self.completion_position {
            if cursor_position != completion_position {
                return None;
            }
        } else {
            return None;
        }

        let completion_text = trim_to_end_of_line_unless_leading_newline(completion_text);

        let completion_text = completion_text.trim_end();

        if !completion_text.trim().is_empty() {
            let snapshot = buffer.read(cx).snapshot();

            // Calculate the range from cursor to end of line correctly
            let cursor_point = cursor_position.to_point(&snapshot);
            let end_of_line = snapshot.anchor_after(language::Point::new(
                cursor_point.row,
                snapshot.line_len(cursor_point.row),
            ));
            let delete_range = cursor_position..end_of_line;

            Some(completion_from_diff(
                snapshot,
                completion_text,
                cursor_position,
                delete_range,
            ))
        } else {
            None
        }
    }
}

fn reset_completion_cache(
    provider: &mut SupermavenCompletionProvider,
    _cx: &mut Context<SupermavenCompletionProvider>,
) {
    provider.pending_refresh = None;
    provider.completion_id = None;
    provider.completion_text = None;
    provider.completion_position = None;
    provider.buffer_id = None;
}

fn trim_to_end_of_line_unless_leading_newline(text: &str) -> &str {
    if has_leading_newline(text) {
        text
    } else if let Some(i) = text.find('\n') {
        &text[..i]
    } else {
        text
    }
}

fn has_leading_newline(text: &str) -> bool {
    for c in text.chars() {
        if c == '\n' {
            return true;
        }
        if !c.is_whitespace() {
            return false;
        }
    }
    false
}
