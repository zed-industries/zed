#![allow(unstable_name_collisions)] // Intentional!

use proc_macro::Span;
use std::path::{Path, PathBuf};

/// Trait providing a fallback method for `Span` methods.
#[allow(unused)]
pub(crate) trait FallbackSpan {
    fn line(&self) -> usize;
    fn column(&self) -> usize;
    fn file(&self) -> String;
    fn local_file(&self) -> Option<PathBuf>;
    fn source_text(&self) -> Option<String>;
}

/// Implement it for references of Span using autoref, giving it lower priority
/// than inherent methods.
impl FallbackSpan for &Span {
    #[inline(always)]
    fn line(&self) -> usize {
        0
    }
    #[inline(always)]
    fn column(&self) -> usize {
        0
    }
    #[inline(always)]
    fn file(&self) -> String {
        String::new()
    }
    #[inline(always)]
    fn local_file(&self) -> Option<PathBuf> {
        None
    }
    #[inline(always)]
    fn source_text(&self) -> Option<String> {
        None
    }
}

#[allow(unused, clippy::needless_borrow)]
pub(crate) fn has_1_88_span_methods(span: Span) -> bool {
    let line_val = (&span).line();

    // If it returns 0, we are on an older compiler.
    line_val != 0
}

#[inline(always)]
pub(crate) fn file(span: &Span) -> String {
    // cargo expand may return a full path here.
    let file = span.file();
    let path = Path::new(&file);
    if Path::new(&file).is_absolute() {
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    } else {
        file
    }
}

#[inline(always)]
pub(crate) fn line(span: &Span) -> usize {
    span.line()
}

#[inline(always)]
pub(crate) fn local_file(span: &Span) -> Option<PathBuf> {
    span.local_file()
}

#[inline(always)]
pub(crate) fn column(span: &Span) -> usize {
    span.column()
}

#[inline(always)]
pub(crate) fn source_text(span: &Span) -> Option<String> {
    span.source_text()
}
