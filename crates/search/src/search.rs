use std::{
    cmp::{self, Ordering},
    ops::Range,
};

use editor::{Anchor, MultiBufferSnapshot};
use gpui::{action, MutableAppContext};

mod buffer_search;
mod project_search;

pub fn init(cx: &mut MutableAppContext) {
    buffer_search::init(cx);
    project_search::init(cx);
}

action!(ToggleSearchOption, SearchOption);
action!(SelectMatch, Direction);

#[derive(Clone, Copy)]
pub enum SearchOption {
    WholeWord,
    CaseSensitive,
    Regex,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

pub(crate) fn active_match_index(
    ranges: &[Range<Anchor>],
    cursor: &Anchor,
    buffer: &MultiBufferSnapshot,
) -> Option<usize> {
    if ranges.is_empty() {
        None
    } else {
        match ranges.binary_search_by(|probe| {
            if probe.end.cmp(&cursor, &*buffer).unwrap().is_lt() {
                Ordering::Less
            } else if probe.start.cmp(&cursor, &*buffer).unwrap().is_gt() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }) {
            Ok(i) | Err(i) => Some(cmp::min(i, ranges.len() - 1)),
        }
    }
}

pub(crate) fn match_index_for_direction(
    ranges: &[Range<Anchor>],
    cursor: &Anchor,
    mut index: usize,
    direction: Direction,
    buffer: &MultiBufferSnapshot,
) -> usize {
    if ranges[index].start.cmp(&cursor, &buffer).unwrap().is_gt() {
        if direction == Direction::Prev {
            if index == 0 {
                index = ranges.len() - 1;
            } else {
                index -= 1;
            }
        }
    } else if ranges[index].end.cmp(&cursor, &buffer).unwrap().is_lt() {
        if direction == Direction::Next {
            index = 0;
        }
    } else if direction == Direction::Prev {
        if index == 0 {
            index = ranges.len() - 1;
        } else {
            index -= 1;
        }
    } else if direction == Direction::Next {
        if index == ranges.len() - 1 {
            index = 0
        } else {
            index += 1;
        }
    };
    index
}
