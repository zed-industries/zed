use copilot::{Copilot, SignOut, Status};
use editor::Editor;
use fs::Fs;
use gpui::{
    div, Action, AnchorCorner, AppContext, Entity, IntoElement, ParentElement,
    Render, Subscription, View, ViewContext,
};
use language::{
    language_settings::{all_language_settings, AllLanguageSettings},
    File, Language,
};
use settings::{update_settings_file, SettingsStore};
use std::sync::Arc;
use workspace::{
    item::ItemHandle,
    ui::{
        popover_menu, ButtonCommon, Clickable, ContextMenu, IconButton, IconName, IconSize, Tooltip,
    },
    StatusItemView, Toast, Workspace,
};
use zed_actions::OpenBrowser;

const SUPERMAVEN_SETTINGS_URL: &str = "https://supermaven.com/account";
const SUPERMAVEN_ACTIVATE_TOAST_ID: usize = 4000;
const SUPERMAVEN_ERROR_TOAST_ID: usize = 4001;

pub struct SupermavenButton {
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    language: Option<Arc<Language>>,
    file: Option<Arc<dyn File>>,
    fs: Arc<dyn Fs>,
}

impl Render for SupermavenButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let all_language_settings = all_language_settings(None, cx);
        if !all_language_settings.supermaven.feature_enabled {
            return div();
        }

        let Some(copilot) = Copilot::global(cx) else {
            return div();
        };
        let status = copilot.read(cx).status();

        let enabled = self
            .editor_enabled
            .unwrap_or_else(|| all_language_settings.copilot_enabled(None, None));

        let icon = match status {
            Status::Error(_) => IconName::SupermavenError,
            Status::Authorized => {
                if enabled {
                    IconName::Supermaven
                } else {
                    IconName::SupermavenDisabled
                }
            }
            _ => IconName::SupermavenInit,
        };

        if let Status::Error(e) = status {
            return div().child(
                IconButton::new("supermaven-error", icon)
                    .icon_size(IconSize::Small)
                    .on_click(cx.listener(move |_, _, cx| {
                        if let Some(workspace) = cx.window_handle().downcast::<Workspace>() {
                            workspace
                                .update(cx, |workspace, cx| {
                                    workspace.show_toast(
                                        Toast::new(
                                            SUPERMAVEN_ERROR_TOAST_ID,
                                            format!("Supermaven can't be started: {}", e),
                                        )
                                        .on_click(
                                            "Reinstall Supermaven",
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
                    .tooltip(|cx| Tooltip::text("Supermaven", cx)),
            );
        }
        let this = cx.view().clone();

        div().child(
            popover_menu("supermaven")
                .menu(move |cx| match status {
                    Status::Authorized => {
                        Some(this.update(cx, |this, cx| this.build_supermaven_menu(cx)))
                    }
                    _ => Some(this.update(cx, |this, cx| this.build_supermaven_start_menu(cx))),
                })
                .anchor(AnchorCorner::BottomRight)
                .trigger(
                    IconButton::new("supermaven-icon", icon)
                        .tooltip(|cx| Tooltip::text("Supermaven", cx)),
                ),
        )
    }
}

impl SupermavenButton {
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

    pub fn build_supermaven_start_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        let fs = self.fs.clone();
        ContextMenu::build(cx, |menu, _cx| {
            menu.entry("Activate Supermaven", None, move |cx| {
                if let Some(workspace) = cx.window_handle().downcast::<Workspace>() {
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.show_toast(
                                Toast::new(
                                    SUPERMAVEN_ACTIVATE_TOAST_ID,
                                    format!("Supermaven needs to be activated before it can be used."),
                                )
                                .on_click(
                                    "Activate Supermaven",
                                    |_cx| {
                                        println!("hi");
                                    },
                                ),
                                cx,
                            );
                        })
                        .ok();
                }
            }).entry("Disable Supermaven", None, move |cx| {
                hide_supermaven(fs.clone(), cx)
            })
        })
    }

    pub fn build_supermaven_menu(&mut self, cx: &mut ViewContext<Self>) -> View<ContextMenu> {
        let fs = self.fs.clone();

        ContextMenu::build(cx, move |menu, _cx| {
            menu.entry("Disable Supermaven", None, move |cx| {
                hide_supermaven(fs.clone(), cx)
            })
            .separator()
            .link(
                "Supermaven Settings",
                OpenBrowser {
                    url: SUPERMAVEN_SETTINGS_URL.to_string(),
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
        let all_language_settings = all_language_settings(None, cx);
        self.editor_enabled = Some(all_language_settings.supermaven.feature_enabled);
        self.language = language.cloned();
        self.file = file;

        cx.notify()
    }
}

impl StatusItemView for SupermavenButton {
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

fn hide_supermaven(fs: Arc<dyn Fs>, cx: &mut AppContext) {
    update_settings_file::<AllLanguageSettings>(fs, cx, move |file| {
        file.features.get_or_insert(Default::default()).supermaven = Some(false);
    });
}
