use std::{cmp::Ordering, ops::Range};

use editor::{
    DisplayPoint, MultiBufferOffset,
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
    /// If outer is true it is the start of "a" object (m a) rather than "inner" object (m i).
    fn next_start(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset>;
    /// The next end since `from` (inclusive).
    /// If outer is true it is the end of "a" object (m a) rather than "inner" object (m i).
    fn next_end(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset>;
    /// The previous start since `from` (inclusive).
    /// If outer is true it is the start of "a" object (m a) rather than "inner" object (m i).
    fn previous_start(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset>;
    /// The previous end since `from` (inclusive).
    /// If outer is true it is the end of "a" object (m a) rather than "inner" object (m i).
    fn previous_end(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset>;

    /// Whether the range inside the object can be zero characters wide.
    /// If so, the trait assumes that these ranges can't be directly adjacent to each other.
    fn inner_range_can_be_zero_width(&self) -> bool;
    /// Whether the "ma" can exceed the "mi" range on both sides at the same time
    fn surround_on_both_sides(&self) -> bool;
    /// Whether the outer range of an object could overlap with the outer range of the neighboring
    /// object. If so, they can't be nested.
    fn ambiguous_outer(&self) -> bool;

    fn can_be_zero_width(&self, around: bool) -> bool {
        if around {
            false
        } else {
            self.inner_range_can_be_zero_width()
        }
    }

    /// Switches from an "mi" range to an "ma" one.
    /// Assumes the inner range is valid.
    fn around(&self, map: &DisplaySnapshot, inner_range: Range<Offset>) -> Range<Offset> {
        if self.surround_on_both_sides() {
            let start = self
                .previous_start(map, inner_range.start, true)
                .unwrap_or(inner_range.start);
            let end = self
                .next_end(map, inner_range.end, true)
                .unwrap_or(inner_range.end);

            return start..end;
        }

        let mut start = inner_range.start;
        let end = self
            .next_end(map, inner_range.end, true)
            .unwrap_or(inner_range.end);
        if end == inner_range.end {
            start = self
                .previous_start(map, inner_range.start, true)
                .unwrap_or(inner_range.start)
        }

        start..end
    }
    /// Switches from an "ma" range to an "mi" one.
    /// Assumes the inner range is valid.
    fn inside(&self, map: &DisplaySnapshot, outer_range: Range<Offset>) -> Range<Offset> {
        let inner_start = self
            .next_start(map, outer_range.start, false)
            .unwrap_or_else(|| {
                log::warn!("The motion might not have found the text object correctly");
                outer_range.start
            });
        let inner_end = self
            .previous_end(map, outer_range.end, false)
            .unwrap_or_else(|| {
                log::warn!("The motion might not have found the text object correctly");
                outer_range.end
            });
        inner_start..inner_end
    }

    /// The next end since `start` (inclusive) on the same nesting level.
    fn close_at_end(&self, start: Offset, map: &DisplaySnapshot, outer: bool) -> Option<Offset> {
        let mut end_search_start = if self.can_be_zero_width(outer) {
            start
        } else {
            start.next(map)?
        };
        let mut start_search_start = start.next(map)?;

        loop {
            let next_end = self.next_end(map, end_search_start, outer)?;
            let maybe_next_start = self.next_start(map, start_search_start, outer);
            if let Some(next_start) = maybe_next_start
                && (next_start.0 < next_end.0
                    || next_start.0 == next_end.0 && self.can_be_zero_width(outer))
                && !self.ambiguous_outer()
            {
                let closing = self.close_at_end(next_start, map, outer)?;
                end_search_start = closing.next(map)?;
                start_search_start = if self.can_be_zero_width(outer) {
                    closing.next(map)?
                } else {
                    closing
                };
            } else {
                return Some(next_end);
            }
        }
    }
    /// The previous start since `end` (inclusive) on the same nesting level.
    fn close_at_start(&self, end: Offset, map: &DisplaySnapshot, outer: bool) -> Option<Offset> {
        let mut start_search_end = if self.can_be_zero_width(outer) {
            end
        } else {
            end.previous(map)?
        };
        let mut end_search_end = end.previous(map)?;

        loop {
            let previous_start = self.previous_start(map, start_search_end, outer)?;
            let maybe_previous_end = self.previous_end(map, end_search_end, outer);
            if let Some(previous_end) = maybe_previous_end
                && (previous_end.0 > previous_start.0
                    || previous_end.0 == previous_start.0 && self.can_be_zero_width(outer))
                && !self.ambiguous_outer()
            {
                let closing = self.close_at_start(previous_end, map, outer)?;
                start_search_end = closing.previous(map)?;
                end_search_end = if self.can_be_zero_width(outer) {
                    closing.previous(map)?
                } else {
                    closing
                };
            } else {
                return Some(previous_start);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd, Ord, Eq)]
struct Offset(MultiBufferOffset);
impl Offset {
    fn next(self, map: &DisplaySnapshot) -> Option<Self> {
        let next = Self(
            map.buffer_snapshot()
                .clip_offset(self.0 + 1usize, Bias::Right),
        );
        (next.0 > self.0).then(|| next)
    }
    fn previous(self, map: &DisplaySnapshot) -> Option<Self> {
        if self.0 == MultiBufferOffset(0) {
            return None;
        }
        Some(Self(
            map.buffer_snapshot().clip_offset(self.0 - 1, Bias::Left),
        ))
    }
    fn range(
        start: (DisplayPoint, Bias),
        end: (DisplayPoint, Bias),
        map: &DisplaySnapshot,
    ) -> Range<Self> {
        Self(start.0.to_offset(map, start.1))..Self(end.0.to_offset(map, end.1))
    }
}

impl<B: BoundedObject> HelixTextObject for B {
    fn range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = Offset::range(
            (relative_to.start, Bias::Left),
            (relative_to.end, Bias::Left),
            map,
        );

        relative_range(self, around, map, |find_outer| {
            let search_start = if self.can_be_zero_width(find_outer) {
                relative_to.end
            } else {
                // If the objects can be directly next to each other an object end the
                // cursor (relative_to) end would not count for close_at_end, so the search
                // needs to start one character to the left.
                relative_to.end.previous(map)?
            };
            let max_end = self.close_at_end(search_start, map, find_outer)?;
            let min_start = self.close_at_start(max_end, map, find_outer)?;

            (min_start <= relative_to.start).then(|| min_start..max_end)
        })
    }

    fn next_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = Offset::range(
            (relative_to.start, Bias::Left),
            (relative_to.end, Bias::Left),
            map,
        );

        relative_range(self, around, map, |find_outer| {
            let min_start = self.next_start(map, relative_to.end, find_outer)?;
            let max_end = self.close_at_end(min_start, map, find_outer)?;

            Some(min_start..max_end)
        })
    }

    fn previous_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = Offset::range(
            (relative_to.start, Bias::Left),
            (relative_to.end, Bias::Left),
            map,
        );

        relative_range(self, around, map, |find_outer| {
            let max_end = self.previous_end(map, relative_to.start, find_outer)?;
            let min_start = self.close_at_start(max_end, map, find_outer)?;

            Some(min_start..max_end)
        })
    }
}

fn relative_range<B: BoundedObject>(
    object: &B,
    outer: bool,
    map: &DisplaySnapshot,
    find_range: impl Fn(bool) -> Option<Range<Offset>>,
) -> Option<Range<DisplayPoint>> {
    // The cursor could be inside the outer range, but not the inner range.
    // Whether that should count as found.
    let find_outer = object.surround_on_both_sides() && !object.ambiguous_outer();
    let range = find_range(find_outer)?;
    let min_start = range.start;
    let max_end = range.end;

    let wanted_range = if outer && !find_outer {
        // max_end is not yet the outer end
        object.around(map, min_start..max_end)
    } else if !outer && find_outer {
        // max_end is the outer end, but the final result should have the inner end
        object.inside(map, min_start..max_end)
    } else {
        min_start..max_end
    };

    let start = wanted_range.start.0.to_display_point(map);
    let end = wanted_range.end.0.to_display_point(map);

    Some(start..end)
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
    fn is_inner_start(&self, left: char, right: char, classifier: CharClassifier) -> bool {
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
    fn is_inner_end(&self, left: char, right: char, classifier: CharClassifier) -> bool {
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
    fn is_outer_start(&self, left: char, right: char, classifier: CharClassifier) -> bool {
        match self {
            word @ Self::Word { .. } => word.is_inner_end(left, right, classifier) || left == '\n',
            subword @ Self::Subword { .. } => {
                subword.is_inner_end(left, right, classifier) || left == '\n'
            }
            Self::AngleBrackets => right == '<',
            Self::BackQuotes => right == '`',
            Self::CurlyBrackets => right == '{',
            Self::DoubleQuotes => right == '"',
            Self::Parentheses => right == '(',
            Self::SingleQuotes => right == '\'',
            Self::SquareBrackets => right == '[',
            Self::VerticalBars => right == '|',
        }
    }
    fn is_outer_end(&self, left: char, right: char, classifier: CharClassifier) -> bool {
        match self {
            word @ Self::Word { .. } => {
                word.is_inner_start(left, right, classifier) || right == '\n'
            }
            subword @ Self::Subword { .. } => {
                subword.is_inner_start(left, right, classifier) || right == '\n'
            }
            Self::AngleBrackets => left == '>',
            Self::BackQuotes => left == '`',
            Self::CurlyBrackets => left == '}',
            Self::DoubleQuotes => left == '"',
            Self::Parentheses => left == ')',
            Self::SingleQuotes => left == '\'',
            Self::SquareBrackets => left == ']',
            Self::VerticalBars => left == '|',
        }
    }
}

impl BoundedObject for ImmediateBoundary {
    fn next_start(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        try_find_boundary(map, from, |left, right| {
            let classifier = map.buffer_snapshot().char_classifier_at(from.0);
            if outer {
                self.is_outer_start(left, right, classifier)
            } else {
                self.is_inner_start(left, right, classifier)
            }
        })
    }
    fn next_end(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        try_find_boundary(map, from, |left, right| {
            let classifier = map.buffer_snapshot().char_classifier_at(from.0);
            if outer {
                self.is_outer_end(left, right, classifier)
            } else {
                self.is_inner_end(left, right, classifier)
            }
        })
    }
    fn previous_start(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        try_find_preceding_boundary(map, from, |left, right| {
            let classifier = map.buffer_snapshot().char_classifier_at(from.0);
            if outer {
                self.is_outer_start(left, right, classifier)
            } else {
                self.is_inner_start(left, right, classifier)
            }
        })
    }
    fn previous_end(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        try_find_preceding_boundary(map, from, |left, right| {
            let classifier = map.buffer_snapshot().char_classifier_at(from.0);
            if outer {
                self.is_outer_end(left, right, classifier)
            } else {
                self.is_inner_end(left, right, classifier)
            }
        })
    }
    fn inner_range_can_be_zero_width(&self) -> bool {
        match self {
            Self::Subword { .. } | Self::Word { .. } => false,
            _ => true,
        }
    }
    fn surround_on_both_sides(&self) -> bool {
        match self {
            Self::Subword { .. } | Self::Word { .. } => false,
            _ => true,
        }
    }
    fn ambiguous_outer(&self) -> bool {
        match self {
            Self::BackQuotes
            | Self::DoubleQuotes
            | Self::SingleQuotes
            | Self::VerticalBars
            | Self::Subword { .. }
            | Self::Word { .. } => true,
            _ => false,
        }
    }
}

impl FuzzyBoundary {
    /// When between two chars that form an easy-to-find identifier boundary,
    /// what's the way to get to the actual start of the object, if any
    fn is_near_potential_inner_start<'a>(
        &self,
        left: char,
        right: char,
        classifier: &CharClassifier,
    ) -> Option<Box<dyn Fn(Offset, &'a DisplaySnapshot) -> Option<Offset>>> {
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
                    Self::Paragraph.is_near_potential_inner_start(left, right, classifier)
                {
                    return Some(find_paragraph_start);
                } else if !is_sentence_end(left, right, classifier) {
                    return None;
                }
                Some(Box::new(|identifier, map| {
                    let word = ImmediateBoundary::Word {
                        ignore_punctuation: false,
                    };
                    word.next_start(map, identifier, false)
                }))
            }
        }
    }
    /// When between two chars that form an easy-to-find identifier boundary,
    /// what's the way to get to the actual end of the object, if any
    fn is_near_potential_inner_end<'a>(
        &self,
        left: char,
        right: char,
        classifier: &CharClassifier,
    ) -> Option<Box<dyn Fn(Offset, &'a DisplaySnapshot) -> Option<Offset>>> {
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
                    Self::Paragraph.is_near_potential_inner_end(left, right, classifier)
                {
                    return Some(find_paragraph_end);
                } else if !is_sentence_end(left, right, classifier) {
                    return None;
                }
                Some(Box::new(|identifier, _| Some(identifier)))
            }
        }
    }
    /// When between two chars that form an easy-to-find identifier boundary,
    /// what's the way to get to the actual end of the object, if any
    fn is_near_potential_outer_start<'a>(
        &self,
        left: char,
        right: char,
        classifier: &CharClassifier,
    ) -> Option<Box<dyn Fn(Offset, &'a DisplaySnapshot) -> Option<Offset>>> {
        match self {
            paragraph @ Self::Paragraph => {
                paragraph.is_near_potential_inner_end(left, right, classifier)
            }
            sentence @ Self::Sentence => {
                sentence.is_near_potential_inner_end(left, right, classifier)
            }
        }
    }
    /// When between two chars that form an easy-to-find identifier boundary,
    /// what's the way to get to the actual end of the object, if any
    fn is_near_potential_outer_end<'a>(
        &self,
        left: char,
        right: char,
        classifier: &CharClassifier,
    ) -> Option<Box<dyn Fn(Offset, &'a DisplaySnapshot) -> Option<Offset>>> {
        match self {
            paragraph @ Self::Paragraph => {
                paragraph.is_near_potential_inner_start(left, right, classifier)
            }
            sentence @ Self::Sentence => {
                sentence.is_near_potential_inner_start(left, right, classifier)
            }
        }
    }

    // The boundary can be on the other side of `from` than the identifier, so the search needs to go both ways.
    // Also, the distance (and direction) between identifier and boundary could vary, so a few ones need to be
    // compared, even if one boundary was already found on the right side of `from`.
    fn to_boundary(
        &self,
        map: &DisplaySnapshot,
        from: Offset,
        outer: bool,
        backward: bool,
        boundary_kind: Boundary,
    ) -> Option<Offset> {
        let generate_boundary_data = |left, right, point: Offset| {
            let classifier = map.buffer_snapshot().char_classifier_at(from.0);
            let reach_boundary = if outer && boundary_kind == Boundary::Start {
                self.is_near_potential_outer_start(left, right, &classifier)
            } else if !outer && boundary_kind == Boundary::Start {
                self.is_near_potential_inner_start(left, right, &classifier)
            } else if outer && boundary_kind == Boundary::End {
                self.is_near_potential_outer_end(left, right, &classifier)
            } else {
                self.is_near_potential_inner_end(left, right, &classifier)
            };

            reach_boundary.map(|reach_start| (point, reach_start))
        };

        let forwards = try_find_boundary_data(map, from, generate_boundary_data);
        let backwards = try_find_preceding_boundary_data(map, from, generate_boundary_data);
        let boundaries = [forwards, backwards]
            .into_iter()
            .flatten()
            .filter_map(|(identifier, reach_boundary)| reach_boundary(identifier, map))
            .filter(|boundary| match boundary.cmp(&from) {
                Ordering::Equal => true,
                Ordering::Less => backward,
                Ordering::Greater => !backward,
            });
        if backward {
            boundaries.max_by_key(|boundary| *boundary)
        } else {
            boundaries.min_by_key(|boundary| *boundary)
        }
    }
}

#[derive(PartialEq)]
enum Boundary {
    Start,
    End,
}

impl BoundedObject for FuzzyBoundary {
    fn next_start(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        self.to_boundary(map, from, outer, false, Boundary::Start)
    }
    fn next_end(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        self.to_boundary(map, from, outer, false, Boundary::End)
    }
    fn previous_start(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        self.to_boundary(map, from, outer, true, Boundary::Start)
    }
    fn previous_end(&self, map: &DisplaySnapshot, from: Offset, outer: bool) -> Option<Offset> {
        self.to_boundary(map, from, outer, true, Boundary::End)
    }
    fn inner_range_can_be_zero_width(&self) -> bool {
        false
    }
    fn surround_on_both_sides(&self) -> bool {
        false
    }
    fn ambiguous_outer(&self) -> bool {
        false
    }
}

/// Returns the first boundary after or at `from` in text direction.
/// The start and end of the file are the chars `'\0'`.
fn try_find_boundary(
    map: &DisplaySnapshot,
    from: Offset,
    is_boundary: impl Fn(char, char) -> bool,
) -> Option<Offset> {
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
    mut from: Offset,
    boundary_information: impl Fn(char, char, Offset) -> Option<T>,
) -> Option<T> {
    let mut prev_ch = map
        .buffer_snapshot()
        .reversed_chars_at(from.0)
        .next()
        .unwrap_or('\0');

    for ch in map.buffer_snapshot().chars_at(from.0).chain(['\0']) {
        if let Some(boundary_information) = boundary_information(prev_ch, ch, from) {
            return Some(boundary_information);
        }
        from.0 += ch.len_utf8();
        prev_ch = ch;
    }

    None
}

/// Returns the first boundary after or at `from` in text direction.
/// The start and end of the file are the chars `'\0'`.
fn try_find_preceding_boundary(
    map: &DisplaySnapshot,
    from: Offset,
    is_boundary: impl Fn(char, char) -> bool,
) -> Option<Offset> {
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
    mut from: Offset,
    is_boundary: impl Fn(char, char, Offset) -> Option<T>,
) -> Option<T> {
    let mut prev_ch = map
        .buffer_snapshot()
        .chars_at(from.0)
        .next()
        .unwrap_or('\0');

    for ch in map
        .buffer_snapshot()
        .reversed_chars_at(from.0)
        .chain(['\0'])
    {
        if let Some(boundary_information) = is_boundary(ch, prev_ch, from) {
            return Some(boundary_information);
        }
        from.0.0 = from.0.0.saturating_sub(ch.len_utf8());
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
