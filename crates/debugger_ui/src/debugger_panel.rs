use anyhow::Result;
use dap::{
    client::DebugAdapterClient,
    requests::{StackTrace, StackTraceArguments},
    types::{StackFrame, ThreadId},
};
use gpui::{
    actions, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView,
    Subscription, Task, View, ViewContext, WeakView,
};
use std::{collections::HashMap, sync::Arc};
use ui::{prelude::*, Tooltip};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

actions!(debug, [TogglePanel]);

#[derive(Default)]
struct ThreadState {
    pub stack_frames: Option<Vec<StackFrame>>,
}

pub struct DebugPanel {
    pub position: DockPosition,
    pub zoomed: bool,
    pub active: bool,
    pub focus_handle: FocusHandle,
    pub size: Pixels,
    _subscriptions: Vec<Subscription>,
    pub thread_id: Option<ThreadId>,
    pub workspace: WeakView<Workspace>,
    thread_state: HashMap<ThreadId, ThreadState>,
}

impl DebugPanel {
    pub fn new(workspace: WeakView<Workspace>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let project = workspace
                .update(cx, |workspace, _| workspace.project().clone())
                .unwrap();

            let _subscriptions = vec![cx.subscribe(&project, {
                move |this: &mut Self, model, event, cx| {
                    if let project::Event::DebugClientStarted(client_id) = event {
                        dbg!(&event, &client_id);
                    }

                    if let project::Event::DebugClientEvent { client_id, event } = event {
                        match event {
                            dap::events::Event::Initialized(_) => return,
                            dap::events::Event::Stopped(event) => {
                                if let Some(thread_id) = event.thread_id {
                                    let client = this.debug_adapter(cx);

                                    cx.spawn(|this, mut cx| async move {
                                        let res = client
                                            .request::<StackTrace>(StackTraceArguments {
                                                thread_id,
                                                start_frame: None,
                                                levels: None,
                                                format: None,
                                            })
                                            .await?;

                                        this.update(&mut cx, |this, cx| {
                                            if let Some(entry) =
                                                this.thread_state.get_mut(&thread_id)
                                            {
                                                entry.stack_frames = Some(res.stack_frames);

                                                cx.notify();
                                            }

                                            anyhow::Ok(())
                                        })
                                    })
                                    .detach();
                                };
                            }
                            dap::events::Event::Continued(_) => todo!(),
                            dap::events::Event::Exited(_) => todo!(),
                            dap::events::Event::Terminated(_) => todo!(),
                            dap::events::Event::Thread(event) => {
                                if event.reason == "started" {
                                    this.thread_state.insert(
                                        event.thread_id,
                                        ThreadState { stack_frames: None },
                                    );
                                    this.thread_id = Some(event.thread_id);
                                } else {
                                    this.thread_id = None;
                                    this.thread_state.remove(&event.thread_id);
                                }
                            }
                            dap::events::Event::Output(_) => todo!(),
                            dap::events::Event::Breakpoint(_) => todo!(),
                            dap::events::Event::Module(_) => todo!(),
                            dap::events::Event::LoadedSource(_) => todo!(),
                            dap::events::Event::Process(_) => todo!(),
                            dap::events::Event::Capabilities(_) => todo!(),
                            dap::events::Event::Memory(_) => todo!(),
                        }
                    }
                }
            })];

            Self {
                position: DockPosition::Bottom,
                zoomed: false,
                active: false,
                focus_handle: cx.focus_handle(),
                size: px(300.),
                _subscriptions,
                thread_id: Some(ThreadId(1)),
                workspace: workspace.clone(),
                thread_state: Default::default(),
            }
        })
    }

    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move { cx.update(|cx| DebugPanel::new(workspace, cx)) })
    }

    fn debug_adapter(&self, cx: &mut ViewContext<Self>) -> Arc<DebugAdapterClient> {
        self.workspace
            .update(cx, |this, cx| {
                this.project()
                    .read(cx)
                    .running_debug_adapters()
                    .next()
                    .unwrap()
            })
            .unwrap()
    }

    fn render_stack_frames(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(thread_state) = self.thread_id.and_then(|t| self.thread_state.get(&t)) else {
            return div().child("No information for this thread yet").into_any();
        };

        let Some(stack_frames) = &thread_state.stack_frames else {
            return div()
                .child("No stack frames for this thread yet")
                .into_any();
        };

        div()
            .gap_3()
            .children(
                stack_frames
                    .iter()
                    .map(|frame| self.render_stack_frame(frame, cx)),
            )
            .into_any()
    }

    fn render_stack_frame(
        &self,
        stack_frame: &StackFrame,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let source = stack_frame.source.clone();

        div()
            .id(("stack-frame", stack_frame.id))
            .p_1()
            .hover(|s| s.bg(cx.theme().colors().element_hover).cursor_pointer())
            .child(
                h_flex()
                    .gap_0p5()
                    .text_ui_sm(cx)
                    .child(stack_frame.name.clone())
                    .child(format!(
                        "{}:{}",
                        source.clone().and_then(|s| s.name).unwrap_or_default(),
                        stack_frame.line,
                    )),
            )
            .child(
                div()
                    .text_ui_xs(cx)
                    .when_some(source.and_then(|s| s.path), |this, path| {
                        this.child(String::from(path.to_string_lossy()))
                    }),
            )
            .into_any()
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}

impl FocusableView for DebugPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DebugPanel {
    fn persistent_name() -> &'static str {
        "DebugPanel"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _cx: &mut ViewContext<Self>) {
        self.position = position;
        // TODO:
        // cx.update_global::<SettingsStore>(f)
    }

    fn size(&self, _cx: &WindowContext) -> Pixels {
        self.size
    }

    fn set_size(&mut self, size: Option<Pixels>, _cx: &mut ViewContext<Self>) {
        self.size = size.unwrap();
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        None
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        None
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(TogglePanel)
    }

    fn icon_label(&self, _: &WindowContext) -> Option<String> {
        None
    }

    fn is_zoomed(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn starts_open(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn set_zoomed(&mut self, _zoomed: bool, _cx: &mut ViewContext<Self>) {}

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}
}

impl Render for DebugPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        IconButton::new("debug-resume", IconName::Play)
                            .on_click(cx.listener(|view, _, cx| {
                                let client = view.debug_adapter(cx);
                                if let Some(thread_id) = view.thread_id {
                                    cx.background_executor()
                                        .spawn(async move { client.resume(thread_id).await })
                                        .detach();
                                }
                            }))
                            .tooltip(move |cx| Tooltip::text("Continue debug", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-over", IconName::Play)
                            .on_click(cx.listener(|view, _, cx| {
                                let client = view.debug_adapter(cx);
                                if let Some(thread_id) = view.thread_id {
                                    cx.background_executor()
                                        .spawn(async move { client.step_over(thread_id).await })
                                        .detach();
                                }
                            }))
                            .tooltip(move |cx| Tooltip::text("Step over", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-in", IconName::Play)
                            .on_click(cx.listener(|view, _, cx| {
                                let client = view.debug_adapter(cx);

                                if let Some(thread_id) = view.thread_id {
                                    cx.background_executor()
                                        .spawn(async move { client.step_in(thread_id).await })
                                        .detach();
                                }
                            }))
                            .tooltip(move |cx| Tooltip::text("Go in", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-out", IconName::Play)
                            .on_click(cx.listener(|view, _, cx| {
                                let client = view.debug_adapter(cx);
                                if let Some(thread_id) = view.thread_id {
                                    cx.background_executor()
                                        .spawn(async move { client.step_out(thread_id).await })
                                        .detach();
                                }
                            }))
                            .tooltip(move |cx| Tooltip::text("Go out", cx)),
                    )
                    .child(
                        IconButton::new("debug-restart", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Restart", cx)),
                    )
                    .child(
                        IconButton::new("debug-stop", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Stop", cx)),
                    ),
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(self.render_stack_frames(cx))
                    .child("Here see all the vars"),
            )
            .into_any()
    }
}
