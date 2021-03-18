use easy_parallel::Parallel;

use super::char_bag::CharBag;

use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::BinaryHeap,
};

const BASE_DISTANCE_PENALTY: f64 = 0.6;
const ADDITIONAL_DISTANCE_PENALTY: f64 = 0.05;
const MIN_DISTANCE_PENALTY: f64 = 0.2;

pub struct PathEntry {
    pub entry_id: usize,
    pub path_chars: CharBag,
    pub path: Vec<char>,
    pub lowercase_path: Vec<char>,
    pub is_ignored: bool,
}

#[derive(Clone, Debug)]
pub struct PathMatch {
    pub score: f64,
    pub positions: Vec<usize>,
    pub tree_id: usize,
    pub entry_id: usize,
    pub skipped_prefix_len: usize,
}

impl PartialEq for PathMatch {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl Eq for PathMatch {}

impl PartialOrd for PathMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for PathMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn match_paths(
    paths_by_tree_id: &[(usize, usize, &[PathEntry])],
    query: &str,
    include_ignored: bool,
    smart_case: bool,
    max_results: usize,
) -> Vec<PathMatch> {
    let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
    let query = query.chars().collect::<Vec<_>>();
    let lowercase_query = &lowercase_query;
    let query = &query;
    let query_chars = CharBag::from(&lowercase_query[..]);

    let cpus = num_cpus::get();
    let path_count = paths_by_tree_id
        .iter()
        .fold(0, |sum, (_, _, paths)| sum + paths.len());
    let segment_size = (path_count + cpus - 1) / cpus;
    let mut segment_results = (0..cpus).map(|_| BinaryHeap::new()).collect::<Vec<_>>();

    Parallel::new()
        .each(
            segment_results.iter_mut().enumerate(),
            |(segment_idx, results)| {
                let segment_start = segment_idx * segment_size;
                let segment_end = segment_start + segment_size;

                let mut min_score = 0.0;
                let mut last_positions = Vec::new();
                last_positions.resize(query.len(), 0);
                let mut match_positions = Vec::new();
                match_positions.resize(query.len(), 0);
                let mut score_matrix = Vec::new();
                let mut best_position_matrix = Vec::new();

                let mut tree_start = 0;
                for (tree_id, skipped_prefix_len, paths) in paths_by_tree_id {
                    let tree_end = tree_start + paths.len();
                    if tree_start < segment_end && segment_start < tree_end {
                        let start = max(tree_start, segment_start) - tree_start;
                        let end = min(tree_end, segment_end) - tree_start;

                        match_single_tree_paths(
                            *tree_id,
                            *skipped_prefix_len,
                            paths,
                            start,
                            end,
                            query,
                            lowercase_query,
                            query_chars,
                            include_ignored,
                            smart_case,
                            results,
                            max_results,
                            &mut min_score,
                            &mut match_positions,
                            &mut last_positions,
                            &mut score_matrix,
                            &mut best_position_matrix,
                        );
                    }
                    if tree_end >= segment_end {
                        break;
                    }
                    tree_start = tree_end;
                }
            },
        )
        .run();

    let mut results = segment_results
        .into_iter()
        .flatten()
        .map(|r| r.0)
        .collect::<Vec<_>>();
    results.sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(max_results);
    results
}

fn match_single_tree_paths(
    tree_id: usize,
    skipped_prefix_len: usize,
    path_entries: &[PathEntry],
    start: usize,
    end: usize,
    query: &[char],
    lowercase_query: &[char],
    query_chars: CharBag,
    include_ignored: bool,
    smart_case: bool,
    results: &mut BinaryHeap<Reverse<PathMatch>>,
    max_results: usize,
    min_score: &mut f64,
    match_positions: &mut Vec<usize>,
    last_positions: &mut Vec<usize>,
    score_matrix: &mut Vec<Option<f64>>,
    best_position_matrix: &mut Vec<usize>,
) {
    for i in start..end {
        let path_entry = unsafe { &path_entries.get_unchecked(i) };

        if !include_ignored && path_entry.is_ignored {
            continue;
        }

        if !path_entry.path_chars.is_superset(query_chars) {
            continue;
        }

        if !find_last_positions(
            last_positions,
            skipped_prefix_len,
            &path_entry.lowercase_path,
            &lowercase_query[..],
        ) {
            continue;
        }

        let matrix_len = query.len() * (path_entry.path.len() - skipped_prefix_len);
        score_matrix.clear();
        score_matrix.resize(matrix_len, None);
        best_position_matrix.clear();
        best_position_matrix.resize(matrix_len, skipped_prefix_len);

        let score = score_match(
            &query[..],
            &lowercase_query[..],
            &path_entry.path,
            &path_entry.lowercase_path,
            skipped_prefix_len,
            smart_case,
            &last_positions,
            score_matrix,
            best_position_matrix,
            match_positions,
            *min_score,
        );

        if score > 0.0 {
            results.push(Reverse(PathMatch {
                tree_id,
                entry_id: path_entry.entry_id,
                score,
                positions: match_positions.clone(),
                skipped_prefix_len,
            }));
            if results.len() == max_results {
                *min_score = results.peek().unwrap().0.score;
            }
        }
    }
}

fn find_last_positions(
    last_positions: &mut Vec<usize>,
    skipped_prefix_len: usize,
    path: &[char],
    query: &[char],
) -> bool {
    let mut path = path.iter();
    for (i, char) in query.iter().enumerate().rev() {
        if let Some(j) = path.rposition(|c| c == char) {
            if j >= skipped_prefix_len {
                last_positions[i] = j;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn score_match(
    query: &[char],
    query_cased: &[char],
    path: &[char],
    path_cased: &[char],
    skipped_prefix_len: usize,
    smart_case: bool,
    last_positions: &[usize],
    score_matrix: &mut [Option<f64>],
    best_position_matrix: &mut [usize],
    match_positions: &mut [usize],
    min_score: f64,
) -> f64 {
    let score = recursive_score_match(
        query,
        query_cased,
        path,
        path_cased,
        skipped_prefix_len,
        smart_case,
        last_positions,
        score_matrix,
        best_position_matrix,
        min_score,
        0,
        skipped_prefix_len,
        query.len() as f64,
    ) * query.len() as f64;

    if score <= 0.0 {
        return 0.0;
    }

    let path_len = path.len() - skipped_prefix_len;
    let mut cur_start = 0;
    for i in 0..query.len() {
        match_positions[i] = best_position_matrix[i * path_len + cur_start] - skipped_prefix_len;
        cur_start = match_positions[i] + 1;
    }

    score
}

fn recursive_score_match(
    query: &[char],
    query_cased: &[char],
    path: &[char],
    path_cased: &[char],
    skipped_prefix_len: usize,
    smart_case: bool,
    last_positions: &[usize],
    score_matrix: &mut [Option<f64>],
    best_position_matrix: &mut [usize],
    min_score: f64,
    query_idx: usize,
    path_idx: usize,
    cur_score: f64,
) -> f64 {
    if query_idx == query.len() {
        return 1.0;
    }

    let path_len = path.len() - skipped_prefix_len;

    if let Some(memoized) = score_matrix[query_idx * path_len + path_idx - skipped_prefix_len] {
        return memoized;
    }

    let mut score = 0.0;
    let mut best_position = 0;

    let query_char = query_cased[query_idx];
    let limit = last_positions[query_idx];

    let mut last_slash = 0;
    for j in path_idx..=limit {
        let path_char = path_cased[j];
        let is_path_sep = path_char == '/' || path_char == '\\';

        if query_idx == 0 && is_path_sep {
            last_slash = j;
        }

        if query_char == path_char || (is_path_sep && query_char == '_' || query_char == '\\') {
            let mut char_score = 1.0;
            if j > path_idx {
                let last = path[j - 1];
                let curr = path[j];

                if last == '/' {
                    char_score = 0.9;
                } else if last == '-' || last == '_' || last == ' ' || last.is_numeric() {
                    char_score = 0.8;
                } else if last.is_lowercase() && curr.is_uppercase() {
                    char_score = 0.8;
                } else if last == '.' {
                    char_score = 0.7;
                } else if query_idx == 0 {
                    char_score = BASE_DISTANCE_PENALTY;
                } else {
                    char_score = MIN_DISTANCE_PENALTY.max(
                        BASE_DISTANCE_PENALTY
                            - (j - path_idx - 1) as f64 * ADDITIONAL_DISTANCE_PENALTY,
                    );
                }
            }

            // Apply a severe penalty if the case doesn't match.
            // This will make the exact matches have higher score than the case-insensitive and the
            // path insensitive matches.
            if (smart_case || path[j] == '/') && query[query_idx] != path[j] {
                char_score *= 0.001;
            }

            let mut multiplier = char_score;

            // Scale the score based on how deep within the patch we found the match.
            if query_idx == 0 {
                multiplier /= (path.len() - last_slash) as f64;
            }

            let mut next_score = 1.0;
            if min_score > 0.0 {
                next_score = cur_score * multiplier;
                // Scores only decrease. If we can't pass the previous best, bail
                if next_score < min_score {
                    // Ensure that score is non-zero so we use it in the memo table.
                    if score == 0.0 {
                        score = 1e-18;
                    }
                    continue;
                }
            }

            let new_score = recursive_score_match(
                query,
                query_cased,
                path,
                path_cased,
                skipped_prefix_len,
                smart_case,
                last_positions,
                score_matrix,
                best_position_matrix,
                min_score,
                query_idx + 1,
                j + 1,
                next_score,
            ) * multiplier;

            if new_score > score {
                score = new_score;
                best_position = j;
                // Optimization: can't score better than 1.
                if new_score == 1.0 {
                    break;
                }
            }
        }
    }

    if best_position != 0 {
        best_position_matrix[query_idx * path_len + path_idx - skipped_prefix_len] = best_position;
    }

    score_matrix[query_idx * path_len + path_idx - skipped_prefix_len] = Some(score);
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_path_entries() {
        let paths = vec![
            "",
            "a",
            "ab",
            "abC",
            "abcd",
            "alphabravocharlie",
            "AlphaBravoCharlie",
            "thisisatestdir",
            "/////ThisIsATestDir",
            "/this/is/a/test/dir",
            "/test/tiatd",
        ];

        assert_eq!(
            match_query("abc", false, &paths),
            vec![
                ("abC", vec![0, 1, 2]),
                ("abcd", vec![0, 1, 2]),
                ("AlphaBravoCharlie", vec![0, 5, 10]),
                ("alphabravocharlie", vec![4, 5, 10]),
            ]
        );
        assert_eq!(
            match_query("t/i/a/t/d", false, &paths),
            vec![("/this/is/a/test/dir", vec![1, 5, 6, 8, 9, 10, 11, 15, 16]),]
        );

        assert_eq!(
            match_query("tiatd", false, &paths),
            vec![
                ("/test/tiatd", vec![6, 7, 8, 9, 10]),
                ("/this/is/a/test/dir", vec![1, 6, 9, 11, 16]),
                ("/////ThisIsATestDir", vec![5, 9, 11, 12, 16]),
                ("thisisatestdir", vec![0, 2, 6, 7, 11]),
            ]
        );
    }

    fn match_query<'a>(
        query: &str,
        smart_case: bool,
        paths: &Vec<&'a str>,
    ) -> Vec<(&'a str, Vec<usize>)> {
        let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
        let query = query.chars().collect::<Vec<_>>();
        let query_chars = CharBag::from(&lowercase_query[..]);

        let mut path_entries = Vec::new();
        for (i, path) in paths.iter().enumerate() {
            let lowercase_path = path.to_lowercase().chars().collect::<Vec<_>>();
            let path_chars = CharBag::from(&lowercase_path[..]);
            let path = path.chars().collect();
            path_entries.push(PathEntry {
                entry_id: i,
                path_chars,
                path,
                lowercase_path,
                is_ignored: false,
            });
        }

        let mut match_positions = Vec::new();
        let mut last_positions = Vec::new();
        match_positions.resize(query.len(), 0);
        last_positions.resize(query.len(), 0);

        let mut results = BinaryHeap::new();
        match_single_tree_paths(
            0,
            0,
            &path_entries,
            0,
            path_entries.len(),
            &query[..],
            &lowercase_query[..],
            query_chars,
            true,
            smart_case,
            &mut results,
            100,
            &mut 0.0,
            &mut match_positions,
            &mut last_positions,
            &mut Vec::new(),
            &mut Vec::new(),
        );

        results
            .into_iter()
            .rev()
            .map(|result| (paths[result.0.entry_id].clone(), result.0.positions))
            .collect()
    }
}
