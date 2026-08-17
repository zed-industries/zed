// Take a look at the license at the top of the repository in the LICENSE file.

use super::utils::{show_error, TestResult};
use std::path::Path;

fn check_supported_signals_decl<'a>(lines: &mut impl Iterator<Item = &'a str>, p: &Path) -> usize {
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("const SUPPORTED_SIGNALS: &'static [Signal]") {
            if trimmed != "const SUPPORTED_SIGNALS: &[Signal] = supported_signals();" {
                show_error(
                    p,
                    "SUPPORTED_SIGNALS should be declared using `supported_signals()`",
                );
                return 1;
            }
            break;
        }
    }
    0
}

fn check_kill_decl<'a>(lines: &mut impl Iterator<Item = &'a str>, p: &Path) -> usize {
    let mut errors = 0;

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.starts_with("fn kill(") {
            show_error(p, "`Process::kill` should not be reimplemented!");
            errors += 1;
        } else if trimmed.starts_with("fn kill_with(") {
            if let Some(line) = lines.next() {
                let trimmed = line.trim();
                if trimmed.ends_with("crate::sys::convert_signal(signal)?;") || trimmed == "None" {
                    continue;
                } else {
                    show_error(p, "`Process::kill_with` should use `convert_signal`");
                    errors += 1;
                }
            }
        }
    }
    errors
}

pub fn check_signals(content: &str, p: &Path) -> TestResult {
    let mut lines = content.lines();
    let mut res = TestResult {
        nb_tests: 0,
        nb_errors: 0,
    };

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.starts_with("impl SystemInner {") {
            res.nb_tests += 1;
            res.nb_errors += check_supported_signals_decl(&mut lines, p);
        } else if trimmed.starts_with("impl ProcessInner {") {
            res.nb_tests += 1;
            res.nb_errors += check_kill_decl(&mut lines, p);
        }
    }
    res
}
