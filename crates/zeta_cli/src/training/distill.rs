use serde::Deserialize;

use crate::{
    DistillArguments,
    example::Example,
    source_location::SourceLocation,
    training::{context::ContextType, teacher::TeacherModel},
};
use anyhow::Result;

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

    for commit in split_commits {
        let distilled = distill_one(commit).await?;
        println!("{}", distilled);
    }

    Ok(())
}

pub async fn distill_one(commit: SplitCommit) -> Result<String> {
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

    let teacher = TeacherModel::new("claude-sonnet-4-5".to_string(), ContextType::CurrentFile);

    let prediction = teacher.predict(example).await;

    prediction
}
