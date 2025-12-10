use serde::Deserialize;
use std::sync::Arc;

use crate::{
    DistillArguments,
    example::Example,
    source_location::SourceLocation,
    training::{
        context::ContextType,
        llm_client::LlmClient,
        teacher::{TeacherModel, TeacherOutput},
    },
};
use anyhow::Result;
use reqwest_client::ReqwestClient;

#[derive(Debug, Deserialize)]
pub struct SplitCommit {
    repo_url: String,
    commit_sha: String,
    edit_history: String,
    expected_patch: String,
    cursor_position: String,
}

pub async fn run_distill(arguments: DistillArguments) -> Result<()> {
    let split_commits: Vec<SplitCommit> = std::fs::read_to_string(&arguments.split_commit_dataset)
        .expect("Failed to read split commit dataset")
        .lines()
        .map(|line| serde_json::from_str(line).expect("Failed to parse JSON line"))
        .collect();

    let http_client: Arc<dyn http_client::HttpClient> = Arc::new(ReqwestClient::new());

    let llm_client = if let Some(cache_path) = arguments.batch {
        LlmClient::batch(&cache_path, http_client)?
    } else {
        LlmClient::plain(http_client)?
    };

    let mut teacher = TeacherModel::new(
        "claude-sonnet-4-5".to_string(),
        ContextType::CurrentFile,
        llm_client,
    );

    let mut num_marked_for_batching = 0;

    for commit in split_commits {
        if let Some(distilled) = distill_one(&mut teacher, commit).await? {
            println!("{}", serde_json::to_string(&distilled)?);
        } else {
            if num_marked_for_batching == 0 {
                log::warn!("Marked for batching");
            }
            num_marked_for_batching += 1;
        }
    }

    eprintln!(
        "{} requests are marked for batching",
        num_marked_for_batching
    );
    let llm_client = teacher.client;
    llm_client.sync_batches().await?;

    Ok(())
}

pub async fn distill_one(
    teacher: &mut TeacherModel,
    commit: SplitCommit,
) -> Result<Option<TeacherOutput>> {
    let cursor: SourceLocation = commit
        .cursor_position
        .parse()
        .expect("Failed to parse cursor position");

    let path = cursor.path.to_rel_path_buf();

    let example = Example {
        repository_url: commit.repo_url,
        revision: commit.commit_sha,
        uncommitted_diff: commit.edit_history.clone(),
        cursor_path: path.as_std_path().to_path_buf(),
        cursor_position: commit.cursor_position,
        edit_history: commit.edit_history, // todo: trim
        expected_patch: commit.expected_patch,
    };

    let prediction = teacher.predict(example).await;

    prediction
}
