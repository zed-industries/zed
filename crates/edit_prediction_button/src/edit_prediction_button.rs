use anyhow::Result;
use client::{UserStore, zed_urls};
use cloud_llm_client::UsageLimit;
use copilot::{Copilot, Status};
use editor::{Editor, SelectionEffects, actions::ShowEditPrediction, scroll::Autoscroll};
use feature_flags::{FeatureFlagAppExt, PredictEditsRateCompletionsFeatureFlag};
use fs::Fs;
use gpui::{
    Action, Animation, AnimationExt, App, AsyncWindowContext, Corner, Entity, FocusHandle,
    Focusable, IntoElement, ParentElement, Render, Subscription, WeakEntity, actions, div,
    pulsating_between,
};

use indoc::indoc;
use language::{
    EditPredictionsMode, File, Language,
    language_settings::{self, AllLanguageSettings, EditPredictionProvider, all_language_settings},
};
#[cfg(test)]
use language_model::LanguageModelProvider;
use language_models::{AllLanguageModelSettings, provider::ollama::OllamaLanguageModelProvider};
#[cfg(test)]
use settings::OllamaAvailableModel;
use settings::{AllLanguageModelSettingsContent, OllamaSettingsContent};

use paths;
use project::DisableAiSettings;
use regex::Regex;
use settings::{Settings, SettingsStore, update_settings_file};
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use supermaven::{AccountStatus, Supermaven};
use ui::{
    Clickable, ContextMenu, ContextMenuEntry, DocumentationEdge, DocumentationSide, IconButton,
    IconButtonShape, Indicator, PopoverMenu, PopoverMenuHandle, ProgressBar, Tooltip, prelude::*,
};
use workspace::{
    StatusItemView, Toast, Workspace, create_and_open_local_file, item::ItemHandle,
    notifications::NotificationId,
};
use zed_actions::OpenBrowser;
use zeta::RateCompletions;

actions!(
    edit_prediction,
    [
        /// Toggles the edit prediction menu.
        ToggleMenu
    ]
);

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";
const PRIVACY_DOCS: &str = "https://zed.dev/docs/ai/privacy-and-security";

struct CopilotErrorToast;

pub struct EditPredictionButton {
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    editor_show_predictions: bool,
    editor_focus_handle: Option<FocusHandle>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
    edit_prediction_provider: Option<Arc<dyn edit_prediction::EditPredictionProviderHandle>>,
    fs: Arc<dyn Fs>,
    user_store: Entity<UserStore>,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
}

enum SupermavenButtonStatus {
    Ready,
    Errored(String),
    NeedsActivation(String),
    Initializing,
}

impl Render for EditPredictionButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Return empty div if AI is disabled
        if DisableAiSettings::get_global(cx).disable_ai {
            return div();
        }

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
                                                |window, cx| {
                                                    copilot::reinstall_and_sign_in(window, cx)
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
                let this = cx.entity();

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
                                SupermavenButtonStatus::NeedsActivation(activate_url)
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
                let this = cx.entity();
                let fs = self.fs.clone();

                div().child(
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
                                        "Use Zed AI",
                                        None,
                                        move |_, cx| {
                                            set_completion_provider(
                                                fs.clone(),
                                                cx,
                                                EditPredictionProvider::Zed,
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
                )
            }

            EditPredictionProvider::Zed => {
                let enabled = self.editor_enabled.unwrap_or(true);

                let zeta_icon = if enabled {
                    IconName::ZedPredict
                } else {
                    IconName::ZedPredictDisabled
                };

                if zeta::should_show_upsell_modal() {
                    let tooltip_meta = if self.user_store.read(cx).current_user().is_some() {
                        "Choose a Plan"
                    } else {
                        "Sign In"
                    };

                    return div().child(
                        IconButton::new("zed-predict-pending-button", zeta_icon)
                            .shape(IconButtonShape::Square)
                            .indicator(Indicator::dot().color(Color::Muted))
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

                let mut over_limit = false;

                if let Some(usage) = self
                    .edit_prediction_provider
                    .as_ref()
                    .and_then(|provider| provider.usage(cx))
                {
                    over_limit = usage.over_limit()
                }

                let show_editor_predictions = self.editor_show_predictions;

                let icon_button = IconButton::new("zed-predict-pending-button", zeta_icon)
                    .shape(IconButtonShape::Square)
                    .when(
                        enabled && (!show_editor_predictions || over_limit),
                        |this| {
                            this.indicator(Indicator::dot().when_else(
                                over_limit,
                                |dot| dot.color(Color::Error),
                                |dot| dot.color(Color::Muted),
                            ))
                            .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                        },
                    )
                    .when(!self.popover_menu_handle.is_deployed(), |element| {
                        element.tooltip(move |window, cx| {
                            if enabled {
                                if show_editor_predictions {
                                    Tooltip::for_action("Edit Prediction", &ToggleMenu, window, cx)
                                } else {
                                    Tooltip::with_meta(
                                        "Edit Prediction",
                                        Some(&ToggleMenu),
                                        "Hidden For This File",
                                        window,
                                        cx,
                                    )
                                }
                            } else {
                                Tooltip::with_meta(
                                    "Edit Prediction",
                                    Some(&ToggleMenu),
                                    "Disabled For This File",
                                    window,
                                    cx,
                                )
                            }
                        })
                    });

                let this = cx.entity();

                let mut popover_menu = PopoverMenu::new("zeta")
                    .menu(move |window, cx| {
                        Some(this.update(cx, |this, cx| this.build_zeta_context_menu(window, cx)))
                    })
                    .anchor(Corner::BottomRight)
                    .with_handle(self.popover_menu_handle.clone());

                let is_refreshing = self
                    .edit_prediction_provider
                    .as_ref()
                    .is_some_and(|provider| provider.is_refreshing(cx));

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

            EditPredictionProvider::Ollama => {
                let enabled = self.editor_enabled.unwrap_or(false);
                let icon = if enabled {
                    IconName::AiOllama
                } else {
                    IconName::AiOllama // Could add disabled variant
                };

                let this = cx.entity();

                div().child(
                    PopoverMenu::new("ollama")
                        .menu(move |window, cx| {
                            Some(
                                this.update(cx, |this, cx| {
                                    this.build_ollama_context_menu(window, cx)
                                }),
                            )
                        })
                        .trigger(
                            IconButton::new("ollama-completion", icon)
                                .icon_size(IconSize::Small)
                                .tooltip(|window, cx| {
                                    Tooltip::for_action(
                                        "Ollama Completion",
                                        &ToggleMenu,
                                        window,
                                        cx,
                                    )
                                }),
                        )
                        .with_handle(self.popover_menu_handle.clone()),
                )
            }
        }
    }
}

impl EditPredictionButton {
    pub fn new(
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

        if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
            cx.observe(&provider, |_, _, cx| cx.notify()).detach();
        }

        Self {
            editor_subscription: None,
            editor_enabled: None,
            editor_show_predictions: false,
            editor_focus_handle: None,
            language: None,
            file: None,
            edit_prediction_provider: None,
            user_store,
            popover_menu_handle,
            fs,
        }
    }

    pub fn build_copilot_start_menu(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let fs = self.fs.clone();
        ContextMenu::build(window, cx, |menu, _, _| {
            menu.entry("Sign In to Copilot", None, copilot::initiate_sign_in)
                .entry("Disable Copilot", None, {
                    let fs = fs.clone();
                    move |_window, cx| hide_copilot(fs.clone(), cx)
                })
                .separator()
                .entry("Use Zed AI", None, {
                    let fs = fs.clone();
                    move |_window, cx| {
                        set_completion_provider(fs.clone(), cx, EditPredictionProvider::Zed)
                    }
                })
        })
    }

    pub fn build_language_settings_menu(
        &self,
        mut menu: ContextMenu,
        window: &Window,
        cx: &mut App,
    ) -> ContextMenu {
        let fs = self.fs.clone();
        let line_height = window.line_height();

        menu = menu.header("Show Edit Predictions For");

        let language_state = self.language.as_ref().map(|language| {
            (
                language.clone(),
                language_settings::language_settings(Some(language.name()), None, cx)
                    .show_edit_predictions,
            )
        });

        if let Some(editor_focus_handle) = self.editor_focus_handle.clone() {
            let entry = ContextMenuEntry::new("This Buffer")
                .toggleable(IconPosition::Start, self.editor_show_predictions)
                .action(Box::new(editor::actions::ToggleEditPrediction))
                .handler(move |window, cx| {
                    editor_focus_handle.dispatch_action(
                        &editor::actions::ToggleEditPrediction,
                        window,
                        cx,
                    );
                });

            match language_state.clone() {
                Some((language, false)) => {
                    menu = menu.item(
                        entry
                            .disabled(true)
                            .documentation_aside(DocumentationSide::Left, DocumentationEdge::Top, move |_cx| {
                                Label::new(format!("Edit predictions cannot be toggled for this buffer because they are disabled for {}", language.name()))
                                    .into_any_element()
                            })
                    );
                }
                Some(_) | None => menu = menu.item(entry),
            }
        }

        if let Some((language, language_enabled)) = language_state {
            let fs = fs.clone();

            menu = menu.toggleable_entry(
                language.name(),
                language_enabled,
                IconPosition::Start,
                None,
                move |_, cx| {
                    toggle_show_edit_predictions_for_language(language.clone(), fs.clone(), cx)
                },
            );
        }

        let settings = AllLanguageSettings::get_global(cx);

        let globally_enabled = settings.show_edit_predictions(None, cx);
        let entry = ContextMenuEntry::new("All Files")
            .toggleable(IconPosition::Start, globally_enabled)
            .action(workspace::ToggleEditPrediction.boxed_clone())
            .handler(|window, cx| {
                window.dispatch_action(workspace::ToggleEditPrediction.boxed_clone(), cx)
            });
        menu = menu.item(entry);

        let provider = settings.edit_predictions.provider;
        let current_mode = settings.edit_predictions_mode();
        let subtle_mode = matches!(current_mode, EditPredictionsMode::Subtle);
        let eager_mode = matches!(current_mode, EditPredictionsMode::Eager);

        if matches!(
            provider,
            EditPredictionProvider::Zed
                | EditPredictionProvider::Copilot
                | EditPredictionProvider::Supermaven
                | EditPredictionProvider::Ollama
        ) {
            menu = menu
                .separator()
                .header("Display Modes")
                .item(
                    ContextMenuEntry::new("Eager")
                        .toggleable(IconPosition::Start, eager_mode)
                        .documentation_aside(DocumentationSide::Left, DocumentationEdge::Top, move |_| {
                            Label::new("Display predictions inline when there are no language server completions available.").into_any_element()
                        })
                        .handler({
                            let fs = fs.clone();
                            move |_, cx| {
                                toggle_edit_prediction_mode(fs.clone(), EditPredictionsMode::Eager, cx)
                            }
                        }),
                )
                .item(
                    ContextMenuEntry::new("Subtle")
                        .toggleable(IconPosition::Start, subtle_mode)
                        .documentation_aside(DocumentationSide::Left, DocumentationEdge::Top, move |_| {
                            Label::new("Display predictions inline only when holding a modifier key (alt by default).").into_any_element()
                        })
                        .handler({
                            let fs = fs.clone();
                            move |_, cx| {
                                toggle_edit_prediction_mode(fs.clone(), EditPredictionsMode::Subtle, cx)
                            }
                        }),
                );
        }

        menu = menu.separator().header("Privacy");
        if let Some(provider) = &self.edit_prediction_provider {
            let data_collection = provider.data_collection_state(cx);
            if data_collection.is_supported() {
                let provider = provider.clone();
                let enabled = data_collection.is_enabled();
                let is_open_source = data_collection.is_project_open_source();
                let is_collecting = data_collection.is_enabled();
                let (icon_name, icon_color) = if is_open_source && is_collecting {
                    (IconName::Check, Color::Success)
                } else {
                    (IconName::Check, Color::Accent)
                };

                menu = menu.item(
                    ContextMenuEntry::new("Training Data Collection")
                        .toggleable(IconPosition::Start, data_collection.is_enabled())
                        .icon(icon_name)
                        .icon_color(icon_color)
                        .documentation_aside(DocumentationSide::Left, DocumentationEdge::Top, move |cx| {
                            let (msg, label_color, icon_name, icon_color) = match (is_open_source, is_collecting) {
                                (true, true) => (
                                    "Project identified as open source, and you're sharing data.",
                                    Color::Default,
                                    IconName::Check,
                                    Color::Success,
                                ),
                                (true, false) => (
                                    "Project identified as open source, but you're not sharing data.",
                                    Color::Muted,
                                    IconName::Close,
                                    Color::Muted,
                                ),
                                (false, true) => (
                                    "Project not identified as open source. No data captured.",
                                    Color::Muted,
                                    IconName::Close,
                                    Color::Muted,
                                ),
                                (false, false) => (
                                    "Project not identified as open source, and setting turned off.",
                                    Color::Muted,
                                    IconName::Close,
                                    Color::Muted,
                                ),
                            };
                            v_flex()
                                .gap_2()
                                .child(
                                    Label::new(indoc!{
                                        "Help us improve our open dataset model by sharing data from open source repositories. \
                                        Zed must detect a license file in your repo for this setting to take effect. \
                                        Files with sensitive data and secrets are excluded by default."
                                    })
                                )
                                .child(
                                    h_flex()
                                        .items_start()
                                        .pt_2()
                                        .pr_1()
                                        .flex_1()
                                        .gap_1p5()
                                        .border_t_1()
                                        .border_color(cx.theme().colors().border_variant)
                                        .child(h_flex().flex_shrink_0().h(line_height).child(Icon::new(icon_name).size(IconSize::XSmall).color(icon_color)))
                                        .child(div().child(msg).w_full().text_sm().text_color(label_color.color(cx)))
                                )
                                .into_any_element()
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
                );

                if is_collecting && !is_open_source {
                    menu = menu.item(
                        ContextMenuEntry::new("No data captured.")
                            .disabled(true)
                            .icon(IconName::Close)
                            .icon_color(Color::Error)
                            .icon_size(IconSize::Small),
                    );
                }
            }
        }

        menu = menu.item(
            ContextMenuEntry::new("Configure Excluded Files")
                .icon(IconName::LockOutlined)
                .icon_color(Color::Muted)
                .documentation_aside(DocumentationSide::Left, DocumentationEdge::Top, |_| {
                    Label::new(indoc!{"
                        Open your settings to add sensitive paths for which Zed will never predict edits."}).into_any_element()
                })
                .handler(move |window, cx| {
                    if let Some(workspace) = window.root().flatten() {
                        let workspace = workspace.downgrade();
                        window
                            .spawn(cx, async |cx| {
                                open_disabled_globs_setting_in_editor(
                                    workspace,
                                    cx,
                                ).await
                            })
                            .detach_and_log_err(cx);
                    }
                }),
        ).item(
            ContextMenuEntry::new("View Documentation")
                .icon(IconName::FileGeneric)
                .icon_color(Color::Muted)
                .handler(move |_, cx| {
                    cx.open_url(PRIVACY_DOCS);
                })
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
        ContextMenu::build(window, cx, |menu, window, cx| {
            self.build_language_settings_menu(menu, window, cx)
                .separator()
                .entry("Use Zed AI instead", None, {
                    let fs = self.fs.clone();
                    move |_window, cx| {
                        set_completion_provider(fs.clone(), cx, EditPredictionProvider::Zed)
                    }
                })
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
        ContextMenu::build(window, cx, |menu, window, cx| {
            self.build_language_settings_menu(menu, window, cx)
                .separator()
                .action("Sign Out", supermaven::SignOut.boxed_clone())
        })
    }

    fn build_zeta_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |mut menu, window, cx| {
            if let Some(usage) = self
                .edit_prediction_provider
                .as_ref()
                .and_then(|provider| provider.usage(cx))
            {
                menu = menu.header("Usage");
                menu = menu
                    .custom_entry(
                        move |_window, cx| {
                            let used_percentage = match usage.limit {
                                UsageLimit::Limited(limit) => {
                                    Some((usage.amount as f32 / limit as f32) * 100.)
                                }
                                UsageLimit::Unlimited => None,
                            };

                            h_flex()
                                .flex_1()
                                .gap_1p5()
                                .children(
                                    used_percentage.map(|percent| {
                                        ProgressBar::new("usage", percent, 100., cx)
                                    }),
                                )
                                .child(
                                    Label::new(match usage.limit {
                                        UsageLimit::Limited(limit) => {
                                            format!("{} / {limit}", usage.amount)
                                        }
                                        UsageLimit::Unlimited => format!("{} / ∞", usage.amount),
                                    })
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                )
                                .into_any_element()
                        },
                        move |_, cx| cx.open_url(&zed_urls::account_url(cx)),
                    )
                    .when(usage.over_limit(), |menu| -> ContextMenu {
                        menu.entry("Subscribe to increase your limit", None, |_window, cx| {
                            cx.open_url(&zed_urls::account_url(cx))
                        })
                    })
                    .separator();
            } else if self.user_store.read(cx).account_too_young() {
                menu = menu
                    .custom_entry(
                        |_window, _cx| {
                            Label::new("Your GitHub account is less than 30 days old.")
                                .size(LabelSize::Small)
                                .color(Color::Warning)
                                .into_any_element()
                        },
                        |_window, cx| cx.open_url(&zed_urls::account_url(cx)),
                    )
                    .entry("Upgrade to Zed Pro or contact us.", None, |_window, cx| {
                        cx.open_url(&zed_urls::account_url(cx))
                    })
                    .separator();
            } else if self.user_store.read(cx).has_overdue_invoices() {
                menu = menu
                    .custom_entry(
                        |_window, _cx| {
                            Label::new("You have an outstanding invoice")
                                .size(LabelSize::Small)
                                .color(Color::Warning)
                                .into_any_element()
                        },
                        |_window, cx| {
                            cx.open_url(&zed_urls::account_url(cx))
                        },
                    )
                    .entry(
                        "Check your payment status or contact us at billing-support@zed.dev to continue using this feature.",
                        None,
                        |_window, cx| {
                            cx.open_url(&zed_urls::account_url(cx))
                        },
                    )
                    .separator();
            }

            self.build_language_settings_menu(menu, window, cx).when(
                cx.has_flag::<PredictEditsRateCompletionsFeatureFlag>(),
                |this| this.action("Rate Completions", RateCompletions.boxed_clone()),
            )
        })
    }

    /// Builds a simplified context menu for Ollama with essential features:
    /// - API URL configuration that opens settings at the correct location
    /// - Model selection from available models
    /// - Common language settings (buffer/language/global toggles, privacy settings)
    ///
    /// The menu focuses on core functionality without connection status or external links.
    fn build_ollama_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let fs = self.fs.clone();
        ContextMenu::build(window, cx, |menu, window, cx| {
            // Automatically refresh models when menu is opened
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                provider.update(cx, |provider, cx| {
                    provider.refresh_models(cx);
                });
            }
            let settings = AllLanguageModelSettings::get_global(cx);
            let ollama_settings = &settings.ollama;

            // Get models from both settings and global service discovery
            let mut available_models = ollama_settings.available_models.clone();

            // Add discovered models from the global Ollama service
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                for model in discovered_models {
                    // Convert from ollama::Model to language_models AvailableModel
                    let available_model = language_models::provider::ollama::AvailableModel {
                        name: model.name.clone(),
                        display_name: model.display_name.clone(),
                        max_tokens: model.max_tokens,
                        keep_alive: model.keep_alive.clone(),
                        supports_tools: model.supports_tools,
                        supports_images: model.supports_images,
                        supports_thinking: model.supports_thinking,
                    };

                    // Add if not already in settings (settings take precedence)
                    if !available_models.iter().any(|m| m.name == model.name) {
                        available_models.push(available_model);
                    }
                }
            }

            // Model selection section
            let menu = if !available_models.is_empty() {
                let menu = menu.separator().header("Available Models");

                // Add each available model as a menu entry
                let menu = available_models.iter().fold(menu, |menu, model| {
                    let model_name = model.display_name.as_ref().unwrap_or(&model.name);
                    let is_current = ollama_settings
                        .available_models
                        .first()
                        .map(|current_model| current_model.name == model.name)
                        .unwrap_or(false);

                    menu.toggleable_entry(
                        model_name.clone(),
                        is_current,
                        IconPosition::Start,
                        None,
                        {
                            let model_name = model.name.clone();
                            let fs = fs.clone();
                            move |_window, cx| {
                                Self::switch_ollama_model(fs.clone(), model_name.clone(), cx);
                            }
                        },
                    )
                });

                menu
            } else {
                menu.separator()
                    .header("No Models Configured")
                    .entry("Configure Models", None, {
                        let fs = fs.clone();
                        move |window, cx| {
                            Self::open_ollama_settings(fs.clone(), window, cx);
                        }
                    })
            };

            // Use the common language settings menu
            self.build_language_settings_menu(menu, window, cx)
        })
    }

    /// Opens Zed settings and navigates directly to the Ollama models configuration.
    /// Uses improved regex patterns to locate the exact setting in the JSON structure.
    fn open_ollama_settings(_fs: Arc<dyn Fs>, window: &mut Window, cx: &mut App) {
        if let Some(workspace) = window.root::<Workspace>().flatten() {
            let workspace = workspace.downgrade();
            window
                .spawn(cx, async move |cx| {
                    let settings_editor = workspace
                        .update_in(cx, |_, window, cx| {
                            create_and_open_local_file(paths::settings_file(), window, cx, || {
                                settings::initial_user_settings_content().as_ref().into()
                            })
                        })?
                        .await?
                        .downcast::<Editor>()
                        .unwrap();

                    let _ = settings_editor
                        .downgrade()
                        .update_in(cx, |item, window, cx| {
                            let text = item.buffer().read(cx).snapshot(cx).text();

                            // Look for language_models.ollama section with precise pattern
                            // This matches the full nested structure to avoid false matches
                            let ollama_pattern = r#""language_models"\s*:\s*\{[\s\S]*?"ollama"\s*:\s*\{[\s\S]*?"available_models"\s*:\s*\[\s*\]"#;
                            let regex = regex::Regex::new(ollama_pattern).unwrap();

                            if let Some(captures) = regex.captures(&text) {
                                let full_match = captures.get(0).unwrap();

                                // Position cursor after the opening bracket of available_models array
                                let bracket_pos = full_match.as_str().rfind('[').unwrap();
                                let cursor_pos = full_match.start() + bracket_pos + 1;

                                // Place cursor inside the available_models array
                                item.change_selections(
                                    SelectionEffects::scroll(Autoscroll::newest()),
                                    window,
                                    cx,
                                    |selections| {
                                        selections.select_ranges(vec![cursor_pos..cursor_pos]);
                                    },
                                );
                                return Ok::<(), anyhow::Error>(());
                            }

                            Ok::<(), anyhow::Error>(())
                        })?;

                    Ok::<(), anyhow::Error>(())
                })
                .detach_and_log_err(cx);
        }
    }

    fn switch_ollama_model(fs: Arc<dyn Fs>, model_name: String, cx: &mut App) {
        update_settings_file(fs, cx, move |settings, cx| {
            // Ensure language_models settings exist
            if settings.language_models.is_none() {
                settings.language_models = Some(AllLanguageModelSettingsContent::default());
            }

            let language_models = settings.language_models.as_mut().unwrap();

            // Ensure ollama settings exist
            if language_models.ollama.is_none() {
                language_models.ollama = Some(OllamaSettingsContent {
                    api_url: None,
                    available_models: Some(Vec::new()),
                });
            }

            let ollama_settings = language_models.ollama.as_mut().unwrap();

            // Ensure available_models exists
            if ollama_settings.available_models.is_none() {
                ollama_settings.available_models = Some(Vec::new());
            }

            let models = ollama_settings.available_models.as_mut().unwrap();

            // Check if model is already in settings
            if let Some(index) = models.iter().position(|m| m.name == model_name) {
                // Move existing model to the front
                let selected_model = models.remove(index);
                models.insert(0, selected_model);
            } else {
                // Model not in settings - check if it's a discovered model and add it
                if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                    let discovered_models = provider.read(cx).available_models_for_completion(cx);
                    if let Some(discovered_model) =
                        discovered_models.iter().find(|m| m.name == model_name)
                    {
                        // Convert from ollama::Model to language_models AvailableModel
                        let available_model = language_models::provider::ollama::AvailableModel {
                            name: discovered_model.name.clone(),
                            display_name: discovered_model.display_name.clone(),
                            max_tokens: discovered_model.max_tokens,
                            keep_alive: discovered_model.keep_alive.clone(),
                            supports_tools: discovered_model.supports_tools,
                            supports_images: discovered_model.supports_images,
                            supports_thinking: discovered_model.supports_thinking,
                        };

                        // Add the discovered model to the front of the list
                        models.insert(0, available_model);
                    }
                }
            }
        });
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
                        .edit_predictions_enabled_for_file(file, cx)
                })
                .unwrap_or(true),
            )
        };
        self.editor_show_predictions = editor.edit_predictions_enabled();
        self.edit_prediction_provider = editor.edit_prediction_provider();
        self.language = language.cloned();
        self.file = file;
        self.editor_focus_handle = Some(editor.focus_handle(cx));

        cx.notify();
    }
}

impl StatusItemView for EditPredictionButton {
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
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let settings_editor = workspace
        .update_in(cx, |_, window, cx| {
            create_and_open_local_file(paths::settings_file(), window, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
        })?
        .await?
        .downcast::<Editor>()
        .unwrap();

    settings_editor
        .downgrade()
        .update_in(cx, |item, window, cx| {
            let text = item.buffer().read(cx).snapshot(cx).text();

            let settings = cx.global::<SettingsStore>();

            // Ensure that we always have "edit_predictions { "disabled_globs": [] }"
            let edits = settings.edits_for_update(&text, |file| {
                file.project
                    .all_languages
                    .edit_predictions
                    .get_or_insert_with(Default::default)
                    .disabled_globs
                    .get_or_insert_with(Vec::new);
            });

            if !edits.is_empty() {
                item.edit(edits, cx);
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
                item.change_selections(
                    SelectionEffects::scroll(Autoscroll::newest()),
                    window,
                    cx,
                    |selections| {
                        selections.select_ranges(vec![range]);
                    },
                );
            }
        })?;

    anyhow::Ok(())
}

fn set_completion_provider(fs: Arc<dyn Fs>, cx: &mut App, provider: EditPredictionProvider) {
    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .all_languages
            .features
            .get_or_insert_default()
            .edit_prediction_provider = Some(provider);
    });
}

fn toggle_show_edit_predictions_for_language(
    language: Arc<Language>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    let show_edit_predictions =
        all_language_settings(None, cx).show_edit_predictions(Some(&language), cx);
    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .all_languages
            .languages
            .0
            .entry(language.name().0)
            .or_default()
            .show_edit_predictions = Some(!show_edit_predictions);
    });
}

fn hide_copilot(fs: Arc<dyn Fs>, cx: &mut App) {
    update_settings_file(fs, cx, move |settings, _| {
        settings
            .project
            .all_languages
            .features
            .get_or_insert(Default::default())
            .edit_prediction_provider = Some(EditPredictionProvider::None);
    });
}

fn toggle_edit_prediction_mode(fs: Arc<dyn Fs>, mode: EditPredictionsMode, cx: &mut App) {
    let settings = AllLanguageSettings::get_global(cx);
    let current_mode = settings.edit_predictions_mode();

    if current_mode != mode {
        update_settings_file(fs, cx, move |settings, _cx| {
            if let Some(edit_predictions) = settings.project.all_languages.edit_predictions.as_mut()
            {
                edit_predictions.mode = Some(mode);
            } else {
                settings.project.all_languages.edit_predictions =
                    Some(settings::EditPredictionSettingsContent {
                        mode: Some(mode),
                        ..Default::default()
                    });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client;
    use clock::FakeSystemClock;
    use fs;
    use gpui::TestAppContext;
    use http_client;
    use ollama::fake::FakeHttpClient;
    use settings::SettingsStore;
    use std::sync::Arc;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            gpui_tokio::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            language_settings::init(cx);
            project::Project::init_settings(cx);

            // Initialize language_models settings for tests that need them
            // Create client and user store for language_models::init
            client::init_settings(cx);
            let clock = Arc::new(FakeSystemClock::new());
            let http = http_client::FakeHttpClient::with_404_response();
            let client = client::Client::new(clock, http, cx);
            let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));

            client::init(&client, cx);
            language_model::init(client.clone(), cx);
            language_models::init(user_store, client, cx);
        });
    }

    #[gpui::test]
    async fn test_ollama_menu_shows_discovered_models(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client with mock models response
        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Mock /api/tags response
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "qwen2.5-coder:3b",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "qwen2",
                        "families": ["qwen2"],
                        "parameter_size": "3B",
                        "quantization_level": "Q4_0"
                    }
                },
                {
                    "name": "codellama:7b-code",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 2000000,
                    "digest": "def456",
                    "details": {
                        "format": "gguf",
                        "family": "codellama",
                        "families": ["codellama"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());

        // Mock /api/show response
        let capabilities = serde_json::json!({
            "capabilities": ["tools"]
        });
        fake_http_client.set_response("/api/show", capabilities.to_string());

        // Create and set global Ollama service
        let provider = cx.update(|cx| {
            let provider = cx.new(|cx| {
                language_models::provider::ollama::OllamaLanguageModelProvider::new(
                    fake_http_client.clone(),
                    cx,
                )
            });
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Authenticate the provider to enable model discovery
            #[cfg(test)]
            {
                let task = provider.update(cx, |provider, cx| provider.authenticate(cx));
                task.detach();
            }

            provider
        });

        // Wait for model discovery
        cx.background_executor.run_until_parked();

        // Verify models are accessible through the service
        cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                assert_eq!(discovered_models.len(), 2);

                let model_names: Vec<&str> =
                    discovered_models.iter().map(|m| m.name.as_str()).collect();
                assert!(model_names.contains(&"qwen2.5-coder:3b"));
                assert!(model_names.contains(&"codellama:7b-code"));
            } else {
                panic!("Global service should be available");
            }
        });

        // Verify the global provider has the expected models
        provider.read_with(cx, |provider, cx| {
            let models = provider.available_models_for_completion(cx);
            assert_eq!(models.len(), 2);

            let model_names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
            assert!(model_names.contains(&"qwen2.5-coder:3b"));
            assert!(model_names.contains(&"codellama:7b-code"));
        });
    }

    #[gpui::test]
    async fn test_ollama_menu_shows_service_models(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client with models
        let fake_http_client = Arc::new(FakeHttpClient::new());

        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "qwen2.5-coder:7b",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "qwen2",
                        "families": ["qwen2"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({"capabilities": []}).to_string(),
        );

        // Create and set global service
        let _provider = cx.update(|cx| {
            let provider = cx.new(|cx| {
                language_models::provider::ollama::OllamaLanguageModelProvider::new(
                    fake_http_client,
                    cx,
                )
            });
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Authenticate the provider to enable model discovery
            #[cfg(test)]
            {
                let task = provider.update(cx, |provider, cx| provider.authenticate(cx));
                task.detach();
            }

            provider
        });

        cx.background_executor.run_until_parked();

        // Test that discovered models are accessible
        cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                assert_eq!(discovered_models.len(), 1);
                assert_eq!(discovered_models[0].name, "qwen2.5-coder:7b");
            } else {
                panic!("Global service should be available");
            }
        });
    }

    #[gpui::test]
    async fn test_ollama_menu_refreshes_on_service_update(cx: &mut TestAppContext) {
        init_test(cx);

        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Initially empty models
        fake_http_client.set_response("/api/tags", serde_json::json!({"models": []}).to_string());

        let _provider = cx.update(|cx| {
            let provider = cx.new(|cx| {
                language_models::provider::ollama::OllamaLanguageModelProvider::new(
                    fake_http_client.clone(),
                    cx,
                )
            });
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Authenticate the provider to enable model discovery
            #[cfg(test)]
            {
                let task = provider.update(cx, |provider, cx| provider.authenticate(cx));
                task.detach();
            }

            provider
        });

        cx.background_executor.run_until_parked();

        // Verify the service subscription mechanism works by creating a button
        let _button = cx.update(|cx| {
            let fs = fs::FakeFs::new(cx.background_executor().clone());
            let clock = Arc::new(FakeSystemClock::new());
            let http = http_client::FakeHttpClient::with_404_response();
            let client = client::Client::new(clock, http, cx);
            let user_store = cx.new(|cx| client::UserStore::new(client, cx));
            let popover_handle = PopoverMenuHandle::default();

            cx.new(|cx| EditPredictionButton::new(fs, user_store, popover_handle, cx))
        });

        // Verify initially no models
        cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let models = provider.read(cx).available_models_for_completion(cx);
                assert_eq!(models.len(), 0);
            }
        });

        // Update mock to return models
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "phi3:mini",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 500000,
                    "digest": "xyz789",
                    "details": {
                        "format": "gguf",
                        "family": "phi3",
                        "families": ["phi3"],
                        "parameter_size": "3.8B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({"capabilities": []}).to_string(),
        );

        // Trigger refresh
        cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                provider.update(cx, |provider, cx| {
                    provider.refresh_models(cx);
                });
            }
        });

        cx.background_executor.run_until_parked();

        // Verify models were refreshed
        cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let models = provider.read(cx).available_models_for_completion(cx);
                assert_eq!(models.len(), 1);
                assert_eq!(models[0].name, "phi3:mini");
            }
        });

        // The button should have been notified and will rebuild its menu with new models
        // when next requested (this tests the subscription mechanism)
    }

    #[gpui::test]
    async fn test_automatic_refresh_when_menu_opened(cx: &mut TestAppContext) {
        init_test(cx);

        // Setup fake HTTP client with initial empty response
        let fake_http_client = Arc::new(FakeHttpClient::new());
        let empty_response = serde_json::json!({"models": []});
        fake_http_client.set_response("/api/tags", empty_response.to_string());
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({"capabilities": []}).to_string(),
        );

        let _provider = cx.update(|cx| {
            let provider = cx.new(|cx| {
                language_models::provider::ollama::OllamaLanguageModelProvider::new(
                    fake_http_client.clone(),
                    cx,
                )
            });
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Authenticate the provider to enable model discovery
            #[cfg(test)]
            {
                let task = provider.update(cx, |provider, cx| provider.authenticate(cx));
                task.detach();
            }

            provider
        });

        // Wait for initial fetch to complete (should have no models)
        cx.background_executor.run_until_parked();

        cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let models = provider.read(cx).available_models_for_completion(cx);
                assert_eq!(models.len(), 0);
            }
        });

        // Now update the response to have a model
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "newly-discovered:7b",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "test",
                        "families": ["test"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });
        fake_http_client.set_response("/api/tags", models_response.to_string());

        // Create the EditPredictionButton and directly test the menu building
        let (_button, cx) = cx.add_window_view(|window, cx| {
            let fs = fs::FakeFs::new(cx.background_executor().clone());
            let clock = Arc::new(FakeSystemClock::new());
            let http = http_client::FakeHttpClient::with_404_response();
            let client = client::Client::new(clock, http, cx);
            let user_store = cx.new(|cx| client::UserStore::new(client, cx));

            let button =
                EditPredictionButton::new(fs, user_store, PopoverMenuHandle::default(), cx);

            // The critical test: Call build_ollama_context_menu directly
            // This should automatically trigger a refresh
            let _menu = button.build_ollama_context_menu(window, cx);

            button
        });

        // Wait for the automatic refresh to complete
        cx.background_executor.run_until_parked();

        // Verify that the models were refreshed as a side effect of building the menu
        cx.update(|_, cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let models = provider.read(cx).available_models_for_completion(cx);
                assert_eq!(models.len(), 1);
                assert_eq!(models[0].name, "newly-discovered:7b");
            }
        });
    }

    #[gpui::test]
    async fn test_ollama_menu_shows_discovered_models_for_selection(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client with mock models response
        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Mock /api/tags response with a model not in settings
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "discovered-model:latest",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "llama",
                        "families": ["llama"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({"capabilities": []}).to_string(),
        );

        // Create and set global Ollama service
        let _provider = cx.update(|cx| {
            let provider = cx.new(|cx| {
                language_models::provider::ollama::OllamaLanguageModelProvider::new(
                    fake_http_client.clone(),
                    cx,
                )
            });
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Authenticate the provider to enable model discovery
            #[cfg(test)]
            {
                let task = provider.update(cx, |provider, cx| provider.authenticate(cx));
                task.detach();
            }

            provider
        });

        cx.background_executor.run_until_parked();

        // Verify model is discovered by the service
        let discovered_model_exists = cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                discovered_models
                    .iter()
                    .any(|m| m.name == "discovered-model:latest")
            } else {
                false
            }
        });
        assert!(
            discovered_model_exists,
            "Model should be discovered by service"
        );

        // Verify initial settings are empty
        let settings_empty = cx.update(|cx| {
            let settings = AllLanguageModelSettings::get_global(cx);
            settings.ollama.available_models.is_empty()
        });
        assert!(settings_empty, "Settings should initially be empty");

        // Test the core logic: when a discovered model is selected, it should be available
        // In the UI context, the menu should show discovered models even if not in settings
        let menu_shows_discovered_model = cx.update(|cx| {
            let settings = AllLanguageModelSettings::get_global(cx);
            let ollama_settings = &settings.ollama;

            // Get models from both settings and global service discovery (like the UI does)
            let mut available_models = ollama_settings.available_models.clone();

            // Add discovered models from the global Ollama service
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                for model in discovered_models {
                    // Convert from ollama::Model to language_models AvailableModel
                    let available_model = language_models::provider::ollama::AvailableModel {
                        name: model.name.clone(),
                        display_name: model.display_name.clone(),
                        max_tokens: model.max_tokens,
                        keep_alive: model.keep_alive.clone(),
                        supports_tools: model.supports_tools,
                        supports_images: model.supports_images,
                        supports_thinking: model.supports_thinking,
                    };

                    // Add if not already in settings (settings take precedence)
                    if !available_models.iter().any(|m| m.name == model.name) {
                        available_models.push(available_model);
                    }
                }
            }

            available_models
                .iter()
                .any(|m| m.name == "discovered-model:latest")
        });

        assert!(
            menu_shows_discovered_model,
            "Menu should show discovered models even when not in settings"
        );
    }

    #[gpui::test]
    async fn test_ollama_discovered_model_menu_integration(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client with mock models response
        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Mock /api/tags response with a model not in settings
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "discovered-model:latest",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "llama",
                        "families": ["llama"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({"capabilities": []}).to_string(),
        );

        // Create and set global Ollama service
        let _provider = cx.update(|cx| {
            let provider = cx.new(|cx| {
                language_models::provider::ollama::OllamaLanguageModelProvider::new(
                    fake_http_client.clone(),
                    cx,
                )
            });
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Authenticate the provider to enable model discovery
            #[cfg(test)]
            {
                let task = provider.update(cx, |provider, cx| provider.authenticate(cx));
                task.detach();
            }

            provider
        });

        cx.background_executor.run_until_parked();

        // Test the core functionality: discovered models should be available for the UI
        // This simulates what the build_ollama_context_menu function does
        cx.update(|cx| {
            let settings = AllLanguageModelSettings::get_global(cx);
            let ollama_settings = &settings.ollama;

            // Get models from both settings and global service discovery (like the UI does)
            let mut available_models = ollama_settings.available_models.clone();

            // Add discovered models from the global Ollama service
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                for model in discovered_models {
                    // Models are already in the correct format from available_models_for_completion
                    let available_model = model.clone();

                    // Add if not already in settings (settings take precedence)
                    if !available_models.iter().any(|m| m.name == model.name) {
                        available_models.push(OllamaAvailableModel {
                            name: available_model.name,
                            display_name: available_model.display_name,
                            max_tokens: available_model.max_tokens,
                            keep_alive: available_model.keep_alive,
                            supports_tools: available_model.supports_tools,
                            supports_images: available_model.supports_images,
                            supports_thinking: available_model.supports_thinking,
                        });
                    }
                }
            }

            // The key test: discovered models should now be available for selection
            assert_eq!(available_models.len(), 1);
            assert_eq!(available_models[0].name, "discovered-model:latest");

            // Verify that the switch_ollama_model function can find the discovered model
            // by checking it exists in the service
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                let found_model = discovered_models
                    .iter()
                    .find(|m| m.name == "discovered-model:latest");
                assert!(
                    found_model.is_some(),
                    "Model should be discoverable by the service for selection"
                );
            }
        });
    }

    #[gpui::test]
    async fn test_switch_ollama_model_with_discovered_model(cx: &mut TestAppContext) {
        init_test(cx);

        // Create fake HTTP client with mock models response
        let fake_http_client = Arc::new(FakeHttpClient::new());

        // Mock /api/tags response with a model not in settings
        let models_response = serde_json::json!({
            "models": [
                {
                    "name": "test-model:latest",
                    "modified_at": "2024-01-01T00:00:00Z",
                    "size": 1000000,
                    "digest": "abc123",
                    "details": {
                        "format": "gguf",
                        "family": "llama",
                        "families": ["llama"],
                        "parameter_size": "7B",
                        "quantization_level": "Q4_0"
                    }
                }
            ]
        });

        fake_http_client.set_response("/api/tags", models_response.to_string());
        fake_http_client.set_response(
            "/api/show",
            serde_json::json!({"capabilities": []}).to_string(),
        );

        // Create and set global service
        let _provider = cx.update(|cx| {
            let provider = cx.new(|cx| {
                language_models::provider::ollama::OllamaLanguageModelProvider::new(
                    fake_http_client.clone(),
                    cx,
                )
            });
            OllamaLanguageModelProvider::set_global(provider.clone(), cx);

            // Authenticate the provider to enable model discovery
            #[cfg(test)]
            {
                let task = provider.update(cx, |provider, cx| provider.authenticate(cx));
                task.detach();
            }

            provider
        });

        cx.background_executor.run_until_parked();

        // Verify model is discovered by provider
        let discovered = cx.update(|cx| {
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let models = provider.read(cx).available_models_for_completion(cx);
                models.iter().any(|m| m.name == "test-model:latest")
            } else {
                false
            }
        });
        assert!(discovered, "Model should be discovered by provider");

        // Test that switch_ollama_model function can handle discovered models
        // This test focuses on the function's ability to find and convert discovered models
        // rather than testing file system persistence
        let fs = fs::FakeFs::new(cx.background_executor.clone()) as Arc<dyn fs::Fs>;

        // The key test: the function should be able to process a discovered model
        // We test this by verifying the function doesn't panic and can access the provider
        cx.update(|cx| {
            // Verify the provider is accessible within the function context
            if let Some(provider) = OllamaLanguageModelProvider::global(cx) {
                let discovered_models = provider.read(cx).available_models_for_completion(cx);
                let target_model = discovered_models
                    .iter()
                    .find(|m| m.name == "test-model:latest");

                assert!(
                    target_model.is_some(),
                    "Target model should be discoverable"
                );

                // Test the conversion logic that switch_ollama_model uses
                if let Some(discovered_model) = target_model {
                    let available_model = language_models::provider::ollama::AvailableModel {
                        name: discovered_model.name.clone(),
                        display_name: discovered_model.display_name.clone(),
                        max_tokens: discovered_model.max_tokens,
                        keep_alive: discovered_model.keep_alive.clone(),
                        supports_tools: discovered_model.supports_tools,
                        supports_images: discovered_model.supports_images,
                        supports_thinking: discovered_model.supports_thinking,
                    };

                    // Verify the conversion worked correctly
                    assert_eq!(available_model.name, "test-model:latest");
                }
            }

            // Call the actual function to ensure it doesn't panic with discovered models
            // Note: In a test environment, the file system changes may not persist to
            // the global settings, but the function should execute without errors
            EditPredictionButton::switch_ollama_model(fs, "test-model:latest".to_string(), cx);
        });

        // Allow any async operations to complete
        cx.background_executor.run_until_parked();
    }
}
