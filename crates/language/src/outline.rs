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

#[derive(Clone, Debug)]
pub struct OutlineItem<T> {
    pub depth: usize,
    pub range: Range<T>,
    pub text: String,
    pub highlight_ranges: Vec<(Range<usize>, HighlightStyle)>,
}

impl<T> Outline<T> {
    pub fn new(items: Vec<OutlineItem<T>>) -> Self {
        let mut path_candidates = Vec::new();
        let mut path_candidate_prefixes = Vec::new();
        let mut item_text = String::new();
        let mut stack = Vec::new();

        for (id, item) in items.iter().enumerate() {
            if item.depth < stack.len() {
                stack.truncate(item.depth);
                item_text.truncate(stack.last().copied().unwrap_or(0));
            }
            if !item_text.is_empty() {
                item_text.push(' ');
            }
            path_candidate_prefixes.push(item_text.len());
            item_text.push_str(&item.text);
            stack.push(item_text.len());

            path_candidates.push(StringMatchCandidate {
                id,
                string: item_text.clone(),
                char_bag: item_text.as_str().into(),
            });
        }

        Self {
            candidates: items
                .iter()
                .enumerate()
                .map(|(id, item)| StringMatchCandidate {
                    id,
                    char_bag: item.text.as_str().into(),
                    string: item.text.clone(),
                })
                .collect(),
            path_candidates,
            path_candidate_prefixes,
            items,
        }
    }

    pub async fn search(&self, query: &str, executor: Arc<Background>) -> Vec<StringMatch> {
        let query = query.trim_start();
        let is_path_query = query.contains(' ');
        let mut matches = fuzzy::match_strings(
            if is_path_query {
                &self.path_candidates
            } else {
                &self.candidates
            },
            query,
            true,
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
