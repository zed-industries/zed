use anyhow::Result;
use dap::{client::DebugAdapterClientId, Module, ModuleEvent};
use gpui::{list, AnyElement, FocusHandle, FocusableView, ListState, Model, Task};
use project::dap_store::DapStore;
use ui::prelude::*;

pub struct ModuleList {
    list: ListState,
    modules: Vec<Module>,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    client_id: DebugAdapterClientId,
}

impl ModuleList {
    pub fn new(
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

        let this = Self {
            list,
            dap_store,
            focus_handle,
            client_id: *client_id,
            modules: Vec::default(),
        };

        this.fetch_modules(cx).detach_and_log_err(cx);

        this
    }

    pub fn on_module_event(&mut self, event: &ModuleEvent, cx: &mut ViewContext<Self>) {
        match event.reason {
            dap::ModuleEventReason::New => self.modules.push(event.module.clone()),
            dap::ModuleEventReason::Changed => {
                if let Some(module) = self.modules.iter_mut().find(|m| m.id == event.module.id) {
                    *module = event.module.clone();
                }
            }
            dap::ModuleEventReason::Removed => self.modules.retain(|m| m.id != event.module.id),
        }

        self.list.reset(self.modules.len());
        cx.notify();
    }

    fn fetch_modules(&self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let task = self
            .dap_store
            .update(cx, |store, cx| store.modules(&self.client_id, cx));

        cx.spawn(|this, mut cx| async move {
            let mut modules = task.await?;

            this.update(&mut cx, |this, cx| {
                std::mem::swap(&mut this.modules, &mut modules);
                this.list.reset(this.modules.len());

                cx.notify();
            })
        })
    }

    fn render_entry(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let module = &self.modules[ix];

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

impl FocusableView for ModuleList {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ModuleList {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
    }
}
