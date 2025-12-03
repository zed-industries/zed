use anyhow::Result;
use collections::{BTreeMap, HashMap};
use context_server::ContextServerId;
use gpui::{App, AppContext, Context, Entity, EventEmitter, SharedString, Task};
use project::context_server_store::{ContextServerStatus, ContextServerStore};
use std::collections::HashMap as StdHashMap;
use util::ResultExt;

pub struct ContextServerPrompt {
    pub server_id: ContextServerId,
    pub prompt: context_server::types::Prompt,
}

pub struct ContextServerPromptRegistry {
    server_store: Entity<ContextServerStore>,
    registered_servers: HashMap<ContextServerId, RegisteredContextServerPrompts>,
    _subscription: gpui::Subscription,
}

struct RegisteredContextServerPrompts {
    prompts: BTreeMap<SharedString, ContextServerPrompt>,
    load_prompts: Task<Result<()>>,
}

pub enum ContextServerPromptRegistryEvent {
    PromptsChanged,
}

impl EventEmitter<ContextServerPromptRegistryEvent> for ContextServerPromptRegistry {}

impl ContextServerPromptRegistry {
    pub fn new(server_store: Entity<ContextServerStore>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            server_store: server_store.clone(),
            registered_servers: HashMap::default(),
            _subscription: cx.subscribe(&server_store, Self::handle_context_server_store_event),
        };
        for server in server_store.read(cx).running_servers() {
            this.reload_prompts_for_server(server.id(), cx);
        }
        this
    }

    pub fn prompts(&self) -> impl Iterator<Item = &ContextServerPrompt> {
        self.registered_servers
            .values()
            .flat_map(|server| server.prompts.values())
    }

    pub fn prompts_for_server(
        &self,
        server_id: &ContextServerId,
    ) -> impl Iterator<Item = &ContextServerPrompt> {
        self.registered_servers
            .get(server_id)
            .map(|server| server.prompts.values())
            .into_iter()
            .flatten()
    }

    pub fn get_prompt(
        &self,
        server_id: &ContextServerId,
        prompt_name: &str,
    ) -> Option<&ContextServerPrompt> {
        self.registered_servers
            .get(server_id)?
            .prompts
            .get(prompt_name)
    }

    pub fn server_store(&self) -> &Entity<ContextServerStore> {
        &self.server_store
    }

    fn reload_prompts_for_server(&mut self, server_id: ContextServerId, cx: &mut Context<Self>) {
        let Some(server) = self.server_store.read(cx).get_running_server(&server_id) else {
            return;
        };
        let Some(client) = server.client() else {
            return;
        };
        if !client.capable(context_server::protocol::ServerCapability::Prompts) {
            return;
        }

        let registered_server = self.registered_servers.entry(server_id.clone()).or_insert(
            RegisteredContextServerPrompts {
                prompts: BTreeMap::default(),
                load_prompts: Task::ready(Ok(())),
            },
        );
        registered_server.load_prompts = cx.spawn(async move |this, cx| {
            let response = client
                .request::<context_server::types::requests::PromptsList>(())
                .await;

            this.update(cx, |this, cx| {
                let Some(registered_server) = this.registered_servers.get_mut(&server_id) else {
                    return;
                };

                registered_server.prompts.clear();
                if let Some(response) = response.log_err() {
                    for prompt in response.prompts {
                        if acceptable_prompt(&prompt) {
                            log::info!(
                                "Registering MCP prompt '{}' from server '{}'",
                                prompt.name,
                                server_id
                            );
                            let name: SharedString = prompt.name.clone().into();
                            registered_server.prompts.insert(
                                name,
                                ContextServerPrompt {
                                    server_id: server_id.clone(),
                                    prompt,
                                },
                            );
                        }
                    }
                    cx.emit(ContextServerPromptRegistryEvent::PromptsChanged);
                    cx.notify();
                }
            })
        });
    }

    fn handle_context_server_store_event(
        &mut self,
        _: Entity<ContextServerStore>,
        event: &project::context_server_store::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::context_server_store::Event::ServerStatusChanged { server_id, status } => {
                match status {
                    ContextServerStatus::Starting => {}
                    ContextServerStatus::Running => {
                        self.reload_prompts_for_server(server_id.clone(), cx);
                    }
                    ContextServerStatus::Stopped | ContextServerStatus::Error(_) => {
                        self.registered_servers.remove(server_id);
                        cx.emit(ContextServerPromptRegistryEvent::PromptsChanged);
                        cx.notify();
                    }
                }
            }
        }
    }
}

/// MCP servers can return prompts with multiple arguments. Since we only
/// support one argument, we ignore all others.
fn acceptable_prompt(prompt: &context_server::types::Prompt) -> bool {
    match &prompt.arguments {
        None => true,
        Some(args) if args.len() <= 1 => true,
        _ => false,
    }
}

/// Execute an MCP prompt and return the result as text.
/// This function spawns an async task to execute the prompt.
pub fn execute_prompt(
    server_store: &Entity<ContextServerStore>,
    server_id: &ContextServerId,
    prompt_name: &str,
    arguments: Option<StdHashMap<String, String>>,
    cx: &App,
) -> Task<Result<String>> {
    let Some(server) = server_store.read(cx).get_running_server(server_id) else {
        return Task::ready(Err(anyhow::anyhow!("Context server not found")));
    };

    let Some(protocol) = server.client() else {
        return Task::ready(Err(anyhow::anyhow!("Context server not initialized")));
    };

    let prompt_name = prompt_name.to_string();
    let arguments = arguments.map(|args| args.into_iter().collect::<HashMap<_, _>>());
    cx.background_spawn(async move {
        let response = protocol
            .request::<context_server::types::requests::PromptsGet>(
                context_server::types::PromptsGetParams {
                    name: prompt_name,
                    arguments,
                    meta: None,
                },
            )
            .await?;

        anyhow::ensure!(
            response
                .messages
                .iter()
                .all(|msg| matches!(msg.role, context_server::types::Role::User)),
            "Prompt contains non-user roles, which is not supported"
        );

        let mut prompt = response
            .messages
            .into_iter()
            .filter_map(|msg| match msg.content {
                context_server::types::MessageContent::Text { text, .. } => Some(text),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        text::LineEnding::normalize(&mut prompt);

        Ok(prompt)
    })
}
