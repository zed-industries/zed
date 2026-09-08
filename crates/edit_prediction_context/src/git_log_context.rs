// Goal:
// - Build an index that finds files that are frequently edited in the same git commit
// - Lookup by path and get a list of related files, sorted by most frequently edited together
//
// Path => Path => usize
//
// This is a symmetric relationship, so for a => (b, 1), also add b => (a, 1)

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context as _, Result, anyhow, bail};
use util::command::new_command;

pub struct GitLogIndex {
    index: HashMap<PathBuf, HashMap<PathBuf, usize>>,
}

impl GitLogIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn add_related(&mut self, path: PathBuf, related: PathBuf) {
        let count = self
            .index
            .entry(path.clone())
            .or_default()
            .entry(related.clone())
            .or_default();
        *count += 1;

        // add the reverse mapping
        let reverse_count = self
            .index
            .entry(related)
            .or_default()
            .entry(path)
            .or_default();
        *reverse_count += 1;
    }

    pub fn get_related(&self, path: &Path, n: usize) -> Vec<PathBuf> {
        self.get_related_with_counts(path, n)
            .into_iter()
            .map(|(path, _)| path)
            .collect()
    }

    pub fn get_related_with_counts(&self, path: &Path, n: usize) -> Vec<(PathBuf, usize)> {
        let Some(counts) = self.index.get(path) else {
            return Vec::new();
        };

        let mut related: Vec<_> = counts.iter().collect();
        related.sort_by(|(left_path, left_count), (right_path, right_count)| {
            right_count
                .cmp(left_count)
                .then_with(|| left_path.cmp(right_path))
        });
        related
            .into_iter()
            .take(n)
            .map(|(path, count)| (path.clone(), *count))
            .collect()
    }
}

impl Default for GitLogIndex {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn build_git_log_index(worktree_dir: &Path) -> Result<GitLogIndex> {
    let mut index = GitLogIndex::new();

    let output = new_command("git")
        .arg("log")
        .arg("-5000")
        .arg("--pretty=tformat:@@COMMIT %H")
        .arg("--name-only")
        .current_dir(worktree_dir)
        .output()
        .await
        .with_context(|| format!("failed to run git log in {}", worktree_dir.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "git log failed in {} with status {}: {}",
            worktree_dir.display(),
            output.status,
            stderr.trim()
        );
    }

    let log = String::from_utf8(output.stdout).context("git log output was not valid UTF-8")?;
    let parsed = parse_git_log(&log);
    for files in parsed {
        for i in 0..files.len() {
            for j in (i + 1)..files.len() {
                index.add_related(files[i].clone(), files[j].clone());
            }
        }
    }

    Ok(index)
}

#[allow(dead_code)]
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

#[allow(dead_code)]
fn run() -> Result<()> {
    let mut arguments = env::args_os();
    let program_name = arguments
        .next()
        .and_then(|path| PathBuf::from(path).file_name().map(|name| name.to_owned()))
        .and_then(|name| name.into_string().ok())
        .unwrap_or_else(|| "git_log_context".to_string());

    let worktree_dir = arguments.next().ok_or_else(|| {
        print_usage(&program_name);
        anyhow!("missing worktree path")
    })?;
    let query_path = arguments.next().ok_or_else(|| {
        print_usage(&program_name);
        anyhow!("missing query path")
    })?;
    if arguments.next().is_some() {
        print_usage(&program_name);
        bail!("too many arguments");
    }

    let worktree_dir = PathBuf::from(worktree_dir);
    let query_path = normalize_query_path(&worktree_dir, &PathBuf::from(query_path));
    let index = futures::executor::block_on(build_git_log_index(&worktree_dir))?;

    for (path, count) in index.get_related_with_counts(&query_path, 10) {
        println!("{count}\t{}", path.display());
    }

    Ok(())
}

#[allow(dead_code)]
fn print_usage(program_name: &str) {
    eprintln!("Usage: {program_name} <worktree-path> <query-path>");
}

#[allow(dead_code)]
fn normalize_query_path(worktree_dir: &Path, query_path: &Path) -> PathBuf {
    if query_path.is_absolute() {
        query_path
            .strip_prefix(worktree_dir)
            .unwrap_or(query_path)
            .components()
            .collect()
    } else {
        query_path.components().collect()
    }
}

fn parse_git_log(log: &str) -> Vec<Vec<PathBuf>> {
    let mut lines = log.lines().peekable();
    let mut commits = Vec::new();

    while let Some(line) = lines.next() {
        if line.starts_with("@@COMMIT ") {
            // skip blank line
            lines.next();
            let mut files = Vec::new();
            while let Some(next) = lines.peek()
                && !next.starts_with("@@COMMIT ")
            {
                let Some(next) = lines.next() else {
                    break;
                };
                if !next.is_empty() {
                    files.push(next.into());
                }
            }
            commits.push(files);
        }
    }

    commits
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_git_log_index() {
        let mut index = GitLogIndex::new();
        index.add_related(PathBuf::from("a"), PathBuf::from("b"));
        index.add_related(PathBuf::from("a"), PathBuf::from("b"));
        index.add_related(PathBuf::from("a"), PathBuf::from("c"));
        index.add_related(PathBuf::from("b"), PathBuf::from("c"));

        let related = index.get_related(&PathBuf::from("a"), 100);
        assert_eq!(related, vec![PathBuf::from("b"), PathBuf::from("c")]);
    }

    #[test]
    fn test_parse_git_log() {
        let log = indoc! {"
            @@COMMIT d2e451dd48be67ef8c943e90dabc02e80a6984c9

            crates/edit_prediction/src/edit_prediction.rs
            crates/edit_prediction_cli/src/format_prompt.rs
            crates/edit_prediction_cli/src/main.rs
            crates/edit_prediction_cli/src/predict.rs
            @@COMMIT d666823f348bd151067464fa31676d28bdb96717

            crates/edit_prediction_cli/src/main.rs
            crates/edit_prediction_cli/src/predict.rs
            "};
        let parsed = parse_git_log(log);

        assert_eq!(parsed.len(), 2);
        assert_eq!(
            parsed[0][0],
            PathBuf::from("crates/edit_prediction/src/edit_prediction.rs")
        );
        assert_eq!(
            parsed[0][1],
            PathBuf::from("crates/edit_prediction_cli/src/format_prompt.rs")
        );

        assert_eq!(
            parsed[1][0],
            PathBuf::from("crates/edit_prediction_cli/src/main.rs")
        );
        assert_eq!(
            parsed[1][1],
            PathBuf::from("crates/edit_prediction_cli/src/predict.rs")
        );
    }
}
