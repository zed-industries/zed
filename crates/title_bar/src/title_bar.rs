mod application_menu;
mod collab;
mod onboarding_banner;
pub mod platform_title_bar;
mod platforms;
mod title_bar_settings;

#[cfg(feature = "stories")]
mod stories;

use crate::{
    application_menu::{ApplicationMenu, show_menus},
    platform_title_bar::PlatformTitleBar,
};

#[cfg(not(target_os = "macos"))]
use crate::application_menu::{
    ActivateDirection, ActivateMenuLeft, ActivateMenuRight, OpenApplicationMenu,
};

use auto_update::AutoUpdateStatus;
use call::ActiveCall;
use client::{Client, UserStore, zed_urls};
use cloud_llm_client::Plan;
use gpui::{
    Action, AnyElement, App, Context, Corner, Element, Entity, Focusable, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, StatefulInteractiveElement, Styled,
    Subscription, WeakEntity, Window, actions, div,
};
use onboarding_banner::OnboardingBanner;
use project::Project;
use settings::Settings as _;
use settings_ui::keybindings;
use std::sync::Arc;
use theme::ActiveTheme;
use title_bar_settings::TitleBarSettings;
use ui::{
    Avatar, Button, ButtonLike, ButtonStyle, Chip, ContextMenu, Icon, IconName, IconSize,
    IconWithIndicator, Indicator, PopoverMenu, PopoverMenuHandle, Tooltip, h_flex, prelude::*,
};
use util::ResultExt;
use workspace::{Workspace, notifications::NotifyResultExt};
use zed_actions::{OpenRecent, OpenRemote};

pub use onboarding_banner::restore_banner;

#[cfg(feature = "stories")]
pub use stories::*;

const MAX_PROJECT_NAME_LENGTH: usize = 40;
const MAX_BRANCH_NAME_LENGTH: usize = 40;
const MAX_SHORT_SHA_LENGTH: usize = 8;

actions!(
    collab,
    [
        /// Toggles the user menu dropdown.
        ToggleUserMenu,
        /// Toggles the project menu dropdown.
        ToggleProjectMenu,
        /// Switches to a different git branch.
        SwitchBranch
    ]
);

pub fn init(cx: &mut App) {
    TitleBarSettings::register(cx);

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let item = cx.new(|cx| TitleBar::new("title-bar", workspace, window, cx));
        workspace.set_titlebar_item(item.into(), window, cx);

        #[cfg(not(target_os = "macos"))]
        workspace.register_action(|workspace, action: &OpenApplicationMenu, window, cx| {
            if let Some(titlebar) = workspace
                .titlebar_item()
                .and_then(|item| item.downcast::<TitleBar>().ok())
            {
                titlebar.update(cx, |titlebar, cx| {
                    if let Some(ref menu) = titlebar.application_menu {
                        menu.update(cx, |menu, cx| menu.open_menu(action, window, cx));
                    }
                });
            }
        });

        #[cfg(not(target_os = "macos"))]
        workspace.register_action(|workspace, _: &ActivateMenuRight, window, cx| {
            if let Some(titlebar) = workspace
                .titlebar_item()
                .and_then(|item| item.downcast::<TitleBar>().ok())
            {
                titlebar.update(cx, |titlebar, cx| {
                    if let Some(ref menu) = titlebar.application_menu {
                        menu.update(cx, |menu, cx| {
                            menu.navigate_menus_in_direction(ActivateDirection::Right, window, cx)
                        });
                    }
                });
            }
        });

        #[cfg(not(target_os = "macos"))]
        workspace.register_action(|workspace, _: &ActivateMenuLeft, window, cx| {
            if let Some(titlebar) = workspace
                .titlebar_item()
                .and_then(|item| item.downcast::<TitleBar>().ok())
            {
                titlebar.update(cx, |titlebar, cx| {
                    if let Some(ref menu) = titlebar.application_menu {
                        menu.update(cx, |menu, cx| {
                            menu.navigate_menus_in_direction(ActivateDirection::Left, window, cx)
                        });
                    }
                });
            }
        });
    })
    .detach();
}

pub struct TitleBar {
    platform_titlebar: Entity<PlatformTitleBar>,
    project: Entity<Project>,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    workspace: WeakEntity<Workspace>,
    application_menu: Option<Entity<ApplicationMenu>>,
    _subscriptions: Vec<Subscription>,
    banner: Entity<OnboardingBanner>,
    screen_share_popover_handle: PopoverMenuHandle<ContextMenu>,
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title_bar_settings = *TitleBarSettings::get_global(cx);

        let show_menus = show_menus(cx);

        let mut children = Vec::new();

        children.push(
            h_flex()
                .gap_1()
                .map(|title_bar| {
                    let mut render_project_items = title_bar_settings.show_branch_name
                        || title_bar_settings.show_project_items;
                    title_bar
                        .when_some(
                            self.application_menu.clone().filter(|_| !show_menus),
                            |title_bar, menu| {
                                render_project_items &=
                                    !menu.update(cx, |menu, cx| menu.all_menus_shown(cx));
                                title_bar.child(menu)
                            },
                        )
                        .when(render_project_items, |title_bar| {
                            title_bar
                                .when(title_bar_settings.show_project_items, |title_bar| {
                                    title_bar
                                        .children(self.render_project_host(cx))
                                        .child(self.render_project_name(cx))
                                })
                                .when(title_bar_settings.show_branch_name, |title_bar| {
                                    title_bar.children(self.render_project_branch(cx))
                                })
                        })
                })
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .into_any_element(),
        );

        children.push(self.render_collaborator_list(window, cx).into_any_element());

        if title_bar_settings.show_onboarding_banner {
            children.push(self.banner.clone().into_any_element())
        }

        let status = self.client.status();
        let status = &*status.borrow();
        let user = self.user_store.read(cx).current_user();

        children.push(
            h_flex()
                .gap_1()
                .pr_1()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .children(self.render_call_controls(window, cx))
                .children(self.render_connection_status(status, cx))
                .when(
                    user.is_none() && TitleBarSettings::get_global(cx).show_sign_in,
                    |el| el.child(self.render_sign_in_button(cx)),
                )
                .when(user.is_some(), |parent| {
                    parent.child(self.render_user_menu_button(cx))
                })
                .into_any_element(),
        );

        if show_menus {
            self.platform_titlebar.update(cx, |this, _| {
                this.set_children(
                    self.application_menu
                        .clone()
                        .map(|menu| menu.into_any_element()),
                );
            });

            let height = PlatformTitleBar::height(window);
            let title_bar_color = self.platform_titlebar.update(cx, |platform_titlebar, cx| {
                platform_titlebar.title_bar_color(window, cx)
            });

            v_flex()
                .w_full()
                .child(self.platform_titlebar.clone().into_any_element())
                .child(
                    h_flex()
                        .bg(title_bar_color)
                        .h(height)
                        .pl_2()
                        .justify_between()
                        .w_full()
                        .children(children),
                )
                .into_any_element()
        } else {
            self.platform_titlebar.update(cx, |this, _| {
                this.set_children(children);
            });
            self.platform_titlebar.clone().into_any_element()
        }
    }
}

impl TitleBar {
    pub fn new(
        id: impl Into<ElementId>,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project = workspace.project().clone();
        let user_store = workspace.app_state().user_store.clone();
        let client = workspace.app_state().client.clone();
        let active_call = ActiveCall::global(cx);

        let platform_style = PlatformStyle::platform();
        let application_menu = match platform_style {
            PlatformStyle::Mac => {
                if option_env!("ZED_USE_CROSS_PLATFORM_MENU").is_some() {
                    Some(cx.new(|cx| ApplicationMenu::new(window, cx)))
                } else {
                    None
                }
            }
            PlatformStyle::Linux | PlatformStyle::Windows => {
                Some(cx.new(|cx| ApplicationMenu::new(window, cx)))
            }
        };

        let mut subscriptions = Vec::new();
        subscriptions.push(
            cx.observe(&workspace.weak_handle().upgrade().unwrap(), |_, _, cx| {
                cx.notify()
            }),
        );
        subscriptions.push(cx.subscribe(&project, |_, _, _: &project::Event, cx| cx.notify()));
        subscriptions.push(cx.observe(&active_call, |this, _, cx| this.active_call_changed(cx)));
        subscriptions.push(cx.observe_window_activation(window, Self::window_activation_changed));
        subscriptions.push(cx.observe(&user_store, |_, _, cx| cx.notify()));

        let banner = cx.new(|cx| {
            OnboardingBanner::new(
                "Debugger Onboarding",
                IconName::Debug,
                "The Debugger",
                None,
                zed_actions::debugger::OpenOnboardingModal.boxed_clone(),
                cx,
            )
        });

        let platform_titlebar = cx.new(|_| PlatformTitleBar::new(id));

        Self {
            platform_titlebar,
            application_menu,
            workspace: workspace.weak_handle(),
            project,
            user_store,
            client,
            _subscriptions: subscriptions,
            banner,
            screen_share_popover_handle: Default::default(),
        }
    }

    fn render_ssh_project_host(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let options = self.project.read(cx).ssh_connection_options(cx)?;
        let host: SharedString = options.connection_string().into();

        let nickname = options
            .nickname
            .map(|nick| nick.into())
            .unwrap_or_else(|| host.clone());

        let (indicator_color, meta) = match self.project.read(cx).ssh_connection_state(cx)? {
            remote::ConnectionState::Connecting => (Color::Info, format!("Connecting to: {host}")),
            remote::ConnectionState::Connected => (Color::Success, format!("Connected to: {host}")),
            remote::ConnectionState::HeartbeatMissed => (
                Color::Warning,
                format!("Connection attempt to {host} missed. Retrying..."),
            ),
            remote::ConnectionState::Reconnecting => (
                Color::Warning,
                format!("Lost connection to {host}. Reconnecting..."),
            ),
            remote::ConnectionState::Disconnected => {
                (Color::Error, format!("Disconnected from {host}"))
            }
        };

        let icon_color = match self.project.read(cx).ssh_connection_state(cx)? {
            remote::ConnectionState::Connecting => Color::Info,
            remote::ConnectionState::Connected => Color::Default,
            remote::ConnectionState::HeartbeatMissed => Color::Warning,
            remote::ConnectionState::Reconnecting => Color::Warning,
            remote::ConnectionState::Disconnected => Color::Error,
        };

        let meta = SharedString::from(meta);

        Some(
            ButtonLike::new("ssh-server-icon")
                .child(
                    h_flex()
                        .gap_2()
                        .max_w_32()
                        .child(
                            IconWithIndicator::new(
                                Icon::new(IconName::Server)
                                    .size(IconSize::Small)
                                    .color(icon_color),
                                Some(Indicator::dot().color(indicator_color)),
                            )
                            .indicator_border_color(Some(cx.theme().colors().title_bar_background))
                            .into_any_element(),
                        )
                        .child(Label::new(nickname).size(LabelSize::Small).truncate()),
                )
                .tooltip(move |window, cx| {
                    Tooltip::with_meta(
                        "Remote Project",
                        Some(&OpenRemote {
                            from_existing_connection: false,
                            create_new_window: false,
                        }),
                        meta.clone(),
                        window,
                        cx,
                    )
                })
                .on_click(|_, window, cx| {
                    window.dispatch_action(
                        OpenRemote {
                            from_existing_connection: false,
                            create_new_window: false,
                        }
                        .boxed_clone(),
                        cx,
                    );
                })
                .into_any_element(),
        )
    }

    pub fn render_project_host(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if self.project.read(cx).is_via_ssh() {
            return self.render_ssh_project_host(cx);
        }

        if self.project.read(cx).is_disconnected(cx) {
            return Some(
                Button::new("disconnected", "Disconnected")
                    .disabled(true)
                    .color(Color::Disabled)
                    .style(ButtonStyle::Subtle)
                    .label_size(LabelSize::Small)
                    .into_any_element(),
            );
        }

        let host = self.project.read(cx).host()?;
        let host_user = self.user_store.read(cx).get_cached_user(host.user_id)?;
        let participant_index = self
            .user_store
            .read(cx)
            .participant_indices()
            .get(&host_user.id)?;
        Some(
            Button::new("project_owner_trigger", host_user.github_login.clone())
                .color(Color::Player(participant_index.0))
                .style(ButtonStyle::Subtle)
                .label_size(LabelSize::Small)
                .tooltip(Tooltip::text(format!(
                    "{} is sharing this project. Click to follow.",
                    host_user.github_login
                )))
                .on_click({
                    let host_peer_id = host.peer_id;
                    cx.listener(move |this, _, window, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                workspace.follow(host_peer_id, window, cx);
                            })
                            .log_err();
                    })
                })
                .into_any_element(),
        )
    }

    pub fn render_project_name(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let name = {
            let mut names = self.project.read(cx).visible_worktrees(cx).map(|worktree| {
                let worktree = worktree.read(cx);
                worktree.root_name()
            });

            names.next()
        };
        let is_project_selected = name.is_some();
        let name = if let Some(name) = name {
            util::truncate_and_trailoff(name, MAX_PROJECT_NAME_LENGTH)
        } else {
            "Open recent project".to_string()
        };

        Button::new("project_name_trigger", name)
            .when(!is_project_selected, |b| b.color(Color::Muted))
            .style(ButtonStyle::Subtle)
            .label_size(LabelSize::Small)
            .tooltip(move |window, cx| {
                Tooltip::for_action(
                    "Recent Projects",
                    &zed_actions::OpenRecent {
                        create_new_window: false,
                    },
                    window,
                    cx,
                )
            })
            .on_click(cx.listener(move |_, _, window, cx| {
                window.dispatch_action(
                    OpenRecent {
                        create_new_window: false,
                    }
                    .boxed_clone(),
                    cx,
                );
            }))
    }

    pub fn render_project_branch(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let repository = self.project.read(cx).active_repository(cx)?;
        let workspace = self.workspace.upgrade()?;
        let branch_name = {
            let repo = repository.read(cx);
            repo.branch
                .as_ref()
                .map(|branch| branch.name())
                .map(|name| util::truncate_and_trailoff(name, MAX_BRANCH_NAME_LENGTH))
                .or_else(|| {
                    repo.head_commit.as_ref().map(|commit| {
                        commit
                            .sha
                            .chars()
                            .take(MAX_SHORT_SHA_LENGTH)
                            .collect::<String>()
                    })
                })
        }?;

        Some(
            Button::new("project_branch_trigger", branch_name)
                .color(Color::Muted)
                .style(ButtonStyle::Subtle)
                .label_size(LabelSize::Small)
                .tooltip(move |window, cx| {
                    Tooltip::with_meta(
                        "Recent Branches",
                        Some(&zed_actions::git::Branch),
                        "Local branches only",
                        window,
                        cx,
                    )
                })
                .on_click(move |_, window, cx| {
                    let _ = workspace.update(cx, |this, cx| {
                        window.focus(&this.active_pane().focus_handle(cx));
                        window.dispatch_action(zed_actions::git::Branch.boxed_clone(), cx);
                    });
                })
                .when(
                    TitleBarSettings::get_global(cx).show_branch_icon,
                    |branch_button| {
                        branch_button
                            .icon(IconName::GitBranch)
                            .icon_position(IconPosition::Start)
                            .icon_color(Color::Muted)
                            .icon_size(IconSize::Indicator)
                    },
                ),
        )
    }

    fn window_activation_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if window.is_window_active() {
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.set_location(Some(&self.project), cx))
                .detach_and_log_err(cx);
        } else if cx.active_window().is_none() {
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.set_location(None, cx))
                .detach_and_log_err(cx);
        }
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.update_active_view_for_followers(window, cx);
            })
            .ok();
    }

    fn active_call_changed(&mut self, cx: &mut Context<Self>) {
        cx.notify();
    }

    fn share_project(&mut self, cx: &mut Context<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.share_project(project, cx))
            .detach_and_log_err(cx);
    }

    fn unshare_project(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.unshare_project(project, cx))
            .log_err();
    }

    fn render_connection_status(
        &self,
        status: &client::Status,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        match status {
            client::Status::ConnectionError
            | client::Status::ConnectionLost
            | client::Status::Reauthenticating
            | client::Status::Reconnecting
            | client::Status::ReconnectionError { .. } => Some(
                div()
                    .id("disconnected")
                    .child(Icon::new(IconName::Disconnected).size(IconSize::Small))
                    .tooltip(Tooltip::text("Disconnected"))
                    .into_any_element(),
            ),
            client::Status::UpgradeRequired => {
                let auto_updater = auto_update::AutoUpdater::get(cx);
                let label = match auto_updater.map(|auto_update| auto_update.read(cx).status()) {
                    Some(AutoUpdateStatus::Updated { .. }) => "Please restart Zed to Collaborate",
                    Some(AutoUpdateStatus::Installing { .. })
                    | Some(AutoUpdateStatus::Downloading { .. })
                    | Some(AutoUpdateStatus::Checking) => "Updating...",
                    Some(AutoUpdateStatus::Idle) | Some(AutoUpdateStatus::Errored) | None => {
                        "Please update Zed to Collaborate"
                    }
                };

                Some(
                    Button::new("connection-status", label)
                        .label_size(LabelSize::Small)
                        .on_click(|_, window, cx| {
                            if let Some(auto_updater) = auto_update::AutoUpdater::get(cx)
                                && auto_updater.read(cx).status().is_updated()
                            {
                                workspace::reload(cx);
                                return;
                            }
                            auto_update::check(&Default::default(), window, cx);
                        })
                        .into_any_element(),
                )
            }
            _ => None,
        }
    }

    pub fn render_sign_in_button(&mut self, _: &mut Context<Self>) -> Button {
        let client = self.client.clone();
        Button::new("sign_in", "Sign in")
            .label_size(LabelSize::Small)
            .on_click(move |_, window, cx| {
                let client = client.clone();
                window
                    .spawn(cx, async move |cx| {
                        client
                            .sign_in_with_optional_connect(true, cx)
                            .await
                            .notify_async_err(cx);
                    })
                    .detach();
            })
    }

    pub fn render_user_menu_button(&mut self, cx: &mut Context<Self>) -> impl Element {
        let user_store = self.user_store.read(cx);
        if let Some(user) = user_store.current_user() {
            let has_subscription_period = user_store.subscription_period().is_some();
            let plan = user_store.plan().filter(|_| {
                // Since the user might be on the legacy free plan we filter based on whether we have a subscription period.
                has_subscription_period
            });

            let user_avatar = user.avatar_uri.clone();
            let free_chip_bg = cx
                .theme()
                .colors()
                .editor_background
                .opacity(0.5)
                .blend(cx.theme().colors().text_accent.opacity(0.05));

            let pro_chip_bg = cx
                .theme()
                .colors()
                .editor_background
                .opacity(0.5)
                .blend(cx.theme().colors().text_accent.opacity(0.2));

            PopoverMenu::new("user-menu")
                .anchor(Corner::TopRight)
                .menu(move |window, cx| {
                    ContextMenu::build(window, cx, |menu, _, _cx| {
                        let user_login = user.github_login.clone();

                        let (plan_name, label_color, bg_color) = match plan {
                            None | Some(Plan::ZedFree) => ("Free", Color::Default, free_chip_bg),
                            Some(Plan::ZedProTrial) => ("Pro Trial", Color::Accent, pro_chip_bg),
                            Some(Plan::ZedPro) => ("Pro", Color::Accent, pro_chip_bg),
                        };

                        menu.custom_entry(
                            move |_window, _cx| {
                                let user_login = user_login.clone();

                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .child(Label::new(user_login))
                                    .child(
                                        Chip::new(plan_name.to_string())
                                            .bg_color(bg_color)
                                            .label_color(label_color),
                                    )
                                    .into_any_element()
                            },
                            move |_, cx| {
                                cx.open_url(&zed_urls::account_url(cx));
                            },
                        )
                        .separator()
                        .action("Settings", zed_actions::OpenSettings.boxed_clone())
                        .action(
                            "Settings Profiles",
                            zed_actions::settings_profile_selector::Toggle.boxed_clone(),
                        )
                        .action("Key Bindings", Box::new(keybindings::OpenKeymapEditor))
                        .action(
                            "Themes…",
                            zed_actions::theme_selector::Toggle::default().boxed_clone(),
                        )
                        .action(
                            "Icon Themes…",
                            zed_actions::icon_theme_selector::Toggle::default().boxed_clone(),
                        )
                        .action(
                            "Extensions",
                            zed_actions::Extensions::default().boxed_clone(),
                        )
                        .separator()
                        .action("Sign Out", client::SignOut.boxed_clone())
                    })
                    .into()
                })
                .trigger_with_tooltip(
                    ButtonLike::new("user-menu")
                        .child(
                            h_flex()
                                .gap_0p5()
                                .children(
                                    TitleBarSettings::get_global(cx)
                                        .show_user_picture
                                        .then(|| Avatar::new(user_avatar)),
                                )
                                .child(
                                    Icon::new(IconName::ChevronDown)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                        .style(ButtonStyle::Subtle),
                    Tooltip::text("Toggle User Menu"),
                )
                .anchor(gpui::Corner::TopRight)
        } else {
            PopoverMenu::new("user-menu")
                .anchor(Corner::TopRight)
                .menu(|window, cx| {
                    ContextMenu::build(window, cx, |menu, _, _| {
                        menu.action("Settings", zed_actions::OpenSettings.boxed_clone())
                            .action(
                                "Settings Profiles",
                                zed_actions::settings_profile_selector::Toggle.boxed_clone(),
                            )
                            .action("Key Bindings", Box::new(keybindings::OpenKeymapEditor))
                            .action(
                                "Themes…",
                                zed_actions::theme_selector::Toggle::default().boxed_clone(),
                            )
                            .action(
                                "Icon Themes…",
                                zed_actions::icon_theme_selector::Toggle::default().boxed_clone(),
                            )
                            .action(
                                "Extensions",
                                zed_actions::Extensions::default().boxed_clone(),
                            )
                    })
                    .into()
                })
                .trigger_with_tooltip(
                    IconButton::new("user-menu", IconName::ChevronDown).icon_size(IconSize::Small),
                    Tooltip::text("Toggle User Menu"),
                )
        }
    }
}
