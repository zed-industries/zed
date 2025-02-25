use std::time::{Duration, Instant};

use crate::{AnyElement, App, Element, ElementId, GlobalElementId, IntoElement, Window};

pub use easing::*;

/// An animation that can be applied to an element.
pub struct Animation {
    /// The amount of time for which this animation should run
    pub duration: Duration,
    /// Whether to repeat this animation when it finishes
    pub oneshot: bool,
    /// A function that takes a delta between 0 and 1 and returns a new delta
    /// between 0 and 1 based on the given easing function.
    pub easing: Box<dyn Fn(f32) -> f32>,
}

impl Animation {
    /// Create a new animation with the given duration.
    /// By default the animation will only run once and will use a linear easing function.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            oneshot: true,
            easing: Box::new(linear),
        }
    }

    /// Set the animation to loop when it finishes.
    pub fn repeat(mut self) -> Self {
        self.oneshot = false;
        self
    }

    /// Set the easing function to use for this animation.
    /// The easing function will take a time delta between 0 and 1 and return a new delta
    /// between 0 and 1
    pub fn with_easing(mut self, easing: impl Fn(f32) -> f32 + 'static) -> Self {
        self.easing = Box::new(easing);
        self
    }
}

/// An animation which can be appied to an element when transitioning between states
pub struct TransitionAnimation {
    /// The amount of time this animation should run for when transitioning from false to true.
    /// When `None`, this transition isn't animated.
    pub forward_duration: Option<Duration>,

    /// The amount of time this animation should run for when transitioning from true to false.
    /// When `None`, this transition isn't animated.
    pub backward_duration: Option<Duration>,

    /// A function that takes a delta between 0 and 1 and returns a new delta
    /// between 0 and 1 based on the given easing function.
    pub easing: Box<dyn Fn(f32) -> f32>,
}

impl TransitionAnimation {
    /// Create a new transition animation with the given duration.
    /// By default the animation will run in both directions and will use a linear easing function.
    pub fn new(duration: Duration) -> Self {
        Self {
            forward_duration: Some(duration),
            backward_duration: Some(duration),
            easing: Box::new(linear),
        }
    }

    /// Sets he amount of time this animation should run for when transitioning from false to true.
    /// When `None`, this transition isn't animated.
    pub fn forward(mut self, duration: Option<Duration>) -> Self {
        self.forward_duration = duration;
        self
    }

    /// Sets he amount of time this animation should run for when transitioning from true to false.
    /// When `None`, this transition isn't animated.
    pub fn backward(mut self, duration: Option<Duration>) -> Self {
        self.backward_duration = duration;
        self
    }

    /// Set the easing function to use for this animation.
    /// The easing function will take a time delta between 0 and 1 and return a new delta
    /// between 0 and 1
    pub fn with_easing(mut self, easing: impl Fn(f32) -> f32 + 'static) -> Self {
        self.easing = Box::new(easing);
        self
    }
}

/// An extension trait for adding the animation wrapper to both Elements and Components
pub trait AnimationExt {
    /// Render this component or element with an animation
    fn with_animation(
        self,
        id: impl Into<ElementId>,
        animation: Animation,
        animator: impl Fn(Self, f32) -> Self + 'static,
    ) -> AnimationElement<Self>
    where
        Self: Sized,
    {
        AnimationElement {
            id: id.into(),
            element: Some(self),
            animator: Box::new(animator),
            animation,
        }
    }

    /// Render this component or element with an animation between states
    fn with_transition(
        self,
        condition: bool,
        id: impl Into<ElementId>,
        animation: TransitionAnimation,
        animator: impl Fn(Self, bool, f32) -> Self + 'static,
    ) -> TransitionElement<Self>
    where
        Self: Sized,
    {
        TransitionElement {
            id: id.into(),
            condition,
            element: Some(self),
            animation,
            animator: Box::new(animator),
        }
    }
}

impl<E> AnimationExt for E {}

/// A GPUI element that applies an animation to another element
pub struct AnimationElement<E> {
    id: ElementId,
    element: Option<E>,
    animation: Animation,
    animator: Box<dyn Fn(E, f32) -> E + 'static>,
}

impl<E> AnimationElement<E> {
    /// Returns a new [`AnimationElement<E>`] after applying the given function
    /// to the element being animated.
    pub fn map_element(mut self, f: impl FnOnce(E) -> E) -> AnimationElement<E> {
        self.element = self.element.map(f);
        self
    }
}

impl<E: IntoElement + 'static> IntoElement for AnimationElement<E> {
    type Element = AnimationElement<E>;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct AnimationState {
    start: Instant,
}

impl<E: IntoElement + 'static> Element for AnimationElement<E> {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        window.with_element_state(global_id.unwrap(), |state, window| {
            let state = state.unwrap_or_else(|| AnimationState {
                start: Instant::now(),
            });
            let mut delta =
                state.start.elapsed().as_secs_f32() / self.animation.duration.as_secs_f32();

            let mut done = false;
            if delta > 1.0 {
                if self.animation.oneshot {
                    done = true;
                    delta = 1.0;
                } else {
                    delta %= 1.0;
                }
            }
            let delta = (self.animation.easing)(delta);

            debug_assert!(
                (0.0..=1.0).contains(&delta),
                "delta should always be between 0 and 1"
            );

            let element = self.element.take().expect("should only be called once");
            let mut element = (self.animator)(element, delta).into_any_element();

            if !done {
                window.request_animation_frame();
            }

            ((element.request_layout(window, cx), element), state)
        })
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: crate::Bounds<crate::Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: crate::Bounds<crate::Pixels>,
        element: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.paint(window, cx);
    }
}

/// A GPUI element that applies an animation to another element when transitioning between states
pub struct TransitionElement<E> {
    id: ElementId,
    condition: bool,
    element: Option<E>,
    animation: TransitionAnimation,
    animator: Box<dyn Fn(E, bool, f32) -> E + 'static>,
}

impl<E> TransitionElement<E> {
    /// Returns a new [`TransitionElement<E>`] after applying the given function
    /// to the element being animated.
    pub fn map_element(mut self, f: impl FnOnce(E) -> E) -> TransitionElement<E> {
        self.element = self.element.map(f);
        self
    }
}

impl<E: IntoElement + 'static> IntoElement for TransitionElement<E> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<E: IntoElement + 'static> Element for TransitionElement<E> {
    type RequestLayoutState = AnyElement;

    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        window.with_element_state(id.unwrap(), |state, window| {
            let (mut start_time, mut start_delta, animating_forward) =
                state.unwrap_or_else(|| (Instant::now(), 1.0, self.condition));

            let raw_delta = match (
                animating_forward,
                self.animation.forward_duration,
                self.animation.backward_duration,
            ) {
                (true, Some(duration), _) | (false, _, Some(duration)) => {
                    start_time.elapsed().as_secs_f32() / duration.as_secs_f32() + start_delta
                }
                _ => 1.0,
            };

            let mut done = raw_delta > 1.0;
            let raw_delta = raw_delta.min(1.0);

            let delta = (self.animation.easing)(if animating_forward {
                raw_delta
            } else {
                1.0 - raw_delta
            });

            if self.condition != animating_forward {
                start_delta = 1.0 - raw_delta;
                start_time = Instant::now();
                done = false;
            }

            // Animate element
            let element = self.element.take().expect("should only be called once");
            let mut element = if !done || animating_forward {
                (self.animator)(element, self.condition, delta)
            } else {
                element
            }
            .into_any_element();

            if !done {
                window.request_animation_frame();
            }

            (
                (element.request_layout(window, cx), element),
                (start_time, start_delta, self.condition),
            )
        })
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: crate::Bounds<crate::Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: crate::Bounds<crate::Pixels>,
        element: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.paint(window, cx);
    }
}

mod easing {
    use std::f32::consts::PI;

    /// The linear easing function, or delta itself
    pub fn linear(delta: f32) -> f32 {
        delta
    }

    /// The quadratic easing function, delta * delta
    pub fn quadratic(delta: f32) -> f32 {
        delta * delta
    }

    /// The quadratic ease-in-out function, which starts and ends slowly but speeds up in the middle
    pub fn ease_in_out(delta: f32) -> f32 {
        if delta < 0.5 {
            2.0 * delta * delta
        } else {
            let x = -2.0 * delta + 2.0;
            1.0 - x * x / 2.0
        }
    }

    /// Apply the given easing function, first in the forward direction and then in the reverse direction
    pub fn bounce(easing: impl Fn(f32) -> f32) -> impl Fn(f32) -> f32 {
        move |delta| {
            if delta < 0.5 {
                easing(delta * 2.0)
            } else {
                easing((1.0 - delta) * 2.0)
            }
        }
    }

    /// A custom easing function for pulsating alpha that slows down as it approaches 0.1
    pub fn pulsating_between(min: f32, max: f32) -> impl Fn(f32) -> f32 {
        let range = max - min;

        move |delta| {
            // Use a combination of sine and cubic functions for a more natural breathing rhythm
            let t = (delta * 2.0 * PI).sin();
            let breath = (t * t * t + t) / 2.0;

            // Map the breath to our desired alpha range
            let normalized_alpha = (breath + 1.0) / 2.0;

            min + (normalized_alpha * range)
        }
    }
}
