use gpui::{list, AnyElement, Empty, Entity, FocusHandle, Focusable, ListState, Subscription};
use project::debugger::session::{Session, SessionEvent};
use ui::prelude::*;
use util::maybe;

pub struct ModuleList {
    list: ListState,
    invalidate: bool,
    session: Entity<Session>,
    focus_handle: FocusHandle,
    _subscription: Subscription,
}

impl ModuleList {
    pub fn new(session: Entity<Session>, cx: &mut Context<Self>) -> Self {
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

        let _subscription = cx.subscribe(&session, |this, _, event, cx| match event {
            SessionEvent::Stopped | SessionEvent::Modules => {
                this.invalidate = true;
                cx.notify();
            }
            _ => {}
        });

        Self {
            list,
            session,
            focus_handle,
            _subscription,
            invalidate: true,
        }
    }

    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let Some(module) = maybe!({
            self.session
                .update(cx, |state, cx| state.modules(cx).get(ix).cloned())
        }) else {
            return Empty.into_any();
        };

        v_flex()
            .rounded_md()
            .w_full()
            .group("")
            .p_1()
            .hover(|s| s.bg(cx.theme().colors().element_hover))
            .child(h_flex().gap_0p5().text_ui_sm(cx).child(module.name.clone()))
            .child(
                h_flex()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .when_some(module.path.clone(), |this, path| this.child(path)),
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
        if self.invalidate {
            let len = self
                .session
                .update(cx, |session, cx| session.modules(cx).len());
            self.list.reset(len);
            self.invalidate = false;
            cx.notify();
        }

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
    }
}
