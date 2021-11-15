use super::wrap_map::{self, Edit as WrapEdit, Snapshot as WrapSnapshot, TextSummary, WrapPoint};
use buffer::{rope, Anchor, Bias, Edit, Point, Rope, ToOffset, ToPoint as _};
use gpui::{fonts::HighlightStyle, AppContext, ModelHandle};
use language::{Buffer, HighlightedChunk};
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
    position: P,
    text: T,
    runs: Vec<(usize, HighlightStyle)>,
    disposition: BlockDisposition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BlockDisposition {
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
    input: Point,
    output: Point,
}

pub struct HighlightedChunks<'a> {
    transforms: sum_tree::Cursor<'a, Transform, (BlockPoint, WrapPoint)>,
    input_chunks: wrap_map::HighlightedChunks<'a>,
    input_chunk: HighlightedChunk<'a>,
    block_chunks: Option<BlockChunks<'a>>,
    output_position: BlockPoint,
    max_output_row: u32,
}

struct BlockChunks<'a> {
    chunks: rope::Chunks<'a>,
    runs: iter::Peekable<slice::Iter<'a, (usize, HighlightStyle)>>,
    chunk: Option<&'a str>,
    run_start: usize,
    offset: usize,
}

impl BlockMap {
    pub fn new(buffer: ModelHandle<Buffer>, wrap_snapshot: WrapSnapshot) -> Self {
        Self {
            buffer,
            next_block_id: AtomicUsize::new(0),
            blocks: Vec::new(),
            transforms: Mutex::new(SumTree::from_item(
                Transform::isomorphic(wrap_snapshot.text_summary().lines),
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
        self.apply_edits(&wrap_snapshot, edits, cx);
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
        self.apply_edits(&wrap_snapshot, edits, cx);
        *self.wrap_snapshot.lock() = wrap_snapshot;
        BlockMapWriter(self)
    }

    fn apply_edits(&self, wrap_snapshot: &WrapSnapshot, edits: Vec<WrapEdit>, cx: &AppContext) {
        let buffer = self.buffer.read(cx);
        let mut transforms = self.transforms.lock();
        let mut new_transforms = SumTree::new();
        let mut cursor = transforms.cursor::<WrapPoint>();
        let mut last_block_ix = 0;
        let mut blocks_in_edit = Vec::new();

        for edit in edits {
            let old_start = WrapPoint::new(edit.old.start, 0);
            if old_start > *cursor.start() {
                new_transforms.push_tree(cursor.slice(&old_start, Bias::Left, &()), &());
            }

            let overshoot = old_start.0 - cursor.start().0;
            if !overshoot.is_zero() {
                new_transforms.push(Transform::isomorphic(overshoot), &());
            }

            let old_end = WrapPoint::new(edit.old.end, 0);
            cursor.seek(&old_end, Bias::Left, &());
            cursor.next(&());

            let start_anchor = buffer.anchor_before(Point::new(edit.new.start, 0));
            let start_block_ix = match self.blocks[last_block_ix..].binary_search_by(|probe| {
                probe
                    .position
                    .cmp(&start_anchor, buffer)
                    .unwrap()
                    .then(Ordering::Greater)
            }) {
                Ok(ix) | Err(ix) => last_block_ix + ix,
            };
            let end_block_ix = if edit.new.end > wrap_snapshot.max_point().row() {
                self.blocks.len()
            } else {
                let end_anchor = buffer.anchor_before(Point::new(edit.new.end, 0));
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
                    .map(|block| (block.position.to_point(buffer).row, block)),
            );
            blocks_in_edit.sort_unstable_by_key(|(row, block)| (*row, block.disposition, block.id));

            for (block_row, block) in blocks_in_edit.iter().copied() {
                let new_transforms_end = new_transforms.summary().input;
                if block.disposition.is_above() {
                    if block_row > new_transforms_end.row {
                        new_transforms.push(
                            Transform::isomorphic(Point::new(block_row, 0) - new_transforms_end),
                            &(),
                        );
                    }
                } else {
                    if block_row >= new_transforms_end.row {
                        new_transforms.push(
                            Transform::isomorphic(
                                Point::new(block_row, wrap_snapshot.line_len(block_row))
                                    - new_transforms_end,
                            ),
                            &(),
                        );
                    }
                }

                new_transforms.push(Transform::block(block.clone()), &());
            }
        }

        new_transforms.push_tree(cursor.suffix(&()), &());

        drop(cursor);
        *transforms = new_transforms;
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

        for block in blocks {
            let id = BlockId(self.0.next_block_id.fetch_add(1, SeqCst));
            ids.push(id);

            let position = buffer.anchor_before(block.position);
            let row = position.to_point(buffer).row;

            let block_ix = match self
                .0
                .blocks
                .binary_search_by(|probe| probe.position.cmp(&position, buffer).unwrap())
            {
                Ok(ix) | Err(ix) => ix,
            };
            let mut text = block.text.into();
            if block.disposition.is_above() {
                text.push("\n");
            } else {
                text.push_front("\n");
            }

            self.0.blocks.insert(
                block_ix,
                Arc::new(Block {
                    id,
                    position,
                    text,
                    runs: block.runs,
                    disposition: block.disposition,
                }),
            );

            if let Err(edit_ix) = edits.binary_search_by_key(&row, |edit| edit.old.start) {
                edits.insert(
                    edit_ix,
                    Edit {
                        old: row..(row + 1),
                        new: row..(row + 1),
                    },
                );
            }
        }

        self.0.apply_edits(&*self.0.wrap_snapshot.lock(), edits, cx);
        ids
    }

    pub fn remove(&mut self, _: HashSet<BlockId>, _: &AppContext) {
        todo!()
    }
}

impl BlockSnapshot {
    #[cfg(test)]
    fn text(&mut self) -> String {
        self.highlighted_chunks_for_rows(0..self.max_point().0.row + 1)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn highlighted_chunks_for_rows(&mut self, rows: Range<u32>) -> HighlightedChunks {
        let mut cursor = self.transforms.cursor::<(BlockPoint, WrapPoint)>();
        let output_position = BlockPoint::new(rows.start, 0);
        cursor.seek(&output_position, Bias::Right, &());
        let (input_start, output_start) = cursor.start();
        let row_overshoot = rows.start - output_start.0.row;
        let input_start_row = input_start.0.row + row_overshoot;
        let input_end_row = self.to_wrap_point(BlockPoint::new(rows.end, 0)).row();
        let input_chunks = self
            .wrap_snapshot
            .highlighted_chunks_for_rows(input_start_row..input_end_row);
        HighlightedChunks {
            input_chunks,
            input_chunk: Default::default(),
            block_chunks: None,
            transforms: cursor,
            output_position,
            max_output_row: rows.end,
        }
    }

    pub fn max_point(&self) -> BlockPoint {
        BlockPoint(self.transforms.summary().output)
    }

    pub fn clip_point(&self, point: BlockPoint, bias: Bias) -> BlockPoint {
        let mut cursor = self.transforms.cursor::<(BlockPoint, WrapPoint)>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            if transform.is_isomorphic() {
                let (output_start, input_start) = cursor.start();
                let output_overshoot = point.0 - output_start.0;
                let input_point = self
                    .wrap_snapshot
                    .clip_point(WrapPoint(input_start.0 + output_overshoot), bias);
                let input_overshoot = input_point.0 - input_start.0;
                BlockPoint(output_start.0 + input_overshoot)
            } else {
                if bias == Bias::Left && cursor.start().1 .0 > Point::zero()
                    || cursor.end(&()).1 == self.wrap_snapshot.max_point()
                {
                    loop {
                        cursor.prev(&());
                        let transform = cursor.item().unwrap();
                        if transform.is_isomorphic() {
                            return BlockPoint(cursor.end(&()).0 .0);
                        }
                    }
                } else {
                    loop {
                        cursor.next(&());
                        let transform = cursor.item().unwrap();
                        if transform.is_isomorphic() {
                            return BlockPoint(cursor.start().0 .0);
                        }
                    }
                }
            }
        } else {
            self.max_point()
        }
    }

    pub fn to_block_point(&self, wrap_point: WrapPoint) -> BlockPoint {
        let mut cursor = self.transforms.cursor::<(WrapPoint, BlockPoint)>();
        cursor.seek(&wrap_point, Bias::Right, &());
        while let Some(item) = cursor.item() {
            if item.is_isomorphic() {
                break;
            }
            cursor.next(&());
        }
        let (input_start, output_start) = cursor.start();
        let input_overshoot = wrap_point.0 - input_start.0;
        BlockPoint(output_start.0 + input_overshoot)
    }

    pub fn to_wrap_point(&self, block_point: BlockPoint) -> WrapPoint {
        let mut cursor = self.transforms.cursor::<(BlockPoint, WrapPoint)>();
        cursor.seek(&block_point, Bias::Right, &());
        let (output_start, input_start) = cursor.start();
        let output_overshoot = block_point.0 - output_start.0;
        WrapPoint(input_start.0 + output_overshoot)
    }
}

impl Transform {
    fn isomorphic(lines: Point) -> Self {
        Self {
            summary: TransformSummary {
                input: lines,
                output: lines,
            },
            block: None,
        }
    }

    fn block(block: Arc<Block>) -> Self {
        Self {
            summary: TransformSummary {
                input: Default::default(),
                output: block.text.summary().lines,
            },
            block: Some(block),
        }
    }

    fn is_isomorphic(&self) -> bool {
        self.block.is_none()
    }

    fn block_disposition(&self) -> Option<BlockDisposition> {
        self.block.as_ref().map(|b| b.disposition)
    }
}

impl<'a> Iterator for HighlightedChunks<'a> {
    type Item = HighlightedChunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_position.row >= self.max_output_row {
            return None;
        }

        if let Some(block_chunks) = self.block_chunks.as_mut() {
            if let Some(block_chunk) = block_chunks.next() {
                self.output_position.0 += Point::from_str(block_chunk.text);
                return Some(block_chunk);
            } else {
                self.block_chunks.take();
            }
        }

        let transform = self.transforms.item()?;
        if let Some(block) = transform.block.as_ref() {
            let block_start = self.transforms.start().0 .0;
            let block_end = self.transforms.end(&()).0 .0;
            let start_in_block = self.output_position.0 - block_start;
            let end_in_block =
                cmp::min(Point::new(self.max_output_row, 0), block_end) - block_start;
            self.transforms.next(&());
            let mut block_chunks = BlockChunks::new(block, start_in_block..end_in_block);
            if let Some(block_chunk) = block_chunks.next() {
                self.output_position.0 += Point::from_str(block_chunk.text);
                return Some(block_chunk);
            }
        }

        if self.input_chunk.text.is_empty() {
            if let Some(input_chunk) = self.input_chunks.next() {
                self.input_chunk = input_chunk;
            }
        }

        let transform_end = self.transforms.end(&()).0 .0;
        let (prefix_lines, prefix_bytes) = offset_for_point(
            self.input_chunk.text,
            transform_end - self.output_position.0,
        );
        self.output_position.0 += prefix_lines;
        let (prefix, suffix) = self.input_chunk.text.split_at(prefix_bytes);
        self.input_chunk.text = suffix;
        if self.output_position.0 == transform_end {
            self.transforms.next(&());
        }

        Some(HighlightedChunk {
            text: prefix,
            ..self.input_chunk
        })
    }
}

impl<'a> BlockChunks<'a> {
    fn new(block: &'a Block, point_range: Range<Point>) -> Self {
        let offset_range = block.text.point_to_offset(point_range.start)
            ..block.text.point_to_offset(point_range.end);

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
    type Item = HighlightedChunk<'a>;

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

        Some(HighlightedChunk {
            text: chunk,
            highlight_id: Default::default(),
            diagnostic: None,
        })
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
        self.input += summary.input;
        self.output += summary.output;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for WrapPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.input;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for BlockPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output;
    }
}

impl BlockDisposition {
    fn is_above(&self) -> bool {
        matches!(self, BlockDisposition::Above)
    }
}

// Count the number of bytes prior to a target point. If the string doesn't contain the target
// point, return its total extent. Otherwise return the target point itself.
fn offset_for_point(s: &str, target: Point) -> (Point, usize) {
    let mut point = Point::zero();
    let mut offset = 0;
    for (row, line) in s.split('\n').enumerate().take(target.row as usize + 1) {
        let row = row as u32;
        if row > 0 {
            offset += 1;
        }
        point.row = row;
        point.column = if row == target.row {
            cmp::min(line.len() as u32, target.column)
        } else {
            line.len() as u32
        };
        offset += point.column as usize;
    }
    (point, offset)
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
        assert_eq!(
            snapshot.to_block_point(WrapPoint::new(1, 0)),
            BlockPoint::new(3, 0)
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

    #[gpui::test(iterations = 100)]
    fn test_random_blocks(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let wrap_width = None;
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
                // 0..=19 => {
                //     let wrap_width = if rng.gen_bool(0.2) {
                //         None
                //     } else {
                //         Some(rng.gen_range(0.0..=1000.0))
                //     };
                //     log::info!("Setting wrap width to {:?}", wrap_width);
                //     wrap_map.update(cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                // }
                20..=39 => {
                    let block_count = rng.gen_range(1..=1);
                    let block_properties = (0..block_count)
                        .map(|_| {
                            let buffer = buffer.read(cx);
                            let position = buffer.anchor_before(rng.gen_range(0..=buffer.len()));

                            let len = rng.gen_range(0..10);
                            let text = Rope::from(
                                RandomCharIter::new(&mut rng)
                                    .take(len)
                                    .collect::<String>()
                                    .as_str(),
                            );
                            let disposition = if rng.gen() {
                                BlockDisposition::Above
                            } else {
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
                // 40..=59 if !expected_blocks.is_empty() => {
                //     let block_count = rng.gen_range(1..=4.min(expected_blocks.len()));
                //     let block_ids_to_remove = (0..block_count)
                //         .map(|_| {
                //             expected_blocks
                //                 .remove(rng.gen_range(0..expected_blocks.len()))
                //                 .0
                //         })
                //         .collect();

                //     let (folds_snapshot, fold_edits) = fold_map.read(cx);
                //     let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
                //     let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                //         wrap_map.sync(tabs_snapshot, tab_edits, cx)
                //     });
                //     let mut block_map = block_map.write(wraps_snapshot, wrap_edits, cx);
                //     block_map.remove(block_ids_to_remove, cx);
                // }
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
                blocks_snapshot.transforms.summary().input,
                wraps_snapshot.max_point().0
            );
            log::info!("blocks text: {:?}", blocks_snapshot.text());

            let buffer = buffer.read(cx);
            let mut sorted_blocks = expected_blocks
                .iter()
                .cloned()
                .map(|(id, block)| {
                    (
                        id,
                        BlockProperties {
                            position: block.position.to_point(buffer),
                            text: block.text,
                            runs: block.runs,
                            disposition: block.disposition,
                        },
                    )
                })
                .collect::<Vec<_>>();
            sorted_blocks
                .sort_unstable_by_key(|(id, block)| (block.position.row, block.disposition, *id));
            let mut sorted_blocks = sorted_blocks.into_iter().peekable();

            let mut expected_text = String::new();
            let input_text = wraps_snapshot.text();
            for (row, input_line) in input_text.split('\n').enumerate() {
                let row = row as u32;
                if row > 0 {
                    expected_text.push('\n');
                }

                while let Some((_, block)) = sorted_blocks.peek() {
                    if block.position.row == row && block.disposition == BlockDisposition::Above {
                        expected_text.extend(block.text.chunks());
                        expected_text.push('\n');
                        sorted_blocks.next();
                    } else {
                        break;
                    }
                }

                expected_text.push_str(input_line);

                while let Some((_, block)) = sorted_blocks.peek() {
                    if block.position.row == row && block.disposition == BlockDisposition::Below {
                        expected_text.push('\n');
                        expected_text.extend(block.text.chunks());
                        sorted_blocks.next();
                    } else {
                        break;
                    }
                }
            }

            assert_eq!(blocks_snapshot.text(), expected_text);
        }
    }
}
