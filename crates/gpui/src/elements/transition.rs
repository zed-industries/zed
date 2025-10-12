use std::{
    borrow::BorrowMut,
    fmt::Debug,
    ops::{Add, Mul, Sub},
    rc::Rc,
    time::{Duration, Instant},
};

use crate::{
    AnimatableExt, AnyElement, App, Bounds, Corners, DevicePixels, Edges, ElementId, Entity,
    GlobalElementId, Hsla, InspectorElementId, IntoElement, Percentage, Pixels, Point, Radians,
    Rems, Rgba, ScaledPixels, Size, Window, colors::Colors, linear,
};

/// A transition that can be applied to an element.
#[derive(Clone)]
pub struct Transition<T: TransitionGoal + Clone> {
    /// The amount of time for which this transtion should run.
    duration_secs: f32,

    /// A function that takes a delta between 0 and 1 and returns a new delta
    /// between 0 and 1 based on the given easing function.
    easing: Rc<dyn Fn(f32) -> f32>,

    state: Entity<TransitionState<T>>,
}

impl<T: TransitionGoal + Clone + PartialEq + 'static> Transition<T> {
    /// Reads the transition's goal.
    pub fn read_goal<'a>(&self, cx: &'a App) -> &'a T {
        &self.state.read(cx).current_goal
    }

    /// Updates the goal for the transition without notifying gpui of any changes.
    pub fn update_goal_silently(&self, new_goal: T, cx: &mut App) -> bool {
        let mut was_updated = false;

        self.state.update(cx, |state, _cx| {
            if state.current_goal == new_goal {
                return;
            };

            state.goal_last_updated_at = Instant::now();
            state.last_goal = std::mem::replace(&mut state.current_goal, new_goal);
            state.start_delta = 1. - state.last_delta;

            was_updated = true;
        });

        was_updated
    }

    /// Updates the goal for the transition and notifies gpui
    /// of the change if the new goal is different from the last.
    pub fn update_goal(&self, new_goal: impl Into<T>, cx: &mut App) -> bool {
        let was_updated = self.update_goal_silently(new_goal.into(), cx);

        if was_updated {
            cx.notify(self.state.entity_id());
        }

        was_updated
    }
}

/// State for a transition.
#[derive(Clone)]
pub struct TransitionState<T: TransitionGoal + Clone> {
    goal_last_updated_at: Instant,
    current_goal: T,
    start_delta: f32,
    last_delta: f32,
    last_goal: T,
}

impl<T: TransitionGoal + Clone> TransitionState<T> {
    fn new(initial_goal: T) -> Self {
        Self {
            goal_last_updated_at: Instant::now(),
            current_goal: initial_goal.clone(),
            start_delta: 1.,
            last_delta: 1.,
            last_goal: initial_goal,
        }
    }
}

impl<T: TransitionGoal + Clone + 'static> Transition<T> {
    /// Create a new transition with the given duration and goal.
    pub fn new(
        id: impl Into<ElementId>,
        duration: Duration,
        initial_goal: T,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            duration_secs: duration.as_secs_f32(),
            easing: Rc::new(linear),
            state: window
                .use_keyed_state(id, cx, |_window, _cx| TransitionState::new(initial_goal)),
        }
    }

    /// Create a new transition with the given duration using the specified state.
    pub fn from_state(state: Entity<TransitionState<T>>, duration: Duration) -> Self {
        Self {
            duration_secs: duration.as_secs_f32(),
            easing: Rc::new(linear),
            state,
        }
    }

    /// Set the easing function to use for this transition.
    /// The easing function will take a time delta between 0 and 1 and return a new delta
    /// between 0 and 1
    pub fn with_easing(mut self, easing: impl Fn(f32) -> f32 + 'static) -> Self {
        self.easing = Rc::new(easing);
        self
    }
}

/// An extension trait for adding the transition wrapper to both Elements and Components
pub trait TransitionExt {
    /// Render this component or element with a transition
    fn with_transition<T: TransitionGoal + Clone>(
        self,
        transition: Transition<T>,
        animator: impl Fn(&mut App, Self, T) -> Self + 'static,
    ) -> TransitionElement<Self, T>
    where
        Self: Sized,
    {
        TransitionElement {
            element: Some(self),
            animator: Box::new(animator),
            transition,
        }
    }
}

impl<E: IntoElement + 'static> TransitionExt for E {}

/// A GPUI element that applies a transition to another element
pub struct TransitionElement<E, T: TransitionGoal + Clone> {
    element: Option<E>,
    transition: Transition<T>,
    animator: Box<dyn Fn(&mut App, E, T) -> E + 'static>,
}

impl<E, T: TransitionGoal + Clone> TransitionElement<E, T> {
    /// Returns a new [`TransitionElement<E, T>`] after applying the given function
    /// to the element being animated.
    pub fn map_element(mut self, f: impl FnOnce(E) -> E) -> TransitionElement<E, T> {
        self.element = self.element.map(f);
        self
    }
}

impl<E: IntoElement + 'static, T: TransitionGoal + Clone + 'static> IntoElement
    for TransitionElement<E, T>
{
    type Element = TransitionElement<E, T>;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<E: IntoElement + 'static, T: TransitionGoal + Clone + 'static> AnimatableExt
    for TransitionElement<E, T>
{
    fn request_layout(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> (bool, AnyElement) {
        let mut state_entity = self.transition.state.as_mut(cx);
        let state: &mut TransitionState<T> = state_entity.borrow_mut();

        let elapsed_secs = state.goal_last_updated_at.elapsed().as_secs_f32();
        let duration_secs = self.transition.duration_secs;
        let delta =
            (self.transition.easing)((state.start_delta + (elapsed_secs / duration_secs)).min(1.));

        debug_assert!(
            (0.0..=1.0).contains(&delta),
            "delta should always be between 0 and 1"
        );

        let transition_value = state.last_goal.apply_delta(&state.current_goal, delta);

        state.last_delta = delta;
        drop(state_entity);

        let element = self.element.take().expect("should only be called once");
        let mut element = (self.animator)(cx, element, transition_value).into_any_element();

        (delta != 1., element)
    }
}

/// A data type which can be used as a transition goal.
pub trait TransitionGoal {
    /// Defines how a delta is applied to a value.
    fn apply_delta(&self, to: &Self, delta: f32) -> Self;
}

macro_rules! float_transition_goals {
    ( $( $ty:ty ),+ ) => {
        $(
            impl TransitionGoal for $ty {
                fn apply_delta(&self, to: &Self, delta: f32) -> Self {
                    lerp(*self, *to, delta as $ty)
                }
            }
        )+
    };
}

float_transition_goals!(f32, f64);

macro_rules! int_transition_goals {
    ( $( $ty:ident as $ty_into:ident ),+ ) => {
        $(
            impl TransitionGoal for $ty {
                fn apply_delta(&self, to: &Self, delta: f32) -> Self {
                    lerp(*self as $ty_into, *to as $ty_into, delta as $ty_into) as $ty
                }
            }
        )+
    };
}

int_transition_goals!(
    usize as f32,
    u8 as f32,
    u16 as f32,
    u32 as f32,
    u64 as f64,
    u128 as f64,
    isize as f32,
    i8 as f32,
    i16 as f32,
    i32 as f32,
    i64 as f64,
    i128 as f64
);

macro_rules! struct_transition_goals {
    ( $( $ty:ident $( < $gen:ident > )? { $( $n:ident ),+ } ),+ $(,)? ) => {
        $(
            impl$(<$gen: TransitionGoal + Clone + Debug + Default + PartialEq>)? TransitionGoal for $ty$(<$gen>)? {
                fn apply_delta(&self, to: &Self, delta: f32) -> Self {
                    $ty$(::<$gen>)? {
                        $(
                            $n: self.$n.apply_delta(&to.$n, delta)
                        ),+
                    }
                }
            }
        )+
    };
}

struct_transition_goals!(
    Point<T> { x, y },
    Size<T> { width, height },
    Edges<T> { top, right, bottom, left },
    Corners<T> { top_left, top_right, bottom_right, bottom_left },
    Bounds<T> { origin, size },
    Rgba { r, g, b, a },
    Hsla { h, s, l, a },
    Colors { text, selected_text, background, disabled, selected, border, separator, container }
);

macro_rules! tuple_struct_transition_goals {
    ( $( $ty:ident ( $n:ty ) ),+ ) => {
        $(
            impl TransitionGoal for $ty {
                fn apply_delta(&self, to: &Self, delta: f32) -> Self {
                    $ty(self.0.apply_delta(&to.0, delta))
                }
            }
        )+
    };
}

tuple_struct_transition_goals!(
    Radians(f32),
    Percentage(f32),
    Pixels(f32),
    DevicePixels(i32),
    ScaledPixels(f32),
    Rems(f32)
);

fn lerp<T>(a: T, b: T, t: T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    a + (b - a) * t
}
