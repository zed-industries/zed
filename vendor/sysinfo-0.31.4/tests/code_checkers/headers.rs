// Take a look at the license at the top of the repository in the LICENSE file.

use super::utils::{show_error, TestResult};
use std::path::Path;

pub fn check_license_header(content: &str, p: &Path) -> TestResult {
    let mut lines = content.lines();
    let next = lines.next();
    let header = "// Take a look at the license at the top of the repository in the LICENSE file.";

    match next {
        Some(s) if s == header => {
            let next = lines.next();
            match next {
                Some("") => TestResult {
                    nb_tests: 1,
                    nb_errors: 0,
                },
                Some(s) => {
                    show_error(
                        p,
                        &format!("Expected empty line after license header, found `{s}`"),
                    );
                    TestResult {
                        nb_tests: 1,
                        nb_errors: 1,
                    }
                }
                None => {
                    show_error(p, "This file should very likely not exist...");
                    TestResult {
                        nb_tests: 1,
                        nb_errors: 1,
                    }
                }
            }
        }
        Some(s) => {
            show_error(
                p,
                &format!(
                    "Expected license header at the top of the file (`{header}`), found: `{s}`",
                ),
            );
            TestResult {
                nb_tests: 1,
                nb_errors: 1,
            }
        }
        None => {
            show_error(p, "This (empty?) file should very likely not exist...");
            TestResult {
                nb_tests: 1,
                nb_errors: 1,
            }
        }
    }
}
