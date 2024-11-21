#[cfg(test)]
mod patch_tests;

use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use editor::{ExcerptRange, ProposedChangesEditor};
use gpui::{AppContext, AsyncAppContext, EventEmitter, Model, ModelContext, SharedString, Task};
use language::{AutoindentMode, Buffer};
use project::{Project, ProjectPath};
use rope::Rope;
use std::{
    cmp::{self, Ordering},
    ops::Range,
    path::Path,
    sync::Arc,
};
use text::{Anchor, AnchorRangeExt as _, Bias, OffsetRangeExt as _, Point};
use ui::ViewContext;
use util::ResultExt;

/// A set of patches that apply to a given project.
pub struct PatchStore {
    next_patch_id: usize,
    project: Model<Project>,
    entries: HashMap<PatchId, PatchStoreEntry>,
}

struct PatchStoreEntry {
    patch: LocatedPatch,
    locate_task: Option<Task<Result<()>>>,
    next_input: Option<AssistantPatch>,
}

pub enum PatchStoreEvent {
    PatchUpdated(PatchId),
    PatchRemoved(PatchId),
}

/// A unique identifier for a given patch within a specific `PatchStore`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PatchId(usize);

/// The raw data for a patch provided by an AI assistant.
#[derive(Clone, Debug)]
pub struct AssistantPatch {
    pub range: Range<Anchor>,
    pub title: SharedString,
    pub edits: Arc<[AssistantEdit]>,
    pub status: AssistantPatchStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AssistantPatchStatus {
    Pending,
    Ready,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssistantEdit {
    pub path: String,
    pub kind: AssistantEditKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssistantEditKind {
    Update {
        old_text: String,
        new_text: String,
        description: Option<String>,
    },
    Create {
        new_text: String,
        description: Option<String>,
    },
    InsertBefore {
        old_text: String,
        new_text: String,
        description: Option<String>,
    },
    InsertAfter {
        old_text: String,
        new_text: String,
        description: Option<String>,
    },
    Delete {
        old_text: String,
    },
}

#[derive(Clone, Debug)]
struct LocatedPatch {
    buffers: Vec<LocatedPatchBuffer>,
    input: AssistantPatch,
}

#[derive(Clone, Debug)]
struct LocatedPatchBuffer {
    path: Arc<Path>,
    content: Rope,
    edits: Vec<LocatedEdit>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LocatedEdit {
    range: Range<usize>,
    new_text: String,
    description: Option<String>,
    input_ix: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedPatch {
    title: SharedString,
    edit_groups: HashMap<Model<Buffer>, Vec<ResolvedEditGroup>>,
    pub errors: Vec<AssistantPatchResolutionError>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedEditGroup {
    context_range: Range<Anchor>,
    edits: Vec<ResolvedEdit>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedEdit {
    range: Range<Anchor>,
    new_text: String,
    description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssistantPatchResolutionError {
    pub edit_ix: usize,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchDirection {
    Up,
    Left,
    Diagonal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    cost: u32,
    direction: SearchDirection,
}

impl SearchState {
    fn new(cost: u32, direction: SearchDirection) -> Self {
        Self { cost, direction }
    }
}

struct SearchMatrix {
    cols: usize,
    data: Vec<SearchState>,
}

impl EventEmitter<PatchStoreEvent> for PatchStore {}

impl PatchStore {
    pub fn new(project: Model<Project>) -> Self {
        Self {
            next_patch_id: 0,
            project,
            entries: HashMap::default(),
        }
    }

    pub fn get(&self, id: PatchId) -> Option<&AssistantPatch> {
        Some(&self.entries.get(&id)?.patch.input)
    }

    pub fn insert(&mut self, patch: AssistantPatch, cx: &mut ModelContext<Self>) -> PatchId {
        let id = PatchId(self.next_patch_id);
        self.next_patch_id += 1;
        self.entries.insert(
            id,
            PatchStoreEntry {
                patch: LocatedPatch {
                    input: patch.clone(),
                    buffers: Vec::new(),
                },
                locate_task: None,
                next_input: None,
            },
        );
        cx.emit(PatchStoreEvent::PatchUpdated(id));
        self.update(id, patch, cx).unwrap();
        id
    }

    pub fn update(
        &mut self,
        id: PatchId,
        patch: AssistantPatch,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let Some(entry) = self.entries.get_mut(&id) else {
            return Err(anyhow!("no such patch"));
        };

        if entry.locate_task.is_some() {
            entry.next_input = Some(patch);
        } else {
            entry.locate_task =
                Self::update_internal(id, patch, entry.patch.clone(), self.project.clone(), cx);
        }

        Ok(())
    }

    fn update_internal(
        id: PatchId,
        patch: AssistantPatch,
        prev_patch: LocatedPatch,
        project: Model<Project>,
        cx: &mut ModelContext<PatchStore>,
    ) -> Option<Task<std::result::Result<(), anyhow::Error>>> {
        Some(cx.spawn(|this, mut cx| async move {
            let (located_patch, patch_did_change) =
                Self::locate_patch(patch, project.clone(), prev_patch, &mut cx).await?;
            this.update(&mut cx, |this, cx| {
                if let Some(entry) = this.entries.get_mut(&id) {
                    entry.patch = located_patch;
                    if patch_did_change {
                        cx.emit(PatchStoreEvent::PatchUpdated(id));
                    }
                    if let Some(input) = entry.next_input.take() {
                        entry.locate_task =
                            Self::update_internal(id, input, entry.patch.clone(), project, cx);
                    } else {
                        entry.locate_task = None;
                    }
                }
            })
        }))
    }

    pub fn remove(&mut self, id: PatchId, cx: &mut ModelContext<Self>) {
        if self.entries.remove(&id).is_some() {
            cx.emit(PatchStoreEvent::PatchRemoved(id));
        }
    }

    pub fn resolve_patch(&self, id: PatchId, cx: &AppContext) -> Task<Result<ResolvedPatch>> {
        let project = self.project.clone();
        let Some(entry) = self.entries.get(&id) else {
            return Task::ready(Err(anyhow!("no patch for the given range")));
        };
        let patch = entry.patch.clone();
        let title = patch.input.title.clone();

        cx.spawn(|mut cx| async move {
            let mut result = ResolvedPatch {
                title,
                edit_groups: HashMap::default(),
                errors: Vec::new(),
            };

            for mut patch_buffer in patch.buffers {
                let buffer =
                    open_buffer_for_edit_path(&project, patch_buffer.path.clone(), &mut cx);
                if let Some(buffer) = buffer {
                    let buffer = buffer.await?;
                    let snapshot = buffer.read_with(&cx, |buffer, _| buffer.text_snapshot())?;

                    let diff = buffer
                        .update(&mut cx, |buffer, cx| {
                            buffer.diff_rope(&patch_buffer.content, cx)
                        })?
                        .await;

                    let mut delta = 0isize;
                    let mut patch_edits = patch_buffer.edits.iter_mut().peekable();
                    for (diff_range, new_text) in &diff.edits {
                        while let Some(edit) = patch_edits.peek_mut() {
                            if diff_range.start >= edit.range.end {
                                break;
                            } else {
                                if diff_range.end > edit.range.start {
                                    edit.range.start = cmp::min(edit.range.start, diff_range.start);
                                    edit.range.end = diff_range.start
                                        + new_text.len()
                                        + edit.range.end.saturating_sub(diff_range.end);
                                }

                                edit.range.start = (edit.range.start as isize + delta) as usize;
                                edit.range.end = (edit.range.end as isize + delta) as usize;
                                patch_edits.next();
                            }
                        }

                        delta += new_text.len() as isize - diff_range.len() as isize;
                    }

                    for edit in patch_edits {
                        edit.range.start = (edit.range.start as isize + delta) as usize;
                        edit.range.end = (edit.range.end as isize + delta) as usize;
                    }

                    let edits = patch_buffer
                        .edits
                        .into_iter()
                        .map(|edit| ResolvedEdit {
                            range: snapshot.anchor_before(edit.range.start)
                                ..snapshot.anchor_after(edit.range.end),
                            new_text: edit.new_text,
                            description: edit.description,
                        })
                        .collect::<Vec<_>>();
                    result
                        .edit_groups
                        .insert(buffer, Self::group_edits(edits, &snapshot));
                }
            }

            Ok(result)
        })
    }

    async fn locate_patch(
        patch: AssistantPatch,
        project: Model<Project>,
        old_located_patch: LocatedPatch,
        cx: &mut AsyncAppContext,
    ) -> Result<(LocatedPatch, bool)> {
        let old_input_edits = old_located_patch.input.edits;
        let old_outputs = old_located_patch.buffers;
        let mut equals_old_patch = old_located_patch.input.range == patch.range
            && old_located_patch.input.title == patch.title
            && old_located_patch.input.status == patch.status;

        // Convert each input edit into a located edit.
        let mut new_outputs = Vec::<LocatedPatchBuffer>::new();
        for (input_edit_ix, input_edit) in patch.edits.iter().enumerate() {
            let path: Arc<Path> = Path::new(&input_edit.path).into();

            let new_buffer_ix = new_outputs.binary_search_by_key(&&path, |buffer| &buffer.path);
            let old_buffer_ix = old_outputs.binary_search_by_key(&&path, |buffer| &buffer.path);
            let old_buffer = old_buffer_ix.ok().map(|ix| &old_outputs[ix]);

            // Find or load the buffer for this edit.
            let new_buffer_ix = match new_buffer_ix {
                Ok(ix) => ix,
                Err(ix) => {
                    let content = if let Some(old_buffer) = old_buffer {
                        old_buffer.content.clone()
                    } else {
                        let Some(buffer) = open_buffer_for_edit_path(&project, path.clone(), cx)
                        else {
                            continue;
                        };
                        let Some(buffer) = buffer.await.log_err() else {
                            continue;
                        };
                        buffer.read_with(cx, |buffer, _| buffer.as_rope().clone())?
                    };

                    new_outputs.insert(
                        ix,
                        LocatedPatchBuffer {
                            content,
                            path,
                            edits: Vec::new(),
                        },
                    );
                    ix
                }
            };
            let new_buffer = &mut new_outputs[new_buffer_ix];

            // Determine if this edit has already been located in the previoius patch.
            // If this edit is new, then locate it.
            let old_located_edit = old_input_edits
                .iter()
                .position(|old_input_edit| old_input_edit == input_edit)
                .and_then(|old_input_edit_ix| {
                    old_buffer?
                        .edits
                        .iter()
                        .find(|old_edit| old_edit.input_ix == old_input_edit_ix)
                });

            let mut located_edit = if let Some(old_located_edit) = old_located_edit {
                old_located_edit.clone()
            } else {
                equals_old_patch = false;
                cx.background_executor()
                    .spawn({
                        let edit = input_edit.kind.clone();
                        let content = new_buffer.content.clone();
                        async move { edit.locate(input_edit_ix, &content) }
                    })
                    .await
            };

            located_edit.input_ix = input_edit_ix;

            match new_buffer.edits.binary_search_by_key(
                &(located_edit.range.start, located_edit.range.end),
                |edit| (edit.range.start, edit.range.end),
            ) {
                Ok(ix) => new_buffer.edits[ix] = located_edit,
                Err(ix) => new_buffer.edits.insert(ix, located_edit),
            }
        }

        equals_old_patch &=
            old_outputs
                .iter()
                .zip(new_outputs.iter())
                .all(|(old_output, new_output)| {
                    old_output.path == new_output.path && old_output.edits == new_output.edits
                });

        Ok((
            LocatedPatch {
                input: patch,
                buffers: new_outputs,
            },
            !equals_old_patch,
        ))
    }

    fn group_edits(
        mut edits: Vec<ResolvedEdit>,
        snapshot: &text::BufferSnapshot,
    ) -> Vec<ResolvedEditGroup> {
        let mut edit_groups = Vec::<ResolvedEditGroup>::new();
        // Sort edits by their range so that earlier, larger ranges come first
        edits.sort_by(|a, b| a.range.cmp(&b.range, &snapshot));

        // Merge overlapping edits
        edits.dedup_by(|a, b| b.try_merge(a, &snapshot));

        // Create context ranges for each edit
        for edit in edits {
            let context_range = {
                let edit_point_range = edit.range.to_point(&snapshot);
                let start_row = edit_point_range.start.row.saturating_sub(5);
                let end_row = cmp::min(edit_point_range.end.row + 5, snapshot.max_point().row);
                let start = snapshot.anchor_before(Point::new(start_row, 0));
                let end = snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
                start..end
            };

            if let Some(last_group) = edit_groups.last_mut() {
                if last_group
                    .context_range
                    .end
                    .cmp(&context_range.start, &snapshot)
                    .is_ge()
                {
                    // Merge with the previous group if context ranges overlap
                    last_group.context_range.end = context_range.end;
                    last_group.edits.push(edit);
                } else {
                    // Create a new group
                    edit_groups.push(ResolvedEditGroup {
                        context_range,
                        edits: vec![edit],
                    });
                }
            } else {
                // Create the first group
                edit_groups.push(ResolvedEditGroup {
                    context_range,
                    edits: vec![edit],
                });
            }
        }

        edit_groups
    }
}

fn open_buffer_for_edit_path(
    project: &Model<Project>,
    path: Arc<Path>,
    cx: &mut AsyncAppContext,
) -> Option<Task<Result<Model<Buffer>>>> {
    project
        .update(cx, |project, cx| {
            let project_path = project
                .find_project_path(&path, cx)
                .or_else(|| {
                    // If we couldn't find a project path for it, put it in the active worktree
                    // so that when we create the buffer, it can be saved.
                    let worktree = project
                        .active_entry()
                        .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
                        .or_else(|| project.worktrees(cx).next())?;
                    let worktree = worktree.read(cx);

                    Some(ProjectPath {
                        worktree_id: worktree.id(),
                        path: path.clone(),
                    })
                })
                .with_context(|| format!("worktree not found for {:?}", path))
                .log_err();
            Some(project.open_buffer(project_path?, cx))
        })
        .ok()
        .flatten()
}

impl SearchMatrix {
    fn new(rows: usize, cols: usize) -> Self {
        SearchMatrix {
            cols,
            data: vec![SearchState::new(0, SearchDirection::Diagonal); rows * cols],
        }
    }

    fn get(&self, row: usize, col: usize) -> SearchState {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, cost: SearchState) {
        self.data[row * self.cols + col] = cost;
    }
}

impl ResolvedPatch {
    pub fn apply(
        &self,
        editor: &mut ProposedChangesEditor,
        old_patch: Option<&Self>,
        cx: &mut ViewContext<ProposedChangesEditor>,
    ) {
        editor.set_title(self.title.clone(), cx);

        // ensure branch buffers for every file in the patch
        for (buffer, new_edit_groups) in &self.edit_groups {
            let branch_buffer = editor.add_buffer(buffer.clone(), cx);
            let empty = Vec::new();
            let old_edit_groups = old_patch
                .and_then(|patch| patch.edit_groups.get(&buffer))
                .unwrap_or(&empty);
            Self::apply_buffer_edits(old_edit_groups, new_edit_groups, &branch_buffer, cx);
        }

        // Update the multibuffer's excerpts to reflect the new patch.
        let mut edit_groups = self.edit_groups.iter().collect::<Vec<_>>();
        edit_groups.sort_by_key(|(buffer, _)| buffer.read(cx).file().map(|file| file.path()));
        let multibuffer = editor.multibuffer();
        let snapshot = multibuffer.read(cx).snapshot(cx);
        let mut old_excerpts = snapshot.excerpts().peekable();
        let mut new_excerpts = edit_groups
            .iter()
            .flat_map(|(buffer, groups)| {
                let buffer = editor.branch_buffer_for_base(buffer).unwrap();
                groups
                    .iter()
                    .map(move |group| (buffer.clone(), group.context_range.clone()))
            })
            .peekable();

        let mut excerpts_to_add = Vec::new();
        let mut excerpts_to_remove = Vec::new();
        loop {
            let old_excerpt = old_excerpts.peek();
            let new_excerpt = new_excerpts.peek();

            let (old_excerpt, new_excerpt) = match (old_excerpt, new_excerpt) {
                (None, None) => break,
                (None, Some(_)) => {
                    excerpts_to_add.push(new_excerpts.next().unwrap());
                    continue;
                }
                (Some(_), None) => {
                    excerpts_to_remove.push(old_excerpts.next().unwrap().0);
                    continue;
                }
                (Some(old_excerpt), Some(new_excerpt)) => (old_excerpt, new_excerpt),
            };

            let (_, old_buffer, old_range) = old_excerpt;
            let (new_buffer, new_range) = new_excerpt;

            match old_buffer
                .file()
                .map(|f| f.path())
                .cmp(&new_buffer.read(cx).file().map(|f| f.path()))
            {
                Ordering::Less => {
                    excerpts_to_remove.push(old_excerpts.next().unwrap().0);
                    continue;
                }
                Ordering::Greater => {
                    excerpts_to_add.push(new_excerpts.next().unwrap());
                    continue;
                }
                Ordering::Equal => {}
            }

            match old_range.context.cmp(&new_range, old_buffer) {
                Ordering::Less => {
                    excerpts_to_remove.push(old_excerpts.next().unwrap().0);
                    continue;
                }
                Ordering::Greater => {
                    excerpts_to_add.push(new_excerpts.next().unwrap());
                    continue;
                }
                Ordering::Equal => {
                    old_excerpts.next().unwrap();
                    new_excerpts.next().unwrap();
                }
            }
        }

        multibuffer.update(cx, |multibuffer, cx| {
            for (buffer, range) in excerpts_to_add {
                multibuffer.push_excerpts(
                    buffer.clone(),
                    vec![ExcerptRange {
                        context: range,
                        primary: None,
                    }],
                    cx,
                );
            }
            multibuffer.remove_excerpts(excerpts_to_remove, cx);
        });
    }

    fn apply_buffer_edits(
        old_edit_groups: &Vec<ResolvedEditGroup>,
        new_edit_groups: &Vec<ResolvedEditGroup>,
        branch_buffer: &Model<Buffer>,
        cx: &mut AppContext,
    ) {
        let mut old_edit_groups = old_edit_groups
            .iter()
            .flat_map(|group| group.edits.iter())
            .peekable();

        branch_buffer.update(cx, |branch_buffer, cx| {
            let mut edits = Vec::new();
            for group in new_edit_groups {
                for new_edit in &group.edits {
                    let mut edit_already_performed = false;
                    while let Some(old_edit) = old_edit_groups.peek() {
                        match old_edit.range.cmp(&new_edit.range, &branch_buffer) {
                            Ordering::Greater => break,
                            Ordering::Less => {
                                // todo!(max): undo the old edit
                                old_edit_groups.next().unwrap();
                            }
                            Ordering::Equal => {
                                if old_edit.new_text == new_edit.new_text {
                                    edit_already_performed = true;
                                }
                                old_edit_groups.next().unwrap();
                            }
                        }
                    }

                    if !edit_already_performed {
                        edits.push((new_edit.range.clone(), new_edit.new_text.clone()));
                    }
                }
            }

            branch_buffer.edit(
                edits,
                Some(AutoindentMode::Block {
                    original_indent_columns: Vec::new(),
                }),
                cx,
            );
        })
    }
}

impl ResolvedEdit {
    pub fn try_merge(&mut self, other: &Self, buffer: &text::BufferSnapshot) -> bool {
        let range = &self.range;
        let other_range = &other.range;

        // Don't merge if we don't contain the other suggestion.
        if range.start.cmp(&other_range.start, buffer).is_gt()
            || range.end.cmp(&other_range.end, buffer).is_lt()
        {
            return false;
        }

        let other_offset_range = other_range.to_offset(buffer);
        let offset_range = range.to_offset(buffer);

        // If the other range is empty at the start of this edit's range, combine the new text
        if other_offset_range.is_empty() && other_offset_range.start == offset_range.start {
            self.new_text = format!("{}\n{}", other.new_text, self.new_text);
            self.range.start = other_range.start;

            if let Some((description, other_description)) =
                self.description.as_mut().zip(other.description.as_ref())
            {
                *description = format!("{}\n{}", other_description, description)
            }
        } else {
            if let Some((description, other_description)) =
                self.description.as_mut().zip(other.description.as_ref())
            {
                description.push('\n');
                description.push_str(other_description);
            }
        }

        true
    }
}

impl AssistantEdit {
    pub fn new(
        path: Option<String>,
        operation: Option<String>,
        old_text: Option<String>,
        new_text: Option<String>,
        description: Option<String>,
    ) -> Result<Self> {
        let path = path.ok_or_else(|| anyhow!("missing path"))?;
        let operation = operation.ok_or_else(|| anyhow!("missing operation"))?;

        let kind = match operation.as_str() {
            "update" => AssistantEditKind::Update {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description,
            },
            "insert_before" => AssistantEditKind::InsertBefore {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description,
            },
            "insert_after" => AssistantEditKind::InsertAfter {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description,
            },
            "delete" => AssistantEditKind::Delete {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
            },
            "create" => AssistantEditKind::Create {
                description,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
            },
            _ => Err(anyhow!("unknown operation {operation:?}"))?,
        };

        Ok(Self { path, kind })
    }
}

impl AssistantEditKind {
    fn locate(self, input_ix: usize, buffer: &Rope) -> LocatedEdit {
        match self {
            Self::Update {
                old_text,
                new_text,
                description,
            } => {
                let range = Self::resolve_location(&buffer, &old_text);
                LocatedEdit {
                    range,
                    new_text,
                    description,
                    input_ix,
                }
            }
            Self::Create {
                new_text,
                description,
            } => LocatedEdit {
                range: 0..buffer.len(),
                description,
                new_text,
                input_ix,
            },
            Self::InsertBefore {
                old_text,
                mut new_text,
                description,
            } => {
                let range = Self::resolve_location(&buffer, &old_text);
                new_text.push('\n');
                LocatedEdit {
                    range: range.start..range.start,
                    new_text,
                    description,
                    input_ix,
                }
            }
            Self::InsertAfter {
                old_text,
                mut new_text,
                description,
            } => {
                let range = Self::resolve_location(&buffer, &old_text);
                new_text.insert(0, '\n');
                LocatedEdit {
                    range: range.end..range.end,
                    new_text,
                    description,
                    input_ix,
                }
            }
            Self::Delete { old_text } => {
                let range = Self::resolve_location(&buffer, &old_text);
                LocatedEdit {
                    range,
                    new_text: String::new(),
                    description: None,
                    input_ix,
                }
            }
        }
    }

    fn resolve_location(buffer: &Rope, search_query: &str) -> Range<usize> {
        const INSERTION_COST: u32 = 3;
        const DELETION_COST: u32 = 10;
        const WHITESPACE_INSERTION_COST: u32 = 1;
        const WHITESPACE_DELETION_COST: u32 = 1;

        let buffer_len = buffer.len();
        let query_len = search_query.len();
        let mut matrix = SearchMatrix::new(query_len + 1, buffer_len + 1);
        let mut leading_deletion_cost = 0_u32;
        for (row, query_byte) in search_query.bytes().enumerate() {
            let deletion_cost = if query_byte.is_ascii_whitespace() {
                WHITESPACE_DELETION_COST
            } else {
                DELETION_COST
            };

            leading_deletion_cost = leading_deletion_cost.saturating_add(deletion_cost);
            matrix.set(
                row + 1,
                0,
                SearchState::new(leading_deletion_cost, SearchDirection::Diagonal),
            );

            for (col, buffer_byte) in buffer.bytes_in_range(0..buffer.len()).flatten().enumerate() {
                let insertion_cost = if buffer_byte.is_ascii_whitespace() {
                    WHITESPACE_INSERTION_COST
                } else {
                    INSERTION_COST
                };

                let up = SearchState::new(
                    matrix.get(row, col + 1).cost.saturating_add(deletion_cost),
                    SearchDirection::Up,
                );
                let left = SearchState::new(
                    matrix.get(row + 1, col).cost.saturating_add(insertion_cost),
                    SearchDirection::Left,
                );
                let diagonal = SearchState::new(
                    if query_byte == *buffer_byte {
                        matrix.get(row, col).cost
                    } else {
                        matrix
                            .get(row, col)
                            .cost
                            .saturating_add(deletion_cost + insertion_cost)
                    },
                    SearchDirection::Diagonal,
                );
                matrix.set(row + 1, col + 1, up.min(left).min(diagonal));
            }
        }

        // Traceback to find the best match
        let mut best_buffer_end = buffer_len;
        let mut best_cost = u32::MAX;
        for col in 1..=buffer_len {
            let cost = matrix.get(query_len, col).cost;
            if cost < best_cost {
                best_cost = cost;
                best_buffer_end = col;
            }
        }

        let mut query_ix = query_len;
        let mut buffer_ix = best_buffer_end;
        while query_ix > 0 && buffer_ix > 0 {
            let current = matrix.get(query_ix, buffer_ix);
            match current.direction {
                SearchDirection::Diagonal => {
                    query_ix -= 1;
                    buffer_ix -= 1;
                }
                SearchDirection::Up => {
                    query_ix -= 1;
                }
                SearchDirection::Left => {
                    buffer_ix -= 1;
                }
            }
        }

        let mut start_offset = buffer.clip_offset(buffer_ix, Bias::Left);
        let mut end_offset = buffer.clip_offset(best_buffer_end, Bias::Right);

        let start = buffer.offset_to_point(start_offset);
        let end = buffer.offset_to_point(end_offset);

        start_offset -= start.column as usize;
        if end.column > 0 {
            end_offset += (buffer.line_len(end.row) - end.column) as usize;
        }

        start_offset..end_offset
    }
}

impl AssistantPatch {
    pub fn path_count(&self) -> usize {
        self.paths().count()
    }

    pub fn paths(&self) -> impl '_ + Iterator<Item = &str> {
        let mut prev_path = None;
        self.edits.iter().filter_map(move |edit| {
            let path = Some(edit.path.as_str());
            if path != prev_path {
                prev_path = path;
                return path;
            }
            None
        })
    }
}

impl PartialEq for AssistantPatch {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
            && self.title == other.title
            && Arc::ptr_eq(&self.edits, &other.edits)
    }
}

impl Eq for AssistantPatch {}
