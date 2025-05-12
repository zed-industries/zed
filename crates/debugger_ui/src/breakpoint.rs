use gpui::{Bounds, Hsla, Path, Pixels, canvas, fill, hsla, point, prelude::*};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    AbsoluteLength, ActiveTheme, Color, Component, ComponentScope, IntoElement, RegisterComponent,
    RenderOnce, Vector, VectorName, Window, div, px, rems_from_px,
};

#[derive(IntoElement, RegisterComponent)]
pub struct BreakpointIndicator {
    line: usize,
    width: Pixels,
    active: bool,
    disabled: bool,
    reachable: bool,
    color: Hsla,
}

impl BreakpointIndicator {
    pub fn new(line: usize, width: Pixels, active: bool, disabled: bool, color: Hsla) -> Self {
        Self {
            line,
            width,
            active,
            disabled,
            reachable: true,
            color,
        }
    }

    pub fn reachable(mut self, reachable: bool) -> Self {
        self.reachable = reachable;
        self
    }
}

impl RenderOnce for BreakpointIndicator {
    fn render(self, window: &mut ui::Window, cx: &mut ui::App) -> impl ui::IntoElement {
        let font_size = ThemeSettings::get_global(cx).buffer_font_size(cx);
        let line_height = ThemeSettings::get_global(cx).buffer_font_size(cx);

        let opacity = if self.reachable { 0.5 } else { 0.85 };

        let bg = if self.active {
            cx.theme().status().error
        } else {
            cx.theme()
                .status()
                .error
                .alpha(1.0)
                .blend(cx.theme().colors().editor_background.alpha(opacity))
        };

        let vector_width = rems_from_px(line_height.0);
        let right_adjustment = self.width - vector_width.to_pixels(window.rem_size()) / 2.0;
        let middle_segment = self.width - vector_width.to_pixels(window.rem_size());

        div()
            .flex()
            .h(line_height)
            .w(self.width)
            // .child(widest_line_string)
            .child(
                div().absolute().top_0().left(px(-4.0)).child(
                    Vector::square(VectorName::BreakpointFlagStart, rems_from_px(line_height.0))
                        .color(Color::Custom(bg)),
                ),
            )
            .child(
                div()
                    .w(middle_segment)
                    .ml(vector_width / 2.)
                    .h_full()
                    .bg(bg),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(rems_from_px(right_adjustment.0))
                    .child(
                        Vector::square(VectorName::BreakpointFlagEnd, rems_from_px(line_height.0))
                            .color(Color::Custom(bg)),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .mr(px(2.))
                    .text_right()
                    .text_color(cx.theme().colors().text)
                    .text_size(font_size)
                    .line_height(line_height)
                    .child(self.line.to_string()),
            )

        // let color = self.color;
        // let active = self.active;
        // let line_string = self.line.to_string().clone();
        // let widest_line_string = self.widest_line_number.to_string().clone();
        // let triangle_width = px(7.); // Triangle width

        // let bg = canvas(
        //     move |_, _, _| {},
        //     move |bounds, _, window, _| {
        //         // XCode style triangle - pointing right toward the code
        //         let height = line_height;
        //         let triangle_width = px(7.); // Triangle width
        //         let vertical_padding = Pixels::from(2.0);
        //         let total_width = bounds.size.width;
        //         let square_width = total_width - triangle_width;

        //         // Get the fill color - add slight opacity if inactive
        //         let fill_color = if active {
        //             color
        //         } else {
        //             hsla(color.h, color.s, color.l, 0.7)
        //         };

        //         // Draw square on the left
        //         let square_bounds = Bounds::from_corners(
        //             bounds.origin,
        //             point(bounds.origin.x + square_width, bounds.origin.y + height),
        //         );
        //         window.paint_quad(fill(square_bounds, fill_color));

        //         // Align the triangle to the right of the square
        //         let triangle_left = bounds.origin.x + square_width;
        //         let center_y = bounds.origin.y + (height / 2.0);
        //         let top = bounds.origin.y + vertical_padding;
        //         let bottom = bounds.origin.y + height - vertical_padding;

        //         // Create the right-facing triangle
        //         let mut path = Path::new(point(triangle_left, top));
        //         path.line_to(point(triangle_left, bottom));
        //         path.line_to(point(triangle_left + triangle_width, center_y));
        //         // Path is automatically closed when painted

        //         window.paint_path(path, fill_color);
        //     },
        // )
        // .w_full();

        // div()
        //     .relative()
        //     .flex()
        //     .flex_none()
        //     .items_center()
        //     .h(line_height)
        //     .text_color(cx.theme().colors().text)
        //     .w(px(129.))
        //     .child(div().absolute().left_0().top_0().child(bg))
        //     .child(
        //         div()
        //             .absolute()
        //             .top_0()
        //             .right(triangle_width + px(4.))
        //             .child(line_string),
        //     )
        //     .child(div().opacity(0.).child(widest_line_string))
    }
}

impl Component for BreakpointIndicator {
    fn scope() -> ComponentScope {
        ComponentScope::Debugger
    }

    fn preview(_window: &mut Window, cx: &mut gpui::App) -> Option<gpui::AnyElement> {
        let active_color = cx.theme().status().info;
        let inactive_color = cx.theme().colors().text.alpha(0.12);

        Some(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .p_4()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(BreakpointIndicator::new(
                            142,
                            px(50.),
                            true,
                            true,
                            active_color,
                        )),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(BreakpointIndicator::new(
                            26,
                            px(26.),
                            false,
                            false,
                            inactive_color,
                        )),
                )
                .child(
                    div().flex().items_center().gap_2().child(
                        BreakpointIndicator::new(831, px(38.), false, true, inactive_color)
                            .reachable(false),
                    ),
                )
                .into_any_element(),
        )
    }
}
