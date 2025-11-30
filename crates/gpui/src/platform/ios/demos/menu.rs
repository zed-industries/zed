//! Main menu for iOS demos.
//!
//! Provides a menu to select between different demo views.

use super::{
    AnimationPlayground, BACKGROUND, BLUE, MAUVE, OVERLAY, SUBTEXT, SURFACE, ShaderShowcase, TEXT,
};
use crate::{
    App, Bounds, Context, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Render,
    Window, div, hsla, point, prelude::*, px, rgb, size,
};

/// Which demo is currently active
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ActiveDemo {
    #[default]
    Menu,
    AnimationPlayground,
    ShaderShowcase,
}

/// Root application view that manages demo navigation
pub struct DemoApp {
    active: ActiveDemo,
    animation_playground: Option<AnimationPlayground>,
    shader_showcase: Option<ShaderShowcase>,
}

impl DemoApp {
    pub fn new() -> Self {
        Self {
            active: ActiveDemo::Menu,
            animation_playground: None,
            shader_showcase: None,
        }
    }

    fn go_to_animation_playground(&mut self, cx: &mut Context<Self>) {
        self.animation_playground = Some(AnimationPlayground::new());
        self.active = ActiveDemo::AnimationPlayground;
        cx.notify();
    }

    fn go_to_shader_showcase(&mut self, cx: &mut Context<Self>) {
        self.shader_showcase = Some(ShaderShowcase::new());
        self.active = ActiveDemo::ShaderShowcase;
        cx.notify();
    }

    fn go_to_menu(&mut self, cx: &mut Context<Self>) {
        self.active = ActiveDemo::Menu;
        self.animation_playground = None;
        self.shader_showcase = None;
        cx.notify();
    }

    fn handle_animation_touch_down(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        if let Some(playground) = &mut self.animation_playground {
            let pos = point(event.position.x.0, event.position.y.0);
            playground.touch_start = Some((pos, std::time::Instant::now()));
            playground.current_touch = Some(pos);
            cx.notify();
        }
    }

    fn handle_animation_touch_up(&mut self, event: &MouseUpEvent, cx: &mut Context<Self>) {
        if let Some(playground) = &mut self.animation_playground {
            let position = point(event.position.x.0, event.position.y.0);

            if let Some((start_pos, start_time)) = playground.touch_start.take() {
                let elapsed = start_time.elapsed();
                let dx = position.x - start_pos.x;
                let dy = position.y - start_pos.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if elapsed < std::time::Duration::from_millis(200) && distance < 20.0 {
                    let color_rgb = super::random_color(playground.next_ball_id);
                    playground.spawn_particles(position, rgb(color_rgb).into());
                    playground.next_ball_id += 1;
                } else {
                    let dt = elapsed.as_secs_f32().max(0.01);
                    let velocity = point(dx / dt * 0.5, dy / dt * 0.5);
                    playground.spawn_ball(start_pos, velocity);
                }
            }
            playground.current_touch = None;
            cx.notify();
        }
    }

    fn handle_shader_touch_down(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        if let Some(showcase) = &mut self.shader_showcase {
            let pos = point(event.position.x.0, event.position.y.0);
            showcase.touch_position = Some(pos);
            showcase.spawn_ripple(pos);
            cx.notify();
        }
    }

    fn handle_shader_touch_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if let Some(showcase) = &mut self.shader_showcase {
            let pos = point(event.position.x.0, event.position.y.0);
            showcase.touch_position = Some(pos);
            cx.notify();
        }
    }

    fn handle_shader_touch_up(&mut self, _event: &MouseUpEvent, cx: &mut Context<Self>) {
        if let Some(showcase) = &mut self.shader_showcase {
            showcase.touch_position = None;
            cx.notify();
        }
    }
}

impl Render for DemoApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active {
            ActiveDemo::Menu => self.render_menu(window, cx).into_any_element(),
            ActiveDemo::AnimationPlayground => self
                .render_animation_playground(window, cx)
                .into_any_element(),
            ActiveDemo::ShaderShowcase => {
                self.render_shader_showcase(window, cx).into_any_element()
            }
        }
    }
}

impl DemoApp {
    fn render_menu(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> crate::AnyElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(BACKGROUND))
            .justify_center()
            .items_center()
            .gap_6()
            // Title
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(div().text_3xl().text_color(rgb(TEXT)).child("GPUI on iOS"))
                    .child(
                        div()
                            .text_lg()
                            .text_color(rgb(SUBTEXT))
                            .child("Interactive Demos"),
                    ),
            )
            // Demo buttons
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .w(px(300.0))
                    // Animation Playground button
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .px_6()
                            .py_4()
                            .bg(rgb(SURFACE))
                            .rounded_xl()
                            .border_l_4()
                            .border_color(rgb(BLUE))
                            .child(
                                div()
                                    .text_xl()
                                    .text_color(rgb(TEXT))
                                    .child("Animation Playground"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(SUBTEXT))
                                    .child("Bouncing balls & particle effects"),
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.go_to_animation_playground(cx);
                                }),
                            ),
                    )
                    // Shader Showcase button
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .px_6()
                            .py_4()
                            .bg(rgb(SURFACE))
                            .rounded_xl()
                            .border_l_4()
                            .border_color(rgb(MAUVE))
                            .child(
                                div()
                                    .text_xl()
                                    .text_color(rgb(TEXT))
                                    .child("Shader Showcase"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(SUBTEXT))
                                    .child("Dynamic gradients & visual effects"),
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.go_to_shader_showcase(cx);
                                }),
                            ),
                    ),
            )
            // Footer
            .child(
                div()
                    .mt_8()
                    .text_sm()
                    .text_color(rgb(OVERLAY))
                    .child("Powered by GPUI"),
            )
            .into_any_element()
    }
}

impl DemoApp {
    fn render_animation_playground(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> crate::AnyElement {
        // Request continuous animation frame
        window.request_animation_frame();

        // Update bounds
        let viewport = window.viewport_size();
        if let Some(playground) = &mut self.animation_playground {
            playground.set_bounds(Bounds {
                origin: point(0.0, 0.0),
                size: size(viewport.width.0, viewport.height.0),
            });
        }

        div()
            .size_full()
            .bg(rgb(BACKGROUND))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event, _window, cx| {
                    this.handle_animation_touch_down(event, cx);
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event, _window, cx| {
                    this.handle_animation_touch_up(event, cx);
                }),
            )
            .child(if let Some(playground) = &mut self.animation_playground {
                playground
                    .render_with_back_button(window, |_, _window, _cx| {
                        // Back button handled below
                    })
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            .child(back_button(cx.listener(|this, _, _window, cx| {
                this.go_to_menu(cx);
            })))
            .into_any_element()
    }

    fn render_shader_showcase(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> crate::AnyElement {
        // Request continuous animation frame
        window.request_animation_frame();

        // Update screen center
        if let Some(showcase) = &mut self.shader_showcase {
            let viewport = window.viewport_size();
            showcase.set_screen_center(point(viewport.width.0 / 2.0, viewport.height.0 / 2.0));
        }

        div()
            .size_full()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event, _window, cx| {
                    this.handle_shader_touch_down(event, cx);
                }),
            )
            .on_mouse_move(cx.listener(|this, event, _window, cx| {
                this.handle_shader_touch_move(event, cx);
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event, _window, cx| {
                    this.handle_shader_touch_up(event, cx);
                }),
            )
            .child(if let Some(showcase) = &mut self.shader_showcase {
                showcase
                    .render_with_back_button(window, |_, _window, _cx| {
                        // Back button handled below
                    })
                    .into_any_element()
            } else {
                div().into_any_element()
            })
            .child(back_button(cx.listener(|this, _, _window, cx| {
                this.go_to_menu(cx);
            })))
            .into_any_element()
    }
}

/// Back button component for returning to menu
pub fn back_button<F>(on_click: F) -> impl IntoElement
where
    F: Fn(&(), &mut Window, &mut App) + 'static,
{
    div()
        .absolute()
        .top(px(50.0))
        .left(px(20.0))
        .px_4()
        .py_2()
        .bg(hsla(0.0, 0.0, 0.2, 0.8))
        .rounded_lg()
        .text_color(rgb(TEXT))
        .child("< Back")
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            on_click(&(), window, cx);
        })
}
