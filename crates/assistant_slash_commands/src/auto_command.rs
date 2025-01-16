use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use feature_flags::FeatureFlag;
use futures::StreamExt;
use gpui::{AppContext, AsyncAppContext, AsyncWindowContext, Task, WeakView, WindowContext};
use language::{CodeLabel, LspAdapterDelegate};
use language_model::{
    LanguageModelCompletionEvent, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, Role,
};
use semantic_index::{FileSummary, SemanticDb};
use smol::channel;
use std::sync::{atomic::AtomicBool, Arc};
use ui::{prelude::*, BorrowAppContext};
use util::ResultExt;
use workspace::Workspace;

use crate::create_label_for_command;

pub struct AutoSlashCommandFeatureFlag;

impl FeatureFlag for AutoSlashCommandFeatureFlag {
    const NAME: &'static str = "auto-slash-command";
}

pub struct AutoCommand;

impl SlashCommand for AutoCommand {
    fn name(&self) -> String {
        "auto".into()
    }

    fn description(&self) -> String {
        "Automatically infer what context to add".into()
    }

    fn icon(&self) -> IconName {
        IconName::Wand
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("auto", &["--prompt"], cx)
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        // There's no autocomplete for a prompt, since it's arbitrary text.
        // However, we can use this opportunity to kick off a drain of the backlog.
        // That way, it can hopefully be done resummarizing by the time we've actually
        // typed out our prompt. This re-runs on every keystroke during autocomplete,
        // but in the future, we could instead do it only once, when /auto is first entered.
        let Some(workspace) = workspace.and_then(|ws| ws.upgrade()) else {
            log::warn!("workspace was dropped or unavailable during /auto autocomplete");

            return Task::ready(Ok(Vec::new()));
        };

        let project = workspace.read(cx).project().clone();
        let Some(project_index) =
            cx.update_global(|index: &mut SemanticDb, cx| index.project_index(project, cx))
        else {
            return Task::ready(Err(anyhow!("No project indexer, cannot use /auto")));
        };

        let cx: &mut AppContext = cx;

        cx.spawn(|cx: gpui::AsyncAppContext| async move {
            let task = project_index.read_with(&cx, |project_index, cx| {
                project_index.flush_summary_backlogs(cx)
            })?;

            cx.background_executor().spawn(task).await;

            anyhow::Ok(Vec::new())
        })
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: language::BufferSnapshot,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };
        if arguments.is_empty() {
            return Task::ready(Err(anyhow!("missing prompt")));
        };
        let argument = arguments.join(" ");
        let original_prompt = argument.to_string();
        let project = workspace.read(cx).project().clone();
        let Some(project_index) =
            cx.update_global(|index: &mut SemanticDb, cx| index.project_index(project, cx))
        else {
            return Task::ready(Err(anyhow!("no project indexer")));
        };

        let task = cx.spawn(|cx: AsyncWindowContext| async move {
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
            }
            .to_event_stream())
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
        log::warn!("Inferring no context because there were no summaries available.");
        return Ok(Vec::new());
    }

    // Use the globally configured model to translate the summaries into slash-commands,
    // because Qwen2-7B-Instruct has not done a good job at that task.
    let Some(model) = cx.update(|cx| LanguageModelRegistry::read_global(cx).active_model())? else {
        log::warn!("Can't infer context because there's no active model.");
        return Ok(Vec::new());
    };
    // Only go up to 90% of the actual max token count, to reduce chances of
    // exceeding the token count due to inaccuracies in the token counting heuristic.
    let max_token_count = (model.max_token_count() * 9) / 10;

    // Rather than recursing (which would require this async function use a pinned box),
    // we use an explicit stack of arguments and answers for when we need to "recurse."
    let mut stack = vec![summaries];
    let mut final_response = Vec::new();
    let mut prompts = Vec::new();

    // TODO We only need to create multiple Requests because we currently
    // don't have the ability to tell if a CompletionProvider::complete response
    // was a "too many tokens in this request" error. If we had that, then
    // we could try the request once, instead of having to make separate requests
    // to check the token count and then afterwards to run the actual prompt.
    let make_request = |prompt: String| LanguageModelRequest {
        messages: vec![LanguageModelRequestMessage {
            role: Role::User,
            content: vec![prompt.into()],
            // Nothing in here will benefit from caching
            cache: false,
        }],
        tools: Vec::new(),
        stop: Vec::new(),
        temperature: None,
    };

    while let Some(current_summaries) = stack.pop() {
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
        let start = std::time::Instant::now();
        // Per OpenAI, 1 token ~= 4 chars in English (we go with 4.5 to overestimate a bit, because failed API requests cost a lot of perf)
        // Verifying this against an actual model.count_tokens() confirms that it's usually within ~5% of the correct answer, whereas
        // getting the correct answer from tiktoken takes hundreds of milliseconds (compared to this arithmetic being ~free).
        // source: https://help.openai.com/en/articles/4936856-what-are-tokens-and-how-to-count-them
        let token_estimate = prompt.len() * 2 / 9;
        let duration = start.elapsed();
        log::info!(
            "Time taken to count tokens for prompt of length {:?}B: {:?}",
            prompt.len(),
            duration
        );

        if token_estimate < max_token_count {
            prompts.push(prompt);
        } else if current_summaries.len() == 1 {
            log::warn!("Inferring context for a single file's summary failed because the prompt's token length exceeded the model's token limit.");
        } else {
            log::info!(
                "Context inference using file summaries resulted in a prompt containing {token_estimate} tokens, which exceeded the model's max of {max_token_count}. Retrying as two separate prompts, each including half the number of summaries.",
            );
            let (left, right) = current_summaries.split_at(current_summaries.len() / 2);
            stack.push(right);
            stack.push(left);
        }
    }

    let all_start = std::time::Instant::now();

    let (tx, rx) = channel::bounded(1024);

    let completion_streams = prompts
        .into_iter()
        .map(|prompt| {
            let request = make_request(prompt.clone());
            let model = model.clone();
            let tx = tx.clone();
            let stream = model.stream_completion(request, &cx);

            (stream, tx)
        })
        .collect::<Vec<_>>();

    cx.background_executor()
        .spawn(async move {
            let futures = completion_streams
                .into_iter()
                .enumerate()
                .map(|(ix, (stream, tx))| async move {
                    let start = std::time::Instant::now();
                    let events = stream.await?;
                    log::info!("Time taken for awaiting /await chunk stream #{ix}: {:?}", start.elapsed());

                    let completion: String = events
                        .filter_map(|event| async {
                            if let Ok(LanguageModelCompletionEvent::Text(text)) = event {
                                Some(text)
                            } else {
                                None
                            }
                        })
                        .collect()
                        .await;

                    log::info!("Time taken for all /auto chunks to come back for #{ix}: {:?}", start.elapsed());

                    for line in completion.split('\n') {
                        if let Some(first_space) = line.find(' ') {
                            let command = &line[..first_space].trim();
                            let arg = &line[first_space..].trim();

                            tx.send(CommandToRun {
                                name: command.to_string(),
                                arg: arg.to_string(),
                            })
                            .await?;
                        } else if !line.trim().is_empty() {
                            // All slash-commands currently supported in context inference need a space for the argument.
                            log::warn!(
                                "Context inference returned a non-blank line that contained no spaces (meaning no argument for the slash command): {:?}",
                                line
                            );
                        }
                    }

                    anyhow::Ok(())
                })
                .collect::<Vec<_>>();

            let _ = futures::future::try_join_all(futures).await.log_err();

            let duration = all_start.elapsed();
            eprintln!("All futures completed in {:?}", duration);
        })
        .await;

    drop(tx); // Close the channel so that rx.collect() won't hang. This is safe because all futures have completed.
    let results = rx.collect::<Vec<_>>().await;
    eprintln!(
        "Finished collecting from the channel with {} results",
        results.len()
    );
    for command in results {
        // Don't return empty or duplicate commands
        if !command.name.is_empty()
            && !final_response
                .iter()
                .any(|cmd: &CommandToRun| cmd.name == command.name && cmd.arg == command.arg)
        {
            if SUPPORTED_SLASH_COMMANDS
                .iter()
                .any(|supported| &command.name == supported)
            {
                final_response.push(command);
            } else {
                log::warn!(
                    "Context inference returned an unrecognized slash command: {:?}",
                    command
                );
            }
        }
    }

    // Sort the commands by name (reversed just so that /search appears before /file)
    final_response.sort_by(|cmd1, cmd2| cmd1.name.cmp(&cmd2.name).reverse());

    Ok(final_response)
}
