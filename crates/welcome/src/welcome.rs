use std::borrow::Cow;

use gpui::{
    elements::{Empty, Flex, Image, Label, MouseEventHandler, ParentElement, Svg},
    Action, Element, ElementBox, Entity, MouseButton, MutableAppContext, RenderContext,
    Subscription, View, ViewContext,
};
use settings::{settings_file::SettingsFile, Settings, SettingsFileContent};
use theme::CheckboxStyle;
use workspace::{item::Item, PaneBackdrop, Welcome, Workspace, WorkspaceId};

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|workspace: &mut Workspace, _: &Welcome, cx| {
        let welcome_page = cx.add_view(WelcomePage::new);
        workspace.add_item(Box::new(welcome_page), cx)
    })
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
                    Image::new("images/zed-logo-90x90.png")
                        .constrained()
                        .with_width(90.)
                        .with_height(90.)
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
                    self.render_cta_button(2, "Choose a theme", theme_selector::Toggle, width, cx),
                    self.render_cta_button(3, "Choose a keymap", theme_selector::Toggle, width, cx),
                    self.render_cta_button(4, "Install the CLI", install_cli::Install, width, cx),
                    self.render_settings_checkbox::<Metrics>(
                        "Do you want to send telemetry?",
                        &theme.welcome.checkbox,
                        metrics,
                        cx,
                        |content, checked| content.telemetry.set_metrics(checked),
                    ),
                    self.render_settings_checkbox::<Diagnostics>(
                        "Send crash reports",
                        &theme.welcome.checkbox,
                        diagnostics,
                        cx,
                        |content, checked| content.telemetry.set_diagnostics(checked),
                    ),
                ])
                .constrained()
                .with_max_width(width)
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
        region_id: usize,
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
        MouseEventHandler::<A>::new(region_id, cx, |state, _| {
            let style = theme.welcome.button.style_for(state, false);
            Label::new(label, style.text.clone())
                .aligned()
                .contained()
                .with_style(style.container)
                .constrained()
                .with_width(width)
                .boxed()
        })
        .on_click(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(action.clone())
        })
        .with_cursor_style(gpui::CursorStyle::PointingHand)
        .boxed()
    }

    fn render_settings_checkbox<T: 'static>(
        &self,
        label: &'static str,
        style: &CheckboxStyle,
        checked: bool,
        cx: &mut RenderContext<Self>,
        set_value: fn(&mut SettingsFileContent, checked: bool) -> (),
    ) -> ElementBox {
        MouseEventHandler::<T>::new(0, cx, |state, _| {
            let indicator = if checked {
                Svg::new(style.check_icon.clone())
                    .with_color(style.check_icon_color)
                    .constrained()
            } else {
                Empty::new().constrained()
            };

            Flex::row()
                .with_children([
                    indicator
                        .with_width(style.width)
                        .with_height(style.height)
                        .contained()
                        .with_style(if checked {
                            if state.hovered() {
                                style.hovered_and_checked
                            } else {
                                style.checked
                            }
                        } else {
                            if state.hovered() {
                                style.hovered
                            } else {
                                style.default
                            }
                        })
                        .boxed(),
                    Label::new(label, style.label.text.clone()).contained().with_style(style.label.container).boxed(),
                ])
                .align_children_center()
                .boxed()
        })
        .on_click(gpui::MouseButton::Left, move |_, cx| {
            SettingsFile::update(cx, move |content| set_value(content, !checked))
        })
        .with_cursor_style(gpui::CursorStyle::PointingHand)
        .contained()
        .with_style(style.container)
        .boxed()
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
