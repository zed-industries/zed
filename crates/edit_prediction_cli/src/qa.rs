//! Quality assessment of predictions using LLM-as-a-judge.
//!
//! This module uses the Anthropic Batch API to evaluate prediction quality.
//! Caching is handled by the underlying AnthropicClient.

use crate::anthropic_client::AnthropicClient;
use crate::example::Example;
use crate::format_prompt::extract_cursor_excerpt_from_example;
use crate::paths::LLM_CACHE_DB;
use crate::word_diff::unified_to_word_diff;
use anthropic::{Message, RequestContent, Role};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

/// Model to use for QA evaluation.
const MODEL: &str = "claude-sonnet-4-5";

const PROMPT_TEMPLATE: &str = include_str!("prompts/qa.md");

/// Arguments for the QA command.
#[derive(Debug, Clone, clap::Args)]
pub struct QaArgs {
    /// Use synchronous API instead of batch
    #[clap(long)]
    pub no_batch: bool,

    /// Wait for batch to complete (polls every 30s)
    #[clap(long)]
    pub wait: bool,
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
pub fn build_prompt(example: &Example) -> Option<String> {
    let prediction = example.predictions.first()?;
    let actual_patch = prediction.actual_patch.as_ref()?;
    let prompt_inputs = example.prompt_inputs.as_ref()?;

    let actual_patch_word_diff = unified_to_word_diff(actual_patch);

    // Format cursor excerpt (reuse from format_prompt)
    let cursor_excerpt = extract_cursor_excerpt_from_example(example)?;

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

    Some(
        PROMPT_TEMPLATE
            .replace("{edit_history}", &edit_history)
            .replace("{cursor_excerpt}", &cursor_excerpt)
            .replace("{actual_patch_word_diff}", &actual_patch_word_diff),
    )
}

/// Extract a code block from a response.
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

/// Parse the LLM response into a QaResult.
fn parse_response(response_text: &str) -> QaResult {
    let codeblock = extract_codeblock(response_text);

    // Try parsing codeblock first, then fall back to raw response
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

    // If all parsing attempts fail, return error
    QaResult {
        reasoning: Some(response_text.to_string()),
        reverts_edits: None,
        confidence: None,
        response: Some(response_text.to_string()),
        error: Some("Could not parse JSON from response".to_string()),
    }
}

/// Run the QA evaluation on a set of examples.
pub async fn run_qa(
    examples: &mut [Example],
    args: &QaArgs,
    output_path: Option<&PathBuf>,
) -> Result<()> {
    let client = if args.no_batch {
        AnthropicClient::plain()?
    } else {
        AnthropicClient::batch(&LLM_CACHE_DB)?
    };

    eprintln!("Using model: {}, batching: {}", MODEL, !args.no_batch);

    // First pass: send requests (client handles caching internally)
    let mut prompts: Vec<(usize, String)> = Vec::new();
    let mut skipped_count = 0;

    for (idx, example) in examples.iter().enumerate() {
        let Some(prompt) = build_prompt(example) else {
            skipped_count += 1;
            continue;
        };
        prompts.push((idx, prompt));
    }

    if skipped_count > 0 {
        eprintln!("Skipping {} items with missing actual_patch", skipped_count);
    }

    eprintln!("{} items to process", prompts.len());

    // Process all items
    let mut results: Vec<(usize, Option<QaResult>)> = Vec::new();

    if args.no_batch {
        // Synchronous processing
        for (i, (idx, prompt)) in prompts.iter().enumerate() {
            eprint!("\rProcessing {}/{}", i + 1, prompts.len());

            let messages = vec![Message {
                role: Role::User,
                content: vec![RequestContent::Text {
                    text: prompt.clone(),
                    cache_control: None,
                }],
            }];

            let response = client.generate(MODEL, 1024, messages).await?;
            let result = response.map(|r| {
                let text = r
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        anthropic::ResponseContent::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                parse_response(&text)
            });
            results.push((*idx, result));
        }
        eprintln!();
    } else {
        // Queue all for batching
        for (idx, prompt) in &prompts {
            let messages = vec![Message {
                role: Role::User,
                content: vec![RequestContent::Text {
                    text: prompt.clone(),
                    cache_control: None,
                }],
            }];

            let response = client.generate(MODEL, 1024, messages).await?;
            let result = response.map(|r| {
                let text = r
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        anthropic::ResponseContent::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                parse_response(&text)
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
                for (result_idx, (idx, prompt)) in prompts.iter().enumerate() {
                    if results[result_idx].1.is_none() {
                        let messages = vec![Message {
                            role: Role::User,
                            content: vec![RequestContent::Text {
                                text: prompt.clone(),
                                cache_control: None,
                            }],
                        }];

                        let response = client.generate(MODEL, 1024, messages).await?;
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
                            results[result_idx] = (*idx, Some(parse_response(&text)));
                        } else {
                            all_done = false;
                        }
                    }
                }

                let done_count = results.iter().filter(|(_, r)| r.is_some()).count();
                if all_done {
                    break;
                }
                eprintln!("Still waiting... {}/{} results", done_count, prompts.len());
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
    let mut results_by_idx: std::collections::HashMap<usize, QaResult> =
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

    let mut num_total = 0;
    let mut num_reverts_edits = 0;

    for (idx, example) in examples.iter_mut().enumerate() {
        // Skip examples that couldn't be processed
        if build_prompt(example).is_none() {
            continue;
        }

        let result = results_by_idx.get(&idx).cloned();

        if result.as_ref().and_then(|r| r.reverts_edits) == Some(true) {
            num_reverts_edits += 1;
        }
        num_total += 1;

        // Populate QA results for each prediction (currently only first prediction is evaluated)
        example.qa = example
            .predictions
            .iter()
            .enumerate()
            .map(|(i, _)| if i == 0 { result.clone() } else { None })
            .collect();

        writeln!(writer, "{}", serde_json::to_string(&example)?)?;
    }

    if let Some(path) = output_path {
        eprintln!("Results written to {}", path.display());
    }

    eprintln!("Processed:     {} items", num_total);
    if num_total > 0 {
        eprintln!(
            "Reverts edits: {} ({:.2}%)",
            num_reverts_edits,
            num_reverts_edits as f64 / num_total as f64 * 100.0
        );
    }

    Ok(())
}
