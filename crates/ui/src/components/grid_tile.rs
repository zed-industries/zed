#![allow(warnings, dead_code)]

use gpui::Length;

use crate::prelude::*;

static DEFAULT_WIDTH: DefiniteLength = DefiniteLength::Fraction(0.33);

/// A tile component for grid layouts
#[derive(IntoElement, RegisterComponent)]
pub struct GridTile {
    image: Option<AnyElement>,
    image_contained: bool,
    width: DefiniteLength,

    /// Whether the tile should be selectable
    selectable: bool,
    /// Whether the tile is currently selected
    selected: bool,
    /// Custom padding for the tile
    padding: Option<Pixels>,
    label: SharedString,
    description: Option<AnyElement>,
}

impl GridTile {
    /// Creates a new grid tile
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            image: None,
            image_contained: false,
            width: DEFAULT_WIDTH,
            selectable: false,
            selected: false,
            padding: None,
            label: label.into(),
            description: None,
        }
    }
    pub fn with_image(label: impl Into<SharedString>, image: impl IntoElement) -> Self {
        Self {
            image: Some(image.into_any_element()),
            image_contained: true,
            width: DEFAULT_WIDTH,
            selectable: false,
            selected: false,
            padding: None,
            label: label.into(),
            description: None,
        }
    }

    /// Makes the tile selectable
    pub fn selectable(mut self) -> Self {
        self.selectable = true;
        self
    }

    /// Sets whether the tile is selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets custom padding for the tile
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn text_description(mut self, description: impl Into<SharedString>, cx: &App) -> Self {
        let description_element = div()
            .text_ui_sm()
            .text_color(cx.theme().colors().text_muted)
            .child(description.into())
            .into_any_element();
        self.description = Some(description_element);
        self
    }
}

impl RenderOnce for GridTile {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let bg_color = if self.selected {
            cx.theme().colors().ghost_element_selected
        } else {
            cx.theme().colors().ghost_element_background
        };

        let hover_color = cx.theme().colors().element_hover;

        let padding = self.padding.unwrap_or_else(|| px(8.0));

        h_flex()
            .p(padding)
            .bg(bg_color)
            .flex_initial()
            .w(self.width)
            .max_w(self.width)
            .overflow_hidden()
            .rounded_md()
            .when(self.selectable, |this| {
                this.cursor_pointer().hover(|s| s.bg(hover_color))
            })
            .flex_1()
            .gap_3()
            .when_some(self.image, |this, image| {
                this.child(
                    h_flex()
                        .mt_neg_2()
                        .flex_none()
                        .rounded_full()
                        .size_12()
                        .items_center()
                        .justify_center()
                        .overflow_hidden()
                        .when(self.image_contained, |this| {
                            this.bg(cx.theme().colors().icon.alpha(0.12))
                        })
                        .child(image),
                )
            })
            .child(
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .max_w_full()
                    .child(Label::new(self.label))
                    .when_some(self.description, |this, description| {
                        this.child(div().max_w_full().child(description))
                    }),
            )
    }
}

impl Component for GridTile {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn description() -> Option<&'static str> {
        Some("A versatile tile component for creating grid layouts")
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Tiles",
                        vec![
                            single_example(
                                "Default",
                                GridTile::new("Tile Content")
                                    .into_any_element(),
                            ),
                            single_example(
                                "Selectable",
                                GridTile::new("Selectable Tile")
                                    .selectable()
                                    .into_any_element(),
                            ),
                            single_example(
                                "Selected",
                                GridTile::new("Selected Tile")
                                    .selectable()
                                    .selected(true)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Icon as Image",
                                GridTile::with_image(
                                    "Time Travel",
                                    Icon::new(IconName::HistoryRerun),
                                )
                                .text_description("Seamlessly jump back in time to any point in your codebase–down to the keystroke.", cx)
                                .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Grid Example",
                        vec![single_example(
                            "2x2 Grid",
                            h_flex()
                                .w_full()
                                .gap_2()
                                .flex_wrap()
                                .children(vec![
                                    GridTile::new("Tile 1"),
                                    GridTile::new("Tile 2"),
                                    GridTile::new("Tile 3"),
                                    GridTile::new("Tile 4"),
                                ])
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
