mod base_keymap_picker;

use std::sync::Arc;

use db::kvp::KEY_VALUE_STORE;
use gpui::{
    elements::{Flex, Label, ParentElement},
    Element, ElementBox, Entity, MutableAppContext, Subscription, View, ViewContext,
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
                            theme::ui::svg(&theme.welcome.logo)
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
                            theme::ui::cta_button(
                                "Choose a theme",
                                theme_selector::Toggle,
                                width,
                                &theme.welcome.button,
                                cx,
                            ),
                            theme::ui::cta_button(
                                "Choose a keymap",
                                ToggleBaseKeymapSelector,
                                width,
                                &theme.welcome.button,
                                cx,
                            ),
                            theme::ui::cta_button(
                                "Install the CLI",
                                install_cli::Install,
                                width,
                                &theme.welcome.button,
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
        WelcomePage {
            _settings_subscription: cx.observe_global::<Settings, _>(move |_, cx| cx.notify()),
        }
    }
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
