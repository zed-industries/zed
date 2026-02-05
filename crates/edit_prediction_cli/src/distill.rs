use anyhow::Result;

use crate::{PredictionProvider, example::Example};

pub async fn run_distill(example: &mut Example) -> Result<()> {
    let has_repair = example
        .predictions
        .iter()
        .find(|p| p.provider == PredictionProvider::Repair);
    let predictions = if let Some(has_repair) = has_repair {
        vec![has_repair]
    } else {
        example.predictions.iter().collect()
    };

    let expected_patches = predictions
        .into_iter()
        .filter_map(|p| Some((p.actual_patch.clone()?, p.actual_cursor_offset)))
        .collect();

    example
        .spec
        .set_expected_patches_with_cursor_positions(expected_patches);
    example.prompt = None;
    example.predictions = Vec::new();
    example.score = Vec::new();
    example.qa = Vec::new();
    Ok(())
}
