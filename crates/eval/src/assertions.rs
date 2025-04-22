use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fmt::{self};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AssertionsReport {
    pub assertions: Vec<Assertion>,
    pub max: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assertion {
    pub id: String,
    pub result: Result<AssertionResult, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssertionResult {
    pub analysis: Option<String>,
    pub passed: bool,
}

impl AssertionsReport {
    pub fn new(max: Option<usize>) -> Self {
        AssertionsReport {
            assertions: Vec::new(),
            max,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.assertions.is_empty()
    }

    pub fn total_count(&self) -> usize {
        self.run_count().max(self.max.unwrap_or(0))
    }

    pub fn run_count(&self) -> usize {
        self.assertions.len()
    }

    pub fn passed_count(&self) -> usize {
        self.assertions
            .iter()
            .filter(|a| a.result.as_ref().map_or(false, |result| result.passed))
            .count()
    }

    pub fn passed_percentage(&self) -> f32 {
        if self.total_count() == 0 {
            0.0
        } else {
            (self.passed_count() as f32 / self.total_count() as f32) * 100.0
        }
    }
}

const ROUND_WIDTH: usize = "Round".len();
const ASSERTIONS_WIDTH: usize = 42;
const RESULTS_WIDTH: usize = 8;

pub fn print_table_header() {
    println!(
        "┌─{}─┬─{}─┬─{}─┐",
        "─".repeat(ROUND_WIDTH),
        "─".repeat(ASSERTIONS_WIDTH),
        "─".repeat(RESULTS_WIDTH)
    );

    println!(
        "│ {:^ROUND_WIDTH$} │ {:^ASSERTIONS_WIDTH$} │ {:^RESULTS_WIDTH$} │",
        "Round", "Assertion", "Result"
    );

    println!(
        "├─{}─┼─{}─┼─{}─┤",
        "─".repeat(ROUND_WIDTH),
        "─".repeat(ASSERTIONS_WIDTH),
        "─".repeat(RESULTS_WIDTH)
    )
}

pub fn display_error_row(f: &mut String, round: usize, error: String) -> fmt::Result {
    let last_two_columns = ASSERTIONS_WIDTH + RESULTS_WIDTH;
    writeln!(
        f,
        "│ {:^ROUND_WIDTH$} │ {:<last_two_columns$} |",
        round,
        truncate(&error, last_two_columns)
    )
}

pub fn display_table_row(f: &mut String, round: usize, assertion: &Assertion) -> fmt::Result {
    let result = match &assertion.result {
        Ok(result) if result.passed => "\x1b[32m✔︎ Passed\x1b[0m",
        Ok(_) => "\x1b[31m✗ Failed\x1b[0m",
        Err(_) => "\x1b[31m💥 Judge Error\x1b[0m",
    };

    writeln!(
        f,
        "│ {:^ROUND_WIDTH$} │ {:<ASSERTIONS_WIDTH$} │ {:>RESULTS_WIDTH$} │",
        round,
        truncate(&assertion.id, ASSERTIONS_WIDTH),
        result
    )
}

pub fn print_table_round_summary<'a>(
    round: &str,
    reports: impl Iterator<Item = &'a AssertionsReport>,
) {
    let mut passed = 0;
    let mut total = 0;
    for report in reports {
        passed += report.passed_count();
        total += report.total_count();
    }

    println!(
        "│ {:^ROUND_WIDTH$} │ {:<ASSERTIONS_WIDTH$} │ {:>RESULTS_WIDTH$} │",
        round,
        "total",
        format!("{}%", passed as f32 / total as f32 * 100.0)
    )
}

pub fn print_table_footer() {
    println!(
        "└─{}─┴─{}─┴─{}─┘",
        "─".repeat(ROUND_WIDTH),
        "─".repeat(ASSERTIONS_WIDTH),
        "─".repeat(RESULTS_WIDTH)
    )
}

pub fn print_table_divider() {
    println!(
        "├─{}─┼─{}─┼─{}─┤",
        "─".repeat(ROUND_WIDTH),
        "─".repeat(ASSERTIONS_WIDTH),
        "─".repeat(RESULTS_WIDTH)
    )
}

fn truncate(assertion: &str, max_width: usize) -> String {
    if assertion.len() <= max_width {
        assertion.to_string()
    } else {
        let mut end_ix = max_width - 1;
        while !assertion.is_char_boundary(end_ix) {
            end_ix -= 1;
        }
        format!("{}…", &assertion[..end_ix])
    }
}
