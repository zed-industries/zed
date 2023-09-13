use git2::{Object, Oid, Repository};
use semantic_index::SearchResult;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Deserialize, Clone)]
struct EvaluationQuery {
    query: String,
    matches: Vec<String>,
}

impl EvaluationQuery {
    fn match_pairs(&self) -> Vec<(PathBuf, usize)> {
        let mut pairs = Vec::new();
        for match_identifier in self.matches {
            let match_parts = match_identifier.split(":");

            if let Some(file_path) = match_parts.next() {
                if let Some(row_number) = match_parts.next() {
                    pairs.push((PathBuf::from(file_path), from_str::<usize>(row_number)));
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

const TMP_REPO_PATH: &str = "./target/eval_repos";

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
    let clone_path = Path::new(TMP_REPO_PATH).join(&repo_name).to_path_buf();

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

    return dcg(hits) / dcg(ideal);
}

fn evaluate_map(eval_query: EvaluationQuery, search_results: Vec<SearchResult>, k: usize) -> f32 {

}

fn evaluate_repo(repo_eval: RepoEval, clone_path: PathBuf) {

    // Launch new repo as a new Zed workspace/project
    // Index the project
    // Search each eval_query
    // Calculate Statistics

}

fn main() {

    // zed/main.rs
    // creating an app and running it, gives you the context.
    // create a project, find_or_create_local_worktree.

    if let Ok(repo_evals) = parse_eval() {
        for repo in repo_evals {
            let cloned = clone_repo(repo.clone());
            match cloned {
                Ok(clone_path) => {
                    println!(
                        "Cloned {:?} @ {:?} into {:?}",
                        repo.repo, repo.commit, &clone_path
                    );

                    // Evaluate Repo
                    evaluate_repo(repo, clone_path);

                }
                Err(err) => {
                    println!("Error Cloning: {:?}", err);
                }
            }
        }
    }
}
