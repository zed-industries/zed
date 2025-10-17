use agent::ContextServerRegistry;
use collections::HashMap;
use context_server::ContextServerId;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle, Window, prelude::*,
};
use ui::{Divider, DividerColor, Modal, ModalHeader, WithScrollbar, prelude::*};
use workspace::{ModalView, Workspace};

pub struct ConfigureContextServerToolsModal {
    context_server_id: ContextServerId,
    context_server_registry: Entity<ContextServerRegistry>,
    focus_handle: FocusHandle,
    expanded_tools: HashMap<SharedString, bool>,
    scroll_handle: ScrollHandle,
}

impl ConfigureContextServerToolsModal {
    fn new(
        context_server_id: ContextServerId,
        context_server_registry: Entity<ContextServerRegistry>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            context_server_id,
            context_server_registry,
            focus_handle: cx.focus_handle(),
            expanded_tools: HashMap::default(),
            scroll_handle: ScrollHandle::new(),
        }
    }

    pub fn toggle(
        context_server_id: ContextServerId,
        context_server_registry: Entity<ContextServerRegistry>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| {
            Self::new(context_server_id, context_server_registry, window, cx)
        });
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }

    fn render_modal_content(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let tools = self
            .context_server_registry
            .read(cx)
            .tools_for_server(&self.context_server_id)
            .collect::<Vec<_>>();

        div()
            .size_full()
            .pb_2()
            .child(
                v_flex()
                    .id("modal_content")
                    .px_2()
                    .gap_1()
                    .max_h_128()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(tools.iter().enumerate().flat_map(|(index, tool)| {
                        let tool_name = tool.name();
                        let is_expanded = self
                            .expanded_tools
                            .get(tool_name.as_ref())
                            .copied()
                            .unwrap_or(false);

                        let icon = if is_expanded {
                            IconName::ChevronUp
                        } else {
                            IconName::ChevronDown
                        };

                        let mut items = vec![
                            v_flex()
                                .child(
                                    h_flex()
                                        .id(SharedString::from(format!("tool-header-{}", index)))
                                        .py_1()
                                        .pl_1()
                                        .pr_2()
                                        .w_full()
                                        .justify_between()
                                        .rounded_sm()
                                        .hover(|s| s.bg(cx.theme().colors().element_hover))
                                        .child(
                                            Label::new(tool_name.clone())
                                                .buffer_font(cx)
                                                .size(LabelSize::Small),
                                        )
                                        .child(
                                            Icon::new(icon)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .on_click(cx.listener({
                                            move |this, _event, _window, _cx| {
                                                let current = this
                                                    .expanded_tools
                                                    .get(tool_name.as_ref())
                                                    .copied()
                                                    .unwrap_or(false);
                                                this.expanded_tools
                                                    .insert(tool_name.clone(), !current);
                                                _cx.notify();
                                            }
                                        })),
                                )
                                .when(is_expanded, |this| {
                                    this.child(
                                        Label::new(tool.description()).color(Color::Muted).mx_1(),
                                    )
                                })
                                .into_any_element(),
                        ];

                        if index < tools.len() - 1 {
                            items.push(
                                h_flex()
                                    .w_full()
                                    .child(Divider::horizontal().color(DividerColor::BorderVariant))
                                    .into_any_element(),
                            );
                        }

                        items
                    })),
            )
            .vertical_scrollbar_for(self.scroll_handle.clone(), window, cx)
            .into_any_element()
    }
}

impl ModalView for ConfigureContextServerToolsModal {}

impl Focusable for ConfigureContextServerToolsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ConfigureContextServerToolsModal {}

impl Render for ConfigureContextServerToolsModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("ContextServerToolsModal")
            .occlude()
            .elevation_3(cx)
            .w(rems(34.))
            .on_action(cx.listener(Self::cancel))
            .track_focus(&self.focus_handle)
            .child(
                Modal::new("configure-context-server-tools", None::<ScrollHandle>)
                    .header(
                        ModalHeader::new()
                            .headline(format!("Tools from {}", self.context_server_id.0))
                            .show_dismiss_button(true),
                    )
                    .child(self.render_modal_content(window, cx)),
            )
    }
}
