use super::wrap_map::{self, WrapEdit, WrapPoint, WrapSnapshot};
use crate::{Anchor, ToPoint as _};
use collections::{Bound, HashMap, HashSet};
use gpui::{AppContext, ElementBox};
use language::{BufferSnapshot, Chunk};
use parking_lot::Mutex;
use std::{
    cmp::{self, Ordering},
    fmt::Debug,
    ops::{Deref, Range},
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use sum_tree::{Bias, SumTree};
use text::{Edit, Point};

const NEWLINES: &'static [u8] = &[b'\n'; u8::MAX as usize];

pub struct BlockMap {
    next_block_id: AtomicUsize,
    wrap_snapshot: Mutex<WrapSnapshot>,
    blocks: Vec<Arc<Block>>,
    transforms: Mutex<SumTree<Transform>>,
    excerpt_header_height: u8,
}

pub struct BlockMapWriter<'a>(&'a mut BlockMap);

pub struct BlockSnapshot {
    wrap_snapshot: WrapSnapshot,
    transforms: SumTree<Transform>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(usize);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct BlockPoint(pub super::Point);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
struct BlockRow(u32);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
struct WrapRow(u32);

pub type RenderBlock = Arc<dyn Fn(&BlockContext) -> ElementBox>;

pub struct Block {
    id: BlockId,
    position: Anchor,
    height: u8,
    render: Mutex<RenderBlock>,
    disposition: BlockDisposition,
}

#[derive(Clone)]
pub struct BlockProperties<P>
where
    P: Clone,
{
    pub position: P,
    pub height: u8,
    pub render: Arc<dyn Fn(&BlockContext) -> ElementBox>,
    pub disposition: BlockDisposition,
}

pub struct BlockContext<'a> {
    pub cx: &'a AppContext,
    pub anchor_x: f32,
    pub scroll_x: f32,
    pub gutter_width: f32,
    pub gutter_padding: f32,
    pub em_width: f32,
    pub line_height: f32,
}

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

#[derive(Clone)]
pub enum TransformBlock {
    Custom(Arc<Block>),
    ExcerptHeader {
        buffer: BufferSnapshot,
        range: Range<text::Anchor>,
        height: u8,
    },
}

impl TransformBlock {
    fn disposition(&self) -> BlockDisposition {
        match self {
            TransformBlock::Custom(block) => block.disposition,
            TransformBlock::ExcerptHeader { .. } => BlockDisposition::Above,
        }
    }

    pub fn height(&self) -> u8 {
        match self {
            TransformBlock::Custom(block) => block.height,
            TransformBlock::ExcerptHeader { height, .. } => *height,
        }
    }
}

impl Debug for TransformBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(block) => f.debug_struct("Custom").field("block", block).finish(),
            Self::ExcerptHeader { buffer, .. } => f
                .debug_struct("ExcerptHeader")
                .field("path", &buffer.path())
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

pub struct BlockBufferRows<'a> {
    transforms: sum_tree::Cursor<'a, Transform, (BlockRow, WrapRow)>,
    input_buffer_rows: wrap_map::WrapBufferRows<'a>,
    output_row: u32,
    started: bool,
}

impl BlockMap {
    pub fn new(wrap_snapshot: WrapSnapshot, excerpt_header_height: u8) -> Self {
        let row_count = wrap_snapshot.max_point().row() + 1;
        let map = Self {
            next_block_id: AtomicUsize::new(0),
            blocks: Vec::new(),
            transforms: Mutex::new(SumTree::from_item(Transform::isomorphic(row_count), &())),
            wrap_snapshot: Mutex::new(wrap_snapshot.clone()),
            excerpt_header_height,
        };
        map.sync(
            &wrap_snapshot,
            vec![Edit {
                old: 0..row_count,
                new: 0..row_count,
            }],
        );
        map
    }

    pub fn read(&self, wrap_snapshot: WrapSnapshot, edits: Vec<WrapEdit>) -> BlockSnapshot {
        self.sync(&wrap_snapshot, edits);
        *self.wrap_snapshot.lock() = wrap_snapshot.clone();
        BlockSnapshot {
            wrap_snapshot,
            transforms: self.transforms.lock().clone(),
        }
    }

    pub fn write(&mut self, wrap_snapshot: WrapSnapshot, edits: Vec<WrapEdit>) -> BlockMapWriter {
        self.sync(&wrap_snapshot, edits);
        *self.wrap_snapshot.lock() = wrap_snapshot;
        BlockMapWriter(self)
    }

    fn sync(&self, wrap_snapshot: &WrapSnapshot, edits: Vec<WrapEdit>) {
        if edits.is_empty() {
            return;
        }

        let buffer = wrap_snapshot.buffer_snapshot();
        let mut transforms = self.transforms.lock();
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
            new_transforms.push_tree(cursor.slice(&old_start, Bias::Left, &()), &());
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
            let start_anchor = buffer.anchor_before(new_buffer_start);
            let start_bound = Bound::Included(start_anchor.clone());
            let start_block_ix = match self.blocks[last_block_ix..].binary_search_by(|probe| {
                probe
                    .position
                    .cmp(&start_anchor, &buffer)
                    .unwrap()
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
                let end_anchor = buffer.anchor_before(new_buffer_end);
                end_bound = Bound::Excluded(end_anchor.clone());
                match self.blocks[start_block_ix..].binary_search_by(|probe| {
                    probe
                        .position
                        .cmp(&end_anchor, &buffer)
                        .unwrap()
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
                        let mut position = block.position.to_point(&buffer);
                        match block.disposition {
                            BlockDisposition::Above => position.column = 0,
                            BlockDisposition::Below => {
                                position.column = buffer.line_len(position.row)
                            }
                        }
                        let position = wrap_snapshot.from_point(position, Bias::Left);
                        (position.row(), TransformBlock::Custom(block.clone()))
                    }),
            );
            blocks_in_edit.extend(
                buffer
                    .excerpt_boundaries_in_range((start_bound, end_bound))
                    .map(|excerpt_boundary| {
                        (
                            wrap_snapshot
                                .from_point(Point::new(excerpt_boundary.row, 0), Bias::Left)
                                .row(),
                            TransformBlock::ExcerptHeader {
                                buffer: excerpt_boundary.buffer,
                                range: excerpt_boundary.range,
                                height: self.excerpt_header_height,
                            },
                        )
                    }),
            );

            // Place excerpt headers above custom blocks on the same row.
            blocks_in_edit.sort_unstable_by(|(row_a, block_a), (row_b, block_b)| {
                row_a.cmp(&row_b).then_with(|| match (block_a, block_b) {
                    (
                        TransformBlock::ExcerptHeader { .. },
                        TransformBlock::ExcerptHeader { .. },
                    ) => Ordering::Equal,
                    (TransformBlock::ExcerptHeader { .. }, _) => Ordering::Less,
                    (_, TransformBlock::ExcerptHeader { .. }) => Ordering::Greater,
                    (TransformBlock::Custom(block_a), TransformBlock::Custom(block_b)) => block_a
                        .disposition
                        .cmp(&block_b.disposition)
                        .then_with(|| block_a.id.cmp(&block_b.id)),
                })
            });

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

        new_transforms.push_tree(cursor.suffix(&()), &());
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
        let mut edits = Vec::<Edit<u32>>::new();
        let wrap_snapshot = &*self.0.wrap_snapshot.lock();
        let buffer = wrap_snapshot.buffer_snapshot();

        for block in blocks {
            let id = BlockId(self.0.next_block_id.fetch_add(1, SeqCst));
            ids.push(id);

            let position = block.position;
            let point = position.to_point(&buffer);
            let wrap_row = wrap_snapshot
                .from_point(Point::new(point.row, 0), Bias::Left)
                .row();
            let start_row = wrap_snapshot.prev_row_boundary(WrapPoint::new(wrap_row, 0));
            let end_row = wrap_snapshot
                .next_row_boundary(WrapPoint::new(wrap_row, 0))
                .unwrap_or(wrap_snapshot.max_point().row() + 1);

            let block_ix = match self
                .0
                .blocks
                .binary_search_by(|probe| probe.position.cmp(&position, &buffer).unwrap())
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
                }),
            );

            if let Err(edit_ix) = edits.binary_search_by_key(&start_row, |edit| edit.old.start) {
                edits.insert(
                    edit_ix,
                    Edit {
                        old: start_row..end_row,
                        new: start_row..end_row,
                    },
                );
            }
        }

        self.0.sync(wrap_snapshot, edits);
        ids
    }

    pub fn remove(&mut self, block_ids: HashSet<BlockId>) {
        let wrap_snapshot = &*self.0.wrap_snapshot.lock();
        let buffer = wrap_snapshot.buffer_snapshot();
        let mut edits = Vec::new();
        let mut last_block_buffer_row = None;
        self.0.blocks.retain(|block| {
            if block_ids.contains(&block.id) {
                let buffer_row = block.position.to_point(&buffer).row;
                if last_block_buffer_row != Some(buffer_row) {
                    last_block_buffer_row = Some(buffer_row);
                    let wrap_row = wrap_snapshot
                        .from_point(Point::new(buffer_row, 0), Bias::Left)
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
        self.chunks(0..self.transforms.summary().output_rows, false)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn chunks<'a>(&'a self, rows: Range<u32>, language_aware: bool) -> BlockChunks<'a> {
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
            input_chunks: self
                .wrap_snapshot
                .chunks(input_start..input_end, language_aware),
            input_chunk: Default::default(),
            transforms: cursor,
            output_row: rows.start,
            max_output_row,
        }
    }

    pub fn buffer_rows<'a>(&'a self, start_row: u32) -> BlockBufferRows<'a> {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(start_row), Bias::Right, &());
        let (output_start, input_start) = cursor.start();
        let overshoot = if cursor.item().map_or(false, |t| t.is_isomorphic()) {
            start_row - output_start.0
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

    pub fn blocks_in_range<'a>(
        &'a self,
        rows: Range<u32>,
    ) -> impl Iterator<Item = (u32, &'a TransformBlock)> {
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
        BlockPoint::new(row, self.line_len(row))
    }

    pub fn longest_row(&self) -> u32 {
        let input_row = self.wrap_snapshot.longest_row();
        self.to_block_point(WrapPoint::new(input_row, 0)).row
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(row), Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let (output_start, input_start) = cursor.start();
            let overshoot = row - output_start.0;
            if transform.block.is_some() {
                0
            } else {
                self.wrap_snapshot.line_len(input_start.0 + overshoot)
            }
        } else {
            panic!("row out of range");
        }
    }

    pub fn is_block_line(&self, row: u32) -> bool {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(row), Bias::Right, &());
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
                highlight_id: None,
                diagnostic: None,
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
            ..self.input_chunk
        })
    }
}

impl<'a> Iterator for BlockBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.output_row += 1;
        } else {
            self.started = true;
        }

        if self.output_row >= self.transforms.end(&()).0 .0 {
            self.transforms.next(&());
        }

        let transform = self.transforms.item()?;
        if transform.block.is_some() {
            Some(None)
        } else {
            Some(self.input_buffer_rows.next().unwrap())
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

impl<'a> Deref for BlockContext<'a> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl Block {
    pub fn render(&self, cx: &BlockContext) -> ElementBox {
        self.render.lock()(cx)
    }

    pub fn position(&self) -> &Anchor {
        &self.position
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
        offset += line.len() as usize;
    }
    (row, offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_map::{fold_map::FoldMap, tab_map::TabMap, wrap_map::WrapMap};
    use crate::multi_buffer::MultiBuffer;
    use gpui::{elements::Empty, Element};
    use rand::prelude::*;
    use std::env;
    use text::RandomCharIter;

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
    fn test_basic_blocks(cx: &mut gpui::MutableAppContext) {
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();

        let text = "aaa\nbbb\nccc\nddd";

        let buffer = MultiBuffer::build_simple(text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (fold_map, folds_snapshot) = FoldMap::new(buffer_snapshot.clone());
        let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), 1);
        let (wrap_map, wraps_snapshot) = WrapMap::new(tabs_snapshot, font_id, 14.0, None, cx);
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), vec![]);
        writer.insert(vec![
            BlockProperties {
                position: buffer_snapshot.anchor_after(Point::new(1, 0)),
                height: 1,
                disposition: BlockDisposition::Above,
                render: Arc::new(|_| Empty::new().named("block 1")),
            },
            BlockProperties {
                position: buffer_snapshot.anchor_after(Point::new(1, 2)),
                height: 2,
                disposition: BlockDisposition::Above,
                render: Arc::new(|_| Empty::new().named("block 2")),
            },
            BlockProperties {
                position: buffer_snapshot.anchor_after(Point::new(3, 3)),
                height: 3,
                disposition: BlockDisposition::Below,
                render: Arc::new(|_| Empty::new().named("block 3")),
            },
        ]);

        let snapshot = block_map.read(wraps_snapshot, vec![]);
        assert_eq!(snapshot.text(), "aaa\n\n\n\nbbb\nccc\nddd\n\n\n");

        let blocks = snapshot
            .blocks_in_range(0..8)
            .map(|(start_row, block)| {
                let block = block.as_custom().unwrap();
                (
                    start_row..start_row + block.height as u32,
                    block
                        .render(&BlockContext {
                            cx,
                            anchor_x: 0.,
                            gutter_padding: 0.,
                            scroll_x: 0.,
                            gutter_width: 0.,
                            line_height: 0.,
                            em_width: 0.,
                        })
                        .name()
                        .unwrap()
                        .to_string(),
                )
            })
            .collect::<Vec<_>>();

        // When multiple blocks are on the same line, the newer blocks appear first.
        assert_eq!(
            blocks,
            &[
                (1..2, "block 1".to_string()),
                (2..4, "block 2".to_string()),
                (7..10, "block 3".to_string()),
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
            snapshot.buffer_rows(0).collect::<Vec<_>>(),
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
            buffer.edit([Point::new(1, 1)..Point::new(1, 1)], "!!!\n", cx);
            buffer.snapshot(cx)
        });

        let (folds_snapshot, fold_edits) =
            fold_map.read(buffer_snapshot, subscription.consume().into_inner());
        let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
        let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
            wrap_map.sync(tabs_snapshot, tab_edits, cx)
        });
        let snapshot = block_map.read(wraps_snapshot, wrap_edits);
        assert_eq!(snapshot.text(), "aaa\n\nb!!!\n\n\nbb\nccc\nddd\n\n\n");
    }

    #[gpui::test]
    fn test_blocks_on_wrapped_lines(cx: &mut gpui::MutableAppContext) {
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();

        let text = "one two three\nfour five six\nseven eight";

        let buffer = MultiBuffer::build_simple(text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, folds_snapshot) = FoldMap::new(buffer_snapshot.clone());
        let (_, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), 1);
        let (_, wraps_snapshot) = WrapMap::new(tabs_snapshot, font_id, 14.0, Some(60.), cx);
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), 1);

        let mut writer = block_map.write(wraps_snapshot.clone(), vec![]);
        writer.insert(vec![
            BlockProperties {
                position: buffer_snapshot.anchor_after(Point::new(1, 12)),
                disposition: BlockDisposition::Above,
                render: Arc::new(|_| Empty::new().named("block 1")),
                height: 1,
            },
            BlockProperties {
                position: buffer_snapshot.anchor_after(Point::new(1, 1)),
                disposition: BlockDisposition::Below,
                render: Arc::new(|_| Empty::new().named("block 2")),
                height: 1,
            },
        ]);

        // Blocks with an 'above' disposition go above their corresponding buffer line.
        // Blocks with a 'below' disposition go below their corresponding buffer line.
        let snapshot = block_map.read(wraps_snapshot, vec![]);
        assert_eq!(
            snapshot.text(),
            "one two \nthree\n\nfour five \nsix\n\nseven \neight"
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_blocks(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let wrap_width = if rng.gen_bool(0.2) {
            None
        } else {
            Some(rng.gen_range(0.0..=100.0))
        };
        let tab_size = 1;
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;
        let excerpt_header_height = rng.gen_range(1..=5);

        log::info!("Wrap width: {:?}", wrap_width);
        log::info!("Excerpt Header Height: {:?}", excerpt_header_height);

        let buffer = if rng.gen() {
            let len = rng.gen_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            log::info!("initial buffer text: {:?}", text);
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };

        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (fold_map, folds_snapshot) = FoldMap::new(buffer_snapshot.clone());
        let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), tab_size);
        let (wrap_map, wraps_snapshot) =
            WrapMap::new(tabs_snapshot, font_id, font_size, wrap_width, cx);
        let mut block_map = BlockMap::new(wraps_snapshot.clone(), excerpt_header_height);
        let mut custom_blocks = Vec::new();

        for _ in 0..operations {
            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=19 => {
                    let wrap_width = if rng.gen_bool(0.2) {
                        None
                    } else {
                        Some(rng.gen_range(0.0..=100.0))
                    };
                    log::info!("Setting wrap width to {:?}", wrap_width);
                    wrap_map.update(cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                20..=39 => {
                    let block_count = rng.gen_range(1..=5);
                    let block_properties = (0..block_count)
                        .map(|_| {
                            let buffer = buffer.read(cx).read(cx);
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
                                position,
                                height,
                                disposition,
                                render: Arc::new(|_| Empty::new().boxed()),
                            }
                        })
                        .collect::<Vec<_>>();

                    let (folds_snapshot, fold_edits) =
                        fold_map.read(buffer_snapshot.clone(), vec![]);
                    let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tabs_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits);
                    let block_ids = block_map.insert(block_properties.clone());
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

                    let (folds_snapshot, fold_edits) =
                        fold_map.read(buffer_snapshot.clone(), vec![]);
                    let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tabs_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits);
                    block_map.remove(block_ids_to_remove);
                }
                _ => {
                    buffer.update(cx, |buffer, cx| {
                        let edit_count = rng.gen_range(1..=5);
                        let subscription = buffer.subscribe();
                        buffer.randomly_edit(&mut rng, edit_count, cx);
                        buffer_snapshot = buffer.snapshot(cx);
                        buffer_edits.extend(subscription.consume());
                        log::info!("buffer text: {:?}", buffer_snapshot.text());
                    });
                }
            }

            let (folds_snapshot, fold_edits) = fold_map.read(buffer_snapshot.clone(), buffer_edits);
            let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
            let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                wrap_map.sync(tabs_snapshot, tab_edits, cx)
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
                        position.column = buffer_snapshot.line_len(position.row);
                    }
                };
                let row = wraps_snapshot.from_point(position, Bias::Left).row();
                (row, block.disposition, Some(*id), block.height)
            }));
            expected_blocks.extend(buffer_snapshot.excerpt_boundaries_in_range(0..).map(
                |boundary| {
                    let position =
                        wraps_snapshot.from_point(Point::new(boundary.row, 0), Bias::Left);
                    (
                        position.row(),
                        BlockDisposition::Above,
                        None,
                        excerpt_header_height,
                    )
                },
            ));
            expected_blocks
                .sort_unstable_by_key(|(row, disposition, id, _)| (*row, *disposition, *id));
            let mut sorted_blocks_iter = expected_blocks.iter().peekable();

            let input_buffer_rows = buffer_snapshot.buffer_rows(0).collect::<Vec<_>>();
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

                while let Some((block_row, disposition, id, height)) = sorted_blocks_iter.peek() {
                    if *block_row == row && *disposition == BlockDisposition::Above {
                        expected_block_positions
                            .push((expected_text.matches('\n').count() as u32, *id));
                        let text = "\n".repeat(*height as usize);
                        expected_text.push_str(&text);
                        for _ in 0..*height {
                            expected_buffer_rows.push(None);
                        }
                        sorted_blocks_iter.next();
                    } else {
                        break;
                    }
                }

                let soft_wrapped = wraps_snapshot.to_tab_point(WrapPoint::new(row, 0)).column() > 0;
                expected_buffer_rows.push(if soft_wrapped { None } else { buffer_row });
                expected_text.push_str(input_line);

                while let Some((block_row, disposition, id, height)) = sorted_blocks_iter.peek() {
                    if *block_row == row && *disposition == BlockDisposition::Below {
                        expected_block_positions
                            .push((expected_text.matches('\n').count() as u32 + 1, *id));
                        let text = "\n".repeat(*height as usize);
                        expected_text.push_str(&text);
                        for _ in 0..*height {
                            expected_buffer_rows.push(None);
                        }
                        sorted_blocks_iter.next();
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
                    .chunks(start_row as u32..expected_row_count as u32, false)
                    .map(|chunk| chunk.text)
                    .collect::<String>();
                assert_eq!(
                    actual_text, expected_text,
                    "incorrect text starting from row {}",
                    start_row
                );
                assert_eq!(
                    blocks_snapshot
                        .buffer_rows(start_row as u32)
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[start_row..]
                );
            }

            assert_eq!(
                blocks_snapshot
                    .blocks_in_range(0..(expected_row_count as u32))
                    .map(|(row, block)| { (row, block.as_custom().map(|b| b.id)) })
                    .collect::<Vec<_>>(),
                expected_block_positions
            );

            let mut expected_longest_rows = Vec::new();
            let mut longest_line_len = -1_isize;
            for (row, line) in expected_lines.iter().enumerate() {
                let row = row as u32;

                assert_eq!(
                    blocks_snapshot.line_len(row),
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
    }

    impl TransformBlock {
        fn as_custom(&self) -> Option<&Block> {
            match self {
                TransformBlock::Custom(block) => Some(block),
                TransformBlock::ExcerptHeader { .. } => None,
            }
        }
    }

    impl BlockSnapshot {
        fn to_point(&self, point: BlockPoint, bias: Bias) -> Point {
            self.wrap_snapshot.to_point(self.to_wrap_point(point), bias)
        }
    }
}
