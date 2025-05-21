use crate::div_inspector::render_or_load_div_inspector;
use crate::options::InspectorOptions;
use anyhow::Context as _;
use gpui::App;
use std::cell::RefCell;
use std::rc::Rc;
use ui::prelude::*;
use util::ResultExt as _;

// todo!
//
// * Refine "Open code" - confusing that it doesn't do anything when checked. Add another way to
// open code?
//
// * Way to open actual user code instead of ui component

pub fn init(cx: &mut App) {
    // TODO: Instead toggle a global debug mode? Not all windows support the command pallete.
    cx.on_action(|_: &zed_actions::dev::ToggleInspector, cx| {
        let Some(active_window) = cx
            .active_window()
            .context("no active window to toggle inspector")
            .log_err()
        else {
            return;
        };
        // This is deferred to avoid double lease due to window already being updated.
        cx.defer(move |cx| {
            active_window
                .update(cx, |_, window, cx| window.toggle_inspector(cx))
                .log_err();
        });
    });

    let inspector_options = cx.new(|_cx| InspectorOptions {
        open_code_on_inspect: false,
    });

    let load_state = Rc::new(RefCell::new(None));
    cx.register_inspector_element(move |id, state, window, cx| {
        render_or_load_div_inspector(
            inspector_options.clone(),
            &load_state,
            id,
            state,
            window,
            cx,
        )
    })
}
