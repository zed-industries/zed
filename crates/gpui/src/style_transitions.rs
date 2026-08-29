use std::time::Instant;

use crate::{
    AbsoluteLength, Bounds, DefiniteLength, Fill, Hsla, Interpolate, Length, Motion, Pixels,
};

#[derive(Clone, Copy)]
pub(crate) struct StyleTransitionContext {
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) rem_size: Pixels,
}

impl StyleTransitionContext {
    pub(crate) fn new(bounds: Bounds<Pixels>, rem_size: Pixels) -> Self {
        Self { bounds, rem_size }
    }
}

pub(crate) struct StyleTransitionPropertyState<T> {
    start: Option<T>,
    target: Option<T>,
    started_at: Option<Instant>,
}

struct SizeTransitionState<T> {
    width: Option<StyleTransitionPropertyState<T>>,
    height: Option<StyleTransitionPropertyState<T>>,
}

impl<T> Default for SizeTransitionState<T> {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
        }
    }
}

struct EdgesTransitionState<T> {
    top: Option<StyleTransitionPropertyState<T>>,
    right: Option<StyleTransitionPropertyState<T>>,
    bottom: Option<StyleTransitionPropertyState<T>>,
    left: Option<StyleTransitionPropertyState<T>>,
}

impl<T> Default for EdgesTransitionState<T> {
    fn default() -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
}

struct CornersTransitionState<T> {
    top_left: Option<StyleTransitionPropertyState<T>>,
    top_right: Option<StyleTransitionPropertyState<T>>,
    bottom_right: Option<StyleTransitionPropertyState<T>>,
    bottom_left: Option<StyleTransitionPropertyState<T>>,
}

impl<T> Default for CornersTransitionState<T> {
    fn default() -> Self {
        Self {
            top_left: None,
            top_right: None,
            bottom_right: None,
            bottom_left: None,
        }
    }
}

#[derive(Default)]
struct TextStyleTransitionState {
    color: Option<StyleTransitionPropertyState<Hsla>>,
    background_color: Option<StyleTransitionPropertyState<Hsla>>,
    font_size: Option<StyleTransitionPropertyState<AbsoluteLength>>,
    line_height: Option<StyleTransitionPropertyState<DefiniteLength>>,
    line_clamp: Option<StyleTransitionPropertyState<usize>>,
}

#[derive(Default)]
pub(crate) struct StyleTransitionState {
    inset: EdgesTransitionState<Length>,
    size: SizeTransitionState<Length>,
    min_size: SizeTransitionState<Length>,
    max_size: SizeTransitionState<Length>,
    margin: EdgesTransitionState<Length>,
    padding: EdgesTransitionState<DefiniteLength>,
    border_widths: EdgesTransitionState<AbsoluteLength>,
    gap: SizeTransitionState<DefiniteLength>,
    corner_radii: CornersTransitionState<AbsoluteLength>,
    scrollbar_width: Option<StyleTransitionPropertyState<AbsoluteLength>>,
    aspect_ratio: Option<StyleTransitionPropertyState<f32>>,
    flex_basis: Option<StyleTransitionPropertyState<Length>>,
    flex_grow: Option<StyleTransitionPropertyState<f32>>,
    flex_shrink: Option<StyleTransitionPropertyState<f32>>,
    background: Option<StyleTransitionPropertyState<Fill>>,
    border_color: Option<StyleTransitionPropertyState<Hsla>>,
    text: TextStyleTransitionState,
    opacity: Option<StyleTransitionPropertyState<f32>>,
}

fn evaluate<T>(
    state: &mut Option<StyleTransitionPropertyState<T>>,
    target: Option<T>,
    motion: &Motion,
    now: Instant,
    reduce_motion: bool,
) -> (bool, Option<T>)
where
    T: Interpolate + Clone + PartialEq,
{
    state
        .get_or_insert_with(|| StyleTransitionPropertyState::new(target.clone()))
        .evaluate(target, motion, now, reduce_motion)
}

fn apply_required<T>(
    state: &mut Option<StyleTransitionPropertyState<T>>,
    value: &mut T,
    motion: Option<&Motion>,
    now: Instant,
    reduce_motion: bool,
) -> bool
where
    T: Interpolate + Clone + PartialEq,
{
    let target = value.clone();
    apply_required_target(state, value, Some(target), motion, now, reduce_motion)
}

fn apply_required_target<T>(
    state: &mut Option<StyleTransitionPropertyState<T>>,
    value: &mut T,
    target: Option<T>,
    motion: Option<&Motion>,
    now: Instant,
    reduce_motion: bool,
) -> bool
where
    T: Interpolate + Clone + PartialEq,
{
    let Some(motion) = motion else {
        *state = None;
        return false;
    };
    let Some(target) = target else {
        return false;
    };

    let (in_progress, evaluated_value) = evaluate(state, Some(target), motion, now, reduce_motion);
    if let Some(evaluated_value) = evaluated_value {
        *value = evaluated_value;
    }
    in_progress
}

fn apply_optional<T>(
    state: &mut Option<StyleTransitionPropertyState<T>>,
    value: &mut Option<T>,
    motion: Option<&Motion>,
    now: Instant,
    reduce_motion: bool,
) -> bool
where
    T: Interpolate + Clone + PartialEq,
{
    let Some(motion) = motion else {
        *state = None;
        return false;
    };

    let target = value.clone();
    let (in_progress, evaluated_value) = evaluate(state, target, motion, now, reduce_motion);
    *value = evaluated_value;
    in_progress
}

fn apply_optional_with_default<T>(
    state: &mut Option<StyleTransitionPropertyState<T>>,
    value: &mut Option<T>,
    default: T,
    motion: Option<&Motion>,
    now: Instant,
    reduce_motion: bool,
) -> bool
where
    T: Interpolate + Clone + PartialEq,
{
    let Some(motion) = motion else {
        *state = None;
        return false;
    };

    let target = value.clone().unwrap_or(default);
    let (in_progress, evaluated_value) = evaluate(state, Some(target), motion, now, reduce_motion);
    *value = evaluated_value;
    in_progress
}

impl<T> StyleTransitionPropertyState<T> {
    fn new(initial_target: Option<T>) -> Self {
        Self {
            start: None,
            target: initial_target,
            started_at: None,
        }
    }
}

impl<T> StyleTransitionPropertyState<T>
where
    T: Interpolate + Clone + PartialEq,
{
    fn evaluate(
        &mut self,
        target: Option<T>,
        motion: &Motion,
        now: Instant,
        reduce_motion: bool,
    ) -> (bool, Option<T>) {
        if reduce_motion {
            self.jump_to(target.clone());
            return (false, target);
        }

        if self.target != target {
            let current = self.value_at(motion, now).1;
            self.start = current;
            self.target = target;

            if self.start.is_none() || self.target.is_none() || self.start == self.target {
                self.start = self.target.clone();
                self.started_at = None;
            } else {
                self.started_at = Some(now);
            }
        }

        self.value_at(motion, now)
    }

    fn value_at(&mut self, motion: &Motion, now: Instant) -> (bool, Option<T>) {
        let Some(started_at) = self.started_at else {
            return (false, self.target.clone());
        };

        let sample = motion.sample(now.saturating_duration_since(started_at));
        let value = match (self.start.clone(), self.target.clone()) {
            (Some(start), Some(target)) => Some(T::interpolate(start, target, sample.phase)),
            _ => self.target.clone(),
        };

        if !sample.is_active {
            self.start = self.target.clone();
            self.started_at = None;
        }

        (sample.is_active, value)
    }

    fn jump_to(&mut self, target: Option<T>) {
        self.start = target.clone();
        self.target = target;
        self.started_at = None;
    }
}

gpui_macros::style_transitions!();

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::time::Duration;

    use super::*;
    use crate::{
        AbsoluteLength, AnyWindowHandle, Bounds, Corners, DefiniteLength, InputEvent as _, Length,
        MouseButton, MouseDownEvent, MouseUpEvent, Pixels, Style, TestAppContext, Window, canvas,
        div, point, prelude::*, px, rems, size,
    };

    fn length(value: f32) -> Length {
        Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(value))))
    }

    fn corners(value: f32) -> Corners<AbsoluteLength> {
        let value = AbsoluteLength::Pixels(px(value));
        Corners {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }

    fn size_transition_after(
        transitions: StyleTransitions,
        elapsed: Duration,
    ) -> (bool, Style, StyleTransitionState) {
        let started_at = Instant::now();
        let mut state = StyleTransitionState::default();
        let mut style = Style {
            size: size(length(10.0), length(10.0)),
            ..Style::default()
        };

        assert!(!transitions.apply(&mut style, &mut state, None, started_at, false));

        style.size = size(length(20.0), length(20.0));
        assert!(transitions.apply(&mut style, &mut state, None, started_at, false));

        style.size = size(length(20.0), length(20.0));
        let in_progress =
            transitions.apply(&mut style, &mut state, None, started_at + elapsed, false);

        (in_progress, style, state)
    }

    #[test]
    fn property_transitions_follow_motion_and_retarget() {
        let motion = Motion::new(Duration::from_secs(1));
        let started_at = Instant::now();
        let mut state = StyleTransitionPropertyState::new(Some(0.0_f32));

        assert_eq!(
            state.evaluate(Some(10.0), &motion, started_at, false),
            (true, Some(0.0))
        );
        assert_eq!(
            state.evaluate(
                Some(10.0),
                &motion,
                started_at + Duration::from_millis(500),
                false,
            ),
            (true, Some(5.0))
        );
        assert_eq!(
            state.evaluate(
                Some(20.0),
                &motion,
                started_at + Duration::from_millis(500),
                false,
            ),
            (true, Some(5.0))
        );
        assert_eq!(
            state.evaluate(
                Some(20.0),
                &motion,
                started_at + Duration::from_millis(1_000),
                false,
            ),
            (true, Some(12.5))
        );
        assert_eq!(
            state.evaluate(
                Some(20.0),
                &motion,
                started_at + Duration::from_millis(1_500),
                false,
            ),
            (false, Some(20.0))
        );

        let mut optional_state = StyleTransitionPropertyState::new(None::<f32>);
        assert_eq!(
            optional_state.evaluate(Some(10.0), &motion, started_at, false),
            (false, Some(10.0))
        );

        let mut immediate_state = StyleTransitionPropertyState::new(Some(0.0_f32));
        assert_eq!(
            immediate_state.evaluate(Some(10.0), &Motion::new(Duration::ZERO), started_at, false,),
            (false, Some(10.0))
        );
        assert_eq!(
            immediate_state.evaluate(Some(20.0), &motion, started_at, true),
            (false, Some(20.0))
        );
    }

    #[test]
    fn generated_transitions_apply_property_groups_and_builder_precedence() {
        let (in_progress, style, _) = size_transition_after(
            StyleTransitions::new().w(Duration::from_secs(1)),
            Duration::from_millis(500),
        );
        assert!(in_progress);
        assert_eq!(style.size, size(length(15.0), length(20.0)));

        let (in_progress, style, state) = size_transition_after(
            StyleTransitions::new()
                .size(Duration::from_secs(1))
                .w(Duration::from_secs(2)),
            Duration::from_secs(1),
        );
        assert!(in_progress);
        assert_eq!(style.size, size(length(15.0), length(20.0)));
        assert!(state.size.width.is_some());
        assert!(state.size.height.is_some());

        let (in_progress, style, _) = size_transition_after(
            StyleTransitions::new()
                .w(Duration::from_secs(2))
                .size(Duration::from_secs(1)),
            Duration::from_secs(1),
        );
        assert!(!in_progress);
        assert_eq!(style.size, size(length(20.0), length(20.0)));

        let started_at = Instant::now();
        let mut state = StyleTransitionState::default();
        let transitions = StyleTransitions::new()
            .opacity(Duration::from_secs(1))
            .rounded(Duration::from_secs(1));
        let context = StyleTransitionContext::new(
            Bounds {
                origin: point(px(0.0), px(0.0)),
                size: size(px(100.0), px(60.0)),
            },
            px(16.0),
        );
        let mut style = Style {
            corner_radii: corners(30.0),
            ..Style::default()
        };

        assert!(!transitions.apply(&mut style, &mut state, Some(context), started_at, false,));
        assert_eq!(style.opacity, Some(1.0));
        assert_eq!(style.corner_radii, corners(30.0));

        style.opacity = Some(0.5);
        style.corner_radii = corners(0.0);
        assert!(transitions.apply(&mut style, &mut state, Some(context), started_at, false,));

        style.opacity = Some(0.5);
        style.corner_radii = corners(0.0);
        assert!(transitions.apply(
            &mut style,
            &mut state,
            Some(context),
            started_at + Duration::from_millis(500),
            false,
        ));
        assert_eq!(style.opacity, Some(0.75));
        assert_eq!(style.corner_radii, corners(15.0));

        let pixels = AbsoluteLength::Pixels(px(10.0));
        let rems = AbsoluteLength::Rems(rems(2.0));
        assert_eq!(AbsoluteLength::interpolate(pixels, rems, 0.5), pixels);
        assert_eq!(AbsoluteLength::interpolate(pixels, rems, 1.0), rems);
    }

    struct StyleTransitionTestView {
        transitions_enabled: bool,
        base_width: Pixels,
        presented_width: Rc<Cell<Pixels>>,
    }

    impl Render for StyleTransitionTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let presented_width = self.presented_width.clone();

            div().size_full().child(
                div()
                    .id("transition-target")
                    .h(px(50.0))
                    .w(self.base_width)
                    .when(self.transitions_enabled, |element| {
                        element.transitions(|transitions| transitions.w(Duration::from_millis(200)))
                    })
                    .hover(|style| style.w(px(200.0)))
                    .active(|style| style.w(px(50.0)))
                    .child(canvas(
                        move |bounds, _, _| presented_width.set(bounds.size.width),
                        |_, _, _, _| {},
                    )),
            )
        }
    }

    #[gpui::test]
    fn style_transitions_follow_interaction_and_reset_persistent_state(cx: &mut TestAppContext) {
        let presented_width = Rc::new(Cell::new(px(0.0)));
        let window = cx.add_window({
            let presented_width = presented_width.clone();
            move |_, _| StyleTransitionTestView {
                transitions_enabled: true,
                base_width: px(100.0),
                presented_width,
            }
        });
        let any_window = AnyWindowHandle::from(window);
        let mouse_position = point(px(10.0), px(10.0));

        let draw = |cx: &mut TestAppContext| {
            if let Err(error) =
                cx.update_window(any_window, |_, window, cx| window.draw(cx).clear(cx))
            {
                panic!("failed to draw transition test window: {error:#}");
            }
        };
        let assert_width_near = |expected: f32| {
            assert!(
                (presented_width.get().0 - expected).abs() < 0.01,
                "expected width near {expected}, got {}",
                presented_width.get().0,
            );
        };

        draw(cx);
        assert_width_near(100.0);

        if let Err(error) = cx.update_window(any_window, |_, window, cx| {
            window.simulate_mouse_move(mouse_position, cx);
            window.draw(cx).clear(cx);
        }) {
            panic!("failed to move the mouse into the transition target: {error:#}");
        }
        assert_width_near(100.0);

        cx.executor().advance_clock(Duration::from_millis(100));
        draw(cx);
        assert_width_near(150.0);

        if let Err(error) = cx.update_window(any_window, |_, window, cx| {
            window.dispatch_event(
                MouseDownEvent {
                    position: mouse_position,
                    button: MouseButton::Left,
                    modifiers: Default::default(),
                    click_count: 1,
                    first_mouse: false,
                }
                .to_platform_input(),
                cx,
            );
            window.draw(cx).clear(cx);
        }) {
            panic!("failed to press the transition target: {error:#}");
        }
        assert_width_near(150.0);

        cx.executor().advance_clock(Duration::from_millis(100));
        draw(cx);
        assert_width_near(100.0);
        cx.executor().advance_clock(Duration::from_millis(100));
        draw(cx);
        assert_width_near(50.0);

        if let Err(error) = cx.update_window(any_window, |_, window, cx| {
            window.dispatch_event(
                MouseUpEvent {
                    position: mouse_position,
                    button: MouseButton::Left,
                    modifiers: Default::default(),
                    click_count: 1,
                }
                .to_platform_input(),
                cx,
            );
            window.simulate_mouse_move(point(px(300.0), px(300.0)), cx);
        }) {
            panic!("failed to release the transition target: {error:#}");
        }
        if let Err(error) = window.update(cx, |view, _, cx| {
            view.transitions_enabled = false;
            view.base_width = px(240.0);
            cx.notify();
        }) {
            panic!("failed to disable transitions: {error:#}");
        }
        draw(cx);
        assert_width_near(240.0);

        if let Err(error) = window.update(cx, |view, _, cx| {
            view.transitions_enabled = true;
            cx.notify();
        }) {
            panic!("failed to re-enable transitions: {error:#}");
        }
        draw(cx);
        assert_width_near(240.0);
    }
}
