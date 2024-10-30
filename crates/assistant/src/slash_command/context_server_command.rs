use anyhow::{anyhow, Result};
use assistant_slash_command::{
    AfterCompletion, ArgumentCompletion, SlashCommand, SlashCommandOutput,
    SlashCommandOutputSection, SlashCommandResult,
};
use collections::HashMap;
use context_servers::{
    manager::{ContextServer, ContextServerManager},
    types::Prompt,
};
use gpui::{AppContext, Task, WeakView, WindowContext};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use text::LineEnding;
use ui::{IconName, SharedString};
use workspace::Workspace;

use crate::slash_command::create_label_for_command;

pub struct ContextServerSlashCommand {
    server_id: String,
    prompt: Prompt,
}

impl ContextServerSlashCommand {
    pub fn new(server: &Arc<ContextServer>, prompt: Prompt) -> Self {
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

    fn label(&self, cx: &AppContext) -> language::CodeLabel {
        let mut parts = vec![self.prompt.name.as_str()];
        if let Some(args) = &self.prompt.arguments {
            if let Some(arg) = args.first() {
                parts.push(arg.name.as_str());
            }
        }
        create_label_for_command(&parts[0], &parts[1..], cx)
    }

    fn description(&self) -> String {
        match &self.prompt.description {
            Some(desc) => desc.clone(),
            None => format!("Run '{}' from {}", self.prompt.name, self.server_id),
        }
    }

    fn menu_text(&self) -> String {
        match &self.prompt.description {
            Some(desc) => desc.clone(),
            None => format!("Run '{}' from {}", self.prompt.name, self.server_id),
        }
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
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
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

                // Check that there are only user roles
                if result
                    .messages
                    .iter()
                    .any(|msg| !matches!(msg.role, context_servers::types::SamplingRole::User))
                {
                    return Err(anyhow!(
                        "Prompt contains non-user roles, which is not supported"
                    ));
                }

                // Extract text from user messages into a single prompt string
                let mut prompt = result
                    .messages
                    .into_iter()
                    .filter_map(|msg| match msg.content {
                        context_servers::types::SamplingContent::Text { text } => Some(text),
                        _ => None,
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n");

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
                        metadata: None,
                    }],
                    text: prompt,
                    run_commands_in_text: false,
                }
                .to_event_stream())
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
    }
}

fn completion_argument(prompt: &Prompt, arguments: &[String]) -> Result<(String, String)> {
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

fn prompt_arguments(prompt: &Prompt, arguments: &[String]) -> Result<HashMap<String, String>> {
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
pub fn acceptable_prompt(prompt: &Prompt) -> bool {
    match &prompt.arguments {
        None => true,
        Some(args) if args.len() <= 1 => true,
        _ => false,
    }
}
