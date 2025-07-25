use std::{error::Error, fmt::Display};

use editor::{
    DisplayPoint,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
};
use language::{CharClassifier, CharKind};
use text::Bias;

use crate::object::Object;

#[derive(Debug)]
pub struct UnboundedErr;
impl Display for UnboundedErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "object can't be found with simple boundary checking")
    }
}
impl Error for UnboundedErr {}

impl Object {
    /// Returns the beginning of the inside of the closest object after the cursor if it can easily be found.
    /// Follows helix convention.
    pub fn helix_next_start(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<DisplayPoint>, UnboundedErr> {
        try_find_boundary(map, relative_to, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(relative_to.to_point(map));
            self.helix_is_start(right, left, classifier)
        })
    }
    /// Returns the end of the inside of the closest object after the cursor if it can easily be found.
    /// Follows helix convention.
    pub fn helix_next_end(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<DisplayPoint>, UnboundedErr> {
        try_find_boundary(map, relative_to, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(relative_to.to_point(map));
            self.helix_is_end(right, left, classifier)
        })
    }
    /// Returns the beginning of the inside of the closest object before the cursor if it can easily be found.
    /// Follows helix convention.
    pub fn helix_previous_start(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<DisplayPoint>, UnboundedErr> {
        try_find_preceding_boundary(map, relative_to, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(relative_to.to_point(map));
            self.helix_is_start(right, left, classifier)
        })
    }
    /// Returns the end of the inside of the closest object before the cursor if it can easily be found.
    /// Follows helix convention.
    pub fn helix_previous_end(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<DisplayPoint>, UnboundedErr> {
        try_find_preceding_boundary(map, relative_to, |left, right| {
            let classifier = map
                .buffer_snapshot
                .char_classifier_at(relative_to.to_point(map));
            self.helix_is_end(right, left, classifier)
        })
    }

    fn helix_is_start(
        self,
        right: char,
        left: char,
        classifier: CharClassifier,
    ) -> Result<bool, UnboundedErr> {
        match self {
            Self::Word { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(ignore_punctuation);
                Ok(is_word_start(left, right, &classifier)
                    || (is_buffer_start(left) && classifier.kind(right) != CharKind::Whitespace))
            }
            Self::Subword { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(ignore_punctuation);
                Ok(movement::is_subword_start(left, right, &classifier)
                    || (is_buffer_start(left) && classifier.kind(right) != CharKind::Whitespace))
            }
            Self::AngleBrackets => Ok(left == '<'),
            Self::BackQuotes => Ok(left == '`'),
            Self::CurlyBrackets => Ok(left == '{'),
            Self::DoubleQuotes => Ok(left == '"'),
            Self::Parentheses => Ok(left == '('),
            Self::SquareBrackets => Ok(left == '['),
            Self::VerticalBars => Ok(left == '|'),
            _ => Err(UnboundedErr),
        }
    }

    fn helix_is_end(
        self,
        right: char,
        left: char,
        classifier: CharClassifier,
    ) -> Result<bool, UnboundedErr> {
        match self {
            Self::Word { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(ignore_punctuation);
                Ok(is_word_end(left, right, &classifier)
                    || (is_buffer_end(right) && classifier.kind(left) != CharKind::Whitespace))
            }
            Self::Subword { ignore_punctuation } => {
                let classifier = classifier.ignore_punctuation(ignore_punctuation);
                Ok(movement::is_subword_end(left, right, &classifier)
                    || (is_buffer_end(right) && classifier.kind(right) != CharKind::Whitespace))
            }
            Self::AngleBrackets => Ok(right == '>'),
            Self::BackQuotes => Ok(right == '`'),
            Self::CurlyBrackets => Ok(right == '}'),
            Self::DoubleQuotes => Ok(right == '"'),
            Self::Parentheses => Ok(right == ')'),
            Self::SquareBrackets => Ok(right == ']'),
            Self::VerticalBars => Ok(right == '|'),
            Self::Sentence => Ok(left == '.'),
            _ => Err(UnboundedErr),
        }
    }
}

fn try_find_boundary(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> Result<bool, UnboundedErr>,
) -> Result<Option<DisplayPoint>, UnboundedErr> {
    let mut offset = from.to_offset(map, Bias::Right);
    let mut prev_ch = map
        .buffer_snapshot
        .reversed_chars_at(offset)
        .next()
        .unwrap_or('\0');

    for ch in map.buffer_snapshot.chars_at(offset).chain(['\0']) {
        if is_boundary(prev_ch, ch)? {
            return Ok(Some(
                map.clip_point(offset.to_display_point(map), Bias::Right),
            ));
        }
        offset += ch.len_utf8();
        prev_ch = ch;
    }

    Ok(None)
}

fn try_find_preceding_boundary(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> Result<bool, UnboundedErr>,
) -> Result<Option<DisplayPoint>, UnboundedErr> {
    let mut offset = from.to_offset(map, Bias::Right);
    let mut prev_ch = map.buffer_snapshot.chars_at(offset).next().unwrap_or('\0');

    for ch in map.buffer_snapshot.reversed_chars_at(offset).chain(['\0']) {
        if is_boundary(ch, prev_ch)? {
            return Ok(Some(
                map.clip_point(offset.to_display_point(map), Bias::Right),
            ));
        }
        offset = offset.saturating_sub(ch.len_utf8());
        prev_ch = ch;
    }

    Ok(None)
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
