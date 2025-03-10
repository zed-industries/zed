use crate::repository::CommitDetails;
use anyhow::{anyhow, Result};
use std::{path::Path, process::Stdio};

pub struct CommitHistory {}

impl CommitHistory {
    pub fn list(
        git_binary: &Path,
        working_directory: &Path,
        skip: i32,
        limit: i32,
    ) -> Result<Vec<CommitDetails>> {
        const COMMIT_WRAPPER_START: &str = "<COMMIT_START>";
        const COMMIT_WRAPPER_END: &str = "<COMMIT_END>";
        const DATA_MARKER: &str = "<DATA_MARKER>";
        // "--format=%H%n%aN%n%aE%n%at%n%ct%n%P%n%D%n%B",
        let child = util::command::new_std_command(git_binary)
            .current_dir(working_directory)
            .arg("log")
            .arg(format!(
                "--format={}%H<DATA_MARKER>%aN<DATA_MARKER>%aE<DATA_MARKER>%ct<DATA_MARKER>%B{}%n",
                COMMIT_WRAPPER_START, COMMIT_WRAPPER_END
            ))
            .arg("-z")
            .arg(format!("-n{}", limit))
            .arg(format!("--skip={}", skip))
            .args(["--topo-order", "--decorate=full"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to start git commit history process: {e}"))?;

        let output = child
            .wait_with_output()
            .map_err(|e| anyhow!("Failed to read git commit history output: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("git commit history process failed: {stderr}"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let commit_history = stdout
            .split('\0')
            .filter_map(|commit| {
                let trimmed_commit = commit
                    .trim()
                    .replace(COMMIT_WRAPPER_START, "")
                    .replace(COMMIT_WRAPPER_END, "");
                if trimmed_commit == "" || trimmed_commit == " " {
                    return None;
                };
                let records: Vec<String> = trimmed_commit
                    .split(DATA_MARKER)
                    .map(|s| s.trim().to_string())
                    .collect();
                if records.len() >= 4 {
                    return Some(CommitDetails {
                        sha: records[0].to_string().into(),
                        committer_name: records[1].to_string().into(),
                        committer_email: records[2].to_string().into(),
                        commit_timestamp: records[3].to_string().parse::<i64>().unwrap_or(0),
                        message: records[4].to_string().into(),
                    });
                } else {
                    return None;
                }
            })
            .collect::<Vec<CommitDetails>>();
        Ok(commit_history.into())
    }
}
