use super::selectable_tile::SelectableTile;
use component::{example_group_with_title, single_example};
use gpui::{
    AnyElement, App, IntoElement, RenderOnce, StatefulInteractiveElement, Window, prelude::*,
};
use smallvec::SmallVec;
use ui::{Label, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct SelectableTileRow {
    gap: Pixels,
    tiles: SmallVec<[SelectableTile; 8]>,
}

impl SelectableTileRow {
    pub fn new() -> Self {
        Self {
            gap: px(12.),
            tiles: SmallVec::new(),
        }
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into();
        self
    }

    pub fn tile(mut self, tile: SelectableTile) -> Self {
        self.tiles.push(tile);
        self
    }
}

impl RenderOnce for SelectableTileRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex().w_full().px_5().gap(self.gap).children(self.tiles)
    }
}

impl Component for SelectableTileRow {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn sort_name() -> &'static str {
        "RowSelectableTile"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let examples = example_group_with_title(
            "SelectableTileRow Examples",
            vec![
                single_example(
                    "Theme Tiles",
                    SelectableTileRow::new()
                        .gap(px(12.))
                        .tile(
                            SelectableTile::new("tile1", px(100.), px(80.))
                                .selected(true)
                                .child(
                                    div()
                                        .size_full()
                                        .bg(gpui::red())
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(Label::new("Dark")),
                                ),
                        )
                        .tile(
                            SelectableTile::new("tile2", px(100.), px(80.)).child(
                                div()
                                    .size_full()
                                    .bg(gpui::green())
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(Label::new("Light")),
                            ),
                        )
                        .tile(
                            SelectableTile::new("tile3", px(100.), px(80.))
                                .parent_focused(true)
                                .child(
                                    div()
                                        .size_full()
                                        .bg(gpui::blue())
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(Label::new("Auto")),
                                ),
                        )
                        .into_any_element(),
                ),
                single_example(
                    "Icon Tiles",
                    SelectableTileRow::new()
                        .gap(px(8.))
                        .tile(
                            SelectableTile::new("icon1", px(48.), px(48.))
                                .selected(true)
                                .child(Icon::new(IconName::Code).size(IconSize::Medium)),
                        )
                        .tile(
                            SelectableTile::new("icon2", px(48.), px(48.))
                                .child(Icon::new(IconName::Terminal).size(IconSize::Medium)),
                        )
                        .tile(
                            SelectableTile::new("icon3", px(48.), px(48.))
                                .child(Icon::new(IconName::FileCode).size(IconSize::Medium)),
                        )
                        .tile(
                            SelectableTile::new("icon4", px(48.), px(48.))
                                .child(Icon::new(IconName::Settings).size(IconSize::Medium)),
                        )
                        .into_any_element(),
                ),
            ],
        );

        Some(v_flex().p_4().gap_4().child(examples).into_any_element())
    }
}
