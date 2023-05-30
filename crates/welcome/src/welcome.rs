mod base_keymap_picker;
mod base_keymap_setting;

use crate::base_keymap_picker::ToggleBaseKeymapSelector;
use client::TelemetrySettings;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    elements::{Flex, Label, ParentElement},
    AnyElement, AppContext, Element, Entity, Subscription, View, ViewContext, WeakViewHandle,
};
use settings::{update_settings_file, SettingsStore};
use std::{borrow::Cow, sync::Arc};
use workspace::{
    dock::DockPosition, item::Item, open_new, AppState, PaneBackdrop, Welcome, Workspace,
    WorkspaceId,
};

pub use base_keymap_setting::BaseKeymap;

pub const FIRST_OPEN: &str = "first_open";

pub fn init(cx: &mut AppContext) {
    settings::register::<BaseKeymap>(cx);

    cx.add_action(|workspace: &mut Workspace, _: &Welcome, cx| {
        let welcome_page = cx.add_view(|cx| WelcomePage::new(workspace, cx));
        workspace.add_item(Box::new(welcome_page), cx)
    });

    base_keymap_picker::init(cx);
}

pub fn show_welcome_experience(app_state: &Arc<AppState>, cx: &mut AppContext) {
    open_new(&app_state, cx, |workspace, cx| {
        workspace.toggle_dock(DockPosition::Left, cx);
        let welcome_page = cx.add_view(|cx| WelcomePage::new(workspace, cx));
        workspace.add_item_to_center(Box::new(welcome_page.clone()), cx);
        cx.focus(&welcome_page);
        cx.notify();
    })
    .detach();

    db::write_and_log(cx, || {
        KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
    });
}

pub struct WelcomePage {
    workspace: WeakViewHandle<Workspace>,
    _settings_subscription: Subscription,
}

impl Entity for WelcomePage {
    type Event = ();
}

impl View for WelcomePage {
    fn ui_name() -> &'static str {
        "WelcomePage"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> AnyElement<Self> {
        let self_handle = cx.handle();
        let theme = theme::current(cx);
        let width = theme.welcome.page_width;

        let telemetry_settings = *settings::get::<TelemetrySettings>(cx);

        enum Metrics {}
        enum Diagnostics {}

        PaneBackdrop::new(
            self_handle.id(),
            Flex::column()
                .with_child(
                    Flex::column()
                        .with_child(
                            theme::ui::svg(&theme.welcome.logo)
                                .aligned()
                                .contained()
                                .aligned(),
                        )
                        .with_child(
                            Label::new(
                                "Code at the speed of thought",
                                theme.welcome.logo_subheading.text.clone(),
                            )
                            .aligned()
                            .contained()
                            .with_style(theme.welcome.logo_subheading.container),
                        )
                        .contained()
                        .with_style(theme.welcome.heading_group)
                        .constrained()
                        .with_width(width),
                )
                .with_child(
                    Flex::column()
                        .with_child(theme::ui::cta_button::<theme_selector::Toggle, _, _, _>(
                            "Choose a theme",
                            width,
                            &theme.welcome.button,
                            cx,
                            |_, this, cx| {
                                if let Some(workspace) = this.workspace.upgrade(cx) {
                                    workspace.update(cx, |workspace, cx| {
                                        theme_selector::toggle(workspace, &Default::default(), cx)
                                    })
                                }
                            },
                        ))
                        .with_child(theme::ui::cta_button::<ToggleBaseKeymapSelector, _, _, _>(
                            "Choose a keymap",
                            width,
                            &theme.welcome.button,
                            cx,
                            |_, this, cx| {
                                if let Some(workspace) = this.workspace.upgrade(cx) {
                                    workspace.update(cx, |workspace, cx| {
                                        base_keymap_picker::toggle(
                                            workspace,
                                            &Default::default(),
                                            cx,
                                        )
                                    })
                                }
                            },
                        ))
                        .with_child(theme::ui::cta_button::<install_cli::Install, _, _, _>(
                            "Install the CLI",
                            width,
                            &theme.welcome.button,
                            cx,
                            |_, _, cx| {
                                cx.app_context()
                                    .spawn(|cx| async move { install_cli::install_cli(&cx).await })
                                    .detach_and_log_err(cx);
                            },
                        ))
                        .contained()
                        .with_style(theme.welcome.button_group)
                        .constrained()
                        .with_width(width),
                )
                .with_child(
                    Flex::column()
                        .with_child(
                            theme::ui::checkbox_with_label::<Metrics, _, Self, _>(
                                Flex::column()
                                    .with_child(
                                        Label::new(
                                            "Send anonymous usage data",
                                            theme.welcome.checkbox.label.text.clone(),
                                        )
                                        .contained()
                                        .with_style(theme.welcome.checkbox.label.container),
                                    )
                                    .with_child(
                                        Label::new(
                                            "Help > View Telemetry",
                                            theme.welcome.usage_note.text.clone(),
                                        )
                                        .contained()
                                        .with_style(theme.welcome.usage_note.container),
                                    ),
                                &theme.welcome.checkbox,
                                telemetry_settings.metrics,
                                0,
                                cx,
                                |this, checked, cx| {
                                    if let Some(workspace) = this.workspace.upgrade(cx) {
                                        let fs = workspace.read(cx).app_state().fs.clone();
                                        update_settings_file::<TelemetrySettings>(
                                            fs,
                                            cx,
                                            move |setting| setting.metrics = Some(checked),
                                        )
                                    }
                                },
                            )
                            .contained()
                            .with_style(theme.welcome.checkbox_container),
                        )
                        .with_child(
                            theme::ui::checkbox::<Diagnostics, Self, _>(
                                "Send crash reports",
                                &theme.welcome.checkbox,
                                telemetry_settings.diagnostics,
                                0,
                                cx,
                                |this, checked, cx| {
                                    if let Some(workspace) = this.workspace.upgrade(cx) {
                                        let fs = workspace.read(cx).app_state().fs.clone();
                                        update_settings_file::<TelemetrySettings>(
                                            fs,
                                            cx,
                                            move |setting| setting.diagnostics = Some(checked),
                                        )
                                    }
                                },
                            )
                            .contained()
                            .with_style(theme.welcome.checkbox_container),
                        )
                        .contained()
                        .with_style(theme.welcome.checkbox_group)
                        .constrained()
                        .with_width(width),
                )
                .constrained()
                .with_max_width(width)
                .contained()
                .with_uniform_padding(10.)
                .aligned()
                .into_any(),
        )
        .into_any_named("welcome page")
    }
}

impl WelcomePage {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        WelcomePage {
            workspace: workspace.weak_handle(),
            _settings_subscription: cx.observe_global::<SettingsStore, _>(move |_, cx| cx.notify()),
        }
    }
}

impl Item for WelcomePage {
    fn tab_tooltip_text(&self, _: &AppContext) -> Option<Cow<str>> {
        Some("Welcome to Zed!".into())
    }

    fn tab_content<T: View>(
        &self,
        _detail: Option<usize>,
        style: &theme::Tab,
        _cx: &gpui::AppContext,
    ) -> AnyElement<T> {
        Flex::row()
            .with_child(
                Label::new("Welcome to Zed!", style.label.clone())
                    .aligned()
                    .contained(),
            )
            .into_any()
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Self> {
        Some(WelcomePage {
            workspace: self.workspace.clone(),
            _settings_subscription: cx.observe_global::<SettingsStore, _>(move |_, cx| cx.notify()),
        })
    }
}
