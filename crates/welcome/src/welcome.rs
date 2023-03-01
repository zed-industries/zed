use gpui::{
    color::Color,
    elements::{Empty, Flex, Label, MouseEventHandler, ParentElement, Svg},
    Element, ElementBox, Entity, MutableAppContext, RenderContext, Subscription, View, ViewContext,
};
use settings::{settings_file::SettingsFile, Settings, SettingsFileContent};
use theme::CheckboxStyle;
use workspace::{item::Item, Welcome, Workspace};

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
        let settings = cx.global::<Settings>();
        let theme = settings.theme.clone();

        let (diagnostics, metrics) = {
            let telemetry = settings.telemetry();
            (telemetry.diagnostics(), telemetry.metrics())
        };

        enum Metrics {}
        enum Diagnostics {}

        Flex::column()
            .with_children([
                Flex::row()
                    .with_children([
                        Svg::new("icons/terminal_16.svg")
                            .with_color(Color::red())
                            .constrained()
                            .with_width(100.)
                            .with_height(100.)
                            .aligned()
                            .contained()
                            .boxed(),
                        Label::new("Zed", theme.editor.hover_popover.prose.clone()).boxed(),
                    ])
                    .boxed(),
                Label::new(
                    "Code at the speed of thought",
                    theme.editor.hover_popover.prose.clone(),
                )
                .boxed(),
                Flex::row()
                    .with_children([
                        self.render_settings_checkbox::<Metrics>(
                            &theme.welcome.checkbox,
                            metrics,
                            cx,
                            |content, checked| {
                                content.telemetry.set_metrics(checked);
                            },
                        ),
                        Label::new(
                            "Do you want to send telemetry?",
                            theme.editor.hover_popover.prose.clone(),
                        )
                        .boxed(),
                    ])
                    .boxed(),
                Flex::row()
                    .with_children([
                        self.render_settings_checkbox::<Diagnostics>(
                            &theme.welcome.checkbox,
                            diagnostics,
                            cx,
                            |content, checked| content.telemetry.set_diagnostics(checked),
                        ),
                        Label::new(
                            "Send crash reports",
                            theme.editor.hover_popover.prose.clone(),
                        )
                        .boxed(),
                    ])
                    .boxed(),
            ])
            .aligned()
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

    fn render_settings_checkbox<T: 'static>(
        &self,
        style: &CheckboxStyle,
        checked: bool,
        cx: &mut RenderContext<Self>,
        set_value: fn(&mut SettingsFileContent, checked: bool) -> (),
    ) -> ElementBox {
        MouseEventHandler::<T>::new(0, cx, |state, _| {
            Empty::new()
                .constrained()
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
                .boxed()
        })
        .on_click(gpui::MouseButton::Left, move |_, cx| {
            SettingsFile::update(cx, move |content| set_value(content, !checked))
        })
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
}
