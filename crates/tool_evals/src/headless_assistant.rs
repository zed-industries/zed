use anyhow::anyhow;
use assistant2::{Message, RequestKind, Thread, ThreadEvent, ThreadStore};
use assistant_tool::ToolWorkingSet;
use client::Client;
use git::GitHostingProviderRegistry;
use gpui::{prelude::*, App, Entity, Subscription, Task};
use language::LanguageRegistry;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelProviderId, LanguageModelRegistry,
};
use project::{Project, RealFs};
use prompt_store::PromptBuilder;
use settings::SettingsStore;
use smol::channel;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use workspace::AppState;

pub struct HeadlessAssistant {
    pub project: Entity<Project>,
    pub thread: Entity<Thread>,
    pub done_tx: channel::Sender<anyhow::Result<()>>,
    _subscription: Subscription,
}

impl HeadlessAssistant {
    pub fn new(
        app_state: Arc<AppState>,
        cx: &mut App,
    ) -> anyhow::Result<(Entity<Self>, channel::Receiver<anyhow::Result<()>>)> {
        let env = None;
        let project = Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            env,
            cx,
        );

        let tools = Arc::new(ToolWorkingSet::default());
        let thread_store = ThreadStore::new(project.clone(), tools, cx)?;

        let thread = thread_store.update(cx, |thread_store, cx| thread_store.create_thread(cx));

        let (done_tx, done_rx) = channel::unbounded::<anyhow::Result<()>>();

        let headless_thread = cx.new(move |cx| Self {
            _subscription: cx.subscribe(&thread, Self::handle_thread_event),
            thread,
            project,
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
                if thread.read(cx).all_tools_finished() {
                    self.done_tx.send_blocking(Ok(())).unwrap()
                }
            }
            ThreadEvent::UsePendingTools => {
                thread.update(cx, |thread, cx| {
                    thread.use_pending_tools(cx);
                });
            }
            ThreadEvent::ToolFinished { .. } => {
                if thread.read(cx).all_tools_finished() {
                    let model_registry = LanguageModelRegistry::read_global(cx);
                    if let Some(model) = model_registry.active_model() {
                        thread.update(cx, |thread, cx| {
                            thread.send_tool_results_to_model(model, cx);
                        });
                    }
                }
            }
            ThreadEvent::ScriptFinished { .. } => {
                let model_registry = LanguageModelRegistry::read_global(cx);
                if let Some(model) = model_registry.active_model() {
                    thread.update(cx, |thread, cx| {
                        // TODO: this was copied from active_thread.rs - why is use_tools false?
                        let use_tools = false;
                        thread.send_to_model(model, RequestKind::Chat, use_tools, cx);
                    });
                }
            }
            ThreadEvent::StreamedCompletion
            | ThreadEvent::SummaryChanged
            | ThreadEvent::StreamedAssistantText(_, _)
            | ThreadEvent::MessageAdded(_)
            | ThreadEvent::MessageEdited(_)
            | ThreadEvent::MessageDeleted(_) => {}
        }
    }
}

pub fn init(cx: &mut App) -> Arc<AppState> {
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

    let git_hosting_provider_registry = GitHostingProviderRegistry::default_global(cx);
    let git_binary_path = None;
    let fs = Arc::new(RealFs::new(git_hosting_provider_registry, git_binary_path));

    let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));

    let test_app_state = AppState::test(cx);
    let app_state = Arc::new(AppState {
        client,
        fs: fs.clone(),
        languages,
        // Clone fields from test_app_state
        user_store: test_app_state.user_store.clone(),
        workspace_store: test_app_state.workspace_store.clone(),
        build_window_options: test_app_state.build_window_options,
        node_runtime: test_app_state.node_runtime.clone(),
        session: test_app_state.session.clone(),
    });

    language_model::init(app_state.client.clone(), cx);
    language_models::init(
        app_state.user_store.clone(),
        app_state.client.clone(),
        app_state.fs.clone(),
        cx,
    );
    assistant_tools::init(cx);
    scripting_tool::init(cx);
    context_server::init(cx);
    let stdout_is_a_pty = false;
    let prompt_builder = PromptBuilder::load(app_state.fs.clone(), stdout_is_a_pty, cx);
    assistant2::init(
        app_state.fs.clone(),
        app_state.client.clone(),
        prompt_builder.clone(),
        cx,
    );

    app_state
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
