use scheduler::Instant;
use std::{cell::Cell, rc::Rc, time::Duration};

use crate::{
    AnyElement, App, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    ParentElement, Window,
};

pub use easing::*;
use smallvec::SmallVec;

/// An animation that can be applied to an element.
#[derive(Clone)]
pub struct Animation {
    /// The amount of time for which this animation should run
    pub duration: Duration,
    /// Whether to repeat this animation when it finishes
    pub oneshot: bool,
    /// Whether to derive the phase from a shared clock. See [`Animation::repeat_synced`].
    pub synced: bool,
    /// A function that takes a delta between 0 and 1 and returns a new delta
    /// between 0 and 1 based on the given easing function.
    pub easing: Rc<dyn Fn(f32) -> f32>,
    /// The maximum number of times per second this animation re-renders.
    /// When `None`, the animation re-renders on every frame.
    pub max_fps: Option<f32>,
}

impl Animation {
    /// Create a new animation with the given duration.
    /// By default the animation will only run once and will use a linear easing function.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            oneshot: true,
            synced: false,
            easing: Rc::new(linear),
            max_fps: None,
        }
    }

    /// Set the animation to loop when it finishes.
    pub fn repeat(mut self) -> Self {
        self.oneshot = false;
        self
    }

    /// Set the animation to loop when it finishes, phase-locked to a clock shared by the whole [`App`].
    pub fn repeat_synced(mut self) -> Self {
        self.oneshot = false;
        self.synced = true;
        self
    }

    /// Set the easing function to use for this animation.
    /// The easing function will take a time delta between 0 and 1 and return a new delta
    /// between 0 and 1
    pub fn with_easing(mut self, easing: impl Fn(f32) -> f32 + 'static) -> Self {
        self.easing = Rc::new(easing);
        self
    }

    /// Limit how often this animation re-renders. Instead of re-rendering on
    /// every frame, the animation schedules its next render `1 / max_fps`
    /// seconds after the current one. Values that are not finite and positive
    /// are ignored.
    pub fn with_max_fps(mut self, max_fps: f32) -> Self {
        self.max_fps = Some(max_fps);
        self
    }
}

/// An extension trait for adding the animation wrapper to both Elements and Components
///
/// Animations rendered through this trait automatically respect
/// [`App::reduce_motion`](crate::App::reduce_motion): when it is set,
/// the element is rendered in a static state (the end state for oneshot
/// animations, the start state for repeating ones) and no animation frames are
/// scheduled.
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
            animator: Box::new(move |this, _, value| animator(this, value)),
            animations: smallvec::smallvec![animation],
        }
    }

    /// Render this component or element with a chain of animations
    fn with_animations(
        self,
        id: impl Into<ElementId>,
        animations: Vec<Animation>,
        animator: impl Fn(Self, usize, f32) -> Self + 'static,
    ) -> AnimationElement<Self>
    where
        Self: Sized,
    {
        AnimationElement {
            id: id.into(),
            element: Some(self),
            animator: Box::new(animator),
            animations: animations.into(),
        }
    }
}

impl<E: IntoElement + 'static> AnimationExt for E {}

/// A GPUI element that applies an animation to another element
pub struct AnimationElement<E> {
    id: ElementId,
    element: Option<E>,
    animations: SmallVec<[Animation; 1]>,
    animator: Box<dyn Fn(E, usize, f32) -> E + 'static>,
}

impl<E: ParentElement> ParentElement for AnimationElement<E> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        let Some(element) = &mut self.element else {
            return;
        };

        element.extend(elements);
    }
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
    animation_ix: usize,
    /// Whether a throttled re-render (see [`Animation::with_max_fps`]) is
    /// already scheduled, so overlapping renders don't stack extra timers.
    delayed_frame_pending: Rc<Cell<bool>>,
}

impl<E: IntoElement + 'static> Element for AnimationElement<E> {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        window.with_element_state(global_id.unwrap(), |state, window| {
            let mut state = state.unwrap_or_else(|| AnimationState {
                start: Instant::now(),
                animation_ix: 0,
                delayed_frame_pending: Rc::new(Cell::new(false)),
            });
            let (animation_ix, delta, done) = if cx.reduce_motion() {
                let animation_ix = self.animations.len() - 1;
                let delta = if self.animations[animation_ix].oneshot {
                    1.0
                } else {
                    0.0
                };
                (animation_ix, delta, true)
            } else {
                let animation_ix = state.animation_ix;
                let duration = self.animations[animation_ix].duration;

                let elapsed = if self.animations[animation_ix].synced && !duration.is_zero() {
                    let elapsed = cx.background_executor().now() - cx.synced_animation_epoch;
                    // Reduce modulo the duration before f32 conversion, which loses sub-second precision at scale.
                    Duration::from_nanos((elapsed.as_nanos() % duration.as_nanos()) as u64)
                } else {
                    state.start.elapsed()
                };
                let mut delta = elapsed.as_secs_f32() / duration.as_secs_f32();

                let mut done = false;
                if delta > 1.0 {
                    if self.animations[animation_ix].oneshot {
                        if animation_ix >= self.animations.len() - 1 {
                            done = true;
                        } else {
                            state.start = Instant::now();
                            state.animation_ix += 1;
                        }
                        delta = 1.0;
                    } else {
                        delta %= 1.0;
                    }
                }
                (animation_ix, delta, done)
            };
            let delta = (self.animations[animation_ix].easing)(delta);

            debug_assert!(
                (0.0..=1.0).contains(&delta),
                "delta should always be between 0 and 1"
            );

            let element = self.element.take().expect("should only be called once");
            let mut element = (self.animator)(element, animation_ix, delta).into_any_element();

            if !done {
                match self.animations[animation_ix].max_fps {
                    Some(max_fps) if max_fps.is_finite() && max_fps > 0.0 => {
                        if !state.delayed_frame_pending.get() {
                            state.delayed_frame_pending.set(true);
                            let delayed_frame_pending = state.delayed_frame_pending.clone();
                            let view = window.current_view();
                            let interval = Duration::from_secs_f32(1.0 / max_fps);
                            window
                                .spawn(cx, async move |cx| {
                                    cx.background_executor().timer(interval).await;
                                    delayed_frame_pending.set(false);
                                    cx.update(move |_, cx| cx.notify(view)).ok();
                                })
                                .detach();
                        }
                    }
                    _ => window.request_animation_frame(),
                }
            }

            ((element.request_layout(window, cx), element), state)
        })
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
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
        _inspector_id: Option<&InspectorElementId>,
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

    /// The Quint ease-out function, which starts quickly and decelerates to a stop
    pub fn ease_out_quint() -> impl Fn(f32) -> f32 {
        move |delta| 1.0 - (1.0 - delta).powi(5)
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, time::Duration};

    use crate::{
        Animation, Context, InteractiveElement, Render, TestAppContext, WindowHandle, div,
        prelude::*, px, size,
    };

    use super::*;

    struct AnimationTestView {
        rendered_deltas: Rc<RefCell<Vec<f32>>>,
        max_fps: Option<f32>,
    }

    struct SyncedAnimationTestView {
        show_second: bool,
        first_deltas: Rc<RefCell<Vec<f32>>>,
        second_deltas: Rc<RefCell<Vec<f32>>>,
    }

    impl Render for SyncedAnimationTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let record_deltas = |deltas: Rc<RefCell<Vec<f32>>>| {
                move |this, delta| {
                    deltas.borrow_mut().push(delta);
                    this
                }
            };
            div()
                .size_full()
                .child(div().with_animation(
                    "first-synced-animation",
                    Animation::new(Duration::from_secs(1)).repeat_synced(),
                    record_deltas(self.first_deltas.clone()),
                ))
                .when(self.show_second, |this| {
                    this.child(div().with_animation(
                        "second-synced-animation",
                        Animation::new(Duration::from_secs(1)).repeat_synced(),
                        record_deltas(self.second_deltas.clone()),
                    ))
                })
        }
    }

    impl Render for AnimationTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let rendered_deltas = self.rendered_deltas.clone();
            // The throttled variant syncs to the shared clock so the deltas
            // follow the test scheduler's clock rather than wall time.
            let mut animation = Animation::new(Duration::from_secs(1));
            if let Some(max_fps) = self.max_fps {
                animation = animation.repeat_synced().with_max_fps(max_fps);
            } else {
                animation = animation.repeat();
            }
            div().size_full().child(div().with_animation(
                "repeating-animation",
                animation,
                move |this, delta| {
                    rendered_deltas.borrow_mut().push(delta);
                    this
                },
            ))
        }
    }

    fn open_test_window(
        cx: &mut TestAppContext,
    ) -> (Rc<RefCell<Vec<f32>>>, WindowHandle<AnimationTestView>) {
        open_test_window_with_max_fps(cx, None)
    }

    fn open_test_window_with_max_fps(
        cx: &mut TestAppContext,
        max_fps: Option<f32>,
    ) -> (Rc<RefCell<Vec<f32>>>, WindowHandle<AnimationTestView>) {
        let rendered_deltas = Rc::new(RefCell::new(Vec::new()));
        let window = cx.open_window(size(px(100.), px(100.)), {
            let rendered_deltas = rendered_deltas.clone();
            move |_, _| AnimationTestView {
                rendered_deltas,
                max_fps,
            }
        });
        cx.run_until_parked();
        (rendered_deltas, window)
    }

    fn simulate_next_frame<V: Render>(window: &WindowHandle<V>, cx: &mut TestAppContext) -> usize {
        let callback_count = window
            .update(cx, |_, window, cx| window.simulate_next_frame(cx))
            .unwrap();
        cx.run_until_parked();
        callback_count
    }
    // Before parent-animation-element, using .with_animation
    // would not allow chaining .parent after. This is just a
    // build check that we can call div().id().with_animation().child()
    #[test]
    fn test_animation_parent() {
        div()
            .id("id")
            //
            .with_animation(
                "animation",
                Animation::new(Duration::from_secs(1)),
                |el, _t| {
                    //
                    el
                },
            )
            .child(
                //
                div(),
            );
    }

    #[gpui::test]
    fn test_repeating_animation_schedules_animation_frames(cx: &mut TestAppContext) {
        let (rendered_deltas, window) = open_test_window(cx);

        assert_eq!(rendered_deltas.borrow().len(), 1);

        for expected_frames in 2..=3 {
            assert_eq!(simulate_next_frame(&window, cx), 1);
            assert_eq!(rendered_deltas.borrow().len(), expected_frames);
        }
    }

    #[gpui::test]
    fn test_max_fps_schedules_timer_driven_frames(cx: &mut TestAppContext) {
        let (rendered_deltas, window) = open_test_window_with_max_fps(cx, Some(10.0));

        // The test scheduler's clock jitters forward slightly on each poll,
        // so compare against expectations loosely.
        let assert_deltas_approx_eq = |expected: &[f32]| {
            let actual = rendered_deltas.borrow();
            assert_eq!(actual.len(), expected.len(), "deltas: {actual:?}");
            for (actual, expected) in actual.iter().zip(expected) {
                assert!(
                    (actual - expected).abs() < 1e-2,
                    "expected {expected}, got {actual}"
                );
            }
        };

        assert_deltas_approx_eq(&[0.0]);

        // No per-frame callback is scheduled; re-renders are timer-driven.
        assert_eq!(simulate_next_frame(&window, cx), 0);
        assert_deltas_approx_eq(&[0.0]);

        cx.executor().advance_clock(Duration::from_millis(105));
        cx.run_until_parked();
        assert_deltas_approx_eq(&[0.0, 0.105]);

        cx.executor().advance_clock(Duration::from_millis(105));
        cx.run_until_parked();
        assert_deltas_approx_eq(&[0.0, 0.105, 0.21]);
    }

    #[gpui::test]
    fn test_synced_animations_share_phase_across_elements(cx: &mut TestAppContext) {
        let first_deltas = Rc::new(RefCell::new(Vec::new()));
        let second_deltas = Rc::new(RefCell::new(Vec::new()));
        let window = cx.open_window(size(px(100.), px(100.)), {
            let first_deltas = first_deltas.clone();
            let second_deltas = second_deltas.clone();
            move |_, _| SyncedAnimationTestView {
                show_second: false,
                first_deltas,
                second_deltas,
            }
        });
        cx.run_until_parked();

        assert_eq!(*first_deltas.borrow(), vec![0.0]);

        cx.executor().advance_clock(Duration::from_millis(250));
        simulate_next_frame(&window, cx);
        assert_eq!(*first_deltas.borrow(), vec![0.0, 0.25]);

        // The second element mounts a quarter through the cycle, yet renders
        // the shared phase rather than starting at zero.
        window
            .update(cx, |view, _, cx| {
                view.show_second = true;
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(250));
        simulate_next_frame(&window, cx);

        assert_eq!(*second_deltas.borrow().last().unwrap(), 0.5);
        assert_eq!(
            *first_deltas.borrow().last().unwrap(),
            *second_deltas.borrow().last().unwrap()
        );
        assert!(second_deltas.borrow().iter().all(|delta| *delta > 0.0));

        // The phase wraps around each full cycle.
        cx.executor().advance_clock(Duration::from_millis(2250));
        simulate_next_frame(&window, cx);
        assert_eq!(*first_deltas.borrow().last().unwrap(), 0.75);

        // Sub-second precision survives months of uptime: converting the raw
        // elapsed time to f32 would round 0.25 away entirely.
        cx.executor()
            .advance_clock(Duration::from_secs(300 * 24 * 60 * 60) + Duration::from_millis(500));
        simulate_next_frame(&window, cx);
        assert_eq!(*first_deltas.borrow().last().unwrap(), 0.25);
    }

    #[gpui::test]
    fn test_reduce_motion_renders_single_static_frame(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_reduce_motion(true));
        let (rendered_deltas, window) = open_test_window(cx);

        assert_eq!(*rendered_deltas.borrow(), vec![0.0]);

        assert_eq!(simulate_next_frame(&window, cx), 0);
        assert_eq!(*rendered_deltas.borrow(), vec![0.0]);
    }
}
