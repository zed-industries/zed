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
    let should_skip_whitespace = matches!(CHR_F_WHITESPACE, ChrfWhitespace::Ignore);

    let skip_whitespace = |c: &char| !should_skip_whitespace || !c.is_whitespace();

    let mut orig_iter = original.chars().filter(skip_whitespace);
    let mut exp_iter = expected.chars().filter(skip_whitespace);
    let mut act_iter = actual.chars().filter(skip_whitespace);

    loop {
        match (orig_iter.next(), exp_iter.next(), act_iter.next()) {
            (Some(o), Some(e), Some(a)) => {
                if o != e || o != a {
                    break;
                }
            }
            (None, None, None) => {
                return 100.0;
            }
            _ => {
                break;
            }
        }
    }

    let original_chars: Vec<char> = original.chars().filter(skip_whitespace).collect();
    let expected_chars: Vec<char> = expected.chars().filter(skip_whitespace).collect();
    let actual_chars: Vec<char> = actual.chars().filter(skip_whitespace).collect();

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
    use std::hint::black_box;
    use util_macros::perf;

    // expected scores for calls to this function collected from reference implementation
    fn assert_score_eq(original: &str, expected: &str, actual: &str, expected_score: f64) {
        let score = delta_chr_f(original, expected, actual);
        assert!(
            (score - expected_score).abs() < 1e-8,
            "Score mismatch for original={:?}, expected={:?}, actual={:?}: got {}, expected {}",
            original,
            expected,
            actual,
            score,
            expected_score
        );
    }

    #[test]
    fn test_delta_chr_f_basic() {
        assert_score_eq("hello", "hello", "hello", 100.0);
        assert_score_eq("", "", "", 100.0);
        assert_score_eq("a", "a", "a", 100.0);
        assert_score_eq("original", "modified", "modified", 100.0);
        assert_score_eq(
            "fn main() {}",
            "fn main() { println!(); }",
            "fn main() { println!(); }",
            100.0,
        );
        assert_score_eq("hello", "hello", "goodbye", 0.0);
        assert_score_eq("test", "test", "different", 0.0);
        assert_score_eq("hello", "goodbye", "hello", 0.0);
        assert_score_eq("old", "new", "old", 50.0);
        assert_score_eq("let x = 42;", "let x = 100;", "let x = 99;", 43.7131630648);
        assert_score_eq("one two three", "one three", "one two four", 38.6574721213);
        assert_score_eq("a", "b", "b", 100.0);
        assert_score_eq("a", "b", "c", 91.6666666667);
        assert_score_eq("héllo", "héllo!", "héllo!", 100.0);
        assert_score_eq("日本語", "日本語テスト", "日本語テスト", 100.0);
        assert_score_eq("", "hello", "hello", 100.0);
        assert_score_eq("", "hello", "", 16.6666666667);
        assert_score_eq("hello", "", "", 100.0);
        assert_score_eq("hello", "", "hello", 16.6666666667);

        assert_score_eq("aaa", "aaabbb", "aaabbb", 100.0);
        assert_score_eq("aaa", "aaabbb", "aaaccc", 0.0);
        assert_score_eq("abcdef", "abXYZdef", "abXYZdef", 100.0);
        assert_score_eq("abcdef", "abXYZdef", "abABCdef", 31.9444444444);
        assert_score_eq(
            "the quick brown fox",
            "the slow brown fox",
            "the slow brown fox",
            100.0,
        );
        assert_score_eq(
            "the quick brown fox",
            "the slow brown fox",
            "the fast brown fox",
            55.8430458430,
        );

        assert_score_eq("aaaa", "aaa", "aaa", 100.0);
        assert_score_eq("aaaa", "aaa", "aa", 93.75);
        assert_score_eq("aabb", "ab", "ab", 100.0);
        assert_score_eq("aabb", "ab", "ba", 98.2142857143);
    }

    #[test]
    fn test_delta_chr_f_different_lengths() {
        assert_score_eq(
            "short",
            "this is a much longer string now",
            "this is a much longer string now",
            100.0,
        );
        assert_score_eq("this is a long string", "hi", "hi", 100.0);
        assert_score_eq(
            "abc",
            "abcdefghijklmnopqrstuvwxyz",
            "abcdefghijklmnopqrstuvwxyz",
            100.0,
        );
        assert_score_eq("abcdefghijklmnopqrstuvwxyz", "abc", "abc", 100.0);

        assert_score_eq("", "x", "x", 100.0);
        assert_score_eq("", "x", "y", 83.3333333333);
        assert_score_eq("x", "", "", 100.0);
        assert_score_eq("x", "", "x", 83.3333333333);
    }

    #[test]
    fn test_delta_chr_f_repeated_patterns() {
        assert_score_eq("ababab", "abababab", "abababab", 100.0);
        assert_score_eq("ababab", "abababab", "ababab", 0.0);
        assert_score_eq("aaabbb", "aaabbbbbb", "aaabbbbbb", 100.0);
        assert_score_eq("aaabbb", "aaabbbbbb", "aaabbbccc", 0.0);

        assert_score_eq("xyzxyzxyz", "xyzxyz", "xyzxyz", 100.0);
        assert_score_eq("xyzxyzxyz", "xyzxyz", "xyzxyzxyz", 0.0);
    }

    #[test]
    fn test_delta_chr_f_ngram_boundary_cases() {
        assert_score_eq("12345", "123456", "123456", 100.0);
        assert_score_eq("123456", "1234567", "1234567", 100.0);
        assert_score_eq("1234567", "12345678", "12345678", 100.0);

        assert_score_eq("abcdefg", "Xbcdefg", "Xbcdefg", 100.0);
        assert_score_eq("abcdefg", "abcdefX", "abcdefX", 100.0);
        assert_score_eq("abcdefg", "abcXefg", "abcXefg", 100.0);
        assert_score_eq("abcdefg", "abcXefg", "abcYefg", 50.0);
    }

    #[test]
    fn test_delta_chr_f_unicode_comprehensive() {
        assert_score_eq(
            "日本語テスト",
            "日本語テスト結果",
            "日本語テスト結果",
            100.0,
        );
        assert_score_eq("日本語テスト", "日本語テスト結果", "日本語テスト失敗", 0.0);

        assert_score_eq("こんにちは", "こんにちは世界", "こんにちは世界", 100.0);

        assert_score_eq("αβγδ", "αβγδεζ", "αβγδεζ", 100.0);
        assert_score_eq("αβγδ", "αβγδεζ", "αβγδηθ", 0.0);

        assert_score_eq("🎉🎊", "🎉🎊🎈", "🎉🎊🎈", 100.0);
        assert_score_eq("🎉🎊", "🎉🎊🎈", "🎉🎊🎁", 50.0);

        assert_score_eq("Привет", "Привет мир", "Привет мир", 100.0);
    }

    #[test]
    fn test_delta_chr_f_whitespace_handling() {
        assert_score_eq("a b c", "abc", "abc", 100.0);
        assert_score_eq("a  b  c", "abc", "abc", 100.0);
        assert_score_eq("a\tb\nc", "abc", "abc", 100.0);

        assert_score_eq("hello world", "helloworld", "helloworld", 100.0);
        assert_score_eq("hello world", "hello  world", "hello  world", 100.0);
    }

    #[test]
    fn test_delta_chr_f_addition_deletion_interaction() {
        assert_score_eq("abc", "Xbc", "Xbc", 100.0);
        assert_score_eq("abc", "Xbc", "Ybc", 75.0);

        assert_score_eq("abc", "aXc", "aXc", 100.0);
        assert_score_eq("abc", "aXc", "aYc", 75.0);

        assert_score_eq("abcdef", "abXYZef", "abXYZef", 100.0);
        assert_score_eq("abcdef", "abXYZef", "abABCef", 40.5820105820);

        assert_score_eq("abc", "XYZabc", "XYZabc", 100.0);
        assert_score_eq("abc", "abcXYZ", "abcXYZ", 100.0);
    }

    #[test]
    fn test_delta_chr_f_count_changes() {
        assert_score_eq("aa", "aaa", "aaa", 100.0);
        assert_score_eq("aa", "aaa", "aa", 50.0);
        assert_score_eq("aa", "aaa", "aaaa", 76.7543859649);

        assert_score_eq("aaa", "aa", "aa", 100.0);
        assert_score_eq("aaa", "aa", "aaa", 50.0);
        assert_score_eq("aaa", "aa", "a", 96.1538461538);

        assert_score_eq("aabb", "aaabbb", "aaabbb", 100.0);
        assert_score_eq("aabb", "aaabbb", "aabbcc", 0.0);
    }

    #[test]
    fn test_delta_chr_f_real_code_examples() {
        assert_score_eq(
            "fn foo() {}",
            "fn foo() { bar(); }",
            "fn foo() { bar(); }",
            100.0,
        );

        assert_score_eq(
            "fn foo() {}",
            "fn foo() { bar(); }",
            "fn foo() { baz(); }",
            57.6388888889,
        );

        assert_score_eq("let x = 1;", "let x = 42;", "let x = 42;", 100.0);

        assert_score_eq("let x = 1;", "let x = 42;", "let x = 99;", 38.8888888889);

        assert_score_eq(
            "if (condition) { action(); }",
            "if (condition) { action(); } else { other(); }",
            "if (condition) { action(); } else { other(); }",
            100.0,
        );

        assert_score_eq(
            "println!(\"hello\");",
            "println!(\"hello, world!\");",
            "println!(\"hello, world!\");",
            100.0,
        );

        assert_score_eq(
            "println!(\"hello\");",
            "println!(\"hello, world!\");",
            "println!(\"goodbye, world!\");",
            64.3973946892,
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
