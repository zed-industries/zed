#![allow(unused_imports)]

use crate::example::ActualCursor;

pub use edit_prediction_metrics::ClassificationMetrics;
pub use edit_prediction_metrics::Counts;
pub use edit_prediction_metrics::DeltaChrFMetrics;
pub use edit_prediction_metrics::KeptRateResult;
pub use edit_prediction_metrics::TokenChangeCounts;
pub use edit_prediction_metrics::braces_disbalance;
pub use edit_prediction_metrics::compute_kept_rate;
pub use edit_prediction_metrics::count_patch_token_changes;
pub use edit_prediction_metrics::delta_chr_f;
pub use edit_prediction_metrics::delta_chr_f_beta;
pub use edit_prediction_metrics::exact_lines_match;
pub use edit_prediction_metrics::extract_changed_lines_from_diff;
pub use edit_prediction_metrics::is_editable_region_correct;

pub fn has_isolated_whitespace_changes(patch_str: &str, cursor: Option<&ActualCursor>) -> bool {
    edit_prediction_metrics::has_isolated_whitespace_changes(
        patch_str,
        cursor.map(|cursor| cursor.row),
    )
}
