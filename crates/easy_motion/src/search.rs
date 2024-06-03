use futures::{Future, FutureExt};
use std::{ops::Range, sync::Arc};

use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement::TextLayoutDetails,
    DisplayPoint, Editor,
};
use gpui::{point, Bounds, EntityId, Point, Task};
use project::search::SearchQuery;
use text::{Bias, Selection};
use ui::{Pixels, ViewContext};
use workspace::searchable::SearchableItem;

use crate::{
    util::{ranges, window_bottom, window_top, word_starts_in_range},
    Direction, WordType,
};

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
