use std::path::{Path, PathBuf};

use fuzzy::StringMatch;
use ui::{prelude::*, HighlightedLabel};
use util::paths::PathExt;

#[derive(Clone)]
pub struct HighlightedMatchWithPaths {
    names: HighlightedText,
    paths: Vec<HighlightedText>,
    render_paths: bool,
}

#[derive(Debug, Clone, IntoElement)]
struct HighlightedText {
    text: String,
    highlight_positions: Vec<usize>,
    char_count: usize,
}

impl HighlightedText {
    fn join(components: impl Iterator<Item = Self>, separator: &str) -> Self {
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
        }
    }
}

impl RenderOnce for HighlightedText {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        HighlightedLabel::new(self.text, self.highlight_positions)
    }
}

impl HighlightedMatchWithPaths {
    pub fn new(
        label: &StringMatch,
        paths: &[PathBuf],
        highlight_in_paths: bool,
        render_paths: bool,
    ) -> Self {
        let (names, paths) = if highlight_in_paths {
            let mut path_start_offset = 0;
            paths
                .iter()
                .map(|path| {
                    let path = path.compact();
                    let highlighted_text = Self::highlights_for_path(
                        path.as_ref(),
                        &label.positions,
                        path_start_offset,
                    );

                    path_start_offset += highlighted_text.1.char_count;
                    highlighted_text
                })
                .unzip()
        } else {
            (
                vec![Some(HighlightedText {
                    text: label.string.clone(),
                    highlight_positions: label.positions.clone(),
                    char_count: label.string.chars().count(),
                })],
                paths
                    .iter()
                    .map(|path| {
                        let path_string = path.to_string_lossy().to_string();
                        HighlightedText {
                            char_count: path_string.chars().count(),
                            highlight_positions: Vec::new(),
                            text: path_string,
                        }
                    })
                    .collect(),
            )
        };
        Self {
            names: HighlightedText::join(names.into_iter().filter_map(|name| name), ", "),
            paths,
            render_paths,
        }
    }

    pub fn render_paths_children(&mut self, element: Div) -> Div {
        element.children(self.paths.clone().into_iter().map(|path| {
            HighlightedLabel::new(path.text, path.highlight_positions)
                .size(LabelSize::Small)
                .color(Color::Muted)
        }))
    }

    // Compute the highlighted text for the name and path
    fn highlights_for_path(
        path: &Path,
        match_positions: &Vec<usize>,
        path_start_offset: usize,
    ) -> (Option<HighlightedText>, HighlightedText) {
        let path_string = path.to_string_lossy();
        let path_char_count = path_string.chars().count();
        // Get the subset of match highlight positions that line up with the given path.
        // Also adjusts them to start at the path start
        let path_positions = match_positions
            .iter()
            .copied()
            .skip_while(|position| *position < path_start_offset)
            .take_while(|position| *position < path_start_offset + path_char_count)
            .map(|position| position - path_start_offset)
            .collect::<Vec<_>>();

        // Again subset the highlight positions to just those that line up with the file_name
        // again adjusted to the start of the file_name
        let file_name_text_and_positions = path.file_name().map(|file_name| {
            let text = file_name.to_string_lossy();
            let char_count = text.chars().count();
            let file_name_start = path_char_count - char_count;
            let highlight_positions = path_positions
                .iter()
                .copied()
                .skip_while(|position| *position < file_name_start)
                .take_while(|position| *position < file_name_start + char_count)
                .map(|position| position - file_name_start)
                .collect::<Vec<_>>();
            HighlightedText {
                text: text.to_string(),
                highlight_positions,
                char_count,
            }
        });

        (
            file_name_text_and_positions,
            HighlightedText {
                text: path_string.to_string(),
                highlight_positions: path_positions,
                char_count: path_char_count,
            },
        )
    }
}

impl RenderOnce for HighlightedMatchWithPaths {
    fn render(mut self, _: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .child(self.names.clone())
            .when(self.render_paths, |this| self.render_paths_children(this))
    }
}
