use client::{Client, ProxySettings, UserStore};
use collections::HashMap;
use extension::ExtensionHostProxy;
use fs::RealFs;
use gpui::http_client::read_proxy_from_env;
use gpui::{App, AppContext, Entity};
use gpui_tokio::Tokio;
use language::LanguageRegistry;
use language_extension::LspAccess;
use node_runtime::{NodeBinaryOptions, NodeRuntime};
use project::{Project, project_settings::ProjectSettings};
use release_channel::{AppCommitSha, AppVersion};
use reqwest_client::ReqwestClient;
use settings::{Settings, SettingsStore};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use util::ResultExt as _;

/// Headless subset of `workspace::AppState`.
pub struct EpAppState {
    pub languages: Arc<LanguageRegistry>,
    pub client: Arc<Client>,
    pub user_store: Entity<UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub node_runtime: NodeRuntime,
    pub project_cache: ProjectCache,
}

#[derive(Default)]
pub struct ProjectCache(Mutex<HashMap<String, Entity<Project>>>);

impl ProjectCache {
    pub fn insert(&self, repository_url: String, project: Entity<Project>) {
        self.0.lock().unwrap().insert(repository_url, project);
    }

    pub fn get(&self, repository_url: &String) -> Option<Entity<Project>> {
        self.0.lock().unwrap().get(repository_url).cloned()
    }

    pub fn remove(&self, repository_url: &String) {
        self.0.lock().unwrap().remove(repository_url);
    }
}

pub fn init(cx: &mut App) -> EpAppState {
    let app_commit_sha = option_env!("ZED_COMMIT_SHA").map(|s| AppCommitSha::new(s.to_owned()));

    let app_version = AppVersion::load(
        env!("ZED_PKG_VERSION"),
        option_env!("ZED_BUILD_ID"),
        app_commit_sha,
    );
    release_channel::init(app_version.clone(), cx);
    gpui_tokio::init(cx);

    let settings_store = SettingsStore::new(cx, &settings::default_settings());
    cx.set_global(settings_store);

    // Set User-Agent so we can download language servers from GitHub
    let user_agent = format!(
        "Zeta CLI/{} ({}; {})",
        app_version,
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let proxy_str = ProxySettings::get_global(cx).proxy.to_owned();
    let proxy_url = proxy_str
        .as_ref()
        .and_then(|input| input.parse().ok())
        .or_else(read_proxy_from_env);
    let http = {
        let _guard = Tokio::handle(cx).enter();

        ReqwestClient::proxy_and_user_agent(proxy_url, &user_agent)
            .expect("could not start HTTP client")
    };
    cx.set_http_client(Arc::new(http));

    let client = Client::production(cx);
    cx.set_http_client(client.http_client());

    let git_binary_path = None;
    let fs = Arc::new(RealFs::new(
        git_binary_path,
        cx.background_executor().clone(),
    ));

    let mut languages = LanguageRegistry::new(cx.background_executor().clone());
    languages.set_language_server_download_dir(paths::languages_dir().clone());
    let languages = Arc::new(languages);

    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));

    extension::init(cx);

    let (mut tx, rx) = watch::channel(None);
    cx.observe_global::<SettingsStore>(move |cx| {
        let settings = &ProjectSettings::get_global(cx).node;
        let options = NodeBinaryOptions {
            allow_path_lookup: !settings.ignore_system_version,
            allow_binary_download: true,
            use_paths: settings.path.as_ref().map(|node_path| {
                let node_path = PathBuf::from(shellexpand::tilde(node_path).as_ref());
                let npm_path = settings
                    .npm_path
                    .as_ref()
                    .map(|path| PathBuf::from(shellexpand::tilde(&path).as_ref()));
                (
                    node_path.clone(),
                    npm_path.unwrap_or_else(|| {
                        let base_path = PathBuf::new();
                        node_path.parent().unwrap_or(&base_path).join("npm")
                    }),
                )
            }),
        };
        tx.send(Some(options)).log_err();
    })
    .detach();
    let node_runtime = NodeRuntime::new(client.http_client(), None, rx);

    let extension_host_proxy = ExtensionHostProxy::global(cx);

    debug_adapter_extension::init(extension_host_proxy.clone(), cx);
    language_extension::init(LspAccess::Noop, extension_host_proxy, languages.clone());
    language_model::init(client.clone(), cx);
    language_models::init(user_store.clone(), client.clone(), cx);
    languages::init(languages.clone(), fs.clone(), node_runtime.clone(), cx);
    prompt_store::init(cx);
    terminal_view::init(cx);

    let project_cache = ProjectCache::default();

    EpAppState {
        languages,
        client,
        user_store,
        fs,
        node_runtime,
        project_cache,
    }
}
