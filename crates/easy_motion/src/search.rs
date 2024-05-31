use std::sync::Arc;

use futures::{Future, FutureExt};

use editor::{display_map::ToDisplayPoint, DisplayPoint, Editor};
use project::search::SearchQuery;
use text::Bias;
use ui::ViewContext;
use workspace::searchable::SearchableItem;

use crate::util::{window_bottom, window_top};

pub fn search(
    query: String,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Option<impl Future<Output = Vec<DisplayPoint>>> {
    let query: Arc<_> = SearchQuery::text(query, false, false, false, Vec::new(), Vec::new())
        .ok()?
        .into();

    let map = editor.snapshot(cx).display_snapshot;
    let text_layout_details = editor.text_layout_details(cx);
    // todo switch editor to view and do search in background?
    let start = window_top(&map, &text_layout_details);
    let end = window_bottom(&map, &text_layout_details);
    let start = map.display_point_to_anchor(start, Bias::Left);
    let end = map.display_point_to_anchor(end, Bias::Left);
    let ranges = [start..end];
    editor.set_search_within_ranges(&ranges, cx);
    let matches = editor
        .find_matches(query, cx)
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
    query: String,
    editor: &mut Editor,
    cx: &mut ViewContext<Editor>,
) -> Option<impl Future<Output = Vec<DisplayPoint>>> {
    let query: Arc<_> = SearchQuery::text(query, false, false, false, Vec::new(), Vec::new())
        .ok()?
        .into();

    let map = editor.snapshot(cx).display_snapshot;
    let text_layout_details = editor.text_layout_details(cx);
    // todo switch editor to view and do search in background?
    let start = window_top(&map, &text_layout_details);
    let end = window_bottom(&map, &text_layout_details);
    let start = map.display_point_to_anchor(start, Bias::Left);
    let end = map.display_point_to_anchor(end, Bias::Left);
    let ranges = [start..end];
    editor.set_search_within_ranges(&ranges, cx);
    let matches = editor
        .find_matches(query, cx)
        .then(move |matches| async move {
            let map = map;
            matches
                .into_iter()
                .map(|anchor| anchor.start.to_display_point(&map))
                .collect::<Vec<_>>()
        });
    Some(matches)
}
