use std::sync::Arc;

use context_menu::{ContextMenu, ContextMenuItem};
use editor::Editor;
use gpui::{
    elements::*, impl_internal_actions, CursorStyle, Element, ElementBox, Entity, MouseButton,
    MouseState, MutableAppContext, RenderContext, Subscription, View, ViewContext, ViewHandle,
};
use settings::{settings_file::SettingsFile, Settings};
use workspace::{
    item::ItemHandle, notifications::simple_message_notification::OsOpen, StatusItemView,
};

use copilot::{Copilot, SignIn, SignOut, Status};

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";

#[derive(Clone, PartialEq)]
pub struct DeployCopilotMenu;

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
        DeployCopilotModal,
        ToggleCopilotForLanguage,
        ToggleCopilotGlobally
    ]
);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CopilotButton::deploy_copilot_menu);
    cx.add_action(
        |_: &mut CopilotButton, action: &ToggleCopilotForLanguage, cx| {
            let language = action.language.to_owned();

            let current_langauge = cx.global::<Settings>().copilot_on(Some(&language));

            SettingsFile::update(cx, move |file_contents| {
                file_contents.languages.insert(
                    language.to_owned(),
                    settings::EditorSettings {
                        copilot: Some((!current_langauge).into()),
                        ..Default::default()
                    },
                );
            })
        },
    );

    cx.add_action(|_: &mut CopilotButton, _: &ToggleCopilotGlobally, cx| {
        let copilot_on = cx.global::<Settings>().copilot_on(None);

        SettingsFile::update(cx, move |file_contents| {
            file_contents.editor.copilot = Some((!copilot_on).into())
        })
    });
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

        if !settings.enable_copilot_integration {
            return Empty::new().boxed();
        }

        let theme = settings.theme.clone();
        let active = self.popup_menu.read(cx).visible() /* || modal.is_shown */;
        let authorized = Copilot::global(cx).unwrap().read(cx).status() == Status::Authorized;
        let enabled = self.editor_enabled.unwrap_or(settings.copilot_on(None));

        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, {
                    let theme = theme.clone();
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
                                    if authorized {
                                        if enabled {
                                            "icons/copilot_16.svg"
                                        } else {
                                            "icons/copilot_disabled_16.svg"
                                        }
                                    } else {
                                        "icons/copilot_init_16.svg"
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
                .on_click(MouseButton::Left, move |_, cx| {
                    if authorized {
                        cx.dispatch_action(DeployCopilotMenu);
                    } else {
                        cx.dispatch_action(SignIn);
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
        cx.observe(&Copilot::global(cx).unwrap(), |_, _, cx| cx.notify())
            .detach();
        let this_handle = cx.handle();
        cx.observe_global::<Settings, _>(move |cx| this_handle.update(cx, |_, cx| cx.notify()))
            .detach();

        Self {
            popup_menu: menu,
            editor_subscription: None,
            editor_enabled: None,
            language: None,
        }
    }

    pub fn deploy_copilot_menu(&mut self, _: &DeployCopilotMenu, cx: &mut ViewContext<Self>) {
        let settings = cx.global::<Settings>();

        let mut menu_options = Vec::with_capacity(6);

        if let Some((_, view_id)) = self.editor_subscription.as_ref() {
            let locally_enabled = self.editor_enabled.unwrap_or(settings.copilot_on(None));
            menu_options.push(ContextMenuItem::item_for_view(
                if locally_enabled {
                    "Pause Copilot for file"
                } else {
                    "Resume Copilot for file"
                },
                *view_id,
                copilot::Toggle,
            ));
        }

        if let Some(language) = &self.language {
            let language_enabled = settings.copilot_on(Some(language.as_ref()));

            menu_options.push(ContextMenuItem::item(
                format!(
                    "{} Copilot for {}",
                    if language_enabled {
                        "Disable"
                    } else {
                        "Enable"
                    },
                    language
                ),
                ToggleCopilotForLanguage {
                    language: language.to_owned(),
                },
            ));
        }

        let globally_enabled = cx.global::<Settings>().copilot_on(None);
        menu_options.push(ContextMenuItem::item(
            if globally_enabled {
                "Disable Copilot Globally"
            } else {
                "Enable Copilot Locally"
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

        if let Some(enabled) = editor.copilot_state.user_enabled {
            self.editor_enabled = Some(enabled);
            cx.notify();
            return;
        }

        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let settings = cx.global::<Settings>();
        let suggestion_anchor = editor.selections.newest_anchor().start;

        let language_name = snapshot
            .language_at(suggestion_anchor)
            .map(|language| language.name());

        self.language = language_name.clone();
        self.editor_enabled = Some(settings.copilot_on(language_name.as_deref()));
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
