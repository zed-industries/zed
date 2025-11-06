use crate::{CharBag, matcher};
use gpui::BackgroundExecutor;
use nucleo::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use std::{
    borrow::Borrow,
    cmp, iter,
    ops::Range,
    sync::atomic::{AtomicBool, Ordering},
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
        // dbg!(&self.string, self.score);
        // dbg!(&other.string, other.score);
        self.score
            .total_cmp(&other.score)
            .reverse()
            .then_with(|| self.string.cmp(&other.string))
    }
}

pub async fn match_strings<T>(
    candidates: &[T],
    query: &str,
    smart_case: bool,
    prefer_shorter: bool,
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
    // FIXME should support fzf syntax with Pattern::parse
    let pattern = Pattern::new(
        query,
        if smart_case {
            CaseMatching::Smart
        } else {
            CaseMatching::Ignore
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

    let mut config = nucleo::Config::DEFAULT;
    config.prefer_prefix = true; // TODO: consider making this a setting
    let mut matchers = matcher::get_matchers(num_cpus, config);

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
                            let length_modifier = candidate.string.chars().count() as f64 / 10_000.;
                            results.push(StringMatch {
                                candidate_id: candidate.id,
                                score: score as f64
                                    + if prefer_shorter {
                                        -length_modifier
                                    } else {
                                        length_modifier
                                    },

                                // TODO: need to convert indices/positions from char offsets to byte offsets.
                                positions: indices.into_iter().map(|n| n as usize).collect(),
                                string: candidate.string.clone(),
                            })
                        };
                    }
                });
            }
        })
        .await;

    matcher::return_matchers(matchers);

    if cancel_flag.load(Ordering::Relaxed) {
        return Vec::new();
    }

    let mut results = segment_results.concat();
    util::truncate_to_bottom_n_sorted(&mut results, max_results);
    for r in &mut results {
        r.positions.sort();
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    use gpui::TestAppContext;

    async fn get_matches(
        cx: &mut TestAppContext,
        candidates: &[&'static str],
        query: &'static str,
        penalize_length: bool,
    ) -> Vec<StringMatch> {
        let candidates: Vec<_> = candidates
            .iter()
            .enumerate()
            .map(|(i, s)| StringMatchCandidate::new(i, s))
            .collect();

        let cancellation_flag = AtomicBool::new(false);
        let executor = cx.background_executor.clone();
        cx.foreground_executor
            .spawn(async move {
                super::match_strings(
                    &candidates,
                    query,
                    true,
                    penalize_length,
                    100,
                    &cancellation_flag,
                    executor,
                )
                .await
            })
            .await
    }

    async fn string_matches(
        cx: &mut TestAppContext,
        candidates: &[&'static str],
        query: &'static str,
        penalize_length: bool,
    ) -> Vec<String> {
        let matches = get_matches(cx, candidates, query, penalize_length).await;
        matches
            .iter()
            .map(|sm| dbg!(sm).string.clone())
            .collect::<Vec<_>>()
    }

    async fn match_positions(
        cx: &mut TestAppContext,
        candidates: &[&'static str],
        query: &'static str,
        penalize_length: bool,
    ) -> Vec<usize> {
        let mut matches = get_matches(cx, candidates, query, penalize_length).await;
        matches.remove(0).positions
    }

    #[gpui::test]
    async fn prefer_shorter_matches(cx: &mut TestAppContext) {
        let candidates = &["a", "aa", "aaa"];
        assert_eq!(
            string_matches(cx, candidates, "a", true).await,
            ["a", "aa", "aaa"]
        );
    }

    #[gpui::test]
    async fn prefer_longer_matches(cx: &mut TestAppContext) {
        let candidates = &["unreachable", "unreachable!()"];
        assert_eq!(
            string_matches(cx, candidates, "unreac", false).await,
            ["unreachable!()", "unreachable",]
        );
    }

    #[gpui::test]
    async fn shorter_over_lexicographical(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &["qr", "qqqqqqqqqqqq"];
        assert_eq!(
            string_matches(cx, CANDIDATES, "q", true).await,
            ["qr", "qqqqqqqqqqqq"]
        );
    }

    #[gpui::test]
    async fn indices_are_sorted_and_correct(cx: &mut TestAppContext) {
        const CANDIDATES: &'static [&'static str] = &["hello how are you"];
        assert_eq!(
            match_positions(cx, CANDIDATES, "you hello", true).await,
            vec![0, 1, 2, 3, 4, 14, 15, 16]
        );

        // const CANDIDATES: &'static [&'static str] =
        //     &["crates/livekit_api/vendored/protocol/README.md"];
    }

    // This is broken?
    #[gpui::test]
    async fn broken_nucleo_matcher(cx: &mut TestAppContext) {
        let candidates = &["lsp_code_lens", "code_lens"];
        assert_eq!(
            string_matches(cx, candidates, "lens", false).await,
            ["code_lens", "lsp_code_lens",]
        );
    }
}
