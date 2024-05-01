use anyhow::Result;
use copilot::{Copilot, CopilotCodeVerification, SignOut, Status};
use editor::{scroll::Autoscroll, Editor};
use fs::Fs;
use gpui::{
    div, Action, AnchorCorner, AppContext, AsyncWindowContext, Entity, IntoElement, ParentElement,
    Render, Subscription, View, ViewContext, WeakView, WindowContext,
};
use language::{
    language_settings::{
        self, all_language_settings, AllLanguageSettings, InlineCompletionProvider,
    },
    File, Language,
};
use settings::{update_settings_file, Settings, SettingsStore};
use std::{path::Path, sync::Arc};
use util::{paths, ResultExt};
use workspace::{
    create_and_open_local_file,
    item::ItemHandle,
    notifications::NotificationId,
    ui::{
        popover_menu, ButtonCommon, Clickable, ContextMenu, IconButton, IconName, IconSize, Tooltip,
    },
    StatusItemView, Toast, Workspace,
};
use zed_actions::OpenBrowser;

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";

struct CopilotStartingToast;

struct CopilotErrorToast;

pub struct CopilotButton {
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
    fs: Arc<dyn Fs>,
}

impl Render for CopilotButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let all_language_settings = all_language_settings(None, cx);
        if all_language_settings.inline_completions.provider != InlineCompletionProvider::Copilot {
            return div();
        }

        let Some(copilot) = Copilot::global(cx) else {
            return div();
        };
        let status = copilot.read(cx).status();

        let enabled = self
            .editor_enabled
            .unwrap_or_else(|| all_language_settings.inline_completions_enabled(None, None));

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
                    .on_click(cx.listener(move |_, _, cx| {
                        if let Some(workspace) = cx.window_handle().downcast::<Workspace>() {
                            workspace
                                .update(cx, |workspace, cx| {
                                    workspace.show_toast(
                                        Toast::new(
                                            NotificationId::unique::<CopilotErrorToast>(),
                                            format!("Copilot can't be started: {}", e),
                                        )
                                        .on_click(
                                            "Reinstall Copilot",
                                            |cx| {
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
                                })
                                .ok();
                        }
                    }))
                    .tooltip(|cx| Tooltip::text("GitHub Copilot", cx)),
            );
        }
        let this = cx.view().clone();

        div().child(
            popover_menu("copilot")
                .menu(move |cx| match status {
                    Status::Authorized => {
                        Some(this.update(cx, |this, cx| this.build_copilot_menu(cx)))
                    }
                    _ => Some(this.update(cx, |this, cx| this.build_copilot_start_menu(cx))),
                })
                .anchor(AnchorCorner::BottomRight)
                .trigger(
                    IconButton::new("copilot-icon", icon)
                        .tooltip(|cx| Tooltip::text("GitHub Copilot", cx)),
                ),
        )
    }
}

impl CopilotButton {
    pub fn new(fs: Arc<dyn Fs>, cx: &mut ViewContext<Self>) -> Self {
        if let Some(copilot) = Copilot::global(cx) {
            cx.observe(&copilot, |_, _, cx| cx.notify()).detach()
        }

        cx.observe_global::<SettingsStore>(move |_, cx| cx.notify())
            .detach();

        Self {
            editor_subscription: None,
            editor_enabled: None,
            language: None,
            file: None,
            fs,
        }
    }

    pub fn build_copilot_start_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        let fs = self.fs.clone();
        ContextMenu::build(cx, |menu, _| {
            menu.entry("Sign In", None, initiate_sign_in).entry(
                "Disable Copilot",
                None,
                move |cx| hide_copilot(fs.clone(), cx),
            )
        })
    }

    pub fn build_copilot_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        let fs = self.fs.clone();

        ContextMenu::build(cx, move |mut menu, cx| {
            if let Some(language) = self.language.clone() {
                let fs = fs.clone();
                let language_enabled =
                    language_settings::language_settings(Some(&language), None, cx)
                        .show_inline_completions;

                menu = menu.entry(
                    format!(
                        "{} Inline Completions for {}",
                        if language_enabled { "Hide" } else { "Show" },
                        language.name()
                    ),
                    None,
                    move |cx| {
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
                    move |cx| {
                        if let Some(workspace) = cx.window_handle().downcast::<Workspace>() {
                            if let Ok(workspace) = workspace.root_view(cx) {
                                let workspace = workspace.downgrade();
                                cx.spawn(|cx| {
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

            let globally_enabled = settings.inline_completions_enabled(None, None);
            menu.entry(
                if globally_enabled {
                    "Hide Inline Completions for All Files"
                } else {
                    "Show Inline Completions for All Files"
                },
                None,
                move |cx| toggle_inline_completions_globally(fs.clone(), cx),
            )
            .separator()
            .link(
                "Copilot Settings",
                OpenBrowser {
                    url: COPILOT_SETTINGS_URL.to_string(),
                }
                .boxed_clone(),
            )
            .action("Sign Out", SignOut.boxed_clone())
        })
    }

    pub fn update_enabled(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
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
                    ),
            )
        };
        self.language = language.cloned();
        self.file = file;

        cx.notify()
    }
}

impl StatusItemView for CopilotButton {
    fn set_active_pane_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
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

async fn configure_disabled_globs(
    workspace: WeakView<Workspace>,
    path_to_disable: Option<Arc<Path>>,
    mut cx: AsyncWindowContext,
) -> Result<()> {
    let settings_editor = workspace
        .update(&mut cx, |_, cx| {
            create_and_open_local_file(&paths::SETTINGS, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
        })?
        .await?
        .downcast::<Editor>()
        .unwrap();

    settings_editor.downgrade().update(&mut cx, |item, cx| {
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
            item.change_selections(Some(Autoscroll::newest()), cx, |selections| {
                selections.select_ranges(edits.iter().map(|e| e.0.clone()));
            });

            // When *enabling* a path, don't actually perform an edit, just select the range.
            if path_to_disable.is_some() {
                item.edit(edits.iter().cloned(), cx);
            }
        }
    })?;

    anyhow::Ok(())
}

fn toggle_inline_completions_globally(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    let show_inline_completions =
        all_language_settings(None, cx).inline_completions_enabled(None, None);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file| {
        file.defaults.show_inline_completions = Some(!show_inline_completions)
    });
}

fn toggle_inline_completions_for_language(
    language: Arc<Language>,
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
) {
    let show_inline_completions =
        all_language_settings(None, cx).inline_completions_enabled(Some(&language), None);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file| {
        file.languages
            .entry(language.name())
            .or_default()
            .show_inline_completions = Some(!show_inline_completions);
    });
}

fn hide_copilot(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file| {
        file.features.get_or_insert(Default::default()).copilot = Some(false);
    });
}

pub fn initiate_sign_in(cx: &mut WindowContext) {
    let Some(copilot) = Copilot::global(cx) else {
        return;
    };
    let status = copilot.read(cx).status();
    let Some(workspace) = cx.window_handle().downcast::<Workspace>() else {
        return;
    };
    match status {
        Status::Starting { task } => {
            let Some(workspace) = cx.window_handle().downcast::<Workspace>() else {
                return;
            };

            let Ok(workspace) = workspace.update(cx, |workspace, cx| {
                workspace.show_toast(
                    Toast::new(
                        NotificationId::unique::<CopilotStartingToast>(),
                        "Copilot is starting...",
                    ),
                    cx,
                );
                workspace.weak_handle()
            }) else {
                return;
            };

            cx.spawn(|mut cx| async move {
                task.await;
                if let Some(copilot) = cx.update(|cx| Copilot::global(cx)).ok().flatten() {
                    workspace
                        .update(&mut cx, |workspace, cx| match copilot.read(cx).status() {
                            Status::Authorized => workspace.show_toast(
                                Toast::new(
                                    NotificationId::unique::<CopilotStartingToast>(),
                                    "Copilot has started!",
                                ),
                                cx,
                            ),
                            _ => {
                                workspace.dismiss_toast(
                                    &NotificationId::unique::<CopilotStartingToast>(),
                                    cx,
                                );
                                copilot
                                    .update(cx, |copilot, cx| copilot.sign_in(cx))
                                    .detach_and_log_err(cx);
                            }
                        })
                        .log_err();
                }
            })
            .detach();
        }
        _ => {
            copilot.update(cx, |this, cx| this.sign_in(cx)).detach();
            workspace
                .update(cx, |this, cx| {
                    this.toggle_modal(cx, |cx| CopilotCodeVerification::new(&copilot, cx));
                })
                .ok();
        }
    }
}
