use super::create_label_for_command;
use super::{SlashCommand, SlashCommandOutput};
use crate::{
    completion_provider, CompletionProvider, LanguageModel, LanguageModelRequest,
    LanguageModelRequestMessage, Role,
};
use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use futures::StreamExt;
use gpui::{AppContext, Task, WeakView};
use language::{CodeLabel, LspAdapterDelegate};
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

        let provider = CompletionProvider::global(cx);
        let summaries_future = summaries(SUMMARY.to_string(), provider, cx);

        cx.background_executor().spawn(async move {
            let mut answer = String::new();

            for summary in summaries_future.await? {
                let prompt = format!("{PROMPT_INSTRUCTIONS_BEFORE_SUMMARY}\n{SUMMARY}\n{PROMPT_INSTRUCTIONS_AFTER_SUMMARY}\n{argument}");
                let request = LanguageModelRequest {
                    model: CompletionProvider::global(cx).model(),
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: prompt,
                    }],
                    stop: vec![],
                    temperature: 1.0,
                };

                let stream = provider.complete(request);
                let mut wip_action: String = String::new();
                let task: Task<Result<String>> = cx.spawn(|_cx| async move {
                    let mut actions_text = String::new();
                    let stream_completion = async {
                        let mut messages = stream.await?;

                        while let Some(message) = messages.next().await {
                            let text = message?;

                            chunked_line(&mut wip_action, &text, |line| {
                                actions_text.push('/');
                                actions_text.push_str(line);
                                actions_text.push('\n');
                            });

                            smol::future::yield_now().await;
                        }

                        anyhow::Ok(())
                    };

                    stream_completion.await?;

                    Ok(actions_text)
                });

                answer.push_str(&task.await?);
            }

            // As a convenience, append /auto's argument to the end of the prompt
            // so you don't have to write it again.
            let argument = argument.to_string();

            answer.push_str(&argument);

            Ok(SlashCommandOutput {
                text: answer,
                sections: Vec::new(),
                run_commands_in_text: true,
            })
        })
    }
}

const PROMPT_INSTRUCTIONS_BEFORE_SUMMARY: &str = include_str!("prompt_before_summary.txt");
const PROMPT_INSTRUCTIONS_AFTER_SUMMARY: &str = include_str!("prompt_after_summary.txt");
const SUMMARY: &str =
    include_str!("/Users/rtfeldman/code/summarize-dir/zed-output/combined_summaries.xml");

const OPENING_CONTEXT_TAG: &str = "<context>";
const CLOSING_CONTEXT_TAG: &str = "</context>";

async fn summaries(
    full_summary: String,
    provider: &CompletionProvider,
    cx: &AppContext,
) -> Result<Vec<String>> {
    let model = provider.model();
    let max_token_count = model.max_token_count();
    let tokens_needed = |content: String| {
        let request = LanguageModelRequest {
            model: model.clone(),
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content,
            }],
            stop: Vec::new(),
            temperature: 1.0,
        };

        provider.count_tokens(request, cx)
    };
    let full_summary_tokens = tokens_needed(full_summary.to_string()).await?;

    // If the full summary is under the max token count, we're done!
    if full_summary_tokens <= max_token_count {
        Ok(vec![full_summary])
    } else {
        let mut answer = Vec::new();
        let mut chunk = String::new();

        // Split up the request into smaller chunks, each of which fits .
        for context in full_summary.trim().split(OPENING_CONTEXT_TAG) {
            let context = context.trim();

            if context.ends_with(CLOSING_CONTEXT_TAG) {
                let candidate_chunk = format!("{chunk}{OPENING_CONTEXT_TAG}{context}");

                // In the case of custom model providers, this will do a network request.
                // That means this will perform one network request per file, which is way too much!
                // We can address that by changing this algorithm to split the contexts up into chunks
                // of several context pieces at a time, see if the chunks fit; if they don't,
                // try a smaller chunk size until they fit, etc. That will be much more complex, but faster.
                if tokens_needed(candidate_chunk).await? <= max_token_count {
                    chunk.push_str(OPENING_CONTEXT_TAG);
                    chunk.push_str(context);
                } else {
                    // Adding the current context to the accumulated chunk puts it over the token limit,
                    // so push what we've accumulated so far to the final list of chunks, and make the
                    // current context be the beginning of the next accumulated chunk.
                    if !chunk.is_empty() {
                        // Don't bother pushing empty chunks.
                        answer.push(chunk);
                    }

                    chunk = format!("{OPENING_CONTEXT_TAG}{context}");
                }
            }
        }

        Ok(answer)
    }
}

fn chunked_line(wip: &mut String, chunk: &str, mut on_line_end: impl FnMut(&str)) {
    // The first iteration of the loop should just push to wip
    // and nothing else. We only push what we encountered in
    // previous iterations of the loop.
    //
    // This correctly handles both the scenario where no
    // newlines are encountered (the loop will only run once,
    // and so will only push to wip), as well as the scenario
    // where the chunk contains at least one newline but
    // does not end in a newline (the last iteration of the
    // loop will update wip but will not run anything).
    let mut is_first_iteration = true;

    for line in chunk.split('\n') {
        if is_first_iteration {
            is_first_iteration = false;
        } else {
            // Since this isn't the first iteration of the loop, we definitely hit a newline
            // at the end of the previous iteration! Run the function on whatever wip we have.
            on_line_end(wip);
            wip.clear();
        }

        wip.push_str(line);
    }
}
