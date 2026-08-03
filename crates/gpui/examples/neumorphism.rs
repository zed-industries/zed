use gpui::{
    App, AppContext as _, Bounds, BoxShadow, Context, Div, Hsla, InteractiveElement, IntoElement,
    MouseButton, MouseDownEvent, ParentElement as _, Render, Rgba, Styled as _, Window,
    WindowBounds, WindowOptions, div, point, px, rgb, size,
};
use gpui_platform::application;

/// Layout dimensions in pixels.
mod layout {
    pub const CARD_SIZE: f32 = 100.;
    pub const CARD_RADIUS: f32 = 8.;
    pub const CIRCLE_SIZE: f32 = 80.;
    pub const WINDOW_WIDTH: f32 = 1000.;
    pub const WINDOW_HEIGHT: f32 = 800.;
}

/// Neumorphic color theme.
///
/// All surface colors are derived from a single `bg` base by uniformly
/// shifting each RGB channel. Shadow colors are grayscale HSLA with
/// alpha values tuned for the base lightness.
struct Theme {
    /// Base surface color (RGB hex).
    bg: u32,
    /// Text color (RGB hex).
    text: u32,
    /// White highlight cast from the top-left light source.
    highlight: Hsla,
    /// Dark shadow cast from the bottom-right.
    shadow: Hsla,
    /// Softer highlight for the pressed state.
    highlight_pressed: Hsla,
    /// Softer shadow for the pressed state.
    shadow_pressed: Hsla,
    /// Shadow offset and blur for the standard raised depth.
    raised_distance: f32,
    raised_blur: f32,
    /// Shadow offset and blur for the deep raised depth.
    deep_distance: f32,
    deep_blur: f32,
    /// Shadow offset and blur for the subtle raised depth.
    subtle_distance: f32,
    subtle_blur: f32,
    /// Shadow offset and blur for the pressed state.
    pressed_distance: f32,
    pressed_blur: f32,
    /// How much the shadow expands beyond the element bounds.
    spread_radius: f32,
}

impl Theme {
    #[rustfmt::skip]
    fn light() -> Self {
        Self {
            bg:                0xe0e5ec,
            text:              0x4a5568,
            highlight:         Hsla { h: 0., s: 0., l: 1.0, a: 0.7 },
            shadow:            Hsla { h: 0., s: 0., l: 0.0, a: 0.15 },
            highlight_pressed: Hsla { h: 0., s: 0., l: 1.0, a: 0.9 },
            shadow_pressed:    Hsla { h: 0., s: 0., l: 0.0, a: 0.15 },
            raised_distance:   6., raised_blur:   12.,
            deep_distance:    10., deep_blur:     20.,
            subtle_distance:   3., subtle_blur:    6.,
            pressed_distance:  4., pressed_blur:    8.,
            spread_radius:     0.,
        }
    }

    #[rustfmt::skip]
    fn dark() -> Self {
        Self {
            bg:                0x2d3440,
            text:              0xc8cdd4,
            highlight:         Hsla { h: 0., s: 0., l: 1.0, a: 0.07 },
            shadow:            Hsla { h: 0., s: 0., l: 0.0, a: 0.5 },
            highlight_pressed: Hsla { h: 0., s: 0., l: 1.0, a: 0.04 },
            shadow_pressed:    Hsla { h: 0., s: 0., l: 0.0, a: 0.4 },
            raised_distance:   5., raised_blur:   10.,
            deep_distance:     8., deep_blur:     16.,
            subtle_distance:   2., subtle_blur:    5.,
            pressed_distance:  2., pressed_blur:   4.,
            spread_radius:     0.,
        }
    }

    fn bg(&self) -> Rgba {
        rgb(self.bg)
    }
    fn text(&self) -> Rgba {
        rgb(self.text)
    }

    /// Build a highlight/shadow pair at the given distance and blur.
    fn shadow_pair(
        &self,
        highlight: Hsla,
        shadow: Hsla,
        distance: f32,
        blur: f32,
        inset: bool,
    ) -> Vec<BoxShadow> {
        vec![
            BoxShadow {
                color: highlight,
                offset: point(px(-distance), px(-distance)),
                blur_radius: px(blur),
                spread_radius: px(self.spread_radius),
                inset,
            },
            BoxShadow {
                color: shadow,
                offset: point(px(distance), px(distance)),
                blur_radius: px(blur),
                spread_radius: px(self.spread_radius),
                inset,
            },
        ]
    }

    /// Standard raised neumorphic surface.
    fn raised_shadow(&self) -> Vec<BoxShadow> {
        self.shadow_pair(
            self.highlight,
            self.shadow,
            self.raised_distance,
            self.raised_blur,
            false,
        )
    }

    /// Deeply raised neumorphic surface.
    fn deep_shadow(&self) -> Vec<BoxShadow> {
        self.shadow_pair(
            self.highlight,
            self.shadow,
            self.deep_distance,
            self.deep_blur,
            false,
        )
    }

    /// Subtly raised neumorphic surface.
    fn subtle_shadow(&self) -> Vec<BoxShadow> {
        self.shadow_pair(
            self.highlight,
            self.shadow,
            self.subtle_distance,
            self.subtle_blur,
            false,
        )
    }

    /// Pressed/concave surface using real inset shadows.
    /// Uses the same color order as raised shadows (highlight first, shadow
    /// second). The inset flag naturally reverses the visual direction.
    fn pressed_shadow(&self) -> Vec<BoxShadow> {
        self.shadow_pair(
            self.highlight_pressed,
            self.shadow_pressed,
            self.pressed_distance,
            self.pressed_blur,
            true,
        )
    }

    /// A basic card element with the shared neumorphic background and sizing.
    fn card(&self, label: &str) -> Div {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .w(px(layout::CARD_SIZE))
            .h(px(layout::CARD_SIZE))
            .rounded(px(layout::CARD_RADIUS))
            .bg(self.bg())
            .text_color(self.text())
            .text_xs()
            .child(label.to_string())
    }
}

struct Neumorphism {
    dark: bool,
}

impl Neumorphism {
    fn theme(&self) -> Theme {
        if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }
}

impl Render for Neumorphism {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme();
        let mode_label = if self.dark { "Light mode" } else { "Dark mode" };

        div()
            .flex()
            .flex_col()
            .size_full()
            .text_xs()
            .bg(theme.bg())
            .items_center()
            .justify_center()
            .gap_8()
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, _: &MouseDownEvent, _window: &mut Window, cx| {
                    this.dark = !this.dark;
                    cx.notify();
                }),
            )
            .child(
                div()
                    .text_xl()
                    .text_color(theme.text())
                    .child("Neumorphic Shadows in GPUI"),
            )
            // Row of raised cards
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    .child(theme.card("Raised").shadow(theme.raised_shadow()))
                    .child(theme.card("Deep").shadow(theme.deep_shadow()))
                    .child(theme.card("Subtle").shadow(theme.subtle_shadow())),
            )
            // Second row -- inset shadows
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    // Pressed card using real inset shadows
                    .child(theme.card("Pressed").shadow(theme.pressed_shadow()))
                    // Circular raised element
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(layout::CIRCLE_SIZE))
                            .h(px(layout::CIRCLE_SIZE))
                            .rounded(px(layout::CIRCLE_SIZE / 2.))
                            .bg(theme.bg())
                            .text_color(theme.text())
                            .shadow(theme.raised_shadow())
                            .text_xs()
                            .child("Icon"),
                    )
                    // Inset card
                    .child(theme.card("Inset").shadow(theme.pressed_shadow())),
            )
            // Mode toggle hint
            .child(
                div()
                    .text_color(theme.text())
                    .child(format!("Right-click to switch to {mode_label}")),
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(
            None,
            size(px(layout::WINDOW_WIDTH), px(layout::WINDOW_HEIGHT)),
            cx,
        );
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                window.set_rem_size(px(12.));
                cx.new(|_cx| Neumorphism { dark: false })
            },
        )
        .unwrap();
    });
}
