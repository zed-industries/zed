use crate::{ScrollAxes, Scrollbars, WithScrollbar};
use gpui::{
    Bounds, Context, Entity, InputEvent, Modifiers, MouseButton, Pixels, Point, ScrollHandle,
    TestAppContext, TouchEvent, TouchId, TouchPhase, VisualTestContext, Window, div, point,
    prelude::*, px,
};

#[gpui::test]
fn touch_scrubbing_tracks_the_finger_and_releases(cx: &mut TestAppContext) {
    for tracked in [false, true] {
        let (_, cx) = setup(cx, tracked, true);
        cx.run_until_parked();
        let resting = thumb(cx);
        let travel = px(200.) - resting.origin.y * 2. - resting.size.height;
        let start = point(px(165.), resting.center().y);
        touch(cx, TouchPhase::Started, start);
        assert_eq!(content_top(cx), px(0.));
        assert_eq!(thumb(cx).size.width, resting.size.width + px(4.));

        // Two steps catch coordinates accidentally moving with the scrolled content.
        for fraction in [0.25, 0.5] {
            touch(
                cx,
                TouchPhase::Moved,
                start + point(px(0.), travel * fraction),
            );
            assert!((content_top(cx) + px(1800.) * fraction).abs() < px(1.));
            assert!((thumb(cx).origin.y - resting.origin.y - travel * fraction).abs() < px(1.));
        }
        touch(cx, TouchPhase::Ended, point(px(300.), px(300.)));
        assert_eq!(thumb(cx).size.width, resting.size.width);
        assert!((content_top(cx) + px(900.)).abs() < px(1.));

        let start = thumb(cx).center();
        touch(cx, TouchPhase::Started, start);
        touch(cx, TouchPhase::Cancelled, start);
        assert_eq!(thumb(cx).size.width, resting.size.width);
    }
}

#[gpui::test]
fn mouse_keeps_its_original_target_and_thumb_width(cx: &mut TestAppContext) {
    let (_, cx) = setup(cx, true, false);
    touch(cx, TouchPhase::Started, point(px(165.), px(10.)));
    touch(cx, TouchPhase::Moved, point(px(165.), px(50.)));
    touch(cx, TouchPhase::Cancelled, point(px(165.), px(50.)));
    assert_eq!(content_top(cx), px(0.));

    let resting = thumb(cx);
    let start = resting.center();
    cx.simulate_mouse_move(start, None, Modifiers::default());
    cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
    cx.simulate_mouse_move(
        start + point(px(0.), px(40.)),
        Some(MouseButton::Left),
        Modifiers::default(),
    );
    assert!(content_top(cx) < px(-300.));
    assert_eq!(thumb(cx).size.width, resting.size.width);
    cx.simulate_mouse_up(start, MouseButton::Left, Modifiers::default());
}

fn setup(
    cx: &mut TestAppContext,
    tracked: bool,
    touch: bool,
) -> (Entity<TestView>, &mut VisualTestContext) {
    cx.update(|cx| {
        let settings = settings::SettingsStore::test(cx);
        cx.set_global(settings);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        crate::responsive::set_touch_input(touch, cx);
    });
    cx.add_window_view(|_, _| TestView {
        tracked,
        scroll: ScrollHandle::new(),
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

fn content_top(cx: &mut VisualTestContext) -> Pixels {
    cx.debug_bounds("content").unwrap().origin.y
}

fn thumb(cx: &mut VisualTestContext) -> Bounds<Pixels> {
    cx.update(|window, _| {
        let quads = window.painted_quads();
        assert_eq!(quads.len(), 1);
        let bounds = quads[0].bounds;
        Bounds::new(
            bounds
                .origin
                .map(|value| px(value.as_f32() / window.scale_factor())),
            bounds
                .size
                .map(|value| px(value.as_f32() / window.scale_factor())),
        )
    })
}

struct TestView {
    tracked: bool,
    scroll: ScrollHandle,
}

impl Render for TestView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = div()
            .h(px(2000.))
            .w(px(100.))
            .debug_selector(|| "content".into());
        let container = div().relative().w(px(200.)).h(px(200.));
        let scrollbars = Scrollbars::always_visible(ScrollAxes::Vertical);
        if self.tracked {
            container
                .child(
                    div()
                        .id("scroller")
                        .size_full()
                        .overflow_y_scroll()
                        .track_scroll(&self.scroll)
                        .child(content),
                )
                .custom_scrollbars(scrollbars.tracked_scroll_handle(&self.scroll), window, cx)
        } else {
            container
                .child(content)
                .custom_scrollbars(scrollbars, window, cx)
        }
    }
}
