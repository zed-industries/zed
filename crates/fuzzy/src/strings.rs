use crate::{CharBag, matcher::MatchCandidate};
use gpui::BackgroundExecutor;
use nucleo::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use std::{
    borrow::{Borrow, Cow},
    cmp, iter,
    ops::Range,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

#[derive(Clone, Debug)]
pub struct StringMatchCandidate {
    pub id: usize,
    pub string: String,
    pub char_bag: CharBag,
}

impl StringMatchCandidate {
    pub fn new(id: usize, string: &str) -> Self {
        Self {
            id,
            string: string.into(),
            char_bag: string.into(),
        }
    }
}

impl MatchCandidate for &StringMatchCandidate {
    fn has_chars(&self, bag: CharBag) -> bool {
        self.char_bag.is_superset(bag)
    }

    fn candidate_chars(&self) -> impl Iterator<Item = char> {
        self.string.chars()
    }
}

#[derive(Clone, Debug)]
pub struct StringMatch {
    pub candidate_id: usize,
    pub score: f64,
    pub positions: Vec<usize>,
    pub string: String,
}

impl StringMatch {
    pub fn ranges(&self) -> impl '_ + Iterator<Item = Range<usize>> {
        let mut positions = self.positions.iter().peekable();
        iter::from_fn(move || {
            if let Some(start) = positions.next().copied() {
                let Some(char_len) = self.char_len_at_index(start) else {
                    log::error!(
                        "Invariant violation: Index {start} out of range or not on a utf-8 boundary in string {:?}",
                        self.string
                    );
                    return None;
                };
                let mut end = start + char_len;
                while let Some(next_start) = positions.peek() {
                    if end == **next_start {
                        let Some(char_len) = self.char_len_at_index(end) else {
                            log::error!(
                                "Invariant violation: Index {end} out of range or not on a utf-8 boundary in string {:?}",
                                self.string
                            );
                            return None;
                        };
                        end += char_len;
                        positions.next();
                    } else {
                        break;
                    }
                }

                return Some(start..end);
            }
            None
        })
    }

    /// Gets the byte length of the utf-8 character at a byte offset. If the index is out of range
    /// or not on a utf-8 boundary then None is returned.
    fn char_len_at_index(&self, ix: usize) -> Option<usize> {
        self.string
            .get(ix..)
            .and_then(|slice| slice.chars().next().map(|char| char.len_utf8()))
    }
}

impl PartialEq for StringMatch {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for StringMatch {}

impl PartialOrd for StringMatch {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StringMatch {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or(cmp::Ordering::Equal)
            .then_with(|| self.candidate_id.cmp(&other.candidate_id))
    }
}

pub async fn match_strings<T>(
    candidates: &[T],
    query: &str,
    smart_case: bool,
    penalize_length: bool, // TODO: re-add this functionality for lsp completions
    max_results: usize,
    cancel_flag: &AtomicBool,
    executor: BackgroundExecutor,
) -> Vec<StringMatch>
where
    T: Borrow<StringMatchCandidate> + Sync,
{
    if candidates.is_empty() || max_results == 0 {
        return Default::default();
    }
    let pattern = Pattern::new(
        query,
        if smart_case {
            CaseMatching::Smart
        } else {
            CaseMatching::Respect
        },
        Normalization::Smart,
        AtomKind::Fuzzy,
    );

    if query.is_empty() {
        return candidates
            .iter()
            .map(|candidate| StringMatch {
                candidate_id: candidate.borrow().id,
                score: 0.,
                positions: Default::default(),
                string: candidate.borrow().string.clone(),
            })
            .collect();
    }

    let num_cpus = executor.num_cpus().min(candidates.len());
    let segment_size = candidates.len().div_ceil(num_cpus);
    let mut segment_results = (0..num_cpus)
        .map(|_| Vec::<StringMatch>::with_capacity(max_results.min(candidates.len())))
        .collect::<Vec<_>>();

    static MATCHERS: Mutex<Vec<nucleo::Matcher>> = Mutex::new(Vec::new());
    let mut matchers: Vec<_> = {
        let mut matchers = MATCHERS.lock().unwrap();
        let numb_matchers = matchers.len();
        matchers.drain(0..cmp::min(num_cpus, numb_matchers)).collect()
    };
    let mut config = nucleo::Config::DEFAULT;
    config.prefer_prefix = true;
    matchers.resize_with(num_cpus, || nucleo::Matcher::new(config.clone()));

    executor
        .scoped(|scope| {
            for (segment_idx, (results, matcher)) in segment_results
                .iter_mut()
                .zip(matchers.iter_mut())
                .enumerate()
            {
                let cancel_flag = &cancel_flag;
                let pattern = pattern.clone();
                scope.spawn(async move {
                    let segment_start = cmp::min(segment_idx * segment_size, candidates.len());
                    let segment_end = cmp::min(segment_start + segment_size, candidates.len());

                    for c in candidates[segment_start..segment_end].iter() {
                        if cancel_flag.load(Ordering::Relaxed) {
                            break;
                        }
                        let candidate = c.borrow();
                        let mut indices = Vec::new();
                        let mut buf = Vec::new();
                        if let Some(score) = pattern.indices(
                            nucleo::Utf32Str::new(&candidate.string, &mut buf),
                            matcher,
                            &mut indices,
                        ) {
                            results.push(StringMatch {
                                candidate_id: candidate.id,
                                score: score as f64,
                                positions: indices.into_iter().map(|n| n as usize).collect(),
                                string: candidate.string.clone(),
                            })
                        };
                    }
                });
            }
        })
        .await;

    MATCHERS.lock().unwrap().append(&mut matchers);

    if cancel_flag.load(Ordering::Relaxed) {
        return Vec::new();
    }

    let mut results = segment_results.concat();
    util::truncate_to_bottom_n_sorted_by(&mut results, max_results, &|a, b| b.cmp(a));
    results
}
