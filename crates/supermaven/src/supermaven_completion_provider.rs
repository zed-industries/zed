use crate::{Supermaven, SupermavenCompletionStateId};
use anyhow::Result;
use client::telemetry::Telemetry;
use editor::{CompletionProposal, Direction, InlayProposal, InlineCompletionProvider};
use futures::StreamExt as _;
use gpui::{AppContext, EntityId, Model, ModelContext, Task};
use language::{language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot};
use std::{ops::Range, path::Path, sync::Arc, time::Duration};
use text::{ToOffset, ToPoint};

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub struct SupermavenCompletionProvider {
    supermaven: Model<Supermaven>,
    buffer_id: Option<EntityId>,
    completion_id: Option<SupermavenCompletionStateId>,
    file_extension: Option<String>,
    pending_refresh: Task<Result<()>>,
    telemetry: Option<Arc<Telemetry>>,
}

impl SupermavenCompletionProvider {
    pub fn new(supermaven: Model<Supermaven>) -> Self {
        Self {
            supermaven,
            buffer_id: None,
            completion_id: None,
            file_extension: None,
            pending_refresh: Task::ready(Ok(())),
            telemetry: None,
        }
    }

    pub fn with_telemetry(mut self, telemetry: Arc<Telemetry>) -> Self {
        self.telemetry = Some(telemetry);
        self
    }
}

// Computes the completion state from the difference between the completion text.
// this is defined by greedily matching the buffer text against the completion text, with any leftover buffer placed at the end.
// for example, given the completion text "moo cows are cool" and the buffer text "cowsre pool", the completion state would be
// the inlays "moo ", " a", and "cool" which will render as "[moo ]cows[ a]re [cool]pool" in the editor.
fn completion_state_from_diff(
    snapshot: BufferSnapshot,
    completion_text: &str,
    position: Anchor,
    delete_range: Range<Anchor>,
) -> CompletionProposal {
    let buffer_text = snapshot
        .text_for_range(delete_range.clone())
        .collect::<String>();
    let mut compl_ix_converter = StringIndexConverter::new(completion_text);
    let mut buffer_ix_converter = StringIndexConverter::new(buffer_text);

    let mut inlays: Vec<InlayProposal> = Vec::new();
    let mut offset = position.to_offset(&snapshot);
    while compl_ix_converter.can_loop() && buffer_ix_converter.can_loop() {
        // find the next instance of the buffer text in the completion text.
        match compl_ix_converter.has_char_from_current_ix(buffer_ix_converter.current_char()) {
            Some((utf8_dist, char_dist)) => {
                // the range from the current position to item is an inlay.
                if utf8_dist != 0 {
                    debug_assert!(char_dist != 0);
                    inlays.push(InlayProposal::Suggestion(
                        snapshot.anchor_after(offset),
                        completion_text
                            [compl_ix_converter.utf8_ix..compl_ix_converter.utf8_ix + utf8_dist]
                            .into(),
                    ));
                }
                compl_ix_converter.update_ix(utf8_dist + 1, char_dist + 1);
                buffer_ix_converter.update_ix(1, 1);
                offset = snapshot.clip_offset(offset + 1, text::Bias::Right);
            }
            None => {
                // there are no more matching completions, so drop the remaining
                // completion text as an inlay.
                break;
            }
        }
    }

    if buffer_ix_converter.utf8_ix == buffer_ix_converter.text.len()
        && compl_ix_converter.utf8_ix < completion_text.len()
    {
        // there is leftover completion text, so drop it as an inlay.
        inlays.push(InlayProposal::Suggestion(
            snapshot.anchor_after(offset),
            completion_text[compl_ix_converter.utf8_ix..completion_text.len()].into(),
        ));
    }

    CompletionProposal {
        inlays,
        text: completion_text.into(),
        delete_range: Some(delete_range),
    }
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
        settings.inline_completions_enabled(language.as_ref(), file.map(|f| f.path().as_ref()))
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
        if self.completion_id.is_some() {
            if let Some(telemetry) = self.telemetry.as_ref() {
                telemetry.report_inline_completion_event(
                    Self::name().to_string(),
                    true,
                    self.file_extension.clone(),
                );
            }
        }
        self.pending_refresh = Task::ready(Ok(()));
        self.completion_id = None;
    }

    fn discard(
        &mut self,
        should_report_inline_completion_event: bool,
        _cx: &mut ModelContext<Self>,
    ) {
        if should_report_inline_completion_event && self.completion_id.is_some() {
            if let Some(telemetry) = self.telemetry.as_ref() {
                telemetry.report_inline_completion_event(
                    Self::name().to_string(),
                    false,
                    self.file_extension.clone(),
                );
            }
        }

        self.pending_refresh = Task::ready(Ok(()));
        self.completion_id = None;
    }

    fn active_completion_text<'a>(
        &'a self,
        buffer: &Model<Buffer>,
        cursor_position: Anchor,
        cx: &'a AppContext,
    ) -> Option<CompletionProposal> {
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
            Some(completion_state_from_diff(
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

struct StringIndexConverter {
    text: String,
    chars: Vec<char>,
    utf8_ix: usize,
    char_ix: usize,
}

impl StringIndexConverter {
    fn new(input: impl Into<String>) -> Self {
        let text = input.into();
        let chars = text.chars().collect();
        Self {
            text,
            chars,
            utf8_ix: 0,
            char_ix: 0,
        }
    }

    fn update_ix(&mut self, utf8_step: usize, char_step: usize) {
        self.utf8_ix += utf8_step;
        while !self.text.is_char_boundary(self.utf8_ix) {
            self.utf8_ix += 1;
        }
        self.char_ix += char_step;
    }

    fn can_loop(&self) -> bool {
        self.utf8_ix < self.text.len()
    }

    fn has_char_from_current_ix(&self, ch: char) -> Option<(usize, usize)> {
        let char_dist = self.chars[self.char_ix..].iter().position(|c| *c == ch)?;
        let utf8_dist = self.chars[self.char_ix..self.char_ix + char_dist]
            .iter()
            .map(|c| c.len_utf8())
            .sum::<usize>();
        Some((utf8_dist, char_dist))
    }

    fn current_char(&self) -> char {
        self.chars[self.char_ix]
    }
}
