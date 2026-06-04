use gpui::{
    App, Context, Empty, EventEmitter, IntoElement, ParentElement, Render, SharedString, Window,
};
use settings::Settings;
use ui::{Button, Indicator, Tooltip, prelude::*};
use util::{maybe, paths::PathStyle};

use crate::{
    HideStatusItem, StatusItemView, TabBarSettings, item::ItemHandle,
    workspace_settings::StatusBarSettings,
};

pub struct ActiveFileName {
    project_path: Option<SharedString>,
    full_path: Option<SharedString>,
    is_dirty: bool,
    has_conflict: bool,
}

impl ActiveFileName {
    pub fn new() -> Self {
        Self {
            project_path: None,
            full_path: None,
            is_dirty: false,
            has_conflict: false,
        }
    }
    fn render_file_indicator(&self) -> Option<Indicator> {
        maybe!({
            let indicator_color = match (self.has_conflict, self.is_dirty) {
                (true, _) => Color::Warning,
                (_, true) => Color::Accent,
                (false, false) => return None,
            };

            Some(Indicator::dot().color(indicator_color))
        })
    }
}
impl Render for ActiveFileName {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).show_active_file {
            return Empty.into_any_element();
        }

        let Some(project_path) = self.project_path.clone() else {
            return Empty.into_any_element();
        };

        let tooltip_text = self
            .full_path
            .clone()
            .unwrap_or_else(|| project_path.clone());

        h_flex()
            .children(if !TabBarSettings::get_global(cx).show {
                self.render_file_indicator()
            } else {
                None
            })
            .child(
                Button::new("active-file-name-button", project_path)
                    .label_size(LabelSize::Small)
                    .tooltip(Tooltip::text(tooltip_text)),
            )
            .into_any_element()
    }
}

impl EventEmitter<crate::ToolbarItemEvent> for ActiveFileName {}

impl StatusItemView for ActiveFileName {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(item) = active_pane_item {
            self.project_path = item
                .project_path(cx)
                .map(|path| path.path.display(PathStyle::local()).into_owned().into());
            self.full_path = item.tab_tooltip_text(cx);
            self.is_dirty = item.is_dirty(cx);
            self.has_conflict = item.has_conflict(cx);
        } else {
            self.project_path = None;
            self.full_path = None;
            self.is_dirty = false;
            self.has_conflict = false;
        }
        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings.status_bar.get_or_insert_default().show_active_file = Some(false);
        }))
    }
}
