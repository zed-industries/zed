use gpui::scoped_pool;

use crate::sum_tree::SeekBias;

use super::{char_bag::CharBag, Entry, FileCount, Snapshot};

use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::BinaryHeap,
    path::Path,
    sync::Arc,
};

const BASE_DISTANCE_PENALTY: f64 = 0.6;
const ADDITIONAL_DISTANCE_PENALTY: f64 = 0.05;
const MIN_DISTANCE_PENALTY: f64 = 0.2;

#[derive(Clone, Debug)]
pub struct PathEntry {
    pub ino: u64,
    pub path_chars: CharBag,
    pub path: Arc<[char]>,
    pub lowercase_path: Arc<[char]>,
}

impl PathEntry {
    pub fn new(ino: u64, path: &Path) -> Self {
        let path = path.to_string_lossy();
        let lowercase_path = path.to_lowercase().chars().collect::<Vec<_>>().into();
        let path: Arc<[char]> = path.chars().collect::<Vec<_>>().into();
        let path_chars = CharBag::from(path.as_ref());

        Self {
            ino,
            path_chars,
            path,
            lowercase_path,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathMatch {
    pub score: f64,
    pub positions: Vec<usize>,
    pub path: String,
    pub tree_id: usize,
    pub entry_id: u64,
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

pub fn match_paths<'a, T>(
    snapshots: T,
    query: &str,
    include_root_name: bool,
    include_ignored: bool,
    smart_case: bool,
    max_results: usize,
    pool: scoped_pool::Pool,
) -> Vec<PathMatch>
where
    T: Clone + Send + Iterator<Item = &'a Snapshot>,
{
    let lowercase_query = query.to_lowercase().chars().collect::<Vec<_>>();
    let query = query.chars().collect::<Vec<_>>();

    let lowercase_query = &lowercase_query;
    let query = &query;
    let query_chars = CharBag::from(&lowercase_query[..]);

    let cpus = num_cpus::get();
    let path_count: usize = snapshots.clone().map(Snapshot::file_count).sum();
    let segment_size = (path_count + cpus - 1) / cpus;
    let mut segment_results = (0..cpus).map(|_| BinaryHeap::new()).collect::<Vec<_>>();

    pool.scoped(|scope| {
        for (segment_idx, results) in segment_results.iter_mut().enumerate() {
            let trees = snapshots.clone();
            scope.execute(move || {
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
                for snapshot in trees {
                    let tree_end = tree_start + snapshot.file_count();
                    if tree_start < segment_end && segment_start < tree_end {
                        let start = max(tree_start, segment_start) - tree_start;
                        let end = min(tree_end, segment_end) - tree_start;
                        let mut cursor = snapshot.entries.cursor::<_, ()>();
                        cursor.seek(&FileCount(start), SeekBias::Right);
                        let path_entries = cursor
                            .filter_map(|e| {
                                if let Entry::File { path, .. } = e {
                                    Some(path)
                                } else {
                                    None
                                }
                            })
                            .take(end - start);

                        let skipped_prefix_len = if include_root_name {
                            0
                        } else if let Some(Entry::Dir { .. }) = snapshot.root_entry() {
                            if let Some(name) = snapshot.root_name() {
                                name.to_string_lossy().chars().count() + 1
                            } else {
                                1
                            }
                        } else {
                            0
                        };

                        match_single_tree_paths(
                            snapshot,
                            skipped_prefix_len,
                            path_entries,
                            query,
                            lowercase_query,
                            query_chars.clone(),
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
            })
        }
    });

    let mut results = segment_results
        .into_iter()
        .flatten()
        .map(|r| r.0)
        .collect::<Vec<_>>();
    results.sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results.truncate(max_results);
    results
}

fn match_single_tree_paths<'a>(
    snapshot: &Snapshot,
    skipped_prefix_len: usize,
    path_entries: impl Iterator<Item = &'a PathEntry>,
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
    for path_entry in path_entries {
        if !path_entry.path_chars.is_superset(query_chars.clone()) {
            continue;
        }

        if !include_ignored && snapshot.is_inode_ignored(path_entry.ino).unwrap_or(true) {
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
                tree_id: snapshot.id,
                entry_id: path_entry.ino,
                path: path_entry.path.iter().skip(skipped_prefix_len).collect(),
                score,
                positions: match_positions.clone(),
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
    use std::path::PathBuf;

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
            let lowercase_path: Arc<[char]> =
                path.to_lowercase().chars().collect::<Vec<_>>().into();
            let path_chars = CharBag::from(lowercase_path.as_ref());
            let path = path.chars().collect();
            path_entries.push(PathEntry {
                ino: i as u64,
                path_chars,
                path,
                lowercase_path,
            });
        }

        let mut match_positions = Vec::new();
        let mut last_positions = Vec::new();
        match_positions.resize(query.len(), 0);
        last_positions.resize(query.len(), 0);

        let mut results = BinaryHeap::new();
        match_single_tree_paths(
            &Snapshot {
                id: 0,
                path: PathBuf::new().into(),
                root_inode: None,
                ignores: Default::default(),
                entries: Default::default(),
            },
            0,
            path_entries.iter(),
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
            .map(|result| {
                (
                    paths[result.0.entry_id as usize].clone(),
                    result.0.positions,
                )
            })
            .collect()
    }
}
