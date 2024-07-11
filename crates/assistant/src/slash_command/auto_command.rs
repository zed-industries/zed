use super::create_label_for_command;
use super::{SlashCommand, SlashCommandOutput};
use crate::{CompletionProvider, LanguageModelRequest, LanguageModelRequestMessage, Role};
use anyhow::{anyhow, Result};
use futures::StreamExt;
use gpui::{AppContext, AsyncAppContext, Task, WeakView};
use language::{CodeLabel, LspAdapterDelegate};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicBool, Arc};
use ui::WindowContext;
use workspace::Workspace;

pub(crate) struct AutoCommand;

impl SlashCommand for AutoCommand {
    fn name(&self) -> String {
        "auto".into()
    }

    fn description(&self) -> String {
        "Automatically infer what context to add, based on your prompt".into()
    }

    fn menu_text(&self) -> String {
        "Automatically Infer Context".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("auto", &["--prompt"], cx)
    }

    fn complete_argument(
        self: Arc<Self>,
        _query: String,
        _cancellation_flag: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        // There's no autocomplete for a prompt, since it's arbitrary text.
        Task::ready(Ok(Vec::new()))
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing prompt")));
        };

        // to_string() is needed so it can live long enough to be used in cx.spawn
        let original_prompt = argument.to_string();
        let task = cx.spawn(|cx: gpui::AsyncWindowContext| async move {
            let summaries: Vec<FileSummary> = serde_json::from_str(SUMMARY).unwrap_or_else(|_| {
                // Since we generate the JSON ourselves, this parsing should never fail. If it does, that's a bug.
                log::error!("JSON parsing of project file summaries failed");

                // Handle this gracefully by not including any summaries. Assistant results
                // will be worse than if we actually had summaries, but we won't block the user.
                Vec::new()
            });

            commands_for_summaries(&summaries, &original_prompt, &cx).await
        });

        // As a convenience, append /auto's argument to the end of the prompt
        // so you don't have to write it again.
        let original_prompt = argument.to_string();

        cx.background_executor().spawn(async move {
            let commands = task.await?;
            let mut prompt = String::new();

            log::info!(
                "Translating this response into slash-commands: {:?}",
                commands
            );

            for command in commands {
                prompt.push('/');
                prompt.push_str(&command.name);
                prompt.push(' ');
                prompt.push_str(&command.arg);
                prompt.push('\n');
            }

            prompt.push('\n');
            prompt.push_str(&original_prompt);

            Ok(SlashCommandOutput {
                text: prompt,
                sections: Vec::new(),
                run_commands_in_text: true,
            })
        })
    }
}

const PROMPT_INSTRUCTIONS_BEFORE_SUMMARY: &str = include_str!("prompt_before_summary.txt");
const PROMPT_INSTRUCTIONS_AFTER_SUMMARY: &str = include_str!("prompt_after_summary.txt");
const SUMMARY: &str = include_str!("/Users/rtfeldman/code/summarize-dir/combined_summaries.json");

#[derive(Serialize, Deserialize)]
struct FileSummary {
    filename: String,
    summary: String,
}

fn summaries_prompt(summaries: &[FileSummary], original_prompt: &str) -> String {
    let json_summaries = serde_json::to_string(summaries).unwrap();

    format!("{PROMPT_INSTRUCTIONS_BEFORE_SUMMARY}\n{json_summaries}\n{PROMPT_INSTRUCTIONS_AFTER_SUMMARY}\n{original_prompt}")
}

/// The slash commands that the model is told about, and which we look for in the inference response.
const SUPPORTED_SLASH_COMMANDS: &[&str] = &["search", "file"];

#[derive(Debug, Clone)]
struct CommandToRun {
    name: String,
    arg: String,
}

/// Given the pre-indexed file summaries for this project, as well as the original prompt
/// string passed to `/auto`, get a list of slash commands to run, along with their arguments.
///
/// The prompt's output does not include the slashes (to reduce the chance that it makes a mistake),
/// so taking one of these returned Strings and turning it into a real slash-command-with-argument
/// involves prepending a slash to it.
///
/// This function will validate that each of the returned lines begins with one of SUPPORTED_SLASH_COMMANDS.
/// Any other lines it encounters will be discarded, with a warning logged.
async fn commands_for_summaries(
    summaries: &[FileSummary],
    original_prompt: &str,
    cx: &AsyncAppContext,
) -> Result<Vec<CommandToRun>> {
    if summaries.is_empty() {
        return Ok(Vec::new());
    }

    let model = cx.update(|cx| CompletionProvider::global(cx).model())?;
    let max_token_count = model.max_token_count();

    // Rather than recursing (which would require this async function use a pinned box),
    // we use an explicit stack of arguments and answers for when we need to "recurse."
    let mut stack = vec![(summaries, String::new())];
    let mut final_response = Vec::new();

    while let Some((current_summaries, mut accumulated_response)) = stack.pop() {
        // The split can result in one slice being empty and the other having one element.
        // Whenever that happens, skip the empty one.
        if current_summaries.is_empty() {
            continue;
        }

        log::info!(
            "Inferring prompt context using {} file summaries",
            current_summaries.len()
        );

        let request = LanguageModelRequest {
            model: model.clone(),
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: summaries_prompt(&current_summaries, original_prompt),
            }],
            stop: Vec::new(),
            temperature: 1.0,
        };

        let token_count = cx
            .update(|cx| CompletionProvider::global(cx).count_tokens(request.clone(), cx))?
            .await?;

        if token_count < max_token_count {
            let mut response_chunks = cx
                .update(|cx| CompletionProvider::global(cx).complete(request))?
                .await?;

            while let Some(chunk) = response_chunks.next().await {
                accumulated_response.push_str(&chunk?);
            }

            for line in accumulated_response.split('\n') {
                if let Some(first_space) = line.find(' ') {
                    let command = &line[..first_space].trim();
                    let arg = &line[first_space..].trim();

                    // Don't return empty or duplicate or duplicate commands
                    if !command.is_empty()
                        && !final_response
                            .iter()
                            .any(|cmd: &CommandToRun| cmd.name == *command && cmd.arg == *arg)
                    {
                        if SUPPORTED_SLASH_COMMANDS
                            .iter()
                            .any(|supported| command == supported)
                        {
                            final_response.push(CommandToRun {
                                name: command.to_string(),
                                arg: arg.to_string(),
                            });
                        } else {
                            log::warn!(
                                "Context inference returned an unrecognized slash-commend line: {:?}",
                                line
                            );
                        }
                    }
                } else if !line.trim().is_empty() {
                    // All slash-commands currently supported in context inference need a space for the argument.
                    log::warn!(
                        "Context inference returned a non-blank line that contained no spaces (meaning no argument for the slash-command): {:?}",
                        line
                    );
                }
            }
        } else if current_summaries.len() == 1 {
            log::warn!("Inferring context for a single file's summary failed because the prompt's token length exceeded the model's token limit.");
        } else {
            log::info!(
                "Context inference using file summaries resulted in a prompt containing {token_count} tokens, which exceeded the model's max of {max_token_count}. Retrying as two separate prompts, each including half the number of summaries.",
            );
            let (left, right) = current_summaries.split_at(current_summaries.len() / 2);
            stack.push((right, accumulated_response.clone()));
            stack.push((left, accumulated_response));
        }
    }

    // Sort the commands by name (reversed just so that /search appears before /file)
    final_response.sort_by(|cmd1, cmd2| cmd1.name.cmp(&cmd2.name).reverse());

    Ok(final_response)
}
