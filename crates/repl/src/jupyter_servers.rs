use gpui::{
    div, prelude::*, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    Refineable, Render, ScrollHandle, ViewContext, WeakView,
};
use ui::{ActiveTheme, IntoElement};
use workspace::{ModalView, Workspace};

gpui::actions!(repl, [ConnectJupyterServer]);

pub struct JupyterServers {
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    workspace: WeakView<Workspace>,
}

impl JupyterServers {
    pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        //
        workspace.register_action(|workspace, _: &ConnectJupyterServer, cx| {
            let handle = cx.view().downgrade();
            workspace.toggle_modal(cx, |cx| Self::new(cx, handle));
        });
    }

    pub fn new(cx: &mut ViewContext<Self>, workspace: WeakView<Workspace>) -> Self {
        let focus_handle = cx.focus_handle();
        // let dev_server_store = dev_server_projects::Store::global(cx);

        // let subscription = cx.observe(&dev_server_store, |_, _, cx| {
        //     cx.notify();
        // });

        let mut base_style = cx.text_style();
        base_style.refine(&gpui::TextStyleRefinement {
            color: Some(cx.theme().colors().editor_foreground),
            ..Default::default()
        });

        Self {
            // mode: Mode::Default,
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            // dev_server_store,
            workspace,
            // _dev_server_subscription: subscription,
            // selectable_items: Default::default(),
        }
    }
}

impl ModalView for JupyterServers {}

impl EventEmitter<DismissEvent> for JupyterServers {}

impl FocusableView for JupyterServers {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for JupyterServers {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("test")
        //
    }
}
