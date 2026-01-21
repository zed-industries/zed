enum ChrfWhitespace {
    #[allow(unused)]
    Unchanged,
    Ignore,
}

const CHR_F_CHAR_ORDER: usize = 6;
const CHR_F_BETA: f64 = 2.0;
const CHR_F_WHITESPACE: ChrfWhitespace = ChrfWhitespace::Ignore;

/// Computes a delta-chrF score that compares two sets of edits.
///
/// This metric works by:
/// 1. Computing n-gram count differences (deltas) between original→expected and original→actual
/// 2. Comparing these deltas to measure how well actual edits match expected edits
///
/// Returns a score from 0.0 to 100.0, where 100.0 means the actual edits perfectly match
/// the expected edits.
pub fn delta_chr_f(original: &str, expected: &str, actual: &str) -> f64 {
    let skip_whitespace = matches!(CHR_F_WHITESPACE, ChrfWhitespace::Ignore);

    let mut orig_iter = original
        .chars()
        .filter(|c| !skip_whitespace || !c.is_whitespace());
    let mut exp_iter = expected
        .chars()
        .filter(|c| !skip_whitespace || !c.is_whitespace());
    let mut act_iter = actual
        .chars()
        .filter(|c| !skip_whitespace || !c.is_whitespace());

    let mut all_equal = true;
    loop {
        match (orig_iter.next(), exp_iter.next(), act_iter.next()) {
            (Some(o), Some(e), Some(a)) => {
                if o != e || o != a {
                    all_equal = false;
                    break;
                }
            }
            _ => {
                all_equal = false;
                break;
            }
        }
    }

    if all_equal {
        return 100.0;
    }

    let original_chars: Vec<char> = original
        .chars()
        .filter(|c| !skip_whitespace || !c.is_whitespace())
        .collect();
    let expected_chars: Vec<char> = expected
        .chars()
        .filter(|c| !skip_whitespace || !c.is_whitespace())
        .collect();
    let actual_chars: Vec<char> = actual
        .chars()
        .filter(|c| !skip_whitespace || !c.is_whitespace())
        .collect();

    // Key: [order, char1, char2, ..., char6]
    // Value: [original_count, expected_count, actual_count]
    type Ngram = [char; CHR_F_CHAR_ORDER + 1];
    type AllCounts = collections::FxHashMap<Ngram, [usize; 3]>;

    let mut all_counts = AllCounts::default();
    all_counts.reserve(CHR_F_CHAR_ORDER * original_chars.len());

    // Collect all ngrams from all texts in a single HashMap
    for (text_idx, chars) in [&original_chars, &expected_chars, &actual_chars]
        .iter()
        .enumerate()
    {
        for order in 1..=CHR_F_CHAR_ORDER {
            for window in chars.windows(order) {
                let mut ngram: Ngram = ['\0'; 7];
                ngram[0] = char::from_u32(order as u32).unwrap();
                ngram[1..1 + order].copy_from_slice(window);
                all_counts.entry(ngram).or_default()[text_idx] += 1;
            }
        }
    }

    let mut order_stats = [(0usize, 0usize, 0usize); CHR_F_CHAR_ORDER];

    for (ngram, &[orig, exp, act]) in &all_counts {
        let order = ngram[0] as usize;
        if order == 0 || order > CHR_F_CHAR_ORDER {
            continue;
        }
        let (ref mut true_positives, ref mut false_positives, ref mut false_negatives) =
            order_stats[order - 1];

        let exp_delta = exp as isize - orig as isize;
        let act_delta = act as isize - orig as isize;

        if exp_delta > 0 || act_delta > 0 {
            let exp_add = exp_delta.max(0) as usize;
            let act_add = act_delta.max(0) as usize;
            let matched = exp_add.min(act_add);
            *true_positives += matched;
            *false_positives += act_add.saturating_sub(matched);
            *false_negatives += exp_add.saturating_sub(matched);
        }

        if exp_delta < 0 || act_delta < 0 {
            let exp_del = (-exp_delta).max(0) as usize;
            let act_del = (-act_delta).max(0) as usize;
            let matched = exp_del.min(act_del);
            *true_positives += matched;
            *false_positives += act_del.saturating_sub(matched);
            *false_negatives += exp_del.saturating_sub(matched);
        }
    }

    let mut total_precision = 0.0;
    let mut total_recall = 0.0;

    for (true_positives, false_positives, false_negatives) in order_stats {
        if true_positives == 0 && false_positives == 0 && false_negatives == 0 {
            total_precision += 1.0;
            total_recall += 1.0;
            continue;
        }

        let precision = if true_positives + false_positives == 0 {
            0.0
        } else {
            true_positives as f64 / (true_positives + false_positives) as f64
        };

        let recall = if true_positives + false_negatives == 0 {
            0.0
        } else {
            true_positives as f64 / (true_positives + false_negatives) as f64
        };

        total_precision += precision;
        total_recall += recall;
    }

    let prec = total_precision / CHR_F_CHAR_ORDER as f64;
    let recall = total_recall / CHR_F_CHAR_ORDER as f64;
    let f_score = if prec + recall == 0.0 {
        0.0
    } else {
        (1.0 + CHR_F_BETA * CHR_F_BETA) * prec * recall / (CHR_F_BETA * CHR_F_BETA * prec + recall)
    };

    f_score * 100.0
}

#[cfg(test)]
mod test {
    use super::*;
    use collections::HashMap;
    use std::hint::black_box;
    use util_macros::perf;

    fn reference_delta_chr_f(original: &str, expected: &str, actual: &str) -> f64 {
        let original_chars: Vec<char> = original.chars().filter(|c| !c.is_whitespace()).collect();
        let expected_chars: Vec<char> = expected.chars().filter(|c| !c.is_whitespace()).collect();
        let actual_chars: Vec<char> = actual.chars().filter(|c| !c.is_whitespace()).collect();

        fn get_ngram_counts(chars: &[char], order: usize) -> HashMap<Vec<char>, usize> {
            let mut counts = HashMap::default();
            for window in chars.windows(order) {
                *counts.entry(window.to_vec()).or_insert(0) += 1;
            }
            counts
        }

        fn compute_deltas(
            orig_counts: &HashMap<Vec<char>, usize>,
            new_counts: &HashMap<Vec<char>, usize>,
        ) -> HashMap<Vec<char>, isize> {
            let mut deltas = HashMap::default();
            for (ngram, &count) in new_counts {
                let orig = *orig_counts.get(ngram).unwrap_or(&0);
                let delta = count as isize - orig as isize;
                if delta != 0 {
                    deltas.insert(ngram.clone(), delta);
                }
            }
            for (ngram, &count) in orig_counts {
                if !new_counts.contains_key(ngram) {
                    deltas.insert(ngram.clone(), -(count as isize));
                }
            }
            deltas
        }

        let mut total_precision = 0.0;
        let mut total_recall = 0.0;

        for order in 1..=CHR_F_CHAR_ORDER {
            let orig_counts = get_ngram_counts(&original_chars, order);
            let exp_counts = get_ngram_counts(&expected_chars, order);
            let act_counts = get_ngram_counts(&actual_chars, order);

            let exp_deltas = compute_deltas(&orig_counts, &exp_counts);
            let act_deltas = compute_deltas(&orig_counts, &act_counts);

            let mut true_positives = 0usize;
            let mut false_positives = 0usize;
            let mut false_negatives = 0usize;

            let mut all_ngrams: std::collections::HashSet<Vec<char>> =
                std::collections::HashSet::new();
            all_ngrams.extend(exp_deltas.keys().cloned());
            all_ngrams.extend(act_deltas.keys().cloned());

            for ngram in all_ngrams {
                let exp_delta = *exp_deltas.get(&ngram).unwrap_or(&0);
                let act_delta = *act_deltas.get(&ngram).unwrap_or(&0);

                if exp_delta > 0 || act_delta > 0 {
                    let exp_add = exp_delta.max(0) as usize;
                    let act_add = act_delta.max(0) as usize;
                    let matched = exp_add.min(act_add);
                    true_positives += matched;
                    false_positives += act_add.saturating_sub(matched);
                    false_negatives += exp_add.saturating_sub(matched);
                }

                if exp_delta < 0 || act_delta < 0 {
                    let exp_del = (-exp_delta).max(0) as usize;
                    let act_del = (-act_delta).max(0) as usize;
                    let matched = exp_del.min(act_del);
                    true_positives += matched;
                    false_positives += act_del.saturating_sub(matched);
                    false_negatives += exp_del.saturating_sub(matched);
                }
            }

            if true_positives == 0 && false_positives == 0 && false_negatives == 0 {
                total_precision += 1.0;
                total_recall += 1.0;
            } else {
                let precision = if true_positives + false_positives == 0 {
                    0.0
                } else {
                    true_positives as f64 / (true_positives + false_positives) as f64
                };
                let recall = if true_positives + false_negatives == 0 {
                    0.0
                } else {
                    true_positives as f64 / (true_positives + false_negatives) as f64
                };
                total_precision += precision;
                total_recall += recall;
            }
        }

        let prec = total_precision / CHR_F_CHAR_ORDER as f64;
        let recall = total_recall / CHR_F_CHAR_ORDER as f64;
        let f_score = if prec + recall == 0.0 {
            0.0
        } else {
            (1.0 + CHR_F_BETA * CHR_F_BETA) * prec * recall
                / (CHR_F_BETA * CHR_F_BETA * prec + recall)
        };

        f_score * 100.0
    }

    fn assert_score_eq(original: &str, expected: &str, actual: &str) {
        let optimized = delta_chr_f(original, expected, actual);
        let reference = reference_delta_chr_f(original, expected, actual);
        assert!(
            (optimized - reference).abs() < 1e-10,
            "Mismatch for original={:?}, expected={:?}, actual={:?}: optimized={}, reference={}",
            original,
            expected,
            actual,
            optimized,
            reference
        );
    }

    #[test]
    fn test_delta_chr_f_against_reference_impl() {
        assert_score_eq("hello", "hello", "hello");
        assert_score_eq("", "", "");
        assert_score_eq("a", "a", "a");
        assert_score_eq("original", "modified", "modified");
        assert_score_eq(
            "fn main() {}",
            "fn main() { println!(); }",
            "fn main() { println!(); }",
        );
        assert_score_eq("hello", "hello", "goodbye");
        assert_score_eq("test", "test", "different");
        assert_score_eq("hello", "goodbye", "hello");
        assert_score_eq("old", "new", "old");
        assert_score_eq("let x = 42;", "let x = 100;", "let x = 99;");
        assert_score_eq("one two three", "one three", "one two four");
        assert_score_eq("a", "b", "b");
        assert_score_eq("a", "b", "c");
        assert_score_eq("héllo", "héllo!", "héllo!");
        assert_score_eq("日本語", "日本語テスト", "日本語テスト");
        assert_score_eq("", "hello", "hello");
        assert_score_eq("", "hello", "");
        assert_score_eq("hello", "", "");
        assert_score_eq("hello", "", "hello");

        assert_score_eq("aaa", "aaabbb", "aaabbb");
        assert_score_eq("aaa", "aaabbb", "aaaccc");
        assert_score_eq("abcdef", "abXYZdef", "abXYZdef");
        assert_score_eq("abcdef", "abXYZdef", "abABCdef");
        assert_score_eq(
            "the quick brown fox",
            "the slow brown fox",
            "the slow brown fox",
        );
        assert_score_eq(
            "the quick brown fox",
            "the slow brown fox",
            "the fast brown fox",
        );

        assert_score_eq("aaaa", "aaa", "aaa");
        assert_score_eq("aaaa", "aaa", "aa");
        assert_score_eq("aabb", "ab", "ab");
        assert_score_eq("aabb", "ab", "ba");
    }

    #[test]
    fn test_delta_chr_f_different_lengths() {
        assert_score_eq(
            "short",
            "this is a much longer string now",
            "this is a much longer string now",
        );
        assert_score_eq("this is a long string", "hi", "hi");
        assert_score_eq(
            "abc",
            "abcdefghijklmnopqrstuvwxyz",
            "abcdefghijklmnopqrstuvwxyz",
        );
        assert_score_eq("abcdefghijklmnopqrstuvwxyz", "abc", "abc");

        assert_score_eq("", "x", "x");
        assert_score_eq("", "x", "y");
        assert_score_eq("x", "", "");
        assert_score_eq("x", "", "x");
    }

    #[test]
    fn test_delta_chr_f_repeated_patterns() {
        assert_score_eq("ababab", "abababab", "abababab");
        assert_score_eq("ababab", "abababab", "ababab");
        assert_score_eq("aaabbb", "aaabbbbbb", "aaabbbbbb");
        assert_score_eq("aaabbb", "aaabbbbbb", "aaabbbccc");

        assert_score_eq("xyzxyzxyz", "xyzxyz", "xyzxyz");
        assert_score_eq("xyzxyzxyz", "xyzxyz", "xyzxyzxyz");
    }

    #[test]
    fn test_delta_chr_f_ngram_boundary_cases() {
        assert_score_eq("12345", "123456", "123456");
        assert_score_eq("123456", "1234567", "1234567");
        assert_score_eq("1234567", "12345678", "12345678");

        assert_score_eq("abcdefg", "Xbcdefg", "Xbcdefg");
        assert_score_eq("abcdefg", "abcdefX", "abcdefX");
        assert_score_eq("abcdefg", "abcXefg", "abcXefg");
        assert_score_eq("abcdefg", "abcXefg", "abcYefg");
    }

    #[test]
    fn test_delta_chr_f_symmetry_properties() {
        let s1 = delta_chr_f("abc", "abcXYZ", "abcXYZ");
        assert!(
            (s1 - 100.0).abs() < 1e-10,
            "perfect match should be 100: {}",
            s1
        );

        let s2 = delta_chr_f("abc", "abcXYZ", "abc");
        let s3 = delta_chr_f("abc", "abc", "abcXYZ");
        assert!(s2 < 50.0, "missed addition should score low: {}", s2);
        assert!(s3 < 50.0, "unwanted addition should score low: {}", s3);

        let s4 = delta_chr_f("abcXYZ", "abc", "abc");
        assert!(
            (s4 - 100.0).abs() < 1e-10,
            "perfect deletion match should be 100: {}",
            s4
        );

        let s5 = delta_chr_f("abcXYZ", "abc", "abcXYZ");
        let s6 = delta_chr_f("abcXYZ", "abcXYZ", "abc");
        assert!(s5 < 50.0, "missed deletion should score low: {}", s5);
        assert!(s6 < 50.0, "unwanted deletion should score low: {}", s6);
    }

    #[test]
    fn test_delta_chr_f_unicode_comprehensive() {
        assert_score_eq("日本語テスト", "日本語テスト結果", "日本語テスト結果");
        assert_score_eq("日本語テスト", "日本語テスト結果", "日本語テスト失敗");

        assert_score_eq("こんにちは", "こんにちは世界", "こんにちは世界");

        assert_score_eq("αβγδ", "αβγδεζ", "αβγδεζ");
        assert_score_eq("αβγδ", "αβγδεζ", "αβγδηθ");

        assert_score_eq("🎉🎊", "🎉🎊🎈", "🎉🎊🎈");
        assert_score_eq("🎉🎊", "🎉🎊🎈", "🎉🎊🎁");

        assert_score_eq("Привет", "Привет мир", "Привет мир");
    }

    #[test]
    fn test_delta_chr_f_whitespace_handling() {
        assert_score_eq("a b c", "abc", "abc");
        assert_score_eq("a  b  c", "abc", "abc");
        assert_score_eq("a\tb\nc", "abc", "abc");

        assert_score_eq("hello world", "helloworld", "helloworld");
        assert_score_eq("hello world", "hello  world", "hello  world");

        let s1 = delta_chr_f("a b", "a b c", "a b c");
        let s2 = delta_chr_f("ab", "abc", "abc");
        assert!(
            (s1 - s2).abs() < 1e-10,
            "whitespace should be ignored: {} vs {}",
            s1,
            s2
        );
    }

    #[test]
    fn test_delta_chr_f_correctness() {
        assert_eq!(delta_chr_f("hello", "hello", "hello"), 100.0);
        assert_eq!(delta_chr_f("", "", ""), 100.0);
        assert_eq!(delta_chr_f("a", "a", "a"), 100.0);

        assert_eq!(delta_chr_f("original", "modified", "modified"), 100.0);
        assert_eq!(
            delta_chr_f(
                "fn main() {}",
                "fn main() { println!(); }",
                "fn main() { println!(); }"
            ),
            100.0
        );

        assert_eq!(delta_chr_f("a b c", "abc", "abc"), 100.0);
        assert_eq!(
            delta_chr_f("hello world", "helloworld", "helloworld"),
            100.0
        );

        assert_eq!(delta_chr_f("hello", "hello", "goodbye"), 0.0);
        assert_eq!(delta_chr_f("test", "test", "different"), 0.0);

        let score = delta_chr_f("hello", "goodbye", "hello");
        assert!(score < 10.0, "missed edit score should be low: {}", score);
        let score = delta_chr_f("old", "new", "old");
        assert!(score < 60.0, "missed edit score should be low: {}", score);

        let score = delta_chr_f("let x = 42;", "let x = 100;", "let x = 99;");
        assert!(
            score > 40.0 && score < 60.0,
            "partial match score: {}",
            score
        );

        let score = delta_chr_f("one two three", "one three", "one two four");
        assert!(score > 20.0 && score < 40.0, "wrong edit score: {}", score);

        assert_eq!(delta_chr_f("a", "b", "b"), 100.0);

        let score = delta_chr_f("a", "b", "c");
        assert!(
            score > 80.0,
            "single char score should be high due to empty orders: {}",
            score
        );

        assert_eq!(delta_chr_f("héllo", "héllo!", "héllo!"), 100.0);
        assert_eq!(delta_chr_f("日本語", "日本語テスト", "日本語テスト"), 100.0);

        assert_eq!(delta_chr_f("", "hello", "hello"), 100.0);
        let score = delta_chr_f("", "hello", "");
        assert!(score < 20.0, "empty actual when expected change: {}", score);
        assert_eq!(delta_chr_f("hello", "", ""), 100.0);
        let score = delta_chr_f("hello", "", "hello");
        assert!(score < 20.0, "no change when deletion expected: {}", score);
    }

    #[test]
    fn test_delta_chr_f_addition_deletion_interaction() {
        assert_score_eq("abc", "Xbc", "Xbc");
        assert_score_eq("abc", "Xbc", "Ybc");

        assert_score_eq("abc", "aXc", "aXc");
        assert_score_eq("abc", "aXc", "aYc");

        assert_score_eq("abcdef", "abXYZef", "abXYZef");
        assert_score_eq("abcdef", "abXYZef", "abABCef");

        assert_score_eq("abc", "XYZabc", "XYZabc");
        assert_score_eq("abc", "abcXYZ", "abcXYZ");
    }

    #[test]
    fn test_delta_chr_f_count_changes() {
        assert_score_eq("aa", "aaa", "aaa");
        assert_score_eq("aa", "aaa", "aa");
        assert_score_eq("aa", "aaa", "aaaa");

        assert_score_eq("aaa", "aa", "aa");
        assert_score_eq("aaa", "aa", "aaa");
        assert_score_eq("aaa", "aa", "a");

        assert_score_eq("aabb", "aaabbb", "aaabbb");
        assert_score_eq("aabb", "aaabbb", "aabbcc");
    }

    #[test]
    fn test_delta_chr_f_real_code_examples() {
        assert_score_eq("fn foo() {}", "fn foo() { bar(); }", "fn foo() { bar(); }");

        assert_score_eq("fn foo() {}", "fn foo() { bar(); }", "fn foo() { baz(); }");

        assert_score_eq("let x = 1;", "let x = 42;", "let x = 42;");

        assert_score_eq("let x = 1;", "let x = 42;", "let x = 99;");

        assert_score_eq(
            "if (condition) { action(); }",
            "if (condition) { action(); } else { other(); }",
            "if (condition) { action(); } else { other(); }",
        );

        assert_score_eq(
            "println!(\"hello\");",
            "println!(\"hello, world!\");",
            "println!(\"hello, world!\");",
        );

        assert_score_eq(
            "println!(\"hello\");",
            "println!(\"hello, world!\");",
            "println!(\"goodbye, world!\");",
        );
    }

    #[perf]
    fn test_delta_chr_f_perfect_match() {
        let original = "fn main() {    println!(\"Hello\");}";
        let expected = "fn main() {    println!(\"Hello, World!\");}";

        let score = delta_chr_f(original, expected, expected);
        assert!((score - 100.0).abs() < 1e-2);
    }

    #[perf]
    fn test_delta_chr_f_wrong_edit() {
        // When the edit is wrong
        let original = "one two three";
        let expected = "one three"; // deleted "two "
        let actual = "one two four"; // deleted "three", added "four"

        // Then the score should be low
        let score = delta_chr_f(original, expected, actual);
        assert!(score > 20.0 && score < 40.0);
    }

    #[perf]
    fn test_delta_chr_f_partial_match() {
        let original = "let x = 42;";
        let expected = "let x = 100;";
        let actual = "let x = 99;";

        // We got the edit location right, but the replacement text is wrong.
        // Deleted ngrams will match, bringing the score somewhere in the middle.
        let score = delta_chr_f(original, expected, actual);
        assert!(score > 40.0 && score < 60.0);
    }

    #[perf]
    fn test_delta_chr_f_missed_edit() {
        // When predictions makes no changes
        let original = "prefix old suffix";
        let expected = "prefix new suffix";
        let actual = "prefix old suffix"; // no change

        // Then the score should be low (all expected changes are false negatives)
        let score = delta_chr_f(original, expected, actual);
        assert!(score < 20.0);
    }

    #[perf]
    fn test_delta_chr_f_extra_edit() {
        // When adding unexpected content
        let original = "helloworld";
        let expected = "helloworld"; // no change expected
        let actual = "helloextraworld"; // added "extra"

        // Then the score should be low (all actual changes are false positives)
        let score = delta_chr_f(original, expected, actual);
        assert!(score < 20.0);
    }

    #[perf]
    fn test_delta_chr_f_no_changes() {
        let text = "unchanged text";
        let score = delta_chr_f(text, text, text);
        assert!((score - 100.0).abs() < 1e-2);
    }

    #[perf(weight = 100, fluff)]
    fn bench_delta_chr_f_small_text() {
        let original = "fn main() { println!(\"Hello\"); }";
        let expected = "fn main() { println!(\"Hello, World!\"); }";
        let actual = "fn main() { println!(\"Hello, World!\"); }";

        let score = black_box(delta_chr_f(
            black_box(original),
            black_box(expected),
            black_box(actual),
        ));
        assert!(score > 0.0);
    }

    #[perf(weight = 100, fluff)]
    fn bench_delta_chr_f_medium_text() {
        let original = r#"
            fn calculate_sum(numbers: &[i32]) -> i32 {
                let mut sum = 0;
                for num in numbers {
                    sum += num;
                }
                sum
            }

            fn main() {
                let nums = vec![1, 2, 3, 4, 5];
                let result = calculate_sum(&nums);
                println!("Sum: {}", result);
            }
        "#;
        let expected = r#"
            fn calculate_sum(numbers: &[i32]) -> i32 {
                numbers.iter().sum()
            }

            fn main() {
                let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
                let result = calculate_sum(&nums);
                println!("The sum is: {}", result);
            }
        "#;
        let actual = r#"
            fn calculate_sum(numbers: &[i32]) -> i32 {
                numbers.iter().sum()
            }

            fn main() {
                let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
                let result = calculate_sum(&nums);
                println!("Sum: {}", result);
            }
        "#;

        let score = black_box(delta_chr_f(
            black_box(original),
            black_box(expected),
            black_box(actual),
        ));
        assert!(score > 0.0);
    }
}
