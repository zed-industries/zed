use gpui::{BorderStyle, Corners, Edges, Hsla, canvas, quad};

use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct RadioIndicator {
    is_selected: bool,
    border_color: Hsla,
    background: Hsla,
}

impl RadioIndicator {
    pub fn new(is_selected: bool, border_color: Hsla, background: Hsla) -> Self {
        Self {
            is_selected,
            border_color,
            background,
        }
    }
}

impl RenderOnce for RadioIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Self {
            is_selected,
            border_color,
            background,
        } = self;

        div().size_3().flex_none().child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, cx| {
                    let bounds = window.pixel_snap_bounds(bounds);
                    let radius = bounds.size.width.min(bounds.size.height) / 2.;
                    window.paint_quad(quad(
                        bounds,
                        Corners::all(radius),
                        background,
                        Edges::all(px(1.)),
                        border_color,
                        BorderStyle::Solid,
                    ));

                    if is_selected {
                        // Equal whole-device-pixel insets keep both circles concentric
                        // when their independently rounded diameters would have different parity.
                        let scale_factor = window.scale_factor();
                        let inset =
                            px((f32::from(radius) * scale_factor / 2.).round() / scale_factor);
                        let dot_bounds = bounds.dilate(-inset);
                        window.paint_quad(quad(
                            dot_bounds,
                            Corners::all(dot_bounds.size.width.min(dot_bounds.size.height) / 2.),
                            Color::Accent.color(cx),
                            Edges::default(),
                            Hsla::transparent_black(),
                            BorderStyle::Solid,
                        ));
                    }
                },
            )
            .size_full(),
        )
    }
}

impl Component for RadioIndicator {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> &'static str {
        "A non-interactive radio indicator for single-selection controls."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let border_color = cx.theme().colors().border.opacity(0.8);
        let background = cx.theme().colors().editor_background;

        example_group_with_title(
            "Radio Indicator States",
            vec![
                single_example(
                    "Unselected",
                    Self::new(false, border_color, background).into_any_element(),
                ),
                single_example(
                    "Selected",
                    Self::new(true, border_color, background).into_any_element(),
                ),
            ],
        )
        .into_any_element()
    }
}
