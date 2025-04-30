use std::sync::Arc;
use theme::{Theme, ThemeRegistry};
use ui::{
    IntoElement, RenderOnce, component_prelude::Documented, prelude::*, utils::inner_corner_radius,
};

/// Shows a preview of a theme as an abstract illustration
/// of a thumbnail-sized editor.
#[derive(IntoElement, RegisterComponent, Documented)]
pub struct ThemePreviewTile {
    theme: Arc<Theme>,
    selected: bool,
}

impl ThemePreviewTile {
    pub fn new(theme: Arc<Theme>, selected: bool) -> Self {
        Self { theme, selected }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for ThemePreviewTile {
    fn render(self, _window: &mut ui::Window, _cx: &mut ui::App) -> impl IntoElement {
        let color = self.theme.colors();
        let root_radius = px(8.0);
        let root_border = px(2.0);
        let root_padding = px(2.0);
        let child_border = px(1.0);
        let inner_radius =
            inner_corner_radius(root_radius, root_border, root_padding, child_border);

        div()
            .size_full()
            .rounded(root_radius)
            .p(root_padding)
            .border(root_border)
            .border_color(color.border_transparent)
            .when(self.selected, |this| {
                this.border_color(color.border_selected)
            })
            .child(
                div()
                    .size_full()
                    .rounded(inner_radius)
                    .border(child_border)
                    .border_color(color.border)
                    .bg(color.background),
            )
    }
}

impl Component for ThemePreviewTile {
    fn description() -> Option<&'static str> {
        Some(Self::DOCS)
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let theme_registry = ThemeRegistry::global(cx);

        let one_dark = theme_registry.get("One Dark");
        let one_light = theme_registry.get("One Light");
        let gruvbox_dark = theme_registry.get("Gruvbox Dark");
        let gruvbox_light = theme_registry.get("Gruvbox Light");

        let themes_to_preview = vec![
            one_dark.clone().ok(),
            one_light.clone().ok(),
            gruvbox_dark.clone().ok(),
            gruvbox_light.clone().ok(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        Some(
            v_flex()
                .gap_6()
                .p_4()
                .children({
                    if let Some(one_dark) = one_dark.ok() {
                        vec![example_group(vec![
                            single_example(
                                "Default",
                                div()
                                    .w(px(240.))
                                    .h(px(180.))
                                    .child(ThemePreviewTile::new(one_dark.clone(), false))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Selected",
                                div()
                                    .w(px(240.))
                                    .h(px(180.))
                                    .child(ThemePreviewTile::new(one_dark, true))
                                    .into_any_element(),
                            ),
                        ])]
                    } else {
                        vec![]
                    }
                })
                .child(
                    example_group(vec![single_example(
                        "Default Themes",
                        h_flex()
                            .gap_4()
                            .children(
                                themes_to_preview
                                    .iter()
                                    .map(|theme| {
                                        div()
                                            .w(px(200.))
                                            .h(px(140.))
                                            .child(ThemePreviewTile::new(theme.clone(), false))
                                    })
                                    .collect::<Vec<_>>(),
                            )
                            .into_any_element(),
                    )])
                    .grow(),
                )
                .into_any_element(),
        )
    }
}
