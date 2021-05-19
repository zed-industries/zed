mod fold_map;

use super::{buffer, Anchor, Buffer, Edit, Point, ToOffset, ToPoint};
pub use fold_map::BufferRows;
use fold_map::{FoldMap, FoldMapSnapshot};
use gpui::{AppContext, ModelHandle};
use std::ops::Range;

#[derive(Copy, Clone)]
pub enum Bias {
    Left,
    Right,
}

pub struct DisplayMap {
    buffer: ModelHandle<Buffer>,
    fold_map: FoldMap,
    tab_size: usize,
}

impl DisplayMap {
    pub fn new(buffer: ModelHandle<Buffer>, tab_size: usize, ctx: &AppContext) -> Self {
        DisplayMap {
            buffer: buffer.clone(),
            fold_map: FoldMap::new(buffer, ctx),
            tab_size,
        }
    }

    pub fn snapshot(&self, ctx: &AppContext) -> DisplayMapSnapshot {
        DisplayMapSnapshot {
            folds_snapshot: self.fold_map.snapshot(ctx),
            tab_size: self.tab_size,
        }
    }

    pub fn folds_in_range<'a, T>(
        &'a self,
        range: Range<T>,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a Range<Anchor>>
    where
        T: ToOffset,
    {
        self.fold_map.folds_in_range(range, app)
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        ctx: &AppContext,
    ) {
        self.fold_map.fold(ranges, ctx)
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        ctx: &AppContext,
    ) {
        self.fold_map.unfold(ranges, ctx)
    }

    pub fn is_line_folded(&self, display_row: u32, ctx: &AppContext) -> bool {
        self.fold_map.is_line_folded(display_row, ctx)
    }

    pub fn text(&self, ctx: &AppContext) -> String {
        self.snapshot(ctx)
            .chunks_at(DisplayPoint::zero(), ctx)
            .collect()
    }

    pub fn line(&self, display_row: u32, ctx: &AppContext) -> String {
        let mut result = String::new();
        for chunk in self
            .snapshot(ctx)
            .chunks_at(DisplayPoint::new(display_row, 0), ctx)
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

    pub fn line_indent(&self, display_row: u32, ctx: &AppContext) -> (u32, bool) {
        let mut indent = 0;
        let mut is_blank = true;
        for c in self
            .snapshot(ctx)
            .chars_at(DisplayPoint::new(display_row, 0), ctx)
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

    pub fn line_len(&self, row: u32, ctx: &AppContext) -> u32 {
        DisplayPoint::new(row, self.fold_map.line_len(row, ctx))
            .expand_tabs(self, ctx)
            .column()
    }

    pub fn max_point(&self, ctx: &AppContext) -> DisplayPoint {
        self.fold_map.max_point(ctx).expand_tabs(self, ctx)
    }

    pub fn rightmost_point(&self, ctx: &AppContext) -> DisplayPoint {
        self.fold_map.rightmost_point(ctx)
    }

    pub fn anchor_before(&self, point: DisplayPoint, bias: Bias, app: &AppContext) -> Anchor {
        self.buffer
            .read(app)
            .anchor_before(point.to_buffer_point(self, bias, app))
    }

    pub fn anchor_after(&self, point: DisplayPoint, bias: Bias, app: &AppContext) -> Anchor {
        self.buffer
            .read(app)
            .anchor_after(point.to_buffer_point(self, bias, app))
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

    pub fn chunks_at<'a>(&'a self, point: DisplayPoint, app: &'a AppContext) -> Chunks<'a> {
        let (point, expanded_char_column, to_next_stop) =
            self.collapse_tabs(point, Bias::Left, app);
        let fold_chunks = self
            .folds_snapshot
            .chunks_at(self.folds_snapshot.to_display_offset(point, app), app);
        Chunks {
            fold_chunks,
            column: expanded_char_column,
            tab_size: self.tab_size,
            chunk: &SPACES[0..to_next_stop],
            skip_leading_tab: to_next_stop > 0,
        }
    }

    pub fn chars_at<'a>(
        &'a self,
        point: DisplayPoint,
        app: &'a AppContext,
    ) -> impl Iterator<Item = char> + 'a {
        self.chunks_at(point, app).flat_map(str::chars)
    }

    pub fn column_to_chars(&self, display_row: u32, target: u32, ctx: &AppContext) -> u32 {
        let mut count = 0;
        let mut column = 0;
        for c in self.chars_at(DisplayPoint::new(display_row, 0), ctx) {
            count += 1;
            column += c.len_utf8() as u32;
            if column >= target {
                break;
            }
        }
        count
    }

    pub fn column_from_chars(&self, display_row: u32, char_count: u32, ctx: &AppContext) -> u32 {
        let mut count = 0;
        let mut column = 0;
        for c in self.chars_at(DisplayPoint::new(display_row, 0), ctx) {
            count += 1;
            column += c.len_utf8() as u32;
            if count >= char_count {
                break;
            }
        }
        column
    }

    fn expand_tabs(&self, mut point: DisplayPoint, ctx: &AppContext) -> DisplayPoint {
        let chars = self
            .folds_snapshot
            .chars_at(DisplayPoint(Point::new(point.row(), 0)), ctx);
        let expanded = expand_tabs(chars, point.column() as usize, self.tab_size);
        *point.column_mut() = expanded as u32;
        point
    }

    fn collapse_tabs(
        &self,
        mut point: DisplayPoint,
        bias: Bias,
        ctx: &AppContext,
    ) -> (DisplayPoint, usize, usize) {
        let chars = self
            .folds_snapshot
            .chars_at(DisplayPoint(Point::new(point.row(), 0)), ctx);
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

    pub fn to_buffer_point(self, map: &DisplayMap, bias: Bias, ctx: &AppContext) -> Point {
        map.fold_map
            .to_buffer_point(self.collapse_tabs(map, bias, ctx), ctx)
    }

    pub fn to_buffer_offset(self, map: &DisplayMap, bias: Bias, ctx: &AppContext) -> usize {
        map.fold_map
            .to_buffer_offset(self.collapse_tabs(&map, bias, ctx), ctx)
    }

    fn expand_tabs(self, map: &DisplayMap, ctx: &AppContext) -> Self {
        map.snapshot(ctx).expand_tabs(self, ctx)
    }

    fn collapse_tabs(self, map: &DisplayMap, bias: Bias, ctx: &AppContext) -> Self {
        map.snapshot(ctx).collapse_tabs(self, bias, ctx).0
    }
}

impl Point {
    pub fn to_display_point(self, map: &DisplayMap, ctx: &AppContext) -> DisplayPoint {
        let mut display_point = map.fold_map.to_display_point(self, ctx);
        let snapshot = map.fold_map.snapshot(ctx);
        let chars = snapshot.chars_at(DisplayPoint::new(display_point.row(), 0), ctx);
        *display_point.column_mut() =
            expand_tabs(chars, display_point.column() as usize, map.tab_size) as u32;
        display_point
    }
}

impl Anchor {
    pub fn to_display_point(&self, map: &DisplayMap, app: &AppContext) -> DisplayPoint {
        self.to_point(map.buffer.read(app))
            .to_display_point(map, app)
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
        if expanded_bytes == column {
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
        collapsed_bytes += c.len_utf8();

        if expanded_bytes > column {
            panic!("column {} is inside of character {:?}", column, c);
        }
    }
    (collapsed_bytes, expanded_chars, 0)
}

#[cfg(test)]
mod tests {
    use gpui::MutableAppContext;

    use super::*;
    use crate::test::*;

    #[gpui::test]
    fn test_chunks_at(app: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6);
        let buffer = app.add_model(|ctx| Buffer::new(0, text, ctx));
        let map = DisplayMap::new(buffer.clone(), 4, app.as_ref());
        buffer
            .update(app, |buffer, ctx| {
                buffer.edit(
                    vec![
                        Point::new(1, 0)..Point::new(1, 0),
                        Point::new(1, 1)..Point::new(1, 1),
                        Point::new(2, 1)..Point::new(2, 1),
                    ],
                    "\t",
                    Some(ctx),
                )
            })
            .unwrap();

        assert_eq!(
            &map.snapshot(app.as_ref())
                .chunks_at(DisplayPoint::new(1, 0), app.as_ref())
                .collect::<String>()[0..10],
            "    b   bb"
        );
        assert_eq!(
            &map.snapshot(app.as_ref())
                .chunks_at(DisplayPoint::new(1, 2), app.as_ref())
                .collect::<String>()[0..10],
            "  b   bbbb"
        );
        assert_eq!(
            &map.snapshot(app.as_ref())
                .chunks_at(DisplayPoint::new(1, 6), app.as_ref())
                .collect::<String>()[0..13],
            "  bbbbb\nc   c"
        );
    }

    #[test]
    fn test_expand_tabs() {
        assert_eq!(expand_tabs("\t".chars(), 0, 4), 0);
        assert_eq!(expand_tabs("\t".chars(), 1, 4), 4);
        assert_eq!(expand_tabs("\ta".chars(), 2, 4), 5);
    }

    #[gpui::test]
    fn test_tabs_with_multibyte_chars(app: &mut MutableAppContext) {
        let text = "✅\t\tx\nα\t\n🏀α\t\ty";
        let buffer = app.add_model(|ctx| Buffer::new(0, text, ctx));
        let ctx = app.as_ref();
        let map = DisplayMap::new(buffer.clone(), 4, ctx);
        assert_eq!(map.text(ctx), "✅       x\nα   \n🏀α      y");

        let point = Point::new(0, "✅\t\t".len() as u32);
        let display_point = DisplayPoint::new(0, "✅       ".len() as u32);
        assert_eq!(point.to_display_point(&map, ctx), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left, ctx), point,);

        let point = Point::new(1, "α\t".len() as u32);
        let display_point = DisplayPoint::new(1, "α   ".len() as u32);
        assert_eq!(point.to_display_point(&map, ctx), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left, ctx), point,);

        let point = Point::new(2, "🏀α\t\t".len() as u32);
        let display_point = DisplayPoint::new(2, "🏀α      ".len() as u32);
        assert_eq!(point.to_display_point(&map, ctx), display_point);
        assert_eq!(display_point.to_buffer_point(&map, Bias::Left, ctx), point,);

        // Display points inside of expanded tabs
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Bias::Right, ctx),
            Point::new(0, "✅\t\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅      ".len() as u32).to_buffer_point(&map, Bias::Left, ctx),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            map.snapshot(ctx)
                .chunks_at(DisplayPoint::new(0, "✅      ".len() as u32), ctx)
                .collect::<String>(),
            " x\nα   \n🏀α      y"
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Bias::Right, ctx),
            Point::new(0, "✅\t".len() as u32),
        );
        assert_eq!(
            DisplayPoint::new(0, "✅ ".len() as u32).to_buffer_point(&map, Bias::Left, ctx),
            Point::new(0, "✅".len() as u32),
        );
        assert_eq!(
            map.snapshot(ctx)
                .chunks_at(DisplayPoint::new(0, "✅ ".len() as u32), ctx)
                .collect::<String>(),
            "      x\nα   \n🏀α      y"
        );
    }

    #[gpui::test]
    fn test_max_point(app: &mut gpui::MutableAppContext) {
        let buffer = app.add_model(|ctx| Buffer::new(0, "aaa\n\t\tbbb", ctx));
        let map = DisplayMap::new(buffer.clone(), 4, app.as_ref());
        assert_eq!(map.max_point(app.as_ref()), DisplayPoint::new(1, 11))
    }
}
