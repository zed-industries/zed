use anyhow::Result;
use async_dispatcher::{set_dispatcher, Dispatcher, Runnable};
use client::telemetry::Telemetry;
use collections::HashMap;
use editor::{display_map::BlockId, Anchor, Editor};
use gpui::{
    prelude::*, AppContext, AsyncWindowContext, EntityId, EventEmitter, FocusHandle, FocusOutEvent,
    FocusableView, Model, PlatformDispatcher, Subscription, Task, View, WeakView,
};
use project::Fs;
use settings::{Settings as _, SettingsStore};
use std::{ops::Range, sync::Arc, time::Duration};
use ui::prelude::*;
use workspace::{
    dock::{Panel, PanelEvent},
    Workspace,
};

use crate::{run, ExecutionView, JupyterSettings, Kernel, RuntimeManager, ToggleFocus};

#[allow(unused)]
struct EditorBlock {
    code_range: Range<Anchor>,
    block_id: BlockId,
    execution_view: View<ExecutionView>,
}

#[allow(unused)]
struct EditorState {
    blocks: Vec<EditorBlock>,
    _subscription: Subscription,
}

pub fn zed_dispatcher(cx: &mut AppContext) -> impl Dispatcher {
    struct ZedDispatcher {
        dispatcher: Arc<dyn PlatformDispatcher>,
    }

    // PlatformDispatcher is _super_ close to the same interface we put in
    // async-dispatcher, except for the task label in dispatch. Later we should
    // just make that consistent so we have this dispatcher ready to go for
    // other crates in Zed.
    impl Dispatcher for ZedDispatcher {
        fn dispatch(&self, runnable: Runnable) {
            self.dispatcher.dispatch(runnable, None)
        }

        fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
            self.dispatcher.dispatch_after(duration, runnable);
        }
    }

    ZedDispatcher {
        dispatcher: cx.background_executor().dispatcher.clone(),
    }
}

pub fn init(cx: &mut AppContext) {
    set_dispatcher(zed_dispatcher(cx));
    JupyterSettings::register(cx);
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    let settings = JupyterSettings::get_global(cx);
                    if !settings.enabled {
                        return;
                    }

                    workspace.toggle_panel_focus::<RuntimePanel>(cx);
                })
                .register_action(run);
            // .register_action(RuntimePanel::start_kernel)
            // .register_action(RuntimePanel::stop_kernel)
            // .register_action(RuntimePanel::execute_code);
        },
    )
    .detach();
}

#[allow(unused)]
pub struct RuntimePanel {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    running_kernels: HashMap<EntityId, Kernel>,
    runtime_manager: Model<RuntimeManager>,
    editors: HashMap<WeakView<Editor>, EditorState>,
    execution_views: HashMap<String, View<ExecutionView>>,
    telemetry: Arc<Telemetry>,
    fs: Arc<dyn Fs>,
    _subscriptions: Vec<Subscription>,
}

impl RuntimePanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let view = workspace.update(&mut cx, |workspace, cx| {
                cx.new_view::<Self>(|cx| {
                    let focus_handle = cx.focus_handle();

                    let fs = workspace.app_state().fs.clone();

                    let runtime_manager = cx.new_model(|cx| RuntimeManager::new(fs.clone(), cx));
                    RuntimeManager::set_global(runtime_manager.clone(), cx);

                    let subscriptions = vec![
                        cx.on_focus_in(&focus_handle, Self::focus_in),
                        cx.on_focus_out(&focus_handle, Self::focus_out),
                        cx.observe(&runtime_manager, Self::handle_update),
                        cx.observe_global::<SettingsStore>(move |_this, cx| {
                            let settings = JupyterSettings::get_global(cx);
                            if settings.enabled && RuntimeManager::global(cx).is_none() {
                                // todo!(): turn panel and operations on and off
                            } else {
                                // todo!()
                            }
                        }),
                    ];

                    Self {
                        width: None,
                        focus_handle,
                        runtime_manager: runtime_manager.clone(),
                        workspace: workspace.weak_handle(),
                        running_kernels: Default::default(),
                        editors: Default::default(),
                        execution_views: Default::default(),
                        telemetry: workspace.client().telemetry().clone(),
                        fs: fs.clone(),
                        _subscriptions: subscriptions,
                    }
                })
            })?;

            view.update(&mut cx, |this, cx| {
                this.runtime_manager
                    .update(cx, |runtime_manager, cx| runtime_manager.load(cx))
            })?;

            Ok(view)
        })
    }

    pub fn handle_update(&mut self, _model: Model<RuntimeManager>, cx: &mut ViewContext<Self>) {
        dbg!(_model);
        cx.notify();
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn focus_out(&mut self, _event: FocusOutEvent, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl Panel for RuntimePanel {
    fn persistent_name() -> &'static str {
        "RuntimePanel"
    }

    fn position(&self, _cx: &ui::WindowContext) -> workspace::dock::DockPosition {
        // todo!(): Pull from settings
        workspace::dock::DockPosition::Right
    }

    fn position_is_valid(&self, _position: workspace::dock::DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        _position: workspace::dock::DockPosition,
        _cx: &mut ViewContext<Self>,
    ) {
        // todo!()
    }

    fn size(&self, cx: &ui::WindowContext) -> Pixels {
        let settings = JupyterSettings::get_global(cx);

        self.width.unwrap_or(settings.default_width)
    }

    fn set_size(&mut self, size: Option<ui::Pixels>, _cx: &mut ViewContext<Self>) {
        self.width = size;
    }

    fn icon(&self, _cx: &ui::WindowContext) -> Option<ui::IconName> {
        // todo!(): get an icon for runtime panel
        Some(IconName::Code)
    }

    fn icon_tooltip(&self, _cx: &ui::WindowContext) -> Option<&'static str> {
        Some("Runtime Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for RuntimePanel {}

impl FocusableView for RuntimePanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RuntimePanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let runtime_manager = self.runtime_manager.read(cx);

        v_flex()
            .child(Label::new("Runtime Panel"))
            .children(
                runtime_manager
                    .runtime_specifications
                    .iter()
                    .map(|spec| div().child(spec.name.clone())),
            )
            .into_any_element()
    }
}

// Goal: move the execution views to be owned by the runtime panel
//       and to have all messages get collected as one stream
