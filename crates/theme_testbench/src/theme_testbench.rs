use gpui::{
    actions,
    color::Color,
    elements::{
        Canvas, Container, ContainerStyle, ElementBox, Flex, Label, Margin, MouseEventHandler,
        Padding, ParentElement,
    },
    fonts::TextStyle,
    Border, Element, Entity, MutableAppContext, Quad, RenderContext, View, ViewContext,
};
use project::{Project, ProjectEntryId, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use theme::{ColorScheme, Elevation, Layer, Style, StyleSet};
use workspace::{Item, Workspace};

actions!(theme, [DeployThemeTestbench]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ThemeTestbench::deploy);
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

    fn render_ramps(color_scheme: &ColorScheme) -> Flex {
        fn display_ramp(ramp: &Vec<Color>) -> ElementBox {
            Flex::row()
                .with_children(ramp.iter().cloned().map(|color| {
                    Canvas::new(move |bounds, _, cx| {
                        cx.scene.push_quad(Quad {
                            bounds,
                            background: Some(color),
                            ..Default::default()
                        });
                    })
                    .flex(1.0, false)
                    .boxed()
                }))
                .flex(1.0, false)
                .boxed()
        }

        Flex::column()
            .with_child(display_ramp(&color_scheme.lowest.ramps.neutral))
            .with_child(display_ramp(&color_scheme.lowest.ramps.red))
            .with_child(display_ramp(&color_scheme.lowest.ramps.orange))
            .with_child(display_ramp(&color_scheme.lowest.ramps.yellow))
            .with_child(display_ramp(&color_scheme.lowest.ramps.green))
            .with_child(display_ramp(&color_scheme.lowest.ramps.cyan))
            .with_child(display_ramp(&color_scheme.lowest.ramps.blue))
            .with_child(display_ramp(&color_scheme.lowest.ramps.violet))
            .with_child(display_ramp(&color_scheme.lowest.ramps.magenta))
    }

    fn render_elevation(
        elevation_index: usize,
        elevation: &Elevation,
        cx: &mut RenderContext<'_, Self>,
    ) -> Flex {
        Flex::column()
            .with_child(
                Self::render_layer(elevation_index * 1000 + 100, &elevation.bottom, cx)
                    .flex(1., true)
                    .boxed(),
            )
            .with_child(
                Self::render_layer(elevation_index * 1000 + 200, &elevation.middle, cx)
                    .flex(1., true)
                    .boxed(),
            )
            .with_child(
                Self::render_layer(elevation_index * 1000 + 300, &elevation.top, cx)
                    .flex(1., true)
                    .boxed(),
            )
    }

    fn render_layer(
        layer_index: usize,
        layer: &Layer,
        cx: &mut RenderContext<'_, Self>,
    ) -> Container {
        Flex::column()
            .with_child(
                Self::render_button_set(0, layer_index, "base", &layer.base, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(1, layer_index, "variant", &layer.variant, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(2, layer_index, "on", &layer.on, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(3, layer_index, "info", &layer.info, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(4, layer_index, "positive", &layer.positive, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(5, layer_index, "warning", &layer.warning, cx)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Self::render_button_set(6, layer_index, "negative", &layer.negative, cx)
                    .flex(1., false)
                    .boxed(),
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
        cx: &mut RenderContext<'_, Self>,
    ) -> Flex {
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
    }

    fn render_button(
        button_index: usize,
        layer_index: usize,
        text: &'static str,
        style_set: &StyleSet,
        style_override: Option<fn(&StyleSet) -> &Style>,
        cx: &mut RenderContext<'_, Self>,
    ) -> ElementBox {
        enum TestBenchButton {}
        MouseEventHandler::<TestBenchButton>::new(layer_index + button_index, cx, |state, cx| {
            let style = if let Some(style_override) = style_override {
                style_override(&style_set)
            } else if state.clicked.is_some() {
                &style_set.pressed
            } else if state.hovered {
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
                .boxed()
        })
        .flex(1., true)
        .boxed()
    }

    fn render_label(text: String, style: &Style, cx: &mut RenderContext<'_, Self>) -> Label {
        let settings = cx.global::<Settings>();
        let font_cache = cx.font_cache();
        let family_id = settings.buffer_font_family;
        let font_size = settings.buffer_font_size;
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

    fn elevation_style(elevation: &Elevation) -> ContainerStyle {
        let style = ContainerStyle {
            margin: Margin {
                top: 10.,
                bottom: 10.,
                left: 10.,
                right: 10.,
            },
            background_color: Some(elevation.bottom.base.default.background),
            ..Default::default()
        };

        if elevation.shadow.is_some() {
            ContainerStyle {
                padding: Padding {
                    top: 10.,
                    bottom: 10.,
                    left: 10.,
                    right: 10.,
                },
                border: Border {
                    width: 1.,
                    color: elevation.bottom.base.default.border,
                    overlay: false,
                    top: true,
                    bottom: true,
                    left: true,
                    right: true,
                },
                corner_radius: 32.,
                shadow: elevation.shadow,
                ..style
            }
        } else {
            style
        }
    }
}

impl Entity for ThemeTestbench {
    type Event = ();
}

impl View for ThemeTestbench {
    fn ui_name() -> &'static str {
        "ThemeTestbench"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let color_scheme = &cx.global::<Settings>().theme.clone().color_scheme;

        Flex::row()
            .with_child(
                Self::render_ramps(color_scheme)
                    .contained()
                    .with_margin_right(10.)
                    .flex(0.2, false)
                    .boxed(),
            )
            .with_child(
                Self::render_elevation(0, &color_scheme.lowest, cx)
                    .flex(1., true)
                    .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(
                        Self::render_elevation(1, &color_scheme.middle, cx)
                            .flex(1., true)
                            .boxed(),
                    )
                    .with_child(
                        Container::new(
                            Self::render_elevation(2, &color_scheme.highest, cx).boxed(),
                        )
                        .with_style(Self::elevation_style(&color_scheme.highest))
                        .flex(1., true)
                        .boxed(),
                    )
                    .contained()
                    .with_style(Self::elevation_style(&color_scheme.middle))
                    .flex(2., true)
                    .boxed(),
            )
            .contained()
            .with_style(Self::elevation_style(&color_scheme.lowest))
            .boxed()
    }
}

impl Item for ThemeTestbench {
    fn tab_content(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &gpui::AppContext,
    ) -> gpui::ElementBox {
        Label::new("Theme Testbench".into(), style.label.clone())
            .aligned()
            .contained()
            .boxed()
    }

    fn project_path(&self, _: &gpui::AppContext) -> Option<ProjectPath> {
        None
    }

    fn project_entry_ids(&self, _: &gpui::AppContext) -> SmallVec<[ProjectEntryId; 3]> {
        SmallVec::new()
    }

    fn is_singleton(&self, _: &gpui::AppContext) -> bool {
        false
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {}

    fn can_save(&self, _: &gpui::AppContext) -> bool {
        false
    }

    fn save(
        &mut self,
        _: gpui::ModelHandle<Project>,
        _: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save should not have been called");
    }

    fn save_as(
        &mut self,
        _: gpui::ModelHandle<Project>,
        _: std::path::PathBuf,
        _: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save_as should not have been called");
    }

    fn reload(
        &mut self,
        _: gpui::ModelHandle<Project>,
        _: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        gpui::Task::ready(Ok(()))
    }

    fn to_item_events(_: &Self::Event) -> Vec<workspace::ItemEvent> {
        Vec::new()
    }
}
