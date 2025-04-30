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
        let syntax = self.theme.syntax();
        let syntax_colors = [syntax.get("foo.bar")];

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
                item_skeleton(
                    relative(width).into(),
                    px(3.),
                    color.text.alpha(width - 0.25),
                )
            })
            .collect::<Vec<_>>();

        // This should be rows of horizontal item skeletons
        // using colors from the syntax_colors vec.
        // It will create something that looks like syntax highlighting.
        //
        // let pseudo_code_skeleton = todo!("Implement pseudo code skeleton");

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

        // Generate abstract code blocks using syntax theme colors
        let pseudo_code_skeleton = |theme: Arc<Theme>, seed: f32| -> AnyElement {
            let colors = theme.colors();
            let syntax = theme.syntax();

            // Get syntax colors for different code elements
            let keyword_color = syntax.get("keyword").color;
            let function_color = syntax.get("function").color;
            let string_color = syntax.get("string").color;
            let comment_color = syntax.get("comment").color;
            let variable_color = syntax.get("variable").color;
            let type_color = syntax.get("type").color;
            let punctuation_color = syntax.get("punctuation").color;

            // Array of colors to pick from for code elements
            let syntax_colors = [
                keyword_color,
                function_color,
                string_color,
                variable_color,
                type_color,
                punctuation_color,
                comment_color,
            ];

            // Generate random line configuration based on seed
            let line_width = |line_idx: usize, block_idx: usize| -> f32 {
                let val = (seed * 100.0 + line_idx as f32 * 20.0 + block_idx as f32 * 5.0).sin()
                    * 0.5
                    + 0.5;
                0.05 + val * 0.25 // Width between 5% and 30%
            };

            // Generate indentation based on seed and line index
            let indentation = |line_idx: usize| -> f32 {
                let val = (seed * 200.0 + line_idx as f32 * 15.0).cos() * 0.5 + 0.5;
                val * 0.3 // Indent between 0% and 30%
            };

            // Pick a color based on seed, line and block index
            let pick_color = |line_idx: usize, block_idx: usize| -> Hsla {
                let idx = ((seed * 10.0 + line_idx as f32 * 7.0 + block_idx as f32 * 3.0).sin()
                    * 3.5)
                    .abs() as usize
                    % syntax_colors.len();
                syntax_colors[idx].unwrap_or(colors.text)
            };

            // Number of code lines
            let line_count = 7;

            // Create the lines
            let lines = (0..line_count)
                .map(|line_idx| {
                    // Each line has 1-4 blocks based on seed and line index
                    let block_count = (((seed * 30.0 + line_idx as f32 * 12.0).sin() * 0.5 + 0.5)
                        * 3.0)
                        .round() as usize
                        + 1;

                    let indent = indentation(line_idx);

                    // Create the blocks for this line
                    let blocks = (0..block_count)
                        .map(|block_idx| {
                            let width = line_width(line_idx, block_idx);
                            let color = pick_color(line_idx, block_idx);
                            item_skeleton(relative(width).into(), px(6.), color)
                        })
                        .collect::<Vec<_>>();

                    // Create a flex container for this line
                    h_flex().gap_2().ml(relative(indent)).children(blocks)
                })
                .collect::<Vec<_>>();

            v_flex()
                .size_full()
                .p_1()
                .gap_2()
                .children(lines)
                .into_any_element()
        };

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
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .bg(color.editor_background)
                    .p_2()
                    .child(pseudo_code_skeleton(self.theme.clone(), self.seed)),
            );

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
