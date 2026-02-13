//! Repair predictions that received poor quality signals.
//!
//! This module takes examples with predictions, identifies predictions that need
//! improvement, and uses an LLM to generate improved predictions. It supports
//! two sources of quality signals:
//! - QA feedback (reverts_edits or low confidence)
//! - Computed scores when QA is unavailable (high reversal_ratio or wrong_editable_region)

use crate::{
    BatchProvider, PredictionProvider,
    anthropic_client::AnthropicClient,
    example::{ActualCursor, Example, ExamplePrediction},
    format_prompt::{TeacherPrompt, extract_cursor_excerpt_from_example, extract_last_codeblock},
    openai_client::OpenAiClient,
    parse_output::run_parse_output,
    paths::LLM_CACHE_DB,
    progress::{ExampleProgress, Step},
    word_diff::unified_to_word_diff,
};
use anyhow::{Context as _, Result};
use std::sync::OnceLock;

const KEEP_PREVIOUS: &str = "KEEP_PREVIOUS";

/// Print a summary report of repair results across all examples.
pub fn print_report(examples: &[Example], confidence_threshold: u8) {
    let total = examples.len();
    let mut no_repair_needed = 0;
    let mut repaired = 0;
    let mut repair_failed = 0;

    for example in examples {
        if !needs_repair(example, confidence_threshold) {
            no_repair_needed += 1;
            continue;
        }

        if has_successful_repair(example) {
            repaired += 1;
        } else {
            repair_failed += 1;
        }
    }

    let needed_repair = total - no_repair_needed;

    eprintln!();
    eprintln!("Repair summary ({total} examples):");
    eprintln!(
        "  {no_repair_needed}/{total} didn't need repair (confidence > {confidence_threshold})"
    );
    if needed_repair > 0 {
        eprintln!("  {needed_repair}/{total} needed repair:");
        if repaired > 0 {
            eprintln!("    {repaired} repaired successfully");
        }
        if repair_failed > 0 {
            eprintln!("    {repair_failed} failed to repair");
        }
    }
}

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

/// Build the quality feedback string from QA results.
fn build_qa_feedback(example: &Example) -> Option<String> {
    let qa = example.qa.first()?.as_ref()?;

    let qa_reasoning = qa.reasoning.as_deref().unwrap_or("No reasoning provided");
    let reverts_edits = qa
        .reverts_edits
        .map_or("unknown", |v| if v { "yes" } else { "no" });
    let confidence = qa
        .confidence
        .map_or("unknown".to_string(), |v| v.to_string());

    Some(format!(
        "- **Reverts user edits**: {reverts_edits}\n\
         - **Confidence score**: {confidence}/5\n\
         - **Reasoning**: {qa_reasoning}"
    ))
}

/// Build the quality feedback string from computed scores when QA is unavailable.
fn build_score_feedback(example: &Example) -> Option<String> {
    let score = example.score.first()?;

    let mut issues = Vec::new();

    if score.reversal_ratio > 0.9 {
        issues.push(format!(
            "Automated analysis detected a high reversal ratio ({:.2}), which suggests this \
             prediction may be reverting changes the user intentionally made. Double-check that \
             the prediction doesn't undo the user's recent edits. If the prediction is actually \
             fine and the edits are intentional completions rather than reversals, keep it as-is. \
             If it truly reverts the user's changes, generate an improved prediction that \
             continues the user's intent instead.",
            score.reversal_ratio
        ));
    }

    if score.wrong_editable_region == Some(true) {
        issues.push(
            "Automated analysis detected that the prediction may be modifying code outside \
             the expected editable region, or producing changes misaligned with the editable \
             region boundaries. Make sure the prediction only modifies code within the editable \
             region and is properly aligned."
                .to_string(),
        );
    }

    if issues.is_empty() {
        return None;
    }

    let mut feedback = String::from(
        "No human quality assessment is available, but automated scoring flagged potential issues:\n\n",
    );
    for issue in &issues {
        feedback.push_str(&format!("- {issue}\n"));
    }
    feedback.push_str(
        "\nRemember: if the previous prediction was actually correct, output `KEEP_PREVIOUS`. \
         If no edits should be made at all and you are unsure how to improve it, output `NO_EDITS`.",
    );

    Some(feedback)
}

/// Build the repair prompt for an example that needs improvement.
pub fn build_repair_prompt(example: &Example) -> Result<String> {
    let prediction = example
        .predictions
        .first()
        .context("no predictions available")?;
    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs missing (run context retrieval first)")?;
    let actual_patch = prediction
        .actual_patch
        .as_ref()
        .context("no actual_patch available (run predict first)")?;

    let quality_feedback = build_qa_feedback(example)
        .or_else(|| build_score_feedback(example))
        .context("no quality feedback available (need either QA results or computed scores)")?;

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

    let prompt_template = crate::prompt_assets::get_prompt("repair.md");
    Ok(prompt_template
        .replace("{edit_history}", &edit_history)
        .replace("{context}", &context)
        .replace("{cursor_excerpt}", &cursor_excerpt)
        .replace("{actual_patch_word_diff}", &actual_patch_word_diff)
        .replace("{quality_feedback}", &quality_feedback))
}

/// Check if an example needs repair based on QA feedback or computed scores.
pub fn needs_repair(example: &Example, confidence_threshold: u8) -> bool {
    // Check QA-based signals first.
    if let Some(qa) = example.qa.first().and_then(|q| q.as_ref()) {
        if qa.reverts_edits == Some(true) {
            return true;
        }

        if let Some(confidence) = qa.confidence {
            if confidence <= confidence_threshold {
                return true;
            }
        }

        return false;
    }

    // When QA is unavailable, fall back to computed score signals.
    if let Some(score) = example.score.first() {
        if score.reversal_ratio > 0.9 {
            return true;
        }

        if score.wrong_editable_region == Some(true) {
            return true;
        }
    }

    false
}

/// Parse repair model output into a patch and optional cursor.
///
/// Handles the `KEEP_PREVIOUS` sentinel by copying the teacher's prediction,
/// and delegates normal output to `TeacherPrompt::parse`.
pub fn parse(example: &Example, actual_output: &str) -> Result<(String, Option<ActualCursor>)> {
    let last_codeblock = extract_last_codeblock(actual_output);
    if last_codeblock.trim() == KEEP_PREVIOUS {
        let original = example
            .predictions
            .first()
            .context("no original prediction to keep")?;
        let patch = original.actual_patch.clone().unwrap_or_default();
        let cursor = original.actual_cursor.clone();
        return Ok((patch, cursor));
    }

    TeacherPrompt::parse(example, actual_output)
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

    let parse_result = parse(example, &response);
    let err = parse_result
        .as_ref()
        .err()
        .map(|e| format!("Failed to parse repair response: {}", e));

    let (actual_patch, actual_cursor) = parse_result.ok().unzip();
    let actual_cursor = actual_cursor.flatten();

    example.predictions.push(ExamplePrediction {
        actual_patch,
        actual_output: response,
        actual_cursor,
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
