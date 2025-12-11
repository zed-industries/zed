use anyhow::Result;
use gpui::Entity;
use project::Project;
use std::path::PathBuf;
use time::{OffsetDateTime, UtcOffset};
use util::command::new_smol_command;

use crate::graph_rendering::BRANCH_COLORS;

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
        let Some(sha) = parts.first() else { continue };
        let Some(short_sha) = parts.get(1) else {
            continue;
        };
        let Some(subject) = parts.get(2) else {
            continue;
        };
        let Some(author_name) = parts.get(3) else {
            continue;
        };
        let Some(timestamp_str) = parts.get(4) else {
            continue;
        };
        let Some(parents_str) = parts.get(5) else {
            continue;
        };

        let timestamp: i64 = timestamp_str.parse().unwrap_or(0);
        let parents: Vec<String> = parents_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let refs: Vec<String> = parts
            .get(6)
            .filter(|r| !r.is_empty())
            .map(|r| r.split(", ").map(|s| s.to_string()).collect())
            .unwrap_or_default();

        raw_commits.push((
            sha.to_string(),
            short_sha.to_string(),
            subject.to_string(),
            author_name.to_string(),
            timestamp,
            parents,
            refs,
        ));
    }

    let (commits, max_lanes) = build_graph(raw_commits);
    Ok((commits, max_lanes))
}

/// Builds the visual graph layout from raw commit data.
/// Returns a list of CommitEntry with lane assignments and graph lines, plus the max lane count.
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

    // Active lanes track which SHA is expected in each column.
    // Each lane is either None (empty/available) or Some((sha, color_index)).
    // When we see a commit, we look for its SHA in active_lanes to know where to place it.
    let mut active_lanes: Vec<Option<(String, usize)>> = Vec::new();

    // Maps lane index -> color index for consistent coloring within a branch
    let mut lane_colors: HashMap<usize, usize> = HashMap::new();

    // Maps lane index -> row where that lane started (for potential future use)
    let mut lane_start_row: HashMap<usize, usize> = HashMap::new();

    // Color counter that cycles through BRANCH_COLORS
    let mut next_color = 0;

    // Track the maximum number of lanes used (for graph width)
    let mut max_lanes = 0;

    for (row_idx, (sha, short_sha, subject, author_name, timestamp, parents, refs)) in
        raw_commits.into_iter().enumerate()
    {
        // Lines to draw for this row (vertical lines, merges, branches)
        let mut lines = Vec::new();

        // ========== STEP 1: Check if this commit was expected ==========
        // A commit is "expected" if a previous commit listed it as a parent,
        // meaning there's already a lane waiting for this SHA.
        let was_expected = active_lanes
            .iter()
            .any(|s| s.as_ref().map(|(h, _)| h) == Some(&sha));

        // ========== STEP 2: Find which lane this commit belongs in ==========
        // First, try to find a lane that's expecting this exact SHA.
        // If not found, use the first empty lane.
        // If no empty lanes, create a new one.
        let commit_lane = active_lanes
            .iter()
            .position(|s| s.as_ref().map(|(h, _)| h) == Some(&sha))
            .unwrap_or_else(|| {
                active_lanes
                    .iter()
                    .position(|s| s.is_none())
                    .unwrap_or_else(|| {
                        active_lanes.push(None);
                        active_lanes.len() - 1
                    })
            });

        // ========== STEP 3: Assign a color to this lane ==========
        // Reuse existing color for the lane, or assign a new one
        let color_idx = *lane_colors.entry(commit_lane).or_insert_with(|| {
            let color = next_color;
            next_color = (next_color + 1) % BRANCH_COLORS.len();
            color
        });

        // ========== STEP 4: Draw pass-through lines for other active lanes ==========
        // For every other lane that's active (waiting for a different commit),
        // draw a vertical line passing through this row.
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

        // ========== STEP 5: Clear the commit's lane ==========
        // This commit has arrived, so its lane is no longer waiting for it.
        // We'll potentially reuse this lane for the first parent below.
        if commit_lane < active_lanes.len() {
            active_lanes[commit_lane] = None;
            lane_start_row.remove(&commit_lane);
        }

        // ========== STEP 6: Process each parent of this commit ==========
        // For each parent:
        // - If the parent is already expected in another lane, draw a merge line
        // - If this is the first parent (i == 0), continue the current lane
        // - If this is a secondary parent, branch out to a new lane
        for (i, parent) in parents.iter().enumerate() {
            // Check if any lane is already waiting for this parent
            let existing_lane = active_lanes
                .iter()
                .position(|s| s.as_ref().map(|(h, _)| h) == Some(parent));

            if let Some(target_lane) = existing_lane {
                // ===== CASE A: Parent already has a lane (merge scenario) =====
                // Another branch is already tracking this parent commit.
                // Draw a merge line from our lane to that lane.
                let target_color = active_lanes[target_lane]
                    .as_ref()
                    .map(|(_, c)| *c)
                    .unwrap_or(color_idx);
                if target_lane != commit_lane {
                    // If this commit was expected (continuing a branch), draw the
                    // incoming vertical line that ends at this commit
                    // todo! expand on this more
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
                    // Draw the diagonal merge line to the existing parent lane
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
                // ===== CASE B: First parent, no existing lane =====
                // Continue the current lane downward to this parent.
                // The lane now expects the first parent SHA.
                if commit_lane < active_lanes.len() {
                    active_lanes[commit_lane] = Some((parent.clone(), color_idx));
                } else {
                    active_lanes.push(Some((parent.clone(), color_idx)));
                }
                lane_start_row.insert(commit_lane, row_idx);
                dbg!("This case is hit");
                // Draw the vertical line continuing down from this commit
                lines.push(GraphLine {
                    from_lane: commit_lane,
                    to_lane: commit_lane,
                    line_type: LineType::Straight,
                    color_idx,
                    continues_from_above: was_expected,
                    ends_at_commit: false,
                });
            } else {
                // ===== CASE C: Secondary parent (i > 0), no existing lane =====
                // This is a merge commit with multiple parents.
                // Branch out to a new lane for this additional parent.
                let target_lane = active_lanes
                    .iter()
                    .position(|s| s.is_none())
                    .unwrap_or_else(|| {
                        active_lanes.push(None);
                        active_lanes.len() - 1
                    });

                // Assign a new color to the branching lane
                let branch_color = *lane_colors.entry(target_lane).or_insert_with(|| {
                    let color = next_color;
                    next_color = (next_color + 1) % BRANCH_COLORS.len();
                    color
                });

                // Mark this lane as expecting the secondary parent
                active_lanes[target_lane] = Some((parent.clone(), branch_color));
                lane_start_row.insert(target_lane, row_idx);

                // Draw the diagonal branch-out line
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

        // ========== STEP 7: Update max lanes and create the commit entry ==========
        max_lanes = max_lanes.max(active_lanes.len());

        commits.push(CommitEntry {
            sha,
            short_sha,
            subject,
            author_name,
            formatted_time: format_timestamp(timestamp),
            parents,
            refs,
            lane: commit_lane,
            color_idx,
            lines,
        });
    }

    (commits, max_lanes.max(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp_valid() {
        let timestamp = 1700000000; // Nov 14, 2023
        let formatted = format_timestamp(timestamp);
        assert!(!formatted.is_empty());
        assert_ne!(formatted, "Unknown");
    }

    #[test]
    fn test_format_timestamp_invalid() {
        let timestamp = i64::MAX;
        let formatted = format_timestamp(timestamp);
        assert_eq!(formatted, "Unknown");
    }

    #[test]
    fn test_build_graph_empty() {
        let raw_commits = vec![];
        let (commits, max_lanes) = build_graph(raw_commits);
        assert!(commits.is_empty());
        assert_eq!(max_lanes, 1);
    }

    #[test]
    fn test_build_graph_single_commit() {
        let raw_commits = vec![(
            "abc123".to_string(),
            "abc".to_string(),
            "Initial commit".to_string(),
            "Author".to_string(),
            1700000000i64,
            vec![],
            vec![],
        )];
        let (commits, max_lanes) = build_graph(raw_commits);
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].sha, "abc123");
        assert_eq!(commits[0].short_sha, "abc");
        assert_eq!(commits[0].subject, "Initial commit");
        assert_eq!(commits[0].author_name, "Author");
        assert_eq!(commits[0].lane, 0);
        assert_eq!(max_lanes, 1);
    }

    #[test]
    fn test_build_graph_linear_history() {
        let raw_commits = vec![
            (
                "commit3".to_string(),
                "c3".to_string(),
                "Third".to_string(),
                "Author".to_string(),
                1700000003i64,
                vec!["commit2".to_string()],
                vec![],
            ),
            (
                "commit2".to_string(),
                "c2".to_string(),
                "Second".to_string(),
                "Author".to_string(),
                1700000002i64,
                vec!["commit1".to_string()],
                vec![],
            ),
            (
                "commit1".to_string(),
                "c1".to_string(),
                "First".to_string(),
                "Author".to_string(),
                1700000001i64,
                vec![],
                vec![],
            ),
        ];
        let (commits, max_lanes) = build_graph(raw_commits);
        assert_eq!(commits.len(), 3);
        assert_eq!(max_lanes, 1);

        for commit in &commits {
            assert_eq!(commit.lane, 0);
        }
    }

    #[test]
    fn test_build_graph_with_refs() {
        let raw_commits = vec![(
            "abc123".to_string(),
            "abc".to_string(),
            "Commit with refs".to_string(),
            "Author".to_string(),
            1700000000i64,
            vec![],
            vec!["HEAD -> main".to_string(), "origin/main".to_string()],
        )];
        let (commits, _) = build_graph(raw_commits);
        assert_eq!(commits[0].refs.len(), 2);
        assert!(commits[0].refs.contains(&"HEAD -> main".to_string()));
        assert!(commits[0].refs.contains(&"origin/main".to_string()));
    }

    #[test]
    fn test_build_graph_merge_commit() {
        let raw_commits = vec![
            (
                "merge".to_string(),
                "m".to_string(),
                "Merge branch".to_string(),
                "Author".to_string(),
                1700000003i64,
                vec!["parent1".to_string(), "parent2".to_string()],
                vec![],
            ),
            (
                "parent1".to_string(),
                "p1".to_string(),
                "Parent 1".to_string(),
                "Author".to_string(),
                1700000002i64,
                vec!["base".to_string()],
                vec![],
            ),
            (
                "parent2".to_string(),
                "p2".to_string(),
                "Parent 2".to_string(),
                "Author".to_string(),
                1700000001i64,
                vec!["base".to_string()],
                vec![],
            ),
            (
                "base".to_string(),
                "b".to_string(),
                "Base commit".to_string(),
                "Author".to_string(),
                1700000000i64,
                vec![],
                vec![],
            ),
        ];
        let (commits, max_lanes) = build_graph(raw_commits);
        assert_eq!(commits.len(), 4);
        assert!(max_lanes >= 1);

        let merge_commit = &commits[0];
        assert_eq!(merge_commit.parents.len(), 2);
    }
}
