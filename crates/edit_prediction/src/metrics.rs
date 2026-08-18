pub use edit_prediction_metrics::KeptRateResult;

pub use edit_prediction_metrics::compute_kept_rate;
use language::SyntaxLayer;

pub fn count_tree_sitter_errors<'a>(layers: impl Iterator<Item = SyntaxLayer<'a>>) -> usize {
    edit_prediction_metrics::count_tree_sitter_errors(layers.map(|layer| layer.node()))
}
