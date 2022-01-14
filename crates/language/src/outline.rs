use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{executor::Background, fonts::HighlightStyle};
use std::{ops::Range, sync::Arc};

#[derive(Debug)]
pub struct Outline<T> {
    pub items: Vec<OutlineItem<T>>,
    candidates: Vec<StringMatchCandidate>,
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
        Self {
            candidates: items
                .iter()
                .map(|item| StringMatchCandidate {
                    char_bag: item.text.as_str().into(),
                    string: item.text.clone(),
                })
                .collect(),
            items,
        }
    }

    pub async fn search(&self, query: &str, executor: Arc<Background>) -> Vec<StringMatch> {
        let mut matches = fuzzy::match_strings(
            &self.candidates,
            query,
            true,
            100,
            &Default::default(),
            executor,
        )
        .await;
        matches.sort_unstable_by_key(|m| m.candidate_index);

        let mut tree_matches = Vec::new();

        let mut prev_item_ix = 0;
        for string_match in matches {
            let outline_match = &self.items[string_match.candidate_index];
            let insertion_ix = tree_matches.len();
            let mut cur_depth = outline_match.depth;
            for (ix, item) in self.items[prev_item_ix..string_match.candidate_index]
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
                            candidate_index,
                            score: Default::default(),
                            positions: Default::default(),
                            string: Default::default(),
                        },
                    );
                    cur_depth -= 1;
                }
            }

            prev_item_ix = string_match.candidate_index + 1;
            tree_matches.push(string_match);
        }

        tree_matches
    }
}
