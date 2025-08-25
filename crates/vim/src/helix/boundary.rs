use std::ops::Range;

use editor::{
    DisplayPoint,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
};
use language::{CharClassifier, CharKind};
use text::Bias;

use crate::helix::object::HelixTextObject;

/// Text objects (after helix definition) that can easily be
/// found by reading a buffer and comparing two neighboring chars
/// until a start / end is found
trait BoundedObject {
    /// The next start since `from` (inclusive).
    fn next_start(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint>;
    /// The next end since `from` (inclusive).
    fn next_end(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint>;
    /// The previous start since `from` (inclusive).
    fn previous_start(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint>;
    /// The previous end since `from` (inclusive).
    fn previous_end(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint>;
    /// Switches from an 'mi' range to an 'ma' range. Follows helix convention.
    fn surround(
        &self,
        map: &DisplaySnapshot,
        inner_range: Range<DisplayPoint>,
    ) -> Range<DisplayPoint>;
    /// Whether these objects can be inside ones of the same kind.
    /// If so, the trait assumes they can have zero width.
    fn can_be_nested(&self) -> bool;
    /// The next end since `start` (inclusive) on the same nesting level.
    fn close_at_end(&self, start: DisplayPoint, map: &DisplaySnapshot) -> Option<DisplayPoint> {
        if !self.can_be_nested() {
            return self.next_end(map, movement::right(map, start));
        }
        let mut end_search_start = start;
        let mut start_search_start = movement::right(map, start);
        loop {
            let next_end = self.next_end(map, end_search_start)?;
            let maybe_next_start = self.next_start(map, start_search_start);
            if let Some(next_start) = maybe_next_start
                && next_start <= next_end
            {
                let closing = self.close_at_end(next_start, map)?;
                end_search_start = movement::right(map, closing);
                start_search_start = movement::right(map, closing);
                continue;
            } else {
                return Some(next_end);
            }
        }
    }
    /// The previous start since `end` (inclusive) on the same nesting level.
    fn close_at_start(&self, end: DisplayPoint, map: &DisplaySnapshot) -> Option<DisplayPoint> {
        if !self.can_be_nested() {
            return self.previous_start(map, end);
        }
        let mut start_search_start = end;
        let mut end_search_start = movement::left(map, end);
        loop {
            let prev_start = self.previous_start(map, start_search_start)?;
            let maybe_prev_end = self.previous_end(map, end_search_start);
            if let Some(prev_end) = maybe_prev_end
                && prev_end >= prev_start
            {
                let closing = self.close_at_start(prev_end, map)?;
                end_search_start = movement::left(map, closing);
                start_search_start = movement::left(map, closing);
                continue;
            } else {
                return Some(prev_start);
            }
        }
    }
}

impl<B: BoundedObject> HelixTextObject for B {
    fn range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let start = self.close_at_start(relative_to.start, map)?;
        let end = self.close_at_end(start, map)?;
        if end < relative_to.end {
            return None;
        }

        if around {
            Some(self.surround(map, start..end))
        } else {
            Some(start..end)
        }
    }

    fn next_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let start = self.next_start(map, relative_to.end)?;
        let end = self.close_at_end(start, map)?;
        let range = if around {
            self.surround(map, start..end)
        } else {
            start..end
        };

        Some(range)
    }

    fn previous_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let end = self.previous_end(map, relative_to.start)?;
        let start = self.close_at_start(end, map)?;
        let range = if around {
            self.surround(map, start..end)
        } else {
            start..end
        };

        Some(range)
    }
}

/// A textobject whose boundaries can easily be found between two chars
pub enum ImmediateBoundary {
    Word { ignore_punctuation: bool },
    Subword { ignore_punctuation: bool },
    AngleBrackets,
    BackQuotes,
    CurlyBrackets,
    DoubleQuotes,
    Parentheses,
    SingleQuotes,
    SquareBrackets,
    VerticalBars,
}

/// A textobject whose start and end can be found from an easy-to-find
/// boundary between two chars by following a simple path from there
pub enum FuzzyBoundary {
    Sentence,
    Paragraph,
}

impl ImmediateBoundary {
    fn is_start(&self, left: char, right: char, classifier: CharClassifier) -> bool {
        match self {
            Self::Word { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(*ignore_punctuation);
                is_word_start(left, right, &classifier)
                    || (is_buffer_start(left) && classifier.kind(right) != CharKind::Whitespace)
            }
            Self::Subword { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(*ignore_punctuation);
                movement::is_subword_start(left, right, &classifier)
                    || (is_buffer_start(left) && classifier.kind(right) != CharKind::Whitespace)
            }
            Self::AngleBrackets => left == '<',
            Self::BackQuotes => left == '`',
            Self::CurlyBrackets => left == '{',
            Self::DoubleQuotes => left == '"',
            Self::Parentheses => left == '(',
            Self::SingleQuotes => left == '\'',
            Self::SquareBrackets => left == '[',
            Self::VerticalBars => left == '|',
        }
    }

    fn is_end(&self, left: char, right: char, classifier: CharClassifier) -> bool {
        match self {
            Self::Word { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(*ignore_punctuation);
                is_word_end(left, right, &classifier)
                    || (is_buffer_end(right) && classifier.kind(left) != CharKind::Whitespace)
            }
            Self::Subword { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(*ignore_punctuation);
                movement::is_subword_start(left, right, &classifier)
                    || (is_buffer_end(right) && classifier.kind(left) != CharKind::Whitespace)
            }
            Self::AngleBrackets => right == '>',
            Self::BackQuotes => right == '`',
            Self::CurlyBrackets => right == '}',
            Self::DoubleQuotes => right == '"',
            Self::Parentheses => right == ')',
            Self::SingleQuotes => right == '\'',
            Self::SquareBrackets => right == ']',
            Self::VerticalBars => right == '|',
        }
    }
}

impl BoundedObject for ImmediateBoundary {
    fn next_start(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        try_find_boundary(map, from, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(from.to_offset(map, Bias::Left));
            self.is_start(left, right, classifier)
        })
    }
    fn next_end(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        try_find_boundary(map, from, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(from.to_offset(map, Bias::Left));
            self.is_end(left, right, classifier)
        })
    }
    fn previous_start(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        try_find_preceding_boundary(map, from, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(from.to_offset(map, Bias::Left));
            self.is_start(left, right, classifier)
        })
    }
    fn previous_end(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        try_find_preceding_boundary(map, from, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(from.to_offset(map, Bias::Left));
            self.is_end(left, right, classifier)
        })
    }
    fn surround(
        &self,
        map: &DisplaySnapshot,
        inner_range: Range<DisplayPoint>,
    ) -> Range<DisplayPoint> {
        match self {
            Self::AngleBrackets
            | Self::BackQuotes
            | Self::CurlyBrackets
            | Self::DoubleQuotes
            | Self::Parentheses
            | Self::SingleQuotes
            | Self::SquareBrackets
            | Self::VerticalBars => {
                movement::left(map, inner_range.start)..movement::right(map, inner_range.end)
            }
            Self::Subword { .. } | Self::Word { .. } => {
                let row = inner_range.end.row();
                let line_start = DisplayPoint::new(row, 0);
                let line_end = DisplayPoint::new(row, map.line_len(row));
                let next_start = self.next_start(map, inner_range.end).unwrap().min(line_end);
                let prev_end = self
                    .previous_end(map, inner_range.start)
                    .unwrap()
                    .max(line_start);
                if next_start > inner_range.end {
                    inner_range.start..next_start
                } else {
                    prev_end..inner_range.end
                }
            }
        }
    }
    fn can_be_nested(&self) -> bool {
        match self {
            Self::Subword { .. } | Self::Word { .. } => false,
            _ => true,
        }
    }
}

impl FuzzyBoundary {
    /// When between two chars that form an easy-to-find identifier boundary,
    /// what's the way to get to the actual start of the object, if any
    fn is_near_potential_start<'a>(
        &self,
        left: char,
        right: char,
        classifier: &CharClassifier,
    ) -> Option<Box<dyn Fn(DisplayPoint, &'a DisplaySnapshot) -> Option<DisplayPoint>>> {
        if is_buffer_start(left) {
            return Some(Box::new(|identifier, _| Some(identifier)));
        }
        match self {
            Self::Paragraph => {
                if left != '\n' || right != '\n' {
                    return None;
                }
                Some(Box::new(|identifier, map| {
                    try_find_boundary(map, identifier, |left, right| left == '\n' && right != '\n')
                }))
            }
            Self::Sentence => {
                if let Some(find_paragraph_start) =
                    Self::Paragraph.is_near_potential_start(left, right, classifier)
                {
                    return Some(find_paragraph_start);
                } else if !is_sentence_end(left, right, classifier) {
                    return None;
                }
                Some(Box::new(|identifier, map| {
                    let word = ImmediateBoundary::Word {
                        ignore_punctuation: false,
                    };
                    word.next_start(map, identifier)
                }))
            }
        }
    }
    /// When between two chars that form an easy-to-find identifier boundary,
    /// what's the way to get to the actual end of the object, if any
    fn is_near_potential_end<'a>(
        &self,
        left: char,
        right: char,
        classifier: &CharClassifier,
    ) -> Option<Box<dyn Fn(DisplayPoint, &'a DisplaySnapshot) -> Option<DisplayPoint>>> {
        if is_buffer_end(right) {
            return Some(Box::new(|identifier, _| Some(identifier)));
        }
        match self {
            Self::Paragraph => {
                if left != '\n' || right != '\n' {
                    return None;
                }
                Some(Box::new(|identifier, map| {
                    try_find_preceding_boundary(map, identifier, |left, right| {
                        left != '\n' && right == '\n'
                    })
                }))
            }
            Self::Sentence => {
                if let Some(find_paragraph_end) =
                    Self::Paragraph.is_near_potential_end(left, right, classifier)
                {
                    return Some(find_paragraph_end);
                } else if !is_sentence_end(left, right, classifier) {
                    return None;
                }
                Some(Box::new(|identifier, _| Some(identifier)))
            }
        }
    }
}

impl BoundedObject for FuzzyBoundary {
    fn next_start(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        let mut previous_search_start = from;
        while let Some((identifier, reach_start)) =
            try_find_boundary_data(map, previous_search_start, |left, right, point| {
                let classifier = map
                    .buffer_snapshot
                    .char_classifier_at(point.to_offset(map, Bias::Left));
                self.is_near_potential_start(left, right, &classifier)
                    .map(|reach_start| (point, reach_start))
            })
        {
            let Some(start) = reach_start(identifier, map) else {
                continue;
            };
            if start < from {
                previous_search_start = movement::right(map, identifier);
            } else {
                return Some(start);
            }
        }
        None
    }
    fn next_end(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        let mut previous_search_start = from;
        while let Some((identifier, reach_end)) =
            try_find_boundary_data(map, previous_search_start, |left, right, point| {
                let classifier = map
                    .buffer_snapshot
                    .char_classifier_at(point.to_offset(map, Bias::Left));
                self.is_near_potential_end(left, right, &classifier)
                    .map(|reach_end| (point, reach_end))
            })
        {
            let Some(end) = reach_end(identifier, map) else {
                continue;
            };
            if end < from {
                previous_search_start = movement::right(map, identifier);
            } else {
                return Some(end);
            }
        }
        None
    }
    fn previous_start(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        let mut previous_search_start = from;
        while let Some((identifier, reach_start)) =
            try_find_preceding_boundary_data(map, previous_search_start, |left, right, point| {
                let classifier = map
                    .buffer_snapshot
                    .char_classifier_at(point.to_offset(map, Bias::Left));
                self.is_near_potential_start(left, right, &classifier)
                    .map(|reach_start| (point, reach_start))
            })
        {
            let Some(start) = reach_start(identifier, map) else {
                continue;
            };
            if start > from {
                previous_search_start = movement::left(map, identifier);
            } else {
                return Some(start);
            }
        }
        None
    }
    fn previous_end(&self, map: &DisplaySnapshot, from: DisplayPoint) -> Option<DisplayPoint> {
        let mut previous_search_start = from;
        while let Some((identifier, reach_end)) =
            try_find_preceding_boundary_data(map, previous_search_start, |left, right, point| {
                let classifier = map
                    .buffer_snapshot
                    .char_classifier_at(point.to_offset(map, Bias::Left));
                self.is_near_potential_end(left, right, &classifier)
                    .map(|reach_end| (point, reach_end))
            })
        {
            let Some(end) = reach_end(identifier, map) else {
                continue;
            };
            if end > from {
                previous_search_start = movement::left(map, identifier);
            } else {
                return Some(end);
            }
        }
        None
    }
    fn surround(
        &self,
        map: &DisplaySnapshot,
        inner_range: Range<DisplayPoint>,
    ) -> Range<DisplayPoint> {
        let next_start = self
            .next_start(map, inner_range.end)
            .unwrap_or(map.max_point());
        if next_start > inner_range.end {
            return inner_range.start..next_start;
        }
        let previous_end = self
            .previous_end(map, inner_range.end)
            .unwrap_or(DisplayPoint::zero());
        previous_end..inner_range.end
    }
    fn can_be_nested(&self) -> bool {
        false
    }
}

/// Returns the first boundary after or at `from` in text direction.
/// The start and end of the file are the chars `'\0'`.
fn try_find_boundary(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    is_boundary: impl Fn(char, char) -> bool,
) -> Option<DisplayPoint> {
    let boundary = try_find_boundary_data(map, from, |left, right, point| {
        if is_boundary(left, right) {
            Some(point)
        } else {
            None
        }
    })?;
    Some(boundary)
}

/// Returns some information about it (of type `T`) as soon as
/// there is a boundary after or at `from` in text direction
/// The start and end of the file are the chars `'\0'`.
fn try_find_boundary_data<T>(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    boundary_information: impl Fn(char, char, DisplayPoint) -> Option<T>,
) -> Option<T> {
    let mut offset = from.to_offset(map, Bias::Right);
    let mut prev_ch = map
        .buffer_snapshot
        .reversed_chars_at(offset)
        .next()
        .unwrap_or('\0');

    for ch in map.buffer_snapshot.chars_at(offset).chain(['\0']) {
        let display_point = offset.to_display_point(map);
        if let Some(boundary_information) = boundary_information(prev_ch, ch, display_point) {
            return Some(boundary_information);
        }
        offset += ch.len_utf8();
        prev_ch = ch;
    }

    None
}

/// Returns the first boundary after or at `from` in text direction.
/// The start and end of the file are the chars `'\0'`.
fn try_find_preceding_boundary(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    is_boundary: impl Fn(char, char) -> bool,
) -> Option<DisplayPoint> {
    let boundary = try_find_preceding_boundary_data(map, from, |left, right, point| {
        if is_boundary(left, right) {
            Some(point)
        } else {
            None
        }
    })?;
    Some(boundary)
}

/// Returns some information about it (of type `T`) as soon as
/// there is a boundary before or at `from` in opposite text direction
/// The start and end of the file are the chars `'\0'`.
fn try_find_preceding_boundary_data<T>(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    is_boundary: impl Fn(char, char, DisplayPoint) -> Option<T>,
) -> Option<T> {
    let mut offset = from.to_offset(map, Bias::Left);
    let mut prev_ch = map.buffer_snapshot.chars_at(offset).next().unwrap_or('\0');

    for ch in map.buffer_snapshot.reversed_chars_at(offset).chain(['\0']) {
        let display_point = offset.to_display_point(map);
        if let Some(boundary_information) = is_boundary(ch, prev_ch, display_point) {
            return Some(boundary_information);
        }
        offset = offset.saturating_sub(ch.len_utf8());
        prev_ch = ch;
    }

    None
}

fn is_buffer_start(left: char) -> bool {
    left == '\0'
}

fn is_buffer_end(right: char) -> bool {
    right == '\0'
}

fn is_word_start(left: char, right: char, classifier: &CharClassifier) -> bool {
    classifier.kind(left) != classifier.kind(right)
        && classifier.kind(right) != CharKind::Whitespace
}

fn is_word_end(left: char, right: char, classifier: &CharClassifier) -> bool {
    classifier.kind(left) != classifier.kind(right) && classifier.kind(left) != CharKind::Whitespace
}

fn is_sentence_end(left: char, right: char, classifier: &CharClassifier) -> bool {
    const ENDS: [char; 1] = ['.'];

    if classifier.kind(right) != CharKind::Whitespace {
        return false;
    }
    ENDS.into_iter().any(|end| left == end)
}
