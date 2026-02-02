//! Quality assessment of predictions using LLM-as-a-judge.
//!
//! This module uses LLM Batch APIs to evaluate prediction quality.
//! Caching is handled by the underlying client.

use crate::{
    BatchProvider,
    anthropic_client::AnthropicClient,
    example::Example,
    format_prompt::extract_cursor_excerpt_from_example,
    openai_client::OpenAiClient,
    parse_output::run_parse_output,
    paths::LLM_CACHE_DB,
    progress::{ExampleProgress, Step},
    word_diff::unified_to_word_diff,
};
use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Arguments for the QA command.
#[derive(Debug, Clone, clap::Args)]
pub struct QaArgs {
    /// Use synchronous API instead of batch
    #[clap(long)]
    pub no_batch: bool,

    /// Which LLM provider to use (anthropic or openai)
    #[clap(long, default_value = "openai")]
    pub backend: BatchProvider,
}

fn model_for_backend(backend: BatchProvider) -> &'static str {
    match backend {
        BatchProvider::Anthropic => "claude-sonnet-4-5",
        BatchProvider::Openai => "gpt-5.2",
    }
}

/// Result of QA evaluation for a single prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaResult {
    /// Free-form reasoning from the judge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,

    /// Does the prediction undo/revert changes the user intentionally made?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverts_edits: Option<bool>,

    /// Confidence score (1-5) for user acceptance likelihood.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<u8>,

    /// The raw response from the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,

    /// Error message if parsing or request failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Build the assessment prompt for an example.
pub fn build_prompt(example: &Example) -> Result<String> {
    let prediction = example
        .predictions
        .first()
        .context("no predictions available")?;
    let actual_patch = prediction
        .actual_patch
        .as_ref()
        .context("no actual_patch available (run predict first)")?;
    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs missing (run context retrieval first)")?;

    let actual_patch_word_diff = unified_to_word_diff(actual_patch);

    let cursor_excerpt =
        extract_cursor_excerpt_from_example(example).context("failed to extract cursor excerpt")?;

    let mut edit_history = String::new();
    for event in &prompt_inputs.edit_history {
        match event.as_ref() {
            zeta_prompt::Event::BufferChange {
                path,
                old_path,
                diff,
                predicted: _,
                in_open_source_repo: _,
            } => {
                edit_history.push_str(&format!("--- a{}\n", old_path.display()));
                edit_history.push_str(&format!("+++ b{}\n", path.display()));
                let diff_word_diff = unified_to_word_diff(diff);
                edit_history.push_str(&diff_word_diff);
                edit_history.push_str("\n\n");
            }
        }
    }

    let prompt_template = crate::prompt_assets::get_prompt("qa.md");
    Ok(prompt_template
        .replace("{edit_history}", &edit_history)
        .replace("{cursor_excerpt}", &cursor_excerpt)
        .replace("{actual_patch_word_diff}", &actual_patch_word_diff))
}

fn extract_codeblock(response: &str) -> Option<String> {
    let lines: Vec<&str> = response.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("```") {
            let start = i + 1;
            for (j, end_line) in lines[start..].iter().enumerate() {
                if end_line.starts_with("```") {
                    return Some(lines[start..start + j].join("\n"));
                }
            }
            return Some(lines[start..].join("\n"));
        }
    }
    None
}

fn parse_response(response_text: &str) -> QaResult {
    let codeblock = extract_codeblock(response_text);

    for text_to_parse in [codeblock.as_deref(), Some(response_text.trim())] {
        let Some(text) = text_to_parse else {
            continue;
        };

        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
            return QaResult {
                reasoning: parsed
                    .get("reasoning")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                reverts_edits: parsed.get("reverts_edits").and_then(|v| v.as_bool()),
                confidence: parsed
                    .get("confidence")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u8),
                response: Some(response_text.to_string()),
                error: None,
            };
        }
    }

    QaResult {
        reasoning: Some(response_text.to_string()),
        reverts_edits: None,
        confidence: None,
        response: Some(response_text.to_string()),
        error: Some("Could not parse JSON from response".to_string()),
    }
}

static ANTHROPIC_CLIENT_BATCH: OnceLock<AnthropicClient> = OnceLock::new();
static ANTHROPIC_CLIENT_PLAIN: OnceLock<AnthropicClient> = OnceLock::new();
static OPENAI_CLIENT_BATCH: OnceLock<OpenAiClient> = OnceLock::new();
static OPENAI_CLIENT_PLAIN: OnceLock<OpenAiClient> = OnceLock::new();

/// Run QA evaluation for a single example.
pub async fn run_qa(
    example: &mut Example,
    args: &QaArgs,
    example_progress: &ExampleProgress,
) -> Result<()> {
    if example
        .qa
        .first()
        .and_then(|q| q.as_ref())
        .and_then(|q| q.confidence)
        .is_some()
    {
        return Ok(());
    }

    run_parse_output(example).context("Failed to execute run_parse_output")?;

    if example.prompt_inputs.is_none() {
        anyhow::bail!("prompt_inputs missing (run context retrieval first)");
    }

    let step_progress = example_progress.start(Step::Qa);

    let model = model_for_backend(args.backend);
    let prompt = build_prompt(example).context("Failed to build QA prompt")?;

    step_progress.set_substatus("generating");

    let response = match args.backend {
        BatchProvider::Anthropic => {
            let client = if args.no_batch {
                ANTHROPIC_CLIENT_PLAIN.get_or_init(|| {
                    AnthropicClient::plain().expect("Failed to create Anthropic client")
                })
            } else {
                ANTHROPIC_CLIENT_BATCH.get_or_init(|| {
                    AnthropicClient::batch(&LLM_CACHE_DB)
                        .expect("Failed to create Anthropic client")
                })
            };

            let messages = vec![anthropic::Message {
                role: anthropic::Role::User,
                content: vec![anthropic::RequestContent::Text {
                    text: prompt,
                    cache_control: None,
                }],
            }];

            let Some(response) = client.generate(model, 1024, messages, None, false).await? else {
                return Ok(());
            };

            response
                .content
                .iter()
                .filter_map(|c| match c {
                    anthropic::ResponseContent::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("")
        }
        BatchProvider::Openai => {
            let client = if args.no_batch {
                OPENAI_CLIENT_PLAIN
                    .get_or_init(|| OpenAiClient::plain().expect("Failed to create OpenAI client"))
            } else {
                OPENAI_CLIENT_BATCH.get_or_init(|| {
                    OpenAiClient::batch(&LLM_CACHE_DB).expect("Failed to create OpenAI client")
                })
            };

            let messages = vec![open_ai::RequestMessage::User {
                content: open_ai::MessageContent::Plain(prompt),
            }];

            let Some(response) = client.generate(model, 1024, messages, None, false).await? else {
                return Ok(());
            };

            response
                .choices
                .into_iter()
                .filter_map(|choice| match choice.message {
                    open_ai::RequestMessage::Assistant { content, .. } => {
                        content.map(|c| match c {
                            open_ai::MessageContent::Plain(text) => text,
                            open_ai::MessageContent::Multipart(parts) => parts
                                .into_iter()
                                .filter_map(|p| match p {
                                    open_ai::MessagePart::Text { text } => Some(text),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join(""),
                        })
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("")
        }
    };

    let result = parse_response(&response);

    example.qa = example
        .predictions
        .iter()
        .enumerate()
        .map(|(i, _)| if i == 0 { Some(result.clone()) } else { None })
        .collect();

    Ok(())
}

/// Sync batches for QA (upload pending requests, download finished results).
pub async fn sync_batches(args: &QaArgs) -> Result<()> {
    if args.no_batch {
        return Ok(());
    }

    match args.backend {
        BatchProvider::Anthropic => {
            let client = ANTHROPIC_CLIENT_BATCH.get_or_init(|| {
                AnthropicClient::batch(&LLM_CACHE_DB).expect("Failed to create Anthropic client")
            });
            client.sync_batches().await?;
        }
        BatchProvider::Openai => {
            let client = OPENAI_CLIENT_BATCH.get_or_init(|| {
                OpenAiClient::batch(&LLM_CACHE_DB).expect("Failed to create OpenAI client")
            });
            client.sync_batches().await?;
        }
    }
    Ok(())
}
