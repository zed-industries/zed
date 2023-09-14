use anyhow::{anyhow, Result};
use client::{self, UserStore};
use git2::{Object, Oid, Repository};
use gpui::{AppContext, AssetSource, ModelHandle, Task};
use language::LanguageRegistry;
use node_runtime::RealNodeRuntime;
use project::{Fs, Project, RealFs};
use rust_embed::RustEmbed;
use semantic_index::embedding::OpenAIEmbeddings;
use semantic_index::semantic_index_settings::SemanticIndexSettings;
use semantic_index::{SearchResult, SemanticIndex};
use serde::Deserialize;
use settings::{default_settings, handle_settings_file_changes, watch_config_file, SettingsStore};
use std::path::{self, Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use std::{cmp, env, fs};
use util::channel::{RELEASE_CHANNEL, RELEASE_CHANNEL_NAME};
use util::http::{self, HttpClient};
use util::paths::{self, EMBEDDINGS_DIR};
use zed::languages;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[include = "themes/**/*"]
#[include = "sounds/**/*"]
#[include = "*.md"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Vec<std::borrow::Cow<'static, str>> {
        Self::iter().filter(|p| p.starts_with(path)).collect()
    }
}

#[derive(Deserialize, Clone)]
struct EvaluationQuery {
    query: String,
    matches: Vec<String>,
}

impl EvaluationQuery {
    fn match_pairs(&self) -> Vec<(PathBuf, usize)> {
        let mut pairs = Vec::new();
        for match_identifier in self.matches.iter() {
            let mut match_parts = match_identifier.split(":");

            if let Some(file_path) = match_parts.next() {
                if let Some(row_number) = match_parts.next() {
                    pairs.push((
                        PathBuf::from(file_path),
                        row_number.parse::<usize>().unwrap(),
                    ));
                }
            }
        }
        pairs
    }
}

#[derive(Deserialize, Clone)]
struct RepoEval {
    repo: String,
    commit: String,
    assertions: Vec<EvaluationQuery>,
}

const TMP_REPO_PATH: &str = "eval_repos";

fn parse_eval() -> anyhow::Result<Vec<RepoEval>> {
    let eval_folder = env::current_dir()?
        .as_path()
        .parent()
        .unwrap()
        .join("crates/semantic_index/eval");

    let mut repo_evals: Vec<RepoEval> = Vec::new();
    for entry in fs::read_dir(eval_folder)? {
        let file_path = entry.unwrap().path();
        if let Some(extension) = file_path.extension() {
            if extension == "json" {
                if let Ok(file) = fs::read_to_string(file_path) {
                    let repo_eval = serde_json::from_str(file.as_str());

                    match repo_eval {
                        Ok(repo_eval) => {
                            repo_evals.push(repo_eval);
                        }
                        Err(err) => {
                            println!("Err: {:?}", err);
                        }
                    }
                }
            }
        }
    }

    Ok(repo_evals)
}

fn clone_repo(repo_eval: RepoEval) -> anyhow::Result<PathBuf> {
    let repo_name = Path::new(repo_eval.repo.as_str())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace(".git", "");

    let clone_path = fs::canonicalize(env::current_dir()?)?
        .parent()
        .ok_or(anyhow!("path canonicalization failed"))?
        .join(TMP_REPO_PATH)
        .join(&repo_name);

    // Delete Clone Path if already exists
    let _ = fs::remove_dir_all(&clone_path);

    // Clone in Repo
    git2::build::RepoBuilder::new()
        // .branch(repo_eval.sha.as_str())
        .clone(repo_eval.repo.as_str(), clone_path.as_path())?;

    let repo: Repository = Repository::open(clone_path.clone())?;
    let obj: Object = repo
        .find_commit(Oid::from_str(repo_eval.commit.as_str())?)?
        .into_object();
    repo.checkout_tree(&obj, None)?;
    repo.set_head_detached(obj.id())?;

    Ok(clone_path)
}

fn dcg(hits: Vec<usize>) -> f32 {
    let mut result = 0.0;
    for (idx, hit) in hits.iter().enumerate() {
        result += *hit as f32 / (2.0 + idx as f32).log2();
    }

    println!("DCG: {:?}", result);
    result
}

fn evaluate_ndcg(eval_query: EvaluationQuery, search_results: Vec<SearchResult>, k: usize) -> f32 {
    // NDCG or Normalized Discounted Cumulative Gain, is determined by comparing the relevance of
    // items returned by the search engine relative to the hypothetical ideal.
    // Relevance is represented as a series of booleans, in which each search result returned
    // is identified as being inside the test set of matches (1) or not (0).

    // For example, if result 1, 3 and 5 match the 3 relevant results provided
    // actual dcg is calculated against a vector of [1, 0, 1, 0, 1]
    // whereas ideal dcg is calculated against a vector of [1, 1, 1, 0, 0]
    // as this ideal vector assumes the 3 relevant results provided were returned first
    // normalized dcg is then calculated as actual dcg / ideal dcg.

    // NDCG ranges from 0 to 1, which higher values indicating better performance
    // Commonly NDCG is expressed as NDCG@k, in which k represents the metric calculated
    // including only the top k values returned.
    // The @k metrics can help you identify, at what point does the relevant results start to fall off.
    // Ie. a NDCG@1 of 0.9 and a NDCG@3 of 0.5 may indicate that the first result returned in usually
    // very high quality, whereas rank results quickly drop off after the first result.

    let ideal = vec![1; cmp::min(eval_query.matches.len(), k)];
    let hits = vec![1];

    return dcg(hits) / dcg(ideal);
}

// fn evaluate_map(eval_query: EvaluationQuery, search_results: Vec<SearchResult>, k: usize) -> f32 {}

fn init_logger() {
    env_logger::init();
}

fn main() {
    // Launch new repo as a new Zed workspace/project
    let app = gpui::App::new(Assets).unwrap();
    let fs = Arc::new(RealFs);
    let http = http::client();
    let user_settings_file_rx =
        watch_config_file(app.background(), fs.clone(), paths::SETTINGS.clone());
    let http_client = http::client();
    init_logger();

    app.run(move |cx| {
        cx.set_global(*RELEASE_CHANNEL);

        let client = client::Client::new(http.clone(), cx);
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client.clone(), cx));

        // Initialize Settings
        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        handle_settings_file_changes(user_settings_file_rx, cx);

        // Initialize Languages
        let login_shell_env_loaded = Task::ready(());
        let mut languages = LanguageRegistry::new(login_shell_env_loaded);
        languages.set_executor(cx.background().clone());
        let languages = Arc::new(languages);

        let node_runtime = RealNodeRuntime::new(http.clone());
        languages::init(languages.clone(), node_runtime.clone());

        project::Project::init(&client, cx);
        semantic_index::init(fs.clone(), http.clone(), languages.clone(), cx);

        settings::register::<SemanticIndexSettings>(cx);

        let db_file_path = EMBEDDINGS_DIR
            .join(Path::new(RELEASE_CHANNEL_NAME.as_str()))
            .join("embeddings_db");

        let languages = languages.clone();
        let fs = fs.clone();
        cx.spawn(|mut cx| async move {
            let semantic_index = SemanticIndex::new(
                fs.clone(),
                db_file_path,
                Arc::new(OpenAIEmbeddings::new(http_client, cx.background())),
                languages.clone(),
                cx.clone(),
            )
            .await?;

            if let Ok(repo_evals) = parse_eval() {
                for repo in repo_evals {
                    let cloned = clone_repo(repo.clone());
                    match cloned {
                        Ok(clone_path) => {
                            log::trace!(
                                "Cloned {:?} @ {:?} into {:?}",
                                repo.repo,
                                repo.commit,
                                &clone_path
                            );

                            // Create Project
                            let project = cx.update(|cx| {
                                Project::local(
                                    client.clone(),
                                    user_store.clone(),
                                    languages.clone(),
                                    fs.clone(),
                                    cx,
                                )
                            });

                            // Register Worktree
                            let _ = project
                                .update(&mut cx, |project, cx| {
                                    println!(
                                        "Creating worktree in project: {:?}",
                                        clone_path.clone()
                                    );
                                    project.find_or_create_local_worktree(clone_path, true, cx)
                                })
                                .await;

                            let _ = semantic_index
                                .update(&mut cx, |index, cx| index.index_project(project, cx))
                                .await;
                        }
                        Err(err) => {
                            log::trace!("Error cloning: {:?}", err);
                        }
                    }
                }
            }

            anyhow::Ok(())
        })
        .detach();
    });
}
