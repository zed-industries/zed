use super::*;
use gpui::{Empty, TestAppContext, size};

#[gpui::test]
fn phone_layout_requires_both_touch_and_narrow_viewport(cx: &mut TestAppContext) {
    let (_, cx) = cx.add_window_view(|_, _| Empty);
    cx.update(|_, cx| assert!(!is_touch_input(cx)));
    for (width, touch, expected) in [
        (500., false, false),
        (500., true, true),
        (501., true, false),
    ] {
        cx.simulate_resize(size(px(width), px(800.)));
        cx.update(|window, cx| {
            set_touch_input(touch, cx);
            assert_eq!(uses_mobile_layout(window, cx), expected);
        });
    }
}
