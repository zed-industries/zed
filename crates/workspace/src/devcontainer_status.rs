use gpui::{App, Context, EventEmitter, FocusHandle, Focusable, Render, Window};
use ui::{prelude::*, Button, ButtonStyle, Color, IconName, Tooltip};

use crate::{ItemHandle, StatusItemView};

pub struct DevcontainerStatusView {
    pub container_id: Option<String>,
    pub container_name: Option<String>,
    pub is_connected: bool,
    focus_handle: FocusHandle,
}

#[derive(Debug, Clone)]
pub struct DevcontainerStatus {
    pub container_id: String,
    pub container_name: String,
    pub is_connected: bool,
}

impl EventEmitter<()> for DevcontainerStatusView {}
impl Focusable for DevcontainerStatusView {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl DevcontainerStatusView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            container_id: None,
            container_name: None,
            is_connected: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_devcontainer_status(&mut self, status: Option<DevcontainerStatus>, cx: &mut Context<Self>) {
        if let Some(status) = status {
            self.container_id = Some(status.container_id);
            self.container_name = Some(status.container_name);
            self.is_connected = status.is_connected;
        } else {
            self.container_id = None;
            self.container_name = None;
            self.is_connected = false;
        }
        cx.notify();
    }

    fn render_devcontainer_button(&self, _cx: &mut Context<Self>) -> Button {
        let container_name = self.container_name.as_deref().unwrap_or("devcontainer").to_string();
        let tooltip_text = if self.is_connected {
            format!("Connected to devcontainer: {}", container_name)
        } else {
            format!("Devcontainer available: {}", container_name)
        };

        Button::new("devcontainer-status", container_name)
            .icon(IconName::Server)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .color(if self.is_connected { Color::Success } else { Color::Muted })
            .tooltip(Tooltip::text(tooltip_text.clone()))
    }
}

impl StatusItemView for DevcontainerStatusView {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // This status item doesn't change based on active pane item
    }
}

impl Render for DevcontainerStatusView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.container_id.is_some() {
            div().child(self.render_devcontainer_button(cx))
        } else {
            div() // Empty div when no devcontainer
        }
    }
} 