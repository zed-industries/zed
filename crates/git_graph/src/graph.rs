use std::{ops::Range, rc::Rc, sync::Arc};

use collections::HashMap;
use git::{
    Oid,
    repository::{GRAPH_CHUNK_SIZE, InitialGraphCommitData},
};
use smallvec::{SmallVec, smallvec};
use time::{OffsetDateTime, UtcOffset};

pub(crate) const CHUNK_SIZE: usize = GRAPH_CHUNK_SIZE;

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

// todo! On accent colors updating it's len we need to update lane colors to use different indices
#[derive(Copy, Clone, Debug)]
struct BranchColor(u8);

#[derive(Debug)]
enum LaneState {
    Empty,
    Active {
        child: Oid,
        parent: Oid,
        color: BranchColor,
        starting_row: usize,
        starting_col: usize,
        destination_column: Option<usize>,
        segments: SmallVec<[CommitLineSegment; 1]>,
    },
}

impl LaneState {
    fn to_commit_lines(&mut self, ending_row: usize, current_column: usize) -> Option<CommitLine> {
        let state = std::mem::replace(self, LaneState::Empty);

        match state {
            LaneState::Active {
                parent,
                child,
                color,
                starting_row,
                starting_col,
                destination_column,
                mut segments,
            } => Some(CommitLine {
                child,
                parent,
                child_column: starting_col,
                full_interval: starting_row..ending_row,
                color_idx: color.0 as usize,
                segments: {
                    match segments.last_mut() {
                        Some(CommitLineSegment::Straight { to_row }) if *to_row == usize::MAX => {
                            if destination_column.is_some_and(|dest| dest != current_column) {
                                *to_row = ending_row - 1;

                                let curved_line = CommitLineSegment::Curve {
                                    to_column: destination_column.unwrap(),
                                    on_row: ending_row,
                                    curve_kind: CurveKind::Checkout,
                                };

                                if *to_row == starting_row {
                                    let last_index = segments.len() - 1;
                                    segments[last_index] = curved_line;
                                } else {
                                    segments.push(curved_line);
                                }
                            } else {
                                *to_row = ending_row;
                            }
                        }
                        Some(CommitLineSegment::Curve {
                            on_row, to_column, ..
                        }) if *on_row == usize::MAX => {
                            // todo! remove this in the future
                            assert!(destination_column.is_none_or(|column| column == *to_column));
                            *on_row = ending_row;
                        }
                        Some(CommitLineSegment::Curve {
                            on_row, to_column, ..
                        }) => {
                            assert_eq!(*to_column, current_column);
                            if *on_row < ending_row {
                                segments.push(CommitLineSegment::Straight { to_row: ending_row });
                            }
                        }
                        _ => {}
                    }

                    segments
                },
            }),
            LaneState::Empty => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            LaneState::Empty => true,
            LaneState::Active { .. } => false,
        }
    }
}

pub struct CommitEntry {
    pub data: Arc<InitialGraphCommitData>,
    pub lane: usize,
    pub color_idx: usize,
}

type ActiveLaneIdx = usize;

pub(crate) enum AllCommitCount {
    NotLoaded,
    Loaded(usize),
}

#[derive(Debug)]
pub(crate) enum CurveKind {
    Merge,
    Checkout,
}

#[derive(Debug)]
pub(crate) enum CommitLineSegment {
    Straight {
        to_row: usize,
    },
    Curve {
        to_column: usize,
        on_row: usize,
        curve_kind: CurveKind,
    },
}

#[derive(Debug)]
pub struct CommitLine {
    pub child: Oid,
    pub child_column: usize,
    pub parent: Oid,
    pub full_interval: Range<usize>,
    pub color_idx: usize,
    pub segments: SmallVec<[CommitLineSegment; 1]>,
}

impl CommitLine {
    pub fn get_first_visible_segment_idx(
        &self,
        first_visible_row: usize,
    ) -> Option<(usize, usize)> {
        if first_visible_row > self.full_interval.end {
            return None;
        } else if first_visible_row <= self.full_interval.start {
            return Some((0, self.child_column));
        }

        let mut current_column = self.child_column;

        for (idx, segment) in self.segments.iter().enumerate() {
            match segment {
                CommitLineSegment::Straight { to_row } => {
                    if *to_row >= first_visible_row {
                        return Some((idx, current_column));
                    }
                }
                CommitLineSegment::Curve {
                    to_column, on_row, ..
                } => {
                    if *on_row >= first_visible_row {
                        return Some((idx, current_column));
                    }
                    current_column = *to_column;
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CommitLineKey {
    child: Oid,
    parent: Oid,
}

pub struct GitGraph {
    lane_states: SmallVec<[LaneState; 8]>,
    lane_colors: HashMap<ActiveLaneIdx, BranchColor>,
    parent_to_lane: HashMap<Oid, SmallVec<[(usize, Option<usize>); 1]>>,
    next_color: BranchColor,
    accent_colors_count: usize,
    pub commits: Vec<Rc<CommitEntry>>,
    pub max_commit_count: AllCommitCount,
    pub max_lanes: usize,
    pub lines: Vec<Rc<CommitLine>>,
    active_commit_lines: HashMap<CommitLineKey, usize>,
    active_commit_lines_by_parent: HashMap<Oid, SmallVec<[usize; 1]>>,
}

impl GitGraph {
    pub fn new(accent_colors_count: usize) -> Self {
        GitGraph {
            lane_states: SmallVec::default(),
            lane_colors: HashMap::default(),
            parent_to_lane: HashMap::default(),
            next_color: BranchColor(0),
            accent_colors_count,
            commits: Vec::default(),
            max_commit_count: AllCommitCount::NotLoaded,
            max_lanes: 0,
            lines: Vec::default(),
            active_commit_lines: HashMap::default(),
            active_commit_lines_by_parent: HashMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.lane_states.clear();
        self.lane_colors.clear();
        self.parent_to_lane.clear();
        self.commits.clear();
        self.lines.clear();
        self.active_commit_lines.clear();
        self.active_commit_lines_by_parent.clear();
        self.next_color = BranchColor(0);
        self.max_commit_count = AllCommitCount::NotLoaded;
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

    pub(crate) fn add_commits(&mut self, commits: &[Arc<InitialGraphCommitData>]) {
        for commit in commits.into_iter() {
            let commit_row = self.commits.len();

            let commit_lane = self.parent_to_lane.get(&commit.sha).and_then(|lanes| {
                lanes.iter().find_map(|(lane_idx, dest)| {
                    if dest.is_none() {
                        Some(*lane_idx)
                    } else {
                        None
                    }
                })
            });

            if commit_lane.is_some() {
                if let Some(lanes) = self.parent_to_lane.remove(&commit.sha) {
                    for (column, _) in lanes {
                        if let Some(commit_line) =
                            self.lane_states[column].to_commit_lines(commit_row, column)
                        {
                            self.lines.push(Rc::new(commit_line));
                        }
                    }
                }
            }

            let commit_lane = commit_lane.unwrap_or_else(|| self.first_empty_lane_idx());
            let commit_color = self.get_lane_color(commit_lane);

            commit
                .parents
                .iter()
                .enumerate()
                .for_each(|(parent_idx, parent)| {
                    let parent_lane = self.parent_to_lane.get(parent).and_then(|lanes| {
                        lanes.iter().find_map(|(lane_idx, dest)| {
                            let final_destination = dest.unwrap_or(*lane_idx);
                            Some(final_destination)
                        })
                    });

                    if let Some(parent_lane) = parent_lane
                        && parent_lane != commit_lane
                    {
                        self.lane_states[commit_lane] = LaneState::Active {
                            child: commit.sha,
                            parent: *parent,
                            color: commit_color,
                            starting_row: commit_row,
                            starting_col: commit_lane,
                            destination_column: Some(parent_lane),
                            segments: smallvec![CommitLineSegment::Straight { to_row: usize::MAX }],
                        };

                        self.parent_to_lane
                            .entry(*parent)
                            .or_default()
                            .push((commit_lane, Some(parent_lane)));
                    } else if parent_idx == 0 {
                        self.lane_states[commit_lane] = LaneState::Active {
                            parent: *parent,
                            child: commit.sha,
                            color: commit_color,
                            starting_col: commit_lane,
                            starting_row: commit_row,
                            destination_column: None,
                            segments: smallvec![CommitLineSegment::Straight { to_row: usize::MAX }],
                        };

                        self.parent_to_lane
                            .entry(*parent)
                            .or_default()
                            .push((commit_lane, None));
                    } else {
                        let parent_lane = self.first_empty_lane_idx();
                        let parent_color = self.get_lane_color(parent_lane);

                        self.lane_states[parent_lane] = LaneState::Active {
                            parent: *parent,
                            child: commit.sha,
                            color: parent_color,
                            starting_col: commit_lane,
                            starting_row: commit_row,
                            destination_column: None,
                            segments: smallvec![CommitLineSegment::Curve {
                                to_column: parent_lane,
                                on_row: commit_row + 1,
                                curve_kind: CurveKind::Merge,
                            },],
                        };

                        self.parent_to_lane
                            .entry(*parent)
                            .or_default()
                            .push((parent_lane, None));
                    }
                });

            self.max_lanes = self.max_lanes.max(self.lane_states.len());

            self.commits.push(Rc::new(CommitEntry {
                data: commit.clone(),
                lane: commit_lane,
                color_idx: commit_color.0 as usize,
            }));
        }
    }
}
