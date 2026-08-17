//! Current `backtrace::trace` + collector based backtrace implementation
//!
//! This implementation may eventually be extracted into a separate `tokio-taskdump` crate.

use std::ptr;

use crate::runtime::task::trace::{trace_with, Trace, TraceMeta};

/// Capture using the default `backtrace::trace`-based implementation.
#[inline(never)]
pub(super) fn capture<F, R>(f: F) -> (R, Trace)
where
    F: FnOnce() -> R,
{
    let mut trace = Trace::empty();

    let result = trace_with(f, |meta| trace_leaf(meta, &mut trace));

    (result, trace)
}

/// Capture a backtrace via `backtrace::trace` and collect it into `trace`.
pub(crate) fn trace_leaf(meta: &TraceMeta, trace: &mut Trace) {
    let mut frames: Vec<backtrace::BacktraceFrame> = vec![];
    let mut above_leaf = false;

    if let Some(root_addr) = meta.root_addr {
        backtrace::trace(|frame| {
            let below_root = !ptr::eq(frame.symbol_address(), root_addr);

            if above_leaf && below_root {
                frames.push(frame.to_owned().into());
            }

            if ptr::eq(frame.symbol_address(), meta.trace_leaf_addr) {
                above_leaf = true;
            }

            below_root
        });
    }
    trace.push_backtrace(frames);
}
