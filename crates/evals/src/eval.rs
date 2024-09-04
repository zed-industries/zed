use ::fs::RealFs;
use anyhow::Result;
use clap::Parser;
use client::{Client, UserStore};
use clock::RealSystemClock;
use collections::BTreeMap;
use git::GitHostingProviderRegistry;
use gpui::{AsyncAppContext, BackgroundExecutor, Context, Task};
use http_client::{HttpClient, Method};
use language::LanguageRegistry;
use node_runtime::FakeNodeRuntime;
use open_ai::OpenAiEmbeddingModel;
use project::Project;
use semantic_index::{OpenAiEmbeddingProvider, SemanticIndex};
use serde::{Deserialize, Serialize};
use smol::io::AsyncReadExt;
use std::{
    fs,
    ops::Range,
    path::Path,
    process::{exit, Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
// use tempfile;

const CODESEARCH_NET_DIR: &'static str = "target/datasets/code-search-net";
const EVAL_REPOS_DIR: &'static str = "target/datasets/eval-repos";
const EVAL_DB_PATH: &'static str = "target/eval_db";

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Fetch {},
    Run {},
}

#[derive(Clone, Deserialize, Serialize)]
struct EvaluationProject {
    repo: String,
    sha: String,
    queries: Vec<EvaluationQuery>,
}

#[derive(Clone, Deserialize, Serialize)]
struct EvaluationQuery {
    query: String,
    results: Vec<EvaluationResult>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
struct EvaluationResult {
    file: String,
    lines: Range<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    gpui::App::headless().run(move |cx| {
        let executor = cx.background_executor().clone();

        match cli.command {
            Commands::Fetch {} => {
                executor
                    .clone()
                    .spawn(async move {
                        if let Err(err) = fetch_evaluation_resources(&executor).await {
                            eprintln!("Error: {}", err);
                            exit(1);
                        }
                        exit(0);
                    })
                    .detach();
            }
            Commands::Run {} => {
                cx.spawn(|mut cx| async move {
                    if let Err(err) = run_evaluation(&executor, &mut cx).await {
                        eprintln!("Error: {}", err);
                        exit(1);
                    }
                    exit(0);
                })
                .detach();
            }
        }
    });

    Ok(())
}

async fn fetch_evaluation_resources(executor: &BackgroundExecutor) -> Result<()> {
    let http_client = http_client::HttpClientWithProxy::new(None, None);
    fetch_code_search_net_resources(&http_client).await?;
    fetch_eval_repos(executor, &http_client).await?;
    Ok(())
}

async fn fetch_code_search_net_resources(http_client: &dyn HttpClient) -> Result<()> {
    eprintln!("Fetching CodeSearchNet evaluations...");

    let annotations_url = "https://raw.githubusercontent.com/github/CodeSearchNet/master/resources/annotationStore.csv";

    let dataset_dir = Path::new(CODESEARCH_NET_DIR);
    fs::create_dir_all(&dataset_dir).expect("failed to create CodeSearchNet directory");

    // Fetch the annotations CSV, which contains the human-annotated search relevances
    let annotations_path = dataset_dir.join("annotations.csv");
    let annotations_csv_content = if annotations_path.exists() {
        fs::read_to_string(&annotations_path).expect("failed to read annotations")
    } else {
        let response = http_client
            .get(annotations_url, Default::default(), true)
            .await
            .expect("failed to fetch annotations csv");
        let mut body = String::new();
        response
            .into_body()
            .read_to_string(&mut body)
            .await
            .expect("failed to read annotations.csv response");
        fs::write(annotations_path, &body).expect("failed to write annotations.csv");
        body
    };

    // Parse the annotations CSV. Skip over queries with zero relevance.
    let rows = annotations_csv_content.lines().filter_map(|line| {
        let mut values = line.split(',');
        let _language = values.next()?;
        let query = values.next()?;
        let github_url = values.next()?;
        let score = values.next()?;

        if score == "0" {
            return None;
        }

        let url_path = github_url.strip_prefix("https://github.com/")?;
        let (url_path, hash) = url_path.split_once('#')?;
        let (repo_name, url_path) = url_path.split_once("/blob/")?;
        let (sha, file_path) = url_path.split_once('/')?;
        let line_range = if let Some((start, end)) = hash.split_once('-') {
            start.strip_prefix("L")?.parse::<usize>().ok()?..end.strip_prefix("L")?.parse().ok()?
        } else {
            let row = hash.strip_prefix("L")?.parse().ok()?;
            row..row + 1
        };
        Some((repo_name, sha, query, file_path, line_range))
    });

    // Group the annotations by repo and sha.
    let mut evaluations_by_repo = BTreeMap::new();
    for (repo_name, sha, query, file_path, lines) in rows {
        let evaluation_project = evaluations_by_repo
            .entry((repo_name, sha))
            .or_insert_with(|| EvaluationProject {
                repo: repo_name.to_string(),
                sha: sha.to_string(),
                queries: Vec::new(),
            });

        let ix = evaluation_project
            .queries
            .iter()
            .position(|entry| entry.query == query)
            .unwrap_or_else(|| {
                evaluation_project.queries.push(EvaluationQuery {
                    query: query.to_string(),
                    results: Vec::new(),
                });
                evaluation_project.queries.len() - 1
            });
        let results = &mut evaluation_project.queries[ix].results;
        let result = EvaluationResult {
            file: file_path.to_string(),
            lines,
        };
        if !results.contains(&result) {
            results.push(result);
        }
    }

    // eprint!("Checking repositories...");
    let evaluations = evaluations_by_repo.into_values().collect::<Vec<_>>();
    // let len = evaluations_by_repo.len();
    // for (ix, ((repo, _), evaluation)) in evaluations_by_repo.into_iter().enumerate() {
    //     // eprint!("\rChecking repositories ({ix}/{len})...",);
    //     evaluations.push(evaluation);
    // }

    let evaluations_path = dataset_dir.join("evaluations.json");
    fs::write(
        &evaluations_path,
        serde_json::to_vec_pretty(&evaluations).unwrap(),
    )
    .unwrap();

    eprintln!(
        "Fetched CodeSearchNet evaluations into {}",
        evaluations_path.display()
    );

    Ok(())
}

async fn run_evaluation(executor: &BackgroundExecutor, cx: &mut AsyncAppContext) -> Result<()> {
    let dataset_dir = Path::new(CODESEARCH_NET_DIR);
    let evaluations_path = dataset_dir.join("evaluations.json");
    let repos_dir = Path::new(EVAL_REPOS_DIR);
    let db_path = Path::new(EVAL_DB_PATH);
    let http_client = http_client::HttpClientWithProxy::new(None, None);
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let git_hosting_provider_registry = Arc::new(GitHostingProviderRegistry::new());
    let fs = Arc::new(RealFs::new(git_hosting_provider_registry, None));
    let clock = Arc::new(RealSystemClock);
    let client = cx
        .update(|cx| {
            Client::new(
                clock,
                Arc::new(http_client::HttpClientWithUrl::new(
                    "https://zed.dev",
                    None,
                    None,
                )),
                cx,
            )
        })
        .unwrap();

    let evaluations = fs::read(&evaluations_path).expect("failed to read evaluations.json");
    let evaluations: Vec<EvaluationProject> = serde_json::from_slice(&evaluations).unwrap();

    let embedding_provider = Arc::new(OpenAiEmbeddingProvider::new(
        http_client.clone(),
        OpenAiEmbeddingModel::TextEmbedding3Small,
        open_ai::OPEN_AI_API_URL.to_string(),
        api_key,
    ));

    let semantic_index = SemanticIndex::new(db_path.into(), embedding_provider, cx).await;
    let language_registry = Arc::new(LanguageRegistry::new(Task::ready(()), executor.clone()));

    for evaluation in evaluations {
        let repo_dir = repos_dir.join(&evaluation.repo);
        if !repo_dir.exists() {
            eprintln!("Skipping {}: directory not found", evaluation.repo);
            continue;
        }

        let project = cx
            .update(|cx| {
                Project::local(
                    client.clone(),
                    Arc::new(FakeNodeRuntime {}),
                    cx.new_model(|cx| UserStore::new(client.clone(), cx)),
                    language_registry.clone(),
                    fs.clone(),
                    None,
                    cx,
                )
            })
            .unwrap();

        let (worktree, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(repo_dir, true, cx)
            })?
            .await?;

        // TODO: Implement the actual evaluation logic here
        // This will involve running queries and comparing results
    }

    Ok(())
}

async fn fetch_eval_repos(
    executor: &BackgroundExecutor,
    http_client: &dyn HttpClient,
) -> Result<()> {
    let dataset_dir = Path::new(CODESEARCH_NET_DIR);
    let evaluations_path = dataset_dir.join("evaluations.json");
    let repos_dir = Path::new(EVAL_REPOS_DIR);

    let evaluations = fs::read(&evaluations_path).expect("failed to read evaluations.json");
    let evaluations: Vec<EvaluationProject> = serde_json::from_slice(&evaluations).unwrap();

    eprint!("Fetching evaluation repositories...");

    executor
        .scoped(move |scope| {
            let done_count = Arc::new(AtomicUsize::new(0));
            let len = evaluations.len();
            for chunk in evaluations.chunks(evaluations.len() / 8) {
                let chunk = chunk.to_vec();
                let done_count = done_count.clone();
                scope.spawn(async move {
                    for EvaluationProject { repo, sha, .. } in chunk {
                        eprint!(
                            "\rFetching evaluation repositories ({}/{})...",
                            done_count.load(SeqCst),
                            len,
                        );

                        fetch_eval_repo(repo, sha, repos_dir, http_client).await;
                        done_count.fetch_add(1, SeqCst);
                    }
                });
            }
        })
        .await;

    Ok(())
}

async fn fetch_eval_repo(
    repo: String,
    sha: String,
    repos_dir: &Path,
    http_client: &dyn HttpClient,
) {
    let Some((owner, repo_name)) = repo.split_once('/') else {
        return;
    };
    let repo_dir = repos_dir.join(owner).join(repo_name);
    fs::create_dir_all(&repo_dir).unwrap();
    let skip_eval_path = repo_dir.join(".skip-eval");
    if skip_eval_path.exists() {
        return;
    }
    if let Ok(head_content) = fs::read_to_string(&repo_dir.join(".git").join("HEAD")) {
        if head_content.trim() == sha {
            return;
        }
    }
    let repo_response = http_client
        .send(
            http_client::Request::builder()
                .method(Method::HEAD)
                .uri(format!("https://github.com/{}", repo))
                .body(Default::default())
                .expect(""),
        )
        .await
        .expect("failed to check github repo");
    if !repo_response.status().is_success() && !repo_response.status().is_redirection() {
        fs::write(&skip_eval_path, "").unwrap();
        eprintln!(
            "Repo {repo} is no longer public ({:?}). Skipping",
            repo_response.status()
        );
        return;
    }
    if !repo_dir.join(".git").exists() {
        let init_output = Command::new("git")
            .current_dir(&repo_dir)
            .args(&["init"])
            .output()
            .unwrap();
        if !init_output.status.success() {
            eprintln!(
                "Failed to initialize git repository for {}: {}",
                repo,
                String::from_utf8_lossy(&init_output.stderr)
            );
            return;
        }
    }
    let url = format!("https://github.com/{}.git", repo);
    Command::new("git")
        .current_dir(&repo_dir)
        .args(&["remote", "add", "-f", "origin", &url])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let fetch_output = Command::new("git")
        .current_dir(&repo_dir)
        .args(&["fetch", "--depth", "1", "origin", &sha])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    if !fetch_output.status.success() {
        eprintln!(
            "Failed to fetch {} for {}: {}",
            sha,
            repo,
            String::from_utf8_lossy(&fetch_output.stderr)
        );
        return;
    }
    let checkout_output = Command::new("git")
        .current_dir(&repo_dir)
        .args(&["checkout", &sha])
        .output()
        .unwrap();

    if !checkout_output.status.success() {
        eprintln!(
            "Failed to checkout {} for {}: {}",
            sha,
            repo,
            String::from_utf8_lossy(&checkout_output.stderr)
        );
    }
}
