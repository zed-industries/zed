use client::Client;
use futures::channel::oneshot;
use futures::task::waker;
use gpui::{App, Global, TestAppContext};
use language::language_settings::AllLanguageSettings;
use project::Project;
use semantic_index::embedding::{EmbeddingModel, FakeEmbeddingProvider, OpenaiEmbeddingProvider};
use semantic_index::SemanticIndex;
use settings::SettingsStore;
use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::{path::Path, sync::Arc};
use tempfile::tempdir;
use util::http::HttpClientWithUrl;

pub fn init_test(cx: &mut TestAppContext) {
    _ = cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        language::init(cx);
        Project::init_settings(cx);
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });
    });
}

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
        let http = Arc::new(HttpClientWithUrl::new("http://localhost:11434"));

        let client = client::Client::new(clock, http.clone(), cx);
        Client::set_global(client.clone(), cx);

        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: cargo run --example index -p semantic_index -- <project_path>");
            cx.quit();
            return;
        }

        let embedding_provider = FakeEmbeddingProvider::new();

        // let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        // let embedding_provider = OpenaiEmbeddingProvider::new(
        //     http.clone(),
        //     EmbeddingModel::OpenaiTextEmbedding3Small,
        //     api_key,
        // );

        let semantic_index = SemanticIndex::new(
            Path::new("/tmp/semantic-index-db.mdb"),
            Arc::new(embedding_provider),
            cx,
        );

        cx.spawn(|mut cx| async move {
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
                        _ = tx.send(event.clone());
                    }
                })
            });

            let index_start = std::time::Instant::now();
            rx.await.expect("no event emitted");
            drop(subscription);
            dbg!(index_start.elapsed());

            let results = cx
                .update(|cx| {
                    let project_index = project_index.read(cx);
                    let query = "converting an anchor to a point";
                    project_index.search(query, 10, cx)
                })
                .unwrap()
                .await;

            for search_result in results {
                let path = search_result.path.clone();
                let range = search_result.range.clone();

                let content = cx
                    .update(|cx| {
                        println!("{:?} = {:?}", path, range);
                        let worktree = search_result.worktree.read(cx);
                        let entry_abs_path = worktree.abs_path().join(search_result.path.clone());
                        let fs = project.read(cx).fs().clone();
                        cx.spawn(|_| async move { fs.load(&entry_abs_path).await.unwrap() })
                    })
                    .unwrap()
                    .await;

                // Now only show the range from content, on line breaks
                // Range is in buffer terms (byte offsets)
                let range = search_result.range.clone();
                let start = range.start;
                let end = range.end;
                let content = content[start..end].to_owned();

                println!("✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄✄");
                println!("{:?}:{:?}:{:?}", path, range.start, range.end);
                println!("{}", content);
            }

            cx.update(|cx| cx.quit()).unwrap();
        })
        .detach();
    });
}
