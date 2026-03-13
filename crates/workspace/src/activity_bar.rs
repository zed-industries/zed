use crate::dock::PanelHandle;
use crate::workspace_settings::{ActivityBarSide, WorkspaceSettings};
use crate::Workspace;
use fs::Fs;
use gpui::{Context, Entity, Render, SharedString, Subscription, WeakEntity, Window};
use settings::{Settings, SettingsStore, update_settings_file};
use std::collections::HashMap;
use std::sync::Arc;
use ui::prelude::*;
use ui::{ContextMenu, Icon, IconSize, Label, LabelSize, Tooltip, right_click_menu};

pub struct ActivityBar {
    workspace: WeakEntity<Workspace>,
    fs: Arc<dyn Fs>,
    _settings_subscription: Subscription,
    _workspace_subscription: Subscription,
}

impl ActivityBar {
    pub fn new(workspace: Entity<Workspace>, fs: Arc<dyn Fs>, cx: &mut Context<Self>) -> Self {
        let settings_subscription = cx.observe_global::<SettingsStore>(|_, cx| cx.notify());
        let workspace_subscription = cx.observe(&workspace, |_, _, cx| cx.notify());
        Self {
            workspace: workspace.downgrade(),
            fs,
            _settings_subscription: settings_subscription,
            _workspace_subscription: workspace_subscription,
        }
    }
}

impl Render for ActivityBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = WorkspaceSettings::get_global(cx);
        let pinned_panels = settings.activity_bar.panels.clone();
        let side = settings.activity_bar.side.clone();

        let Some(workspace) = self.workspace.upgrade() else {
            return div().v_flex().h_full();
        };

        let mut panel_map: HashMap<String, Arc<dyn PanelHandle>> = HashMap::default();

        let workspace_ref = workspace.read(cx);
        let left_dock = workspace_ref.left_dock().clone();
        let right_dock = workspace_ref.right_dock().clone();
        let bottom_dock = workspace_ref.bottom_dock().clone();
        for dock in [&left_dock, &right_dock, &bottom_dock] {
            let dock_ref = dock.read(cx);
            for panel in dock_ref.panels() {
                panel_map.insert(panel.persistent_name().to_string(), panel.clone());
            }
        }

        let buttons: Vec<_> = pinned_panels
            .iter()
            .filter_map(|name| {
                let panel = panel_map.get(name)?.clone();

                let icon = panel.icon(window, cx)?;
                let icon_tooltip = panel.icon_tooltip(window, cx)?;
                let action = panel.toggle_action(window, cx);

                let is_active = [&left_dock, &right_dock, &bottom_dock]
                    .iter()
                    .any(|dock| {
                        let dock_ref = dock.read(cx);
                        dock_ref.is_open()
                            && dock_ref
                                .active_panel()
                                .is_some_and(|p| p.persistent_name() == name.as_str())
                    });

                let icon_label = panel.icon_label(window, cx);
                let tooltip: SharedString = icon_tooltip.into();
                let name_for_id = SharedString::from(name.clone());
                let name_for_menu = name.clone();
                let badge_on_right = side == ActivityBarSide::Left;
                let fs = self.fs.clone();
                let action_for_tooltip = action.boxed_clone();

                Some(
                    right_click_menu(name_for_id.clone())
                        .menu(move |window, cx| {
                            let name_owned = name_for_menu.clone();
                            let fs = fs.clone();
                            ContextMenu::build(window, cx, move |menu, _, _| {
                                let name_owned = name_owned.clone();
                                let fs = fs.clone();
                                menu.entry(
                                    "Remove from Activity Bar",
                                    None,
                                    move |_window, cx| {
                                        let name_owned = name_owned.clone();
                                        update_settings_file(fs.clone(), cx, move |content, _| {
                                            if let Some(activity_bar) =
                                                content.workspace.activity_bar.as_mut()
                                            {
                                                if let Some(panels) =
                                                    activity_bar.panels.as_mut()
                                                {
                                                    panels.retain(|p| p != &name_owned);
                                                }
                                            }
                                        });
                                    },
                                )
                            })
                        })
                        .trigger(move |_is_menu_open, _window, cx| {
                            let active_bg = if is_active {
                                Some(cx.theme().colors().element_selected)
                            } else {
                                None
                            };
                            let hover_bg = cx.theme().colors().element_hover;

                            div()
                                .id(SharedString::from(format!("{}-btn", name_for_id)))
                                .relative()
                                .w(px(48.))
                                .h(px(48.))
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .when_some(active_bg, |this, bg| this.bg(bg))
                                .hover(move |this| this.bg(hover_bg))
                                .on_click({
                                    let action = action.boxed_clone();
                                    move |_, window: &mut Window, cx| {
                                        window.dispatch_action(action.boxed_clone(), cx);
                                    }
                                })
                                .tooltip({
                                    let tooltip = tooltip.clone();
                                    move |_window, cx| {
                                        Tooltip::for_action(tooltip.clone(), &*action_for_tooltip, cx)
                                    }
                                })
                                .child(
                                    Icon::new(icon)
                                        .size(IconSize::Medium)
                                        .color(if is_active {
                                            Color::Accent
                                        } else {
                                            Color::Muted
                                        }),
                                )
                                .when_some(icon_label.clone().filter(|_| !is_active), |this, label| {
                                    this.child(
                                        div()
                                            .absolute()
                                            .top(px(10.))
                                            .when(badge_on_right, |this| this.right(px(10.)))
                                            .when(!badge_on_right, |this| this.left(px(10.)))
                                            .min_w(rems_from_px(14.))
                                            .h(rems_from_px(14.))
                                            .px(px(2.))
                                            .rounded_full()
                                            .bg(cx.theme().colors().version_control_added)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(
                                                Label::new(label)
                                                    .size(LabelSize::XSmall)
                                                    .color(Color::Default),
                                            ),
                                    )
                                })
                        }),
                )
            })
            .collect();

        div()
            .v_flex()
            .h_full()
            .w(px(48.))
            .bg(cx.theme().colors().panel_background)
            .when(side == ActivityBarSide::Left, |this| {
                this.border_r_1()
                    .border_color(cx.theme().colors().border)
            })
            .when(side == ActivityBarSide::Right, |this| {
                this.border_l_1()
                    .border_color(cx.theme().colors().border)
            })
            .children(buttons)
    }
}
