use super::create_label_for_command;
use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::ArgumentCompletion;
use completion::LanguageModelCompletionProvider;
use gpui::{AppContext, AsyncAppContext, Task, WeakView};
use language::{CodeLabel, LspAdapterDelegate};
use language_model::{LanguageModelRequest, LanguageModelRequestMessage, Role};
use semantic_index::{FileSummary, SemanticIndex};
use std::sync::{atomic::AtomicBool, Arc};
use ui::{BorrowAppContext, WindowContext};
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
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        // There's no autocomplete for a prompt, since it's arbitrary text.
        Task::ready(Ok(Vec::new()))
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };
        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing prompt")));
        };

        let original_prompt = argument.to_string();
        let project = workspace.read(cx).project().clone();
        let project_index =
            cx.update_global(|index: &mut SemanticIndex, cx| index.project_index(project, cx));

        let task = cx.spawn(|cx: gpui::AsyncWindowContext| async move {
            let summaries = project_index
                .read_with(&cx, |project_index, cx| project_index.all_summaries(cx))?
                .await?;

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

    let Some(model) =
        cx.update(|cx| LanguageModelCompletionProvider::read_global(cx).active_model())?
    else {
        log::info!("Can't infer context because there's no active model.");
        return Ok(Vec::new());
    };
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

        let prompt = summaries_prompt(&current_summaries, original_prompt);

        // TODO We only need to create multiple Requests becuase we currently
        // don't have the ability to tell if a CompletionProvider::complete response
        // was a "too many tokens in this request" error. If we had that, then
        // we could try the request once, instead of having to make separate requests
        // to check the token count and then afterwards to run the actual prompt.
        let make_request = |prompt: String| LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: prompt.clone(),
            }],
            stop: Vec::new(),
            temperature: 1.0,
        };

        if let Some(token_count_future) = cx.update(|cx| {
            LanguageModelCompletionProvider::read_global(cx)
                .count_tokens(make_request(prompt.clone()), cx)
        })? {
            let token_count = token_count_future.await?;

            if token_count < max_token_count {
                let response = cx
                    .update(|cx| {
                        LanguageModelCompletionProvider::read_global(cx)
                            .complete(make_request(prompt), cx)
                    })?
                    .await?;

                accumulated_response.push_str(&response);

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
        } else {
            log::warn!("Inferring context for a single file's summary failed because getting the token count for the prompt returned None (which might have meant there was no global active model).");
        }
    }

    // Sort the commands by name (reversed just so that /search appears before /file)
    final_response.sort_by(|cmd1, cmd2| cmd1.name.cmp(&cmd2.name).reverse());

    Ok(final_response)
}
