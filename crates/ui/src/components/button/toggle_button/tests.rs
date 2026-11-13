use std::{cell::Cell, rc::Rc};

use gpui::{Context, Pixels, Render, TestAppContext, VisualTestContext, Window};

use super::*;

#[gpui::test]
fn toggle_buttons_center_icons_without_reserving_label_space(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = settings::SettingsStore::test(cx);
        cx.set_global(settings);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
    });
    let clicks = Rc::new(Cell::new(0));
    let window = cx.add_window(|_, _| TestToggleGroup {
        style: ToggleButtonGroupStyle::Transparent,
        size: ToggleButtonGroupSize::Default,
        rem_size: px(16.),
        auto_width: true,
        labeled: false,
        clicks: clicks.clone(),
    });
    let view = window.root(cx).expect("test view");
    let cx = &mut VisualTestContext::from_window(*window, cx);

    for (size, style, rem_size, auto_width, expected_height) in [
        (
            ToggleButtonGroupSize::Default,
            ToggleButtonGroupStyle::Transparent,
            px(16.),
            true,
            px(22.),
        ),
        (
            ToggleButtonGroupSize::Medium,
            ToggleButtonGroupStyle::Filled,
            px(16.),
            true,
            px(28.),
        ),
        (
            ToggleButtonGroupSize::Large,
            ToggleButtonGroupStyle::Outlined,
            px(16.),
            true,
            px(32.),
        ),
        (
            ToggleButtonGroupSize::Custom(rems_from_px(24_f32)),
            ToggleButtonGroupStyle::Outlined,
            px(20.),
            true,
            px(30.),
        ),
        (
            ToggleButtonGroupSize::Default,
            ToggleButtonGroupStyle::Outlined,
            px(16.),
            false,
            px(22.),
        ),
    ] {
        view.update(cx, |view, cx| {
            view.style = style;
            view.size = size;
            view.rem_size = rem_size;
            view.auto_width = auto_width;
            cx.notify();
        });
        paint(cx);
        for (button_selector, content_selector) in [
            ("toggles-0", "toggles-0-content"),
            ("toggles-1", "toggles-1-content"),
        ] {
            let button = cx.debug_bounds(button_selector).expect("button bounds");
            let content = cx.debug_bounds(content_selector).expect("icon bounds");
            assert_eq!(button.size.height, expected_height);
            if auto_width {
                assert_eq!(button.size.width, button.size.height);
            } else {
                assert!(button.size.width > button.size.height);
            }
            let icon_size = IconSize::XSmall.rems().to_pixels(rem_size);
            assert_eq!(content.size, gpui::size(icon_size, icon_size));
            let offset = content.center() - button.center();
            assert!(offset.x.abs() < px(0.01) && offset.y.abs() < px(0.01));
            let before = clicks.get();
            cx.simulate_click(button.center(), Default::default());
            assert_eq!(clicks.get(), before + 1);
        }
    }

    view.update(cx, |view, cx| {
        view.auto_width = true;
        view.labeled = true;
        cx.notify();
    });
    paint(cx);
    let button = cx.debug_bounds("toggles-0").expect("labeled button bounds");
    let content = cx
        .debug_bounds("toggles-0-content")
        .expect("labeled content bounds");
    assert!(button.size.width > button.size.height);
    assert!(content.size.width > IconSize::XSmall.rems().to_pixels(px(16.)));
}

#[test]
fn icon_only_toggle_keeps_its_label_for_accessibility() {
    let button = ToggleButtonWithIcon::icon_only("Show Commits", IconName::GitCommit, |_, _, _| {})
        .into_configuration();
    assert!(button.label.is_none());
    assert_eq!(button.aria_label.as_deref(), Some("Show Commits"));
}

struct TestToggleGroup {
    style: ToggleButtonGroupStyle,
    size: ToggleButtonGroupSize,
    rem_size: Pixels,
    auto_width: bool,
    labeled: bool,
    clicks: Rc<Cell<usize>>,
}

impl Render for TestToggleGroup {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        window.set_rem_size(self.rem_size);
        let buttons =
            [("All", IconName::Menu), ("Commits", IconName::GitCommit)].map(|(label, icon)| {
                let clicks = self.clicks.clone();
                let on_click = move |_: &ClickEvent, _: &mut Window, _: &mut App| {
                    clicks.set(clicks.get() + 1);
                };
                if self.labeled {
                    ToggleButtonWithIcon::new(label, icon, on_click)
                } else {
                    ToggleButtonWithIcon::icon_only(label, icon, on_click)
                }
            });
        v_flex().items_start().child(
            ToggleButtonGroup::single_row("toggles", buttons)
                .style(self.style)
                .size(self.size)
                .selected_index(1)
                .map(|group| {
                    if self.auto_width {
                        group.auto_width()
                    } else {
                        group.width(px(160.))
                    }
                }),
        )
    }
}

fn paint(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    cx.update(|window, _| window.refresh());
    cx.run_until_parked();
}
