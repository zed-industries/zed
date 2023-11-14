use crate::prelude::*;
use crate::{h_stack, v_stack, KeyBinding, Label, TextColor};

#[derive(Component)]
pub struct Palette {
    id: ElementId,
    input_placeholder: SharedString,
    empty_string: SharedString,
    items: Vec<PaletteItem>,
    default_order: OrderMethod,
}

impl Palette {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            input_placeholder: "Find something...".into(),
            empty_string: "No items found.".into(),
            items: vec![],
            default_order: OrderMethod::default(),
        }
    }

    pub fn items(mut self, items: Vec<PaletteItem>) -> Self {
        self.items = items;
        self
    }

    pub fn placeholder(mut self, input_placeholder: impl Into<SharedString>) -> Self {
        self.input_placeholder = input_placeholder.into();
        self
    }

    pub fn empty_string(mut self, empty_string: impl Into<SharedString>) -> Self {
        self.empty_string = empty_string.into();
        self
    }

    // TODO: Hook up sort order
    pub fn default_order(mut self, default_order: OrderMethod) -> Self {
        self.default_order = default_order;
        self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        v_stack()
            .id(self.id.clone())
            .w_96()
            .rounded_lg()
            .bg(cx.theme().colors().elevated_surface_background)
            .border()
            .border_color(cx.theme().colors().border)
            .child(
                v_stack()
                    .gap_px()
                    .child(v_stack().py_0p5().px_1().child(div().px_2().py_0p5().child(
                        Label::new(self.input_placeholder.clone()).color(TextColor::Placeholder),
                    )))
                    .child(
                        div()
                            .h_px()
                            .w_full()
                            .bg(cx.theme().colors().element_background),
                    )
                    .child(
                        v_stack()
                            .id("items")
                            .py_0p5()
                            .px_1()
                            .grow()
                            .max_h_96()
                            .overflow_y_scroll()
                            .children(
                                vec![if self.items.is_empty() {
                                    Some(
                                        h_stack().justify_between().px_2().py_1().child(
                                            Label::new(self.empty_string.clone())
                                                .color(TextColor::Muted),
                                        ),
                                    )
                                } else {
                                    None
                                }]
                                .into_iter()
                                .flatten(),
                            )
                            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                                h_stack()
                                    .id(index)
                                    .justify_between()
                                    .px_2()
                                    .py_0p5()
                                    .rounded_lg()
                                    .hover(|style| {
                                        style.bg(cx.theme().colors().ghost_element_hover)
                                    })
                                    .active(|style| {
                                        style.bg(cx.theme().colors().ghost_element_active)
                                    })
                                    .child(item)
                            })),
                    ),
            )
    }
}

#[derive(Component)]
pub struct PaletteItem {
    pub label: SharedString,
    pub sublabel: Option<SharedString>,
    pub key_binding: Option<KeyBinding>,
}

impl PaletteItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            sublabel: None,
            key_binding: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn sublabel(mut self, sublabel: impl Into<Option<SharedString>>) -> Self {
        self.sublabel = sublabel.into();
        self
    }

    pub fn key_binding(mut self, key_binding: impl Into<Option<KeyBinding>>) -> Self {
        self.key_binding = key_binding.into();
        self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div()
            .flex()
            .flex_row()
            .grow()
            .justify_between()
            .child(
                v_stack()
                    .child(Label::new(self.label.clone()))
                    .children(self.sublabel.clone().map(|sublabel| Label::new(sublabel))),
            )
            .children(self.key_binding)
    }
}

use gpui::ElementId;
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui::{Div, Render};

    use crate::{binding, Story};

    use super::*;

    pub struct PaletteStory;

    impl Render for PaletteStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            {
                Story::container(cx)
                    .child(Story::title_for::<_, Palette>(cx))
                    .child(Story::label(cx, "Default"))
                    .child(Palette::new("palette-1"))
                    .child(Story::label(cx, "With Items"))
                    .child(
                        Palette::new("palette-2")
                            .placeholder("Execute a command...")
                            .items(vec![
                                PaletteItem::new("theme selector: toggle")
                                    .key_binding(KeyBinding::new(binding("cmd-k cmd-t"))),
                                PaletteItem::new("assistant: inline assist")
                                    .key_binding(KeyBinding::new(binding("cmd-enter"))),
                                PaletteItem::new("assistant: quote selection")
                                    .key_binding(KeyBinding::new(binding("cmd-<"))),
                                PaletteItem::new("assistant: toggle focus")
                                    .key_binding(KeyBinding::new(binding("cmd-?"))),
                                PaletteItem::new("auto update: check"),
                                PaletteItem::new("auto update: view release notes"),
                                PaletteItem::new("branches: open recent")
                                    .key_binding(KeyBinding::new(binding("cmd-alt-b"))),
                                PaletteItem::new("chat panel: toggle focus"),
                                PaletteItem::new("cli: install"),
                                PaletteItem::new("client: sign in"),
                                PaletteItem::new("client: sign out"),
                                PaletteItem::new("editor: cancel")
                                    .key_binding(KeyBinding::new(binding("escape"))),
                            ]),
                    )
            }
        }
    }
}
