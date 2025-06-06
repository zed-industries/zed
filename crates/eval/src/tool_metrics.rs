use collections::HashMap;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ToolMetrics {
    pub use_counts: HashMap<Arc<str>, u32>,
    pub failure_counts: HashMap<Arc<str>, u32>,
}

impl ToolMetrics {
    pub fn insert(&mut self, tool_name: Arc<str>, succeeded: bool) {
        *self.use_counts.entry(tool_name.clone()).or_insert(0) += 1;
        if !succeeded {
            *self.failure_counts.entry(tool_name).or_insert(0) += 1;
        }
    }

    pub fn merge(&mut self, other: &ToolMetrics) {
        for (tool_name, use_count) in &other.use_counts {
            *self.use_counts.entry(tool_name.clone()).or_insert(0) += use_count;
        }
        for (tool_name, failure_count) in &other.failure_counts {
            *self.failure_counts.entry(tool_name.clone()).or_insert(0) += failure_count;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.use_counts.is_empty() && self.failure_counts.is_empty()
    }
}

impl Display for ToolMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut failure_rates: Vec<(Arc<str>, f64)> = Vec::new();

        for (tool_name, use_count) in &self.use_counts {
            let failure_count = self.failure_counts.get(tool_name).cloned().unwrap_or(0);
            if *use_count > 0 {
                let failure_rate = failure_count as f64 / *use_count as f64;
                failure_rates.push((tool_name.clone(), failure_rate));
            }
        }

        // Sort by failure rate descending
        failure_rates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Table dimensions
        let tool_width = 30;
        let count_width = 10;
        let rate_width = 10;

        // Write table top border
        writeln!(
            f,
            "┌{}┬{}┬{}┬{}┐",
            "─".repeat(tool_width),
            "─".repeat(count_width),
            "─".repeat(count_width),
            "─".repeat(rate_width)
        )?;

        // Write header row
        writeln!(
            f,
            "│{:^30}│{:^10}│{:^10}│{:^10}│",
            "Tool", "Uses", "Failures", "Rate"
        )?;

        // Write header-data separator
        writeln!(
            f,
            "├{}┼{}┼{}┼{}┤",
            "─".repeat(tool_width),
            "─".repeat(count_width),
            "─".repeat(count_width),
            "─".repeat(rate_width)
        )?;

        // Write data rows
        for (tool_name, failure_rate) in failure_rates {
            let use_count = self.use_counts.get(&tool_name).cloned().unwrap_or(0);
            let failure_count = self.failure_counts.get(&tool_name).cloned().unwrap_or(0);
            writeln!(
                f,
                "│{:<30}│{:^10}│{:^10}│{:^10}│",
                tool_name,
                use_count,
                failure_count,
                format!("{}%", (failure_rate * 100.0).round())
            )?;
        }

        // Write table bottom border
        writeln!(
            f,
            "└{}┴{}┴{}┴{}┘",
            "─".repeat(tool_width),
            "─".repeat(count_width),
            "─".repeat(count_width),
            "─".repeat(rate_width)
        )?;

        Ok(())
    }
}
