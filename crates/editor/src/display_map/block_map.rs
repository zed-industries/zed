use super::{
    Highlights,
    fold_map::Chunk,
    wrap_map::{self, WrapEdit, WrapPatch, WrapPoint, WrapSnapshot},
};
use crate::{
    EditorStyle, GutterDimensions,
    display_map::{Companion, dimensions::RowDelta, wrap_map::WrapRow},
};
use collections::{Bound, HashMap, HashSet};
use gpui::{AnyElement, App, EntityId, Pixels, Window};
use language::{Patch, Point};
use multi_buffer::{
    Anchor, ExcerptId, ExcerptInfo, MultiBuffer, MultiBufferOffset, MultiBufferPoint,
    MultiBufferRow, MultiBufferSnapshot, RowInfo, ToOffset, ToPoint as _,
};
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    fmt::Debug,
    ops::{Deref, DerefMut, Not, Range, RangeBounds, RangeInclusive},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};
use sum_tree::{Bias, ContextLessSummary, Dimensions, SumTree, TreeMap};
use text::{BufferId, Edit};
use ui::ElementId;

const NEWLINES: &[u8; rope::Chunk::MASK_BITS] = &[b'\n'; _];
const BULLETS: &[u8; rope::Chunk::MASK_BITS] = &[b'*'; _];

/// Tracks custom blocks such as diagnostics that should be displayed within buffer.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub struct BlockMap {
    pub(super) wrap_snapshot: RefCell<WrapSnapshot>,
    next_block_id: AtomicUsize,
    custom_blocks: Vec<Arc<CustomBlock>>,
    custom_blocks_by_id: TreeMap<CustomBlockId, Arc<CustomBlock>>,
    transforms: RefCell<SumTree<Transform>>,
    buffer_header_height: u32,
    excerpt_header_height: u32,
    pub(super) folded_buffers: HashSet<BufferId>,
    buffers_with_disabled_headers: HashSet<BufferId>,
}

pub struct BlockMapReader<'a> {
    blocks: &'a Vec<Arc<CustomBlock>>,
    pub snapshot: BlockSnapshot,
}

pub struct BlockMapWriter<'a> {
    block_map: &'a mut BlockMap,
    companion: Option<BlockMapWriterCompanion<'a>>,
}

struct BlockMapWriterCompanion<'a> {
    companion: &'a Companion,
    snapshot: &'a WrapSnapshot,
    entity: EntityId,
}

#[derive(Clone)]
pub struct BlockSnapshot {
    pub(super) wrap_snapshot: WrapSnapshot,
    transforms: SumTree<Transform>,
    custom_blocks_by_id: TreeMap<CustomBlockId, Arc<CustomBlock>>,
    pub(super) buffer_header_height: u32,
    pub(super) excerpt_header_height: u32,
}

impl Deref for BlockSnapshot {
    type Target = WrapSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.wrap_snapshot
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomBlockId(pub usize);

impl From<CustomBlockId> for ElementId {
    fn from(val: CustomBlockId) -> Self {
        val.0.into()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpacerId(pub usize);

/// A zero-indexed point in a text buffer consisting of a row and column
/// adjusted for inserted blocks, wrapped rows, tabs, folds and inlays.
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct BlockPoint(pub Point);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct BlockRow(pub u32);

impl_for_row_types! {
    BlockRow => RowDelta
}

impl BlockPoint {
    pub fn row(&self) -> BlockRow {
        BlockRow(self.0.row)
    }
}

pub type RenderBlock = Arc<dyn Send + Sync + Fn(&mut BlockContext) -> AnyElement>;

/// Where to place a block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlockPlacement<T> {
    /// Place the block above the given position.
    Above(T),
    /// Place the block below the given position.
    Below(T),
    /// Place the block next the given position.
    Near(T),
    /// Replace the given range of positions with the block.
    Replace(RangeInclusive<T>),
}

impl<T> BlockPlacement<T> {
    pub fn start(&self) -> &T {
        match self {
            BlockPlacement::Above(position) => position,
            BlockPlacement::Below(position) => position,
            BlockPlacement::Near(position) => position,
            BlockPlacement::Replace(range) => range.start(),
        }
    }

    fn end(&self) -> &T {
        match self {
            BlockPlacement::Above(position) => position,
            BlockPlacement::Below(position) => position,
            BlockPlacement::Near(position) => position,
            BlockPlacement::Replace(range) => range.end(),
        }
    }

    pub fn as_ref(&self) -> BlockPlacement<&T> {
        match self {
            BlockPlacement::Above(position) => BlockPlacement::Above(position),
            BlockPlacement::Below(position) => BlockPlacement::Below(position),
            BlockPlacement::Near(position) => BlockPlacement::Near(position),
            BlockPlacement::Replace(range) => BlockPlacement::Replace(range.start()..=range.end()),
        }
    }

    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> BlockPlacement<R> {
        match self {
            BlockPlacement::Above(position) => BlockPlacement::Above(f(position)),
            BlockPlacement::Below(position) => BlockPlacement::Below(f(position)),
            BlockPlacement::Near(position) => BlockPlacement::Near(f(position)),
            BlockPlacement::Replace(range) => {
                let (start, end) = range.into_inner();
                BlockPlacement::Replace(f(start)..=f(end))
            }
        }
    }

    fn tie_break(&self) -> u8 {
        match self {
            BlockPlacement::Replace(_) => 0,
            BlockPlacement::Above(_) => 1,
            BlockPlacement::Near(_) => 2,
            BlockPlacement::Below(_) => 3,
        }
    }
}

impl BlockPlacement<Anchor> {
    #[ztracing::instrument(skip_all)]
    fn cmp(&self, other: &Self, buffer: &MultiBufferSnapshot) -> Ordering {
        self.start()
            .cmp(other.start(), buffer)
            .then_with(|| other.end().cmp(self.end(), buffer))
            .then_with(|| self.tie_break().cmp(&other.tie_break()))
    }

    #[ztracing::instrument(skip_all)]
    fn to_wrap_row(&self, wrap_snapshot: &WrapSnapshot) -> Option<BlockPlacement<WrapRow>> {
        let buffer_snapshot = wrap_snapshot.buffer_snapshot();
        match self {
            BlockPlacement::Above(position) => {
                let mut position = position.to_point(buffer_snapshot);
                position.column = 0;
                let wrap_row = wrap_snapshot.make_wrap_point(position, Bias::Left).row();
                Some(BlockPlacement::Above(wrap_row))
            }
            BlockPlacement::Near(position) => {
                let mut position = position.to_point(buffer_snapshot);
                position.column = buffer_snapshot.line_len(MultiBufferRow(position.row));
                let wrap_row = wrap_snapshot.make_wrap_point(position, Bias::Left).row();
                Some(BlockPlacement::Near(wrap_row))
            }
            BlockPlacement::Below(position) => {
                let mut position = position.to_point(buffer_snapshot);
                position.column = buffer_snapshot.line_len(MultiBufferRow(position.row));
                let wrap_row = wrap_snapshot.make_wrap_point(position, Bias::Left).row();
                Some(BlockPlacement::Below(wrap_row))
            }
            BlockPlacement::Replace(range) => {
                let mut start = range.start().to_point(buffer_snapshot);
                let mut end = range.end().to_point(buffer_snapshot);
                if start == end {
                    None
                } else {
                    start.column = 0;
                    let start_wrap_row = wrap_snapshot.make_wrap_point(start, Bias::Left).row();
                    end.column = buffer_snapshot.line_len(MultiBufferRow(end.row));
                    let end_wrap_row = wrap_snapshot.make_wrap_point(end, Bias::Left).row();
                    Some(BlockPlacement::Replace(start_wrap_row..=end_wrap_row))
                }
            }
        }
    }
}

pub struct CustomBlock {
    pub id: CustomBlockId,
    pub placement: BlockPlacement<Anchor>,
    pub height: Option<u32>,
    style: BlockStyle,
    render: Arc<Mutex<RenderBlock>>,
    priority: usize,
}

#[derive(Clone)]
pub struct BlockProperties<P> {
    pub placement: BlockPlacement<P>,
    // None if the block takes up no space
    // (e.g. a horizontal line)
    pub height: Option<u32>,
    pub style: BlockStyle,
    pub render: RenderBlock,
    pub priority: usize,
}

impl<P: Debug> Debug for BlockProperties<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockProperties")
            .field("placement", &self.placement)
            .field("height", &self.height)
            .field("style", &self.style)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BlockStyle {
    Fixed,
    Flex,
    Sticky,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct EditorMargins {
    pub gutter: GutterDimensions,
    pub right: Pixels,
}

#[derive(gpui::AppContext, gpui::VisualContext)]
pub struct BlockContext<'a, 'b> {
    #[window]
    pub window: &'a mut Window,
    #[app]
    pub app: &'b mut App,
    pub anchor_x: Pixels,
    pub max_width: Pixels,
    pub margins: &'b EditorMargins,
    pub em_width: Pixels,
    pub line_height: Pixels,
    pub block_id: BlockId,
    pub selected: bool,
    pub editor_style: &'b EditorStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum BlockId {
    ExcerptBoundary(ExcerptId),
    FoldedBuffer(ExcerptId),
    Custom(CustomBlockId),
    Spacer(SpacerId),
}

impl From<BlockId> for ElementId {
    fn from(value: BlockId) -> Self {
        match value {
            BlockId::Custom(CustomBlockId(id)) => ("Block", id).into(),
            BlockId::ExcerptBoundary(excerpt_id) => {
                ("ExcerptBoundary", EntityId::from(excerpt_id)).into()
            }
            BlockId::FoldedBuffer(id) => ("FoldedBuffer", EntityId::from(id)).into(),
            BlockId::Spacer(SpacerId(id)) => ("Spacer", id).into(),
        }
    }
}

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(id) => write!(f, "Block({id:?})"),
            Self::ExcerptBoundary(id) => write!(f, "ExcerptHeader({id:?})"),
            Self::FoldedBuffer(id) => write!(f, "FoldedBuffer({id:?})"),
            Self::Spacer(id) => write!(f, "Spacer({id:?})"),
        }
    }
}

#[derive(Clone, Debug)]
struct Transform {
    summary: TransformSummary,
    block: Option<Block>,
}

#[derive(Clone)]
pub enum Block {
    Custom(Arc<CustomBlock>),
    FoldedBuffer {
        first_excerpt: ExcerptInfo,
        height: u32,
    },
    ExcerptBoundary {
        excerpt: ExcerptInfo,
        height: u32,
    },
    BufferHeader {
        excerpt: ExcerptInfo,
        height: u32,
    },
    Spacer {
        id: SpacerId,
        height: u32,
        is_below: bool,
    },
}

impl Block {
    pub fn id(&self) -> BlockId {
        match self {
            Block::Custom(block) => BlockId::Custom(block.id),
            Block::ExcerptBoundary {
                excerpt: next_excerpt,
                ..
            } => BlockId::ExcerptBoundary(next_excerpt.id),
            Block::FoldedBuffer { first_excerpt, .. } => BlockId::FoldedBuffer(first_excerpt.id),
            Block::BufferHeader {
                excerpt: next_excerpt,
                ..
            } => BlockId::ExcerptBoundary(next_excerpt.id),
            Block::Spacer { id, .. } => BlockId::Spacer(*id),
        }
    }

    pub fn has_height(&self) -> bool {
        match self {
            Block::Custom(block) => block.height.is_some(),
            Block::ExcerptBoundary { .. }
            | Block::FoldedBuffer { .. }
            | Block::BufferHeader { .. }
            | Block::Spacer { .. } => true,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Block::Custom(block) => block.height.unwrap_or(0),
            Block::ExcerptBoundary { height, .. }
            | Block::FoldedBuffer { height, .. }
            | Block::BufferHeader { height, .. }
            | Block::Spacer { height, .. } => *height,
        }
    }

    pub fn style(&self) -> BlockStyle {
        match self {
            Block::Custom(block) => block.style,
            Block::ExcerptBoundary { .. }
            | Block::FoldedBuffer { .. }
            | Block::BufferHeader { .. }
            | Block::Spacer { .. } => BlockStyle::Sticky,
        }
    }

    fn place_above(&self) -> bool {
        match self {
            Block::Custom(block) => matches!(block.placement, BlockPlacement::Above(_)),
            Block::FoldedBuffer { .. } => false,
            Block::ExcerptBoundary { .. } => true,
            Block::BufferHeader { .. } => true,
            Block::Spacer { is_below, .. } => !*is_below,
        }
    }

    pub fn place_near(&self) -> bool {
        match self {
            Block::Custom(block) => matches!(block.placement, BlockPlacement::Near(_)),
            Block::FoldedBuffer { .. } => false,
            Block::ExcerptBoundary { .. } => false,
            Block::BufferHeader { .. } => false,
            Block::Spacer { .. } => false,
        }
    }

    fn place_below(&self) -> bool {
        match self {
            Block::Custom(block) => matches!(
                block.placement,
                BlockPlacement::Below(_) | BlockPlacement::Near(_)
            ),
            Block::FoldedBuffer { .. } => false,
            Block::ExcerptBoundary { .. } => false,
            Block::BufferHeader { .. } => false,
            Block::Spacer { is_below, .. } => *is_below,
        }
    }

    fn is_replacement(&self) -> bool {
        match self {
            Block::Custom(block) => matches!(block.placement, BlockPlacement::Replace(_)),
            Block::FoldedBuffer { .. } => true,
            Block::ExcerptBoundary { .. } => false,
            Block::BufferHeader { .. } => false,
            Block::Spacer { .. } => false,
        }
    }

    fn is_header(&self) -> bool {
        match self {
            Block::Custom(_) => false,
            Block::FoldedBuffer { .. } => true,
            Block::ExcerptBoundary { .. } => true,
            Block::BufferHeader { .. } => true,
            Block::Spacer { .. } => false,
        }
    }

    pub fn is_buffer_header(&self) -> bool {
        match self {
            Block::Custom(_) => false,
            Block::FoldedBuffer { .. } => true,
            Block::ExcerptBoundary { .. } => false,
            Block::BufferHeader { .. } => true,
            Block::Spacer { .. } => false,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(block) => f.debug_struct("Custom").field("block", block).finish(),
            Self::FoldedBuffer {
                first_excerpt,
                height,
            } => f
                .debug_struct("FoldedBuffer")
                .field("first_excerpt", &first_excerpt)
                .field("height", height)
                .finish(),
            Self::ExcerptBoundary { excerpt, height } => f
                .debug_struct("ExcerptBoundary")
                .field("excerpt", excerpt)
                .field("height", height)
                .finish(),
            Self::BufferHeader { excerpt, height } => f
                .debug_struct("BufferHeader")
                .field("excerpt", excerpt)
                .field("height", height)
                .finish(),
            Self::Spacer {
                id,
                height,
                is_below: _,
            } => f
                .debug_struct("Spacer")
                .field("id", id)
                .field("height", height)
                .finish(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct TransformSummary {
    input_rows: WrapRow,
    output_rows: BlockRow,
    longest_row: BlockRow,
    longest_row_chars: u32,
}

pub struct BlockChunks<'a> {
    transforms: sum_tree::Cursor<'a, 'static, Transform, Dimensions<BlockRow, WrapRow>>,
    input_chunks: wrap_map::WrapChunks<'a>,
    input_chunk: Chunk<'a>,
    output_row: BlockRow,
    max_output_row: BlockRow,
    line_count_overflow: RowDelta,
    masked: bool,
}

#[derive(Clone)]
pub struct BlockRows<'a> {
    transforms: sum_tree::Cursor<'a, 'static, Transform, Dimensions<BlockRow, WrapRow>>,
    input_rows: wrap_map::WrapRows<'a>,
    output_row: BlockRow,
    started: bool,
}

impl BlockMap {
    #[ztracing::instrument(skip_all)]
    pub fn new(
        wrap_snapshot: WrapSnapshot,
        buffer_header_height: u32,
        excerpt_header_height: u32,
    ) -> Self {
        let row_count = wrap_snapshot.max_point().row() + WrapRow(1);
        let mut transforms = SumTree::default();
        push_isomorphic(&mut transforms, row_count - WrapRow(0), &wrap_snapshot);
        let map = Self {
            next_block_id: AtomicUsize::new(0),
            custom_blocks: Vec::new(),
            custom_blocks_by_id: TreeMap::default(),
            folded_buffers: HashSet::default(),
            buffers_with_disabled_headers: HashSet::default(),
            transforms: RefCell::new(transforms),
            wrap_snapshot: RefCell::new(wrap_snapshot.clone()),
            buffer_header_height,
            excerpt_header_height,
        };
        map.sync(
            &wrap_snapshot,
            Patch::new(vec![Edit {
                old: WrapRow(0)..row_count,
                new: WrapRow(0)..row_count,
            }]),
            None,
            None,
        );
        map
    }

    #[ztracing::instrument(skip_all)]
    pub(crate) fn read(
        &self,
        wrap_snapshot: WrapSnapshot,
        edits: WrapPatch,
        companion_wrap_edits: Option<(&WrapSnapshot, &WrapPatch)>,
        companion: Option<(&Companion, EntityId)>,
    ) -> BlockMapReader<'_> {
        self.sync(&wrap_snapshot, edits, companion_wrap_edits, companion);
        *self.wrap_snapshot.borrow_mut() = wrap_snapshot.clone();
        BlockMapReader {
            blocks: &self.custom_blocks,
            snapshot: BlockSnapshot {
                wrap_snapshot,
                transforms: self.transforms.borrow().clone(),
                custom_blocks_by_id: self.custom_blocks_by_id.clone(),
                buffer_header_height: self.buffer_header_height,
                excerpt_header_height: self.excerpt_header_height,
            },
        }
    }

    #[ztracing::instrument(skip_all)]
    pub(crate) fn write<'a>(
        &'a mut self,
        wrap_snapshot: WrapSnapshot,
        edits: WrapPatch,
        companion_wrap_edits: Option<(&'a WrapSnapshot, &'a WrapPatch)>,
        companion: Option<(&'a Companion, EntityId)>,
    ) -> BlockMapWriter<'a> {
        self.sync(&wrap_snapshot, edits, companion_wrap_edits, companion);
        *self.wrap_snapshot.borrow_mut() = wrap_snapshot;
        let companion = match (companion_wrap_edits, companion) {
            (Some(_), None) | (None, Some(_)) => unreachable!(),
            (None, None) => None,
            (Some(companion_wrap_edits), Some(companion)) => Some(BlockMapWriterCompanion {
                companion: companion.0,
                snapshot: companion_wrap_edits.0,
                entity: companion.1,
            }),
        };
        BlockMapWriter {
            block_map: self,
            companion,
        }
    }

    #[ztracing::instrument(skip_all, fields(edits = ?edits))]
    fn sync(
        &self,
        wrap_snapshot: &WrapSnapshot,
        mut edits: WrapPatch,
        companion_wrap_edits: Option<(&WrapSnapshot, &WrapPatch)>,
        companion: Option<(&Companion, EntityId)>,
    ) {
        let buffer = wrap_snapshot.buffer_snapshot();

        // Handle changing the last excerpt if it is empty.
        if buffer.trailing_excerpt_update_count()
            != self
                .wrap_snapshot
                .borrow()
                .buffer_snapshot()
                .trailing_excerpt_update_count()
        {
            let max_point = wrap_snapshot.max_point();
            let edit_start = wrap_snapshot.prev_row_boundary(max_point);
            let edit_end = max_point.row() + WrapRow(1); // this is end of file
            edits = edits.compose([WrapEdit {
                old: edit_start..edit_end,
                new: edit_start..edit_end,
            }]);
        }

        // Pull in companion edits to ensure we recompute spacers in ranges that have changed in the companion.
        if let Some((companion_new_snapshot, companion_edits)) = companion_wrap_edits
            && let Some((companion, display_map_id)) = companion
        {
            let mut companion_edits_in_my_space: Vec<WrapEdit> = companion_edits
                .clone()
                .into_inner()
                .iter()
                .map(|edit| {
                    let companion_start = companion_new_snapshot
                        .to_point(WrapPoint::new(edit.new.start, 0), Bias::Left);
                    let companion_end = companion_new_snapshot
                        .to_point(WrapPoint::new(edit.new.end, 0), Bias::Left);

                    let my_start = companion
                        .convert_rows_from_companion(
                            display_map_id,
                            wrap_snapshot.buffer_snapshot(),
                            companion_new_snapshot.buffer_snapshot(),
                            (
                                Bound::Included(companion_start),
                                Bound::Included(companion_start),
                            ),
                        )
                        .first()
                        .and_then(|t| t.boundaries.first())
                        .map(|(_, range)| range.start)
                        .unwrap_or(wrap_snapshot.buffer_snapshot().max_point());
                    let my_end = companion
                        .convert_rows_from_companion(
                            display_map_id,
                            wrap_snapshot.buffer_snapshot(),
                            companion_new_snapshot.buffer_snapshot(),
                            (
                                Bound::Included(companion_end),
                                Bound::Included(companion_end),
                            ),
                        )
                        .first()
                        .and_then(|t| t.boundaries.last())
                        .map(|(_, range)| range.end)
                        .unwrap_or(wrap_snapshot.buffer_snapshot().max_point());

                    let mut my_start = wrap_snapshot.make_wrap_point(my_start, Bias::Left);
                    let mut my_end = wrap_snapshot.make_wrap_point(my_end, Bias::Left);
                    if my_end.column() > 0 || my_end == wrap_snapshot.max_point() {
                        *my_end.row_mut() += 1;
                        *my_end.column_mut() = 0;
                    }

                    // Empty edits won't survive Patch::compose, but we still need to make sure
                    // we recompute spacers when we get them.
                    if my_start.row() == my_end.row() {
                        if my_end.row() <= wrap_snapshot.max_point().row() {
                            *my_end.row_mut() += 1;
                            *my_end.column_mut() = 0;
                        } else if my_start.row() > WrapRow(0) {
                            *my_start.row_mut() += 1;
                            *my_start.column_mut() = 0;
                        }
                    }

                    WrapEdit {
                        old: my_start.row()..my_end.row(),
                        new: my_start.row()..my_end.row(),
                    }
                })
                .collect();

            companion_edits_in_my_space.sort_by_key(|edit| edit.old.start);
            let mut merged_edits: Vec<WrapEdit> = Vec::new();
            for edit in companion_edits_in_my_space {
                if let Some(last) = merged_edits.last_mut() {
                    if edit.old.start <= last.old.end {
                        last.old.end = last.old.end.max(edit.old.end);
                        last.new.end = last.new.end.max(edit.new.end);
                        continue;
                    }
                }
                merged_edits.push(edit);
            }

            edits = edits.compose(merged_edits);
        }

        let edits = edits.into_inner();
        if edits.is_empty() {
            return;
        }

        let mut transforms = self.transforms.borrow_mut();
        let mut new_transforms = SumTree::default();
        let mut cursor = transforms.cursor::<WrapRow>(());
        let mut last_block_ix = 0;
        let mut blocks_in_edit = Vec::new();
        let mut edits = edits.into_iter().peekable();

        let mut inlay_point_cursor = wrap_snapshot.inlay_point_cursor();
        let mut tab_point_cursor = wrap_snapshot.tab_point_cursor();
        let mut fold_point_cursor = wrap_snapshot.fold_point_cursor();
        let mut wrap_point_cursor = wrap_snapshot.wrap_point_cursor();

        while let Some(edit) = edits.next() {
            let span = ztracing::debug_span!("while edits", edit = ?edit);
            let _enter = span.enter();

            let mut old_start = edit.old.start;
            let mut new_start = edit.new.start;

            // Only preserve transforms that:
            // * Strictly precedes this edit
            // * Isomorphic transforms that end *at* the start of the edit
            // * Below blocks that end at the start of the edit
            // However, if we hit a replace block that ends at the start of the edit we want to reconstruct it.
            new_transforms.append(cursor.slice(&old_start, Bias::Left), ());
            if let Some(transform) = cursor.item()
                && transform.summary.input_rows > WrapRow(0)
                && cursor.end() == old_start
                && transform.block.as_ref().is_none_or(|b| !b.is_replacement())
            {
                // Preserve the transform (push and next)
                new_transforms.push(transform.clone(), ());
                cursor.next();

                // Preserve below blocks at start of edit
                while let Some(transform) = cursor.item() {
                    if transform.block.as_ref().is_some_and(|b| b.place_below()) {
                        new_transforms.push(transform.clone(), ());
                        cursor.next();
                    } else {
                        break;
                    }
                }
            }

            // Ensure the edit starts at a transform boundary.
            // If the edit starts within an isomorphic transform, preserve its prefix
            // If the edit lands within a replacement block, expand the edit to include the start of the replaced input range
            let transform = cursor.item().unwrap();
            let transform_rows_before_edit = old_start - *cursor.start();
            if transform_rows_before_edit > RowDelta(0) {
                if transform.block.is_none() {
                    // Preserve any portion of the old isomorphic transform that precedes this edit.
                    push_isomorphic(
                        &mut new_transforms,
                        transform_rows_before_edit,
                        wrap_snapshot,
                    );
                } else {
                    // We landed within a block that replaces some lines, so we
                    // extend the edit to start at the beginning of the
                    // replacement.
                    debug_assert!(transform.summary.input_rows > WrapRow(0));
                    old_start -= transform_rows_before_edit;
                    new_start -= transform_rows_before_edit;
                }
            }

            // Decide where the edit ends
            // * It should end at a transform boundary
            // * Coalesce edits that intersect the same transform
            let mut old_end = edit.old.end;
            let mut new_end = edit.new.end;
            loop {
                let span = ztracing::debug_span!("decide where edit ends loop");
                let _enter = span.enter();
                // Seek to the transform starting at or after the end of the edit
                cursor.seek(&old_end, Bias::Left);
                cursor.next();

                // Extend edit to the end of the discarded transform so it is reconstructed in full
                let transform_rows_after_edit = *cursor.start() - old_end;
                old_end += transform_rows_after_edit;
                new_end += transform_rows_after_edit;

                // Combine this edit with any subsequent edits that intersect the same transform.
                while let Some(next_edit) = edits.peek() {
                    if next_edit.old.start <= *cursor.start() {
                        old_end = next_edit.old.end;
                        new_end = next_edit.new.end;
                        cursor.seek(&old_end, Bias::Left);
                        cursor.next();
                        edits.next();
                    } else {
                        break;
                    }
                }

                if *cursor.start() == old_end {
                    break;
                }
            }

            // Discard below blocks at the end of the edit. They'll be reconstructed.
            while let Some(transform) = cursor.item() {
                if transform
                    .block
                    .as_ref()
                    .is_some_and(|b| b.place_below() || matches!(b, Block::Spacer { .. }))
                {
                    cursor.next();
                } else {
                    break;
                }
            }

            // Find the blocks within this edited region.
            let new_buffer_start = wrap_snapshot.to_point(WrapPoint::new(new_start, 0), Bias::Left);
            let start_bound = Bound::Included(new_buffer_start);
            let start_block_ix =
                match self.custom_blocks[last_block_ix..].binary_search_by(|probe| {
                    probe
                        .start()
                        .to_point(buffer)
                        .cmp(&new_buffer_start)
                        // Move left until we find the index of the first block starting within this edit
                        .then(Ordering::Greater)
                }) {
                    Ok(ix) | Err(ix) => last_block_ix + ix,
                };

            let end_bound;
            let end_block_ix = if new_end > wrap_snapshot.max_point().row() {
                end_bound = Bound::Unbounded;
                self.custom_blocks.len()
            } else {
                let new_buffer_end = wrap_snapshot.to_point(WrapPoint::new(new_end, 0), Bias::Left);
                end_bound = Bound::Excluded(new_buffer_end);
                match self.custom_blocks[start_block_ix..].binary_search_by(|probe| {
                    probe
                        .start()
                        .to_point(buffer)
                        .cmp(&new_buffer_end)
                        .then(Ordering::Greater)
                }) {
                    Ok(ix) | Err(ix) => start_block_ix + ix,
                }
            };
            last_block_ix = end_block_ix;

            debug_assert!(blocks_in_edit.is_empty());
            // + 8 is chosen arbitrarily to cover some multibuffer headers
            blocks_in_edit
                .reserve(end_block_ix - start_block_ix + if buffer.is_singleton() { 0 } else { 8 });

            blocks_in_edit.extend(
                self.custom_blocks[start_block_ix..end_block_ix]
                    .iter()
                    .filter_map(|block| {
                        let placement = block.placement.to_wrap_row(wrap_snapshot)?;
                        if let BlockPlacement::Above(row) = placement
                            && row < new_start
                        {
                            return None;
                        }
                        Some((placement, Block::Custom(block.clone())))
                    }),
            );

            blocks_in_edit.extend(self.header_and_footer_blocks(
                buffer,
                (start_bound, end_bound),
                |point, bias| {
                    wrap_point_cursor
                        .map(
                            tab_point_cursor
                                .map(fold_point_cursor.map(inlay_point_cursor.map(point), bias)),
                        )
                        .row()
                },
            ));

            if let Some((companion_snapshot, _)) = companion_wrap_edits
                && let Some((companion, display_map_id)) = companion
            {
                blocks_in_edit.extend(self.spacer_blocks(
                    (start_bound, end_bound),
                    wrap_snapshot,
                    companion_snapshot,
                    companion,
                    display_map_id,
                ));
            }

            BlockMap::sort_blocks(&mut blocks_in_edit);

            // For each of these blocks, insert a new isomorphic transform preceding the block,
            // and then insert the block itself.
            let mut just_processed_folded_buffer = false;
            for (block_placement, block) in blocks_in_edit.drain(..) {
                let span =
                    ztracing::debug_span!("for block in edits", block_height = block.height());
                let _enter = span.enter();

                let mut summary = TransformSummary {
                    input_rows: WrapRow(0),
                    output_rows: BlockRow(block.height()),
                    longest_row: BlockRow(0),
                    longest_row_chars: 0,
                };

                let rows_before_block;
                match block_placement {
                    BlockPlacement::Above(position) => {
                        rows_before_block = position - new_transforms.summary().input_rows;
                        just_processed_folded_buffer = false;
                    }
                    BlockPlacement::Near(position) | BlockPlacement::Below(position) => {
                        if just_processed_folded_buffer {
                            continue;
                        }
                        if position + RowDelta(1) < new_transforms.summary().input_rows {
                            continue;
                        }
                        rows_before_block =
                            (position + RowDelta(1)) - new_transforms.summary().input_rows;
                    }
                    BlockPlacement::Replace(ref range) => {
                        rows_before_block = *range.start() - new_transforms.summary().input_rows;
                        summary.input_rows = WrapRow(1) + (*range.end() - *range.start());
                        just_processed_folded_buffer = matches!(block, Block::FoldedBuffer { .. });
                    }
                }

                push_isomorphic(&mut new_transforms, rows_before_block, wrap_snapshot);
                new_transforms.push(
                    Transform {
                        summary,
                        block: Some(block),
                    },
                    (),
                );
            }

            // Insert an isomorphic transform after the final block.
            let rows_after_last_block =
                RowDelta(new_end.0).saturating_sub(RowDelta(new_transforms.summary().input_rows.0));
            push_isomorphic(&mut new_transforms, rows_after_last_block, wrap_snapshot);
        }

        new_transforms.append(cursor.suffix(), ());
        debug_assert_eq!(
            new_transforms.summary().input_rows,
            wrap_snapshot.max_point().row() + WrapRow(1),
        );

        drop(cursor);
        *transforms = new_transforms;
    }

    #[ztracing::instrument(skip_all)]
    pub fn replace_blocks(&mut self, mut renderers: HashMap<CustomBlockId, RenderBlock>) {
        for block in &mut self.custom_blocks {
            if let Some(render) = renderers.remove(&block.id) {
                *block.render.lock() = render;
            }
        }
    }

    /// Guarantees that `wrap_row_for` is called with points in increasing order.
    #[ztracing::instrument(skip_all)]
    fn header_and_footer_blocks<'a, R, T>(
        &'a self,
        buffer: &'a multi_buffer::MultiBufferSnapshot,
        range: R,
        mut wrap_row_for: impl 'a + FnMut(Point, Bias) -> WrapRow,
    ) -> impl Iterator<Item = (BlockPlacement<WrapRow>, Block)> + 'a
    where
        R: RangeBounds<T>,
        T: multi_buffer::ToOffset,
    {
        let mut boundaries = buffer.excerpt_boundaries_in_range(range).peekable();

        std::iter::from_fn(move || {
            loop {
                let excerpt_boundary = boundaries.next()?;
                let wrap_row = wrap_row_for(Point::new(excerpt_boundary.row.0, 0), Bias::Left);

                let new_buffer_id = match (&excerpt_boundary.prev, &excerpt_boundary.next) {
                    (None, next) => Some(next.buffer_id),
                    (Some(prev), next) => {
                        if prev.buffer_id != next.buffer_id {
                            Some(next.buffer_id)
                        } else {
                            None
                        }
                    }
                };

                let mut height = 0;

                if let Some(new_buffer_id) = new_buffer_id {
                    let first_excerpt = excerpt_boundary.next.clone();
                    if self.buffers_with_disabled_headers.contains(&new_buffer_id) {
                        continue;
                    }
                    if self.folded_buffers.contains(&new_buffer_id) && buffer.show_headers() {
                        let mut last_excerpt_end_row = first_excerpt.end_row;

                        while let Some(next_boundary) = boundaries.peek() {
                            if next_boundary.next.buffer_id == new_buffer_id {
                                last_excerpt_end_row = next_boundary.next.end_row;
                            } else {
                                break;
                            }

                            boundaries.next();
                        }
                        let wrap_end_row = wrap_row_for(
                            Point::new(
                                last_excerpt_end_row.0,
                                buffer.line_len(last_excerpt_end_row),
                            ),
                            Bias::Right,
                        );

                        return Some((
                            BlockPlacement::Replace(wrap_row..=wrap_end_row),
                            Block::FoldedBuffer {
                                height: height + self.buffer_header_height,
                                first_excerpt,
                            },
                        ));
                    }
                }

                let starts_new_buffer = new_buffer_id.is_some();
                let block = if starts_new_buffer && buffer.show_headers() {
                    height += self.buffer_header_height;
                    Block::BufferHeader {
                        excerpt: excerpt_boundary.next,
                        height,
                    }
                } else if excerpt_boundary.prev.is_some() {
                    height += self.excerpt_header_height;
                    Block::ExcerptBoundary {
                        excerpt: excerpt_boundary.next,
                        height,
                    }
                } else {
                    continue;
                };

                return Some((BlockPlacement::Above(wrap_row), block));
            }
        })
    }

    fn spacer_blocks(
        &self,
        bounds: (Bound<MultiBufferPoint>, Bound<MultiBufferPoint>),
        wrap_snapshot: &WrapSnapshot,
        companion_snapshot: &WrapSnapshot,
        companion: &Companion,
        display_map_id: EntityId,
    ) -> Vec<(BlockPlacement<WrapRow>, Block)> {
        let our_buffer = wrap_snapshot.buffer_snapshot();
        let companion_buffer = companion_snapshot.buffer_snapshot();

        let row_mappings = companion.convert_rows_to_companion(
            display_map_id,
            companion_buffer,
            our_buffer,
            bounds,
        );

        let determine_spacer = |our_point: Point, their_point: Point, delta: i32| {
            let our_wrap = wrap_snapshot.make_wrap_point(our_point, Bias::Left).row();
            let companion_wrap = companion_snapshot
                .make_wrap_point(their_point, Bias::Left)
                .row();
            let new_delta = companion_wrap.0 as i32 - our_wrap.0 as i32;

            let spacer = if new_delta > delta {
                let height = (new_delta - delta) as u32;
                Some((our_wrap, height))
            } else {
                None
            };
            (new_delta, spacer)
        };

        let mut result = Vec::new();

        for row_mapping in row_mappings {
            let mut iter = row_mapping.boundaries.iter().cloned().peekable();

            let Some(((first_boundary, first_range), first_group)) =
                iter.peek().cloned().zip(row_mapping.first_group.clone())
            else {
                continue;
            };

            // Because we calculate spacers based on differences in wrap row
            // counts between the RHS and LHS for corresponding buffer points,
            // we need to calibrate our expectations based on the difference
            // in counts before the start of the edit. This difference in
            // counts should have been balanced already by spacers above this
            // edit, so we only need to insert spacers for when the difference
            // in counts diverges from that baseline value.
            let (our_baseline, their_baseline) = if first_group.start < first_boundary {
                (first_group.start, first_range.start)
            } else if let Some((prev_boundary, prev_range)) = row_mapping.prev_boundary {
                (prev_boundary, prev_range.end)
            } else {
                (first_boundary, first_range.start)
            };
            let our_baseline = wrap_snapshot
                .make_wrap_point(our_baseline, Bias::Left)
                .row();
            let their_baseline = companion_snapshot
                .make_wrap_point(their_baseline, Bias::Left)
                .row();

            let mut delta = their_baseline.0 as i32 - our_baseline.0 as i32;

            if first_group.start < first_boundary {
                let mut current_boundary = first_boundary;
                let current_range = first_range;
                while let Some((next_boundary, next_range)) = iter.peek().cloned()
                    && next_range.end <= current_range.end
                {
                    iter.next();
                    current_boundary = next_boundary;
                }

                let (new_delta, spacer) =
                    determine_spacer(current_boundary, current_range.end, delta);

                delta = new_delta;
                if let Some((wrap_row, height)) = spacer {
                    result.push((
                        BlockPlacement::Above(wrap_row),
                        Block::Spacer {
                            id: SpacerId(self.next_block_id.fetch_add(1, SeqCst)),
                            height,
                            is_below: false,
                        },
                    ));
                }
            }

            while let Some((boundary, range)) = iter.next() {
                let mut current_boundary = boundary;
                let current_range = range;

                // This can only occur at the end of an excerpt.
                if current_boundary.column > 0 {
                    debug_assert_eq!(current_boundary, row_mapping.source_excerpt_end);
                    break;
                }

                // Align the two sides at the start of this group.
                let (delta_at_start, mut spacer_at_start) =
                    determine_spacer(current_boundary, current_range.start, delta);
                delta = delta_at_start;

                while let Some((next_boundary, next_range)) = iter.peek()
                    && next_range.end <= current_range.end
                {
                    if let Some((wrap_row, height)) = spacer_at_start.take() {
                        result.push((
                            BlockPlacement::Above(wrap_row),
                            Block::Spacer {
                                id: SpacerId(self.next_block_id.fetch_add(1, SeqCst)),
                                height,
                                is_below: false,
                            },
                        ));
                    }

                    current_boundary = *next_boundary;
                    iter.next();
                }

                // This can only occur at the end of an excerpt.
                if current_boundary.column > 0 {
                    debug_assert_eq!(current_boundary, row_mapping.source_excerpt_end);
                    break;
                }

                let (delta_at_end, spacer_at_end) =
                    determine_spacer(current_boundary, current_range.end, delta);
                delta = delta_at_end;

                if let Some((wrap_row, mut height)) = spacer_at_start {
                    if let Some((_, additional_height)) = spacer_at_end {
                        height += additional_height;
                    }
                    result.push((
                        BlockPlacement::Above(wrap_row),
                        Block::Spacer {
                            id: SpacerId(self.next_block_id.fetch_add(1, SeqCst)),
                            height,
                            is_below: false,
                        },
                    ));
                } else if let Some((wrap_row, height)) = spacer_at_end {
                    result.push((
                        BlockPlacement::Above(wrap_row),
                        Block::Spacer {
                            id: SpacerId(self.next_block_id.fetch_add(1, SeqCst)),
                            height,
                            is_below: false,
                        },
                    ));
                }
            }

            let (last_boundary, _last_range) = row_mapping.boundaries.last().cloned().unwrap();
            if last_boundary == row_mapping.source_excerpt_end {
                let (_new_delta, spacer) =
                    determine_spacer(last_boundary, row_mapping.target_excerpt_end, delta);
                if let Some((wrap_row, height)) = spacer {
                    result.push((
                        BlockPlacement::Below(wrap_row),
                        Block::Spacer {
                            id: SpacerId(self.next_block_id.fetch_add(1, SeqCst)),
                            height,
                            is_below: true,
                        },
                    ));
                }
            }
        }

        result
    }

    #[ztracing::instrument(skip_all)]
    fn sort_blocks(blocks: &mut Vec<(BlockPlacement<WrapRow>, Block)>) {
        blocks.sort_unstable_by(|(placement_a, block_a), (placement_b, block_b)| {
            placement_a
                .start()
                .cmp(placement_b.start())
                .then_with(|| placement_b.end().cmp(placement_a.end()))
                .then_with(|| placement_a.tie_break().cmp(&placement_b.tie_break()))
                .then_with(|| {
                    if block_a.is_header() {
                        Ordering::Less
                    } else if block_b.is_header() {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
                .then_with(|| match (block_a, block_b) {
                    (
                        Block::ExcerptBoundary {
                            excerpt: excerpt_a, ..
                        }
                        | Block::BufferHeader {
                            excerpt: excerpt_a, ..
                        },
                        Block::ExcerptBoundary {
                            excerpt: excerpt_b, ..
                        }
                        | Block::BufferHeader {
                            excerpt: excerpt_b, ..
                        },
                    ) => Some(excerpt_a.id).cmp(&Some(excerpt_b.id)),
                    (
                        Block::ExcerptBoundary { .. } | Block::BufferHeader { .. },
                        Block::Spacer { .. } | Block::Custom(_),
                    ) => Ordering::Less,
                    (
                        Block::Spacer { .. } | Block::Custom(_),
                        Block::ExcerptBoundary { .. } | Block::BufferHeader { .. },
                    ) => Ordering::Greater,
                    (Block::Spacer { .. }, Block::Custom(_)) => Ordering::Less,
                    (Block::Custom(_), Block::Spacer { .. }) => Ordering::Greater,
                    (Block::Custom(block_a), Block::Custom(block_b)) => block_a
                        .priority
                        .cmp(&block_b.priority)
                        .then_with(|| block_a.id.cmp(&block_b.id)),
                    _ => {
                        unreachable!("comparing blocks: {block_a:?} vs {block_b:?}")
                    }
                })
        });
        blocks.dedup_by(|right, left| match (left.0.clone(), right.0.clone()) {
            (BlockPlacement::Replace(range), BlockPlacement::Above(row))
            | (BlockPlacement::Replace(range), BlockPlacement::Below(row)) => range.contains(&row),
            (BlockPlacement::Replace(range_a), BlockPlacement::Replace(range_b)) => {
                if range_a.end() >= range_b.start() && range_a.start() <= range_b.end() {
                    left.0 = BlockPlacement::Replace(
                        *range_a.start()..=*range_a.end().max(range_b.end()),
                    );
                    true
                } else {
                    false
                }
            }
            _ => false,
        });
    }
}

#[ztracing::instrument(skip(tree, wrap_snapshot))]
fn push_isomorphic(tree: &mut SumTree<Transform>, rows: RowDelta, wrap_snapshot: &WrapSnapshot) {
    if rows == RowDelta(0) {
        return;
    }

    let wrap_row_start = tree.summary().input_rows;
    let wrap_row_end = wrap_row_start + rows;
    let wrap_summary = wrap_snapshot.text_summary_for_range(wrap_row_start..wrap_row_end);
    let summary = TransformSummary {
        input_rows: WrapRow(rows.0),
        output_rows: BlockRow(rows.0),
        longest_row: BlockRow(wrap_summary.longest_row),
        longest_row_chars: wrap_summary.longest_row_chars,
    };
    let mut merged = false;
    tree.update_last(
        |last_transform| {
            if last_transform.block.is_none() {
                last_transform.summary.add_summary(&summary);
                merged = true;
            }
        },
        (),
    );
    if !merged {
        tree.push(
            Transform {
                summary,
                block: None,
            },
            (),
        );
    }
}

impl BlockPoint {
    pub fn new(row: BlockRow, column: u32) -> Self {
        Self(Point::new(row.0, column))
    }
}

impl Deref for BlockPoint {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BlockPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for BlockMapReader<'_> {
    type Target = BlockSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl DerefMut for BlockMapReader<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.snapshot
    }
}

impl BlockMapReader<'_> {
    #[ztracing::instrument(skip_all)]
    pub fn row_for_block(&self, block_id: CustomBlockId) -> Option<BlockRow> {
        let block = self.blocks.iter().find(|block| block.id == block_id)?;
        let buffer_row = block
            .start()
            .to_point(self.wrap_snapshot.buffer_snapshot())
            .row;
        let wrap_row = self
            .wrap_snapshot
            .make_wrap_point(Point::new(buffer_row, 0), Bias::Left)
            .row();
        let start_wrap_row = self
            .wrap_snapshot
            .prev_row_boundary(WrapPoint::new(wrap_row, 0));
        let end_wrap_row = self
            .wrap_snapshot
            .next_row_boundary(WrapPoint::new(wrap_row, 0))
            .unwrap_or(self.wrap_snapshot.max_point().row() + WrapRow(1));

        let mut cursor = self.transforms.cursor::<Dimensions<WrapRow, BlockRow>>(());
        cursor.seek(&start_wrap_row, Bias::Left);
        while let Some(transform) = cursor.item() {
            if cursor.start().0 > end_wrap_row {
                break;
            }

            if let Some(BlockId::Custom(id)) = transform.block.as_ref().map(|block| block.id())
                && id == block_id
            {
                return Some(cursor.start().1);
            }
            cursor.next();
        }

        None
    }
}

impl BlockMapWriter<'_> {
    #[ztracing::instrument(skip_all)]
    pub fn insert(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
    ) -> Vec<CustomBlockId> {
        let blocks = blocks.into_iter();
        let mut ids = Vec::with_capacity(blocks.size_hint().1.unwrap_or(0));
        let mut edits = Patch::default();
        let wrap_snapshot = &*self.block_map.wrap_snapshot.borrow();
        let buffer = wrap_snapshot.buffer_snapshot();

        let mut previous_wrap_row_range: Option<Range<WrapRow>> = None;
        for block in blocks {
            if let BlockPlacement::Replace(_) = &block.placement {
                debug_assert!(block.height.unwrap() > 0);
            }

            let id = CustomBlockId(self.block_map.next_block_id.fetch_add(1, SeqCst));
            ids.push(id);

            let start = block.placement.start().to_point(buffer);
            let end = block.placement.end().to_point(buffer);
            let start_wrap_row = wrap_snapshot.make_wrap_point(start, Bias::Left).row();
            let end_wrap_row = wrap_snapshot.make_wrap_point(end, Bias::Left).row();

            let (start_row, end_row) = {
                previous_wrap_row_range.take_if(|range| {
                    !range.contains(&start_wrap_row) || !range.contains(&end_wrap_row)
                });
                let range = previous_wrap_row_range.get_or_insert_with(|| {
                    let start_row =
                        wrap_snapshot.prev_row_boundary(WrapPoint::new(start_wrap_row, 0));
                    let end_row = wrap_snapshot
                        .next_row_boundary(WrapPoint::new(end_wrap_row, 0))
                        .unwrap_or(wrap_snapshot.max_point().row() + WrapRow(1));
                    start_row..end_row
                });
                (range.start, range.end)
            };
            let block_ix = match self
                .block_map
                .custom_blocks
                .binary_search_by(|probe| probe.placement.cmp(&block.placement, buffer))
            {
                Ok(ix) | Err(ix) => ix,
            };
            let new_block = Arc::new(CustomBlock {
                id,
                placement: block.placement,
                height: block.height,
                render: Arc::new(Mutex::new(block.render)),
                style: block.style,
                priority: block.priority,
            });
            self.block_map
                .custom_blocks
                .insert(block_ix, new_block.clone());
            self.block_map.custom_blocks_by_id.insert(id, new_block);

            edits = edits.compose([Edit {
                old: start_row..end_row,
                new: start_row..end_row,
            }]);
        }

        let default_patch = Patch::default();
        let (companion_snapshot, companion) = self
            .companion
            .as_ref()
            .map(|companion| {
                (
                    (companion.snapshot, &default_patch),
                    (companion.companion, companion.entity),
                )
            })
            .unzip();
        self.block_map
            .sync(wrap_snapshot, edits, companion_snapshot, companion);
        ids
    }

    #[ztracing::instrument(skip_all)]
    pub fn resize(&mut self, mut heights: HashMap<CustomBlockId, u32>) {
        let wrap_snapshot = &*self.block_map.wrap_snapshot.borrow();
        let buffer = wrap_snapshot.buffer_snapshot();
        let mut edits = Patch::default();
        let mut last_block_buffer_row = None;

        for block in &mut self.block_map.custom_blocks {
            if let Some(new_height) = heights.remove(&block.id) {
                if let BlockPlacement::Replace(_) = &block.placement {
                    debug_assert!(new_height > 0);
                }

                if block.height != Some(new_height) {
                    let new_block = CustomBlock {
                        id: block.id,
                        placement: block.placement.clone(),
                        height: Some(new_height),
                        style: block.style,
                        render: block.render.clone(),
                        priority: block.priority,
                    };
                    let new_block = Arc::new(new_block);
                    *block = new_block.clone();
                    self.block_map
                        .custom_blocks_by_id
                        .insert(block.id, new_block);

                    let start_row = block.placement.start().to_point(buffer).row;
                    let end_row = block.placement.end().to_point(buffer).row;
                    if last_block_buffer_row != Some(end_row) {
                        last_block_buffer_row = Some(end_row);
                        let start_wrap_row = wrap_snapshot
                            .make_wrap_point(Point::new(start_row, 0), Bias::Left)
                            .row();
                        let end_wrap_row = wrap_snapshot
                            .make_wrap_point(Point::new(end_row, 0), Bias::Left)
                            .row();
                        let start =
                            wrap_snapshot.prev_row_boundary(WrapPoint::new(start_wrap_row, 0));
                        let end = wrap_snapshot
                            .next_row_boundary(WrapPoint::new(end_wrap_row, 0))
                            .unwrap_or(wrap_snapshot.max_point().row() + WrapRow(1));
                        edits.push(Edit {
                            old: start..end,
                            new: start..end,
                        })
                    }
                }
            }
        }

        let default_patch = Patch::default();
        let (companion_snapshot, companion) = self
            .companion
            .as_ref()
            .map(|companion| {
                (
                    (companion.snapshot, &default_patch),
                    (companion.companion, companion.entity),
                )
            })
            .unzip();
        self.block_map
            .sync(wrap_snapshot, edits, companion_snapshot, companion);
    }

    #[ztracing::instrument(skip_all)]
    pub fn remove(&mut self, block_ids: HashSet<CustomBlockId>) {
        let wrap_snapshot = &*self.block_map.wrap_snapshot.borrow();
        let buffer = wrap_snapshot.buffer_snapshot();
        let mut edits = Patch::default();
        let mut last_block_buffer_row = None;
        let mut previous_wrap_row_range: Option<Range<WrapRow>> = None;
        self.block_map.custom_blocks.retain(|block| {
            if block_ids.contains(&block.id) {
                let start = block.placement.start().to_point(buffer);
                let end = block.placement.end().to_point(buffer);
                if last_block_buffer_row != Some(end.row) {
                    last_block_buffer_row = Some(end.row);
                    let start_wrap_row = wrap_snapshot.make_wrap_point(start, Bias::Left).row();
                    let end_wrap_row = wrap_snapshot.make_wrap_point(end, Bias::Left).row();
                    let (start_row, end_row) = {
                        previous_wrap_row_range.take_if(|range| {
                            !range.contains(&start_wrap_row) || !range.contains(&end_wrap_row)
                        });
                        let range = previous_wrap_row_range.get_or_insert_with(|| {
                            let start_row =
                                wrap_snapshot.prev_row_boundary(WrapPoint::new(start_wrap_row, 0));
                            let end_row = wrap_snapshot
                                .next_row_boundary(WrapPoint::new(end_wrap_row, 0))
                                .unwrap_or(wrap_snapshot.max_point().row() + WrapRow(1));
                            start_row..end_row
                        });
                        (range.start, range.end)
                    };

                    edits.push(Edit {
                        old: start_row..end_row,
                        new: start_row..end_row,
                    })
                }
                false
            } else {
                true
            }
        });
        self.block_map
            .custom_blocks_by_id
            .retain(|id, _| !block_ids.contains(id));
        let default_patch = Patch::default();
        let (companion_snapshot, companion) = self
            .companion
            .as_ref()
            .map(|companion| {
                (
                    (companion.snapshot, &default_patch),
                    (companion.companion, companion.entity),
                )
            })
            .unzip();

        self.block_map
            .sync(wrap_snapshot, edits, companion_snapshot, companion);
    }

    #[ztracing::instrument(skip_all)]
    pub fn remove_intersecting_replace_blocks(
        &mut self,
        ranges: impl IntoIterator<Item = Range<MultiBufferOffset>>,
        inclusive: bool,
    ) {
        let wrap_snapshot = self.block_map.wrap_snapshot.borrow();
        let mut blocks_to_remove = HashSet::default();
        for range in ranges {
            for block in self.blocks_intersecting_buffer_range(range, inclusive) {
                if matches!(block.placement, BlockPlacement::Replace(_)) {
                    blocks_to_remove.insert(block.id);
                }
            }
        }
        drop(wrap_snapshot);
        self.remove(blocks_to_remove);
    }

    pub fn disable_header_for_buffer(&mut self, buffer_id: BufferId) {
        self.block_map
            .buffers_with_disabled_headers
            .insert(buffer_id);
    }

    #[ztracing::instrument(skip_all)]
    pub fn fold_buffers(
        &mut self,
        buffer_ids: impl IntoIterator<Item = BufferId>,
        multi_buffer: &MultiBuffer,
        cx: &App,
    ) {
        self.fold_or_unfold_buffers(true, buffer_ids, multi_buffer, cx);
    }

    #[ztracing::instrument(skip_all)]
    pub fn unfold_buffers(
        &mut self,
        buffer_ids: impl IntoIterator<Item = BufferId>,
        multi_buffer: &MultiBuffer,
        cx: &App,
    ) {
        self.fold_or_unfold_buffers(false, buffer_ids, multi_buffer, cx);
    }

    #[ztracing::instrument(skip_all)]
    fn fold_or_unfold_buffers(
        &mut self,
        fold: bool,
        buffer_ids: impl IntoIterator<Item = BufferId>,
        multi_buffer: &MultiBuffer,
        cx: &App,
    ) {
        let mut ranges = Vec::new();
        for buffer_id in buffer_ids {
            if fold {
                self.block_map.folded_buffers.insert(buffer_id);
            } else {
                self.block_map.folded_buffers.remove(&buffer_id);
            }
            ranges.extend(multi_buffer.excerpt_ranges_for_buffer(buffer_id, cx));
        }
        ranges.sort_unstable_by_key(|range| range.start);

        let mut edits = Patch::default();
        let wrap_snapshot = self.block_map.wrap_snapshot.borrow().clone();
        for range in ranges {
            let last_edit_row = cmp::min(
                wrap_snapshot.make_wrap_point(range.end, Bias::Right).row() + WrapRow(1),
                wrap_snapshot.max_point().row(),
            ) + WrapRow(1);
            let range = wrap_snapshot.make_wrap_point(range.start, Bias::Left).row()..last_edit_row;
            edits.push(Edit {
                old: range.clone(),
                new: range,
            });
        }

        let default_patch = Patch::default();
        let (companion_snapshot, companion) = self
            .companion
            .as_ref()
            .map(|companion| {
                (
                    (companion.snapshot, &default_patch),
                    (companion.companion, companion.entity),
                )
            })
            .unzip();
        self.block_map
            .sync(&wrap_snapshot, edits, companion_snapshot, companion);
    }

    #[ztracing::instrument(skip_all)]
    fn blocks_intersecting_buffer_range(
        &self,
        range: Range<MultiBufferOffset>,
        inclusive: bool,
    ) -> &[Arc<CustomBlock>] {
        if range.is_empty() && !inclusive {
            return &[];
        }
        let wrap_snapshot = self.block_map.wrap_snapshot.borrow();
        let buffer = wrap_snapshot.buffer_snapshot();

        let start_block_ix = match self.block_map.custom_blocks.binary_search_by(|block| {
            let block_end = block.end().to_offset(buffer);
            block_end.cmp(&range.start).then(Ordering::Greater)
        }) {
            Ok(ix) | Err(ix) => ix,
        };
        let end_block_ix =
            match self.block_map.custom_blocks[start_block_ix..].binary_search_by(|block| {
                let block_start = block.start().to_offset(buffer);
                block_start.cmp(&range.end).then(if inclusive {
                    Ordering::Less
                } else {
                    Ordering::Greater
                })
            }) {
                Ok(ix) | Err(ix) => ix,
            };

        &self.block_map.custom_blocks[start_block_ix..][..end_block_ix]
    }
}

impl BlockSnapshot {
    #[cfg(test)]
    #[ztracing::instrument(skip_all)]
    pub fn text(&self) -> String {
        self.chunks(
            BlockRow(0)..self.transforms.summary().output_rows,
            false,
            false,
            Highlights::default(),
        )
        .map(|chunk| chunk.text)
        .collect()
    }

    #[ztracing::instrument(skip_all)]
    pub(crate) fn chunks<'a>(
        &'a self,
        rows: Range<BlockRow>,
        language_aware: bool,
        masked: bool,
        highlights: Highlights<'a>,
    ) -> BlockChunks<'a> {
        let max_output_row = cmp::min(rows.end, self.transforms.summary().output_rows);

        let mut cursor = self.transforms.cursor::<Dimensions<BlockRow, WrapRow>>(());
        cursor.seek(&rows.start, Bias::Right);
        let transform_output_start = cursor.start().0;
        let transform_input_start = cursor.start().1;

        let mut input_start = transform_input_start;
        let mut input_end = transform_input_start;
        if let Some(transform) = cursor.item()
            && transform.block.is_none()
        {
            input_start += rows.start - transform_output_start;
            input_end += cmp::min(
                rows.end - transform_output_start,
                RowDelta(transform.summary.input_rows.0),
            );
        }

        BlockChunks {
            input_chunks: self.wrap_snapshot.chunks(
                input_start..input_end,
                language_aware,
                highlights,
            ),
            input_chunk: Default::default(),
            transforms: cursor,
            output_row: rows.start,
            line_count_overflow: RowDelta(0),
            max_output_row,
            masked,
        }
    }

    #[ztracing::instrument(skip_all)]
    pub(super) fn row_infos(&self, start_row: BlockRow) -> BlockRows<'_> {
        let mut cursor = self.transforms.cursor::<Dimensions<BlockRow, WrapRow>>(());
        cursor.seek(&start_row, Bias::Right);
        let Dimensions(output_start, input_start, _) = cursor.start();
        let overshoot = if cursor
            .item()
            .is_some_and(|transform| transform.block.is_none())
        {
            start_row - *output_start
        } else {
            RowDelta(0)
        };
        let input_start_row = *input_start + overshoot;
        BlockRows {
            transforms: cursor,
            input_rows: self.wrap_snapshot.row_infos(input_start_row),
            output_row: start_row,
            started: false,
        }
    }

    #[ztracing::instrument(skip_all)]
    pub fn blocks_in_range(
        &self,
        rows: Range<BlockRow>,
    ) -> impl Iterator<Item = (BlockRow, &Block)> {
        let mut cursor = self.transforms.cursor::<BlockRow>(());
        cursor.seek(&rows.start, Bias::Left);
        while *cursor.start() < rows.start && cursor.end() <= rows.start {
            cursor.next();
        }

        std::iter::from_fn(move || {
            while let Some(transform) = cursor.item() {
                let start_row = *cursor.start();
                if start_row > rows.end
                    || (start_row == rows.end
                        && transform
                            .block
                            .as_ref()
                            .is_some_and(|block| block.height() > 0))
                {
                    break;
                }
                if let Some(block) = &transform.block {
                    cursor.next();
                    return Some((start_row, block));
                } else {
                    cursor.next();
                }
            }
            None
        })
    }

    #[ztracing::instrument(skip_all)]
    pub(crate) fn sticky_header_excerpt(&self, position: f64) -> Option<StickyHeaderExcerpt<'_>> {
        let top_row = position as u32;
        let mut cursor = self.transforms.cursor::<BlockRow>(());
        cursor.seek(&BlockRow(top_row), Bias::Right);

        while let Some(transform) = cursor.item() {
            match &transform.block {
                Some(
                    Block::ExcerptBoundary { excerpt, .. } | Block::BufferHeader { excerpt, .. },
                ) => {
                    return Some(StickyHeaderExcerpt { excerpt });
                }
                Some(block) if block.is_buffer_header() => return None,
                _ => {
                    cursor.prev();
                    continue;
                }
            }
        }

        None
    }

    #[ztracing::instrument(skip_all)]
    pub fn block_for_id(&self, block_id: BlockId) -> Option<Block> {
        let buffer = self.wrap_snapshot.buffer_snapshot();
        let wrap_point = match block_id {
            BlockId::Custom(custom_block_id) => {
                let custom_block = self.custom_blocks_by_id.get(&custom_block_id)?;
                return Some(Block::Custom(custom_block.clone()));
            }
            BlockId::ExcerptBoundary(next_excerpt_id) => {
                let excerpt_range = buffer.range_for_excerpt(next_excerpt_id)?;
                self.wrap_snapshot
                    .make_wrap_point(excerpt_range.start, Bias::Left)
            }
            BlockId::FoldedBuffer(excerpt_id) => self
                .wrap_snapshot
                .make_wrap_point(buffer.range_for_excerpt(excerpt_id)?.start, Bias::Left),
            BlockId::Spacer(_) => return None,
        };
        let wrap_row = wrap_point.row();

        let mut cursor = self.transforms.cursor::<WrapRow>(());
        cursor.seek(&wrap_row, Bias::Left);

        while let Some(transform) = cursor.item() {
            if let Some(block) = transform.block.as_ref() {
                if block.id() == block_id {
                    return Some(block.clone());
                }
            } else if *cursor.start() > wrap_row {
                break;
            }

            cursor.next();
        }

        None
    }

    #[ztracing::instrument(skip_all)]
    pub fn max_point(&self) -> BlockPoint {
        let row = self
            .transforms
            .summary()
            .output_rows
            .saturating_sub(RowDelta(1));
        BlockPoint::new(row, self.line_len(row))
    }

    #[ztracing::instrument(skip_all)]
    pub fn longest_row(&self) -> BlockRow {
        self.transforms.summary().longest_row
    }

    #[ztracing::instrument(skip_all)]
    pub fn longest_row_in_range(&self, range: Range<BlockRow>) -> BlockRow {
        let mut cursor = self.transforms.cursor::<Dimensions<BlockRow, WrapRow>>(());
        cursor.seek(&range.start, Bias::Right);

        let mut longest_row = range.start;
        let mut longest_row_chars = 0;
        if let Some(transform) = cursor.item() {
            if transform.block.is_none() {
                let &Dimensions(output_start, input_start, _) = cursor.start();
                let overshoot = range.start - output_start;
                let wrap_start_row = input_start + WrapRow(overshoot.0);
                let wrap_end_row = cmp::min(
                    input_start + WrapRow((range.end - output_start).0),
                    cursor.end().1,
                );
                let summary = self
                    .wrap_snapshot
                    .text_summary_for_range(wrap_start_row..wrap_end_row);
                longest_row = BlockRow(range.start.0 + summary.longest_row);
                longest_row_chars = summary.longest_row_chars;
            }
            cursor.next();
        }

        let cursor_start_row = cursor.start().0;
        if range.end > cursor_start_row {
            let summary = cursor.summary::<_, TransformSummary>(&range.end, Bias::Right);
            if summary.longest_row_chars > longest_row_chars {
                longest_row = cursor_start_row + summary.longest_row;
                longest_row_chars = summary.longest_row_chars;
            }

            if let Some(transform) = cursor.item()
                && transform.block.is_none()
            {
                let &Dimensions(output_start, input_start, _) = cursor.start();
                let overshoot = range.end - output_start;
                let wrap_start_row = input_start;
                let wrap_end_row = input_start + overshoot;
                let summary = self
                    .wrap_snapshot
                    .text_summary_for_range(wrap_start_row..wrap_end_row);
                if summary.longest_row_chars > longest_row_chars {
                    longest_row = output_start + RowDelta(summary.longest_row);
                }
            }
        }

        longest_row
    }

    #[ztracing::instrument(skip_all)]
    pub(super) fn line_len(&self, row: BlockRow) -> u32 {
        let (start, _, item) =
            self.transforms
                .find::<Dimensions<BlockRow, WrapRow>, _>((), &row, Bias::Right);
        if let Some(transform) = item {
            let Dimensions(output_start, input_start, _) = start;
            let overshoot = row - output_start;
            if transform.block.is_some() {
                0
            } else {
                self.wrap_snapshot.line_len(input_start + overshoot)
            }
        } else if row == BlockRow(0) {
            0
        } else {
            panic!("row out of range");
        }
    }

    #[ztracing::instrument(skip_all)]
    pub(super) fn is_block_line(&self, row: BlockRow) -> bool {
        let (_, _, item) = self.transforms.find::<BlockRow, _>((), &row, Bias::Right);
        item.is_some_and(|t| t.block.is_some())
    }

    #[ztracing::instrument(skip_all)]
    pub(super) fn is_folded_buffer_header(&self, row: BlockRow) -> bool {
        let (_, _, item) = self.transforms.find::<BlockRow, _>((), &row, Bias::Right);
        let Some(transform) = item else {
            return false;
        };
        matches!(transform.block, Some(Block::FoldedBuffer { .. }))
    }

    #[ztracing::instrument(skip_all)]
    pub(super) fn is_line_replaced(&self, row: MultiBufferRow) -> bool {
        let wrap_point = self
            .wrap_snapshot
            .make_wrap_point(Point::new(row.0, 0), Bias::Left);
        let (_, _, item) = self
            .transforms
            .find::<WrapRow, _>((), &wrap_point.row(), Bias::Right);
        item.is_some_and(|transform| {
            transform
                .block
                .as_ref()
                .is_some_and(|block| block.is_replacement())
        })
    }

    #[ztracing::instrument(skip_all)]
    pub fn clip_point(&self, point: BlockPoint, bias: Bias) -> BlockPoint {
        let mut cursor = self.transforms.cursor::<Dimensions<BlockRow, WrapRow>>(());
        cursor.seek(&BlockRow(point.row), Bias::Right);

        let max_input_row = self.transforms.summary().input_rows;
        let mut search_left = (bias == Bias::Left && cursor.start().1 > WrapRow(0))
            || cursor.end().1 == max_input_row;
        let mut reversed = false;

        loop {
            if let Some(transform) = cursor.item() {
                let Dimensions(output_start_row, input_start_row, _) = cursor.start();
                let Dimensions(output_end_row, input_end_row, _) = cursor.end();
                let output_start = Point::new(output_start_row.0, 0);
                let input_start = Point::new(input_start_row.0, 0);
                let input_end = Point::new(input_end_row.0, 0);

                match transform.block.as_ref() {
                    Some(block) => {
                        if block.is_replacement()
                            && (((bias == Bias::Left || search_left) && output_start <= point.0)
                                || (!search_left && output_start >= point.0))
                        {
                            return BlockPoint(output_start);
                        }
                    }
                    None => {
                        let input_point = if point.row >= output_end_row.0 {
                            let line_len = self.wrap_snapshot.line_len(input_end_row - RowDelta(1));
                            self.wrap_snapshot.clip_point(
                                WrapPoint::new(input_end_row - RowDelta(1), line_len),
                                bias,
                            )
                        } else {
                            let output_overshoot = point.0.saturating_sub(output_start);
                            self.wrap_snapshot
                                .clip_point(WrapPoint(input_start + output_overshoot), bias)
                        };

                        if (input_start..input_end).contains(&input_point.0) {
                            let input_overshoot = input_point.0.saturating_sub(input_start);
                            return BlockPoint(output_start + input_overshoot);
                        }
                    }
                }

                if search_left {
                    cursor.prev();
                } else {
                    cursor.next();
                }
            } else if reversed {
                return self.max_point();
            } else {
                reversed = true;
                search_left = !search_left;
                cursor.seek(&BlockRow(point.row), Bias::Right);
            }
        }
    }

    #[ztracing::instrument(skip_all)]
    pub fn to_block_point(&self, wrap_point: WrapPoint) -> BlockPoint {
        let (start, _, item) = self.transforms.find::<Dimensions<WrapRow, BlockRow>, _>(
            (),
            &wrap_point.row(),
            Bias::Right,
        );
        if let Some(transform) = item {
            if transform.block.is_some() {
                BlockPoint::new(start.1, 0)
            } else {
                let Dimensions(input_start_row, output_start_row, _) = start;
                let input_start = Point::new(input_start_row.0, 0);
                let output_start = Point::new(output_start_row.0, 0);
                let input_overshoot = wrap_point.0 - input_start;
                BlockPoint(output_start + input_overshoot)
            }
        } else {
            self.max_point()
        }
    }

    #[ztracing::instrument(skip_all)]
    pub fn to_wrap_point(&self, block_point: BlockPoint, bias: Bias) -> WrapPoint {
        let (start, end, item) = self.transforms.find::<Dimensions<BlockRow, WrapRow>, _>(
            (),
            &BlockRow(block_point.row),
            Bias::Right,
        );
        if let Some(transform) = item {
            match transform.block.as_ref() {
                Some(block) => {
                    if block.place_below() {
                        let wrap_row = start.1 - RowDelta(1);
                        WrapPoint::new(wrap_row, self.wrap_snapshot.line_len(wrap_row))
                    } else if block.place_above() {
                        WrapPoint::new(start.1, 0)
                    } else if bias == Bias::Left {
                        WrapPoint::new(start.1, 0)
                    } else {
                        let wrap_row = end.1 - RowDelta(1);
                        WrapPoint::new(wrap_row, self.wrap_snapshot.line_len(wrap_row))
                    }
                }
                None => {
                    let overshoot = block_point.row() - start.0;
                    let wrap_row = start.1 + RowDelta(overshoot.0);
                    WrapPoint::new(wrap_row, block_point.column)
                }
            }
        } else {
            self.wrap_snapshot.max_point()
        }
    }
}

impl BlockChunks<'_> {
    /// Go to the next transform
    #[ztracing::instrument(skip_all)]
    fn advance(&mut self) {
        self.input_chunk = Chunk::default();
        self.transforms.next();
        while let Some(transform) = self.transforms.item() {
            if transform
                .block
                .as_ref()
                .is_some_and(|block| block.height() == 0)
            {
                self.transforms.next();
            } else {
                break;
            }
        }

        if self
            .transforms
            .item()
            .is_some_and(|transform| transform.block.is_none())
        {
            let start_input_row = self.transforms.start().1;
            let start_output_row = self.transforms.start().0;
            if start_output_row < self.max_output_row {
                let end_input_row = cmp::min(
                    self.transforms.end().1,
                    start_input_row + (self.max_output_row - start_output_row),
                );
                self.input_chunks.seek(start_input_row..end_input_row);
            }
        }
    }
}

pub struct StickyHeaderExcerpt<'a> {
    pub excerpt: &'a ExcerptInfo,
}

impl<'a> Iterator for BlockChunks<'a> {
    type Item = Chunk<'a>;

    #[ztracing::instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.output_row >= self.max_output_row {
            return None;
        }

        if self.line_count_overflow > RowDelta(0) {
            let lines = self.line_count_overflow.0.min(u128::BITS);
            self.line_count_overflow.0 -= lines;
            self.output_row += RowDelta(lines);
            return Some(Chunk {
                text: unsafe { std::str::from_utf8_unchecked(&NEWLINES[..lines as usize]) },
                chars: 1u128.unbounded_shl(lines).wrapping_sub(1),
                ..Default::default()
            });
        }

        let transform = self.transforms.item()?;
        if transform.block.is_some() {
            let block_start = self.transforms.start().0;
            let mut block_end = self.transforms.end().0;
            self.advance();
            if self.transforms.item().is_none() {
                block_end -= RowDelta(1);
            }

            let start_in_block = self.output_row - block_start;
            let end_in_block = cmp::min(self.max_output_row, block_end) - block_start;
            let line_count = end_in_block - start_in_block;
            let lines = RowDelta(line_count.0.min(u128::BITS));
            self.line_count_overflow = line_count - lines;
            self.output_row += lines;

            return Some(Chunk {
                text: unsafe { std::str::from_utf8_unchecked(&NEWLINES[..lines.0 as usize]) },
                chars: 1u128.unbounded_shl(lines.0).wrapping_sub(1),
                ..Default::default()
            });
        }

        if self.input_chunk.text.is_empty() {
            if let Some(input_chunk) = self.input_chunks.next() {
                self.input_chunk = input_chunk;
            } else {
                if self.output_row < self.max_output_row {
                    self.output_row.0 += 1;
                    self.advance();
                    if self.transforms.item().is_some() {
                        return Some(Chunk {
                            text: "\n",
                            chars: 1,
                            ..Default::default()
                        });
                    }
                }
                return None;
            }
        }

        let transform_end = self.transforms.end().0;
        let (prefix_rows, prefix_bytes) =
            offset_for_row(self.input_chunk.text, transform_end - self.output_row);
        self.output_row += prefix_rows;

        let (mut prefix, suffix) = self.input_chunk.text.split_at(prefix_bytes);
        self.input_chunk.text = suffix;
        self.input_chunk.tabs >>= prefix_bytes.saturating_sub(1);
        self.input_chunk.chars >>= prefix_bytes.saturating_sub(1);

        let mut tabs = self.input_chunk.tabs;
        let mut chars = self.input_chunk.chars;

        if self.masked {
            // Not great for multibyte text because to keep cursor math correct we
            // need to have the same number of chars in the input as output.
            let chars_count = prefix.chars().count();
            let bullet_len = chars_count;
            prefix = unsafe { std::str::from_utf8_unchecked(&BULLETS[..bullet_len]) };
            chars = 1u128.unbounded_shl(bullet_len as u32).wrapping_sub(1);
            tabs = 0;
        }

        let chunk = Chunk {
            text: prefix,
            tabs,
            chars,
            ..self.input_chunk.clone()
        };

        if self.output_row == transform_end {
            self.advance();
        }

        Some(chunk)
    }
}

impl Iterator for BlockRows<'_> {
    type Item = RowInfo;

    #[ztracing::instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.output_row.0 += 1;
        } else {
            self.started = true;
        }

        if self.output_row >= self.transforms.end().0 {
            self.transforms.next();
            while let Some(transform) = self.transforms.item() {
                if transform
                    .block
                    .as_ref()
                    .is_some_and(|block| block.height() == 0)
                {
                    self.transforms.next();
                } else {
                    break;
                }
            }

            let transform = self.transforms.item()?;
            if transform
                .block
                .as_ref()
                .is_none_or(|block| block.is_replacement())
            {
                self.input_rows.seek(self.transforms.start().1);
            }
        }

        let transform = self.transforms.item()?;
        if transform.block.as_ref().is_none_or(|block| {
            block.is_replacement()
                && self.transforms.start().0 == self.output_row
                && matches!(block, Block::FoldedBuffer { .. }).not()
        }) {
            self.input_rows.next()
        } else {
            Some(RowInfo::default())
        }
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        self.summary.clone()
    }
}

impl sum_tree::ContextLessSummary for TransformSummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        if summary.longest_row_chars > self.longest_row_chars {
            self.longest_row = self.output_rows + summary.longest_row;
            self.longest_row_chars = summary.longest_row_chars;
        }
        self.input_rows += summary.input_rows;
        self.output_rows += summary.output_rows;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for WrapRow {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        *self += summary.input_rows;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for BlockRow {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        *self += summary.output_rows;
    }
}

impl Deref for BlockContext<'_, '_> {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl DerefMut for BlockContext<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.app
    }
}

impl CustomBlock {
    #[ztracing::instrument(skip_all)]
    pub fn render(&self, cx: &mut BlockContext) -> AnyElement {
        self.render.lock()(cx)
    }

    #[ztracing::instrument(skip_all)]
    pub fn start(&self) -> Anchor {
        *self.placement.start()
    }

    #[ztracing::instrument(skip_all)]
    pub fn end(&self) -> Anchor {
        *self.placement.end()
    }

    pub fn style(&self) -> BlockStyle {
        self.style
    }
}

impl Debug for CustomBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("id", &self.id)
            .field("placement", &self.placement)
            .field("height", &self.height)
            .field("style", &self.style)
            .field("priority", &self.priority)
            .finish_non_exhaustive()
    }
}

// Count the number of bytes prior to a target point. If the string doesn't contain the target
// point, return its total extent. Otherwise return the target point itself.
fn offset_for_row(s: &str, target: RowDelta) -> (RowDelta, usize) {
    let mut row = 0;
    let mut offset = 0;
    for (ix, line) in s.split('\n').enumerate() {
        if ix > 0 {
            row += 1;
            offset += 1;
        }
        if row >= target.0 {
            break;
        }
        offset += line.len();
    }
    (RowDelta(row), offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{
            Companion, fold_map::FoldMap, inlay_map::InlayMap, tab_map::TabMap, wrap_map::WrapMap,
        },
        split::{convert_lhs_rows_to_rhs, convert_rhs_rows_to_lhs},
        test::test_font,
    };
    use buffer_diff::BufferDiff;
    use gpui::{App, AppContext as _, Element, div, font, px};
    use itertools::Itertools;
    use language::{Buffer, Capability};
    use multi_buffer::{ExcerptRange, MultiBuffer};
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::env;
    use util::RandomCharIter;

    #[gpui::test]
    fn test_offset_for_row() {
        assert_eq!(offset_for_row("", RowDelta(0)), (RowDelta(0), 0));
        assert_eq!(offset_for_row("", RowDelta(1)), (RowDelta(0), 0));
        assert_eq!(offset_for_row("abcd", RowDelta(0)), (RowDelta(0), 0));
        assert_eq!(offset_for_row("abcd", RowDelta(1)), (RowDelta(0), 4));
        assert_eq!(offset_for_row("\n", RowDelta(0)), (RowDelta(0), 0));
        assert_eq!(offset_for_row("\n", RowDelta(1)), (RowDelta(1), 1));
        assert_eq!(
            offset_for_row("abc\ndef\nghi", RowDelta(0)),
            (RowDelta(0), 0)
        );
        assert_eq!(
            offset_for_row("abc\ndef\nghi", RowDelta(1)),
            (RowDelta(1), 4)
        );
        assert_eq!(
            offset_for_row("abc\ndef\nghi", RowDelta(2)),
            (RowDelta(2), 8)
        );
        assert_eq!(
            offset_for_row("abc\ndef\nghi", RowDelta(3)),
            (RowDelta(2), 11)
        );
    }

    #[gpui::test]
    fn test_basic_blocks(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "aaa\nbbb\nccc\nddd";

        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut tab_map, tab_snapshot) = TabMap::new(fold_snapshot, 1.try_into().unwrap());
        let (wrap_map, wraps_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), 1, 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        let block_ids = writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 2))),
                height: Some(2),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(buffer_snapshot.anchor_after(Point::new(3, 3))),
                height: Some(3),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
        ]);

        let snapshot = block_map.read(wraps_snapshot, Default::default(), None, None);
        assert_eq!(snapshot.text(), "aaa\n\n\n\nbbb\nccc\nddd\n\n\n");

        let blocks = snapshot
            .blocks_in_range(BlockRow(0)..BlockRow(8))
            .map(|(start_row, block)| {
                let block = block.as_custom().unwrap();
                (start_row.0..start_row.0 + block.height.unwrap(), block.id)
            })
            .collect::<Vec<_>>();

        // When multiple blocks are on the same line, the newer blocks appear first.
        assert_eq!(
            blocks,
            &[
                (1..2, block_ids[0]),
                (2..4, block_ids[1]),
                (7..10, block_ids[2]),
            ]
        );

        assert_eq!(
            snapshot.to_block_point(WrapPoint::new(WrapRow(0), 3)),
            BlockPoint::new(BlockRow(0), 3)
        );
        assert_eq!(
            snapshot.to_block_point(WrapPoint::new(WrapRow(1), 0)),
            BlockPoint::new(BlockRow(4), 0)
        );
        assert_eq!(
            snapshot.to_block_point(WrapPoint::new(WrapRow(3), 3)),
            BlockPoint::new(BlockRow(6), 3)
        );

        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(BlockRow(0), 3), Bias::Left),
            WrapPoint::new(WrapRow(0), 3)
        );
        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(BlockRow(1), 0), Bias::Left),
            WrapPoint::new(WrapRow(1), 0)
        );
        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(BlockRow(3), 0), Bias::Left),
            WrapPoint::new(WrapRow(1), 0)
        );
        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(BlockRow(7), 0), Bias::Left),
            WrapPoint::new(WrapRow(3), 3)
        );

        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(1), 0), Bias::Left),
            BlockPoint::new(BlockRow(0), 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(1), 0), Bias::Right),
            BlockPoint::new(BlockRow(4), 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(1), 1), Bias::Left),
            BlockPoint::new(BlockRow(0), 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(1), 1), Bias::Right),
            BlockPoint::new(BlockRow(4), 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(4), 0), Bias::Left),
            BlockPoint::new(BlockRow(4), 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(4), 0), Bias::Right),
            BlockPoint::new(BlockRow(4), 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(6), 3), Bias::Left),
            BlockPoint::new(BlockRow(6), 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(6), 3), Bias::Right),
            BlockPoint::new(BlockRow(6), 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(7), 0), Bias::Left),
            BlockPoint::new(BlockRow(6), 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(BlockRow(7), 0), Bias::Right),
            BlockPoint::new(BlockRow(6), 3)
        );

        assert_eq!(
            snapshot
                .row_infos(BlockRow(0))
                .map(|row_info| row_info.buffer_row)
                .collect::<Vec<_>>(),
            &[
                Some(0),
                None,
                None,
                None,
                Some(1),
                Some(2),
                Some(3),
                None,
                None,
                None
            ]
        );

        // Insert a line break, separating two block decorations into separate lines.
        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 1)..Point::new(1, 1), "!!!\n")], None, cx);
            buffer.snapshot(cx)
        });

        let (inlay_snapshot, inlay_edits) =
            inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
        let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        let (tab_snapshot, tab_edits) =
            tab_map.sync(fold_snapshot, fold_edits, 4.try_into().unwrap());
        let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
            wrap_map.sync(tab_snapshot, tab_edits, cx)
        });
        let snapshot = block_map.read(wraps_snapshot, wrap_edits, None, None);
        assert_eq!(snapshot.text(), "aaa\n\nb!!!\n\n\nbb\nccc\nddd\n\n\n");
    }

    #[gpui::test]
    fn test_multibuffer_headers_and_footers(cx: &mut App) {
        init_test(cx);

        let buffer1 = cx.new(|cx| Buffer::local("Buffer 1", cx));
        let buffer2 = cx.new(|cx| Buffer::local("Buffer 2", cx));
        let buffer3 = cx.new(|cx| Buffer::local("Buffer 3", cx));

        let mut excerpt_ids = Vec::new();
        let multi_buffer = cx.new(|cx| {
            let mut multi_buffer = MultiBuffer::new(Capability::ReadWrite);
            excerpt_ids.extend(multi_buffer.push_excerpts(
                buffer1.clone(),
                [ExcerptRange::new(0..buffer1.read(cx).len())],
                cx,
            ));
            excerpt_ids.extend(multi_buffer.push_excerpts(
                buffer2.clone(),
                [ExcerptRange::new(0..buffer2.read(cx).len())],
                cx,
            ));
            excerpt_ids.extend(multi_buffer.push_excerpts(
                buffer3.clone(),
                [ExcerptRange::new(0..buffer3.read(cx).len())],
                cx,
            ));

            multi_buffer
        });

        let font = test_font();
        let font_size = px(14.);
        let font_id = cx.text_system().resolve_font(&font);
        let mut wrap_width = px(0.);
        for c in "Buff".chars() {
            wrap_width += cx
                .text_system()
                .advance(font_id, font_size, c)
                .unwrap()
                .width;
        }

        let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(multi_buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_, wraps_snapshot) = WrapMap::new(tab_snapshot, font, font_size, Some(wrap_width), cx);

        let block_map = BlockMap::new(wraps_snapshot.clone(), 1, 1);
        let snapshot = block_map.read(wraps_snapshot, Default::default(), None, None);

        // Each excerpt has a header above and footer below. Excerpts are also *separated* by a newline.
        assert_eq!(snapshot.text(), "\nBuff\ner 1\n\nBuff\ner 2\n\nBuff\ner 3");

        let blocks: Vec<_> = snapshot
            .blocks_in_range(BlockRow(0)..BlockRow(u32::MAX))
            .map(|(row, block)| (row.0..row.0 + block.height(), block.id()))
            .collect();
        assert_eq!(
            blocks,
            vec![
                (0..1, BlockId::ExcerptBoundary(excerpt_ids[0])), // path, header
                (3..4, BlockId::ExcerptBoundary(excerpt_ids[1])), // path, header
                (6..7, BlockId::ExcerptBoundary(excerpt_ids[2])), // path, header
            ]
        );
    }

    #[gpui::test]
    fn test_replace_with_heights(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "aaa\nbbb\nccc\nddd";

        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let _subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (_inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_tab_map, tab_snapshot) = TabMap::new(fold_snapshot, 1.try_into().unwrap());
        let (_wrap_map, wraps_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), 1, 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        let block_ids = writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 2))),
                height: Some(2),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(buffer_snapshot.anchor_after(Point::new(3, 3))),
                height: Some(3),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
        ]);

        {
            let snapshot = block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
            assert_eq!(snapshot.text(), "aaa\n\n\n\nbbb\nccc\nddd\n\n\n");

            let mut block_map_writer =
                block_map.write(wraps_snapshot.clone(), Default::default(), None, None);

            let mut new_heights = HashMap::default();
            new_heights.insert(block_ids[0], 2);
            block_map_writer.resize(new_heights);
            let snapshot = block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
            assert_eq!(snapshot.text(), "aaa\n\n\n\n\nbbb\nccc\nddd\n\n\n");
        }

        {
            let mut block_map_writer =
                block_map.write(wraps_snapshot.clone(), Default::default(), None, None);

            let mut new_heights = HashMap::default();
            new_heights.insert(block_ids[0], 1);
            block_map_writer.resize(new_heights);

            let snapshot = block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
            assert_eq!(snapshot.text(), "aaa\n\n\n\nbbb\nccc\nddd\n\n\n");
        }

        {
            let mut block_map_writer =
                block_map.write(wraps_snapshot.clone(), Default::default(), None, None);

            let mut new_heights = HashMap::default();
            new_heights.insert(block_ids[0], 0);
            block_map_writer.resize(new_heights);

            let snapshot = block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
            assert_eq!(snapshot.text(), "aaa\n\n\nbbb\nccc\nddd\n\n\n");
        }

        {
            let mut block_map_writer =
                block_map.write(wraps_snapshot.clone(), Default::default(), None, None);

            let mut new_heights = HashMap::default();
            new_heights.insert(block_ids[0], 3);
            block_map_writer.resize(new_heights);

            let snapshot = block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
            assert_eq!(snapshot.text(), "aaa\n\n\n\n\n\nbbb\nccc\nddd\n\n\n");
        }

        {
            let mut block_map_writer =
                block_map.write(wraps_snapshot.clone(), Default::default(), None, None);

            let mut new_heights = HashMap::default();
            new_heights.insert(block_ids[0], 3);
            block_map_writer.resize(new_heights);

            let snapshot = block_map.read(wraps_snapshot, Default::default(), None, None);
            // Same height as before, should remain the same
            assert_eq!(snapshot.text(), "aaa\n\n\n\n\n\nbbb\nccc\nddd\n\n\n");
        }
    }

    #[gpui::test]
    fn test_blocks_on_wrapped_lines(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "one two three\nfour five six\nseven eight";

        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_, wraps_snapshot) = cx.update(|cx| {
            WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), Some(px(90.)), cx)
        });
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), 1, 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 12))),
                render: Arc::new(|_| div().into_any()),
                height: Some(1),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(buffer_snapshot.anchor_after(Point::new(1, 1))),
                render: Arc::new(|_| div().into_any()),
                height: Some(1),
                priority: 0,
            },
        ]);

        // Blocks with an 'above' disposition go above their corresponding buffer line.
        // Blocks with a 'below' disposition go below their corresponding buffer line.
        let snapshot = block_map.read(wraps_snapshot, Default::default(), None, None);
        assert_eq!(
            snapshot.text(),
            "one two \nthree\n\nfour five \nsix\n\nseven \neight"
        );
    }

    #[gpui::test]
    fn test_replace_lines(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "line1\nline2\nline3\nline4\nline5";

        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let buffer_subscription = buffer.update(cx, |buffer, _cx| buffer.subscribe());
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let tab_size = 1.try_into().unwrap();
        let (mut tab_map, tab_snapshot) = TabMap::new(fold_snapshot, tab_size);
        let (wrap_map, wraps_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), 1, 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        let replace_block_id = writer.insert(vec![BlockProperties {
            style: BlockStyle::Fixed,
            placement: BlockPlacement::Replace(
                buffer_snapshot.anchor_after(Point::new(1, 3))
                    ..=buffer_snapshot.anchor_before(Point::new(3, 1)),
            ),
            height: Some(4),
            render: Arc::new(|_| div().into_any()),
            priority: 0,
        }])[0];

        let blocks_snapshot = block_map.read(wraps_snapshot, Default::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "line1\n\n\n\n\nline5");

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(3, 0), "")], None, cx);
            buffer.snapshot(cx)
        });
        let (inlay_snapshot, inlay_edits) =
            inlay_map.sync(buffer_snapshot, buffer_subscription.consume().into_inner());
        let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        let (tab_snapshot, tab_edits) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
        let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
            wrap_map.sync(tab_snapshot, tab_edits, cx)
        });
        let blocks_snapshot = block_map.read(wraps_snapshot, wrap_edits, None, None);
        assert_eq!(blocks_snapshot.text(), "line1\n\n\n\n\nline5");

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(
                    Point::new(1, 5)..Point::new(1, 5),
                    "\nline 2.1\nline2.2\nline 2.3\nline 2.4",
                )],
                None,
                cx,
            );
            buffer.snapshot(cx)
        });
        let (inlay_snapshot, inlay_edits) = inlay_map.sync(
            buffer_snapshot.clone(),
            buffer_subscription.consume().into_inner(),
        );
        let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        let (tab_snapshot, tab_edits) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
        let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
            wrap_map.sync(tab_snapshot, tab_edits, cx)
        });
        let blocks_snapshot = block_map.read(wraps_snapshot.clone(), wrap_edits, None, None);
        assert_eq!(blocks_snapshot.text(), "line1\n\n\n\n\nline5");

        // Blocks inserted right above the start or right below the end of the replaced region are hidden.
        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(0, 3))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 3))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(buffer_snapshot.anchor_after(Point::new(6, 2))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
        ]);
        let blocks_snapshot =
            block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "\nline1\n\n\n\n\nline5");

        // Ensure blocks inserted *inside* replaced region are hidden.
        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(buffer_snapshot.anchor_after(Point::new(1, 3))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(2, 1))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(6, 1))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
        ]);
        let blocks_snapshot =
            block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "\nline1\n\n\n\n\nline5");

        // Removing the replace block shows all the hidden blocks again.
        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        writer.remove(HashSet::from_iter([replace_block_id]));
        let blocks_snapshot = block_map.read(wraps_snapshot, Default::default(), None, None);
        assert_eq!(
            blocks_snapshot.text(),
            "\nline1\n\nline2\n\n\nline 2.1\nline2.2\nline 2.3\nline 2.4\n\nline4\n\nline5"
        );
    }

    #[gpui::test]
    fn test_custom_blocks_inside_buffer_folds(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "111\n222\n333\n444\n555\n666";

        let buffer = cx.update(|cx| {
            MultiBuffer::build_multi(
                [
                    (text, vec![Point::new(0, 0)..Point::new(0, 3)]),
                    (
                        text,
                        vec![
                            Point::new(1, 0)..Point::new(1, 3),
                            Point::new(2, 0)..Point::new(2, 3),
                            Point::new(3, 0)..Point::new(3, 3),
                        ],
                    ),
                    (
                        text,
                        vec![
                            Point::new(4, 0)..Point::new(4, 3),
                            Point::new(5, 0)..Point::new(5, 3),
                        ],
                    ),
                ],
                cx,
            )
        });
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let buffer_ids = buffer_snapshot
            .excerpts()
            .map(|(_, buffer_snapshot, _)| buffer_snapshot.remote_id())
            .dedup()
            .collect::<Vec<_>>();
        assert_eq!(buffer_ids.len(), 3);
        let buffer_id_1 = buffer_ids[0];
        let buffer_id_2 = buffer_ids[1];
        let buffer_id_3 = buffer_ids[2];

        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_, wrap_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wrap_snapshot.clone(), 2, 1);
        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);

        assert_eq!(
            blocks_snapshot.text(),
            "\n\n111\n\n\n222\n\n333\n\n444\n\n\n555\n\n666"
        );
        assert_eq!(
            blocks_snapshot
                .row_infos(BlockRow(0))
                .map(|i| i.buffer_row)
                .collect::<Vec<_>>(),
            vec![
                None,
                None,
                Some(0),
                None,
                None,
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
                None,
                None,
                Some(4),
                None,
                Some(5),
            ]
        );

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        let excerpt_blocks_2 = writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(2, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(buffer_snapshot.anchor_after(Point::new(3, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
        ]);
        let excerpt_blocks_3 = writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(4, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(buffer_snapshot.anchor_after(Point::new(5, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            },
        ]);

        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);
        assert_eq!(
            blocks_snapshot.text(),
            "\n\n111\n\n\n\n222\n\n\n333\n\n444\n\n\n\n\n555\n\n666\n"
        );
        assert_eq!(
            blocks_snapshot
                .row_infos(BlockRow(0))
                .map(|i| i.buffer_row)
                .collect::<Vec<_>>(),
            vec![
                None,
                None,
                Some(0),
                None,
                None,
                None,
                Some(1),
                None,
                None,
                Some(2),
                None,
                Some(3),
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(5),
                None,
            ]
        );

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        buffer.read_with(cx, |buffer, cx| {
            writer.fold_buffers([buffer_id_1], buffer, cx);
        });
        let excerpt_blocks_1 = writer.insert(vec![BlockProperties {
            style: BlockStyle::Fixed,
            placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(0, 0))),
            height: Some(1),
            render: Arc::new(|_| div().into_any()),
            priority: 0,
        }]);
        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);
        let blocks = blocks_snapshot
            .blocks_in_range(BlockRow(0)..BlockRow(u32::MAX))
            .collect::<Vec<_>>();
        for (_, block) in &blocks {
            if let BlockId::Custom(custom_block_id) = block.id() {
                assert!(
                    !excerpt_blocks_1.contains(&custom_block_id),
                    "Should have no blocks from the folded buffer"
                );
                assert!(
                    excerpt_blocks_2.contains(&custom_block_id)
                        || excerpt_blocks_3.contains(&custom_block_id),
                    "Should have only blocks from unfolded buffers"
                );
            }
        }
        assert_eq!(
            1,
            blocks
                .iter()
                .filter(|(_, block)| matches!(block, Block::FoldedBuffer { .. }))
                .count(),
            "Should have one folded block, producing a header of the second buffer"
        );
        assert_eq!(
            blocks_snapshot.text(),
            "\n\n\n\n\n222\n\n\n333\n\n444\n\n\n\n\n555\n\n666\n"
        );
        assert_eq!(
            blocks_snapshot
                .row_infos(BlockRow(0))
                .map(|i| i.buffer_row)
                .collect::<Vec<_>>(),
            vec![
                None,
                None,
                None,
                None,
                None,
                Some(1),
                None,
                None,
                Some(2),
                None,
                Some(3),
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(5),
                None,
            ]
        );

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        buffer.read_with(cx, |buffer, cx| {
            writer.fold_buffers([buffer_id_2], buffer, cx);
        });
        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);
        let blocks = blocks_snapshot
            .blocks_in_range(BlockRow(0)..BlockRow(u32::MAX))
            .collect::<Vec<_>>();
        for (_, block) in &blocks {
            if let BlockId::Custom(custom_block_id) = block.id() {
                assert!(
                    !excerpt_blocks_1.contains(&custom_block_id),
                    "Should have no blocks from the folded buffer_1"
                );
                assert!(
                    !excerpt_blocks_2.contains(&custom_block_id),
                    "Should have no blocks from the folded buffer_2"
                );
                assert!(
                    excerpt_blocks_3.contains(&custom_block_id),
                    "Should have only blocks from unfolded buffers"
                );
            }
        }
        assert_eq!(
            2,
            blocks
                .iter()
                .filter(|(_, block)| matches!(block, Block::FoldedBuffer { .. }))
                .count(),
            "Should have two folded blocks, producing headers"
        );
        assert_eq!(blocks_snapshot.text(), "\n\n\n\n\n\n\n555\n\n666\n");
        assert_eq!(
            blocks_snapshot
                .row_infos(BlockRow(0))
                .map(|i| i.buffer_row)
                .collect::<Vec<_>>(),
            vec![
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(5),
                None,
            ]
        );

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        buffer.read_with(cx, |buffer, cx| {
            writer.unfold_buffers([buffer_id_1], buffer, cx);
        });
        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);
        let blocks = blocks_snapshot
            .blocks_in_range(BlockRow(0)..BlockRow(u32::MAX))
            .collect::<Vec<_>>();
        for (_, block) in &blocks {
            if let BlockId::Custom(custom_block_id) = block.id() {
                assert!(
                    !excerpt_blocks_2.contains(&custom_block_id),
                    "Should have no blocks from the folded buffer_2"
                );
                assert!(
                    excerpt_blocks_1.contains(&custom_block_id)
                        || excerpt_blocks_3.contains(&custom_block_id),
                    "Should have only blocks from unfolded buffers"
                );
            }
        }
        assert_eq!(
            1,
            blocks
                .iter()
                .filter(|(_, block)| matches!(block, Block::FoldedBuffer { .. }))
                .count(),
            "Should be back to a single folded buffer, producing a header for buffer_2"
        );
        assert_eq!(
            blocks_snapshot.text(),
            "\n\n\n111\n\n\n\n\n\n555\n\n666\n",
            "Should have extra newline for 111 buffer, due to a new block added when it was folded"
        );
        assert_eq!(
            blocks_snapshot
                .row_infos(BlockRow(0))
                .map(|i| i.buffer_row)
                .collect::<Vec<_>>(),
            vec![
                None,
                None,
                None,
                Some(0),
                None,
                None,
                None,
                None,
                None,
                Some(4),
                None,
                Some(5),
                None,
            ]
        );

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        buffer.read_with(cx, |buffer, cx| {
            writer.fold_buffers([buffer_id_3], buffer, cx);
        });
        let blocks_snapshot = block_map.read(wrap_snapshot, Patch::default(), None, None);
        let blocks = blocks_snapshot
            .blocks_in_range(BlockRow(0)..BlockRow(u32::MAX))
            .collect::<Vec<_>>();
        for (_, block) in &blocks {
            if let BlockId::Custom(custom_block_id) = block.id() {
                assert!(
                    excerpt_blocks_1.contains(&custom_block_id),
                    "Should have no blocks from the folded buffer_1"
                );
                assert!(
                    !excerpt_blocks_2.contains(&custom_block_id),
                    "Should have only blocks from unfolded buffers"
                );
                assert!(
                    !excerpt_blocks_3.contains(&custom_block_id),
                    "Should have only blocks from unfolded buffers"
                );
            }
        }

        assert_eq!(
            blocks_snapshot.text(),
            "\n\n\n111\n\n\n\n",
            "Should have a single, first buffer left after folding"
        );
        assert_eq!(
            blocks_snapshot
                .row_infos(BlockRow(0))
                .map(|i| i.buffer_row)
                .collect::<Vec<_>>(),
            vec![None, None, None, Some(0), None, None, None, None,]
        );
    }

    #[gpui::test]
    fn test_basic_buffer_fold(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "111";

        let buffer = cx.update(|cx| {
            MultiBuffer::build_multi([(text, vec![Point::new(0, 0)..Point::new(0, 3)])], cx)
        });
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let buffer_ids = buffer_snapshot
            .excerpts()
            .map(|(_, buffer_snapshot, _)| buffer_snapshot.remote_id())
            .dedup()
            .collect::<Vec<_>>();
        assert_eq!(buffer_ids.len(), 1);
        let buffer_id = buffer_ids[0];

        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_, wrap_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wrap_snapshot.clone(), 2, 1);
        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);

        assert_eq!(blocks_snapshot.text(), "\n\n111");

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        buffer.read_with(cx, |buffer, cx| {
            writer.fold_buffers([buffer_id], buffer, cx);
        });
        let blocks_snapshot = block_map.read(wrap_snapshot, Patch::default(), None, None);
        let blocks = blocks_snapshot
            .blocks_in_range(BlockRow(0)..BlockRow(u32::MAX))
            .collect::<Vec<_>>();
        assert_eq!(
            1,
            blocks
                .iter()
                .filter(|(_, block)| { matches!(block, Block::FoldedBuffer { .. }) })
                .count(),
            "Should have one folded block, producing a header of the second buffer"
        );
        assert_eq!(blocks_snapshot.text(), "\n");
        assert_eq!(
            blocks_snapshot
                .row_infos(BlockRow(0))
                .map(|i| i.buffer_row)
                .collect::<Vec<_>>(),
            vec![None, None],
            "When fully folded, should be no buffer rows"
        );
    }

    #[gpui::test(iterations = 60)]
    fn test_random_blocks(cx: &mut gpui::TestAppContext, mut rng: StdRng) {
        cx.update(init_test);

        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let wrap_width = if rng.random_bool(0.2) {
            None
        } else {
            Some(px(rng.random_range(0.0..=100.0)))
        };
        let tab_size = 1.try_into().unwrap();
        let font_size = px(14.0);
        let buffer_start_header_height = rng.random_range(1..=5);
        let excerpt_header_height = rng.random_range(1..=5);

        log::info!("Wrap width: {:?}", wrap_width);
        log::info!("Excerpt Header Height: {:?}", excerpt_header_height);
        let is_singleton = rng.random();
        let buffer = if is_singleton {
            let len = rng.random_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            log::info!("initial singleton buffer text: {:?}", text);
            cx.update(|cx| MultiBuffer::build_simple(&text, cx))
        } else {
            cx.update(|cx| {
                let multibuffer = MultiBuffer::build_random(&mut rng, cx);
                log::info!(
                    "initial multi-buffer text: {:?}",
                    multibuffer.read(cx).read(cx).text()
                );
                multibuffer
            })
        };

        let mut buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut tab_map, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let font = test_font();
        let (wrap_map, wraps_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font, font_size, wrap_width, cx));
        let mut block_map = BlockMap::new(
            wraps_snapshot,
            buffer_start_header_height,
            excerpt_header_height,
        );

        for _ in 0..operations {
            let mut buffer_edits = Vec::new();
            match rng.random_range(0..=100) {
                0..=19 => {
                    let wrap_width = if rng.random_bool(0.2) {
                        None
                    } else {
                        Some(px(rng.random_range(0.0..=100.0)))
                    };
                    log::info!("Setting wrap width to {:?}", wrap_width);
                    wrap_map.update(cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                20..=39 => {
                    let block_count = rng.random_range(1..=5);
                    let block_properties = (0..block_count)
                        .map(|_| {
                            let buffer = cx.update(|cx| buffer.read(cx).read(cx).clone());
                            let offset = buffer.clip_offset(
                                rng.random_range(MultiBufferOffset(0)..=buffer.len()),
                                Bias::Left,
                            );
                            let mut min_height = 0;
                            let placement = match rng.random_range(0..3) {
                                0 => {
                                    min_height = 1;
                                    let start = buffer.anchor_after(offset);
                                    let end = buffer.anchor_after(buffer.clip_offset(
                                        rng.random_range(offset..=buffer.len()),
                                        Bias::Left,
                                    ));
                                    BlockPlacement::Replace(start..=end)
                                }
                                1 => BlockPlacement::Above(buffer.anchor_after(offset)),
                                _ => BlockPlacement::Below(buffer.anchor_after(offset)),
                            };

                            let height = rng.random_range(min_height..512);
                            BlockProperties {
                                style: BlockStyle::Fixed,
                                placement,
                                height: Some(height),
                                render: Arc::new(|_| div().into_any()),
                                priority: 0,
                            }
                        })
                        .collect::<Vec<_>>();

                    let (inlay_snapshot, inlay_edits) =
                        inlay_map.sync(buffer_snapshot.clone(), vec![]);
                    let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
                    let (tab_snapshot, tab_edits) =
                        tab_map.sync(fold_snapshot, fold_edits, tab_size);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tab_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits, None, None);
                    let block_ids =
                        block_map.insert(block_properties.iter().map(|props| BlockProperties {
                            placement: props.placement.clone(),
                            height: props.height,
                            style: props.style,
                            render: Arc::new(|_| div().into_any()),
                            priority: 0,
                        }));

                    for (block_properties, block_id) in block_properties.iter().zip(block_ids) {
                        log::info!(
                            "inserted block {:?} with height {:?} and id {:?}",
                            block_properties
                                .placement
                                .as_ref()
                                .map(|p| p.to_point(&buffer_snapshot)),
                            block_properties.height,
                            block_id
                        );
                    }
                }
                40..=59 if !block_map.custom_blocks.is_empty() => {
                    let block_count = rng.random_range(1..=4.min(block_map.custom_blocks.len()));
                    let block_ids_to_remove = block_map
                        .custom_blocks
                        .choose_multiple(&mut rng, block_count)
                        .map(|block| block.id)
                        .collect::<HashSet<_>>();

                    let (inlay_snapshot, inlay_edits) =
                        inlay_map.sync(buffer_snapshot.clone(), vec![]);
                    let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
                    let (tab_snapshot, tab_edits) =
                        tab_map.sync(fold_snapshot, fold_edits, tab_size);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tab_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits, None, None);
                    log::info!(
                        "removing {} blocks: {:?}",
                        block_ids_to_remove.len(),
                        block_ids_to_remove
                    );
                    block_map.remove(block_ids_to_remove);
                }
                60..=79 => {
                    if buffer.read_with(cx, |buffer, _| buffer.is_singleton()) {
                        log::info!("Noop fold/unfold operation on a singleton buffer");
                        continue;
                    }
                    let (inlay_snapshot, inlay_edits) =
                        inlay_map.sync(buffer_snapshot.clone(), vec![]);
                    let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
                    let (tab_snapshot, tab_edits) =
                        tab_map.sync(fold_snapshot, fold_edits, tab_size);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tab_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits, None, None);
                    let (unfolded_buffers, folded_buffers) = buffer.read_with(cx, |buffer, _| {
                        let folded_buffers: Vec<_> =
                            block_map.block_map.folded_buffers.iter().cloned().collect();
                        let mut unfolded_buffers = buffer.excerpt_buffer_ids();
                        unfolded_buffers.dedup();
                        log::debug!("All buffers {unfolded_buffers:?}");
                        log::debug!("Folded buffers {folded_buffers:?}");
                        unfolded_buffers.retain(|buffer_id| {
                            !block_map.block_map.folded_buffers.contains(buffer_id)
                        });
                        (unfolded_buffers, folded_buffers)
                    });
                    let mut folded_count = folded_buffers.len();
                    let mut unfolded_count = unfolded_buffers.len();

                    let fold = !unfolded_buffers.is_empty() && rng.random_bool(0.5);
                    let unfold = !folded_buffers.is_empty() && rng.random_bool(0.5);
                    if !fold && !unfold {
                        log::info!(
                            "Noop fold/unfold operation. Unfolded buffers: {unfolded_count}, folded buffers: {folded_count}"
                        );
                        continue;
                    }

                    buffer.update(cx, |buffer, cx| {
                        if fold {
                            let buffer_to_fold =
                                unfolded_buffers[rng.random_range(0..unfolded_buffers.len())];
                            log::info!("Folding {buffer_to_fold:?}");
                            let related_excerpts = buffer_snapshot
                                .excerpts()
                                .filter_map(|(excerpt_id, buffer, range)| {
                                    if buffer.remote_id() == buffer_to_fold {
                                        Some((
                                            excerpt_id,
                                            buffer
                                                .text_for_range(range.context)
                                                .collect::<String>(),
                                        ))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>();
                            log::info!(
                                "Folding {buffer_to_fold:?}, related excerpts: {related_excerpts:?}"
                            );
                            folded_count += 1;
                            unfolded_count -= 1;
                            block_map.fold_buffers([buffer_to_fold], buffer, cx);
                        }
                        if unfold {
                            let buffer_to_unfold =
                                folded_buffers[rng.random_range(0..folded_buffers.len())];
                            log::info!("Unfolding {buffer_to_unfold:?}");
                            unfolded_count += 1;
                            folded_count -= 1;
                            block_map.unfold_buffers([buffer_to_unfold], buffer, cx);
                        }
                        log::info!(
                            "Unfolded buffers: {unfolded_count}, folded buffers: {folded_count}"
                        );
                    });
                }
                _ => {
                    buffer.update(cx, |buffer, cx| {
                        let mutation_count = rng.random_range(1..=5);
                        let subscription = buffer.subscribe();
                        buffer.randomly_mutate(&mut rng, mutation_count, cx);
                        buffer_snapshot = buffer.snapshot(cx);
                        buffer_edits.extend(subscription.consume());
                        log::info!("buffer text: {:?}", buffer_snapshot.text());
                    });
                }
            }

            let (inlay_snapshot, inlay_edits) =
                inlay_map.sync(buffer_snapshot.clone(), buffer_edits);
            let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
            let (tab_snapshot, tab_edits) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
            let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                wrap_map.sync(tab_snapshot, tab_edits, cx)
            });
            let blocks_snapshot = block_map.read(wraps_snapshot.clone(), wrap_edits, None, None);
            assert_eq!(
                blocks_snapshot.transforms.summary().input_rows,
                wraps_snapshot.max_point().row() + RowDelta(1)
            );
            log::info!("wrapped text: {:?}", wraps_snapshot.text());
            log::info!("blocks text: {:?}", blocks_snapshot.text());

            let mut expected_blocks = Vec::new();
            expected_blocks.extend(block_map.custom_blocks.iter().filter_map(|block| {
                Some((
                    block.placement.to_wrap_row(&wraps_snapshot)?,
                    Block::Custom(block.clone()),
                ))
            }));

            let mut inlay_point_cursor = wraps_snapshot.inlay_point_cursor();
            let mut tab_point_cursor = wraps_snapshot.tab_point_cursor();
            let mut fold_point_cursor = wraps_snapshot.fold_point_cursor();
            let mut wrap_point_cursor = wraps_snapshot.wrap_point_cursor();

            // Note that this needs to be synced with the related section in BlockMap::sync
            expected_blocks.extend(block_map.header_and_footer_blocks(
                &buffer_snapshot,
                MultiBufferOffset(0)..,
                |point, bias| {
                    wrap_point_cursor
                        .map(
                            tab_point_cursor
                                .map(fold_point_cursor.map(inlay_point_cursor.map(point), bias)),
                        )
                        .row()
                },
            ));

            BlockMap::sort_blocks(&mut expected_blocks);

            for (placement, block) in &expected_blocks {
                log::info!(
                    "Block {:?} placement: {:?} Height: {:?}",
                    block.id(),
                    placement,
                    block.height()
                );
            }

            let mut sorted_blocks_iter = expected_blocks.into_iter().peekable();

            let input_buffer_rows = buffer_snapshot
                .row_infos(MultiBufferRow(0))
                .map(|row| row.buffer_row)
                .collect::<Vec<_>>();
            let mut expected_buffer_rows = Vec::new();
            let mut expected_text = String::new();
            let mut expected_block_positions = Vec::new();
            let mut expected_replaced_buffer_rows = HashSet::default();
            let input_text = wraps_snapshot.text();

            // Loop over the input lines, creating (N - 1) empty lines for
            // blocks of height N.
            //
            // It's important to note that output *starts* as one empty line,
            // so we special case row 0 to assume a leading '\n'.
            //
            // Linehood is the birthright of strings.
            let input_text_lines = input_text.split('\n').enumerate().peekable();
            let mut block_row = 0;
            for (wrap_row, input_line) in input_text_lines {
                let wrap_row = WrapRow(wrap_row as u32);
                let multibuffer_row = wraps_snapshot
                    .to_point(WrapPoint::new(wrap_row, 0), Bias::Left)
                    .row;

                // Create empty lines for the above block
                while let Some((placement, block)) = sorted_blocks_iter.peek() {
                    if *placement.start() == wrap_row && block.place_above() {
                        let (_, block) = sorted_blocks_iter.next().unwrap();
                        expected_block_positions.push((block_row, block.id()));
                        if block.height() > 0 {
                            let text = "\n".repeat((block.height() - 1) as usize);
                            if block_row > 0 {
                                expected_text.push('\n')
                            }
                            expected_text.push_str(&text);
                            for _ in 0..block.height() {
                                expected_buffer_rows.push(None);
                            }
                            block_row += block.height();
                        }
                    } else {
                        break;
                    }
                }

                // Skip lines within replace blocks, then create empty lines for the replace block's height
                let mut is_in_replace_block = false;
                if let Some((BlockPlacement::Replace(replace_range), block)) =
                    sorted_blocks_iter.peek()
                    && wrap_row >= *replace_range.start()
                {
                    is_in_replace_block = true;

                    if wrap_row == *replace_range.start() {
                        if matches!(block, Block::FoldedBuffer { .. }) {
                            expected_buffer_rows.push(None);
                        } else {
                            expected_buffer_rows.push(input_buffer_rows[multibuffer_row as usize]);
                        }
                    }

                    if wrap_row == *replace_range.end() {
                        expected_block_positions.push((block_row, block.id()));
                        let text = "\n".repeat((block.height() - 1) as usize);
                        if block_row > 0 {
                            expected_text.push('\n');
                        }
                        expected_text.push_str(&text);

                        for _ in 1..block.height() {
                            expected_buffer_rows.push(None);
                        }
                        block_row += block.height();

                        sorted_blocks_iter.next();
                    }
                }

                if is_in_replace_block {
                    expected_replaced_buffer_rows.insert(MultiBufferRow(multibuffer_row));
                } else {
                    let buffer_row = input_buffer_rows[multibuffer_row as usize];
                    let soft_wrapped = wraps_snapshot
                        .to_tab_point(WrapPoint::new(wrap_row, 0))
                        .column()
                        > 0;
                    expected_buffer_rows.push(if soft_wrapped { None } else { buffer_row });
                    if block_row > 0 {
                        expected_text.push('\n');
                    }
                    expected_text.push_str(input_line);
                    block_row += 1;
                }

                while let Some((placement, block)) = sorted_blocks_iter.peek() {
                    if *placement.end() == wrap_row && block.place_below() {
                        let (_, block) = sorted_blocks_iter.next().unwrap();
                        expected_block_positions.push((block_row, block.id()));
                        if block.height() > 0 {
                            let text = "\n".repeat((block.height() - 1) as usize);
                            if block_row > 0 {
                                expected_text.push('\n')
                            }
                            expected_text.push_str(&text);
                            for _ in 0..block.height() {
                                expected_buffer_rows.push(None);
                            }
                            block_row += block.height();
                        }
                    } else {
                        break;
                    }
                }
            }

            let expected_lines = expected_text.split('\n').collect::<Vec<_>>();
            let expected_row_count = expected_lines.len();
            log::info!("expected text: {expected_text:?}");

            assert_eq!(
                blocks_snapshot.max_point().row + 1,
                expected_row_count as u32,
                "actual row count != expected row count",
            );
            assert_eq!(
                blocks_snapshot.text(),
                expected_text,
                "actual text != expected text",
            );

            for start_row in 0..expected_row_count {
                let end_row = rng.random_range(start_row + 1..=expected_row_count);
                let mut expected_text = expected_lines[start_row..end_row].join("\n");
                if end_row < expected_row_count {
                    expected_text.push('\n');
                }

                let actual_text = blocks_snapshot
                    .chunks(
                        BlockRow(start_row as u32)..BlockRow(end_row as u32),
                        false,
                        false,
                        Highlights::default(),
                    )
                    .map(|chunk| chunk.text)
                    .collect::<String>();
                assert_eq!(
                    actual_text,
                    expected_text,
                    "incorrect text starting row row range {:?}",
                    start_row..end_row
                );
                assert_eq!(
                    blocks_snapshot
                        .row_infos(BlockRow(start_row as u32))
                        .map(|row_info| row_info.buffer_row)
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[start_row..],
                    "incorrect buffer_rows starting at row {:?}",
                    start_row
                );
            }

            assert_eq!(
                blocks_snapshot
                    .blocks_in_range(BlockRow(0)..BlockRow(expected_row_count as u32))
                    .map(|(row, block)| (row.0, block.id()))
                    .collect::<Vec<_>>(),
                expected_block_positions,
                "invalid blocks_in_range({:?})",
                0..expected_row_count
            );

            for (_, expected_block) in
                blocks_snapshot.blocks_in_range(BlockRow(0)..BlockRow(expected_row_count as u32))
            {
                let actual_block = blocks_snapshot.block_for_id(expected_block.id());
                assert_eq!(
                    actual_block.map(|block| block.id()),
                    Some(expected_block.id())
                );
            }

            for (block_row, block_id) in expected_block_positions {
                if let BlockId::Custom(block_id) = block_id {
                    assert_eq!(
                        blocks_snapshot.row_for_block(block_id),
                        Some(BlockRow(block_row))
                    );
                }
            }

            let mut expected_longest_rows = Vec::new();
            let mut longest_line_len = -1_isize;
            for (row, line) in expected_lines.iter().enumerate() {
                let row = row as u32;

                assert_eq!(
                    blocks_snapshot.line_len(BlockRow(row)),
                    line.len() as u32,
                    "invalid line len for row {}",
                    row
                );

                let line_char_count = line.chars().count() as isize;
                match line_char_count.cmp(&longest_line_len) {
                    Ordering::Less => {}
                    Ordering::Equal => expected_longest_rows.push(row),
                    Ordering::Greater => {
                        longest_line_len = line_char_count;
                        expected_longest_rows.clear();
                        expected_longest_rows.push(row);
                    }
                }
            }

            let longest_row = blocks_snapshot.longest_row();
            assert!(
                expected_longest_rows.contains(&longest_row.0),
                "incorrect longest row {}. expected {:?} with length {}",
                longest_row.0,
                expected_longest_rows,
                longest_line_len,
            );

            for _ in 0..10 {
                let end_row = rng.random_range(1..=expected_lines.len());
                let start_row = rng.random_range(0..end_row);

                let mut expected_longest_rows_in_range = vec![];
                let mut longest_line_len_in_range = 0;

                let mut row = start_row as u32;
                for line in &expected_lines[start_row..end_row] {
                    let line_char_count = line.chars().count() as isize;
                    match line_char_count.cmp(&longest_line_len_in_range) {
                        Ordering::Less => {}
                        Ordering::Equal => expected_longest_rows_in_range.push(row),
                        Ordering::Greater => {
                            longest_line_len_in_range = line_char_count;
                            expected_longest_rows_in_range.clear();
                            expected_longest_rows_in_range.push(row);
                        }
                    }
                    row += 1;
                }

                let longest_row_in_range = blocks_snapshot
                    .longest_row_in_range(BlockRow(start_row as u32)..BlockRow(end_row as u32));
                assert!(
                    expected_longest_rows_in_range.contains(&longest_row_in_range.0),
                    "incorrect longest row {} in range {:?}. expected {:?} with length {}",
                    longest_row.0,
                    start_row..end_row,
                    expected_longest_rows_in_range,
                    longest_line_len_in_range,
                );
            }

            // Ensure that conversion between block points and wrap points is stable.
            for row in 0..=blocks_snapshot.wrap_snapshot.max_point().row().0 {
                let wrap_point = WrapPoint::new(WrapRow(row), 0);
                let block_point = blocks_snapshot.to_block_point(wrap_point);
                let left_wrap_point = blocks_snapshot.to_wrap_point(block_point, Bias::Left);
                let right_wrap_point = blocks_snapshot.to_wrap_point(block_point, Bias::Right);
                assert_eq!(blocks_snapshot.to_block_point(left_wrap_point), block_point);
                assert_eq!(
                    blocks_snapshot.to_block_point(right_wrap_point),
                    block_point
                );
            }

            let mut block_point = BlockPoint::new(BlockRow(0), 0);
            for c in expected_text.chars() {
                let left_point = blocks_snapshot.clip_point(block_point, Bias::Left);
                let left_buffer_point = blocks_snapshot.to_point(left_point, Bias::Left);
                assert_eq!(
                    blocks_snapshot
                        .to_block_point(blocks_snapshot.to_wrap_point(left_point, Bias::Left)),
                    left_point,
                    "block point: {:?}, wrap point: {:?}",
                    block_point,
                    blocks_snapshot.to_wrap_point(left_point, Bias::Left)
                );
                assert_eq!(
                    left_buffer_point,
                    buffer_snapshot.clip_point(left_buffer_point, Bias::Right),
                    "{:?} is not valid in buffer coordinates",
                    left_point
                );

                let right_point = blocks_snapshot.clip_point(block_point, Bias::Right);
                let right_buffer_point = blocks_snapshot.to_point(right_point, Bias::Right);
                assert_eq!(
                    blocks_snapshot
                        .to_block_point(blocks_snapshot.to_wrap_point(right_point, Bias::Right)),
                    right_point,
                    "block point: {:?}, wrap point: {:?}",
                    block_point,
                    blocks_snapshot.to_wrap_point(right_point, Bias::Right)
                );
                assert_eq!(
                    right_buffer_point,
                    buffer_snapshot.clip_point(right_buffer_point, Bias::Left),
                    "{:?} is not valid in buffer coordinates",
                    right_point
                );

                if c == '\n' {
                    block_point.0 += Point::new(1, 0);
                } else {
                    block_point.column += c.len_utf8() as u32;
                }
            }

            for buffer_row in 0..=buffer_snapshot.max_point().row {
                let buffer_row = MultiBufferRow(buffer_row);
                assert_eq!(
                    blocks_snapshot.is_line_replaced(buffer_row),
                    expected_replaced_buffer_rows.contains(&buffer_row),
                    "incorrect is_line_replaced({buffer_row:?}), expected replaced rows: {expected_replaced_buffer_rows:?}",
                );
            }
        }
    }

    #[gpui::test]
    fn test_remove_intersecting_replace_blocks_edge_case(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "abc\ndef\nghi\njkl\nmno";
        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let (_inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_tab_map, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_wrap_map, wraps_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), 1, 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        let _block_id = writer.insert(vec![BlockProperties {
            style: BlockStyle::Fixed,
            placement: BlockPlacement::Above(buffer_snapshot.anchor_after(Point::new(1, 0))),
            height: Some(1),
            render: Arc::new(|_| div().into_any()),
            priority: 0,
        }])[0];

        let blocks_snapshot =
            block_map.read(wraps_snapshot.clone(), Default::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "abc\n\ndef\nghi\njkl\nmno");

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default(), None, None);
        writer.remove_intersecting_replace_blocks(
            [buffer_snapshot
                .anchor_after(Point::new(1, 0))
                .to_offset(&buffer_snapshot)
                ..buffer_snapshot
                    .anchor_after(Point::new(1, 0))
                    .to_offset(&buffer_snapshot)],
            false,
        );
        let blocks_snapshot = block_map.read(wraps_snapshot, Default::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "abc\n\ndef\nghi\njkl\nmno");
    }

    #[gpui::test]
    fn test_folded_buffer_with_near_blocks(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "line 1\nline 2\nline 3";
        let buffer = cx.update(|cx| {
            MultiBuffer::build_multi([(text, vec![Point::new(0, 0)..Point::new(2, 6)])], cx)
        });
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let buffer_ids = buffer_snapshot
            .excerpts()
            .map(|(_, buffer_snapshot, _)| buffer_snapshot.remote_id())
            .dedup()
            .collect::<Vec<_>>();
        assert_eq!(buffer_ids.len(), 1);
        let buffer_id = buffer_ids[0];

        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_, wrap_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wrap_snapshot.clone(), 1, 1);

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        writer.insert(vec![BlockProperties {
            style: BlockStyle::Fixed,
            placement: BlockPlacement::Near(buffer_snapshot.anchor_after(Point::new(0, 0))),
            height: Some(1),
            render: Arc::new(|_| div().into_any()),
            priority: 0,
        }]);

        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "\nline 1\n\nline 2\nline 3");

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        buffer.read_with(cx, |buffer, cx| {
            writer.fold_buffers([buffer_id], buffer, cx);
        });

        let blocks_snapshot = block_map.read(wrap_snapshot, Patch::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "");
    }

    #[gpui::test]
    fn test_folded_buffer_with_near_blocks_on_last_line(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let text = "line 1\nline 2\nline 3\nline 4";
        let buffer = cx.update(|cx| {
            MultiBuffer::build_multi([(text, vec![Point::new(0, 0)..Point::new(3, 6)])], cx)
        });
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let buffer_ids = buffer_snapshot
            .excerpts()
            .map(|(_, buffer_snapshot, _)| buffer_snapshot.remote_id())
            .dedup()
            .collect::<Vec<_>>();
        assert_eq!(buffer_ids.len(), 1);
        let buffer_id = buffer_ids[0];

        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_, wrap_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wrap_snapshot.clone(), 1, 1);

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        writer.insert(vec![BlockProperties {
            style: BlockStyle::Fixed,
            placement: BlockPlacement::Near(buffer_snapshot.anchor_after(Point::new(3, 6))),
            height: Some(1),
            render: Arc::new(|_| div().into_any()),
            priority: 0,
        }]);

        let blocks_snapshot = block_map.read(wrap_snapshot.clone(), Patch::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "\nline 1\nline 2\nline 3\nline 4\n");

        let mut writer = block_map.write(wrap_snapshot.clone(), Patch::default(), None, None);
        buffer.read_with(cx, |buffer, cx| {
            writer.fold_buffers([buffer_id], buffer, cx);
        });

        let blocks_snapshot = block_map.read(wrap_snapshot, Patch::default(), None, None);
        assert_eq!(blocks_snapshot.text(), "");
    }

    #[gpui::test]
    fn test_companion_spacer_blocks(cx: &mut gpui::TestAppContext) {
        cx.update(init_test);

        let base_text = "aaa\nbbb\nccc\nddd\nddd\nddd\neee\n";
        let main_text = "aaa\nddd\nddd\nddd\nXXX\nYYY\nZZZ\neee\n";

        let rhs_buffer = cx.new(|cx| Buffer::local(main_text, cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(base_text, &rhs_buffer.read(cx).text_snapshot(), cx)
        });
        let lhs_buffer = diff.read_with(cx, |diff, _| diff.base_text_buffer());

        let lhs_multibuffer = cx.new(|cx| {
            let mut mb = MultiBuffer::new(Capability::ReadWrite);
            mb.push_excerpts(
                lhs_buffer.clone(),
                [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                cx,
            );
            mb.add_inverted_diff(diff.clone(), cx);
            mb
        });
        let rhs_multibuffer = cx.new(|cx| {
            let mut mb = MultiBuffer::new(Capability::ReadWrite);
            mb.push_excerpts(
                rhs_buffer.clone(),
                [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
                cx,
            );
            mb.add_diff(diff.clone(), cx);
            mb
        });
        let subscription =
            rhs_multibuffer.update(cx, |rhs_multibuffer, _| rhs_multibuffer.subscribe());

        let lhs_excerpt_id =
            lhs_multibuffer.read_with(cx, |mb, cx| mb.snapshot(cx).excerpts().next().unwrap().0);
        let rhs_excerpt_id =
            rhs_multibuffer.read_with(cx, |mb, cx| mb.snapshot(cx).excerpts().next().unwrap().0);

        let lhs_buffer_snapshot = cx.update(|cx| lhs_multibuffer.read(cx).snapshot(cx));
        let (mut _lhs_inlay_map, lhs_inlay_snapshot) = InlayMap::new(lhs_buffer_snapshot);
        let (mut _lhs_fold_map, lhs_fold_snapshot) = FoldMap::new(lhs_inlay_snapshot);
        let (mut _lhs_tab_map, lhs_tab_snapshot) =
            TabMap::new(lhs_fold_snapshot, 4.try_into().unwrap());
        let (_lhs_wrap_map, lhs_wrap_snapshot) =
            cx.update(|cx| WrapMap::new(lhs_tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let lhs_block_map = BlockMap::new(lhs_wrap_snapshot.clone(), 0, 0);

        let rhs_buffer_snapshot = cx.update(|cx| rhs_multibuffer.read(cx).snapshot(cx));
        let (mut rhs_inlay_map, rhs_inlay_snapshot) = InlayMap::new(rhs_buffer_snapshot);
        let (mut rhs_fold_map, rhs_fold_snapshot) = FoldMap::new(rhs_inlay_snapshot);
        let (mut rhs_tab_map, rhs_tab_snapshot) =
            TabMap::new(rhs_fold_snapshot, 4.try_into().unwrap());
        let (_rhs_wrap_map, rhs_wrap_snapshot) =
            cx.update(|cx| WrapMap::new(rhs_tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let rhs_block_map = BlockMap::new(rhs_wrap_snapshot.clone(), 0, 0);

        let rhs_entity_id = rhs_multibuffer.entity_id();

        let companion = cx.new(|_| {
            let mut c = Companion::new(
                rhs_entity_id,
                Default::default(),
                convert_rhs_rows_to_lhs,
                convert_lhs_rows_to_rhs,
            );
            c.add_excerpt_mapping(lhs_excerpt_id, rhs_excerpt_id);
            c
        });

        let rhs_edits = Patch::new(vec![text::Edit {
            old: WrapRow(0)..rhs_wrap_snapshot.max_point().row(),
            new: WrapRow(0)..rhs_wrap_snapshot.max_point().row(),
        }]);
        let lhs_edits = Patch::new(vec![text::Edit {
            old: WrapRow(0)..lhs_wrap_snapshot.max_point().row(),
            new: WrapRow(0)..lhs_wrap_snapshot.max_point().row(),
        }]);

        let rhs_snapshot = companion.read_with(cx, |companion, _cx| {
            rhs_block_map.read(
                rhs_wrap_snapshot.clone(),
                rhs_edits.clone(),
                Some((&lhs_wrap_snapshot, &lhs_edits)),
                Some((companion, rhs_entity_id)),
            )
        });

        let lhs_entity_id = lhs_multibuffer.entity_id();
        let lhs_snapshot = companion.read_with(cx, |companion, _cx| {
            lhs_block_map.read(
                lhs_wrap_snapshot.clone(),
                lhs_edits.clone(),
                Some((&rhs_wrap_snapshot, &rhs_edits)),
                Some((companion, lhs_entity_id)),
            )
        });

        // LHS:
        //   aaa
        // - bbb
        // - ccc
        //   ddd
        //   ddd
        //   ddd
        //   <extra line>
        //   <extra line>
        //   <extra line>
        //   *eee
        //
        // RHS:
        //   aaa
        //   <extra line>
        //   <extra line>
        //   ddd
        //   ddd
        //   ddd
        // + XXX
        // + YYY
        // + ZZZ
        //   eee

        assert_eq!(
            rhs_snapshot.snapshot.text(),
            "aaa\n\n\nddd\nddd\nddd\nXXX\nYYY\nZZZ\neee\n",
            "RHS should have 2 spacer lines after 'aaa' to align with LHS's deleted lines"
        );

        assert_eq!(
            lhs_snapshot.snapshot.text(),
            "aaa\nbbb\nccc\nddd\nddd\nddd\n\n\n\neee\n",
            "LHS should have 3 spacer lines in place of RHS's inserted lines"
        );

        // LHS:
        //   aaa
        // - bbb
        // - ccc
        //   ddd
        //   ddd
        //   ddd
        //   <extra line>
        //   <extra line>
        //   <extra line>
        //   eee
        //
        // RHS:
        //   aaa
        //   <extra line>
        //   <extra line>
        //   ddd
        //   foo
        //   foo
        //   foo
        //   ddd
        //   ddd
        // + XXX
        // + YYY
        // + ZZZ
        //   eee

        let rhs_buffer_snapshot = rhs_multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.edit(
                [(Point::new(2, 0)..Point::new(2, 0), "foo\nfoo\nfoo\n")],
                None,
                cx,
            );
            multibuffer.snapshot(cx)
        });

        let (rhs_inlay_snapshot, rhs_inlay_edits) =
            rhs_inlay_map.sync(rhs_buffer_snapshot, subscription.consume().into_inner());
        let (rhs_fold_snapshot, rhs_fold_edits) =
            rhs_fold_map.read(rhs_inlay_snapshot, rhs_inlay_edits);
        let (rhs_tab_snapshot, rhs_tab_edits) =
            rhs_tab_map.sync(rhs_fold_snapshot, rhs_fold_edits, 4.try_into().unwrap());
        let (rhs_wrap_snapshot, rhs_wrap_edits) = _rhs_wrap_map.update(cx, |wrap_map, cx| {
            wrap_map.sync(rhs_tab_snapshot, rhs_tab_edits, cx)
        });

        let rhs_snapshot = companion.read_with(cx, |companion, _cx| {
            rhs_block_map.read(
                rhs_wrap_snapshot.clone(),
                rhs_wrap_edits.clone(),
                Some((&lhs_wrap_snapshot, &Default::default())),
                Some((companion, rhs_entity_id)),
            )
        });

        let lhs_snapshot = companion.read_with(cx, |companion, _cx| {
            lhs_block_map.read(
                lhs_wrap_snapshot.clone(),
                Default::default(),
                Some((&rhs_wrap_snapshot, &rhs_wrap_edits)),
                Some((companion, lhs_entity_id)),
            )
        });

        assert_eq!(
            rhs_snapshot.snapshot.text(),
            "aaa\n\n\nddd\nfoo\nfoo\nfoo\nddd\nddd\nXXX\nYYY\nZZZ\neee\n",
            "RHS should have the insertion"
        );

        assert_eq!(
            lhs_snapshot.snapshot.text(),
            "aaa\nbbb\nccc\nddd\n\n\n\nddd\nddd\n\n\n\neee\n",
            "LHS should have 3 more spacer lines to balance the insertion"
        );
    }

    fn init_test(cx: &mut gpui::App) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
        assets::Assets.load_test_fonts(cx);
    }

    impl Block {
        fn as_custom(&self) -> Option<&CustomBlock> {
            match self {
                Block::Custom(block) => Some(block),
                _ => None,
            }
        }
    }

    impl BlockSnapshot {
        fn to_point(&self, point: BlockPoint, bias: Bias) -> Point {
            self.wrap_snapshot
                .to_point(self.to_wrap_point(point, bias), bias)
        }
    }
}
