use crate::Supermaven;
use anyhow::Result;
use editor::{Direction, InlineCompletionProvider};
use gpui::{AppContext, Global, Model, ModelContext, Task};
use language::{language_settings::all_language_settings, Anchor, Buffer, ToOffset};
use std::path::PathBuf;

pub struct SupermavenCompletionProvider {
    completions: Vec<String>,
    active_completion_index: usize,
    pending_refresh: Task<Result<()>>,
}

impl SupermavenCompletionProvider {
    pub fn new() -> Self {
        Self {
            completions: Vec::new(),
            active_completion_index: 0,
            pending_refresh: Task::ready(Ok(())),
        }
    }

    fn update_completions(
        &mut self,
        buffer: &Model<Buffer>,
        cursor_position: Anchor,
        cx: &mut ModelContext<Self>,
    ) {
        self.completions = Supermaven::get(cx).completions(buffer, cursor_position, cx);
        self.active_completion_index = 0;
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
        let buffer = buffer_handle.read(cx);
        let path = buffer
            .file()
            .and_then(|file| Some(file.as_local()?.abs_path(cx)))
            .unwrap_or_else(|| PathBuf::from("untitled"));
        let content = buffer.text();
        let cursor_offset = cursor_position.to_offset(buffer);
        let refresh = Supermaven::update(cx, |supermaven, _| {
            supermaven.complete(&path, content, cursor_offset)
        });

        self.update_completions(&buffer_handle, cursor_position, cx);
        self.pending_refresh = cx.spawn(|this, mut cx| async move {
            refresh.await;
            this.update(&mut cx, |this, cx| {
                this.update_completions(&buffer_handle, cursor_position, cx);
                cx.notify();
            })
        });
    }

    fn cycle(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: Anchor,
        direction: Direction,
        cx: &mut ModelContext<Self>,
    ) {
        match direction {
            Direction::Prev => {
                self.active_completion_index = if self.active_completion_index == 0 {
                    self.completions.len().saturating_sub(1)
                } else {
                    self.active_completion_index - 1
                };
            }
            Direction::Next => {
                if self.completions.len() == 0 {
                    self.active_completion_index = 0
                } else {
                    self.active_completion_index =
                        (self.active_completion_index + 1) % self.completions.len();
                }
            }
        }
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
    ) -> Option<&str> {
        Some(self.completions.get(self.active_completion_index)?.as_str())
    }
}
