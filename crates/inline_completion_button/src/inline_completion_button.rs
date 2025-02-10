use anyhow::Result;
use client::UserStore;
use copilot::{Copilot, Status};
use editor::{
    actions::{ShowEditPrediction, ToggleEditPrediction},
    scroll::Autoscroll,
    Editor,
};
use feature_flags::{
    FeatureFlagAppExt, PredictEditsFeatureFlag, PredictEditsRateCompletionsFeatureFlag,
};
use fs::Fs;
use gpui::{
    actions, div, pulsating_between, Action, Animation, AnimationExt, App, AsyncWindowContext,
    Corner, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render, Subscription,
    WeakEntity,
};
use indoc::indoc;
use language::{
    language_settings::{self, all_language_settings, AllLanguageSettings, EditPredictionProvider},
    File, Language,
};
use regex::Regex;
use settings::{update_settings_file, Settings, SettingsStore};
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use supermaven::{AccountStatus, Supermaven};
use ui::{
    prelude::*, Clickable, ContextMenu, ContextMenuEntry, IconButton, IconButtonShape, Indicator,
    PopoverMenu, PopoverMenuHandle, Tooltip,
};
use workspace::{
    create_and_open_local_file, item::ItemHandle, notifications::NotificationId, StatusItemView,
    Toast, Workspace,
};
use zed_actions::OpenBrowser;
use zeta::RateCompletionModal;

actions!(zeta, [RateCompletions]);
actions!(edit_prediction, [ToggleMenu]);

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";

struct CopilotErrorToast;

pub struct InlineCompletionButton {
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    editor_show_predictions: bool,
    editor_focus_handle: Option<FocusHandle>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
    edit_prediction_provider: Option<Arc<dyn inline_completion::InlineCompletionProviderHandle>>,
    fs: Arc<dyn Fs>,
    workspace: WeakEntity<Workspace>,
    user_store: Entity<UserStore>,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
}

enum SupermavenButtonStatus {
    Ready,
    Errored(String),
    NeedsActivation(String),
    Initializing,
}

impl Render for InlineCompletionButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_language_settings = all_language_settings(None, cx);

        match all_language_settings.edit_predictions.provider {
            EditPredictionProvider::None => div(),

            EditPredictionProvider::Copilot => {
                let Some(copilot) = Copilot::global(cx) else {
                    return div();
                };
                let status = copilot.read(cx).status();

                let enabled = self.editor_enabled.unwrap_or(false);

                let icon = match status {
                    Status::Error(_) => IconName::CopilotError,
                    Status::Authorized => {
                        if enabled {
                            IconName::Copilot
                        } else {
                            IconName::CopilotDisabled
                        }
                    }
                    _ => IconName::CopilotInit,
                };

                if let Status::Error(e) = status {
                    return div().child(
                        IconButton::new("copilot-error", icon)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(move |_, _, window, cx| {
                                if let Some(workspace) = window.root::<Workspace>().flatten() {
                                    workspace.update(cx, |workspace, cx| {
                                        workspace.show_toast(
                                            Toast::new(
                                                NotificationId::unique::<CopilotErrorToast>(),
                                                format!("Copilot can't be started: {}", e),
                                            )
                                            .on_click(
                                                "Reinstall Copilot",
                                                |_, cx| {
                                                    if let Some(copilot) = Copilot::global(cx) {
                                                        copilot
                                                            .update(cx, |copilot, cx| {
                                                                copilot.reinstall(cx)
                                                            })
                                                            .detach();
                                                    }
                                                },
                                            ),
                                            cx,
                                        );
                                    });
                                }
                            }))
                            .tooltip(|window, cx| {
                                Tooltip::for_action("GitHub Copilot", &ToggleMenu, window, cx)
                            }),
                    );
                }
                let this = cx.entity().clone();

                div().child(
                    PopoverMenu::new("copilot")
                        .menu(move |window, cx| {
                            Some(match status {
                                Status::Authorized => this.update(cx, |this, cx| {
                                    this.build_copilot_context_menu(window, cx)
                                }),
                                _ => this.update(cx, |this, cx| {
                                    this.build_copilot_start_menu(window, cx)
                                }),
                            })
                        })
                        .anchor(Corner::BottomRight)
                        .trigger_with_tooltip(
                            IconButton::new("copilot-icon", icon),
                            |window, cx| {
                                Tooltip::for_action("GitHub Copilot", &ToggleMenu, window, cx)
                            },
                        )
                        .with_handle(self.popover_menu_handle.clone()),
                )
            }

            EditPredictionProvider::Supermaven => {
                let Some(supermaven) = Supermaven::global(cx) else {
                    return div();
                };

                let supermaven = supermaven.read(cx);

                let status = match supermaven {
                    Supermaven::Starting => SupermavenButtonStatus::Initializing,
                    Supermaven::FailedDownload { error } => {
                        SupermavenButtonStatus::Errored(error.to_string())
                    }
                    Supermaven::Spawned(agent) => {
                        let account_status = agent.account_status.clone();
                        match account_status {
                            AccountStatus::NeedsActivation { activate_url } => {
                                SupermavenButtonStatus::NeedsActivation(activate_url.clone())
                            }
                            AccountStatus::Unknown => SupermavenButtonStatus::Initializing,
                            AccountStatus::Ready => SupermavenButtonStatus::Ready,
                        }
                    }
                    Supermaven::Error { error } => {
                        SupermavenButtonStatus::Errored(error.to_string())
                    }
                };

                let icon = status.to_icon();
                let tooltip_text = status.to_tooltip();
                let has_menu = status.has_menu();
                let this = cx.entity().clone();
                let fs = self.fs.clone();

                return div().child(
                    PopoverMenu::new("supermaven")
                        .menu(move |window, cx| match &status {
                            SupermavenButtonStatus::NeedsActivation(activate_url) => {
                                Some(ContextMenu::build(window, cx, |menu, _, _| {
                                    let fs = fs.clone();
                                    let activate_url = activate_url.clone();
                                    menu.entry("Sign In", None, move |_, cx| {
                                        cx.open_url(activate_url.as_str())
                                    })
                                    .entry(
                                        "Use Copilot",
                                        None,
                                        move |_, cx| {
                                            set_completion_provider(
                                                fs.clone(),
                                                cx,
                                                EditPredictionProvider::Copilot,
                                            )
                                        },
                                    )
                                }))
                            }
                            SupermavenButtonStatus::Ready => Some(this.update(cx, |this, cx| {
                                this.build_supermaven_context_menu(window, cx)
                            })),
                            _ => None,
                        })
                        .anchor(Corner::BottomRight)
                        .trigger_with_tooltip(
                            IconButton::new("supermaven-icon", icon),
                            move |window, cx| {
                                if has_menu {
                                    Tooltip::for_action(
                                        tooltip_text.clone(),
                                        &ToggleMenu,
                                        window,
                                        cx,
                                    )
                                } else {
                                    Tooltip::text(tooltip_text.clone())(window, cx)
                                }
                            },
                        )
                        .with_handle(self.popover_menu_handle.clone()),
                );
            }

            EditPredictionProvider::Zed => {
                if !cx.has_flag::<PredictEditsFeatureFlag>() {
                    return div();
                }

                let enabled = self.editor_enabled.unwrap_or(true);

                let zeta_icon = if enabled {
                    IconName::ZedPredict
                } else {
                    IconName::ZedPredictDisabled
                };

                let current_user_terms_accepted =
                    self.user_store.read(cx).current_user_has_accepted_terms();

                if !current_user_terms_accepted.unwrap_or(false) {
                    let signed_in = current_user_terms_accepted.is_some();
                    let tooltip_meta = if signed_in {
                        "Read Terms of Service"
                    } else {
                        "Sign in to use"
                    };

                    return div().child(
                        IconButton::new("zed-predict-pending-button", zeta_icon)
                            .shape(IconButtonShape::Square)
                            .indicator(Indicator::dot().color(Color::Error))
                            .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                            .tooltip(move |window, cx| {
                                Tooltip::with_meta(
                                    "Edit Predictions",
                                    None,
                                    tooltip_meta,
                                    window,
                                    cx,
                                )
                            })
                            .on_click(cx.listener(move |_, _, window, cx| {
                                telemetry::event!(
                                    "Pending ToS Clicked",
                                    source = "Edit Prediction Status Button"
                                );
                                window.dispatch_action(
                                    zed_actions::OpenZedPredictOnboarding.boxed_clone(),
                                    cx,
                                );
                            })),
                    );
                }

                let show_editor_predictions = self.editor_show_predictions;

                let icon_button = IconButton::new("zed-predict-pending-button", zeta_icon)
                    .shape(IconButtonShape::Square)
                    .when(enabled && !show_editor_predictions, |this| {
                        this.indicator(Indicator::dot().color(Color::Muted))
                            .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                    });

                let this = cx.entity().clone();

                let mut popover_menu = PopoverMenu::new("zeta")
                    .menu(move |window, cx| {
                        Some(this.update(cx, |this, cx| this.build_zeta_context_menu(window, cx)))
                    })
                    .anchor(Corner::BottomRight)
                    .with_handle(self.popover_menu_handle.clone());

                let is_refreshing = self
                    .edit_prediction_provider
                    .as_ref()
                    .map_or(false, |provider| provider.is_refreshing(cx));

                if is_refreshing {
                    popover_menu = popover_menu.trigger(
                        icon_button.with_animation(
                            "pulsating-label",
                            Animation::new(Duration::from_secs(2))
                                .repeat()
                                .with_easing(pulsating_between(0.2, 1.0)),
                            |icon_button, delta| icon_button.alpha(delta),
                        ),
                    );
                } else {
                    popover_menu = popover_menu.trigger(icon_button);
                }

                div().child(popover_menu.into_any_element())
            }
        }
    }
}

impl InlineCompletionButton {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        fs: Arc<dyn Fs>,
        user_store: Entity<UserStore>,
        popover_menu_handle: PopoverMenuHandle<ContextMenu>,
        cx: &mut Context<Self>,
    ) -> Self {
        if let Some(copilot) = Copilot::global(cx) {
            cx.observe(&copilot, |_, _, cx| cx.notify()).detach()
        }

        cx.observe_global::<SettingsStore>(move |_, cx| cx.notify())
            .detach();

        Self {
            editor_subscription: None,
            editor_enabled: None,
            editor_show_predictions: true,
            editor_focus_handle: None,
            language: None,
            file: None,
            edit_prediction_provider: None,
            popover_menu_handle,
            workspace,
            fs,
            user_store,
        }
    }

    pub fn build_copilot_start_menu(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let fs = self.fs.clone();
        ContextMenu::build(window, cx, |menu, _, _| {
            menu.entry("Sign In", None, copilot::initiate_sign_in)
                .entry("Disable Copilot", None, {
                    let fs = fs.clone();
                    move |_window, cx| hide_copilot(fs.clone(), cx)
                })
                .entry("Use Supermaven", None, {
                    let fs = fs.clone();
                    move |_window, cx| {
                        set_completion_provider(fs.clone(), cx, EditPredictionProvider::Supermaven)
                    }
                })
        })
    }

    pub fn build_language_settings_menu(&self, mut menu: ContextMenu, cx: &mut App) -> ContextMenu {
        let fs = self.fs.clone();

        menu = menu.header("Show Edit Predictions For");

        if let Some(editor_focus_handle) = self.editor_focus_handle.clone() {
            menu = menu.toggleable_entry(
                "This File",
                self.editor_show_predictions,
                IconPosition::Start,
                Some(Box::new(ToggleEditPrediction)),
                {
                    let editor_focus_handle = editor_focus_handle.clone();
                    move |window, cx| {
                        editor_focus_handle.dispatch_action(&ToggleEditPrediction, window, cx);
                    }
                },
            );
        }

        if let Some(language) = self.language.clone() {
            let fs = fs.clone();
            let language_enabled =
                language_settings::language_settings(Some(language.name()), None, cx)
                    .show_edit_predictions;

            menu = menu.toggleable_entry(
                language.name(),
                language_enabled,
                IconPosition::Start,
                None,
                move |_, cx| {
                    toggle_show_inline_completions_for_language(language.clone(), fs.clone(), cx)
                },
            );
        }

        let settings = AllLanguageSettings::get_global(cx);
        let globally_enabled = settings.show_inline_completions(None, cx);
        menu = menu.toggleable_entry(
            "All Files",
            globally_enabled,
            IconPosition::Start,
            None,
            move |_, cx| toggle_inline_completions_globally(fs.clone(), cx),
        );
        menu = menu.separator().header("Privacy Settings");

        if let Some(provider) = &self.edit_prediction_provider {
            let data_collection = provider.data_collection_state(cx);
            if data_collection.is_supported() {
                let provider = provider.clone();
                let enabled = data_collection.is_enabled();

                menu = menu.item(
                    // TODO: We want to add something later that communicates whether
                    // the current project is open-source.
                    ContextMenuEntry::new("Share Training Data")
                        .toggleable(IconPosition::Start, data_collection.is_enabled())
                        .documentation_aside(|_| {
                            Label::new(indoc!{"
                                Help us improve our open model by sharing data from open source repositories. \
                                Zed must detect a license file in your repo for this setting to take effect.\
                            "}).into_any_element()
                        })
                        .handler(move |_, cx| {
                            provider.toggle_data_collection(cx);

                            if !enabled {
                                telemetry::event!(
                                    "Data Collection Enabled",
                                    source = "Edit Prediction Status Menu"
                                );
                            } else {
                                telemetry::event!(
                                    "Data Collection Disabled",
                                    source = "Edit Prediction Status Menu"
                                );
                            }
                        })
                )
            }
        }

        menu = menu.item(
            ContextMenuEntry::new("Configure Excluded Files")
                .icon(IconName::LockOutlined)
                .icon_color(Color::Muted)
                .documentation_aside(|_| {
                    Label::new(indoc!{"
                        Open your settings to add sensitive paths for which Zed will never predict edits."}).into_any_element()
                })
                .handler(move |window, cx| {
                    if let Some(workspace) = window.root().flatten() {
                        let workspace = workspace.downgrade();
                        window
                            .spawn(cx, |cx| {
                                open_disabled_globs_setting_in_editor(
                                    workspace,
                                    cx,
                                )
                            })
                            .detach_and_log_err(cx);
                    }
                }),
        );

        if !self.editor_enabled.unwrap_or(true) {
            menu = menu.item(
                ContextMenuEntry::new("This file is excluded.")
                    .disabled(true)
                    .icon(IconName::ZedPredictDisabled)
                    .icon_size(IconSize::Small),
            );
        }

        if let Some(editor_focus_handle) = self.editor_focus_handle.clone() {
            menu = menu
                .separator()
                .entry(
                    "Predict Edit at Cursor",
                    Some(Box::new(ShowEditPrediction)),
                    {
                        let editor_focus_handle = editor_focus_handle.clone();
                        move |window, cx| {
                            editor_focus_handle.dispatch_action(&ShowEditPrediction, window, cx);
                        }
                    },
                )
                .context(editor_focus_handle);
        }

        menu
    }

    fn build_copilot_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |menu, _, cx| {
            self.build_language_settings_menu(menu, cx)
                .separator()
                .link(
                    "Go to Copilot Settings",
                    OpenBrowser {
                        url: COPILOT_SETTINGS_URL.to_string(),
                    }
                    .boxed_clone(),
                )
                .action("Sign Out", copilot::SignOut.boxed_clone())
        })
    }

    fn build_supermaven_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |menu, _, cx| {
            self.build_language_settings_menu(menu, cx)
                .separator()
                .action("Sign Out", supermaven::SignOut.boxed_clone())
        })
    }

    fn build_zeta_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let workspace = self.workspace.clone();
        ContextMenu::build(window, cx, |menu, _window, cx| {
            self.build_language_settings_menu(menu, cx).when(
                cx.has_flag::<PredictEditsRateCompletionsFeatureFlag>(),
                |this| {
                    this.entry(
                        "Rate Completions",
                        Some(RateCompletions.boxed_clone()),
                        move |window, cx| {
                            workspace
                                .update(cx, |workspace, cx| {
                                    RateCompletionModal::toggle(workspace, window, cx)
                                })
                                .ok();
                        },
                    )
                },
            )
        })
    }

    pub fn update_enabled(&mut self, editor: Entity<Editor>, cx: &mut Context<Self>) {
        let editor = editor.read(cx);
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let suggestion_anchor = editor.selections.newest_anchor().start;
        let language = snapshot.language_at(suggestion_anchor);
        let file = snapshot.file_at(suggestion_anchor).cloned();
        self.editor_enabled = {
            let file = file.as_ref();
            Some(
                file.map(|file| {
                    all_language_settings(Some(file), cx)
                        .inline_completions_enabled_for_path(file.path())
                })
                .unwrap_or(true),
            )
        };
        self.editor_show_predictions = editor.should_show_inline_completions(cx);
        self.edit_prediction_provider = editor.edit_prediction_provider();
        self.language = language.cloned();
        self.file = file;
        self.editor_focus_handle = Some(editor.focus_handle(cx));

        cx.notify();
    }

    pub fn toggle_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.popover_menu_handle.toggle(window, cx);
    }
}

impl StatusItemView for InlineCompletionButton {
    fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.editor_subscription = Some((
                cx.observe(&editor, Self::update_enabled),
                editor.entity_id().as_u64() as usize,
            ));
            self.update_enabled(editor, cx);
        } else {
            self.language = None;
            self.editor_subscription = None;
            self.editor_enabled = None;
        }
        cx.notify();
    }
}

impl SupermavenButtonStatus {
    fn to_icon(&self) -> IconName {
        match self {
            SupermavenButtonStatus::Ready => IconName::Supermaven,
            SupermavenButtonStatus::Errored(_) => IconName::SupermavenError,
            SupermavenButtonStatus::NeedsActivation(_) => IconName::SupermavenInit,
            SupermavenButtonStatus::Initializing => IconName::SupermavenInit,
        }
    }

    fn to_tooltip(&self) -> String {
        match self {
            SupermavenButtonStatus::Ready => "Supermaven is ready".to_string(),
            SupermavenButtonStatus::Errored(error) => format!("Supermaven error: {}", error),
            SupermavenButtonStatus::NeedsActivation(_) => "Supermaven needs activation".to_string(),
            SupermavenButtonStatus::Initializing => "Supermaven initializing".to_string(),
        }
    }

    fn has_menu(&self) -> bool {
        match self {
            SupermavenButtonStatus::Ready | SupermavenButtonStatus::NeedsActivation(_) => true,
            SupermavenButtonStatus::Errored(_) | SupermavenButtonStatus::Initializing => false,
        }
    }
}

async fn open_disabled_globs_setting_in_editor(
    workspace: WeakEntity<Workspace>,
    mut cx: AsyncWindowContext,
) -> Result<()> {
    let settings_editor = workspace
        .update_in(&mut cx, |_, window, cx| {
            create_and_open_local_file(paths::settings_file(), window, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
        })?
        .await?
        .downcast::<Editor>()
        .unwrap();

    settings_editor
        .downgrade()
        .update_in(&mut cx, |item, window, cx| {
            let text = item.buffer().read(cx).snapshot(cx).text();

            let settings = cx.global::<SettingsStore>();

            // Ensure that we always have "inline_completions { "disabled_globs": [] }"
            let edits = settings.edits_for_update::<AllLanguageSettings>(&text, |file| {
                file.edit_predictions
                    .get_or_insert_with(Default::default)
                    .disabled_globs
                    .get_or_insert_with(Vec::new);
            });

            if !edits.is_empty() {
                item.edit(edits.iter().cloned(), cx);
            }

            let text = item.buffer().read(cx).snapshot(cx).text();

            static DISABLED_GLOBS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
                Regex::new(r#""disabled_globs":\s*\[\s*(?P<content>(?:.|\n)*?)\s*\]"#).unwrap()
            });
            // Only capture [...]
            let range = DISABLED_GLOBS_REGEX.captures(&text).and_then(|captures| {
                captures
                    .name("content")
                    .map(|inner_match| inner_match.start()..inner_match.end())
            });
            if let Some(range) = range {
                item.change_selections(Some(Autoscroll::newest()), window, cx, |selections| {
                    selections.select_ranges(vec![range]);
                });
            }
        })?;

    anyhow::Ok(())
}

fn toggle_inline_completions_globally(fs: Arc<dyn Fs>, cx: &mut App) {
    let show_edit_predictions = all_language_settings(None, cx).show_inline_completions(None, cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.defaults.show_edit_predictions = Some(!show_edit_predictions)
    });
}

fn set_completion_provider(fs: Arc<dyn Fs>, cx: &mut App, provider: EditPredictionProvider) {
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.features
            .get_or_insert(Default::default())
            .edit_prediction_provider = Some(provider);
    });
}

fn toggle_show_inline_completions_for_language(
    language: Arc<Language>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    let show_edit_predictions =
        all_language_settings(None, cx).show_inline_completions(Some(&language), cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.languages
            .entry(language.name())
            .or_default()
            .show_edit_predictions = Some(!show_edit_predictions);
    });
}

fn hide_copilot(fs: Arc<dyn Fs>, cx: &mut App) {
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.features
            .get_or_insert(Default::default())
            .edit_prediction_provider = Some(EditPredictionProvider::None);
    });
}
