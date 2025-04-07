use ui::{HighlightedLabel, prelude::*};

#[derive(Clone)]
pub struct HighlightedMatchWithPaths {
    pub match_label: HighlightedMatch,
    pub paths: Vec<HighlightedMatch>,
}

#[derive(Debug, Clone, IntoElement)]
pub struct HighlightedMatch {
    pub text: String,
    pub highlight_positions: Vec<usize>,
    pub char_count: usize,
    pub color: Color,
}

impl HighlightedMatch {
    pub fn join(components: impl Iterator<Item = Self>, separator: &str) -> Self {
        let mut char_count = 0;
        let separator_char_count = separator.chars().count();
        let mut text = String::new();
        let mut highlight_positions = Vec::new();
        for component in components {
            if char_count != 0 {
                text.push_str(separator);
                char_count += separator_char_count;
            }

            highlight_positions.extend(
                component
                    .highlight_positions
                    .iter()
                    .map(|position| position + char_count),
            );
            text.push_str(&component.text);
            char_count += component.text.chars().count();
        }

        Self {
            text,
            highlight_positions,
            char_count,
            color: Color::Default,
        }
    }

    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }
}
impl RenderOnce for HighlightedMatch {
    fn render(self, _window: &mut Window, _: &mut App) -> impl IntoElement {
        HighlightedLabel::new(self.text, self.highlight_positions).color(self.color)
    }
}

impl HighlightedMatchWithPaths {
    pub fn render_paths_children(&mut self, element: Div) -> Div {
        element.children(self.paths.clone().into_iter().map(|path| {
            HighlightedLabel::new(path.text, path.highlight_positions)
                .size(LabelSize::Small)
                .color(Color::Muted)
        }))
    }
}

impl RenderOnce for HighlightedMatchWithPaths {
    fn render(mut self, _window: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .child(self.match_label.clone())
            .when(!self.paths.is_empty(), |this| {
                self.render_paths_children(this)
            })
    }
}
