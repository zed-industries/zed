use crate::{
    DeploySearch,
    dock::{
        Dock, PanelButtonLayout, panel_button_icon_size, render_panel_button,
        vertical_panel_button_container,
    },
    status_bar::{HideStatusItem, add_hide_button_entry},
    workspace_settings::ActivityBarSettings,
};
use gpui::{
    Anchor, AnyElement, App, Context, Decorations, Entity, Hsla, IntoElement, ParentElement,
    Pixels, Render, Styled, Subscription, Window, px,
};
use settings::{Settings, SettingsStore};
use theme::CLIENT_SIDE_DECORATION_ROUNDING;
use ui::{ContextMenu, IconButton, IconName, IconSize, Tooltip, prelude::*, right_click_menu};

const ACTIVITY_BAR_WIDTH: Pixels = px(48.);

pub const SEARCH_BUTTON_KEY: &str = "search";

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
        let icon_size = panel_button_icon_size(settings.icon_size);

        let mut items: Vec<(String, u32, AnyElement)> = Vec::new();
        for dock in [&self.left_dock, &self.right_dock, &self.bottom_dock] {
            let dock_entity = dock.clone();
            for entry in dock.read(cx).panel_button_entries() {
                let priority = entry.panel.activation_priority(cx);
                let key = entry.panel.panel_key().to_string();
                if !panel_button_shown_in_activity_bar(&key, &settings) {
                    continue;
                }
                if let Some(element) = render_panel_button(
                    dock_entity.clone(),
                    entry.panel_index,
                    entry.panel,
                    PanelButtonLayout::Vertical,
                    icon_size,
                    window,
                    cx,
                ) {
                    items.push((key, priority, element.into_any_element()));
                }
            }
        }

        if project_search_button_visible(cx)
            && panel_button_shown_in_activity_bar(SEARCH_BUTTON_KEY, &settings)
        {
            let active_border_color = cx.theme().colors().element_selected;
            items.push((
                SEARCH_BUTTON_KEY.to_string(),
                2,
                render_project_search_button(icon_size, active_border_color).into_any_element(),
            ));
        }

        sort_activity_bar_items(&mut items, &settings);

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
            .children(items.into_iter().map(|(_, _, element)| element))
    }
}

fn default_button_order() -> Vec<&'static str> {
    vec![
        "ProjectPanel",
        SEARCH_BUTTON_KEY,
        "GitPanel",
        "agent_panel",
        "TerminalPanel",
        "OutlinePanel",
        "CollaborationPanel",
        "DebugPanel",
    ]
}

fn sort_activity_bar_items(items: &mut [(String, u32, AnyElement)], settings: &ActivityBarSettings) {
    let order: Vec<String> = settings
        .button_order
        .clone()
        .unwrap_or_else(|| default_button_order().into_iter().map(str::to_string).collect());

    items.sort_by(|(left_key, left_priority, _), (right_key, right_priority, _)| {
        let left_index = order.iter().position(|key| key == left_key);
        let right_index = order.iter().position(|key| key == right_key);

        match (left_index, right_index) {
            (Some(left), Some(right)) => left.cmp(&right),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => left_priority.cmp(right_priority),
        }
    });
}

pub(crate) fn project_search_button_visible(cx: &App) -> bool {
    cx.global::<SettingsStore>()
        .merged_settings()
        .editor
        .search
        .as_ref()
        .and_then(|search| search.button)
        .unwrap_or(true)
}

pub fn activity_bar_hides_search_button(cx: &App) -> bool {
    let settings = ActivityBarSettings::get_global(cx);
    settings.enabled
        && project_search_button_visible(cx)
        && panel_button_shown_in_activity_bar(SEARCH_BUTTON_KEY, &settings)
}

pub(crate) fn status_bar_buttons_contains(settings: &ActivityBarSettings, key: &str) -> bool {
    settings
        .status_bar_buttons
        .as_ref()
        .is_some_and(|buttons| buttons.iter().any(|button| button == key))
}

pub(crate) fn panel_button_shown_in_activity_bar(panel_key: &str, settings: &ActivityBarSettings) -> bool {
    settings.enabled && !status_bar_buttons_contains(settings, panel_key)
}

fn render_project_search_button(icon_size: IconSize, active_border_color: Hsla) -> impl IntoElement {
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
            let button = IconButton::new("activity-bar-project-search", IconName::MagnifyingGlass)
                .icon_size(icon_size)
                .tooltip(|_, cx| {
                    Tooltip::for_action("Project Search", &DeploySearch::default(), cx)
                })
                .on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(DeploySearch::default()), cx);
                });

            vertical_panel_button_container(false, active_border_color, button)
        })
}
