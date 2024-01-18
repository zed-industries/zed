use gpui::{div, Div, Rgba};
use ui::StyledExt;

pub struct SurfaceState {
    background: Rgba,
    foreground_1: Rgba,
    foreground_2: Rgba,
    foreground_3: Rgba,
    border: Rgba,
}

pub fn h_flex() -> Div {
    div().h_flex()
}
