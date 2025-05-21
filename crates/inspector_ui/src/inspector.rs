use anyhow::Context as _;
use gpui::{App, IntoElement, Window};
use std::{cell::RefCell, rc::Rc};
use ui::{CheckboxWithLabel, Label, prelude::*};
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
        crate::div_inspector::render_or_load(
            inspector_options.clone(),
            &load_state,
            id,
            state,
            window,
            cx,
        )
    })
}

pub(crate) struct InspectorOptions {
    pub open_code_on_inspect: bool,
}

impl Render for InspectorOptions {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        CheckboxWithLabel::new(
            "open-code-on-inspect",
            Label::new("Open code"),
            self.open_code_on_inspect.into(),
            cx.listener(|this, selection: &ToggleState, _, _| {
                this.open_code_on_inspect = selection.selected();
            }),
        )
    }
}
