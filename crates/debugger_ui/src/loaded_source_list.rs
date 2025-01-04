use anyhow::Result;
use dap::{client::DebugAdapterClientId, LoadedSourceEvent, Source};
use gpui::{
    list, AnyElement, FocusHandle, FocusableView, ListState, Model, Subscription, Task, View,
};
use project::dap_store::DapStore;
use ui::prelude::*;

use crate::debugger_panel_item::{self, DebugPanelItem, DebugPanelItemEvent};

pub struct LoadedSourceList {
    list: ListState,
    sources: Vec<Source>,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    client_id: DebugAdapterClientId,
    _subscriptions: Vec<Subscription>,
}

impl LoadedSourceList {
    pub fn new(
        debug_panel_item: &View<DebugPanelItem>,
        dap_store: Model<DapStore>,
        client_id: &DebugAdapterClientId,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let weakview = cx.view().downgrade();
        let focus_handle = cx.focus_handle();

        let list = ListState::new(0, gpui::ListAlignment::Top, px(1000.), move |ix, cx| {
            weakview
                .upgrade()
                .map(|view| view.update(cx, |this, cx| this.render_entry(ix, cx)))
                .unwrap_or(div().into_any())
        });

        let _subscriptions =
            vec![cx.subscribe(debug_panel_item, Self::handle_debug_panel_item_event)];

        Self {
            list,
            dap_store,
            focus_handle,
            _subscriptions,
            client_id: *client_id,
            sources: Vec::default(),
        }
    }

    fn handle_debug_panel_item_event(
        &mut self,
        _: View<DebugPanelItem>,
        event: &debugger_panel_item::DebugPanelItemEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            DebugPanelItemEvent::Stopped { .. } => {
                self.fetch_loaded_sources(cx).detach_and_log_err(cx);
            }
            _ => {}
        }
    }

    pub fn on_loaded_source_event(
        &mut self,
        event: &LoadedSourceEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event.reason {
            dap::LoadedSourceEventReason::New => self.sources.push(event.source.clone()),
            dap::LoadedSourceEventReason::Changed => {
                let updated_source =
                    if let Some(ref_id) = event.source.source_reference.filter(|&r| r != 0) {
                        self.sources
                            .iter_mut()
                            .find(|s| s.source_reference == Some(ref_id))
                    } else if let Some(path) = &event.source.path {
                        self.sources
                            .iter_mut()
                            .find(|s| s.path.as_ref() == Some(path))
                    } else {
                        self.sources
                            .iter_mut()
                            .find(|s| s.name == event.source.name)
                    };

                if let Some(loaded_source) = updated_source {
                    *loaded_source = event.source.clone();
                }
            }
            dap::LoadedSourceEventReason::Removed => {
                self.sources.retain(|source| *source != event.source)
            }
        }

        self.list.reset(self.sources.len());
        cx.notify();
    }

    fn fetch_loaded_sources(&self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let task = self
            .dap_store
            .update(cx, |store, cx| store.loaded_sources(&self.client_id, cx));

        cx.spawn(|this, mut cx| async move {
            let mut sources = task.await?;

            this.update(&mut cx, |this, cx| {
                std::mem::swap(&mut this.sources, &mut sources);
                this.list.reset(this.sources.len());

                cx.notify();
            })
        })
    }

    fn render_entry(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let source = &self.sources[ix];

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
                    .when_some(source.path.clone(), |this, path| this.child(path)),
            )
            .into_any()
    }
}

impl FocusableView for LoadedSourceList {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LoadedSourceList {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
    }
}
