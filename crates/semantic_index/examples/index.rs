use client::Client;
use futures::channel::oneshot;
use gpui::App;
use http_client::HttpClientWithUrl;
use language::language_settings::AllLanguageSettings;
use project::Project;
use semantic_index::{OpenAiEmbeddingModel, OpenAiEmbeddingProvider, SemanticIndex};
use settings::SettingsStore;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

fn main() {
    env_logger::init();

    use clock::FakeSystemClock;

    App::new().run(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        language::init(cx);
        Project::init_settings(cx);
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });

        let clock = Arc::new(FakeSystemClock::default());
        let http = Arc::new(HttpClientWithUrl::new("http://localhost:11434", None, None));

        let client = client::Client::new(clock, http.clone(), cx);
        Client::set_global(client.clone(), cx);

        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: cargo run --example index -p semantic_index -- <project_path>");
            cx.quit();
            return;
        }

        // let embedding_provider = semantic_index::FakeEmbeddingProvider;

        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        let embedding_provider = Arc::new(OpenAiEmbeddingProvider::new(
            http.clone(),
            OpenAiEmbeddingModel::TextEmbedding3Small,
            open_ai::OPEN_AI_API_URL.to_string(),
            api_key,
        ));

        cx.spawn(|mut cx| async move {
            let semantic_index = SemanticIndex::new(
                PathBuf::from("/tmp/semantic-index-db.mdb"),
                embedding_provider,
                &mut cx,
            );

            let mut semantic_index = semantic_index.await.unwrap();

            let project_path = Path::new(&args[1]);

            let project = Project::example([project_path], &mut cx).await;

            cx.update(|cx| {
                let language_registry = project.read(cx).languages().clone();
                let node_runtime = project.read(cx).node_runtime().unwrap().clone();
                languages::init(language_registry, node_runtime, cx);
            })
            .unwrap();

            let project_index = cx
                .update(|cx| semantic_index.project_index(project.clone(), cx))
                .unwrap();

            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);
            let subscription = cx.update(|cx| {
                cx.subscribe(&project_index, move |_, event, _| {
                    if let Some(tx) = tx.take() {
                        _ = tx.send(*event);
                    }
                })
            });

            let index_start = std::time::Instant::now();
            rx.await.expect("no event emitted");
            drop(subscription);
            println!("Index time: {:?}", index_start.elapsed());

            let results = cx
                .update(|cx| {
                    let project_index = project_index.read(cx);
                    let query = "converting an anchor to a point";
                    project_index.search(query.into(), 4, cx)
                })
                .unwrap()
                .await
                .unwrap();

            for search_result in results {
                let path = search_result.path.clone();

                let content = cx
                    .update(|cx| {
                        let worktree = search_result.worktree.read(cx);
                        let entry_abs_path = worktree.abs_path().join(search_result.path.clone());
                        let fs = project.read(cx).fs().clone();
                        cx.spawn(|_| async move { fs.load(&entry_abs_path).await.unwrap() })
                    })
                    .unwrap()
                    .await;

                let range = search_result.range.clone();
                let content = content[search_result.range].to_owned();

                println!(
                    "✄✄✄✄✄✄✄✄✄✄✄✄✄✄ {:?} @ {} ✄✄✄✄✄✄✄✄✄✄✄✄✄✄",
                    path, search_result.score
                );
                println!("{:?}:{:?}:{:?}", path, range.start, range.end);
                println!("{}", content);
            }

            cx.background_executor()
                .timer(std::time::Duration::from_secs(100000))
                .await;

            cx.update(|cx| cx.quit()).unwrap();
        })
        .detach();
    });
}
