use std::{
    borrow::Borrow,
    collections::BTreeMap,
    sync::atomic::{self, AtomicBool},
};

use crate::CharBag;

const BASE_DISTANCE_PENALTY: f64 = 0.6;
const ADDITIONAL_DISTANCE_PENALTY: f64 = 0.05;
const MIN_DISTANCE_PENALTY: f64 = 0.2;

// TODO:
// Use `Path` instead of `&str` for paths.
pub struct Matcher<'a> {
    query: &'a [char],
    lowercase_query: &'a [char],
    query_char_bag: CharBag,
    smart_case: bool,
    penalize_length: bool,
    min_score: f64,
    match_positions: Vec<usize>,
    last_positions: Vec<usize>,
    score_matrix: Vec<Option<f64>>,
    best_position_matrix: Vec<usize>,
}

pub trait MatchCandidate {
    fn has_chars(&self, bag: CharBag) -> bool;
    fn candidate_chars(&self) -> impl Iterator<Item = char>;
}

impl<'a> Matcher<'a> {
    pub fn new(
        query: &'a [char],
        lowercase_query: &'a [char],
        query_char_bag: CharBag,
        smart_case: bool,
        penalize_length: bool,
    ) -> Self {
        Self {
            query,
            lowercase_query,
            query_char_bag,
            min_score: 0.0,
            last_positions: vec![0; lowercase_query.len()],
            match_positions: vec![0; query.len()],
            score_matrix: Vec::new(),
            best_position_matrix: Vec::new(),
            smart_case,
            penalize_length,
        }
    }

    /// Filter and score fuzzy match candidates. Results are returned unsorted, in the same order as
    /// the input candidates.
    pub(crate) fn match_candidates<C, R, F, T>(
        &mut self,
        prefix: &[char],
        lowercase_prefix: &[char],
        candidates: impl Iterator<Item = T>,
        results: &mut Vec<R>,
        cancel_flag: &AtomicBool,
        build_match: F,
    ) where
        C: MatchCandidate,
        T: Borrow<C>,
        F: Fn(&C, f64, &Vec<usize>) -> R,
    {
        self.match_candidates_with_words(
            prefix,
            lowercase_prefix,
            candidates,
            results,
            cancel_flag,
            build_match,
            &[],
        )
    }

    /// Filter and score fuzzy match candidates with optional word-based matching
    pub(crate) fn match_candidates_with_words<C, R, F, T>(
        &mut self,
        prefix: &[char],
        lowercase_prefix: &[char],
        candidates: impl Iterator<Item = T>,
        results: &mut Vec<R>,
        cancel_flag: &AtomicBool,
        build_match: F,
        query_words: &[&str],
    ) where
        C: MatchCandidate,
        T: Borrow<C>,
        F: Fn(&C, f64, &Vec<usize>) -> R,
    {
        let mut candidate_chars = Vec::new();
        let mut lowercase_candidate_chars = Vec::new();
        let mut extra_lowercase_chars = BTreeMap::new();

        for candidate in candidates {
            if !candidate.borrow().has_chars(self.query_char_bag) {
                continue;
            }

            if cancel_flag.load(atomic::Ordering::Acquire) {
                break;
            }

            candidate_chars.clear();
            lowercase_candidate_chars.clear();
            extra_lowercase_chars.clear();
            for (i, c) in candidate.borrow().candidate_chars().enumerate() {
                candidate_chars.push(c);
                let mut char_lowercased = c.to_lowercase().collect::<Vec<_>>();
                if char_lowercased.len() > 1 {
                    extra_lowercase_chars.insert(i, char_lowercased.len() - 1);
                }
                lowercase_candidate_chars.append(&mut char_lowercased);
            }

            // If we have word-based query, check if all words exist in the path
            let use_word_matching = !query_words.is_empty();
            if use_word_matching {
                if !Self::check_words_match(
                    query_words,
                    lowercase_prefix,
                    &lowercase_candidate_chars,
                ) {
                    continue;
                }
            } else {
                // Use the original sequential matching for non-word queries
                if !self.find_last_positions(lowercase_prefix, &lowercase_candidate_chars) {
                    continue;
                }
            }

            let score = if use_word_matching {
                // Score word-based matches
                self.score_word_match(
                    query_words,
                    &candidate_chars,
                    &lowercase_candidate_chars,
                    prefix,
                    lowercase_prefix,
                )
            } else {
                // Use original scoring for sequential matches
                let matrix_len = self.query.len() * (prefix.len() + candidate_chars.len());
                self.score_matrix.clear();
                self.score_matrix.resize(matrix_len, None);
                self.best_position_matrix.clear();
                self.best_position_matrix.resize(matrix_len, 0);

                self.score_match(
                    &candidate_chars,
                    &lowercase_candidate_chars,
                    prefix,
                    lowercase_prefix,
                    &extra_lowercase_chars,
                )
            };

            if score > 0.0 {
                results.push(build_match(
                    candidate.borrow(),
                    score,
                    &self.match_positions,
                ));
            }
        }
    }

    fn find_last_positions(
        &mut self,
        lowercase_prefix: &[char],
        lowercase_candidate: &[char],
    ) -> bool {
        let mut lowercase_prefix = lowercase_prefix.iter();
        let mut lowercase_candidate = lowercase_candidate.iter();
        for (i, char) in self.lowercase_query.iter().enumerate().rev() {
            if let Some(j) = lowercase_candidate.rposition(|c| c == char) {
                self.last_positions[i] = j + lowercase_prefix.len();
            } else if let Some(j) = lowercase_prefix.rposition(|c| c == char) {
                self.last_positions[i] = j;
            } else {
                return false;
            }
        }
        true
    }

    /// Check if all words in the query can be found in the candidate path
    /// This is used for word-based matching when the query contains spaces
    fn check_words_match(
        query_words: &[&str],
        lowercase_prefix: &[char],
        lowercase_candidate: &[char],
    ) -> bool {
        // Combine prefix and candidate into a single string for matching
        let mut full_path = String::new();
        for c in lowercase_prefix.iter() {
            full_path.push(*c);
        }
        for c in lowercase_candidate.iter() {
            full_path.push(*c);
        }
        let full_path_lower = full_path.to_lowercase();

        // Check if all words can be found in the path
        for word in query_words {
            let word_lower = word.to_lowercase();
            if !full_path_lower.contains(&word_lower) {
                return false;
            }
        }
        true
    }

    /// Score a match based on word matching
    fn score_word_match(
        &mut self,
        query_words: &[&str],
        path: &[char],
        path_lowercased: &[char],
        prefix: &[char],
        lowercase_prefix: &[char],
    ) -> f64 {
        // Combine prefix and path for word matching
        let mut full_path = String::new();
        for c in prefix.iter() {
            full_path.push(*c);
        }
        for c in path.iter() {
            full_path.push(*c);
        }

        let mut full_path_lower = String::new();
        for c in lowercase_prefix.iter() {
            full_path_lower.push(*c);
        }
        for c in path_lowercased.iter() {
            full_path_lower.push(*c);
        }

        // Calculate score based on how well words match
        let mut total_score = 0.0;
        let mut all_positions = Vec::new();

        for word in query_words {
            let word_lower = word.to_lowercase();
            if let Some(pos) = full_path_lower.find(&word_lower) {
                // Base score for finding the word
                let mut word_score = 1.0;

                // Bonus if word matches at path boundaries
                if pos == 0 || full_path.chars().nth(pos.saturating_sub(1)) == Some('/') {
                    word_score *= 1.5;
                }

                // Bonus if word matches exactly (case-sensitive)
                let word_at_pos = &full_path[pos..pos + word.len()];
                if word_at_pos == *word {
                    word_score *= 1.2;
                }

                // Track positions for all matched words
                for i in 0..word.len() {
                    all_positions.push(pos + i);
                }

                total_score += word_score;
            }
        }

        // Normalize score based on number of words and path length
        if !query_words.is_empty() {
            total_score = total_score / query_words.len() as f64;

            // Penalize longer paths slightly
            let length_penalty = 1.0 / (1.0 + (full_path.len() as f64 * 0.01));
            total_score *= length_penalty;
        }

        // Update match positions for highlighting
        all_positions.sort();
        all_positions.dedup();
        self.match_positions.clear();
        for &pos in all_positions.iter().take(self.query.len()) {
            self.match_positions.push(pos);
        }
        // Fill remaining positions if needed
        while self.match_positions.len() < self.query.len() {
            self.match_positions.push(0);
        }

        total_score
    }

    fn score_match(
        &mut self,
        path: &[char],
        path_lowercased: &[char],
        prefix: &[char],
        lowercase_prefix: &[char],
        extra_lowercase_chars: &BTreeMap<usize, usize>,
    ) -> f64 {
        let score = self.recursive_score_match(
            path,
            path_lowercased,
            prefix,
            lowercase_prefix,
            0,
            0,
            self.query.len() as f64,
            extra_lowercase_chars,
        ) * self.query.len() as f64;

        if score <= 0.0 {
            return 0.0;
        }
        let path_len = prefix.len() + path.len();
        let mut cur_start = 0;
        let mut byte_ix = 0;
        let mut char_ix = 0;
        for i in 0..self.query.len() {
            let match_char_ix = self.best_position_matrix[i * path_len + cur_start];
            while char_ix < match_char_ix {
                let ch = prefix
                    .get(char_ix)
                    .or_else(|| path.get(char_ix - prefix.len()))
                    .unwrap();
                byte_ix += ch.len_utf8();
                char_ix += 1;
            }

            self.match_positions[i] = byte_ix;

            let matched_ch = prefix
                .get(match_char_ix)
                .or_else(|| path.get(match_char_ix - prefix.len()))
                .unwrap();
            byte_ix += matched_ch.len_utf8();

            cur_start = match_char_ix + 1;
            char_ix = match_char_ix + 1;
        }

        score
    }

    fn recursive_score_match(
        &mut self,
        path: &[char],
        path_lowercased: &[char],
        prefix: &[char],
        lowercase_prefix: &[char],
        query_idx: usize,
        path_idx: usize,
        cur_score: f64,
        extra_lowercase_chars: &BTreeMap<usize, usize>,
    ) -> f64 {
        if query_idx == self.query.len() {
            return 1.0;
        }

        let limit = self.last_positions[query_idx];
        let max_valid_index = (prefix.len() + path_lowercased.len()).saturating_sub(1);
        let safe_limit = limit.min(max_valid_index);

        if path_idx > safe_limit {
            return 0.0;
        }

        let path_len = prefix.len() + path.len();
        if let Some(memoized) = self.score_matrix[query_idx * path_len + path_idx] {
            return memoized;
        }

        let mut score = 0.0;
        let mut best_position = 0;

        let query_char = self.lowercase_query[query_idx];

        let mut last_slash = 0;

        for j in path_idx..=safe_limit {
            let extra_lowercase_chars_count = extra_lowercase_chars
                .iter()
                .take_while(|&(&i, _)| i < j)
                .map(|(_, increment)| increment)
                .sum::<usize>();
            let j_regular = j - extra_lowercase_chars_count;

            let path_char = if j < prefix.len() {
                lowercase_prefix[j]
            } else {
                let path_index = j - prefix.len();
                match path_lowercased.get(path_index) {
                    Some(&char) => char,
                    None => continue,
                }
            };
            let is_path_sep = path_char == '/';

            if query_idx == 0 && is_path_sep {
                last_slash = j_regular;
            }
            let need_to_score = query_char == path_char || (is_path_sep && query_char == '_');
            if need_to_score {
                let curr = match prefix.get(j_regular) {
                    Some(&curr) => curr,
                    None => path[j_regular - prefix.len()],
                };

                let mut char_score = 1.0;
                if j > path_idx {
                    let last = match prefix.get(j_regular - 1) {
                        Some(&last) => last,
                        None => path[j_regular - 1 - prefix.len()],
                    };

                    if last == '/' {
                        char_score = 0.9;
                    } else if (last == '-' || last == '_' || last == ' ' || last.is_numeric())
                        || (last.is_lowercase() && curr.is_uppercase())
                    {
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
                if (self.smart_case || curr == '/') && self.query[query_idx] != curr {
                    char_score *= 0.001;
                }

                let mut multiplier = char_score;

                // Scale the score based on how deep within the path we found the match.
                if self.penalize_length && query_idx == 0 {
                    multiplier /= ((prefix.len() + path.len()) - last_slash) as f64;
                }

                let mut next_score = 1.0;
                if self.min_score > 0.0 {
                    next_score = cur_score * multiplier;
                    // Scores only decrease. If we can't pass the previous best, bail
                    if next_score < self.min_score {
                        // Ensure that score is non-zero so we use it in the memo table.
                        if score == 0.0 {
                            score = 1e-18;
                        }
                        continue;
                    }
                }

                let new_score = self.recursive_score_match(
                    path,
                    path_lowercased,
                    prefix,
                    lowercase_prefix,
                    query_idx + 1,
                    j + 1,
                    next_score,
                    extra_lowercase_chars,
                ) * multiplier;

                if new_score > score {
                    score = new_score;
                    best_position = j_regular;
                    // Optimization: can't score better than 1.
                    if new_score == 1.0 {
                        break;
                    }
                }
            }
        }

        if best_position != 0 {
            self.best_position_matrix[query_idx * path_len + path_idx] = best_position;
        }

        self.score_matrix[query_idx * path_len + path_idx] = Some(score);
        score
    }
}

#[cfg(test)]
mod tests {
    use util::rel_path::{RelPath, rel_path};

    use crate::{PathMatch, PathMatchCandidate};

    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_get_last_positions() {
        let mut query: &[char] = &['d', 'c'];
        let mut matcher = Matcher::new(query, query, query.into(), false, true);
        let result = matcher.find_last_positions(&['a', 'b', 'c'], &['b', 'd', 'e', 'f']);
        assert!(!result);

        query = &['c', 'd'];
        let mut matcher = Matcher::new(query, query, query.into(), false, true);
        let result = matcher.find_last_positions(&['a', 'b', 'c'], &['b', 'd', 'e', 'f']);
        assert!(result);
        assert_eq!(matcher.last_positions, vec![2, 4]);

        query = &['z', '/', 'z', 'f'];
        let mut matcher = Matcher::new(query, query, query.into(), false, true);
        let result = matcher.find_last_positions(&['z', 'e', 'd', '/'], &['z', 'e', 'd', '/', 'f']);
        assert!(result);
        assert_eq!(matcher.last_positions, vec![0, 3, 4, 8]);
    }

    #[test]
    fn test_word_based_matching() {
        // Test that word-based matching works regardless of word order
        let paths = vec![
            "manager/page.tsx",
            "page/manager.tsx",
            "apps/web/manager/page.tsx",
            "apps/web/page/manager.tsx",
            "controller/user.rs",
            "user/controller.rs",
        ];

        // Test "page manager" should find paths with both words
        let results = match_single_path_query("page manager", false, &paths);
        assert!(results.iter().any(|(path, _)| *path == "manager/page.tsx"));
        assert!(results.iter().any(|(path, _)| *path == "page/manager.tsx"));
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "apps/web/manager/page.tsx")
        );
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "apps/web/page/manager.tsx")
        );

        // Test "manager page" should also find the same paths
        let results = match_single_path_query("manager page", false, &paths);
        assert!(results.iter().any(|(path, _)| *path == "manager/page.tsx"));
        assert!(results.iter().any(|(path, _)| *path == "page/manager.tsx"));
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "apps/web/manager/page.tsx")
        );
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "apps/web/page/manager.tsx")
        );

        // Test "user controller" should find user/controller paths
        let results = match_single_path_query("user controller", false, &paths);
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "controller/user.rs")
        );
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "user/controller.rs")
        );

        // Test "controller user" should also find the same paths
        let results = match_single_path_query("controller user", false, &paths);
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "controller/user.rs")
        );
        assert!(
            results
                .iter()
                .any(|(path, _)| *path == "user/controller.rs")
        );
    }

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
            "ThisIsATestDir",
            "this/is/a/test/dir",
            "test/tiatd",
        ];

        assert_eq!(
            match_single_path_query("abc", false, &paths),
            vec![
                ("abC", vec![0, 1, 2]),
                ("abcd", vec![0, 1, 2]),
                ("AlphaBravoCharlie", vec![0, 5, 10]),
                ("alphabravocharlie", vec![4, 5, 10]),
            ]
        );
        assert_eq!(
            match_single_path_query("t/i/a/t/d", false, &paths),
            vec![("this/is/a/test/dir", vec![0, 4, 5, 7, 8, 9, 10, 14, 15]),]
        );

        assert_eq!(
            match_single_path_query("tiatd", false, &paths),
            vec![
                ("test/tiatd", vec![5, 6, 7, 8, 9]),
                ("ThisIsATestDir", vec![0, 4, 6, 7, 11]),
                ("this/is/a/test/dir", vec![0, 5, 8, 10, 15]),
                ("thisisatestdir", vec![0, 2, 6, 7, 11]),
            ]
        );
    }

    #[test]
    fn test_lowercase_longer_than_uppercase() {
        // This character has more chars in lower-case than in upper-case.
        let paths = vec!["\u{0130}"];
        let query = "\u{0130}";
        assert_eq!(
            match_single_path_query(query, false, &paths),
            vec![("\u{0130}", vec![0])]
        );

        // Path is the lower-case version of the query
        let paths = vec!["i\u{307}"];
        let query = "\u{0130}";
        assert_eq!(
            match_single_path_query(query, false, &paths),
            vec![("i\u{307}", vec![0])]
        );
    }

    #[test]
    fn test_match_multibyte_path_entries() {
        let paths = vec![
            "aαbβ/cγdδ",
            "αβγδ/bcde",
            "c1️⃣2️⃣3️⃣/d4️⃣5️⃣6️⃣/e7️⃣8️⃣9️⃣/f",
            "d/🆒/h",
        ];
        assert_eq!("1️⃣".len(), 7);
        assert_eq!(
            match_single_path_query("bcd", false, &paths),
            vec![
                ("αβγδ/bcde", vec![9, 10, 11]),
                ("aαbβ/cγdδ", vec![3, 7, 10]),
            ]
        );
        assert_eq!(
            match_single_path_query("cde", false, &paths),
            vec![
                ("αβγδ/bcde", vec![10, 11, 12]),
                ("c1️⃣2️⃣3️⃣/d4️⃣5️⃣6️⃣/e7️⃣8️⃣9️⃣/f", vec![0, 23, 46]),
            ]
        );
    }

    #[test]
    fn match_unicode_path_entries() {
        let mixed_unicode_paths = vec![
            "İolu/oluş",
            "İstanbul/code",
            "Athens/Şanlıurfa",
            "Çanakkale/scripts",
            "paris/Düzce_İl",
            "Berlin_Önemli_Ğündem",
            "KİTAPLIK/london/dosya",
            "tokyo/kyoto/fuji",
            "new_york/san_francisco",
        ];

        assert_eq!(
            match_single_path_query("İo/oluş", false, &mixed_unicode_paths),
            vec![("İolu/oluş", vec![0, 2, 4, 6, 8, 10, 12])]
        );

        assert_eq!(
            match_single_path_query("İst/code", false, &mixed_unicode_paths),
            vec![("İstanbul/code", vec![0, 2, 4, 6, 8, 10, 12, 14])]
        );

        assert_eq!(
            match_single_path_query("athens/şa", false, &mixed_unicode_paths),
            vec![("Athens/Şanlıurfa", vec![0, 1, 2, 3, 4, 5, 6, 7, 9])]
        );

        assert_eq!(
            match_single_path_query("BerlinÖĞ", false, &mixed_unicode_paths),
            vec![("Berlin_Önemli_Ğündem", vec![0, 1, 2, 3, 4, 5, 7, 15])]
        );

        assert_eq!(
            match_single_path_query("tokyo/fuji", false, &mixed_unicode_paths),
            vec![("tokyo/kyoto/fuji", vec![0, 1, 2, 3, 4, 5, 12, 13, 14, 15])]
        );

        let mixed_script_paths = vec![
            "résumé_Москва",
            "naïve_київ_implementation",
            "café_北京_app",
            "東京_über_driver",
            "déjà_vu_cairo",
            "seoul_piñata_game",
            "voilà_istanbul_result",
        ];

        assert_eq!(
            match_single_path_query("résmé", false, &mixed_script_paths),
            vec![("résumé_Москва", vec![0, 1, 3, 5, 6])]
        );

        assert_eq!(
            match_single_path_query("café北京", false, &mixed_script_paths),
            vec![("café_北京_app", vec![0, 1, 2, 3, 6, 9])]
        );

        assert_eq!(
            match_single_path_query("ista", false, &mixed_script_paths),
            vec![("voilà_istanbul_result", vec![7, 8, 9, 10])]
        );

        let complex_paths = vec![
            "document_📚_library",
            "project_👨‍👩‍👧‍👦_family",
            "flags_🇯🇵🇺🇸🇪🇺_world",
            "code_😀😃😄😁_happy",
            "photo_👩‍👩‍👧‍👦_album",
        ];

        assert_eq!(
            match_single_path_query("doc📚lib", false, &complex_paths),
            vec![("document_📚_library", vec![0, 1, 2, 9, 14, 15, 16])]
        );

        assert_eq!(
            match_single_path_query("codehappy", false, &complex_paths),
            vec![("code_😀😃😄😁_happy", vec![0, 1, 2, 3, 22, 23, 24, 25, 26])]
        );
    }

    fn match_single_path_query<'a>(
        query: &str,
        smart_case: bool,
        paths: &[&'a str],
    ) -> Vec<(&'a str, Vec<usize>)> {
        // Check if query contains spaces (word-based search)
        let has_spaces = query.contains(' ');
        let query_words: Vec<&str> = if has_spaces {
            query.split_whitespace().collect()
        } else {
            vec![]
        };

        let query_for_processing = if has_spaces {
            query
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
        } else {
            query.to_string()
        };

        let lowercase_query = query_for_processing
            .to_lowercase()
            .chars()
            .collect::<Vec<_>>();
        let query = query_for_processing.chars().collect::<Vec<_>>();
        let query_chars = CharBag::from(&lowercase_query[..]);

        let path_arcs: Vec<Arc<RelPath>> = paths
            .iter()
            .map(|path| Arc::from(rel_path(path)))
            .collect::<Vec<_>>();
        let mut path_entries = Vec::new();
        for (i, path) in paths.iter().enumerate() {
            let lowercase_path = path.to_lowercase().chars().collect::<Vec<_>>();
            let char_bag = CharBag::from(lowercase_path.as_slice());
            path_entries.push(PathMatchCandidate {
                is_dir: false,
                char_bag,
                path: &path_arcs[i],
            });
        }

        let mut matcher = Matcher::new(&query, &lowercase_query, query_chars, smart_case, true);

        let cancel_flag = AtomicBool::new(false);
        let mut results = Vec::new();

        matcher.match_candidates_with_words(
            &[],
            &[],
            path_entries.into_iter(),
            &mut results,
            &cancel_flag,
            |candidate, score, positions| PathMatch {
                score,
                worktree_id: 0,
                positions: positions.clone(),
                path: candidate.path.into(),
                path_prefix: RelPath::empty().into(),
                distance_to_relative_ancestor: usize::MAX,
                is_dir: false,
            },
            &query_words,
        );
        results.sort_by(|a, b| b.cmp(a));

        results
            .into_iter()
            .map(|result| {
                (
                    paths
                        .iter()
                        .copied()
                        .find(|p| result.path.as_ref() == rel_path(p))
                        .unwrap(),
                    result.positions,
                )
            })
            .collect()
    }
}
