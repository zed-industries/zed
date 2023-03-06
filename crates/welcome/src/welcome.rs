use std::borrow::Cow;

use gpui::{
    elements::{Canvas, Empty, Flex, Image, Label, MouseEventHandler, ParentElement, Stack},
    geometry::rect::RectF,
    Action, Element, ElementBox, Entity, MouseButton, MouseRegion, MutableAppContext,
    RenderContext, Subscription, View, ViewContext,
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
        let self_handle = cx.handle();
        let settings = cx.global::<Settings>();
        let theme = settings.theme.clone();

        let (diagnostics, metrics) = {
            let telemetry = settings.telemetry();
            (telemetry.diagnostics(), telemetry.metrics())
        };

        enum Metrics {}
        enum Diagnostics {}

        let background = theme.editor.background;

        Stack::new()
            .with_child(
                // TODO: Can this be moved into the pane?
                Canvas::new(move |bounds, visible_bounds, cx| {
                    let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

                    cx.paint_layer(Some(visible_bounds), |cx| {
                        cx.scene.push_quad(gpui::Quad {
                            bounds: RectF::new(bounds.origin(), bounds.size()),
                            background: Some(background),
                            ..Default::default()
                        })
                    });

                    cx.scene.push_mouse_region(
                        MouseRegion::new::<Self>(self_handle.id(), 0, visible_bounds)
                            .on_down(gpui::MouseButton::Left, |_, cx| cx.focus_parent_view()),
                    );
                })
                .boxed(),
            )
            .with_child(
                Flex::column()
                    .with_children([
                        Flex::row()
                            .with_children([
                                Image::new("images/zed-logo-90x90.png")
                                    .constrained()
                                    .with_width(90.)
                                    .with_height(90.)
                                    .aligned()
                                    .contained()
                                    .boxed(),
                                // Label::new("Zed", theme.editor.hover_popover.prose.clone()).boxed(),
                            ])
                            .boxed(),
                        Label::new(
                            "Code at the speed of thought",
                            theme.editor.hover_popover.prose.clone(),
                        )
                        .boxed(),
                        self.render_cta_button(2, "Choose a theme", theme_selector::Toggle, cx),
                        self.render_cta_button(3, "Choose a keymap", theme_selector::Toggle, cx),
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
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(action.clone())
        })
        .aligned()
        .boxed()
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
