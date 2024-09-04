use anyhow::Result;
use clap::Parser;
use collections::BTreeMap;
use gpui::BackgroundExecutor;
use http_client::Method;
use serde::{Deserialize, Serialize};
use smol::io::AsyncReadExt;
use std::{
    fs,
    ops::Range,
    path::Path,
    process::{exit, Command, Stdio},
    sync::{atomic::AtomicUsize, Arc},
};

const CODESEARCH_NET_DIR: &'static str = "target/datasets/code-search-net";
const EVAL_REPOS_DIR: &'static str = "target/datasets/eval-repos";

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Fetch {},
    Run {
        #[arg(value_name = "LANGUAGE")]
        language: String,
    },
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

    match cli.command {
        Commands::Fetch {} => {
            gpui::App::headless().run(|cx| {
                let executor = cx.background_executor().clone();
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
            });
        }
        Commands::Run { .. } => {
            gpui::App::headless().run(|_cx| {
                // cx.spawn(|mut cx| async move {
                //     // if let Err(err) = fetch_code_search_net_repos(&mut cx).await {
                //     //     eprintln!("Error: {}", err);
                //     //     std::process::exit(1);
                //     // }
                //     exit(0);
                // })
                // .detach();
            });
        }
    }

    Ok(())
}

async fn fetch_evaluation_resources(executor: &BackgroundExecutor) -> Result<()> {
    fetch_code_search_net_resources().await?;
    fetch_eval_repos(executor).await?;
    Ok(())
}

async fn fetch_code_search_net_resources() -> Result<()> {
    eprintln!("Fetching CodeSearchNet evaluations...");

    let annotations_url = "https://raw.githubusercontent.com/github/CodeSearchNet/master/resources/annotationStore.csv";

    let dataset_dir = Path::new(CODESEARCH_NET_DIR);
    fs::create_dir_all(&dataset_dir).expect("failed to create CodeSearchNet directory");

    // Fetch the annotations CSV, which contains the human-annotated search relevances
    let http_client = http_client::HttpClientWithProxy::new(None, None);
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

    eprint!("Checking repositories...");
    let mut evaluations = Vec::new();
    let len = evaluations_by_repo.len();
    for (ix, ((repo, _), evaluation)) in evaluations_by_repo.into_iter().enumerate() {
        eprint!("\rChecking repositories ({ix}/{len})...",);
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
            eprintln!(
                "Repo {repo} is no longer public ({:?}). Skipping",
                repo_response.status()
            );
            continue;
        }
        evaluations.push(evaluation);
    }

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

async fn fetch_eval_repos(executor: &BackgroundExecutor) -> Result<()> {
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
                            done_count.load(std::sync::atomic::Ordering::SeqCst),
                            len,
                        );

                        let repo_dir = repos_dir.join(&repo.replace("/", "__"));
                        if !repo_dir.join(".git").exists() {
                            fs::create_dir_all(&repo_dir).unwrap();

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
                                continue;
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
                            continue;
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
                            continue;
                        }

                        done_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                });
            }
        })
        .await;

    Ok(())
}
