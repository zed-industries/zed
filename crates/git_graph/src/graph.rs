use std::{path::PathBuf, rc::Rc, str::FromStr};

use anyhow::Result;
use collections::HashMap;
use git::Oid;
use gpui::SharedString;
use smallvec::SmallVec;
use time::{OffsetDateTime, UtcOffset};
use util::command::new_smol_command;

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
pub(crate) const CHUNK_SIZE: usize = 60;

pub fn format_timestamp(timestamp: i64) -> String {
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

// todo! change to repo path
// move to repo as well
pub async fn commit_count(worktree_path: &PathBuf) -> Result<usize> {
    let git_log_output = new_smol_command("git")
        .current_dir(worktree_path)
        .arg("rev-list")
        .arg("--all")
        .arg("--count")
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&git_log_output.stdout);
    Ok(stdout.trim().parse::<usize>()?)
}

// todo: This function should be on a background thread, and it should return a chunk of commits at a time
// we should also be able to specify the order
// todo: Make this function work over collab as well
pub async fn load_commits(
    chunk_position: usize,
    worktree_path: PathBuf, //todo! Change to repo path
) -> Result<Vec<CommitData>> {
    let start = chunk_position * CHUNK_SIZE;

    let git_log_output = new_smol_command("git")
        .current_dir(worktree_path)
        .arg("log")
        .arg("--all")
        .arg(COMMIT_FORMAT)
        .arg("--date-order")
        .arg(format!("--skip={start}"))
        .arg(format!("--max-count={}", CHUNK_SIZE))
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

            Some(CommitData {
                author_name: SharedString::new(*author_name),
                _author_email: SharedString::new(*author_email),
                sha: Oid::from_str(sha).ok()?,
                parents,
                commit_timestamp: format_timestamp(commit_timestamp.parse().ok()?).into(), //todo!
                subject: SharedString::new(*summary),                                      // todo!
                _ref_names: parts
                    .get(7)
                    .filter(|ref_name| !ref_name.is_empty())
                    .map(|ref_names| ref_names.split(", ").map(SharedString::new).collect())
                    .unwrap_or_default(),
            })
        })
        .collect::<Vec<_>>())
}

/// Commit data needed for the graph
#[derive(Debug)]
pub struct CommitData {
    pub sha: Oid,
    /// Most commits have a single parent, so we use a small vec to avoid allocations
    pub parents: smallvec::SmallVec<[Oid; 1]>,
    pub author_name: SharedString,
    pub _author_email: SharedString,
    pub commit_timestamp: SharedString,
    pub subject: SharedString,
    pub _ref_names: Vec<SharedString>,
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

// todo! On accent colors updating it's len we need to update lane colors to use different indices
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

#[derive(Debug)]
pub struct CommitEntry {
    pub data: CommitData,
    pub lane: usize,
    pub color_idx: usize,
    pub lines: Vec<GraphLine>,
}

type ActiveLaneIdx = usize;

pub struct GitGraph {
    lane_states: SmallVec<[LaneState; 8]>,
    lane_colors: HashMap<ActiveLaneIdx, BranchColor>,
    next_color: BranchColor,
    accent_colors_count: usize,
    pub commits: Vec<Rc<CommitEntry>>,
    pub max_commit_count: usize,
    pub max_lanes: usize,
}

impl GitGraph {
    pub fn new(accent_colors_count: usize) -> Self {
        GitGraph {
            lane_states: SmallVec::default(),
            lane_colors: HashMap::default(),
            next_color: BranchColor(0),
            accent_colors_count,
            commits: Vec::default(),
            max_commit_count: 0,
            max_lanes: 0,
        }
    }

    pub fn clear(&mut self) {
        self.lane_states.clear();
        self.lane_colors.clear();
        self.next_color = BranchColor(0);
        self.commits.clear();
        self.max_lanes = 0;
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
        let accent_colors_count = self.accent_colors_count;
        *self.lane_colors.entry(lane_idx).or_insert_with(|| {
            let color_idx = self.next_color;
            self.next_color = BranchColor((self.next_color.0 + 1) % accent_colors_count as u8);
            color_idx
        })
    }

    pub(crate) fn add_commits(&mut self, commits: Vec<CommitData>) {
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
                    // base commit
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

            self.commits.push(Rc::new(CommitEntry {
                data: commit,
                lane: commit_lane,
                color_idx: commit_color.0 as usize,
                lines,
            }));
        }
    }
}
