use crate::{BufferSnapshot, Point, ToPoint};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{relative, AppContext, BackgroundExecutor, HighlightStyle, StyledText, TextStyle};
use settings::Settings;
use std::ops::Range;
use theme::{color_alpha, ActiveTheme, ThemeSettings};

/// An outline of all the symbols contained in a buffer.
#[derive(Debug)]
pub struct Outline<T> {
    pub items: Vec<OutlineItem<T>>,
    candidates: Vec<StringMatchCandidate>,
    pub path_candidates: Vec<StringMatchCandidate>,
    path_candidate_prefixes: Vec<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct OutlineItem<T> {
    pub depth: usize,
    pub range: Range<T>,
    pub text: String,
    pub highlight_ranges: Vec<(Range<usize>, HighlightStyle)>,
    pub name_ranges: Vec<Range<usize>>,
    pub body_range: Option<Range<T>>,
    pub annotation_range: Option<Range<T>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolPath(pub String);

impl<T: ToPoint> OutlineItem<T> {
    /// Converts to an equivalent outline item, but with parameterized over Points.
    pub fn to_point(&self, buffer: &BufferSnapshot) -> OutlineItem<Point> {
        OutlineItem {
            depth: self.depth,
            range: self.range.start.to_point(buffer)..self.range.end.to_point(buffer),
            text: self.text.clone(),
            highlight_ranges: self.highlight_ranges.clone(),
            name_ranges: self.name_ranges.clone(),
            body_range: self
                .body_range
                .as_ref()
                .map(|r| r.start.to_point(buffer)..r.end.to_point(buffer)),
            annotation_range: self
                .annotation_range
                .as_ref()
                .map(|r| r.start.to_point(buffer)..r.end.to_point(buffer)),
        }
    }
}

impl<T> Outline<T> {
    pub fn new(items: Vec<OutlineItem<T>>) -> Self {
        let mut candidates = Vec::new();
        let mut path_candidates = Vec::new();
        let mut path_candidate_prefixes = Vec::new();
        let mut path_text = String::new();
        let mut path_stack = Vec::new();

        for (id, item) in items.iter().enumerate() {
            if item.depth < path_stack.len() {
                path_stack.truncate(item.depth);
                path_text.truncate(path_stack.last().copied().unwrap_or(0));
            }
            if !path_text.is_empty() {
                path_text.push(' ');
            }
            path_candidate_prefixes.push(path_text.len());
            path_text.push_str(&item.text);
            path_stack.push(path_text.len());

            let candidate_text = item
                .name_ranges
                .iter()
                .map(|range| &item.text[range.start..range.end])
                .collect::<String>();

            path_candidates.push(StringMatchCandidate::new(id, path_text.clone()));
            candidates.push(StringMatchCandidate::new(id, candidate_text));
        }

        Self {
            candidates,
            path_candidates,
            path_candidate_prefixes,
            items,
        }
    }

    /// Find the most similar symbol to the provided query using normalized Levenshtein distance.
    pub fn find_most_similar(&self, query: &str) -> Option<(SymbolPath, &OutlineItem<T>)> {
        const SIMILARITY_THRESHOLD: f64 = 0.6;

        let (position, similarity) = self
            .path_candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| {
                let similarity = strsim::normalized_levenshtein(&candidate.string, query);
                (index, similarity)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())?;

        if similarity >= SIMILARITY_THRESHOLD {
            self.path_candidates
                .get(position)
                .map(|candidate| SymbolPath(candidate.string.clone()))
                .zip(self.items.get(position))
        } else {
            None
        }
    }

    /// Find all outline symbols according to a longest subsequence match with the query, ordered descending by match score.
    pub async fn search(&self, query: &str, executor: BackgroundExecutor) -> Vec<StringMatch> {
        let query = query.trim_start();
        let is_path_query = query.contains(' ');
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let mut matches = fuzzy::match_strings(
            if is_path_query {
                &self.path_candidates
            } else {
                &self.candidates
            },
            query,
            smart_case,
            100,
            &Default::default(),
            executor.clone(),
        )
        .await;
        matches.sort_unstable_by_key(|m| m.candidate_id);

        let mut tree_matches = Vec::new();

        let mut prev_item_ix = 0;
        for mut string_match in matches {
            let outline_match = &self.items[string_match.candidate_id];
            string_match.string.clone_from(&outline_match.text);

            if is_path_query {
                let prefix_len = self.path_candidate_prefixes[string_match.candidate_id];
                string_match
                    .positions
                    .retain(|position| *position >= prefix_len);
                for position in &mut string_match.positions {
                    *position -= prefix_len;
                }
            } else {
                let mut name_ranges = outline_match.name_ranges.iter();
                let mut name_range = name_ranges.next().unwrap();
                let mut preceding_ranges_len = 0;
                for position in &mut string_match.positions {
                    while *position >= preceding_ranges_len + name_range.len() {
                        preceding_ranges_len += name_range.len();
                        name_range = name_ranges.next().unwrap();
                    }
                    *position = name_range.start + (*position - preceding_ranges_len);
                }
            }

            let insertion_ix = tree_matches.len();
            let mut cur_depth = outline_match.depth;
            for (ix, item) in self.items[prev_item_ix..string_match.candidate_id]
                .iter()
                .enumerate()
                .rev()
            {
                if cur_depth == 0 {
                    break;
                }

                let candidate_index = ix + prev_item_ix;
                if item.depth == cur_depth - 1 {
                    tree_matches.insert(
                        insertion_ix,
                        StringMatch {
                            candidate_id: candidate_index,
                            score: Default::default(),
                            positions: Default::default(),
                            string: Default::default(),
                        },
                    );
                    cur_depth -= 1;
                }
            }

            prev_item_ix = string_match.candidate_id + 1;
            tree_matches.push(string_match);
        }

        tree_matches
    }
}

pub fn render_item<T>(
    outline_item: &OutlineItem<T>,
    match_ranges: impl IntoIterator<Item = Range<usize>>,
    cx: &AppContext,
) -> StyledText {
    let highlight_style = HighlightStyle {
        background_color: Some(color_alpha(cx.theme().colors().text_accent, 0.3)),
        ..Default::default()
    };
    let custom_highlights = match_ranges
        .into_iter()
        .map(|range| (range, highlight_style));

    let settings = ThemeSettings::get_global(cx);

    // TODO: We probably shouldn't need to build a whole new text style here
    // but I'm not sure how to get the current one and modify it.
    // Before this change TextStyle::default() was used here, which was giving us the wrong font and text color.
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.),
        ..Default::default()
    };
    let highlights = gpui::combine_highlights(
        custom_highlights,
        outline_item.highlight_ranges.iter().cloned(),
    );

    StyledText::new(outline_item.text.clone()).with_highlights(&text_style, highlights)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_most_similar_with_low_similarity() {
        let outline = Outline::new(vec![
            OutlineItem {
                depth: 0,
                range: Point::new(0, 0)..Point::new(5, 0),
                text: "fn process".to_string(),
                highlight_ranges: vec![],
                name_ranges: vec![3..10],
                body_range: None,
                annotation_range: None,
            },
            OutlineItem {
                depth: 0,
                range: Point::new(7, 0)..Point::new(12, 0),
                text: "struct DataProcessor".to_string(),
                highlight_ranges: vec![],
                name_ranges: vec![7..20],
                body_range: None,
                annotation_range: None,
            },
        ]);
        assert_eq!(
            outline.find_most_similar("pub fn process"),
            Some((SymbolPath("fn process".into()), &outline.items[0]))
        );
        assert_eq!(
            outline.find_most_similar("async fn process"),
            Some((SymbolPath("fn process".into()), &outline.items[0])),
        );
        assert_eq!(
            outline.find_most_similar("struct Processor"),
            Some((SymbolPath("struct DataProcessor".into()), &outline.items[1]))
        );
        assert_eq!(outline.find_most_similar("struct User"), None);
        assert_eq!(outline.find_most_similar("struct"), None);
    }
}
