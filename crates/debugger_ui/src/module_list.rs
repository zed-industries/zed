use anyhow::Result;
use dap::{
    client::DebugAdapterClientId, proto_conversions::ProtoConversion, session::DebugSessionId,
    Module, ModuleEvent,
};
use gpui::{list, AnyElement, FocusHandle, FocusableView, ListState, Model, Task};
use project::dap_store::DapStore;
use rpc::proto::{DebuggerModuleList, UpdateDebugAdapter};
use ui::prelude::*;
use util::ResultExt;

pub struct ModuleList {
    list: ListState,
    modules: Vec<Module>,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    client_id: DebugAdapterClientId,
    session_id: DebugSessionId,
}

impl ModuleList {
    pub fn new(
        dap_store: Model<DapStore>,
        client_id: &DebugAdapterClientId,
        session_id: &DebugSessionId,
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
            session_id: *session_id,
            modules: Vec::default(),
        };

        this.fetch_modules(cx).detach_and_log_err(cx);

        this
    }

    pub(crate) fn set_from_proto(
        &mut self,
        module_list: &DebuggerModuleList,
        cx: &mut ViewContext<Self>,
    ) {
        self.modules = module_list
            .modules
            .iter()
            .filter_map(|payload| Module::from_proto(payload.clone()).log_err())
            .collect();

        self.client_id = DebugAdapterClientId::from_proto(module_list.client_id);

        self.list.reset(self.modules.len());
        cx.notify();
    }

    pub(crate) fn to_proto(&self) -> DebuggerModuleList {
        DebuggerModuleList {
            client_id: self.client_id.to_proto(),
            modules: self
                .modules
                .iter()
                .map(|module| module.to_proto())
                .collect(),
        }
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

                if let Some((client, id)) = this.dap_store.read(cx).downstream_client() {
                    let request = UpdateDebugAdapter {
                        session_id: this.session_id.to_proto(),
                        client_id: this.client_id.to_proto(),
                        project_id: *id,
                        thread_id: None,
                        variant: Some(rpc::proto::update_debug_adapter::Variant::Modules(
                            this.to_proto(),
                        )),
                    };

                    client.send(request).log_err();
                }
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
