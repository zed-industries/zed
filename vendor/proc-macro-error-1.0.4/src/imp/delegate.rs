//! This implementation uses [`proc_macro::Diagnostic`], nightly only.

use std::cell::Cell;

use proc_macro::{Diagnostic as PDiag, Level as PLevel};

use crate::{
    abort_now, check_correctness,
    diagnostic::{Diagnostic, Level, SuggestionKind},
};

pub fn abort_if_dirty() {
    check_correctness();
    if IS_DIRTY.with(|c| c.get()) {
        abort_now()
    }
}

pub(crate) fn cleanup() -> Vec<Diagnostic> {
    IS_DIRTY.with(|c| c.set(false));
    vec![]
}

pub(crate) fn emit_diagnostic(diag: Diagnostic) {
    let Diagnostic {
        level,
        span_range,
        msg,
        suggestions,
        children,
    } = diag;

    let span = span_range.collapse().unwrap();

    let level = match level {
        Level::Warning => PLevel::Warning,
        Level::Error => {
            IS_DIRTY.with(|c| c.set(true));
            PLevel::Error
        }
        _ => unreachable!(),
    };

    let mut res = PDiag::spanned(span, level, msg);

    for (kind, msg, span) in suggestions {
        res = match (kind, span) {
            (SuggestionKind::Note, Some(span_range)) => {
                res.span_note(span_range.collapse().unwrap(), msg)
            }
            (SuggestionKind::Help, Some(span_range)) => {
                res.span_help(span_range.collapse().unwrap(), msg)
            }
            (SuggestionKind::Note, None) => res.note(msg),
            (SuggestionKind::Help, None) => res.help(msg),
        }
    }

    for (span_range, msg) in children {
        let span = span_range.collapse().unwrap();
        res = res.span_error(span, msg);
    }

    res.emit()
}

thread_local! {
    static IS_DIRTY: Cell<bool> = Cell::new(false);
}
