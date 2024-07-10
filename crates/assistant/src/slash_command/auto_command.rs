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

        let stream = CompletionProvider::global(cx).complete(request);
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

        // As a convenience, append /auto's argument to the end of the prompt
        // so you don't have to write it again.
        let argument = argument.to_string();

        cx.background_executor().spawn(async move {
            let mut text = task.await?;

            text.push_str(&argument);

            Ok(SlashCommandOutput {
                text,
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
    full_summary: &str,
    completion_provider: CompletionProvider,
    cx: &AppContext,
) -> Result<Vec<String>> {
    let model = completion_provider.model();
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

        completion_provider.count_tokens(request, cx)
    };
    let full_summary_tokens = tokens_needed(full_summary.to_string()).await?;

    // See if the summary will fit in one request. If so, we're done!
    if full_summary_tokens <= max_token_count {
        Ok(vec![full_summary.to_string()])
    } else {
        let mut buf = String::new();

        // We need to split up the request into smaller chunks.
        for context in full_summary.trim().split(OPENING_CONTEXT_TAG) {
            let context = context.trim();

            if context.ends_with(CLOSING_CONTEXT_TAG) {
                buf.push_str(OPENING_CONTEXT_TAG);
                buf.push_str(context);
            }
        }
    }

    // If we can't get chunks of at least this size, decide it's hopeless and we give up.
    // Otherwise it's going to take an unreasonable number of calls to the model to get everything.
    const MIN_CHUNK_LENGTH: usize = 2048 + OPENING_CONTEXT_TAG.len() + CLOSING_CONTEXT_TAG.len();
    let mut chunk_start = 0;
    let mut chunk_len = full_summary.len();
    let mut answer = Vec::new();

    loop {
        let mut chunk = &full_summary[chunk_start..(chunk_start + chunk_len)];

        let Some(last_opened) = chunk.rfind(OPENING_CONTEXT_TAG) else {
            // If we don't have any opened tags in the slice, then we're done!
            // One way this can happen is if the slice is empty, because we've done the last chunk.
            // (This implies that if the whole summary had no tags, we return an empty vec.)
            return Ok(answer);
        };
        let last_closed = chunk.rfind(CLOSING_CONTEXT_TAG).unwrap_or(chunk.len());

        // If we opened one without closing it, defer handling it to the next slice.
        if last_closed < last_opened {
            chunk = &chunk[..(last_closed + CLOSING_CONTEXT_TAG.len())];
            chunk_start = last_opened;
        }

        let tokens_needed = {
            let request = LanguageModelRequest {
                model: model.clone(),
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: chunk.to_string(),
                }],
                stop: Vec::new(),
                temperature: 1.0,
            };

            completion_provider.count_tokens(request, cx).await?
        };

        // This isn't going to fit in one request; calculate a new chunk size,
        // and then loop back to try again with a different chunk size.
        if tokens_needed > max_token_count {
            // A chunk length based on a heuristic
            let heuristic_len = {
                let ratio = (tokens_needed as f64) / (max_token_count as f64);

                full_summary.len() as f64 * ratio
            };

            if heuristic_len.is_finite() {
                chunk_len = (heuristic_len
                    .round()
                    // Always make sure the chunk is *at least* divided in half. This way, if we pick
                    // a chunk size and the next token request says it's still over the limit (which
                    // could happen!) then we still make progress with a smaller chunk, and can't get
                    // stuck in an infinit loop.
                    .min(heuristic_len / 2.0) as usize)
                    .max(MIN_CHUNK_LENGTH);
            } else {
                // There was division by zero, or something overflowed; we won't be able to summarize.
                return Ok(answer);
            }
        } else {
            // This will fit in a request! Add it to the answer.
            answer.push(chunk.to_string());

            chunk_start += 1;
        }
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
