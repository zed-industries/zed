use anyhow::Result;
use context_menu::{ContextMenu, ContextMenuItem};
use copilot::{Copilot, SignOut, Status};
use editor::{scroll::autoscroll::Autoscroll, Editor};
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    AnyElement, AppContext, AsyncAppContext, Element, Entity, MouseState, Subscription, View,
    ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use settings::{settings_file::SettingsFile, Settings};
use std::{path::Path, sync::Arc};
use util::{paths, ResultExt};
use workspace::{
    create_and_open_local_file, item::ItemHandle,
    notifications::simple_message_notification::OsOpen, StatusItemView, Toast, Workspace,
};

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";
const COPILOT_STARTING_TOAST_ID: usize = 1337;
const COPILOT_ERROR_TOAST_ID: usize = 1338;

pub struct CopilotButton {
    popup_menu: ViewHandle<ContextMenu>,
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    language: Option<Arc<str>>,
    path: Option<Arc<Path>>,
}

impl Entity for CopilotButton {
    type Event = ();
}

impl View for CopilotButton {
    fn ui_name() -> &'static str {
        "CopilotButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let settings = cx.global::<Settings>();

        if !settings.features.copilot {
            return Empty::new().into_any();
        }

        let theme = settings.theme.clone();
        let active = self.popup_menu.read(cx).visible();
        let Some(copilot) = Copilot::global(cx) else {
            return Empty::new().into_any();
        };
        let status = copilot.read(cx).status();

        let enabled = self
            .editor_enabled
            .unwrap_or(settings.show_copilot_suggestions(None, None));

        Stack::new()
            .with_child(
                MouseEventHandler::<Self, _>::new(0, cx, {
                    let theme = theme.clone();
                    let status = status.clone();
                    move |state, _cx| {
                        let style = theme
                            .workspace
                            .status_bar
                            .sidebar_buttons
                            .item
                            .style_for(state, active);

                        Flex::row()
                            .with_child(
                                Svg::new({
                                    match status {
                                        Status::Error(_) => "icons/copilot_error_16.svg",
                                        Status::Authorized => {
                                            if enabled {
                                                "icons/copilot_16.svg"
                                            } else {
                                                "icons/copilot_disabled_16.svg"
                                            }
                                        }
                                        _ => "icons/copilot_init_16.svg",
                                    }
                                })
                                .with_color(style.icon_color)
                                .constrained()
                                .with_width(style.icon_size)
                                .aligned()
                                .into_any_named("copilot-icon"),
                            )
                            .constrained()
                            .with_height(style.icon_size)
                            .contained()
                            .with_style(style.container)
                    }
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, {
                    let status = status.clone();
                    move |_, this, cx| match status {
                        Status::Authorized => this.deploy_copilot_menu(cx),
                        Status::Error(ref e) => {
                            if let Some(workspace) = cx.root_view().clone().downcast::<Workspace>()
                            {
                                workspace.update(cx, |workspace, cx| {
                                    workspace.show_toast(
                                        Toast::new(
                                            COPILOT_ERROR_TOAST_ID,
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
                                });
                            }
                        }
                        _ => this.deploy_copilot_start_menu(cx),
                    }
                })
                .with_tooltip::<Self>(
                    0,
                    "GitHub Copilot".into(),
                    None,
                    theme.tooltip.clone(),
                    cx,
                ),
            )
            .with_child(ChildView::new(&self.popup_menu, cx).aligned().top().right())
            .into_any()
    }
}

impl CopilotButton {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let button_view_id = cx.view_id();
        let menu = cx.add_view(|cx| {
            let mut menu = ContextMenu::new(button_view_id, cx);
            menu.set_position_mode(OverlayPositionMode::Local);
            menu
        });

        cx.observe(&menu, |_, _, cx| cx.notify()).detach();

        Copilot::global(cx).map(|copilot| cx.observe(&copilot, |_, _, cx| cx.notify()).detach());

        cx.observe_global::<Settings, _>(move |_, cx| cx.notify())
            .detach();

        Self {
            popup_menu: menu,
            editor_subscription: None,
            editor_enabled: None,
            language: None,
            path: None,
        }
    }

    pub fn deploy_copilot_start_menu(&mut self, cx: &mut ViewContext<Self>) {
        let mut menu_options = Vec::with_capacity(2);

        menu_options.push(ContextMenuItem::handler("Sign In", |cx| {
            initiate_sign_in(cx)
        }));
        menu_options.push(ContextMenuItem::handler("Disable Copilot", |cx| {
            hide_copilot(cx)
        }));

        self.popup_menu.update(cx, |menu, cx| {
            menu.show(
                Default::default(),
                AnchorCorner::BottomRight,
                menu_options,
                cx,
            );
        });
    }

    pub fn deploy_copilot_menu(&mut self, cx: &mut ViewContext<Self>) {
        let settings = cx.global::<Settings>();

        let mut menu_options = Vec::with_capacity(8);

        if let Some(language) = self.language.clone() {
            let language_enabled = settings.copilot_enabled_for_language(Some(language.as_ref()));
            menu_options.push(ContextMenuItem::handler(
                format!(
                    "{} Suggestions for {}",
                    if language_enabled { "Hide" } else { "Show" },
                    language
                ),
                move |cx| toggle_copilot_for_language(language.clone(), cx),
            ));
        }

        if let Some(path) = self.path.as_ref() {
            let path_enabled = settings.copilot_enabled_for_path(path);
            let path = path.clone();
            menu_options.push(ContextMenuItem::handler(
                format!(
                    "{} Suggestions for This Path",
                    if path_enabled { "Hide" } else { "Show" }
                ),
                move |cx| {
                    if let Some(workspace) = cx.root_view().clone().downcast::<Workspace>() {
                        let workspace = workspace.downgrade();
                        cx.spawn(|_, cx| {
                            configure_disabled_globs(
                                workspace,
                                path_enabled.then_some(path.clone()),
                                cx,
                            )
                        })
                        .detach_and_log_err(cx);
                    }
                },
            ));
        }

        let globally_enabled = cx.global::<Settings>().features.copilot;
        menu_options.push(ContextMenuItem::handler(
            if globally_enabled {
                "Hide Suggestions for All Files"
            } else {
                "Show Suggestions for All Files"
            },
            |cx| toggle_copilot_globally(cx),
        ));

        menu_options.push(ContextMenuItem::Separator);

        let icon_style = settings.theme.copilot.out_link_icon.clone();
        menu_options.push(ContextMenuItem::action(
            move |state: &mut MouseState, style: &theme::ContextMenuItem| {
                Flex::row()
                    .with_child(Label::new("Copilot Settings", style.label.clone()))
                    .with_child(theme::ui::icon(icon_style.style_for(state, false)))
                    .align_children_center()
                    .into_any()
            },
            OsOpen::new(COPILOT_SETTINGS_URL),
        ));

        menu_options.push(ContextMenuItem::action("Sign Out", SignOut));

        self.popup_menu.update(cx, |menu, cx| {
            menu.show(
                Default::default(),
                AnchorCorner::BottomRight,
                menu_options,
                cx,
            );
        });
    }

    pub fn update_enabled(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);

        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let settings = cx.global::<Settings>();
        let suggestion_anchor = editor.selections.newest_anchor().start;

        let language_name = snapshot
            .language_at(suggestion_anchor)
            .map(|language| language.name());
        let path = snapshot
            .file_at(suggestion_anchor)
            .map(|file| file.path().clone());

        self.editor_enabled =
            Some(settings.show_copilot_suggestions(language_name.as_deref(), path.as_deref()));
        self.language = language_name;
        self.path = path;

        cx.notify()
    }
}

impl StatusItemView for CopilotButton {
    fn set_active_pane_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        if let Some(editor) = item.map(|item| item.act_as::<Editor>(cx)).flatten() {
            self.editor_subscription =
                Some((cx.observe(&editor, Self::update_enabled), editor.id()));
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
    workspace: WeakViewHandle<Workspace>,
    path_to_disable: Option<Arc<Path>>,
    mut cx: AsyncAppContext,
) -> Result<()> {
    let settings_editor = workspace
        .update(&mut cx, |_, cx| {
            create_and_open_local_file(&paths::SETTINGS, cx, || {
                Settings::initial_user_settings_content(&assets::Assets)
                    .as_ref()
                    .into()
            })
        })?
        .await?
        .downcast::<Editor>()
        .unwrap();

    settings_editor.downgrade().update(&mut cx, |item, cx| {
        let text = item.buffer().read(cx).snapshot(cx).text();

        let edits = SettingsFile::update_unsaved(&text, cx, |file| {
            let copilot = file.copilot.get_or_insert_with(Default::default);
            let globs = copilot.disabled_globs.get_or_insert_with(|| {
                cx.global::<Settings>()
                    .copilot
                    .disabled_globs
                    .iter()
                    .map(|glob| glob.as_str().to_string())
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

fn toggle_copilot_globally(cx: &mut AppContext) {
    let show_copilot_suggestions = cx.global::<Settings>().show_copilot_suggestions(None, None);
    SettingsFile::update(cx, move |file_contents| {
        file_contents.editor.show_copilot_suggestions = Some((!show_copilot_suggestions).into())
    });
}

fn toggle_copilot_for_language(language: Arc<str>, cx: &mut AppContext) {
    let show_copilot_suggestions = cx
        .global::<Settings>()
        .show_copilot_suggestions(Some(&language), None);

    SettingsFile::update(cx, move |file_contents| {
        file_contents.languages.insert(
            language,
            settings::EditorSettings {
                show_copilot_suggestions: Some((!show_copilot_suggestions).into()),
                ..Default::default()
            },
        );
    });
}

fn hide_copilot(cx: &mut AppContext) {
    SettingsFile::update(cx, move |file_contents| {
        file_contents.features.copilot = Some(false)
    });
}

fn initiate_sign_in(cx: &mut WindowContext) {
    let Some(copilot) = Copilot::global(cx) else {
        return;
    };
    let status = copilot.read(cx).status();

    match status {
        Status::Starting { task } => {
            let Some(workspace) = cx.root_view().clone().downcast::<Workspace>() else {
                return;
            };

            workspace.update(cx, |workspace, cx| {
                workspace.show_toast(
                    Toast::new(COPILOT_STARTING_TOAST_ID, "Copilot is starting..."),
                    cx,
                )
            });
            let workspace = workspace.downgrade();
            cx.spawn(|mut cx| async move {
                task.await;
                if let Some(copilot) = cx.read(Copilot::global) {
                    workspace
                        .update(&mut cx, |workspace, cx| match copilot.read(cx).status() {
                            Status::Authorized => workspace.show_toast(
                                Toast::new(COPILOT_STARTING_TOAST_ID, "Copilot has started!"),
                                cx,
                            ),
                            _ => {
                                workspace.dismiss_toast(COPILOT_STARTING_TOAST_ID, cx);
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
            copilot
                .update(cx, |copilot, cx| copilot.sign_in(cx))
                .detach_and_log_err(cx);
        }
    }
}
