use anyhow::Result;
use copilot::{Copilot, Status};
use editor::{scroll::Autoscroll, Editor};
use feature_flags::{FeatureFlagAppExt, ZetaFeatureFlag};
use fs::Fs;
use gpui::{
    actions, div, Action, AppContext, AsyncWindowContext, Corner, Entity, IntoElement, Model,
    ModelContext, ParentElement, Render, Subscription, WeakModel, Window,
};
use language::{
    language_settings::{
        self, all_language_settings, AllLanguageSettings, InlineCompletionProvider,
    },
    File, Language,
};
use settings::{update_settings_file, Settings, SettingsStore};
use std::{path::Path, sync::Arc};
use supermaven::{AccountStatus, Supermaven};
use workspace::{
    create_and_open_local_file,
    item::ItemHandle,
    notifications::NotificationId,
    ui::{
        ButtonCommon, Clickable, ContextMenu, IconButton, IconName, IconSize, PopoverMenu, Tooltip,
    },
    StatusItemView, Toast, Workspace,
};
use zed_actions::OpenBrowser;
use zeta::RateCompletionModal;

actions!(zeta, [RateCompletions]);

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";

struct CopilotErrorToast;

pub struct InlineCompletionButton {
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
    fs: Arc<dyn Fs>,
    workspace: WeakModel<Workspace>,
}

enum SupermavenButtonStatus {
    Ready,
    Errored(String),
    NeedsActivation(String),
    Initializing,
}

impl Render for InlineCompletionButton {
    fn render(&mut self, window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
        let all_language_settings = all_language_settings(None, cx);

        match all_language_settings.inline_completions.provider {
            InlineCompletionProvider::None => div(),

            InlineCompletionProvider::Copilot => {
                let Some(copilot) = Copilot::global(cx) else {
                    return div();
                };
                let status = copilot.read(cx).status();

                let enabled = self.editor_enabled.unwrap_or_else(|| {
                    all_language_settings.inline_completions_enabled(None, None, cx)
                });

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
                                if let Some(workspace) =
                                    window.window_handle().downcast::<Workspace>()
                                {
                                    workspace
                                        .update(cx, |workspace, window, cx| {
                                            workspace.show_toast(
                                                Toast::new(
                                                    NotificationId::unique::<CopilotErrorToast>(),
                                                    format!("Copilot can't be started: {}", e),
                                                )
                                                .on_click("Reinstall Copilot", |window, cx| {
                                                    if let Some(copilot) = Copilot::global(cx) {
                                                        copilot
                                                            .update(cx, |copilot, cx| {
                                                                copilot.reinstall(cx)
                                                            })
                                                            .detach();
                                                    }
                                                }),
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            }))
                            .tooltip(|window, cx| Tooltip::text("GitHub Copilot", window, cx)),
                    );
                }
                let this = cx.model().clone();

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
                        .trigger(
                            IconButton::new("copilot-icon", icon)
                                .tooltip(|window, cx| Tooltip::text("GitHub Copilot", window, cx)),
                        ),
                )
            }

            InlineCompletionProvider::Supermaven => {
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
                let this = cx.model().clone();
                let fs = self.fs.clone();

                return div().child(
                    PopoverMenu::new("supermaven")
                        .menu(move |window, cx| match &status {
                            SupermavenButtonStatus::NeedsActivation(activate_url) => {
                                Some(ContextMenu::build(window, cx, |menu, _, _| {
                                    let fs = fs.clone();
                                    let activate_url = activate_url.clone();
                                    menu.entry("Sign In", None, move |window, cx| {
                                        cx.open_url(activate_url.as_str())
                                    })
                                    .entry(
                                        "Use Copilot",
                                        None,
                                        move |window, cx| {
                                            set_completion_provider(
                                                fs.clone(),
                                                cx,
                                                InlineCompletionProvider::Copilot,
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
                        .trigger(IconButton::new("supermaven-icon", icon).tooltip(
                            move |window, cx| Tooltip::text(tooltip_text.clone(), window, cx),
                        )),
                );
            }

            InlineCompletionProvider::Zeta => {
                if !cx.has_flag::<ZetaFeatureFlag>() {
                    return div();
                }

                div().child(
                    IconButton::new("zeta", IconName::ZedPredict)
                        .tooltip(|window, cx| {
                            Tooltip::with_meta(
                                "Zed Predict",
                                Some(&RateCompletions),
                                "Click to rate completions",
                                window,
                                cx,
                            )
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(workspace) = this.workspace.upgrade() {
                                workspace.update(cx, |workspace, cx| {
                                    RateCompletionModal::toggle(workspace, window, cx)
                                });
                            }
                        })),
                )
            }
        }
    }
}

impl InlineCompletionButton {
    pub fn new(
        workspace: WeakModel<Workspace>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        if let Some(copilot) = Copilot::global(cx) {
            cx.observe_in(&copilot, window, |_, _, window, cx| cx.notify())
                .detach()
        }

        cx.observe_global_in::<SettingsStore>(window, move |_, window, cx| cx.notify())
            .detach();

        Self {
            editor_subscription: None,
            editor_enabled: None,
            language: None,
            file: None,
            workspace,
            fs,
        }
    }

    pub fn build_copilot_start_menu(
        &mut self,
        window: &mut Window,
        cx: &mut ModelContext<Self>,
    ) -> Model<ContextMenu> {
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
                        set_completion_provider(
                            fs.clone(),
                            cx,
                            InlineCompletionProvider::Supermaven,
                        )
                    }
                })
        })
    }

    pub fn build_language_settings_menu(
        &self,
        mut menu: ContextMenu,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> ContextMenu {
        let fs = self.fs.clone();

        if let Some(language) = self.language.clone() {
            let fs = fs.clone();
            let language_enabled =
                language_settings::language_settings(Some(language.name()), None, cx)
                    .show_inline_completions;

            menu = menu.entry(
                format!(
                    "{} Inline Completions for {}",
                    if language_enabled { "Hide" } else { "Show" },
                    language.name()
                ),
                None,
                move |window, cx| {
                    toggle_inline_completions_for_language(language.clone(), fs.clone(), cx)
                },
            );
        }

        let settings = AllLanguageSettings::get_global(cx);

        if let Some(file) = &self.file {
            let path = file.path().clone();
            let path_enabled = settings.inline_completions_enabled_for_path(&path);

            menu = menu.entry(
                format!(
                    "{} Inline Completions for This Path",
                    if path_enabled { "Hide" } else { "Show" }
                ),
                None,
                move |window, cx| {
                    if let Some(workspace) = window.window_handle().downcast::<Workspace>() {
                        if let Ok(workspace) = workspace.root_view(cx) {
                            let workspace = workspace.downgrade();
                            window
                                .spawn(cx, |cx| {
                                    configure_disabled_globs(
                                        workspace,
                                        path_enabled.then_some(path.clone()),
                                        cx,
                                    )
                                })
                                .detach_and_log_err(cx);
                        }
                    }
                },
            );
        }

        let globally_enabled = settings.inline_completions_enabled(None, None, cx);
        menu.entry(
            if globally_enabled {
                "Hide Inline Completions for All Files"
            } else {
                "Show Inline Completions for All Files"
            },
            None,
            move |window, cx| toggle_inline_completions_globally(fs.clone(), cx),
        )
    }

    fn build_copilot_context_menu(
        &self,
        window: &mut Window,
        cx: &mut ModelContext<Self>,
    ) -> Model<ContextMenu> {
        ContextMenu::build(window, cx, |menu, window, cx| {
            self.build_language_settings_menu(menu, window, cx)
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
        cx: &mut ModelContext<Self>,
    ) -> Model<ContextMenu> {
        ContextMenu::build(window, cx, |menu, window, cx| {
            self.build_language_settings_menu(menu, window, cx)
                .separator()
                .action("Sign Out", supermaven::SignOut.boxed_clone())
        })
    }

    pub fn update_enabled(
        &mut self,
        editor: Model<Editor>,
        window: &mut Window,
        cx: &mut ModelContext<Self>,
    ) {
        let editor = editor.read(cx);
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let suggestion_anchor = editor.selections.newest_anchor().start;
        let language = snapshot.language_at(suggestion_anchor);
        let file = snapshot.file_at(suggestion_anchor).cloned();
        self.editor_enabled = {
            let file = file.as_ref();
            Some(
                file.map(|file| !file.is_private()).unwrap_or(true)
                    && all_language_settings(file, cx).inline_completions_enabled(
                        language,
                        file.map(|file| file.path().as_ref()),
                        cx,
                    ),
            )
        };
        self.language = language.cloned();
        self.file = file;

        cx.notify()
    }
}

impl StatusItemView for InlineCompletionButton {
    fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(editor) = item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.editor_subscription = Some((
                cx.observe_in(&editor, window, Self::update_enabled),
                editor.entity_id().as_u64() as usize,
            ));
            self.update_enabled(editor, window, cx);
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
}

async fn configure_disabled_globs(
    workspace: WeakModel<Workspace>,
    path_to_disable: Option<Arc<Path>>,
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
            let edits = settings.edits_for_update::<AllLanguageSettings>(&text, |file| {
                let copilot = file.inline_completions.get_or_insert_with(Default::default);
                let globs = copilot.disabled_globs.get_or_insert_with(|| {
                    settings
                        .get::<AllLanguageSettings>(None)
                        .inline_completions
                        .disabled_globs
                        .iter()
                        .map(|glob| glob.glob().to_string())
                        .collect()
                });

                if let Some(path_to_disable) = &path_to_disable {
                    globs.push(path_to_disable.to_string_lossy().into_owned());
                } else {
                    globs.clear();
                }
            });

            if !edits.is_empty() {
                item.change_selections(Some(Autoscroll::newest()), window, cx, |selections| {
                    selections.select_ranges(edits.iter().map(|e| e.0.clone()));
                });

                // When *enabling* a path, don't actually perform an edit, just select the range.
                if path_to_disable.is_some() {
                    item.edit(edits.iter().cloned(), window, cx);
                }
            }
        })?;

    anyhow::Ok(())
}

fn toggle_inline_completions_globally(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    let show_inline_completions =
        all_language_settings(None, cx).inline_completions_enabled(None, None, cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.defaults.show_inline_completions = Some(!show_inline_completions)
    });
}

fn set_completion_provider(
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
    provider: InlineCompletionProvider,
) {
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.features
            .get_or_insert(Default::default())
            .inline_completion_provider = Some(provider);
    });
}

fn toggle_inline_completions_for_language(
    language: Arc<Language>,
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
) {
    let show_inline_completions =
        all_language_settings(None, cx).inline_completions_enabled(Some(&language), None, cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.languages
            .entry(language.name())
            .or_default()
            .show_inline_completions = Some(!show_inline_completions);
    });
}

fn hide_copilot(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file, _| {
        file.features
            .get_or_insert(Default::default())
            .inline_completion_provider = Some(InlineCompletionProvider::None);
    });
}
