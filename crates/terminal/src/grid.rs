//! TODO kb docs

use std::cmp;

use alacritty_terminal::{
    grid::Dimensions,
    index::{Column, Line},
    sync::FairMutex,
    Term,
};

use crate::{TaskState, ZedListener};

const SEPARATOR: &str = "√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√🚀√√√";

pub(super) fn display_task_results(
    term: &FairMutex<Term<ZedListener>>,
    task: &mut TaskState,
    error_code: Option<i32>,
) {
    let escaped_full_label = task.full_label.replace("\r\n", "\r").replace('\n', "\r");
    let task_line = match error_code {
        Some(0) => {
            format!("Task `{escaped_full_label}` finished successfully")
        }
        Some(error_code) => {
            format!("Task `{escaped_full_label}` finished with non-zero error code: {error_code}")
        }
        None => {
            format!("Task `{escaped_full_label}` finished")
        }
    };
    let escaped_command_label = task.command_label.replace("\r\n", "\r").replace('\n', "\r");
    let command_line = format!("Command: '{escaped_command_label}'");
    append_text_to_grid(term, &["", SEPARATOR, &task_line, command_line.trim_end()]);
}

fn append_text_to_grid(term: &FairMutex<Term<ZedListener>>, text_lines: &[&str]) {
    let mut term = term.lock();
    let grid_mut = term.grid_mut();

    let max_columns = grid_mut.columns();
    let new_rows_for_message = text_lines
        .iter()
        .map(|text_line| {
            let mut grid_lines = 0;
            let mut grid_chars = text_line.chars().count();
            while grid_chars > 0 {
                grid_chars = grid_chars.saturating_sub(max_columns);
                grid_lines += 1;
            }
            grid_lines
        })
        .sum::<usize>();

    let bottommost_line = grid_mut.bottommost_line();
    let current_position = grid_mut.cursor.point;
    let mut first_clear_line = bottommost_line.max(current_position.line);
    loop {
        if grid_mut[first_clear_line]
            .into_iter()
            .any(|cell| cell.c != ' ')
        {
            first_clear_line += 1;
            break;
        }
        if first_clear_line == Line(0) {
            break;
        }
        first_clear_line -= 1;
    }
    let first_clear_line = first_clear_line;
    let new_lines_len = new_rows_for_message + 1;

    let mut rows_to_scroll = new_lines_len;
    let rows_to_grow = match bottommost_line.cmp(&first_clear_line) {
        cmp::Ordering::Less => {
            let difference = (first_clear_line - bottommost_line).0 as usize;
            rows_to_scroll += difference;
            rows_to_scroll
        }
        cmp::Ordering::Equal => rows_to_scroll,
        cmp::Ordering::Greater => {
            let difference = (bottommost_line - first_clear_line).0 as usize;
            rows_to_scroll = rows_to_scroll.saturating_sub(difference);
            rows_to_scroll
        }
    };
    if rows_to_grow > 0 {
        let screen_size = grid_mut.screen_lines();
        grid_mut.resize(
            false,
            bottommost_line.0 as usize + rows_to_grow + screen_size,
            max_columns,
        );
        grid_mut.scroll_display(alacritty_terminal::grid::Scroll::Delta(
            -(new_lines_len as i32),
        ));
    }

    let mut current_line = first_clear_line;
    for text_line in text_lines {
        let mut current_column = 0;
        for c in text_line.chars() {
            if current_column >= max_columns {
                current_line += 1;
                current_column = 0;
            }
            grid_mut[current_line][Column(current_column)].c = c;
            current_column += 1;
        }
        current_line += 1;
    }
    let _ = current_position;
    grid_mut.cursor.point.line = first_clear_line + new_lines_len;
    grid_mut.cursor.point.column = Column(0);
}
