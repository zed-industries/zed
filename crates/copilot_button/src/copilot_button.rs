use std::sync::Arc;

use context_menu::{ContextMenu, ContextMenuItem};
use editor::Editor;
use gpui::{
    elements::*,
    impl_internal_actions,
    platform::{CursorStyle, MouseButton},
    AppContext, Element, ElementBox, Entity, MouseState, RenderContext, Subscription, View,
    ViewContext, ViewHandle,
};
use settings::{settings_file::SettingsFile, Settings};
use workspace::{
    item::ItemHandle, notifications::simple_message_notification::OsOpen, DismissToast,
    StatusItemView,
};

use copilot::{Copilot, Reinstall, SignIn, SignOut, Status};

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";
const COPILOT_STARTING_TOAST_ID: usize = 1337;
const COPILOT_ERROR_TOAST_ID: usize = 1338;

#[derive(Clone, PartialEq)]
pub struct DeployCopilotMenu;

#[derive(Clone, PartialEq)]
pub struct DeployCopilotStartMenu;

#[derive(Clone, PartialEq)]
pub struct HideCopilot;

#[derive(Clone, PartialEq)]
pub struct InitiateSignIn;

#[derive(Clone, PartialEq)]
pub struct ToggleCopilotForLanguage {
    language: Arc<str>,
}

#[derive(Clone, PartialEq)]
pub struct ToggleCopilotGlobally;

// TODO: Make the other code path use `get_or_insert` logic for this modal
#[derive(Clone, PartialEq)]
pub struct DeployCopilotModal;

impl_internal_actions!(
    copilot,
    [
        DeployCopilotMenu,
        DeployCopilotStartMenu,
        HideCopilot,
        InitiateSignIn,
        DeployCopilotModal,
        ToggleCopilotForLanguage,
        ToggleCopilotGlobally,
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.add_action(CopilotButton::deploy_copilot_menu);
    cx.add_action(CopilotButton::deploy_copilot_start_menu);
    cx.add_action(
        |_: &mut CopilotButton, action: &ToggleCopilotForLanguage, cx| {
            let language = action.language.clone();
            let show_copilot_suggestions = cx
                .global::<Settings>()
                .show_copilot_suggestions(Some(&language));

            SettingsFile::update(cx, move |file_contents| {
                file_contents.languages.insert(
                    language,
                    settings::EditorSettings {
                        show_copilot_suggestions: Some((!show_copilot_suggestions).into()),
                        ..Default::default()
                    },
                );
            })
        },
    );

    cx.add_action(|_: &mut CopilotButton, _: &ToggleCopilotGlobally, cx| {
        let show_copilot_suggestions = cx.global::<Settings>().show_copilot_suggestions(None);
        SettingsFile::update(cx, move |file_contents| {
            file_contents.editor.show_copilot_suggestions = Some((!show_copilot_suggestions).into())
        })
    });

    cx.add_action(|_: &mut CopilotButton, _: &HideCopilot, cx| {
        SettingsFile::update(cx, move |file_contents| {
            file_contents.features.copilot = Some(false)
        })
    });

    cx.add_action(|_: &mut CopilotButton, _: &InitiateSignIn, cx| {
        let Some(copilot) = Copilot::global(cx) else {
            return;
        };
        let status = copilot.read(cx).status();

        match status {
            Status::Starting { task } => {
                cx.dispatch_action(workspace::Toast::new(
                    COPILOT_STARTING_TOAST_ID,
                    "Copilot is starting...",
                ));
                let window_id = cx.window_id();
                let task = task.to_owned();
                cx.spawn(|handle, mut cx| async move {
                    task.await;
                    cx.update(|cx| {
                        if let Some(copilot) = Copilot::global(cx) {
                            let status = copilot.read(cx).status();
                            match status {
                                Status::Authorized => cx.dispatch_action_at(
                                    window_id,
                                    handle.id(),
                                    workspace::Toast::new(
                                        COPILOT_STARTING_TOAST_ID,
                                        "Copilot has started!",
                                    ),
                                ),
                                _ => {
                                    cx.dispatch_action_at(
                                        window_id,
                                        handle.id(),
                                        DismissToast::new(COPILOT_STARTING_TOAST_ID),
                                    );
                                    cx.dispatch_action_at(window_id, handle.id(), SignIn)
                                }
                            }
                        }
                    })
                })
                .detach();
            }
            _ => cx.dispatch_action(SignIn),
        }
    })
}

pub struct CopilotButton {
    popup_menu: ViewHandle<ContextMenu>,
    editor_subscription: Option<(Subscription, usize)>,
    editor_enabled: Option<bool>,
    language: Option<Arc<str>>,
}

impl Entity for CopilotButton {
    type Event = ();
}

impl View for CopilotButton {
    fn ui_name() -> &'static str {
        "CopilotButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let settings = cx.global::<Settings>();

        if !settings.features.copilot {
            return Empty::new().boxed();
        }

        let theme = settings.theme.clone();
        let active = self.popup_menu.read(cx).visible();
        let Some(copilot) = Copilot::global(cx) else {
            return Empty::new().boxed();
        };
        let status = copilot.read(cx).status();

        let enabled = self
            .editor_enabled
            .unwrap_or(settings.show_copilot_suggestions(None));

        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, {
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
                                .named("copilot-icon"),
                            )
                            .constrained()
                            .with_height(style.icon_size)
                            .contained()
                            .with_style(style.container)
                            .boxed()
                    }
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, {
                    let status = status.clone();
                    move |_, cx| match status {
                        Status::Authorized => cx.dispatch_action(DeployCopilotMenu),
                        Status::Error(ref e) => cx.dispatch_action(workspace::Toast::new_action(
                            COPILOT_ERROR_TOAST_ID,
                            format!("Copilot can't be started: {}", e),
                            "Reinstall Copilot",
                            Reinstall,
                        )),
                        _ => cx.dispatch_action(DeployCopilotStartMenu),
                    }
                })
                .with_tooltip::<Self, _>(
                    0,
                    "GitHub Copilot".into(),
                    None,
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed(),
            )
            .with_child(
                ChildView::new(&self.popup_menu, cx)
                    .aligned()
                    .top()
                    .right()
                    .boxed(),
            )
            .boxed()
    }
}

impl CopilotButton {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let menu = cx.add_view(|cx| {
            let mut menu = ContextMenu::new(cx);
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
        }
    }

    pub fn deploy_copilot_start_menu(
        &mut self,
        _: &DeployCopilotStartMenu,
        cx: &mut ViewContext<Self>,
    ) {
        let mut menu_options = Vec::with_capacity(2);

        menu_options.push(ContextMenuItem::item("Sign In", InitiateSignIn));
        menu_options.push(ContextMenuItem::item("Hide Copilot", HideCopilot));

        self.popup_menu.update(cx, |menu, cx| {
            menu.show(
                Default::default(),
                AnchorCorner::BottomRight,
                menu_options,
                cx,
            );
        });
    }

    pub fn deploy_copilot_menu(&mut self, _: &DeployCopilotMenu, cx: &mut ViewContext<Self>) {
        let settings = cx.global::<Settings>();

        let mut menu_options = Vec::with_capacity(6);

        if let Some(language) = &self.language {
            let language_enabled = settings.show_copilot_suggestions(Some(language.as_ref()));

            menu_options.push(ContextMenuItem::item(
                format!(
                    "{} Suggestions for {}",
                    if language_enabled { "Hide" } else { "Show" },
                    language
                ),
                ToggleCopilotForLanguage {
                    language: language.to_owned(),
                },
            ));
        }

        let globally_enabled = cx.global::<Settings>().show_copilot_suggestions(None);
        menu_options.push(ContextMenuItem::item(
            if globally_enabled {
                "Hide Suggestions for All Files"
            } else {
                "Show Suggestions for All Files"
            },
            ToggleCopilotGlobally,
        ));

        menu_options.push(ContextMenuItem::Separator);

        let icon_style = settings.theme.copilot.out_link_icon.clone();
        menu_options.push(ContextMenuItem::element_item(
            Box::new(
                move |state: &mut MouseState, style: &theme::ContextMenuItem| {
                    Flex::row()
                        .with_children([
                            Label::new("Copilot Settings", style.label.clone()).boxed(),
                            theme::ui::icon(icon_style.style_for(state, false)).boxed(),
                        ])
                        .align_children_center()
                        .boxed()
                },
            ),
            OsOpen::new(COPILOT_SETTINGS_URL),
        ));

        menu_options.push(ContextMenuItem::item("Sign Out", SignOut));

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

        self.language = language_name.clone();

        self.editor_enabled = Some(settings.show_copilot_suggestions(language_name.as_deref()));

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
