use futures::{Future, FutureExt};
use language::{char_kind, coerce_punctuation, CharKind};
use std::{ops::Range, sync::Arc};

use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement::{find_boundary_range, TextLayoutDetails},
    DisplayPoint, Editor,
};
use gpui::{point, Bounds, EntityId, Point, Task};
use project::search::SearchQuery;
use text::{Bias, Selection};
use ui::{Pixels, ViewContext};
use workspace::searchable::SearchableItem;

use crate::{
    util::{ranges, window_bottom, window_top},
    Direction, WordType,
};

pub fn word_starts_in_range(
    map: &DisplaySnapshot,
    mut from: DisplayPoint,
    to: DisplayPoint,
    full_word: bool,
) -> Vec<DisplayPoint> {
    let scope = map.buffer_snapshot.language_scope_at(from.to_point(map));
    let mut result = Vec::new();

    if from.is_zero() {
        let offset = from.to_offset(map, text::Bias::Left);
        let first_char = map.buffer_snapshot.chars_at(offset).next();
        if let Some(first_char) = first_char {
            if char_kind(&scope, first_char) == CharKind::Word {
                result.push(DisplayPoint::zero());
            }
        }
    }

    while from < to {
        let new_point = find_boundary_range(map, from, to, |left, right| {
            let left_kind = coerce_punctuation(char_kind(&scope, left), false);
            let right_kind = coerce_punctuation(char_kind(&scope, right), false);
            // TODO ignore just punctuation words i.e. ' {} '?
            let found = if full_word {
                left_kind == CharKind::Whitespace && right_kind == CharKind::Word
            } else {
                left_kind != right_kind && right_kind == CharKind::Word
            };

            found
        });

        let Some(new_point) = new_point else {
            break;
        };
        if from == new_point {
            break;
        }
        result.push(new_point);
        from = new_point;
    }
    result
}

pub fn word_starts(
    word_type: WordType,
    direction: Direction,
    map: &DisplaySnapshot,
    selections: &Selection<DisplayPoint>,
    text_layout_details: &TextLayoutDetails,
) -> Vec<DisplayPoint> {
    let Range { start, end } = ranges(direction, map, selections, text_layout_details);
    let full_word = match word_type {
        WordType::Word => false,
        WordType::FullWord => true,
        _ => false,
    };
    word_starts_in_range(&map, start, end, full_word)
}

pub fn get_word_task(
    word_type: WordType,
    bounding_box: Bounds<Pixels>,
    entity_id: EntityId,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Task<Vec<(DisplayPoint, EntityId, Point<Pixels>)>> {
    let style = cx.text_style();
    let font_size = style.font_size.to_pixels(cx.rem_size());
    let line_height = style.line_height_in_pixels(cx.rem_size());
    let font_id = cx.text_system().resolve_font(&style.font());
    let em_width = cx
        .text_system()
        .typographic_bounds(font_id, font_size, 'm')
        .unwrap()
        .size
        .width;

    let selections = editor.selections.newest_display(cx);
    let snapshot = editor.snapshot(cx);
    let text_layout_details = editor.text_layout_details(cx);

    cx.background_executor().spawn(async move {
        let line_height = line_height;
        let em_width = em_width;

        let selections = selections;
        let snapshot = snapshot;
        let text_layout_details = text_layout_details;
        let map = &snapshot.display_snapshot;
        let scroll_position = snapshot.scroll_position();
        let scroll_pixel_position = point(
            scroll_position.x * em_width,
            scroll_position.y * line_height,
        );
        let bounding_box = bounding_box;

        let words = word_starts(
            word_type,
            Direction::BiDirectional,
            map,
            &selections,
            &text_layout_details,
        );
        let x = words
            .iter()
            .map(|word| {
                bounding_box.origin.x + 2.0 * map.x_for_display_point(*word, &text_layout_details)
            })
            // to get around borrowing issue, just change this and below into boomer loop
            .collect::<Vec<_>>()
            .into_iter();
        words
            .into_iter()
            .zip(x)
            .map(move |(word, x)| {
                let y = bounding_box.origin.y + word.row().0 as f32 * line_height;
                let p = point(x, y) - scroll_pixel_position;
                (word, entity_id, p)
            })
            .collect::<Vec<_>>()
    })
}

pub fn search_window(
    query: &str,
    is_regex: bool,
    direction: Direction,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Option<impl Future<Output = Vec<DisplayPoint>>> {
    let query = if is_regex {
        SearchQuery::regex(query, false, false, false, Vec::new(), Vec::new()).ok()?
    } else {
        SearchQuery::text(query, false, false, false, Vec::new(), Vec::new()).ok()?
    };

    let map = editor.snapshot(cx).display_snapshot;
    let selections = editor.selections.newest_display(cx);
    let range = ranges(
        direction,
        &map,
        &selections,
        &editor.text_layout_details(cx),
    );
    let start = map.display_point_to_anchor(range.start, Bias::Left);
    let end = map.display_point_to_anchor(range.end, Bias::Left);

    let ranges = [start..end];
    editor.set_search_within_ranges(&ranges, cx);
    let matches = editor
        .find_matches(Arc::new(query), cx)
        .then(move |matches| async move {
            let map = map;
            matches
                .into_iter()
                .map(|anchor| anchor.start.to_display_point(&map))
                .collect::<Vec<_>>()
        });
    Some(matches)
}

pub fn search_multipane(
    query: &str,
    is_regex: bool,
    bounding_box: Bounds<Pixels>,
    entity_id: EntityId,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Option<impl Future<Output = Vec<(DisplayPoint, EntityId, Point<Pixels>)>>> {
    let query = if is_regex {
        SearchQuery::regex(query, false, true, false, Vec::new(), Vec::new()).ok()?
    } else {
        SearchQuery::text(query, false, false, false, Vec::new(), Vec::new()).ok()?
    };

    let map = editor.snapshot(cx).display_snapshot;
    let text_layout_details = editor.text_layout_details(cx);
    let start = map.display_point_to_anchor(window_top(&map, &text_layout_details), Bias::Left);
    let end = map.display_point_to_anchor(window_bottom(&map, &text_layout_details), Bias::Left);
    let map = editor.snapshot(cx).display_snapshot;

    let style = cx.text_style();
    let line_height = style
        .line_height
        .to_pixels(style.font_size, cx.rem_size())
        .0;

    let text_layout_details = editor.text_layout_details(cx);
    let window_top = window_top(&map, &text_layout_details);

    let ranges = [start..end];
    editor.set_search_within_ranges(&ranges, cx);
    let matches = editor
        .find_matches(Arc::new(query), cx)
        .then(move |matches| async move {
            let map = map;
            matches
                .into_iter()
                .map(|anchor| {
                    let word = anchor.start.to_display_point(&map);
                    let x = bounding_box.origin.x.0 + word.column() as f32 * line_height * 0.5;
                    let y = bounding_box.origin.y.0 - window_top.row().0 as f32 * line_height
                        + word.row().0 as f32 * line_height;
                    let x = Pixels(x);
                    let y = Pixels(y);
                    (word, entity_id, Point::new(x, y))
                })
                .collect::<Vec<_>>()
        });
    Some(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{display_map::DisplayRow, test::marked_display_snapshot};
    use gpui::AppContext;
    use project::Project;
    use settings::SettingsStore;

    fn display_point(x: u32, y: u32) -> DisplayPoint {
        DisplayPoint::new(DisplayRow(y), x)
    }

    fn test_helper(text: &str, list: Vec<DisplayPoint>, full_word: bool, cx: &mut AppContext) {
        let (snapshot, display_points) = marked_display_snapshot(text, cx);
        let point = display_points.first().unwrap().clone();
        let end = display_points.last().unwrap().clone();
        let starts = word_starts_in_range(&snapshot, point, end, full_word);
        assert_eq!(starts, list, "full_word: {:?}, text: {}", full_word, text);
    }

    #[gpui::test]
    fn test_get_word_starts(cx: &mut AppContext) {
        init_test(cx);

        let marked_text = "ˇlorem ipsuˇm hi hello ";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(6, 0)],
            false,
            cx,
        );

        let marked_text = "ˇlorem ipsum hi helloˇ";
        test_helper(
            marked_text,
            vec![
                display_point(0, 0),
                display_point(6, 0),
                display_point(12, 0),
                display_point(15, 0),
            ],
            false,
            cx,
        );

        let marked_text = "ˇlorem.ipsum.hi.helloˇ";
        test_helper(
            marked_text,
            vec![
                display_point(0, 0),
                display_point(6, 0),
                display_point(12, 0),
                display_point(15, 0),
            ],
            false,
            cx,
        );

        let marked_text = "ˇ lorem.ipsum.hi.helloˇ";
        test_helper(
            marked_text,
            vec![
                display_point(1, 0),
                display_point(7, 0),
                display_point(13, 0),
                display_point(16, 0),
            ],
            false,
            cx,
        );

        let marked_text = "ˇ lorem.ipsum.hi.helloˇ";
        test_helper(marked_text, vec![display_point(1, 0)], true, cx);

        let marked_text = "ˇlorem.ipsum hi.helloˇ";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(11, 0)],
            true,
            cx,
        );

        let marked_text = "ˇlorem.ipsum\nhi.helloˇ";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(0, 1)],
            true,
            cx,
        );

        let marked_text = "ˇlorem.ipsum \n\n hi.hello \"\"";
        test_helper(
            marked_text,
            vec![display_point(0, 0), display_point(1, 2)],
            true,
            cx,
        );

        let marked_text = "ˇlorem ipsum \n{}\n hi hello \"\"";
        test_helper(
            marked_text,
            vec![
                display_point(0, 0),
                display_point(6, 0),
                display_point(1, 2),
                display_point(4, 2),
            ],
            true,
            cx,
        );
    }

    fn init_test(cx: &mut gpui::AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}
