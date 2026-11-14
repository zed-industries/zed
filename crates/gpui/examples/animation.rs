#![cfg_attr(target_family = "wasm", no_main)]

#[path = "example_support/fonts.rs"]
mod example_support;

use std::time::Duration;

use anyhow::Result;
use gpui::{
    Animation, AnimationExt as _, AnimationPhase, App, AssetSource, Bounds, Context, MouseButton,
    MouseDownEvent, MouseMoveEvent, Pixels, SharedString, SpringAnimation, SpringConfig,
    Transformation, Window, WindowBounds, WindowOptions, bounce, div, ease_in_out, percentage,
    prelude::*, px, relative, rgba, size, svg,
};
use gpui_platform::application;

struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().into_owned(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

const ARROW_CIRCLE_SVG: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/image/arrow_circle.svg"
);

struct AnimationExample {
    spring_phase: u8,
    spring_damping: f32,
    spring_damping_drag: Option<(Pixels, f32)>,
}

impl Render for AnimationExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const MINIMUM_DAMPING: f32 = 2.0;
        const MAXIMUM_DAMPING: f32 = 32.0;
        const DAMPING_SLIDER_WIDTH: Pixels = px(224.0);

        let spring_phase = self.spring_phase;
        let spring_position = px(98.0 * f32::from(spring_phase));
        let spring_damping = self.spring_damping;
        let spring = SpringConfig::new(170.0, spring_damping, 1.0);
        let (_, damping_ratio) = spring.canonical();
        let damping_fraction =
            (spring_damping - MINIMUM_DAMPING) / (MAXIMUM_DAMPING - MINIMUM_DAMPING);

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .justify_around()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .justify_around()
                    .child(
                        div()
                            .id("content")
                            .flex()
                            .flex_col()
                            .h(px(150.))
                            .overflow_y_scroll()
                            .w_full()
                            .flex_1()
                            .justify_center()
                            .items_center()
                            .text_xl()
                            .gap_4()
                            .child("Hello Animation")
                            .child(
                                div()
                                    .id("spring-demo")
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .p_2()
                                    .w(px(240.0))
                                    .rounded_md()
                                    .bg(rgba(0x00000010))
                                    .cursor_pointer()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, _, cx| {
                                            this.spring_phase = (this.spring_phase + 1) % 3;
                                            cx.notify();
                                        }),
                                    )
                                    .child(format!(
                                        "Target phase {spring_phase}: click rapidly to redirect momentum"
                                    ))
                                    .child(
                                        div()
                                            .id("spring-damping-controls")
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .text_sm()
                                            .on_click(|_, _, cx| cx.stop_propagation())
                                            .child(format!(
                                                "Drag damping: {spring_damping:.1} (ζ {damping_ratio:.2})"
                                            ))
                                            .child(
                                                div()
                                                    .id("spring-damping")
                                                    .relative()
                                                    .h(px(20.0))
                                                    .w(DAMPING_SLIDER_WIDTH)
                                                    .cursor_pointer()
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(
                                                            |this,
                                                             event: &MouseDownEvent,
                                                             _,
                                                             cx| {
                                                                this.spring_damping_drag = Some((
                                                                    event.position.x,
                                                                    this.spring_damping,
                                                                ));
                                                                cx.stop_propagation();
                                                            },
                                                        ),
                                                    )
                                                    .on_mouse_move(cx.listener(
                                                        |this,
                                                         event: &MouseMoveEvent,
                                                         _,
                                                         cx| {
                                                            let Some((
                                                                start_position,
                                                                start_damping,
                                                            )) = this.spring_damping_drag
                                                            else {
                                                                return;
                                                            };

                                                            let delta = (event.position.x
                                                                - start_position)
                                                                / DAMPING_SLIDER_WIDTH;
                                                            this.spring_damping = (start_damping
                                                                + delta
                                                                    * (MAXIMUM_DAMPING
                                                                        - MINIMUM_DAMPING))
                                                                .clamp(
                                                                    MINIMUM_DAMPING,
                                                                    MAXIMUM_DAMPING,
                                                                );
                                                            cx.stop_propagation();
                                                            cx.notify();
                                                        },
                                                    ))
                                                    .on_mouse_up(
                                                        MouseButton::Left,
                                                        cx.listener(|this, _, _, cx| {
                                                            this.spring_damping_drag = None;
                                                            cx.stop_propagation();
                                                        }),
                                                    )
                                                    .on_mouse_up_out(
                                                        MouseButton::Left,
                                                        cx.listener(|this, _, _, cx| {
                                                            this.spring_damping_drag = None;
                                                            cx.stop_propagation();
                                                        }),
                                                    )
                                                    .child(
                                                        div()
                                                            .absolute()
                                                            .top(px(8.0))
                                                            .h(px(4.0))
                                                            .w_full()
                                                            .rounded_full()
                                                            .bg(rgba(0x00000030)),
                                                    )
                                                    .child(
                                                        div()
                                                            .absolute()
                                                            .top(px(8.0))
                                                            .h(px(4.0))
                                                            .w(relative(damping_fraction))
                                                            .rounded_full()
                                                            .bg(rgba(0x3b82f6ff)),
                                                    )
                                                    .child(
                                                        div()
                                                            .absolute()
                                                            .top(px(3.0))
                                                            .left(relative(damping_fraction))
                                                            .ml(px(-7.0))
                                                            .size(px(14.0))
                                                            .rounded_full()
                                                            .bg(rgba(0x2563ebff)),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div().relative().h(px(32.0)).w_full().child(
                                            div()
                                                .absolute()
                                                .top_1()
                                                .size(px(24.0))
                                                .rounded_full()
                                                .bg(rgba(0x3b82f6ff))
                                                .cursor_pointer()
                                                .debug_selector(|| "spring-position".into())
                                                .with_spring(
                                                    "spring-position",
                                                    SpringAnimation::new(spring)
                                                        .to(spring_position)
                                                        .with_epsilon(0.25),
                                                    |this, left| this.left(left),
                                                ),
                                        ),
                                    )
                                    .child(
                                        div().h(px(24.0)).rounded_sm().with_spring(
                                            "spring-phase",
                                            SpringAnimation::new(spring)
                                                .to(AnimationPhase(f32::from(spring_phase)))
                                                .with_epsilon(0.001),
                                            |this, phase| {
                                                let (width, color) = if phase.0 <= 1.0 {
                                                    (
                                                        phase.interpolate_between(
                                                            0.0..=1.0,
                                                            px(48.0),
                                                            px(224.0),
                                                        ),
                                                        phase.interpolate_between_clamped(
                                                            0.0..=1.0,
                                                            rgba(0xf97316ff),
                                                            rgba(0x22c55eff),
                                                        ),
                                                    )
                                                } else {
                                                    (
                                                        phase.interpolate_between(
                                                            1.0..=2.0,
                                                            px(224.0),
                                                            px(96.0),
                                                        ),
                                                        phase.interpolate_between_clamped(
                                                            1.0..=2.0,
                                                            rgba(0x22c55eff),
                                                            rgba(0xa855f7ff),
                                                        ),
                                                    )
                                                };
                                                this.w(width).bg(color)
                                            },
                                        ),
                                    ),
                            )
                            .child(
                                svg()
                                    .size_20()
                                    .overflow_hidden()
                                    .path(ARROW_CIRCLE_SVG)
                                    .text_color(gpui::black())
                                    .with_animation(
                                        "image_circle",
                                        Animation::new(Duration::from_secs(2))
                                            .repeat()
                                            .with_easing(bounce(ease_in_out)),
                                        |svg, delta| {
                                            svg.with_transformation(Transformation::rotate(
                                                percentage(delta),
                                            ))
                                        },
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .h(px(64.))
                            .w_full()
                            .p_2()
                            .justify_center()
                            .items_center()
                            .border_t_1()
                            .border_color(gpui::black().opacity(0.1))
                            .bg(gpui::black().opacity(0.05))
                            .child("Other Panel"),
                    ),
            )
    }
}

fn run_example() {
    application().with_assets(Assets {}).run(|cx: &mut App| {
        if !example_support::load_fonts(cx) {
            return;
        }
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(300.), px(300.)),
                cx,
            ))),
            ..Default::default()
        };
        cx.open_window(options, |_, cx| {
            cx.activate(false);
            cx.new(|_| AnimationExample {
                spring_phase: 0,
                spring_damping: 14.0,
                spring_damping_drag: None,
            })
        })
        .unwrap();
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}

#[cfg(all(test, feature = "test-support"))]
mod tests {
    use gpui::{Modifiers, TestAppContext};

    use super::*;

    #[gpui::test]
    fn clicking_spring_position_changes_the_target(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, _| AnimationExample {
            spring_phase: 0,
            spring_damping: 14.0,
            spring_damping_drag: None,
        });
        cx.simulate_resize(size(px(300.0), px(300.0)));
        cx.run_until_parked();

        let position_bounds = cx
            .debug_bounds("spring-position")
            .expect("spring position should be rendered");
        let position = position_bounds.center();
        cx.simulate_mouse_move(position, None, Modifiers::default());
        cx.simulate_click(position, Modifiers::default());
        cx.run_until_parked();

        assert_eq!(cx.update(|_, cx| view.read(cx).spring_phase), 1);
    }
}
