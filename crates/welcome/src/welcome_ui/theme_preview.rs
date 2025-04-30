use gpui::{Hsla, Length};
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
    seed: f32,
}

impl ThemePreviewTile {
    pub fn new(theme: Arc<Theme>, selected: bool, seed: f32) -> Self {
        Self {
            theme,
            selected,
            seed,
        }
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

        let item_skeleton = |w: Length, h: Pixels, bg: Hsla| div().w(w).h(h).rounded_full().bg(bg);

        let sidebar_seeded_width = |seed: f32, index: usize| {
            let value = (seed * 1000.0 + index as f32 * 10.0).sin() * 0.5 + 0.5;
            0.5 + value * 0.35
        };

        let sidebar_skeleton_items = 11;

        let sidebar_skeleton = (0..sidebar_skeleton_items)
            .map(|i| {
                let width = sidebar_seeded_width(self.seed, i);
                item_skeleton(relative(width).into(), px(3.), color.text_muted)
            })
            .collect::<Vec<_>>();

        let sidebar = div()
            .h_full()
            .w(relative(0.25))
            .border_r(px(1.))
            .border_color(color.border_transparent)
            .bg(color.panel_background)
            .child(
                div()
                    .p_2()
                    .flex()
                    .flex_col()
                    .size_full()
                    .gap(px(4.))
                    .children(sidebar_skeleton),
            );

        let pane = div()
            .h_full()
            .flex_grow()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .border_color(color.border)
                    .border_b(px(1.))
                    .h(relative(0.1))
                    .bg(color.tab_bar_background),
            )
            .child(div().flex_1().w_full().bg(color.editor_background));

        let content = div().size_full().flex().child(sidebar).child(pane);

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
                    .bg(color.background)
                    .child(content),
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
                                    .child(ThemePreviewTile::new(one_dark.clone(), false, 0.42))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Selected",
                                div()
                                    .w(px(240.))
                                    .h(px(180.))
                                    .child(ThemePreviewTile::new(one_dark, true, 0.42))
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
                                    .enumerate()
                                    .map(|(i, theme)| {
                                        div().w(px(200.)).h(px(140.)).child(ThemePreviewTile::new(
                                            theme.clone(),
                                            false,
                                            0.1 * i as f32 + 0.5,
                                        ))
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
