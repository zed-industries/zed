use editor::{Editor, SelectionEffects, display_map::ToDisplayPoint as _, movement};
use gpui::{Context, Window, actions};
use language::SelectionGoal;
use text::Bias;

use crate::{
    Helix,
    selection::{HelixRange, nth_next_grapheme_boundary, nth_prev_grapheme_boundary},
};

actions!(
    helix,
    [
        /// Moves each cursor one character left.
        MoveCharLeft,
        /// Moves each cursor one character right.
        MoveCharRight,
        /// Moves each cursor one display line up.
        MoveVisualLineUp,
        /// Moves each cursor one display line down.
        MoveVisualLineDown,
        /// Extends each selection one character left.
        ExtendCharLeft,
        /// Extends each selection one character right.
        ExtendCharRight,
        /// Extends each selection one display line up.
        ExtendVisualLineUp,
        /// Extends each selection one display line down.
        ExtendVisualLineDown,
    ]
);

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
}

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Helix>) {
    Helix::action(editor, cx, |helix, _: &MoveCharLeft, window, cx| {
        helix.move_horizontally(Direction::Backward, false, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &MoveCharRight, window, cx| {
        helix.move_horizontally(Direction::Forward, false, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &MoveVisualLineUp, window, cx| {
        helix.move_vertically(Direction::Backward, false, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &MoveVisualLineDown, window, cx| {
        helix.move_vertically(Direction::Forward, false, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &ExtendCharLeft, window, cx| {
        helix.move_horizontally(Direction::Backward, true, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &ExtendCharRight, window, cx| {
        helix.move_horizontally(Direction::Forward, true, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &ExtendVisualLineUp, window, cx| {
        helix.move_vertically(Direction::Backward, true, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &ExtendVisualLineDown, window, cx| {
        helix.move_vertically(Direction::Forward, true, window, cx)
    });
}

impl Helix {
    fn move_horizontally(
        &mut self,
        direction: Direction,
        extend: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.take_count(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
                selections.move_offsets_with(&mut |snapshot, selection| {
                    let range = HelixRange::from_selection(selection);
                    let cursor = range.cursor(snapshot);
                    let new_cursor = match direction {
                        Direction::Forward => nth_next_grapheme_boundary(snapshot, cursor, count),
                        Direction::Backward => nth_prev_grapheme_boundary(snapshot, cursor, count),
                    };
                    range
                        .put_cursor(snapshot, new_cursor, extend)
                        .min_width_1(snapshot)
                        .apply_to(selection);
                    selection.goal = SelectionGoal::None;
                });
            });
        });
    }

    fn move_vertically(
        &mut self,
        direction: Direction,
        extend: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.take_count(cx);
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window, cx);
            editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
                let display_map = selections.display_snapshot();
                selections.move_offsets_with(&mut |snapshot, selection| {
                    let range = HelixRange::from_selection(selection);
                    let mut point = range.cursor(snapshot).to_display_point(&display_map);
                    let mut goal = selection.goal;
                    for _ in 0..count {
                        (point, goal) = match direction {
                            Direction::Forward => movement::down(
                                &display_map,
                                point,
                                goal,
                                true,
                                &text_layout_details,
                            ),
                            Direction::Backward => {
                                movement::up(&display_map, point, goal, true, &text_layout_details)
                            }
                        };
                    }
                    let new_cursor = point.to_offset(&display_map, Bias::Left);
                    // Helix doesn't extend onto the empty line after a trailing newline.
                    if extend && new_cursor == snapshot.len() && snapshot.max_point().column == 0 {
                        return;
                    }
                    range
                        .put_cursor(snapshot, new_cursor, extend)
                        .min_width_1(snapshot)
                        .apply_to(selection);
                    selection.goal = goal;
                });
            });
        });
    }
}
