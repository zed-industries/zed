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
            .chars_at(DisplayPoint::zero(), ctx)
            .collect()
    }

    pub fn line(&self, display_row: u32, ctx: &AppContext) -> String {
        self.snapshot(ctx)
            .chars_at(DisplayPoint::new(display_row, 0), ctx)
            .take_while(|c| *c != '\n')
            .collect()
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

    pub fn chars_at<'a>(&'a self, point: DisplayPoint, app: &'a AppContext) -> Chars<'a> {
        let column = point.column() as usize;
        let (point, to_next_stop) = self.collapse_tabs(point, Bias::Left, app);
        let mut fold_chars = self.folds_snapshot.chars_at(point, app);
        if to_next_stop > 0 {
            fold_chars.next();
        }
        Chars {
            fold_chars,
            column,
            to_next_stop,
            tab_size: self.tab_size,
        }
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
    ) -> (DisplayPoint, usize) {
        let chars = self
            .folds_snapshot
            .chars_at(DisplayPoint(Point::new(point.row(), 0)), ctx);
        let expanded = point.column() as usize;
        let (collapsed, to_next_stop) = collapse_tabs(chars, expanded, bias, self.tab_size);
        *point.column_mut() = collapsed as u32;

        (point, to_next_stop)
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
            .to_buffer_point(self.collapse_tabs(map, bias, ctx).0, ctx)
    }

    pub fn to_buffer_offset(self, map: &DisplayMap, bias: Bias, ctx: &AppContext) -> usize {
        map.fold_map
            .to_buffer_offset(self.collapse_tabs(&map, bias, ctx).0, ctx)
    }

    fn expand_tabs(self, map: &DisplayMap, ctx: &AppContext) -> Self {
        map.snapshot(ctx).expand_tabs(self, ctx)
    }

    fn collapse_tabs(self, map: &DisplayMap, bias: Bias, ctx: &AppContext) -> (Self, usize) {
        map.snapshot(ctx).collapse_tabs(self, bias, ctx)
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

pub struct Chars<'a> {
    fold_chars: fold_map::Chars<'a>,
    column: usize,
    to_next_stop: usize,
    tab_size: usize,
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_next_stop > 0 {
            self.to_next_stop -= 1;
            self.column += 1;
            Some(' ')
        } else {
            self.fold_chars.next().map(|c| match c {
                '\t' => {
                    self.to_next_stop = self.tab_size - self.column % self.tab_size - 1;
                    self.column += 1;
                    ' '
                }
                '\n' => {
                    self.column = 0;
                    c
                }
                _ => {
                    self.column += 1;
                    c
                }
            })
        }
    }
}

pub fn expand_tabs(chars: impl Iterator<Item = char>, column: usize, tab_size: usize) -> usize {
    let mut expanded = 0;
    for c in chars.take(column) {
        if c == '\t' {
            expanded += tab_size - expanded % tab_size;
        } else {
            expanded += 1;
        }
    }
    expanded
}

pub fn collapse_tabs(
    mut chars: impl Iterator<Item = char>,
    column: usize,
    bias: Bias,
    tab_size: usize,
) -> (usize, usize) {
    let mut expanded = 0;
    let mut collapsed = 0;
    while let Some(c) = chars.next() {
        if expanded == column {
            break;
        }

        if c == '\t' {
            expanded += tab_size - (expanded % tab_size);
            if expanded > column {
                return match bias {
                    Bias::Left => (collapsed, expanded - column),
                    Bias::Right => (collapsed + 1, 0),
                };
            }
            collapsed += 1;
        } else {
            expanded += 1;
            collapsed += 1;
        }
    }
    (collapsed, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[gpui::test]
    fn test_chars_at(app: &mut gpui::MutableAppContext) {
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
            map.snapshot(app.as_ref())
                .chars_at(DisplayPoint::new(1, 0), app.as_ref())
                .take(10)
                .collect::<String>(),
            "    b   bb"
        );
        assert_eq!(
            map.snapshot(app.as_ref())
                .chars_at(DisplayPoint::new(1, 2), app.as_ref())
                .take(10)
                .collect::<String>(),
            "  b   bbbb"
        );
        assert_eq!(
            map.snapshot(app.as_ref())
                .chars_at(DisplayPoint::new(1, 6), app.as_ref())
                .take(13)
                .collect::<String>(),
            "  bbbbb\nc   c"
        );
    }

    #[test]
    fn test_expand_tabs() {
        assert_eq!(expand_tabs("\t".chars(), 0, 4), 0);
        assert_eq!(expand_tabs("\t".chars(), 1, 4), 4);
        assert_eq!(expand_tabs("\ta".chars(), 2, 4), 5);
    }

    #[test]
    fn test_collapse_tabs() {
        assert_eq!(collapse_tabs("\t".chars(), 0, Bias::Left, 4), (0, 0));
        assert_eq!(collapse_tabs("\t".chars(), 0, Bias::Right, 4), (0, 0));
        assert_eq!(collapse_tabs("\t".chars(), 1, Bias::Left, 4), (0, 3));
        assert_eq!(collapse_tabs("\t".chars(), 1, Bias::Right, 4), (1, 0));
        assert_eq!(collapse_tabs("\t".chars(), 2, Bias::Left, 4), (0, 2));
        assert_eq!(collapse_tabs("\t".chars(), 2, Bias::Right, 4), (1, 0));
        assert_eq!(collapse_tabs("\t".chars(), 3, Bias::Left, 4), (0, 1));
        assert_eq!(collapse_tabs("\t".chars(), 3, Bias::Right, 4), (1, 0));
        assert_eq!(collapse_tabs("\t".chars(), 4, Bias::Left, 4), (1, 0));
        assert_eq!(collapse_tabs("\t".chars(), 4, Bias::Right, 4), (1, 0));
        assert_eq!(collapse_tabs("\ta".chars(), 5, Bias::Left, 4), (2, 0));
        assert_eq!(collapse_tabs("\ta".chars(), 5, Bias::Right, 4), (2, 0));
    }

    #[gpui::test]
    fn test_max_point(app: &mut gpui::MutableAppContext) {
        let buffer = app.add_model(|ctx| Buffer::new(0, "aaa\n\t\tbbb", ctx));
        let map = DisplayMap::new(buffer.clone(), 4, app.as_ref());
        assert_eq!(map.max_point(app.as_ref()), DisplayPoint::new(1, 11))
    }
}
