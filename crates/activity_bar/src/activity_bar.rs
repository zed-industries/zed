//! A persistent vertical icon rail at the left edge of the workspace that
//! toggles panels, in the spirit of VS Code's activity bar.

mod activity_bar_settings;

pub use activity_bar_settings::ActivityBarSettings;

use gpui::{
    Action, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render,
    Subscription, WeakEntity, Window, div,
};
use gpui::{DefiniteLength, Pixels};
use settings::{Settings as _, SettingsStore};
use theme::ActiveTheme as _;
use ui::{
    ButtonCommon as _, ButtonLike, Clickable as _, Color, DynamicSpacing, FixedWidth as _, Icon,
    IconName, IconSize, SharedString, Styled as _, Toggleable as _, Tooltip, h_flex, px,
    rems_from_px, v_flex,
};
use workspace::{Workspace, dock::Dock};

const RAIL_WIDTH: Pixels = px(44.);
/// Square, so the icons sit in an evenly padded target.
const BUTTON_SIZE: Pixels = px(36.);

/// One button on the activity bar, described statically so the bar does not
/// need to hold live panel handles.
struct ActivityBarEntry {
    /// Stable identity for the button element.
    id: &'static str,
    /// The panel this entry stands for, matching its `Panel::persistent_name()`.
    /// `None` for entries that open somewhere other than a dock, which
    /// therefore have no toggled-on state to reflect.
    panel_name: Option<&'static str>,
    icon: IconName,
    /// Optical size, in pixels. Zed's icons all share a 16x16 viewBox but not
    /// the same visual weight: the dense glyphs read larger than the airy
    /// outline ones at an identical box size, so the light ones are given a
    /// slightly bigger box to make the column look evenly sized.
    icon_size: f32,
    tooltip: SharedString,
    action: Box<dyn Action>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let workspace_handle = cx.entity().downgrade();
        let docks = workspace.all_docks().map(|dock| dock.clone());
        let activity_bar = cx.new(|cx| {
            ActivityBar::new(workspace_handle, ActivityBar::default_entries(), docks, cx)
        });
        workspace.set_activity_bar_item(activity_bar.into(), window, cx);
    })
    .detach();
}

pub struct ActivityBar {
    workspace: WeakEntity<Workspace>,
    entries: Vec<ActivityBarEntry>,
    _subscriptions: Vec<Subscription>,
}

impl ActivityBar {
    fn new(
        workspace: WeakEntity<Workspace>,
        entries: Vec<ActivityBarEntry>,
        docks: [Entity<Dock>; 3],
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = docks
            .iter()
            .map(|dock| cx.observe(dock, |_, _, cx| cx.notify()))
            .collect::<Vec<_>>();
        subscriptions.push(cx.observe_global::<SettingsStore>(|_, cx| cx.notify()));

        Self {
            workspace,
            entries,
            _subscriptions: subscriptions,
        }
    }

    /// Whether the panel a bar entry stands for is the visible panel of an
    /// open dock, regardless of which dock the panel is configured to use.
    fn is_entry_active(workspace: &Workspace, persistent_name: &str, cx: &App) -> bool {
        workspace.all_docks().iter().any(|dock| {
            let dock = dock.read(cx);
            dock.is_open()
                && dock
                    .visible_panel()
                    .is_some_and(|panel| panel.persistent_name() == persistent_name)
        })
    }

    fn default_entries() -> Vec<ActivityBarEntry> {
        vec![
            ActivityBarEntry {
                id: "project-panel",
                panel_name: Some("Project Panel"),
                icon: IconName::FileTree,
                icon_size: 21.,
                tooltip: "Project Panel".into(),
                action: Box::new(zed_actions::project_panel::ToggleFocus),
            },
            ActivityBarEntry {
                id: "worktree-panel",
                panel_name: Some("WorktreePanel"),
                icon: IconName::GitWorktree,
                icon_size: 21.,
                tooltip: "Worktree Panel".into(),
                action: Box::new(zed_actions::worktree_panel::ToggleFocus),
            },
            ActivityBarEntry {
                id: "github-issues-panel",
                panel_name: Some("GithubIssuesPanel"),
                icon: IconName::Github,
                icon_size: 20.,
                tooltip: "GitHub Issues".into(),
                action: Box::new(zed_actions::github_issues_panel::ToggleFocus),
            },
            ActivityBarEntry {
                id: "project-search",
                panel_name: None,
                icon: IconName::MagnifyingGlass,
                icon_size: 22.,
                tooltip: "Search Project".into(),
                action: Box::new(workspace::DeploySearch::default()),
            },
            ActivityBarEntry {
                id: "terminal-panel",
                panel_name: Some("TerminalPanel"),
                icon: IconName::TerminalAlt,
                // A closed rectangle reads larger than the open glyphs around
                // it, so it gets a smaller box than its neighbours.
                icon_size: 19.,
                tooltip: "Terminal Panel".into(),
                action: Box::new(terminal_view::terminal_panel::Toggle),
            },
            ActivityBarEntry {
                id: "agent-panel",
                panel_name: Some("AgentPanel"),
                icon: IconName::ZedAssistant,
                icon_size: 20.,
                tooltip: "Agent Panel".into(),
                action: Box::new(zed_actions::assistant::ToggleFocus),
            },
        ]
    }
}

impl Render for ActivityBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !ActivityBarSettings::get_global(cx).show {
            return div().into_any_element();
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return div().into_any_element();
        };
        let workspace = workspace.read(cx);
        let colors = cx.theme().colors();

        v_flex()
            .h_full()
            .flex_none()
            .w(RAIL_WIDTH)
            .items_center()
            .py(DynamicSpacing::Base08.rems(cx))
            .gap(DynamicSpacing::Base04.rems(cx))
            .bg(colors
                .title_bar_background
                .blend(colors.panel_background.opacity(0.25)))
            .border_r_1()
            .border_t_1()
            .border_b_1()
            .border_color(colors.border)
            .children(self.entries.iter().map(|entry| {
                let is_active = entry
                    .panel_name
                    .is_some_and(|name| Self::is_entry_active(workspace, name, cx));
                let tooltip = entry.tooltip.clone();
                let action = entry.action.boxed_clone();
                // The active state is part of the element ID so that the cached
                // tooltip is invalidated when the panel is toggled elsewhere.
                ButtonLike::new((entry.id, is_active as u64))
                    .toggle_state(is_active)
                    .width(DefiniteLength::from(BUTTON_SIZE))
                    .height(DefiniteLength::from(BUTTON_SIZE))
                    .tab_index(0isize)
                    .aria_label(tooltip.clone())
                    .child(
                        h_flex().w_full().justify_center().child(
                            Icon::new(entry.icon)
                                .size(IconSize::Custom(rems_from_px(entry.icon_size)))
                                .color(if is_active {
                                    Color::Selected
                                } else {
                                    Color::Muted
                                }),
                        ),
                    )
                    .on_click({
                        let action = action.boxed_clone();
                        move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx)
                    })
                    .tooltip(move |_window, cx| Tooltip::for_action(tooltip.clone(), &*action, cx))
            }))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{Entity, TestAppContext, UpdateGlobal as _, VisualTestContext};
    use project::Project;
    use settings::SettingsStore;
    use std::{cell::Cell, rc::Rc};
    use theme::LoadThemes;
    use workspace::MultiWorkspace;
    use workspace::dock::DockPosition;
    use workspace::dock::test::{TestPanel, ToggleTestPanel};

    fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(LoadThemes::JustBase, cx);
        });
    }

    async fn setup_workspace(
        cx: &mut TestAppContext,
    ) -> (Entity<Workspace>, &mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        (workspace, cx)
    }

    /// The outer acceptance test: a bar button toggles its panel, and the
    /// button's active state follows the panel both ways.
    #[gpui::test]
    async fn test_activity_bar_toggles_panel(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| crate::init(cx));
        let (workspace, cx) = setup_workspace(cx).await;

        let activity_bar = workspace.read_with(cx, |workspace, _| {
            workspace
                .activity_bar_item()
                .and_then(|item| item.downcast::<ActivityBar>().ok())
                .expect("activity bar should be registered")
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| TestPanel::new(DockPosition::Right, 0, cx));
            workspace.add_panel(panel, window, cx);
            workspace.register_action(|workspace, _: &ToggleTestPanel, window, cx| {
                workspace.toggle_panel_focus::<TestPanel>(window, cx);
            });
            cx.focus_self(window);
        });
        cx.run_until_parked();

        let toggle_action =
            activity_bar.read_with(cx, |bar, _| bar.entries[0].action.boxed_clone());
        assert!(!entry_active(&workspace, "TestPanel", cx));

        let notified = observe_notifications(&activity_bar, cx);

        cx.dispatch_action(ToggleTestPanel);
        cx.run_until_parked();
        assert!(
            entry_active(&workspace, "TestPanel", cx),
            "toggling should open the dock and activate the entry"
        );
        assert!(
            notified.get(),
            "the bar should re-render when the dock changes"
        );

        // Toggling follows the panel's own semantics: the dock only closes
        // again when the user asked for that.
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.workspace.close_panel_on_toggle = Some(true);
                });
            });
        });
        cx.dispatch_action(ToggleTestPanel);
        cx.run_until_parked();
        assert!(
            !entry_active(&workspace, "TestPanel", cx),
            "toggling again should close the dock and deactivate the entry"
        );

        // The default entries dispatch real panel actions, not the test one.
        assert!(!toggle_action.partial_eq(&ToggleTestPanel));
    }

    fn entry_active(
        workspace: &Entity<Workspace>,
        persistent_name: &str,
        cx: &mut VisualTestContext,
    ) -> bool {
        workspace.read_with(cx, |workspace, cx| {
            ActivityBar::is_entry_active(workspace, persistent_name, cx)
        })
    }

    fn observe_notifications(
        activity_bar: &Entity<ActivityBar>,
        cx: &mut VisualTestContext,
    ) -> Rc<Cell<bool>> {
        let notified = Rc::new(Cell::new(false));
        cx.update(|_, cx| {
            cx.observe(activity_bar, {
                let notified = notified.clone();
                move |_, _| notified.set(true)
            })
            .detach();
        });
        notified
    }

    #[gpui::test]
    async fn test_bar_registers_on_new_workspaces(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| crate::init(cx));
        let (workspace, cx) = setup_workspace(cx).await;
        workspace.read_with(cx, |workspace, _| {
            let item = workspace
                .activity_bar_item()
                .expect("activity bar should register itself on new workspaces");
            assert!(item.downcast::<ActivityBar>().is_ok());
        });
    }

    #[gpui::test]
    async fn test_entry_active_follows_dock_visibility(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, cx) = setup_workspace(cx).await;

        workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| TestPanel::new(DockPosition::Right, 0, cx));
            workspace.add_panel(panel, window, cx);
        });

        workspace.read_with(cx, |workspace, cx| {
            assert!(
                !ActivityBar::is_entry_active(workspace, "TestPanel", cx),
                "panel should be inactive while its dock is closed"
            );
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.open_panel::<TestPanel>(window, cx);
        });
        workspace.read_with(cx, |workspace, cx| {
            assert!(
                ActivityBar::is_entry_active(workspace, "TestPanel", cx),
                "panel should be active when visible in an open dock"
            );
            assert!(
                !ActivityBar::is_entry_active(workspace, "Project Panel", cx),
                "entries for other panels should stay inactive"
            );
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Right, window, cx);
        });
        workspace.read_with(cx, |workspace, cx| {
            assert!(
                !ActivityBar::is_entry_active(workspace, "TestPanel", cx),
                "closing the dock should deactivate the entry"
            );
        });
    }

    #[test]
    fn test_default_entries() {
        let entries = ActivityBar::default_entries();
        let described: Vec<_> = entries
            .iter()
            .map(|entry| (entry.panel_name, entry.icon, entry.tooltip.as_ref()))
            .collect();
        assert_eq!(
            described,
            vec![
                (
                    Some("Project Panel"),
                    ui::IconName::FileTree,
                    "Project Panel"
                ),
                (
                    Some("WorktreePanel"),
                    ui::IconName::GitWorktree,
                    "Worktree Panel"
                ),
                // Project search opens in the center pane, so it has no panel
                // to reflect an active state from.
                (None, ui::IconName::MagnifyingGlass, "Search Project"),
                (
                    Some("TerminalPanel"),
                    ui::IconName::TerminalAlt,
                    "Terminal Panel"
                ),
                (
                    Some("AgentPanel"),
                    ui::IconName::ZedAssistant,
                    "Agent Panel"
                ),
            ]
        );
        assert!(
            entries[0]
                .action
                .partial_eq(&zed_actions::project_panel::ToggleFocus)
        );
        assert!(
            entries[1]
                .action
                .partial_eq(&zed_actions::worktree_panel::ToggleFocus)
        );
        assert!(
            entries[2]
                .action
                .partial_eq(&workspace::DeploySearch::default())
        );
        assert!(
            entries[3]
                .action
                .partial_eq(&terminal_view::terminal_panel::Toggle)
        );
        assert!(
            entries[4]
                .action
                .partial_eq(&zed_actions::assistant::ToggleFocus)
        );
    }

    #[gpui::test]
    fn test_settings_resolve_from_defaults_and_user_override(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            assert!(
                ActivityBarSettings::get_global(cx).show,
                "activity bar should be shown by default"
            );

            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.activity_bar.get_or_insert_default().show = Some(false);
                });
            });
            assert!(
                !ActivityBarSettings::get_global(cx).show,
                "user setting should hide the activity bar"
            );
        });
    }
}
