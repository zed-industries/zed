//! Utilities for evaluation and benchmarking.

use std::{
    collections::HashMap,
    sync::{Arc, mpsc},
};

fn report_progress(evaluated_count: usize, failed_count: usize, iterations: usize) {
    let passed_count = evaluated_count - failed_count;
    let passed_ratio = if evaluated_count == 0 {
        0.0
    } else {
        passed_count as f64 / evaluated_count as f64
    };
    println!(
        "\r\x1b[KEvaluated {}/{} ({:.2}% passed)",
        evaluated_count,
        iterations,
        passed_ratio * 100.0
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutcomeKind {
    Passed,
    Failed,
    Error,
}

pub trait EvalOutputProcessor {
    type Metadata: 'static + Send;
    fn process(&mut self, output: &EvalOutput<Self::Metadata>);
    fn assert(&mut self);
}

#[derive(Clone, Debug)]
pub struct EvalOutput<M> {
    pub outcome: OutcomeKind,
    pub data: String,
    pub metadata: M,
}

pub struct NoProcessor;
impl EvalOutputProcessor for NoProcessor {
    type Metadata = ();

    fn process(&mut self, _output: &EvalOutput<Self::Metadata>) {}

    fn assert(&mut self) {}
}

pub fn eval<P>(
    iterations: usize,
    expected_pass_ratio: f32,
    mut processor: P,
    evalf: impl Fn() -> EvalOutput<P::Metadata> + Send + Sync + 'static,
) where
    P: EvalOutputProcessor,
{
    let mut evaluated_count = 0;
    let mut failed_count = 0;
    let evalf = Arc::new(evalf);
    report_progress(evaluated_count, failed_count, iterations);

    let (tx, rx) = mpsc::channel();

    let executor = gpui::background_executor();
    let semaphore = Arc::new(smol::lock::Semaphore::new(32));
    let evalf = Arc::new(evalf);
    // Warm the cache once
    let first_output = evalf();
    tx.send(first_output).ok();

    for _ in 1..iterations {
        let tx = tx.clone();
        let semaphore = semaphore.clone();
        let evalf = evalf.clone();
        executor
            .spawn(async move {
                let _guard = semaphore.acquire().await;
                let output = evalf();
                tx.send(output).ok();
            })
            .detach();
    }
    drop(tx);

    let mut failed_evals = Vec::new();
    let mut errored_evals = HashMap::new();
    while let Ok(output) = rx.recv() {
        processor.process(&output);

        match output.outcome {
            OutcomeKind::Passed => {}
            OutcomeKind::Failed => {
                failed_count += 1;
                failed_evals.push(output);
            }
            OutcomeKind::Error => {
                failed_count += 1;
                *errored_evals.entry(output.data).or_insert(0) += 1;
            }
        }

        evaluated_count += 1;
        report_progress(evaluated_count, failed_count, iterations);
    }

    let actual_pass_ratio = (iterations - failed_count) as f32 / iterations as f32;
    println!("Actual pass ratio: {}\n", actual_pass_ratio);
    if actual_pass_ratio < expected_pass_ratio {
        for (error, count) in errored_evals {
            println!("Eval errored {} times. Error: {}", count, error);
        }

        for failed in failed_evals {
            println!("Eval failed");
            println!("{}", failed.data);
        }

        panic!(
            "Actual pass ratio: {}\nExpected pass ratio: {}",
            actual_pass_ratio, expected_pass_ratio
        );
    }

    processor.assert();
}
