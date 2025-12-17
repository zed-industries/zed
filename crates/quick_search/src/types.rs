use collections::FxHasher;
use gpui::{Img, SharedString};
use language::HighlightId;
use project::ProjectPath;
use std::{
    hash::{Hash, Hasher},
    ops::Range,
    path,
    path::Path,
    sync::Arc,
};
use text::ToOffset;
use text::{Anchor as TextAnchor, BufferId};
use text::Point;
use ui::IconName;

pub type MatchId = u64;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MatchKey(pub u64);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GroupKey(pub u64);

#[derive(Clone)]
pub struct GroupHeader {
    pub icon_name: IconName,
    pub icon_path: Option<SharedString>,
    pub title: Arc<str>,
    pub subtitle: Option<Arc<str>>,
}

#[derive(Clone)]
pub struct GroupInfo {
    pub key: GroupKey,
    pub header: GroupHeader,
}

#[derive(Clone, Debug, Default)]
pub enum PatchValue<T> {
    #[default]
    Unchanged,
    #[allow(dead_code)]
    Clear,
    SetTo(T),
}

#[derive(Clone, Default)]
pub struct QuickMatchPatch {
    pub snippet: PatchValue<Arc<str>>,
    pub snippet_syntax_highlights: PatchValue<Arc<[(Range<usize>, HighlightId)]>>,
    pub blame: PatchValue<Arc<str>>,
    pub location_label: PatchValue<Arc<str>>,
    pub path_label: PatchValue<Arc<str>>,
    pub path_segments: PatchValue<Arc<[Arc<str>]>>,
    pub file_name: PatchValue<Arc<str>>,
}

#[derive(Clone)]
pub enum QuickMatchKind {
    Buffer {
        buffer_id: BufferId,
        ranges: Vec<Range<Point>>,
        position: Option<(u32, u32)>,
    },
    ProjectPath {
        project_path: ProjectPath,
    },
    GitCommit {
        repo_workdir: Arc<Path>,
        sha: Arc<str>,
        subject: Arc<str>,
        author: Arc<str>,
        repo_label: Arc<str>,
        branch: Option<Arc<str>>,
        commit_timestamp: i64,
    },
}

#[derive(Clone)]
pub enum MatchAction {
    OpenProjectPath {
        project_path: ProjectPath,
        point_range: Option<Range<Point>>,
    },
    OpenGitCommit {
        repo_workdir: Arc<Path>,
        sha: Arc<str>,
    },
    Dismiss,
}

#[derive(Clone)]
pub struct QuickMatch {
    pub id: MatchId,
    pub key: MatchKey,
    pub source_id: Arc<str>,
    pub action: MatchAction,
    pub group: Option<Arc<GroupInfo>>,
    pub path_label: Arc<str>,
    pub display_path: Arc<str>,
    pub display_path_positions: Option<Arc<[usize]>>,
    pub path_segments: Arc<[Arc<str>]>,
    pub file_name: Arc<str>,
    pub file_name_positions: Option<Arc<[usize]>>,
    pub location_label: Option<Arc<str>>,
    pub snippet: Option<Arc<str>>,
    pub first_line_snippet: Option<Arc<str>>,
    pub snippet_match_positions: Option<Arc<[Range<usize>]>>,
    pub snippet_syntax_highlights: Option<Arc<[(Range<usize>, HighlightId)]>>,
    pub blame: Option<Arc<str>>,
    pub kind: QuickMatchKind,
}

pub struct QuickMatchBuilder {
    match_item: QuickMatch,
}

impl QuickMatchBuilder {
    pub fn new(source_id: Arc<str>, kind: QuickMatchKind) -> Self {
        let action = match &kind {
            QuickMatchKind::ProjectPath { project_path } => MatchAction::OpenProjectPath {
                project_path: project_path.clone(),
                point_range: None,
            },
            QuickMatchKind::Buffer { .. } => MatchAction::Dismiss,
            QuickMatchKind::GitCommit {
                repo_workdir, sha, ..
            } => MatchAction::OpenGitCommit {
                repo_workdir: repo_workdir.clone(),
                sha: sha.clone(),
            },
        };
        Self {
            match_item: QuickMatch {
                id: 0,
                key: MatchKey(0),
                source_id,
                action,
                group: None,
                path_label: Arc::<str>::from(""),
                display_path: Arc::<str>::from(""),
                display_path_positions: None,
                path_segments: Arc::from(Box::<[Arc<str>]>::default()),
                file_name: Arc::<str>::from(""),
                file_name_positions: None,
                location_label: None,
                snippet: None,
                first_line_snippet: None,
                snippet_match_positions: None,
                snippet_syntax_highlights: None,
                blame: None,
                kind,
            },
        }
    }

    pub fn action(mut self, action: MatchAction) -> Self {
        self.match_item.action = action;
        self
    }

    pub fn group(mut self, group: Option<Arc<GroupInfo>>) -> Self {
        self.match_item.group = group;
        self
    }

    pub fn path_label(mut self, path_label: Arc<str>) -> Self {
        self.match_item.path_label = path_label;
        self
    }

    pub fn display_path(mut self, display_path: Arc<str>) -> Self {
        self.match_item.display_path = display_path;
        self
    }

    pub fn display_path_positions(mut self, positions: Option<Arc<[usize]>>) -> Self {
        self.match_item.display_path_positions = positions;
        self
    }

    pub fn path_segments(mut self, path_segments: Arc<[Arc<str>]>) -> Self {
        self.match_item.path_segments = path_segments;
        self
    }

    pub fn path_segments_from_label(mut self) -> Self {
        self.match_item.path_segments = split_path_segments(&self.match_item.path_label);
        self
    }

    pub fn file_name(mut self, file_name: Arc<str>) -> Self {
        self.match_item.file_name = file_name;
        self
    }

    pub fn file_name_positions(mut self, positions: Option<Arc<[usize]>>) -> Self {
        self.match_item.file_name_positions = positions;
        self
    }

    pub fn location_label(mut self, label: Option<Arc<str>>) -> Self {
        self.match_item.location_label = label;
        self
    }

    pub fn snippet(mut self, snippet: Option<Arc<str>>) -> Self {
        self.match_item.first_line_snippet = snippet
            .as_deref()
            .and_then(|snippet| snippet.lines().next())
            .map(Arc::<str>::from);
        self.match_item.snippet = snippet;
        self
    }

    pub fn first_line_snippet(mut self, first_line_snippet: Option<Arc<str>>) -> Self {
        self.match_item.first_line_snippet = first_line_snippet;
        self
    }

    pub fn snippet_match_positions(mut self, positions: Option<Arc<[Range<usize>]>>) -> Self {
        self.match_item.snippet_match_positions = positions;
        self
    }

    pub fn snippet_syntax_highlights(
        mut self,
        highlights: Option<Arc<[(Range<usize>, HighlightId)]>>,
    ) -> Self {
        self.match_item.snippet_syntax_highlights = highlights;
        self
    }

    pub fn build(self) -> QuickMatch {
        self.match_item
    }
}

fn hash_part<H: Hasher, T: Hash>(hasher: &mut H, value: &T) {
    value.hash(hasher);
    0u8.hash(hasher);
}

pub fn compute_match_key(quick_match: &QuickMatch) -> MatchKey {
    let mut hasher = FxHasher::default();
    hash_part(&mut hasher, &quick_match.source_id);

    match &quick_match.kind {
        QuickMatchKind::ProjectPath { project_path } => {
            hash_part(&mut hasher, b"path");
            hash_part(&mut hasher, &project_path.worktree_id.to_proto());
            hash_part(&mut hasher, &project_path.path.as_unix_str());
        }
        QuickMatchKind::Buffer {
            buffer_id,
            position,
            ..
        } => {
            hash_part(&mut hasher, b"buf");
            let id_u64: u64 = (*buffer_id).into();
            hash_part(&mut hasher, &id_u64);
            let row: u32 = position.map(|(row, _)| row).unwrap_or(0);
            hash_part(&mut hasher, &row);
        }
        QuickMatchKind::GitCommit {
            repo_workdir, sha, ..
        } => {
            hash_part(&mut hasher, b"commit");
            hash_part(&mut hasher, &repo_workdir.to_string_lossy());
            hash_part(&mut hasher, sha);
        }
    }

    MatchKey(hasher.finish())
}

pub fn compute_group_key_for_project_path(
    source_id: &Arc<str>,
    project_path: &ProjectPath,
) -> GroupKey {
    let mut hasher = FxHasher::default();
    hash_part(&mut hasher, source_id);
    hash_part(&mut hasher, b"group");
    hash_part(&mut hasher, &project_path.worktree_id.to_proto());
    hash_part(&mut hasher, &project_path.path.as_unix_str());
    GroupKey(hasher.finish())
}

impl QuickMatch {
    pub fn ranges(&self) -> Option<&[Range<Point>]> {
        match &self.kind {
            QuickMatchKind::Buffer { ranges, .. } => Some(ranges),
            _ => None,
        }
    }

    pub fn buffer_id(&self) -> Option<BufferId> {
        match &self.kind {
            QuickMatchKind::Buffer { buffer_id, .. } => Some(*buffer_id),
            _ => None,
        }
    }

    pub fn position(&self) -> Option<(u32, u32)> {
        match &self.kind {
            QuickMatchKind::Buffer { position, .. } => *position,
            _ => None,
        }
    }

    pub fn project_path(&self) -> Option<&ProjectPath> {
        match &self.action {
            MatchAction::OpenProjectPath { project_path, .. } => Some(project_path),
            _ => None,
        }
    }

    pub fn is_likely_binary(&self) -> bool {
        let extension = self
            .file_name
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        Img::extensions().contains(&extension.as_str()) && !extension.contains("svg")
    }

    pub fn apply_patch(&mut self, patch: QuickMatchPatch) -> bool {
        let mut changed = false;

        match patch.snippet {
            PatchValue::Unchanged => {}
            PatchValue::Clear => {
                if self.snippet.is_some() {
                    self.snippet = None;
                    self.first_line_snippet = None;
                    self.snippet_match_positions = None;
                    self.snippet_syntax_highlights = None;
                    changed = true;
                }
            }
            PatchValue::SetTo(value) => {
                if self.snippet.as_ref() != Some(&value) {
                    self.first_line_snippet = value.lines().next().map(Arc::<str>::from);
                    self.snippet = Some(value);
                    self.snippet_match_positions = None;
                    self.snippet_syntax_highlights = None;
                    changed = true;
                }
            }
        }

        match patch.snippet_syntax_highlights {
            PatchValue::Unchanged => {}
            PatchValue::Clear => {
                if self.snippet_syntax_highlights.is_some() {
                    self.snippet_syntax_highlights = None;
                    changed = true;
                }
            }
            PatchValue::SetTo(value) => {
                if self.snippet_syntax_highlights.as_ref() != Some(&value) {
                    self.snippet_syntax_highlights = Some(value);
                    changed = true;
                }
            }
        }

        match patch.blame {
            PatchValue::Unchanged => {}
            PatchValue::Clear => {
                if self.blame.is_some() {
                    self.blame = None;
                    changed = true;
                }
            }
            PatchValue::SetTo(value) => {
                if self.blame.as_ref() != Some(&value) {
                    self.blame = Some(value);
                    changed = true;
                }
            }
        }

        match patch.location_label {
            PatchValue::Unchanged => {}
            PatchValue::Clear => {
                if self.location_label.is_some() {
                    self.location_label = None;
                    changed = true;
                }
            }
            PatchValue::SetTo(value) => {
                if self.location_label.as_ref() != Some(&value) {
                    self.location_label = Some(value);
                    changed = true;
                }
            }
        }

        match patch.path_label {
            PatchValue::Unchanged => {}
            PatchValue::Clear => {}
            PatchValue::SetTo(value) => {
                if self.path_label != value {
                    self.path_label = value;
                    changed = true;
                }
            }
        }

        match patch.path_segments {
            PatchValue::Unchanged => {}
            PatchValue::Clear => {}
            PatchValue::SetTo(value) => {
                if self.path_segments != value {
                    self.path_segments = value;
                    changed = true;
                }
            }
        }

        match patch.file_name {
            PatchValue::Unchanged => {}
            PatchValue::Clear => {}
            PatchValue::SetTo(value) => {
                if self.file_name != value {
                    self.file_name = value;
                    changed = true;
                }
            }
        }

        changed
    }
}

pub fn split_path_segments(path_label: &str) -> Arc<[Arc<str>]> {
    if path_label.is_empty() {
        return Arc::from(Box::<[Arc<str>]>::default());
    }
    let mut segments: Vec<Arc<str>> = path_label
        .split(|c| c == '/' || c == path::MAIN_SEPARATOR)
        .filter(|part| !part.is_empty())
        .map(Arc::<str>::from)
        .collect();
    if segments.is_empty() {
        segments.push(Arc::<str>::from(path_label));
    }
    Arc::from(segments.into_boxed_slice())
}

pub fn point_range_to_anchor_range(
    range: Range<Point>,
    buffer: &text::BufferSnapshot,
) -> Range<TextAnchor> {
    let start_offset = range.start.to_offset(buffer);
    let end_offset = range.end.to_offset(buffer);
    if start_offset == end_offset {
        buffer.anchor_before(start_offset)..buffer.anchor_before(end_offset)
    } else {
        buffer.anchor_after(range.start)..buffer.anchor_before(range.end)
    }
}
