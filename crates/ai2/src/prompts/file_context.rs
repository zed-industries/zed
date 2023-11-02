use anyhow::anyhow;
use language::BufferSnapshot;
use language::ToOffset;

use crate::models::LanguageModel;
use crate::models::TruncationDirection;
use crate::prompts::base::PromptArguments;
use crate::prompts::base::PromptTemplate;
use std::fmt::Write;
use std::ops::Range;
use std::sync::Arc;

fn retrieve_context(
    buffer: &BufferSnapshot,
    selected_range: &Option<Range<usize>>,
    model: Arc<dyn LanguageModel>,
    max_token_count: Option<usize>,
) -> anyhow::Result<(String, usize, bool)> {
    let mut prompt = String::new();
    let mut truncated = false;
    if let Some(selected_range) = selected_range {
        let start = selected_range.start.to_offset(buffer);
        let end = selected_range.end.to_offset(buffer);

        let start_window = buffer.text_for_range(0..start).collect::<String>();

        let mut selected_window = String::new();
        if start == end {
            write!(selected_window, "<|START|>").unwrap();
        } else {
            write!(selected_window, "<|START|").unwrap();
        }

        write!(
            selected_window,
            "{}",
            buffer.text_for_range(start..end).collect::<String>()
        )
        .unwrap();

        if start != end {
            write!(selected_window, "|END|>").unwrap();
        }

        let end_window = buffer.text_for_range(end..buffer.len()).collect::<String>();

        if let Some(max_token_count) = max_token_count {
            let selected_tokens = model.count_tokens(&selected_window)?;
            if selected_tokens > max_token_count {
                return Err(anyhow!(
                    "selected range is greater than model context window, truncation not possible"
                ));
            };

            let mut remaining_tokens = max_token_count - selected_tokens;
            let start_window_tokens = model.count_tokens(&start_window)?;
            let end_window_tokens = model.count_tokens(&end_window)?;
            let outside_tokens = start_window_tokens + end_window_tokens;
            if outside_tokens > remaining_tokens {
                let (start_goal_tokens, end_goal_tokens) =
                    if start_window_tokens < end_window_tokens {
                        let start_goal_tokens = (remaining_tokens / 2).min(start_window_tokens);
                        remaining_tokens -= start_goal_tokens;
                        let end_goal_tokens = remaining_tokens.min(end_window_tokens);
                        (start_goal_tokens, end_goal_tokens)
                    } else {
                        let end_goal_tokens = (remaining_tokens / 2).min(end_window_tokens);
                        remaining_tokens -= end_goal_tokens;
                        let start_goal_tokens = remaining_tokens.min(start_window_tokens);
                        (start_goal_tokens, end_goal_tokens)
                    };

                let truncated_start_window =
                    model.truncate(&start_window, start_goal_tokens, TruncationDirection::Start)?;
                let truncated_end_window =
                    model.truncate(&end_window, end_goal_tokens, TruncationDirection::End)?;
                writeln!(
                    prompt,
                    "{truncated_start_window}{selected_window}{truncated_end_window}"
                )
                .unwrap();
                truncated = true;
            } else {
                writeln!(prompt, "{start_window}{selected_window}{end_window}").unwrap();
            }
        } else {
            // If we dont have a selected range, include entire file.
            writeln!(prompt, "{}", &buffer.text()).unwrap();

            // Dumb truncation strategy
            if let Some(max_token_count) = max_token_count {
                if model.count_tokens(&prompt)? > max_token_count {
                    truncated = true;
                    prompt = model.truncate(&prompt, max_token_count, TruncationDirection::End)?;
                }
            }
        }
    }

    let token_count = model.count_tokens(&prompt)?;
    anyhow::Ok((prompt, token_count, truncated))
}

pub struct FileContext {}

impl PromptTemplate for FileContext {
    fn generate(
        &self,
        args: &PromptArguments,
        max_token_length: Option<usize>,
    ) -> anyhow::Result<(String, usize)> {
        if let Some(buffer) = &args.buffer {
            let mut prompt = String::new();
            // Add Initial Preamble
            // TODO: Do we want to add the path in here?
            writeln!(
                prompt,
                "The file you are currently working on has the following content:"
            )
            .unwrap();

            let language_name = args
                .language_name
                .clone()
                .unwrap_or("".to_string())
                .to_lowercase();

            let (context, _, truncated) = retrieve_context(
                buffer,
                &args.selected_range,
                args.model.clone(),
                max_token_length,
            )?;
            writeln!(prompt, "```{language_name}\n{context}\n```").unwrap();

            if truncated {
                writeln!(prompt, "Note the content has been truncated and only represents a portion of the file.").unwrap();
            }

            if let Some(selected_range) = &args.selected_range {
                let start = selected_range.start.to_offset(buffer);
                let end = selected_range.end.to_offset(buffer);

                if start == end {
                    writeln!(prompt, "In particular, the user's cursor is currently on the '<|START|>' span in the above content, with no text selected.").unwrap();
                } else {
                    writeln!(prompt, "In particular, the user has selected a section of the text between the '<|START|' and '|END|>' spans.").unwrap();
                }
            }

            // Really dumb truncation strategy
            if let Some(max_tokens) = max_token_length {
                prompt = args
                    .model
                    .truncate(&prompt, max_tokens, TruncationDirection::End)?;
            }

            let token_count = args.model.count_tokens(&prompt)?;
            anyhow::Ok((prompt, token_count))
        } else {
            Err(anyhow!("no buffer provided to retrieve file context from"))
        }
    }
}
