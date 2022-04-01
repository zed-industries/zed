use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{executor::Background, fonts::HighlightStyle};
use std::{ops::Range, sync::Arc};

#[derive(Debug)]
pub struct Outline<T> {
    pub items: Vec<OutlineItem<T>>,
    candidates: Vec<StringMatchCandidate>,
    path_candidates: Vec<StringMatchCandidate>,
    path_candidate_prefixes: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutlineItem<T> {
    pub depth: usize,
    pub range: Range<T>,
    pub text: String,
    pub highlight_ranges: Vec<(Range<usize>, HighlightStyle)>,
    pub name_ranges: Vec<Range<usize>>,
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
                .map(|range| &item.text[range.start as usize..range.end as usize])
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

    pub async fn search(&self, query: &str, executor: Arc<Background>) -> Vec<StringMatch> {
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
                    while *position >= preceding_ranges_len + name_range.len() as usize {
                        preceding_ranges_len += name_range.len();
                        name_range = name_ranges.next().unwrap();
                    }
                    *position = name_range.start as usize + (*position - preceding_ranges_len);
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
