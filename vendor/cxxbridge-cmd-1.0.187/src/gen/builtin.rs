use crate::gen::block::Block;
use crate::gen::ifndef;
use crate::gen::include::Includes;
use crate::gen::out::{Content, OutFile};
use crate::gen::pragma::Pragma;

#[derive(Default, PartialEq)]
pub(crate) struct Builtins<'a> {
    pub panic: bool,
    pub rust_string: bool,
    pub rust_str: bool,
    pub rust_slice: bool,
    pub rust_box: bool,
    pub rust_vec: bool,
    pub rust_fn: bool,
    pub rust_isize: bool,
    pub opaque: bool,
    pub layout: bool,
    pub unsafe_bitcopy: bool,
    pub unsafe_bitcopy_t: bool,
    pub rust_error: bool,
    pub manually_drop: bool,
    pub maybe_uninit: bool,
    pub trycatch: bool,
    pub ptr_len: bool,
    pub repr_fat: bool,
    pub rust_str_new_unchecked: bool,
    pub rust_str_repr: bool,
    pub rust_slice_new: bool,
    pub rust_slice_repr: bool,
    pub relocatable: bool,
    pub relocatable_or_array: bool,
    pub friend_impl: bool,
    pub is_complete: bool,
    pub destroy: bool,
    pub deleter_if: bool,
    pub shared_ptr: bool,
    pub vector: bool,
    pub alignmax: bool,
    pub content: Content<'a>,
}

impl<'a> Builtins<'a> {
    pub(crate) fn new() -> Self {
        Builtins::default()
    }
}

pub(super) fn write(out: &mut OutFile) {
    if out.builtin == Default::default() {
        return;
    }

    let include = &mut out.include;
    let pragma = &mut out.pragma;
    let builtin = &mut out.builtin;
    let out = &mut builtin.content;

    if builtin.rust_string {
        include.array = true;
        include.cstdint = true;
        include.string = true;
    }

    if builtin.rust_str {
        include.array = true;
        include.cstdint = true;
        include.string = true;
        include.string_view = true;
        builtin.friend_impl = true;
    }

    if builtin.rust_vec {
        include.algorithm = true;
        include.array = true;
        include.cassert = true;
        include.cstddef = true;
        include.cstdint = true;
        include.initializer_list = true;
        include.iterator = true;
        include.new = true;
        include.stdexcept = true;
        include.type_traits = true;
        include.utility = true;
        builtin.panic = true;
        builtin.rust_slice = true;
        builtin.unsafe_bitcopy_t = true;
    }

    if builtin.rust_slice {
        include.array = true;
        include.cassert = true;
        include.cstddef = true;
        include.cstdint = true;
        include.iterator = true;
        include.ranges = true;
        include.stdexcept = true;
        include.type_traits = true;
        builtin.friend_impl = true;
        builtin.layout = true;
        builtin.panic = true;
    }

    if builtin.rust_box {
        include.new = true;
        include.type_traits = true;
        include.utility = true;
    }

    if builtin.rust_fn {
        include.utility = true;
    }

    if builtin.rust_error {
        include.exception = true;
        builtin.friend_impl = true;
    }

    if builtin.rust_isize {
        include.basetsd = true;
        include.sys_types = true;
    }

    if builtin.relocatable_or_array {
        include.cstddef = true;
        builtin.relocatable = true;
    }

    if builtin.relocatable {
        include.type_traits = true;
    }

    if builtin.layout {
        include.type_traits = true;
        include.cstddef = true;
        builtin.is_complete = true;
    }

    if builtin.shared_ptr {
        include.memory = true;
        include.type_traits = true;
        builtin.is_complete = true;
    }

    if builtin.is_complete {
        include.cstddef = true;
        include.type_traits = true;
    }

    if builtin.unsafe_bitcopy {
        builtin.unsafe_bitcopy_t = true;
    }

    if builtin.trycatch {
        builtin.ptr_len = true;
    }

    out.begin_block(Block::Namespace("rust"));
    out.begin_block(Block::InlineNamespace("cxxbridge1"));

    let cxx_header = include.has_cxx_header();
    if !cxx_header {
        writeln!(out, "// #include \"rust/cxx.h\"");

        ifndef::write(out, builtin.panic, "CXXBRIDGE1_PANIC");

        if builtin.rust_string {
            out.next_section();
            writeln!(out, "struct unsafe_bitcopy_t;");
        }

        if builtin.friend_impl {
            out.begin_block(Block::AnonymousNamespace);
            writeln!(out, "template <typename T>");
            writeln!(out, "class impl;");
            out.end_block(Block::AnonymousNamespace);
        }

        out.next_section();
        if builtin.rust_str && !builtin.rust_string {
            writeln!(out, "class String;");
        }
        if builtin.layout && !builtin.opaque {
            writeln!(out, "class Opaque;");
        }

        if builtin.rust_slice {
            out.next_section();
            writeln!(out, "template <typename T>");
            writeln!(out, "::std::size_t size_of();");
            writeln!(out, "template <typename T>");
            writeln!(out, "::std::size_t align_of();");
        }

        ifndef::write(out, builtin.rust_string, "CXXBRIDGE1_RUST_STRING");
        ifndef::write(out, builtin.rust_str, "CXXBRIDGE1_RUST_STR");
        ifndef::write(out, builtin.rust_slice, "CXXBRIDGE1_RUST_SLICE");
        ifndef::write(out, builtin.rust_box, "CXXBRIDGE1_RUST_BOX");
        ifndef::write(out, builtin.unsafe_bitcopy_t, "CXXBRIDGE1_RUST_BITCOPY_T");
        ifndef::write(out, builtin.unsafe_bitcopy, "CXXBRIDGE1_RUST_BITCOPY");
        ifndef::write(out, builtin.rust_vec, "CXXBRIDGE1_RUST_VEC");
        ifndef::write(out, builtin.rust_fn, "CXXBRIDGE1_RUST_FN");
        ifndef::write(out, builtin.rust_error, "CXXBRIDGE1_RUST_ERROR");
        ifndef::write(out, builtin.rust_isize, "CXXBRIDGE1_RUST_ISIZE");
        ifndef::write(out, builtin.opaque, "CXXBRIDGE1_RUST_OPAQUE");
        ifndef::write(out, builtin.is_complete, "CXXBRIDGE1_IS_COMPLETE");
        ifndef::write(out, builtin.layout, "CXXBRIDGE1_LAYOUT");
        ifndef::write(out, builtin.relocatable, "CXXBRIDGE1_RELOCATABLE");
    }

    out.end_block(Block::InlineNamespace("cxxbridge1"));
    out.end_block(Block::Namespace("rust"));

    macro_rules! write_builtin {
        ($path:literal) => {
            write_builtin(out, include, pragma, include_str!($path));
        };
    }

    // namespace rust::cxxbridge1

    if builtin.rust_str_new_unchecked {
        write_builtin!("builtin/rust_str_uninit.h");
    }

    if builtin.rust_slice_new {
        write_builtin!("builtin/rust_slice_uninit.h");
    }

    // namespace rust::cxxbridge1::repr

    if builtin.repr_fat {
        write_builtin!("builtin/repr_fat.h");
    }

    if builtin.ptr_len {
        write_builtin!("builtin/ptr_len.h");
    }

    if builtin.alignmax {
        write_builtin!("builtin/alignmax.h");
    }

    // namespace rust::cxxbridge1::detail

    if builtin.maybe_uninit {
        write_builtin!("builtin/maybe_uninit_detail.h");
    }

    if builtin.trycatch {
        write_builtin!("builtin/trycatch_detail.h");
    }

    // namespace rust::cxxbridge1

    if builtin.manually_drop {
        write_builtin!("builtin/manually_drop.h");
    }

    if builtin.maybe_uninit {
        write_builtin!("builtin/maybe_uninit.h");
    }

    out.begin_block(Block::Namespace("rust"));
    out.begin_block(Block::InlineNamespace("cxxbridge1"));
    out.begin_block(Block::AnonymousNamespace);

    if builtin.rust_str_new_unchecked || builtin.rust_str_repr {
        out.next_section();
        writeln!(out, "template <>");
        writeln!(out, "class impl<Str> final {{");
        writeln!(out, "public:");
        if builtin.rust_str_new_unchecked {
            writeln!(
                out,
                "  static Str new_unchecked(repr::Fat repr) noexcept {{",
            );
            writeln!(out, "    Str str = Str::uninit{{}};");
            writeln!(out, "    str.repr = repr;");
            writeln!(out, "    return str;");
            writeln!(out, "  }}");
        }
        if builtin.rust_str_repr {
            writeln!(out, "  static repr::Fat repr(Str str) noexcept {{");
            writeln!(out, "    return str.repr;");
            writeln!(out, "  }}");
        }
        writeln!(out, "}};");
    }

    if builtin.rust_slice_new || builtin.rust_slice_repr {
        out.next_section();
        writeln!(out, "template <typename T>");
        writeln!(out, "class impl<Slice<T>> final {{");
        writeln!(out, "public:");
        if builtin.rust_slice_new {
            writeln!(out, "  static Slice<T> slice(repr::Fat repr) noexcept {{");
            writeln!(out, "    Slice<T> slice = typename Slice<T>::uninit{{}};");
            writeln!(out, "    slice.repr = repr;");
            writeln!(out, "    return slice;");
            writeln!(out, "  }}");
        }
        if builtin.rust_slice_repr {
            writeln!(out, "  static repr::Fat repr(Slice<T> slice) noexcept {{");
            writeln!(out, "    return slice.repr;");
            writeln!(out, "  }}");
        }
        writeln!(out, "}};");
    }

    out.end_block(Block::AnonymousNamespace);
    out.end_block(Block::InlineNamespace("cxxbridge1"));
    out.end_block(Block::Namespace("rust"));

    // namespace rust::cxxbridge1::(anonymous)

    if builtin.rust_error {
        write_builtin!("builtin/rust_error.h");
    }

    if builtin.destroy {
        write_builtin!("builtin/destroy.h");
    }

    if builtin.deleter_if {
        write_builtin!("builtin/deleter_if.h");
    }

    if builtin.shared_ptr {
        write_builtin!("builtin/shared_ptr.h");
    }

    if builtin.vector {
        write_builtin!("builtin/vector.h");
    }

    if builtin.relocatable_or_array {
        write_builtin!("builtin/relocatable_or_array.h");
    }

    // namespace rust::behavior

    if builtin.trycatch {
        write_builtin!("builtin/trycatch.h");
    }
}

fn write_builtin<'a>(
    out: &mut Content<'a>,
    include: &mut Includes,
    pragma: &mut Pragma<'a>,
    src: &'a str,
) {
    let mut namespace = Vec::new();
    let mut ready = false;

    for line in src.lines() {
        if line == "#pragma once" || line.starts_with("#include \".") {
            continue;
        } else if let Some(rest) = line.strip_prefix("#include <") {
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
                basetsd: _,
                sys_types: _,
                content: _,
            } = include;
            match rest.strip_suffix(">").unwrap() {
                "algorithm" => *algorithm = true,
                "array" => *array = true,
                "cassert" => *cassert = true,
                "cstddef" => *cstddef = true,
                "cstdint" => *cstdint = true,
                "cstring" => *cstring = true,
                "exception" => *exception = true,
                "functional" => *functional = true,
                "initializer_list" => *initializer_list = true,
                "iterator" => *iterator = true,
                "limits" => *limits = true,
                "memory" => *memory = true,
                "new" => *new = true,
                "ranges" => *ranges = true,
                "stdexcept" => *stdexcept = true,
                "string" => *string = true,
                "string_view" => *string_view = true,
                "type_traits" => *type_traits = true,
                "utility" => *utility = true,
                "vector" => *vector = true,
                _ => unimplemented!("{}", line),
            }
        } else if let Some(rest) = line.strip_prefix("#pragma GCC diagnostic ignored \"") {
            let diagnostic = rest.strip_suffix('"').unwrap();
            pragma.gnu_diagnostic_ignore.insert(diagnostic);
            ready = false;
        } else if let Some(rest) = line.strip_prefix("#pragma clang diagnostic ignored \"") {
            let diagnostic = rest.strip_suffix('"').unwrap();
            pragma.clang_diagnostic_ignore.insert(diagnostic);
            ready = false;
        } else if line == "namespace {" {
            namespace.push(Block::AnonymousNamespace);
            out.begin_block(Block::AnonymousNamespace);
        } else if let Some(rest) = line.strip_prefix("namespace ") {
            let name = rest.strip_suffix(" {").unwrap();
            namespace.push(Block::Namespace(name));
            out.begin_block(Block::Namespace(name));
        } else if let Some(rest) = line.strip_prefix("inline namespace ") {
            let name = rest.strip_suffix(" {").unwrap();
            namespace.push(Block::InlineNamespace(name));
            out.begin_block(Block::InlineNamespace(name));
        } else if line.starts_with("} // namespace") {
            out.end_block(namespace.pop().unwrap());
        } else if line.is_empty() && !ready {
            out.next_section();
            ready = true;
        } else if !line.trim_start_matches(' ').starts_with("//") {
            assert!(ready);
            writeln!(out, "{}", line);
        }
    }

    assert!(namespace.is_empty());
    assert!(ready);
}

#[cfg(test)]
mod tests {
    use crate::gen::include::Includes;
    use crate::gen::out::Content;
    use crate::gen::pragma::Pragma;
    use std::fs;

    #[test]
    fn test_write_builtin() {
        let mut builtin_src = Vec::new();

        for entry in fs::read_dir("src/gen/builtin").unwrap() {
            let path = entry.unwrap().path();
            let src = fs::read_to_string(path).unwrap();
            builtin_src.push(src);
        }

        assert_ne!(builtin_src.len(), 0);
        builtin_src.sort();

        let mut content = Content::new();
        let mut include = Includes::new();
        let mut pragma = Pragma::new();
        for src in &builtin_src {
            super::write_builtin(&mut content, &mut include, &mut pragma, src);
        }
    }
}
