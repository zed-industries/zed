use anyhow::Result;
use client::{Client, UserStore, zed_urls};
use cloud_llm_client::UsageLimit;
use codestral::CodestralEditPredictionDelegate;
use copilot::Status;
use edit_prediction::{
    EditPredictionStore, MercuryFeatureFlag, SweepFeatureFlag, Zeta2FeatureFlag,
};
use edit_prediction_types::EditPredictionDelegateHandle;
use editor::{
    Editor, MultiBufferOffset, SelectionEffects, actions::ShowEditPrediction, scroll::Autoscroll,
};
use feature_flags::FeatureFlagAppExt;
use fs::Fs;
use gpui::{
    Action, Animation, AnimationExt, App, AsyncWindowContext, Corner, Entity, FocusHandle,
    Focusable, IntoElement, ParentElement, Render, Subscription, WeakEntity, actions, div,
    ease_in_out, pulsating_between,
};
use indoc::indoc;
use language::{
    EditPredictionsMode, File, Language,
    language_settings::{self, AllLanguageSettings, EditPredictionProvider, all_language_settings},
};
use project::{DisableAiSettings, Project};
use regex::Regex;
use settings::{
    EXPERIMENTAL_MERCURY_EDIT_PREDICTION_PROVIDER_NAME,
    EXPERIMENTAL_SWEEP_EDIT_PREDICTION_PROVIDER_NAME,
    EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME, Settings, SettingsStore,
    update_settings_file,
};
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use supermaven::{AccountStatus, Supermaven};
use ui::{
    Clickable, ContextMenu, ContextMenuEntry, DocumentationSide, IconButton, IconButtonShape,
    Indicator, PopoverMenu, PopoverMenuHandle, ProgressBar, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{
    StatusItemView, Toast, Workspace, create_and_open_local_file, item::ItemHandle,
    notifications::NotificationId,
};
use zed_actions::{OpenBrowser, OpenSettingsAt};

use crate::{
    CaptureExample, RatePredictions, rate_prediction_modal::PredictEditsRatePredictionsFeatureFlag,
};

actions!(
    edit_prediction,
    [
        /// Toggles the edit prediction menu.
        ToggleMenu
    ]
);

const COPILOT_SETTINGS_PATH: &str = "/settings/copilot";
const COPILOT_SETTINGS_URL: &str = concat!("https://github.com", "/settings/copilot");
const PRIVACY_DOCS: &str = "https://zed.dev/docs/ai/privacy-and-security";

struct CopilotErrorToast;

pub struct EditPredictionButton {
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    editor_show_predictions: bool,
    editor_focus_handle: Option<FocusHandle>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
    edit_prediction_provider: Option<Arc<dyn EditPredictionDelegateHandle>>,
    fs: Arc<dyn Fs>,
    user_store: Entity<UserStore>,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    project: WeakEntity<Project>,
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
            return div().hidden();
        }

        let all_language_settings = all_language_settings(None, cx);

        match all_language_settings.edit_predictions.provider {
            EditPredictionProvider::Copilot => {
                let Some(copilot) = EditPredictionStore::try_global(cx)
                    .and_then(|store| store.read(cx).copilot_for_project(&self.project.upgrade()?))
                else {
                    return div().hidden();
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
                                        let copilot = copilot.clone();
                                        workspace.show_toast(
                                            Toast::new(
                                                NotificationId::unique::<CopilotErrorToast>(),
                                                format!("Copilot can't be started: {}", e),
                                            )
                                            .on_click(
                                                "Reinstall Copilot",
                                                move |window, cx| {
                                                    copilot_ui::reinstall_and_sign_in(
                                                        copilot.clone(),
                                                        window,
                                                        cx,
                                                    )
                                                },
                                            ),
                                            cx,
                                        );
                                    });
                                }
                            }))
                            .tooltip(|_window, cx| {
                                Tooltip::for_action("GitHub Copilot", &ToggleMenu, cx)
                            }),
                    );
                }
                let this = cx.weak_entity();
                let project = self.project.clone();
                div().child(
                    PopoverMenu::new("copilot")
                        .menu(move |window, cx| {
                            let current_status = EditPredictionStore::try_global(cx)
                                .and_then(|store| {
                                    store.read(cx).copilot_for_project(&project.upgrade()?)
                                })?
                                .read(cx)
                                .status();
                            match current_status {
                                Status::Authorized => this.update(cx, |this, cx| {
                                    this.build_copilot_context_menu(window, cx)
                                }),
                                _ => this.update(cx, |this, cx| {
                                    this.build_copilot_start_menu(window, cx)
                                }),
                            }
                            .ok()
                        })
                        .anchor(Corner::BottomRight)
                        .trigger_with_tooltip(
                            IconButton::new("copilot-icon", icon),
                            |_window, cx| Tooltip::for_action("GitHub Copilot", &ToggleMenu, cx),
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
                let this = cx.weak_entity();
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
                            SupermavenButtonStatus::Ready => this
                                .update(cx, |this, cx| {
                                    this.build_supermaven_context_menu(window, cx)
                                })
                                .ok(),
                            _ => None,
                        })
                        .anchor(Corner::BottomRight)
                        .trigger_with_tooltip(
                            IconButton::new("supermaven-icon", icon),
                            move |window, cx| {
                                if has_menu {
                                    Tooltip::for_action(tooltip_text.clone(), &ToggleMenu, cx)
                                } else {
                                    Tooltip::text(tooltip_text.clone())(window, cx)
                                }
                            },
                        )
                        .with_handle(self.popover_menu_handle.clone()),
                )
            }

            EditPredictionProvider::Codestral => {
                let enabled = self.editor_enabled.unwrap_or(true);
                let has_api_key = CodestralEditPredictionDelegate::has_api_key(cx);
                let this = cx.weak_entity();

                let tooltip_meta = if has_api_key {
                    "Powered by Codestral"
                } else {
                    "Missing API key for Codestral"
                };

                div().child(
                    PopoverMenu::new("codestral")
                        .menu(move |window, cx| {
                            this.update(cx, |this, cx| {
                                this.build_codestral_context_menu(window, cx)
                            })
                            .ok()
                        })
                        .anchor(Corner::BottomRight)
                        .trigger_with_tooltip(
                            IconButton::new("codestral-icon", IconName::AiMistral)
                                .shape(IconButtonShape::Square)
                                .when(!has_api_key, |this| {
                                    this.indicator(Indicator::dot().color(Color::Error))
                                        .indicator_border_color(Some(
                                            cx.theme().colors().status_bar_background,
                                        ))
                                })
                                .when(has_api_key && !enabled, |this| {
                                    this.indicator(Indicator::dot().color(Color::Ignored))
                                        .indicator_border_color(Some(
                                            cx.theme().colors().status_bar_background,
                                        ))
                                }),
                            move |_window, cx| {
                                Tooltip::with_meta(
                                    "Edit Prediction",
                                    Some(&ToggleMenu),
                                    tooltip_meta,
                                    cx,
                                )
                            },
                        )
                        .with_handle(self.popover_menu_handle.clone()),
                )
            }
            provider @ (EditPredictionProvider::Experimental(_) | EditPredictionProvider::Zed) => {
                let enabled = self.editor_enabled.unwrap_or(true);

                let ep_icon;
                let tooltip_meta;
                let mut missing_token = false;

                match provider {
                    EditPredictionProvider::Experimental(
                        EXPERIMENTAL_SWEEP_EDIT_PREDICTION_PROVIDER_NAME,
                    ) => {
                        ep_icon = IconName::SweepAi;
                        tooltip_meta = if missing_token {
                            "Missing API key for Sweep"
                        } else {
                            "Powered by Sweep"
                        };
                        missing_token = edit_prediction::EditPredictionStore::try_global(cx)
                            .is_some_and(|ep_store| !ep_store.read(cx).has_sweep_api_token(cx));
                    }
                    EditPredictionProvider::Experimental(
                        EXPERIMENTAL_MERCURY_EDIT_PREDICTION_PROVIDER_NAME,
                    ) => {
                        ep_icon = IconName::Inception;
                        missing_token = edit_prediction::EditPredictionStore::try_global(cx)
                            .is_some_and(|ep_store| !ep_store.read(cx).has_mercury_api_token(cx));
                        tooltip_meta = if missing_token {
                            "Missing API key for Mercury"
                        } else {
                            "Powered by Mercury"
                        };
                    }
                    _ => {
                        ep_icon = if enabled {
                            IconName::ZedPredict
                        } else {
                            IconName::ZedPredictDisabled
                        };
                        tooltip_meta = "Powered by Zeta"
                    }
                };

                if edit_prediction::should_show_upsell_modal() {
                    let tooltip_meta = if self.user_store.read(cx).current_user().is_some() {
                        "Choose a Plan"
                    } else {
                        "Sign In To Use"
                    };

                    return div().child(
                        IconButton::new("zed-predict-pending-button", ep_icon)
                            .shape(IconButtonShape::Square)
                            .indicator(Indicator::dot().color(Color::Muted))
                            .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                            .tooltip(move |_window, cx| {
                                Tooltip::with_meta("Edit Predictions", None, tooltip_meta, cx)
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
                let user = self.user_store.read(cx).current_user();

                let indicator_color = if missing_token {
                    Some(Color::Error)
                } else if enabled && (!show_editor_predictions || over_limit) {
                    Some(if over_limit {
                        Color::Error
                    } else {
                        Color::Muted
                    })
                } else {
                    None
                };

                let icon_button = IconButton::new("zed-predict-pending-button", ep_icon)
                    .shape(IconButtonShape::Square)
                    .when_some(indicator_color, |this, color| {
                        this.indicator(Indicator::dot().color(color))
                            .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                    })
                    .when(!self.popover_menu_handle.is_deployed(), |element| {
                        let user = user.clone();

                        element.tooltip(move |_window, cx| {
                            let description = if enabled {
                                if show_editor_predictions {
                                    tooltip_meta
                                } else if user.is_none() {
                                    "Sign In To Use"
                                } else {
                                    "Hidden For This File"
                                }
                            } else {
                                "Disabled For This File"
                            };

                            Tooltip::with_meta(
                                "Edit Prediction",
                                Some(&ToggleMenu),
                                description,
                                cx,
                            )
                        })
                    });

                let this = cx.weak_entity();

                let mut popover_menu = PopoverMenu::new("edit-prediction")
                    .when(user.is_some(), |popover_menu| {
                        let this = this.clone();

                        popover_menu.menu(move |window, cx| {
                            this.update(cx, |this, cx| {
                                this.build_edit_prediction_context_menu(provider, window, cx)
                            })
                            .ok()
                        })
                    })
                    .when(user.is_none(), |popover_menu| {
                        let this = this.clone();

                        popover_menu.menu(move |window, cx| {
                            this.update(cx, |this, cx| {
                                this.build_zeta_upsell_context_menu(window, cx)
                            })
                            .ok()
                        })
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

            EditPredictionProvider::None => div().hidden(),
        }
    }
}

impl EditPredictionButton {
    pub fn new(
        fs: Arc<dyn Fs>,
        user_store: Entity<UserStore>,
        popover_menu_handle: PopoverMenuHandle<ContextMenu>,
        client: Arc<Client>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let copilot = EditPredictionStore::try_global(cx).and_then(|store| {
            store.update(cx, |this, cx| this.start_copilot_for_project(&project, cx))
        });
        if let Some(copilot) = copilot {
            cx.observe(&copilot, |_, _, cx| cx.notify()).detach()
        }

        cx.observe_global::<SettingsStore>(move |_, cx| cx.notify())
            .detach();

        cx.observe_global::<EditPredictionStore>(move |_, cx| cx.notify())
            .detach();

        let sweep_api_token_task = edit_prediction::sweep_ai::load_sweep_api_token(cx);
        let mercury_api_token_task = edit_prediction::mercury::load_mercury_api_token(cx);

        cx.spawn(async move |this, cx| {
            _ = futures::join!(sweep_api_token_task, mercury_api_token_task);
            this.update(cx, |_, cx| {
                cx.notify();
            })
            .ok();
        })
        .detach();

        CodestralEditPredictionDelegate::ensure_api_key_loaded(client.http_client(), cx);

        Self {
            editor_subscription: None,
            editor_enabled: None,
            editor_show_predictions: true,
            editor_focus_handle: None,
            language: None,
            file: None,
            edit_prediction_provider: None,
            user_store,
            popover_menu_handle,
            project: project.downgrade(),
            fs,
        }
    }

    fn get_available_providers(&self, cx: &mut App) -> Vec<EditPredictionProvider> {
        let mut providers = Vec::new();

        providers.push(EditPredictionProvider::Zed);

        if cx.has_flag::<Zeta2FeatureFlag>() {
            providers.push(EditPredictionProvider::Experimental(
                EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME,
            ));
        }

        if let Some(_) = EditPredictionStore::try_global(cx)
            .and_then(|store| store.read(cx).copilot_for_project(&self.project.upgrade()?))
        {
            providers.push(EditPredictionProvider::Copilot);
        }

        if let Some(supermaven) = Supermaven::global(cx) {
            if let Supermaven::Spawned(agent) = supermaven.read(cx) {
                if matches!(agent.account_status, AccountStatus::Ready) {
                    providers.push(EditPredictionProvider::Supermaven);
                }
            }
        }

        if CodestralEditPredictionDelegate::has_api_key(cx) {
            providers.push(EditPredictionProvider::Codestral);
        }

        if cx.has_flag::<SweepFeatureFlag>()
            && edit_prediction::sweep_ai::sweep_api_token(cx)
                .read(cx)
                .has_key()
        {
            providers.push(EditPredictionProvider::Experimental(
                EXPERIMENTAL_SWEEP_EDIT_PREDICTION_PROVIDER_NAME,
            ));
        }

        if cx.has_flag::<MercuryFeatureFlag>()
            && edit_prediction::mercury::mercury_api_token(cx)
                .read(cx)
                .has_key()
        {
            providers.push(EditPredictionProvider::Experimental(
                EXPERIMENTAL_MERCURY_EDIT_PREDICTION_PROVIDER_NAME,
            ));
        }

        providers
    }

    fn add_provider_switching_section(
        &self,
        mut menu: ContextMenu,
        current_provider: EditPredictionProvider,
        cx: &mut App,
    ) -> ContextMenu {
        let available_providers = self.get_available_providers(cx);

        let providers: Vec<_> = available_providers
            .into_iter()
            .filter(|p| *p != EditPredictionProvider::None)
            .collect();

        if !providers.is_empty() {
            menu = menu.separator().header("Providers");

            for provider in providers {
                let is_current = provider == current_provider;
                let fs = self.fs.clone();

                let name = match provider {
                    EditPredictionProvider::Zed => "Zed AI",
                    EditPredictionProvider::Copilot => "GitHub Copilot",
                    EditPredictionProvider::Supermaven => "Supermaven",
                    EditPredictionProvider::Codestral => "Codestral",
                    EditPredictionProvider::Experimental(
                        EXPERIMENTAL_SWEEP_EDIT_PREDICTION_PROVIDER_NAME,
                    ) => "Sweep",
                    EditPredictionProvider::Experimental(
                        EXPERIMENTAL_MERCURY_EDIT_PREDICTION_PROVIDER_NAME,
                    ) => "Mercury",
                    EditPredictionProvider::Experimental(
                        EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME,
                    ) => "Zeta2",
                    EditPredictionProvider::None | EditPredictionProvider::Experimental(_) => {
                        continue;
                    }
                };

                menu = menu.item(
                    ContextMenuEntry::new(name)
                        .toggleable(IconPosition::Start, is_current)
                        .handler(move |_, cx| {
                            set_completion_provider(fs.clone(), cx, provider);
                        }),
                )
            }
        }

        menu
    }

    pub fn build_copilot_start_menu(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let fs = self.fs.clone();
        let project = self.project.clone();
        ContextMenu::build(window, cx, |menu, _, _| {
            menu.entry("Sign In to Copilot", None, move |window, cx| {
                if let Some(copilot) = EditPredictionStore::try_global(cx).and_then(|store| {
                    store.update(cx, |this, cx| {
                        this.start_copilot_for_project(&project.upgrade()?, cx)
                    })
                }) {
                    copilot_ui::initiate_sign_in(copilot, window, cx);
                }
            })
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
                            .documentation_aside(DocumentationSide::Left, move |_cx| {
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

        menu = menu
                .separator()
                .header("Display Modes")
                .item(
                    ContextMenuEntry::new("Eager")
                        .toggleable(IconPosition::Start, eager_mode)
                        .documentation_aside(DocumentationSide::Left, move |_| {
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
                        .documentation_aside(DocumentationSide::Left, move |_| {
                            Label::new("Display predictions inline only when holding a modifier key (alt by default).").into_any_element()
                        })
                        .handler({
                            let fs = fs.clone();
                            move |_, cx| {
                                toggle_edit_prediction_mode(fs.clone(), EditPredictionsMode::Subtle, cx)
                            }
                        }),
                );

        menu = menu.separator().header("Privacy");

        if matches!(
            provider,
            EditPredictionProvider::Zed
                | EditPredictionProvider::Experimental(
                    EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME,
                )
        ) {
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
                            .disabled(cx.is_staff())
                            .documentation_aside(DocumentationSide::Left, move |cx| {
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
        }

        menu = menu.item(
            ContextMenuEntry::new("Configure Excluded Files")
                .icon(IconName::LockOutlined)
                .icon_color(Color::Muted)
                .documentation_aside(DocumentationSide::Left, |_| {
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
            ContextMenuEntry::new("View Docs")
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
                .header("Actions")
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
                .context(editor_focus_handle)
                .when(
                    cx.has_flag::<PredictEditsRatePredictionsFeatureFlag>(),
                    |this| {
                        this.action("Capture Prediction Example", CaptureExample.boxed_clone())
                            .action("Rate Predictions", RatePredictions.boxed_clone())
                    },
                );
        }

        menu
    }

    fn build_copilot_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let all_language_settings = all_language_settings(None, cx);
        let next_edit_suggestions = all_language_settings
            .edit_predictions
            .copilot
            .enable_next_edit_suggestions
            .unwrap_or(true);
        let copilot_config = copilot_chat::CopilotChatConfiguration {
            enterprise_uri: all_language_settings
                .edit_predictions
                .copilot
                .enterprise_uri
                .clone(),
        };
        let settings_url = copilot_settings_url(copilot_config.enterprise_uri.as_deref());

        ContextMenu::build(window, cx, |menu, window, cx| {
            let menu = self.build_language_settings_menu(menu, window, cx);
            let menu =
                self.add_provider_switching_section(menu, EditPredictionProvider::Copilot, cx);

            menu.separator()
                .item(
                    ContextMenuEntry::new("Copilot: Next Edit Suggestions")
                        .toggleable(IconPosition::Start, next_edit_suggestions)
                        .handler({
                            let fs = self.fs.clone();
                            move |_, cx| {
                                update_settings_file(fs.clone(), cx, move |settings, _| {
                                    settings
                                        .project
                                        .all_languages
                                        .edit_predictions
                                        .get_or_insert_default()
                                        .copilot
                                        .get_or_insert_default()
                                        .enable_next_edit_suggestions =
                                        Some(!next_edit_suggestions);
                                });
                            }
                        }),
                )
                .separator()
                .link(
                    "Go to Copilot Settings",
                    OpenBrowser { url: settings_url }.boxed_clone(),
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
            let menu = self.build_language_settings_menu(menu, window, cx);
            let menu =
                self.add_provider_switching_section(menu, EditPredictionProvider::Supermaven, cx);

            menu.separator()
                .action("Sign Out", supermaven::SignOut.boxed_clone())
        })
    }

    fn build_codestral_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |menu, window, cx| {
            let menu = self.build_language_settings_menu(menu, window, cx);
            let menu =
                self.add_provider_switching_section(menu, EditPredictionProvider::Codestral, cx);

            menu
        })
    }

    fn build_edit_prediction_context_menu(
        &self,
        provider: EditPredictionProvider,
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

            menu = self.build_language_settings_menu(menu, window, cx);

            menu = self.add_provider_switching_section(menu, provider, cx);
            menu = menu.separator().item(
                ContextMenuEntry::new("Configure Providers")
                    .icon(IconName::Settings)
                    .icon_position(IconPosition::Start)
                    .icon_color(Color::Muted)
                    .handler(move |window, cx| {
                        window.dispatch_action(
                            OpenSettingsAt {
                                path: "edit_predictions.providers".to_string(),
                            }
                            .boxed_clone(),
                            cx,
                        );
                    }),
            );

            menu
        })
    }

    fn build_zeta_upsell_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |mut menu, _window, cx| {
            menu = menu
                .custom_row(move |_window, cx| {
                    let description = indoc! {
                        "You get 2,000 accepted suggestions at every keystroke for free, \
                        powered by Zeta, our open-source, open-data model"
                    };

                    v_flex()
                        .max_w_64()
                        .h(rems_from_px(148.))
                        .child(render_zeta_tab_animation(cx))
                        .child(Label::new("Edit Prediction"))
                        .child(
                            Label::new(description)
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        )
                        .into_any_element()
                })
                .separator()
                .entry("Sign In & Start Using", None, |window, cx| {
                    let client = Client::global(cx);
                    window
                        .spawn(cx, async move |cx| {
                            client
                                .sign_in_with_optional_connect(true, &cx)
                                .await
                                .log_err();
                        })
                        .detach();
                })
                .link(
                    "Learn More",
                    OpenBrowser {
                        url: zed_urls::edit_prediction_docs(cx),
                    }
                    .boxed_clone(),
                );

            menu
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
                item.edit(
                    edits
                        .into_iter()
                        .map(|(r, s)| (MultiBufferOffset(r.start)..MultiBufferOffset(r.end), s)),
                    cx,
                );
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
                let range = MultiBufferOffset(range.start)..MultiBufferOffset(range.end);
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
            .entry(language.name().0.to_string())
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

fn render_zeta_tab_animation(cx: &App) -> impl IntoElement {
    let tab = |n: u64, inverted: bool| {
        let text_color = cx.theme().colors().text;

        h_flex().child(
            h_flex()
                .text_size(TextSize::XSmall.rems(cx))
                .text_color(text_color)
                .child("tab")
                .with_animation(
                    ElementId::Integer(n),
                    Animation::new(Duration::from_secs(3)).repeat(),
                    move |tab, delta| {
                        let n_f32 = n as f32;

                        let offset = if inverted {
                            0.2 * (4.0 - n_f32)
                        } else {
                            0.2 * n_f32
                        };

                        let phase = (delta - offset + 1.0) % 1.0;
                        let pulse = if phase < 0.6 {
                            let t = phase / 0.6;
                            1.0 - (0.5 - t).abs() * 2.0
                        } else {
                            0.0
                        };

                        let eased = ease_in_out(pulse);
                        let opacity = 0.1 + 0.5 * eased;

                        tab.text_color(text_color.opacity(opacity))
                    },
                ),
        )
    };

    let tab_sequence = |inverted: bool| {
        h_flex()
            .gap_1()
            .child(tab(0, inverted))
            .child(tab(1, inverted))
            .child(tab(2, inverted))
            .child(tab(3, inverted))
            .child(tab(4, inverted))
    };

    h_flex()
        .my_1p5()
        .p_4()
        .justify_center()
        .gap_2()
        .rounded_xs()
        .border_1()
        .border_dashed()
        .border_color(cx.theme().colors().border)
        .bg(gpui::pattern_slash(
            cx.theme().colors().border.opacity(0.5),
            1.,
            8.,
        ))
        .child(tab_sequence(true))
        .child(Icon::new(IconName::ZedPredict))
        .child(tab_sequence(false))
}

fn copilot_settings_url(enterprise_uri: Option<&str>) -> String {
    match enterprise_uri {
        Some(uri) => {
            format!("{}{}", uri.trim_end_matches('/'), COPILOT_SETTINGS_PATH)
        }
        None => COPILOT_SETTINGS_URL.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_copilot_settings_url_with_enterprise_uri(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        cx.update_global(|settings_store: &mut SettingsStore, cx| {
            settings_store
                .set_user_settings(
                    r#"{"edit_predictions":{"copilot":{"enterprise_uri":"https://my-company.ghe.com"}}}"#,
                    cx,
                )
                .unwrap();
        });

        let url = cx.update(|cx| {
            let all_language_settings = all_language_settings(None, cx);
            copilot_settings_url(
                all_language_settings
                    .edit_predictions
                    .copilot
                    .enterprise_uri
                    .as_deref(),
            )
        });

        assert_eq!(url, "https://my-company.ghe.com/settings/copilot");
    }

    #[gpui::test]
    async fn test_copilot_settings_url_with_enterprise_uri_trailing_slash(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        cx.update_global(|settings_store: &mut SettingsStore, cx| {
            settings_store
                .set_user_settings(
                    r#"{"edit_predictions":{"copilot":{"enterprise_uri":"https://my-company.ghe.com/"}}}"#,
                    cx,
                )
                .unwrap();
        });

        let url = cx.update(|cx| {
            let all_language_settings = all_language_settings(None, cx);
            copilot_settings_url(
                all_language_settings
                    .edit_predictions
                    .copilot
                    .enterprise_uri
                    .as_deref(),
            )
        });

        assert_eq!(url, "https://my-company.ghe.com/settings/copilot");
    }

    #[gpui::test]
    async fn test_copilot_settings_url_without_enterprise_uri(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        let url = cx.update(|cx| {
            let all_language_settings = all_language_settings(None, cx);
            copilot_settings_url(
                all_language_settings
                    .edit_predictions
                    .copilot
                    .enterprise_uri
                    .as_deref(),
            )
        });

        assert_eq!(url, "https://github.com/settings/copilot");
    }
}
