//! Utilities for evaluation and benchmarking.

use std::{
    collections::HashMap,
    io::Write as _,
    sync::{Arc, mpsc},
};

fn report_progress(evaluated_count: usize, failed_count: usize, iterations: usize) {
    let passed_count = evaluated_count - failed_count;
    let passed_ratio = if evaluated_count == 0 {
        0.0
    } else {
        passed_count as f64 / evaluated_count as f64
    };
    print!(
        "\r\x1b[KEvaluated {}/{} ({:.2}% passed)",
        evaluated_count,
        iterations,
        passed_ratio * 100.0
    );
    std::io::stdout().flush().unwrap();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutcomeKind {
    Passed,
    Failed,
    Error(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvalOutput {
    pub data: String,
    pub mismatched_tags: usize,
    pub tags: usize,
    pub outcome_kind: OutcomeKind,
}

pub fn eval(
    iterations: usize,
    expected_pass_ratio: f32,
    mismatched_tag_threshold: f32,
    evalf: &(dyn Fn(mpsc::Sender<EvalOutput>) + Send),
) {
    let mut evaluated_count = 0;
    let mut failed_count = 0;
    report_progress(evaluated_count, failed_count, iterations);

    let (tx, rx) = mpsc::channel();

    let executor = gpui::background_executor();
    let semaphore = Arc::new(smol::lock::Semaphore::new(32));
    for _ in 1..iterations {
        let eval = eval.clone();
        let tx = tx.clone();
        let semaphore = semaphore.clone();
        executor
            .spawn(async move {
                let _guard = semaphore.acquire().await;
                evalf(tx);
            })
            .detach();
    }
    drop(tx);

    let mut failed_evals = Vec::new();
    let mut errored_evals = HashMap::default();
    let mut eval_outputs = Vec::new();
    let mut cumulative_mismatched_tags = 0usize;
    let mut cumulative_tags = 0usize;
    while let Ok(output) = rx.recv() {
        if matches!(
            output.outcome_kind,
            OutcomeKind::Passed | OutcomeKind::Failed
        ) {
            cumulative_mismatched_tags += output.cumulative_mismatched_tags;
            cumulative_tags += output.cumulative_tags;
            eval_outputs.push(output.clone());
        }

        match output.outcome_kind {
            OutcomeKind::Passed => {}
            OutcomeKind::Failed => {
                failed_count += 1;
                failed_evals.push(output);
            }
            OutcomeKind::Error(s) => {
                failed_count += 1;
                *errored_evals.entry(s).or_insert(0) += 1;
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

        let mut failed_evals = failed_evals.into_iter().collect::<Vec<_>>();

        for failed in failed_evals {
            println!("Eval failed");
            println!("{}", failed.data);
        }

        panic!(
            "Actual pass ratio: {}\nExpected pass ratio: {}",
            actual_pass_ratio, expected_pass_ratio
        );
    }

    let mismatched_tag_ratio = cumulative_mismatched_tags as f32 / cumulative_tags as f32;
    if mismatched_tag_ratio > mismatched_tag_threshold {
        for eval_output in eval_outputs {
            println!("{}", eval_output.data);
        }
        panic!("Too many mismatched tags: {:?}", cumulative_mismatched_tags);
    }
}
