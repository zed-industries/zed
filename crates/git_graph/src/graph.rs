use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use collections::HashMap;
use git::Oid;
use smallvec::SmallVec;
use util::command::new_smol_command;

use crate::{
    commit_data::{CommitEntry, GraphLine, LineType, format_timestamp},
    graph_rendering::BRANCH_COLORS,
};

/// %H - Full commit hash
/// %aN - Author name
/// %aE - Author email
/// %at - Author timestamp
/// %ct - Commit timestamp
/// %s - Commit summary
/// %P - Parent hashes
/// %D - Ref names
/// %x1E - ASCII record separator, used to split up commit data
static COMMIT_FORMAT: &str = "--format=%H%x1E%aN%x1E%aE%x1E%at%x1E%ct%x1E%s%x1E%P%x1E%D%x1E";

/// Commit data needed for the graph
#[derive(Debug)]
pub struct GraphCommit {
    pub sha: Oid,
    /// Most commits have a single parent, so we use a small vec to avoid allocations
    pub parents: smallvec::SmallVec<[Oid; 1]>,
    pub author_name: String,
    pub _author_email: String,
    pub commit_timestamp: i64,
    pub subject: String,
    pub ref_names: Vec<String>,
}

// todo: This function should be on a background thread, and it should return a chunk of commits at a time
// we should also be able to specify the order
// todo: Make this function work over collab as well
pub async fn load_commits(worktree_path: PathBuf) -> Result<Vec<GraphCommit>> {
    let git_log_output = new_smol_command("git")
        .current_dir(worktree_path)
        .arg("log")
        .arg("--all")
        .arg(COMMIT_FORMAT)
        .arg("--date-order")
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&git_log_output.stdout);

    Ok(stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            // todo! clean this up
            let parts: Vec<&str> = line.split('\x1E').collect();

            let sha = parts.get(0)?;
            let author_name = parts.get(1)?;
            let author_email = parts.get(2)?;
            // todo! do we use the author or the commit timestamp
            let _author_timestamp = parts.get(3)?;
            let commit_timestamp = parts.get(4)?;

            let summary = parts.get(5)?;
            let parents = parts
                .get(6)?
                .split_ascii_whitespace()
                .filter_map(|hash| Oid::from_str(hash).ok())
                .collect();

            Some(GraphCommit {
                author_name: author_name.to_string(),
                _author_email: author_email.to_string(),
                sha: Oid::from_str(sha).ok()?,
                parents,
                commit_timestamp: commit_timestamp.parse().ok()?, //todo!
                subject: summary.to_string(),                     // todo!
                ref_names: parts
                    .get(7)
                    .filter(|ref_name| !ref_name.is_empty())
                    .map(|ref_names| ref_names.split(", ").map(ToString::to_string).collect())
                    .unwrap_or_default(),
            })
        })
        .collect::<Vec<_>>())
}

#[derive(Copy, Clone)]
struct BranchColor(u8);

enum LaneState {
    Empty,
    Active { sha: Oid, color: BranchColor },
}

impl LaneState {
    fn is_commit(&self, other: &Oid) -> bool {
        match self {
            LaneState::Empty => false,
            LaneState::Active { sha, .. } => sha == other,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            LaneState::Empty => true,
            LaneState::Active { .. } => false,
        }
    }
}

type ActiveLaneIdx = usize;

pub struct GitGraph {
    lane_states: SmallVec<[LaneState; 8]>,
    lane_colors: HashMap<ActiveLaneIdx, BranchColor>,
    next_color: BranchColor,
    pub commits: Vec<CommitEntry>,
    pub max_lanes: usize,
}

impl GitGraph {
    pub fn new() -> Self {
        GitGraph {
            lane_states: SmallVec::default(),
            lane_colors: HashMap::default(),
            next_color: BranchColor(0),
            commits: Vec::default(),
            max_lanes: 0,
        }
    }

    fn first_empty_lane_idx(&mut self) -> ActiveLaneIdx {
        self.lane_states
            .iter()
            .position(LaneState::is_empty)
            .unwrap_or_else(|| {
                self.lane_states.push(LaneState::Empty);
                self.lane_states.len() - 1
            })
    }

    fn get_lane_color(&mut self, lane_idx: ActiveLaneIdx) -> BranchColor {
        *self.lane_colors.entry(lane_idx).or_insert_with(|| {
            let color_idx = self.next_color;
            self.next_color = BranchColor((self.next_color.0 + 1) % BRANCH_COLORS.len() as u8);
            color_idx
        })
    }

    pub(crate) fn add_commits(&mut self, commits: Vec<GraphCommit>) {
        for commit in commits.into_iter() {
            let commit_lane = self
                .lane_states
                .iter()
                .position(|lane: &LaneState| lane.is_commit(&commit.sha));

            let branch_continued = commit_lane.is_some();
            let commit_lane = commit_lane.unwrap_or_else(|| self.first_empty_lane_idx());
            let commit_color = self.get_lane_color(commit_lane);

            let mut lines = Vec::from_iter(self.lane_states.iter().enumerate().filter_map(
                |(idx, lane)| {
                    match lane {
                        // todo!: We can probably optimize this by using commit_lane != idx && !was_expected
                        LaneState::Active { sha, color } if sha != &commit.sha => {
                            Some(GraphLine {
                                from_lane: idx,
                                to_lane: idx,
                                line_type: LineType::Straight,
                                color_idx: color.0 as usize, // todo! change this
                                continues_from_above: true,
                                ends_at_commit: false,
                            })
                        }
                        _ => None,
                    }
                },
            ));

            self.lane_states[commit_lane] = LaneState::Empty;

            if commit.parents.is_empty() && branch_continued {
                lines.push(GraphLine {
                    from_lane: commit_lane,
                    to_lane: commit_lane,
                    line_type: LineType::Straight,
                    color_idx: commit_color.0 as usize,
                    continues_from_above: true,
                    ends_at_commit: true,
                });
            }

            commit
                .parents
                .iter()
                .enumerate()
                .for_each(|(parent_idx, parent)| {
                    let parent_lane =
                        self.lane_states
                            .iter()
                            .enumerate()
                            .find_map(|(lane_idx, lane_state)| match lane_state {
                                LaneState::Active { sha, color } if sha == parent => {
                                    Some((lane_idx, color))
                                }
                                _ => None,
                            });

                    if let Some((parent_lane, parent_color)) = parent_lane
                        && parent_lane != commit_lane
                    {
                        // todo! add comment explaining why this is necessary
                        if branch_continued {
                            lines.push(GraphLine {
                                from_lane: commit_lane,
                                to_lane: commit_lane,
                                line_type: LineType::Straight,
                                // todo! this field should be a byte
                                color_idx: commit_color.0 as usize,
                                continues_from_above: true,
                                ends_at_commit: true,
                            });
                        }

                        lines.push(GraphLine {
                            from_lane: commit_lane,
                            to_lane: parent_lane,
                            line_type: LineType::MergeDown,
                            color_idx: parent_color.0 as usize,
                            continues_from_above: false,
                            ends_at_commit: false,
                        });
                    } else if parent_idx == 0 {
                        self.lane_states[commit_lane] = LaneState::Active {
                            sha: *parent,
                            color: commit_color,
                        };
                        lines.push(GraphLine {
                            from_lane: commit_lane,
                            to_lane: commit_lane,
                            line_type: LineType::Straight,
                            color_idx: commit_color.0 as usize,
                            continues_from_above: branch_continued,
                            ends_at_commit: false,
                        });
                    } else {
                        let parent_lane = self.first_empty_lane_idx();
                        let parent_color = self.get_lane_color(parent_lane);

                        lines.push(GraphLine {
                            from_lane: commit_lane,
                            to_lane: parent_lane,
                            line_type: LineType::BranchOut,
                            color_idx: parent_color.0 as usize,
                            continues_from_above: false,
                            ends_at_commit: false,
                        });
                    }
                });

            self.max_lanes = self.max_lanes.max(self.lane_states.len());

            self.commits.push(CommitEntry {
                sha: commit.sha.to_string(),
                short_sha: commit.sha.display_short(),
                subject: commit.subject,
                author_name: commit.author_name,
                formatted_time: format_timestamp(commit.commit_timestamp),
                parents: commit
                    .parents
                    .into_iter()
                    .map(|parent| parent.to_string())
                    .collect(),
                refs: commit.ref_names,
                lane: commit_lane,
                color_idx: commit_color.0 as usize,
                lines,
            });
        }
    }
}
