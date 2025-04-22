use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assertions {
    pub success: Vec<String>,
    pub failure: Vec<String>,
    pub max: Option<usize>,
}

impl Assertions {
    pub fn new(max: Option<usize>) -> Self {
        Assertions {
            success: Vec::new(),
            failure: Vec::new(),
            max,
        }
    }

    pub fn total_count(&self) -> usize {
        self.run_count().max(self.max.unwrap_or(0))
    }

    pub fn run_count(&self) -> usize {
        self.success.len() + self.failure.len()
    }

    pub fn success_percentage(&self) -> f32 {
        if self.total_count() == 0 {
            0.0
        } else {
            (self.success.len() as f32 / self.total_count() as f32) * 100.0
        }
    }
}

const ASSERTIONS_WIDTH: usize = 50;
const RESULTS_WIDTH: usize = 8;

impl Display for Assertions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Do nothing if no assertions
        if self.total_count() == 0 {
            return Ok(());
        }

        // Write table top border
        writeln!(
            f,
            "┌─{}─┬─{}─┐",
            "─".repeat(ASSERTIONS_WIDTH),
            "─".repeat(RESULTS_WIDTH)
        )?;

        // Write header row
        writeln!(
            f,
            "│ {:^ASSERTIONS_WIDTH$} │ {:^RESULTS_WIDTH$} │",
            "Assertion", "Result"
        )?;

        // Write header-data separator
        writeln!(
            f,
            "├─{}─┼─{}─┤",
            "─".repeat(ASSERTIONS_WIDTH),
            "─".repeat(RESULTS_WIDTH)
        )?;

        // Print successful assertions
        for assertion in &self.success {
            writeln!(
                f,
                "│ {:<ASSERTIONS_WIDTH$} │ {} │",
                truncate_assertion(assertion),
                "\x1b[32m✔︎ Passed\x1b[0m"
            )?;
        }

        // Print failed assertions
        for assertion in &self.failure {
            writeln!(
                f,
                "│ {:<ASSERTIONS_WIDTH$} │ {} │",
                truncate_assertion(assertion),
                "\x1b[31m✗ Failed\x1b[0m"
            )?;
        }

        // Write table bottom border
        writeln!(
            f,
            "└─{}─┴─{}─┘",
            "─".repeat(ASSERTIONS_WIDTH),
            "─".repeat(RESULTS_WIDTH)
        )?;

        // Write summary
        writeln!(
            f,
            "\n{} assertion{} failed, {} passed ({}%)",
            self.failure.len(),
            if self.failure.len() == 1 { "" } else { "s" },
            self.success.len(),
            self.success_percentage().round(),
        )?;

        if let Some(max) = self.max {
            let missing = max - self.run_count();

            if missing > 0 {
                writeln!(f, "\n{} assertions didn't run.", missing)?;
            }
        }

        Ok(())
    }
}

fn truncate_assertion(assertion: &str) -> String {
    if assertion.len() <= ASSERTIONS_WIDTH {
        assertion.to_string()
    } else {
        format!("{}…", &assertion[..ASSERTIONS_WIDTH - 1])
    }
}
