use crate::gen::out::{Content, OutFile};
use std::collections::BTreeSet;

#[derive(Default)]
pub(crate) struct Pragma<'a> {
    pub gnu_diagnostic_ignore: BTreeSet<&'a str>,
    pub clang_diagnostic_ignore: BTreeSet<&'a str>,
    pub dollar_in_identifier: bool,
    pub missing_declarations: bool,
    pub return_type_c_linkage: bool,
    pub begin: Content<'a>,
    pub end: Content<'a>,
}

impl<'a> Pragma<'a> {
    pub fn new() -> Self {
        Pragma::default()
    }
}

pub(super) fn write(out: &mut OutFile) {
    let Pragma {
        ref mut gnu_diagnostic_ignore,
        ref mut clang_diagnostic_ignore,
        dollar_in_identifier,
        missing_declarations,
        return_type_c_linkage,
        ref mut begin,
        ref mut end,
    } = out.pragma;

    if dollar_in_identifier {
        clang_diagnostic_ignore.insert("-Wdollar-in-identifier-extension");
    }
    if missing_declarations {
        gnu_diagnostic_ignore.insert("-Wmissing-declarations");
    }
    if return_type_c_linkage {
        clang_diagnostic_ignore.insert("-Wreturn-type-c-linkage");
    }
    let gnu_diagnostic_ignore = &*gnu_diagnostic_ignore;
    let clang_diagnostic_ignore = &*clang_diagnostic_ignore;

    if !gnu_diagnostic_ignore.is_empty() {
        writeln!(begin, "#ifdef __GNUC__");
        if out.header {
            writeln!(begin, "#pragma GCC diagnostic push");
        }
        for diag in gnu_diagnostic_ignore {
            writeln!(begin, "#pragma GCC diagnostic ignored \"{diag}\"");
        }
    }
    if !clang_diagnostic_ignore.is_empty() {
        writeln!(begin, "#ifdef __clang__");
        if out.header && gnu_diagnostic_ignore.is_empty() {
            writeln!(begin, "#pragma clang diagnostic push");
        }
        for diag in clang_diagnostic_ignore {
            writeln!(begin, "#pragma clang diagnostic ignored \"{diag}\"");
        }
        writeln!(begin, "#endif // __clang__");
    }
    if !gnu_diagnostic_ignore.is_empty() {
        writeln!(begin, "#endif // __GNUC__");
    }

    if out.header {
        if !gnu_diagnostic_ignore.is_empty() {
            writeln!(end, "#ifdef __GNUC__");
            writeln!(end, "#pragma GCC diagnostic pop");
            writeln!(end, "#endif // __GNUC__");
        } else if !clang_diagnostic_ignore.is_empty() {
            writeln!(end, "#ifdef __clang__");
            writeln!(end, "#pragma clang diagnostic pop");
            writeln!(end, "#endif // __clang__");
        }
    }
}
