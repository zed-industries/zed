use futures::{Future, FutureExt};
use std::sync::Arc;

use editor::{display_map::ToDisplayPoint, DisplayPoint, Editor};
use gpui::{Bounds, EntityId, Point};
use project::search::SearchQuery;
use text::Bias;
use ui::{Pixels, ViewContext};
use workspace::searchable::SearchableItem;

use crate::{
    util::{ranges, window_bottom, window_top},
    Direction,
};

pub fn search_window(
    query: &str,
    direction: Direction,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Option<impl Future<Output = Vec<DisplayPoint>>> {
    let query = SearchQuery::text(query, false, false, false, Vec::new(), Vec::new()).ok()?;

    let map = editor.snapshot(cx).display_snapshot;
    let selections = editor.selections.newest_display(cx);
    let range = ranges(direction, &map, &selections, editor, cx);
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
    bounding_box: Bounds<Pixels>,
    entity_id: EntityId,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Option<impl Future<Output = Vec<(DisplayPoint, EntityId, Point<Pixels>)>>> {
    let query = SearchQuery::text(query, false, false, false, Vec::new(), Vec::new()).ok()?;

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
