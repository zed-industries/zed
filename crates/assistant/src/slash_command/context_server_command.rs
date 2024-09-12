use anyhow::{anyhow, Result};
use assistant_slash_command::{
    AfterCompletion, ArgumentCompletion, SlashCommand, SlashCommandOutput,
    SlashCommandOutputSection,
};
use collections::HashMap;
use context_servers::{
    manager::{ContextServer, ContextServerManager},
    protocol::PromptInfo,
    types::{SamplingContent, SamplingMessage, SamplingRole},
};
use gpui::{Task, WeakView, WindowContext};
use language::{CodeLabel, LspAdapterDelegate};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use text::LineEnding;
use ui::{IconName, SharedString};
use workspace::Workspace;

pub struct ContextServerSlashCommand {
    server_id: String,
    prompt: PromptInfo,
}

impl ContextServerSlashCommand {
    pub fn new(server: &Arc<ContextServer>, prompt: PromptInfo) -> Self {
        Self {
            server_id: server.id.clone(),
            prompt,
        }
    }
}

impl SlashCommand for ContextServerSlashCommand {
    fn name(&self) -> String {
        self.prompt.name.clone()
    }

    fn description(&self) -> String {
        format!("Run context server command: {}", self.prompt.name)
    }

    fn menu_text(&self) -> String {
        format!("Run '{}' from {}", self.prompt.name, self.server_id)
    }

    fn requires_argument(&self) -> bool {
        self.prompt.arguments.as_ref().map_or(false, |args| {
            args.iter().any(|arg| arg.required == Some(true))
        })
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let server_id = self.server_id.clone();
        let prompt_name = self.prompt.name.clone();
        let manager = ContextServerManager::global(cx);
        let manager = manager.read(cx);

        let (arg_name, arg_val) = match completion_argument(&self.prompt, arguments) {
            Ok(tp) => tp,
            Err(e) => {
                return Task::ready(Err(e));
            }
        };
        if let Some(server) = manager.get_server(&server_id) {
            cx.foreground_executor().spawn(async move {
                let Some(protocol) = server.client.read().clone() else {
                    return Err(anyhow!("Context server not initialized"));
                };

                let completion_result = protocol
                    .completion(
                        context_servers::types::CompletionReference::Prompt(
                            context_servers::types::PromptReference {
                                r#type: context_servers::types::PromptReferenceType::Prompt,
                                name: prompt_name,
                            },
                        ),
                        arg_name,
                        arg_val,
                    )
                    .await?;

                let completions = completion_result
                    .values
                    .into_iter()
                    .map(|value| ArgumentCompletion {
                        label: CodeLabel::plain(value.clone(), None),
                        new_text: value,
                        after_completion: AfterCompletion::Continue,
                        replace_previous_arguments: false,
                    })
                    .collect();

                Ok(completions)
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let server_id = self.server_id.clone();
        let prompt_name = self.prompt.name.clone();

        let prompt_args = match prompt_arguments(&self.prompt, arguments) {
            Ok(args) => args,
            Err(e) => return Task::ready(Err(e)),
        };

        let manager = ContextServerManager::global(cx);
        let manager = manager.read(cx);
        if let Some(server) = manager.get_server(&server_id) {
            cx.foreground_executor().spawn(async move {
                let Some(protocol) = server.client.read().clone() else {
                    return Err(anyhow!("Context server not initialized"));
                };
                let result = protocol.run_prompt(&prompt_name, prompt_args).await?;
                let mut prompt = format_messages(&result.messages);
                // We must normalize the line endings here, since servers might return CR characters.
                LineEnding::normalize(&mut prompt);

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: 0..(prompt.len()),
                        icon: IconName::ZedAssistant,
                        label: SharedString::from(
                            result
                                .description
                                .unwrap_or(format!("Result from {}", prompt_name)),
                        ),
                    }],
                    text: prompt,
                    run_commands_in_text: false,
                })
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
    }
}

fn completion_argument(prompt: &PromptInfo, arguments: &[String]) -> Result<(String, String)> {
    if arguments.is_empty() {
        return Err(anyhow!("No arguments given"));
    }

    match &prompt.arguments {
        Some(args) if args.len() == 1 => {
            let arg_name = args[0].name.clone();
            let arg_value = arguments.join(" ");
            Ok((arg_name, arg_value))
        }
        Some(_) => Err(anyhow!("Prompt must have exactly one argument")),
        None => Err(anyhow!("Prompt has no arguments")),
    }
}

fn prompt_arguments(prompt: &PromptInfo, arguments: &[String]) -> Result<HashMap<String, String>> {
    match &prompt.arguments {
        Some(args) if args.len() > 1 => Err(anyhow!(
            "Prompt has more than one argument, which is not supported"
        )),
        Some(args) if args.len() == 1 => {
            if !arguments.is_empty() {
                let mut map = HashMap::default();
                map.insert(args[0].name.clone(), arguments.join(" "));
                Ok(map)
            } else if arguments.is_empty() && args[0].required == Some(false) {
                Ok(HashMap::default())
            } else {
                Err(anyhow!("Prompt expects argument but none given"))
            }
        }
        Some(_) | None => {
            if arguments.is_empty() {
                Ok(HashMap::default())
            } else {
                Err(anyhow!("Prompt expects no arguments but some were given"))
            }
        }
    }
}

/// MCP servers can return prompts with multiple arguments. Since we only
/// support one argument, we ignore all others. This is the necessary predicate
/// for this.
pub fn acceptable_prompt(prompt: &PromptInfo) -> bool {
    match &prompt.arguments {
        None => true,
        Some(args) if args.len() <= 1 => true,
        _ => false,
    }
}

/// Formats a list of `SamplingMessage`s into a single string.
///
/// This function takes a slice of `SamplingMessage`s and converts them into a formatted string.
/// The formatting depends on the content of the messages:
///
/// - If all messages are from the User, it concatenates their text content, separated by newlines.
/// - Otherwise, it formats each message as "Role: Content", with roles being either "User" or "Assistant".
///
/// # Notes
///
/// - Image content is currently ignored for User-only messages and represented as "[Image]" for mixed messages.
/// - Empty strings are used for image content in User-only messages to maintain consistency in newline separation.
///
/// # TODO
/// We should properly render these messages with the respective User/Assistant labels that the assistant panel
/// supports, but Slash command do not yet support within that assistant panel.
fn format_messages(messages: &[SamplingMessage]) -> String {
    if messages
        .iter()
        .all(|msg| matches!(msg.role, SamplingRole::User))
    {
        messages
            .iter()
            .map(|msg| {
                match &msg.content {
                    SamplingContent::Text(text) => text,
                    SamplingContent::Image { .. } => "", // Ignore images for now
                }
            })
            .collect::<Vec<&str>>()
            .join("\n")
    } else {
        messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    SamplingRole::User => "User",
                    SamplingRole::Assistant => "Assistant",
                };
                let content = match &msg.content {
                    SamplingContent::Text(text) => text,
                    SamplingContent::Image { .. } => "",
                };
                format!("{}: {}", role, content)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
