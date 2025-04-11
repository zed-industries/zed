mod example;

use assistant_settings::AssistantSettings;
use client::{Client, UserStore};
pub(crate) use example::*;

use ::fs::RealFs;
use anyhow::anyhow;
use gpui::{App, AppContext, Application, Entity, SemanticVersion, Task};
use language::LanguageRegistry;
use language_model::{
    AuthenticateError, LanguageModel, LanguageModelProviderId, LanguageModelRegistry,
};
use node_runtime::NodeRuntime;
use project::Project;
use prompt_store::PromptBuilder;
use reqwest_client::ReqwestClient;
use settings::{Settings, SettingsStore};
use std::sync::Arc;

fn main() {
    env_logger::init();
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client.clone());

    app.run(move |cx| {
        let app_state = init(cx);

        let model = find_model("claude-3-7-sonnet-thinking-latest", cx).unwrap();

        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            registry.set_default_model(Some(model.clone()), cx);
        });

        let model_provider_id = model.provider_id();

        let authenticate = authenticate_model_provider(model_provider_id.clone(), cx);

        cx.spawn(async move |cx| {
            authenticate.await.unwrap();

            let example =
                Example::load_from_directory("./crates/eval/examples/find_and_replace_diff_card")?;
            example.setup()?;
            cx.update(|cx| example.run(model, app_state, cx))?.await?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    });
}

/// Subset of `workspace::AppState` needed by `HeadlessAssistant`, with additional fields.
pub struct AgentAppState {
    pub languages: Arc<LanguageRegistry>,
    pub client: Arc<Client>,
    pub user_store: Entity<UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub node_runtime: NodeRuntime,

    // Additional fields not present in `workspace::AppState`.
    pub prompt_builder: Arc<PromptBuilder>,
}

pub fn init(cx: &mut App) -> Arc<AgentAppState> {
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

    AssistantSettings::override_global(
        AssistantSettings {
            always_allow_tool_actions: true,
            ..AssistantSettings::get_global(cx).clone()
        },
        cx,
    );

    Arc::new(AgentAppState {
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
