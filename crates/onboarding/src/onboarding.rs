use crate::welcome::{ShowWelcome, WelcomePage};
use client::{Client, UserStore};
use command_palette_hooks::CommandPaletteFilter;
use db::kvp::KEY_VALUE_STORE;
use feature_flags::{FeatureFlag, FeatureFlagViewExt as _};
use fs::Fs;
use gpui::{
    Action, AnyElement, App, AppContext, AsyncWindowContext, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, SharedString, Subscription, Task, WeakEntity,
    Window, actions,
};
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{SettingsStore, VsCodeSettingsSource};
use std::sync::Arc;
use ui::{
    Avatar, FluentBuilder, Headline, KeyBinding, ParentElement as _, StatefulInteractiveElement,
    Vector, VectorName, prelude::*, rems_from_px,
};
use workspace::{
    AppState, Workspace, WorkspaceId,
    dock::DockPosition,
    item::{Item, ItemEvent},
    notifications::NotifyResultExt as _,
    open_new, with_active_or_new_workspace,
};

mod basics_page;
mod editing_page;
mod theme_preview;
mod welcome;

pub struct OnBoardingFeatureFlag {}

impl FeatureFlag for OnBoardingFeatureFlag {
    const NAME: &'static str = "onboarding";
}

/// Imports settings from Visual Studio Code.
#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct ImportVsCodeSettings {
    #[serde(default)]
    pub skip_prompt: bool,
}

/// Imports settings from Cursor editor.
#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct ImportCursorSettings {
    #[serde(default)]
    pub skip_prompt: bool,
}

pub const FIRST_OPEN: &str = "first_open";

actions!(
    zed,
    [
        /// Opens the onboarding view.
        OpenOnboarding
    ]
);

pub fn init(cx: &mut App) {
    cx.on_action(|_: &OpenOnboarding, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace
                .with_local_workspace(window, cx, |workspace, window, cx| {
                    let existing = workspace
                        .active_pane()
                        .read(cx)
                        .items()
                        .find_map(|item| item.downcast::<Onboarding>());

                    if let Some(existing) = existing {
                        workspace.activate_item(&existing, true, true, window, cx);
                    } else {
                        let settings_page = Onboarding::new(
                            workspace.weak_handle(),
                            workspace.user_store().clone(),
                            cx,
                        );
                        workspace.add_item_to_active_pane(
                            Box::new(settings_page),
                            None,
                            true,
                            window,
                            cx,
                        )
                    }
                })
                .detach();
        });
    });

    cx.on_action(|_: &ShowWelcome, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace
                .with_local_workspace(window, cx, |workspace, window, cx| {
                    let existing = workspace
                        .active_pane()
                        .read(cx)
                        .items()
                        .find_map(|item| item.downcast::<WelcomePage>());

                    if let Some(existing) = existing {
                        workspace.activate_item(&existing, true, true, window, cx);
                    } else {
                        let settings_page = WelcomePage::new(window, cx);
                        workspace.add_item_to_active_pane(
                            Box::new(settings_page),
                            None,
                            true,
                            window,
                            cx,
                        )
                    }
                })
                .detach();
        });
    });

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_workspace, action: &ImportVsCodeSettings, window, cx| {
            let fs = <dyn Fs>::global(cx);
            let action = *action;

            window
                .spawn(cx, async move |cx: &mut AsyncWindowContext| {
                    handle_import_vscode_settings(
                        VsCodeSettingsSource::VsCode,
                        action.skip_prompt,
                        fs,
                        cx,
                    )
                    .await
                })
                .detach();
        });

        workspace.register_action(|_workspace, action: &ImportCursorSettings, window, cx| {
            let fs = <dyn Fs>::global(cx);
            let action = *action;

            window
                .spawn(cx, async move |cx: &mut AsyncWindowContext| {
                    handle_import_vscode_settings(
                        VsCodeSettingsSource::Cursor,
                        action.skip_prompt,
                        fs,
                        cx,
                    )
                    .await
                })
                .detach();
        });
    })
    .detach();

    cx.observe_new::<Workspace>(|_, window, cx| {
        let Some(window) = window else {
            return;
        };

        let onboarding_actions = [
            std::any::TypeId::of::<OpenOnboarding>(),
            std::any::TypeId::of::<ShowWelcome>(),
        ];

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.hide_action_types(&onboarding_actions);
        });

        cx.observe_flag::<OnBoardingFeatureFlag, _>(window, move |is_enabled, _, _, cx| {
            if is_enabled {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.show_action_types(onboarding_actions.iter());
                });
            } else {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.hide_action_types(&onboarding_actions);
                });
            }
        })
        .detach();
    })
    .detach();
}

pub fn show_onboarding_view(app_state: Arc<AppState>, cx: &mut App) -> Task<anyhow::Result<()>> {
    open_new(
        Default::default(),
        app_state,
        cx,
        |workspace, window, cx| {
            {
                workspace.toggle_dock(DockPosition::Left, window, cx);
                let onboarding_page =
                    Onboarding::new(workspace.weak_handle(), workspace.user_store().clone(), cx);
                workspace.add_item_to_center(Box::new(onboarding_page.clone()), window, cx);

                window.focus(&onboarding_page.focus_handle(cx));

                cx.notify();
            };
            db::write_and_log(cx, || {
                KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
            });
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectedPage {
    Basics,
    Editing,
    AiSetup,
}

struct Onboarding {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    selected_page: SelectedPage,
    user_store: Entity<UserStore>,
    _settings_subscription: Subscription,
}

impl Onboarding {
    fn new(
        workspace: WeakEntity<Workspace>,
        user_store: Entity<UserStore>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            workspace,
            user_store,
            focus_handle: cx.focus_handle(),
            selected_page: SelectedPage::Basics,
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        })
    }

    fn render_nav_button(
        &mut self,
        page: SelectedPage,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let text = match page {
            SelectedPage::Basics => "Basics",
            SelectedPage::Editing => "Editing",
            SelectedPage::AiSetup => "AI Setup",
        };

        let binding = match page {
            SelectedPage::Basics => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-1").unwrap()], cx)
                    .map(|kb| kb.size(rems_from_px(12.)))
            }
            SelectedPage::Editing => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-2").unwrap()], cx)
                    .map(|kb| kb.size(rems_from_px(12.)))
            }
            SelectedPage::AiSetup => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-3").unwrap()], cx)
                    .map(|kb| kb.size(rems_from_px(12.)))
            }
        };

        let selected = self.selected_page == page;

        h_flex()
            .id(text)
            .relative()
            .w_full()
            .gap_2()
            .px_2()
            .py_0p5()
            .justify_between()
            .rounded_sm()
            .when(selected, |this| {
                this.child(
                    div()
                        .h_4()
                        .w_px()
                        .bg(cx.theme().colors().text_accent)
                        .absolute()
                        .left_0(),
                )
            })
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .child(Label::new(text).map(|this| {
                if selected {
                    this.color(Color::Default)
                } else {
                    this.color(Color::Muted)
                }
            }))
            .child(binding)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.selected_page = page;
                cx.notify();
            }))
    }

    fn render_nav(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(rems_from_px(220.))
            .flex_shrink_0()
            .gap_4()
            .justify_between()
            .child(
                v_flex()
                    .gap_6()
                    .child(
                        h_flex()
                            .px_2()
                            .gap_4()
                            .child(Vector::square(VectorName::ZedLogo, rems(2.5)))
                            .child(
                                v_flex()
                                    .child(
                                        Headline::new("Welcome to Zed").size(HeadlineSize::Small),
                                    )
                                    .child(
                                        Label::new("The editor for what's next")
                                            .color(Color::Muted)
                                            .size(LabelSize::Small)
                                            .italic(),
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_4()
                            .child(
                                v_flex()
                                    .py_4()
                                    .border_y_1()
                                    .border_color(cx.theme().colors().border_variant.opacity(0.5))
                                    .gap_1()
                                    .children([
                                        self.render_nav_button(SelectedPage::Basics, window, cx)
                                            .into_element(),
                                        self.render_nav_button(SelectedPage::Editing, window, cx)
                                            .into_element(),
                                        self.render_nav_button(SelectedPage::AiSetup, window, cx)
                                            .into_element(),
                                    ]),
                            )
                            .child(Button::new("skip_all", "Skip All")),
                    ),
            )
            .child(
                if let Some(user) = self.user_store.read(cx).current_user() {
                    h_flex()
                        .gap_2()
                        .child(Avatar::new(user.avatar_uri.clone()))
                        .child(Label::new(user.github_login.clone()))
                        .into_any_element()
                } else {
                    Button::new("sign_in", "Sign In")
                        .style(ButtonStyle::Outlined)
                        .full_width()
                        .on_click(|_, window, cx| {
                            let client = Client::global(cx);
                            window
                                .spawn(cx, async move |cx| {
                                    client
                                        .authenticate_and_connect(true, &cx)
                                        .await
                                        .into_response()
                                        .notify_async_err(cx);
                                })
                                .detach();
                        })
                        .into_any_element()
                },
            )
    }

    fn render_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.selected_page {
            SelectedPage::Basics => {
                crate::basics_page::render_basics_page(window, cx).into_any_element()
            }
            SelectedPage::Editing => {
                crate::editing_page::render_editing_page(window, cx).into_any_element()
            }
            SelectedPage::AiSetup => self.render_ai_setup_page(window, cx).into_any_element(),
        }
    }

    fn render_ai_setup_page(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child("ai setup page")
    }
}

impl Render for Onboarding {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .image_cache(gpui::retain_all("onboarding-page"))
            .key_context("onboarding-page")
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .max_w(rems_from_px(1100.))
                    .size_full()
                    .m_auto()
                    .py_20()
                    .px_12()
                    .items_start()
                    .gap_12()
                    .child(self.render_nav(window, cx))
                    .child(
                        div()
                            .pl_12()
                            .border_l_1()
                            .border_color(cx.theme().colors().border_variant.opacity(0.5))
                            .size_full()
                            .child(self.render_page(window, cx)),
                    ),
            )
    }
}

impl EventEmitter<ItemEvent> for Onboarding {}

impl Focusable for Onboarding {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for Onboarding {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Onboarding".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Onboarding Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        Some(Onboarding::new(
            self.workspace.clone(),
            self.user_store.clone(),
            cx,
        ))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

pub async fn handle_import_vscode_settings(
    source: VsCodeSettingsSource,
    skip_prompt: bool,
    fs: Arc<dyn Fs>,
    cx: &mut AsyncWindowContext,
) {
    use util::truncate_and_remove_front;

    let vscode_settings =
        match settings::VsCodeSettings::load_user_settings(source, fs.clone()).await {
            Ok(vscode_settings) => vscode_settings,
            Err(err) => {
                zlog::error!("{err}");
                let _ = cx.prompt(
                    gpui::PromptLevel::Info,
                    &format!("Could not find or load a {source} settings file"),
                    None,
                    &["Ok"],
                );
                return;
            }
        };

    if !skip_prompt {
        let prompt = cx.prompt(
            gpui::PromptLevel::Warning,
            &format!(
                "Importing {} settings may overwrite your existing settings. \
                Will import settings from {}",
                vscode_settings.source,
                truncate_and_remove_front(&vscode_settings.path.to_string_lossy(), 128),
            ),
            None,
            &["Ok", "Cancel"],
        );
        let result = cx.spawn(async move |_| prompt.await.ok()).await;
        if result != Some(0) {
            return;
        }
    };

    cx.update(|_, cx| {
        let source = vscode_settings.source;
        let path = vscode_settings.path.clone();
        cx.global::<SettingsStore>()
            .import_vscode_settings(fs, vscode_settings);
        zlog::info!("Imported {source} settings from {}", path.display());
    })
    .ok();
}
