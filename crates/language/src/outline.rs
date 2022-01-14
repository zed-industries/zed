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
    pub name_ranges: Vec<Range<u32>>,
    pub highlight_ranges: Vec<(Range<usize>, HighlightStyle)>,
}

impl<T> Outline<T> {
    pub fn new(items: Vec<OutlineItem<T>>) -> Self {
        Self {
            candidates: items
                .iter()
                .map(|item| {
                    let text = item
                        .name_ranges
                        .iter()
                        .map(|range| &item.text[range.start as usize..range.end as usize])
                        .collect::<String>();
                    StringMatchCandidate {
                        char_bag: text.as_str().into(),
                        string: text,
                    }
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
        for mut string_match in matches {
            let outline_match = &self.items[string_match.candidate_index];

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
