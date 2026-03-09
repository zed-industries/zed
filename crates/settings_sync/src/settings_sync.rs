use credentials_provider::CredentialsProvider;
use editor::Editor;
use gpui::{
    div, Action, App, AsyncApp, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, Styled, WeakEntity,
    Window,
};
use schemars::JsonSchema;
use serde::Deserialize;
use settings::Settings;
use ui::prelude::*;
use util::ResultExt;
use workspace::{notifications::NotificationId, with_active_or_new_workspace, ModalView, Toast, Workspace};
use zed_actions::{PullSettingsFromGit, SetSettingsSyncToken, SyncSettingsToGit};

mod sync_engine;

const CREDENTIALS_URL: &str = "https://zed.dev/settings-sync";

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct SyncSettings {
    pub git: GitSyncSettings,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct GitSyncSettings {
    pub repo_url: Option<String>,
    pub branch: String,
}

impl Settings for SyncSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let mut settings = Self {
            git: GitSyncSettings {
                repo_url: None,
                branch: "main".to_string(),
            },
        };

        if let Some(sync) = &content.sync {
            if let Some(git) = &sync.git {
                if let Some(repo_url) = &git.repo_url {
                    settings.git.repo_url = Some(repo_url.clone());
                }
                if let Some(branch) = &git.branch {
                    settings.git.branch = branch.clone();
                }
            }
        }

        settings
    }
}

pub struct TokenPrompt {
    editor: Entity<Editor>,
}

impl TokenPrompt {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("GitHub Personal Access Token (PAT)", window, cx);
            editor
        });
        Self { editor }
    }

    fn confirm(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        let token = self.editor.read(cx).text(cx).trim().to_string();
        if !token.is_empty() {
            let credentials_provider = <dyn CredentialsProvider>::global(cx);
            cx.spawn(async move |this: WeakEntity<TokenPrompt>, cx: &mut AsyncApp| {
                credentials_provider
                    .write_credentials(CREDENTIALS_URL, "PAT", token.as_bytes(), &cx)
                    .await
                    .log_err();
                this.update(cx, |_: &mut TokenPrompt, cx: &mut Context<TokenPrompt>| {
                    cx.emit(DismissEvent);
                })
                .log_err();
            })
            .detach();
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for TokenPrompt {}
impl ModalView for TokenPrompt {}

impl Focusable for TokenPrompt {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl Render for TokenPrompt {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .p_4()
            .gap_4()
            .child(Label::new("Enter GitHub Personal Access Token"))
            .child(
                div()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(self.editor.clone()),
            )
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("cancel", "Cancel").on_click(cx.listener(|this, _, window, cx| {
                            this.cancel(&menu::Cancel, window, cx)
                        })),
                    )
                    .child(
                        Button::new("confirm", "Save Token")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&menu::Confirm, window, cx)
                            })),
                    ),
            )
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
    }
}

pub fn init(cx: &mut App) {
    SyncSettings::register(cx);

    cx.on_action(|_: &SetSettingsSyncToken, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |window, cx| TokenPrompt::new(window, cx));
        });
    });

    cx.on_action(|_: &SyncSettingsToGit, cx| {
        let settings = SyncSettings::get_global(cx);
        let Some(repo_url) = settings.git.repo_url.clone() else {
            with_active_or_new_workspace(cx, |workspace, _, cx| {
                workspace.show_toast(
                    Toast::new(
                        NotificationId::Named("settings_sync_missing_url".into()),
                        "Settings sync: Please configure sync.git.repo_url in settings.json",
                    ),
                    cx,
                );
            });
            return;
        };
        let branch = settings.git.branch.clone();
        let credentials_provider = <dyn CredentialsProvider>::global(cx);

        with_active_or_new_workspace(cx, move |workspace, _, cx| {
            workspace.show_toast(
                Toast::new(
                    NotificationId::Named("settings_sync_status".into()),
                    "Syncing settings...",
                ),
                cx,
            );

            cx.spawn(async move |this: WeakEntity<Workspace>, cx: &mut AsyncApp| {
                let token = if let Ok(Some((_, password))) =
                    credentials_provider.read_credentials(CREDENTIALS_URL, &cx).await
                {
                    Some(String::from_utf8_lossy(&password).to_string())
                } else {
                    None
                };

                if token.is_none() {
                    this.update(cx, |workspace, cx| {
                        workspace.dismiss_notification(
                            &NotificationId::Named("settings_sync_status".into()),
                            cx,
                        );
                        workspace.show_toast(
                            Toast::new(
                                NotificationId::Named("settings_sync_no_token".into()),
                                "No GitHub token configured",
                            )
                            .on_click("Configure Token", |window, cx| {
                                window.dispatch_action(SetSettingsSyncToken.boxed_clone(), cx);
                            }),
                            cx,
                        );
                    })
                    .log_err();
                    return;
                }

                let result: anyhow::Result<()> = cx
                    .background_executor()
                    .spawn(async move {
                        let engine = sync_engine::SyncEngine::new();
                        engine.push(&repo_url, &branch, token.as_deref())
                    })
                    .await;

                this.update(cx, |workspace, cx| {
                    workspace.dismiss_notification(
                        &NotificationId::Named("settings_sync_status".into()),
                        cx,
                    );
                    match result {
                        Ok(_) => {
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::Named("settings_sync".into()),
                                    "Settings synced successfully",
                                )
                                .autohide(),
                                cx,
                            );
                        }
                        Err(error) => {
                            let error_str = error.to_string();
                            let is_auth_error = error_str.contains("Authentication failed")
                                || error_str.contains("Access denied")
                                || error_str.contains("Write access denied")
                                || error_str.contains("token");
                            let toast = Toast::new(
                                NotificationId::Named("settings_sync_error".into()),
                                error_str,
                            );
                            let toast = if is_auth_error {
                                toast.on_click("Configure Token", |window, cx| {
                                    window.dispatch_action(SetSettingsSyncToken.boxed_clone(), cx);
                                })
                            } else {
                                toast
                            };
                            workspace.show_toast(toast, cx);
                        }
                    }
                })
                .log_err();
            })
            .detach();
        });
    });

    cx.on_action(|_: &PullSettingsFromGit, cx| {
        let settings = SyncSettings::get_global(cx);
        let Some(repo_url) = settings.git.repo_url.clone() else {
            with_active_or_new_workspace(cx, |workspace, _, cx| {
                workspace.show_toast(
                    Toast::new(
                        NotificationId::Named("settings_pull_missing_url".into()),
                        "Settings sync: Please configure sync.git.repo_url in settings.json",
                    ),
                    cx,
                );
            });
            return;
        };
        let branch = settings.git.branch.clone();
        let credentials_provider = <dyn CredentialsProvider>::global(cx);

        with_active_or_new_workspace(cx, move |workspace, _, cx| {
            workspace.show_toast(
                Toast::new(
                    NotificationId::Named("settings_pull_status".into()),
                    "Pulling settings...",
                ),
                cx,
            );

            cx.spawn(async move |this: WeakEntity<Workspace>, cx: &mut AsyncApp| {
                let token = if let Ok(Some((_, password))) =
                    credentials_provider.read_credentials(CREDENTIALS_URL, &cx).await
                {
                    Some(String::from_utf8_lossy(&password).to_string())
                } else {
                    None
                };

                if token.is_none() {
                    this.update(cx, |workspace, cx| {
                        workspace.dismiss_notification(
                            &NotificationId::Named("settings_pull_status".into()),
                            cx,
                        );
                        workspace.show_toast(
                            Toast::new(
                                NotificationId::Named("settings_pull_no_token".into()),
                                "No GitHub token configured",
                            )
                            .on_click("Configure Token", |window, cx| {
                                window.dispatch_action(SetSettingsSyncToken.boxed_clone(), cx);
                            }),
                            cx,
                        );
                    })
                    .log_err();
                    return;
                }

                let result: anyhow::Result<()> = cx
                    .background_executor()
                    .spawn(async move {
                        let engine = sync_engine::SyncEngine::new();
                        engine.pull(&repo_url, &branch, token.as_deref())
                    })
                    .await;

                this.update(cx, |workspace, cx| {
                    workspace.dismiss_notification(
                        &NotificationId::Named("settings_pull_status".into()),
                        cx,
                    );
                    match result {
                        Ok(_) => {
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::Named("settings_pull".into()),
                                    "Settings pulled successfully",
                                )
                                .autohide(),
                                cx,
                            );
                        }
                        Err(error) => {
                            let error_str = error.to_string();
                            let is_auth_error = error_str.contains("Authentication failed")
                                || error_str.contains("Access denied")
                                || error_str.contains("Write access denied")
                                || error_str.contains("token");
                            let toast = Toast::new(
                                NotificationId::Named("settings_pull_error".into()),
                                error_str,
                            );
                            let toast = if is_auth_error {
                                toast.on_click("Configure Token", |window, cx| {
                                    window.dispatch_action(SetSettingsSyncToken.boxed_clone(), cx);
                                })
                            } else {
                                toast
                            };
                            workspace.show_toast(toast, cx);
                        }
                    }
                })
                .log_err();
            })
            .detach();
        });
    });
}
