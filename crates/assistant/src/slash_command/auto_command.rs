use super::create_label_for_command;
use super::{SlashCommand, SlashCommandOutput};
use crate::{CompletionProvider, LanguageModelRequest, LanguageModelRequestMessage, Role};
use anyhow::{anyhow, Result};
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

        // As a convenience, append /auto's argument to the end of the prompt so you don't have to write it again.
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
const PROMPT_INSTRUCTIONS_BEFORE_SUMMARY: &str = r#"
I'm going to give you a prompt. I don't want you to respond
to the prompt itself. I want you to figure out which of the following
actions on my project, if any, would help you answer the prompt.

Here are the actions:

## file

This action's parameter is a file path to one of the files
in the project. If you ask for this action, I will tell you
the full contents of the file, so you  can learn all the
details of the file.

## search

This action's parameter is a string to search for across
the project. It will tell you which files this string
(or similar strings; it is a semantic search) appear in,
as well as some context of the lines surrounding each result.

---

That was the end of the list of actions.

Here is an XML summary of each of the files in my project:
"#;

const PROMPT_INSTRUCTIONS_AFTER_SUMMARY: &str = r#"
Actions have a cost, so only include actions that you think
will be helpful to you in doing a great job answering the
prompt in the future.

You must respond ONLY with a list of actions you would like to
perform. Each action should be on its own line, and followed by a space and then its parameter.

Actions can be performed more than once with different parameters.
Here is an example valid response:

```
file path/to/my/file.txt
file path/to/another/file.txt
search something to search for
search something else to search for
```

Once again, do not forget: you must respond ONLY in the format of
one action per line, and the action name should be followed by
its parameter. Your response must not include anything other
than a list of actions, with one action per line, in this format.
It is extremely important that you do not deviate from this format even slightly!

This is the end of my instructions for how to respond. The rest is the prompt:
"#;

const SUMMARY: &str = "";

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
