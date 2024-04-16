//! TODO kb docs

use alacritty_terminal::{index::Column, sync::FairMutex, Term};

use crate::{TaskState, ZedListener};

const TASK_DELIMITER: &str = "⏵ ";

pub(super) fn display_task_results(
    term: &FairMutex<Term<ZedListener>>,
    task: &mut TaskState,
    error_code: Option<i32>,
) {
    let escaped_full_label = task.full_label.replace("\r\n", "\r").replace('\n', "\r");
    let task_line = match error_code {
        Some(0) => {
            format!("{TASK_DELIMITER}Task `{escaped_full_label}` finished successfully")
        }
        Some(error_code) => {
            format!("{TASK_DELIMITER}Task `{escaped_full_label}` finished with non-zero error code: {error_code}")
        }
        None => {
            format!("{TASK_DELIMITER}Task `{escaped_full_label}` finished")
        }
    };
    let escaped_command_label = task.command_label.replace("\r\n", "\r").replace('\n', "\r");
    let command_line = format!("{TASK_DELIMITER}Command: '{escaped_command_label}'");
    append_text_to_term(&mut term.lock(), &[&task_line, command_line.trim_end()]);
}

fn append_text_to_term(term: &mut Term<ZedListener>, text_lines: &[&str]) {
    use alacritty_terminal::vte::ansi::Handler;
    term.newline();
    term.grid_mut().cursor.point.column = Column(0);
    for line in text_lines {
        for c in line.chars() {
            term.input(c);
        }
        term.newline();
        term.grid_mut().cursor.point.column = Column(0);
    }
}
