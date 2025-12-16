use gpui::{Entity, Img};
use language::Buffer;
use project::ProjectPath;
use std::{ops::Range, path::Path, sync::Arc};
use text::{Anchor as TextAnchor, BufferId};

pub type MatchId = u64;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MatchKey(pub u64);

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

pub fn compute_match_key(quick_match: &QuickMatch) -> MatchKey {
    use std::hash::{Hash, Hasher};

    struct FnvHasher(u64);

    impl FnvHasher {
        const OFFSET: u64 = 0xcbf29ce484222325;
        const PRIME: u64 = 0x100000001b3;

        fn new() -> Self {
            Self(Self::OFFSET)
        }
    }

    impl Hasher for FnvHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.0 ^= *byte as u64;
                self.0 = self.0.wrapping_mul(Self::PRIME);
            }
        }
    }

    let mut hasher = FnvHasher::new();
    quick_match.source_id.hash(&mut hasher);
    0u8.hash(&mut hasher);

    match &quick_match.kind {
        QuickMatchKind::ProjectPath { project_path } => {
            b"path".hash(&mut hasher);
            project_path.worktree_id.to_proto().hash(&mut hasher);
            project_path.path.as_unix_str().hash(&mut hasher);
        }
        QuickMatchKind::Buffer {
            buffer_id, position, ..
        } => {
            b"buf".hash(&mut hasher);
            let id_u64: u64 = (*buffer_id).into();
            id_u64.hash(&mut hasher);
            let row: u32 = position.map(|(row, _)| row).unwrap_or(0);
            row.hash(&mut hasher);
        }
        QuickMatchKind::GitCommit {
            repo_workdir, sha, ..
        } => {
            b"commit".hash(&mut hasher);
            repo_workdir.to_string_lossy().hash(&mut hasher);
            sha.hash(&mut hasher);
        }
    }

    MatchKey(hasher.finish())
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

