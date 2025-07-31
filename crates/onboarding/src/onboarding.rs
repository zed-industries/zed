use crate::welcome::{ShowWelcome, WelcomePage};
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
use settings::{Settings, SettingsStore, VsCodeSettingsSource, update_settings_file};
use std::{alloc::System, sync::Arc};
use theme::{
    Appearance, ThemeMode, ThemeName, ThemeRegistry, ThemeSelection, ThemeSettings,
    ThemeSettingsContent,
};
use ui::{
    Divider, FluentBuilder, Headline, KeyBinding, ParentElement as _, StatefulInteractiveElement,
    ToggleButton, ToggleButtonGroup, ToggleButtonSimple, Vector, VectorName, prelude::*,
    rems_from_px,
};
use workspace::{
    AppState, Workspace, WorkspaceId,
    dock::DockPosition,
    item::{Item, ItemEvent},
    open_new, with_active_or_new_workspace,
};

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
                        let settings_page = Onboarding::new(workspace.weak_handle(), cx);
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
                let onboarding_page = Onboarding::new(workspace.weak_handle(), cx);
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
    _settings_subscription: Subscription,
}

impl Onboarding {
    fn new(workspace: WeakEntity<Workspace>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            workspace,
            focus_handle: cx.focus_handle(),
            selected_page: SelectedPage::Basics,
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        })
    }

    fn render_page_nav(
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
            }
            SelectedPage::Editing => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-2").unwrap()], cx)
            }
            SelectedPage::AiSetup => {
                KeyBinding::new(vec![gpui::Keystroke::parse("cmd-3").unwrap()], cx)
            }
        };
        let selected = self.selected_page == page;
        h_flex()
            .id(text)
            .rounded_sm()
            .child(text)
            .child(binding)
            .h_8()
            .gap_2()
            .px_2()
            .py_0p5()
            .w_full()
            .justify_between()
            .map(|this| {
                if selected {
                    this.bg(Color::Selected.color(cx))
                        .border_l_1()
                        .border_color(Color::Accent.color(cx))
                } else {
                    this.text_color(Color::Muted.color(cx))
                }
            })
            .hover(|style| {
                if selected {
                    style.bg(Color::Selected.color(cx).opacity(0.6))
                } else {
                    style.bg(Color::Selected.color(cx).opacity(0.3))
                }
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                this.selected_page = page;
                cx.notify();
            }))
    }

    fn render_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.selected_page {
            SelectedPage::Basics => self.render_basics_page(window, cx).into_any_element(),
            SelectedPage::Editing => {
                crate::editing_page::render_editing_page(window, cx).into_any_element()
            }
            SelectedPage::AiSetup => self.render_ai_setup_page(window, cx).into_any_element(),
        }
    }

    fn render_basics_page(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // todo! implementing the following logic:
        // - [x] when system toggled, either set to system, or dark/light based on system appearance
        // - [*] style system like group buttons
        // - [ ] When selecting in light or dark
        // - [ ] if system selected
        // - [ ]  -> light selects light theme
        // - [ ]  -> dark selects dark theme
        // - [ ] else:
        // - [ ]  -> light sets mode to light and sets light variant
        // - [ ]  -> dark sets mode to dark and sets dark variant
        // - [ ] abastract updates into function

        let theme_selection = ThemeSettings::get_global(cx).theme_selection.clone();
        let system_appearance = theme::SystemAppearance::global(cx);
        let appearance_state = window.use_state(cx, |_, cx| {
            theme_selection
                .as_ref()
                .and_then(|selection| selection.mode())
                .and_then(|mode| match mode {
                    ThemeMode::System => None,
                    ThemeMode::Light => Some(Appearance::Light),
                    ThemeMode::Dark => Some(Appearance::Dark),
                })
                .unwrap_or(*system_appearance)
        });
        let appearance = appearance_state.read(cx).clone();
        let theme_selection = theme_selection.unwrap_or_else(|| ThemeSelection::Dynamic {
            mode: match *system_appearance {
                Appearance::Light => ThemeMode::Light,
                Appearance::Dark => ThemeMode::Dark,
            },
            light: ThemeName("One Light".into()),
            dark: ThemeName("One Dark".into()),
        });
        let theme_registry = ThemeRegistry::global(cx);

        let current_theme_name = theme_selection.theme(appearance);
        let theme_mode = theme_selection.mode();

        let selected_index = match appearance {
            Appearance::Light => 0,
            Appearance::Dark => 1,
        };

        let theme_seed = 0xBEEF as f32;

        const LIGHT_THEMES: [&'static str; 3] = ["One Light", "Ayu Light", "Gruvbox Light"];
        const DARK_THEMES: [&'static str; 3] = ["One Dark", "Ayu Dark", "Gruvbox Dark"];

        let theme_names = match appearance {
            Appearance::Light => LIGHT_THEMES,
            Appearance::Dark => DARK_THEMES,
        };
        let themes = theme_names
            .map(|theme_name| theme_registry.get(theme_name))
            .map(Result::unwrap);

        let theme_previews = themes.map(|theme| {
            let is_selected = theme.name == current_theme_name;
            let name = theme.name.clone();
            v_flex()
                .id(name.clone())
                .on_click({
                    let theme_name = theme.name.clone();
                    let appearance = appearance.clone();
                    move |_, window, cx| {
                        let fs = <dyn Fs>::global(cx);
                        let theme_name = theme_name.clone();
                        update_settings_file::<ThemeSettings>(fs, cx, move |settings, cx| {
                            settings.set_theme(theme_name, appearance);
                        });
                    }
                })
                .flex_1()
                .child(theme_preview::ThemePreviewTile::new(
                    theme,
                    is_selected,
                    theme_seed,
                ))
                .child(
                    h_flex()
                        .justify_center()
                        .items_baseline()
                        .child(Label::new(name).color(Color::Muted)),
                )
        });

        return v_flex()
            .child(
                h_flex().justify_between().child(Label::new("Theme")).child(
                    h_flex()
                        .gap_2()
                        .child(
                            ToggleButtonGroup::single_row(
                                "theme-selector-onboarding",
                                [
                                    ToggleButtonSimple::new("Light", {
                                        let appearance_state = appearance_state.clone();
                                        move |_, _, cx| {
                                            write_appearance_change(
                                                &appearance_state,
                                                Appearance::Light,
                                                cx,
                                            );
                                        }
                                    }),
                                    ToggleButtonSimple::new("Dark", {
                                        let appearance_state = appearance_state.clone();
                                        move |_, _, cx| {
                                            write_appearance_change(
                                                &appearance_state,
                                                Appearance::Dark,
                                                cx,
                                            );
                                        }
                                    }),
                                ],
                            )
                            .selected_index(selected_index)
                            .style(ui::ToggleButtonGroupStyle::Outlined)
                            .button_width(rems_from_px(64.)),
                        )
                        .child(
                            ToggleButton::new("System", "System")
                                .style(ButtonStyle::Outlined)
                                .width(rems_from_px(64.).into())
                                .on_click({
                                    let theme = theme_selection.clone();
                                    move |_, _, cx| {
                                        toggle_system_theme_mode(theme.clone(), appearance, cx);
                                    }
                                }),
                        ),
                ),
            )
            .child(h_flex().justify_between().children(theme_previews));

        fn write_appearance_change(
            appearance_state: &Entity<Appearance>,
            new_appearance: Appearance,
            cx: &mut App,
        ) {
            appearance_state.update(cx, |appearance, cx| {
                *appearance = new_appearance;
            });
            let fs = <dyn Fs>::global(cx);

            update_settings_file::<ThemeSettings>(fs, cx, move |settings, cx| {
                if settings.theme.as_ref().and_then(ThemeSelection::mode) == Some(ThemeMode::System)
                {
                    return;
                }
                let new_mode = match new_appearance {
                    Appearance::Light => ThemeMode::Light,
                    Appearance::Dark => ThemeMode::Dark,
                };
                settings.set_mode(new_mode);
            });
        }

        fn toggle_system_theme_mode(
            theme_selection: ThemeSelection,
            appearance: Appearance,
            cx: &mut App,
        ) {
            let fs = <dyn Fs>::global(cx);

            update_settings_file::<ThemeSettings>(fs, cx, move |settings, cx| {
                settings.theme = Some(match theme_selection {
                    ThemeSelection::Static(theme_name) => ThemeSelection::Dynamic {
                        mode: ThemeMode::System,
                        light: theme_name.clone(),
                        dark: theme_name.clone(),
                    },
                    ThemeSelection::Dynamic { mode, light, dark } if mode == ThemeMode::System => {
                        let mode = match appearance {
                            Appearance::Light => ThemeMode::Light,
                            Appearance::Dark => ThemeMode::Dark,
                        };
                        ThemeSelection::Dynamic { mode, light, dark }
                    }

                    ThemeSelection::Dynamic {
                        mode: _,
                        light,
                        dark,
                    } => ThemeSelection::Dynamic {
                        mode: ThemeMode::System,
                        light,
                        dark,
                    },
                });
            });
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
            .px_24()
            .py_12()
            .items_start()
            .child(
                v_flex()
                    .w_1_3()
                    .h_full()
                    .child(
                        h_flex()
                            .pt_0p5()
                            .child(Vector::square(VectorName::ZedLogo, rems(2.)))
                            .child(
                                v_flex()
                                    .left_1()
                                    .items_center()
                                    .child(Headline::new("Welcome to Zed"))
                                    .child(
                                        Label::new("The editor for what's next")
                                            .color(Color::Muted)
                                            .italic(),
                                    ),
                            ),
                    )
                    .p_1()
                    .child(Divider::horizontal())
                    .child(
                        v_flex().gap_1().children([
                            self.render_page_nav(SelectedPage::Basics, window, cx)
                                .into_element(),
                            self.render_page_nav(SelectedPage::Editing, window, cx)
                                .into_element(),
                            self.render_page_nav(SelectedPage::AiSetup, window, cx)
                                .into_element(),
                        ]),
                    ),
            )
            .child(div().child(Divider::vertical()).h_full())
            .child(div().w_2_3().h_full().child(self.render_page(window, cx)))
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
        Some(Onboarding::new(self.workspace.clone(), cx))
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
