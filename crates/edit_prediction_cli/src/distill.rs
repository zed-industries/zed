use anyhow::{Result, anyhow};
use std::mem;

use crate::example::Example;

pub async fn run_distill(example: &mut Example) -> Result<()> {
    let [prediction]: [_; 1] =
        mem::take(&mut example.predictions)
            .try_into()
            .map_err(|preds: Vec<_>| {
                anyhow!(
                    "Example has {} predictions, but it should have exactly one",
                    preds.len()
                )
            })?;

    example.spec.expected_patch = prediction.actual_patch;
    example.prompt = None;
    example.predictions = Vec::new();
    example.score = Vec::new();
    Ok(())
}
