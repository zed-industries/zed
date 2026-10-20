use super::*;
use gpui::{
    InputEvent, Modifiers, TestAppContext, TouchEvent, TouchId, VisualTestContext, div, point,
};

#[gpui::test]
fn touch_thumb_scrubs_and_releases_outside_viewport(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, true);
    let resting_width = painted_thumb_width(cx);
    let start = point(px(165.), px(10.));
    touch(cx, TouchPhase::Started, start);
    assert_eq!(
        painted_thumb_width(cx),
        resting_width + TOUCH_HELD_EXPANSION
    );
    view.read_with(cx, |view, cx| {
        let state = view.scrollbar.read(cx);
        assert!(matches!(state.thumb_state, ThumbState::TouchDragging(..)));
        assert_eq!(view.scroll.offset(), Point::default());
    });
    touch(cx, TouchPhase::Moved, point(px(165.), px(100.)));
    view.read_with(cx, |view, _| assert!(view.scroll.offset().y < px(-500.)));
    touch(cx, TouchPhase::Ended, point(px(300.), px(300.)));
    assert_eq!(painted_thumb_width(cx), resting_width);
    view.read_with(cx, |view, cx| {
        assert_eq!(view.scrollbar.read(cx).thumb_state, ThumbState::Inactive);
    });

    // A cancellation on the visible thumb must unwind exactly like a release.
    let start = view.read_with(cx, |view, cx| {
        view.scrollbar
            .read(cx)
            .last_prepaint_state
            .as_ref()
            .unwrap()
            .thumbs[0]
            .thumb_bounds
            .center()
    });
    touch(cx, TouchPhase::Started, start);
    view.read_with(cx, |view, cx| {
        assert!(view.scrollbar.read(cx).is_dragging())
    });
    touch(cx, TouchPhase::Cancelled, start);
    assert_eq!(painted_thumb_width(cx), resting_width);
    view.read_with(cx, |view, cx| {
        assert_eq!(view.scrollbar.read(cx).thumb_state, ThumbState::Inactive);
    });
}

#[gpui::test]
fn non_touch_input_does_not_claim_expanded_target(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, false);
    touch(cx, TouchPhase::Started, point(px(165.), px(10.)));
    view.read_with(cx, |view, cx| {
        assert_eq!(view.scrollbar.read(cx).thumb_state, ThumbState::Inactive);
        assert!(
            view.scrollbar
                .read(cx)
                .last_prepaint_state
                .as_ref()
                .unwrap()
                .thumbs[0]
                .touch_hitbox
                .is_none()
        );
    });
    touch(cx, TouchPhase::Cancelled, point(px(165.), px(10.)));

    let start = point(px(193.), px(10.));
    let width = painted_thumb_width(cx);
    cx.simulate_mouse_move(start, None, Modifiers::default());
    cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
    cx.simulate_mouse_move(
        point(px(193.), px(100.)),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    view.read_with(cx, |view, _| assert!(view.scroll.offset().y < px(-500.)));
    assert_eq!(painted_thumb_width(cx), width);
    cx.simulate_mouse_up(
        point(px(193.), px(100.)),
        MouseButton::Left,
        Modifiers::default(),
    );
}

#[test]
fn touch_geometry_preserves_scrub_axis_and_stays_inside_parent() {
    let parent = Bounds::new(point(px(10.), px(20.)), size(px(200.), px(200.)));
    for (axis, thumb) in [
        (
            ScrollbarAxis::Vertical,
            Bounds::new(point(px(200.), px(20.)), size(px(6.), px(25.))),
        ),
        (
            ScrollbarAxis::Horizontal,
            Bounds::new(point(px(10.), px(210.)), size(px(25.), px(6.))),
        ),
    ] {
        let target = touch_thumb_bounds(thumb, parent, axis);
        assert_eq!(target.size, size(px(44.), px(44.)));
        assert!(parent.contains(&target.origin));
        assert!(target.right() <= parent.right());
        assert!(target.bottom() <= parent.bottom());
        let held = held_thumb_bounds(thumb, axis);
        assert_eq!(held.origin.along(axis), thumb.origin.along(axis));
        assert_eq!(held.size.along(axis), thumb.size.along(axis));
        assert_eq!(held.size.along(axis.invert()), px(10.));
    }
}

fn setup(cx: &mut TestAppContext, touch: bool) -> (Entity<TestView>, &mut VisualTestContext) {
    cx.update(|cx| {
        let settings = settings::SettingsStore::test(cx);
        cx.set_global(settings);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        crate::responsive::set_touch_input(touch, cx);
    });
    cx.add_window_view(|_, cx| {
        let scroll = ScrollHandle::new();
        let parent = cx.entity_id();
        let scrollbar = cx.new(|cx| {
            ScrollbarState::new_from_config(
                Scrollbars::always_visible(ScrollAxes::Vertical).tracked_scroll_handle(&scroll),
                Corners::default(),
                parent,
                cx,
            )
        });
        TestView { scroll, scrollbar }
    })
}

fn touch(cx: &mut VisualTestContext, phase: TouchPhase, position: Point<Pixels>) {
    cx.update(|window, cx| {
        window.dispatch_event(
            TouchEvent {
                id: TouchId(1),
                phase,
                position,
                predicted_position: None,
                force: None,
            }
            .to_platform_input(),
            cx,
        );
    });
    cx.run_until_parked();
}

fn painted_thumb_width(cx: &mut VisualTestContext) -> Pixels {
    cx.update(|window, _| {
        let quads = window.painted_quads();
        assert_eq!(quads.len(), 1);
        px(quads[0].bounds.size.width.as_f32() / window.scale_factor())
    })
}

struct TestView {
    scroll: ScrollHandle,
    scrollbar: Entity<ScrollbarState>,
}

impl Render for TestView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("scroll-view")
            .relative()
            .w(px(200.))
            .h(px(200.))
            .child(
                div()
                    .id("content")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll)
                    .child(div().h(px(2000.)).w(px(100.))),
            )
            .child(self.scrollbar.clone())
    }
}
