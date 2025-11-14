use anyhow::Result;
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use gpui::{App, Context, Entity, EntityId, Task};
use http_client::HttpClient;
use language::{Anchor, Buffer, ToOffset};
use std::{path::Path, sync::Arc, time::Duration};

use crate::{ResponseChoice, CompletionRequest, stream_complete};

pub const LMSTUDIO_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub struct LMStudioCompletionProvider {
    http_client: Arc<dyn HttpClient>,
    api_url: String,
    model_name: String,
    buffer_id: Option<EntityId>,
    completion_text: Option<String>,
    file_extension: Option<String>,
    pending_refresh: Option<Task<Result<()>>>,
    completion_position: Option<Anchor>,
}

impl LMStudioCompletionProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        api_url: String,
        model_name: String,
    ) -> Self {
        Self {
            http_client,
            api_url,
            model_name,
            buffer_id: None,
            completion_text: None,
            file_extension: None,
            pending_refresh: None,
            completion_position: None,
        }
    }

    fn reset_completion(&mut self) {
        self.pending_refresh = None;
        self.completion_text = None;
        self.completion_position = None;
        self.buffer_id = None;
    }
}

impl EditPredictionProvider for LMStudioCompletionProvider {
    fn name() -> &'static str {
        "lmstudio"
    }

    fn display_name() -> &'static str {
        "LM Studio"
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

    fn is_enabled(
        &self,
        _buffer: &Entity<Buffer>,
        _cursor_position: Anchor,
        _cx: &App,
    ) -> bool {
        true
    }

    fn is_refreshing(&self) -> bool {
        self.pending_refresh.is_some()
    }

    fn refresh(
        &mut self,
        buffer_handle: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        if !debounce {
            return;
        }

        self.reset_completion();

        let http_client = self.http_client.clone();
        let api_url = self.api_url.clone();
        let model_name = self.model_name.clone();

        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            if debounce {
                cx.background_executor()
                    .timer(LMSTUDIO_DEBOUNCE_TIMEOUT)
                    .await;
            }

            let (buffer_text, cursor_offset) = buffer_handle.read_with(cx, |buffer, _cx| {
                let snapshot = buffer.snapshot();
                let cursor_offset = cursor_position.to_offset(&snapshot);
                let text = snapshot.text();
                (text, cursor_offset)
            })?;

            let prefix = buffer_text[..cursor_offset].to_string();
            let suffix = buffer_text[cursor_offset..].to_string();

            // TODO: Make this configurable
            // For more templates: https://github.com/continuedev/continue/blob/main/core/autocomplete/templating/AutocompleteTemplate.ts
            let prompt = format!(
                "<|fim_prefix|>{}<|fim_suffix|>{}<|fim_middle|>",
                prefix, suffix
            );

            let stop_vec = vec![
                "<|endoftext|>".to_string(),
                "<|fim_prefix|>".to_string(),
                "<|fim_middle|>".to_string(),
                "<|fim_suffix|>".to_string(),
                "<|fim_pad|>".to_string(),
                "<|repo_name|>".to_string(),
                "<|file_sep|>".to_string(),
                "<|im_start|>".to_string(),
                "<|im_end|>".to_string(),
            ];

            let request = CompletionRequest {
                model: model_name.clone(),
                prompt,
                stream: true,
                max_tokens: Some(350),
                stop: Some(stop_vec),
                temperature: Some(0.2),
            };

            let mut stream = stream_complete(&*http_client, &api_url, crate::Request::Completion(request)).await?;

            let mut completion_text = String::new();

            while let Some(event) = futures::StreamExt::next(&mut stream).await {
                for choice in event?.choices {
                    let ResponseChoice::Text(text_choice) = choice else {
                        continue;
                    };

                    completion_text.push_str(&text_choice.text);

                    this.update(cx, |this, cx| {
                        this.completion_text = Some(completion_text.clone());
                        this.completion_position = Some(cursor_position);
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

    fn accept(&mut self, cx: &mut Context<Self>) {
        self.reset_completion();
        cx.notify();
    }

    fn discard(&mut self, cx: &mut Context<Self>) {
        self.reset_completion();
        cx.notify();
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

        let completion_text = self.completion_text.as_ref()?;

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
            let cursor_offset = cursor_position.to_offset(&snapshot);
            let anchor = snapshot.anchor_after(cursor_offset);

            Some(EditPrediction::Local {
                id: None,
                edits: vec![(anchor..anchor, completion_text.into())],
                edit_preview: None,
            })
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
