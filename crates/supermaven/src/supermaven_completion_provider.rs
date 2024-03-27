use crate::{common_prefix, ResponseItem, Supermaven};
use anyhow::Result;
use editor::{Direction, InlineCompletionProvider};
use gpui::{AppContext, Global, Model, ModelContext, Task};
use language::{language_settings::all_language_settings, Anchor, Buffer, ToOffset};
use std::path::PathBuf;

pub struct SupermavenCompletionProvider {
    pending_refresh: Task<Result<()>>,
}

impl SupermavenCompletionProvider {
    pub fn new() -> Self {
        Self {
            pending_refresh: Task::ready(Ok(())),
        }
    }
}

impl InlineCompletionProvider for SupermavenCompletionProvider {
    fn is_enabled(&self, buffer: &Model<Buffer>, cursor_position: Anchor, cx: &AppContext) -> bool {
        if !Supermaven::get(cx).is_enabled() {
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
        dbg!("refresh");

        let refresh = Supermaven::update(cx, |supermaven, cx| {
            supermaven.complete(&buffer_handle, cursor_position, cx)
        });

        self.pending_refresh = cx.spawn(|this, mut cx| async move {
            refresh.await;
            this.update(&mut cx, |this, cx| cx.notify())
        });
    }

    fn cycle(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: Anchor,
        direction: Direction,
        cx: &mut ModelContext<Self>,
    ) {
        // implement cycling
        // match direction {
        //     Direction::Prev => {
        //         self.active_completion_index = if self.active_completion_index == 0 {
        //             self.completions.len().saturating_sub(1)
        //         } else {
        //             self.active_completion_index - 1
        //         };
        //     }
        //     Direction::Next => {
        //         if self.completions.len() == 0 {
        //             self.active_completion_index = 0
        //         } else {
        //             self.active_completion_index =
        //                 (self.active_completion_index + 1) % self.completions.len();
        //         }
        //     }
        // }
    }

    fn accept(&mut self, cx: &mut ModelContext<Self>) {
        // todo!("accept!")
    }

    fn discard(&mut self, cx: &mut ModelContext<Self>) {
        // todo!("discard")
    }

    fn active_completion_text(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: Anchor,
        cx: &AppContext,
    ) -> Option<String> {
        struct Candidate {
            prefix_len: usize,
            text: String,
        }

        let buffer = buffer.read(cx);
        // let cursor_offset = cursor_position.to_offset(buffer);
        let mut candidate: Option<Candidate> = None;
        for completion in Supermaven::get(cx).completions() {
            let mut completion_start = completion.start.to_offset(buffer);
            let completion_text = completion
                .completion
                .iter()
                .map(|completion| {
                    if let ResponseItem::Text { text } = completion {
                        text.as_str()
                    } else {
                        ""
                    }
                })
                .collect::<String>();
            let prefix_len =
                common_prefix(buffer.chars_at(completion_start), completion_text.chars());

            // completion_start += prefix_len;

            let completion_text = &completion_text[prefix_len..];
            if prefix_len != 0 && !completion_text.trim().is_empty() {
                if candidate.as_ref().map_or(true, |candidate| {
                    (prefix_len, completion_text.len())
                        >= (candidate.prefix_len, candidate.text.len())
                }) {
                    candidate = Some(Candidate {
                        prefix_len,
                        text: completion_text.to_string(),
                    });
                }
            }
        }

        candidate.map(|candidate| candidate.text)
    }
}
