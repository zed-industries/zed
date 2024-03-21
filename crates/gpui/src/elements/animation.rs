use std::time::{Duration, Instant};

use crate::{AnyElement, Element, ElementId, IntoElement};

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
}

impl<E> AnimationExt for E {}

/// A GPUI element that applies an animation to another element
pub struct AnimationElement<E> {
    id: ElementId,
    element: Option<E>,
    animation: Animation,
    animator: Box<dyn Fn(E, f32) -> E + 'static>,
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
    type BeforeLayout = AnyElement;

    type AfterLayout = ();

    fn before_layout(
        &mut self,
        cx: &mut crate::ElementContext,
    ) -> (crate::LayoutId, Self::BeforeLayout) {
        cx.with_element_state(Some(self.id.clone()), |state, cx| {
            let state = state.unwrap().unwrap_or_else(|| AnimationState {
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
                    delta = delta % 1.0;
                }
            }
            let delta = (self.animation.easing)(delta);

            debug_assert!(
                delta >= 0.0 && delta <= 1.0,
                "delta should always be between 0 and 1"
            );

            let element = self.element.take().expect("should only be called once");
            let mut element = (self.animator)(element, delta).into_any_element();

            if !done {
                let parent_id = cx.parent_view_id();
                cx.on_next_frame(move |cx| {
                    if let Some(parent_id) = parent_id {
                        cx.notify(parent_id)
                    } else {
                        cx.refresh()
                    }
                })
            }

            ((element.before_layout(cx), element), Some(state))
        })
    }

    fn after_layout(
        &mut self,
        _bounds: crate::Bounds<crate::Pixels>,
        element: &mut Self::BeforeLayout,
        cx: &mut crate::ElementContext,
    ) -> Self::AfterLayout {
        element.after_layout(cx);
    }

    fn paint(
        &mut self,
        _bounds: crate::Bounds<crate::Pixels>,
        element: &mut Self::BeforeLayout,
        _: &mut Self::AfterLayout,
        cx: &mut crate::ElementContext,
    ) {
        element.paint(cx);
    }
}

mod easing {
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
}
