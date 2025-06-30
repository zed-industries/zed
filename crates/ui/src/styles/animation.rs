use crate::{ContentGroup, prelude::*};
use gpui::{AnimationElement, AnimationExt, Styled};
use std::time::Duration;

use gpui::ease_out_quint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationDuration {
    Instant = 50,
    Fast = 150,
    Slow = 300,
}

impl AnimationDuration {
    pub fn duration(&self) -> Duration {
        Duration::from_millis(*self as u64)
    }
}

impl Into<std::time::Duration> for AnimationDuration {
    fn into(self) -> Duration {
        self.duration()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationDirection {
    FromBottom,
    FromLeft,
    FromRight,
    FromTop,
}

pub struct AnimationOptions {
    pub animation_type: AnimationDirection,
    pub fade_in: bool,
    pub(crate) repeat: bool,
}

impl AnimationOptions {
    pub fn new(animation_type: AnimationDirection, fade_in: bool) -> Self {
        Self {
            animation_type,
            fade_in,
            repeat: false,
        }
    }

    pub fn with_repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }
}

pub trait DefaultAnimations: Styled + Sized {
    fn animate_in(self, options: AnimationOptions) -> AnimationElement<Self> {
        let animation_name = match options.animation_type {
            AnimationDirection::FromBottom => "animate_from_bottom",
            AnimationDirection::FromLeft => "animate_from_left",
            AnimationDirection::FromRight => "animate_from_right",
            AnimationDirection::FromTop => "animate_from_top",
        };

        let mut animations = vec![
            gpui::Animation::new(AnimationDuration::Slow.into()).with_easing(ease_out_quint()),
        ];

        if options.repeat {
            animations.push(gpui::Animation::new(Duration::from_secs(1)));
        }

        let animation_element = self.with_animations(
            animation_name,
            animations,
            move |mut this, animation_idx, delta| match animation_idx {
                0 => {
                    let start_opacity = 0.4;
                    let start_pos = 0.0;
                    let end_pos = 40.0;

                    if options.fade_in {
                        this = this.opacity(start_opacity + delta * (1.0 - start_opacity));
                    }

                    match options.animation_type {
                        AnimationDirection::FromBottom => {
                            this.bottom(px(start_pos + delta * (end_pos - start_pos)))
                        }
                        AnimationDirection::FromLeft => {
                            this.left(px(start_pos + delta * (end_pos - start_pos)))
                        }
                        AnimationDirection::FromRight => {
                            this.right(px(start_pos + delta * (end_pos - start_pos)))
                        }
                        AnimationDirection::FromTop => {
                            this.top(px(start_pos + delta * (end_pos - start_pos)))
                        }
                    }
                }
                1 => {
                    if options.fade_in {
                        this = this.opacity(1.0);
                    }

                    match options.animation_type {
                        AnimationDirection::FromBottom => this.bottom(px(40.0)),
                        AnimationDirection::FromLeft => this.left(px(40.0)),
                        AnimationDirection::FromRight => this.right(px(40.0)),
                        AnimationDirection::FromTop => this.top(px(40.0)),
                    }
                }
                _ => this,
            },
        );

        if options.repeat {
            animation_element.repeat()
        } else {
            animation_element
        }
    }

    fn animate_in_from_bottom(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationOptions::new(AnimationDirection::FromBottom, fade))
    }

    fn animate_in_from_left(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationOptions::new(AnimationDirection::FromLeft, fade))
    }

    fn animate_in_from_right(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationOptions::new(AnimationDirection::FromRight, fade))
    }

    fn animate_in_from_top(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationOptions::new(AnimationDirection::FromTop, fade))
    }
}

impl<E: Styled> DefaultAnimations for E {}

// Don't use this directly, it only exists to show animation previews
#[derive(RegisterComponent)]
struct Animation {}

impl Component for Animation {
    fn scope() -> ComponentScope {
        ComponentScope::None
    }

    fn description() -> Option<&'static str> {
        Some("Demonstrates various animation patterns and transitions available in the UI system.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let container_size = 128.0;
        let element_size = 32.0;
        let offset = container_size / 2.0 - element_size / 2.0;
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Animate In",
                        vec![
                            single_example(
                                "From Bottom",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("animate-in-from-bottom")
                                            .absolute()
                                            .size(px(element_size))
                                            .left(px(offset))
                                            .rounded_md()
                                            .bg(gpui::red())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromBottom,
                                                    false,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                            single_example(
                                "From Top",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("animate-in-from-top")
                                            .absolute()
                                            .size(px(element_size))
                                            .left(px(offset))
                                            .rounded_md()
                                            .bg(gpui::blue())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromTop,
                                                    false,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                            single_example(
                                "From Left",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("animate-in-from-left")
                                            .absolute()
                                            .size(px(element_size))
                                            .top(px(offset))
                                            .rounded_md()
                                            .bg(gpui::green())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromLeft,
                                                    false,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                            single_example(
                                "From Right",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("animate-in-from-right")
                                            .absolute()
                                            .size(px(element_size))
                                            .top(px(offset))
                                            .rounded_md()
                                            .bg(gpui::yellow())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromRight,
                                                    false,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                        ],
                    )
                    .grow(),
                    example_group_with_title(
                        "Fade and Animate In",
                        vec![
                            single_example(
                                "From Bottom",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("fade-animate-in-from-bottom")
                                            .absolute()
                                            .size(px(element_size))
                                            .left(px(offset))
                                            .rounded_md()
                                            .bg(gpui::red())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromBottom,
                                                    true,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                            single_example(
                                "From Top",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("fade-animate-in-from-top")
                                            .absolute()
                                            .size(px(element_size))
                                            .left(px(offset))
                                            .rounded_md()
                                            .bg(gpui::blue())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromTop,
                                                    true,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                            single_example(
                                "From Left",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("fade-animate-in-from-left")
                                            .absolute()
                                            .size(px(element_size))
                                            .top(px(offset))
                                            .rounded_md()
                                            .bg(gpui::green())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromLeft,
                                                    true,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                            single_example(
                                "From Right",
                                ContentGroup::new()
                                    .relative()
                                    .items_center()
                                    .justify_center()
                                    .size(px(container_size))
                                    .child(
                                        div()
                                            .id("fade-animate-in-from-right")
                                            .absolute()
                                            .size(px(element_size))
                                            .top(px(offset))
                                            .rounded_md()
                                            .bg(gpui::yellow())
                                            .animate_in(
                                                AnimationOptions::new(
                                                    AnimationDirection::FromRight,
                                                    true,
                                                )
                                                .with_repeat(true),
                                            ),
                                    )
                                    .into_any_element(),
                            ),
                        ],
                    )
                    .grow(),
                ])
                .into_any_element(),
        )
    }
}
