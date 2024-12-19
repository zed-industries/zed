use std::path::Path;

use anyhow::{anyhow, Result};
use dap::client::DebugAdapterClientId;
use dap::proto_conversions::ProtoConversion;
use dap::StackFrame;
use gpui::{
    list, AnyElement, EventEmitter, FocusHandle, ListState, Subscription, Task, View, WeakView,
};
use gpui::{FocusableView, Model};
use project::dap_store::DapStore;
use project::ProjectPath;
use rpc::proto::{DebuggerStackFrameList, UpdateDebugAdapter};
use ui::ViewContext;
use ui::{prelude::*, Tooltip};
use util::ResultExt;
use workspace::Workspace;

use crate::debugger_panel_item::DebugPanelItemEvent::Stopped;
use crate::debugger_panel_item::{self, DebugPanelItem};

#[derive(Debug)]
pub enum StackFrameListEvent {
    SelectedStackFrameChanged,
    StackFramesUpdated,
}

pub struct StackFrameList {
    thread_id: u64,
    list: ListState,
    focus_handle: FocusHandle,
    dap_store: Model<DapStore>,
    current_stack_frame_id: u64,
    stack_frames: Vec<StackFrame>,
    workspace: WeakView<Workspace>,
    client_id: DebugAdapterClientId,
    _subscriptions: Vec<Subscription>,
    fetch_stack_frames_task: Option<Task<Result<()>>>,
}

impl StackFrameList {
    pub fn new(
        workspace: &WeakView<Workspace>,
        debug_panel_item: &View<DebugPanelItem>,
        dap_store: &Model<DapStore>,
        client_id: &DebugAdapterClientId,
        thread_id: u64,
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
            thread_id,
            focus_handle,
            _subscriptions,
            client_id: *client_id,
            workspace: workspace.clone(),
            dap_store: dap_store.clone(),
            fetch_stack_frames_task: None,
            stack_frames: Default::default(),
            current_stack_frame_id: Default::default(),
        }
    }

    pub(crate) fn thread_id(&self) -> u64 {
        self.thread_id
    }

    pub(crate) fn to_proto(&self) -> DebuggerStackFrameList {
        DebuggerStackFrameList {
            thread_id: self.thread_id,
            client_id: self.client_id.to_proto(),
            current_stack_frame: self.current_stack_frame_id,
            stack_frames: self.stack_frames.to_proto(),
        }
    }

    pub(crate) fn set_from_proto(
        &mut self,
        stack_frame_list: DebuggerStackFrameList,
        cx: &mut ViewContext<Self>,
    ) {
        self.thread_id = stack_frame_list.thread_id;
        self.client_id = DebugAdapterClientId::from_proto(stack_frame_list.client_id);
        self.current_stack_frame_id = stack_frame_list.current_stack_frame;
        self.stack_frames = Vec::from_proto(stack_frame_list.stack_frames);
        self.list.reset(self.stack_frames.len());

        cx.notify();
    }

    pub fn stack_frames(&self) -> &Vec<StackFrame> {
        &self.stack_frames
    }

    pub fn current_stack_frame_id(&self) -> u64 {
        self.current_stack_frame_id
    }

    fn handle_debug_panel_item_event(
        &mut self,
        _: View<DebugPanelItem>,
        event: &debugger_panel_item::DebugPanelItemEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            Stopped { go_to_stack_frame } => {
                self.fetch_stack_frames(*go_to_stack_frame, cx);
            }
            _ => {}
        }
    }

    pub fn invalidate(&mut self, cx: &mut ViewContext<Self>) {
        self.fetch_stack_frames(true, cx);
    }

    fn fetch_stack_frames(&mut self, go_to_stack_frame: bool, cx: &mut ViewContext<Self>) {
        let task = self.dap_store.update(cx, |store, cx| {
            store.stack_frames(&self.client_id, self.thread_id, cx)
        });

        self.fetch_stack_frames_task = Some(cx.spawn(|this, mut cx| async move {
            let mut stack_frames = task.await?;

            let task = this.update(&mut cx, |this, cx| {
                std::mem::swap(&mut this.stack_frames, &mut stack_frames);

                this.list.reset(this.stack_frames.len());

                cx.emit(StackFrameListEvent::StackFramesUpdated);

                let stack_frame = this
                    .stack_frames
                    .first()
                    .cloned()
                    .ok_or_else(|| anyhow!("No stack frame found to select"))?;

                anyhow::Ok(this.select_stack_frame(&stack_frame, go_to_stack_frame, cx))
            })?;

            task?.await?;

            this.update(&mut cx, |this, _| {
                this.fetch_stack_frames_task.take();
            })
        }));
    }

    pub fn select_stack_frame(
        &mut self,
        stack_frame: &StackFrame,
        go_to_stack_frame: bool,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.current_stack_frame_id = stack_frame.id;

        cx.emit(StackFrameListEvent::SelectedStackFrameChanged);
        cx.notify();

        if let Some((client, id)) = self.dap_store.read(cx).downstream_client() {
            let request = UpdateDebugAdapter {
                client_id: self.client_id.to_proto(),
                project_id: *id,
                thread_id: Some(self.thread_id),
                variant: Some(rpc::proto::update_debug_adapter::Variant::StackFrameList(
                    self.to_proto(),
                )),
            };

            client.send(request).log_err();
        }

        if !go_to_stack_frame {
            return Task::ready(Ok(()));
        };

        let row = (stack_frame.line.saturating_sub(1)) as u32;

        let Some(project_path) = self.project_path_from_stack_frame(&stack_frame, cx) else {
            return Task::ready(Err(anyhow!("Project path not found")));
        };

        cx.spawn({
            let client_id = self.client_id;
            let workspace = self.workspace.clone();
            move |this, mut cx| async move {
                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.open_path_preview(project_path.clone(), None, false, true, cx)
                    })?
                    .await?;

                this.update(&mut cx, |this, cx| {
                    this.dap_store.update(cx, |store, cx| {
                        store.set_active_debug_line(&client_id, &project_path, row, cx);
                    })
                })
            }
        })
    }

    pub fn project_path_from_stack_frame(
        &self,
        stack_frame: &StackFrame,
        cx: &mut ViewContext<Self>,
    ) -> Option<ProjectPath> {
        let path = stack_frame.source.as_ref().and_then(|s| s.path.as_ref())?;

        self.workspace
            .update(cx, |workspace, cx| {
                workspace.project().read_with(cx, |project, cx| {
                    project.project_path_for_absolute_path(&Path::new(path), cx)
                })
            })
            .ok()?
    }

    fn render_entry(&self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let stack_frame = &self.stack_frames[ix];

        let source = stack_frame.source.clone();
        let is_selected_frame = stack_frame.id == self.current_stack_frame_id;

        let formatted_path = format!(
            "{}:{}",
            source.clone().and_then(|s| s.name).unwrap_or_default(),
            stack_frame.line,
        );

        v_flex()
            .rounded_md()
            .w_full()
            .group("")
            .id(("stack-frame", stack_frame.id))
            .tooltip({
                let formatted_path = formatted_path.clone();
                move |cx| Tooltip::text(formatted_path.clone(), cx)
            })
            .p_1()
            .when(is_selected_frame, |this| {
                this.bg(cx.theme().colors().element_hover)
            })
            .on_click(cx.listener({
                let stack_frame = stack_frame.clone();
                move |this, _, cx| {
                    this.select_stack_frame(&stack_frame, true, cx)
                        .detach_and_log_err(cx);
                }
            }))
            .hover(|s| s.bg(cx.theme().colors().element_hover).cursor_pointer())
            .child(
                h_flex()
                    .gap_0p5()
                    .text_ui_sm(cx)
                    .child(stack_frame.name.clone())
                    .child(formatted_path),
            )
            .child(
                h_flex()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .when_some(source.and_then(|s| s.path), |this, path| this.child(path)),
            )
            .into_any()
    }
}

impl Render for StackFrameList {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
    }
}

impl FocusableView for StackFrameList {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<StackFrameListEvent> for StackFrameList {}
