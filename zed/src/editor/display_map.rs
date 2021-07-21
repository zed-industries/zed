mod fold_map;
mod tab_map;
mod wrap_map;

use super::{buffer, Anchor, Bias, Buffer, Point, Settings, ToOffset, ToPoint};
use fold_map::FoldMap;
use gpui::{AppContext, ModelHandle};
use postage::prelude::Stream;
use std::ops::Range;
use tab_map::TabMap;
pub use wrap_map::BufferRows;
use wrap_map::WrapMap;

pub struct DisplayMap {
    buffer: ModelHandle<Buffer>,
    fold_map: FoldMap,
    tab_map: TabMap,
    wrap_map: WrapMap,
}

impl DisplayMap {
    pub fn new(
        buffer: ModelHandle<Buffer>,
        settings: Settings,
        wrap_width: Option<f32>,
        cx: &AppContext,
    ) -> Self {
        let (fold_map, snapshot) = FoldMap::new(buffer.clone(), cx);
        let (tab_map, snapshot) = TabMap::new(snapshot, settings.tab_size);
        let wrap_map = WrapMap::new(snapshot, settings, wrap_width, cx);
        DisplayMap {
            buffer,
            fold_map,
            tab_map,
            wrap_map,
        }
    }

    pub fn snapshot(&self, cx: &AppContext) -> DisplayMapSnapshot {
        let (folds_snapshot, edits) = self.fold_map.read(cx);
        let (tabs_snapshot, edits) = self.tab_map.sync(folds_snapshot.clone(), edits);
        let wraps_snapshot = self.wrap_map.sync(tabs_snapshot.clone(), edits, cx);
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
        cx: &AppContext,
    ) {
        let (mut fold_map, snapshot, edits) = self.fold_map.write(cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map.sync(snapshot, edits, cx);
        let (snapshot, edits) = fold_map.fold(ranges, cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map.sync(snapshot, edits, cx);
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &AppContext,
    ) {
        let (mut fold_map, snapshot, edits) = self.fold_map.write(cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map.sync(snapshot, edits, cx);
        let (snapshot, edits) = fold_map.unfold(ranges, cx);
        let (snapshot, edits) = self.tab_map.sync(snapshot, edits);
        self.wrap_map.sync(snapshot, edits, cx);
    }

    pub fn set_wrap_width(&self, width: Option<f32>) {
        self.wrap_map.set_wrap_width(width);
    }

    pub fn notifications(&self) -> impl Stream<Item = ()> {
        self.wrap_map.notifications()
    }
}

pub struct DisplayMapSnapshot {
    buffer_snapshot: buffer::Snapshot,
    folds_snapshot: fold_map::Snapshot,
    tabs_snapshot: tab_map::Snapshot,
    wraps_snapshot: wrap_map::Snapshot,
}

impl DisplayMapSnapshot {
    pub fn buffer_rows(&self, start_row: u32) -> BufferRows {
        self.wraps_snapshot.buffer_rows(start_row)
    }

    pub fn max_point(&self) -> DisplayPoint {
        DisplayPoint(self.wraps_snapshot.max_point())
    }

    pub fn chunks_at(&self, point: DisplayPoint) -> wrap_map::Chunks {
        self.wraps_snapshot.chunks_at(point.0)
    }

    pub fn highlighted_chunks_for_rows(&mut self, rows: Range<u32>) -> wrap_map::HighlightedChunks {
        self.wraps_snapshot.highlighted_chunks_for_rows(rows)
    }

    pub fn chars_at<'a>(&'a self, point: DisplayPoint) -> impl Iterator<Item = char> + 'a {
        self.chunks_at(point).flat_map(str::chars)
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
        self.folds_snapshot.is_line_folded(display_row)
    }

    pub fn text(&self) -> String {
        self.chunks_at(DisplayPoint::zero()).collect()
    }

    pub fn line(&self, display_row: u32) -> String {
        let mut result = String::new();
        for chunk in self.chunks_at(DisplayPoint::new(display_row, 0)) {
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
        self.tabs_snapshot.line_len(row)
    }

    pub fn longest_row(&self) -> u32 {
        self.tabs_snapshot.longest_row()
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
pub struct DisplayPoint(wrap_map::OutputPoint);

impl DisplayPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(wrap_map::OutputPoint::new(row, column))
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
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
        let unwrapped_point = map.wraps_snapshot.to_input_point(self.0);
        let unexpanded_point = map.tabs_snapshot.to_input_point(unwrapped_point, bias).0;
        map.folds_snapshot.to_input_point(unexpanded_point)
    }

    pub fn to_buffer_offset(self, map: &DisplayMapSnapshot, bias: Bias) -> usize {
        let unwrapped_point = map.wraps_snapshot.to_input_point(self.0);
        let unexpanded_point = map.tabs_snapshot.to_input_point(unwrapped_point, bias).0;
        map.folds_snapshot.to_input_offset(unexpanded_point)
    }
}

impl Point {
    pub fn to_display_point(self, map: &DisplayMapSnapshot) -> DisplayPoint {
        let folded_point = map.folds_snapshot.to_output_point(self);
        let expanded_point = map.tabs_snapshot.to_output_point(folded_point);
        let wrapped_point = map.wraps_snapshot.to_output_point(expanded_point);
        DisplayPoint(wrapped_point)
    }
}

impl Anchor {
    pub fn to_display_point(&self, map: &DisplayMapSnapshot) -> DisplayPoint {
        self.to_point(&map.buffer_snapshot).to_display_point(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        language::{Language, LanguageConfig},
        settings::Theme,
        test::*,
        util::RandomCharIter,
    };
    use buffer::History;
    use rand::prelude::*;
    use std::{env, sync::Arc};

    #[gpui::test]
    async fn test_random(mut cx: gpui::TestAppContext) {
        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);
        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().expect("invalid `ITERATIONS` variable"))
            .unwrap_or(100);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let seed_range = if let Ok(seed) = env::var("SEED") {
            let seed = seed.parse().expect("invalid `SEED` variable");
            seed..seed + 1
        } else {
            0..iterations
        };
        let font_cache = cx.font_cache();

        for seed in seed_range {
            dbg!(seed);
            let mut rng = StdRng::seed_from_u64(seed);
            let settings = Settings {
                buffer_font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
                ui_font_family: font_cache.load_family(&["Helvetica"]).unwrap(),
                buffer_font_size: 12.0,
                ui_font_size: 12.0,
                tab_size: rng.gen_range(1..=4),
                theme: Arc::new(Theme::default()),
            };

            let buffer = cx.add_model(|cx| {
                let len = rng.gen_range(0..10);
                let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                log::info!("Initial buffer text: {:?}", text);
                Buffer::new(0, text, cx)
            });
            let wrap_width = Some(rng.gen_range(20.0..=100.0));
            let map = cx.read(|cx| DisplayMap::new(buffer.clone(), settings, wrap_width, cx));

            for _op_ix in 0..operations {
                buffer.update(&mut cx, |buffer, cx| buffer.randomly_mutate(&mut rng, cx));
                let snapshot = cx.read(|cx| map.snapshot(cx));
                let expected_buffer_rows = (0..=snapshot.max_point().row())
                    .map(|display_row| {
                        DisplayPoint::new(display_row, 0)
                            .to_buffer_point(&snapshot, Bias::Left)
                            .row
                    })
                    .collect::<Vec<_>>();
                for start_display_row in 0..expected_buffer_rows.len() {
                    assert_eq!(
                        snapshot
                            .buffer_rows(start_display_row as u32)
                            .collect::<Vec<_>>(),
                        &expected_buffer_rows[start_display_row..],
                        "invalid buffer_rows({}..)",
                        start_display_row
                    );
                }
            }
        }
    }

    #[gpui::test]
    async fn test_soft_wraps(mut cx: gpui::TestAppContext) {
        cx.foreground().set_block_on_ticks(usize::MAX..=usize::MAX);

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
        let map = cx.read(|cx| DisplayMap::new(buffer.clone(), settings, wrap_width, cx));

        let snapshot = cx.read(|cx| map.snapshot(cx));
        assert_eq!(
            snapshot
                .chunks_at(DisplayPoint::new(0, 3))
                .collect::<String>(),
            " two \nthree four \nfive\nsix seven \neight"
        );

        buffer.update(&mut cx, |buffer, cx| {
            let ix = buffer.text().find("seven").unwrap();
            buffer.edit(vec![ix..ix], "and ", cx);
        });

        let snapshot = cx.read(|cx| map.snapshot(cx));
        assert_eq!(
            snapshot
                .chunks_at(DisplayPoint::new(1, 0))
                .collect::<String>(),
            "three four \nfive\nsix and \nseven eight"
        );
    }

    #[gpui::test]
    fn test_chunks_at(cx: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6);
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let map = DisplayMap::new(
            buffer.clone(),
            Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
            None,
            cx.as_ref(),
        );
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
            &map.snapshot(cx.as_ref())
                .chunks_at(DisplayPoint::new(1, 0))
                .collect::<String>()[0..10],
            "    b   bb"
        );
        assert_eq!(
            &map.snapshot(cx.as_ref())
                .chunks_at(DisplayPoint::new(1, 2))
                .collect::<String>()[0..10],
            "  b   bbbb"
        );
        assert_eq!(
            &map.snapshot(cx.as_ref())
                .chunks_at(DisplayPoint::new(1, 6))
                .collect::<String>()[0..13],
            "  bbbbb\nc   c"
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

        let mut map = cx.read(|cx| {
            DisplayMap::new(
                buffer,
                Settings::new(cx.font_cache()).unwrap().with_tab_size(2),
                None,
                cx,
            )
        });
        assert_eq!(
            cx.read(|cx| highlighted_chunks(0..5, &map, &theme, cx)),
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
            cx.read(|cx| highlighted_chunks(3..5, &map, &theme, cx)),
            vec![
                ("    fn ".to_string(), Some("mod.body")),
                ("inner".to_string(), Some("fn.name")),
                ("() {}\n}".to_string(), Some("mod.body")),
            ]
        );

        cx.read(|cx| map.fold(vec![Point::new(0, 6)..Point::new(3, 2)], cx));
        assert_eq!(
            cx.read(|cx| highlighted_chunks(0..2, &map, &theme, cx)),
            vec![
                ("fn ".to_string(), None),
                ("out".to_string(), Some("fn.name")),
                ("…".to_string(), None),
                ("  fn ".to_string(), Some("mod.body")),
                ("inner".to_string(), Some("fn.name")),
                ("() {}\n}".to_string(), Some("mod.body")),
            ]
        );

        fn highlighted_chunks<'a>(
            rows: Range<u32>,
            map: &DisplayMap,
            theme: &'a Theme,
            cx: &AppContext,
        ) -> Vec<(String, Option<&'a str>)> {
            let mut chunks: Vec<(String, Option<&str>)> = Vec::new();
            for (chunk, style_id) in map.snapshot(cx).highlighted_chunks_for_rows(rows) {
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

    #[gpui::test]
    fn test_clip_point(cx: &mut gpui::MutableAppContext) {
        use Bias::{Left, Right};

        let text = "\n'a', 'α',\t'✋',\t'❎', '🍐'\n";
        let display_text = "\n'a', 'α',   '✋',    '❎', '🍐'\n";
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let cx = cx.as_ref();
        let map = DisplayMap::new(
            buffer.clone(),
            Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
            None,
            cx,
        );
        let map = map.snapshot(cx);

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
        let cx = cx.as_ref();
        let map = DisplayMap::new(
            buffer.clone(),
            Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
            None,
            cx,
        );
        let map = map.snapshot(cx);
        assert_eq!(map.text(), "✅       α\nβ   \n🏀β      γ");

        let point = Point::new(0, "✅\t\t".len() as u32);
        let display_point = DisplayPoint::new(0, "✅       ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left), point,);

        let point = Point::new(1, "β\t".len() as u32);
        let display_point = DisplayPoint::new(1, "β   ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left), point,);

        let point = Point::new(2, "🏀β\t\t".len() as u32);
        let display_point = DisplayPoint::new(2, "🏀β      ".len() as u32);
        assert_eq!(point.to_display_point(&map), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left), point,);

        // Display points inside of expanded tabs
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Bias::Right),
            Point::new(0, "✅\t\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Bias::Left),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            map.chunks_at(DisplayPoint::new(0, "✅      ".len() as u32))
                .collect::<String>(),
            " α\nβ   \n🏀β      γ"
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Bias::Right),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Bias::Left),
            Point::new(0, "✅".len() as u32),
        );
        assert_eq!(
            map.chunks_at(DisplayPoint::new(0, "✅ ".len() as u32))
                .collect::<String>(),
            "      α\nβ   \n🏀β      γ"
        );

        // Clipping display points inside of multi-byte characters
        assert_eq!(
            map.clip_point(DisplayPoint::new(0, "✅".len() as u32 - 1), Bias::Left),
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
        let map = DisplayMap::new(
            buffer.clone(),
            Settings::new(cx.font_cache()).unwrap().with_tab_size(4),
            None,
            cx.as_ref(),
        );
        assert_eq!(
            map.snapshot(cx.as_ref()).max_point(),
            DisplayPoint::new(1, 11)
        )
    }
}
