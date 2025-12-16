use gpui::{Entity, Img, SharedString};
use language::Buffer;
use collections::FxHasher;
use project::ProjectPath;
use std::{
    hash::{Hash, Hasher},
    ops::Range,
    path::Path,
    sync::Arc,
};
use text::{Anchor as TextAnchor, BufferId};
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
    pub blame: PatchValue<Arc<str>>,
    pub location_label: PatchValue<Arc<str>>,
    pub path_label: PatchValue<Arc<str>>,
    pub path_segments: PatchValue<Arc<[Arc<str>]>>,
    pub file_name: PatchValue<Arc<str>>,
}

#[derive(Clone)]
pub enum QuickMatchKind {
    Buffer {
        buffer: Entity<Buffer>,
        project_path: Option<ProjectPath>,
        ranges: Vec<Range<TextAnchor>>,
        buffer_id: BufferId,
        position: Option<(u32, u32)>,
        position_end: Option<(u32, u32)>,
    },
    ProjectPath {
        project_path: ProjectPath,
    },
    GitCommit {
        repo_workdir: Arc<Path>,
        sha: Arc<str>,
        branch: Option<Arc<str>>,
        commit_timestamp: i64,
    },
}

#[derive(Clone)]
pub struct QuickMatch {
    pub id: MatchId,
    pub key: MatchKey,
    pub source_id: Arc<str>,
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
    pub blame: Option<Arc<str>>,
    pub kind: QuickMatchKind,
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
            buffer_id, position, ..
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

pub fn compute_group_key_for_project_path(source_id: &Arc<str>, project_path: &ProjectPath) -> GroupKey {
    let mut hasher = FxHasher::default();
    hash_part(&mut hasher, source_id);
    hash_part(&mut hasher, b"group");
    hash_part(&mut hasher, &project_path.worktree_id.to_proto());
    hash_part(&mut hasher, &project_path.path.as_unix_str());
    GroupKey(hasher.finish())
}

impl QuickMatch {
    pub fn buffer(&self) -> Option<&Entity<Buffer>> {
        match &self.kind {
            QuickMatchKind::Buffer { buffer, .. } => Some(buffer),
            _ => None,
        }
    }

    pub fn ranges(&self) -> Option<&[Range<TextAnchor>]> {
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

    pub fn position_end(&self) -> Option<(u32, u32)> {
        match &self.kind {
            QuickMatchKind::Buffer { position_end, .. } => *position_end,
            _ => None,
        }
    }

    pub fn project_path(&self) -> Option<&ProjectPath> {
        match &self.kind {
            QuickMatchKind::ProjectPath { project_path } => Some(project_path),
            QuickMatchKind::Buffer { project_path, .. } => project_path.as_ref(),
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
                    changed = true;
                }
            }
            PatchValue::SetTo(value) => {
                if self.snippet.as_ref() != Some(&value) {
                    self.first_line_snippet = value.lines().next().map(Arc::<str>::from);
                    self.snippet = Some(value);
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

