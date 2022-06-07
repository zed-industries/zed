pub use buffer_search::BufferSearchBar;
use editor::{display_map::ToDisplayPoint, Anchor, Bias, Editor, MultiBufferSnapshot};
use gpui::{actions, impl_internal_actions, MutableAppContext, ViewHandle};
pub use project_search::{ProjectSearchBar, ProjectSearchView};
use std::{
    cmp::{self, Ordering},
    ops::Range,
};

pub mod buffer_search;
pub mod project_search;

pub fn init(cx: &mut MutableAppContext) {
    buffer_search::init(cx);
    project_search::init(cx);
}

#[derive(Clone, PartialEq)]
pub struct ToggleSearchOption(pub SearchOption);

actions!(search, [SelectNextMatch, SelectPrevMatch]);
impl_internal_actions!(search, [ToggleSearchOption]);

#[derive(Clone, Copy, PartialEq)]
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
            if probe.end.cmp(&cursor, &*buffer).is_lt() {
                Ordering::Less
            } else if probe.start.cmp(&cursor, &*buffer).is_gt() {
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
    if ranges[index].start.cmp(&cursor, &buffer).is_gt() {
        if direction == Direction::Prev {
            if index == 0 {
                index = ranges.len() - 1;
            } else {
                index -= 1;
            }
        }
    } else if ranges[index].end.cmp(&cursor, &buffer).is_lt() {
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

pub(crate) fn query_suggestion_for_editor(
    editor: &ViewHandle<Editor>,
    cx: &mut MutableAppContext,
) -> String {
    let display_map = editor
        .update(cx, |editor, cx| editor.snapshot(cx))
        .display_snapshot;
    let selection = editor.read(cx).selections.newest::<usize>(cx);
    if selection.start == selection.end {
        let point = selection.start.to_display_point(&display_map);
        let range = editor::movement::surrounding_word(&display_map, point);
        let range = range.start.to_offset(&display_map, Bias::Left)
            ..range.end.to_offset(&display_map, Bias::Right);
        let text: String = display_map.buffer_snapshot.text_for_range(range).collect();
        if text.trim().is_empty() {
            String::new()
        } else {
            text
        }
    } else {
        display_map
            .buffer_snapshot
            .text_for_range(selection.start..selection.end)
            .collect()
    }
}
