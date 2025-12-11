use anyhow::Result;
use std::path::PathBuf;
use time::{OffsetDateTime, UtcOffset};
use util::command::new_smol_command;

pub(crate) fn format_timestamp(timestamp: i64) -> String {
    let Ok(datetime) = OffsetDateTime::from_unix_timestamp(timestamp) else {
        return "Unknown".to_string();
    };

    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local_datetime = datetime.to_offset(local_offset);

    // todo! do we have to parse this function every time?
    let format = time::format_description::parse("[day] [month repr:short] [year] [hour]:[minute]")
        .unwrap_or_default();
    local_datetime.format(&format).unwrap_or_default()
}

#[derive(Clone, Debug)]
pub struct GraphLine {
    pub from_lane: usize,
    pub to_lane: usize,
    pub line_type: LineType,
    pub color_idx: usize,
    pub continues_from_above: bool,
    pub ends_at_commit: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    Straight,
    MergeDown,
    BranchOut,
}

#[derive(Clone, Debug)]
pub struct CommitEntry {
    pub sha: String,
    pub short_sha: String,
    pub subject: String,
    pub author_name: String,
    pub formatted_time: String,
    pub parents: Vec<String>,
    pub refs: Vec<String>,
    pub lane: usize,
    pub color_idx: usize,
    pub lines: Vec<GraphLine>,
}

pub async fn run_git_command(work_dir: &PathBuf, args: &[&str]) -> Result<String> {
    let output = new_smol_command("git")
        .current_dir(work_dir)
        .args(args)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("{}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
