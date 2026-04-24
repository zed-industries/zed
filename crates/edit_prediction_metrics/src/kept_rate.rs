use crate::tokenize::tokenize;
use serde::Serialize;

const MAX_DIRTY_LENGTH_DELTA_CHARS: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenAnnotation {
    Context,
    Kept,
    Discarded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AnnotatedToken {
    pub token: String,
    pub annotation: TokenAnnotation,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct KeptRateResult {
    /// Characters newly introduced by the candidate
    pub candidate_new_chars: usize,
    /// Characters newly introduced by the reference
    pub reference_new_chars: usize,
    /// Characters from `base` that are deleted by the candidate.
    pub candidate_deleted_chars: usize,
    /// Characters from `base` that are deleted by the reference.
    pub reference_deleted_chars: usize,
    /// Candidate new characters that are also present in the reference.
    pub kept_chars: usize,
    /// Base characters deleted by both the candidate and the reference.
    pub correctly_deleted_chars: usize,
    /// Candidate new characters that are not kept in the reference.
    pub discarded_chars: usize,
    /// Candidate characters treated as unchanged context
    pub context_chars: usize,
    /// Fraction of candidate edit characters that match the reference edit.
    ///
    /// This includes both kept newly introduced characters and correctly
    /// deleted base characters.
    pub kept_rate: f64,
    /// Fraction of reference edit characters covered by the candidate edit.
    ///
    /// This includes both kept newly introduced characters and correctly
    /// deleted base characters.
    pub recall_rate: f64,
    /// Per-token classification for candidate tokens.
    pub token_annotations: Vec<TokenAnnotation>,
}

fn dp_index(width: usize, row: usize, column: usize) -> usize {
    row * width + column
}

/// Fill masks over `a` and `b` using one-sided LCS tie-breaking for each side
/// while sharing a single DP table construction.
fn fill_lcs_keep_masks<T: Eq>(
    a: &[T],
    b: &[T],
    mut keep_a: Option<&mut [bool]>,
    mut keep_b: Option<&mut [bool]>,
) {
    if a.is_empty() || b.is_empty() {
        return;
    }

    if a == b {
        if let Some(keep_a) = keep_a.as_mut() {
            keep_a.fill(true);
        }
        if let Some(keep_b) = keep_b.as_mut() {
            keep_b.fill(true);
        }
        return;
    }

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
        if let Some(keep_a) = keep_a.as_mut() {
            keep_a[index] = true;
        }
        if let Some(keep_b) = keep_b.as_mut() {
            keep_b[index] = true;
        }
    }

    for offset in 0..suffix_len {
        let a_index = a.len() - suffix_len + offset;
        let b_index = b.len() - suffix_len + offset;
        if let Some(keep_a) = keep_a.as_mut() {
            keep_a[a_index] = true;
        }
        if let Some(keep_b) = keep_b.as_mut() {
            keep_b[b_index] = true;
        }
    }

    let a_mid = &a[prefix_len..a.len() - suffix_len];
    let b_mid = &b[prefix_len..b.len() - suffix_len];

    if a_mid.is_empty() || b_mid.is_empty() {
        return;
    }

    let row_count = a_mid.len() + 1;
    let column_count = b_mid.len() + 1;
    let mut dp = vec![0u32; row_count * column_count];

    for i in 1..row_count {
        let token_a = &a_mid[i - 1];
        for j in 1..column_count {
            let index = dp_index(column_count, i, j);
            if token_a == &b_mid[j - 1] {
                dp[index] = dp[dp_index(column_count, i - 1, j - 1)] + 1;
            } else {
                let up = dp[dp_index(column_count, i - 1, j)];
                let left = dp[dp_index(column_count, i, j - 1)];
                dp[index] = up.max(left);
            }
        }
    }

    if let Some(keep_a) = keep_a.as_mut() {
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
    }

    if let Some(keep_b) = keep_b.as_mut() {
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
    }
}

fn lcs_keep_mask<T: Eq>(a: &[T], b: &[T]) -> Vec<bool> {
    let mut keep_a = vec![false; a.len()];
    fill_lcs_keep_masks(a, b, Some(&mut keep_a), None);
    keep_a
}

fn lcs_keep_masks<T: Eq>(a: &[T], b: &[T]) -> (Vec<bool>, Vec<bool>) {
    let mut keep_a = vec![false; a.len()];
    let mut keep_b = vec![false; b.len()];
    fill_lcs_keep_masks(a, b, Some(&mut keep_a), Some(&mut keep_b));
    (keep_a, keep_b)
}

#[derive(Debug, Clone)]
struct ComparisonUnit {
    text: String,
    token_start: usize,
    token_end: usize,
}

fn is_identifier_token(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|character| character.is_alphanumeric() || character == '_')
}

fn build_comparison_units(tokens: &[&str]) -> Vec<ComparisonUnit> {
    let mut units = Vec::new();
    let mut index = 0;

    while index < tokens.len() {
        let token_start = index;

        if is_identifier_token(tokens[index]) {
            let mut text = String::new();

            while index < tokens.len() && is_identifier_token(tokens[index]) {
                text.push_str(tokens[index]);
                index += 1;
            }

            units.push(ComparisonUnit {
                text,
                token_start,
                token_end: index,
            });
        } else {
            units.push(ComparisonUnit {
                text: tokens[index].to_string(),
                token_start,
                token_end: index + 1,
            });
            index += 1;
        }
    }

    units
}

fn analyze_masked_units<'a>(
    units: &'a [ComparisonUnit],
    mask: &[bool],
) -> (Vec<&'a str>, usize, usize) {
    let mut unmasked_units = Vec::with_capacity(units.len());
    let mut unmasked_chars = 0;
    let mut masked_chars = 0;

    for (unit, &is_masked) in units.iter().zip(mask.iter()) {
        if is_masked {
            masked_chars += unit.text.len();
        } else {
            unmasked_units.push(unit.text.as_str());
            unmasked_chars += unit.text.len();
        }
    }

    (unmasked_units, unmasked_chars, masked_chars)
}

fn count_unmasked_unit_chars(units: &[ComparisonUnit], mask: &[bool]) -> usize {
    units
        .iter()
        .zip(mask.iter())
        .filter_map(|(unit, &is_masked)| (!is_masked).then_some(unit.text.len()))
        .sum()
}

fn should_bail_for_dirty_final(base: &str, candidate: &str, reference: &str) -> bool {
    let candidate_delta_chars = candidate.len().abs_diff(base.len());
    let reference_delta_chars = reference.len().abs_diff(base.len());
    candidate_delta_chars.abs_diff(reference_delta_chars) > MAX_DIRTY_LENGTH_DELTA_CHARS
}

pub fn compute_kept_rate(base: &str, candidate: &str, reference: &str) -> KeptRateResult {
    if base == candidate && candidate == reference {
        let candidate_tokens = tokenize(candidate);
        let context_chars = candidate_tokens.iter().map(|token| token.len()).sum();
        return KeptRateResult {
            candidate_new_chars: 0,
            reference_new_chars: 0,
            candidate_deleted_chars: 0,
            reference_deleted_chars: 0,
            kept_chars: 0,
            correctly_deleted_chars: 0,
            discarded_chars: 0,
            context_chars,
            kept_rate: 1.0,
            recall_rate: 1.0,
            token_annotations: vec![TokenAnnotation::Context; candidate_tokens.len()],
        };
    }

    if should_bail_for_dirty_final(base, candidate, reference) {
        let candidate_new_chars = candidate.len().abs_diff(base.len());
        let reference_new_chars = reference.len().abs_diff(base.len());
        return KeptRateResult {
            candidate_new_chars,
            reference_new_chars,
            candidate_deleted_chars: 0,
            reference_deleted_chars: 0,
            kept_chars: 0,
            correctly_deleted_chars: 0,
            discarded_chars: candidate_new_chars,
            context_chars: 0,
            kept_rate: 0.0,
            recall_rate: 0.0,
            token_annotations: vec![TokenAnnotation::Discarded; tokenize(candidate).len()],
        };
    }

    let base_tokens = tokenize(base);
    let candidate_tokens = tokenize(candidate);
    let reference_tokens = tokenize(reference);

    let candidate_units = build_comparison_units(&candidate_tokens);
    let base_units = build_comparison_units(&base_tokens);
    let reference_units = build_comparison_units(&reference_tokens);

    let candidate_unit_texts: Vec<&str> = candidate_units
        .iter()
        .map(|unit| unit.text.as_str())
        .collect();
    let base_unit_texts: Vec<&str> = base_units.iter().map(|unit| unit.text.as_str()).collect();
    let reference_unit_texts: Vec<&str> = reference_units
        .iter()
        .map(|unit| unit.text.as_str())
        .collect();

    let (candidate_base_mask, base_candidate_mask) =
        lcs_keep_masks(&candidate_unit_texts, &base_unit_texts);
    let (stripped_candidate, candidate_new_chars, context_chars) =
        analyze_masked_units(&candidate_units, &candidate_base_mask);

    let (reference_base_mask, base_reference_mask) =
        lcs_keep_masks(&reference_unit_texts, &base_unit_texts);
    let (stripped_reference, reference_new_chars, _) =
        analyze_masked_units(&reference_units, &reference_base_mask);

    let keep_mask = lcs_keep_mask(&stripped_candidate, &stripped_reference);

    let kept_chars: usize = stripped_candidate
        .iter()
        .zip(keep_mask.iter())
        .filter_map(|(&token, &is_kept)| is_kept.then_some(token.len()))
        .sum();

    let candidate_deleted_chars = count_unmasked_unit_chars(&base_units, &base_candidate_mask);
    let reference_deleted_chars = count_unmasked_unit_chars(&base_units, &base_reference_mask);
    let correctly_deleted_chars: usize = base_units
        .iter()
        .zip(base_candidate_mask.iter().zip(base_reference_mask.iter()))
        .filter_map(|(unit, (&in_candidate, &in_reference))| {
            (!in_candidate && !in_reference).then_some(unit.text.len())
        })
        .sum();

    let discarded_chars = candidate_new_chars - kept_chars;
    let matched_edit_chars = kept_chars + correctly_deleted_chars;
    let candidate_edit_chars = candidate_new_chars + candidate_deleted_chars;
    let reference_edit_chars = reference_new_chars + reference_deleted_chars;

    let kept_rate = if candidate_edit_chars == 0 {
        if reference_edit_chars == 0 { 1.0 } else { 0.0 }
    } else {
        matched_edit_chars as f64 / candidate_edit_chars as f64
    };

    let recall_rate = if reference_edit_chars == 0 {
        if candidate_edit_chars == 0 { 1.0 } else { 0.0 }
    } else {
        matched_edit_chars as f64 / reference_edit_chars as f64
    };

    let token_annotations = {
        let mut token_annotations = vec![TokenAnnotation::Context; candidate_tokens.len()];
        let mut new_index = 0;

        for (unit_index, unit) in candidate_units.iter().enumerate() {
            let annotation = if candidate_base_mask[unit_index] {
                TokenAnnotation::Context
            } else {
                let annotation = if keep_mask[new_index] {
                    TokenAnnotation::Kept
                } else {
                    TokenAnnotation::Discarded
                };
                new_index += 1;
                annotation
            };

            for token_index in unit.token_start..unit.token_end {
                token_annotations[token_index] = annotation;
            }
        }

        token_annotations
    };

    KeptRateResult {
        candidate_new_chars,
        reference_new_chars,
        candidate_deleted_chars,
        reference_deleted_chars,
        kept_chars,
        correctly_deleted_chars,
        discarded_chars,
        context_chars,
        kept_rate,
        recall_rate,
        token_annotations,
    }
}

pub fn annotate_kept_rate_tokens(
    base: &str,
    candidate: &str,
    reference: &str,
) -> Vec<AnnotatedToken> {
    let result = compute_kept_rate(base, candidate, reference);
    tokenize(candidate)
        .into_iter()
        .zip(result.token_annotations)
        .map(|(token, annotation)| AnnotatedToken {
            token: token.to_string(),
            annotation,
        })
        .collect()
}

#[cfg(test)]
mod test_kept_rate {
    use super::*;
    use indoc::indoc;

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
        assert_eq!(a_mask, lcs_keep_mask(&a, &b));
        assert_eq!(b_mask, lcs_keep_mask(&b, &a));
    }

    #[test]
    fn test_rate_extremes() {
        let no_change = compute_kept_rate("foo bar", "foo bar", "foo bar");
        assert!((no_change.kept_rate - 1.0).abs() < 1e-6);
        assert!((no_change.recall_rate - 1.0).abs() < 1e-6);
        assert_eq!(no_change.candidate_new_chars, 0);
        assert!(
            no_change
                .token_annotations
                .iter()
                .all(|&annotation| annotation == TokenAnnotation::Context)
        );

        let accepted = compute_kept_rate("old", "new", "new");
        assert!((accepted.kept_rate - 1.0).abs() < 1e-6);
        assert!((accepted.recall_rate - 1.0).abs() < 1e-6);

        let discarded = compute_kept_rate("old", "old", "new");
        assert!((discarded.kept_rate - 0.0).abs() < 1e-6);
        assert!((discarded.recall_rate - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_pure_addition() {
        let kept = compute_kept_rate("", "brand new line\n", "brand new line\n");
        assert_eq!(kept.kept_chars, kept.candidate_new_chars);
        assert!(
            kept.token_annotations
                .iter()
                .all(|&annotation| annotation == TokenAnnotation::Kept)
        );

        let discarded =
            compute_kept_rate("", "brand new line\n", "something completely different\n");
        assert!(discarded.kept_chars < discarded.candidate_new_chars);
    }

    #[test]
    fn test_decoy_when_base_excluded() {
        let base = "    decoy.when(mock_sync_hardware_api.sp()).then_return(SpeedStatus.IDLE)\n";
        let candidate = "    decoy.when(mock_sync_module_hardware.speed_status).then_return(SpeedStatus.IDLE)\n";
        let reference = "    decoy.when(mock_sync_module_hardware.speed_status).then_return(SpeedStatus.IDLE)\n";
        let result = compute_kept_rate(base, candidate, reference);
        let expected_new = "mock_sync_module_hardware".len() + "speed_status".len();
        assert_eq!(result.candidate_new_chars, expected_new);
        assert!(result.correctly_deleted_chars > 0);
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
        assert!((result.recall_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_missing_deletion() {
        let base = indoc! {"
            fn example() {
                epr
        "};
        let candidate = indoc! {r#"
            fn example() {
                epr
            eprintln!("");
        "#};
        let reference = indoc! {r#"
            fn example() {
            eprintln!("");
        "#};

        let result = compute_kept_rate(base, candidate, reference);
        assert!((result.kept_rate - (14.0 / 15.0)).abs() < 1e-6);
        assert_eq!(result.kept_chars, 14);
        assert_eq!(result.discarded_chars, 1);
    }

    #[test]
    fn test_empty_prediction() {
        let result = compute_kept_rate("old line\n", "", "new line\n");
        assert_eq!(result.candidate_new_chars, 0);
        assert!(result.candidate_deleted_chars > 0);
        assert!(result.correctly_deleted_chars > 0);
        assert!(result.correctly_deleted_chars < result.candidate_deleted_chars);
        assert!(result.kept_rate > 0.0 && result.kept_rate < 1.0);
        assert!(result.recall_rate > 0.0 && result.recall_rate < 1.0);
    }

    #[test]
    fn test_partial_kept() {
        let result = compute_kept_rate("old\n", "alpha\nbeta\ngamma\n", "alpha\ngamma\n");
        assert!(result.kept_chars > 0);
        assert!(result.discarded_chars > 0);
        assert!(result.kept_rate > 0.0 && result.kept_rate < 1.0);
    }

    #[test]
    fn test_bails_for_dirty_final() {
        let base = indoc! {"
            fn example() {
                work();
            }
        "};
        let candidate = indoc! {"
            fn example() {
                work();
                predicted();
            }
        "};
        let reference = format!(
            "fn example() {{\n    work();\n    {}\n}}\n",
            "settled();\n    ".repeat(MAX_DIRTY_LENGTH_DELTA_CHARS / 8 + 64)
        );

        let result = compute_kept_rate(base, candidate, &reference);
        assert_eq!(result.kept_rate, 0.0);
        assert_eq!(result.recall_rate, 0.0);
        assert_eq!(result.kept_chars, 0);
        assert_eq!(result.discarded_chars, result.candidate_new_chars);
    }

    #[test]
    fn test_eprintln_token_alignment() {
        let base = indoc! {"
            fn example() {
                epr
        "};
        let candidate = indoc! {r#"
            fn example() {
                eprintln!("hello world!");
        "#};
        let reference = indoc! {r#"
            fn example() {
                eprintln!("");
        "#};

        let result = compute_kept_rate(base, candidate, reference);
        assert!(result.discarded_chars > 0);
        assert!(result.kept_chars > 0);
        assert!(result.kept_rate > 0.0 && result.kept_rate < 1.0);
        assert_eq!(result.kept_chars, 14);
        assert_eq!(result.discarded_chars, 12);
    }

    #[test]
    fn test_kept_rate_treats_unchanged_stale_text_as_context() {
        let base = indoc! {"
            a=fomr
            b=old
        "};
        let candidate = indoc! {"
            a=formula;
            b=old
        "};
        let reference = indoc! {"
            a=formula;
            b=new
        "};

        let result = compute_kept_rate(base, candidate, reference);
        let candidate_tokens = tokenize(candidate);

        assert_eq!(result.candidate_new_chars, "formula".len() + ";".len());
        assert_eq!(result.kept_chars, "formula".len() + ";".len());
        assert_eq!(result.discarded_chars, 0);
        assert_eq!(result.candidate_deleted_chars, "fomr".len());
        assert_eq!(result.correctly_deleted_chars, "fomr".len());
        assert!((result.kept_rate - 1.0).abs() < 1e-6);
        assert!((result.recall_rate - (2.0 / 3.0)).abs() < 1e-6);

        let old_index = candidate_tokens
            .iter()
            .position(|&token| token == "old")
            .expect("old token not found");
        assert_eq!(
            result.token_annotations[old_index],
            TokenAnnotation::Context
        );
    }

    #[test]
    fn test_annotations_rename() {
        let base = "    foo(old_name)\n";
        let candidate = "    foo(new_name)\n";
        let reference = "    foo(new_name)\n";
        let result = compute_kept_rate(base, candidate, reference);

        assert_eq!(result.candidate_new_chars, "new_name".len());
        assert_eq!(result.candidate_deleted_chars, "old_name".len());
        assert_eq!(result.reference_deleted_chars, "old_name".len());
        assert_eq!(result.correctly_deleted_chars, "old_name".len());
        assert!((result.recall_rate - 1.0).abs() < 1e-6);
        assert_eq!(result.token_annotations.len(), tokenize(candidate).len());

        for (&token, &annotation) in tokenize(candidate).iter().zip(&result.token_annotations) {
            if matches!(token, "new" | "_" | "name") {
                assert_eq!(annotation, TokenAnnotation::Kept);
            } else {
                assert_eq!(annotation, TokenAnnotation::Context);
            }
        }
    }

    #[test]
    fn test_annotations_eprintln_coloring() {
        let base = indoc! {"
            fn example() {
                epr
        "};
        let candidate = indoc! {r#"
            fn example() {
                eprintln!("hello world!");
        "#};
        let reference = indoc! {r#"
            fn example() {
                eprintln!("");
        "#};
        let result = compute_kept_rate(base, candidate, reference);
        let candidate_tokens = tokenize(candidate);

        let eprintln_index = candidate_tokens
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
        let candidate = "foo + foo + prediction_token + foo + foo\n".repeat(16);
        let reference = "foo + foo + kept_token + foo + foo\n".repeat(16);
        let result = compute_kept_rate(&base, &candidate, &reference);

        assert_eq!(result.kept_chars, 0);
        assert_eq!(result.correctly_deleted_chars, "foo".len() * 16);
        assert_eq!(result.discarded_chars, result.candidate_new_chars);
        assert_eq!(result.candidate_new_chars, "prediction_token".len() * 16);
        assert!(result.kept_rate > 0.0);
        assert!(result.recall_rate > 0.0);
    }
}
