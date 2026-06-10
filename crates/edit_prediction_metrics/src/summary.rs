use serde::Serialize;

use crate::patch_metrics::ClassificationMetrics;
use crate::prediction_score::PredictionScore;

#[derive(Clone, Copy, Debug, Default)]
pub struct QaSummaryData {
    pub reverts_edits: Option<bool>,
    pub confidence: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
pub struct PredictionSummaryInput<'a> {
    pub score: &'a PredictionScore,
    pub qa: Option<QaSummaryData>,
    pub retrieved_context_bytes: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SummaryJson {
    pub total_examples: usize,
    pub avg_delta_chr_f: f32,
    pub delta_chr_f_beta: f64,
    pub delta_chr_f_true_positives: usize,
    pub delta_chr_f_false_positives: usize,
    pub delta_chr_f_false_negatives: usize,
    pub delta_chr_f_precision: f64,
    pub delta_chr_f_recall: f64,
    pub avg_braces_disbalance: f32,
    pub exact_lines_true_positives: usize,
    pub exact_lines_false_positives: usize,
    pub exact_lines_false_negatives: usize,
    pub exact_lines_precision: f64,
    pub exact_lines_recall: f64,
    pub exact_lines_f1: f64,
    pub avg_reversal_ratio: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_avg_reverts_edits: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_avg_confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_exact_match_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_avg_distance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_total_evaluated: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_editable_region_rate: Option<f32>,
    pub isolated_whitespace_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_kept_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_recall_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_kept_chars: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_correctly_deleted_chars: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discarded_chars: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_editable_context_lines_precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_editable_context_lines_recall: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_editable_context_lines_f1: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_context_lines_tp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_context_lines_fp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_context_lines_fn: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_editable_context_files_precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_editable_context_files_recall: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_editable_context_files_f1: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_context_files_tp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_context_files_fp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_context_files_fn: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_retrieved_context_bytes: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_retrieved_context_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_context_examples: Option<usize>,
}

pub fn compute_summary<'a>(
    predictions: impl IntoIterator<Item = PredictionSummaryInput<'a>>,
) -> SummaryJson {
    let mut all_delta_chr_f_scores = Vec::new();
    let mut all_reversal_ratios = Vec::new();
    let mut braces_disbalance_sum: usize = 0;
    let mut total_delta_chr_f = ClassificationMetrics::default();
    let mut total_delta_chr_f_precision = 0.0;
    let mut total_delta_chr_f_recall = 0.0;
    let mut delta_chr_f_beta = 0.0;
    let mut total_exact_lines = ClassificationMetrics::default();
    let mut total_scores: usize = 0;
    let mut qa_reverts_count: usize = 0;
    let mut qa_reverts_total: usize = 0;
    let mut qa_confidence_sum: u64 = 0;
    let mut qa_confidence_count: usize = 0;
    let mut cursor_exact_matches: usize = 0;
    let mut cursor_total: usize = 0;
    let mut cursor_distance_sum: usize = 0;
    let mut cursor_distance_count: usize = 0;
    let mut wrong_editable_region_count: usize = 0;
    let mut wrong_editable_region_total: usize = 0;
    let mut isolated_whitespace_count: usize = 0;
    let mut kept_rate_sum: f64 = 0.0;
    let mut kept_rate_count: usize = 0;
    let mut kept_chars_total: usize = 0;
    let mut kept_chars_count: usize = 0;
    let mut correctly_deleted_chars_total: usize = 0;
    let mut correctly_deleted_chars_count: usize = 0;
    let mut discarded_chars_total: usize = 0;
    let mut discarded_chars_count: usize = 0;
    let mut recall_rate_sum: f64 = 0.0;
    let mut recall_rate_count: usize = 0;
    let mut editable_context_lines_precision_sum: f64 = 0.0;
    let mut editable_context_lines_recall_sum: f64 = 0.0;
    let mut editable_context_lines_f1_sum: f64 = 0.0;
    let mut editable_context_files_precision_sum: f64 = 0.0;
    let mut editable_context_files_recall_sum: f64 = 0.0;
    let mut editable_context_files_f1_sum: f64 = 0.0;
    let mut editable_context_coverage_count: usize = 0;
    let mut editable_context_lines_tp: usize = 0;
    let mut editable_context_lines_fp: usize = 0;
    let mut editable_context_lines_fn: usize = 0;
    let mut editable_context_files_tp: usize = 0;
    let mut editable_context_files_fp: usize = 0;
    let mut editable_context_files_fn: usize = 0;
    let mut retrieved_context_bytes_total: usize = 0;
    let mut retrieved_context_bytes_count: usize = 0;

    for prediction in predictions {
        let score = prediction.score;

        all_delta_chr_f_scores.push(score.delta_chr_f);
        all_reversal_ratios.push(score.reversal_ratio);
        total_scores += 1;
        braces_disbalance_sum += score.braces_disbalance;
        total_delta_chr_f.accumulate(&score.delta_chr_f_counts());
        total_delta_chr_f_precision += score.delta_chr_f_precision;
        total_delta_chr_f_recall += score.delta_chr_f_recall;
        delta_chr_f_beta = score.delta_chr_f_beta;
        total_exact_lines.accumulate(&score.exact_lines_counts());

        if let Some(qa) = prediction.qa {
            if let Some(reverts) = qa.reverts_edits {
                qa_reverts_total += 1;
                if reverts {
                    qa_reverts_count += 1;
                }
            }
            if let Some(confidence) = qa.confidence {
                qa_confidence_sum += confidence as u64;
                qa_confidence_count += 1;
            }
        }

        if let Some(wrong) = score.wrong_editable_region {
            wrong_editable_region_total += 1;
            if wrong {
                wrong_editable_region_count += 1;
            }
        }

        if score.has_isolated_whitespace_changes {
            isolated_whitespace_count += 1;
        }

        if let Some(kept_rate) = score.kept_rate {
            kept_rate_sum += kept_rate;
            kept_rate_count += 1;
        }
        if let Some(kept_chars) = score.kept_chars {
            kept_chars_total += kept_chars;
            kept_chars_count += 1;
        }
        if let Some(correctly_deleted_chars) = score.correctly_deleted_chars {
            correctly_deleted_chars_total += correctly_deleted_chars;
            correctly_deleted_chars_count += 1;
        }
        if let Some(discarded_chars) = score.discarded_chars {
            discarded_chars_total += discarded_chars;
            discarded_chars_count += 1;
        }
        if let Some(recall_rate) = score.recall_rate {
            recall_rate_sum += recall_rate;
            recall_rate_count += 1;
        }
        if let Some(retrieved_context_bytes) = prediction.retrieved_context_bytes {
            retrieved_context_bytes_total += retrieved_context_bytes;
            retrieved_context_bytes_count += 1;
        }

        if let Some(coverage) = &score.editable_context_coverage {
            editable_context_lines_precision_sum += coverage.lines_precision;
            editable_context_lines_recall_sum += coverage.lines_recall;
            editable_context_lines_f1_sum += coverage.lines_f1;
            editable_context_files_precision_sum += coverage.files_precision;
            editable_context_files_recall_sum += coverage.files_recall;
            editable_context_files_f1_sum += coverage.files_f1;
            editable_context_coverage_count += 1;
            editable_context_lines_tp += coverage.lines_tp;
            editable_context_lines_fp += coverage.lines_fp;
            editable_context_lines_fn += coverage.lines_fn;
            editable_context_files_tp += coverage.files_tp;
            editable_context_files_fp += coverage.files_fp;
            editable_context_files_fn += coverage.files_fn;
        }

        if let Some(exact_match) = score.cursor_exact_match {
            cursor_total += 1;
            if exact_match {
                cursor_exact_matches += 1;
            }
        }
        if let Some(distance) = score.cursor_distance {
            cursor_distance_sum += distance;
            cursor_distance_count += 1;
        }
    }

    let avg_delta_chr_f = if all_delta_chr_f_scores.is_empty() {
        0.0
    } else {
        all_delta_chr_f_scores.iter().sum::<f32>() / all_delta_chr_f_scores.len() as f32
    };

    let avg_reversal_ratio = if all_reversal_ratios.is_empty() {
        0.0
    } else {
        all_reversal_ratios.iter().sum::<f32>() / all_reversal_ratios.len() as f32
    };

    let avg_braces_disbalance = if total_scores == 0 {
        0.0
    } else {
        braces_disbalance_sum as f32 / total_scores as f32
    };

    let qa_avg_reverts_edits = if qa_reverts_total > 0 {
        Some(qa_reverts_count as f32 / qa_reverts_total as f32)
    } else {
        None
    };

    let qa_avg_confidence = if qa_confidence_count > 0 {
        Some(qa_confidence_sum as f32 / qa_confidence_count as f32)
    } else {
        None
    };

    let cursor_exact_match_rate = if cursor_total > 0 {
        Some(cursor_exact_matches as f32 / cursor_total as f32)
    } else {
        None
    };

    let cursor_avg_distance = if cursor_distance_count > 0 {
        Some(cursor_distance_sum as f32 / cursor_distance_count as f32)
    } else {
        None
    };

    let cursor_total_evaluated = if cursor_total > 0 {
        Some(cursor_total)
    } else {
        None
    };

    let wrong_editable_region_rate = if wrong_editable_region_total > 0 {
        Some(wrong_editable_region_count as f32 / wrong_editable_region_total as f32)
    } else {
        None
    };

    let isolated_whitespace_rate = if total_scores > 0 {
        Some(isolated_whitespace_count as f32 / total_scores as f32)
    } else {
        None
    };

    let avg_kept_rate = if kept_rate_count > 0 {
        Some(kept_rate_sum / kept_rate_count as f64)
    } else {
        None
    };

    let avg_recall_rate = if recall_rate_count > 0 {
        Some(recall_rate_sum / recall_rate_count as f64)
    } else {
        None
    };

    let total_kept_chars = if kept_chars_count > 0 {
        Some(kept_chars_total)
    } else {
        None
    };

    let total_correctly_deleted_chars = if correctly_deleted_chars_count > 0 {
        Some(correctly_deleted_chars_total)
    } else {
        None
    };

    let total_discarded_chars = if discarded_chars_count > 0 {
        Some(discarded_chars_total)
    } else {
        None
    };

    let avg_editable_context_lines_precision = if editable_context_coverage_count > 0 {
        Some(editable_context_lines_precision_sum / editable_context_coverage_count as f64)
    } else {
        None
    };
    let avg_editable_context_lines_recall = if editable_context_coverage_count > 0 {
        Some(editable_context_lines_recall_sum / editable_context_coverage_count as f64)
    } else {
        None
    };
    let avg_editable_context_lines_f1 = if editable_context_coverage_count > 0 {
        Some(editable_context_lines_f1_sum / editable_context_coverage_count as f64)
    } else {
        None
    };
    let editable_context_lines_tp = if editable_context_coverage_count > 0 {
        Some(editable_context_lines_tp)
    } else {
        None
    };
    let editable_context_lines_fp = if editable_context_coverage_count > 0 {
        Some(editable_context_lines_fp)
    } else {
        None
    };
    let editable_context_lines_fn = if editable_context_coverage_count > 0 {
        Some(editable_context_lines_fn)
    } else {
        None
    };
    let avg_editable_context_files_precision = if editable_context_coverage_count > 0 {
        Some(editable_context_files_precision_sum / editable_context_coverage_count as f64)
    } else {
        None
    };
    let avg_editable_context_files_recall = if editable_context_coverage_count > 0 {
        Some(editable_context_files_recall_sum / editable_context_coverage_count as f64)
    } else {
        None
    };
    let avg_editable_context_files_f1 = if editable_context_coverage_count > 0 {
        Some(editable_context_files_f1_sum / editable_context_coverage_count as f64)
    } else {
        None
    };
    let editable_context_files_tp = if editable_context_coverage_count > 0 {
        Some(editable_context_files_tp)
    } else {
        None
    };
    let editable_context_files_fp = if editable_context_coverage_count > 0 {
        Some(editable_context_files_fp)
    } else {
        None
    };
    let editable_context_files_fn = if editable_context_coverage_count > 0 {
        Some(editable_context_files_fn)
    } else {
        None
    };
    let avg_retrieved_context_bytes = if retrieved_context_bytes_count > 0 {
        Some(retrieved_context_bytes_total as f64 / retrieved_context_bytes_count as f64)
    } else {
        None
    };
    let total_retrieved_context_bytes = if retrieved_context_bytes_count > 0 {
        Some(retrieved_context_bytes_total)
    } else {
        None
    };
    let retrieved_context_examples = if retrieved_context_bytes_count > 0 {
        Some(retrieved_context_bytes_count)
    } else {
        None
    };

    SummaryJson {
        total_examples: total_scores,
        avg_delta_chr_f,
        delta_chr_f_beta,
        delta_chr_f_true_positives: total_delta_chr_f.true_positives,
        delta_chr_f_false_positives: total_delta_chr_f.false_positives,
        delta_chr_f_false_negatives: total_delta_chr_f.false_negatives,
        delta_chr_f_precision: if total_scores == 0 {
            0.0
        } else {
            total_delta_chr_f_precision / total_scores as f64
        },
        delta_chr_f_recall: if total_scores == 0 {
            0.0
        } else {
            total_delta_chr_f_recall / total_scores as f64
        },
        avg_braces_disbalance,
        exact_lines_true_positives: total_exact_lines.true_positives,
        exact_lines_false_positives: total_exact_lines.false_positives,
        exact_lines_false_negatives: total_exact_lines.false_negatives,
        exact_lines_precision: total_exact_lines.precision(),
        exact_lines_recall: total_exact_lines.recall(),
        exact_lines_f1: total_exact_lines.f1(),
        avg_reversal_ratio,
        qa_avg_reverts_edits,
        qa_avg_confidence,
        cursor_exact_match_rate,
        cursor_avg_distance,
        cursor_total_evaluated,
        wrong_editable_region_rate,
        isolated_whitespace_rate,
        avg_kept_rate,
        avg_recall_rate,
        total_kept_chars,
        total_correctly_deleted_chars,
        total_discarded_chars,
        avg_editable_context_lines_precision,
        avg_editable_context_lines_recall,
        avg_editable_context_lines_f1,
        editable_context_lines_tp,
        editable_context_lines_fp,
        editable_context_lines_fn,
        avg_editable_context_files_precision,
        avg_editable_context_files_recall,
        avg_editable_context_files_f1,
        editable_context_files_tp,
        editable_context_files_fp,
        editable_context_files_fn,
        avg_retrieved_context_bytes,
        total_retrieved_context_bytes,
        retrieved_context_examples,
    }
}
