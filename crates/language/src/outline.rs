use std::ops::Range;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::AppContext;

#[derive(Debug)]
pub struct Outline<T> {
    pub items: Vec<OutlineItem<T>>,
    candidates: Vec<StringMatchCandidate>,
}

#[derive(Clone, Debug)]
pub struct OutlineItem<T> {
    pub id: usize,
    pub depth: usize,
    pub range: Range<T>,
    pub text: String,
    pub name_range_in_text: Range<usize>,
}

impl<T> Outline<T> {
    pub fn new(items: Vec<OutlineItem<T>>) -> Self {
        Self {
            candidates: items
                .iter()
                .map(|item| {
                    let text = &item.text[item.name_range_in_text.clone()];
                    StringMatchCandidate {
                        string: text.to_string(),
                        char_bag: text.into(),
                    }
                })
                .collect(),
            items,
        }
    }

    pub fn search(&self, query: &str, cx: &AppContext) -> Vec<StringMatch> {
        let mut matches = smol::block_on(fuzzy::match_strings(
            &self.candidates,
            query,
            true,
            100,
            &Default::default(),
            cx.background().clone(),
        ));
        matches.sort_unstable_by_key(|m| m.candidate_index);

        let mut tree_matches = Vec::new();

        let mut prev_item_ix = 0;
        for mut string_match in matches {
            let outline_match = &self.items[string_match.candidate_index];
            for position in &mut string_match.positions {
                *position += outline_match.name_range_in_text.start;
            }

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
                    tree_matches.push(StringMatch {
                        candidate_index,
                        score: Default::default(),
                        positions: Default::default(),
                        string: Default::default(),
                    });
                    cur_depth -= 1;
                }
            }

            prev_item_ix = string_match.candidate_index + 1;
            tree_matches.push(string_match);
        }

        tree_matches
    }
}
