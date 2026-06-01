use gpui::{AppContext, Entity};
use ui::{App, Pixels, Window};

use crate::render::window_controls::DragPreview;

fn highlighted_drag_preview<T>(
    is_highlighted: gpui::Entity<bool>,
) -> impl Fn(&T, gpui::Point<Pixels>, &mut Window, &mut App) -> gpui::Entity<DragPreview> {
    move |_, _, _, cx| {
        is_highlighted.write(cx, true);
        cx.new(|_| DragPreview)
    }
}

fn clear_resize_highlight<T>(is_highlighted: Entity<bool>) -> impl Fn(&T, &mut Window, &mut App) {
    move |_, _, cx| is_highlighted.write(cx, false)
}
