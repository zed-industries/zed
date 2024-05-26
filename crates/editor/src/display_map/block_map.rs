use super::{
    wrap_map::{self, WrapEdit, WrapPoint, WrapSnapshot},
    Highlights,
};
use crate::{EditorStyle, GutterDimensions};
use collections::{Bound, HashMap, HashSet};
use gpui::{AnyElement, Pixels, WindowContext};
use language::{BufferSnapshot, Chunk, Patch, Point};
use multi_buffer::{Anchor, ExcerptId, ExcerptRange, MultiBufferRow, ToPoint as _};
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    fmt::Debug,
    ops::{Deref, DerefMut, Range, RangeBounds},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use sum_tree::{Bias, SumTree};
use text::Edit;

const NEWLINES: &[u8] = &[b'\n'; u8::MAX as usize];

/// Tracks custom blocks such as diagnostics that should be displayed within buffer.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub struct BlockMap {
    next_block_id: AtomicUsize,
    wrap_snapshot: RefCell<WrapSnapshot>,
    blocks: Vec<Arc<Block>>,
    transforms: RefCell<SumTree<Transform>>,
    show_excerpt_controls: bool,
    buffer_header_height: u8,
    excerpt_header_height: u8,
    excerpt_footer_height: u8,
}

pub struct BlockMapWriter<'a>(&'a mut BlockMap);

#[derive(Clone)]
pub struct BlockSnapshot {
    wrap_snapshot: WrapSnapshot,
    transforms: SumTree<Transform>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(usize);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct BlockPoint(pub Point);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct BlockRow(pub(super) u32);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
struct WrapRow(u32);

pub type RenderBlock = Box<dyn Send + Fn(&mut BlockContext) -> AnyElement>;

pub struct Block {
    id: BlockId,
    position: Anchor,
    height: u8,
    style: BlockStyle,
    render: Mutex<RenderBlock>,
    disposition: BlockDisposition,
}

pub struct BlockProperties<P> {
    pub position: P,
    pub height: u8,
    pub style: BlockStyle,
    pub render: Box<dyn Send + Fn(&mut BlockContext) -> AnyElement>,
    pub disposition: BlockDisposition,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BlockStyle {
    Fixed,
    Flex,
    Sticky,
}

pub struct BlockContext<'a, 'b> {
    pub context: &'b mut WindowContext<'a>,
    pub anchor_x: Pixels,
    pub max_width: Pixels,
    pub gutter_dimensions: &'b GutterDimensions,
    pub em_width: Pixels,
    pub line_height: Pixels,
    pub block_id: usize,
    pub editor_style: &'b EditorStyle,
}

/// Whether the block should be considered above or below the anchor line
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockDisposition {
    Above,
    Below,
}

#[derive(Clone, Debug)]
struct Transform {
    summary: TransformSummary,
    block: Option<TransformBlock>,
}

pub(crate) enum BlockType {
    Custom(BlockId),
    Header,
    Footer,
}

pub(crate) trait BlockLike {
    fn block_type(&self) -> BlockType;
    fn disposition(&self) -> BlockDisposition;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum TransformBlock {
    Custom(Arc<Block>),
    ExcerptHeader {
        id: ExcerptId,
        buffer: BufferSnapshot,
        range: ExcerptRange<text::Anchor>,
        height: u8,
        starts_new_buffer: bool,
        show_excerpt_controls: bool,
    },
    ExcerptFooter {
        id: ExcerptId,
        disposition: BlockDisposition,
        height: u8,
    },
}

impl BlockLike for TransformBlock {
    fn block_type(&self) -> BlockType {
        match self {
            TransformBlock::Custom(block) => BlockType::Custom(block.id),
            TransformBlock::ExcerptHeader { .. } => BlockType::Header,
            TransformBlock::ExcerptFooter { .. } => BlockType::Footer,
        }
    }

    fn disposition(&self) -> BlockDisposition {
        self.disposition()
    }
}

impl TransformBlock {
    fn disposition(&self) -> BlockDisposition {
        match self {
            TransformBlock::Custom(block) => block.disposition,
            TransformBlock::ExcerptHeader { .. } => BlockDisposition::Above,
            TransformBlock::ExcerptFooter { disposition, .. } => *disposition,
        }
    }

    pub fn height(&self) -> u8 {
        match self {
            TransformBlock::Custom(block) => block.height,
            TransformBlock::ExcerptHeader { height, .. } => *height,
            TransformBlock::ExcerptFooter { height, .. } => *height,
        }
    }
}

impl Debug for TransformBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(block) => f.debug_struct("Custom").field("block", block).finish(),
            Self::ExcerptHeader {
                buffer,
                starts_new_buffer,
                id,
                ..
            } => f
                .debug_struct("ExcerptHeader")
                .field("id", &id)
                .field("path", &buffer.file().map(|f| f.path()))
                .field("starts_new_buffer", &starts_new_buffer)
                .finish(),
            TransformBlock::ExcerptFooter {
                id, disposition, ..
            } => f
                .debug_struct("ExcerptFooter")
                .field("id", &id)
                .field("disposition", &disposition)
                .finish(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct TransformSummary {
    input_rows: u32,
    output_rows: u32,
}

pub struct BlockChunks<'a> {
    transforms: sum_tree::Cursor<'a, Transform, (BlockRow, WrapRow)>,
    input_chunks: wrap_map::WrapChunks<'a>,
    input_chunk: Chunk<'a>,
    output_row: u32,
    max_output_row: u32,
}

#[derive(Clone)]
pub struct BlockBufferRows<'a> {
    transforms: sum_tree::Cursor<'a, Transform, (BlockRow, WrapRow)>,
    input_buffer_rows: wrap_map::WrapBufferRows<'a>,
    output_row: BlockRow,
    started: bool,
}

impl BlockMap {
    pub fn new(
        wrap_snapshot: WrapSnapshot,
        show_excerpt_controls: bool,
        buffer_header_height: u8,
        excerpt_header_height: u8,
        excerpt_footer_height: u8,
    ) -> Self {
        let row_count = wrap_snapshot.max_point().row() + 1;
        let map = Self {
            next_block_id: AtomicUsize::new(0),
            blocks: Vec::new(),
            transforms: RefCell::new(SumTree::from_item(Transform::isomorphic(row_count), &())),
            wrap_snapshot: RefCell::new(wrap_snapshot.clone()),
            show_excerpt_controls,
            buffer_header_height,
            excerpt_header_height,
            excerpt_footer_height,
        };
        map.sync(
            &wrap_snapshot,
            Patch::new(vec![Edit {
                old: 0..row_count,
                new: 0..row_count,
            }]),
        );
        map
    }

    pub fn read(&self, wrap_snapshot: WrapSnapshot, edits: Patch<u32>) -> BlockSnapshot {
        self.sync(&wrap_snapshot, edits);
        *self.wrap_snapshot.borrow_mut() = wrap_snapshot.clone();
        BlockSnapshot {
            wrap_snapshot,
            transforms: self.transforms.borrow().clone(),
        }
    }

    pub fn write(&mut self, wrap_snapshot: WrapSnapshot, edits: Patch<u32>) -> BlockMapWriter {
        self.sync(&wrap_snapshot, edits);
        *self.wrap_snapshot.borrow_mut() = wrap_snapshot;
        BlockMapWriter(self)
    }

    fn sync(&self, wrap_snapshot: &WrapSnapshot, mut edits: Patch<u32>) {
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
            let edit_end = max_point.row() + 1;
            edits = edits.compose([WrapEdit {
                old: edit_start..edit_end,
                new: edit_start..edit_end,
            }]);
        }

        let edits = edits.into_inner();
        if edits.is_empty() {
            return;
        }

        let mut transforms = self.transforms.borrow_mut();
        let mut new_transforms = SumTree::new();
        let old_row_count = transforms.summary().input_rows;
        let new_row_count = wrap_snapshot.max_point().row() + 1;
        let mut cursor = transforms.cursor::<WrapRow>();
        let mut last_block_ix = 0;
        let mut blocks_in_edit = Vec::new();
        let mut edits = edits.into_iter().peekable();

        while let Some(edit) = edits.next() {
            // Preserve any old transforms that precede this edit.
            let old_start = WrapRow(edit.old.start);
            let new_start = WrapRow(edit.new.start);
            new_transforms.append(cursor.slice(&old_start, Bias::Left, &()), &());
            if let Some(transform) = cursor.item() {
                if transform.is_isomorphic() && old_start == cursor.end(&()) {
                    new_transforms.push(transform.clone(), &());
                    cursor.next(&());
                    while let Some(transform) = cursor.item() {
                        if transform
                            .block
                            .as_ref()
                            .map_or(false, |b| b.disposition().is_below())
                        {
                            new_transforms.push(transform.clone(), &());
                            cursor.next(&());
                        } else {
                            break;
                        }
                    }
                }
            }

            // Preserve any portion of an old transform that precedes this edit.
            let extent_before_edit = old_start.0 - cursor.start().0;
            push_isomorphic(&mut new_transforms, extent_before_edit);

            // Skip over any old transforms that intersect this edit.
            let mut old_end = WrapRow(edit.old.end);
            let mut new_end = WrapRow(edit.new.end);
            cursor.seek(&old_end, Bias::Left, &());
            cursor.next(&());
            if old_end == *cursor.start() {
                while let Some(transform) = cursor.item() {
                    if transform
                        .block
                        .as_ref()
                        .map_or(false, |b| b.disposition().is_below())
                    {
                        cursor.next(&());
                    } else {
                        break;
                    }
                }
            }

            // Combine this edit with any subsequent edits that intersect the same transform.
            while let Some(next_edit) = edits.peek() {
                if next_edit.old.start <= cursor.start().0 {
                    old_end = WrapRow(next_edit.old.end);
                    new_end = WrapRow(next_edit.new.end);
                    cursor.seek(&old_end, Bias::Left, &());
                    cursor.next(&());
                    if old_end == *cursor.start() {
                        while let Some(transform) = cursor.item() {
                            if transform
                                .block
                                .as_ref()
                                .map_or(false, |b| b.disposition().is_below())
                            {
                                cursor.next(&());
                            } else {
                                break;
                            }
                        }
                    }
                    edits.next();
                } else {
                    break;
                }
            }

            // Find the blocks within this edited region.
            let new_buffer_start =
                wrap_snapshot.to_point(WrapPoint::new(new_start.0, 0), Bias::Left);
            let start_bound = Bound::Included(new_buffer_start);
            let start_block_ix = match self.blocks[last_block_ix..].binary_search_by(|probe| {
                probe
                    .position
                    .to_point(buffer)
                    .cmp(&new_buffer_start)
                    .then(Ordering::Greater)
            }) {
                Ok(ix) | Err(ix) => last_block_ix + ix,
            };

            let end_bound;
            let end_block_ix = if new_end.0 > wrap_snapshot.max_point().row() {
                end_bound = Bound::Unbounded;
                self.blocks.len()
            } else {
                let new_buffer_end =
                    wrap_snapshot.to_point(WrapPoint::new(new_end.0, 0), Bias::Left);
                end_bound = Bound::Excluded(new_buffer_end);
                match self.blocks[start_block_ix..].binary_search_by(|probe| {
                    probe
                        .position
                        .to_point(buffer)
                        .cmp(&new_buffer_end)
                        .then(Ordering::Greater)
                }) {
                    Ok(ix) | Err(ix) => start_block_ix + ix,
                }
            };
            last_block_ix = end_block_ix;

            debug_assert!(blocks_in_edit.is_empty());
            blocks_in_edit.extend(
                self.blocks[start_block_ix..end_block_ix]
                    .iter()
                    .map(|block| {
                        let mut position = block.position.to_point(buffer);
                        match block.disposition {
                            BlockDisposition::Above => position.column = 0,
                            BlockDisposition::Below => {
                                position.column = buffer.line_len(MultiBufferRow(position.row))
                            }
                        }
                        let position = wrap_snapshot.make_wrap_point(position, Bias::Left);
                        (position.row(), TransformBlock::Custom(block.clone()))
                    }),
            );

            if buffer.show_headers() {
                blocks_in_edit.extend(BlockMap::header_blocks(
                    self.show_excerpt_controls,
                    self.excerpt_footer_height,
                    self.buffer_header_height,
                    self.excerpt_header_height,
                    buffer,
                    (start_bound, end_bound),
                    wrap_snapshot,
                ));
            }

            BlockMap::sort_blocks(&mut blocks_in_edit);

            // For each of these blocks, insert a new isomorphic transform preceding the block,
            // and then insert the block itself.
            for (block_row, block) in blocks_in_edit.drain(..) {
                let insertion_row = match block.disposition() {
                    BlockDisposition::Above => block_row,
                    BlockDisposition::Below => block_row + 1,
                };
                let extent_before_block = insertion_row - new_transforms.summary().input_rows;
                push_isomorphic(&mut new_transforms, extent_before_block);
                new_transforms.push(Transform::block(block), &());
            }

            old_end = WrapRow(old_end.0.min(old_row_count));
            new_end = WrapRow(new_end.0.min(new_row_count));

            // Insert an isomorphic transform after the final block.
            let extent_after_last_block = new_end.0 - new_transforms.summary().input_rows;
            push_isomorphic(&mut new_transforms, extent_after_last_block);

            // Preserve any portion of the old transform after this edit.
            let extent_after_edit = cursor.start().0 - old_end.0;
            push_isomorphic(&mut new_transforms, extent_after_edit);
        }

        new_transforms.append(cursor.suffix(&()), &());
        debug_assert_eq!(
            new_transforms.summary().input_rows,
            wrap_snapshot.max_point().row() + 1
        );

        drop(cursor);
        *transforms = new_transforms;
    }

    pub fn replace(&mut self, mut renderers: HashMap<BlockId, RenderBlock>) {
        for block in &self.blocks {
            if let Some(render) = renderers.remove(&block.id) {
                *block.render.lock() = render;
            }
        }
    }

    pub fn show_excerpt_controls(&self) -> bool {
        self.show_excerpt_controls
    }

    pub fn header_blocks<'a, 'b: 'a, 'c: 'a + 'b, R, T>(
        show_excerpt_controls: bool,
        excerpt_footer_height: u8,
        buffer_header_height: u8,
        excerpt_header_height: u8,
        buffer: &'b multi_buffer::MultiBufferSnapshot,
        range: R,
        wrap_snapshot: &'c WrapSnapshot,
    ) -> impl Iterator<Item = (u32, TransformBlock)> + 'b
    where
        R: RangeBounds<T>,
        T: multi_buffer::ToOffset,
    {
        buffer
            .excerpt_boundaries_in_range(range)
            .flat_map(move |excerpt_boundary| {
                let wrap_row = wrap_snapshot
                    .make_wrap_point(Point::new(excerpt_boundary.row.0, 0), Bias::Left)
                    .row();

                [
                    show_excerpt_controls
                        .then(|| {
                            excerpt_boundary.prev.as_ref().map(|prev| {
                                (
                                    wrap_row,
                                    TransformBlock::ExcerptFooter {
                                        id: prev.id,
                                        height: excerpt_footer_height,
                                        disposition: if excerpt_boundary.next.is_some() {
                                            BlockDisposition::Above
                                        } else {
                                            BlockDisposition::Below
                                        },
                                    },
                                )
                            })
                        })
                        .flatten(),
                    excerpt_boundary.next.map(|next| {
                        let starts_new_buffer = excerpt_boundary
                            .prev
                            .map_or(true, |prev| prev.buffer_id != next.buffer_id);

                        (
                            wrap_row,
                            TransformBlock::ExcerptHeader {
                                id: next.id,
                                buffer: next.buffer,
                                range: next.range,
                                height: if starts_new_buffer {
                                    buffer_header_height
                                } else {
                                    excerpt_header_height
                                },
                                starts_new_buffer,
                                show_excerpt_controls,
                            },
                        )
                    }),
                ]
            })
            .flatten()
    }

    pub(crate) fn sort_blocks<B: BlockLike>(blocks: &mut Vec<(u32, B)>) {
        // Place excerpt headers and footers above custom blocks on the same row
        blocks.sort_unstable_by(|(row_a, block_a), (row_b, block_b)| {
            row_a.cmp(row_b).then_with(|| {
                block_a
                    .disposition()
                    .cmp(&block_b.disposition())
                    .then_with(|| match ((block_a.block_type()), (block_b.block_type())) {
                        (BlockType::Footer, BlockType::Footer) => Ordering::Equal,
                        (BlockType::Footer, _) => Ordering::Less,
                        (_, BlockType::Footer) => Ordering::Greater,
                        (BlockType::Header, BlockType::Header) => Ordering::Equal,
                        (BlockType::Header, _) => Ordering::Less,
                        (_, BlockType::Header) => Ordering::Greater,
                        (BlockType::Custom(a_id), BlockType::Custom(b_id)) => a_id.cmp(&b_id),
                    })
            })
        });
    }
}

fn push_isomorphic(tree: &mut SumTree<Transform>, rows: u32) {
    if rows == 0 {
        return;
    }

    let mut extent = Some(rows);
    tree.update_last(
        |last_transform| {
            if last_transform.is_isomorphic() {
                let extent = extent.take().unwrap();
                last_transform.summary.input_rows += extent;
                last_transform.summary.output_rows += extent;
            }
        },
        &(),
    );
    if let Some(extent) = extent {
        tree.push(Transform::isomorphic(extent), &());
    }
}

impl BlockPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
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

impl<'a> BlockMapWriter<'a> {
    pub fn insert(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
    ) -> Vec<BlockId> {
        let mut ids = Vec::new();
        let mut edits = Patch::default();
        let wrap_snapshot = &*self.0.wrap_snapshot.borrow();
        let buffer = wrap_snapshot.buffer_snapshot();

        for block in blocks {
            let id = BlockId(self.0.next_block_id.fetch_add(1, SeqCst));
            ids.push(id);

            let position = block.position;
            let point = position.to_point(buffer);
            let wrap_row = wrap_snapshot
                .make_wrap_point(Point::new(point.row, 0), Bias::Left)
                .row();
            let start_row = wrap_snapshot.prev_row_boundary(WrapPoint::new(wrap_row, 0));
            let end_row = wrap_snapshot
                .next_row_boundary(WrapPoint::new(wrap_row, 0))
                .unwrap_or(wrap_snapshot.max_point().row() + 1);

            let block_ix = match self
                .0
                .blocks
                .binary_search_by(|probe| probe.position.cmp(&position, buffer))
            {
                Ok(ix) | Err(ix) => ix,
            };
            self.0.blocks.insert(
                block_ix,
                Arc::new(Block {
                    id,
                    position,
                    height: block.height,
                    render: Mutex::new(block.render),
                    disposition: block.disposition,
                    style: block.style,
                }),
            );

            edits = edits.compose([Edit {
                old: start_row..end_row,
                new: start_row..end_row,
            }]);
        }

        self.0.sync(wrap_snapshot, edits);
        ids
    }

    pub fn remove(&mut self, block_ids: HashSet<BlockId>) {
        let wrap_snapshot = &*self.0.wrap_snapshot.borrow();
        let buffer = wrap_snapshot.buffer_snapshot();
        let mut edits = Patch::default();
        let mut last_block_buffer_row = None;
        self.0.blocks.retain(|block| {
            if block_ids.contains(&block.id) {
                let buffer_row = block.position.to_point(buffer).row;
                if last_block_buffer_row != Some(buffer_row) {
                    last_block_buffer_row = Some(buffer_row);
                    let wrap_row = wrap_snapshot
                        .make_wrap_point(Point::new(buffer_row, 0), Bias::Left)
                        .row();
                    let start_row = wrap_snapshot.prev_row_boundary(WrapPoint::new(wrap_row, 0));
                    let end_row = wrap_snapshot
                        .next_row_boundary(WrapPoint::new(wrap_row, 0))
                        .unwrap_or(wrap_snapshot.max_point().row() + 1);
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
        self.0.sync(wrap_snapshot, edits);
    }
}

impl BlockSnapshot {
    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(
            0..self.transforms.summary().output_rows,
            false,
            Highlights::default(),
        )
        .map(|chunk| chunk.text)
        .collect()
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        rows: Range<u32>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> BlockChunks<'a> {
        let max_output_row = cmp::min(rows.end, self.transforms.summary().output_rows);
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        let input_end = {
            cursor.seek(&BlockRow(rows.end), Bias::Right, &());
            let overshoot = if cursor
                .item()
                .map_or(false, |transform| transform.is_isomorphic())
            {
                rows.end - cursor.start().0 .0
            } else {
                0
            };
            cursor.start().1 .0 + overshoot
        };
        let input_start = {
            cursor.seek(&BlockRow(rows.start), Bias::Right, &());
            let overshoot = if cursor
                .item()
                .map_or(false, |transform| transform.is_isomorphic())
            {
                rows.start - cursor.start().0 .0
            } else {
                0
            };
            cursor.start().1 .0 + overshoot
        };
        BlockChunks {
            input_chunks: self.wrap_snapshot.chunks(
                input_start..input_end,
                language_aware,
                highlights,
            ),
            input_chunk: Default::default(),
            transforms: cursor,
            output_row: rows.start,
            max_output_row,
        }
    }

    pub(super) fn buffer_rows(&self, start_row: BlockRow) -> BlockBufferRows {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&start_row, Bias::Right, &());
        let (output_start, input_start) = cursor.start();
        let overshoot = if cursor.item().map_or(false, |t| t.is_isomorphic()) {
            start_row.0 - output_start.0
        } else {
            0
        };
        let input_start_row = input_start.0 + overshoot;
        BlockBufferRows {
            transforms: cursor,
            input_buffer_rows: self.wrap_snapshot.buffer_rows(input_start_row),
            output_row: start_row,
            started: false,
        }
    }

    pub fn blocks_in_range(
        &self,
        rows: Range<u32>,
    ) -> impl Iterator<Item = (u32, &TransformBlock)> {
        let mut cursor = self.transforms.cursor::<BlockRow>();
        cursor.seek(&BlockRow(rows.start), Bias::Right, &());
        std::iter::from_fn(move || {
            while let Some(transform) = cursor.item() {
                let start_row = cursor.start().0;
                if start_row >= rows.end {
                    break;
                }
                if let Some(block) = &transform.block {
                    cursor.next(&());
                    return Some((start_row, block));
                } else {
                    cursor.next(&());
                }
            }
            None
        })
    }

    pub fn max_point(&self) -> BlockPoint {
        let row = self.transforms.summary().output_rows - 1;
        BlockPoint::new(row, self.line_len(BlockRow(row)))
    }

    pub fn longest_row(&self) -> u32 {
        let input_row = self.wrap_snapshot.longest_row();
        self.to_block_point(WrapPoint::new(input_row, 0)).row
    }

    pub(super) fn line_len(&self, row: BlockRow) -> u32 {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(row.0), Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let (output_start, input_start) = cursor.start();
            let overshoot = row.0 - output_start.0;
            if transform.block.is_some() {
                0
            } else {
                self.wrap_snapshot.line_len(input_start.0 + overshoot)
            }
        } else {
            panic!("row out of range");
        }
    }

    pub(super) fn is_block_line(&self, row: BlockRow) -> bool {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&row, Bias::Right, &());
        cursor.item().map_or(false, |t| t.block.is_some())
    }

    pub fn clip_point(&self, point: BlockPoint, bias: Bias) -> BlockPoint {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(point.row), Bias::Right, &());

        let max_input_row = WrapRow(self.transforms.summary().input_rows);
        let mut search_left =
            (bias == Bias::Left && cursor.start().1 .0 > 0) || cursor.end(&()).1 == max_input_row;
        let mut reversed = false;

        loop {
            if let Some(transform) = cursor.item() {
                if transform.is_isomorphic() {
                    let (output_start_row, input_start_row) = cursor.start();
                    let (output_end_row, input_end_row) = cursor.end(&());
                    let output_start = Point::new(output_start_row.0, 0);
                    let input_start = Point::new(input_start_row.0, 0);
                    let input_end = Point::new(input_end_row.0, 0);
                    let input_point = if point.row >= output_end_row.0 {
                        let line_len = self.wrap_snapshot.line_len(input_end_row.0 - 1);
                        self.wrap_snapshot
                            .clip_point(WrapPoint::new(input_end_row.0 - 1, line_len), bias)
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

                if search_left {
                    cursor.prev(&());
                } else {
                    cursor.next(&());
                }
            } else if reversed {
                return self.max_point();
            } else {
                reversed = true;
                search_left = !search_left;
                cursor.seek(&BlockRow(point.row), Bias::Right, &());
            }
        }
    }

    pub fn to_block_point(&self, wrap_point: WrapPoint) -> BlockPoint {
        let mut cursor = self.transforms.cursor::<(WrapRow, BlockRow)>();
        cursor.seek(&WrapRow(wrap_point.row()), Bias::Right, &());
        if let Some(transform) = cursor.item() {
            debug_assert!(transform.is_isomorphic());
        } else {
            return self.max_point();
        }

        let (input_start_row, output_start_row) = cursor.start();
        let input_start = Point::new(input_start_row.0, 0);
        let output_start = Point::new(output_start_row.0, 0);
        let input_overshoot = wrap_point.0 - input_start;
        BlockPoint(output_start + input_overshoot)
    }

    pub fn to_wrap_point(&self, block_point: BlockPoint) -> WrapPoint {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(block_point.row), Bias::Right, &());
        if let Some(transform) = cursor.item() {
            match transform.block.as_ref().map(|b| b.disposition()) {
                Some(BlockDisposition::Above) => WrapPoint::new(cursor.start().1 .0, 0),
                Some(BlockDisposition::Below) => {
                    let wrap_row = cursor.start().1 .0 - 1;
                    WrapPoint::new(wrap_row, self.wrap_snapshot.line_len(wrap_row))
                }
                None => {
                    let overshoot = block_point.row - cursor.start().0 .0;
                    let wrap_row = cursor.start().1 .0 + overshoot;
                    WrapPoint::new(wrap_row, block_point.column)
                }
            }
        } else {
            self.wrap_snapshot.max_point()
        }
    }
}

impl Transform {
    fn isomorphic(rows: u32) -> Self {
        Self {
            summary: TransformSummary {
                input_rows: rows,
                output_rows: rows,
            },
            block: None,
        }
    }

    fn block(block: TransformBlock) -> Self {
        Self {
            summary: TransformSummary {
                input_rows: 0,
                output_rows: block.height() as u32,
            },
            block: Some(block),
        }
    }

    fn is_isomorphic(&self) -> bool {
        self.block.is_none()
    }
}

impl<'a> Iterator for BlockChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_row >= self.max_output_row {
            return None;
        }

        let transform = self.transforms.item()?;
        if transform.block.is_some() {
            let block_start = self.transforms.start().0 .0;
            let mut block_end = self.transforms.end(&()).0 .0;
            self.transforms.next(&());
            if self.transforms.item().is_none() {
                block_end -= 1;
            }

            let start_in_block = self.output_row - block_start;
            let end_in_block = cmp::min(self.max_output_row, block_end) - block_start;
            let line_count = end_in_block - start_in_block;
            self.output_row += line_count;

            return Some(Chunk {
                text: unsafe { std::str::from_utf8_unchecked(&NEWLINES[..line_count as usize]) },
                ..Default::default()
            });
        }

        if self.input_chunk.text.is_empty() {
            if let Some(input_chunk) = self.input_chunks.next() {
                self.input_chunk = input_chunk;
            } else {
                self.output_row += 1;
                if self.output_row < self.max_output_row {
                    self.transforms.next(&());
                    return Some(Chunk {
                        text: "\n",
                        ..Default::default()
                    });
                } else {
                    return None;
                }
            }
        }

        let transform_end = self.transforms.end(&()).0 .0;
        let (prefix_rows, prefix_bytes) =
            offset_for_row(self.input_chunk.text, transform_end - self.output_row);
        self.output_row += prefix_rows;
        let (prefix, suffix) = self.input_chunk.text.split_at(prefix_bytes);
        self.input_chunk.text = suffix;
        if self.output_row == transform_end {
            self.transforms.next(&());
        }

        Some(Chunk {
            text: prefix,
            ..self.input_chunk.clone()
        })
    }
}

impl<'a> Iterator for BlockBufferRows<'a> {
    type Item = Option<BlockRow>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.output_row.0 += 1;
        } else {
            self.started = true;
        }

        if self.output_row.0 >= self.transforms.end(&()).0 .0 {
            self.transforms.next(&());
        }

        let transform = self.transforms.item()?;
        if transform.block.is_some() {
            Some(None)
        } else {
            Some(self.input_buffer_rows.next().unwrap().map(BlockRow))
        }
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        self.summary.clone()
    }
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.input_rows += summary.input_rows;
        self.output_rows += summary.output_rows;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for WrapRow {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.input_rows;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for BlockRow {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output_rows;
    }
}

impl BlockDisposition {
    fn is_below(&self) -> bool {
        matches!(self, BlockDisposition::Below)
    }
}

impl<'a> Deref for BlockContext<'a, '_> {
    type Target = WindowContext<'a>;

    fn deref(&self) -> &Self::Target {
        self.context
    }
}

impl DerefMut for BlockContext<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.context
    }
}

impl Block {
    pub fn render(&self, cx: &mut BlockContext) -> AnyElement {
        self.render.lock()(cx)
    }

    pub fn position(&self) -> &Anchor {
        &self.position
    }

    pub fn style(&self) -> BlockStyle {
        self.style
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("id", &self.id)
            .field("position", &self.position)
            .field("disposition", &self.disposition)
            .finish()
    }
}

// Count the number of bytes prior to a target point. If the string doesn't contain the target
// point, return its total extent. Otherwise return the target point itself.
fn offset_for_row(s: &str, target: u32) -> (u32, usize) {
    let mut row = 0;
    let mut offset = 0;
    for (ix, line) in s.split('\n').enumerate() {
        if ix > 0 {
            row += 1;
            offset += 1;
        }
        if row >= target {
            break;
        }
        offset += line.len();
    }
    (row, offset)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::display_map::inlay_map::InlayMap;
    use crate::display_map::{fold_map::FoldMap, tab_map::TabMap, wrap_map::WrapMap};
    use gpui::{div, font, px, Element};
    use multi_buffer::MultiBuffer;
    use rand::prelude::*;
    use settings::SettingsStore;
    use util::RandomCharIter;

    #[gpui::test]
    fn test_offset_for_row() {
        assert_eq!(offset_for_row("", 0), (0, 0));
        assert_eq!(offset_for_row("", 1), (0, 0));
        assert_eq!(offset_for_row("abcd", 0), (0, 0));
        assert_eq!(offset_for_row("abcd", 1), (0, 4));
        assert_eq!(offset_for_row("\n", 0), (0, 0));
        assert_eq!(offset_for_row("\n", 1), (1, 1));
        assert_eq!(offset_for_row("abc\ndef\nghi", 0), (0, 0));
        assert_eq!(offset_for_row("abc\ndef\nghi", 1), (1, 4));
        assert_eq!(offset_for_row("abc\ndef\nghi", 2), (2, 8));
        assert_eq!(offset_for_row("abc\ndef\nghi", 3), (2, 11));
    }

    #[gpui::test]
    fn test_basic_blocks(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx));

        let text = "aaa\nbbb\nccc\nddd";

        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut tab_map, tab_snapshot) = TabMap::new(fold_snapshot, 1.try_into().unwrap());
        let (wrap_map, wraps_snapshot) =
            cx.update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), None, cx));
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), true, 1, 1, 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default());
        let block_ids = writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                position: buffer_snapshot.anchor_after(Point::new(1, 0)),
                height: 1,
                disposition: BlockDisposition::Above,
                render: Box::new(|_| div().into_any()),
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                position: buffer_snapshot.anchor_after(Point::new(1, 2)),
                height: 2,
                disposition: BlockDisposition::Above,
                render: Box::new(|_| div().into_any()),
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                position: buffer_snapshot.anchor_after(Point::new(3, 3)),
                height: 3,
                disposition: BlockDisposition::Below,
                render: Box::new(|_| div().into_any()),
            },
        ]);

        let snapshot = block_map.read(wraps_snapshot, Default::default());
        assert_eq!(snapshot.text(), "aaa\n\n\n\nbbb\nccc\nddd\n\n\n");

        let blocks = snapshot
            .blocks_in_range(0..8)
            .map(|(start_row, block)| {
                let block = block.as_custom().unwrap();
                (start_row..start_row + block.height as u32, block.id)
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
            snapshot.to_block_point(WrapPoint::new(0, 3)),
            BlockPoint::new(0, 3)
        );
        assert_eq!(
            snapshot.to_block_point(WrapPoint::new(1, 0)),
            BlockPoint::new(4, 0)
        );
        assert_eq!(
            snapshot.to_block_point(WrapPoint::new(3, 3)),
            BlockPoint::new(6, 3)
        );

        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(0, 3)),
            WrapPoint::new(0, 3)
        );
        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(1, 0)),
            WrapPoint::new(1, 0)
        );
        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(3, 0)),
            WrapPoint::new(1, 0)
        );
        assert_eq!(
            snapshot.to_wrap_point(BlockPoint::new(7, 0)),
            WrapPoint::new(3, 3)
        );

        assert_eq!(
            snapshot.clip_point(BlockPoint::new(1, 0), Bias::Left),
            BlockPoint::new(0, 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(1, 0), Bias::Right),
            BlockPoint::new(4, 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(1, 1), Bias::Left),
            BlockPoint::new(0, 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(1, 1), Bias::Right),
            BlockPoint::new(4, 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(4, 0), Bias::Left),
            BlockPoint::new(4, 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(4, 0), Bias::Right),
            BlockPoint::new(4, 0)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(6, 3), Bias::Left),
            BlockPoint::new(6, 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(6, 3), Bias::Right),
            BlockPoint::new(6, 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(7, 0), Bias::Left),
            BlockPoint::new(6, 3)
        );
        assert_eq!(
            snapshot.clip_point(BlockPoint::new(7, 0), Bias::Right),
            BlockPoint::new(6, 3)
        );

        assert_eq!(
            snapshot
                .buffer_rows(BlockRow(0))
                .map(|row| row.map(|r| r.0))
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
        let snapshot = block_map.read(wraps_snapshot, wrap_edits);
        assert_eq!(snapshot.text(), "aaa\n\nb!!!\n\n\nbb\nccc\nddd\n\n\n");
    }

    #[gpui::test]
    fn test_blocks_on_wrapped_lines(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| init_test(cx));

        let _font_id = cx.text_system().font_id(&font("Helvetica")).unwrap();

        let text = "one two three\nfour five six\nseven eight";

        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (_, wraps_snapshot) = cx.update(|cx| {
            WrapMap::new(tab_snapshot, font("Helvetica"), px(14.0), Some(px(60.)), cx)
        });
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), true, 1, 1, 0);

        let mut writer = block_map.write(wraps_snapshot.clone(), Default::default());
        writer.insert(vec![
            BlockProperties {
                style: BlockStyle::Fixed,
                position: buffer_snapshot.anchor_after(Point::new(1, 12)),
                disposition: BlockDisposition::Above,
                render: Box::new(|_| div().into_any()),
                height: 1,
            },
            BlockProperties {
                style: BlockStyle::Fixed,
                position: buffer_snapshot.anchor_after(Point::new(1, 1)),
                disposition: BlockDisposition::Below,
                render: Box::new(|_| div().into_any()),
                height: 1,
            },
        ]);

        // Blocks with an 'above' disposition go above their corresponding buffer line.
        // Blocks with a 'below' disposition go below their corresponding buffer line.
        let snapshot = block_map.read(wraps_snapshot, Default::default());
        assert_eq!(
            snapshot.text(),
            "one two \nthree\n\nfour five \nsix\n\nseven \neight"
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_blocks(cx: &mut gpui::TestAppContext, mut rng: StdRng) {
        cx.update(|cx| init_test(cx));

        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let wrap_width = if rng.gen_bool(0.2) {
            None
        } else {
            Some(px(rng.gen_range(0.0..=100.0)))
        };
        let tab_size = 1.try_into().unwrap();
        let font_size = px(14.0);
        let buffer_start_header_height = rng.gen_range(1..=5);
        let excerpt_header_height = rng.gen_range(1..=5);
        let excerpt_footer_height = rng.gen_range(1..=5);

        log::info!("Wrap width: {:?}", wrap_width);
        log::info!("Excerpt Header Height: {:?}", excerpt_header_height);
        log::info!("Excerpt Footer Height: {:?}", excerpt_footer_height);

        let buffer = if rng.gen() {
            let len = rng.gen_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            log::info!("initial buffer text: {:?}", text);
            cx.update(|cx| MultiBuffer::build_simple(&text, cx))
        } else {
            cx.update(|cx| MultiBuffer::build_random(&mut rng, cx))
        };

        let mut buffer_snapshot = cx.update(|cx| buffer.read(cx).snapshot(cx));
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut tab_map, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        let (wrap_map, wraps_snapshot) = cx
            .update(|cx| WrapMap::new(tab_snapshot, font("Helvetica"), font_size, wrap_width, cx));
        let mut block_map = BlockMap::new(
            wraps_snapshot,
            true,
            buffer_start_header_height,
            excerpt_header_height,
            excerpt_footer_height,
        );
        let mut custom_blocks = Vec::new();

        for _ in 0..operations {
            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=19 => {
                    let wrap_width = if rng.gen_bool(0.2) {
                        None
                    } else {
                        Some(px(rng.gen_range(0.0..=100.0)))
                    };
                    log::info!("Setting wrap width to {:?}", wrap_width);
                    wrap_map.update(cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                20..=39 => {
                    let block_count = rng.gen_range(1..=5);
                    let block_properties = (0..block_count)
                        .map(|_| {
                            let buffer = cx.update(|cx| buffer.read(cx).read(cx).clone());
                            let position = buffer.anchor_after(
                                buffer.clip_offset(rng.gen_range(0..=buffer.len()), Bias::Left),
                            );

                            let disposition = if rng.gen() {
                                BlockDisposition::Above
                            } else {
                                BlockDisposition::Below
                            };
                            let height = rng.gen_range(1..5);
                            log::info!(
                                "inserting block {:?} {:?} with height {}",
                                disposition,
                                position.to_point(&buffer),
                                height
                            );
                            BlockProperties {
                                style: BlockStyle::Fixed,
                                position,
                                height,
                                disposition,
                                render: Box::new(|_| div().into_any()),
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
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits);
                    let block_ids =
                        block_map.insert(block_properties.iter().map(|props| BlockProperties {
                            position: props.position,
                            height: props.height,
                            style: props.style,
                            render: Box::new(|_| div().into_any()),
                            disposition: props.disposition,
                        }));
                    for (block_id, props) in block_ids.into_iter().zip(block_properties) {
                        custom_blocks.push((block_id, props));
                    }
                }
                40..=59 if !custom_blocks.is_empty() => {
                    let block_count = rng.gen_range(1..=4.min(custom_blocks.len()));
                    let block_ids_to_remove = (0..block_count)
                        .map(|_| {
                            custom_blocks
                                .remove(rng.gen_range(0..custom_blocks.len()))
                                .0
                        })
                        .collect();

                    let (inlay_snapshot, inlay_edits) =
                        inlay_map.sync(buffer_snapshot.clone(), vec![]);
                    let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
                    let (tab_snapshot, tab_edits) =
                        tab_map.sync(fold_snapshot, fold_edits, tab_size);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tab_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits);
                    block_map.remove(block_ids_to_remove);
                }
                _ => {
                    buffer.update(cx, |buffer, cx| {
                        let mutation_count = rng.gen_range(1..=5);
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
            let blocks_snapshot = block_map.read(wraps_snapshot.clone(), wrap_edits);
            assert_eq!(
                blocks_snapshot.transforms.summary().input_rows,
                wraps_snapshot.max_point().row() + 1
            );
            log::info!("blocks text: {:?}", blocks_snapshot.text());

            let mut expected_blocks = Vec::new();
            expected_blocks.extend(custom_blocks.iter().map(|(id, block)| {
                let mut position = block.position.to_point(&buffer_snapshot);
                match block.disposition {
                    BlockDisposition::Above => {
                        position.column = 0;
                    }
                    BlockDisposition::Below => {
                        position.column = buffer_snapshot.line_len(MultiBufferRow(position.row));
                    }
                };
                let row = wraps_snapshot.make_wrap_point(position, Bias::Left).row();
                (
                    row,
                    ExpectedBlock::Custom {
                        disposition: block.disposition,
                        id: *id,
                        height: block.height,
                    },
                )
            }));

            // Note that this needs to be synced with the related section in BlockMap::sync
            expected_blocks.extend(
                BlockMap::header_blocks(
                    true,
                    excerpt_footer_height,
                    buffer_start_header_height,
                    excerpt_header_height,
                    &buffer_snapshot,
                    0..,
                    &wraps_snapshot,
                )
                .map(|(row, block)| (row, block.into())),
            );

            BlockMap::sort_blocks(&mut expected_blocks);

            let mut sorted_blocks_iter = expected_blocks.into_iter().peekable();

            let input_buffer_rows = buffer_snapshot
                .buffer_rows(MultiBufferRow(0))
                .collect::<Vec<_>>();
            let mut expected_buffer_rows = Vec::new();
            let mut expected_text = String::new();
            let mut expected_block_positions = Vec::new();
            let input_text = wraps_snapshot.text();
            for (row, input_line) in input_text.split('\n').enumerate() {
                let row = row as u32;
                if row > 0 {
                    expected_text.push('\n');
                }

                let buffer_row = input_buffer_rows[wraps_snapshot
                    .to_point(WrapPoint::new(row, 0), Bias::Left)
                    .row as usize];

                while let Some((block_row, block)) = sorted_blocks_iter.peek() {
                    if *block_row == row && block.disposition() == BlockDisposition::Above {
                        let (_, block) = sorted_blocks_iter.next().unwrap();
                        let height = block.height() as usize;
                        expected_block_positions
                            .push((expected_text.matches('\n').count() as u32, block));
                        let text = "\n".repeat(height);
                        expected_text.push_str(&text);
                        for _ in 0..height {
                            expected_buffer_rows.push(None);
                        }
                    } else {
                        break;
                    }
                }

                let soft_wrapped = wraps_snapshot.to_tab_point(WrapPoint::new(row, 0)).column() > 0;
                expected_buffer_rows.push(if soft_wrapped { None } else { buffer_row });
                expected_text.push_str(input_line);

                while let Some((block_row, block)) = sorted_blocks_iter.peek() {
                    if *block_row == row && block.disposition() == BlockDisposition::Below {
                        let (_, block) = sorted_blocks_iter.next().unwrap();
                        let height = block.height() as usize;
                        expected_block_positions
                            .push((expected_text.matches('\n').count() as u32 + 1, block));
                        let text = "\n".repeat(height);
                        expected_text.push_str(&text);
                        for _ in 0..height {
                            expected_buffer_rows.push(None);
                        }
                    } else {
                        break;
                    }
                }
            }

            let expected_lines = expected_text.split('\n').collect::<Vec<_>>();
            let expected_row_count = expected_lines.len();
            for start_row in 0..expected_row_count {
                let expected_text = expected_lines[start_row..].join("\n");
                let actual_text = blocks_snapshot
                    .chunks(
                        start_row as u32..blocks_snapshot.max_point().row + 1,
                        false,
                        Highlights::default(),
                    )
                    .map(|chunk| chunk.text)
                    .collect::<String>();
                assert_eq!(
                    actual_text, expected_text,
                    "incorrect text starting from row {}",
                    start_row
                );
                assert_eq!(
                    blocks_snapshot
                        .buffer_rows(BlockRow(start_row as u32))
                        .map(|row| row.map(|r| r.0))
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[start_row..]
                );
            }

            assert_eq!(
                blocks_snapshot
                    .blocks_in_range(0..(expected_row_count as u32))
                    .map(|(row, block)| (row, block.clone().into()))
                    .collect::<Vec<_>>(),
                expected_block_positions
            );

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
                expected_longest_rows.contains(&longest_row),
                "incorrect longest row {}. expected {:?} with length {}",
                longest_row,
                expected_longest_rows,
                longest_line_len,
            );

            for row in 0..=blocks_snapshot.wrap_snapshot.max_point().row() {
                let wrap_point = WrapPoint::new(row, 0);
                let block_point = blocks_snapshot.to_block_point(wrap_point);
                assert_eq!(blocks_snapshot.to_wrap_point(block_point), wrap_point);
            }

            let mut block_point = BlockPoint::new(0, 0);
            for c in expected_text.chars() {
                let left_point = blocks_snapshot.clip_point(block_point, Bias::Left);
                let left_buffer_point = blocks_snapshot.to_point(left_point, Bias::Left);
                assert_eq!(
                    blocks_snapshot.to_block_point(blocks_snapshot.to_wrap_point(left_point)),
                    left_point
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
                    blocks_snapshot.to_block_point(blocks_snapshot.to_wrap_point(right_point)),
                    right_point
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
        }

        #[derive(Debug, Eq, PartialEq)]
        enum ExpectedBlock {
            ExcerptHeader {
                height: u8,
                starts_new_buffer: bool,
            },
            ExcerptFooter {
                height: u8,
                disposition: BlockDisposition,
            },
            Custom {
                disposition: BlockDisposition,
                id: BlockId,
                height: u8,
            },
        }

        impl BlockLike for ExpectedBlock {
            fn block_type(&self) -> BlockType {
                match self {
                    ExpectedBlock::Custom { id, .. } => BlockType::Custom(*id),
                    ExpectedBlock::ExcerptHeader { .. } => BlockType::Header,
                    ExpectedBlock::ExcerptFooter { .. } => BlockType::Footer,
                }
            }

            fn disposition(&self) -> BlockDisposition {
                self.disposition()
            }
        }

        impl ExpectedBlock {
            fn height(&self) -> u8 {
                match self {
                    ExpectedBlock::ExcerptHeader { height, .. } => *height,
                    ExpectedBlock::Custom { height, .. } => *height,
                    ExpectedBlock::ExcerptFooter { height, .. } => *height,
                }
            }

            fn disposition(&self) -> BlockDisposition {
                match self {
                    ExpectedBlock::ExcerptHeader { .. } => BlockDisposition::Above,
                    ExpectedBlock::Custom { disposition, .. } => *disposition,
                    ExpectedBlock::ExcerptFooter { disposition, .. } => *disposition,
                }
            }
        }

        impl From<TransformBlock> for ExpectedBlock {
            fn from(block: TransformBlock) -> Self {
                match block {
                    TransformBlock::Custom(block) => ExpectedBlock::Custom {
                        id: block.id,
                        disposition: block.disposition,
                        height: block.height,
                    },
                    TransformBlock::ExcerptHeader {
                        height,
                        starts_new_buffer,
                        ..
                    } => ExpectedBlock::ExcerptHeader {
                        height,
                        starts_new_buffer,
                    },
                    TransformBlock::ExcerptFooter {
                        height,
                        disposition,
                        ..
                    } => ExpectedBlock::ExcerptFooter {
                        height,
                        disposition,
                    },
                }
            }
        }
    }

    fn init_test(cx: &mut gpui::AppContext) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
    }

    impl TransformBlock {
        fn as_custom(&self) -> Option<&Block> {
            match self {
                TransformBlock::Custom(block) => Some(block),
                TransformBlock::ExcerptHeader { .. } => None,
                TransformBlock::ExcerptFooter { .. } => None,
            }
        }
    }

    impl BlockSnapshot {
        fn to_point(&self, point: BlockPoint, bias: Bias) -> Point {
            self.wrap_snapshot.to_point(self.to_wrap_point(point), bias)
        }
    }
}
