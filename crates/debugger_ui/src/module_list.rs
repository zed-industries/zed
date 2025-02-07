use dap::{client::DebugAdapterClientId, ModuleEvent};
use gpui::{list, AnyElement, Empty, Entity, FocusHandle, Focusable, ListState};
use project::dap_session::DebugSession;
use ui::prelude::*;

pub struct ModuleList {
    list: ListState,
    focus_handle: FocusHandle,
    session: Entity<DebugSession>,
    client_id: DebugAdapterClientId,
    modules_len: usize,
}

impl ModuleList {
    pub fn new(
        session: Entity<DebugSession>,
        client_id: &DebugAdapterClientId,
        cx: &mut Context<Self>,
    ) -> Self {
        let weak_entity = cx.weak_entity();
        let focus_handle = cx.focus_handle();

        let list = ListState::new(
            0,
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, _window, cx| {
                weak_entity
                    .upgrade()
                    .map(|module_list| module_list.update(cx, |this, cx| this.render_entry(ix, cx)))
                    .unwrap_or(div().into_any())
            },
        );

        session.update(cx, |session, _cx| {
            session.client_state(*client_id).unwrap();
        });

        Self {
            list,
            session,
            focus_handle,
            client_id: *client_id,
            modules_len: 0,
        }
    }

    pub fn on_module_event(&mut self, event: &ModuleEvent, cx: &mut Context<Self>) {
        if let Some(state) = self.session.read(cx).client_state(self.client_id) {
            let modules_len = state.update(cx, |state, cx| {
                state.handle_module_event(event);
                state.modules(cx).len()
            });

            if modules_len != self.modules_len {
                self.modules_len = modules_len;
                self.list.reset(self.modules_len);
            }

            cx.notify()
        }
    }

    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let Some((module_name, module_path)) = self.session.update(cx, |session, cx| {
            session
                .client_state(self.client_id)?
                .update(cx, |state, cx| {
                    state
                        .modules(cx)
                        .get(ix)
                        .map(|module| (module.name.clone(), module.path.clone()))
                })
        }) else {
            return Empty.into_any();
        };

        v_flex()
            .rounded_md()
            .w_full()
            .group("")
            .p_1()
            .hover(|s| s.bg(cx.theme().colors().element_hover))
            .child(h_flex().gap_0p5().text_ui_sm(cx).child(module_name))
            .child(
                h_flex()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .when_some(module_path, |this, path| this.child(path)),
            )
            .into_any()
    }
}

impl Focusable for ModuleList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ModuleList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let session = self.session.read(cx);
        let modules_len = session
            .client_state(self.client_id)
            .map_or(0usize, |state| {
                state.update(cx, |state, cx| state.modules(cx).len())
            });

        if modules_len != self.modules_len {
            self.modules_len = modules_len;
            self.list.reset(self.modules_len);
        }

        div()
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
    }
}

#[cfg(any(test, feature = "test-support"))]
use dap::Module;

#[cfg(any(test, feature = "test-support"))]
impl ModuleList {
    pub fn modules(&self, cx: &mut Context<Self>) -> Vec<Module> {
        let Some(state) = self.session.read(cx).client_state(self.client_id) else {
            return vec![];
        };

        state.update(cx, |state, cx| state.modules(cx).iter().cloned().collect())
    }
}
