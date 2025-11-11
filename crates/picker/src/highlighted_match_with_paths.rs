use ui::{HighlightedLabel, prelude::*};

#[derive(Clone)]
pub struct HighlightedMatchWithPaths {
    pub prefix: Option<SharedString>,
    pub match_label: HighlightedMatch,
    pub paths: Vec<HighlightedMatch>,
}

#[derive(Debug, Clone, IntoElement)]
pub struct HighlightedMatch {
    pub text: String,
    pub highlight_positions: Vec<usize>,
    pub color: Color,
}

impl HighlightedMatch {
    pub fn join(components: impl Iterator<Item = Self>, separator: &str) -> Self {
        // Track a running byte offset and insert separators between parts.
        let mut first = true;
        let mut byte_offset = 0;
        let mut text = String::new();
        let mut highlight_positions = Vec::new();
        for component in components {
            if !first {
                text.push_str(separator);
                byte_offset += separator.len();
            }
            first = false;

            highlight_positions.extend(
                component
                    .highlight_positions
                    .iter()
                    .map(|position| position + byte_offset),
            );
            text.push_str(&component.text);
            byte_offset += component.text.len();
        }

        Self {
            text,
            highlight_positions,
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
            .child(
                h_flex().gap_1().child(self.match_label.clone()).when_some(
                    self.prefix.as_ref(),
                    |this, prefix| {
                        this.child(Label::new(format!("({})", prefix)).color(Color::Muted))
                    },
                ),
            )
            .when(!self.paths.is_empty(), |this| {
                self.render_paths_children(this)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_offsets_positions_by_bytes_not_chars() {
        // "αβγ" is 3 Unicode scalar values, 6 bytes in UTF-8.
        let left_text = "αβγ".to_string();
        let right_text = "label".to_string();
        let left = HighlightedMatch {
            text: left_text,
            highlight_positions: vec![],
            color: Color::Default,
        };
        let right = HighlightedMatch {
            text: right_text,
            highlight_positions: vec![0, 1],
            color: Color::Default,
        };
        let joined = HighlightedMatch::join([left, right].into_iter(), "");

        assert!(
            joined
                .highlight_positions
                .iter()
                .all(|&p| joined.text.is_char_boundary(p)),
            "join produced non-boundary positions {:?} for text {:?}",
            joined.highlight_positions,
            joined.text
        );
    }
}
