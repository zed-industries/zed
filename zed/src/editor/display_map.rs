mod fold_map;
mod line_wrapper;
mod tab_map;
mod wrap_map;

use super::{buffer, Anchor, Bias, Buffer, Point, Settings, ToOffset, ToPoint};
use fold_map::FoldMap;
use gpui::{Entity, ModelContext, ModelHandle};
use std::ops::Range;
use tab_map::TabMap;
pub use wrap_map::BufferRows;
use wrap_map::WrapMap;

pub struct DisplayMap {
    buffer: ModelHandle<Buffer>,
    fold_map: FoldMap,
    tab_map: TabMap,
    wrap_map: ModelHandle<WrapMap>,
}

impl Entity for DisplayMap {
    type Event = ();
}

impl DisplayMap {
    pub fn new(
        buffer: ModelHandle<Buffer>,
        settings: Settings,
        wrap_width: Option<f32>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let (fold_map, snapshot) = FoldMap::new(buffer.clone(), cx);
        let (tab_map, snapshot) = TabMap::new(snapshot, settings.tab_size);
        let wrap_map = cx.add_model(|cx| WrapMap::new(snapshot, settings, wrap_width, cx));
        cx.observe(&wrap_map, |_, _, cx| cx.notify());
        DisplayMap {
            buffer,
            fold_map,
            tab_map,
            wrap_map,
        }
    }

    pub fn snapshot(&self, cx: &mut ModelContext<Self>) -> DisplayMapSnapshot {
        let (folds_snapshot, edits) = self.fold_map.read(cx);
        let (tabs_snapshot, edits) = self.tab_map.sync(folds_snapshot.clone(), edits);
        let wraps_snapshot = self
            .wrap_map
            .update(cx, |map, cx| map.sync(tabs_snapshot.clone(), edits, cx));
        DisplayMapSnapshot {
            buffer_snapshot: self.buffer.read(cx).snapshot(),
            folds_snapshot,
            tabs_snapshot,
            wraps_snapshot,
        }
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &mut ModelContext<Self>,
    ) {
        let (mut fold_map, snapshot, edits) = self.fold_map.write(cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        let (snapshot, edits) = fold_map.fold(ranges, cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &mut ModelContext<Self>,
    ) {
        let (mut fold_map, snapshot, edits) = self.fold_map.write(cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
        let (snapshot, edits) = fold_map.unfold(ranges, cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map
            .update(cx, |map, cx| map.sync(snapshot, edits, cx));
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

pub struct DisplayMapSnapshot {
    buffer_snapshot: buffer::Snapshot,
    folds_snapshot: fold_map::Snapshot,
    tabs_snapshot: tab_map::Snapshot,
    wraps_snapshot: wrap_map::Snapshot,
}

impl DisplayMapSnapshot {
    #[cfg(test)]
    pub fn fold_count(&self) -> usize {
        self.folds_snapshot.fold_count()
    }

    pub fn buffer_rows(&self, start_row: u32) -> BufferRows {
        self.wraps_snapshot.buffer_rows(start_row)
    }

    pub fn buffer_row_count(&self) -> u32 {
        self.buffer_snapshot.max_point().row + 1
    }

    pub fn prev_row_boundary(&self, mut display_point: DisplayPoint) -> (DisplayPoint, Point) {
        loop {
            *display_point.column_mut() = 0;
            let mut point = display_point.to_buffer_point(self, Bias::Left);
            point.column = 0;
            let next_display_point = point.to_display_point(self, Bias::Left);
            if next_display_point == display_point {
                return (display_point, point);
            }
            display_point = next_display_point;
        }
    }

    pub fn next_row_boundary(&self, mut display_point: DisplayPoint) -> (DisplayPoint, Point) {
        loop {
            *display_point.column_mut() = self.line_len(display_point.row());
            let mut point = display_point.to_buffer_point(self, Bias::Right);
            point.column = self.buffer_snapshot.line_len(point.row);
            let next_display_point = point.to_display_point(self, Bias::Right);
            if next_display_point == display_point {
                return (display_point, point);
            }
            display_point = next_display_point;
        }
    }

    pub fn max_point(&self) -> DisplayPoint {
        DisplayPoint(self.wraps_snapshot.max_point())
    }

    pub fn chunks_at(&self, display_row: u32) -> wrap_map::Chunks {
        self.wraps_snapshot.chunks_at(display_row)
    }

    pub fn highlighted_chunks_for_rows(
        &mut self,
        display_rows: Range<u32>,
    ) -> wrap_map::HighlightedChunks {
        self.wraps_snapshot
            .highlighted_chunks_for_rows(display_rows)
    }

    pub fn chars_at<'a>(&'a self, display_row: u32) -> impl Iterator<Item = char> + 'a {
        self.chunks_at(display_row).flat_map(str::chars)
    }

    pub fn column_to_chars(&self, display_row: u32, target: u32) -> u32 {
        let mut count = 0;
        let mut column = 0;
        for c in self.chars_at(display_row) {
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
        for c in self.chars_at(display_row) {
            if c == '\n' || count >= char_count {
                break;
            }
            count += 1;
            column += c.len_utf8() as u32;
        }
        column
    }

    pub fn clip_point(&self, point: DisplayPoint, bias: Bias) -> DisplayPoint {
        DisplayPoint(self.wraps_snapshot.clip_point(point.0, bias))
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

    pub fn intersects_fold<T: ToOffset>(&self, offset: T) -> bool {
        self.folds_snapshot.intersects_fold(offset)
    }

    pub fn is_line_folded(&self, display_row: u32) -> bool {
        let wrap_point = DisplayPoint::new(display_row, 0).0;
        let row = self.wraps_snapshot.to_tab_point(wrap_point).row();
        self.folds_snapshot.is_line_folded(row)
    }

    pub fn soft_wrap_indent(&self, display_row: u32) -> Option<u32> {
        self.wraps_snapshot.soft_wrap_indent(display_row)
    }

    pub fn text(&self) -> String {
        self.chunks_at(0).collect()
    }

    pub fn line(&self, display_row: u32) -> String {
        let mut result = String::new();
        for chunk in self.chunks_at(display_row) {
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
        for c in self.chars_at(display_row) {
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
        self.wraps_snapshot.line_len(row)
    }

    pub fn longest_row(&self) -> u32 {
        self.wraps_snapshot.longest_row()
    }

    pub fn anchor_before(&self, point: DisplayPoint, bias: Bias) -> Anchor {
        self.buffer_snapshot
            .anchor_before(point.to_buffer_point(self, bias))
    }

    pub fn anchor_after(&self, point: DisplayPoint, bias: Bias) -> Anchor {
        self.buffer_snapshot
            .anchor_after(point.to_buffer_point(self, bias))
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DisplayPoint(wrap_map::WrapPoint);

impl DisplayPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(wrap_map::WrapPoint::new(row, column))
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    #[cfg(test)]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn row(self) -> u32 {
        self.0.row()
    }

    pub fn column(self) -> u32 {
        self.0.column()
    }

    pub fn row_mut(&mut self) -> &mut u32 {
        self.0.row_mut()
    }

    pub fn column_mut(&mut self) -> &mut u32 {
        self.0.column_mut()
    }

    pub fn to_buffer_point(self, map: &DisplayMapSnapshot, bias: Bias) -> Point {
        let unwrapped_point = map.wraps_snapshot.to_tab_point(self.0);
        let unexpanded_point = map.tabs_snapshot.to_fold_point(unwrapped_point, bias).0;
        unexpanded_point.to_buffer_point(&map.folds_snapshot)
    }

    pub fn to_buffer_offset(self, map: &DisplayMapSnapshot, bias: Bias) -> usize {
        let unwrapped_point = map.wraps_snapshot.to_tab_point(self.0);
        let unexpanded_point = map.tabs_snapshot.to_fold_point(unwrapped_point, bias).0;
        unexpanded_point.to_buffer_offset(&map.folds_snapshot)
    }
}

impl Point {
    pub fn to_display_point(self, map: &DisplayMapSnapshot, bias: Bias) -> DisplayPoint {
        let fold_point = self.to_fold_point(&map.folds_snapshot, bias);
        let tab_point = map.tabs_snapshot.to_tab_point(fold_point);
        let wrap_point = map.wraps_snapshot.to_wrap_point(tab_point);
        DisplayPoint(wrap_point)
    }
}

impl Anchor {
    pub fn to_display_point(&self, map: &DisplayMapSnapshot, bias: Bias) -> DisplayPoint {
        self.to_point(&map.buffer_snapshot)
            .to_display_point(map, bias)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor::movement,
        language::{Language, LanguageConfig},
        settings::Theme,
        test::*,
        util::RandomCharIter,
    };
    use buffer::{History, SelectionGoal};
    use gpui::MutableAppContext;
    use rand::{prelude::StdRng, Rng};
    use std::{env, sync::Arc};
    use Bias::*;

    #[gpui::test(iterations = 100)]
    async fn test_random(mut cx: gpui::TestAppContext, mut rng: StdRng) {
        cx.foreground().set_block_on_ticks(0..=50);
        cx.foreground().forbid_parking();
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let font_cache = cx.font_cache().clone();
        let settings = Settings {
            tab_size: rng.gen_range(1..=4),
            buffer_font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
            buffer_font_size: 14.0,
            ..Settings::new(&font_cache).unwrap()
        };
        let max_wrap_width = 300.0;
        let mut wrap_width = if rng.gen_bool(0.1) {
            None
        } else {
            Some(rng.gen_range(0.0..=max_wrap_width))
        };

        log::info!("tab size: {}", settings.tab_size);
        log::info!("wrap width: {:?}", wrap_width);

        let buffer = cx.add_model(|cx| {
            let len = rng.gen_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            Buffer::new(0, text, cx)
        });

        let map = cx.add_model(|cx| DisplayMap::new(buffer.clone(), settings, wrap_width, cx));
        let (_observer, notifications) = Observer::new(&map, &mut cx);
        let mut fold_count = 0;

        for _i in 0..operations {
            match rng.gen_range(0..100) {
                0..=19 => {
                    wrap_width = if rng.gen_bool(0.2) {
                        None
                    } else {
                        Some(rng.gen_range(0.0..=max_wrap_width))
                    };
                    log::info!("setting wrap width to {:?}", wrap_width);
                    map.update(&mut cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                20..=80 => {
                    let mut ranges = Vec::new();
                    for _ in 0..rng.gen_range(1..=3) {
                        buffer.read_with(&cx, |buffer, _| {
                            let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                            let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                            ranges.push(start..end);
                        });
                    }

                    if rng.gen() && fold_count > 0 {
                        log::info!("unfolding ranges: {:?}", ranges);
                        map.update(&mut cx, |map, cx| {
                            map.unfold(ranges, cx);
                        });
                    } else {
                        log::info!("folding ranges: {:?}", ranges);
                        map.update(&mut cx, |map, cx| {
                            map.fold(ranges, cx);
                        });
                    }
                }
                _ => {
                    buffer.update(&mut cx, |buffer, cx| buffer.randomly_mutate(&mut rng, cx));
                }
            }

            if map.read_with(&cx, |map, cx| map.is_rewrapping(cx)) {
                notifications.recv().await.unwrap();
            }

            let snapshot = map.update(&mut cx, |map, cx| map.snapshot(cx));
            fold_count = snapshot.fold_count();
            log::info!("buffer text: {:?}", buffer.read_with(&cx, |b, _| b.text()));
            log::info!("display text: {:?}", snapshot.text());

            // Line boundaries
            for _ in 0..5 {
                let row = rng.gen_range(0..=snapshot.max_point().row());
                let column = rng.gen_range(0..=snapshot.line_len(row));
                let point = snapshot.clip_point(DisplayPoint::new(row, column), Left);

                let (prev_display_bound, prev_buffer_bound) = snapshot.prev_row_boundary(point);
                let (next_display_bound, next_buffer_bound) = snapshot.next_row_boundary(point);

                assert!(prev_display_bound <= point);
                assert!(next_display_bound >= point);
                assert_eq!(prev_buffer_bound.column, 0);
                assert_eq!(prev_display_bound.column(), 0);
                if next_display_bound < snapshot.max_point() {
                    assert_eq!(
                        buffer
                            .read_with(&cx, |buffer, _| buffer.chars_at(next_buffer_bound).next()),
                        Some('\n')
                    )
                }

                assert_eq!(
                    prev_display_bound,
                    prev_buffer_bound.to_display_point(&snapshot, Left),
                    "row boundary before {:?}. reported buffer row boundary: {:?}",
                    point,
                    prev_buffer_bound
                );
                assert_eq!(
                    next_display_bound,
                    next_buffer_bound.to_display_point(&snapshot, Right),
                    "display row boundary after {:?}. reported buffer row boundary: {:?}",
                    point,
                    next_buffer_bound
                );
                assert_eq!(
                    prev_buffer_bound,
                    prev_display_bound.to_buffer_point(&snapshot, Left),
                    "row boundary before {:?}. reported display row boundary: {:?}",
                    point,
                    prev_display_bound
                );
                assert_eq!(
                    next_buffer_bound,
                    next_display_bound.to_buffer_point(&snapshot, Right),
                    "row boundary after {:?}. reported display row boundary: {:?}",
                    point,
                    next_display_bound
                );
            }

            // Movement
            for _ in 0..5 {
                let row = rng.gen_range(0..=snapshot.max_point().row());
                let column = rng.gen_range(0..=snapshot.line_len(row));
                let point = snapshot.clip_point(DisplayPoint::new(row, column), Left);

                log::info!("Moving from point {:?}", point);

                let moved_right = movement::right(&snapshot, point).unwrap();
                log::info!("Right {:?}", moved_right);
                if point < snapshot.max_point() {
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
                if !point.is_zero() {
                    assert!(moved_left < point);
                    if point.column() == 0 {
                        assert!(moved_left.row() < point.row());
                    }
                } else {
                    assert!(moved_left.is_zero());
                }
            }
        }
    }

    #[gpui::test]
    async fn test_soft_wraps(mut cx: gpui::TestAppContext) {
        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);
        cx.foreground().forbid_parking();

        let font_cache = cx.font_cache();

        let settings = Settings {
            buffer_font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
            ui_font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
            buffer_font_size: 12.0,
            ui_font_size: 12.0,
            tab_size: 4,
            theme: Arc::new(Theme::default()),
        };
        let wrap_width = Some(64.);

        let text = "one two three four five\nsix seven eight";
        let buffer = cx.add_model(|cx| Buffer::new(0, text.to_string(), cx));
        let map = cx.add_model(|cx| DisplayMap::new(buffer.clone(), settings, wrap_width, cx));

        let snapshot = map.update(&mut cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            snapshot.chunks_at(0).collect::<String>(),
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

        buffer.update(&mut cx, |buffer, cx| {
            let ix = buffer.text().find("seven").unwrap();
            buffer.edit(vec![ix..ix], "and ", cx);
        });

        let snapshot = map.update(&mut cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            snapshot.chunks_at(1).collect::<String>(),
            "three four \nfive\nsix and \nseven eight"
        );
    }

    #[gpui::test]
    fn test_chunks_at(cx: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6);
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
                None,
                cx,
            )
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
                .chunks_at(1)
                .collect::<String>()
                .lines()
                .next(),
            Some("    b   bbbbb")
        );
        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx))
                .chunks_at(2)
                .collect::<String>()
                .lines()
                .next(),
            Some("c   ccccc")
        );
    }

    #[gpui::test]
    async fn test_highlighted_chunks_at(mut cx: gpui::TestAppContext) {
        use unindent::Unindent as _;

        let grammar = tree_sitter_rust::language();
        let text = r#"
            fn outer() {}

            mod module {
                fn inner() {}
            }"#
        .unindent();
        let highlight_query = tree_sitter::Query::new(
            grammar,
            r#"
            (mod_item name: (identifier) body: _ @mod.body)
            (function_item name: (identifier) @fn.name)"#,
        )
        .unwrap();
        let theme = Theme::parse(
            r#"
            [syntax]
            "mod.body" = 0xff0000
            "fn.name" = 0x00ff00"#,
        )
        .unwrap();
        let lang = Arc::new(Language {
            config: LanguageConfig {
                name: "Test".to_string(),
                path_suffixes: vec![".test".to_string()],
                ..Default::default()
            },
            grammar: grammar.clone(),
            highlight_query,
            brackets_query: tree_sitter::Query::new(grammar, "").unwrap(),
            theme_mapping: Default::default(),
        });
        lang.set_theme(&theme);

        let buffer = cx.add_model(|cx| {
            Buffer::from_history(0, History::new(text.into()), None, Some(lang), cx)
        });
        buffer.condition(&cx, |buf, _| !buf.is_parsing()).await;

        let map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer,
                Settings::new(cx.font_cache()).unwrap().with_tab_size(2),
                None,
                cx,
            )
        });
        assert_eq!(
            cx.update(|cx| highlighted_chunks(0..5, &map, &theme, cx)),
            vec![
                ("fn ".to_string(), None),
                ("outer".to_string(), Some("fn.name")),
                ("() {}\n\nmod module ".to_string(), None),
                ("{\n    fn ".to_string(), Some("mod.body")),
                ("inner".to_string(), Some("fn.name")),
                ("() {}\n}".to_string(), Some("mod.body")),
            ]
        );
        assert_eq!(
            cx.update(|cx| highlighted_chunks(3..5, &map, &theme, cx)),
            vec![
                ("    fn ".to_string(), Some("mod.body")),
                ("inner".to_string(), Some("fn.name")),
                ("() {}\n}".to_string(), Some("mod.body")),
            ]
        );

        map.update(&mut cx, |map, cx| {
            map.fold(vec![Point::new(0, 6)..Point::new(3, 2)], cx)
        });
        assert_eq!(
            cx.update(|cx| highlighted_chunks(0..2, &map, &theme, cx)),
            vec![
                ("fn ".to_string(), None),
                ("out".to_string(), Some("fn.name")),
                ("…".to_string(), None),
                ("  fn ".to_string(), Some("mod.body")),
                ("inner".to_string(), Some("fn.name")),
                ("() {}\n}".to_string(), Some("mod.body")),
            ]
        );
    }

    #[gpui::test]
    async fn test_highlighted_chunks_with_soft_wrapping(mut cx: gpui::TestAppContext) {
        use unindent::Unindent as _;

        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);

        let grammar = tree_sitter_rust::language();
        let text = r#"
            fn outer() {}

            mod module {
                fn inner() {}
            }"#
        .unindent();
        let highlight_query = tree_sitter::Query::new(
            grammar,
            r#"
            (mod_item name: (identifier) body: _ @mod.body)
            (function_item name: (identifier) @fn.name)"#,
        )
        .unwrap();
        let theme = Theme::parse(
            r#"
            [syntax]
            "mod.body" = 0xff0000
            "fn.name" = 0x00ff00"#,
        )
        .unwrap();
        let lang = Arc::new(Language {
            config: LanguageConfig {
                name: "Test".to_string(),
                path_suffixes: vec![".test".to_string()],
                ..Default::default()
            },
            grammar: grammar.clone(),
            highlight_query,
            brackets_query: tree_sitter::Query::new(grammar, "").unwrap(),
            theme_mapping: Default::default(),
        });
        lang.set_theme(&theme);

        let buffer = cx.add_model(|cx| {
            Buffer::from_history(0, History::new(text.into()), None, Some(lang), cx)
        });
        buffer.condition(&cx, |buf, _| !buf.is_parsing()).await;

        let font_cache = cx.font_cache();
        let settings = Settings {
            tab_size: 4,
            buffer_font_family: font_cache.load_family(&["Courier"]).unwrap(),
            buffer_font_size: 16.0,
            ..Settings::new(&font_cache).unwrap()
        };
        let map = cx.add_model(|cx| DisplayMap::new(buffer, settings, Some(40.0), cx));
        assert_eq!(
            cx.update(|cx| highlighted_chunks(0..5, &map, &theme, cx)),
            [
                ("fn \n".to_string(), None),
                ("oute\nr".to_string(), Some("fn.name")),
                ("() \n{}\n\n".to_string(), None),
            ]
        );
        assert_eq!(
            cx.update(|cx| highlighted_chunks(3..5, &map, &theme, cx)),
            [("{}\n\n".to_string(), None)]
        );

        map.update(&mut cx, |map, cx| {
            map.fold(vec![Point::new(0, 6)..Point::new(3, 2)], cx)
        });
        assert_eq!(
            cx.update(|cx| highlighted_chunks(1..4, &map, &theme, cx)),
            [
                ("out".to_string(), Some("fn.name")),
                ("…\n".to_string(), None),
                ("  \nfn ".to_string(), Some("mod.body")),
                ("i\n".to_string(), Some("fn.name"))
            ]
        );
    }

    #[gpui::test]
    fn test_clip_point(cx: &mut gpui::MutableAppContext) {
        use Bias::{Left, Right};

        let text = "\n'a', 'α',\t'✋',\t'❎', '🍐'\n";
        let display_text = "\n'a', 'α',   '✋',    '❎', '🍐'\n";
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
                None,
                cx,
            )
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
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
                None,
                cx,
            )
        });
        let map = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(map.text(), "✅       α\nβ   \n🏀β      γ");
        assert_eq!(
            map.chunks_at(0).collect::<String>(),
            "✅       α\nβ   \n🏀β      γ"
        );
        assert_eq!(map.chunks_at(1).collect::<String>(), "β   \n🏀β      γ");
        assert_eq!(map.chunks_at(2).collect::<String>(), "🏀β      γ");

        let point = Point::new(0, "✅\t\t".len() as u32);
        let display_point = DisplayPoint::new(0, "✅       ".len() as u32);
        assert_eq!(point.to_display_point(&map, Left), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Left), point,);

        let point = Point::new(1, "β\t".len() as u32);
        let display_point = DisplayPoint::new(1, "β   ".len() as u32);
        assert_eq!(point.to_display_point(&map, Left), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Left), point,);

        let point = Point::new(2, "🏀β\t\t".len() as u32);
        let display_point = DisplayPoint::new(2, "🏀β      ".len() as u32);
        assert_eq!(point.to_display_point(&map, Left), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Left), point,);

        // Display points inside of expanded tabs
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Right),
            Point::new(0, "✅\t\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Left),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Right),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Left),
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
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaa\n\t\tbbb", cx));
        let map = cx.add_model(|cx| {
            DisplayMap::new(
                buffer.clone(),
                Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
                None,
                cx,
            )
        });
        assert_eq!(
            map.update(cx, |map, cx| map.snapshot(cx)).max_point(),
            DisplayPoint::new(1, 11)
        )
    }

    fn highlighted_chunks<'a>(
        rows: Range<u32>,
        map: &ModelHandle<DisplayMap>,
        theme: &'a Theme,
        cx: &mut MutableAppContext,
    ) -> Vec<(String, Option<&'a str>)> {
        let mut snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut chunks: Vec<(String, Option<&str>)> = Vec::new();
        for (chunk, style_id) in snapshot.highlighted_chunks_for_rows(rows) {
            let style_name = theme.syntax_style_name(style_id);
            if let Some((last_chunk, last_style_name)) = chunks.last_mut() {
                if style_name == *last_style_name {
                    last_chunk.push_str(chunk);
                } else {
                    chunks.push((chunk.to_string(), style_name));
                }
            } else {
                chunks.push((chunk.to_string(), style_name));
            }
        }
        chunks
    }
}
