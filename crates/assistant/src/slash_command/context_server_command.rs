use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
};
use context_servers::{
    manager::{ContextServer, ContextServerManager},
    protocol::PromptInfo,
};
use gpui::{AppContext, Global, ReadGlobal, Task, WeakView, WindowContext};
use language::LspAdapterDelegate;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::RwLock;
use ui::{IconName, SharedString};
use workspace::Workspace;

struct GlobalContextServerRegistry(Arc<ContextServerRegistry>);

impl Global for GlobalContextServerRegistry {}

pub struct ContextServerRegistry {
    registry: RwLock<HashMap<String, Vec<String>>>,
}

impl ContextServerRegistry {
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalContextServerRegistry::global(cx).0.clone()
    }

    pub fn register(cx: &mut AppContext) {
        cx.set_global(GlobalContextServerRegistry(Arc::new(
            ContextServerRegistry {
                registry: RwLock::new(HashMap::new()),
            },
        )))
    }

    pub fn register_command(&self, server_id: String, command_name: String) {
        let mut registry = self.registry.write().unwrap();
        registry.entry(server_id).or_default().push(command_name);
    }

    pub fn unregister_command(&self, server_id: &str, command_name: &str) {
        let mut registry = self.registry.write().unwrap();
        if let Some(commands) = registry.get_mut(server_id) {
            commands.retain(|name| name != command_name);
        }
    }

    pub fn get_commands(&self, server_id: &str) -> Option<Vec<String>> {
        let registry = self.registry.read().unwrap();
        registry.get(server_id).cloned()
    }
}

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
        let argument = arguments.first().cloned();

        let manager = ContextServerManager::global(cx);
        let manager = manager.read(cx);
        if let Some(server) = manager.get_server(&server_id) {
            cx.foreground_executor().spawn(async move {
                let Some(protocol) = server.client.read().clone() else {
                    return Err(anyhow!("Context server not initialized"));
                };

                let result = protocol
                    .run_prompt(&prompt_name, prompt_arguments(&self.prompt, argument)?)
                    .await?;

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: 0..result.len(),
                        icon: IconName::ZedAssistant,
                        label: SharedString::from(format!("Result from {}", prompt_name)),
                    }],
                    text: result,
                    run_commands_in_text: false,
                })
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
    }
}

fn prompt_arguments(
    prompt: &PromptInfo,
    argument: Option<String>,
) -> Result<HashMap<String, String>> {
    match &prompt.arguments {
        Some(args) if args.len() >= 2 => Err(anyhow!(
            "Prompt has more than one argument, which is not supported"
        )),
        Some(args) if args.len() == 1 => match argument {
            Some(value) => {
                let mut map = HashMap::new();
                map.insert(args[0].name.clone(), value);
                Ok(map)
            }
            None => Err(anyhow!("Prompt expects argument but none given")),
        },
        Some(_) | None => Ok(HashMap::new()),
    }
}

// MCP servers can return prompts with multiple arguments. Since we only
// support one argument, we ignore all others. This is the necessary predicate
// for this.
pub fn acceptable_prompt(prompt: &PromptInfo) -> bool {
    match &prompt.arguments {
        None => true,
        Some(args) if args.len() == 1 => true,
        _ => false,
    }
}
