use std::{borrow::Cow, ops::Deref};

use alacritty_terminal::{
    grid::Dimensions,
    index::{Column, Direction, Line, Point},
    term::search::{Match, RegexIter, RegexSearch},
    Term,
};

const MAX_SEARCH_LINES: usize = 100;

///Header and impl fom alacritty/src/display/content.rs HintMatches
#[derive(Default)]
pub struct SearchMatches<'a> {
    /// All visible matches.
    matches: Cow<'a, [Match]>,

    /// Index of the last match checked.
    index: usize,
}

impl<'a> SearchMatches<'a> {
    /// Create new renderable matches iterator..
    fn new(matches: impl Into<Cow<'a, [Match]>>) -> Self {
        Self {
            matches: matches.into(),
            index: 0,
        }
    }

    /// Create from regex matches on term visable part.
    pub fn visible_regex_matches<T>(term: &Term<T>, dfas: &RegexSearch) -> Self {
        let matches = visible_regex_match_iter(term, dfas).collect::<Vec<_>>();
        Self::new(matches)
    }

    /// Advance the regex tracker to the next point.
    ///
    /// This will return `true` if the point passed is part of a regex match.
    fn advance(&mut self, point: Point) -> bool {
        while let Some(bounds) = self.get(self.index) {
            if bounds.start() > &point {
                break;
            } else if bounds.end() < &point {
                self.index += 1;
            } else {
                return true;
            }
        }
        false
    }
}

impl<'a> Deref for SearchMatches<'a> {
    type Target = [Match];

    fn deref(&self) -> &Self::Target {
        self.matches.deref()
    }
}

/// Copied from alacritty/src/display/hint.rs
/// Iterate over all visible regex matches.
fn visible_regex_match_iter<'a, T>(
    term: &'a Term<T>,
    regex: &'a RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    let viewport_start = Line(-(term.grid().display_offset() as i32));
    let viewport_end = viewport_start + term.bottommost_line();
    let mut start = term.line_search_left(Point::new(viewport_start, Column(0)));
    let mut end = term.line_search_right(Point::new(viewport_end, Column(0)));
    start.line = start.line.max(viewport_start - MAX_SEARCH_LINES);
    end.line = end.line.min(viewport_end + MAX_SEARCH_LINES);

    RegexIter::new(start, end, Direction::Right, term, regex)
        .skip_while(move |rm| rm.end().line < viewport_start)
        .take_while(move |rm| rm.start().line <= viewport_end)
}
