use crate::{Codestral, CodestralRequest, CodestralResponse};
use anyhow::Result;
use http_client::HttpClient;
use futures::AsyncReadExt;
use gpui::{App, Context, Entity, Task};
use inline_completion::{Direction, EditPredictionProvider, InlineCompletion};
use language::{Anchor, Buffer, language_settings::all_language_settings};
use project::Project;
use std::{
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use text::{ToOffset, ToPoint};

pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);
const CODESTRAL_API_URL: &str = "https://codestral.mistral.ai/v1/fim/completions";

pub struct CodestralCompletionProvider {
    codestral: Entity<Codestral>,
    buffer_id: Option<gpui::EntityId>,
    pending_request: Option<Task<Result<()>>>,
    last_completion: Option<String>,
    http_client: Arc<dyn HttpClient>,
}

impl CodestralCompletionProvider {
    pub fn new(codestral: Entity<Codestral>, http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            codestral,
            buffer_id: None,
            pending_request: None,
            last_completion: None,
            http_client,
        }
    }

    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        api_key: String,
        prompt: String,
        suffix: String,
        file_extension: Option<String>,
        model: String,
        max_tokens: Option<u32>,
    ) -> Result<String> {
        let start_time = Instant::now();
        
        // Log request details
        log::debug!(
            "Codestral: Requesting completion (model: {}, max_tokens: {:?}, file_type: {:?})",
            model, max_tokens, file_extension
        );
        
        // Log truncated prompt and suffix for debugging
        let prompt_preview = if prompt.len() > 200 {
            format!("{}...", &prompt[..200])
        } else {
            prompt.clone()
        };
        let suffix_preview = if suffix.len() > 200 {
            format!("{}...", &suffix[..200])
        } else {
            suffix.clone()
        };
        
        log::debug!("Codestral: Prompt preview: {:?}", prompt_preview);
        log::debug!("Codestral: Suffix preview: {:?}", suffix_preview);
        let request = CodestralRequest {
            model,
            prompt,
            suffix: if suffix.is_empty() { None } else { Some(suffix) },
            max_tokens: max_tokens.or(Some(150)),
            temperature: Some(0.2),
            top_p: Some(1.0),
            stream: Some(false),
            stop: Some(vec!["\n\n".to_string()]),
            random_seed: None,
            min_tokens: None,
        };

        let request_body = serde_json::to_string(&request)?;
        
        log::debug!(
            "Codestral: Sending request to {} with body: {}",
            CODESTRAL_API_URL,
            request_body
        );
        
        let http_request = http_client::Request::builder()
            .method(http_client::Method::POST)
            .uri(CODESTRAL_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .body(http_client::AsyncBody::from(request_body))?;
        
        let mut response = http_client.send(http_request).await?;
        let status = response.status();
        
        log::debug!("Codestral: Response status: {}", status);

        if !status.is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            log::error!("Codestral API error: {} - {}", status, body);
            return Err(anyhow::anyhow!("Codestral API error: {} - {}", status, body));
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        
        log::debug!("Codestral: Raw response: {}", body);
        
        let codestral_response: CodestralResponse = serde_json::from_str(&body)?;
        
        let elapsed = start_time.elapsed();
        
        if let Some(choice) = codestral_response.choices.first() {
            let completion = &choice.message.content;
            let completion_preview = if completion.len() > 200 {
                format!("{}...", &completion[..200])
            } else {
                completion.clone()
            };
            
            log::info!(
                "Codestral: Completion received ({} tokens, {:.2}s): {:?}",
                codestral_response.usage.completion_tokens,
                elapsed.as_secs_f64(),
                completion_preview
            );
            
            Ok(completion.clone())
        } else {
            log::error!("Codestral: No completion returned in response");
            Err(anyhow::anyhow!("No completion returned from Codestral"))
        }
    }
}

impl EditPredictionProvider for CodestralCompletionProvider {
    fn name() -> &'static str {
        "codestral"
    }

    fn display_name() -> &'static str {
        "Codestral"
    }

    fn show_completions_in_menu() -> bool {
        false
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        let enabled = self.codestral.read(cx).is_enabled();
        log::debug!("Codestral: Provider enabled: {}", enabled);
        enabled
    }

    fn is_refreshing(&self) -> bool {
        self.pending_request.is_some()
    }

    fn refresh(
        &mut self,
        _project: Option<Entity<Project>>,
        buffer_handle: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        log::debug!("Codestral: Refresh called (debounce: {})", debounce);
        
        let Some(api_key) = self.codestral.read(cx).api_key().map(|k| k.to_string()) else {
            log::warn!("Codestral: No API key configured, skipping refresh");
            return;
        };

        let buffer = buffer_handle.read(cx);
        let snapshot = buffer.snapshot();
        let cursor_offset = cursor_position.to_offset(&snapshot);
        
        // Get text before and after cursor
        let prompt = snapshot.text_for_range(0..cursor_offset).collect::<String>();
        let suffix = snapshot.text_for_range(cursor_offset..snapshot.len()).collect::<String>();
        
        // Get file extension for context
        let file_extension = buffer.file().and_then(|file| {
            Path::new(file.file_name(cx))
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_string())
        });

        self.buffer_id = Some(buffer_handle.entity_id());
        let http_client = self.http_client.clone();
        
        // Get settings for model and max_tokens
        let settings = all_language_settings(None, cx);
        let model = settings.edit_predictions.codestral.model.clone()
            .unwrap_or_else(|| "codestral-latest".to_string());
        let max_tokens = settings.edit_predictions.codestral.max_tokens;

        self.pending_request = Some(cx.spawn(async move |this, cx| {
            if debounce {
                log::debug!("Codestral: Debouncing for {:?}", DEBOUNCE_TIMEOUT);
                smol::Timer::after(DEBOUNCE_TIMEOUT).await;
            }

            let completion = match Self::fetch_completion(
                http_client,
                api_key,
                prompt,
                suffix,
                file_extension,
                model,
                max_tokens,
            ).await {
                Ok(completion) => completion,
                Err(e) => {
                    log::error!("Codestral: Failed to fetch completion: {}", e);
                    return Err(e);
                }
            };

            this.update(cx, |this, cx| {
                this.last_completion = Some(completion);
                this.pending_request = None;
                cx.notify();
            })?;

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
        // Codestral doesn't support multiple completions, so cycling does nothing
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Codestral: Completion accepted");
        self.pending_request = None;
        self.last_completion = None;
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Codestral: Completion discarded");
        self.pending_request = None;
        self.last_completion = None;
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<InlineCompletion> {
        let completion_text = self.last_completion.as_ref()?;
        
        if completion_text.trim().is_empty() {
            log::debug!("Codestral: Empty completion text, not suggesting");
            return None;
        }
        
        log::debug!("Codestral: Suggesting completion of {} chars", completion_text.len());

        let snapshot = buffer.read(cx).snapshot();
        let _cursor_point = cursor_position.to_point(&snapshot);
        let _cursor_offset = cursor_position.to_offset(&snapshot);
        
        // For now, we'll treat the completion as a simple insertion at the cursor
        // In the future, we could implement more sophisticated diff logic like Supermaven
        let mut edits = Vec::new();
        
        // Create insertion at cursor position
        let edit_range = cursor_position..cursor_position;
        edits.push((edit_range, completion_text.clone()));

        Some(InlineCompletion {
            id: None,
            edits,
            edit_preview: None,
        })
    }
}