use crate::{
    DeploySearch, PanelHandle,
    dock::{Dock, PanelButtonLayout, render_panel_button},
    status_bar::{HideStatusItem, add_hide_button_entry},
    workspace_settings::ActivityBarSettings,
};
use gpui::{
    Anchor, App, Context, Decorations, Entity, IntoElement, ParentElement, Render, Styled,
    Subscription, Window, px,
};
use settings::{ActivityBarIconSize, Settings, SettingsStore};
use theme::CLIENT_SIDE_DECORATION_ROUNDING;
use ui::{ContextMenu, IconButton, IconName, IconSize, Tooltip, prelude::*, right_click_menu};

const ACTIVITY_BAR_WIDTH: Pixels = px(48.);

pub struct ActivityBar {
    left_dock: Entity<Dock>,
    right_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    _subscriptions: Vec<Subscription>,
}

impl ActivityBar {
    pub fn new(
        left_dock: Entity<Dock>,
        right_dock: Entity<Dock>,
        bottom_dock: Entity<Dock>,
        cx: &mut Context<Self>,
    ) -> Self {
        for dock in [&left_dock, &right_dock, &bottom_dock] {
            cx.observe(dock, |_, _, cx| cx.notify()).detach();
        }

        Self {
            left_dock,
            right_dock,
            bottom_dock,
            _subscriptions: vec![cx.observe_global::<SettingsStore>(|_, cx| cx.notify())],
        }
    }
}

impl Render for ActivityBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ActivityBarSettings::get_global(cx);
        let icon_size = match settings.icon_size {
            ActivityBarIconSize::Small => IconSize::Small,
            ActivityBarIconSize::Medium => IconSize::Medium,
        };

        let mut panel_targets: Vec<(u32, Entity<Dock>, usize, std::sync::Arc<dyn PanelHandle>)> =
            Vec::new();
        for dock in [&self.left_dock, &self.right_dock, &self.bottom_dock] {
            let dock_entity = dock.clone();
            for entry in dock.read(cx).panel_button_entries() {
                let priority = entry.panel.activation_priority(cx);
                panel_targets.push((
                    priority,
                    dock_entity.clone(),
                    entry.panel_index,
                    entry.panel,
                ));
            }
        }
        panel_targets.sort_by_key(|(priority, _, _, _)| *priority);

        let mut buttons: Vec<AnyElement> = panel_targets
            .into_iter()
            .filter_map(|(_, dock, panel_index, panel)| {
                render_panel_button(
                    dock,
                    panel_index,
                    panel,
                    PanelButtonLayout::Vertical,
                    icon_size,
                    window,
                    cx,
                )
                .map(|element| element.into_any_element())
            })
            .collect();

        if project_search_button_visible(cx) {
            buttons.push(render_project_search_button(icon_size).into_any_element());
        }

        let colors = cx.theme().colors();

        v_flex()
            .id("activity-bar")
            .when(!settings.enabled, |this| this.hidden())
            .flex_none()
            .w(ACTIVITY_BAR_WIDTH)
            .h_full()
            .py_1()
            .gap_0p5()
            .bg(colors.status_bar_background)
            .border_r_1()
            .border_color(colors.border)
            .map(|el| match window.window_decorations() {
                Decorations::Server => el,
                Decorations::Client { tiling, .. } => el.when(!tiling.left, |el| {
                    el.rounded_tl(CLIENT_SIDE_DECORATION_ROUNDING)
                        .rounded_bl(CLIENT_SIDE_DECORATION_ROUNDING)
                }),
            })
            .children(buttons)
    }
}

fn project_search_button_visible(cx: &App) -> bool {
    cx.global::<SettingsStore>()
        .merged_settings()
        .editor
        .search
        .as_ref()
        .and_then(|search| search.button)
        .unwrap_or(true)
}

fn render_project_search_button(icon_size: IconSize) -> impl IntoElement {
    let hide = HideStatusItem::new(|settings| {
        settings.editor.search.get_or_insert_default().button = Some(false);
    });

    right_click_menu("activity-bar-search-menu")
        .menu(move |window, cx| {
            let hide = hide.clone();
            ContextMenu::build(window, cx, move |menu, _, _| {
                add_hide_button_entry(menu, hide)
            })
        })
        .anchor(Anchor::RightCenter)
        .attach(Anchor::LeftCenter)
        .trigger(move |_active, _window, _cx| {
            IconButton::new("activity-bar-project-search", IconName::MagnifyingGlass)
                .icon_size(icon_size)
                .tooltip(|_, cx| {
                    Tooltip::for_action("Project Search", &DeploySearch::default(), cx)
                })
                .on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(DeploySearch::default()), cx);
                })
        })
}
