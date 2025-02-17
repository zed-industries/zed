use gpui::{App, FocusHandle, Focusable};
use ui::{
    div, h_flex, v_flex, Context, ContextMenu, DropdownMenu, Element, InteractiveElement,
    ParentElement, Render, Styled,
};

pub(super) struct InertState {
    focus_handle: FocusHandle,
}

impl InertState {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}
impl Focusable for InertState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InertState {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<'_, Self>,
    ) -> impl ui::IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_1()
            .child(h_flex().child(DropdownMenu::new(
                "dap-adapter-picker",
                "Select Debug Adapter",
                ContextMenu::build(window, cx, |this, _, _| {
                    this.entry("GDB", None, |_, _| {})
                        .entry("Delve", None, |_, _| {})
                        .entry("LLDB", None, |_, _| {})
                }),
            )))
    }
}
