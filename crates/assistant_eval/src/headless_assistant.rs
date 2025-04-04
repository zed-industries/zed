use agent::{RequestKind, Thread, ThreadEvent, ThreadStore};
use anyhow::anyhow;
use assistant_tool::ToolWorkingSet;
use client::{Client, UserStore};
use collections::HashMap;
use dap::DapRegistry;
use futures::StreamExt;
use gpui::{App, AsyncApp, Entity, SemanticVersion, Subscription, Task, prelude::*};
use language::LanguageRegistry;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelProviderId, LanguageModelRegistry,
    LanguageModelRequest,
};
use node_runtime::NodeRuntime;
use project::{Project, RealFs};
use prompt_store::PromptBuilder;
use settings::SettingsStore;
use smol::channel;
use std::sync::Arc;

/// Subset of `workspace::AppState` needed by `HeadlessAssistant`, with additional fields.
pub struct HeadlessAppState {
    pub languages: Arc<LanguageRegistry>,
    pub client: Arc<Client>,
    pub user_store: Entity<UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub node_runtime: NodeRuntime,

    // Additional fields not present in `workspace::AppState`.
    pub prompt_builder: Arc<PromptBuilder>,
}

pub struct HeadlessAssistant {
    pub thread: Entity<Thread>,
    pub project: Entity<Project>,
    #[allow(dead_code)]
    pub thread_store: Entity<ThreadStore>,
    pub tool_use_counts: HashMap<Arc<str>, u32>,
    pub done_tx: channel::Sender<anyhow::Result<()>>,
    _subscription: Subscription,
}

impl HeadlessAssistant {
    pub fn new(
        app_state: Arc<HeadlessAppState>,
        cx: &mut App,
    ) -> anyhow::Result<(Entity<Self>, channel::Receiver<anyhow::Result<()>>)> {
        let env = None;
        let project = Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            Arc::new(DapRegistry::default()),
            app_state.fs.clone(),
            env,
            cx,
        );

        let tools = Arc::new(ToolWorkingSet::default());
        let thread_store =
            ThreadStore::new(project.clone(), tools, app_state.prompt_builder.clone(), cx)?;

        let thread = thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx));

        let (done_tx, done_rx) = channel::unbounded::<anyhow::Result<()>>();

        let headless_thread = cx.new(move |cx| Self {
            _subscription: cx.subscribe(&thread, Self::handle_thread_event),
            thread,
            project,
            thread_store,
            tool_use_counts: HashMap::default(),
            done_tx,
        });

        Ok((headless_thread, done_rx))
    }

    fn handle_thread_event(
        &mut self,
        thread: Entity<Thread>,
        event: &ThreadEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            ThreadEvent::ShowError(err) => self
                .done_tx
                .send_blocking(Err(anyhow!("{:?}", err)))
                .unwrap(),
            ThreadEvent::DoneStreaming => {
                let thread = thread.read(cx);
                if let Some(message) = thread.messages().last() {
                    println!("Message: {}", message.to_string());
                }
                if thread.all_tools_finished() {
                    self.done_tx.send_blocking(Ok(())).unwrap()
                }
            }
            ThreadEvent::UsePendingTools => {
                thread.update(cx, |thread, cx| {
                    thread.use_pending_tools(cx);
                });
            }
            ThreadEvent::ToolConfirmationNeeded => {
                // Automatically approve all tools that need confirmation in headless mode
                println!("Tool confirmation needed - automatically approving in headless mode");

                // Get the tools needing confirmation
                let tools_needing_confirmation: Vec<_> = thread
                    .read(cx)
                    .tools_needing_confirmation()
                    .cloned()
                    .collect();

                // Run each tool that needs confirmation
                for tool_use in tools_needing_confirmation {
                    if let Some(tool) = thread.read(cx).tools().tool(&tool_use.name, cx) {
                        thread.update(cx, |thread, cx| {
                            println!("Auto-approving tool: {}", tool_use.name);

                            // Create a request to send to the tool
                            let request = thread.to_completion_request(RequestKind::Chat, cx);
                            let messages = Arc::new(request.messages);

                            // Run the tool
                            thread.run_tool(
                                tool_use.id.clone(),
                                tool_use.ui_text.clone(),
                                tool_use.input.clone(),
                                &messages,
                                tool,
                                cx,
                            );
                        });
                    }
                }
            }
            ThreadEvent::ToolFinished {
                tool_use_id,
                pending_tool_use,
                ..
            } => {
                if let Some(pending_tool_use) = pending_tool_use {
                    println!(
                        "Used tool {} with input: {}",
                        pending_tool_use.name, pending_tool_use.input
                    );
                    *self
                        .tool_use_counts
                        .entry(pending_tool_use.name.clone())
                        .or_insert(0) += 1;
                }
                if let Some(tool_result) = thread.read(cx).tool_result(tool_use_id) {
                    println!("Tool result: {:?}", tool_result);
                }
                if thread.read(cx).all_tools_finished() {
                    let model_registry = LanguageModelRegistry::read_global(cx);
                    if let Some(model) = model_registry.default_model() {
                        thread.update(cx, |thread, cx| {
                            thread.attach_tool_results(cx);
                            thread.send_to_model(model.model, RequestKind::Chat, cx);
                        });
                    } else {
                        println!(
                            "Warning: No active language model available to continue conversation"
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn init(cx: &mut App) -> Arc<HeadlessAppState> {
    release_channel::init(SemanticVersion::default(), cx);
    gpui_tokio::init(cx);

    let mut settings_store = SettingsStore::new(cx);
    settings_store
        .set_default_settings(settings::default_settings().as_ref(), cx)
        .unwrap();
    cx.set_global(settings_store);
    client::init_settings(cx);
    Project::init_settings(cx);

    let client = Client::production(cx);
    cx.set_http_client(client.http_client().clone());

    let git_binary_path = None;
    let fs = Arc::new(RealFs::new(
        git_binary_path,
        cx.background_executor().clone(),
    ));

    let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));

    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));

    language::init(cx);
    language_model::init(client.clone(), cx);
    language_models::init(user_store.clone(), client.clone(), fs.clone(), cx);
    assistant_tools::init(client.http_client().clone(), cx);
    context_server::init(cx);
    let stdout_is_a_pty = false;
    let prompt_builder = PromptBuilder::load(fs.clone(), stdout_is_a_pty, cx);
    agent::init(fs.clone(), client.clone(), prompt_builder.clone(), cx);

    Arc::new(HeadlessAppState {
        languages,
        client,
        user_store,
        fs,
        node_runtime: NodeRuntime::unavailable(),
        prompt_builder,
    })
}

pub fn find_model(model_name: &str, cx: &App) -> anyhow::Result<Arc<dyn LanguageModel>> {
    let model_registry = LanguageModelRegistry::read_global(cx);
    let model = model_registry
        .available_models(cx)
        .find(|model| model.id().0 == model_name);

    let Some(model) = model else {
        return Err(anyhow!(
            "No language model named {} was available. Available models: {}",
            model_name,
            model_registry
                .available_models(cx)
                .map(|model| model.id().0.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    };

    Ok(model)
}

pub fn authenticate_model_provider(
    provider_id: LanguageModelProviderId,
    cx: &mut App,
) -> Task<std::result::Result<(), AuthenticateError>> {
    let model_registry = LanguageModelRegistry::read_global(cx);
    let model_provider = model_registry.provider(&provider_id).unwrap();
    model_provider.authenticate(cx)
}

pub async fn send_language_model_request(
    model: Arc<dyn LanguageModel>,
    request: LanguageModelRequest,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    match model.stream_completion_text(request, &cx).await {
        Ok(mut stream) => {
            let mut full_response = String::new();

            // Process the response stream
            while let Some(chunk_result) = stream.stream.next().await {
                match chunk_result {
                    Ok(chunk_str) => {
                        full_response.push_str(&chunk_str);
                    }
                    Err(err) => {
                        return Err(anyhow!(
                            "Error receiving response from language model: {err}"
                        ));
                    }
                }
            }

            Ok(full_response)
        }
        Err(err) => Err(anyhow!(
            "Failed to get response from language model. Error was: {err}"
        )),
    }
}
