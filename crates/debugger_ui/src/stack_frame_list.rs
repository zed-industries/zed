use std::path::Path;

use anyhow::{anyhow, Result};
use dap::client::DebugAdapterClientId;
use dap::StackFrame;
use editor::Editor;
use gpui::{
    list, AnyElement, EventEmitter, FocusHandle, ListState, Subscription, Task, View, WeakView,
};
use gpui::{FocusableView, Model};
use project::dap_store::DapStore;
use project::ProjectPath;
use ui::ViewContext;
use ui::{prelude::*, Tooltip};
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

                let previous_stack_frame_id = this.current_stack_frame_id;
                if let Some(stack_frame) = this.stack_frames.first() {
                    this.current_stack_frame_id = stack_frame.id;

                    if previous_stack_frame_id != this.current_stack_frame_id {
                        cx.emit(StackFrameListEvent::SelectedStackFrameChanged);
                    }
                }

                this.list.reset(this.stack_frames.len());
                cx.notify();

                cx.emit(StackFrameListEvent::StackFramesUpdated);

                if go_to_stack_frame {
                    Some(this.go_to_stack_frame(cx))
                } else {
                    None
                }
            })?;

            if let Some(task) = task {
                task.await?;
            }

            this.update(&mut cx, |this, _| {
                this.fetch_stack_frames_task.take();
            })
        }));
    }

    pub fn go_to_stack_frame(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let stack_frame = self
            .stack_frames
            .iter()
            .find(|s| s.id == self.current_stack_frame_id)
            .cloned();

        let Some(stack_frame) = stack_frame else {
            return Task::ready(Ok(())); // this could never happen
        };

        let row = (stack_frame.line.saturating_sub(1)) as u32;
        let column = (stack_frame.column.saturating_sub(1)) as u32;

        let Some(project_path) = self.project_path_from_stack_frame(&stack_frame, cx) else {
            return Task::ready(Err(anyhow!("Project path not found")));
        };

        cx.spawn({
            let workspace = self.workspace.clone();
            move |this, mut cx| async move {
                let task = workspace.update(&mut cx, |workspace, cx| {
                    workspace.open_path_preview(project_path.clone(), None, false, true, cx)
                })?;

                let editor = task.await?.downcast::<Editor>().unwrap();

                this.update(&mut cx, |this, cx| {
                    this.dap_store.update(cx, |store, cx| {
                        store.set_active_debug_line(&project_path, row, column, cx);
                    })
                })?;

                workspace.update(&mut cx, |_, cx| {
                    editor.update(cx, |editor, cx| editor.go_to_active_debug_line(cx))
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
                let stack_frame_id = stack_frame.id;
                move |this, _, cx| {
                    this.current_stack_frame_id = stack_frame_id;

                    this.go_to_stack_frame(cx).detach_and_log_err(cx);

                    cx.notify();

                    cx.emit(StackFrameListEvent::SelectedStackFrameChanged);
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
