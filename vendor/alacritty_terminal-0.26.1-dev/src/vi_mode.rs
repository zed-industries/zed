use std::cmp::min;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::event::EventListener;
use crate::grid::{Dimensions, GridCell};
use crate::index::{Boundary, Column, Direction, Line, Point, Side};
use crate::term::Term;
use crate::term::cell::Flags;

/// Possible vi mode motion movements.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "lowercase"))]
pub enum ViMotion {
    /// Move up.
    Up,
    /// Move down.
    Down,
    /// Move left.
    Left,
    /// Move right.
    Right,
    /// First column, or beginning of the line when already at the first column.
    First,
    /// Last column, or beginning of the line when already at the last column.
    Last,
    /// First non-empty cell in this terminal row, or first non-empty cell
    /// of the line when already at the first cell of the row.
    FirstOccupied,
    /// Move to top of screen.
    High,
    /// Move to center of screen.
    Middle,
    /// Move to bottom of screen.
    Low,
    /// Move to start of semantically separated word.
    SemanticLeft,
    /// Move to start of next semantically separated word.
    SemanticRight,
    /// Move to end of previous semantically separated word.
    SemanticLeftEnd,
    /// Move to end of semantically separated word.
    SemanticRightEnd,
    /// Move to start of whitespace separated word.
    WordLeft,
    /// Move to start of next whitespace separated word.
    WordRight,
    /// Move to end of previous whitespace separated word.
    WordLeftEnd,
    /// Move to end of whitespace separated word.
    WordRightEnd,
    /// Move to opposing bracket.
    Bracket,
    /// Move above the current paragraph.
    ParagraphUp,
    /// Move below the current paragraph.
    ParagraphDown,
}

/// Cursor tracking vi mode position.
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ViModeCursor {
    pub point: Point,
}

impl ViModeCursor {
    pub fn new(point: Point) -> Self {
        Self { point }
    }

    /// Move vi mode cursor.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn motion<T: EventListener>(mut self, term: &mut Term<T>, motion: ViMotion) -> Self {
        match motion {
            ViMotion::Up => {
                if self.point.line > term.topmost_line() {
                    self.point.line -= 1;
                }
            },
            ViMotion::Down => {
                if self.point.line + 1 < term.screen_lines() as i32 {
                    self.point.line += 1;
                }
            },
            ViMotion::Left => {
                self.point = term.expand_wide(self.point, Direction::Left);
                let wrap_point = Point::new(self.point.line - 1, term.last_column());
                if self.point.column == 0
                    && self.point.line > term.topmost_line()
                    && is_wrap(term, wrap_point)
                {
                    self.point = wrap_point;
                } else {
                    self.point.column = Column(self.point.column.saturating_sub(1));
                }
            },
            ViMotion::Right => {
                self.point = term.expand_wide(self.point, Direction::Right);
                if is_wrap(term, self.point) {
                    self.point = Point::new(self.point.line + 1, Column(0));
                } else {
                    self.point.column = min(self.point.column + 1, term.last_column());
                }
            },
            ViMotion::First => {
                self.point = term.expand_wide(self.point, Direction::Left);
                while self.point.column == 0
                    && self.point.line > term.topmost_line()
                    && is_wrap(term, Point::new(self.point.line - 1, term.last_column()))
                {
                    self.point.line -= 1;
                }
                self.point.column = Column(0);
            },
            ViMotion::Last => self.point = last(term, self.point),
            ViMotion::FirstOccupied => self.point = first_occupied(term, self.point),
            ViMotion::High => {
                let line = Line(-(term.grid().display_offset() as i32));
                let col = first_occupied_in_line(term, line).unwrap_or_default().column;
                self.point = Point::new(line, col);
            },
            ViMotion::Middle => {
                let display_offset = term.grid().display_offset() as i32;
                let line = Line(-display_offset + term.screen_lines() as i32 / 2 - 1);
                let col = first_occupied_in_line(term, line).unwrap_or_default().column;
                self.point = Point::new(line, col);
            },
            ViMotion::Low => {
                let display_offset = term.grid().display_offset() as i32;
                let line = Line(-display_offset + term.screen_lines() as i32 - 1);
                let col = first_occupied_in_line(term, line).unwrap_or_default().column;
                self.point = Point::new(line, col);
            },
            ViMotion::SemanticLeft => {
                self.point = semantic(term, self.point, Direction::Left, Side::Left);
            },
            ViMotion::SemanticRight => {
                self.point = semantic(term, self.point, Direction::Right, Side::Left);
            },
            ViMotion::SemanticLeftEnd => {
                self.point = semantic(term, self.point, Direction::Left, Side::Right);
            },
            ViMotion::SemanticRightEnd => {
                self.point = semantic(term, self.point, Direction::Right, Side::Right);
            },
            ViMotion::WordLeft => {
                self.point = word(term, self.point, Direction::Left, Side::Left);
            },
            ViMotion::WordRight => {
                self.point = word(term, self.point, Direction::Right, Side::Left);
            },
            ViMotion::WordLeftEnd => {
                self.point = word(term, self.point, Direction::Left, Side::Right);
            },
            ViMotion::WordRightEnd => {
                self.point = word(term, self.point, Direction::Right, Side::Right);
            },
            ViMotion::Bracket => self.point = term.bracket_search(self.point).unwrap_or(self.point),
            ViMotion::ParagraphUp => {
                // Skip empty lines until we find the next paragraph,
                // then skip over the paragraph until we reach the next empty line.
                let topmost_line = term.topmost_line();
                self.point.line = (*topmost_line..*self.point.line)
                    .rev()
                    .skip_while(|line| term.grid()[Line(*line)].is_clear())
                    .find(|line| term.grid()[Line(*line)].is_clear())
                    .map_or(topmost_line, Line);
                self.point.column = Column(0);
            },
            ViMotion::ParagraphDown => {
                // Skip empty lines until we find the next paragraph,
                // then skip over the paragraph until we reach the next empty line.
                let bottommost_line = term.bottommost_line();
                self.point.line = (*self.point.line..*bottommost_line)
                    .skip_while(|line| term.grid()[Line(*line)].is_clear())
                    .find(|line| term.grid()[Line(*line)].is_clear())
                    .map_or(bottommost_line, Line);
                self.point.column = Column(0);
            },
        }

        term.scroll_to_point(self.point);

        self
    }

    /// Get target cursor point for vim-like page movement.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn scroll<T: EventListener>(mut self, term: &Term<T>, lines: i32) -> Self {
        // Clamp movement to within visible region.
        let line = (self.point.line - lines).grid_clamp(term, Boundary::Grid);

        // Find the first occupied cell after scrolling has been performed.
        let column = first_occupied_in_line(term, line).unwrap_or_default().column;

        // Move cursor.
        self.point = Point::new(line, column);

        self
    }
}

/// Find next end of line to move to.
fn last<T>(term: &Term<T>, mut point: Point) -> Point {
    // Expand across wide cells.
    point = term.expand_wide(point, Direction::Right);

    // Find last non-empty cell in the current line.
    let occupied = last_occupied_in_line(term, point.line).unwrap_or_default();

    if point.column < occupied.column {
        // Jump to last occupied cell when not already at or beyond it.
        occupied
    } else if is_wrap(term, point) {
        // Jump to last occupied cell across linewraps.
        while is_wrap(term, point) {
            point.line += 1;
        }

        last_occupied_in_line(term, point.line).unwrap_or(point)
    } else {
        // Jump to last column when beyond the last occupied cell.
        Point::new(point.line, term.last_column())
    }
}

/// Find next non-empty cell to move to.
fn first_occupied<T>(term: &Term<T>, mut point: Point) -> Point {
    let last_column = term.last_column();

    // Expand left across wide chars, since we're searching lines left to right.
    point = term.expand_wide(point, Direction::Left);

    // Find first non-empty cell in current line.
    let occupied = first_occupied_in_line(term, point.line)
        .unwrap_or_else(|| Point::new(point.line, last_column));

    // Jump across wrapped lines if we're already at this line's first occupied cell.
    if point == occupied {
        let mut occupied = None;

        // Search for non-empty cell in previous lines.
        for line in (term.topmost_line().0..point.line.0).rev().map(Line::from) {
            if !is_wrap(term, Point::new(line, last_column)) {
                break;
            }

            occupied = first_occupied_in_line(term, line).or(occupied);
        }

        // Fallback to the next non-empty cell.
        let mut line = point.line;
        occupied.unwrap_or_else(|| {
            loop {
                if let Some(occupied) = first_occupied_in_line(term, line) {
                    break occupied;
                }

                let last_cell = Point::new(line, last_column);
                if !is_wrap(term, last_cell) {
                    break last_cell;
                }

                line += 1;
            }
        })
    } else {
        occupied
    }
}

/// Move by semantically separated word, like w/b/e/ge in vi.
fn semantic<T: EventListener>(
    term: &Term<T>,
    mut point: Point,
    direction: Direction,
    side: Side,
) -> Point {
    // Expand semantically based on movement direction.
    let expand_semantic = |point: Point| {
        // Do not expand when currently on a semantic escape char.
        let cell = &term.grid()[point];
        if term.semantic_escape_chars().contains(cell.c)
            && !cell.flags.intersects(Flags::WIDE_CHAR_SPACER | Flags::LEADING_WIDE_CHAR_SPACER)
        {
            point
        } else if direction == Direction::Left {
            term.semantic_search_left(point)
        } else {
            term.semantic_search_right(point)
        }
    };

    // Move to word boundary.
    if direction != side && !is_boundary(term, point, direction) {
        point = expand_semantic(point);
    }

    // Make sure we jump above wide chars.
    point = term.expand_wide(point, direction);

    // Skip whitespace.
    let mut next_point = advance(term, point, direction);
    while !is_boundary(term, point, direction) && is_space(term, next_point) {
        point = next_point;
        next_point = advance(term, point, direction);
    }

    // Assure minimum movement of one cell.
    if !is_boundary(term, point, direction) {
        point = advance(term, point, direction);

        // Skip over wide cell spacers.
        if direction == Direction::Left {
            point = term.expand_wide(point, direction);
        }
    }

    // Move to word boundary.
    if direction == side && !is_boundary(term, point, direction) {
        point = expand_semantic(point);
    }

    point
}

/// Move by whitespace separated word, like W/B/E/gE in vi.
fn word<T: EventListener>(
    term: &Term<T>,
    mut point: Point,
    direction: Direction,
    side: Side,
) -> Point {
    // Make sure we jump above wide chars.
    point = term.expand_wide(point, direction);

    if direction == side {
        // Skip whitespace until right before a word.
        let mut next_point = advance(term, point, direction);
        while !is_boundary(term, point, direction) && is_space(term, next_point) {
            point = next_point;
            next_point = advance(term, point, direction);
        }

        // Skip non-whitespace until right inside word boundary.
        let mut next_point = advance(term, point, direction);
        while !is_boundary(term, point, direction) && !is_space(term, next_point) {
            point = next_point;
            next_point = advance(term, point, direction);
        }
    }

    if direction != side {
        // Skip non-whitespace until just beyond word.
        while !is_boundary(term, point, direction) && !is_space(term, point) {
            point = advance(term, point, direction);
        }

        // Skip whitespace until right inside word boundary.
        while !is_boundary(term, point, direction) && is_space(term, point) {
            point = advance(term, point, direction);
        }
    }

    point
}

/// Find first non-empty cell in line.
fn first_occupied_in_line<T>(term: &Term<T>, line: Line) -> Option<Point> {
    (0..term.columns())
        .map(|col| Point::new(line, Column(col)))
        .find(|&point| !is_space(term, point))
}

/// Find last non-empty cell in line.
fn last_occupied_in_line<T>(term: &Term<T>, line: Line) -> Option<Point> {
    (0..term.columns())
        .map(|col| Point::new(line, Column(col)))
        .rfind(|&point| !is_space(term, point))
}

/// Advance point based on direction.
fn advance<T>(term: &Term<T>, point: Point, direction: Direction) -> Point {
    if direction == Direction::Left {
        point.sub(term, Boundary::Grid, 1)
    } else {
        point.add(term, Boundary::Grid, 1)
    }
}

/// Check if cell at point contains whitespace.
fn is_space<T>(term: &Term<T>, point: Point) -> bool {
    let cell = &term.grid()[point.line][point.column];
    !cell.flags().intersects(Flags::WIDE_CHAR_SPACER | Flags::LEADING_WIDE_CHAR_SPACER)
        && (cell.c == ' ' || cell.c == '\t')
}

/// Check if the cell at a point contains the WRAPLINE flag.
fn is_wrap<T>(term: &Term<T>, point: Point) -> bool {
    term.grid()[point].flags.contains(Flags::WRAPLINE)
}

/// Check if point is at screen boundary.
fn is_boundary<T>(term: &Term<T>, point: Point, direction: Direction) -> bool {
    (point.line <= term.topmost_line() && point.column == 0 && direction == Direction::Left)
        || (point.line == term.bottommost_line()
            && point.column + 1 >= term.columns()
            && direction == Direction::Right)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::VoidListener;
    use crate::index::{Column, Line};
    use crate::term::test::TermSize;
    use crate::term::{Config, Term};
    use crate::vte::ansi::Handler;

    fn term() -> Term<VoidListener> {
        let size = TermSize::new(20, 20);
        Term::new(Config::default(), &size, VoidListener)
    }

    #[test]
    fn motion_simple() {
        let mut term = term();

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::Right);
        assert_eq!(cursor.point, Point::new(Line(0), Column(1)));

        cursor = cursor.motion(&mut term, ViMotion::Left);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::Down);
        assert_eq!(cursor.point, Point::new(Line(1), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::Up);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    #[test]
    fn simple_wide() {
        let mut term = term();
        term.grid_mut()[Line(0)][Column(0)].c = 'a';
        term.grid_mut()[Line(0)][Column(1)].c = '汉';
        term.grid_mut()[Line(0)][Column(1)].flags.insert(Flags::WIDE_CHAR);
        term.grid_mut()[Line(0)][Column(2)].c = ' ';
        term.grid_mut()[Line(0)][Column(2)].flags.insert(Flags::WIDE_CHAR_SPACER);
        term.grid_mut()[Line(0)][Column(3)].c = 'a';

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(1)));
        cursor = cursor.motion(&mut term, ViMotion::Right);
        assert_eq!(cursor.point, Point::new(Line(0), Column(3)));

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(2)));
        cursor = cursor.motion(&mut term, ViMotion::Left);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    #[test]
    fn motion_start_end() {
        let mut term = term();

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::Last);
        assert_eq!(cursor.point, Point::new(Line(0), Column(19)));

        cursor = cursor.motion(&mut term, ViMotion::First);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    #[test]
    fn motion_first_occupied() {
        let mut term = term();
        term.grid_mut()[Line(0)][Column(0)].c = ' ';
        term.grid_mut()[Line(0)][Column(1)].c = 'x';
        term.grid_mut()[Line(0)][Column(2)].c = ' ';
        term.grid_mut()[Line(0)][Column(3)].c = 'y';
        term.grid_mut()[Line(0)][Column(19)].flags.insert(Flags::WRAPLINE);
        term.grid_mut()[Line(1)][Column(19)].flags.insert(Flags::WRAPLINE);
        term.grid_mut()[Line(2)][Column(0)].c = 'z';
        term.grid_mut()[Line(2)][Column(1)].c = ' ';

        let mut cursor = ViModeCursor::new(Point::new(Line(2), Column(1)));

        cursor = cursor.motion(&mut term, ViMotion::FirstOccupied);
        assert_eq!(cursor.point, Point::new(Line(2), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::FirstOccupied);
        assert_eq!(cursor.point, Point::new(Line(0), Column(1)));
    }

    #[test]
    fn motion_high_middle_low() {
        let mut term = term();

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::High);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::Middle);
        assert_eq!(cursor.point, Point::new(Line(9), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::Low);
        assert_eq!(cursor.point, Point::new(Line(19), Column(0)));
    }

    #[test]
    fn motion_bracket() {
        let mut term = term();
        term.grid_mut()[Line(0)][Column(0)].c = '(';
        term.grid_mut()[Line(0)][Column(1)].c = 'x';
        term.grid_mut()[Line(0)][Column(2)].c = ')';

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::Bracket);
        assert_eq!(cursor.point, Point::new(Line(0), Column(2)));

        cursor = cursor.motion(&mut term, ViMotion::Bracket);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    fn motion_semantic_term() -> Term<VoidListener> {
        let mut term = term();

        term.grid_mut()[Line(0)][Column(0)].c = 'x';
        term.grid_mut()[Line(0)][Column(1)].c = ' ';
        term.grid_mut()[Line(0)][Column(2)].c = 'x';
        term.grid_mut()[Line(0)][Column(3)].c = 'x';
        term.grid_mut()[Line(0)][Column(4)].c = ' ';
        term.grid_mut()[Line(0)][Column(5)].c = ' ';
        term.grid_mut()[Line(0)][Column(6)].c = ':';
        term.grid_mut()[Line(0)][Column(7)].c = ' ';
        term.grid_mut()[Line(0)][Column(8)].c = 'x';
        term.grid_mut()[Line(0)][Column(9)].c = ':';
        term.grid_mut()[Line(0)][Column(10)].c = 'x';
        term.grid_mut()[Line(0)][Column(11)].c = ' ';
        term.grid_mut()[Line(0)][Column(12)].c = ' ';
        term.grid_mut()[Line(0)][Column(13)].c = ':';
        term.grid_mut()[Line(0)][Column(14)].c = ' ';
        term.grid_mut()[Line(0)][Column(15)].c = 'x';

        term
    }

    #[test]
    fn motion_semantic_right_end() {
        let mut term = motion_semantic_term();

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(3)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(6)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(8)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(9)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(10)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(13)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(15)));
    }

    #[test]
    fn motion_semantic_left_start() {
        let mut term = motion_semantic_term();

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(15)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(13)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(10)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(9)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(8)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(6)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(2)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    #[test]
    fn motion_semantic_right_start() {
        let mut term = motion_semantic_term();

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(2)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(6)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(8)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(9)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(10)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(13)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(15)));
    }

    #[test]
    fn motion_semantic_left_end() {
        let mut term = motion_semantic_term();

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(15)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(13)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(10)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(9)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(8)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(6)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(3)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    #[test]
    fn scroll_semantic() {
        let mut term = term();
        term.grid_mut().scroll_up(&(Line(0)..Line(20)), 5);

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(-5), Column(0)));
        assert_eq!(term.grid().display_offset(), 5);

        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(19), Column(19)));
        assert_eq!(term.grid().display_offset(), 0);

        cursor = cursor.motion(&mut term, ViMotion::SemanticLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(-5), Column(0)));
        assert_eq!(term.grid().display_offset(), 5);

        cursor = cursor.motion(&mut term, ViMotion::SemanticRightEnd);
        assert_eq!(cursor.point, Point::new(Line(19), Column(19)));
        assert_eq!(term.grid().display_offset(), 0);
    }

    #[test]
    fn semantic_wide() {
        let mut term = term();
        term.grid_mut()[Line(0)][Column(0)].c = 'a';
        term.grid_mut()[Line(0)][Column(1)].c = ' ';
        term.grid_mut()[Line(0)][Column(2)].c = '汉';
        term.grid_mut()[Line(0)][Column(2)].flags.insert(Flags::WIDE_CHAR);
        term.grid_mut()[Line(0)][Column(3)].c = ' ';
        term.grid_mut()[Line(0)][Column(3)].flags.insert(Flags::WIDE_CHAR_SPACER);
        term.grid_mut()[Line(0)][Column(4)].c = ' ';
        term.grid_mut()[Line(0)][Column(5)].c = 'a';

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(2)));
        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(5)));

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(3)));
        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    #[test]
    fn motion_word() {
        let mut term = term();
        term.grid_mut()[Line(0)][Column(0)].c = 'a';
        term.grid_mut()[Line(0)][Column(1)].c = ';';
        term.grid_mut()[Line(0)][Column(2)].c = ' ';
        term.grid_mut()[Line(0)][Column(3)].c = ' ';
        term.grid_mut()[Line(0)][Column(4)].c = 'a';
        term.grid_mut()[Line(0)][Column(5)].c = ';';

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::WordRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(1)));

        cursor = cursor.motion(&mut term, ViMotion::WordRightEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(5)));

        cursor = cursor.motion(&mut term, ViMotion::WordLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(4)));

        cursor = cursor.motion(&mut term, ViMotion::WordLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::WordRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(4)));

        cursor = cursor.motion(&mut term, ViMotion::WordLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(0), Column(1)));
    }

    #[test]
    fn scroll_word() {
        let mut term = term();
        term.grid_mut().scroll_up(&(Line(0)..Line(20)), 5);

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.motion(&mut term, ViMotion::WordLeft);
        assert_eq!(cursor.point, Point::new(Line(-5), Column(0)));
        assert_eq!(term.grid().display_offset(), 5);

        cursor = cursor.motion(&mut term, ViMotion::WordRight);
        assert_eq!(cursor.point, Point::new(Line(19), Column(19)));
        assert_eq!(term.grid().display_offset(), 0);

        cursor = cursor.motion(&mut term, ViMotion::WordLeftEnd);
        assert_eq!(cursor.point, Point::new(Line(-5), Column(0)));
        assert_eq!(term.grid().display_offset(), 5);

        cursor = cursor.motion(&mut term, ViMotion::WordRightEnd);
        assert_eq!(cursor.point, Point::new(Line(19), Column(19)));
        assert_eq!(term.grid().display_offset(), 0);
    }

    #[test]
    fn word_wide() {
        let mut term = term();
        term.grid_mut()[Line(0)][Column(0)].c = 'a';
        term.grid_mut()[Line(0)][Column(1)].c = ' ';
        term.grid_mut()[Line(0)][Column(2)].c = '汉';
        term.grid_mut()[Line(0)][Column(2)].flags.insert(Flags::WIDE_CHAR);
        term.grid_mut()[Line(0)][Column(3)].c = ' ';
        term.grid_mut()[Line(0)][Column(3)].flags.insert(Flags::WIDE_CHAR_SPACER);
        term.grid_mut()[Line(0)][Column(4)].c = ' ';
        term.grid_mut()[Line(0)][Column(5)].c = 'a';

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(2)));
        cursor = cursor.motion(&mut term, ViMotion::WordRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(5)));

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(3)));
        cursor = cursor.motion(&mut term, ViMotion::WordLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }

    #[test]
    fn scroll_simple() {
        let mut term = term();

        // Create 1 line of scrollback.
        for _ in 0..20 {
            term.newline();
        }

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));

        cursor = cursor.scroll(&term, -1);
        assert_eq!(cursor.point, Point::new(Line(1), Column(0)));

        cursor = cursor.scroll(&term, 1);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));

        cursor = cursor.scroll(&term, 1);
        assert_eq!(cursor.point, Point::new(Line(-1), Column(0)));
    }

    #[test]
    fn scroll_over_top() {
        let mut term = term();

        // Create 40 lines of scrollback.
        for _ in 0..59 {
            term.newline();
        }

        let mut cursor = ViModeCursor::new(Point::new(Line(19), Column(0)));

        cursor = cursor.scroll(&term, 20);
        assert_eq!(cursor.point, Point::new(Line(-1), Column(0)));

        cursor = cursor.scroll(&term, 20);
        assert_eq!(cursor.point, Point::new(Line(-21), Column(0)));

        cursor = cursor.scroll(&term, 20);
        assert_eq!(cursor.point, Point::new(Line(-40), Column(0)));

        cursor = cursor.scroll(&term, 20);
        assert_eq!(cursor.point, Point::new(Line(-40), Column(0)));
    }

    #[test]
    fn scroll_over_bottom() {
        let mut term = term();

        // Create 40 lines of scrollback.
        for _ in 0..59 {
            term.newline();
        }

        let mut cursor = ViModeCursor::new(Point::new(Line(-40), Column(0)));

        cursor = cursor.scroll(&term, -20);
        assert_eq!(cursor.point, Point::new(Line(-20), Column(0)));

        cursor = cursor.scroll(&term, -20);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));

        cursor = cursor.scroll(&term, -20);
        assert_eq!(cursor.point, Point::new(Line(19), Column(0)));

        cursor = cursor.scroll(&term, -20);
        assert_eq!(cursor.point, Point::new(Line(19), Column(0)));
    }

    #[test]
    fn wide_semantic_char() {
        let mut term = term();
        term.set_semantic_escape_chars("－");
        term.grid_mut()[Line(0)][Column(0)].c = 'x';
        term.grid_mut()[Line(0)][Column(1)].c = 'x';
        term.grid_mut()[Line(0)][Column(2)].c = '－';
        term.grid_mut()[Line(0)][Column(2)].flags.insert(Flags::WIDE_CHAR);
        term.grid_mut()[Line(0)][Column(3)].c = ' ';
        term.grid_mut()[Line(0)][Column(3)].flags.insert(Flags::WIDE_CHAR_SPACER);
        term.grid_mut()[Line(0)][Column(4)].c = 'x';
        term.grid_mut()[Line(0)][Column(5)].c = 'x';

        // Test motion to the right.

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(0)));
        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(2)));

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(2)));
        cursor = cursor.motion(&mut term, ViMotion::SemanticRight);
        assert_eq!(cursor.point, Point::new(Line(0), Column(4)));

        // Test motion to the left.

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(5)));
        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(4)));

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(4)));
        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(2)));

        let mut cursor = ViModeCursor::new(Point::new(Line(0), Column(2)));
        cursor = cursor.motion(&mut term, ViMotion::SemanticLeft);
        assert_eq!(cursor.point, Point::new(Line(0), Column(0)));
    }
}
