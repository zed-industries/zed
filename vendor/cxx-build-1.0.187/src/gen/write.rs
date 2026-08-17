use crate::gen::block::Block;
use crate::gen::guard::Guard;
use crate::gen::nested::NamespaceEntries;
use crate::gen::out::OutFile;
use crate::gen::{builtin, include, pragma, Opt};
use crate::syntax::atom::Atom::{self, *};
use crate::syntax::discriminant::{Discriminant, Limits};
use crate::syntax::instantiate::{ImplKey, NamedImplKey};
use crate::syntax::map::UnorderedMap as Map;
use crate::syntax::namespace::Namespace;
use crate::syntax::primitive::{self, PrimitiveKind};
use crate::syntax::set::UnorderedSet;
use crate::syntax::symbol::{self, Symbol};
use crate::syntax::trivial::{self, TrivialReason};
use crate::syntax::{
    derive, mangle, Api, Doc, Enum, ExternFn, ExternType, FnKind, Lang, Pair, Signature, Struct,
    Trait, Type, TypeAlias, Types, Var,
};
use proc_macro2::Ident;

pub(super) fn gen(apis: &[Api], types: &Types, opt: &Opt, header: bool) -> Vec<u8> {
    let mut out_file = OutFile::new(header, opt, types);
    let out = &mut out_file;

    pick_includes_and_builtins(out, apis);
    out.include.extend(&opt.include);

    write_macros(out, apis);
    write_forward_declarations(out, apis);
    write_data_structures(out, apis);
    write_functions(out, apis);
    write_generic_instantiations(out);

    builtin::write(out);
    pragma::write(out);
    include::write(out);

    out_file.content()
}

fn write_macros(out: &mut OutFile, apis: &[Api]) {
    let mut needs_default_value = false;
    for api in apis {
        if let Api::Struct(strct) = api {
            if !out.types.cxx.contains(&strct.name.rust) {
                for field in &strct.fields {
                    needs_default_value |= primitive::kind(&field.ty).is_some();
                }
            }
        }
    }

    if needs_default_value {
        out.next_section();
        writeln!(out, "#if __cplusplus >= 201402L");
        writeln!(out, "#define CXX_DEFAULT_VALUE(value) = value");
        writeln!(out, "#else");
        writeln!(out, "#define CXX_DEFAULT_VALUE(value)");
        writeln!(out, "#endif");
    }
}

fn write_forward_declarations(out: &mut OutFile, apis: &[Api]) {
    let needs_forward_declaration = |api: &&Api| match api {
        Api::Struct(_) | Api::CxxType(_) | Api::RustType(_) => true,
        Api::Enum(enm) => !out.types.cxx.contains(&enm.name.rust),
        _ => false,
    };

    let apis_by_namespace =
        NamespaceEntries::new(apis.iter().filter(needs_forward_declaration).collect());

    out.next_section();
    write(out, &apis_by_namespace, 0);

    fn write(out: &mut OutFile, ns_entries: &NamespaceEntries, indent: usize) {
        let apis = ns_entries.direct_content();

        for api in apis {
            write!(out, "{:1$}", "", indent);
            match api {
                Api::Struct(strct) => write_struct_decl(out, &strct.name),
                Api::Enum(enm) => write_enum_decl(out, enm),
                Api::CxxType(ety) => write_struct_using(out, &ety.name),
                Api::RustType(ety) => write_struct_decl(out, &ety.name),
                _ => unreachable!(),
            }
        }

        for (namespace, nested_ns_entries) in ns_entries.nested_content() {
            writeln!(out, "{:2$}namespace {} {{", "", namespace, indent);
            write(out, nested_ns_entries, indent + 2);
            writeln!(out, "{:1$}}}", "", indent);
        }
    }
}

fn write_data_structures<'a>(out: &mut OutFile<'a>, apis: &'a [Api]) {
    let mut methods_for_type = Map::new();
    for api in apis {
        if let Api::CxxFunction(efn) | Api::RustFunction(efn) = api {
            if let Some(self_type) = efn.self_type() {
                methods_for_type
                    .entry(self_type)
                    .or_insert_with(Vec::new)
                    .push(efn);
            }
        }
    }

    let mut structs_written = UnorderedSet::new();
    let mut toposorted_structs = out.types.toposorted_structs.iter();
    for api in apis {
        match api {
            Api::Struct(strct) if !structs_written.contains(&strct.name.rust) => {
                for next in &mut toposorted_structs {
                    if !out.types.cxx.contains(&next.name.rust) {
                        out.next_section();
                        let methods = methods_for_type
                            .get(&next.name.rust)
                            .map(Vec::as_slice)
                            .unwrap_or_default();
                        write_struct(out, next, methods);
                    }
                    structs_written.insert(&next.name.rust);
                    if next.name.rust == strct.name.rust {
                        break;
                    }
                }
            }
            Api::Enum(enm) => {
                out.next_section();
                if out.types.cxx.contains(&enm.name.rust) {
                    check_enum(out, enm);
                } else {
                    write_enum(out, enm);
                }
            }
            Api::RustType(ety) => {
                out.next_section();
                let methods = methods_for_type
                    .get(&ety.name.rust)
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                write_opaque_type(out, ety, methods);
            }
            _ => {}
        }
    }

    if out.header {
        return;
    }

    out.set_namespace(Default::default());

    out.next_section();
    for api in apis {
        if let Api::TypeAlias(ety) = api {
            if let Some(reasons) = out.types.required_trivial.get(&ety.name.rust) {
                check_trivial_extern_type(out, ety, reasons);
            }
        }
    }
}

fn write_functions<'a>(out: &mut OutFile<'a>, apis: &'a [Api]) {
    if !out.header {
        for api in apis {
            match api {
                Api::Struct(strct) => write_struct_operator_decls(out, strct),
                Api::RustType(ety) => write_opaque_type_layout_decls(out, ety),
                Api::CxxFunction(efn) => write_cxx_function_shim(out, efn),
                Api::RustFunction(efn) => write_rust_function_decl(out, efn),
                _ => {}
            }
        }

        write_std_specializations(out, apis);
    }

    for api in apis {
        match api {
            Api::Struct(strct) => write_struct_operators(out, strct),
            Api::RustType(ety) => write_opaque_type_layout(out, ety),
            Api::RustFunction(efn) => {
                out.next_section();
                write_rust_function_shim(out, efn);
            }
            _ => {}
        }
    }
}

fn write_std_specializations(out: &mut OutFile, apis: &[Api]) {
    out.set_namespace(Default::default());
    out.begin_block(Block::Namespace("std"));

    for api in apis {
        if let Api::Struct(strct) = api {
            if derive::contains(&strct.derives, Trait::Hash) {
                out.next_section();
                out.include.cstddef = true;
                out.include.functional = true;
                out.pragma.dollar_in_identifier = true;
                let qualified = strct.name.to_fully_qualified();
                writeln!(out, "template <> struct hash<{}> {{", qualified);
                writeln!(
                    out,
                    "  ::std::size_t operator()({} const &self) const noexcept {{",
                    qualified,
                );
                let link_name = mangle::operator(&strct.name, "hash");
                write!(out, "    return ::");
                for name in &strct.name.namespace {
                    write!(out, "{}::", name);
                }
                writeln!(out, "{}(self);", link_name);
                writeln!(out, "  }}");
                writeln!(out, "}};");
            }
        }
    }

    out.end_block(Block::Namespace("std"));
}

fn pick_includes_and_builtins(out: &mut OutFile, apis: &[Api]) {
    for api in apis {
        if let Api::Include(include) = api {
            out.include.insert(include);
        }
    }

    for ty in out.types {
        match ty {
            Type::Ident(ident) => match Atom::from(&ident.rust) {
                Some(U8 | U16 | U32 | U64 | I8 | I16 | I32 | I64) => out.include.cstdint = true,
                Some(Usize) => out.include.cstddef = true,
                Some(Isize) => out.builtin.rust_isize = true,
                Some(CxxString) => out.include.string = true,
                Some(RustString) => out.builtin.rust_string = true,
                Some(Bool | Char | F32 | F64) | None => {}
            },
            Type::RustBox(_) => out.builtin.rust_box = true,
            Type::RustVec(_) => out.builtin.rust_vec = true,
            Type::UniquePtr(_) => out.include.memory = true,
            Type::SharedPtr(_) | Type::WeakPtr(_) => out.include.memory = true,
            Type::Str(_) => out.builtin.rust_str = true,
            Type::CxxVector(_) => out.include.vector = true,
            Type::Fn(_) => out.builtin.rust_fn = true,
            Type::SliceRef(_) => out.builtin.rust_slice = true,
            Type::Array(_) => out.include.array = true,
            Type::Ref(_) | Type::Void(_) | Type::Ptr(_) => {}
        }
    }
}

fn write_doc(out: &mut OutFile, indent: &str, doc: &Doc) {
    let mut lines = 0;
    for line in doc.to_string().lines() {
        if out.opt.doxygen {
            writeln!(out, "{}///{}", indent, line);
        } else {
            writeln!(out, "{}//{}", indent, line);
        }
        lines += 1;
    }
    // According to https://www.doxygen.nl/manual/docblocks.html, Doxygen only
    // interprets `///` as a Doxygen comment block if there are at least 2 of
    // them. In Rust, a single `///` is definitely still documentation so we
    // make sure to propagate that as a Doxygen comment.
    if out.opt.doxygen && lines == 1 {
        writeln!(out, "{}///", indent);
    }
}

fn write_struct<'a>(out: &mut OutFile<'a>, strct: &'a Struct, methods: &[&ExternFn]) {
    let operator_eq = derive::contains(&strct.derives, Trait::PartialEq);
    let operator_ord = derive::contains(&strct.derives, Trait::PartialOrd);

    out.set_namespace(&strct.name.namespace);
    let guard = Guard::new(out, "CXXBRIDGE1_STRUCT", &strct.name);
    writeln!(out, "#ifndef {}", guard);
    writeln!(out, "#define {}", guard);
    write_doc(out, "", &strct.doc);
    write!(out, "struct");
    if let Some(align) = &strct.align {
        out.builtin.alignmax = true;
        writeln!(out, " alignas(::rust::repr::alignmax<");
        writeln!(out, "  {},", align.base10_parse::<u32>().unwrap());
        for (i, field) in strct.fields.iter().enumerate() {
            write!(out, "  alignof(");
            write_type(out, &field.ty);
            write!(out, ")");
            if i + 1 != strct.fields.len() {
                write!(out, ",");
            }
            writeln!(out);
        }
        write!(out, ">)");
    }
    writeln!(out, " {} final {{", strct.name.cxx);

    for field in &strct.fields {
        write_doc(out, "  ", &field.doc);
        write!(out, "  ");
        write_type_space(out, &field.ty);
        write!(out, "{}", field.name.cxx);
        if let Some(primitive) = primitive::kind(&field.ty) {
            let default_value = match primitive {
                PrimitiveKind::Boolean => "false",
                PrimitiveKind::Number => "0",
                PrimitiveKind::Pointer => "nullptr",
            };
            write!(out, " CXX_DEFAULT_VALUE({})", default_value);
        }
        writeln!(out, ";");
    }

    out.next_section();

    for method in methods {
        if !method.doc.is_empty() {
            out.next_section();
        }
        write_doc(out, "  ", &method.doc);
        write!(out, "  ");
        let local_name = method.name.cxx.to_string();
        let sig = &method.sig;
        let in_class = true;
        let indirect_call = false;
        let main = false;
        write_rust_function_shim_decl(out, &local_name, sig, in_class, indirect_call, main);
        writeln!(out, ";");
        if !method.doc.is_empty() {
            out.next_section();
        }
    }

    if operator_eq {
        writeln!(
            out,
            "  bool operator==({} const &) const noexcept;",
            strct.name.cxx,
        );
        writeln!(
            out,
            "  bool operator!=({} const &) const noexcept;",
            strct.name.cxx,
        );
    }

    if operator_ord {
        writeln!(
            out,
            "  bool operator<({} const &) const noexcept;",
            strct.name.cxx,
        );
        writeln!(
            out,
            "  bool operator<=({} const &) const noexcept;",
            strct.name.cxx,
        );
        writeln!(
            out,
            "  bool operator>({} const &) const noexcept;",
            strct.name.cxx,
        );
        writeln!(
            out,
            "  bool operator>=({} const &) const noexcept;",
            strct.name.cxx,
        );
    }

    out.include.type_traits = true;
    writeln!(out, "  using IsRelocatable = ::std::true_type;");

    writeln!(out, "}};");
    writeln!(out, "#endif // {}", guard);
}

fn write_struct_decl(out: &mut OutFile, ident: &Pair) {
    writeln!(out, "struct {};", ident.cxx);
}

fn write_enum_decl(out: &mut OutFile, enm: &Enum) {
    write!(out, "enum class {} : ", enm.name.cxx);
    write_atom(out, enm.repr.atom);
    writeln!(out, ";");
}

fn write_struct_using(out: &mut OutFile, ident: &Pair) {
    writeln!(out, "using {} = {};", ident.cxx, ident.to_fully_qualified());
}

fn write_opaque_type<'a>(out: &mut OutFile<'a>, ety: &'a ExternType, methods: &[&ExternFn]) {
    out.set_namespace(&ety.name.namespace);
    let guard = Guard::new(out, "CXXBRIDGE1_STRUCT", &ety.name);
    writeln!(out, "#ifndef {}", guard);
    writeln!(out, "#define {}", guard);
    write_doc(out, "", &ety.doc);

    out.builtin.opaque = true;
    writeln!(
        out,
        "struct {} final : public ::rust::Opaque {{",
        ety.name.cxx,
    );

    for (i, method) in methods.iter().enumerate() {
        if i > 0 && !method.doc.is_empty() {
            out.next_section();
        }
        write_doc(out, "  ", &method.doc);
        write!(out, "  ");
        let local_name = method.name.cxx.to_string();
        let sig = &method.sig;
        let in_class = true;
        let indirect_call = false;
        let main = false;
        write_rust_function_shim_decl(out, &local_name, sig, in_class, indirect_call, main);
        writeln!(out, ";");
        if !method.doc.is_empty() {
            out.next_section();
        }
    }

    writeln!(out, "  ~{}() = delete;", ety.name.cxx);
    writeln!(out);

    out.builtin.layout = true;
    out.include.cstddef = true;
    writeln!(out, "private:");
    writeln!(out, "  friend ::rust::layout;");
    writeln!(out, "  struct layout {{");
    writeln!(out, "    static ::std::size_t size() noexcept;");
    writeln!(out, "    static ::std::size_t align() noexcept;");
    writeln!(out, "  }};");
    writeln!(out, "}};");
    writeln!(out, "#endif // {}", guard);
}

fn write_enum<'a>(out: &mut OutFile<'a>, enm: &'a Enum) {
    out.set_namespace(&enm.name.namespace);
    let guard = Guard::new(out, "CXXBRIDGE1_ENUM", &enm.name);
    writeln!(out, "#ifndef {}", guard);
    writeln!(out, "#define {}", guard);

    write_doc(out, "", &enm.doc);
    write!(out, "enum class {} : ", enm.name.cxx);
    write_atom(out, enm.repr.atom);
    writeln!(out, " {{");
    for variant in &enm.variants {
        write_doc(out, "  ", &variant.doc);
        write!(out, "  {} = ", variant.name.cxx);
        write_discriminant(out, enm.repr.atom, variant.discriminant);
        writeln!(out, ",");
    }
    writeln!(out, "}};");

    if out.header {
        write_enum_operators(out, enm);
    }

    writeln!(out, "#endif // {}", guard);
}

fn check_enum<'a>(out: &mut OutFile<'a>, enm: &'a Enum) {
    out.set_namespace(&enm.name.namespace);
    out.include.type_traits = true;
    writeln!(
        out,
        "static_assert(::std::is_enum<{}>::value, \"expected enum\");",
        enm.name.cxx,
    );
    write!(out, "static_assert(sizeof({}) == sizeof(", enm.name.cxx);
    write_atom(out, enm.repr.atom);
    writeln!(out, "), \"incorrect size\");");
    for variant in &enm.variants {
        write!(out, "static_assert(static_cast<");
        write_atom(out, enm.repr.atom);
        writeln!(out, ">({}::{}) == ", enm.name.cxx, variant.name.cxx);
        write_discriminant(out, enm.repr.atom, variant.discriminant);
        writeln!(out, ", \"disagrees with the value in #[cxx::bridge]\");");
    }

    if out.header
        && (derive::contains(&enm.derives, Trait::BitAnd)
            || derive::contains(&enm.derives, Trait::BitOr)
            || derive::contains(&enm.derives, Trait::BitXor))
    {
        out.next_section();
        let guard = Guard::new(out, "CXXBRIDGE1_ENUM", &enm.name);
        writeln!(out, "#ifndef {}", guard);
        writeln!(out, "#define {}", guard);
        out.suppress_next_section();
        write_enum_operators(out, enm);
        writeln!(out, "#endif // {}", guard);
    }
}

fn write_discriminant(out: &mut OutFile, repr: Atom, discriminant: Discriminant) {
    let limits = Limits::of(repr).unwrap();
    if discriminant == limits.min && limits.min < Discriminant::zero() {
        out.include.limits = true;
        write!(out, "::std::numeric_limits<");
        write_atom(out, repr);
        write!(out, ">::min()");
    } else {
        write!(out, "{}", discriminant);
    }
}

fn write_binary_bitwise_op(out: &mut OutFile, op: &str, enm: &Enum) {
    let enum_name = &enm.name.cxx;
    writeln!(
        out,
        "inline {enum_name} operator{op}({enum_name} lhs, {enum_name} rhs) {{",
    );
    write!(out, "  return static_cast<{enum_name}>(static_cast<");
    write_atom(out, enm.repr.atom);
    write!(out, ">(lhs) {op} static_cast<");
    write_atom(out, enm.repr.atom);
    writeln!(out, ">(rhs));");
    writeln!(out, "}}");
}

fn write_enum_operators(out: &mut OutFile, enm: &Enum) {
    if derive::contains(&enm.derives, Trait::BitAnd) {
        out.next_section();
        write_binary_bitwise_op(out, "&", enm);
    }

    if derive::contains(&enm.derives, Trait::BitOr) {
        out.next_section();
        write_binary_bitwise_op(out, "|", enm);
    }

    if derive::contains(&enm.derives, Trait::BitXor) {
        out.next_section();
        write_binary_bitwise_op(out, "^", enm);
    }
}

fn check_trivial_extern_type(out: &mut OutFile, alias: &TypeAlias, reasons: &[TrivialReason]) {
    // NOTE: The following static assertion is just nice-to-have and not
    // necessary for soundness. That's because triviality is always declared by
    // the user in the form of an unsafe impl of cxx::ExternType:
    //
    //     unsafe impl ExternType for MyType {
    //         type Id = cxx::type_id!("...");
    //         type Kind = cxx::kind::Trivial;
    //     }
    //
    // Since the user went on the record with their unsafe impl to unsafely
    // claim they KNOW that the type is trivial, it's fine for that to be on
    // them if that were wrong. However, in practice correctly reasoning about
    // the relocatability of C++ types is challenging, particularly if the type
    // definition were to change over time, so for now we add this check.
    //
    // There may be legitimate reasons to opt out of this assertion for support
    // of types that the programmer knows are soundly Rust-movable despite not
    // being recognized as such by the C++ type system due to a move constructor
    // or destructor. To opt out of the relocatability check, they need to do
    // one of the following things in any header used by `include!` in their
    // bridge.
    //
    //      --- if they define the type:
    //      struct MyType {
    //        ...
    //    +   using IsRelocatable = std::true_type;
    //      };
    //
    //      --- otherwise:
    //    + template <>
    //    + struct rust::IsRelocatable<MyType> : std::true_type {};
    //

    let id = alias.name.to_fully_qualified();
    out.builtin.relocatable = true;

    let mut rust_type_ok = true;
    let mut array_ok = true;
    for reason in reasons {
        // Allow extern type that inherits from ::rust::Opaque in positions
        // where an opaque Rust type would be allowed.
        rust_type_ok &= match reason {
            TrivialReason::BoxTarget { .. }
            | TrivialReason::VecElement { .. }
            | TrivialReason::SliceElement { .. } => true,
            TrivialReason::StructField(_)
            | TrivialReason::FunctionArgument(_)
            | TrivialReason::FunctionReturn(_) => false,
        };
        // If the type is only used as a struct field or Vec element, not as
        // by-value function argument or return value, then C array of trivially
        // relocatable type is also permissible.
        //
        //     --- means something sane:
        //     struct T { char buf[N]; };
        //
        //     --- means something totally different:
        //     void f(char buf[N]);
        //
        array_ok &= match reason {
            TrivialReason::StructField(_) | TrivialReason::VecElement { .. } => true,
            TrivialReason::FunctionArgument(_)
            | TrivialReason::FunctionReturn(_)
            | TrivialReason::BoxTarget { .. }
            | TrivialReason::SliceElement { .. } => false,
        };
    }

    writeln!(out, "static_assert(");
    write!(out, "    ");
    if rust_type_ok {
        out.include.type_traits = true;
        out.builtin.opaque = true;
        write!(out, "::std::is_base_of<::rust::Opaque, {}>::value || ", id);
    }
    if array_ok {
        out.builtin.relocatable_or_array = true;
        writeln!(out, "::rust::IsRelocatableOrArray<{}>::value,", id);
    } else {
        writeln!(out, "::rust::IsRelocatable<{}>::value,", id);
    }
    writeln!(
        out,
        "    \"type {} should be trivially move constructible and trivially destructible in C++ to be used as {} in Rust\");",
        id.trim_start_matches("::"),
        trivial::as_what(&alias.name, reasons),
    );
}

fn write_struct_operator_decls<'a>(out: &mut OutFile<'a>, strct: &'a Struct) {
    out.set_namespace(&strct.name.namespace);
    out.begin_block(Block::ExternC);

    if derive::contains(&strct.derives, Trait::PartialEq) {
        out.pragma.dollar_in_identifier = true;
        out.pragma.missing_declarations = true;
        let link_name = mangle::operator(&strct.name, "eq");
        writeln!(
            out,
            "bool {}({1} const &, {1} const &) noexcept;",
            link_name, strct.name.cxx,
        );

        if !derive::contains(&strct.derives, Trait::Eq) {
            let link_name = mangle::operator(&strct.name, "ne");
            writeln!(
                out,
                "bool {}({1} const &, {1} const &) noexcept;",
                link_name, strct.name.cxx,
            );
        }
    }

    if derive::contains(&strct.derives, Trait::PartialOrd) {
        out.pragma.dollar_in_identifier = true;
        out.pragma.missing_declarations = true;
        let link_name = mangle::operator(&strct.name, "lt");
        writeln!(
            out,
            "bool {}({1} const &, {1} const &) noexcept;",
            link_name, strct.name.cxx,
        );

        let link_name = mangle::operator(&strct.name, "le");
        writeln!(
            out,
            "bool {}({1} const &, {1} const &) noexcept;",
            link_name, strct.name.cxx,
        );

        if !derive::contains(&strct.derives, Trait::Ord) {
            let link_name = mangle::operator(&strct.name, "gt");
            writeln!(
                out,
                "bool {}({1} const &, {1} const &) noexcept;",
                link_name, strct.name.cxx,
            );

            let link_name = mangle::operator(&strct.name, "ge");
            writeln!(
                out,
                "bool {}({1} const &, {1} const &) noexcept;",
                link_name, strct.name.cxx,
            );
        }
    }

    if derive::contains(&strct.derives, Trait::Hash) {
        out.include.cstddef = true;
        out.pragma.dollar_in_identifier = true;
        out.pragma.missing_declarations = true;
        let link_name = mangle::operator(&strct.name, "hash");
        writeln!(
            out,
            "::std::size_t {}({} const &) noexcept;",
            link_name, strct.name.cxx,
        );
    }

    out.end_block(Block::ExternC);
}

fn write_struct_operators<'a>(out: &mut OutFile<'a>, strct: &'a Struct) {
    if out.header {
        return;
    }

    out.set_namespace(&strct.name.namespace);

    if derive::contains(&strct.derives, Trait::PartialEq) {
        out.pragma.dollar_in_identifier = true;

        out.next_section();
        writeln!(
            out,
            "bool {0}::operator==({0} const &rhs) const noexcept {{",
            strct.name.cxx,
        );
        let link_name = mangle::operator(&strct.name, "eq");
        writeln!(out, "  return {}(*this, rhs);", link_name);
        writeln!(out, "}}");

        out.next_section();
        writeln!(
            out,
            "bool {0}::operator!=({0} const &rhs) const noexcept {{",
            strct.name.cxx,
        );
        if derive::contains(&strct.derives, Trait::Eq) {
            writeln!(out, "  return !(*this == rhs);");
        } else {
            let link_name = mangle::operator(&strct.name, "ne");
            writeln!(out, "  return {}(*this, rhs);", link_name);
        }
        writeln!(out, "}}");
    }

    if derive::contains(&strct.derives, Trait::PartialOrd) {
        out.pragma.dollar_in_identifier = true;

        out.next_section();
        writeln!(
            out,
            "bool {0}::operator<({0} const &rhs) const noexcept {{",
            strct.name.cxx,
        );
        let link_name = mangle::operator(&strct.name, "lt");
        writeln!(out, "  return {}(*this, rhs);", link_name);
        writeln!(out, "}}");

        out.next_section();
        writeln!(
            out,
            "bool {0}::operator<=({0} const &rhs) const noexcept {{",
            strct.name.cxx,
        );
        let link_name = mangle::operator(&strct.name, "le");
        writeln!(out, "  return {}(*this, rhs);", link_name);
        writeln!(out, "}}");

        out.next_section();
        writeln!(
            out,
            "bool {0}::operator>({0} const &rhs) const noexcept {{",
            strct.name.cxx,
        );
        if derive::contains(&strct.derives, Trait::Ord) {
            writeln!(out, "  return !(*this <= rhs);");
        } else {
            let link_name = mangle::operator(&strct.name, "gt");
            writeln!(out, "  return {}(*this, rhs);", link_name);
        }
        writeln!(out, "}}");

        out.next_section();
        writeln!(
            out,
            "bool {0}::operator>=({0} const &rhs) const noexcept {{",
            strct.name.cxx,
        );
        if derive::contains(&strct.derives, Trait::Ord) {
            writeln!(out, "  return !(*this < rhs);");
        } else {
            let link_name = mangle::operator(&strct.name, "ge");
            writeln!(out, "  return {}(*this, rhs);", link_name);
        }
        writeln!(out, "}}");
    }
}

fn write_opaque_type_layout_decls<'a>(out: &mut OutFile<'a>, ety: &'a ExternType) {
    out.set_namespace(&ety.name.namespace);
    out.begin_block(Block::ExternC);
    out.pragma.dollar_in_identifier = true;
    out.pragma.missing_declarations = true;

    let link_name = mangle::operator(&ety.name, "sizeof");
    writeln!(out, "::std::size_t {}() noexcept;", link_name);

    let link_name = mangle::operator(&ety.name, "alignof");
    writeln!(out, "::std::size_t {}() noexcept;", link_name);

    out.end_block(Block::ExternC);
}

fn write_opaque_type_layout<'a>(out: &mut OutFile<'a>, ety: &'a ExternType) {
    if out.header {
        return;
    }

    out.set_namespace(&ety.name.namespace);
    out.pragma.dollar_in_identifier = true;

    out.next_section();
    let link_name = mangle::operator(&ety.name, "sizeof");
    writeln!(
        out,
        "::std::size_t {}::layout::size() noexcept {{",
        ety.name.cxx,
    );
    writeln!(out, "  return {}();", link_name);
    writeln!(out, "}}");

    out.next_section();
    let link_name = mangle::operator(&ety.name, "alignof");
    writeln!(
        out,
        "::std::size_t {}::layout::align() noexcept {{",
        ety.name.cxx,
    );
    writeln!(out, "  return {}();", link_name);
    writeln!(out, "}}");
}

fn begin_function_definition(out: &mut OutFile) {
    if let Some(annotation) = &out.opt.cxx_impl_annotations {
        write!(out, "{} ", annotation);
    }
}

fn write_cxx_function_shim<'a>(out: &mut OutFile<'a>, efn: &'a ExternFn) {
    out.pragma.dollar_in_identifier = true;
    out.pragma.missing_declarations = true;
    out.next_section();
    out.set_namespace(&efn.name.namespace);
    out.begin_block(Block::ExternC);
    begin_function_definition(out);
    if efn.throws {
        out.builtin.ptr_len = true;
        write!(out, "::rust::repr::PtrLen ");
    } else {
        write_extern_return_type_space(out, efn, efn.lang);
    }
    let mangled = mangle::extern_fn(efn, out.types);
    write!(out, "{}(", mangled);
    if let FnKind::Method(receiver) = &efn.kind {
        write!(
            out,
            "{}",
            out.types.resolve(&receiver.ty).name.to_fully_qualified(),
        );
        if !receiver.mutable {
            write!(out, " const");
        }
        write!(out, " &self");
    }
    for (i, arg) in efn.args.iter().enumerate() {
        if i > 0 || matches!(efn.kind, FnKind::Method(_)) {
            write!(out, ", ");
        }
        if arg.ty == RustString {
            write_type_space(out, &arg.ty);
            write!(out, "const *{}", arg.name.cxx);
        } else if let Type::RustVec(_) = arg.ty {
            write_type_space(out, &arg.ty);
            write!(out, "const *{}", arg.name.cxx);
        } else {
            write_extern_arg(out, arg);
        }
    }
    let indirect_return = indirect_return(efn, out.types, efn.lang);
    if indirect_return {
        if !efn.args.is_empty() || matches!(efn.kind, FnKind::Method(_)) {
            write!(out, ", ");
        }
        write_indirect_return_type_space(out, efn.ret.as_ref().unwrap());
        write!(out, "*return$");
    }
    write!(out, ")");
    match efn.lang {
        Lang::Cxx => write!(out, " noexcept"),
        Lang::CxxUnwind => {}
        Lang::Rust => unreachable!(),
    }
    writeln!(out, " {{");
    write!(out, "  ");
    write_return_type(out, &efn.ret);
    match efn.receiver() {
        None => write!(out, "(*{}$)(", efn.name.rust),
        Some(receiver) => write!(
            out,
            "({}::*{}$)(",
            out.types.resolve(&receiver.ty).name.to_fully_qualified(),
            efn.name.rust,
        ),
    }
    for (i, arg) in efn.args.iter().enumerate() {
        if i > 0 {
            write!(out, ", ");
        }
        write_type(out, &arg.ty);
    }
    write!(out, ")");
    if let Some(receiver) = efn.receiver() {
        if !receiver.mutable {
            write!(out, " const");
        }
    }
    write!(out, " = ");
    match efn.self_type() {
        None => write!(out, "{}", efn.name.to_fully_qualified()),
        Some(self_type) => write!(
            out,
            "&{}::{}",
            out.types.resolve(self_type).name.to_fully_qualified(),
            efn.name.cxx,
        ),
    }
    writeln!(out, ";");
    write!(out, "  ");
    if efn.throws {
        out.builtin.ptr_len = true;
        out.builtin.trycatch = true;
        writeln!(out, "::rust::repr::PtrLen throw$;");
        writeln!(out, "  ::rust::behavior::trycatch(");
        writeln!(out, "      [&] {{");
        write!(out, "        ");
    }
    if indirect_return {
        out.include.new = true;
        write!(out, "new (return$) ");
        write_indirect_return_type(out, efn.ret.as_ref().unwrap());
        write!(out, "(");
    } else if efn.ret.is_some() {
        write!(out, "return ");
    }
    match &efn.ret {
        Some(Type::Ref(_)) => write!(out, "&"),
        Some(Type::Str(_)) if !indirect_return => {
            out.builtin.rust_str_repr = true;
            write!(out, "::rust::impl<::rust::Str>::repr(");
        }
        Some(ty @ Type::SliceRef(_)) if !indirect_return => {
            out.builtin.rust_slice_repr = true;
            write!(out, "::rust::impl<");
            write_type(out, ty);
            write!(out, ">::repr(");
        }
        _ => {}
    }
    match efn.receiver() {
        None => write!(out, "{}$(", efn.name.rust),
        Some(_) => write!(out, "(self.*{}$)(", efn.name.rust),
    }
    for (i, arg) in efn.args.iter().enumerate() {
        if i > 0 {
            write!(out, ", ");
        }
        if let Type::RustBox(_) = &arg.ty {
            write_type(out, &arg.ty);
            write!(out, "::from_raw({})", arg.name.cxx);
        } else if let Type::UniquePtr(_) = &arg.ty {
            write_type(out, &arg.ty);
            write!(out, "({})", arg.name.cxx);
        } else if arg.ty == RustString {
            out.builtin.unsafe_bitcopy = true;
            write!(
                out,
                "::rust::String(::rust::unsafe_bitcopy, *{})",
                arg.name.cxx,
            );
        } else if let Type::RustVec(_) = arg.ty {
            out.builtin.unsafe_bitcopy = true;
            write_type(out, &arg.ty);
            write!(out, "(::rust::unsafe_bitcopy, *{})", arg.name.cxx);
        } else if out.types.needs_indirect_abi(&arg.ty) {
            out.include.utility = true;
            write!(out, "::std::move(*{})", arg.name.cxx);
        } else {
            write!(out, "{}", arg.name.cxx);
        }
    }
    write!(out, ")");
    match &efn.ret {
        Some(Type::RustBox(_)) => write!(out, ".into_raw()"),
        Some(Type::UniquePtr(_)) => write!(out, ".release()"),
        Some(Type::Str(_) | Type::SliceRef(_)) if !indirect_return => write!(out, ")"),
        _ => {}
    }
    if indirect_return {
        write!(out, ")");
    }
    writeln!(out, ";");
    if efn.throws {
        writeln!(out, "        throw$.ptr = nullptr;");
        writeln!(out, "      }},");
        writeln!(out, "      ::rust::detail::Fail(throw$));");
        writeln!(out, "  return throw$;");
    }
    writeln!(out, "}}");
    for arg in &efn.args {
        if let Type::Fn(f) = &arg.ty {
            let var = &arg.name;
            write_function_pointer_trampoline(out, efn, var, f);
        }
    }
    out.end_block(Block::ExternC);
}

fn write_function_pointer_trampoline(out: &mut OutFile, efn: &ExternFn, var: &Pair, f: &Signature) {
    out.pragma.return_type_c_linkage = true;

    let r_trampoline = mangle::r_trampoline(efn, var, out.types);
    let indirect_call = true;
    write_rust_function_decl_impl(out, &r_trampoline, f, indirect_call);

    out.next_section();
    let c_trampoline = mangle::c_trampoline(efn, var, out.types).to_string();
    let doc = Doc::new();
    let main = false;
    write_rust_function_shim_impl(
        out,
        &c_trampoline,
        f,
        &doc,
        &r_trampoline,
        indirect_call,
        main,
    );
}

fn write_rust_function_decl<'a>(out: &mut OutFile<'a>, efn: &'a ExternFn) {
    out.set_namespace(&efn.name.namespace);
    out.begin_block(Block::ExternC);
    let link_name = mangle::extern_fn(efn, out.types);
    let indirect_call = false;
    write_rust_function_decl_impl(out, &link_name, efn, indirect_call);
    out.end_block(Block::ExternC);
}

fn write_rust_function_decl_impl(
    out: &mut OutFile,
    link_name: &Symbol,
    sig: &Signature,
    indirect_call: bool,
) {
    out.next_section();
    out.pragma.dollar_in_identifier = true;
    if sig.throws {
        out.builtin.ptr_len = true;
        write!(out, "::rust::repr::PtrLen ");
    } else {
        write_extern_return_type_space(out, sig, Lang::Rust);
    }
    write!(out, "{}(", link_name);
    let mut needs_comma = false;
    if let FnKind::Method(receiver) = &sig.kind {
        write!(
            out,
            "{}",
            out.types.resolve(&receiver.ty).name.to_fully_qualified(),
        );
        if !receiver.mutable {
            write!(out, " const");
        }
        write!(out, " &self");
        needs_comma = true;
    }
    for arg in &sig.args {
        if needs_comma {
            write!(out, ", ");
        }
        write_extern_arg(out, arg);
        needs_comma = true;
    }
    if indirect_return(sig, out.types, Lang::Rust) {
        if needs_comma {
            write!(out, ", ");
        }
        match sig.ret.as_ref().unwrap() {
            Type::Ref(ret) => {
                write_type_space(out, &ret.inner);
                if !ret.mutable {
                    write!(out, "const ");
                }
                write!(out, "*");
            }
            ret => write_type_space(out, ret),
        }
        write!(out, "*return$");
        needs_comma = true;
    }
    if indirect_call {
        if needs_comma {
            write!(out, ", ");
        }
        write!(out, "void *");
    }
    writeln!(out, ") noexcept;");
}

fn write_rust_function_shim<'a>(out: &mut OutFile<'a>, efn: &'a ExternFn) {
    out.set_namespace(&efn.name.namespace);
    let local_name = match efn.self_type() {
        None => efn.name.cxx.to_string(),
        Some(self_type) => format!(
            "{}::{}",
            out.types.resolve(self_type).name.cxx,
            efn.name.cxx,
        ),
    };
    let doc = &efn.doc;
    let invoke = mangle::extern_fn(efn, out.types);
    let indirect_call = false;
    let main = efn.name.cxx == *"main"
        && efn.name.namespace == Namespace::ROOT
        && efn.sig.asyncness.is_none()
        && matches!(efn.kind, FnKind::Free)
        && efn.sig.args.is_empty()
        && efn.sig.ret.is_none()
        && !efn.sig.throws;
    write_rust_function_shim_impl(out, &local_name, efn, doc, &invoke, indirect_call, main);
}

fn write_rust_function_shim_decl(
    out: &mut OutFile,
    local_name: &str,
    sig: &Signature,
    in_class: bool,
    indirect_call: bool,
    main: bool,
) {
    begin_function_definition(out);
    if matches!(sig.kind, FnKind::Assoc(_)) && in_class {
        write!(out, "static ");
    }
    if main {
        write!(out, "int ");
    } else {
        write_return_type(out, &sig.ret);
    }
    write!(out, "{}(", local_name);
    for (i, arg) in sig.args.iter().enumerate() {
        if i > 0 {
            write!(out, ", ");
        }
        write_type_space(out, &arg.ty);
        write!(out, "{}", arg.name.cxx);
    }
    if indirect_call {
        if !sig.args.is_empty() {
            write!(out, ", ");
        }
        write!(out, "void *extern$");
    }
    write!(out, ")");
    if let FnKind::Method(receiver) = &sig.kind {
        if !receiver.mutable {
            write!(out, " const");
        }
    }
    if !sig.throws {
        write!(out, " noexcept");
    }
}

fn write_rust_function_shim_impl(
    out: &mut OutFile,
    local_name: &str,
    sig: &Signature,
    doc: &Doc,
    invoke: &Symbol,
    indirect_call: bool,
    main: bool,
) {
    if match sig.kind {
        FnKind::Free => false,
        FnKind::Method(_) | FnKind::Assoc(_) => out.header,
    } {
        // We've already defined this inside the struct.
        return;
    }
    out.pragma.dollar_in_identifier = true;
    if matches!(sig.kind, FnKind::Free) {
        // Member functions already documented at their declaration.
        write_doc(out, "", doc);
    }
    let in_class = false;
    write_rust_function_shim_decl(out, local_name, sig, in_class, indirect_call, main);
    if out.header {
        writeln!(out, ";");
        return;
    }
    writeln!(out, " {{");
    for arg in &sig.args {
        if arg.ty != RustString && out.types.needs_indirect_abi(&arg.ty) {
            out.include.utility = true;
            out.builtin.manually_drop = true;
            write!(out, "  ::rust::ManuallyDrop<");
            write_type(out, &arg.ty);
            writeln!(out, "> {}$(::std::move({0}));", arg.name.cxx);
        }
    }
    write!(out, "  ");
    let indirect_return = indirect_return(sig, out.types, Lang::Rust);
    if indirect_return {
        out.builtin.maybe_uninit = true;
        write!(out, "::rust::MaybeUninit<");
        match sig.ret.as_ref().unwrap() {
            Type::Ref(ret) => {
                write_type_space(out, &ret.inner);
                if !ret.mutable {
                    write!(out, "const ");
                }
                write!(out, "*");
            }
            ret => write_type(out, ret),
        }
        writeln!(out, "> return$;");
        write!(out, "  ");
    } else if let Some(ret) = &sig.ret {
        write!(out, "return ");
        match ret {
            Type::RustBox(_) => {
                write_type(out, ret);
                write!(out, "::from_raw(");
            }
            Type::UniquePtr(_) => {
                write_type(out, ret);
                write!(out, "(");
            }
            Type::Ref(_) => write!(out, "*"),
            Type::Str(_) => {
                out.builtin.rust_str_new_unchecked = true;
                write!(out, "::rust::impl<::rust::Str>::new_unchecked(");
            }
            Type::SliceRef(_) => {
                out.builtin.rust_slice_new = true;
                write!(out, "::rust::impl<");
                write_type(out, ret);
                write!(out, ">::slice(");
            }
            _ => {}
        }
    }
    if sig.throws {
        out.builtin.ptr_len = true;
        write!(out, "::rust::repr::PtrLen error$ = ");
    }
    write!(out, "{}(", invoke);
    let mut needs_comma = false;
    if matches!(sig.kind, FnKind::Method(_)) {
        write!(out, "*this");
        needs_comma = true;
    }
    for arg in &sig.args {
        if needs_comma {
            write!(out, ", ");
        }
        if out.types.needs_indirect_abi(&arg.ty) {
            write!(out, "&");
        }
        write!(out, "{}", arg.name.cxx);
        match &arg.ty {
            Type::RustBox(_) => write!(out, ".into_raw()"),
            Type::UniquePtr(_) => write!(out, ".release()"),
            ty if ty != RustString && out.types.needs_indirect_abi(ty) => write!(out, "$.value"),
            _ => {}
        }
        needs_comma = true;
    }
    if indirect_return {
        if needs_comma {
            write!(out, ", ");
        }
        write!(out, "&return$.value");
        needs_comma = true;
    }
    if indirect_call {
        if needs_comma {
            write!(out, ", ");
        }
        write!(out, "extern$");
    }
    write!(out, ")");
    if !indirect_return {
        if let Some(Type::RustBox(_) | Type::UniquePtr(_) | Type::Str(_) | Type::SliceRef(_)) =
            &sig.ret
        {
            write!(out, ")");
        }
    }
    writeln!(out, ";");
    if sig.throws {
        out.builtin.rust_error = true;
        writeln!(out, "  if (error$.ptr) {{");
        writeln!(out, "    throw ::rust::impl<::rust::Error>::error(error$);");
        writeln!(out, "  }}");
    }
    if indirect_return {
        write!(out, "  return ");
        match sig.ret.as_ref().unwrap() {
            Type::Ref(_) => write!(out, "*return$.value"),
            _ => {
                out.include.utility = true;
                write!(out, "::std::move(return$.value)");
            }
        }
        writeln!(out, ";");
    }
    writeln!(out, "}}");
}

fn write_return_type(out: &mut OutFile, ty: &Option<Type>) {
    match ty {
        None => write!(out, "void "),
        Some(ty) => write_type_space(out, ty),
    }
}

fn indirect_return(sig: &Signature, types: &Types, lang: Lang) -> bool {
    sig.ret.as_ref().is_some_and(|ret| {
        sig.throws
            || types.needs_indirect_abi(ret)
            || match lang {
                Lang::Cxx | Lang::CxxUnwind => types.contains_elided_lifetime(ret),
                Lang::Rust => false,
            }
    })
}

fn write_indirect_return_type(out: &mut OutFile, ty: &Type) {
    match ty {
        Type::RustBox(ty) | Type::UniquePtr(ty) => {
            write_type_space(out, &ty.inner);
            write!(out, "*");
        }
        Type::Ref(ty) => {
            write_type_space(out, &ty.inner);
            if !ty.mutable {
                write!(out, "const ");
            }
            write!(out, "*");
        }
        _ => write_type(out, ty),
    }
}

fn write_indirect_return_type_space(out: &mut OutFile, ty: &Type) {
    write_indirect_return_type(out, ty);
    match ty {
        Type::RustBox(_) | Type::UniquePtr(_) | Type::Ref(_) => {}
        Type::Str(_) | Type::SliceRef(_) => write!(out, " "),
        _ => write_space_after_type(out, ty),
    }
}

fn write_extern_return_type_space(out: &mut OutFile, sig: &Signature, lang: Lang) {
    match &sig.ret {
        Some(_) if indirect_return(sig, out.types, lang) => write!(out, "void "),
        Some(Type::RustBox(ty) | Type::UniquePtr(ty)) => {
            write_type_space(out, &ty.inner);
            write!(out, "*");
        }
        Some(Type::Ref(ty)) => {
            write_type_space(out, &ty.inner);
            if !ty.mutable {
                write!(out, "const ");
            }
            write!(out, "*");
        }
        Some(Type::Str(_) | Type::SliceRef(_)) => {
            out.builtin.repr_fat = true;
            write!(out, "::rust::repr::Fat ");
        }
        ty => write_return_type(out, ty),
    }
}

fn write_extern_arg(out: &mut OutFile, arg: &Var) {
    match &arg.ty {
        Type::RustBox(ty) | Type::UniquePtr(ty) | Type::CxxVector(ty) => {
            write_type_space(out, &ty.inner);
            write!(out, "*");
        }
        _ => write_type_space(out, &arg.ty),
    }
    if out.types.needs_indirect_abi(&arg.ty) {
        write!(out, "*");
    }
    write!(out, "{}", arg.name.cxx);
}

fn write_type(out: &mut OutFile, ty: &Type) {
    match ty {
        Type::Ident(ident) => match Atom::from(&ident.rust) {
            Some(atom) => write_atom(out, atom),
            None => write!(
                out,
                "{}",
                out.types.resolve(ident).name.to_fully_qualified(),
            ),
        },
        Type::RustBox(ty) => {
            write!(out, "::rust::Box<");
            write_type(out, &ty.inner);
            write!(out, ">");
        }
        Type::RustVec(ty) => {
            write!(out, "::rust::Vec<");
            write_type(out, &ty.inner);
            write!(out, ">");
        }
        Type::UniquePtr(ptr) => {
            write!(out, "::std::unique_ptr<");
            write_type(out, &ptr.inner);
            write!(out, ">");
        }
        Type::SharedPtr(ptr) => {
            write!(out, "::std::shared_ptr<");
            write_type(out, &ptr.inner);
            write!(out, ">");
        }
        Type::WeakPtr(ptr) => {
            write!(out, "::std::weak_ptr<");
            write_type(out, &ptr.inner);
            write!(out, ">");
        }
        Type::CxxVector(ty) => {
            write!(out, "::std::vector<");
            write_type(out, &ty.inner);
            write!(out, ">");
        }
        Type::Ref(r) => {
            write_type_space(out, &r.inner);
            if !r.mutable {
                write!(out, "const ");
            }
            write!(out, "&");
        }
        Type::Ptr(p) => {
            write_type_space(out, &p.inner);
            if !p.mutable {
                write!(out, "const ");
            }
            write!(out, "*");
        }
        Type::Str(_) => {
            write!(out, "::rust::Str");
        }
        Type::SliceRef(slice) => {
            write!(out, "::rust::Slice<");
            write_type_space(out, &slice.inner);
            if slice.mutability.is_none() {
                write!(out, "const");
            }
            write!(out, ">");
        }
        Type::Fn(f) => {
            write!(out, "::rust::Fn<");
            match &f.ret {
                Some(ret) => write_type(out, ret),
                None => write!(out, "void"),
            }
            write!(out, "(");
            for (i, arg) in f.args.iter().enumerate() {
                if i > 0 {
                    write!(out, ", ");
                }
                write_type(out, &arg.ty);
            }
            write!(out, ")>");
        }
        Type::Array(a) => {
            write!(out, "::std::array<");
            write_type(out, &a.inner);
            write!(out, ", {}>", &a.len);
        }
        Type::Void(_) => unreachable!(),
    }
}

fn write_atom(out: &mut OutFile, atom: Atom) {
    match atom {
        Bool => write!(out, "bool"),
        Char => write!(out, "char"),
        U8 => write!(out, "::std::uint8_t"),
        U16 => write!(out, "::std::uint16_t"),
        U32 => write!(out, "::std::uint32_t"),
        U64 => write!(out, "::std::uint64_t"),
        Usize => write!(out, "::std::size_t"),
        I8 => write!(out, "::std::int8_t"),
        I16 => write!(out, "::std::int16_t"),
        I32 => write!(out, "::std::int32_t"),
        I64 => write!(out, "::std::int64_t"),
        Isize => write!(out, "::rust::isize"),
        F32 => write!(out, "float"),
        F64 => write!(out, "double"),
        CxxString => write!(out, "::std::string"),
        RustString => write!(out, "::rust::String"),
    }
}

fn write_type_space(out: &mut OutFile, ty: &Type) {
    write_type(out, ty);
    write_space_after_type(out, ty);
}

fn write_space_after_type(out: &mut OutFile, ty: &Type) {
    match ty {
        Type::Ident(_)
        | Type::RustBox(_)
        | Type::UniquePtr(_)
        | Type::SharedPtr(_)
        | Type::WeakPtr(_)
        | Type::Str(_)
        | Type::CxxVector(_)
        | Type::RustVec(_)
        | Type::SliceRef(_)
        | Type::Fn(_)
        | Type::Array(_) => write!(out, " "),
        Type::Ref(_) | Type::Ptr(_) => {}
        Type::Void(_) => unreachable!(),
    }
}

#[derive(Copy, Clone)]
enum UniquePtr<'a> {
    Ident(&'a Ident),
    CxxVector(&'a Ident),
}

trait ToTypename {
    fn to_typename(&self, types: &Types) -> String;
}

impl ToTypename for Ident {
    fn to_typename(&self, types: &Types) -> String {
        types.resolve(self).name.to_fully_qualified()
    }
}

impl<'a> ToTypename for UniquePtr<'a> {
    fn to_typename(&self, types: &Types) -> String {
        match self {
            UniquePtr::Ident(ident) => ident.to_typename(types),
            UniquePtr::CxxVector(element) => {
                format!("::std::vector<{}>", element.to_typename(types))
            }
        }
    }
}

trait ToMangled {
    fn to_mangled(&self, types: &Types) -> Symbol;
}

impl ToMangled for Ident {
    fn to_mangled(&self, types: &Types) -> Symbol {
        types.resolve(self).name.to_symbol()
    }
}

impl<'a> ToMangled for UniquePtr<'a> {
    fn to_mangled(&self, types: &Types) -> Symbol {
        match self {
            UniquePtr::Ident(ident) => ident.to_mangled(types),
            UniquePtr::CxxVector(element) => {
                symbol::join(&[&"std", &"vector", &element.to_mangled(types)])
            }
        }
    }
}

fn write_generic_instantiations(out: &mut OutFile) {
    if out.header {
        return;
    }

    out.next_section();
    out.set_namespace(Default::default());
    out.begin_block(Block::ExternC);
    for impl_key in out.types.impls.keys() {
        out.next_section();
        match impl_key {
            ImplKey::RustBox(ident) => write_rust_box_extern(out, ident),
            ImplKey::RustVec(ident) => write_rust_vec_extern(out, ident),
            ImplKey::UniquePtr(ident) => write_unique_ptr(out, ident),
            ImplKey::SharedPtr(ident) => write_shared_ptr(out, ident),
            ImplKey::WeakPtr(ident) => write_weak_ptr(out, ident),
            ImplKey::CxxVector(ident) => write_cxx_vector(out, ident),
        }
    }
    out.end_block(Block::ExternC);

    out.begin_block(Block::Namespace("rust"));
    out.begin_block(Block::InlineNamespace("cxxbridge1"));
    for impl_key in out.types.impls.keys() {
        match impl_key {
            ImplKey::RustBox(ident) => write_rust_box_impl(out, ident),
            ImplKey::RustVec(ident) => write_rust_vec_impl(out, ident),
            _ => {}
        }
    }
    out.end_block(Block::InlineNamespace("cxxbridge1"));
    out.end_block(Block::Namespace("rust"));
}

fn write_rust_box_extern(out: &mut OutFile, key: &NamedImplKey) {
    let resolve = out.types.resolve(key);
    let inner = resolve.name.to_fully_qualified();
    let instance = resolve.name.to_symbol();

    out.pragma.dollar_in_identifier = true;

    writeln!(
        out,
        "{} *cxxbridge1$box${}$alloc() noexcept;",
        inner, instance,
    );
    writeln!(
        out,
        "void cxxbridge1$box${}$dealloc({} *) noexcept;",
        instance, inner,
    );
    writeln!(
        out,
        "void cxxbridge1$box${}$drop(::rust::Box<{}> *ptr) noexcept;",
        instance, inner,
    );
}

fn write_rust_vec_extern(out: &mut OutFile, key: &NamedImplKey) {
    let element = key.rust;
    let inner = element.to_typename(out.types);
    let instance = element.to_mangled(out.types);

    out.include.cstddef = true;
    out.pragma.dollar_in_identifier = true;

    writeln!(
        out,
        "void cxxbridge1$rust_vec${}$new(::rust::Vec<{}> const *ptr) noexcept;",
        instance, inner,
    );
    writeln!(
        out,
        "void cxxbridge1$rust_vec${}$drop(::rust::Vec<{}> *ptr) noexcept;",
        instance, inner,
    );
    writeln!(
        out,
        "::std::size_t cxxbridge1$rust_vec${}$len(::rust::Vec<{}> const *ptr) noexcept;",
        instance, inner,
    );
    writeln!(
        out,
        "::std::size_t cxxbridge1$rust_vec${}$capacity(::rust::Vec<{}> const *ptr) noexcept;",
        instance, inner,
    );
    writeln!(
        out,
        "{} const *cxxbridge1$rust_vec${}$data(::rust::Vec<{0}> const *ptr) noexcept;",
        inner, instance,
    );
    writeln!(
        out,
        "void cxxbridge1$rust_vec${}$reserve_total(::rust::Vec<{}> *ptr, ::std::size_t new_cap) noexcept;",
        instance, inner,
    );
    writeln!(
        out,
        "void cxxbridge1$rust_vec${}$set_len(::rust::Vec<{}> *ptr, ::std::size_t len) noexcept;",
        instance, inner,
    );
    writeln!(
        out,
        "void cxxbridge1$rust_vec${}$truncate(::rust::Vec<{}> *ptr, ::std::size_t len) noexcept;",
        instance, inner,
    );
}

fn write_rust_box_impl(out: &mut OutFile, key: &NamedImplKey) {
    let resolve = out.types.resolve(key);
    let inner = resolve.name.to_fully_qualified();
    let instance = resolve.name.to_symbol();

    out.pragma.dollar_in_identifier = true;

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(
        out,
        "{} *Box<{}>::allocation::alloc() noexcept {{",
        inner, inner,
    );
    writeln!(out, "  return cxxbridge1$box${}$alloc();", instance);
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(
        out,
        "void Box<{}>::allocation::dealloc({} *ptr) noexcept {{",
        inner, inner,
    );
    writeln!(out, "  cxxbridge1$box${}$dealloc(ptr);", instance);
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(out, "void Box<{}>::drop() noexcept {{", inner);
    writeln!(out, "  cxxbridge1$box${}$drop(this);", instance);
    writeln!(out, "}}");
}

fn write_rust_vec_impl(out: &mut OutFile, key: &NamedImplKey) {
    let element = key.rust;
    let inner = element.to_typename(out.types);
    let instance = element.to_mangled(out.types);

    out.include.cstddef = true;
    out.pragma.dollar_in_identifier = true;

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(out, "Vec<{}>::Vec() noexcept {{", inner);
    writeln!(out, "  cxxbridge1$rust_vec${}$new(this);", instance);
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(out, "void Vec<{}>::drop() noexcept {{", inner);
    writeln!(out, "  return cxxbridge1$rust_vec${}$drop(this);", instance);
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(
        out,
        "::std::size_t Vec<{}>::size() const noexcept {{",
        inner,
    );
    writeln!(out, "  return cxxbridge1$rust_vec${}$len(this);", instance);
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(
        out,
        "::std::size_t Vec<{}>::capacity() const noexcept {{",
        inner,
    );
    writeln!(
        out,
        "  return cxxbridge1$rust_vec${}$capacity(this);",
        instance,
    );
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(out, "{} const *Vec<{0}>::data() const noexcept {{", inner);
    writeln!(out, "  return cxxbridge1$rust_vec${}$data(this);", instance);
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(
        out,
        "void Vec<{}>::reserve_total(::std::size_t new_cap) noexcept {{",
        inner,
    );
    writeln!(
        out,
        "  return cxxbridge1$rust_vec${}$reserve_total(this, new_cap);",
        instance,
    );
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(
        out,
        "void Vec<{}>::set_len(::std::size_t len) noexcept {{",
        inner,
    );
    writeln!(
        out,
        "  return cxxbridge1$rust_vec${}$set_len(this, len);",
        instance,
    );
    writeln!(out, "}}");

    writeln!(out, "template <>");
    begin_function_definition(out);
    writeln!(out, "void Vec<{}>::truncate(::std::size_t len) {{", inner,);
    writeln!(
        out,
        "  return cxxbridge1$rust_vec${}$truncate(this, len);",
        instance,
    );
    writeln!(out, "}}");
}

fn write_unique_ptr(out: &mut OutFile, key: &NamedImplKey) {
    let ty = UniquePtr::Ident(key.rust);
    write_unique_ptr_common(out, ty);
}

// Shared by UniquePtr<T> and UniquePtr<CxxVector<T>>.
fn write_unique_ptr_common(out: &mut OutFile, ty: UniquePtr) {
    out.include.new = true;
    out.include.utility = true;
    out.pragma.dollar_in_identifier = true;
    out.pragma.missing_declarations = true;

    let inner = ty.to_typename(out.types);
    let instance = ty.to_mangled(out.types);

    let can_construct_from_value = match ty {
        // Some aliases are to opaque types; some are to trivial types. We can't
        // know at code generation time, so we generate both C++ and Rust side
        // bindings for a "new" method anyway. But the Rust code can't be called
        // for Opaque types because the 'new' method is not implemented.
        UniquePtr::Ident(ident) => out.types.is_maybe_trivial(ident),
        UniquePtr::CxxVector(_) => false,
    };

    out.builtin.is_complete = true;
    writeln!(
        out,
        "static_assert(::rust::detail::is_complete<::std::remove_extent<{}>::type>::value, \"definition of `{}` is required\");",
        inner, inner,
    );
    writeln!(
        out,
        "static_assert(sizeof(::std::unique_ptr<{}>) == sizeof(void *), \"\");",
        inner,
    );
    writeln!(
        out,
        "static_assert(alignof(::std::unique_ptr<{}>) == alignof(void *), \"\");",
        inner,
    );

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$unique_ptr${}$null(::std::unique_ptr<{}> *ptr) noexcept {{",
        instance, inner,
    );
    writeln!(out, "  ::new (ptr) ::std::unique_ptr<{}>();", inner);
    writeln!(out, "}}");

    if can_construct_from_value {
        out.builtin.maybe_uninit = true;
        begin_function_definition(out);
        writeln!(
            out,
            "{} *cxxbridge1$unique_ptr${}$uninit(::std::unique_ptr<{}> *ptr) noexcept {{",
            inner, instance, inner,
        );
        writeln!(
            out,
            "  {} *uninit = reinterpret_cast<{} *>(new ::rust::MaybeUninit<{}>);",
            inner, inner, inner,
        );
        writeln!(out, "  ::new (ptr) ::std::unique_ptr<{}>(uninit);", inner);
        writeln!(out, "  return uninit;");
        writeln!(out, "}}");
    }

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$unique_ptr${}$raw(::std::unique_ptr<{}> *ptr, ::std::unique_ptr<{}>::pointer raw) noexcept {{",
        instance, inner, inner,
    );
    writeln!(out, "  ::new (ptr) ::std::unique_ptr<{}>(raw);", inner);
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "::std::unique_ptr<{}>::element_type const *cxxbridge1$unique_ptr${}$get(::std::unique_ptr<{}> const &ptr) noexcept {{",
        inner, instance, inner,
    );
    writeln!(out, "  return ptr.get();");
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "::std::unique_ptr<{}>::pointer cxxbridge1$unique_ptr${}$release(::std::unique_ptr<{}> &ptr) noexcept {{",
        inner, instance, inner,
    );
    writeln!(out, "  return ptr.release();");
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$unique_ptr${}$drop(::std::unique_ptr<{}> *ptr) noexcept {{",
        instance, inner,
    );
    out.builtin.deleter_if = true;
    writeln!(
        out,
        "  ::rust::deleter_if<::rust::detail::is_complete<{}>::value>{{}}(ptr);",
        inner,
    );
    writeln!(out, "}}");
}

fn write_shared_ptr(out: &mut OutFile, key: &NamedImplKey) {
    let ident = key.rust;
    let resolve = out.types.resolve(ident);
    let inner = resolve.name.to_fully_qualified();
    let instance = resolve.name.to_symbol();

    out.include.new = true;
    out.include.utility = true;
    out.pragma.dollar_in_identifier = true;
    out.pragma.missing_declarations = true;

    // Some aliases are to opaque types; some are to trivial types. We can't
    // know at code generation time, so we generate both C++ and Rust side
    // bindings for a "new" method anyway. But the Rust code can't be called for
    // Opaque types because the 'new' method is not implemented.
    let can_construct_from_value = out.types.is_maybe_trivial(ident);

    writeln!(
        out,
        "static_assert(sizeof(::std::shared_ptr<{}>) == 2 * sizeof(void *), \"\");",
        inner,
    );
    writeln!(
        out,
        "static_assert(alignof(::std::shared_ptr<{}>) == alignof(void *), \"\");",
        inner,
    );

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$shared_ptr${}$null(::std::shared_ptr<{}> *ptr) noexcept {{",
        instance, inner,
    );
    writeln!(out, "  ::new (ptr) ::std::shared_ptr<{}>();", inner);
    writeln!(out, "}}");

    if can_construct_from_value {
        out.builtin.maybe_uninit = true;
        begin_function_definition(out);
        writeln!(
            out,
            "{} *cxxbridge1$shared_ptr${}$uninit(::std::shared_ptr<{}> *ptr) noexcept {{",
            inner, instance, inner,
        );
        writeln!(
            out,
            "  {} *uninit = reinterpret_cast<{} *>(new ::rust::MaybeUninit<{}>);",
            inner, inner, inner,
        );
        writeln!(out, "  ::new (ptr) ::std::shared_ptr<{}>(uninit);", inner);
        writeln!(out, "  return uninit;");
        writeln!(out, "}}");
    }

    out.builtin.shared_ptr = true;
    begin_function_definition(out);
    writeln!(
        out,
        "bool cxxbridge1$shared_ptr${}$raw(::std::shared_ptr<{}> *ptr, ::std::shared_ptr<{}>::element_type *raw) noexcept {{",
        instance, inner, inner,
    );
    writeln!(
        out,
        "  ::new (ptr) ::rust::shared_ptr_if_destructible<{}>(raw);",
        inner,
    );
    writeln!(out, "  return ::rust::is_destructible<{}>::value;", inner);
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$shared_ptr${}$clone(::std::shared_ptr<{}> const &self, ::std::shared_ptr<{}> *ptr) noexcept {{",
        instance, inner, inner,
    );
    writeln!(out, "  ::new (ptr) ::std::shared_ptr<{}>(self);", inner);
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "::std::shared_ptr<{}>::element_type const *cxxbridge1$shared_ptr${}$get(::std::shared_ptr<{}> const &self) noexcept {{",
        inner, instance, inner,
    );
    writeln!(out, "  return self.get();");
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$shared_ptr${}$drop(::std::shared_ptr<{}> *self) noexcept {{",
        instance, inner,
    );
    writeln!(out, "  self->~shared_ptr();");
    writeln!(out, "}}");
}

fn write_weak_ptr(out: &mut OutFile, key: &NamedImplKey) {
    let resolve = out.types.resolve(key);
    let inner = resolve.name.to_fully_qualified();
    let instance = resolve.name.to_symbol();

    out.include.new = true;
    out.include.utility = true;
    out.pragma.dollar_in_identifier = true;
    out.pragma.missing_declarations = true;

    writeln!(
        out,
        "static_assert(sizeof(::std::weak_ptr<{}>) == 2 * sizeof(void *), \"\");",
        inner,
    );
    writeln!(
        out,
        "static_assert(alignof(::std::weak_ptr<{}>) == alignof(void *), \"\");",
        inner,
    );

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$weak_ptr${}$null(::std::weak_ptr<{}> *ptr) noexcept {{",
        instance, inner,
    );
    writeln!(out, "  ::new (ptr) ::std::weak_ptr<{}>();", inner);
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$weak_ptr${}$clone(::std::weak_ptr<{}> const &self, ::std::weak_ptr<{}> *ptr) noexcept {{",
        instance, inner, inner,
    );
    writeln!(out, "  ::new (ptr) ::std::weak_ptr<{}>(self);", inner);
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$weak_ptr${}$downgrade(::std::shared_ptr<{}> const &shared, ::std::weak_ptr<{}> *weak) noexcept {{",
        instance, inner, inner,
    );
    writeln!(out, "  ::new (weak) ::std::weak_ptr<{}>(shared);", inner);
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$weak_ptr${}$upgrade(::std::weak_ptr<{}> const &weak, ::std::shared_ptr<{}> *shared) noexcept {{",
        instance, inner, inner,
    );
    writeln!(
        out,
        "  ::new (shared) ::std::shared_ptr<{}>(weak.lock());",
        inner,
    );
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "void cxxbridge1$weak_ptr${}$drop(::std::weak_ptr<{}> *self) noexcept {{",
        instance, inner,
    );
    writeln!(out, "  self->~weak_ptr();");
    writeln!(out, "}}");
}

fn write_cxx_vector(out: &mut OutFile, key: &NamedImplKey) {
    let element = key.rust;
    let inner = element.to_typename(out.types);
    let instance = element.to_mangled(out.types);

    out.include.cstddef = true;
    out.include.utility = true;
    out.builtin.destroy = true;
    out.builtin.vector = true;
    out.pragma.dollar_in_identifier = true;
    out.pragma.missing_declarations = true;

    begin_function_definition(out);
    writeln!(
        out,
        "::std::vector<{}> *cxxbridge1$std$vector${}$new() noexcept {{",
        inner, instance,
    );
    writeln!(out, "  return new ::std::vector<{}>();", inner);
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "::std::size_t cxxbridge1$std$vector${}$size(::std::vector<{}> const &s) noexcept {{",
        instance, inner,
    );
    writeln!(out, "  return s.size();");
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "::std::size_t cxxbridge1$std$vector${}$capacity(::std::vector<{}> const &s) noexcept {{",
        instance, inner,
    );
    writeln!(out, "  return s.capacity();");
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "{} *cxxbridge1$std$vector${}$get_unchecked(::std::vector<{}> *s, ::std::size_t pos) noexcept {{",
        inner, instance, inner,
    );
    writeln!(out, "  return &(*s)[pos];");
    writeln!(out, "}}");

    begin_function_definition(out);
    writeln!(
        out,
        "bool cxxbridge1$std$vector${}$reserve(::std::vector<{}> *s, ::std::size_t new_cap) noexcept {{",
        instance, inner,
    );
    writeln!(
        out,
        "  return ::rust::if_move_constructible<{}>::reserve(*s, new_cap);",
        inner,
    );
    writeln!(out, "}}");

    if out.types.is_maybe_trivial(element) {
        begin_function_definition(out);
        writeln!(
            out,
            "void cxxbridge1$std$vector${}$push_back(::std::vector<{}> *v, {} *value) noexcept {{",
            instance, inner, inner,
        );
        writeln!(out, "  v->push_back(::std::move(*value));");
        writeln!(out, "  ::rust::destroy(value);");
        writeln!(out, "}}");

        begin_function_definition(out);
        writeln!(
            out,
            "void cxxbridge1$std$vector${}$pop_back(::std::vector<{}> *v, {} *out) noexcept {{",
            instance, inner, inner,
        );
        writeln!(out, "  ::new (out) {}(::std::move(v->back()));", inner);
        writeln!(out, "  v->pop_back();");
        writeln!(out, "}}");
    }

    out.include.memory = true;
    write_unique_ptr_common(out, UniquePtr::CxxVector(element));
}
