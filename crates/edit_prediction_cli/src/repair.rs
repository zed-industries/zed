//! Repair predictions that received poor QA scores.
//!
//! This module takes examples with predictions and QA feedback, identifies
//! predictions that need improvement (based on reverts_edits or low confidence),
//! and uses an LLM to generate improved predictions.

use crate::{
    BatchProvider, PredictionProvider,
    anthropic_client::AnthropicClient,
    example::{Example, ExamplePrediction},
    format_prompt::{TeacherPrompt, extract_cursor_excerpt_from_example},
    openai_client::OpenAiClient,
    parse_output::run_parse_output,
    paths::LLM_CACHE_DB,
    progress::{ExampleProgress, Step},
    word_diff::unified_to_word_diff,
};
use anyhow::{Context as _, Result};
use std::sync::OnceLock;

/// Arguments for the repair command.
#[derive(Debug, Clone, clap::Args)]
pub struct RepairArgs {
    /// Use synchronous API instead of batch
    #[clap(long)]
    pub no_batch: bool,

    /// Confidence threshold: repair predictions with confidence <= this value (1-5)
    #[clap(long, default_value = "2")]
    pub confidence_threshold: u8,

    /// Which LLM provider to use (anthropic or openai)
    #[clap(long, default_value = "anthropic")]
    pub backend: BatchProvider,
}

fn model_for_backend(backend: BatchProvider) -> &'static str {
    match backend {
        BatchProvider::Anthropic => "claude-sonnet-4-5",
        BatchProvider::Openai => "gpt-5.2",
    }
}

/// Build the repair prompt for an example that needs improvement.
pub fn build_repair_prompt(example: &Example) -> Result<String> {
    let prediction = example
        .predictions
        .first()
        .context("no predictions available")?;
    let qa = example
        .qa
        .first()
        .context("no QA results available")?
        .as_ref()
        .context("QA result is None")?;
    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs missing (run context retrieval first)")?;
    let actual_patch = prediction
        .actual_patch
        .as_ref()
        .context("no actual_patch available (run predict first)")?;

    let actual_patch_word_diff = unified_to_word_diff(actual_patch);

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

    let context = TeacherPrompt::format_context(example);

    let cursor_excerpt =
        extract_cursor_excerpt_from_example(example).context("failed to extract cursor excerpt")?;

    let qa_reasoning = qa.reasoning.as_deref().unwrap_or("No reasoning provided");
    let reverts_edits = qa
        .reverts_edits
        .map_or("unknown", |v| if v { "yes" } else { "no" });
    let confidence = qa
        .confidence
        .map_or("unknown".to_string(), |v| v.to_string());

    let prompt_template = crate::prompt_assets::get_prompt("repair.md");
    Ok(prompt_template
        .replace("{edit_history}", &edit_history)
        .replace("{context}", &context)
        .replace("{cursor_excerpt}", &cursor_excerpt)
        .replace("{actual_patch_word_diff}", &actual_patch_word_diff)
        .replace("{reverts_edits}", reverts_edits)
        .replace("{confidence}", &confidence)
        .replace("{qa_reasoning}", qa_reasoning))
}

/// Check if an example needs repair based on QA feedback.
pub fn needs_repair(example: &Example, confidence_threshold: u8) -> bool {
    let Some(qa) = example.qa.first().and_then(|q| q.as_ref()) else {
        return false;
    };

    if qa.reverts_edits == Some(true) {
        return true;
    }

    if let Some(confidence) = qa.confidence {
        if confidence <= confidence_threshold {
            return true;
        }
    }

    false
}

/// Check if an example already has a successful repair prediction.
fn has_successful_repair(example: &Example) -> bool {
    example
        .predictions
        .iter()
        .any(|p| p.provider == PredictionProvider::Repair && p.actual_patch.is_some())
}

static ANTHROPIC_CLIENT_BATCH: OnceLock<AnthropicClient> = OnceLock::new();
static ANTHROPIC_CLIENT_PLAIN: OnceLock<AnthropicClient> = OnceLock::new();
static OPENAI_CLIENT_BATCH: OnceLock<OpenAiClient> = OnceLock::new();
static OPENAI_CLIENT_PLAIN: OnceLock<OpenAiClient> = OnceLock::new();

/// Run repair for a single example.
pub async fn run_repair(
    example: &mut Example,
    args: &RepairArgs,
    example_progress: &ExampleProgress,
) -> Result<()> {
    if has_successful_repair(example) {
        return Ok(());
    }

    if !needs_repair(example, args.confidence_threshold) {
        return Ok(());
    }

    run_parse_output(example).context("Failed to execute run_parse_output")?;

    if example.prompt_inputs.is_none() {
        anyhow::bail!("prompt_inputs missing (run context retrieval first)");
    }

    if example.predictions.is_empty() {
        anyhow::bail!("no predictions available (run predict first)");
    }

    if example.qa.is_empty() {
        anyhow::bail!("no QA results available (run qa first)");
    }

    let step_progress = example_progress.start(Step::Repair);

    let model = model_for_backend(args.backend);
    let prompt = build_repair_prompt(example).context("Failed to build repair prompt")?;

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

            let Some(response) = client.generate(model, 16384, messages, None, false).await? else {
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

            let Some(response) = client.generate(model, 16384, messages, None, false).await? else {
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

    let parse_result = TeacherPrompt::parse(example, &response);
    let err = parse_result
        .as_ref()
        .err()
        .map(|e| format!("Failed to parse repair response: {}", e));

    let (actual_patch, actual_cursor_offset) = parse_result.ok().unzip();

    example.predictions.push(ExamplePrediction {
        actual_patch,
        actual_output: response,
        actual_cursor_offset: actual_cursor_offset.flatten(),
        error: err,
        provider: PredictionProvider::Repair,
    });

    Ok(())
}

/// Sync batches for repair (upload pending requests, download finished results).
pub async fn sync_batches(args: &RepairArgs) -> Result<()> {
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
