use std::cmp::Reverse;

use collections::HashSet;
use fuzzy_nucleo::StringMatchCandidate;
use gpui::BackgroundExecutor;
use ordered_float::OrderedFloat;

pub(crate) async fn search<T, BuildCandidates>(
    values: Vec<T>,
    query: &str,
    executor: BackgroundExecutor,
    build_candidates: BuildCandidates,
) -> Vec<T>
where
    T: Clone + Send + 'static,
    BuildCandidates: Fn(usize, &T) -> (StringMatchCandidate, StringMatchCandidate) + Send + 'static,
{
    let query = query.to_owned();
    // Preparing and cloning candidates can block input even though matching is already asynchronous.
    executor
        .clone()
        .spawn(async move {
            let (names, candidates): (Vec<_>, Vec<_>) = values
                .iter()
                .enumerate()
                .map(|(index, value)| build_candidates(index, value))
                .unzip();

            match_names_with_metadata(&names, &candidates, &query, executor)
                .await
                .into_iter()
                .map(|index| values[index].clone())
                .collect()
        })
        .await
}

async fn match_names_with_metadata(
    names: &[StringMatchCandidate],
    candidates: &[StringMatchCandidate],
    query: &str,
    executor: BackgroundExecutor,
) -> Vec<usize> {
    let cancel_flag = Default::default();
    let (mut name_matches, mut metadata_matches) = futures::join!(
        fuzzy_nucleo::match_strings_async(
            names,
            query,
            fuzzy_nucleo::Case::Ignore,
            fuzzy_nucleo::LengthPenalty::On,
            names.len(),
            &cancel_flag,
            executor.clone(),
        ),
        fuzzy_nucleo::match_strings_async(
            candidates,
            query,
            fuzzy_nucleo::Case::Ignore,
            fuzzy_nucleo::LengthPenalty::On,
            candidates.len(),
            &cancel_flag,
            executor,
        ),
    );

    // Metadata should expand the results without changing the relevance of name matches.
    let sorting_key = |matched: &fuzzy_nucleo::StringMatch| {
        (Reverse(OrderedFloat(matched.score)), matched.candidate_id)
    };
    name_matches.sort_unstable_by_key(sorting_key);
    metadata_matches.sort_unstable_by_key(sorting_key);

    let matched_name_ids: HashSet<_> = name_matches
        .iter()
        .map(|matched| matched.candidate_id)
        .collect();
    name_matches
        .into_iter()
        .chain(
            metadata_matches
                .into_iter()
                .filter(|matched| !matched_name_ids.contains(&matched.candidate_id)),
        )
        .map(|matched| matched.candidate_id)
        .collect()
}
