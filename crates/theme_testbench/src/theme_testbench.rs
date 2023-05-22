use gpui::{
    actions,
    color::Color,
    elements::{
        AnyElement, Canvas, Container, ContainerStyle, Flex, Label, Margin, MouseEventHandler,
        Padding, ParentElement,
    },
    fonts::TextStyle,
    AppContext, Border, Element, Entity, ModelHandle, Quad, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use project::Project;
use theme::{ColorScheme, Layer, Style, StyleSet, ThemeSettings};
use workspace::{item::Item, register_deserializable_item, Pane, Workspace};

actions!(theme, [DeployThemeTestbench]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(ThemeTestbench::deploy);

    register_deserializable_item::<ThemeTestbench>(cx)
}

pub struct ThemeTestbench {}

impl ThemeTestbench {
    pub fn deploy(
        workspace: &mut Workspace,
        _: &DeployThemeTestbench,
        cx: &mut ViewContext<Workspace>,
    ) {
        let view = cx.add_view(|_| ThemeTestbench {});
        workspace.add_item(Box::new(view), cx);
    }

    fn render_ramps(color_scheme: &ColorScheme) -> Flex<Self> {
        fn display_ramp(ramp: &Vec<Color>) -> AnyElement<ThemeTestbench> {
            Flex::row()
                .with_children(ramp.iter().cloned().map(|color| {
                    Canvas::new(move |scene, bounds, _, _, _| {
                        scene.push_quad(Quad {
                            bounds,
                            background: Some(color),
                            ..Default::default()
                        });
                    })
                    .flex(1.0, false)
                }))
                .flex(1.0, false)
                .into_any()
        }

        Flex::column()
            .with_child(display_ramp(&color_scheme.ramps.neutral))
            .with_child(display_ramp(&color_scheme.ramps.red))
            .with_child(display_ramp(&color_scheme.ramps.orange))
            .with_child(display_ramp(&color_scheme.ramps.yellow))
            .with_child(display_ramp(&color_scheme.ramps.green))
            .with_child(display_ramp(&color_scheme.ramps.cyan))
            .with_child(display_ramp(&color_scheme.ramps.blue))
            .with_child(display_ramp(&color_scheme.ramps.violet))
            .with_child(display_ramp(&color_scheme.ramps.magenta))
    }

    fn render_layer(
        layer_index: usize,
        layer: &Layer,
        cx: &mut ViewContext<Self>,
    ) -> Container<Self> {
        Flex::column()
            .with_child(
                Self::render_button_set(0, layer_index, "base", &layer.base, cx).flex(1., false),
            )
            .with_child(
                Self::render_button_set(1, layer_index, "variant", &layer.variant, cx)
                    .flex(1., false),
            )
            .with_child(
                Self::render_button_set(2, layer_index, "on", &layer.on, cx).flex(1., false),
            )
            .with_child(
                Self::render_button_set(3, layer_index, "accent", &layer.accent, cx)
                    .flex(1., false),
            )
            .with_child(
                Self::render_button_set(4, layer_index, "positive", &layer.positive, cx)
                    .flex(1., false),
            )
            .with_child(
                Self::render_button_set(5, layer_index, "warning", &layer.warning, cx)
                    .flex(1., false),
            )
            .with_child(
                Self::render_button_set(6, layer_index, "negative", &layer.negative, cx)
                    .flex(1., false),
            )
            .contained()
            .with_style(ContainerStyle {
                margin: Margin {
                    top: 10.,
                    bottom: 10.,
                    left: 10.,
                    right: 10.,
                },
                background_color: Some(layer.base.default.background),
                ..Default::default()
            })
    }

    fn render_button_set(
        set_index: usize,
        layer_index: usize,
        set_name: &'static str,
        style_set: &StyleSet,
        cx: &mut ViewContext<Self>,
    ) -> Flex<Self> {
        Flex::row()
            .with_child(Self::render_button(
                set_index * 6,
                layer_index,
                set_name,
                &style_set,
                None,
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 1,
                layer_index,
                "hovered",
                &style_set,
                Some(|style_set| &style_set.hovered),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 2,
                layer_index,
                "pressed",
                &style_set,
                Some(|style_set| &style_set.pressed),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 3,
                layer_index,
                "active",
                &style_set,
                Some(|style_set| &style_set.active),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 4,
                layer_index,
                "disabled",
                &style_set,
                Some(|style_set| &style_set.disabled),
                cx,
            ))
            .with_child(Self::render_button(
                set_index * 6 + 5,
                layer_index,
                "inverted",
                &style_set,
                Some(|style_set| &style_set.inverted),
                cx,
            ))
    }

    fn render_button(
        button_index: usize,
        layer_index: usize,
        text: &'static str,
        style_set: &StyleSet,
        style_override: Option<fn(&StyleSet) -> &Style>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum TestBenchButton {}
        MouseEventHandler::<TestBenchButton, _>::new(layer_index + button_index, cx, |state, cx| {
            let style = if let Some(style_override) = style_override {
                style_override(&style_set)
            } else if state.clicked().is_some() {
                &style_set.pressed
            } else if state.hovered() {
                &style_set.hovered
            } else {
                &style_set.default
            };

            Self::render_label(text.to_string(), style, cx)
                .contained()
                .with_style(ContainerStyle {
                    margin: Margin {
                        top: 4.,
                        bottom: 4.,
                        left: 4.,
                        right: 4.,
                    },
                    padding: Padding {
                        top: 4.,
                        bottom: 4.,
                        left: 4.,
                        right: 4.,
                    },
                    background_color: Some(style.background),
                    border: Border {
                        width: 1.,
                        color: style.border,
                        overlay: false,
                        top: true,
                        bottom: true,
                        left: true,
                        right: true,
                    },
                    corner_radius: 2.,
                    ..Default::default()
                })
        })
        .flex(1., true)
        .into_any()
    }

    fn render_label(text: String, style: &Style, cx: &mut ViewContext<Self>) -> Label {
        let settings = settings::get::<ThemeSettings>(cx);
        let font_cache = cx.font_cache();
        let family_id = settings.buffer_font_family;
        let font_size = settings.buffer_font_size(cx);
        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();

        let text_style = TextStyle {
            color: style.foreground,
            font_family_id: family_id,
            font_family_name: font_cache.family_name(family_id).unwrap(),
            font_id,
            font_size,
            font_properties: Default::default(),
            underline: Default::default(),
        };

        Label::new(text, text_style)
    }
}

impl Entity for ThemeTestbench {
    type Event = ();
}

impl View for ThemeTestbench {
    fn ui_name() -> &'static str {
        "ThemeTestbench"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> AnyElement<Self> {
        let color_scheme = &theme::current(cx).clone().color_scheme;

        Flex::row()
            .with_child(
                Self::render_ramps(color_scheme)
                    .contained()
                    .with_margin_right(10.)
                    .flex(0.1, false),
            )
            .with_child(
                Flex::column()
                    .with_child(Self::render_layer(100, &color_scheme.lowest, cx).flex(1., true))
                    .with_child(Self::render_layer(200, &color_scheme.middle, cx).flex(1., true))
                    .with_child(Self::render_layer(300, &color_scheme.highest, cx).flex(1., true))
                    .flex(1., false),
            )
            .into_any()
    }
}

impl Item for ThemeTestbench {
    fn tab_content<T: View>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &AppContext,
    ) -> AnyElement<T> {
        Label::new("Theme Testbench", style.label.clone())
            .aligned()
            .contained()
            .into_any()
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("ThemeTestBench")
    }

    fn deserialize(
        _project: ModelHandle<Project>,
        _workspace: WeakViewHandle<Workspace>,
        _workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<gpui::anyhow::Result<ViewHandle<Self>>> {
        Task::ready(Ok(cx.add_view(|_| Self {})))
    }
}
