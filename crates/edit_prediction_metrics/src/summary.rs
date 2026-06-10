use serde::Serialize;

use crate::jumps::LineFileClassification;
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
    pub cursor_exact_matches: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_avg_distance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_total_evaluated: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_editable_region_rate: Option<f32>,
    pub isolated_whitespace_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolated_whitespace_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_kept_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kept_rate_examples: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_recall_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall_rate_examples: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_kept_chars: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_correctly_deleted_chars: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discarded_chars: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_context_examples: Option<usize>,
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
    pub jump_location_examples: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_jump_location_lines_precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_jump_location_lines_recall: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_jump_location_lines_f1: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_location_lines_tp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_location_lines_fp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_location_lines_fn: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_jump_location_files_precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_jump_location_files_recall: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_jump_location_files_f1: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_location_files_tp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_location_files_fp: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_location_files_fn: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_retrieved_context_bytes: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_retrieved_context_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_context_examples: Option<usize>,
}

/// Mean of observed values; `None` when nothing was observed.
#[derive(Default)]
struct MeanAggregate {
    sum: f64,
    count: usize,
}

impl MeanAggregate {
    fn add(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
    }

    fn mean(&self) -> Option<f64> {
        (self.count > 0).then(|| self.sum / self.count as f64)
    }

    fn mean_f32(&self) -> Option<f32> {
        self.mean().map(|mean| mean as f32)
    }

    fn count(&self) -> Option<usize> {
        (self.count > 0).then_some(self.count)
    }
}

/// Fraction of observations that were hits; `None` when nothing was observed.
#[derive(Default)]
struct RateAggregate {
    hits: usize,
    total: usize,
}

impl RateAggregate {
    fn add(&mut self, hit: bool) {
        self.total += 1;
        if hit {
            self.hits += 1;
        }
    }

    fn rate(&self) -> Option<f32> {
        (self.total > 0).then(|| self.hits as f32 / self.total as f32)
    }

    fn hits(&self) -> Option<usize> {
        (self.total > 0).then_some(self.hits)
    }

    fn total(&self) -> Option<usize> {
        (self.total > 0).then_some(self.total)
    }
}

/// Sum of observed values; `None` when nothing was observed.
#[derive(Default)]
struct SumAggregate {
    sum: usize,
    count: usize,
}

impl SumAggregate {
    fn add(&mut self, value: usize) {
        self.sum += value;
        self.count += 1;
    }

    fn total(&self) -> Option<usize> {
        (self.count > 0).then_some(self.sum)
    }

    fn mean(&self) -> Option<f64> {
        (self.count > 0).then(|| self.sum as f64 / self.count as f64)
    }

    fn count(&self) -> Option<usize> {
        (self.count > 0).then_some(self.count)
    }
}

/// Macro-averaged precision/recall/F1 plus pooled (micro) TP/FP/FN counts.
#[derive(Default)]
struct PrfAggregate {
    count: usize,
    precision_sum: f64,
    recall_sum: f64,
    f1_sum: f64,
    counts: ClassificationMetrics,
}

impl PrfAggregate {
    fn add(&mut self, precision: f64, recall: f64, f1: f64, counts: &ClassificationMetrics) {
        self.count += 1;
        self.precision_sum += precision;
        self.recall_sum += recall;
        self.f1_sum += f1;
        self.counts.accumulate(counts);
    }

    fn avg_precision(&self) -> Option<f64> {
        (self.count > 0).then(|| self.precision_sum / self.count as f64)
    }

    fn avg_recall(&self) -> Option<f64> {
        (self.count > 0).then(|| self.recall_sum / self.count as f64)
    }

    fn avg_f1(&self) -> Option<f64> {
        (self.count > 0).then(|| self.f1_sum / self.count as f64)
    }

    fn true_positives(&self) -> Option<usize> {
        (self.count > 0).then_some(self.counts.true_positives)
    }

    fn false_positives(&self) -> Option<usize> {
        (self.count > 0).then_some(self.counts.false_positives)
    }

    fn false_negatives(&self) -> Option<usize> {
        (self.count > 0).then_some(self.counts.false_negatives)
    }

    fn count(&self) -> Option<usize> {
        (self.count > 0).then_some(self.count)
    }
}

/// Aggregates the line- and file-level halves of a [`LineFileClassification`].
#[derive(Default)]
struct LineFileAggregate {
    lines: PrfAggregate,
    files: PrfAggregate,
}

impl LineFileAggregate {
    fn add(&mut self, classification: &LineFileClassification) {
        self.lines.add(
            classification.lines_precision,
            classification.lines_recall,
            classification.lines_f1,
            &classification.lines_counts(),
        );
        self.files.add(
            classification.files_precision,
            classification.files_recall,
            classification.files_f1,
            &classification.files_counts(),
        );
    }

    fn count(&self) -> Option<usize> {
        self.lines.count()
    }
}

pub fn compute_summary<'a>(
    predictions: impl IntoIterator<Item = PredictionSummaryInput<'a>>,
) -> SummaryJson {
    let mut total_scores: usize = 0;
    let mut delta_chr_f_sum: f32 = 0.0;
    let mut reversal_ratio_sum: f32 = 0.0;
    let mut braces_disbalance_sum: usize = 0;
    let mut total_delta_chr_f = ClassificationMetrics::default();
    let mut delta_chr_f_precision_sum = 0.0;
    let mut delta_chr_f_recall_sum = 0.0;
    let mut delta_chr_f_beta = 0.0;
    let mut total_exact_lines = ClassificationMetrics::default();
    let mut qa_reverts = RateAggregate::default();
    let mut qa_confidence = MeanAggregate::default();
    let mut cursor_exact = RateAggregate::default();
    let mut cursor_distance = MeanAggregate::default();
    let mut wrong_editable_region = RateAggregate::default();
    let mut isolated_whitespace = RateAggregate::default();
    let mut kept_rate = MeanAggregate::default();
    let mut recall_rate = MeanAggregate::default();
    let mut kept_chars = SumAggregate::default();
    let mut correctly_deleted_chars = SumAggregate::default();
    let mut discarded_chars = SumAggregate::default();
    let mut editable_context = LineFileAggregate::default();
    let mut jump_location = LineFileAggregate::default();
    let mut retrieved_context_bytes = SumAggregate::default();

    for prediction in predictions {
        let score = prediction.score;

        total_scores += 1;
        delta_chr_f_sum += score.delta_chr_f;
        reversal_ratio_sum += score.reversal_ratio;
        braces_disbalance_sum += score.braces_disbalance;
        total_delta_chr_f.accumulate(&score.delta_chr_f_counts());
        delta_chr_f_precision_sum += score.delta_chr_f_precision;
        delta_chr_f_recall_sum += score.delta_chr_f_recall;
        delta_chr_f_beta = score.delta_chr_f_beta;
        total_exact_lines.accumulate(&score.exact_lines_counts());

        if let Some(qa) = prediction.qa {
            if let Some(reverts) = qa.reverts_edits {
                qa_reverts.add(reverts);
            }
            if let Some(confidence) = qa.confidence {
                qa_confidence.add(confidence as f64);
            }
        }

        if let Some(wrong) = score.wrong_editable_region {
            wrong_editable_region.add(wrong);
        }
        isolated_whitespace.add(score.has_isolated_whitespace_changes);

        if let Some(value) = score.kept_rate {
            kept_rate.add(value);
        }
        if let Some(value) = score.recall_rate {
            recall_rate.add(value);
        }
        if let Some(value) = score.kept_chars {
            kept_chars.add(value);
        }
        if let Some(value) = score.correctly_deleted_chars {
            correctly_deleted_chars.add(value);
        }
        if let Some(value) = score.discarded_chars {
            discarded_chars.add(value);
        }
        if let Some(value) = prediction.retrieved_context_bytes {
            retrieved_context_bytes.add(value);
        }

        if let Some(coverage) = &score.editable_context_coverage {
            editable_context.add(coverage);
        }
        if let Some(location) = &score.jump_location {
            jump_location.add(location);
        }

        if let Some(exact_match) = score.cursor_exact_match {
            cursor_exact.add(exact_match);
        }
        if let Some(distance) = score.cursor_distance {
            cursor_distance.add(distance as f64);
        }
    }

    SummaryJson {
        total_examples: total_scores,
        avg_delta_chr_f: if total_scores == 0 {
            0.0
        } else {
            delta_chr_f_sum / total_scores as f32
        },
        delta_chr_f_beta,
        delta_chr_f_true_positives: total_delta_chr_f.true_positives,
        delta_chr_f_false_positives: total_delta_chr_f.false_positives,
        delta_chr_f_false_negatives: total_delta_chr_f.false_negatives,
        delta_chr_f_precision: if total_scores == 0 {
            0.0
        } else {
            delta_chr_f_precision_sum / total_scores as f64
        },
        delta_chr_f_recall: if total_scores == 0 {
            0.0
        } else {
            delta_chr_f_recall_sum / total_scores as f64
        },
        avg_braces_disbalance: if total_scores == 0 {
            0.0
        } else {
            braces_disbalance_sum as f32 / total_scores as f32
        },
        exact_lines_true_positives: total_exact_lines.true_positives,
        exact_lines_false_positives: total_exact_lines.false_positives,
        exact_lines_false_negatives: total_exact_lines.false_negatives,
        exact_lines_precision: total_exact_lines.precision(),
        exact_lines_recall: total_exact_lines.recall(),
        exact_lines_f1: total_exact_lines.f1(),
        avg_reversal_ratio: if total_scores == 0 {
            0.0
        } else {
            reversal_ratio_sum / total_scores as f32
        },
        qa_avg_reverts_edits: qa_reverts.rate(),
        qa_avg_confidence: qa_confidence.mean_f32(),
        cursor_exact_match_rate: cursor_exact.rate(),
        cursor_exact_matches: cursor_exact.hits(),
        cursor_avg_distance: cursor_distance.mean_f32(),
        cursor_total_evaluated: cursor_exact.total(),
        wrong_editable_region_rate: wrong_editable_region.rate(),
        isolated_whitespace_rate: isolated_whitespace.rate(),
        isolated_whitespace_count: isolated_whitespace.hits(),
        avg_kept_rate: kept_rate.mean(),
        kept_rate_examples: kept_rate.count(),
        avg_recall_rate: recall_rate.mean(),
        recall_rate_examples: recall_rate.count(),
        total_kept_chars: kept_chars.total(),
        total_correctly_deleted_chars: correctly_deleted_chars.total(),
        total_discarded_chars: discarded_chars.total(),
        editable_context_examples: editable_context.count(),
        avg_editable_context_lines_precision: editable_context.lines.avg_precision(),
        avg_editable_context_lines_recall: editable_context.lines.avg_recall(),
        avg_editable_context_lines_f1: editable_context.lines.avg_f1(),
        editable_context_lines_tp: editable_context.lines.true_positives(),
        editable_context_lines_fp: editable_context.lines.false_positives(),
        editable_context_lines_fn: editable_context.lines.false_negatives(),
        avg_editable_context_files_precision: editable_context.files.avg_precision(),
        avg_editable_context_files_recall: editable_context.files.avg_recall(),
        avg_editable_context_files_f1: editable_context.files.avg_f1(),
        editable_context_files_tp: editable_context.files.true_positives(),
        editable_context_files_fp: editable_context.files.false_positives(),
        editable_context_files_fn: editable_context.files.false_negatives(),
        jump_location_examples: jump_location.count(),
        avg_jump_location_lines_precision: jump_location.lines.avg_precision(),
        avg_jump_location_lines_recall: jump_location.lines.avg_recall(),
        avg_jump_location_lines_f1: jump_location.lines.avg_f1(),
        jump_location_lines_tp: jump_location.lines.true_positives(),
        jump_location_lines_fp: jump_location.lines.false_positives(),
        jump_location_lines_fn: jump_location.lines.false_negatives(),
        avg_jump_location_files_precision: jump_location.files.avg_precision(),
        avg_jump_location_files_recall: jump_location.files.avg_recall(),
        avg_jump_location_files_f1: jump_location.files.avg_f1(),
        jump_location_files_tp: jump_location.files.true_positives(),
        jump_location_files_fp: jump_location.files.false_positives(),
        jump_location_files_fn: jump_location.files.false_negatives(),
        avg_retrieved_context_bytes: retrieved_context_bytes.mean(),
        total_retrieved_context_bytes: retrieved_context_bytes.total(),
        retrieved_context_examples: retrieved_context_bytes.count(),
    }
}
