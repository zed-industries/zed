# eval_utils

Utilities for evaluation and benchmarking in Zed.

## Overview

This crate provides common data structures and utilities for running evaluations and collecting metrics. It's designed to be used across different evaluation tasks and benchmarking scenarios.

## Usage

Add `eval_utils` to your crate's dependencies in `Cargo.toml`:

```toml
[dependencies]
eval_utils.workspace = true
```

## Examples

### Creating and collecting metrics

```rust
use eval_utils::{EvalMetric, EvalResults};

let mut results = EvalResults::new();

// Add individual metrics
results.add_metric(EvalMetric::new("accuracy", 0.95));
results.add_metric(EvalMetric::new("precision", 0.92));
results.add_metric(EvalMetric::new("recall", 0.88));

// Retrieve a specific metric
if let Some(metric) = results.get_metric("accuracy") {
    println!("Accuracy: {}", metric.value);
}
```

### Serialization

The data structures support serialization via `serde`:

```rust
use eval_utils::EvalResults;

let results = EvalResults::new();
// ... add metrics ...

// Serialize to JSON
let json = serde_json::to_string(&results)?;

// Deserialize from JSON
let loaded_results: EvalResults = serde_json::from_str(&json)?;
```

## License

GPL-3.0-or-later