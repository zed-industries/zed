use crate::word_diff::tokenize;

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

/// Build the full LCS dynamic-programming table for backtracking.
///
/// `dp[i][j]` = LCS length of `a[..i]` and `b[..j]`.
fn lcs_table(a: &[&str], b: &[&str]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    for i in 1..=n {
        let elem_a = a[i - 1];
        for j in 1..=m {
            if elem_a == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                let up = dp[i - 1][j];
                let left = dp[i][j - 1];
                dp[i][j] = up.max(left);
            }
        }
    }
    dp
}

/// Return a boolean mask over `a` where `true` means the token is part of
/// one LCS(a, b), interpreted as "kept".
fn lcs_keep_mask(a: &[&str], b: &[&str]) -> Vec<bool> {
    if a.is_empty() || b.is_empty() {
        return vec![false; a.len()];
    }

    let dp = lcs_table(a, b);
    let mut keep = vec![false; a.len()];

    let mut i = a.len();
    let mut j = b.len();

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            keep[i - 1] = true;
            i -= 1;
            j -= 1;
        } else {
            let up = dp[i - 1][j];
            let left = dp[i][j - 1];
            if up >= left {
                i -= 1;
            } else {
                j -= 1;
            }
        }
    }

    keep
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
    let base_tokens = tokenize(base);
    let predicted_tokens = tokenize(predicted);
    let final_tokens = tokenize(final_text);

    // Context in predicted: tokens matched in BOTH base and final.
    let pred_base_mask = lcs_keep_mask(&predicted_tokens, &base_tokens);
    let pred_final_mask = lcs_keep_mask(&predicted_tokens, &final_tokens);
    let context_mask: Vec<bool> = pred_base_mask
        .iter()
        .zip(pred_final_mask.iter())
        .map(|(&b, &f)| b && f)
        .collect();

    let stripped_predicted: Vec<&str> = predicted_tokens
        .iter()
        .zip(context_mask.iter())
        .filter(|(_, c)| !*c)
        .map(|(t, _)| *t)
        .collect();

    // Context in final: tokens matched in BOTH base and predicted.
    let final_base_mask = lcs_keep_mask(&final_tokens, &base_tokens);
    let final_pred_mask = lcs_keep_mask(&final_tokens, &predicted_tokens);
    let final_context_mask: Vec<bool> = final_base_mask
        .iter()
        .zip(final_pred_mask.iter())
        .map(|(&b, &p)| b && p)
        .collect();

    let stripped_final: Vec<&str> = final_tokens
        .iter()
        .zip(final_context_mask.iter())
        .filter(|(_, c)| !*c)
        .map(|(t, _)| *t)
        .collect();

    let keep_mask = lcs_keep_mask(&stripped_predicted, &stripped_final);

    let predicted_new_chars: usize = stripped_predicted.iter().map(|t| t.len()).sum();
    let final_new_chars: usize = stripped_final.iter().map(|t| t.len()).sum();
    let kept_chars: usize = stripped_predicted
        .iter()
        .zip(keep_mask.iter())
        .filter(|(_, k)| **k)
        .map(|(t, _)| t.len())
        .sum();
    let context_chars: usize = predicted_tokens
        .iter()
        .zip(context_mask.iter())
        .filter(|(_, c)| **c)
        .map(|(t, _)| t.len())
        .sum();
    let discarded_chars = predicted_new_chars - kept_chars;

    let kept_rate = if predicted_new_chars == 0 {
        if final_new_chars == 0 { 1.0 } else { 0.0 }
    } else {
        kept_chars as f64 / predicted_new_chars as f64
    };

    let mut token_annotations = Vec::with_capacity(predicted_tokens.len());
    let mut new_idx = 0;
    for (token_idx, _token) in predicted_tokens.iter().enumerate() {
        if context_mask[token_idx] {
            token_annotations.push(TokenAnnotation::Context);
        } else {
            let annotation = if keep_mask[new_idx] {
                TokenAnnotation::Kept
            } else {
                TokenAnnotation::Discarded
            };
            token_annotations.push(annotation);
            new_idx += 1;
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
}
