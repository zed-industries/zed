use std::path::Path;

use zeta_prompt::ZetaPromptInput;

pub fn compute_prediction_reversal_ratio(
    prompt_inputs: &ZetaPromptInput,
    predicted_content: &str,
    cursor_path: &Path,
) -> f32 {
    edit_prediction_metrics::compute_prediction_reversal_ratio_from_history(
        prompt_inputs.cursor_excerpt.as_ref(),
        &prompt_inputs.events,
        prompt_inputs.excerpt_start_row,
        predicted_content,
        cursor_path,
    )
}
