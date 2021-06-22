mod fold_map;

use crate::settings::StyleId;

use super::{buffer, Anchor, Bias, Buffer, Edit, Point, ToOffset, ToPoint};
pub use fold_map::BufferRows;
use fold_map::{FoldMap, FoldMapSnapshot};
use gpui::{AppContext, ModelHandle};
use std::{mem, ops::Range};

pub struct DisplayMap {
    buffer: ModelHandle<Buffer>,
    fold_map: FoldMap,
    tab_size: usize,
}

impl DisplayMap {
    pub fn new(buffer: ModelHandle<Buffer>, tab_size: usize, cx: &AppContext) -> Self {
        DisplayMap {
            buffer: buffer.clone(),
            fold_map: FoldMap::new(buffer, cx),
            tab_size,
        }
    }

    pub fn snapshot(&self, cx: &AppContext) -> DisplayMapSnapshot {
        DisplayMapSnapshot {
            folds_snapshot: self.fold_map.snapshot(cx),
            tab_size: self.tab_size,
        }
    }

    pub fn folds_in_range<'a, T>(
        &'a self,
        range: Range<T>,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = &'a Range<Anchor>>
    where
        T: ToOffset,
    {
        self.fold_map.folds_in_range(range, cx)
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &AppContext,
    ) {
        self.fold_map.fold(ranges, cx)
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &AppContext,
    ) {
        self.fold_map.unfold(ranges, cx)
    }

    pub fn intersects_fold<T: ToOffset>(&self, offset: T, cx: &AppContext) -> bool {
        self.fold_map.intersects_fold(offset, cx)
    }

    pub fn is_line_folded(&self, display_row: u32, cx: &AppContext) -> bool {
        self.fold_map.is_line_folded(display_row, cx)
    }

    pub fn text(&self, cx: &AppContext) -> String {
        self.snapshot(cx).chunks_at(DisplayPoint::zero()).collect()
    }

    pub fn line(&self, display_row: u32, cx: &AppContext) -> String {
        let mut result = String::new();
        for chunk in self
            .snapshot(cx)
            .chunks_at(DisplayPoint::new(display_row, 0))
        {
            if let Some(ix) = chunk.find('\n') {
                result.push_str(&chunk[0..ix]);
                break;
            } else {
                result.push_str(chunk);
            }
        }
        result
    }

    pub fn line_indent(&self, display_row: u32, cx: &AppContext) -> (u32, bool) {
        let mut indent = 0;
        let mut is_blank = true;
        for c in self
            .snapshot(cx)
            .chars_at(DisplayPoint::new(display_row, 0))
        {
            if c == ' ' {
                indent += 1;
            } else {
                is_blank = c == '\n';
                break;
            }
        }
        (indent, is_blank)
    }

    pub fn line_len(&self, row: u32, cx: &AppContext) -> u32 {
        DisplayPoint::new(row, self.fold_map.line_len(row, cx))
            .expand_tabs(self, cx)
            .column()
    }

    pub fn max_point(&self, cx: &AppContext) -> DisplayPoint {
        self.snapshot(cx).max_point().expand_tabs(self, cx)
    }

    pub fn longest_row(&self, cx: &AppContext) -> u32 {
        self.fold_map.longest_row(cx)
    }

    pub fn anchor_before(&self, point: DisplayPoint, bias: Bias, cx: &AppContext) -> Anchor {
        self.buffer
            .read(cx)
            .anchor_before(point.to_buffer_point(self, bias, cx))
    }

    pub fn anchor_after(&self, point: DisplayPoint, bias: Bias, cx: &AppContext) -> Anchor {
        self.buffer
            .read(cx)
            .anchor_after(point.to_buffer_point(self, bias, cx))
    }
}

pub struct DisplayMapSnapshot {
    folds_snapshot: FoldMapSnapshot,
    tab_size: usize,
}

impl DisplayMapSnapshot {
    pub fn buffer_rows(&self, start_row: u32) -> BufferRows {
        self.folds_snapshot.buffer_rows(start_row)
    }

    pub fn max_point(&self) -> DisplayPoint {
        self.expand_tabs(self.folds_snapshot.max_point())
    }

    pub fn chunks_at(&self, point: DisplayPoint) -> Chunks {
        let (point, expanded_char_column, to_next_stop) = self.collapse_tabs(point, Bias::Left);
        let fold_chunks = self
            .folds_snapshot
            .chunks_at(self.folds_snapshot.to_display_offset(point));
        Chunks {
            fold_chunks,
            column: expanded_char_column,
            tab_size: self.tab_size,
            chunk: &SPACES[0..to_next_stop],
            skip_leading_tab: to_next_stop > 0,
        }
    }

    pub fn highlighted_chunks_for_rows(&mut self, rows: Range<u32>) -> HighlightedChunks {
        let start = DisplayPoint::new(rows.start, 0);
        let start = self.folds_snapshot.to_display_offset(start);
        let end = DisplayPoint::new(rows.end, 0).min(self.max_point());
        let end = self.folds_snapshot.to_display_offset(end);
        HighlightedChunks {
            fold_chunks: self.folds_snapshot.highlighted_chunks(start..end),
            column: 0,
            tab_size: self.tab_size,
            chunk: "",
            style_id: Default::default(),
        }
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
        self.expand_tabs(
            self.folds_snapshot
                .clip_point(self.collapse_tabs(point, bias).0, bias),
        )
    }

    fn expand_tabs(&self, mut point: DisplayPoint) -> DisplayPoint {
        let chars = self
            .folds_snapshot
            .chars_at(DisplayPoint(Point::new(point.row(), 0)));
        let expanded = expand_tabs(chars, point.column() as usize, self.tab_size);
        *point.column_mut() = expanded as u32;
        point
    }

    fn collapse_tabs(&self, mut point: DisplayPoint, bias: Bias) -> (DisplayPoint, usize, usize) {
        let chars = self
            .folds_snapshot
            .chars_at(DisplayPoint(Point::new(point.row(), 0)));
        let expanded = point.column() as usize;
        let (collapsed, expanded_char_column, to_next_stop) =
            collapse_tabs(chars, expanded, bias, self.tab_size);
        *point.column_mut() = collapsed as u32;
        (point, expanded_char_column, to_next_stop)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DisplayPoint(Point);

impl DisplayPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
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

    pub fn to_buffer_point(self, map: &DisplayMap, bias: Bias, cx: &AppContext) -> Point {
        map.fold_map
            .to_buffer_point(self.collapse_tabs(map, bias, cx), cx)
    }

    pub fn to_buffer_offset(self, map: &DisplayMap, bias: Bias, cx: &AppContext) -> usize {
        map.fold_map
            .to_buffer_offset(self.collapse_tabs(&map, bias, cx), cx)
    }

    fn expand_tabs(self, map: &DisplayMap, cx: &AppContext) -> Self {
        map.snapshot(cx).expand_tabs(self)
    }

    fn collapse_tabs(self, map: &DisplayMap, bias: Bias, cx: &AppContext) -> Self {
        map.snapshot(cx).collapse_tabs(self, bias).0
    }
}

impl Point {
    pub fn to_display_point(self, map: &DisplayMap, cx: &AppContext) -> DisplayPoint {
        let mut display_point = map.fold_map.to_display_point(self, cx);
        let snapshot = map.fold_map.snapshot(cx);
        let chars = snapshot.chars_at(DisplayPoint::new(display_point.row(), 0));
        *display_point.column_mut() =
            expand_tabs(chars, display_point.column() as usize, map.tab_size) as u32;
        display_point
    }
}

impl Anchor {
    pub fn to_display_point(&self, map: &DisplayMap, cx: &AppContext) -> DisplayPoint {
        self.to_point(map.buffer.read(cx)).to_display_point(map, cx)
    }
}

// Handles a tab width <= 16
const SPACES: &'static str = "                ";

pub struct Chunks<'a> {
    fold_chunks: fold_map::Chunks<'a>,
    chunk: &'a str,
    column: usize,
    tab_size: usize,
    skip_leading_tab: bool,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            if let Some(chunk) = self.fold_chunks.next() {
                self.chunk = chunk;
                if self.skip_leading_tab {
                    self.chunk = &self.chunk[1..];
                    self.skip_leading_tab = false;
                }
            } else {
                return None;
            }
        }

        for (ix, c) in self.chunk.char_indices() {
            match c {
                '\t' => {
                    if ix > 0 {
                        let (prefix, suffix) = self.chunk.split_at(ix);
                        self.chunk = suffix;
                        return Some(prefix);
                    } else {
                        self.chunk = &self.chunk[1..];
                        let len = self.tab_size - self.column % self.tab_size;
                        self.column += len;
                        return Some(&SPACES[0..len]);
                    }
                }
                '\n' => self.column = 0,
                _ => self.column += 1,
            }
        }

        let result = Some(self.chunk);
        self.chunk = "";
        result
    }
}

pub struct HighlightedChunks<'a> {
    fold_chunks: fold_map::HighlightedChunks<'a>,
    chunk: &'a str,
    style_id: StyleId,
    column: usize,
    tab_size: usize,
}

impl<'a> Iterator for HighlightedChunks<'a> {
    type Item = (&'a str, StyleId);

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            if let Some((chunk, style_id)) = self.fold_chunks.next() {
                self.chunk = chunk;
                self.style_id = style_id;
            } else {
                return None;
            }
        }

        for (ix, c) in self.chunk.char_indices() {
            match c {
                '\t' => {
                    if ix > 0 {
                        let (prefix, suffix) = self.chunk.split_at(ix);
                        self.chunk = suffix;
                        return Some((prefix, self.style_id));
                    } else {
                        self.chunk = &self.chunk[1..];
                        let len = self.tab_size - self.column % self.tab_size;
                        self.column += len;
                        return Some((&SPACES[0..len], self.style_id));
                    }
                }
                '\n' => self.column = 0,
                _ => self.column += 1,
            }
        }

        Some((mem::take(&mut self.chunk), mem::take(&mut self.style_id)))
    }
}

pub fn expand_tabs(chars: impl Iterator<Item = char>, column: usize, tab_size: usize) -> usize {
    let mut expanded_chars = 0;
    let mut expanded_bytes = 0;
    let mut collapsed_bytes = 0;
    for c in chars {
        if collapsed_bytes == column {
            break;
        }
        if c == '\t' {
            let tab_len = tab_size - expanded_chars % tab_size;
            expanded_bytes += tab_len;
            expanded_chars += tab_len;
        } else {
            expanded_bytes += c.len_utf8();
            expanded_chars += 1;
        }
        collapsed_bytes += c.len_utf8();
    }
    expanded_bytes
}

pub fn collapse_tabs(
    mut chars: impl Iterator<Item = char>,
    column: usize,
    bias: Bias,
    tab_size: usize,
) -> (usize, usize, usize) {
    let mut expanded_bytes = 0;
    let mut expanded_chars = 0;
    let mut collapsed_bytes = 0;
    while let Some(c) = chars.next() {
        if expanded_bytes >= column {
            break;
        }

        if c == '\t' {
            let tab_len = tab_size - (expanded_chars % tab_size);
            expanded_chars += tab_len;
            expanded_bytes += tab_len;
            if expanded_bytes > column {
                expanded_chars -= expanded_bytes - column;
                return match bias {
                    Bias::Left => (collapsed_bytes, expanded_chars, expanded_bytes - column),
                    Bias::Right => (collapsed_bytes + 1, expanded_chars, 0),
                };
            }
        } else {
            expanded_chars += 1;
            expanded_bytes += c.len_utf8();
        }

        if expanded_bytes > column && matches!(bias, Bias::Left) {
            expanded_chars -= 1;
            break;
        }

        collapsed_bytes += c.len_utf8();
    }
    (collapsed_bytes, expanded_chars, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        language::{Language, LanguageConfig},
        settings::Theme,
        test::*,
    };
    use buffer::History;
    use std::sync::Arc;

    #[gpui::test]
    fn test_chunks_at(cx: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6);
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let map = DisplayMap::new(buffer.clone(), 4, cx.as_ref());
        buffer
            .update(cx, |buffer, cx| {
                buffer.edit(
                    vec![
                        Point::new(1, 0)..Point::new(1, 0),
                        Point::new(1, 1)..Point::new(1, 1),
                        Point::new(2, 1)..Point::new(2, 1),
                    ],
                    "\t",
                    cx,
                )
            })
            .unwrap();

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

        let mut map = cx.read(|cx| DisplayMap::new(buffer, 2, cx));
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
        let map = DisplayMap::new(buffer.clone(), 4, cx);
        assert_eq!(map.text(cx), display_text);

        let map = map.snapshot(cx);
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

    #[test]
    fn test_expand_tabs() {
        assert_eq!(expand_tabs("\t".chars(), 0, 4), 0);
        assert_eq!(expand_tabs("\t".chars(), 1, 4), 4);
        assert_eq!(expand_tabs("\ta".chars(), 2, 4), 5);
    }

    #[gpui::test]
    fn test_tabs_with_multibyte_chars(cx: &mut gpui::MutableAppContext) {
        let text = "✅\t\tα\nβ\t\n🏀β\t\tγ";
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        let cx = cx.as_ref();
        let map = DisplayMap::new(buffer.clone(), 4, cx);
        assert_eq!(map.text(cx), "✅       α\nβ   \n🏀β      γ");

        let point = Point::new(0, "✅\t\t".len() as u32);
        let display_point = DisplayPoint::new(0, "✅       ".len() as u32);
        assert_eq!(point.to_display_point(&map, cx), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left, cx), point,);

        let point = Point::new(1, "β\t".len() as u32);
        let display_point = DisplayPoint::new(1, "β   ".len() as u32);
        assert_eq!(point.to_display_point(&map, cx), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left, cx), point,);

        let point = Point::new(2, "🏀β\t\t".len() as u32);
        let display_point = DisplayPoint::new(2, "🏀β      ".len() as u32);
        assert_eq!(point.to_display_point(&map, cx), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left, cx), point,);

        // Display points inside of expanded tabs
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Bias::Right, cx),
            Point::new(0, "✅\t\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Bias::Left, cx),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            map.snapshot(cx)
                .chunks_at(DisplayPoint::new(0, "✅      ".len() as u32))
                .collect::<String>(),
            " α\nβ   \n🏀β      γ"
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Bias::Right, cx),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Bias::Left, cx),
            Point::new(0, "✅".len() as u32),
        );
        assert_eq!(
            map.snapshot(cx)
                .chunks_at(DisplayPoint::new(0, "✅ ".len() as u32))
                .collect::<String>(),
            "      α\nβ   \n🏀β      γ"
        );

        // Clipping display points inside of multi-byte characters
        assert_eq!(
            map.snapshot(cx)
                .clip_point(DisplayPoint::new(0, "✅".len() as u32 - 1), Bias::Left),
            DisplayPoint::new(0, 0)
        );
        assert_eq!(
            map.snapshot(cx)
                .clip_point(DisplayPoint::new(0, "✅".len() as u32 - 1), Bias::Right),
            DisplayPoint::new(0, "✅".len() as u32)
        );
    }

    #[gpui::test]
    fn test_max_point(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "aaa\n\t\tbbb", cx));
        let map = DisplayMap::new(buffer.clone(), 4, cx.as_ref());
        assert_eq!(map.max_point(cx.as_ref()), DisplayPoint::new(1, 11))
    }
}
