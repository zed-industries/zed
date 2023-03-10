mod base_keymap_picker;

use std::{borrow::Cow, sync::Arc};

use db::kvp::KEY_VALUE_STORE;
use gpui::{
    elements::{Flex, Label, MouseEventHandler, ParentElement},
    Action, Element, ElementBox, Entity, MouseButton, MutableAppContext, RenderContext,
    Subscription, View, ViewContext,
};
use settings::{settings_file::SettingsFile, Settings};

use workspace::{
    item::Item, open_new, sidebar::SidebarSide, AppState, PaneBackdrop, Welcome, Workspace,
    WorkspaceId,
};

use crate::base_keymap_picker::ToggleBaseKeymapSelector;

pub const FIRST_OPEN: &str = "first_open";

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|workspace: &mut Workspace, _: &Welcome, cx| {
        let welcome_page = cx.add_view(WelcomePage::new);
        workspace.add_item(Box::new(welcome_page), cx)
    });

    base_keymap_picker::init(cx);
}

pub fn show_welcome_experience(app_state: &Arc<AppState>, cx: &mut MutableAppContext) {
    open_new(&app_state, cx, |workspace, cx| {
        workspace.toggle_sidebar(SidebarSide::Left, cx);
        let welcome_page = cx.add_view(|cx| WelcomePage::new(cx));
        workspace.add_item_to_center(Box::new(welcome_page.clone()), cx);
        cx.focus(welcome_page);
        cx.notify();
    })
    .detach();

    db::write_and_log(cx, || {
        KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
    });
}

pub struct WelcomePage {
    _settings_subscription: Subscription,
}

impl Entity for WelcomePage {
    type Event = ();
}

impl View for WelcomePage {
    fn ui_name() -> &'static str {
        "WelcomePage"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let self_handle = cx.handle();
        let settings = cx.global::<Settings>();
        let theme = settings.theme.clone();

        let width = theme.welcome.page_width;

        let (diagnostics, metrics) = {
            let telemetry = settings.telemetry();
            (telemetry.diagnostics(), telemetry.metrics())
        };

        enum Metrics {}
        enum Diagnostics {}

        PaneBackdrop::new(
            self_handle.id(),
            Flex::column()
                .with_children([
                    Flex::column()
                        .with_children([
                            theme::ui::icon(&theme.welcome.logo)
                                .aligned()
                                .contained()
                                .aligned()
                                .boxed(),
                            Label::new(
                                "Code at the speed of thought",
                                theme.welcome.logo_subheading.text.clone(),
                            )
                            .aligned()
                            .contained()
                            .with_style(theme.welcome.logo_subheading.container)
                            .boxed(),
                        ])
                        .contained()
                        .with_style(theme.welcome.heading_group)
                        .constrained()
                        .with_width(width)
                        .boxed(),
                    Flex::column()
                        .with_children([
                            self.render_cta_button(
                                "Choose a theme",
                                theme_selector::Toggle,
                                width,
                                cx,
                            ),
                            self.render_cta_button(
                                "Choose a keymap",
                                ToggleBaseKeymapSelector,
                                width,
                                cx,
                            ),
                            self.render_cta_button(
                                "Install the CLI",
                                install_cli::Install,
                                width,
                                cx,
                            ),
                        ])
                        .contained()
                        .with_style(theme.welcome.button_group)
                        .constrained()
                        .with_width(width)
                        .boxed(),
                    Flex::column()
                        .with_children([
                            theme::ui::checkbox_with_label::<Metrics, Self>(
                                Flex::column()
                                    .with_children([
                                        Label::new(
                                            "Send anonymous usage data",
                                            theme.welcome.checkbox.label.text.clone(),
                                        )
                                        .contained()
                                        .with_style(theme.welcome.checkbox.label.container)
                                        .boxed(),
                                        Label::new(
                                            "Help > View Telemetry",
                                            theme.welcome.usage_note.text.clone(),
                                        )
                                        .contained()
                                        .with_style(theme.welcome.usage_note.container)
                                        .boxed(),
                                    ])
                                    .boxed(),
                                &theme.welcome.checkbox,
                                metrics,
                                cx,
                                |checked, cx| {
                                    SettingsFile::update(cx, move |file| {
                                        file.telemetry.set_metrics(checked)
                                    })
                                },
                            )
                            .contained()
                            .with_style(theme.welcome.checkbox_container)
                            .boxed(),
                            theme::ui::checkbox::<Diagnostics, Self>(
                                "Send crash reports",
                                &theme.welcome.checkbox,
                                diagnostics,
                                cx,
                                |checked, cx| {
                                    SettingsFile::update(cx, move |file| {
                                        file.telemetry.set_diagnostics(checked)
                                    })
                                },
                            )
                            .contained()
                            .with_style(theme.welcome.checkbox_container)
                            .boxed(),
                        ])
                        .contained()
                        .with_style(theme.welcome.checkbox_group)
                        .constrained()
                        .with_width(width)
                        .boxed(),
                ])
                .constrained()
                .with_max_width(width)
                .contained()
                .with_uniform_padding(10.)
                .aligned()
                .boxed(),
        )
        .boxed()
    }
}

impl WelcomePage {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.weak_handle();

        let settings_subscription = cx.observe_global::<Settings, _>(move |cx| {
            if let Some(handle) = handle.upgrade(cx) {
                handle.update(cx, |_, cx| cx.notify())
            }
        });

        WelcomePage {
            _settings_subscription: settings_subscription,
        }
    }

    fn render_cta_button<L, A>(
        &self,
        label: L,
        action: A,
        width: f32,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox
    where
        L: Into<Cow<'static, str>>,
        A: 'static + Action + Clone,
    {
        let theme = cx.global::<Settings>().theme.clone();
        MouseEventHandler::<A>::new(0, cx, |state, _| {
            let style = theme.welcome.button.style_for(state, false);
            Label::new(label, style.text.clone())
                .aligned()
                .contained()
                .with_style(style.container)
                .constrained()
                .with_max_width(width)
                .boxed()
        })
        .on_click(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(action.clone())
        })
        .with_cursor_style(gpui::CursorStyle::PointingHand)
        .boxed()
    }

    // fn render_settings_checkbox<T: 'static>(
    //     &self,
    //     label: &'static str,
    //     style: &CheckboxStyle,
    //     checked: bool,
    //     cx: &mut RenderContext<Self>,
    //     set_value: fn(&mut SettingsFileContent, checked: bool) -> (),
    // ) -> ElementBox {
    //     MouseEventHandler::<T>::new(0, cx, |state, _| {
    //         let indicator = if checked {
    //             Svg::new(style.check_icon.clone())
    //                 .with_color(style.check_icon_color)
    //                 .constrained()
    //         } else {
    //             Empty::new().constrained()
    //         };

    //         Flex::row()
    //             .with_children([
    //                 indicator
    //                     .with_width(style.width)
    //                     .with_height(style.height)
    //                     .contained()
    //                     .with_style(if checked {
    //                         if state.hovered() {
    //                             style.hovered_and_checked
    //                         } else {
    //                             style.checked
    //                         }
    //                     } else {
    //                         if state.hovered() {
    //                             style.hovered
    //                         } else {
    //                             style.default
    //                         }
    //                     })
    //                     .boxed(),
    //                 Label::new(label, style.label.text.clone())
    //                     .contained()
    //                     .with_style(style.label.container)
    //                     .boxed(),
    //             ])
    //             .align_children_center()
    //             .boxed()
    //     })
    //     .on_click(gpui::MouseButton::Left, move |_, cx| {
    //         SettingsFile::update(cx, move |content| set_value(content, !checked))
    //     })
    //     .with_cursor_style(gpui::CursorStyle::PointingHand)
    //     .contained()
    //     .with_style(style.container)
    //     .boxed()
    // }
}

impl Item for WelcomePage {
    fn tab_content(
        &self,
        _detail: Option<usize>,
        style: &theme::Tab,
        _cx: &gpui::AppContext,
    ) -> gpui::ElementBox {
        Flex::row()
            .with_child(
                Label::new("Welcome to Zed!", style.label.clone())
                    .aligned()
                    .contained()
                    .boxed(),
            )
            .boxed()
    }

    fn show_toolbar(&self) -> bool {
        false
    }
    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Self> {
        Some(WelcomePage::new(cx))
    }
}
