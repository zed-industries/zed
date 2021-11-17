use super::wrap_map::{self, Edit as WrapEdit, Snapshot as WrapSnapshot, WrapPoint};
use buffer::{rope, Anchor, Bias, Edit, Point, Rope, ToOffset, ToPoint as _};
use gpui::{fonts::HighlightStyle, AppContext, ModelHandle};
use language::{Buffer, Chunk};
use parking_lot::Mutex;
use std::{
    cmp::{self, Ordering},
    collections::HashSet,
    iter,
    ops::Range,
    slice,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use sum_tree::SumTree;

pub struct BlockMap {
    buffer: ModelHandle<Buffer>,
    next_block_id: AtomicUsize,
    wrap_snapshot: Mutex<WrapSnapshot>,
    blocks: Vec<Arc<Block>>,
    transforms: Mutex<SumTree<Transform>>,
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

#[derive(Debug)]
struct Block {
    id: BlockId,
    position: Anchor,
    text: Rope,
    runs: Vec<(usize, HighlightStyle)>,
    disposition: BlockDisposition,
}

#[derive(Clone)]
pub struct BlockProperties<P, T>
where
    P: Clone,
    T: Clone,
{
    pub position: P,
    pub text: T,
    pub runs: Vec<(usize, HighlightStyle)>,
    pub disposition: BlockDisposition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockDisposition {
    Above,
    Below,
}

#[derive(Clone, Debug)]
struct Transform {
    summary: TransformSummary,
    block: Option<Arc<Block>>,
}

#[derive(Clone, Debug, Default)]
struct TransformSummary {
    input_rows: u32,
    output_rows: u32,
}

pub struct Chunks<'a> {
    transforms: sum_tree::Cursor<'a, Transform, (BlockRow, WrapRow)>,
    input_chunks: wrap_map::Chunks<'a>,
    input_chunk: Chunk<'a>,
    block_chunks: Option<BlockChunks<'a>>,
    output_row: u32,
    max_output_row: u32,
    max_input_row: u32,
}

struct BlockChunks<'a> {
    chunks: rope::Chunks<'a>,
    runs: iter::Peekable<slice::Iter<'a, (usize, HighlightStyle)>>,
    chunk: Option<&'a str>,
    run_start: usize,
    offset: usize,
}

pub struct BufferRows<'a> {
    transforms: sum_tree::Cursor<'a, Transform, (BlockRow, WrapRow)>,
    input_buffer_rows: wrap_map::BufferRows<'a>,
    output_row: u32,
    started: bool,
}

impl BlockMap {
    pub fn new(buffer: ModelHandle<Buffer>, wrap_snapshot: WrapSnapshot) -> Self {
        Self {
            buffer,
            next_block_id: AtomicUsize::new(0),
            blocks: Vec::new(),
            transforms: Mutex::new(SumTree::from_item(
                Transform::isomorphic(wrap_snapshot.text_summary().lines.row + 1),
                &(),
            )),
            wrap_snapshot: Mutex::new(wrap_snapshot),
        }
    }

    pub fn read(
        &self,
        wrap_snapshot: WrapSnapshot,
        edits: Vec<WrapEdit>,
        cx: &AppContext,
    ) -> BlockSnapshot {
        self.sync(&wrap_snapshot, edits, cx);
        *self.wrap_snapshot.lock() = wrap_snapshot.clone();
        BlockSnapshot {
            wrap_snapshot,
            transforms: self.transforms.lock().clone(),
        }
    }

    pub fn write(
        &mut self,
        wrap_snapshot: WrapSnapshot,
        edits: Vec<WrapEdit>,
        cx: &AppContext,
    ) -> BlockMapWriter {
        self.sync(&wrap_snapshot, edits, cx);
        *self.wrap_snapshot.lock() = wrap_snapshot;
        BlockMapWriter(self)
    }

    pub fn sync(&self, wrap_snapshot: &WrapSnapshot, edits: Vec<WrapEdit>, cx: &AppContext) {
        if edits.is_empty() {
            return;
        }

        let buffer = self.buffer.read(cx);
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
                            .map_or(false, |b| b.disposition.is_below())
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
                        .map_or(false, |b| b.disposition.is_below())
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
                                .map_or(false, |b| b.disposition.is_below())
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
            let new_start = wrap_snapshot.to_point(WrapPoint::new(new_start.0, 0), Bias::Left);
            let start_anchor = buffer.anchor_before(new_start);
            let start_block_ix = match self.blocks[last_block_ix..].binary_search_by(|probe| {
                probe
                    .position
                    .cmp(&start_anchor, buffer)
                    .unwrap()
                    .then(Ordering::Greater)
            }) {
                Ok(ix) | Err(ix) => last_block_ix + ix,
            };
            let end_block_ix = if new_end.0 > wrap_snapshot.max_point().row() {
                self.blocks.len()
            } else {
                let new_end = wrap_snapshot.to_point(WrapPoint::new(new_end.0, 0), Bias::Left);
                let end_anchor = buffer.anchor_before(new_end);
                match self.blocks[start_block_ix..].binary_search_by(|probe| {
                    probe
                        .position
                        .cmp(&end_anchor, buffer)
                        .unwrap()
                        .then(Ordering::Greater)
                }) {
                    Ok(ix) | Err(ix) => start_block_ix + ix,
                }
            };
            last_block_ix = end_block_ix;
            blocks_in_edit.clear();
            blocks_in_edit.extend(
                self.blocks[start_block_ix..end_block_ix]
                    .iter()
                    .map(|block| {
                        let mut position = block.position.to_point(buffer);
                        match block.disposition {
                            BlockDisposition::Above => position.column = 0,
                            BlockDisposition::Below => {
                                position.column = buffer.line_len(position.row)
                            }
                        }
                        let position = wrap_snapshot.from_point(position, Bias::Left);
                        (position.row(), block)
                    }),
            );
            blocks_in_edit.sort_unstable_by_key(|(row, block)| (*row, block.disposition, block.id));

            // For each of these blocks, insert a new isomorphic transform preceding the block,
            // and then insert the block itself.
            for (block_row, block) in blocks_in_edit.iter().copied() {
                let insertion_row = match block.disposition {
                    BlockDisposition::Above => block_row,
                    BlockDisposition::Below => block_row + 1,
                };
                let extent_before_block = insertion_row - new_transforms.summary().input_rows;
                push_isomorphic(&mut new_transforms, extent_before_block);
                new_transforms.push(Transform::block(block.clone()), &());
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
    fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }
}

impl std::ops::Deref for BlockPoint {
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
    pub fn insert<P, T>(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<P, T>>,
        cx: &AppContext,
    ) -> Vec<BlockId>
    where
        P: ToOffset + Clone,
        T: Into<Rope> + Clone,
    {
        let buffer = self.0.buffer.read(cx);
        let mut ids = Vec::new();
        let mut edits = Vec::<Edit<u32>>::new();
        let wrap_snapshot = &*self.0.wrap_snapshot.lock();

        for block in blocks {
            let id = BlockId(self.0.next_block_id.fetch_add(1, SeqCst));
            ids.push(id);

            let position = buffer.anchor_before(block.position);
            let point = position.to_point(buffer);
            let start_row = wrap_snapshot
                .from_point(Point::new(point.row, 0), Bias::Left)
                .row();
            let end_row = if point.row == buffer.max_point().row {
                wrap_snapshot.max_point().row() + 1
            } else {
                wrap_snapshot
                    .from_point(Point::new(point.row + 1, 0), Bias::Left)
                    .row()
            };

            let block_ix = match self
                .0
                .blocks
                .binary_search_by(|probe| probe.position.cmp(&position, buffer).unwrap())
            {
                Ok(ix) | Err(ix) => ix,
            };
            self.0.blocks.insert(
                block_ix,
                Arc::new(Block {
                    id,
                    position,
                    text: block.text.into(),
                    runs: block.runs,
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

        self.0.sync(wrap_snapshot, edits, cx);
        ids
    }

    pub fn remove(&mut self, block_ids: HashSet<BlockId>, cx: &AppContext) {
        let buffer = self.0.buffer.read(cx);
        let wrap_snapshot = &*self.0.wrap_snapshot.lock();
        let mut edits = Vec::new();
        let mut last_block_buffer_row = None;
        self.0.blocks.retain(|block| {
            if block_ids.contains(&block.id) {
                let buffer_row = block.position.to_point(buffer).row;
                if last_block_buffer_row != Some(buffer_row) {
                    last_block_buffer_row = Some(buffer_row);
                    let start_row = wrap_snapshot
                        .from_point(Point::new(buffer_row, 0), Bias::Left)
                        .row();
                    let end_row = wrap_snapshot
                        .from_point(
                            Point::new(buffer_row, buffer.line_len(buffer_row)),
                            Bias::Left,
                        )
                        .row()
                        + 1;
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
        self.0.sync(wrap_snapshot, edits, cx);
    }
}

impl BlockSnapshot {
    #[cfg(test)]
    fn text(&mut self) -> String {
        self.chunks(0..self.transforms.summary().output_rows, false)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn chunks(&self, rows: Range<u32>, highlights: bool) -> Chunks {
        let max_input_row = self.transforms.summary().input_rows;
        let max_output_row = cmp::min(rows.end, self.transforms.summary().output_rows);
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        let output_row = rows.start;
        cursor.seek(&BlockRow(output_row), Bias::Right, &());
        let (input_start, output_start) = cursor.start();
        let overshoot = rows.start - output_start.0;
        let output_end_row = max_output_row.saturating_sub(1);
        let input_start_row = input_start.0 + overshoot;
        let input_end_row = self
            .to_wrap_point(BlockPoint::new(
                output_end_row,
                self.wrap_snapshot.line_len(output_end_row),
            ))
            .row()
            + 1;
        let input_chunks = self
            .wrap_snapshot
            .chunks(input_start_row..input_end_row, highlights);
        Chunks {
            input_chunks,
            input_chunk: Default::default(),
            block_chunks: None,
            transforms: cursor,
            output_row,
            max_output_row,
            max_input_row,
        }
    }

    pub fn buffer_rows(&self, start_row: u32) -> BufferRows {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(start_row), Bias::Right, &());
        let (input_start, output_start) = cursor.start();
        let overshoot = if cursor.item().map_or(false, |t| t.is_isomorphic()) {
            start_row - output_start.0
        } else {
            0
        };
        let input_start_row = input_start.0 + overshoot;
        BufferRows {
            transforms: cursor,
            input_buffer_rows: self.wrap_snapshot.buffer_rows(input_start_row),
            output_row: start_row,
            started: false,
        }
    }

    pub fn max_point(&self) -> BlockPoint {
        todo!()
    }

    pub fn clip_point(&self, point: BlockPoint, bias: Bias) -> BlockPoint {
        todo!()
        // let mut cursor = self.transforms.cursor::<(BlockPoint, WrapPoint)>();
        // cursor.seek(&point, Bias::Right, &());
        // if let Some(transform) = cursor.prev_item() {
        //     if transform.is_isomorphic() && point == cursor.start().0 {
        //         return point;
        //     }
        // }
        // if let Some(transform) = cursor.item() {
        //     if transform.is_isomorphic() {
        //         let (output_start, input_start) = cursor.start();
        //         let output_overshoot = point.0 - output_start.0;
        //         let input_point = self
        //             .wrap_snapshot
        //             .clip_point(WrapPoint(input_start.0 + output_overshoot), bias);
        //         let input_overshoot = input_point.0 - input_start.0;
        //         BlockPoint(output_start.0 + input_overshoot)
        //     } else {
        //         if bias == Bias::Left && cursor.start().1 .0 > Point::zero()
        //             || cursor.end(&()).1 == self.wrap_snapshot.max_point()
        //         {
        //             loop {
        //                 cursor.prev(&());
        //                 let transform = cursor.item().unwrap();
        //                 if transform.is_isomorphic() {
        //                     return BlockPoint(cursor.end(&()).0 .0);
        //                 }
        //             }
        //         } else {
        //             loop {
        //                 cursor.next(&());
        //                 let transform = cursor.item().unwrap();
        //                 if transform.is_isomorphic() {
        //                     return BlockPoint(cursor.start().0 .0);
        //                 }
        //             }
        //         }
        //     }
        // } else {
        //     self.max_point()
        // }
    }

    pub fn to_block_point(&self, wrap_point: WrapPoint, bias: Bias) -> BlockPoint {
        todo!()
        // let mut cursor = self.transforms.cursor::<(WrapPoint, BlockPoint)>();
        // cursor.seek(&wrap_point, bias, &());
        // while let Some(item) = cursor.item() {
        //     if item.is_isomorphic() {
        //         break;
        //     }
        //     cursor.next(&());
        // }
        // let (input_start, output_start) = cursor.start();
        // let input_overshoot = wrap_point.0 - input_start.0;
        // BlockPoint(output_start.0 + input_overshoot)
    }

    pub fn to_wrap_point(&self, block_point: BlockPoint) -> WrapPoint {
        let mut cursor = self.transforms.cursor::<(BlockRow, WrapRow)>();
        cursor.seek(&BlockRow(block_point.row), Bias::Right, &());
        if let Some(transform) = cursor.item() {
            match transform.block.as_ref().map(|b| b.disposition) {
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

    fn block(block: Arc<Block>) -> Self {
        Self {
            summary: TransformSummary {
                input_rows: 0,
                output_rows: block.text.summary().lines.row + 1,
            },
            block: Some(block),
        }
    }

    fn is_isomorphic(&self) -> bool {
        self.block.is_none()
    }
}

impl<'a> Iterator for Chunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_row >= self.max_output_row {
            return None;
        }

        if let Some(block_chunks) = self.block_chunks.as_mut() {
            if let Some(block_chunk) = block_chunks.next() {
                self.output_row += block_chunk.text.matches('\n').count() as u32;
                return Some(block_chunk);
            } else {
                self.block_chunks.take();
                self.output_row += 1;
                if self.output_row < self.max_output_row {
                    return Some(Chunk {
                        text: "\n",
                        ..Default::default()
                    });
                } else {
                    return None;
                }
            }
        }

        let transform = self.transforms.item()?;
        if let Some(block) = transform.block.as_ref() {
            let block_start = self.transforms.start().0 .0;
            let block_end = self.transforms.end(&()).0 .0;
            let start_in_block = self.output_row - block_start;
            let end_in_block = cmp::min(self.max_output_row, block_end) - block_start;
            self.transforms.next(&());
            self.block_chunks = Some(BlockChunks::new(block, start_in_block..end_in_block));
            return self.next();
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

impl<'a> BlockChunks<'a> {
    fn new(block: &'a Block, rows: Range<u32>) -> Self {
        let offset_range = block.text.point_to_offset(Point::new(rows.start, 0))
            ..block.text.point_to_offset(Point::new(rows.end, 0));

        let mut runs = block.runs.iter().peekable();
        let mut run_start = 0;
        while let Some((run_len, _)) = runs.peek() {
            let run_end = run_start + run_len;
            if run_end <= offset_range.start {
                run_start = run_end;
                runs.next();
            } else {
                break;
            }
        }

        Self {
            chunk: None,
            run_start,
            chunks: block.text.chunks_in_range(offset_range.clone()),
            runs,
            offset: offset_range.start,
        }
    }
}

impl<'a> Iterator for BlockChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.is_none() {
            self.chunk = self.chunks.next();
        }

        let chunk = self.chunk?;
        let mut chunk_len = chunk.len();
        // let mut highlight_style = None;
        if let Some((run_len, _)) = self.runs.peek() {
            // highlight_style = Some(style.clone());
            let run_end_in_chunk = self.run_start + run_len - self.offset;
            if run_end_in_chunk <= chunk_len {
                chunk_len = run_end_in_chunk;
                self.run_start += run_len;
                self.runs.next();
            }
        }

        self.offset += chunk_len;
        let (chunk, suffix) = chunk.split_at(chunk_len);
        self.chunk = if suffix.is_empty() {
            None
        } else {
            Some(suffix)
        };

        Some(Chunk {
            text: chunk,
            highlight_id: Default::default(),
            diagnostic: None,
        })
    }
}

impl<'a> Iterator for BufferRows<'a> {
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
        if transform.is_isomorphic() {
            Some(self.input_buffer_rows.next().unwrap())
        } else {
            Some(None)
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
    fn is_above(&self) -> bool {
        matches!(self, BlockDisposition::Above)
    }

    fn is_below(&self) -> bool {
        matches!(self, BlockDisposition::Below)
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
    use buffer::RandomCharIter;
    use language::Buffer;
    use rand::prelude::*;
    use std::env;

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

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let (fold_map, folds_snapshot) = FoldMap::new(buffer.clone(), cx);
        let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), 1);
        let (wrap_map, wraps_snapshot) = WrapMap::new(tabs_snapshot, font_id, 14.0, None, cx);
        let mut block_map = BlockMap::new(buffer.clone(), wraps_snapshot.clone());

        let mut writer = block_map.write(wraps_snapshot.clone(), vec![], cx);
        writer.insert(
            vec![
                BlockProperties {
                    position: Point::new(1, 0),
                    text: "BLOCK 1",
                    disposition: BlockDisposition::Above,
                    runs: vec![],
                },
                BlockProperties {
                    position: Point::new(1, 2),
                    text: "BLOCK 2",
                    disposition: BlockDisposition::Above,
                    runs: vec![],
                },
                BlockProperties {
                    position: Point::new(3, 2),
                    text: "BLOCK 3",
                    disposition: BlockDisposition::Below,
                    runs: vec![],
                },
            ],
            cx,
        );

        let mut snapshot = block_map.read(wraps_snapshot, vec![], cx);
        assert_eq!(
            snapshot.text(),
            "aaa\nBLOCK 1\nBLOCK 2\nbbb\nccc\nddd\nBLOCK 3"
        );
        // assert_eq!(
        //     snapshot.to_block_point(WrapPoint::new(1, 0), Bias::Right),
        //     BlockPoint::new(3, 0)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(1, 0), Bias::Left),
        //     BlockPoint::new(1, 0)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(1, 0), Bias::Right),
        //     BlockPoint::new(1, 0)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(1, 1), Bias::Left),
        //     BlockPoint::new(1, 0)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(1, 1), Bias::Right),
        //     BlockPoint::new(3, 0)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(3, 0), Bias::Left),
        //     BlockPoint::new(3, 0)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(3, 0), Bias::Right),
        //     BlockPoint::new(3, 0)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(5, 3), Bias::Left),
        //     BlockPoint::new(5, 3)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(5, 3), Bias::Right),
        //     BlockPoint::new(5, 3)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(6, 0), Bias::Left),
        //     BlockPoint::new(5, 3)
        // );
        // assert_eq!(
        //     snapshot.clip_point(BlockPoint::new(6, 0), Bias::Right),
        //     BlockPoint::new(5, 3)
        // );

        assert_eq!(
            snapshot.buffer_rows(0).collect::<Vec<_>>(),
            &[Some(0), None, None, Some(1), Some(2), Some(3), None]
        );

        // Insert a line break, separating two block decorations into separate
        // lines.
        buffer.update(cx, |buffer, cx| {
            buffer.edit([Point::new(1, 1)..Point::new(1, 1)], "!!!\n", cx)
        });

        let (folds_snapshot, fold_edits) = fold_map.read(cx);
        let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
        let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
            wrap_map.sync(tabs_snapshot, tab_edits, cx)
        });
        let mut snapshot = block_map.read(wraps_snapshot, wrap_edits, cx);
        assert_eq!(
            snapshot.text(),
            "aaa\nBLOCK 1\nb!!!\nBLOCK 2\nbb\nccc\nddd\nBLOCK 3"
        );
    }

    #[gpui::test]
    fn test_blocks_on_wrapped_lines(cx: &mut gpui::MutableAppContext) {
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();

        let text = "one two three\nfour five six\nseven eight";

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let (_, folds_snapshot) = FoldMap::new(buffer.clone(), cx);
        let (_, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), 1);
        let (_, wraps_snapshot) = WrapMap::new(tabs_snapshot, font_id, 14.0, Some(60.), cx);
        let mut block_map = BlockMap::new(buffer.clone(), wraps_snapshot.clone());

        let mut writer = block_map.write(wraps_snapshot.clone(), vec![], cx);
        writer.insert(
            vec![
                BlockProperties {
                    position: Point::new(1, 12),
                    text: "<BLOCK 1",
                    disposition: BlockDisposition::Above,
                    runs: vec![],
                },
                BlockProperties {
                    position: Point::new(1, 1),
                    text: ">BLOCK 2",
                    disposition: BlockDisposition::Below,
                    runs: vec![],
                },
            ],
            cx,
        );

        // Blocks with an 'above' disposition go above their corresponding buffer line.
        // Blocks with a 'below' disposition go below their corresponding buffer line.
        let mut snapshot = block_map.read(wraps_snapshot, vec![], cx);
        assert_eq!(
            snapshot.text(),
            "one two \nthree\n<BLOCK 1\nfour five \nsix\n>BLOCK 2\nseven \neight"
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

        log::info!("Wrap width: {:?}", wrap_width);

        let buffer = cx.add_model(|cx| {
            let len = rng.gen_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            log::info!("initial buffer text: {:?}", text);
            Buffer::new(0, text, cx)
        });
        let (fold_map, folds_snapshot) = FoldMap::new(buffer.clone(), cx);
        let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), tab_size);
        let (wrap_map, wraps_snapshot) =
            WrapMap::new(tabs_snapshot, font_id, font_size, wrap_width, cx);
        let mut block_map = BlockMap::new(buffer.clone(), wraps_snapshot);
        let mut expected_blocks = Vec::new();

        for _ in 0..operations {
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
                    let block_count = rng.gen_range(1..=1);
                    let block_properties = (0..block_count)
                        .map(|_| {
                            let buffer = buffer.read(cx);
                            let position = buffer.anchor_before(rng.gen_range(0..=buffer.len()));

                            let len = rng.gen_range(0..10);
                            let mut text = Rope::from(
                                RandomCharIter::new(&mut rng)
                                    .take(len)
                                    .collect::<String>()
                                    .to_uppercase()
                                    .as_str(),
                            );
                            let disposition = if rng.gen() {
                                text.push_front("<");
                                BlockDisposition::Above
                            } else {
                                text.push_front(">");
                                BlockDisposition::Below
                            };
                            log::info!(
                                "inserting block {:?} {:?} with text {:?}",
                                disposition,
                                position.to_point(buffer),
                                text.to_string()
                            );
                            BlockProperties {
                                position,
                                text,
                                runs: Vec::<(usize, HighlightStyle)>::new(),
                                disposition,
                            }
                        })
                        .collect::<Vec<_>>();

                    let (folds_snapshot, fold_edits) = fold_map.read(cx);
                    let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tabs_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits, cx);
                    let block_ids = block_map.insert(block_properties.clone(), cx);
                    for (block_id, props) in block_ids.into_iter().zip(block_properties) {
                        expected_blocks.push((block_id, props));
                    }
                }
                40..=59 if !expected_blocks.is_empty() => {
                    let block_count = rng.gen_range(1..=4.min(expected_blocks.len()));
                    let block_ids_to_remove = (0..block_count)
                        .map(|_| {
                            expected_blocks
                                .remove(rng.gen_range(0..expected_blocks.len()))
                                .0
                        })
                        .collect();

                    let (folds_snapshot, fold_edits) = fold_map.read(cx);
                    let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
                    let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                        wrap_map.sync(tabs_snapshot, tab_edits, cx)
                    });
                    let mut block_map = block_map.write(wraps_snapshot, wrap_edits, cx);
                    block_map.remove(block_ids_to_remove, cx);
                }
                _ => {
                    buffer.update(cx, |buffer, _| {
                        buffer.randomly_edit(&mut rng, 1);
                        log::info!("buffer text: {:?}", buffer.text());
                    });
                }
            }

            let (folds_snapshot, fold_edits) = fold_map.read(cx);
            let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
            let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                wrap_map.sync(tabs_snapshot, tab_edits, cx)
            });
            let mut blocks_snapshot = block_map.read(wraps_snapshot.clone(), wrap_edits, cx);
            assert_eq!(
                blocks_snapshot.transforms.summary().input_rows,
                wraps_snapshot.max_point().row() + 1
            );
            log::info!("blocks text: {:?}", blocks_snapshot.text());

            let buffer = buffer.read(cx);
            let mut sorted_blocks = expected_blocks
                .iter()
                .cloned()
                .map(|(id, block)| {
                    let mut position = block.position.to_point(buffer);
                    match block.disposition {
                        BlockDisposition::Above => {
                            position.column = 0;
                        }
                        BlockDisposition::Below => {
                            position.column = buffer.line_len(position.row);
                        }
                    };
                    let row = wraps_snapshot.from_point(position, Bias::Left).row();
                    (
                        id,
                        BlockProperties {
                            position: row,
                            text: block.text,
                            runs: block.runs,
                            disposition: block.disposition,
                        },
                    )
                })
                .collect::<Vec<_>>();
            sorted_blocks
                .sort_unstable_by_key(|(id, block)| (block.position, block.disposition, *id));
            let mut sorted_blocks = sorted_blocks.into_iter().peekable();

            let mut expected_buffer_rows = Vec::new();
            let mut expected_text = String::new();
            let input_text = wraps_snapshot.text();
            for (row, input_line) in input_text.split('\n').enumerate() {
                let row = row as u32;
                if row > 0 {
                    expected_text.push('\n');
                }

                let buffer_row = wraps_snapshot
                    .to_point(WrapPoint::new(row, 0), Bias::Left)
                    .row;

                while let Some((_, block)) = sorted_blocks.peek() {
                    if block.position == row && block.disposition == BlockDisposition::Above {
                        let text = block.text.to_string();
                        expected_text.push_str(&text);
                        expected_text.push('\n');
                        for _ in text.split('\n') {
                            expected_buffer_rows.push(None);
                        }
                        sorted_blocks.next();
                    } else {
                        break;
                    }
                }

                let soft_wrapped = wraps_snapshot.to_tab_point(WrapPoint::new(row, 0)).column() > 0;
                expected_buffer_rows.push(if soft_wrapped { None } else { Some(buffer_row) });
                expected_text.push_str(input_line);

                while let Some((_, block)) = sorted_blocks.peek() {
                    if block.position == row && block.disposition == BlockDisposition::Below {
                        let text = block.text.to_string();
                        expected_text.push('\n');
                        expected_text.push_str(&text);
                        for _ in text.split('\n') {
                            expected_buffer_rows.push(None);
                        }
                        sorted_blocks.next();
                    } else {
                        break;
                    }
                }
            }

            assert_eq!(blocks_snapshot.text(), expected_text);
            // for row in 0..=blocks_snapshot.wrap_snapshot.max_point().row() {
            //     let wrap_point = WrapPoint::new(row, 0);
            //     let block_point = blocks_snapshot.to_block_point(wrap_point, Bias::Right);
            //     assert_eq!(blocks_snapshot.to_wrap_point(block_point), wrap_point);
            // }
            assert_eq!(
                blocks_snapshot.buffer_rows(0).collect::<Vec<_>>(),
                expected_buffer_rows
            );
        }
    }
}
