use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle,
    SharedString, Window, actions, px,
};
use ui::{IconName, prelude::*};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use crate::outputs::ExecutionView;

actions!(
    repl,
    [
        /// Toggles the panel that lists every evaluation and its output.
        ToggleOutputPanel,
        /// Clears the evaluations listed in the REPL output panel.
        ClearOutputPanel
    ]
);

struct OutputEntry {
    kernel_name: SharedString,
    code: SharedString,
    execution_view: Entity<ExecutionView>,
}

pub struct ReplOutputPanel {
    entries: Vec<OutputEntry>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    position: DockPosition,
    is_zoomed: bool,
}

impl ReplOutputPanel {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            entries: Vec::new(),
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            position: DockPosition::Right,
            is_zoomed: false,
        }
    }

    pub fn register(workspace: &mut Workspace) {
        workspace.register_action(|workspace, _: &ToggleOutputPanel, window, cx| {
            Self::ensure(workspace, window, cx);
            workspace.toggle_panel_focus::<Self>(window, cx);
        });
        workspace.register_action(|workspace, _: &ClearOutputPanel, _window, cx| {
            if let Some(panel) = workspace.panel::<Self>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.entries.clear();
                    cx.notify();
                });
            }
        });
    }

    pub fn ensure(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        if let Some(panel) = workspace.panel::<Self>(cx) {
            return panel;
        }
        let panel = cx.new(Self::new);
        workspace.add_panel(panel.clone(), window, cx);
        panel
    }

    pub fn push(
        &mut self,
        kernel_name: SharedString,
        code: SharedString,
        execution_view: Entity<ExecutionView>,
        cx: &mut Context<Self>,
    ) {
        self.entries.push(OutputEntry {
            kernel_name,
            code,
            execution_view,
        });
        self.scroll_handle.scroll_to_bottom();
        cx.notify();
    }
}

impl EventEmitter<PanelEvent> for ReplOutputPanel {}

impl Focusable for ReplOutputPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for ReplOutputPanel {
    fn persistent_name() -> &'static str {
        "ReplOutputPanel"
    }

    fn panel_key() -> &'static str {
        "ReplOutputPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(420.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::ReplNeutral)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("REPL Output")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleOutputPanel)
    }

    fn activation_priority(&self) -> u32 {
        8
    }

    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        self.is_zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_zoomed = zoomed;
        cx.notify();
    }
}

impl Render for ReplOutputPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = if self.entries.is_empty() {
            div()
                .p_4()
                .child(
                    Label::new("Run code with repl: run to see its output here.")
                        .color(Color::Muted),
                )
                .into_any_element()
        } else {
            div()
                .flex()
                .flex_col()
                .children(self.entries.iter().map(|entry| {
                    v_flex()
                        .p_2()
                        .gap_1()
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant)
                        .child(
                            Label::new(entry.kernel_name.clone())
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(
                            div()
                                .font_buffer(cx)
                                .text_buffer(cx)
                                .text_color(cx.theme().colors().text_muted)
                                .child(entry.code.clone()),
                        )
                        .child(entry.execution_view.clone())
                }))
                .into_any_element()
        };

        v_flex()
            .id("repl-output-panel")
            .track_focus(&self.focus_handle)
            .size_full()
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new("REPL Output"))
                    .child(
                        IconButton::new("clear-repl-output", IconName::Trash)
                            .icon_size(IconSize::Small)
                            .tooltip(ui::Tooltip::text("Clear"))
                            .on_click(cx.listener(|panel, _, _, cx| {
                                panel.entries.clear();
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .id("repl-output-entries")
                    .flex_1()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(content),
            )
    }
}
