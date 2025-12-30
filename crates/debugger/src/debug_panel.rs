// crates/debugger/src/debug_panel.rs

use crate::session::{DebugSession, DebugSessionEvent, DebugSessionState};
use gpui::{
    div, px, rgb, AppContext, Element, EventEmitter, FocusHandle, FocusableView,
    InteractiveElement, IntoElement, Model, ParentElement, Render, Styled,
    Subscription, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use ui::{
    prelude::*, Button, ButtonCommon, Clickable, Icon, IconButton, IconName,
    IconSize, Label, ListItem, Tooltip,
};
use workspace::{Panel, PanelEvent, Workspace};

pub struct DebugPanel {
    workspace: WeakView<Workspace>,
    session: Option<Model<DebugSession>>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
    selected_frame_index: Option<usize>,
    expanded_variables: collections::HashSet<i64>,
}

impl DebugPanel {
    pub fn new(workspace: WeakView<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            workspace,
            session: None,
            focus_handle: cx.focus_handle(),
            _subscriptions: Vec::new(),
            selected_frame_index: None,
            expanded_variables: collections::HashSet::default(),
        }
    }

    pub fn set_session(
        &mut self,
        session: Model<DebugSession>,
        cx: &mut ViewContext<Self>,
    ) {
        let subscription = cx.subscribe(&session, Self::handle_session_event);
        self._subscriptions.push(subscription);
        self.session = Some(session);
        cx.notify();
    }

    fn handle_session_event(
        &mut self,
        _session: Model<DebugSession>,
        event: &DebugSessionEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            DebugSessionEvent::StateChanged(_)
            | DebugSessionEvent::VariablesUpdated
            | DebugSessionEvent::Stopped { .. } => {
                cx.notify();
            }
            _ => {}
        }
    }

    fn render_toolbar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let session = self.session.as_ref();
        let state = session.map(|s| s.read(cx).state.clone());
        let is_running = matches!(state, Some(DebugSessionState::Running));
        let is_stopped = matches!(state, Some(DebugSessionState::Stopped { .. }));

        div()
            .flex()
            .gap_1()
            .p_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                IconButton::new("continue", IconName::Play)
                    .icon_size(IconSize::Small)
                    .disabled(!is_stopped)
                    .tooltip(|cx| Tooltip::text("Continue (F5)", cx))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(session) = &this.session {
                            session.update(cx, |s, cx| s.continue_execution(cx));
                        }
                    })),
            )
            .child(
                IconButton::new("pause", IconName::Pause)
                    .icon_size(IconSize::Small)
                    .disabled(!is_running)
                    .tooltip(|cx| Tooltip::text("Pause (F6)", cx)),
            )
            .child(
                IconButton::new("step-over", IconName::ArrowRight)
                    .icon_size(IconSize::Small)
                    .disabled(!is_stopped)
                    .tooltip(|cx| Tooltip::text("Step Over (F10)", cx))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(session) = &this.session {
                            session.update(cx, |s, cx| s.step_over(cx));
                        }
                    })),
            )
            .child(
                IconButton::new("step-into", IconName::ArrowDown)
                    .icon_size(IconSize::Small)
                    .disabled(!is_stopped)
                    .tooltip(|cx| Tooltip::text("Step Into (F11)", cx))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(session) = &this.session {
                            session.update(cx, |s, cx| s.step_into(cx));
                        }
                    })),
            )
            .child(
                IconButton::new("step-out", IconName::ArrowUp)
                    .icon_size(IconSize::Small)
                    .disabled(!is_stopped)
                    .tooltip(|cx| Tooltip::text("Step Out (Shift+F11)", cx))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(session) = &this.session {
                            session.update(cx, |s, cx| s.step_out(cx));
                        }
                    })),
            )
            .child(
                IconButton::new("restart", IconName::RotateCw)
                    .icon_size(IconSize::Small)
                    .tooltip(|cx| Tooltip::text("Restart (Ctrl+Shift+F5)", cx)),
            )
            .child(
                IconButton::new("stop", IconName::Stop)
                    .icon_size(IconSize::Small)
                    .disabled(session.is_none())
                    .tooltip(|cx| Tooltip::text("Stop (Shift+F5)", cx)),
            )
    }

    fn render_call_stack(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let frames = self
            .session
            .as_ref()
            .and_then(|s| {
                let session = s.read(cx);
                session
                    .current_thread_id
                    .and_then(|tid| session.stack_frames.get(&tid).cloned())
            })
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().colors().surface_background)
                    .child(Label::new("Call Stack").size(LabelSize::Small).weight(FontWeight::BOLD)),
            )
            .children(frames.into_iter().enumerate().map(|(idx, frame)| {
                let is_selected = self.selected_frame_index == Some(idx);
                let source_info = frame
                    .source
                    .as_ref()
                    .and_then(|s| s.name.clone())
                    .unwrap_or_else(|| "unknown".to_string());

                ListItem::new(("frame", idx))
                    .selected(is_selected)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_0p5()
                            .child(Label::new(frame.name.clone()).size(LabelSize::Small))
                            .child(
                                Label::new(format!("{}:{}", source_info, frame.line))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    )
                    .on_click(cx.listener(move |this, _, cx| {
                        this.selected_frame_index = Some(idx);
                        cx.notify();
                    }))
            }))
    }

    fn render_variables(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let variables = self
            .session
            .as_ref()
            .map(|s| {
                let session = s.read(cx);
                // Get variables for the first scope of selected frame
                session
                    .variables
                    .values()
                    .next()
                    .cloned()
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().colors().surface_background)
                    .child(Label::new("Variables").size(LabelSize::Small).weight(FontWeight::BOLD)),
            )
            .children(variables.into_iter().map(|var| {
                let has_children = var.variables_reference > 0;
                let is_expanded = self.expanded_variables.contains(&var.variables_reference);

                ListItem::new(("var", var.name.clone()))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .when(has_children, |this| {
                                this.child(
                                    Icon::new(if is_expanded {
                                        IconName::ChevronDown
                                    } else {
                                        IconName::ChevronRight
                                    })
                                    .size(IconSize::XSmall),
                                )
                            })
                            .child(
                                Label::new(var.name.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Accent),
                            )
                            .child(Label::new(": ").size(LabelSize::Small))
                            .child(
                                Label::new(var.value.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Modified),
                            )
                            .when(var.type_.is_some(), |this| {
                                this.child(
                                    Label::new(format!(" ({})", var.type_.as_ref().unwrap()))
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                )
                            }),
                    )
            }))
    }

    fn render_watch_expressions(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let watches = self
            .session
            .as_ref()
            .map(|s| s.read(cx).watch_expressions.clone())
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().colors().surface_background)
                    .child(Label::new("Watch").size(LabelSize::Small).weight(FontWeight::BOLD))
                    .child(
                        IconButton::new("add-watch", IconName::Plus)
                            .icon_size(IconSize::XSmall)
                            .tooltip(|cx| Tooltip::text("Add Expression", cx)),
                    ),
            )
            .children(watches.into_iter().map(|watch| {
                ListItem::new(("watch", watch.id)).child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            Label::new(watch.expression.clone())
                                .size(LabelSize::Small)
                                .color(Color::Accent),
                        )
                        .child(Label::new(" = ").size(LabelSize::Small))
                        .child(if let Some(result) = watch.result {
                            Label::new(result)
                                .size(LabelSize::Small)
                                .color(Color::Modified)
                                .into_any_element()
                        } else if let Some(error) = watch.error {
                            Label::new(error)
                                .size(LabelSize::Small)
                                .color(Color::Error)
                                .into_any_element()
                        } else {
                            Label::new("evaluating...")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .into_any_element()
                        }),
                )
            }))
    }

    fn render_breakpoints(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let breakpoints = self
            .session
            .as_ref()
            .map(|s| s.read(cx).breakpoints.clone())
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().colors().surface_background)
                    .child(Label::new("Breakpoints").size(LabelSize::Small).weight(FontWeight::BOLD)),
            )
            .children(breakpoints.into_iter().flat_map(|(path, bps)| {
                let filename = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());

                bps.into_iter().map(move |bp| {
                    let fname = filename.clone();
                    ListItem::new(("bp", format!("{}:{}", fname, bp.line))).child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                Icon::new(if bp.verified {
                                    IconName::Circle
                                } else {
                                    IconName::CircleDashed
                                })
                                .size(IconSize::XSmall)
                                .color(if bp.verified {
                                    Color::Error
                                } else {
                                    Color::Muted
                                }),
                            )
                            .child(
                                Label::new(format!("{}:{}", fname, bp.line))
                                    .size(LabelSize::Small),
                            )
                            .when(bp.condition.is_some(), |this| {
                                this.child(
                                    Label::new(format!(" when {}", bp.condition.as_ref().unwrap()))
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                )
                            }),
                    )
                })
            }))
    }
}

impl FocusableView for DebugPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}

impl Panel for DebugPanel {
    fn persistent_name() -> &'static str {
        "DebugPanel"
    }

    fn position(&self, _cx: &WindowContext) -> workspace::dock::DockPosition {
        workspace::dock::DockPosition::Left
    }

    fn position_is_valid(&self, position: workspace::dock::DockPosition) -> bool {
        matches!(
            position,
            workspace::dock::DockPosition::Left | workspace::dock::DockPosition::Right
        )
    }

    fn set_position(&mut self, _: workspace::dock::DockPosition, _: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &WindowContext) -> ui::Pixels {
        px(320.)
    }

    fn set_size(&mut self, _: Option<ui::Pixels>, _: &mut ViewContext<Self>) {}

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        Some(IconName::Bug)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Debug Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(workspace::ToggleLeftDock)
    }
}

impl Render for DebugPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .child(self.render_toolbar(cx))
            .child(
                div()
                    .flex_1()
                    .overflow_y_scroll()
                    .child(self.render_call_stack(cx))
                    .child(self.render_variables(cx))
                    .child(self.render_watch_expressions(cx))
                    .child(self.render_breakpoints(cx)),
            )
    }
}