use language::{Language, LanguageId};
use project::ProjectEntryId;
use std::ops::Range;
use std::sync::Arc;
use std::{borrow::Cow, path::Path};
use text::{Bias, BufferId, Rope};
use util::paths::{path_ends_with, strip_path_suffix};
use util::rel_path::RelPath;

use crate::outline::OutlineDeclaration;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier {
    pub name: Arc<str>,
    pub language_id: LanguageId,
}

slotmap::new_key_type! {
    pub struct DeclarationId;
}

#[derive(Debug, Clone)]
pub enum Declaration {
    File {
        project_entry_id: ProjectEntryId,
        declaration: FileDeclaration,
        cached_path: CachedDeclarationPath,
    },
    Buffer {
        project_entry_id: ProjectEntryId,
        buffer_id: BufferId,
        rope: Rope,
        declaration: BufferDeclaration,
        cached_path: CachedDeclarationPath,
    },
}

const ITEM_TEXT_TRUNCATION_LENGTH: usize = 1024;

impl Declaration {
    pub fn identifier(&self) -> &Identifier {
        match self {
            Declaration::File { declaration, .. } => &declaration.identifier,
            Declaration::Buffer { declaration, .. } => &declaration.identifier,
        }
    }

    pub fn parent(&self) -> Option<DeclarationId> {
        match self {
            Declaration::File { declaration, .. } => declaration.parent,
            Declaration::Buffer { declaration, .. } => declaration.parent,
        }
    }

    pub fn as_buffer(&self) -> Option<&BufferDeclaration> {
        match self {
            Declaration::File { .. } => None,
            Declaration::Buffer { declaration, .. } => Some(declaration),
        }
    }

    pub fn as_file(&self) -> Option<&FileDeclaration> {
        match self {
            Declaration::Buffer { .. } => None,
            Declaration::File { declaration, .. } => Some(declaration),
        }
    }

    pub fn project_entry_id(&self) -> ProjectEntryId {
        match self {
            Declaration::File {
                project_entry_id, ..
            } => *project_entry_id,
            Declaration::Buffer {
                project_entry_id, ..
            } => *project_entry_id,
        }
    }

    pub fn cached_path(&self) -> &CachedDeclarationPath {
        match self {
            Declaration::File { cached_path, .. } => cached_path,
            Declaration::Buffer { cached_path, .. } => cached_path,
        }
    }

    pub fn item_range(&self) -> Range<usize> {
        match self {
            Declaration::File { declaration, .. } => declaration.item_range.clone(),
            Declaration::Buffer { declaration, .. } => declaration.item_range.clone(),
        }
    }

    pub fn item_text(&self) -> (Cow<'_, str>, bool) {
        match self {
            Declaration::File { declaration, .. } => (
                declaration.text.as_ref().into(),
                declaration.text_is_truncated,
            ),
            Declaration::Buffer {
                rope, declaration, ..
            } => (
                rope.chunks_in_range(declaration.item_range.clone())
                    .collect::<Cow<str>>(),
                declaration.item_range_is_truncated,
            ),
        }
    }

    pub fn signature_text(&self) -> (Cow<'_, str>, bool) {
        match self {
            Declaration::File { declaration, .. } => (
                declaration.text[self.signature_range_in_item_text()].into(),
                declaration.signature_is_truncated,
            ),
            Declaration::Buffer {
                rope, declaration, ..
            } => (
                rope.chunks_in_range(declaration.signature_range.clone())
                    .collect::<Cow<str>>(),
                declaration.signature_range_is_truncated,
            ),
        }
    }

    pub fn signature_range(&self) -> Range<usize> {
        match self {
            Declaration::File { declaration, .. } => declaration.signature_range.clone(),
            Declaration::Buffer { declaration, .. } => declaration.signature_range.clone(),
        }
    }

    pub fn signature_range_in_item_text(&self) -> Range<usize> {
        let signature_range = self.signature_range();
        let item_range = self.item_range();
        signature_range.start.saturating_sub(item_range.start)
            ..(signature_range.end.saturating_sub(item_range.start)).min(item_range.len())
    }
}

fn expand_range_to_line_boundaries_and_truncate(
    range: &Range<usize>,
    limit: usize,
    rope: &Rope,
) -> (Range<usize>, bool) {
    let mut point_range = rope.offset_to_point(range.start)..rope.offset_to_point(range.end);
    point_range.start.column = 0;
    point_range.end.row += 1;
    point_range.end.column = 0;

    let mut item_range =
        rope.point_to_offset(point_range.start)..rope.point_to_offset(point_range.end);
    let is_truncated = item_range.len() > limit;
    if is_truncated {
        item_range.end = item_range.start + limit;
    }
    item_range.end = rope.clip_offset(item_range.end, Bias::Left);
    (item_range, is_truncated)
}

#[derive(Debug, Clone)]
pub struct FileDeclaration {
    pub parent: Option<DeclarationId>,
    pub identifier: Identifier,
    /// offset range of the declaration in the file, expanded to line boundaries and truncated
    pub item_range: Range<usize>,
    /// text of `item_range`
    pub text: Arc<str>,
    /// whether `text` was truncated
    pub text_is_truncated: bool,
    /// offset range of the signature in the file, expanded to line boundaries and truncated
    pub signature_range: Range<usize>,
    /// whether `signature` was truncated
    pub signature_is_truncated: bool,
}

impl FileDeclaration {
    pub fn from_outline(declaration: OutlineDeclaration, rope: &Rope) -> FileDeclaration {
        let (item_range_in_file, text_is_truncated) = expand_range_to_line_boundaries_and_truncate(
            &declaration.item_range,
            ITEM_TEXT_TRUNCATION_LENGTH,
            rope,
        );

        let (mut signature_range_in_file, mut signature_is_truncated) =
            expand_range_to_line_boundaries_and_truncate(
                &declaration.signature_range,
                ITEM_TEXT_TRUNCATION_LENGTH,
                rope,
            );

        if signature_range_in_file.start < item_range_in_file.start {
            signature_range_in_file.start = item_range_in_file.start;
            signature_is_truncated = true;
        }
        if signature_range_in_file.end > item_range_in_file.end {
            signature_range_in_file.end = item_range_in_file.end;
            signature_is_truncated = true;
        }

        FileDeclaration {
            parent: None,
            identifier: declaration.identifier,
            signature_range: signature_range_in_file,
            signature_is_truncated,
            text: rope
                .chunks_in_range(item_range_in_file.clone())
                .collect::<String>()
                .into(),
            text_is_truncated,
            item_range: item_range_in_file,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferDeclaration {
    pub parent: Option<DeclarationId>,
    pub identifier: Identifier,
    pub item_range: Range<usize>,
    pub item_range_is_truncated: bool,
    pub signature_range: Range<usize>,
    pub signature_range_is_truncated: bool,
}

impl BufferDeclaration {
    pub fn from_outline(declaration: OutlineDeclaration, rope: &Rope) -> Self {
        let (item_range, item_range_is_truncated) = expand_range_to_line_boundaries_and_truncate(
            &declaration.item_range,
            ITEM_TEXT_TRUNCATION_LENGTH,
            rope,
        );
        let (signature_range, signature_range_is_truncated) =
            expand_range_to_line_boundaries_and_truncate(
                &declaration.signature_range,
                ITEM_TEXT_TRUNCATION_LENGTH,
                rope,
            );
        Self {
            parent: None,
            identifier: declaration.identifier,
            item_range,
            item_range_is_truncated,
            signature_range,
            signature_range_is_truncated,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CachedDeclarationPath {
    pub worktree_abs_path: Arc<Path>,
    pub rel_path: Arc<RelPath>,
    /// The relative path of the file, possibly stripped according to `import_path_strip_regex`.
    pub rel_path_after_regex_stripping: Arc<RelPath>,
}

impl CachedDeclarationPath {
    pub fn new(
        worktree_abs_path: Arc<Path>,
        path: &Arc<RelPath>,
        language: Option<&Arc<Language>>,
    ) -> Self {
        let rel_path = path.clone();
        let rel_path_after_regex_stripping = if let Some(language) = language
            && let Some(strip_regex) = language.config().import_path_strip_regex.as_ref()
            && let Ok(stripped) = RelPath::unix(&Path::new(
                strip_regex.replace_all(rel_path.as_unix_str(), "").as_ref(),
            )) {
            Arc::from(stripped)
        } else {
            rel_path.clone()
        };
        CachedDeclarationPath {
            worktree_abs_path,
            rel_path,
            rel_path_after_regex_stripping,
        }
    }

    #[cfg(test)]
    pub fn new_for_test(worktree_abs_path: &str, rel_path: &str) -> Self {
        let rel_path: Arc<RelPath> = util::rel_path::rel_path(rel_path).into();
        CachedDeclarationPath {
            worktree_abs_path: std::path::PathBuf::from(worktree_abs_path).into(),
            rel_path_after_regex_stripping: rel_path.clone(),
            rel_path,
        }
    }

    pub fn ends_with_posix_path(&self, path: &Path) -> bool {
        if path.as_os_str().len() <= self.rel_path_after_regex_stripping.as_unix_str().len() {
            path_ends_with(self.rel_path_after_regex_stripping.as_std_path(), path)
        } else {
            if let Some(remaining) =
                strip_path_suffix(path, self.rel_path_after_regex_stripping.as_std_path())
            {
                path_ends_with(&self.worktree_abs_path, remaining)
            } else {
                false
            }
        }
    }

    pub fn equals_absolute_path(&self, path: &Path) -> bool {
        if let Some(remaining) =
            strip_path_suffix(path, &self.rel_path_after_regex_stripping.as_std_path())
        {
            self.worktree_abs_path.as_ref() == remaining
        } else {
            false
        }
    }
}
