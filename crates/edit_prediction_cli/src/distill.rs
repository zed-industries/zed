use anyhow::Result;
use std::mem;

use crate::example::Example;

pub async fn run_distill(example: &mut Example) -> Result<()> {
    let predictions = mem::take(&mut example.predictions)
        .into_iter()
        .map(|p| p.actual_patch)
        .collect();

    example.spec.expected_patches = predictions;
    example.prompt = None;
    example.predictions = Vec::new();
    example.score = Vec::new();
    Ok(())
}
