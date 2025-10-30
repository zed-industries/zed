use gpui::{AnyElement, Empty, Entity, FocusHandle, Focusable, ListState, Subscription, list};
use project::debugger::session::{Session, SessionEvent};
use ui::prelude::*;
use util::maybe;

pub(crate) struct LoadedSourceList {
    list: ListState,
    invalidate: bool,
    focus_handle: FocusHandle,
    _subscription: Subscription,
    session: Entity<Session>,
}

impl LoadedSourceList {
    pub fn new(session: Entity<Session>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let list = ListState::new(0, gpui::ListAlignment::Top, px(1000.));

        let _subscription = cx.subscribe(&session, |this, _, event, cx| match event {
            SessionEvent::Stopped(_) | SessionEvent::LoadedSources => {
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
        let Some(source) = maybe!({
            self.session
                .update(cx, |state, cx| state.loaded_sources(cx).get(ix).cloned())
        }) else {
            return Empty.into_any();
        };

        v_flex()
            .rounded_md()
            .w_full()
            .group("")
            .p_1()
            .hover(|s| s.bg(cx.theme().colors().element_hover))
            .child(
                h_flex()
                    .gap_0p5()
                    .text_ui_sm(cx)
                    .when_some(source.name.clone(), |this, name| this.child(name)),
            )
            .child(
                h_flex()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .when_some(source.path, |this, path| this.child(path)),
            )
            .into_any()
    }
}

impl Focusable for LoadedSourceList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LoadedSourceList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.invalidate {
            let len = self
                .session
                .update(cx, |session, cx| session.loaded_sources(cx).len());
            self.list.reset(len);
            self.invalidate = false;
            cx.notify();
        }

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_1()
            .child(
                list(
                    self.list.clone(),
                    cx.processor(|this, ix, _window, cx| this.render_entry(ix, cx)),
                )
                .size_full(),
            )
    }
}
