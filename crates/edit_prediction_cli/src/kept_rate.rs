use crate::word_diff::tokenize;
use similar::{DiffTag, TextDiff};
use std::collections::HashMap;

/// Per-token annotation for debug/visualization of kept rate results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenAnnotation {
    /// Token is shared context (present in base, predicted, and final).
    Context,
    /// Token is new in the prediction and was kept in the final result.
    Kept,
    /// Token is new in the prediction but was discarded in the final result.
    Discarded,
}

/// Result of `compute_kept_rate`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KeptRateResult {
    /// Number of characters in predicted tokens that are not three-way context.
    pub predicted_new_chars: usize,
    /// Number of characters in final tokens that are not three-way context.
    pub final_new_chars: usize,
    /// Characters from the prediction's new tokens that were kept in the final.
    pub kept_chars: usize,
    /// Characters from the prediction's new tokens that were discarded.
    pub discarded_chars: usize,
    /// Characters in predicted that are three-way shared context.
    pub context_chars: usize,
    /// `kept_chars / predicted_new_chars`, or 1.0 when both sides have zero new chars.
    pub kept_rate: f64,
    /// One annotation per predicted token (same order as `tokenize(predicted)`).
    pub token_annotations: Vec<TokenAnnotation>,
}

fn common_prefix_len(a: &[&str], b: &[&str]) -> usize {
    a.iter()
        .zip(b.iter())
        .take_while(|(left, right)| left == right)
        .count()
}

fn common_suffix_len(a: &[&str], b: &[&str], prefix_len: usize) -> usize {
    let max_suffix = (a.len() - prefix_len).min(b.len() - prefix_len);
    let mut suffix_len = 0;

    while suffix_len < max_suffix {
        let a_index = a.len() - 1 - suffix_len;
        let b_index = b.len() - 1 - suffix_len;
        if a[a_index] != b[b_index] {
            break;
        }
        suffix_len += 1;
    }

    suffix_len
}

const DENSE_REGION_DP_CELL_THRESHOLD: usize = 200_000;
const HIGH_MATCH_DENSITY_NUMERATOR: u128 = 1;
const HIGH_MATCH_DENSITY_DENOMINATOR: u128 = 32;

fn dp_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}

fn estimated_match_pairs(a: &[&str], b: &[&str]) -> u128 {
    let mut counts_by_token = HashMap::new();
    for &token in a {
        *counts_by_token.entry(token).or_insert(0usize) += 1;
    }

    let mut match_pairs = 0u128;
    for &token in b {
        if let Some(&count) = counts_by_token.get(token) {
            match_pairs += count as u128;
        }
    }

    match_pairs
}

#[cold]
fn should_use_diff_alignment(a: &[&str], b: &[&str]) -> bool {
    let dp_cell_count = (a.len() as u128 + 1) * (b.len() as u128 + 1);
    if dp_cell_count < DENSE_REGION_DP_CELL_THRESHOLD as u128 {
        return false;
    }

    let match_pairs = estimated_match_pairs(a, b);
    match_pairs * HIGH_MATCH_DENSITY_DENOMINATOR >= dp_cell_count * HIGH_MATCH_DENSITY_NUMERATOR
}

#[cold]
fn mark_equal_diff_ranges(a: &[&str], b: &[&str], keep_a: &mut [bool], keep_b: &mut [bool]) {
    let diff = TextDiff::from_slices(a, b);
    for operation in diff.ops() {
        if operation.tag() != DiffTag::Equal {
            continue;
        }

        for index in operation.old_range() {
            keep_a[index] = true;
        }
        for index in operation.new_range() {
            keep_b[index] = true;
        }
    }
}

/// Return boolean masks over `a` and `b` where `true` means the token is part
/// of one LCS(a, b), interpreted as "kept".
fn lcs_keep_masks(a: &[&str], b: &[&str]) -> (Vec<bool>, Vec<bool>) {
    if a.is_empty() || b.is_empty() {
        return (vec![false; a.len()], vec![false; b.len()]);
    }

    if a == b {
        return (vec![true; a.len()], vec![true; b.len()]);
    }

    let mut keep_a = vec![false; a.len()];
    let mut keep_b = vec![false; b.len()];

    let prefix_len = common_prefix_len(a, b);
    let suffix_len = common_suffix_len(a, b, prefix_len);

    for index in 0..prefix_len {
        keep_a[index] = true;
        keep_b[index] = true;
    }

    for offset in 0..suffix_len {
        let a_index = a.len() - suffix_len + offset;
        let b_index = b.len() - suffix_len + offset;
        keep_a[a_index] = true;
        keep_b[b_index] = true;
    }

    let a_mid = &a[prefix_len..a.len() - suffix_len];
    let b_mid = &b[prefix_len..b.len() - suffix_len];

    if a_mid.is_empty() || b_mid.is_empty() {
        return (keep_a, keep_b);
    }

    if should_use_diff_alignment(a_mid, b_mid) {
        let a_mid_start = prefix_len;
        let b_mid_start = prefix_len;
        mark_equal_diff_ranges(
            a_mid,
            b_mid,
            &mut keep_a[a_mid_start..a_mid_start + a_mid.len()],
            &mut keep_b[b_mid_start..b_mid_start + b_mid.len()],
        );
        return (keep_a, keep_b);
    }

    let row_count = a_mid.len() + 1;
    let column_count = b_mid.len() + 1;
    let mut dp = vec![0u32; row_count * column_count];

    for i in 1..row_count {
        let token_a = a_mid[i - 1];
        for j in 1..column_count {
            let index = dp_index(column_count, i, j);
            if token_a == b_mid[j - 1] {
                dp[index] = dp[dp_index(column_count, i - 1, j - 1)] + 1;
            } else {
                let up = dp[dp_index(column_count, i - 1, j)];
                let left = dp[dp_index(column_count, i, j - 1)];
                dp[index] = up.max(left);
            }
        }
    }

    let mut i = a_mid.len();
    let mut j = b_mid.len();

    while i > 0 && j > 0 {
        if a_mid[i - 1] == b_mid[j - 1] {
            keep_a[prefix_len + i - 1] = true;
            keep_b[prefix_len + j - 1] = true;
            i -= 1;
            j -= 1;
        } else {
            let up = dp[dp_index(column_count, i - 1, j)];
            let left = dp[dp_index(column_count, i, j - 1)];
            if up >= left {
                i -= 1;
            } else {
                j -= 1;
            }
        }
    }

    (keep_a, keep_b)
}

fn lcs_keep_mask(a: &[&str], b: &[&str]) -> Vec<bool> {
    lcs_keep_masks(a, b).0
}

fn collect_unmasked_tokens<'a>(tokens: &[&'a str], mask: &[bool]) -> Vec<&'a str> {
    tokens
        .iter()
        .zip(mask.iter())
        .filter_map(|(&token, &is_masked)| (!is_masked).then_some(token))
        .collect()
}

fn sum_masked_chars(tokens: &[&str], mask: &[bool], masked_value: bool) -> usize {
    tokens
        .iter()
        .zip(mask.iter())
        .filter_map(|(&token, &is_masked)| (is_masked == masked_value).then_some(token.len()))
        .sum()
}

/// Compute kept rate by comparing predicted vs final full texts, excluding
/// three-way shared context (tokens unchanged across base, predicted, and
/// final).
///
/// Context is defined as tokens in predicted that are present in BOTH base
/// and final (via independent LCS computations). This ensures that tokens
/// the prediction should have deleted (in base, in predicted, but not in
/// final) are NOT treated as context and count against the prediction.
///
/// The result includes per-token annotations for debug visualization:
/// each predicted token is labelled [`TokenAnnotation::Context`],
/// [`TokenAnnotation::Kept`], or [`TokenAnnotation::Discarded`].
pub fn compute_kept_rate(base: &str, predicted: &str, final_text: &str) -> KeptRateResult {
    if base == predicted && predicted == final_text {
        let predicted_tokens = tokenize(predicted);
        let context_chars = predicted_tokens.iter().map(|token| token.len()).sum();
        return KeptRateResult {
            predicted_new_chars: 0,
            final_new_chars: 0,
            kept_chars: 0,
            discarded_chars: 0,
            context_chars,
            kept_rate: 1.0,
            token_annotations: vec![TokenAnnotation::Context; predicted_tokens.len()],
        };
    }

    let base_tokens = tokenize(base);
    let predicted_tokens = tokenize(predicted);
    let final_tokens = tokenize(final_text);

    // Context in predicted: tokens matched in BOTH base and final.
    let (pred_base_mask, _base_pred_mask) = lcs_keep_masks(&predicted_tokens, &base_tokens);
    let (pred_final_mask, final_pred_mask) = lcs_keep_masks(&predicted_tokens, &final_tokens);
    let context_mask: Vec<bool> = pred_base_mask
        .iter()
        .zip(pred_final_mask.iter())
        .map(|(&in_base, &in_final)| in_base && in_final)
        .collect();

    let stripped_predicted = collect_unmasked_tokens(&predicted_tokens, &context_mask);

    // Context in final: tokens matched in BOTH base and predicted.
    let (final_base_mask, _base_final_mask) = lcs_keep_masks(&final_tokens, &base_tokens);
    let final_context_mask: Vec<bool> = final_base_mask
        .iter()
        .zip(final_pred_mask.iter())
        .map(|(&in_base, &in_predicted)| in_base && in_predicted)
        .collect();

    let stripped_final = collect_unmasked_tokens(&final_tokens, &final_context_mask);

    let keep_mask = if stripped_predicted == stripped_final {
        vec![true; stripped_predicted.len()]
    } else {
        lcs_keep_mask(&stripped_predicted, &stripped_final)
    };

    let predicted_new_chars = sum_masked_chars(&predicted_tokens, &context_mask, false);
    let final_new_chars = sum_masked_chars(&final_tokens, &final_context_mask, false);
    let kept_chars: usize = stripped_predicted
        .iter()
        .zip(keep_mask.iter())
        .filter_map(|(&token, &is_kept)| is_kept.then_some(token.len()))
        .sum();
    let context_chars = sum_masked_chars(&predicted_tokens, &context_mask, true);
    let discarded_chars = predicted_new_chars - kept_chars;

    let kept_rate = if predicted_new_chars == 0 {
        if final_new_chars == 0 { 1.0 } else { 0.0 }
    } else {
        kept_chars as f64 / predicted_new_chars as f64
    };

    let mut token_annotations = Vec::with_capacity(predicted_tokens.len());
    let mut new_index = 0;
    for (token_index, _token) in predicted_tokens.iter().enumerate() {
        if context_mask[token_index] {
            token_annotations.push(TokenAnnotation::Context);
        } else {
            let annotation = if keep_mask[new_index] {
                TokenAnnotation::Kept
            } else {
                TokenAnnotation::Discarded
            };
            token_annotations.push(annotation);
            new_index += 1;
        }
    }

    KeptRateResult {
        predicted_new_chars,
        final_new_chars,
        kept_chars,
        discarded_chars,
        context_chars,
        kept_rate,
        token_annotations,
    }
}

#[cfg(test)]
mod test_kept_rate {
    use super::*;

    #[test]
    fn test_lcs_keep_mask_subsequence() {
        let a = vec!["a", "b", "c", "d", "e"];
        let b = vec!["a", "c", "e"];
        let mask = lcs_keep_mask(&a, &b);
        assert_eq!(mask, vec![true, false, true, false, true]);
    }

    #[test]
    fn test_lcs_keep_mask_tokens() {
        let mask = lcs_keep_mask(&["alpha", "beta", "gamma"], &["alpha", "gamma"]);
        assert_eq!(mask, vec![true, false, true]);
    }

    #[test]
    fn test_lcs_keep_masks_returns_both_sides() {
        let (a_mask, b_mask) = lcs_keep_masks(&["alpha", "beta", "gamma"], &["alpha", "gamma"]);
        assert_eq!(a_mask, vec![true, false, true]);
        assert_eq!(b_mask, vec![true, true]);
    }

    #[test]
    fn test_lcs_keep_mask_empty_a() {
        let mask = lcs_keep_mask(&[], &["x"]);
        assert!(mask.is_empty());
    }

    #[test]
    fn test_lcs_keep_mask_empty_b() {
        let mask = lcs_keep_mask(&["x"], &[]);
        assert_eq!(mask, vec![false]);
    }

    #[test]
    fn test_identical_prediction_and_final() {
        let base = "old line\n";
        let predicted = "new line\n";
        let final_text = "new line\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_pure_addition_identical() {
        let base = "";
        let predicted = "brand new line\n";
        let final_text = "brand new line\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert_eq!(result.kept_chars, result.predicted_new_chars);
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_pure_addition_discarded() {
        let base = "";
        let predicted = "brand new line\n";
        let final_text = "something completely different\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert!(result.kept_chars < result.predicted_new_chars);
    }

    #[test]
    fn test_rename_base_chars_excluded() {
        let base = "    foo(old_name)\n";
        let predicted = "    foo(new_name)\n";
        let final_text = "    foo(new_name)\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert_eq!(result.predicted_new_chars, "new_name".len());
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_decoy_when_base_excluded() {
        let base = "    decoy.when(mock_sync_hardware_api.sp()).then_return(SpeedStatus.IDLE)\n";
        let predicted = "    decoy.when(mock_sync_module_hardware.speed_status).then_return(SpeedStatus.IDLE)\n";
        let final_text = "    decoy.when(mock_sync_module_hardware.speed_status).then_return(SpeedStatus.IDLE)\n";
        let result = compute_kept_rate(base, predicted, final_text);
        let expected_new = "mock_sync_module_hardware".len() + "speed_status".len();
        assert_eq!(result.predicted_new_chars, expected_new);
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_missing_deletion() {
        let base = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        epr\n";
        let predicted = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        epr\neprintln!(\"\");\n";
        let final_text = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        eprintln!(\"\");\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert!(
            result.kept_rate < 0.85,
            "expected kept_rate < 0.85, got {}",
            result.kept_rate
        );
        assert!(result.discarded_chars > 0);
    }

    #[test]
    fn test_empty_prediction() {
        let base = "old line\n";
        let predicted = "";
        let final_text = "new line\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert!((result.kept_rate - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_partial_kept() {
        let base = "old\n";
        let predicted = "alpha\nbeta\ngamma\n";
        let final_text = "alpha\ngamma\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert!(result.kept_chars > 0);
        assert!(result.discarded_chars > 0);
        assert!(result.kept_rate > 0.0 && result.kept_rate < 1.0);
    }

    #[test]
    fn test_no_change() {
        let text = "unchanged line\n";
        let result = compute_kept_rate(text, text, text);
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
        assert_eq!(result.predicted_new_chars, 0);
    }

    #[test]
    fn test_eprintln_token_alignment() {
        let base = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        epr\n";
        let predicted = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        eprintln!(\"hello world!\");\n";
        let final_text = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        eprintln!(\"\");\n";
        let result = compute_kept_rate(base, predicted, final_text);
        assert!(result.discarded_chars > 0);
        assert!(result.kept_chars > 0);
        assert!(result.kept_rate > 0.0 && result.kept_rate < 1.0);
        assert_eq!(result.kept_chars, 14);
        assert_eq!(result.discarded_chars, 12);
    }

    #[test]
    fn test_raw_strings() {
        let result = compute_kept_rate("hello world", "hello brave new world", "hello new world");
        assert!(result.kept_chars > 0);
        assert!(result.discarded_chars > 0);
        assert!(result.kept_rate > 0.0 && result.kept_rate < 1.0);
    }

    #[test]
    fn test_all_same() {
        let result = compute_kept_rate("foo bar", "foo bar", "foo bar");
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
        assert_eq!(result.predicted_new_chars, 0);
    }

    #[test]
    fn test_pred_eq_final() {
        let result = compute_kept_rate("old", "new", "new");
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_pred_eq_base() {
        let result = compute_kept_rate("old", "old", "new");
        assert!((result.kept_rate - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_annotations_length_matches_tokens() {
        let base = "    foo(old_name)\n";
        let predicted = "    foo(new_name)\n";
        let final_text = "    foo(new_name)\n";
        let result = compute_kept_rate(base, predicted, final_text);
        let predicted_tokens = tokenize(predicted);
        assert_eq!(result.token_annotations.len(), predicted_tokens.len());
    }

    #[test]
    fn test_annotations_rename() {
        let base = "    foo(old_name)\n";
        let predicted = "    foo(new_name)\n";
        let final_text = "    foo(new_name)\n";
        let result = compute_kept_rate(base, predicted, final_text);
        let predicted_tokens = tokenize(predicted);

        for (i, (&token, &ann)) in predicted_tokens
            .iter()
            .zip(result.token_annotations.iter())
            .enumerate()
        {
            if token == "new_name" {
                assert_eq!(
                    ann,
                    TokenAnnotation::Kept,
                    "token {i} '{token}' should be Kept"
                );
            } else {
                assert_eq!(
                    ann,
                    TokenAnnotation::Context,
                    "token {i} '{token}' should be Context"
                );
            }
        }
    }

    #[test]
    fn test_annotations_eprintln_coloring() {
        let base = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        epr\n";
        let predicted = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        eprintln!(\"hello world!\");\n";
        let final_text = "    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {\n        eprintln!(\"\");\n";
        let result = compute_kept_rate(base, predicted, final_text);
        let predicted_tokens = tokenize(predicted);

        let eprintln_idx = predicted_tokens
            .iter()
            .position(|&t| t == "eprintln")
            .expect("eprintln token not found");

        for i in 0..eprintln_idx {
            assert_eq!(
                result.token_annotations[i],
                TokenAnnotation::Context,
                "token {i} '{}' should be Context",
                predicted_tokens[i]
            );
        }

        assert_eq!(
            result.token_annotations[eprintln_idx],
            TokenAnnotation::Kept
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 1],
            TokenAnnotation::Kept
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 2],
            TokenAnnotation::Kept
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 3],
            TokenAnnotation::Kept
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 4],
            TokenAnnotation::Discarded
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 5],
            TokenAnnotation::Discarded
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 6],
            TokenAnnotation::Discarded
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 7],
            TokenAnnotation::Discarded
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 8],
            TokenAnnotation::Kept
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 9],
            TokenAnnotation::Kept
        );
        assert_eq!(
            result.token_annotations[eprintln_idx + 10],
            TokenAnnotation::Kept
        );

        assert_eq!(
            *result
                .token_annotations
                .last()
                .expect("missing trailing annotation"),
            TokenAnnotation::Context
        );
    }

    #[test]
    fn test_annotations_all_context_when_no_change() {
        let text = "unchanged line\n";
        let result = compute_kept_rate(text, text, text);
        assert!(
            result
                .token_annotations
                .iter()
                .all(|&a| a == TokenAnnotation::Context)
        );
    }

    #[test]
    fn test_annotations_no_context_when_all_new() {
        let result = compute_kept_rate("", "brand new", "brand new");
        assert!(
            result
                .token_annotations
                .iter()
                .all(|&a| a != TokenAnnotation::Context)
        );
        assert!(
            result
                .token_annotations
                .iter()
                .all(|&a| a == TokenAnnotation::Kept)
        );
    }

    #[test]
    fn test_repetitive_tokens_remain_discarded() {
        let base = "foo + foo + foo + foo + foo\n".repeat(16);
        let predicted = "foo + foo + prediction_token + foo + foo\n".repeat(16);
        let final_text = "foo + foo + kept_token + foo + foo\n".repeat(16);
        let result = compute_kept_rate(&base, &predicted, &final_text);

        assert_eq!(result.kept_chars, 0);
        assert_eq!(result.discarded_chars, result.predicted_new_chars);
        assert_eq!(result.predicted_new_chars, "prediction_token".len() * 16);
    }
}
