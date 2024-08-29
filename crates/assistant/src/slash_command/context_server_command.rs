use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use collections::HashMap;
use context_servers::{
    manager::{ContextServer, ContextServerManager},
    protocol::PromptInfo,
};
use gpui::{Task, WeakView, WindowContext};
use language::LspAdapterDelegate;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
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
        self.prompt
            .arguments
            .as_ref()
            .map_or(false, |args| !args.is_empty())
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
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

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: 0..(result.prompt.len()),
                        icon: IconName::ZedAssistant,
                        label: SharedString::from(
                            result
                                .description
                                .unwrap_or(format!("Result from {}", prompt_name)),
                        ),
                    }],
                    text: result.prompt,
                    run_commands_in_text: false,
                })
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
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
        Some(args) if args.len() == 1 => true,
        _ => false,
    }
}
