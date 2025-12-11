use std::mem;

use crate::example::Example;

pub async fn run_distill(example: &mut Example) {
    let [prediction]: [_; 1] = mem::take(&mut example.predictions)
        .try_into()
        .expect("Run predict first with a single repetition");

    example.expected_patch = prediction.actual_patch;
    example.prompt = None;
    example.predictions = Vec::new();
    example.score = Vec::new();
}
