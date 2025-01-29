use anyhow::Result;
use dap::{
    client::DebugAdapterClientId, proto_conversions::ProtoConversion, session::DebugSessionId,
    Module, ModuleEvent,
};
use gpui::{list, AnyElement, Entity, FocusHandle, Focusable, ListState, Task};
use project::dap_store::DapStore;
use rpc::proto::{DebuggerModuleList, UpdateDebugAdapter};
use ui::prelude::*;
use util::ResultExt;

pub struct ModuleList {
    list: ListState,
    modules: Vec<Module>,
    focus_handle: FocusHandle,
    dap_store: Entity<DapStore>,
    client_id: DebugAdapterClientId,
    session_id: DebugSessionId,
}

impl ModuleList {
    pub fn new(
        dap_store: Entity<DapStore>,
        client_id: &DebugAdapterClientId,
        session_id: &DebugSessionId,
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

        let this = Self {
            list,
            dap_store,
            focus_handle,
            client_id: *client_id,
            session_id: *session_id,
            modules: Vec::default(),
        };

        if this.dap_store.read(cx).as_local().is_some() {
            this.fetch_modules(cx).detach_and_log_err(cx);
        }
        this
    }

    pub(crate) fn set_from_proto(
        &mut self,
        module_list: &DebuggerModuleList,
        cx: &mut Context<Self>,
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

    pub fn on_module_event(&mut self, event: &ModuleEvent, cx: &mut Context<Self>) {
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

        let task = cx.spawn(|this, mut cx| async move {
            this.update(&mut cx, |this, cx| {
                this.propagate_updates(cx);
            })
            .log_err();
        });

        cx.background_executor().spawn(task).detach();
    }

    fn fetch_modules(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let task = self
            .dap_store
            .update(cx, |store, cx| store.modules(&self.client_id, cx));

        cx.spawn(|this, mut cx| async move {
            let mut modules = task.await?;

            this.update(&mut cx, |this, cx| {
                std::mem::swap(&mut this.modules, &mut modules);
                this.list.reset(this.modules.len());
                cx.notify();

                this.propagate_updates(cx);
            })
        })
    }

    fn propagate_updates(&self, cx: &Context<Self>) {
        if let Some((client, id)) = self.dap_store.read(cx).downstream_client() {
            let request = UpdateDebugAdapter {
                session_id: self.session_id.to_proto(),
                client_id: self.client_id.to_proto(),
                project_id: *id,
                thread_id: None,
                variant: Some(rpc::proto::update_debug_adapter::Variant::Modules(
                    self.to_proto(),
                )),
            };

            client.send(request).log_err();
        }
    }

    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
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

impl Focusable for ModuleList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ModuleList {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
    }
}

#[cfg(any(test, feature = "test-support"))]
impl ModuleList {
    pub fn modules(&self) -> &Vec<Module> {
        &self.modules
    }
}
