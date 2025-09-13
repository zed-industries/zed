use anyhow::{Context as _, Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use collections::HashMap;
use context_server::ContextServerId;
use gpui::{App, Entity, Task, WeakEntity, Window};
use language::{BufferSnapshot, LspAdapterDelegate};
use project::context_server_store::ContextServerStore;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use text::LineEnding;
use ui::{IconName, SharedString};
use workspace::Workspace;

use crate::create_label_for_command;

pub struct ContextServerSlashCommand {
    store: Entity<ContextServerStore>,
    server_id: ContextServerId,
    prompt: rmcp::model::Prompt,
}

impl ContextServerSlashCommand {
    pub fn new(
        store: Entity<ContextServerStore>,
        id: ContextServerId,
        prompt: rmcp::model::Prompt,
    ) -> Self {
        Self {
            server_id: id,
            prompt,
            store,
        }
    }
}

impl SlashCommand for ContextServerSlashCommand {
    fn name(&self) -> String {
        self.prompt.name.clone()
    }

    fn label(&self, cx: &App) -> language::CodeLabel {
        let mut parts = vec![self.prompt.name.as_str()];
        if let Some(args) = &self.prompt.arguments
            && let Some(arg) = args.first()
        {
            parts.push(arg.name.as_str());
        }
        create_label_for_command(parts[0], &parts[1..], cx)
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
        self.prompt
            .arguments
            .as_ref()
            .is_some_and(|args| args.iter().any(|arg| arg.required == Some(true)))
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        // For now, return empty completions
        // In the future, this could provide intelligent completions based on the tool/prompt schema
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let server_id = self.server_id.clone();
        let prompt_name = self.prompt.name.clone();

        let prompt_args = match prompt_arguments(&self.prompt, arguments) {
            Ok(args) => args,
            Err(e) => return Task::ready(Err(e)),
        };

        let store = self.store.read(cx);
        if let Some(server) = store.get_running_server(&server_id) {
            cx.foreground_executor().spawn(async move {
                let service = server.service().context("Context server not initialized")?;
                let response = service
                    .get_prompt(rmcp::model::GetPromptRequestParam {
                        name: prompt_name.clone(),
                        arguments: Some(rmcp::object!(prompt_args)),
                    })
                    .await?;

                anyhow::ensure!(
                    response
                        .messages
                        .iter()
                        .all(|msg| matches!(msg.role, rmcp::model::Role::User)),
                    "Prompt contains non-user roles, which is not supported"
                );

                // Extract text from user messages into a single prompt string
                let mut prompt = response
                    .messages
                    .into_iter()
                    .filter_map(|msg| match &msg.content {
                        rmcp::model::PromptMessageContent::Text { text } => Some(text.clone()),
                        rmcp::model::PromptMessageContent::Image { .. } => None,
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
                            response
                                .description
                                .unwrap_or(format!("Result from {}", prompt_name)),
                        ),
                        metadata: None,
                    }],
                    text: prompt,
                    run_commands_in_text: false,
                }
                .into_event_stream())
            })
        } else {
            Task::ready(Err(anyhow!("Context server not found")))
        }
    }
}

fn completion_argument(
    prompt: &rmcp::model::Prompt,
    arguments: &[String],
) -> Result<(String, String)> {
    anyhow::ensure!(!arguments.is_empty(), "No arguments given");

    match &prompt.arguments {
        Some(args) if args.len() == 1 => {
            let arg_name = args[0].name.clone();
            let arg_value = arguments.join(" ");
            Ok((arg_name, arg_value))
        }
        Some(_) => anyhow::bail!("Prompt must have exactly one argument"),
        None => anyhow::bail!("Prompt has no arguments"),
    }
}

fn prompt_arguments(prompt: &Prompt, arguments: &[String]) -> Result<HashMap<String, String>> {
    match &prompt.arguments {
        Some(args) if args.len() > 1 => {
            anyhow::bail!("Prompt has more than one argument, which is not supported");
        }
        Some(args) if args.len() == 1 => {
            if !arguments.is_empty() {
                let mut map = HashMap::default();
                map.insert(args[0].name.clone(), arguments.join(" "));
                Ok(map)
            } else if arguments.is_empty() && args[0].required == Some(false) {
                Ok(HashMap::default())
            } else {
                anyhow::bail!("Prompt expects argument but none given");
            }
        }
        Some(_) | None => {
            anyhow::ensure!(
                arguments.is_empty(),
                "Prompt expects no arguments but some were given"
            );
            Ok(HashMap::default())
        }
    }
}

/// MCP servers can return prompts with multiple arguments. Since we only
/// support one argument, we ignore all others. This is the necessary predicate
/// for this.
pub fn acceptable_prompt(prompt: &rmcp::model::Prompt) -> bool {
    match &prompt.arguments {
        None => true,
        Some(args) if args.len() <= 1 => true,
        _ => false,
    }
}
