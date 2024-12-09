use crate::{Supermaven, SupermavenCompletionStateId};
use anyhow::Result;
use futures::StreamExt as _;
use gpui::{AppContext, EntityId, Model, ModelContext, Task};
use inline_completion::{Direction, InlineCompletion, InlineCompletionProvider};
use language::{language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot};
use std::{
    ops::{AddAssign, Range},
    path::Path,
    time::Duration,
};
use text::{ToOffset, ToPoint};
use unicode_segmentation::UnicodeSegmentation;

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub struct SupermavenCompletionProvider {
    supermaven: Model<Supermaven>,
    buffer_id: Option<EntityId>,
    completion_id: Option<SupermavenCompletionStateId>,
    file_extension: Option<String>,
    pending_refresh: Task<Result<()>>,
}

impl SupermavenCompletionProvider {
    pub fn new(supermaven: Model<Supermaven>) -> Self {
        Self {
            supermaven,
            buffer_id: None,
            completion_id: None,
            file_extension: None,
            pending_refresh: Task::ready(Ok(())),
        }
    }
}

// Computes the inline completion from the difference between the completion text.
// this is defined by greedily matching the buffer text against the completion text, with any leftover buffer placed at the end.
// for example, given the completion text "moo cows are cool" and the buffer text "cowsre pool", the completion state would be
// the inlays "moo ", " a", and "cool" which will render as "[moo ]cows[ a]re [cool]pool" in the editor.
fn completion_from_diff(
    snapshot: BufferSnapshot,
    completion_text: &str,
    position: Anchor,
    delete_range: Range<Anchor>,
) -> InlineCompletion {
    let buffer_text = snapshot
        .text_for_range(delete_range.clone())
        .collect::<String>();

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

    InlineCompletion { edits }
}

impl InlineCompletionProvider for SupermavenCompletionProvider {
    fn name() -> &'static str {
        "supermaven"
    }

    fn is_enabled(&self, buffer: &Model<Buffer>, cursor_position: Anchor, cx: &AppContext) -> bool {
        if !self.supermaven.read(cx).is_enabled() {
            return false;
        }

        let buffer = buffer.read(cx);
        let file = buffer.file();
        let language = buffer.language_at(cursor_position);
        let settings = all_language_settings(file, cx);
        settings.inline_completions_enabled(language.as_ref(), file.map(|f| f.path().as_ref()), cx)
    }

    fn refresh(
        &mut self,
        buffer_handle: Model<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(mut completion) = self.supermaven.update(cx, |supermaven, cx| {
            supermaven.complete(&buffer_handle, cursor_position, cx)
        }) else {
            return;
        };

        self.pending_refresh = cx.spawn(|this, mut cx| async move {
            if debounce {
                cx.background_executor().timer(DEBOUNCE_TIMEOUT).await;
            }

            while let Some(()) = completion.updates.next().await {
                this.update(&mut cx, |this, cx| {
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
        });
    }

    fn cycle(
        &mut self,
        _buffer: Model<Buffer>,
        _cursor_position: Anchor,
        _direction: Direction,
        _cx: &mut ModelContext<Self>,
    ) {
    }

    fn accept(&mut self, _cx: &mut ModelContext<Self>) {
        self.pending_refresh = Task::ready(Ok(()));
        self.completion_id = None;
    }

    fn discard(&mut self, _cx: &mut ModelContext<Self>) {
        self.pending_refresh = Task::ready(Ok(()));
        self.completion_id = None;
    }

    fn suggest(
        &mut self,
        buffer: &Model<Buffer>,
        cursor_position: Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Option<InlineCompletion> {
        let completion_text = self
            .supermaven
            .read(cx)
            .completion(buffer, cursor_position, cx)?;

        let completion_text = trim_to_end_of_line_unless_leading_newline(completion_text);

        let completion_text = completion_text.trim_end();

        if !completion_text.trim().is_empty() {
            let snapshot = buffer.read(cx).snapshot();
            let mut point = cursor_position.to_point(&snapshot);
            point.column = snapshot.line_len(point.row);
            let range = cursor_position..snapshot.anchor_after(point);
            Some(completion_from_diff(
                snapshot,
                completion_text,
                cursor_position,
                range,
            ))
        } else {
            None
        }
    }
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
