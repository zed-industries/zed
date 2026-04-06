use crate::word_diff::tokenize;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenAnnotation {
    Context,
    Kept,
    Discarded,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KeptRateResult {
    pub predicted_new_chars: usize,
    pub final_new_chars: usize,
    pub kept_chars: usize,
    pub discarded_chars: usize,
    pub context_chars: usize,
    pub kept_rate: f64,
    #[cfg(test)]
    pub token_annotations: Vec<TokenAnnotation>,
}

fn dp_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}

/// Return masks over `a` and `b` using one-sided LCS tie-breaking for each
/// side while sharing a single DP table construction.
fn lcs_keep_masks(a: &[&str], b: &[&str]) -> (Vec<bool>, Vec<bool>) {
    if a.is_empty() || b.is_empty() {
        return (vec![false; a.len()], vec![false; b.len()]);
    }

    if a == b {
        return (vec![true; a.len()], vec![true; b.len()]);
    }

    let mut keep_a = vec![false; a.len()];
    let mut keep_b = vec![false; b.len()];

    let prefix_len = a
        .iter()
        .zip(b.iter())
        .take_while(|(left, right)| left == right)
        .count();
    let suffix_len = {
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
    };

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

    let mut i = a_mid.len();
    let mut j = b_mid.len();

    while i > 0 && j > 0 {
        if a_mid[i - 1] == b_mid[j - 1] {
            keep_b[prefix_len + j - 1] = true;
            i -= 1;
            j -= 1;
        } else {
            let up = dp[dp_index(column_count, i - 1, j)];
            let left = dp[dp_index(column_count, i, j - 1)];
            if left >= up {
                j -= 1;
            } else {
                i -= 1;
            }
        }
    }

    (keep_a, keep_b)
}

fn analyze_masked_tokens<'a>(tokens: &[&'a str], mask: &[bool]) -> (Vec<&'a str>, usize, usize) {
    let mut unmasked_tokens = Vec::with_capacity(tokens.len());
    let mut unmasked_chars = 0;
    let mut masked_chars = 0;

    for (&token, &is_masked) in tokens.iter().zip(mask.iter()) {
        if is_masked {
            masked_chars += token.len();
        } else {
            unmasked_tokens.push(token);
            unmasked_chars += token.len();
        }
    }

    (unmasked_tokens, unmasked_chars, masked_chars)
}

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
            #[cfg(test)]
            token_annotations: vec![TokenAnnotation::Context; predicted_tokens.len()],
        };
    }

    let base_tokens = tokenize(base);
    let predicted_tokens = tokenize(predicted);
    let final_tokens = tokenize(final_text);

    let (pred_base_mask, _) = lcs_keep_masks(&predicted_tokens, &base_tokens);
    let (pred_final_mask, final_pred_mask) = lcs_keep_masks(&predicted_tokens, &final_tokens);
    let context_mask: Vec<bool> = pred_base_mask
        .iter()
        .zip(pred_final_mask.iter())
        .map(|(&in_base, &in_final)| in_base && in_final)
        .collect();

    let (stripped_predicted, predicted_new_chars, context_chars) =
        analyze_masked_tokens(&predicted_tokens, &context_mask);

    let (final_base_mask, _) = lcs_keep_masks(&final_tokens, &base_tokens);
    let final_context_mask: Vec<bool> = final_base_mask
        .iter()
        .zip(final_pred_mask.iter())
        .map(|(&in_base, &in_predicted)| in_base && in_predicted)
        .collect();

    let (stripped_final, final_new_chars, _) =
        analyze_masked_tokens(&final_tokens, &final_context_mask);

    let keep_mask = lcs_keep_masks(&stripped_predicted, &stripped_final).0;

    let kept_chars: usize = stripped_predicted
        .iter()
        .zip(keep_mask.iter())
        .filter_map(|(&token, &is_kept)| is_kept.then_some(token.len()))
        .sum();

    let discarded_chars = predicted_new_chars - kept_chars;

    let kept_rate = if predicted_new_chars == 0 {
        if final_new_chars == 0 { 1.0 } else { 0.0 }
    } else {
        kept_chars as f64 / predicted_new_chars as f64
    };

    #[cfg(test)]
    let token_annotations = {
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
                #[cfg(test)]
                token_annotations.push(annotation);
                new_index += 1;
            }
        }
        token_annotations
    };

    KeptRateResult {
        predicted_new_chars,
        final_new_chars,
        kept_chars,
        discarded_chars,
        context_chars,
        kept_rate,
        #[cfg(test)]
        token_annotations,
    }
}

#[cfg(test)]
mod test_kept_rate {
    use super::*;

    #[test]
    fn test_lcs_keep_masks() {
        let (a_mask, b_mask) = lcs_keep_masks(&["a", "b", "c", "d", "e"], &["a", "c", "e"]);
        assert_eq!(a_mask, vec![true, false, true, false, true]);
        assert_eq!(b_mask, vec![true, true, true]);

        let (a_mask, b_mask) = lcs_keep_masks(&[], &["x"]);
        assert!(a_mask.is_empty());
        assert_eq!(b_mask, vec![false]);
    }

    #[test]
    fn test_lcs_keep_masks_matches_historical_one_sided_masks() {
        let a = ["x", "a", "x", "b"];
        let b = ["a", "x", "b", "x"];
        let (a_mask, b_mask) = lcs_keep_masks(&a, &b);
        assert_eq!(a_mask, lcs_keep_masks(&a, &b).0);
        assert_eq!(b_mask, lcs_keep_masks(&b, &a).0);
    }

    #[test]
    fn test_rate_extremes() {
        let no_change = compute_kept_rate("foo bar", "foo bar", "foo bar");
        assert!((no_change.kept_rate - 1.0).abs() < 1e-6);
        assert_eq!(no_change.predicted_new_chars, 0);
        assert!(
            no_change
                .token_annotations
                .iter()
                .all(|&annotation| annotation == TokenAnnotation::Context)
        );

        let accepted = compute_kept_rate("old", "new", "new");
        assert!((accepted.kept_rate - 1.0).abs() < 1e-6);

        let discarded = compute_kept_rate("old", "old", "new");
        assert!((discarded.kept_rate - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_pure_addition() {
        let kept = compute_kept_rate("", "brand new line\n", "brand new line\n");
        assert_eq!(kept.kept_chars, kept.predicted_new_chars);
        assert!(
            kept.token_annotations
                .iter()
                .all(|&annotation| annotation == TokenAnnotation::Kept)
        );

        let discarded =
            compute_kept_rate("", "brand new line\n", "something completely different\n");
        assert!(discarded.kept_chars < discarded.predicted_new_chars);
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
        let result = compute_kept_rate("old line\n", "", "new line\n");
        assert!((result.kept_rate - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_partial_kept() {
        let result = compute_kept_rate("old\n", "alpha\nbeta\ngamma\n", "alpha\ngamma\n");
        assert!(result.kept_chars > 0);
        assert!(result.discarded_chars > 0);
        assert!(result.kept_rate > 0.0 && result.kept_rate < 1.0);
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
    fn test_annotations_rename() {
        let base = "    foo(old_name)\n";
        let predicted = "    foo(new_name)\n";
        let final_text = "    foo(new_name)\n";
        let result = compute_kept_rate(base, predicted, final_text);

        assert_eq!(result.predicted_new_chars, "new_name".len());
        assert_eq!(result.token_annotations.len(), tokenize(predicted).len());

        for (&token, &annotation) in tokenize(predicted).iter().zip(&result.token_annotations) {
            if token == "new_name" {
                assert_eq!(annotation, TokenAnnotation::Kept);
            } else {
                assert_eq!(annotation, TokenAnnotation::Context);
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

        let eprintln_index = predicted_tokens
            .iter()
            .position(|&token| token == "eprintln")
            .expect("eprintln token not found");

        for annotation in &result.token_annotations[..eprintln_index] {
            assert_eq!(*annotation, TokenAnnotation::Context);
        }

        assert_eq!(
            &result.token_annotations[eprintln_index..=eprintln_index + 10],
            &[
                TokenAnnotation::Kept,
                TokenAnnotation::Kept,
                TokenAnnotation::Kept,
                TokenAnnotation::Kept,
                TokenAnnotation::Discarded,
                TokenAnnotation::Discarded,
                TokenAnnotation::Discarded,
                TokenAnnotation::Discarded,
                TokenAnnotation::Kept,
                TokenAnnotation::Kept,
                TokenAnnotation::Kept,
            ]
        );
        assert_eq!(
            result.token_annotations.last(),
            Some(&TokenAnnotation::Context)
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
