use std::sync::atomic::{AtomicU32, Ordering};

use gpui::{ScrollHandle, prelude::*};
use ui::prelude::*;

use crate::{NonJsonItem, SettingsWindow, USER, render_non_json_item};

static COUNTER: AtomicU32 = AtomicU32::new(0);

pub(crate) fn render_llm_providers_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let test_item = NonJsonItem {
        title: "Test Counter",
        description: "A test non-JSON-backed setting with a counter.",
        json_path: Some("test.counter"),
        files: USER,
        can_reset: |_cx| COUNTER.load(Ordering::SeqCst) != 0,
        reset: |_window, _cx| {
            COUNTER.store(0, Ordering::SeqCst);
        },
        render_control: |_settings_window, _window, cx| {
            let value = COUNTER.load(Ordering::SeqCst);
            h_flex()
                .gap_2()
                .items_center()
                .child(Label::new(format!("Count: {value}")))
                .child(
                    Button::new("increment", "Increment")
                        .tab_index(0_isize)
                        .style(ButtonStyle::Outlined)
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            COUNTER.fetch_add(1, Ordering::SeqCst);
                            cx.notify();
                        })),
                )
                .into_any_element()
        },
    };

    let item = render_non_json_item(settings_window, &test_item, window, cx);

    v_flex()
        .id("llm-providers-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(item)
        .into_any_element()
}
