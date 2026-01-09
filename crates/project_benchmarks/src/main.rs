use std::sync::Arc;

use clap::Parser;
use client::{Client, UserStore};
use gpui::{AppContext as _, Application};
use http_client::FakeHttpClient;
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use project::{
    Project, RealFs,
    search::{SearchQuery, SearchResult},
};

#[derive(Parser)]
struct Args {
    /// List of worktrees to run the search against.
    worktrees: Vec<String>,
    #[clap(short)]
    query: String,
    /// Treat query as a regex.
    #[clap(short, long)]
    regex: bool,
    /// Matches have to be standalone words.
    #[clap(long)]
    whole_word: bool,
    /// Make matching case-sensitive.
    #[clap(long, default_value_t = false)]
    case_sensitive: bool,
    /// Include gitignored files in the search.
    #[clap(long)]
    include_ignored: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let query = if args.regex {
        SearchQuery::regex(
            args.query,
            args.whole_word,
            args.case_sensitive,
            args.include_ignored,
            false,
            Default::default(),
            Default::default(),
            false,
            None,
        )
    } else {
        SearchQuery::text(
            args.query,
            args.whole_word,
            args.case_sensitive,
            args.include_ignored,
            Default::default(),
            Default::default(),
            false,
            None,
        )
    }?;
    Application::headless().run(|cx| {
        settings::init(cx);
        let client = Client::production(cx);
        let http_client = FakeHttpClient::with_200_response();
        let (_, rx) = watch::channel(None);
        let node = NodeRuntime::new(http_client, None, rx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let registry = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
        let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
        let project = Project::local(
            client,
            node,
            user_store,
            registry,
            fs,
            Some(Default::default()),
            false,
            cx,
        );

        project.clone().update(cx, move |_, cx| {
            cx.spawn(async move |_, cx| {
                println!("Loading worktrees");
                let worktrees = project.update(cx, |this, cx| {
                    args.worktrees
                        .into_iter()
                        .map(|worktree| this.find_or_create_worktree(worktree, true, cx))
                        .collect::<Vec<_>>()
                });

                let worktrees = futures::future::join_all(worktrees)
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, anyhow::Error>>()?;

                for (worktree, _) in &worktrees {
                    worktree
                        .update(cx, |this, _| this.as_local().unwrap().scan_complete())
                        .await;
                }
                println!("Worktrees loaded");

                println!("Starting a project search");
                let timer = std::time::Instant::now();
                let mut first_match = None;
                let matches = project.update(cx, |this, cx| this.search(query, cx));
                let mut matched_files = 0;
                let mut matched_chunks = 0;
                while let Ok(match_result) = matches.rx.recv().await {
                    if first_match.is_none() {
                        let time = timer.elapsed();
                        first_match = Some(time);
                        println!("First match found after {time:?}");
                    }
                    if let SearchResult::Buffer { ranges, .. } = match_result {
                        matched_files += 1;
                        matched_chunks += ranges.len();
                    } else {
                        break;
                    }
                }
                let elapsed = timer.elapsed();
                println!(
                    "Finished project search after {elapsed:?}. Matched {matched_files} files and {matched_chunks} excerpts"
                );
                drop(project);
                cx.update(|cx| cx.quit());

                anyhow::Ok(())
            })
            .detach();
        });
    });
    Ok(())
}
