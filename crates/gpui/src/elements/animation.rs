use scheduler::Instant;
use std::{rc::Rc, rc::Weak, time::Duration};

use crate::{
    AnyElement, App, Element, ElementId, EntityId, GlobalElementId, InspectorElementId,
    IntoElement, ParentElement, Task, Window,
};
use gpui_util::ResultExt as _;

pub use easing::*;
use smallvec::SmallVec;

/// An animation that can be applied to an element.
#[derive(Clone)]
pub struct Animation {
    /// The amount of time for which this animation should run
    pub duration: Duration,
    /// Whether to repeat this animation when it finishes
    pub oneshot: bool,
    /// A function that takes a delta between 0 and 1 and returns a new delta
    /// between 0 and 1 based on the given easing function.
    pub easing: Rc<dyn Fn(f32) -> f32>,
}

impl Animation {
    /// Create a new animation with the given duration.
    /// By default the animation will only run once and will use a linear easing function.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            oneshot: true,
            easing: Rc::new(linear),
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
        self.easing = Rc::new(easing);
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
            frame_interval: None,
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
            frame_interval: None,
        }
    }
}

impl<E: IntoElement + 'static> AnimationExt for E {}

/// A GPUI element that applies an animation to another element
pub struct AnimationElement<E> {
    id: ElementId,
    element: Option<E>,
    animations: SmallVec<[Animation; 1]>,
    frame_interval: Option<Duration>,
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
    /// Update every animation in this element in steps separated by `interval`.
    ///
    /// Cadenced animations use a timer rather than waking at the display's refresh rate, which is
    /// useful when their output only changes at discrete intervals.
    pub fn with_frame_interval(mut self, interval: Duration) -> Self {
        self.frame_interval = Some(interval.max(Duration::from_nanos(1)));
        self
    }

    /// Request redraws for this animation element at most `frames_per_second` times per second.
    ///
    /// The element can still be rendered more often when another change invalidates its view.
    /// Its progress remains quantized to the configured cadence in those additional renders.
    pub fn with_max_fps(self, frames_per_second: u32) -> Self {
        let frames_per_second = frames_per_second.max(1);
        self.with_frame_interval(Duration::from_secs_f64(1.0 / frames_per_second as f64))
    }

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
    scheduled_frame: Option<ScheduledAnimationFrame>,
}

struct ScheduledAnimationFrame {
    deadline: Instant,
    interval: Duration,
    completion_deadline: Option<Instant>,
    _registration: Rc<AnimationFrameRegistration>,
}

struct AnimationFrameRegistration;

#[derive(Default)]
pub(crate) struct AnimationFrameScheduler {
    pending: Vec<PendingAnimationFrame>,
    scheduled_wakeup: Option<ScheduledAnimationWakeup>,
    last_wakeup: Option<(Instant, Duration)>,
}

struct PendingAnimationFrame {
    deadline: Instant,
    interval: Duration,
    completion_deadline: Option<Instant>,
    current_view: EntityId,
    registration: Weak<AnimationFrameRegistration>,
}

struct ScheduledAnimationWakeup {
    deadline: Instant,
    _task: Task<()>,
}

fn duration_from_nanos(nanos: u128) -> Duration {
    const NANOS_PER_SECOND: u128 = 1_000_000_000;
    Duration::new(
        (nanos / NANOS_PER_SECOND) as u64,
        (nanos % NANOS_PER_SECOND) as u32,
    )
}

fn floor_duration_to_interval(elapsed: Duration, interval: Duration) -> Duration {
    let interval_nanos = interval.as_nanos();
    duration_from_nanos(elapsed.as_nanos() - elapsed.as_nanos() % interval_nanos)
}

fn delay_until_next_interval(elapsed: Duration, interval: Duration) -> Duration {
    let interval_nanos = interval.as_nanos();
    let remainder = elapsed.as_nanos() % interval_nanos;
    if remainder == 0 {
        interval
    } else {
        duration_from_nanos(interval_nanos - remainder)
    }
}

impl Window {
    fn schedule_cadenced_animation_frame(
        &mut self,
        deadline: Instant,
        interval: Duration,
        completion_deadline: Option<Instant>,
        current_view: EntityId,
        registration: Weak<AnimationFrameRegistration>,
        cx: &mut App,
    ) {
        self.animation_frame_scheduler
            .pending
            .push(PendingAnimationFrame {
                deadline,
                interval,
                completion_deadline,
                current_view,
                registration,
            });
        self.schedule_next_cadenced_animation_wakeup(cx);
    }

    fn schedule_next_cadenced_animation_wakeup(&mut self, cx: &mut App) {
        self.animation_frame_scheduler
            .pending
            .retain(|frame| frame.registration.strong_count() > 0);

        let Some(earliest_deadline) = self
            .animation_frame_scheduler
            .pending
            .iter()
            .map(|frame| frame.deadline)
            .min()
        else {
            self.animation_frame_scheduler.scheduled_wakeup.take();
            return;
        };
        let Some(minimum_interval) = self
            .animation_frame_scheduler
            .pending
            .iter()
            .map(|frame| frame.interval)
            .min()
        else {
            return;
        };
        let deadline = if let Some((last_wakeup, last_interval)) =
            self.animation_frame_scheduler.last_wakeup
        {
            // Phase-offset animations must share the fastest active cadence or their
            // independent deadlines can combine back into a display-rate redraw loop. Use
            // the faster interval on cadence transitions so ending a fast animation cannot
            // postpone a slower animation by nearly the slower animation's full interval.
            earliest_deadline.max(last_wakeup + minimum_interval.min(last_interval))
        } else {
            earliest_deadline
        };
        // Cadence coalescing may delay decorative steps, but a one-shot animation's final
        // value must still be rendered at its declared duration.
        let deadline = self
            .animation_frame_scheduler
            .pending
            .iter()
            .filter_map(|frame| frame.completion_deadline)
            .min()
            .map_or(deadline, |hard_deadline| deadline.min(hard_deadline));

        if self
            .animation_frame_scheduler
            .scheduled_wakeup
            .as_ref()
            .is_some_and(|wakeup| wakeup.deadline == deadline)
        {
            return;
        }

        self.animation_frame_scheduler.scheduled_wakeup.take();
        let delay = deadline.saturating_duration_since(cx.background_executor().now());
        let task = self.spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| window.fire_cadenced_animation_frames(cx))
                .log_err();
        });
        self.animation_frame_scheduler.scheduled_wakeup = Some(ScheduledAnimationWakeup {
            deadline,
            _task: task,
        });
    }

    fn fire_cadenced_animation_frames(&mut self, cx: &mut App) {
        self.animation_frame_scheduler.scheduled_wakeup.take();
        let now = cx.background_executor().now();
        let mut current_views = Vec::new();
        let mut minimum_fired_interval = None;
        self.animation_frame_scheduler.pending.retain(|frame| {
            if frame.registration.strong_count() == 0 {
                return false;
            }
            let completion_is_due = frame
                .completion_deadline
                .is_some_and(|deadline| deadline <= now);
            if frame.deadline <= now || completion_is_due {
                minimum_fired_interval = Some(
                    minimum_fired_interval.map_or(frame.interval, |interval: Duration| {
                        interval.min(frame.interval)
                    }),
                );
                if !current_views.contains(&frame.current_view) {
                    current_views.push(frame.current_view);
                }
                false
            } else {
                true
            }
        });

        if !current_views.is_empty() {
            self.animation_frame_scheduler.last_wakeup =
                minimum_fired_interval.map(|interval| (now, interval));
            for current_view in current_views {
                cx.notify(current_view);
            }
        }
        self.schedule_next_cadenced_animation_wakeup(cx);
    }
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
            let now = cx.background_executor().now();
            let mut state = state.unwrap_or_else(|| AnimationState {
                start: now,
                animation_ix: 0,
                scheduled_frame: None,
            });
            if state
                .scheduled_frame
                .as_ref()
                .is_some_and(|frame| frame.deadline <= now)
            {
                state.scheduled_frame.take();
            }

            let (animation_ix, delta, done, animation_advanced) = if cx.reduce_motion() {
                let animation_ix = self.animations.len() - 1;
                let delta = if self.animations[animation_ix].oneshot {
                    1.0
                } else {
                    0.0
                };
                (animation_ix, delta, true, false)
            } else {
                let animation_ix = state.animation_ix;
                let animation = &self.animations[animation_ix];
                let elapsed = now.saturating_duration_since(state.start);
                let frame_interval = self.frame_interval;
                let animation_elapsed = frame_interval
                    .map(|interval| floor_duration_to_interval(elapsed, interval))
                    .unwrap_or(elapsed);

                let mut delta = animation_elapsed.as_secs_f32() / animation.duration.as_secs_f32();

                let mut done = false;
                let mut animation_advanced = false;
                if elapsed >= animation.duration {
                    if animation.oneshot {
                        if animation_ix >= self.animations.len() - 1 {
                            done = true;
                        } else {
                            state.start = now;
                            state.animation_ix += 1;
                            animation_advanced = true;
                        }
                        delta = 1.0;
                    }
                }
                if !animation.oneshot && delta >= 1.0 {
                    delta %= 1.0;
                }
                (animation_ix, delta, done, animation_advanced)
            };
            let delta = (self.animations[animation_ix].easing)(delta);

            debug_assert!(
                (0.0..=1.0).contains(&delta),
                "delta should always be between 0 and 1"
            );

            let element = self.element.take().expect("should only be called once");
            let mut element = (self.animator)(element, animation_ix, delta).into_any_element();

            if done {
                state.scheduled_frame.take();
            } else if animation_advanced {
                state.scheduled_frame.take();
                window.request_animation_frame();
            } else if let Some(frame_interval) = self.frame_interval {
                let animation = &self.animations[animation_ix];
                let elapsed = now.saturating_duration_since(state.start);
                let next_interval_delay = delay_until_next_interval(elapsed, frame_interval);
                let completion_deadline = if animation.oneshot {
                    Some(state.start + animation.duration)
                } else {
                    None
                };
                let delay = completion_deadline
                    .map(|deadline| deadline.saturating_duration_since(now))
                    .map_or(next_interval_delay, |remaining| {
                        next_interval_delay.min(remaining)
                    });

                if delay.is_zero() {
                    state.scheduled_frame.take();
                    window.request_animation_frame();
                } else {
                    let deadline = now + delay;
                    let already_scheduled = state.scheduled_frame.as_ref().is_some_and(|frame| {
                        frame.deadline == deadline
                            && frame.interval == frame_interval
                            && frame.completion_deadline == completion_deadline
                    });
                    if !already_scheduled {
                        state.scheduled_frame.take();
                        let registration = Rc::new(AnimationFrameRegistration);
                        window.schedule_cadenced_animation_frame(
                            deadline,
                            frame_interval,
                            completion_deadline,
                            window.current_view(),
                            Rc::downgrade(&registration),
                            cx,
                        );
                        state.scheduled_frame = Some(ScheduledAnimationFrame {
                            deadline,
                            interval: frame_interval,
                            completion_deadline,
                            _registration: registration,
                        });
                    }
                }
            } else {
                state.scheduled_frame.take();
                window.request_animation_frame();
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
    use std::{cell::Cell, cell::RefCell, rc::Rc, time::Duration};

    use crate::{
        Animation, Context, InteractiveElement, Render, TestAppContext, WindowHandle, div,
        prelude::*, px, size,
    };

    use super::*;

    struct AnimationTestView {
        rendered_deltas: Rc<RefCell<Vec<f32>>>,
        render_count: Rc<Cell<usize>>,
        frame_interval: Option<Duration>,
        show_animation: bool,
    }

    struct MultipleAnimationsTestView {
        render_count: Rc<Cell<usize>>,
        show_first: bool,
        show_second: bool,
        first_frame_interval: Duration,
        second_frame_interval: Duration,
    }

    impl Render for MultipleAnimationsTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.render_count.set(self.render_count.get() + 1);
            let animation = || Animation::new(Duration::from_secs(1)).repeat();

            div()
                .when(self.show_first, |this| {
                    this.child(
                        div()
                            .with_animation("first", animation(), |this, _| this)
                            .with_frame_interval(self.first_frame_interval),
                    )
                })
                .when(self.show_second, |this| {
                    this.child(
                        div()
                            .with_animation("second", animation(), |this, _| this)
                            .with_frame_interval(self.second_frame_interval),
                    )
                })
        }
    }

    struct OneShotWithRepeatingAnimationTestView {
        rendered_one_shot_values: Rc<RefCell<Vec<f32>>>,
    }

    impl Render for OneShotWithRepeatingAnimationTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let rendered_one_shot_values = self.rendered_one_shot_values.clone();
            div()
                .child(
                    div()
                        .with_animation(
                            "one-shot",
                            Animation::new(Duration::from_millis(1050)),
                            move |this, delta| {
                                rendered_one_shot_values.borrow_mut().push(delta);
                                this
                            },
                        )
                        .with_frame_interval(Duration::from_secs(1)),
                )
                .child(
                    div()
                        .with_animation(
                            "repeating",
                            Animation::new(Duration::from_secs(1)).repeat(),
                            |this, _| this,
                        )
                        .with_frame_interval(Duration::from_millis(100)),
                )
        }
    }

    impl Render for AnimationTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.render_count.set(self.render_count.get() + 1);
            let rendered_deltas = self.rendered_deltas.clone();
            let animation = Animation::new(Duration::from_secs(1)).repeat();
            let animation =
                div().with_animation("repeating-animation", animation, move |this, delta| {
                    rendered_deltas.borrow_mut().push(delta);
                    this
                });
            let animation = if let Some(frame_interval) = self.frame_interval {
                animation.with_frame_interval(frame_interval)
            } else {
                animation
            };

            div()
                .size_full()
                .when(self.show_animation, |this| this.child(animation))
        }
    }

    fn open_test_window(
        cx: &mut TestAppContext,
        frame_interval: Option<Duration>,
    ) -> (
        Rc<RefCell<Vec<f32>>>,
        Rc<Cell<usize>>,
        WindowHandle<AnimationTestView>,
    ) {
        let rendered_deltas = Rc::new(RefCell::new(Vec::new()));
        let render_count = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(100.), px(100.)), {
            let rendered_deltas = rendered_deltas.clone();
            let render_count = render_count.clone();
            move |_, _| AnimationTestView {
                rendered_deltas,
                render_count,
                frame_interval,
                show_animation: true,
            }
        });
        cx.run_until_parked();
        (rendered_deltas, render_count, window)
    }

    fn simulate_next_frame<V: 'static>(window: &WindowHandle<V>, cx: &mut TestAppContext) -> usize {
        let callback_count = window
            .update(cx, |_, window, cx| window.simulate_next_frame(cx))
            .unwrap();
        cx.run_until_parked();
        callback_count
    }

    fn assert_last_delta(rendered_deltas: &Rc<RefCell<Vec<f32>>>, expected: f32) {
        let actual = rendered_deltas.borrow().last().copied().unwrap();
        assert!((actual - expected).abs() < f32::EPSILON);
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
        let (rendered_deltas, _, window) = open_test_window(cx, None);

        assert_eq!(rendered_deltas.borrow().len(), 1);

        for expected_frames in 2..=3 {
            assert_eq!(simulate_next_frame(&window, cx), 1);
            assert_eq!(rendered_deltas.borrow().len(), expected_frames);
        }
    }

    #[gpui::test]
    fn test_reduce_motion_renders_single_static_frame(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_reduce_motion(true));
        let (rendered_deltas, _, window) = open_test_window(cx, Some(Duration::from_millis(100)));

        assert_eq!(*rendered_deltas.borrow(), vec![0.0]);

        assert_eq!(simulate_next_frame(&window, cx), 0);
        assert_eq!(*rendered_deltas.borrow(), vec![0.0]);
    }

    #[gpui::test]
    fn test_cadenced_animation_only_renders_at_its_deadline(cx: &mut TestAppContext) {
        let frame_interval = Duration::from_millis(100);
        let (rendered_deltas, render_count, window) = open_test_window(cx, Some(frame_interval));

        assert_eq!(rendered_deltas.borrow().len(), 1);
        assert_eq!(simulate_next_frame(&window, cx), 0);

        cx.background_executor
            .advance_clock(Duration::from_millis(50));
        cx.run_until_parked();
        assert_eq!(rendered_deltas.borrow().len(), 1);

        window.update(cx, |_, _, cx| cx.notify()).unwrap();
        cx.run_until_parked();
        assert_eq!(rendered_deltas.borrow().len(), 2);
        assert_last_delta(&rendered_deltas, 0.0);

        cx.background_executor
            .advance_clock(Duration::from_millis(49));
        cx.run_until_parked();
        assert_eq!(rendered_deltas.borrow().len(), 2);

        cx.background_executor
            .advance_clock(Duration::from_millis(1));
        cx.run_until_parked();
        assert_eq!(rendered_deltas.borrow().len(), 3);
        assert_last_delta(&rendered_deltas, 0.1);

        cx.background_executor.advance_clock(frame_interval);
        cx.run_until_parked();
        assert_eq!(rendered_deltas.borrow().len(), 4);
        assert_last_delta(&rendered_deltas, 0.2);

        window
            .update(cx, |this, _, cx| {
                this.frame_interval = Some(Duration::from_secs(1));
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();
        let render_count_after_cadence_change = render_count.get();

        cx.background_executor.advance_clock(frame_interval);
        cx.run_until_parked();
        assert_eq!(render_count.get(), render_count_after_cadence_change);

        cx.background_executor
            .advance_clock(Duration::from_millis(700));
        cx.run_until_parked();
        assert_eq!(render_count.get(), render_count_after_cadence_change + 1);

        window
            .update(cx, |this, _, cx| {
                this.show_animation = false;
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();
        let render_count_after_removal = render_count.get();

        cx.background_executor.advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(render_count.get(), render_count_after_removal);
    }

    #[gpui::test]
    fn test_window_coalesces_phase_offset_animations(cx: &mut TestAppContext) {
        let render_count = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(100.), px(100.)), {
            let render_count = render_count.clone();
            move |_, _| MultipleAnimationsTestView {
                render_count,
                show_first: true,
                show_second: false,
                first_frame_interval: Duration::from_millis(100),
                second_frame_interval: Duration::from_millis(100),
            }
        });
        cx.run_until_parked();
        assert_eq!(render_count.get(), 1);

        cx.background_executor
            .advance_clock(Duration::from_millis(50));
        window
            .update(cx, |this, _, cx| {
                this.show_second = true;
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();
        assert_eq!(render_count.get(), 2);

        cx.background_executor
            .advance_clock(Duration::from_millis(50));
        cx.run_until_parked();
        assert_eq!(render_count.get(), 3);

        cx.background_executor
            .advance_clock(Duration::from_millis(99));
        cx.run_until_parked();
        assert_eq!(render_count.get(), 3);

        cx.background_executor
            .advance_clock(Duration::from_millis(1));
        cx.run_until_parked();
        assert_eq!(render_count.get(), 4);

        window
            .update(cx, |this, _, cx| {
                this.show_first = false;
                this.show_second = false;
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();
    }

    #[gpui::test]
    fn test_ending_fast_animation_does_not_delay_slow_animation(cx: &mut TestAppContext) {
        let render_count = Rc::new(Cell::new(0));
        let window = cx.open_window(size(px(100.), px(100.)), {
            let render_count = render_count.clone();
            move |_, _| MultipleAnimationsTestView {
                render_count,
                show_first: true,
                show_second: true,
                first_frame_interval: Duration::from_millis(100),
                second_frame_interval: Duration::from_millis(250),
            }
        });
        cx.run_until_parked();
        assert_eq!(render_count.get(), 1);

        cx.background_executor
            .advance_clock(Duration::from_millis(100));
        cx.run_until_parked();
        assert_eq!(render_count.get(), 2);

        window
            .update(cx, |this, _, cx| {
                this.show_first = false;
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();
        assert_eq!(render_count.get(), 3);

        cx.background_executor
            .advance_clock(Duration::from_millis(149));
        cx.run_until_parked();
        assert_eq!(render_count.get(), 3);

        cx.background_executor
            .advance_clock(Duration::from_millis(1));
        cx.run_until_parked();
        assert_eq!(render_count.get(), 4);
    }

    #[gpui::test]
    fn test_real_one_shot_completion_survives_cadence_coalescing(cx: &mut TestAppContext) {
        let rendered_one_shot_values = Rc::new(RefCell::new(Vec::new()));
        let _window = cx.open_window(size(px(100.), px(100.)), {
            let rendered_one_shot_values = rendered_one_shot_values.clone();
            move |_, _| OneShotWithRepeatingAnimationTestView {
                rendered_one_shot_values,
            }
        });
        cx.run_until_parked();
        assert_eq!(*rendered_one_shot_values.borrow(), vec![0.0]);

        cx.background_executor.advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert!(
            rendered_one_shot_values
                .borrow()
                .last()
                .is_some_and(|delta| *delta < 1.0)
        );

        cx.background_executor
            .advance_clock(Duration::from_millis(49));
        cx.run_until_parked();
        assert!(
            rendered_one_shot_values
                .borrow()
                .last()
                .is_some_and(|delta| *delta < 1.0)
        );

        cx.background_executor
            .advance_clock(Duration::from_millis(1));
        cx.run_until_parked();
        assert_eq!(rendered_one_shot_values.borrow().last(), Some(&1.0));
    }
}
