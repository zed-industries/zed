use git2::{Object, Oid, Repository};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Deserialize, Clone)]
struct QueryMatches {
    query: String,
    matches: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct RepoEval {
    repo: String,
    commit: String,
    assertions: Vec<QueryMatches>,
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

fn main() {
    if let Ok(repo_evals) = parse_eval() {
        for repo in repo_evals {
            let cloned = clone_repo(repo.clone());
            match cloned {
                Ok(clone_path) => {
                    println!(
                        "Cloned {:?} @ {:?} into {:?}",
                        repo.repo, repo.commit, clone_path
                    );
                }
                Err(err) => {
                    println!("Error Cloning: {:?}", err);
                }
            }
        }
    }
}
