use crate::{ResponseItem, Supermaven};
use anyhow::Result;
use editor::{Direction, InlineCompletionProvider};
use gpui::{AppContext, Global, Model, ModelContext, Task};
use language::{
    language_settings::all_language_settings, Anchor, Buffer, OffsetRangeExt, ToOffset,
};

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
        // todo!(implement cycling)
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

    fn active_completion_text<'a>(
        &'a self,
        buffer: &Model<Buffer>,
        cursor_position: Anchor,
        cx: &'a AppContext,
    ) -> Option<&'a str> {
        let buffer_id = buffer.entity_id();
        let buffer = buffer.read(cx);
        let cursor_offset = cursor_position.to_offset(buffer);
        let mut candidate: Option<&str> = None;
        for completion in Supermaven::get(cx).completions(buffer_id) {
            let mut completion_range = completion.range.to_offset(buffer);

            let prefix_len = common_prefix(
                buffer.chars_for_range(completion_range.clone()),
                completion.text.chars(),
            );
            completion_range.start += prefix_len;
            let suffix_len = common_prefix(
                buffer.reversed_chars_for_range(completion_range.clone()),
                completion.text[prefix_len..].chars().rev(),
            );
            completion_range.end = completion_range.end.saturating_sub(suffix_len);

            let completion_text = &completion.text[prefix_len..completion.text.len() - suffix_len];
            if completion_range.is_empty()
                && completion_range.start == cursor_offset
                && !completion_text.trim().is_empty()
                && candidate
                    .as_ref()
                    .map_or(true, |candidate| completion_text.len() >= candidate.len())
            {
                candidate = Some(completion_text);
            }
        }

        candidate
    }
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}
