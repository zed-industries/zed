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

pub trait DefaultAnimations: Styled + Sized {
    fn animate_in(
        self,
        animation_type: AnimationDirection,
        fade_in: bool,
    ) -> AnimationElement<Self> {
        let animation_name = match animation_type {
            AnimationDirection::FromBottom => "animate_from_bottom",
            AnimationDirection::FromLeft => "animate_from_left",
            AnimationDirection::FromRight => "animate_from_right",
            AnimationDirection::FromTop => "animate_from_top",
        };

        self.with_animation(
            animation_name,
            gpui::Animation::new(AnimationDuration::Fast.into()).with_easing(ease_out_quint()),
            move |mut this, delta| {
                let start_opacity = 0.4;
                let start_pos = 0.0;
                let end_pos = 40.0;

                if fade_in {
                    this = this.opacity(start_opacity + delta * (1.0 - start_opacity));
                }

                match animation_type {
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
            },
        )
    }

    fn animate_in_from_bottom(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationDirection::FromBottom, fade)
    }

    fn animate_in_from_left(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationDirection::FromLeft, fade)
    }

    fn animate_in_from_right(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationDirection::FromRight, fade)
    }

    fn animate_in_from_top(self, fade: bool) -> AnimationElement<Self> {
        self.animate_in(AnimationDirection::FromTop, fade)
    }
}

impl<E: Styled> DefaultAnimations for E {}

// Don't use this directly, it only exists to show animation previews
#[derive(RegisterComponent)]
struct Animation {}

impl Component for Animation {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::None
    }

    fn description() -> Option<&'static str> {
        Some("Demonstrates various animation patterns and transitions available in the UI system.")
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let container_size = 128.0;
        let element_size = 32.0;
        let left_offset = element_size - container_size / 2.0;
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::red())
                                            .animate_in(AnimationDirection::FromBottom, false),
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::blue())
                                            .animate_in(AnimationDirection::FromTop, false),
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::green())
                                            .animate_in(AnimationDirection::FromLeft, false),
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::yellow())
                                            .animate_in(AnimationDirection::FromRight, false),
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::red())
                                            .animate_in(AnimationDirection::FromBottom, true),
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::blue())
                                            .animate_in(AnimationDirection::FromTop, true),
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::green())
                                            .animate_in(AnimationDirection::FromLeft, true),
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
                                            .left(px(left_offset))
                                            .rounded_md()
                                            .bg(gpui::yellow())
                                            .animate_in(AnimationDirection::FromRight, true),
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
