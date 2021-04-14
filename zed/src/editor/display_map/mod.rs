mod fold_map;

use super::{buffer, Anchor, AnchorRangeExt, Buffer, Edit, Point, TextSummary, ToOffset, ToPoint};
use anyhow::Result;
pub use fold_map::BufferRows;
use fold_map::FoldMap;
use gpui::{AppContext, Entity, ModelContext, ModelHandle};
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

impl Entity for DisplayMap {
    type Event = ();
}

impl DisplayMap {
    pub fn new(buffer: ModelHandle<Buffer>, tab_size: usize, ctx: &mut ModelContext<Self>) -> Self {
        ctx.subscribe(&buffer, Self::handle_buffer_event);

        DisplayMap {
            buffer: buffer.clone(),
            fold_map: FoldMap::new(buffer, ctx.as_ref()),
            tab_size,
        }
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        ctx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.fold_map.fold(ranges, ctx.as_ref())?;
        ctx.notify();
        Ok(())
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        ctx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.fold_map.unfold(ranges, ctx.as_ref())?;
        ctx.notify();
        Ok(())
    }

    pub fn is_line_folded(&self, display_row: u32) -> bool {
        self.fold_map.is_line_folded(display_row)
    }

    pub fn text(&self, app: &AppContext) -> String {
        self.chars_at(DisplayPoint::zero(), app).unwrap().collect()
    }

    pub fn line(&self, display_row: u32, app: &AppContext) -> Result<String> {
        let chars = self.chars_at(DisplayPoint::new(display_row, 0), app)?;
        Ok(chars.take_while(|c| *c != '\n').collect())
    }

    pub fn chars_at<'a>(&'a self, point: DisplayPoint, app: &'a AppContext) -> Result<Chars<'a>> {
        let column = point.column() as usize;
        let (point, to_next_stop) = point.collapse_tabs(self, Bias::Left, app)?;
        let mut fold_chars = self.fold_map.chars_at(point, app)?;
        if to_next_stop > 0 {
            fold_chars.next();
        }

        Ok(Chars {
            fold_chars,
            column,
            to_next_stop,
            tab_size: self.tab_size,
        })
    }

    pub fn buffer_rows(&self, start_row: u32) -> Result<BufferRows> {
        self.fold_map.buffer_rows(start_row)
    }

    pub fn line_len(&self, row: u32, ctx: &AppContext) -> Result<u32> {
        DisplayPoint::new(row, self.fold_map.line_len(row, ctx)?)
            .expand_tabs(self, ctx)
            .map(|point| point.column())
    }

    pub fn max_point(&self, app: &AppContext) -> DisplayPoint {
        self.fold_map.max_point().expand_tabs(self, app).unwrap()
    }

    pub fn rightmost_point(&self) -> DisplayPoint {
        self.fold_map.rightmost_point()
    }

    pub fn anchor_before(
        &self,
        point: DisplayPoint,
        bias: Bias,
        app: &AppContext,
    ) -> Result<Anchor> {
        self.buffer
            .read(app)
            .anchor_before(point.to_buffer_point(self, bias, app)?)
    }

    pub fn anchor_after(
        &self,
        point: DisplayPoint,
        bias: Bias,
        app: &AppContext,
    ) -> Result<Anchor> {
        self.buffer
            .read(app)
            .anchor_after(point.to_buffer_point(self, bias, app)?)
    }

    fn handle_buffer_event(&mut self, event: &buffer::Event, ctx: &mut ModelContext<Self>) {
        match event {
            buffer::Event::Edited(edits) => self.fold_map.apply_edits(edits, ctx.as_ref()).unwrap(),
            _ => {}
        }
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

    pub fn to_buffer_point(self, map: &DisplayMap, bias: Bias, app: &AppContext) -> Result<Point> {
        Ok(map
            .fold_map
            .to_buffer_point(self.collapse_tabs(map, bias, app)?.0))
    }

    fn expand_tabs(mut self, map: &DisplayMap, app: &AppContext) -> Result<Self> {
        let chars = map
            .fold_map
            .chars_at(DisplayPoint(Point::new(self.row(), 0)), app)?;
        let expanded = expand_tabs(chars, self.column() as usize, map.tab_size);
        *self.column_mut() = expanded as u32;

        Ok(self)
    }

    fn collapse_tabs(
        mut self,
        map: &DisplayMap,
        bias: Bias,
        app: &AppContext,
    ) -> Result<(Self, usize)> {
        let chars = map
            .fold_map
            .chars_at(DisplayPoint(Point::new(self.0.row, 0)), app)?;
        let expanded = self.column() as usize;
        let (collapsed, to_next_stop) = collapse_tabs(chars, expanded, bias, map.tab_size);
        *self.column_mut() = collapsed as u32;

        Ok((self, to_next_stop))
    }
}

impl Point {
    pub fn to_display_point(self, map: &DisplayMap, app: &AppContext) -> Result<DisplayPoint> {
        let mut display_point = map.fold_map.to_display_point(self);
        let chars = map
            .fold_map
            .chars_at(DisplayPoint::new(display_point.row(), 0), app)?;
        *display_point.column_mut() =
            expand_tabs(chars, display_point.column() as usize, map.tab_size) as u32;
        Ok(display_point)
    }
}

impl Anchor {
    pub fn to_display_point(&self, map: &DisplayMap, app: &AppContext) -> Result<DisplayPoint> {
        self.to_point(map.buffer.read(app))?
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
    use gpui::App;

    #[test]
    fn test_chars_at() {
        App::test((), |app| {
            let text = sample_text(6, 6);
            let buffer = app.add_model(|_| Buffer::new(0, text));
            let map = app.add_model(|ctx| DisplayMap::new(buffer.clone(), 4, ctx));
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

            let map = map.read(app);
            assert_eq!(
                map.chars_at(DisplayPoint::new(1, 0), app.as_ref())
                    .unwrap()
                    .take(10)
                    .collect::<String>(),
                "    b   bb"
            );
            assert_eq!(
                map.chars_at(DisplayPoint::new(1, 2), app.as_ref())
                    .unwrap()
                    .take(10)
                    .collect::<String>(),
                "  b   bbbb"
            );
            assert_eq!(
                map.chars_at(DisplayPoint::new(1, 6), app.as_ref())
                    .unwrap()
                    .take(13)
                    .collect::<String>(),
                "  bbbbb\nc   c"
            );
        });
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

    #[test]
    fn test_max_point() {
        App::test((), |app| {
            let buffer = app.add_model(|_| Buffer::new(0, "aaa\n\t\tbbb"));
            let map = app.add_model(|ctx| DisplayMap::new(buffer.clone(), 4, ctx));
            assert_eq!(
                map.read(app).max_point(app.as_ref()),
                DisplayPoint::new(1, 11)
            )
        });
    }
}
