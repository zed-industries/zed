use gpui::{
    Context, EventEmitter, Render, Styled as _, WeakEntity, Window, prelude::*,
};
use ui::{ButtonCommon, Color, Divider, IconButton, IconName, IconSize, Tooltip, prelude::*};
use workspace::{
    ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
};
use zed_actions::notebook::{
    AddCodeBlock, AddMarkdownBlock, ClearOutputs, DeleteCell, InterruptKernel, RestartKernel,
    RunAll,
};

use super::{NotebookEditor, NotebookToolbarState};
use crate::kernels::KernelStatus;

/// Top toolbar shown above a notebook (between the tab bar and the cells).
///
/// Mirrors VSCode's notebook toolbar: cell-creation, run-all, clear-outputs,
/// restart/interrupt kernel, and the kernel name + status. Hides itself when
/// the active pane item is not a [`NotebookEditor`].
pub struct NotebookToolbar {
    notebook: Option<WeakEntity<NotebookEditor>>,
    _notebook_subscription: Option<gpui::Subscription>,
}

impl Default for NotebookToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl NotebookToolbar {
    pub fn new() -> Self {
        Self {
            notebook: None,
            _notebook_subscription: None,
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for NotebookToolbar {}

impl ToolbarItemView for NotebookToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        let Some(item) = active_pane_item else {
            self.notebook = None;
            self._notebook_subscription = None;
            return ToolbarItemLocation::Hidden;
        };

        if let Some(notebook) = item.act_as::<NotebookEditor>(cx) {
            self.notebook = Some(notebook.downgrade());
            let toolbar_id = cx.entity_id();
            self._notebook_subscription = Some(item.subscribe_to_item_events(
                window,
                cx,
                Box::new(move |_event, _window, cx| cx.notify(toolbar_id)),
            ));
            ToolbarItemLocation::PrimaryLeft
        } else {
            self.notebook = None;
            self._notebook_subscription = None;
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for NotebookToolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(notebook) = self
            .notebook
            .as_ref()
            .and_then(|notebook| notebook.upgrade())
        else {
            return div();
        };

        let state = notebook.read(cx).toolbar_state(cx);
        self.render_contents(state, cx)
    }
}

impl NotebookToolbar {
    fn render_contents(
        &self,
        state: NotebookToolbarState,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let kernel_busy = matches!(state.kernel_status, KernelStatus::Busy);
        let kernel_unavailable = matches!(
            state.kernel_status,
            KernelStatus::Shutdown | KernelStatus::Error | KernelStatus::ShuttingDown
        );
        let can_delete_cell = state.cell_count > 1;

        h_flex()
            .w_full()
            .py_1()
            .px_2()
            .gap_1()
            .items_center()
            .child(
                IconButton::new("notebook-add-code", IconName::Code)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Add code cell", &AddCodeBlock, cx))
                    .on_click(cx.listener(|_this, _, window, cx| {
                        window.dispatch_action(Box::new(AddCodeBlock), cx);
                    })),
            )
            .child(
                IconButton::new("notebook-add-md", IconName::Plus)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Add markdown cell", &AddMarkdownBlock, cx)
                    })
                    .on_click(cx.listener(|_this, _, window, cx| {
                        window.dispatch_action(Box::new(AddMarkdownBlock), cx);
                    })),
            )
            .child(Divider::vertical())
            .child(
                IconButton::new("notebook-run-all", IconName::PlayFilled)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Run all cells", &RunAll, cx))
                    .on_click(cx.listener(|_this, _, window, cx| {
                        window.dispatch_action(Box::new(RunAll), cx);
                    })),
            )
            .child(
                IconButton::new("notebook-clear", IconName::ListX)
                    .icon_size(IconSize::Small)
                    .disabled(!state.has_outputs)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Clear outputs of all cells", &ClearOutputs, cx)
                    })
                    .on_click(cx.listener(|_this, _, window, cx| {
                        window.dispatch_action(Box::new(ClearOutputs), cx);
                    })),
            )
            .child(Divider::vertical())
            .child(
                IconButton::new("notebook-restart", IconName::RotateCw)
                    .icon_size(IconSize::Small)
                    .disabled(kernel_unavailable)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Restart kernel", &RestartKernel, cx)
                    })
                    .on_click(cx.listener(|_this, _, window, cx| {
                        window.dispatch_action(Box::new(RestartKernel), cx);
                    })),
            )
            .child(
                IconButton::new("notebook-interrupt", IconName::Stop)
                    .icon_size(IconSize::Small)
                    .disabled(!kernel_busy)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Interrupt kernel", &InterruptKernel, cx)
                    })
                    .on_click(cx.listener(|_this, _, window, cx| {
                        window.dispatch_action(Box::new(InterruptKernel), cx);
                    })),
            )
            .child(Divider::vertical())
            .child(
                IconButton::new("notebook-delete-cell", IconName::Trash)
                    .icon_size(IconSize::Small)
                    .disabled(!can_delete_cell)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Delete cell", &DeleteCell, cx)
                    })
                    .on_click(cx.listener(|_this, _, window, cx| {
                        window.dispatch_action(Box::new(DeleteCell), cx);
                    })),
            )
            .child(div().flex_1())
            .child(
                ui::Label::new(format!("Kernel: {}", state.kernel_name))
                    .size(LabelSize::Small)
                    .color(if kernel_unavailable {
                        Color::Muted
                    } else {
                        Color::Default
                    }),
            )
    }
}
