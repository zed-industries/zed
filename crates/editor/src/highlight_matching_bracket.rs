use gpui::ViewContext;

use crate::Editor;

enum MatchingBracketHighlight {}

pub fn refresh_matching_bracket_highlights(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    editor.clear_background_highlights::<MatchingBracketHighlight>(cx);

    let newest_selection = editor.selections.newest::<usize>(cx);
    let snapshot = editor.snapshot(cx);
    if let Some((opening_range, closing_range)) = snapshot
        .buffer_snapshot
        .enclosing_bracket_ranges(newest_selection.range())
    {
        let head = newest_selection.head();
        let range_to_highlight = if opening_range.contains(&head) {
            Some(closing_range)
        } else if closing_range.contains(&head) {
            Some(opening_range)
        } else {
            None
        };

        if let Some(range_to_highlight) = range_to_highlight {
            let anchor_range = snapshot
                .buffer_snapshot
                .anchor_before(range_to_highlight.start)
                ..snapshot
                    .buffer_snapshot
                    .anchor_after(range_to_highlight.end);

            editor.highlight_background::<MatchingBracketHighlight>(
                vec![anchor_range],
                |theme| theme.editor.document_highlight_read_background,
                cx,
            )
        }
    }
}
