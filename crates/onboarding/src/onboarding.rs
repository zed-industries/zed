use crate::welcome::{ShowWelcome, WelcomePage};
use client::{Client, UserStore};
use command_palette_hooks::CommandPaletteFilter;
use db::kvp::KEY_VALUE_STORE;
use feature_flags::{FeatureFlag, FeatureFlagViewExt as _};
use fs::Fs;
use gpui::{
    Action, AnyElement, App, AppContext, AsyncWindowContext, Context, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, KeyContext, Render, SharedString, Subscription, Task,
    WeakEntity, Window, actions,
};
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{SettingsStore, VsCodeSettingsSource};
use std::sync::Arc;
use ui::{
    Avatar, ButtonLike, FluentBuilder, Headline, KeyBinding, ParentElement as _,
    StatefulInteractiveElement, Vector, VectorName, prelude::*, rems_from_px,
};
use workspace::{
    AppState, Workspace, WorkspaceId,
    dock::DockPosition,
    item::{Item, ItemEvent},
    notifications::NotifyResultExt as _,
    open_new, register_serializable_item, with_active_or_new_workspace,
};

mod ai_setup_page;
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

actions!(
    onboarding,
    [
        /// Activates the Basics page.
        ActivateBasicsPage,
        /// Activates the Editing page.
        ActivateEditingPage,
        /// Activates the AI Setup page.
        ActivateAISetupPage,
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
                        let settings_page = Onboarding::new(workspace, cx);
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
    register_serializable_item::<Onboarding>(cx);
}

pub fn show_onboarding_view(app_state: Arc<AppState>, cx: &mut App) -> Task<anyhow::Result<()>> {
    open_new(
        Default::default(),
        app_state,
        cx,
        |workspace, window, cx| {
            {
                workspace.toggle_dock(DockPosition::Left, window, cx);
                let onboarding_page = Onboarding::new(workspace, cx);
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
    fn new(workspace: &Workspace, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            workspace: workspace.weak_handle(),
            focus_handle: cx.focus_handle(),
            selected_page: SelectedPage::Basics,
            user_store: workspace.user_store().clone(),
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        })
    }

    fn set_page(&mut self, page: SelectedPage, cx: &mut Context<Self>) {
        self.selected_page = page;
        cx.notify();
        cx.emit(ItemEvent::UpdateTab);
    }

    fn render_nav_buttons(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> [impl IntoElement; 3] {
        let pages = [
            SelectedPage::Basics,
            SelectedPage::Editing,
            SelectedPage::AiSetup,
        ];

        let text = ["Basics", "Editing", "AI Setup"];

        let actions: [&dyn Action; 3] = [
            &ActivateBasicsPage,
            &ActivateEditingPage,
            &ActivateAISetupPage,
        ];

        let mut binding = actions.map(|action| {
            KeyBinding::for_action_in(action, &self.focus_handle, window, cx)
                .map(|kb| kb.size(rems_from_px(12.)))
        });

        pages.map(|page| {
            let i = page as usize;
            let selected = self.selected_page == page;
            h_flex()
                .id(text[i])
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
                .child(Label::new(text[i]).map(|this| {
                    if selected {
                        this.color(Color::Default)
                    } else {
                        this.color(Color::Muted)
                    }
                }))
                .child(binding[i].take().map_or(
                    gpui::Empty.into_any_element(),
                    IntoElement::into_any_element,
                ))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.set_page(page, cx);
                }))
        })
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
                                    .children(self.render_nav_buttons(window, cx)),
                            )
                            .child(
                                ButtonLike::new("skip_all")
                                    .child(Label::new("Skip All").ml_1())
                                    .on_click(|_, _, cx| {
                                        with_active_or_new_workspace(
                                            cx,
                                            |workspace, window, cx| {
                                                let Some((onboarding_id, onboarding_idx)) =
                                                    workspace
                                                        .active_pane()
                                                        .read(cx)
                                                        .items()
                                                        .enumerate()
                                                        .find_map(|(idx, item)| {
                                                            let _ =
                                                                item.downcast::<Onboarding>()?;
                                                            Some((item.item_id(), idx))
                                                        })
                                                else {
                                                    return;
                                                };

                                                workspace.active_pane().update(cx, |pane, cx| {
                                                    // Get the index here to get around the borrow checker
                                                    let idx = pane.items().enumerate().find_map(
                                                        |(idx, item)| {
                                                            let _ =
                                                                item.downcast::<WelcomePage>()?;
                                                            Some(idx)
                                                        },
                                                    );

                                                    if let Some(idx) = idx {
                                                        pane.activate_item(
                                                            idx, true, true, window, cx,
                                                        );
                                                    } else {
                                                        let item =
                                                            Box::new(WelcomePage::new(window, cx));
                                                        pane.add_item(
                                                            item,
                                                            true,
                                                            true,
                                                            Some(onboarding_idx),
                                                            window,
                                                            cx,
                                                        );
                                                    }

                                                    pane.remove_item(
                                                        onboarding_id,
                                                        false,
                                                        false,
                                                        window,
                                                        cx,
                                                    );
                                                });
                                            },
                                        );
                                    }),
                            ),
                    ),
            )
            .child(
                if let Some(user) = self.user_store.read(cx).current_user() {
                    h_flex()
                        .pl_1p5()
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
                                        .sign_in_with_optional_connect(true, &cx)
                                        .await
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
            SelectedPage::AiSetup => {
                crate::ai_setup_page::render_ai_setup_page(&self, window, cx).into_any_element()
            }
        }
    }
}

impl Render for Onboarding {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .image_cache(gpui::retain_all("onboarding-page"))
            .key_context({
                let mut ctx = KeyContext::new_with_defaults();
                ctx.add("Onboarding");
                ctx
            })
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(|this, _: &ActivateBasicsPage, _, cx| {
                this.set_page(SelectedPage::Basics, cx);
            }))
            .on_action(cx.listener(|this, _: &ActivateEditingPage, _, cx| {
                this.set_page(SelectedPage::Editing, cx);
            }))
            .on_action(cx.listener(|this, _: &ActivateAISetupPage, _, cx| {
                this.set_page(SelectedPage::AiSetup, cx);
            }))
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
                        v_flex()
                            .max_w_full()
                            .min_w_0()
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
        self.workspace
            .update(cx, |workspace, cx| Onboarding::new(workspace, cx))
            .ok()
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

impl workspace::SerializableItem for Onboarding {
    fn serialized_item_kind() -> &'static str {
        "OnboardingPage"
    }

    fn cleanup(
        workspace_id: workspace::WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "onboarding_pages",
            &persistence::ONBOARDING_PAGES,
            cx,
        )
    }

    fn deserialize(
        _project: Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            if let Some(page_number) =
                persistence::ONBOARDING_PAGES.get_onboarding_page(item_id, workspace_id)?
            {
                let page = match page_number {
                    0 => Some(SelectedPage::Basics),
                    1 => Some(SelectedPage::Editing),
                    2 => Some(SelectedPage::AiSetup),
                    _ => None,
                };
                workspace.update(cx, |workspace, cx| {
                    let onboarding_page = Onboarding::new(workspace, cx);
                    if let Some(page) = page {
                        zlog::info!("Onboarding page {page:?} loaded");
                        onboarding_page.update(cx, |onboarding_page, cx| {
                            onboarding_page.set_page(page, cx);
                        })
                    }
                    onboarding_page
                })
            } else {
                Err(anyhow::anyhow!("No onboarding page to deserialize"))
            }
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut ui::Context<Self>,
    ) -> Option<gpui::Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let page_number = self.selected_page as u16;
        Some(cx.background_spawn(async move {
            persistence::ONBOARDING_PAGES
                .save_onboarding_page(item_id, workspace_id, page_number)
                .await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        event == &ItemEvent::UpdateTab
    }
}

mod persistence {
    use db::{define_connection, query, sqlez_macros::sql};
    use workspace::WorkspaceDb;

    define_connection! {
        pub static ref ONBOARDING_PAGES: OnboardingPagesDb<WorkspaceDb> =
            &[
                sql!(
                    CREATE TABLE onboarding_pages (
                        workspace_id INTEGER,
                        item_id INTEGER UNIQUE,
                        page_number INTEGER,

                        PRIMARY KEY(workspace_id, item_id),
                        FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                        ON DELETE CASCADE
                    ) STRICT;
                ),
            ];
    }

    impl OnboardingPagesDb {
        query! {
            pub async fn save_onboarding_page(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId,
                page_number: u16
            ) -> Result<()> {
                INSERT OR REPLACE INTO onboarding_pages(item_id, workspace_id, page_number)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_onboarding_page(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<Option<u16>> {
                SELECT page_number
                FROM onboarding_pages
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
