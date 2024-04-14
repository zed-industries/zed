use assets::Assets;
use assistant2::AssistantPanel;
use client::Client;
use gpui::{actions, App, AppContext, KeyBinding, Model, Task, View, WindowOptions};
use language::LanguageRegistry;
use project::{Fs, Project};
use semantic_index::{OpenAiEmbeddingModel, OpenAiEmbeddingProvider, ProjectIndex, SemanticIndex};
use settings::{KeymapFile, DEFAULT_KEYMAP_PATH};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use theme::LoadThemes;
use ui::{div, prelude::*, Render};
use util::http::HttpClientWithUrl;

actions!(example, [Quit]);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    env_logger::init();
    App::new().with_assets(Assets).run(|cx| {
        cx.bind_keys(Some(KeyBinding::new("cmd-q", Quit, None)));
        cx.on_action(|_: &Quit, cx: &mut AppContext| {
            cx.quit();
        });

        if args.len() < 2 {
            eprintln!(
                "Usage: cargo run --example assistant_example -p assistant2 -- <project_path>"
            );
            cx.quit();
            return;
        }

        settings::init(cx);
        language::init(cx);
        Project::init_settings(cx);
        editor::init(cx);
        theme::init(LoadThemes::JustBase, cx);
        Assets.load_fonts(cx).unwrap();
        KeymapFile::load_asset(DEFAULT_KEYMAP_PATH, cx).unwrap();
        client::init_settings(cx);
        release_channel::init("0.130.0", cx);

        let client = Client::production(cx);
        {
            let client = client.clone();
            cx.spawn(|cx| async move { client.authenticate_and_connect(false, &cx).await })
                .detach_and_log_err(cx);
        }
        assistant2::init(client.clone(), cx);

        let language_registry = Arc::new(LanguageRegistry::new(
            Task::ready(()),
            cx.background_executor().clone(),
        ));
        let node_runtime = node_runtime::RealNodeRuntime::new(client.http_client());
        languages::init(language_registry.clone(), node_runtime, cx);

        let http = Arc::new(HttpClientWithUrl::new("http://localhost:11434"));

        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let embedding_provider = OpenAiEmbeddingProvider::new(
            http.clone(),
            OpenAiEmbeddingModel::TextEmbedding3Small,
            open_ai::OPEN_AI_API_URL.to_string(),
            api_key,
        );

        let semantic_index = SemanticIndex::new(
            PathBuf::from("/tmp/semantic-index-db.mdb"),
            Arc::new(embedding_provider),
            cx,
        );

        cx.spawn(|mut cx| async move {
            let project_path = Path::new(&args[1]);
            dbg!(project_path);
            let project = Project::example([project_path], &mut cx).await;
            let mut semantic_index = semantic_index.await?;

            cx.update(|cx| {
                let fs = project.read(cx).fs().clone();

                let project_index = semantic_index.project_index(project.clone(), cx);
                cx.open_window(WindowOptions::default(), |cx| {
                    cx.new_view(|cx| Example::new(language_registry, project_index, fs, cx))
                });
                cx.activate(true);
            })
        })
        .detach_and_log_err(cx);
    })
}

struct Example {
    assistant_panel: View<AssistantPanel>,
}

impl Example {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        project_index: Model<ProjectIndex>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            assistant_panel: cx
                .new_view(|cx| AssistantPanel::new(language_registry, project_index, fs, cx)),
        }
    }
}

impl Render for Example {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl ui::prelude::IntoElement {
        div().size_full().child(self.assistant_panel.clone())
    }
}
