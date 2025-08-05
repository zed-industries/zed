#![allow(unused, dead_code)]
use gpui::{Hsla, Length};
use std::sync::Arc;
use theme::{Theme, ThemeColors, ThemeRegistry};
use ui::{
    IntoElement, RenderOnce, component_prelude::Documented, prelude::*, utils::inner_corner_radius,
};

#[derive(Clone, PartialEq)]
pub enum ThemePreviewStyle {
    Bordered,
    Borderless,
    SideBySide(Arc<Theme>),
}

/// Shows a preview of a theme as an abstract illustration
/// of a thumbnail-sized editor.
#[derive(IntoElement, RegisterComponent, Documented)]
pub struct ThemePreviewTile {
    theme: Arc<Theme>,
    seed: f32,
    style: ThemePreviewStyle,
}

impl ThemePreviewTile {
    pub const SKELETON_HEIGHT_DEFAULT: Pixels = px(2.);
    pub const SIDEBAR_SKELETON_ITEM_COUNT: usize = 8;
    pub const SIDEBAR_WIDTH_DEFAULT: DefiniteLength = relative(0.25);
    pub const ROOT_RADIUS: Pixels = px(8.0);
    pub const ROOT_BORDER: Pixels = px(2.0);
    pub const ROOT_PADDING: Pixels = px(2.0);
    pub const CHILD_BORDER: Pixels = px(1.0);
    pub const CHILD_RADIUS: std::cell::LazyCell<Pixels> = std::cell::LazyCell::new(|| {
        inner_corner_radius(
            Self::ROOT_RADIUS,
            Self::ROOT_BORDER,
            Self::ROOT_PADDING,
            Self::CHILD_BORDER,
        )
    });

    pub fn new(theme: Arc<Theme>, seed: f32) -> Self {
        Self {
            theme,
            seed,
            style: ThemePreviewStyle::Bordered,
        }
    }

    pub fn style(mut self, style: ThemePreviewStyle) -> Self {
        self.style = style;
        self
    }

    pub fn item_skeleton(w: Length, h: Length, bg: Hsla) -> impl IntoElement {
        div().w(w).h(h).rounded_full().bg(bg)
    }

    pub fn render_sidebar_skeleton_items(
        seed: f32,
        colors: &ThemeColors,
        skeleton_height: impl Into<Length> + Clone,
    ) -> [impl IntoElement; Self::SIDEBAR_SKELETON_ITEM_COUNT] {
        let skeleton_height = skeleton_height.into();
        std::array::from_fn(|index| {
            let width = {
                let value = (seed * 1000.0 + index as f32 * 10.0).sin() * 0.5 + 0.5;
                0.5 + value * 0.45
            };
            Self::item_skeleton(
                relative(width).into(),
                skeleton_height,
                colors.text.alpha(0.45),
            )
        })
    }

    pub fn render_pseudo_code_skeleton(
        seed: f32,
        theme: Arc<Theme>,
        skeleton_height: impl Into<Length>,
    ) -> impl IntoElement {
        let colors = theme.colors();
        let syntax = theme.syntax();

        let keyword_color = syntax.get("keyword").color;
        let function_color = syntax.get("function").color;
        let string_color = syntax.get("string").color;
        let comment_color = syntax.get("comment").color;
        let variable_color = syntax.get("variable").color;
        let type_color = syntax.get("type").color;
        let punctuation_color = syntax.get("punctuation").color;

        let syntax_colors = [
            keyword_color,
            function_color,
            string_color,
            variable_color,
            type_color,
            punctuation_color,
            comment_color,
        ];

        let skeleton_height = skeleton_height.into();

        let line_width = |line_idx: usize, block_idx: usize| -> f32 {
            let val =
                (seed * 100.0 + line_idx as f32 * 20.0 + block_idx as f32 * 5.0).sin() * 0.5 + 0.5;
            0.05 + val * 0.2
        };

        let indentation = |line_idx: usize| -> f32 {
            let step = line_idx % 6;
            if step < 3 {
                step as f32 * 0.1
            } else {
                (5 - step) as f32 * 0.1
            }
        };

        let pick_color = |line_idx: usize, block_idx: usize| -> Hsla {
            let idx = ((seed * 10.0 + line_idx as f32 * 7.0 + block_idx as f32 * 3.0).sin() * 3.5)
                .abs() as usize
                % syntax_colors.len();
            syntax_colors[idx].unwrap_or(colors.text)
        };

        let line_count = 13;

        let lines = (0..line_count)
            .map(|line_idx| {
                let block_count = (((seed * 30.0 + line_idx as f32 * 12.0).sin() * 0.5 + 0.5) * 3.0)
                    .round() as usize
                    + 2;

                let indent = indentation(line_idx);

                let blocks = (0..block_count)
                    .map(|block_idx| {
                        let width = line_width(line_idx, block_idx);
                        let color = pick_color(line_idx, block_idx);
                        Self::item_skeleton(relative(width).into(), skeleton_height, color)
                    })
                    .collect::<Vec<_>>();

                h_flex().gap(px(2.)).ml(relative(indent)).children(blocks)
            })
            .collect::<Vec<_>>();

        v_flex().size_full().p_1().gap_1p5().children(lines)
    }

    pub fn render_sidebar(
        seed: f32,
        colors: &ThemeColors,
        width: impl Into<Length> + Clone,
        skeleton_height: impl Into<Length>,
    ) -> impl IntoElement {
        div()
            .h_full()
            .w(width)
            .border_r(px(1.))
            .border_color(colors.border_transparent)
            .bg(colors.panel_background)
            .child(v_flex().p_2().size_full().gap_1().children(
                Self::render_sidebar_skeleton_items(seed, colors, skeleton_height.into()),
            ))
    }

    pub fn render_pane(
        seed: f32,
        theme: Arc<Theme>,
        skeleton_height: impl Into<Length>,
    ) -> impl IntoElement {
        v_flex().h_full().flex_grow().child(
            div()
                .size_full()
                .overflow_hidden()
                .bg(theme.colors().editor_background)
                .p_2()
                .child(Self::render_pseudo_code_skeleton(
                    seed,
                    theme,
                    skeleton_height.into(),
                )),
        )
    }

    pub fn render_editor(
        seed: f32,
        theme: Arc<Theme>,
        sidebar_width: impl Into<Length> + Clone,
        skeleton_height: impl Into<Length> + Clone,
    ) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .bg(theme.colors().background.alpha(1.00))
            .child(Self::render_sidebar(
                seed,
                theme.colors(),
                sidebar_width,
                skeleton_height.clone(),
            ))
            .child(Self::render_pane(seed, theme, skeleton_height.clone()))
    }

    fn render_borderless(seed: f32, theme: Arc<Theme>) -> impl IntoElement {
        return Self::render_editor(
            seed,
            theme,
            Self::SIDEBAR_WIDTH_DEFAULT,
            Self::SKELETON_HEIGHT_DEFAULT,
        );
    }

    fn render_border(seed: f32, theme: Arc<Theme>) -> impl IntoElement {
        div()
            .size_full()
            .p(Self::ROOT_PADDING)
            .rounded(Self::ROOT_RADIUS)
            .child(
                div()
                    .size_full()
                    .rounded(*Self::CHILD_RADIUS)
                    .border(Self::CHILD_BORDER)
                    .border_color(theme.colors().border)
                    .child(Self::render_editor(
                        seed,
                        theme.clone(),
                        Self::SIDEBAR_WIDTH_DEFAULT,
                        Self::SKELETON_HEIGHT_DEFAULT,
                    )),
            )
    }

    fn render_side_by_side(
        seed: f32,
        theme: Arc<Theme>,
        other_theme: Arc<Theme>,
        border_color: Hsla,
    ) -> impl IntoElement {
        let sidebar_width = relative(0.20);

        return div()
            .size_full()
            .p(Self::ROOT_PADDING)
            .rounded(Self::ROOT_RADIUS)
            .child(
                h_flex()
                    .size_full()
                    .relative()
                    .rounded(*Self::CHILD_RADIUS)
                    .border(Self::CHILD_BORDER)
                    .border_color(border_color)
                    .overflow_hidden()
                    .child(div().size_full().child(Self::render_editor(
                        seed,
                        theme.clone(),
                        sidebar_width,
                        Self::SKELETON_HEIGHT_DEFAULT,
                    )))
                    .child(
                        div()
                            .size_full()
                            .absolute()
                            .left_1_2()
                            .bg(other_theme.colors().editor_background)
                            .child(Self::render_editor(
                                seed,
                                other_theme,
                                sidebar_width,
                                Self::SKELETON_HEIGHT_DEFAULT,
                            )),
                    ),
            )
            .into_any_element();
    }
}

impl RenderOnce for ThemePreviewTile {
    fn render(self, _window: &mut ui::Window, _cx: &mut ui::App) -> impl IntoElement {
        match self.style {
            ThemePreviewStyle::Bordered => {
                Self::render_border(self.seed, self.theme).into_any_element()
            }
            ThemePreviewStyle::Borderless => {
                Self::render_borderless(self.seed, self.theme).into_any_element()
            }
            ThemePreviewStyle::SideBySide(other_theme) => Self::render_side_by_side(
                self.seed,
                self.theme,
                other_theme,
                _cx.theme().colors().border,
            )
            .into_any_element(),
        }
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
                        vec![example_group(vec![single_example(
                            "Default",
                            div()
                                .w(px(240.))
                                .h(px(180.))
                                .child(ThemePreviewTile::new(one_dark.clone(), 0.42))
                                .into_any_element(),
                        )])]
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
                                    .map(|(_, theme)| {
                                        div()
                                            .w(px(200.))
                                            .h(px(140.))
                                            .child(ThemePreviewTile::new(theme.clone(), 0.42))
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
