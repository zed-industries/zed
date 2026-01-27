//! Repair predictions that received poor QA scores.
//!
//! This module takes examples with predictions and QA feedback, identifies
//! predictions that need improvement (based on reverts_edits or low confidence),
//! and uses an LLM to generate improved predictions.

use crate::PredictionProvider;
use crate::anthropic_client::AnthropicClient;
use crate::example::{Example, ExamplePrediction};
use crate::format_prompt::{TeacherPrompt, extract_cursor_excerpt_from_example};
use crate::paths::LLM_CACHE_DB;
use crate::word_diff::unified_to_word_diff;
use anthropic::{Message, RequestContent, Role};
use anyhow::Result;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

/// Model to use for repair.
const MODEL: &str = "claude-sonnet-4-5";

const PROMPT_TEMPLATE: &str = include_str!("prompts/repair.md");

/// Arguments for the repair command.
#[derive(Debug, Clone, clap::Args)]
pub struct RepairArgs {
    /// Use synchronous API instead of batch
    #[clap(long)]
    pub no_batch: bool,

    /// Wait for batch to complete (polls every 30s)
    #[clap(long)]
    pub wait: bool,

    /// Confidence threshold: repair predictions with confidence <= this value (1-5)
    #[clap(long, default_value = "2")]
    pub confidence_threshold: u8,
}

/// Build the repair prompt for an example that needs improvement.
///
/// Returns None if the example doesn't have the required data (predictions, qa, prompt_inputs).
pub fn build_repair_prompt(example: &Example) -> Option<String> {
    let prediction = example.predictions.first()?;
    let qa = example.qa.first()?.as_ref()?;
    let prompt_inputs = example.prompt_inputs.as_ref()?;
    let actual_patch = prediction.actual_patch.as_ref()?;

    let actual_patch_word_diff = unified_to_word_diff(actual_patch);

    // Format edit history similar to qa.rs
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

    // Format related files context (reuse from TeacherPrompt)
    let context = TeacherPrompt::format_context(example);

    // Format cursor excerpt with editable region markers (reuse from format_prompt)
    let cursor_excerpt = extract_cursor_excerpt_from_example(example)?;

    // Get QA feedback
    let qa_reasoning = qa.reasoning.as_deref().unwrap_or("No reasoning provided");
    let reverts_edits = qa
        .reverts_edits
        .map_or("unknown", |v| if v { "yes" } else { "no" });
    let confidence = qa
        .confidence
        .map_or("unknown".to_string(), |v| v.to_string());

    Some(
        PROMPT_TEMPLATE
            .replace("{edit_history}", &edit_history)
            .replace("{context}", &context)
            .replace("{cursor_excerpt}", &cursor_excerpt)
            .replace("{actual_patch_word_diff}", &actual_patch_word_diff)
            .replace("{reverts_edits}", reverts_edits)
            .replace("{confidence}", &confidence)
            .replace("{qa_reasoning}", qa_reasoning),
    )
}

/// Check if an example needs repair based on QA feedback.
pub fn needs_repair(example: &Example, confidence_threshold: u8) -> bool {
    let Some(qa) = example.qa.first().and_then(|q| q.as_ref()) else {
        return false;
    };

    // Repair if reverts_edits is true
    if qa.reverts_edits == Some(true) {
        return true;
    }

    // Repair if confidence is at or below threshold
    if let Some(confidence) = qa.confidence {
        if confidence <= confidence_threshold {
            return true;
        }
    }

    false
}

/// Parse the repair response into a prediction.
fn parse_repair_response(example: &Example, response_text: &str) -> Result<ExamplePrediction> {
    let actual_patch = TeacherPrompt::parse(example, response_text)?;

    Ok(ExamplePrediction {
        actual_patch: Some(actual_patch),
        actual_output: response_text.to_string(),
        error: None,
        provider: PredictionProvider::Repair,
    })
}

/// Run the repair process on a set of examples.
pub async fn run_repair(
    examples: &mut [Example],
    args: &RepairArgs,
    output_path: Option<&PathBuf>,
) -> Result<()> {
    let client = if args.no_batch {
        AnthropicClient::plain()?
    } else {
        AnthropicClient::batch(&LLM_CACHE_DB)?
    };

    eprintln!(
        "Using model: {}, batching: {}, confidence_threshold: {}",
        MODEL, !args.no_batch, args.confidence_threshold
    );

    // First pass: identify examples that need repair and build prompts
    let mut repair_items: Vec<(usize, String)> = Vec::new();
    let mut skipped_missing_data = 0;
    let mut skipped_no_repair_needed = 0;

    for (idx, example) in examples.iter().enumerate() {
        // Skip if missing predictions or qa
        if example.predictions.is_empty() || example.qa.is_empty() {
            skipped_missing_data += 1;
            continue;
        }

        // Skip if doesn't need repair
        if !needs_repair(example, args.confidence_threshold) {
            skipped_no_repair_needed += 1;
            continue;
        }

        // Build repair prompt
        let Some(prompt) = build_repair_prompt(example) else {
            skipped_missing_data += 1;
            continue;
        };

        repair_items.push((idx, prompt));
    }

    eprintln!(
        "Skipping {} items with missing data, {} items that don't need repair",
        skipped_missing_data, skipped_no_repair_needed
    );
    eprintln!("{} items to repair", repair_items.len());

    // Process all items
    let mut results: Vec<(usize, Option<String>)> = Vec::new();

    if args.no_batch {
        // Synchronous processing
        for (i, (idx, prompt)) in repair_items.iter().enumerate() {
            eprint!("\rProcessing {}/{}", i + 1, repair_items.len());

            let messages = vec![Message {
                role: Role::User,
                content: vec![RequestContent::Text {
                    text: prompt.clone(),
                    cache_control: None,
                }],
            }];

            let response = client.generate(MODEL, 16384, messages).await?;
            let result = response.map(|r| {
                r.content
                    .iter()
                    .filter_map(|c| match c {
                        anthropic::ResponseContent::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("")
            });
            results.push((*idx, result));
        }
        eprintln!();
    } else {
        // Queue all for batching
        for (idx, prompt) in &repair_items {
            let messages = vec![Message {
                role: Role::User,
                content: vec![RequestContent::Text {
                    text: prompt.clone(),
                    cache_control: None,
                }],
            }];

            let response = client.generate(MODEL, 16384, messages).await?;
            let result = response.map(|r| {
                r.content
                    .iter()
                    .filter_map(|c| match c {
                        anthropic::ResponseContent::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("")
            });
            results.push((*idx, result));
        }

        // Sync batches (upload pending, download finished)
        client.sync_batches().await?;

        if args.wait {
            eprintln!("Waiting for batch to complete...");
            loop {
                std::thread::sleep(std::time::Duration::from_secs(30));
                client.sync_batches().await?;

                // Re-check all items that didn't have results
                let mut all_done = true;
                for (result_idx, (idx, prompt)) in repair_items.iter().enumerate() {
                    if results[result_idx].1.is_none() {
                        let messages = vec![Message {
                            role: Role::User,
                            content: vec![RequestContent::Text {
                                text: prompt.clone(),
                                cache_control: None,
                            }],
                        }];

                        let response = client.generate(MODEL, 16384, messages).await?;
                        if let Some(r) = response {
                            let text = r
                                .content
                                .iter()
                                .filter_map(|c| match c {
                                    anthropic::ResponseContent::Text { text } => {
                                        Some(text.as_str())
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join("");
                            results[result_idx] = (*idx, Some(text));
                        } else {
                            all_done = false;
                        }
                    }
                }

                let done_count = results.iter().filter(|(_, r)| r.is_some()).count();
                if all_done {
                    break;
                }
                eprintln!(
                    "Still waiting... {}/{} results",
                    done_count,
                    repair_items.len()
                );
            }
        } else {
            let pending_count = results.iter().filter(|(_, r)| r.is_none()).count();
            if pending_count > 0 {
                eprintln!(
                    "Batch submitted. {} pending. Run again later to retrieve results.",
                    pending_count
                );
            }
        }
    }

    // Build results map by index
    let mut results_by_idx: std::collections::HashMap<usize, String> =
        std::collections::HashMap::new();
    for (idx, result) in results {
        if let Some(r) = result {
            results_by_idx.insert(idx, r);
        }
    }

    // Output results
    let mut writer: Box<dyn Write> = if let Some(path) = output_path {
        Box::new(BufWriter::new(std::fs::File::create(path)?))
    } else {
        Box::new(std::io::stdout())
    };

    let mut num_repaired = 0;
    let mut num_repair_errors = 0;

    for (idx, example) in examples.iter_mut().enumerate() {
        // Add repair prediction if we have a result
        if let Some(response_text) = results_by_idx.get(&idx) {
            match parse_repair_response(example, response_text) {
                Ok(prediction) => {
                    example.predictions.push(prediction);
                    num_repaired += 1;
                }
                Err(e) => {
                    // Add error prediction
                    example.predictions.push(ExamplePrediction {
                        actual_patch: None,
                        actual_output: response_text.clone(),
                        error: Some(format!("Failed to parse repair response: {}", e)),
                        provider: PredictionProvider::Repair,
                    });
                    num_repair_errors += 1;
                }
            }
        }

        writeln!(writer, "{}", serde_json::to_string(&example)?)?;
    }

    if let Some(path) = output_path {
        eprintln!("Results written to {}", path.display());
    }

    eprintln!("Repaired:      {} items", num_repaired);
    eprintln!("Repair errors: {} items", num_repair_errors);

    Ok(())
}
