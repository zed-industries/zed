use crate::{
    display_map::fold_map::{FoldBufferRows, FoldOffset, FoldSnapshot},
    Highlights,
};
use collections::HashMap;
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{BufferId, Chunk};
use project::buffer_store::BufferChangeSet;
use std::ops::Range;
use sum_tree::{Cursor, SumTree, TreeMap};
use text::Bias;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DiffOffset(pub usize);

struct DiffMap {
    snapshot: DiffMapSnapshot,
    diff_bases: HashMap<BufferId, ChangeSetState>,
    buffer_input_row_counts: Vec<(BufferId, u32)>,
}

struct ChangeSetState {
    change_set: Model<BufferChangeSet>,
    last_version: Option<usize>,
    _subscription: Subscription,
}

#[derive(Clone)]
pub(crate) struct DiffMapSnapshot {
    diffs: TreeMap<BufferId, git::diff::BufferDiff>,
    diff_transforms: SumTree<DiffTransform>,
    fold_snapshot: FoldSnapshot,
}

#[derive(Debug, Clone)]
enum DiffTransform {
    BufferContent {
        row_count: u32,
        buffer_id: BufferId,
    },
    DeletedHunk {
        row_count: u32,
        buffer_id: BufferId,
        hunk_position: text::Anchor,
        base_text_start_row: u32,
    },
}

#[derive(Debug, Clone)]
struct DiffTransformSummary {
    input_row_count: u32,
    output_row_count: u32,
    transform_count: usize,
}

struct DiffMapChunks<'a> {
    cursor: Cursor<'a, DiffTransform, (DiffOffset, FoldOffset)>,
}

struct DiffMapBufferRows<'a> {
    cursor: Cursor<'a, DiffTransform, DiffTransformSummary>,
    input_buffer_rows: FoldBufferRows<'a>,
}

struct InputRowCount(u32);

impl DiffMap {
    pub fn new(fold_snapshot: FoldSnapshot, cx: &mut AppContext) -> (Model<Self>, DiffMapSnapshot) {
        let snapshot = DiffMapSnapshot {
            diffs: TreeMap::default(),
            diff_transforms: SumTree::new(&()),
            fold_snapshot,
        };

        // Determine the extents of every run of excerpts associated with a
        // single buffer.
        let mut buffer_input_row_counts = Vec::new();
        let mut current_buffer_and_start_row = None;
        let fold_snapshot = &snapshot.fold_snapshot;
        let inlay_snapshot = &fold_snapshot.inlay_snapshot;
        let buffer_snapshot = &inlay_snapshot.buffer;
        for excerpt in buffer_snapshot.all_excerpts().map(Some).chain([None]) {
            let buffer_id = excerpt.as_ref().map(|e| e.buffer().remote_id());
            let position = excerpt.map_or(buffer_snapshot.max_point(), |e| e.start_point());
            let position = inlay_snapshot.to_inlay_point(position);
            let position = fold_snapshot.to_fold_point(position, Bias::Right);
            let row = position.row();
            if let Some((prev_buffer_id, prev_start_row)) = &current_buffer_and_start_row {
                if buffer_id != Some(*prev_buffer_id) {
                    buffer_input_row_counts.push((*prev_buffer_id, row - *prev_start_row));
                    current_buffer_and_start_row.take();
                }
            }
            if current_buffer_and_start_row.is_none() {
                if let Some(buffer_id) = buffer_id {
                    current_buffer_and_start_row = Some((buffer_id, row));
                }
            }
        }

        let this = cx.new_model(|_| Self {
            buffer_input_row_counts,
            snapshot: snapshot.clone(),
            diff_bases: HashMap::default(),
        });

        (this, snapshot)
    }

    pub fn add_change_set(
        &mut self,
        change_set: Model<BufferChangeSet>,
        cx: &mut ModelContext<Self>,
    ) {
        let buffer_id = change_set.read(cx).buffer_id;
        self.buffer_diff_changed(change_set.clone(), cx);
        self.diff_bases.insert(
            buffer_id,
            ChangeSetState {
                _subscription: cx.observe(&change_set, Self::buffer_diff_changed),
                last_version: None,
                change_set,
            },
        );
    }

    fn buffer_diff_changed(
        &mut self,
        change_set: Model<BufferChangeSet>,
        cx: &mut ModelContext<Self>,
    ) {
        let change_set = change_set.read(cx);
        let buffer_id = change_set.buffer_id;
        let diff = change_set.diff_to_buffer.clone();

        self.snapshot.diffs.insert(buffer_id, diff);

        let start_input_row = self.start_input_row_for_buffer(buffer_id);
        let mut cursor = self
            .snapshot
            .diff_transforms
            .cursor::<DiffTransformSummary>(&());
        let mut new_transforms = SumTree::default();
        new_transforms.append(
            cursor.slice(&InputRowCount(start_input_row), sum_tree::Bias::Right, &()),
            &(),
        );

        new_transforms.append(cursor.suffix(&()), &());
        drop(cursor);
        self.snapshot.diff_transforms = new_transforms;
    }

    fn start_input_row_for_buffer(&self, buffer_id: BufferId) -> u32 {
        let mut result = 0;
        for (id, row_count) in &self.buffer_input_row_counts {
            if *id == buffer_id {
                break;
            }
            result += *row_count
        }
        result
    }

    pub(super) fn expand_diff_hunks(
        &mut self,
        multi_buffer_range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) {
    }

    pub(super) fn collapse_diff_hunks(
        &mut self,
        multi_buffer_range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) {
    }

    pub(super) fn set_all_hunks_expanded(&mut self, expand_all: bool, cx: &mut ModelContext<Self>) {
        //
    }

    fn snapshot(&self) -> DiffMapSnapshot {
        self.snapshot.clone()
    }
}

impl DiffMapSnapshot {
    pub fn chunks<'a>(
        &'a self,
        range: Range<DiffOffset>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> DiffMapChunks<'a> {
        todo!()
    }

    pub fn buffer_rows(&self, start_row: u32) -> DiffMapBufferRows {
        todo!()
        //
    }
}

impl<'a> Iterator for DiffMapChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl sum_tree::Item for DiffTransform {
    type Summary = DiffTransformSummary;

    fn summary(&self, _: &<Self::Summary as sum_tree::Summary>::Context) -> Self::Summary {
        match self {
            DiffTransform::BufferContent { row_count, .. } => DiffTransformSummary {
                input_row_count: *row_count,
                output_row_count: *row_count,
                transform_count: 1,
            },
            DiffTransform::DeletedHunk { row_count, .. } => DiffTransformSummary {
                input_row_count: 0,
                output_row_count: *row_count,
                transform_count: 1,
            },
        }
    }
}

impl sum_tree::Summary for DiffTransformSummary {
    type Context = ();

    fn zero(_: &Self::Context) -> Self {
        DiffTransformSummary {
            input_row_count: 0,
            output_row_count: 0,
            transform_count: 0,
        }
    }

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.input_row_count += summary.input_row_count;
        self.output_row_count += summary.output_row_count;
        self.transform_count += summary.transform_count;
    }
}

impl<'a> sum_tree::SeekTarget<'a, DiffTransformSummary, DiffTransformSummary> for InputRowCount {
    fn cmp(&self, cursor_location: &DiffTransformSummary, _: &()) -> std::cmp::Ordering {
        Ord::cmp(&self.0, &cursor_location.input_row_count)
    }
}

impl<'a> Iterator for DiffMapBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_map::{fold_map::FoldMap, inlay_map::InlayMap};
    use gpui::AppContext;
    use indoc::indoc;
    use multi_buffer::MultiBuffer;
    use project::Project;
    use settings::SettingsStore;

    #[gpui::test]
    fn test_basic_diff(cx: &mut gpui::AppContext) {
        init_test(cx);

        let text = indoc!(
            "
            one
            two
            five
            "
        );

        let base_text = indoc!(
            "
            one
            two
            three
            four
            five
            six
            "
        );
        let buffer = cx.new_model(|cx| language::Buffer::local(text, cx));
        let change_set = cx.new_model(|cx| {
            BufferChangeSet::new_with_base_text(
                base_text.to_string(),
                buffer.read(cx).text_snapshot(),
                cx,
            )
        });

        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (diff_map, _) = DiffMap::new(fold_snapshot, cx);
        diff_map.update(cx, |diff_map, cx| diff_map.add_change_set(change_set, cx));
    }

    fn init_test(cx: &mut AppContext) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}
