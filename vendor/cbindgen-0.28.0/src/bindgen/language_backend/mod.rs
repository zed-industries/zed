use crate::bindgen::ir::{
    cfg::ConditionWrite, DeprecatedNoteKind, Documentation, Enum, Function, ItemContainer, Literal,
    OpaqueItem, Static, Struct, ToCondition, Type, Typedef, Union,
};
use crate::bindgen::writer::SourceWriter;
use crate::bindgen::{cdecl, Bindings, Layout};
use crate::Config;

use std::io::Write;

mod clike;
mod cython;

pub use clike::CLikeLanguageBackend;
pub use cython::CythonLanguageBackend;

pub trait LanguageBackend: Sized {
    fn open_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>);
    fn close_namespaces<W: Write>(&mut self, out: &mut SourceWriter<W>);
    fn write_headers<W: Write>(&self, out: &mut SourceWriter<W>, package_version: &str);
    fn write_footers<W: Write>(&mut self, out: &mut SourceWriter<W>);
    fn write_enum<W: Write>(&mut self, out: &mut SourceWriter<W>, e: &Enum);
    fn write_struct<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Struct);
    fn write_union<W: Write>(&mut self, out: &mut SourceWriter<W>, u: &Union);
    fn write_opaque_item<W: Write>(&mut self, out: &mut SourceWriter<W>, o: &OpaqueItem);
    fn write_type_def<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Typedef);
    fn write_static<W: Write>(&mut self, out: &mut SourceWriter<W>, s: &Static);

    fn write_function<W: Write>(
        &mut self,
        config: &Config,
        out: &mut SourceWriter<W>,
        f: &Function,
    ) {
        match config.function.args {
            Layout::Horizontal => {
                self.write_function_with_layout(config, out, f, Layout::Horizontal)
            }
            Layout::Vertical => self.write_function_with_layout(config, out, f, Layout::Vertical),
            Layout::Auto => {
                let max_line_length = config.line_length;
                if !out.try_write(
                    |out| self.write_function_with_layout(config, out, f, Layout::Horizontal),
                    max_line_length,
                ) {
                    self.write_function_with_layout(config, out, f, Layout::Vertical);
                }
            }
        }
    }

    fn write_function_with_layout<W: Write>(
        &mut self,
        config: &Config,
        out: &mut SourceWriter<W>,
        func: &Function,
        layout: Layout,
    ) {
        let prefix = config.function.prefix(&func.annotations);
        let postfix = config.function.postfix(&func.annotations);

        let condition = func.cfg.to_condition(config);
        condition.write_before(config, out);

        self.write_documentation(out, &func.documentation);

        fn write_space<W: Write>(layout: Layout, out: &mut SourceWriter<W>) {
            if layout == Layout::Vertical {
                out.new_line();
            } else {
                out.write(" ")
            }
        }
        if func.extern_decl {
            out.write("extern ");
        } else {
            if let Some(ref prefix) = prefix {
                write!(out, "{}", prefix);
                write_space(layout, out);
            }
            if func.annotations.must_use(config) {
                if let Some(ref anno) = config.function.must_use {
                    write!(out, "{}", anno);
                    write_space(layout, out);
                }
            }
            if let Some(note) = func
                .annotations
                .deprecated_note(config, DeprecatedNoteKind::Function)
            {
                write!(out, "{}", note);
                write_space(layout, out);
            }
        }
        cdecl::write_func(self, out, func, layout, config);

        if !func.extern_decl {
            if let Some(ref postfix) = postfix {
                write_space(layout, out);
                write!(out, "{}", postfix);
            }
        }

        if let Some(ref swift_name_macro) = config.function.swift_name_macro {
            if let Some(swift_name) = func.swift_name(config) {
                // XXX Should this account for `layout`?
                write!(out, " {}({})", swift_name_macro, swift_name);
            }
        }

        out.write(";");
        condition.write_after(config, out);
    }

    fn write_type<W: Write>(&mut self, out: &mut SourceWriter<W>, t: &Type);
    fn write_documentation<W: Write>(&mut self, out: &mut SourceWriter<W>, d: &Documentation);
    fn write_literal<W: Write>(&mut self, out: &mut SourceWriter<W>, l: &Literal);

    fn write_bindings<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        self.write_headers(out, &b.package_version);
        self.open_namespaces(out);
        self.write_primitive_constants(out, b);
        self.write_items(out, b);
        self.write_non_primitive_constants(out, b);
        self.write_globals(out, b);
        self.write_functions(out, b);
        self.close_namespaces(out);
        self.write_footers(out);
        self.write_trailer(out, b);
    }

    fn write_primitive_constants<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for constant in &b.constants {
            if constant.uses_only_primitive_types() {
                out.new_line_if_not_start();
                constant.write(&b.config, self, out, None);
                out.new_line();
            }
        }
    }

    /// If the struct is transparent, emit a typedef of its NZST field type instead.
    fn write_struct_or_typedef<W: Write>(
        &mut self,
        out: &mut SourceWriter<W>,
        s: &Struct,
        b: &Bindings,
    ) {
        if let Some(typedef) = s.as_typedef() {
            self.write_type_def(out, &typedef);
            for constant in &s.associated_constants {
                out.new_line();
                constant.write(&b.config, self, out, Some(s));
            }
        } else {
            self.write_struct(out, s);
        }
    }

    fn write_items<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for item in &b.items {
            if item
                .deref()
                .annotations()
                .bool("no-export")
                .unwrap_or(false)
            {
                continue;
            }

            out.new_line_if_not_start();
            match *item {
                ItemContainer::Constant(..) => unreachable!(),
                ItemContainer::Static(..) => unreachable!(),
                ItemContainer::Enum(ref x) => self.write_enum(out, x),
                ItemContainer::Struct(ref x) => self.write_struct_or_typedef(out, x, b),
                ItemContainer::Union(ref x) => self.write_union(out, x),
                ItemContainer::OpaqueItem(ref x) => self.write_opaque_item(out, x),
                ItemContainer::Typedef(ref x) => self.write_type_def(out, x),
            }
            out.new_line();
        }
    }

    fn write_non_primitive_constants<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for constant in &b.constants {
            if !constant.uses_only_primitive_types() {
                out.new_line_if_not_start();
                constant.write(&b.config, self, out, None);
                out.new_line();
            }
        }
    }

    fn write_globals<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        self.write_globals_default(out, b)
    }

    fn write_globals_default<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for global in &b.globals {
            out.new_line_if_not_start();
            self.write_static(out, global);
            out.new_line();
        }
    }

    fn write_functions<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        self.write_functions_default(out, b)
    }

    fn write_functions_default<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        for function in &b.functions {
            out.new_line_if_not_start();
            self.write_function(&b.config, out, function);
            out.new_line();
        }
    }

    fn write_trailer<W: Write>(&mut self, out: &mut SourceWriter<W>, b: &Bindings) {
        if let Some(ref f) = b.config.trailer {
            out.new_line_if_not_start();
            write!(out, "{}", f);
            if !f.ends_with('\n') {
                out.new_line();
            }
        }
    }
}
