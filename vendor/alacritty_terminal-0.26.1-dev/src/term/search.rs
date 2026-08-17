use std::cmp::max;
use std::error::Error;
use std::mem;
use std::ops::RangeInclusive;

use log::{debug, warn};
pub use regex_automata::hybrid::BuildError;
use regex_automata::hybrid::dfa::{Builder, Cache, Config, DFA};
use regex_automata::nfa::thompson::Config as ThompsonConfig;
use regex_automata::util::syntax::Config as SyntaxConfig;
use regex_automata::{Anchored, Input, MatchKind};

use crate::grid::{BidirectionalIterator, Dimensions, GridIterator, Indexed};
use crate::index::{Boundary, Column, Direction, Point, Side};
use crate::term::Term;
use crate::term::cell::{Cell, Flags};

/// Used to match equal brackets, when performing a bracket-pair selection.
const BRACKET_PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

pub type Match = RangeInclusive<Point>;

/// Terminal regex search state.
#[derive(Clone, Debug)]
pub struct RegexSearch {
    left_fdfa: LazyDfa,
    left_rdfa: LazyDfa,
    right_rdfa: LazyDfa,
    right_fdfa: LazyDfa,
}

impl RegexSearch {
    /// Build the forward and backward search DFAs.
    pub fn new(search: &str) -> Result<RegexSearch, Box<BuildError>> {
        // Setup configs for both DFA directions.
        //
        // Bounds are based on Regex's meta engine:
        // https://github.com/rust-lang/regex/blob/061ee815ef2c44101dba7b0b124600fcb03c1912/regex-automata/src/meta/wrappers.rs#L581-L599
        let has_uppercase = search.chars().any(|c| c.is_uppercase());
        let syntax_config = SyntaxConfig::new().case_insensitive(!has_uppercase);
        let config =
            Config::new().minimum_cache_clear_count(Some(3)).minimum_bytes_per_state(Some(10));
        let max_size = config.get_cache_capacity();
        let thompson_config = ThompsonConfig::new().nfa_size_limit(Some(max_size));

        // Create DFAs to find start/end in right-to-left search.
        let left_rdfa = LazyDfa::new(
            search,
            config.clone(),
            syntax_config,
            thompson_config.clone(),
            Direction::Right,
            true,
        )?;
        let has_empty = left_rdfa.dfa.get_nfa().has_empty();
        let left_fdfa = LazyDfa::new(
            search,
            config.clone(),
            syntax_config,
            thompson_config.clone(),
            Direction::Left,
            has_empty,
        )?;

        // Create DFAs to find start/end in left-to-right search.
        let right_fdfa = LazyDfa::new(
            search,
            config.clone(),
            syntax_config,
            thompson_config.clone(),
            Direction::Right,
            has_empty,
        )?;
        let right_rdfa =
            LazyDfa::new(search, config, syntax_config, thompson_config, Direction::Left, true)?;

        Ok(RegexSearch { left_fdfa, left_rdfa, right_fdfa, right_rdfa })
    }
}

/// Runtime-evaluated DFA.
#[derive(Clone, Debug)]
struct LazyDfa {
    dfa: DFA,
    cache: Cache,
    direction: Direction,
    match_all: bool,
}

impl LazyDfa {
    fn new(
        search: &str,
        mut config: Config,
        syntax: SyntaxConfig,
        mut thompson: ThompsonConfig,
        direction: Direction,
        match_all: bool,
    ) -> Result<Self, Box<BuildError>> {
        thompson = match direction {
            Direction::Left => thompson.reverse(true),
            Direction::Right => thompson.reverse(false),
        };
        config = if match_all {
            config.match_kind(MatchKind::All)
        } else {
            config.match_kind(MatchKind::LeftmostFirst)
        };

        // Create the DFA.
        let dfa =
            Builder::new().configure(config).syntax(syntax).thompson(thompson).build(search)?;

        let cache = dfa.create_cache();

        Ok(Self { direction, cache, dfa, match_all })
    }
}

impl<T> Term<T> {
    /// Get next search match in the specified direction.
    pub fn search_next(
        &self,
        regex: &mut RegexSearch,
        mut origin: Point,
        direction: Direction,
        side: Side,
        mut max_lines: Option<usize>,
    ) -> Option<Match> {
        origin = self.expand_wide(origin, direction);

        max_lines = max_lines.filter(|max_lines| max_lines + 1 < self.total_lines());

        match direction {
            Direction::Right => self.next_match_right(regex, origin, side, max_lines),
            Direction::Left => self.next_match_left(regex, origin, side, max_lines),
        }
    }

    /// Find the next match to the right of the origin.
    fn next_match_right(
        &self,
        regex: &mut RegexSearch,
        origin: Point,
        side: Side,
        max_lines: Option<usize>,
    ) -> Option<Match> {
        let start = self.line_search_left(origin);
        let mut end = start;

        // Limit maximum number of lines searched.
        end = match max_lines {
            Some(max_lines) => {
                let line = (start.line + max_lines).grid_clamp(self, Boundary::None);
                Point::new(line, self.last_column())
            },
            _ => end.sub(self, Boundary::None, 1),
        };

        let mut regex_iter = RegexIter::new(start, end, Direction::Right, self, regex).peekable();

        // Check if there's any match at all.
        let first_match = regex_iter.peek()?.clone();

        let regex_match = regex_iter
            .find(|regex_match| {
                let match_point = Self::match_side(regex_match, side);

                // If the match's point is beyond the origin, we're done.
                match_point.line < start.line
                    || match_point.line > origin.line
                    || (match_point.line == origin.line && match_point.column >= origin.column)
            })
            .unwrap_or(first_match);

        Some(regex_match)
    }

    /// Find the next match to the left of the origin.
    fn next_match_left(
        &self,
        regex: &mut RegexSearch,
        origin: Point,
        side: Side,
        max_lines: Option<usize>,
    ) -> Option<Match> {
        let start = self.line_search_right(origin);
        let mut end = start;

        // Limit maximum number of lines searched.
        end = match max_lines {
            Some(max_lines) => {
                let line = (start.line - max_lines).grid_clamp(self, Boundary::None);
                Point::new(line, Column(0))
            },
            _ => end.add(self, Boundary::None, 1),
        };

        let mut regex_iter = RegexIter::new(start, end, Direction::Left, self, regex).peekable();

        // Check if there's any match at all.
        let first_match = regex_iter.peek()?.clone();

        let regex_match = regex_iter
            .find(|regex_match| {
                let match_point = Self::match_side(regex_match, side);

                // If the match's point is beyond the origin, we're done.
                match_point.line > start.line
                    || match_point.line < origin.line
                    || (match_point.line == origin.line && match_point.column <= origin.column)
            })
            .unwrap_or(first_match);

        Some(regex_match)
    }

    /// Get the side of a match.
    fn match_side(regex_match: &Match, side: Side) -> Point {
        match side {
            Side::Right => *regex_match.end(),
            Side::Left => *regex_match.start(),
        }
    }

    /// Find the next regex match to the left of the origin point.
    ///
    /// The origin is always included in the regex.
    pub fn regex_search_left(
        &self,
        regex: &mut RegexSearch,
        start: Point,
        end: Point,
    ) -> Option<Match> {
        // Find start and end of match.
        let match_start = self.regex_search(start, end, &mut regex.left_fdfa)?;
        let match_end = self.regex_search(match_start, start, &mut regex.left_rdfa)?;

        Some(match_start..=match_end)
    }

    /// Find the next regex match to the right of the origin point.
    ///
    /// The origin is always included in the regex.
    pub fn regex_search_right(
        &self,
        regex: &mut RegexSearch,
        start: Point,
        end: Point,
    ) -> Option<Match> {
        // Find start and end of match.
        let match_end = self.regex_search(start, end, &mut regex.right_fdfa)?;
        let match_start = self.regex_search(match_end, start, &mut regex.right_rdfa)?;

        Some(match_start..=match_end)
    }

    /// Find the next regex match.
    ///
    /// This will always return the side of the first match which is farthest from the start point.
    fn regex_search(&self, start: Point, end: Point, regex: &mut LazyDfa) -> Option<Point> {
        match self.regex_search_internal(start, end, regex) {
            Ok(regex_match) => regex_match,
            Err(err) => {
                warn!("Regex exceeded complexity limit");
                debug!("    {err}");
                None
            },
        }
    }

    /// Find the next regex match.
    ///
    /// To automatically log regex complexity errors, use [`Self::regex_search`] instead.
    fn regex_search_internal(
        &self,
        start: Point,
        end: Point,
        regex: &mut LazyDfa,
    ) -> Result<Option<Point>, Box<dyn Error>> {
        let topmost_line = self.topmost_line();
        let screen_lines = self.screen_lines() as i32;
        let last_column = self.last_column();

        // Advance the iterator.
        let next = match regex.direction {
            Direction::Right => GridIterator::next,
            Direction::Left => GridIterator::prev,
        };

        // Get start state for the DFA.
        let regex_anchored = if regex.match_all { Anchored::Yes } else { Anchored::No };
        let input = Input::new(&[]).anchored(regex_anchored);
        let mut state = regex.dfa.start_state_forward(&mut regex.cache, &input).unwrap();

        let mut iter = self.grid.iter_from(start);
        let mut regex_match = None;
        let mut done = false;

        let mut cell = iter.cell();
        self.skip_fullwidth(&mut iter, &mut cell, regex.direction);
        let mut c = cell.c;
        let mut last_wrapped = iter.cell().flags.contains(Flags::WRAPLINE);

        let mut point = iter.point();
        let mut last_point = point;
        let mut consumed_bytes = 0;

        // Reset the regex state to restart the search.
        macro_rules! reset_state {
            () => {{
                state = regex.dfa.start_state_forward(&mut regex.cache, &input)?;
                consumed_bytes = 0;
                regex_match = None;
            }};
        }

        'outer: loop {
            // Convert char to array of bytes.
            let mut buf = [0; 4];
            let utf8_len = c.encode_utf8(&mut buf).len();

            // Pass char to DFA as individual bytes.
            for i in 0..utf8_len {
                // Inverse byte order when going left.
                let byte = match regex.direction {
                    Direction::Right => buf[i],
                    Direction::Left => buf[utf8_len - i - 1],
                };

                state = regex.dfa.next_state(&mut regex.cache, state, byte)?;
                consumed_bytes += 1;

                if i == 0 && state.is_match() {
                    // Matches require one additional BYTE of lookahead, so we check the match state
                    // for the first byte of every new character to determine if the last character
                    // was a match.
                    regex_match = Some(last_point);
                } else if state.is_dead() {
                    if consumed_bytes == 2 {
                        // Reset search if we found an empty match.
                        //
                        // With an unanchored search, a dead state only occurs after the end of a
                        // match has been found. While we want to abort after the first match has
                        // ended, we don't want empty matches since we cannot highlight them.
                        //
                        // So once we encounter an empty match, we reset our parser state and clear
                        // the match, effectively starting a new search one character farther than
                        // before.
                        //
                        // An empty match requires consuming `2` bytes, since the first byte will
                        // report the match for the empty string, while the second byte then
                        // reports the dead state indicating the first character isn't part of the
                        // match.
                        reset_state!();

                        // Retry this character if first byte caused failure.
                        //
                        // After finding an empty match, we want to advance the search start by one
                        // character. So if the first character has multiple bytes and the dead
                        // state isn't reached at `i == 0`, then we continue with the rest of the
                        // loop to advance the parser by one character.
                        if i == 0 {
                            continue 'outer;
                        }
                    } else {
                        // Abort on dead state.
                        break 'outer;
                    }
                }
            }

            // Stop once we've reached the target point.
            if point == end || done {
                // When reaching the end-of-input, we need to notify the parser that no look-ahead
                // is possible and check for state changes.
                state = regex.dfa.next_eoi_state(&mut regex.cache, state)?;
                if state.is_match() {
                    regex_match = Some(point);
                } else if state.is_dead() && consumed_bytes == 1 {
                    // Ignore empty matches.
                    regex_match = None;
                }

                break;
            }

            // Advance grid cell iterator.
            let mut cell = match next(&mut iter) {
                Some(Indexed { cell, .. }) => cell,
                None => {
                    // Wrap around to other end of the scrollback buffer.
                    let line = topmost_line - point.line + screen_lines - 1;
                    let start = Point::new(line, last_column - point.column);
                    iter = self.grid.iter_from(start);
                    iter.cell()
                },
            };

            // Check for completion before potentially skipping over fullwidth characters.
            done = iter.point() == end;

            self.skip_fullwidth(&mut iter, &mut cell, regex.direction);

            c = cell.c;
            let wrapped = iter.cell().flags.contains(Flags::WRAPLINE);

            last_point = mem::replace(&mut point, iter.point());

            // Handle linebreaks.
            if (last_point.column == last_column && point.column == Column(0) && !last_wrapped)
                || (last_point.column == Column(0) && point.column == last_column && !wrapped)
            {
                // When reaching the end-of-input, we need to notify the parser that no
                // look-ahead is possible and check if the current state is still a match.
                state = regex.dfa.next_eoi_state(&mut regex.cache, state)?;
                if state.is_match() {
                    regex_match = Some(last_point);
                }

                match regex_match {
                    // Stop if we found a non-empty match before the linebreak.
                    Some(_) if (!state.is_dead() || consumed_bytes > 1) && consumed_bytes != 0 => {
                        break;
                    },
                    _ => reset_state!(),
                }
            }

            last_wrapped = wrapped;
        }

        Ok(regex_match)
    }

    /// Advance a grid iterator over fullwidth characters.
    fn skip_fullwidth<'a>(
        &self,
        iter: &'a mut GridIterator<'_, Cell>,
        cell: &mut &'a Cell,
        direction: Direction,
    ) {
        match direction {
            // In the alternate screen buffer there might not be a wide char spacer after a wide
            // char, so we only advance the iterator when the wide char is not in the last column.
            Direction::Right
                if cell.flags.contains(Flags::WIDE_CHAR)
                    && iter.point().column < self.last_column() =>
            {
                iter.next();
            },
            Direction::Right if cell.flags.contains(Flags::LEADING_WIDE_CHAR_SPACER) => {
                if let Some(Indexed { cell: new_cell, .. }) = iter.next() {
                    *cell = new_cell;
                }
                iter.next();
            },
            Direction::Left if cell.flags.contains(Flags::WIDE_CHAR_SPACER) => {
                if let Some(Indexed { cell: new_cell, .. }) = iter.prev() {
                    *cell = new_cell;
                }

                let prev = iter.point().sub(self, Boundary::Grid, 1);
                if self.grid[prev].flags.contains(Flags::LEADING_WIDE_CHAR_SPACER) {
                    iter.prev();
                }
            },
            _ => (),
        }
    }

    /// Find next matching bracket.
    pub fn bracket_search(&self, point: Point) -> Option<Point> {
        let start_char = self.grid[point].c;

        // Find the matching bracket we're looking for
        let (forward, end_char) = BRACKET_PAIRS.iter().find_map(|(open, close)| {
            if open == &start_char {
                Some((true, *close))
            } else if close == &start_char {
                Some((false, *open))
            } else {
                None
            }
        })?;

        let mut iter = self.grid.iter_from(point);

        // For every character match that equals the starting bracket, we
        // ignore one bracket of the opposite type.
        let mut skip_pairs = 0;

        loop {
            // Check the next cell
            let cell = if forward { iter.next() } else { iter.prev() };

            // Break if there are no more cells
            let cell = match cell {
                Some(cell) => cell,
                None => break,
            };

            // Check if the bracket matches
            if cell.c == end_char && skip_pairs == 0 {
                return Some(cell.point);
            } else if cell.c == start_char {
                skip_pairs += 1;
            } else if cell.c == end_char {
                skip_pairs -= 1;
            }
        }

        None
    }

    /// Find left end of semantic block.
    #[must_use]
    pub fn semantic_search_left(&self, point: Point) -> Point {
        match self.inline_search_left(point, self.semantic_escape_chars()) {
            // If we found a match, reverse for at least one cell, skipping over wide cell spacers.
            Ok(point) => {
                let wide_spacer = Flags::WIDE_CHAR_SPACER | Flags::LEADING_WIDE_CHAR_SPACER;
                self.grid
                    .iter_from(point)
                    .find(|cell| !cell.flags.intersects(wide_spacer))
                    .map_or(point, |cell| cell.point)
            },
            Err(point) => point,
        }
    }

    /// Find right end of semantic block.
    #[must_use]
    pub fn semantic_search_right(&self, point: Point) -> Point {
        match self.inline_search_right(point, self.semantic_escape_chars()) {
            Ok(point) => self.grid.iter_from(point).prev().map_or(point, |cell| cell.point),
            Err(point) => point,
        }
    }

    /// Searching to the left, find the next character contained in `needles`.
    pub fn inline_search_left(&self, mut point: Point, needles: &str) -> Result<Point, Point> {
        // Limit the starting point to the last line in the history
        point.line = max(point.line, self.topmost_line());

        let mut iter = self.grid.iter_from(point);
        let last_column = self.columns() - 1;

        let wide_spacer = Flags::WIDE_CHAR_SPACER | Flags::LEADING_WIDE_CHAR_SPACER;
        while let Some(cell) = iter.prev() {
            if cell.point.column == last_column && !cell.flags.contains(Flags::WRAPLINE) {
                break;
            }

            point = cell.point;

            if !cell.flags.intersects(wide_spacer) && needles.contains(cell.c) {
                return Ok(point);
            }
        }

        Err(point)
    }

    /// Searching to the right, find the next character contained in `needles`.
    pub fn inline_search_right(&self, mut point: Point, needles: &str) -> Result<Point, Point> {
        // Limit the starting point to the last line in the history
        point.line = max(point.line, self.topmost_line());

        let wide_spacer = Flags::WIDE_CHAR_SPACER | Flags::LEADING_WIDE_CHAR_SPACER;
        let last_column = self.columns() - 1;

        // Immediately stop if start point in on line break.
        if point.column == last_column && !self.grid[point].flags.contains(Flags::WRAPLINE) {
            return Err(point);
        }

        for cell in self.grid.iter_from(point) {
            point = cell.point;

            if !cell.flags.intersects(wide_spacer) && needles.contains(cell.c) {
                return Ok(point);
            }

            if point.column == last_column && !cell.flags.contains(Flags::WRAPLINE) {
                break;
            }
        }

        Err(point)
    }

    /// Find the beginning of the current line across linewraps.
    pub fn line_search_left(&self, mut point: Point) -> Point {
        while point.line > self.topmost_line()
            && self.grid[point.line - 1i32][self.last_column()].flags.contains(Flags::WRAPLINE)
        {
            point.line -= 1;
        }

        point.column = Column(0);

        point
    }

    /// Find the end of the current line across linewraps.
    pub fn line_search_right(&self, mut point: Point) -> Point {
        while point.line + 1 < self.screen_lines()
            && self.grid[point.line][self.last_column()].flags.contains(Flags::WRAPLINE)
        {
            point.line += 1;
        }

        point.column = self.last_column();

        point
    }
}

/// Iterator over regex matches.
pub struct RegexIter<'a, T> {
    point: Point,
    end: Point,
    direction: Direction,
    regex: &'a mut RegexSearch,
    term: &'a Term<T>,
    done: bool,
}

impl<'a, T> RegexIter<'a, T> {
    pub fn new(
        start: Point,
        end: Point,
        direction: Direction,
        term: &'a Term<T>,
        regex: &'a mut RegexSearch,
    ) -> Self {
        Self { point: start, done: false, end, direction, term, regex }
    }

    /// Skip one cell, advancing the origin point to the next one.
    fn skip(&mut self) {
        self.point = self.term.expand_wide(self.point, self.direction);

        self.point = match self.direction {
            Direction::Right => self.point.add(self.term, Boundary::None, 1),
            Direction::Left => self.point.sub(self.term, Boundary::None, 1),
        };
    }

    /// Get the next match in the specified direction.
    fn next_match(&mut self) -> Option<Match> {
        match self.direction {
            Direction::Right => self.term.regex_search_right(self.regex, self.point, self.end),
            Direction::Left => self.term.regex_search_left(self.regex, self.point, self.end),
        }
    }
}

impl<T> Iterator for RegexIter<'_, T> {
    type Item = Match;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Since the end itself might be a single cell match, we search one more time.
        if self.point == self.end {
            self.done = true;
        }

        let regex_match = self.next_match()?;

        self.point = *regex_match.end();
        if self.point == self.end {
            // Stop when the match terminates right on the end limit.
            self.done = true;
        } else {
            // Move the new search origin past the match.
            self.skip();
        }

        Some(regex_match)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::index::{Column, Line};
    use crate::term::Config;
    use crate::term::test::{TermSize, mock_term};

    #[test]
    fn regex_right() {
        #[rustfmt::skip]
        let term = mock_term("\
            testing66\r\n\
            Alacritty\n\
            123\r\n\
            Alacritty\r\n\
            123\
        ");

        // Check regex across wrapped and unwrapped lines.
        let mut regex = RegexSearch::new("Ala.*123").unwrap();
        let start = Point::new(Line(1), Column(0));
        let end = Point::new(Line(4), Column(2));
        let match_start = Point::new(Line(1), Column(0));
        let match_end = Point::new(Line(2), Column(2));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn regex_left() {
        #[rustfmt::skip]
        let term = mock_term("\
            testing66\r\n\
            Alacritty\n\
            123\r\n\
            Alacritty\r\n\
            123\
        ");

        // Check regex across wrapped and unwrapped lines.
        let mut regex = RegexSearch::new("Ala.*123").unwrap();
        let start = Point::new(Line(4), Column(2));
        let end = Point::new(Line(1), Column(0));
        let match_start = Point::new(Line(1), Column(0));
        let match_end = Point::new(Line(2), Column(2));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn nested_regex() {
        #[rustfmt::skip]
        let term = mock_term("\
            Ala -> Alacritty -> critty\r\n\
            critty\
        ");

        // Greedy stopped at linebreak.
        let mut regex = RegexSearch::new("Ala.*critty").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(25));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));

        // Greedy stopped at dead state.
        let mut regex = RegexSearch::new("Ala[^y]*critty").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(15));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));
    }

    #[test]
    fn no_match_right() {
        #[rustfmt::skip]
        let term = mock_term("\
            first line\n\
            broken second\r\n\
            third\
        ");

        let mut regex = RegexSearch::new("nothing").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(2), Column(4));
        assert_eq!(term.regex_search_right(&mut regex, start, end), None);
    }

    #[test]
    fn no_match_left() {
        #[rustfmt::skip]
        let term = mock_term("\
            first line\n\
            broken second\r\n\
            third\
        ");

        let mut regex = RegexSearch::new("nothing").unwrap();
        let start = Point::new(Line(2), Column(4));
        let end = Point::new(Line(0), Column(0));
        assert_eq!(term.regex_search_left(&mut regex, start, end), None);
    }

    #[test]
    fn include_linebreak_left() {
        #[rustfmt::skip]
        let term = mock_term("\
            testing123\r\n\
            xxx\
        ");

        // Make sure the cell containing the linebreak is not skipped.
        let mut regex = RegexSearch::new("te.*123").unwrap();
        let start = Point::new(Line(1), Column(0));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(0), Column(0));
        let match_end = Point::new(Line(0), Column(9));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn include_linebreak_right() {
        #[rustfmt::skip]
        let term = mock_term("\
            xxx\r\n\
            testing123\
        ");

        // Make sure the cell containing the linebreak is not skipped.
        let mut regex = RegexSearch::new("te.*123").unwrap();
        let start = Point::new(Line(0), Column(2));
        let end = Point::new(Line(1), Column(9));
        let match_start = Point::new(Line(1), Column(0));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=end));
    }

    #[test]
    fn skip_dead_cell() {
        let term = mock_term("alacritty");

        // Make sure dead state cell is skipped when reversing.
        let mut regex = RegexSearch::new("alacrit").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(6));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));
    }

    #[test]
    fn reverse_search_dead_recovery() {
        let term = mock_term("zooo lense");

        // Make sure the reverse DFA operates the same as a forward DFA.
        let mut regex = RegexSearch::new("zoo").unwrap();
        let start = Point::new(Line(0), Column(9));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(0), Column(0));
        let match_end = Point::new(Line(0), Column(2));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn multibyte_unicode() {
        let term = mock_term("testвосибing");

        let mut regex = RegexSearch::new("te.*ing").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(11));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));

        let mut regex = RegexSearch::new("te.*ing").unwrap();
        let start = Point::new(Line(0), Column(11));
        let end = Point::new(Line(0), Column(0));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(end..=start));
    }

    #[test]
    fn end_on_multibyte_unicode() {
        let term = mock_term("testвосиб");

        let mut regex = RegexSearch::new("te.*и").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(8));
        let match_end = Point::new(Line(0), Column(7));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=match_end));
    }

    #[test]
    fn fullwidth() {
        let term = mock_term("a🦇x🦇");

        let mut regex = RegexSearch::new("[^ ]*").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(5));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));

        let mut regex = RegexSearch::new("[^ ]*").unwrap();
        let start = Point::new(Line(0), Column(5));
        let end = Point::new(Line(0), Column(0));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(end..=start));
    }

    #[test]
    fn singlecell_fullwidth() {
        let term = mock_term("🦇");

        let mut regex = RegexSearch::new("🦇").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(1));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));

        let mut regex = RegexSearch::new("🦇").unwrap();
        let start = Point::new(Line(0), Column(1));
        let end = Point::new(Line(0), Column(0));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(end..=start));
    }

    #[test]
    fn end_on_fullwidth() {
        let term = mock_term("jarr🦇");

        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(4));

        // Ensure ending without a match doesn't loop indefinitely.
        let mut regex = RegexSearch::new("x").unwrap();
        assert_eq!(term.regex_search_right(&mut regex, start, end), None);

        let mut regex = RegexSearch::new("x").unwrap();
        let match_end = Point::new(Line(0), Column(5));
        assert_eq!(term.regex_search_right(&mut regex, start, match_end), None);

        // Ensure match is captured when only partially inside range.
        let mut regex = RegexSearch::new("jarr🦇").unwrap();
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=match_end));
    }

    #[test]
    fn wrapping() {
        #[rustfmt::skip]
        let term = mock_term("\
            xxx\r\n\
            xxx\
        ");

        let mut regex = RegexSearch::new("xxx").unwrap();
        let start = Point::new(Line(0), Column(2));
        let end = Point::new(Line(1), Column(2));
        let match_start = Point::new(Line(1), Column(0));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=end));

        let mut regex = RegexSearch::new("xxx").unwrap();
        let start = Point::new(Line(1), Column(0));
        let end = Point::new(Line(0), Column(0));
        let match_end = Point::new(Line(0), Column(2));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(end..=match_end));
    }

    #[test]
    fn wrapping_into_fullwidth() {
        #[rustfmt::skip]
        let term = mock_term("\
            🦇xx\r\n\
            xx🦇\
        ");

        let mut regex = RegexSearch::new("🦇x").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(1), Column(3));
        let match_start = Point::new(Line(0), Column(0));
        let match_end = Point::new(Line(0), Column(2));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));

        let mut regex = RegexSearch::new("x🦇").unwrap();
        let start = Point::new(Line(1), Column(2));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(1), Column(1));
        let match_end = Point::new(Line(1), Column(3));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn multiline() {
        #[rustfmt::skip]
        let term = mock_term("\
            test \r\n\
            test\
        ");

        const PATTERN: &str = "[a-z]*";
        let mut regex = RegexSearch::new(PATTERN).unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(3));
        let match_start = Point::new(Line(0), Column(0));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=end));

        let mut regex = RegexSearch::new(PATTERN).unwrap();
        let start = Point::new(Line(0), Column(4));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(1), Column(0));
        let match_end = Point::new(Line(1), Column(3));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn empty_match() {
        #[rustfmt::skip]
        let term = mock_term(" abc ");

        const PATTERN: &str = "[a-z]*";
        let mut regex = RegexSearch::new(PATTERN).unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(4));
        let match_start = Point::new(Line(0), Column(1));
        let match_end = Point::new(Line(0), Column(3));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn empty_match_multibyte() {
        #[rustfmt::skip]
        let term = mock_term(" ↑");

        const PATTERN: &str = "[a-z]*";
        let mut regex = RegexSearch::new(PATTERN).unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(1));
        assert_eq!(term.regex_search_right(&mut regex, start, end), None);
    }

    #[test]
    fn empty_match_multiline() {
        #[rustfmt::skip]
        let term = mock_term("abc          \nxxx");

        const PATTERN: &str = "[a-z]*";
        let mut regex = RegexSearch::new(PATTERN).unwrap();
        let start = Point::new(Line(0), Column(3));
        let end = Point::new(Line(1), Column(2));
        let match_start = Point::new(Line(1), Column(0));
        let match_end = Point::new(Line(1), Column(2));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn leading_spacer() {
        #[rustfmt::skip]
        let mut term = mock_term("\
            xxx \n\
            🦇xx\
        ");
        term.grid[Line(0)][Column(3)].flags.insert(Flags::LEADING_WIDE_CHAR_SPACER);

        let mut regex = RegexSearch::new("🦇x").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(1), Column(3));
        let match_start = Point::new(Line(0), Column(3));
        let match_end = Point::new(Line(1), Column(2));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));

        let mut regex = RegexSearch::new("🦇x").unwrap();
        let start = Point::new(Line(1), Column(3));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(0), Column(3));
        let match_end = Point::new(Line(1), Column(2));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));

        let mut regex = RegexSearch::new("x🦇").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(1), Column(3));
        let match_start = Point::new(Line(0), Column(2));
        let match_end = Point::new(Line(1), Column(1));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));

        let mut regex = RegexSearch::new("x🦇").unwrap();
        let start = Point::new(Line(1), Column(3));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(0), Column(2));
        let match_end = Point::new(Line(1), Column(1));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn wide_without_spacer() {
        let size = TermSize::new(2, 2);
        let mut term = Term::new(Config::default(), &size, ());
        term.grid[Line(0)][Column(0)].c = 'x';
        term.grid[Line(0)][Column(1)].c = '字';
        term.grid[Line(0)][Column(1)].flags = Flags::WIDE_CHAR;

        let mut regex = RegexSearch::new("test").unwrap();

        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(1));

        let mut iter = RegexIter::new(start, end, Direction::Right, &term, &mut regex);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn wrap_around_to_another_end() {
        #[rustfmt::skip]
        let term = mock_term("\
            abc\r\n\
            def\
        ");

        // Bottom to top.
        let mut regex = RegexSearch::new("abc").unwrap();
        let start = Point::new(Line(1), Column(0));
        let end = Point::new(Line(0), Column(2));
        let match_start = Point::new(Line(0), Column(0));
        let match_end = Point::new(Line(0), Column(2));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));

        // Top to bottom.
        let mut regex = RegexSearch::new("def").unwrap();
        let start = Point::new(Line(0), Column(2));
        let end = Point::new(Line(1), Column(0));
        let match_start = Point::new(Line(1), Column(0));
        let match_end = Point::new(Line(1), Column(2));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn nfa_compile_error() {
        assert!(RegexSearch::new("[0-9A-Za-z]{9999999}").is_err());
    }

    #[test]
    fn runtime_cache_error() {
        let term = mock_term(&str::repeat("i", 9999));

        let mut regex = RegexSearch::new("[0-9A-Za-z]{9999}").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(9999));
        assert_eq!(term.regex_search_right(&mut regex, start, end), None);
    }

    #[test]
    fn greed_is_good() {
        #[rustfmt::skip]
        let term = mock_term("https://github.com");

        // Bottom to top.
        let mut regex = RegexSearch::new("/github.com|https://github.com").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(17));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));
    }

    #[test]
    fn anchored_empty() {
        #[rustfmt::skip]
        let term = mock_term("rust");

        // Bottom to top.
        let mut regex = RegexSearch::new(";*|rust").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(0), Column(3));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(start..=end));
    }

    #[test]
    fn newline_breaking_semantic() {
        #[rustfmt::skip]
        let term = mock_term("\
            test abc\r\n\
            def test\
        ");

        // Start at last character.
        let start = term.semantic_search_left(Point::new(Line(0), Column(7)));
        let end = term.semantic_search_right(Point::new(Line(0), Column(7)));
        assert_eq!(start, Point::new(Line(0), Column(5)));
        assert_eq!(end, Point::new(Line(0), Column(7)));

        // Start at first character.
        let start = term.semantic_search_left(Point::new(Line(1), Column(0)));
        let end = term.semantic_search_right(Point::new(Line(1), Column(0)));
        assert_eq!(start, Point::new(Line(1), Column(0)));
        assert_eq!(end, Point::new(Line(1), Column(2)));
    }

    #[test]
    fn inline_word_search() {
        #[rustfmt::skip]
        let term = mock_term("\
            word word word word w\n\
            ord word word word\
        ");

        let mut regex = RegexSearch::new("word").unwrap();
        let start = Point::new(Line(1), Column(4));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(0), Column(20));
        let match_end = Point::new(Line(1), Column(2));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_start..=match_end));
    }

    #[test]
    fn fullwidth_semantic() {
        #[rustfmt::skip]
        let mut term = mock_term("test－x－test");
        term.config.semantic_escape_chars = "－".into();

        let start = term.semantic_search_left(Point::new(Line(0), Column(6)));
        let end = term.semantic_search_right(Point::new(Line(0), Column(6)));
        assert_eq!(start, Point::new(Line(0), Column(6)));
        assert_eq!(end, Point::new(Line(0), Column(6)));
    }

    #[test]
    fn fullwidth_across_lines() {
        let term = mock_term("a🦇\n🦇b");

        let mut regex = RegexSearch::new("🦇🦇").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(1), Column(2));
        let match_start = Point::new(Line(0), Column(1));
        let match_end = Point::new(Line(1), Column(1));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));

        let mut regex = RegexSearch::new("🦇🦇").unwrap();
        let start = Point::new(Line(1), Column(2));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(1), Column(1));
        let match_end = Point::new(Line(0), Column(1));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_end..=match_start));
    }

    #[test]
    fn fullwidth_into_halfwidth_across_lines() {
        let term = mock_term("a🦇\nxab");

        let mut regex = RegexSearch::new("🦇x").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(1), Column(2));
        let match_start = Point::new(Line(0), Column(1));
        let match_end = Point::new(Line(1), Column(0));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));

        let mut regex = RegexSearch::new("🦇x").unwrap();
        let start = Point::new(Line(1), Column(2));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(1), Column(0));
        let match_end = Point::new(Line(0), Column(1));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_end..=match_start));
    }

    #[test]
    fn no_spacer_fullwidth_linewrap() {
        let mut term = mock_term("abY\nxab");
        term.grid_mut()[Line(0)][Column(2)].c = '🦇';

        let mut regex = RegexSearch::new("🦇x").unwrap();
        let start = Point::new(Line(0), Column(0));
        let end = Point::new(Line(1), Column(2));
        let match_start = Point::new(Line(0), Column(2));
        let match_end = Point::new(Line(1), Column(0));
        assert_eq!(term.regex_search_right(&mut regex, start, end), Some(match_start..=match_end));

        let mut regex = RegexSearch::new("🦇x").unwrap();
        let start = Point::new(Line(1), Column(2));
        let end = Point::new(Line(0), Column(0));
        let match_start = Point::new(Line(1), Column(0));
        let match_end = Point::new(Line(0), Column(2));
        assert_eq!(term.regex_search_left(&mut regex, start, end), Some(match_end..=match_start));
    }
}
