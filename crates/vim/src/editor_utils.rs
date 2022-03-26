use editor::{display_map::DisplaySnapshot, Bias, DisplayPoint, Editor};
use gpui::ViewContext;
use language::{Selection, SelectionGoal};

pub trait VimEditorExt {
    fn clip_selections(self: &mut Self, cx: &mut ViewContext<Self>);
    fn clipped_move_selections(
        self: &mut Self,
        cx: &mut ViewContext<Self>,
        move_selection: impl Fn(&DisplaySnapshot, &mut Selection<DisplayPoint>),
    );
    fn clipped_move_selection_heads(
        &mut self,
        cx: &mut ViewContext<Self>,
        update_head: impl Fn(
            &DisplaySnapshot,
            DisplayPoint,
            SelectionGoal,
        ) -> (DisplayPoint, SelectionGoal),
    );
    fn clipped_move_cursors(
        self: &mut Self,
        cx: &mut ViewContext<Self>,
        update_cursor_position: impl Fn(
            &DisplaySnapshot,
            DisplayPoint,
            SelectionGoal,
        ) -> (DisplayPoint, SelectionGoal),
    );
}

pub fn clip_display_point(map: &DisplaySnapshot, mut display_point: DisplayPoint) -> DisplayPoint {
    let next_char = map.chars_at(display_point).next();
    if next_char == Some('\n') || next_char == None {
        *display_point.column_mut() = display_point.column().saturating_sub(1);
        display_point = map.clip_point(display_point, Bias::Left);
    }
    display_point
}

impl VimEditorExt for Editor {
    fn clip_selections(self: &mut Self, cx: &mut ViewContext<Self>) {
        self.move_selections(cx, |map, selection| {
            if selection.is_empty() {
                let adjusted_cursor = clip_display_point(map, selection.start);
                selection.collapse_to(adjusted_cursor, selection.goal);
            } else {
                let adjusted_head = clip_display_point(map, selection.head());
                selection.set_head(adjusted_head, selection.goal);
            }
        })
    }

    fn clipped_move_selections(
        self: &mut Self,
        cx: &mut ViewContext<Self>,
        move_selection: impl Fn(&DisplaySnapshot, &mut Selection<DisplayPoint>),
    ) {
        self.move_selections(cx, |map, selection| {
            move_selection(map, selection);
            let adjusted_head = clip_display_point(map, selection.head());
            selection.set_head(adjusted_head, selection.goal);
        })
    }

    fn clipped_move_selection_heads(
        &mut self,
        cx: &mut ViewContext<Self>,
        update_head: impl Fn(
            &DisplaySnapshot,
            DisplayPoint,
            SelectionGoal,
        ) -> (DisplayPoint, SelectionGoal),
    ) {
        self.clipped_move_selections(cx, |map, selection| {
            let (new_head, new_goal) = update_head(map, selection.head(), selection.goal);
            let adjusted_head = clip_display_point(map, new_head);
            selection.set_head(adjusted_head, new_goal);
        });
    }

    fn clipped_move_cursors(
        self: &mut Self,
        cx: &mut ViewContext<Self>,
        update_cursor_position: impl Fn(
            &DisplaySnapshot,
            DisplayPoint,
            SelectionGoal,
        ) -> (DisplayPoint, SelectionGoal),
    ) {
        self.move_selections(cx, |map, selection| {
            let (cursor, new_goal) = update_cursor_position(map, selection.head(), selection.goal);
            let adjusted_cursor = clip_display_point(map, cursor);
            selection.collapse_to(adjusted_cursor, new_goal);
        });
    }
}
