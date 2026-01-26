use anyhow::Result;

use crate::{PredictionProvider, example::Example};

pub async fn run_distill(example: &mut Example) -> Result<()> {
    let has_repair = example
        .predictions
        .iter()
        .find(|p| p.provider == PredictionProvider::Repair);
    let predictions = if has_repair.is_some() {
        vec![has_repair.unwrap()]
    } else {
        example.predictions.iter().collect()
    };

    let expected_patches = predictions
        .into_iter()
        .filter_map(|p| p.actual_patch.clone())
        .collect();

    example.spec.expected_patches = expected_patches;
    example.prompt = None;
    example.predictions = Vec::new();
    example.score = Vec::new();
    example.qa = Vec::new();
    Ok(())
}
