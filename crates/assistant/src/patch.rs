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
use text::{AnchorRangeExt as _, Bias, OffsetRangeExt as _, Point};
use ui::ViewContext;
use util::ResultExt;

pub struct PatchStore {
    next_patch_id: usize,
    project: Model<Project>,
    entries: HashMap<PatchId, PatchStoreEntry>,
}

struct PatchStoreEntry {
    patch: LocatedPatch,
    locate_task: Option<Task<Result<()>>>,
}

pub enum PatchStoreEvent {
    PatchUpdated(PatchId),
    PatchRemoved(PatchId),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PatchId(usize);

impl EventEmitter<PatchStoreEvent> for PatchStore {}

impl PatchStore {
    pub fn new(project: Model<Project>) -> Self {
        Self {
            next_patch_id: 0,
            project,
            entries: HashMap::default(),
        }
    }

    pub(crate) fn get(&self, id: PatchId) -> Option<&AssistantPatch> {
        Some(&self.entries.get(&id)?.patch.input)
    }

    pub(crate) fn insert(&mut self, patch: AssistantPatch, cx: &mut ModelContext<Self>) -> PatchId {
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
            },
        );
        self.update(id, patch, cx).unwrap();
        id
    }

    pub(crate) fn update(
        &mut self,
        id: PatchId,
        patch: AssistantPatch,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let Some(entry) = self.entries.get_mut(&id) else {
            return Err(anyhow!("no such patch"));
        };

        cx.emit(PatchStoreEvent::PatchUpdated(id));

        let project = self.project.clone();
        let prev_patch = entry.patch.clone();
        entry.locate_task = Some(cx.spawn(|this, mut cx| async move {
            let located_patch = Self::locate_patch(patch, project, prev_patch, &mut cx).await?;
            this.update(&mut cx, |this, cx| {
                this.entries.insert(
                    id,
                    PatchStoreEntry {
                        patch: located_patch,
                        locate_task: None,
                    },
                );
                cx.emit(PatchStoreEvent::PatchUpdated(id));
            })
        }));

        Ok(())
    }

    pub(crate) fn remove(&mut self, id: PatchId, cx: &mut ModelContext<Self>) {
        if self.entries.remove(&id).is_some() {
            cx.emit(PatchStoreEvent::PatchRemoved(id));
        }
    }

    pub(crate) fn resolve_patch(
        &self,
        id: PatchId,
        cx: &AppContext,
    ) -> Task<Result<ResolvedPatch>> {
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
                        .insert(buffer, AssistantPatch::group_edits(edits, &snapshot));
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
    ) -> Result<LocatedPatch> {
        let old_input_edits = old_located_patch.input.edits;
        let old_outputs = old_located_patch.buffers;

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
                cx.background_executor()
                    .spawn({
                        let edit = input_edit.kind.clone();
                        let content = new_buffer.content.clone();
                        async move { edit.clone().locate(input_edit_ix, &content) }
                    })
                    .await
            };

            located_edit.input_ix = input_edit_ix;

            match new_buffer
                .edits
                .binary_search_by_key(&&located_edit.range.start, |edit| &edit.range.start)
            {
                Ok(ix) => new_buffer.edits[ix] = located_edit,
                Err(ix) => new_buffer.edits.insert(ix, located_edit),
            }
        }

        Ok(LocatedPatch {
            input: patch,
            buffers: new_outputs,
        })
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

#[derive(Clone, Debug)]
pub(crate) struct AssistantPatch {
    pub range: Range<language::Anchor>,
    pub title: SharedString,
    pub edits: Arc<[AssistantEdit]>,
    pub status: AssistantPatchStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum AssistantPatchStatus {
    Pending,
    Ready,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AssistantEdit {
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
    pub buffers: Vec<LocatedPatchBuffer>,
    pub input: AssistantPatch,
}

#[derive(Clone, Debug)]
struct LocatedPatchBuffer {
    pub path: Arc<Path>,
    pub content: Rope,
    pub edits: Vec<LocatedEdit>,
}

#[derive(Clone, Debug)]
struct LocatedEdit {
    range: Range<usize>,
    new_text: String,
    description: Option<String>,
    input_ix: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedPatch {
    pub title: SharedString,
    pub edit_groups: HashMap<Model<Buffer>, Vec<ResolvedEditGroup>>,
    pub errors: Vec<AssistantPatchResolutionError>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedEditGroup {
    pub context_range: Range<language::Anchor>,
    pub edits: Vec<ResolvedEdit>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedEdit {
    range: Range<language::Anchor>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{AppContext, Context, TestAppContext};
    use language::{
        language_settings::AllLanguageSettings, Language, LanguageConfig, LanguageMatcher,
    };
    use serde_json::json;
    use settings::SettingsStore;
    use ui::BorrowAppContext;
    use unindent::Unindent as _;
    use util::test::{generate_marked_text, marked_text_ranges};

    #[gpui::test]
    async fn test_patch_store(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.update(language::init);
        cx.update(Project::init_settings);

        let fs = FakeFs::new(cx.background_executor.clone());

        fs.insert_tree(
            "/root",
            json!({
                "src": {
                    "lib.rs": "
                        fn one() -> usize {
                            1
                        }
                        fn two() -> usize {
                            2
                        }
                        fn three() -> usize {
                            3
                        }
                    ".unindent(),
                    "main.rs": "
                        use crate::one;
                        fn main() { one(); }
                    ".unindent(),
                }
            }),
        )
        .await;

        let project = Project::test(fs, [Path::new("/root")], cx).await;
        project.update(cx, |project, _| {
            project.languages().add(Arc::new(rust_lang()));
        });
        let patch_store = cx.new_model(|_| PatchStore::new(project.clone()));
        let context_buffer = cx.new_model(|cx| Buffer::local("hello", cx));
        let context_buffer = context_buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let range = context_buffer.anchor_before(0)..context_buffer.anchor_before(1);

        let patch_id = patch_store.update(cx, |store, cx| {
            store.insert(
                AssistantPatch {
                    range: range.clone(),
                    title: "first patch".into(),
                    edits: vec![AssistantEdit {
                        path: "src/lib.rs".into(),
                        kind: AssistantEditKind::Update {
                            old_text: "1".into(),
                            new_text: "100".into(),
                            description: None,
                        },
                    }]
                    .into(),
                    status: AssistantPatchStatus::Pending,
                },
                cx,
            )
        });

        cx.run_until_parked();
        let patch = patch_store
            .update(cx, |store, cx| store.resolve_patch(patch_id, cx))
            .await
            .unwrap();
        assert_apply_patch(
            &patch,
            cx,
            &[(
                Path::new("src/lib.rs").into(),
                "
                fn one() -> usize {
                    100
                }
                fn two() -> usize {
                    2
                }
                fn three() -> usize {
                    3
                }
                "
                .unindent(),
            )],
        );

        patch_store.update(cx, |store, cx| {
            store
                .update(
                    patch_id,
                    AssistantPatch {
                        range: range.clone(),
                        title: "first patch".into(),
                        edits: vec![
                            AssistantEdit {
                                path: "src/lib.rs".into(),
                                kind: AssistantEditKind::Update {
                                    old_text: "1".into(),
                                    new_text: "100".into(),
                                    description: None,
                                },
                            },
                            AssistantEdit {
                                path: "src/lib.rs".into(),
                                kind: AssistantEditKind::Update {
                                    old_text: "3".into(),
                                    new_text: "300".into(),
                                    description: None,
                                },
                            },
                        ]
                        .into(),
                        status: AssistantPatchStatus::Pending,
                    },
                    cx,
                )
                .unwrap();
        });

        cx.run_until_parked();
        let patch = patch_store
            .update(cx, |store, cx| store.resolve_patch(patch_id, cx))
            .await
            .unwrap();
        assert_apply_patch(
            &patch,
            cx,
            &[(
                Path::new("src/lib.rs").into(),
                "
                fn one() -> usize {
                    100
                }
                fn two() -> usize {
                    2
                }
                fn three() -> usize {
                    300
                }
                "
                .unindent(),
            )],
        );
    }

    #[track_caller]
    fn assert_apply_patch(
        patch: &ResolvedPatch,
        cx: &mut TestAppContext,
        expected_output: &[(Arc<Path>, String)],
    ) {
        let mut actual_output = Vec::new();
        for (buffer, edit_groups) in &patch.edit_groups {
            let branch = buffer.update(cx, |buffer, cx| buffer.branch(cx));
            cx.update(|cx| {
                ResolvedPatch::apply_buffer_edits(&Vec::new(), edit_groups, &branch, cx);
                actual_output.push((
                    buffer.read(cx).file().unwrap().path().clone(),
                    branch.read(cx).text(),
                ));
            });
        }
        pretty_assertions::assert_eq!(actual_output, expected_output);
    }

    #[gpui::test]
    fn test_resolve_location(cx: &mut AppContext) {
        assert_location_resolution(
            concat!(
                "    Lorem\n",
                "«    ipsum\n",
                "    dolor sit amet»\n",
                "    consecteur",
            ),
            "ipsum\ndolor",
            cx,
        );

        assert_location_resolution(
            &"
            «fn foo1(a: usize) -> usize {
                40
            }»

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "fn foo1(b: usize) {\n40\n}",
            cx,
        );

        assert_location_resolution(
            &"
            fn main() {
            «    Foo
                    .bar()
                    .baz()
                    .qux()»
            }

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "Foo.bar.baz.qux()",
            cx,
        );

        assert_location_resolution(
            &"
            class Something {
                one() { return 1; }
            «    two() { return 2222; }
                three() { return 333; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            »    seven() { return 7; }
                eight() { return 8; }
            }
            "
            .unindent(),
            &"
                two() { return 2222; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            "
            .unindent(),
            cx,
        );
    }

    #[gpui::test]
    async fn test_resolve_edits(cx: &mut TestAppContext) {
        cx.update(init_test);

        assert_edits(
            "
                /// A person
                struct Person {
                    name: String,
                    age: usize,
                }

                /// A dog
                struct Dog {
                    weight: f32,
                }

                impl Person {
                    fn name(&self) -> &str {
                        &self.name
                    }
                }
            "
            .unindent(),
            vec![
                AssistantEditKind::Update {
                    old_text: "
                        name: String,
                    "
                    .unindent(),
                    new_text: "
                        first_name: String,
                        last_name: String,
                    "
                    .unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn name(&self) -> &str {
                            &self.name
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn name(&self) -> String {
                            format!(\"{} {}\", self.first_name, self.last_name)
                        }
                    "
                    .unindent(),
                    description: None,
                },
            ],
            "
                /// A person
                struct Person {
                    first_name: String,
                    last_name: String,
                    age: usize,
                }

                /// A dog
                struct Dog {
                    weight: f32,
                }

                impl Person {
                    fn name(&self) -> String {
                        format!(\"{} {}\", self.first_name, self.last_name)
                    }
                }
            "
            .unindent(),
            cx,
        )
        .await;

        // Ensure InsertBefore merges correctly with Update of the same text
        assert_edits(
            "
                fn foo() {

                }
            "
            .unindent(),
            vec![
                AssistantEditKind::InsertBefore {
                    old_text: "
                        fn foo() {"
                        .unindent(),
                    new_text: "
                        fn bar() {
                            qux();
                        }"
                    .unindent(),
                    description: Some("implement bar".into()),
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn foo() {

                        }"
                    .unindent(),
                    new_text: "
                        fn foo() {
                            bar();
                        }"
                    .unindent(),
                    description: Some("call bar in foo".into()),
                },
                AssistantEditKind::InsertAfter {
                    old_text: "
                        fn foo() {

                        }
                    "
                    .unindent(),
                    new_text: "
                        fn qux() {
                            // todo
                        }
                    "
                    .unindent(),
                    description: Some("implement qux".into()),
                },
            ],
            "
                fn bar() {
                    qux();
                }

                fn foo() {
                    bar();
                }

                fn qux() {
                    // todo
                }
            "
            .unindent(),
            cx,
        )
        .await;

        // Correctly indent new text when replacing multiple adjacent indented blocks.
        assert_edits(
            "
            impl Numbers {
                fn one() {
                    1
                }

                fn two() {
                    2
                }

                fn three() {
                    3
                }
            }
            "
            .unindent(),
            vec![
                AssistantEditKind::Update {
                    old_text: "
                        fn one() {
                            1
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn one() {
                            101
                        }
                    "
                    .unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn two() {
                            2
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn two() {
                            102
                        }
                    "
                    .unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn three() {
                            3
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn three() {
                            103
                        }
                    "
                    .unindent(),
                    description: None,
                },
            ],
            "
                impl Numbers {
                    fn one() {
                        101
                    }

                    fn two() {
                        102
                    }

                    fn three() {
                        103
                    }
                }
            "
            .unindent(),
            cx,
        )
        .await;

        assert_edits(
            "
            impl Person {
                fn set_name(&mut self, name: String) {
                    self.name = name;
                }

                fn name(&self) -> String {
                    return self.name;
                }
            }
            "
            .unindent(),
            vec![
                AssistantEditKind::Update {
                    old_text: "self.name = name;".unindent(),
                    new_text: "self._name = name;".unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "return self.name;\n".unindent(),
                    new_text: "return self._name;\n".unindent(),
                    description: None,
                },
            ],
            "
                impl Person {
                    fn set_name(&mut self, name: String) {
                        self._name = name;
                    }

                    fn name(&self) -> String {
                        return self._name;
                    }
                }
            "
            .unindent(),
            cx,
        )
        .await;
    }

    fn init_test(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        language::init(cx);
        Project::init_settings(cx);
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });
    }

    #[track_caller]
    fn assert_location_resolution(
        text_with_expected_range: &str,
        query: &str,
        cx: &mut AppContext,
    ) {
        let (text, _) = marked_text_ranges(text_with_expected_range, false);
        let buffer = cx.new_model(|cx| Buffer::local(text.clone(), cx));
        let snapshot = buffer.read(cx).snapshot();
        let range =
            AssistantEditKind::resolve_location(snapshot.as_rope(), query).to_offset(&snapshot);
        let text_with_actual_range = generate_marked_text(&text, &[range], false);
        pretty_assertions::assert_eq!(text_with_actual_range, text_with_expected_range);
    }

    async fn assert_edits(
        old_text: String,
        edits: Vec<AssistantEditKind>,
        new_text: String,
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"file.rs": old_text})).await;
        let project = Project::test(fs, [Path::new("/root")], cx).await;
        project.update(cx, |project, _| {
            project.languages().add(Arc::new(rust_lang()));
        });
        let patch_store = cx.new_model(|_| PatchStore::new(project));
        let patch_range = language::Anchor::MIN..language::Anchor::MAX;
        let patch_id = patch_store.update(cx, |patch_store, cx| {
            patch_store.insert(
                AssistantPatch {
                    range: patch_range.clone(),
                    title: "test-patch".into(),
                    edits: edits
                        .into_iter()
                        .map(|kind| AssistantEdit {
                            path: "file.rs".into(),
                            kind,
                        })
                        .collect(),
                    status: AssistantPatchStatus::Ready,
                },
                cx,
            )
        });
        cx.run_until_parked();
        let patch = patch_store
            .update(cx, |patch_store, cx| {
                patch_store.resolve_patch(patch_id, cx)
            })
            .await
            .unwrap();

        let (buffer, edit_groups) = patch.edit_groups.into_iter().next().unwrap();
        cx.update(|cx| ResolvedPatch::apply_buffer_edits(&Vec::new(), &edit_groups, &buffer, cx));
        let actual_new_text = buffer.read_with(cx, |buffer, _| buffer.text());
        pretty_assertions::assert_eq!(actual_new_text, new_text);
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(language::tree_sitter_rust::LANGUAGE.into()),
        )
        .with_indents_query(
            r#"
            (call_expression) @indent
            (field_expression) @indent
            (_ "(" ")" @end) @indent
            (_ "{" "}" @end) @indent
            "#,
        )
        .unwrap()
    }
}
