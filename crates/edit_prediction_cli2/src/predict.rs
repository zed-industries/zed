use std::sync::Arc;

use gpui::AsyncApp;

use crate::{PredictionProvider, example::Example, headless::EpAppState};

pub async fn run_predictions(
    example: &mut Example,
    provider: Option<PredictionProvider>,
    app_state: Arc<EpAppState>,
    mut cx: AsyncApp,
) {
}
