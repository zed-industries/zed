//! Modal implementation for the which-key display.

use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, KeybindingKeystroke,
    ScrollHandle, Subscription, WeakEntity, Window,
};
use settings::Settings;
use std::rc::Rc;
use theme_settings::ThemeSettings;
use ui::{DynamicSpacing, prelude::*};
use workspace::{ModalView, Workspace};

use crate::{
    bindings_for_which_key, map_pending_keystrokes,
    pending_bindings::{PendingBindingRow, PendingBindings, prepare_pending_bindings},
};

pub struct WhichKeyModal {
    _workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    bindings: Rc<[PendingBindingRow]>,
    pending_keys: Rc<[KeybindingKeystroke]>,
    _pending_input_subscription: Subscription,
    _focus_out_subscription: Subscription,
}

impl WhichKeyModal {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Keep focus where it currently is
        let focus_handle = window.focused(cx).unwrap_or(cx.focus_handle());

        let handle = cx.weak_entity();
        let mut this = Self {
            _workspace: workspace,
            focus_handle: focus_handle.clone(),
            scroll_handle: ScrollHandle::new(),
            bindings: Rc::from([]),
            pending_keys: Rc::from([]),
            _pending_input_subscription: cx.observe_pending_input(
                window,
                |this: &mut Self, window, cx| {
                    this.update_pending_keys(window, cx);
                },
            ),
            _focus_out_subscription: window.on_focus_out(&focus_handle, cx, move |_, _, cx| {
                handle.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
            }),
        };
        this.update_pending_keys(window, cx);
        this
    }

    pub fn dismiss(&self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }

    fn update_pending_keys(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pending_keys) = window.pending_input_keystrokes() else {
            cx.emit(DismissEvent);
            return;
        };
        self.bindings =
            prepare_pending_bindings(bindings_for_which_key(window, pending_keys), cx).into();
        let pending_keys = map_pending_keystrokes(pending_keys, cx.keyboard_mapper().as_ref());
        if self.pending_keys.as_ref() != pending_keys.as_slice() {
            self.scroll_handle.set_offset(Default::default());
        }
        self.pending_keys = pending_keys.into();
    }
}

impl Render for WhichKeyModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport_size = window.viewport_size();

        let max_panel_width = px((f32::from(viewport_size.width) * 0.5).min(480.0));
        let max_content_height = px(f32::from(viewport_size.height) * 0.4);

        // Push above status bar when visible
        let status_height = self
            ._workspace
            .upgrade()
            .and_then(|workspace| {
                workspace.read_with(cx, |workspace, cx| {
                    if workspace.status_bar_visible(cx) {
                        Some(
                            DynamicSpacing::Base04.px(cx) * 2.0
                                + ThemeSettings::get_global(cx).ui_font_size(cx),
                        )
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(px(0.));

        let margin_bottom = px(16.);
        let bottom_offset = margin_bottom + status_height;

        div()
            .id("which-key-buffer-panel-scroll")
            .occlude()
            .absolute()
            .bottom(bottom_offset)
            .right(px(16.))
            .min_w(px(220.))
            .max_w(max_panel_width)
            .elevation_3(cx)
            .overflow_hidden()
            .child(PendingBindings::new(
                "which-key-content",
                self.pending_keys.clone(),
                self.bindings.clone(),
                self.scroll_handle.clone(),
                max_content_height,
            ))
    }
}

impl EventEmitter<DismissEvent> for WhichKeyModal {}

impl Focusable for WhichKeyModal {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for WhichKeyModal {
    fn render_bare(&self) -> bool {
        true
    }
}
