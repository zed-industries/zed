use ai::embedding::OpenAIEmbeddings;
use anyhow::{anyhow, Result};
use client::{self, UserStore};
use gpui::{AsyncAppContext, ModelHandle, Task};
use language::LanguageRegistry;
use node_runtime::RealNodeRuntime;
use project::{Project, RealFs};
use semantic_index::semantic_index_settings::SemanticIndexSettings;
use semantic_index::{SearchResult, SemanticIndex};
use serde::{Deserialize, Serialize};
use settings::{default_settings, SettingsStore};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{cmp, env, fs};
use util::channel::{RELEASE_CHANNEL, RELEASE_CHANNEL_NAME};
use util::http::{self};
use util::paths::EMBEDDINGS_DIR;
use zed::languages;

#[derive(Deserialize, Clone, Serialize)]
struct EvaluationQuery {
    query: String,
    matches: Vec<String>,
}

impl EvaluationQuery {
    fn match_pairs(&self) -> Vec<(PathBuf, u32)> {
        let mut pairs = Vec::new();
        for match_identifier in self.matches.iter() {
            let mut match_parts = match_identifier.split(":");

            if let Some(file_path) = match_parts.next() {
                if let Some(row_number) = match_parts.next() {
                    pairs.push((PathBuf::from(file_path), row_number.parse::<u32>().unwrap()));
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

fn clone_repo(repo_eval: RepoEval) -> anyhow::Result<(String, PathBuf)> {
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
        .parent()
        .unwrap()
        .join(TMP_REPO_PATH);

    // Delete Clone Path if already exists
    let _ = fs::remove_dir_all(&clone_path);
    let _ = fs::create_dir(&clone_path);

    let _ = Command::new("git")
        .args(["clone", repo_eval.repo.as_str()])
        .current_dir(clone_path.clone())
        .output()?;
    // Update clone path to be new directory housing the repo.
    let clone_path = clone_path.join(repo_name.clone());
    let _ = Command::new("git")
        .args(["checkout", repo_eval.commit.as_str()])
        .current_dir(clone_path.clone())
        .output()?;

    Ok((repo_name, clone_path))
}

fn dcg(hits: Vec<usize>) -> f32 {
    let mut result = 0.0;
    for (idx, hit) in hits.iter().enumerate() {
        result += *hit as f32 / (2.0 + idx as f32).log2();
    }

    result
}

fn get_hits(
    eval_query: EvaluationQuery,
    search_results: Vec<SearchResult>,
    k: usize,
    cx: &AsyncAppContext,
) -> (Vec<usize>, Vec<usize>) {
    let ideal = vec![1; cmp::min(eval_query.matches.len(), k)];

    let mut hits = Vec::new();
    for result in search_results {
        let (path, start_row, end_row) = result.buffer.read_with(cx, |buffer, _cx| {
            let path = buffer.file().unwrap().path().to_path_buf();
            let start_row = buffer.offset_to_point(result.range.start.offset).row;
            let end_row = buffer.offset_to_point(result.range.end.offset).row;
            (path, start_row, end_row)
        });

        let match_pairs = eval_query.match_pairs();
        let mut found = 0;
        for (match_path, match_row) in match_pairs {
            if match_path == path {
                if match_row >= start_row && match_row <= end_row {
                    found = 1;
                    break;
                }
            }
        }

        hits.push(found);
    }

    // For now, we are calculating ideal_hits a bit different, as technically
    // with overlapping ranges, one match can result in more than result.
    let mut ideal_hits = hits.clone();
    ideal_hits.retain(|x| x == &1);

    let ideal = if ideal.len() > ideal_hits.len() {
        ideal
    } else {
        ideal_hits
    };

    // Fill ideal to 10 length
    let mut filled_ideal = [0; 10];
    for (idx, i) in ideal.to_vec().into_iter().enumerate() {
        filled_ideal[idx] = i;
    }

    (filled_ideal.to_vec(), hits)
}

fn evaluate_ndcg(hits: Vec<usize>, ideal: Vec<usize>) -> Vec<f32> {
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

    let mut ndcg = Vec::new();
    for idx in 1..(hits.len() + 1) {
        let hits_at_k = hits[0..idx].to_vec();
        let ideal_at_k = ideal[0..idx].to_vec();

        let at_k = dcg(hits_at_k.clone()) / dcg(ideal_at_k.clone());

        ndcg.push(at_k);
    }

    ndcg
}

fn evaluate_map(hits: Vec<usize>) -> Vec<f32> {
    let mut map_at_k = Vec::new();

    let non_zero = hits.iter().sum::<usize>() as f32;
    if non_zero == 0.0 {
        return vec![0.0; hits.len()];
    }

    let mut rolling_non_zero = 0.0;
    let mut rolling_map = 0.0;
    for (idx, h) in hits.into_iter().enumerate() {
        rolling_non_zero += h as f32;
        if h == 1 {
            rolling_map += rolling_non_zero / (idx + 1) as f32;
        }
        map_at_k.push(rolling_map / non_zero);
    }

    map_at_k
}

fn evaluate_mrr(hits: Vec<usize>) -> f32 {
    for (idx, h) in hits.into_iter().enumerate() {
        if h == 1 {
            return 1.0 / (idx + 1) as f32;
        }
    }

    return 0.0;
}

fn init_logger() {
    env_logger::init();
}

#[derive(Serialize)]
struct QueryMetrics {
    query: EvaluationQuery,
    millis_to_search: Duration,
    ndcg: Vec<f32>,
    map: Vec<f32>,
    mrr: f32,
    hits: Vec<usize>,
    precision: Vec<f32>,
    recall: Vec<f32>,
}

#[derive(Serialize)]
struct SummaryMetrics {
    millis_to_search: f32,
    ndcg: Vec<f32>,
    map: Vec<f32>,
    mrr: f32,
    precision: Vec<f32>,
    recall: Vec<f32>,
}

#[derive(Serialize)]
struct RepoEvaluationMetrics {
    millis_to_index: Duration,
    query_metrics: Vec<QueryMetrics>,
    repo_metrics: Option<SummaryMetrics>,
}

impl RepoEvaluationMetrics {
    fn new(millis_to_index: Duration) -> Self {
        RepoEvaluationMetrics {
            millis_to_index,
            query_metrics: Vec::new(),
            repo_metrics: None,
        }
    }

    fn save(&self, repo_name: String) -> Result<()> {
        let results_string = serde_json::to_string(&self)?;
        fs::write(format!("./{}_evaluation.json", repo_name), results_string)
            .expect("Unable to write file");
        Ok(())
    }

    fn summarize(&mut self) {
        let l = self.query_metrics.len() as f32;
        let millis_to_search: f32 = self
            .query_metrics
            .iter()
            .map(|metrics| metrics.millis_to_search.as_millis())
            .sum::<u128>() as f32
            / l;

        let mut ndcg_sum = vec![0.0; 10];
        let mut map_sum = vec![0.0; 10];
        let mut precision_sum = vec![0.0; 10];
        let mut recall_sum = vec![0.0; 10];
        let mut mmr_sum = 0.0;

        for query_metric in self.query_metrics.iter() {
            for (ndcg, query_ndcg) in ndcg_sum.iter_mut().zip(query_metric.ndcg.clone()) {
                *ndcg += query_ndcg;
            }

            for (mapp, query_map) in map_sum.iter_mut().zip(query_metric.map.clone()) {
                *mapp += query_map;
            }

            for (pre, query_pre) in precision_sum.iter_mut().zip(query_metric.precision.clone()) {
                *pre += query_pre;
            }

            for (rec, query_rec) in recall_sum.iter_mut().zip(query_metric.recall.clone()) {
                *rec += query_rec;
            }

            mmr_sum += query_metric.mrr;
        }

        let ndcg = ndcg_sum.iter().map(|val| val / l).collect::<Vec<f32>>();
        let map = map_sum.iter().map(|val| val / l).collect::<Vec<f32>>();
        let precision = precision_sum
            .iter()
            .map(|val| val / l)
            .collect::<Vec<f32>>();
        let recall = recall_sum.iter().map(|val| val / l).collect::<Vec<f32>>();
        let mrr = mmr_sum / l;

        self.repo_metrics = Some(SummaryMetrics {
            millis_to_search,
            ndcg,
            map,
            mrr,
            precision,
            recall,
        })
    }
}

fn evaluate_precision(hits: Vec<usize>) -> Vec<f32> {
    let mut rolling_hit: f32 = 0.0;
    let mut precision = Vec::new();
    for (idx, hit) in hits.into_iter().enumerate() {
        rolling_hit += hit as f32;
        precision.push(rolling_hit / ((idx as f32) + 1.0));
    }

    precision
}

fn evaluate_recall(hits: Vec<usize>, ideal: Vec<usize>) -> Vec<f32> {
    let total_relevant = ideal.iter().sum::<usize>() as f32;
    let mut recall = Vec::new();
    let mut rolling_hit: f32 = 0.0;
    for hit in hits {
        rolling_hit += hit as f32;
        recall.push(rolling_hit / total_relevant);
    }

    recall
}

async fn evaluate_repo(
    repo_name: String,
    index: ModelHandle<SemanticIndex>,
    project: ModelHandle<Project>,
    query_matches: Vec<EvaluationQuery>,
    cx: &mut AsyncAppContext,
) -> Result<RepoEvaluationMetrics> {
    // Index Project
    let index_t0 = Instant::now();
    index
        .update(cx, |index, cx| index.index_project(project.clone(), cx))
        .await?;
    let mut repo_metrics = RepoEvaluationMetrics::new(index_t0.elapsed());

    for query in query_matches {
        // Query each match in order
        let search_t0 = Instant::now();
        let search_results = index
            .update(cx, |index, cx| {
                index.search_project(project.clone(), query.clone().query, 10, vec![], vec![], cx)
            })
            .await?;
        let millis_to_search = search_t0.elapsed();

        // Get Hits/Ideal
        let k = 10;
        let (ideal, hits) = self::get_hits(query.clone(), search_results, k, cx);

        // Evaluate ndcg@k, for k = 1, 3, 5, 10
        let ndcg = evaluate_ndcg(hits.clone(), ideal.clone());

        // Evaluate map@k, for k = 1, 3, 5, 10
        let map = evaluate_map(hits.clone());

        // Evaluate mrr
        let mrr = evaluate_mrr(hits.clone());

        // Evaluate precision
        let precision = evaluate_precision(hits.clone());

        // Evaluate Recall
        let recall = evaluate_recall(hits.clone(), ideal);

        let query_metrics = QueryMetrics {
            query,
            millis_to_search,
            ndcg,
            map,
            mrr,
            hits,
            precision,
            recall,
        };

        repo_metrics.query_metrics.push(query_metrics);
    }

    repo_metrics.summarize();
    let _ = repo_metrics.save(repo_name);

    anyhow::Ok(repo_metrics)
}

fn main() {
    // Launch new repo as a new Zed workspace/project
    let app = gpui::App::new(()).unwrap();
    let fs = Arc::new(RealFs);
    let http = http::client();
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

        // Initialize Languages
        let login_shell_env_loaded = Task::ready(());
        let mut languages = LanguageRegistry::new(login_shell_env_loaded);
        languages.set_executor(cx.background().clone());
        let languages = Arc::new(languages);

        let node_runtime = RealNodeRuntime::new(http.clone());
        languages::init(languages.clone(), node_runtime.clone(), cx);
        language::init(cx);

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
                        Ok((repo_name, clone_path)) => {
                            println!(
                                "Cloned {:?} @ {:?} into {:?}",
                                repo.repo, repo.commit, &clone_path
                            );

                            // Create Project
                            let project = cx.update(|cx| {
                                Project::local(
                                    client.clone(),
                                    node_runtime::FakeNodeRuntime::new(),
                                    user_store.clone(),
                                    languages.clone(),
                                    fs.clone(),
                                    cx,
                                )
                            });

                            // Register Worktree
                            let _ = project
                                .update(&mut cx, |project, cx| {
                                    project.find_or_create_local_worktree(clone_path, true, cx)
                                })
                                .await;

                            let _ = evaluate_repo(
                                repo_name,
                                semantic_index.clone(),
                                project,
                                repo.assertions,
                                &mut cx,
                            )
                            .await?;
                        }
                        Err(err) => {
                            println!("Error cloning: {:?}", err);
                        }
                    }
                }
            }

            anyhow::Ok(())
        })
        .detach();
    });
}
