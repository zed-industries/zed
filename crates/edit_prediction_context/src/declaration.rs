use language::LanguageId;
use project::ProjectEntryId;
use std::borrow::Cow;
use std::ops::Range;
use std::sync::Arc;
use text::{Bias, BufferId, Rope};

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
    },
    Buffer {
        project_entry_id: ProjectEntryId,
        buffer_id: BufferId,
        rope: Rope,
        declaration: BufferDeclaration,
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
