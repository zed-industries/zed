use crate::gen::out::{Content, OutFile};
use crate::syntax::{self, IncludeKind};
use std::ops::{Deref, DerefMut};

/// The complete contents of the "rust/cxx.h" header.
pub static HEADER: &str = include_str!("include/cxx.h");

/// A header to #include.
///
/// The cxxbridge tool does not parse or even require the given paths to exist;
/// they simply go into the generated C++ code as #include lines.
#[derive(Clone, PartialEq, Debug)]
pub struct Include {
    /// The header's path, not including the enclosing quotation marks or angle
    /// brackets.
    pub path: String,
    /// Whether to emit `#include "path"` or `#include <path>`.
    pub kind: IncludeKind,
}

#[derive(Default, PartialEq)]
pub(crate) struct Includes<'a> {
    pub custom: Vec<Include>,
    pub algorithm: bool,
    pub array: bool,
    pub cassert: bool,
    pub cstddef: bool,
    pub cstdint: bool,
    pub cstring: bool,
    pub exception: bool,
    pub functional: bool,
    pub initializer_list: bool,
    pub iterator: bool,
    pub limits: bool,
    pub memory: bool,
    pub new: bool,
    pub ranges: bool,
    pub stdexcept: bool,
    pub string: bool,
    pub string_view: bool,
    pub type_traits: bool,
    pub utility: bool,
    pub vector: bool,
    pub basetsd: bool,
    pub sys_types: bool,
    pub content: Content<'a>,
}

impl<'a> Includes<'a> {
    pub(crate) fn new() -> Self {
        Includes::default()
    }

    pub(crate) fn insert(&mut self, include: impl Into<Include>) {
        self.custom.push(include.into());
    }

    pub(crate) fn has_cxx_header(&self) -> bool {
        self.custom
            .iter()
            .any(|header| header.path == "rust/cxx.h" || header.path == "rust\\cxx.h")
    }
}

pub(super) fn write(out: &mut OutFile) {
    let header = out.header;
    let include = &mut out.include;
    let cxx_header = include.has_cxx_header();
    let out = &mut include.content;

    if header {
        writeln!(out, "#pragma once");
    }

    for include in &include.custom {
        match include.kind {
            IncludeKind::Quoted => {
                writeln!(out, "#include \"{}\"", include.path.escape_default());
            }
            IncludeKind::Bracketed => {
                writeln!(out, "#include <{}>", include.path);
            }
        }
    }

    let Includes {
        custom: _,
        algorithm,
        array,
        cassert,
        cstddef,
        cstdint,
        cstring,
        exception,
        functional,
        initializer_list,
        iterator,
        limits,
        memory,
        new,
        ranges,
        stdexcept,
        string,
        string_view,
        type_traits,
        utility,
        vector,
        basetsd,
        sys_types,
        content: _,
    } = *include;

    if algorithm && !cxx_header {
        writeln!(out, "#include <algorithm>");
    }
    if array && !cxx_header {
        writeln!(out, "#include <array>");
    }
    if cassert && !cxx_header {
        writeln!(out, "#include <cassert>");
    }
    if cstddef && !cxx_header {
        writeln!(out, "#include <cstddef>");
    }
    if cstdint && !cxx_header {
        writeln!(out, "#include <cstdint>");
    }
    if cstring {
        writeln!(out, "#include <cstring>");
    }
    if exception && !cxx_header {
        writeln!(out, "#include <exception>");
    }
    if functional {
        writeln!(out, "#include <functional>");
    }
    if initializer_list && !cxx_header {
        writeln!(out, "#include <initializer_list>");
    }
    if iterator && !cxx_header {
        writeln!(out, "#include <iterator>");
    }
    if limits {
        writeln!(out, "#include <limits>");
    }
    if memory {
        writeln!(out, "#include <memory>");
    }
    if new && !cxx_header {
        writeln!(out, "#include <new>");
    }
    if stdexcept && !cxx_header {
        writeln!(out, "#include <stdexcept>");
    }
    if string && !cxx_header {
        writeln!(out, "#include <string>");
    }
    if type_traits && !cxx_header {
        writeln!(out, "#include <type_traits>");
    }
    if utility && !cxx_header {
        writeln!(out, "#include <utility>");
    }
    if vector && !cxx_header {
        writeln!(out, "#include <vector>");
    }
    if basetsd && !cxx_header {
        writeln!(out, "#if defined(_WIN32)");
        writeln!(out, "#include <basetsd.h>");
    }
    if sys_types && !cxx_header {
        if basetsd {
            writeln!(out, "#else");
        } else {
            writeln!(out, "#if not defined(_WIN32)");
        }
    }
    if sys_types && !cxx_header {
        writeln!(out, "#include <sys/types.h>");
    }
    if (basetsd || sys_types) && !cxx_header {
        writeln!(out, "#endif");
    }
    if string_view && !cxx_header {
        writeln!(out, "#if __cplusplus >= 201703L");
        writeln!(out, "#include <string_view>");
        writeln!(out, "#endif");
    }
    if ranges && !cxx_header {
        writeln!(out, "#if __cplusplus >= 202002L");
        writeln!(out, "#include <ranges>");
        writeln!(out, "#endif");
    }
}

impl<'i, 'a> Extend<&'i Include> for Includes<'a> {
    fn extend<I: IntoIterator<Item = &'i Include>>(&mut self, iter: I) {
        self.custom.extend(iter.into_iter().cloned());
    }
}

impl From<&syntax::Include> for Include {
    fn from(include: &syntax::Include) -> Self {
        Include {
            path: include.path.clone(),
            kind: include.kind,
        }
    }
}

impl<'a> Deref for Includes<'a> {
    type Target = Content<'a>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<'a> DerefMut for Includes<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
