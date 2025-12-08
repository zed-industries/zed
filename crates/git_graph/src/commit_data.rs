use anyhow::Result;
use gpui::Entity;
use project::Project;
use std::path::PathBuf;
use util::command::new_smol_command;

use crate::graph_rendering::BRANCH_COLORS;

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
    pub timestamp: i64,
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

pub async fn load_commits(
    project: Entity<Project>,
    cx: &mut gpui::AsyncApp,
) -> Result<(Vec<CommitEntry>, usize, PathBuf)> {
    let work_dir = cx
        .update(|cx| {
            let project = project.read(cx);
            project
                .worktrees(cx)
                .next()
                .map(|wt| wt.read(cx).abs_path().to_path_buf())
        })?
        .ok_or_else(|| anyhow::anyhow!("No worktree found"))?;

    let (commits, max_lanes) = fetch_git_log(&work_dir).await?;
    Ok((commits, max_lanes, work_dir))
}

async fn fetch_git_log(work_dir: &PathBuf) -> Result<(Vec<CommitEntry>, usize)> {
    let output = new_smol_command("git")
        .current_dir(work_dir)
        .args([
            "log",
            "--all",
            "--format=%H|%h|%s|%an|%at|%P|%D",
            "--date-order",
        ])
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git log failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut raw_commits = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 6 {
            let sha = parts[0].to_string();
            let short_sha = parts[1].to_string();
            let subject = parts[2].to_string();
            let author_name = parts[3].to_string();
            let timestamp = parts[4].parse().unwrap_or(0);
            let parents: Vec<String> = parts[5].split_whitespace().map(|s| s.to_string()).collect();
            let refs: Vec<String> = if parts.len() > 6 && !parts[6].is_empty() {
                parts[6].split(", ").map(|s| s.to_string()).collect()
            } else {
                Vec::new()
            };

            raw_commits.push((
                sha,
                short_sha,
                subject,
                author_name,
                timestamp,
                parents,
                refs,
            ));
        }
    }

    let (commits, max_lanes) = build_graph(raw_commits);
    Ok((commits, max_lanes))
}

fn build_graph(
    raw_commits: Vec<(
        String,
        String,
        String,
        String,
        i64,
        Vec<String>,
        Vec<String>,
    )>,
) -> (Vec<CommitEntry>, usize) {
    use std::collections::HashMap;

    let mut commits = Vec::new();
    let mut active_lanes: Vec<Option<(String, usize)>> = Vec::new();
    let mut lane_colors: HashMap<usize, usize> = HashMap::new();
    let mut lane_start_row: HashMap<usize, usize> = HashMap::new();
    let mut next_color = 0;
    let mut max_lanes = 0;

    for (row_idx, (sha, short_sha, subject, author_name, timestamp, parents, refs)) in
        raw_commits.into_iter().enumerate()
    {
        let mut lines = Vec::new();

        let was_expected = active_lanes
            .iter()
            .any(|s| s.as_ref().map(|(h, _)| h) == Some(&sha));

        let commit_lane = active_lanes
            .iter()
            .position(|s| s.as_ref().map(|(h, _)| h) == Some(&sha))
            .unwrap_or_else(|| {
                let lane = active_lanes
                    .iter()
                    .position(|s| s.is_none())
                    .unwrap_or_else(|| {
                        active_lanes.push(None);
                        active_lanes.len() - 1
                    });
                lane
            });

        let color_idx = *lane_colors.entry(commit_lane).or_insert_with(|| {
            let color = next_color;
            next_color = (next_color + 1) % BRANCH_COLORS.len();
            color
        });

        for (lane_idx, lane_data) in active_lanes.iter().enumerate() {
            if let Some((hash, lane_color)) = lane_data {
                if hash != &sha {
                    lines.push(GraphLine {
                        from_lane: lane_idx,
                        to_lane: lane_idx,
                        line_type: LineType::Straight,
                        color_idx: *lane_color,
                        continues_from_above: true,
                        ends_at_commit: false,
                    });
                }
            }
        }

        if commit_lane < active_lanes.len() {
            active_lanes[commit_lane] = None;
            lane_start_row.remove(&commit_lane);
        }

        for (i, parent) in parents.iter().enumerate() {
            let existing_lane = active_lanes
                .iter()
                .position(|s| s.as_ref().map(|(h, _)| h) == Some(parent));

            if let Some(target_lane) = existing_lane {
                let target_color = active_lanes[target_lane]
                    .as_ref()
                    .map(|(_, c)| *c)
                    .unwrap_or(color_idx);
                if target_lane != commit_lane {
                    if was_expected {
                        lines.push(GraphLine {
                            from_lane: commit_lane,
                            to_lane: commit_lane,
                            line_type: LineType::Straight,
                            color_idx,
                            continues_from_above: true,
                            ends_at_commit: true,
                        });
                    }
                    lines.push(GraphLine {
                        from_lane: commit_lane,
                        to_lane: target_lane,
                        line_type: LineType::MergeDown,
                        color_idx: target_color,
                        continues_from_above: false,
                        ends_at_commit: false,
                    });
                }
            } else if i == 0 {
                if commit_lane < active_lanes.len() {
                    active_lanes[commit_lane] = Some((parent.clone(), color_idx));
                } else {
                    active_lanes.push(Some((parent.clone(), color_idx)));
                }
                lane_start_row.insert(commit_lane, row_idx);
                lines.push(GraphLine {
                    from_lane: commit_lane,
                    to_lane: commit_lane,
                    line_type: LineType::Straight,
                    color_idx,
                    continues_from_above: was_expected,
                    ends_at_commit: false,
                });
            } else {
                let target_lane = active_lanes
                    .iter()
                    .position(|s| s.is_none())
                    .unwrap_or_else(|| {
                        active_lanes.push(None);
                        active_lanes.len() - 1
                    });

                let branch_color = *lane_colors.entry(target_lane).or_insert_with(|| {
                    let color = next_color;
                    next_color = (next_color + 1) % BRANCH_COLORS.len();
                    color
                });

                active_lanes[target_lane] = Some((parent.clone(), branch_color));
                lane_start_row.insert(target_lane, row_idx);
                lines.push(GraphLine {
                    from_lane: commit_lane,
                    to_lane: target_lane,
                    line_type: LineType::BranchOut,
                    color_idx: branch_color,
                    continues_from_above: false,
                    ends_at_commit: false,
                });
            }
        }

        max_lanes = max_lanes.max(active_lanes.len());

        commits.push(CommitEntry {
            sha,
            short_sha,
            subject,
            author_name,
            timestamp,
            parents,
            refs,
            lane: commit_lane,
            color_idx,
            lines,
        });
    }

    (commits, max_lanes.max(1))
}
