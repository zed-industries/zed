mod block_map;
mod fold_map;
mod tab_map;
mod wrap_map;

use crate::{Anchor, MultiBuffer, MultiBufferSnapshot, ToOffset, ToPoint};
use block_map::{BlockMap, BlockPoint};
use collections::{HashMap, HashSet};
use fold_map::FoldMap;
use gpui::{
    fonts::{FontId, HighlightStyle},
    Entity, ModelContext, ModelHandle,
};
use language::{Point, Subscription as BufferSubscription};
use std::{any::TypeId, ops::Range, sync::Arc};
use sum_tree::{Bias, TreeMap};
use tab_map::TabMap;
use wrap_map::WrapMap;

pub use block_map::{
    BlockBufferRows as DisplayBufferRows, BlockChunks as DisplayChunks, BlockContext,
    BlockDisposition, BlockId, BlockProperties, RenderBlock, TransformBlock,
};

pub trait ToDisplayPoint {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint;
}

type TextHighlights = TreeMap<Option<TypeId>, Arc<(HighlightStyle, Vec<Range<Anchor>>)>>;

pub struct DisplayMap {
    buffer: ModelHandle<MultiBuffer>,
    buffer_subscription: BufferSubscription,
    fold_map: FoldMap,
    tab_map: TabMap,
    wrap_map: ModelHandle<WrapMap>,
    block_map: BlockMap,
    text_highlights: TextHighlights,
}

impl Entity for DisplayMap {
    type Event = ();
}

impl DisplayMap {
    pub fn new(
        buffer: ModelHandle<MultiBuffer>,
        tab_size: usize,
        font_id: FontId,
        font_size: f32,
        wrap_width: Option<f32>,
        buffer_header_height: u8,
        excerpt_header_height: u8,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let buffer_subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (fold_map, snapshot) = FoldMap::new(buffer.read(cx).snapshot(cx));
        let (tab_map, snapshot) = TabMap::new(snapshot, tab_size);
        let (wrap_map, snapshot) = WrapMap::new(snapshot, font_id, font_size, wrap_width, cx);
        let block_map = BlockMap::new(snapshot, buffer_header_height, excerpt_header_height);
        cx.observe(&wrap_map, |_, _, cx| cx.notify()).detach();
        DisplayMap {
            buffer,
            buffer_subscription,
            fold_map,
            tab_map,
            wrap_map,
            block_map,
            text_highlights: Default::default(),
        }
    }

    pub fn snapshot(&self, cx: &mut ModelContext<Self>) -> DisplaySnapshot {
        let buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let (folds_snapshot, edits) = self.fold_map.read(buffer_snapshot, edits);
        let (tabs_snapshot, edits) = self.tab_map.sync(folds_snapshot.clone(), edits);
        let (wraps_snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(tabs_snapshot.clone(), edits, cx));
        let blocks_snapshot = self.block_map.read(wraps_snapshot.clone(), edits);

        DisplaySnapshot {
            buffer_snapshot: self.buffer.read(cx).snapshot(cx),
            folds_snapshot,
            tabs_snapshot,
            wraps_snapshot,
            blocks_snapshot,
            text_highlights: self.text_highlights.clone(),
        }
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
        let (snapshot, edits) = fold_map.fold(ranges);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let (mut fold_map, snapshot, edits) = self.fold_map.write(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
        let (snapshot, edits) = fold_map.unfold(ranges);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        self.block_map.read(snapshot, edits);
    }

    pub fn insert_blocks(
        &mut self,
        blocks: impl IntoIterator<Item = BlockProperties<Anchor>>,
        cx: &mut ModelContext<Self>,
    ) -> Vec<BlockId> {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        let mut block_map = self.block_map.write(snapshot, edits);
        block_map.insert(blocks)
    }

    pub fn replace_blocks(&mut self, styles: HashMap<BlockId, RenderBlock>) {
        self.block_map.replace(styles);
    }

    pub fn remove_blocks(&mut self, ids: HashSet<BlockId>, cx: &mut ModelContext<Self>) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let edits = self.buffer_subscription.consume().into_inner();
        let (snapshot, edits) = self.fold_map.read(snapshot, edits);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        let (snapshot, edits) = self
            .wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        let mut block_map = self.block_map.write(snapshot, edits);
        block_map.remove(ids);
    }

    pub fn highlight_text(
        &mut self,
        type_id: TypeId,
        ranges: Vec<Range<Anchor>>,
        style: HighlightStyle,
    ) {
        self.text_highlights
            .insert(Some(type_id), Arc::new((style, ranges)));
    }

    pub fn clear_text_highlights(&mut self, type_id: TypeId) {
        self.text_highlights.remove(&Some(type_id));
    }

    pub fn set_font(&self, font_id: FontId, font_size: f32, cx: &mut ModelContext<Self>) {
        self.wrap_map
            .update(cx, |map, cx| map.set_font(font_id, font_size, cx));
    }

    pub fn set_wrap_width(&self, width: Option<f32>, cx: &mut ModelContext<Self>) -> bool {
        self.wrap_map
            .update(cx, |map, cx| map.set_wrap_width(width, cx))
    }

    #[cfg(test)]
    pub fn is_rewrapping(&self, cx: &gpui::AppContext) -> bool {
        self.wrap_map.read(cx).is_rewrapping()
    }
}

pub struct DisplaySnapshot {
    pub buffer_snapshot: MultiBufferSnapshot,
    folds_snapshot: fold_map::FoldSnapshot,
    tabs_snapshot: tab_map::TabSnapshot,
    wraps_snapshot: wrap_map::WrapSnapshot,
    blocks_snapshot: block_map::BlockSnapshot,
    text_highlights: TextHighlights,
}

impl DisplaySnapshot {
    #[cfg(test)]
    pub fn fold_count(&self) -> usize {
        self.folds_snapshot.fold_count()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer_snapshot.len() == 0
    }

    pub fn buffer_rows<'a>(&'a self, start_row: u32) -> DisplayBufferRows<'a> {
        self.blocks_snapshot.buffer_rows(start_row)
    }

    pub fn max_buffer_row(&self) -> u32 {
        self.buffer_snapshot.max_buffer_row()
    }

    pub fn prev_line_boundary(&self, mut point: Point) -> (Point, DisplayPoint) {
        loop {
            let mut fold_point = self.folds_snapshot.to_fold_point(point, Bias::Left);
            *fold_point.column_mut() = 0;
            point = fold_point.to_buffer_point(&self.folds_snapshot);

            let mut display_point = self.point_to_display_point(point, Bias::Left);
            *display_point.column_mut() = 0;
            let next_point = self.display_point_to_point(display_point, Bias::Left);
            if next_point == point {
                return (point, display_point);
            }
            point = next_point;
        }
    }

    pub fn next_line_boundary(&self, mut point: Point) -> (Point, DisplayPoint) {
        loop {
            let mut fold_point = self.folds_snapshot.to_fold_point(point, Bias::Right);
            *fold_point.column_mut() = self.folds_snapshot.line_len(fold_point.row());
            point = fold_point.to_buffer_point(&self.folds_snapshot);

            let mut display_point = self.point_to_display_point(point, Bias::Right);
            *display_point.column_mut() = self.line_len(display_point.row());
            let next_point = self.display_point_to_point(display_point, Bias::Right);
            if next_point == point {
                return (point, display_point);
            }
            point = next_point;
        }
    }

    fn point_to_display_point(&self, point: Point, bias: Bias) -> DisplayPoint {
        let fold_point = self.folds_snapshot.to_fold_point(point, bias);
        let tab_point = self.tabs_snapshot.to_tab_point(fold_point);
        let wrap_point = self.wraps_snapshot.from_tab_point(tab_point);
        let block_point = self.blocks_snapshot.to_block_point(wrap_point);
        DisplayPoint(block_point)
    }

    fn display_point_to_point(&self, point: DisplayPoint, bias: Bias) -> Point {
        let block_point = point.0;
        let wrap_point = self.blocks_snapshot.to_wrap_point(block_point);
        let tab_point = self.wraps_snapshot.to_tab_point(wrap_point);
        let fold_point = self.tabs_snapshot.to_fold_point(tab_point, bias).0;
        fold_point.to_buffer_point(&self.folds_snapshot)
    }

    pub fn max_point(&self) -> DisplayPoint {
        DisplayPoint(self.blocks_snapshot.max_point())
    }

    pub fn text_chunks(&self, display_row: u32) -> impl Iterator<Item = &str> {
        self.blocks_snapshot
            .chunks(display_row..self.max_point().row() + 1, false, None)
            .map(|h| h.text)
    }

    pub fn chunks<'a>(
        &'a self,
        display_rows: Range<u32>,
        language_aware: bool,
    ) -> DisplayChunks<'a> {
        self.blocks_snapshot
            .chunks(display_rows, language_aware, Some(&self.text_highlights))
    }

    pub fn chars_at<'a>(&'a self, point: DisplayPoint) -> impl Iterator<Item = char> + 'a {
        let mut column = 0;
        let mut chars = self.text_chunks(point.row()).flat_map(str::chars);
        while column < point.column() {
            if let Some(c) = chars.next() {
                column += c.len_utf8() as u32;
            } else {
                break;
            }
        }
        chars
    }

    pub fn column_to_chars(&self, display_row: u32, target: u32) -> u32 {
        let mut count = 0;
        let mut column = 0;
        for c in self.chars_at(DisplayPoint::new(display_row, 0)) {
            if column >= target {
                break;
            }
            count += 1;
            column += c.len_utf8() as u32;
        }
        count
    }

    pub fn column_from_chars(&self, display_row: u32, char_count: u32) -> u32 {
        let mut count = 0;
        let mut column = 0;
        for c in self.chars_at(DisplayPoint::new(display_row, 0)) {
            if c == '\n' || count >= char_count {
                break;
            }
            count += 1;
            column += c.len_utf8() as u32;
        }
        column
    }

    pub fn clip_point(&self, point: DisplayPoint, bias: Bias) -> DisplayPoint {
        DisplayPoint(self.blocks_snapshot.clip_point(point.0, bias))
    }

    pub fn folds_in_range<'a, T>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = &'a Range<Anchor>>
    where
        T: ToOffset,
    {
        self.folds_snapshot.folds_in_range(range)
    }

    pub fn blocks_in_range<'a>(
        &'a self,
        rows: Range<u32>,
    ) -> impl Iterator<Item = (u32, &'a TransformBlock)> {
        self.blocks_snapshot.blocks_in_range(rows)
    }

    pub fn intersects_fold<T: ToOffset>(&self, offset: T) -> bool {
        self.folds_snapshot.intersects_fold(offset)
    }

    pub fn is_line_folded(&self, display_row: u32) -> bool {
        let block_point = BlockPoint(Point::new(display_row, 0));
        let wrap_point = self.blocks_snapshot.to_wrap_point(block_point);
        let tab_point = self.wraps_snapshot.to_tab_point(wrap_point);
        self.folds_snapshot.is_line_folded(tab_point.row())
    }

    pub fn is_block_line(&self, display_row: u32) -> bool {
        self.blocks_snapshot.is_block_line(display_row)
    }

    pub fn soft_wrap_indent(&self, display_row: u32) -> Option<u32> {
        let wrap_row = self
            .blocks_snapshot
            .to_wrap_point(BlockPoint::new(display_row, 0))
            .row();
        self.wraps_snapshot.soft_wrap_indent(wrap_row)
    }

    pub fn text(&self) -> String {
        self.text_chunks(0).collect()
    }

    pub fn line(&self, display_row: u32) -> String {
        let mut result = String::new();
        for chunk in self.text_chunks(display_row) {
            if let Some(ix) = chunk.find('\n') {
                result.push_str(&chunk[0..ix]);
                break;
            } else {
                result.push_str(chunk);
            }
        }
        result
    }

    pub fn line_indent(&self, display_row: u32) -> (u32, bool) {
        let mut indent = 0;
        let mut is_blank = true;
        for c in self.chars_at(DisplayPoint::new(display_row, 0)) {
            if c == ' ' {
                indent += 1;
            } else {
                is_blank = c == '\n';
                break;
            }
        }
        (indent, is_blank)
    }

    pub fn line_len(&self, row: u32) -> u32 {
        self.blocks_snapshot.line_len(row)
    }

    pub fn longest_row(&self) -> u32 {
        self.blocks_snapshot.longest_row()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DisplayPoint(BlockPoint);

impl DisplayPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(BlockPoint(Point::new(row, column)))
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    #[cfg(test)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn column(self) -> u32 {
        self.0.column
    }

    pub fn row_mut(&mut self) -> &mut u32 {
        &mut self.0.row
    }

    pub fn column_mut(&mut self) -> &mut u32 {
        &mut self.0.column
    }

    pub fn to_point(self, map: &DisplaySnapshot) -> Point {
        map.display_point_to_point(self, Bias::Left)
    }

    pub fn to_offset(self, map: &DisplaySnapshot, bias: Bias) -> usize {
        let unblocked_point = map.blocks_snapshot.to_wrap_point(self.0);
        let unwrapped_point = map.wraps_snapshot.to_tab_point(unblocked_point);
        let unexpanded_point = map.tabs_snapshot.to_fold_point(unwrapped_point, bias).0;
        unexpanded_point.to_buffer_offset(&map.folds_snapshot)
    }
}

impl ToDisplayPoint for usize {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        map.point_to_display_point(self.to_point(&map.buffer_snapshot), Bias::Left)
    }
}

impl ToDisplayPoint for Point {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        map.point_to_display_point(*self, Bias::Left)
    }
}

impl ToDisplayPoint for Anchor {
    fn to_display_point(&self, map: &DisplaySnapshot) -> DisplayPoint {
        self.to_point(&map.buffer_snapshot).to_display_point(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::movement;
    use gpui::{color::Color, elements::*, test::observe, MutableAppContext};
    use language::{Buffer, Language, LanguageConfig, RandomCharIter, SelectionGoal};
    use rand::{prelude::*, Rng};
    use smol::stream::StreamExt;
    use std::{env, sync::Arc};
    use theme::SyntaxTheme;
    use util::test::sample_text;
    use Bias::*;

    #[gpui::test(iterations = 100)]
    async fn test_random_display_map(cx: &mut gpui::TestAppContext, mut rng: StdRng) {
        cx.foreground().set_block_on_ticks(0..=50);
        cx.foreground().forbid_parking();
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let font_cache = cx.font_cache().clone();
        let tab_size = rng.gen_range(1..=4);
        let buffer_start_excerpt_header_height = rng.gen_range(1..=5);
        let excerpt_header_height = rng.gen_range(1..=5);
        let family_id = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;
        let max_wrap_width = 300.0;
        let mut wrap_width = if rng.gen_bool(0.1) {
            None
        } else {
            Some(rng.gen_range(0.0..=max_wrap_width))
        };

        log::info!("tab size: {}", tab_size);
        log::info!("wrap width: {:?}", wrap_width);

        let buffer = cx.update(|cx| {
            if rng.gen() {
                let len = rng.gen_range(0..10);
                let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                MultiBuffer::build_simple(&text, cx)
            } else {
                MultiBuffer::build_random(&mut rng, cx)
            }
        });

        let map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                tab_size,
                font_id,
                font_size,
                wrap_width,
                buffer_start_excerpt_header_height,
                excerpt_header_height,
                cx,
            )
        });
        let mut notifications = observe(&map, cx);
        let mut fold_count = 0;
        let mut blocks = Vec::new();

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        log::info!("buffer text: {:?}", snapshot.buffer_snapshot.text());
        log::info!("fold text: {:?}", snapshot.folds_snapshot.text());
        log::info!("tab text: {:?}", snapshot.tabs_snapshot.text());
        log::info!("wrap text: {:?}", snapshot.wraps_snapshot.text());
        log::info!("block text: {:?}", snapshot.blocks_snapshot.text());
        log::info!("display text: {:?}", snapshot.text());

        for _i in 0..operations {
            match rng.gen_range(0..100) {
                0..=19 => {
                    wrap_width = if rng.gen_bool(0.2) {
                        None
                    } else {
                        Some(rng.gen_range(0.0..=max_wrap_width))
                    };
                    log::info!("setting wrap width to {:?}", wrap_width);
                    map.update(cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                20..=44 => {
                    map.update(cx, |map, cx| {
                        if rng.gen() || blocks.is_empty() {
                            let buffer = map.snapshot(cx).buffer_snapshot;
                            let block_properties = (0..rng.gen_range(1..=1))
                                .map(|_| {
                                    let position =
                                        buffer.anchor_after(buffer.clip_offset(
                                            rng.gen_range(0..=buffer.len()),
                                            Bias::Left,
                                        ));

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
                            blocks.extend(map.insert_blocks(block_properties, cx));
                        } else {
                            blocks.shuffle(&mut rng);
                            let remove_count = rng.gen_range(1..=4.min(blocks.len()));
                            let block_ids_to_remove = (0..remove_count)
                                .map(|_| blocks.remove(rng.gen_range(0..blocks.len())))
                                .collect();
                            log::info!("removing block ids {:?}", block_ids_to_remove);
                            map.remove_blocks(block_ids_to_remove, cx);
                        }
                    });
                }
                45..=79 => {
                    let mut ranges = Vec::new();
                    for _ in 0..rng.gen_range(1..=3) {
                        buffer.read_with(cx, |buffer, cx| {
                            let buffer = buffer.read(cx);
                            let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                            let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                            ranges.push(start..end);
                        });
                    }

                    if rng.gen() && fold_count > 0 {
                        log::info!("unfolding ranges: {:?}", ranges);
                        map.update(cx, |map, cx| {
                            map.unfold(ranges, cx);
                        });
                    } else {
                        log::info!("folding ranges: {:?}", ranges);
                        map.update(cx, |map, cx| {
                            map.fold(ranges, cx);
                        });
                    }
                }
                _ => {
                    buffer.update(cx, |buffer, cx| buffer.randomly_edit(&mut rng, 5, cx));
                }
            }

            if map.read_with(cx, |map, cx| map.is_rewrapping(cx)) {
                notifications.next().await.unwrap();
            }

            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            fold_count = snapshot.fold_count();
            log::info!("buffer text: {:?}", snapshot.buffer_snapshot.text());
            log::info!("fold text: {:?}", snapshot.folds_snapshot.text());
            log::info!("tab text: {:?}", snapshot.tabs_snapshot.text());
            log::info!("wrap text: {:?}", snapshot.wraps_snapshot.text());
            log::info!("block text: {:?}", snapshot.blocks_snapshot.text());
            log::info!("display text: {:?}", snapshot.text());

            // Line boundaries
            let buffer = &snapshot.buffer_snapshot;
            for _ in 0..5 {
                let row = rng.gen_range(0..=buffer.max_point().row);
                let column = rng.gen_range(0..=buffer.line_len(row));
                let point = buffer.clip_point(Point::new(row, column), Left);

                let (prev_buffer_bound, prev_display_bound) = snapshot.prev_line_boundary(point);
                let (next_buffer_bound, next_display_bound) = snapshot.next_line_boundary(point);

                assert!(prev_buffer_bound <= point);
                assert!(next_buffer_bound >= point);
                assert_eq!(prev_buffer_bound.column, 0);
                assert_eq!(prev_display_bound.column(), 0);
                if next_buffer_bound < buffer.max_point() {
                    assert_eq!(buffer.chars_at(next_buffer_bound).next(), Some('\n'));
                }

                assert_eq!(
                    prev_display_bound,
                    prev_buffer_bound.to_display_point(&snapshot),
                    "row boundary before {:?}. reported buffer row boundary: {:?}",
                    point,
                    prev_buffer_bound
                );
                assert_eq!(
                    next_display_bound,
                    next_buffer_bound.to_display_point(&snapshot),
                    "display row boundary after {:?}. reported buffer row boundary: {:?}",
                    point,
                    next_buffer_bound
                );
                assert_eq!(
                    prev_buffer_bound,
                    prev_display_bound.to_point(&snapshot),
                    "row boundary before {:?}. reported display row boundary: {:?}",
                    point,
                    prev_display_bound
                );
                assert_eq!(
                    next_buffer_bound,
                    next_display_bound.to_point(&snapshot),
                    "row boundary after {:?}. reported display row boundary: {:?}",
                    point,
                    next_display_bound
                );
            }

            // Movement
            let min_point = snapshot.clip_point(DisplayPoint::new(0, 0), Left);
            let max_point = snapshot.clip_point(snapshot.max_point(), Right);
            for _ in 0..5 {
                let row = rng.gen_range(0..=snapshot.max_point().row());
                let column = rng.gen_range(0..=snapshot.line_len(row));
                let point = snapshot.clip_point(DisplayPoint::new(row, column), Left);

                log::info!("Moving from point {:?}", point);

                let moved_right = movement::right(&snapshot, point).unwrap();
                log::info!("Right {:?}", moved_right);
                if point < max_point {
                    assert!(moved_right > point);
                    if point.column() == snapshot.line_len(point.row())
                        || snapshot.soft_wrap_indent(point.row()).is_some()
                            && point.column() == snapshot.line_len(point.row()) - 1
                    {
                        assert!(moved_right.row() > point.row());
                    }
                } else {
                    assert_eq!(moved_right, point);
                }

                let moved_left = movement::left(&snapshot, point).unwrap();
                log::info!("Left {:?}", moved_left);
                if point > min_point {
                    assert!(moved_left < point);
                    if point.column() == 0 {
                        assert!(moved_left.row() < point.row());
                    }
                } else {
                    assert_eq!(moved_left, point);
                }
            }
        }
    }

    #[gpui::test(retries = 5)]
    fn test_soft_wraps(cx: &mut MutableAppContext) {
        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.foreground().forbid_parking();

        let font_cache = cx.font_cache();

        let tab_size = 4;
        let family_id = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 12.0;
        let wrap_width = Some(64.);

        let text = "one two three four five\nsix seven eight";
        let buffer = MultiBuffer::build_simple(text, cx);
        let map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                tab_size,
                font_id,
                font_size,
                wrap_width,
                1,
                1,
                cx,
            )
        });

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            snapshot.text_chunks(0).collect::<String>(),
            "one two \nthree four \nfive\nsix seven \neight"
        );
        assert_eq!(
            snapshot.clip_point(DisplayPoint::new(0, 8), Bias::Left),
            DisplayPoint::new(0, 7)
        );
        assert_eq!(
            snapshot.clip_point(DisplayPoint::new(0, 8), Bias::Right),
            DisplayPoint::new(1, 0)
        );
        assert_eq!(
            movement::right(&snapshot, DisplayPoint::new(0, 7)).unwrap(),
            DisplayPoint::new(1, 0)
        );
        assert_eq!(
            movement::left(&snapshot, DisplayPoint::new(1, 0)).unwrap(),
            DisplayPoint::new(0, 7)
        );
        assert_eq!(
            movement::up(&snapshot, DisplayPoint::new(1, 10), SelectionGoal::None).unwrap(),
            (DisplayPoint::new(0, 7), SelectionGoal::Column(10))
        );
        assert_eq!(
            movement::down(
                &snapshot,
                DisplayPoint::new(0, 7),
                SelectionGoal::Column(10)
            )
            .unwrap(),
            (DisplayPoint::new(1, 10), SelectionGoal::Column(10))
        );
        assert_eq!(
            movement::down(
                &snapshot,
                DisplayPoint::new(1, 10),
                SelectionGoal::Column(10)
            )
            .unwrap(),
            (DisplayPoint::new(2, 4), SelectionGoal::Column(10))
        );

        let ix = snapshot.buffer_snapshot.text().find("seven").unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.edit(vec![ix..ix], "and ", cx);
        });

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            snapshot.text_chunks(1).collect::<String>(),
            "three four \nfive\nsix and \nseven eight"
        );

        // Re-wrap on font size changes
        map.update(cx, |map, cx| map.set_font(font_id, font_size + 3., cx));

        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            snapshot.text_chunks(1).collect::<String>(),
            "three \nfour five\nsix and \nseven \neight"
        )
    }

    #[gpui::test]
    fn test_text_chunks(cx: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6, 'a');
        let buffer = MultiBuffer::build_simple(&text, cx);
        let tab_size = 4;
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;
        let map = cx.add_model(|cx| {
            DisplayMap::new(buffer.clone(), tab_size, font_id, font_size, None, 1, 1, cx)
        });
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![
                    Point::new(1, 0)..Point::new(1, 0),
                    Point::new(1, 1)..Point::new(1, 1),
                    Point::new(2, 1)..Point::new(2, 1),
                ],
                "\t",
                cx,
            )
        });

        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx))
                .text_chunks(1)
                .collect::<String>()
                .lines()
                .next(),
            Some("    b   bbbbb")
        );
        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx))
                .text_chunks(2)
                .collect::<String>()
                .lines()
                .next(),
            Some("c   ccccc")
        );
    }

    #[gpui::test]
    async fn test_chunks(cx: &mut gpui::TestAppContext) {
        use unindent::Unindent as _;

        let text = r#"
            fn outer() {}

            mod module {
                fn inner() {}
            }"#
        .unindent();

        let theme = SyntaxTheme::new(vec![
            ("mod.body".to_string(), Color::red().into()),
            ("fn.name".to_string(), Color::blue().into()),
        ]);
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Test".into(),
                    path_suffixes: vec![".test".to_string()],
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
            )
            .with_highlights_query(
                r#"
                (mod_item name: (identifier) body: _ @mod.body)
                (function_item name: (identifier) @fn.name)
                "#,
            )
            .unwrap(),
        );
        language.set_theme(&theme);

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
        buffer.condition(&cx, |buf, _| !buf.is_parsing()).await;
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));

        let tab_size = 2;
        let font_cache = cx.font_cache();
        let family_id = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;

        let map = cx
            .add_model(|cx| DisplayMap::new(buffer, tab_size, font_id, font_size, None, 1, 1, cx));
        assert_eq!(
            cx.update(|cx| chunks(0..5, &map, &theme, cx)),
            vec![
                ("fn ".to_string(), None),
                ("outer".to_string(), Some(Color::blue())),
                ("() {}\n\nmod module ".to_string(), None),
                ("{\n    fn ".to_string(), Some(Color::red())),
                ("inner".to_string(), Some(Color::blue())),
                ("() {}\n}".to_string(), Some(Color::red())),
            ]
        );
        assert_eq!(
            cx.update(|cx| chunks(3..5, &map, &theme, cx)),
            vec![
                ("    fn ".to_string(), Some(Color::red())),
                ("inner".to_string(), Some(Color::blue())),
                ("() {}\n}".to_string(), Some(Color::red())),
            ]
        );

        map.update(cx, |map, cx| {
            map.fold(vec![Point::new(0, 6)..Point::new(3, 2)], cx)
        });
        assert_eq!(
            cx.update(|cx| chunks(0..2, &map, &theme, cx)),
            vec![
                ("fn ".to_string(), None),
                ("out".to_string(), Some(Color::blue())),
                ("…".to_string(), None),
                ("  fn ".to_string(), Some(Color::red())),
                ("inner".to_string(), Some(Color::blue())),
                ("() {}\n}".to_string(), Some(Color::red())),
            ]
        );
    }

    #[gpui::test]
    async fn test_chunks_with_soft_wrapping(cx: &mut gpui::TestAppContext) {
        use unindent::Unindent as _;

        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);

        let text = r#"
            fn outer() {}

            mod module {
                fn inner() {}
            }"#
        .unindent();

        let theme = SyntaxTheme::new(vec![
            ("mod.body".to_string(), Color::red().into()),
            ("fn.name".to_string(), Color::blue().into()),
        ]);
        let language = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Test".into(),
                    path_suffixes: vec![".test".to_string()],
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
            )
            .with_highlights_query(
                r#"
                (mod_item name: (identifier) body: _ @mod.body)
                (function_item name: (identifier) @fn.name)
                "#,
            )
            .unwrap(),
        );
        language.set_theme(&theme);

        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
        buffer.condition(&cx, |buf, _| !buf.is_parsing()).await;
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));

        let font_cache = cx.font_cache();

        let tab_size = 4;
        let family_id = font_cache.load_family(&["Courier"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 16.0;

        let map = cx.add_model(|cx| {
            DisplayMap::new(buffer, tab_size, font_id, font_size, Some(40.0), 1, 1, cx)
        });
        assert_eq!(
            cx.update(|cx| chunks(0..5, &map, &theme, cx)),
            [
                ("fn \n".to_string(), None),
                ("oute\nr".to_string(), Some(Color::blue())),
                ("() \n{}\n\n".to_string(), None),
            ]
        );
        assert_eq!(
            cx.update(|cx| chunks(3..5, &map, &theme, cx)),
            [("{}\n\n".to_string(), None)]
        );

        map.update(cx, |map, cx| {
            map.fold(vec![Point::new(0, 6)..Point::new(3, 2)], cx)
        });
        assert_eq!(
            cx.update(|cx| chunks(1..4, &map, &theme, cx)),
            [
                ("out".to_string(), Some(Color::blue())),
                ("…\n".to_string(), None),
                ("  \nfn ".to_string(), Some(Color::red())),
                ("i\n".to_string(), Some(Color::blue()))
            ]
        );
    }

    #[gpui::test]
    fn test_clip_point(cx: &mut gpui::MutableAppContext) {
        use Bias::{Left, Right};

        let text = "\n'a', 'α',\t'✋',\t'❎', '🍐'\n";
        let display_text = "\n'a', 'α',   '✋',    '❎', '🍐'\n";
        let buffer = MultiBuffer::build_simple(text, cx);

        let tab_size = 4;
        let font_cache = cx.font_cache();
        let family_id = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;
        let map = cx.add_model(|cx| {
            DisplayMap::new(buffer.clone(), tab_size, font_id, font_size, None, 1, 1, cx)
        });
        let map = map.update(cx, |map, cx| map.snapshot(cx));

        assert_eq!(map.text(), display_text);
        for (input_column, bias, output_column) in vec![
            ("'a', '".len(), Left, "'a', '".len()),
            ("'a', '".len() + 1, Left, "'a', '".len()),
            ("'a', '".len() + 1, Right, "'a', 'α".len()),
            ("'a', 'α', ".len(), Left, "'a', 'α',".len()),
            ("'a', 'α', ".len(), Right, "'a', 'α',   ".len()),
            ("'a', 'α',   '".len() + 1, Left, "'a', 'α',   '".len()),
            ("'a', 'α',   '".len() + 1, Right, "'a', 'α',   '✋".len()),
            ("'a', 'α',   '✋',".len(), Right, "'a', 'α',   '✋',".len()),
            ("'a', 'α',   '✋', ".len(), Left, "'a', 'α',   '✋',".len()),
            (
                "'a', 'α',   '✋', ".len(),
                Right,
                "'a', 'α',   '✋',    ".len(),
            ),
        ] {
            assert_eq!(
                map.clip_point(DisplayPoint::new(1, input_column as u32), bias),
                DisplayPoint::new(1, output_column as u32),
                "clip_point(({}, {}))",
                1,
                input_column,
            );
        }
    }

    #[gpui::test]
    fn test_tabs_with_multibyte_chars(cx: &mut gpui::MutableAppContext) {
        let text = "✅\t\tα\nβ\t\n🏀β\t\tγ";
        let buffer = MultiBuffer::build_simple(text, cx);
        let tab_size = 4;
        let font_cache = cx.font_cache();
        let family_id = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;

        let map = cx.add_model(|cx| {
            DisplayMap::new(buffer.clone(), tab_size, font_id, font_size, None, 1, 1, cx)
        });
        let map = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(map.text(), "✅       α\nβ   \n🏀β      γ");
        assert_eq!(
            map.text_chunks(0).collect::<String>(),
            "✅       α\nβ   \n🏀β      γ"
        );
        assert_eq!(map.text_chunks(1).collect::<String>(), "β   \n🏀β      γ");
        assert_eq!(map.text_chunks(2).collect::<String>(), "🏀β      γ");

        let point = Point::new(0, "✅\t\t".len() as u32);
        let display_point = DisplayPoint::new(0, "✅       ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_point(&map), point);

        let point = Point::new(1, "β\t".len() as u32);
        let display_point = DisplayPoint::new(1, "β   ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_point(&map), point,);

        let point = Point::new(2, "🏀β\t\t".len() as u32);
        let display_point = DisplayPoint::new(2, "🏀β      ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_point(&map), point,);

        // Display points inside of expanded tabs
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_point(&map),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_point(&map),
            Point::new(0, "✅".len() as u32),
        );

        // Clipping display points inside of multi-byte characters
        assert_eq!(
            map.clip_point(DisplayPoint::new(0, "✅".len() as u32 - 1), Left),
            DisplayPoint::new(0, 0)
        );
        assert_eq!(
            map.clip_point(DisplayPoint::new(0, "✅".len() as u32 - 1), Bias::Right),
            DisplayPoint::new(0, "✅".len() as u32)
        );
    }

    #[gpui::test]
    fn test_max_point(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple("aaa\n\t\tbbb", cx);
        let tab_size = 4;
        let font_cache = cx.font_cache();
        let family_id = font_cache.load_family(&["Helvetica"]).unwrap();
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;
        let map = cx.add_model(|cx| {
            DisplayMap::new(buffer.clone(), tab_size, font_id, font_size, None, 1, 1, cx)
        });
        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx)).max_point(),
            DisplayPoint::new(1, 11)
        )
    }

    fn chunks<'a>(
        rows: Range<u32>,
        map: &ModelHandle<DisplayMap>,
        theme: &'a SyntaxTheme,
        cx: &mut MutableAppContext,
    ) -> Vec<(String, Option<Color>)> {
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut chunks: Vec<(String, Option<Color>)> = Vec::new();
        for chunk in snapshot.chunks(rows, true) {
            let color = chunk
                .syntax_highlight_id
                .and_then(|id| id.style(theme).map(|s| s.color));
            if let Some((last_chunk, last_color)) = chunks.last_mut() {
                if color == *last_color {
                    last_chunk.push_str(chunk.text);
                } else {
                    chunks.push((chunk.text.to_string(), color));
                }
            } else {
                chunks.push((chunk.text.to_string(), color));
            }
        }
        chunks
    }
}
