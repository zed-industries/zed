use crate::syntax::atom::Atom::{self, *};
use crate::syntax::message::Message;
use crate::syntax::report::Errors;
use crate::syntax::visit::{self, Visit};
use crate::syntax::{
    error, ident, trivial, Api, Array, Enum, ExternFn, ExternType, FnKind, Impl, Lang, Lifetimes,
    NamedType, Ptr, Receiver, Ref, Signature, SliceRef, Struct, Trait, Ty1, Type, TypeAlias, Types,
};
use proc_macro2::{Delimiter, Group, Ident, TokenStream};
use quote::{quote, ToTokens};
use std::fmt::Display;
use syn::{GenericParam, Generics, Lifetime};

pub(crate) struct Check<'a> {
    apis: &'a [Api],
    types: &'a Types<'a>,
    errors: &'a mut Errors,
    generator: Generator,
}

pub(crate) enum Generator {
    // cxx-build crate, cxxbridge cli, cxx-gen.
    #[cfg_attr(proc_macro, expect(dead_code))]
    Build,
    // cxxbridge-macro. This is relevant in that the macro output is going to
    // get fed straight to rustc, so for errors that rustc already contains
    // logic to catch (probably with a better diagnostic than what the proc
    // macro API is able to produce), we avoid duplicating them in our own
    // diagnostics.
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    Macro,
}

pub(crate) fn typecheck(cx: &mut Errors, apis: &[Api], types: &Types, generator: Generator) {
    do_typecheck(&mut Check {
        apis,
        types,
        errors: cx,
        generator,
    });
}

fn do_typecheck(cx: &mut Check) {
    ident::check_all(cx, cx.apis);

    for ty in cx.types {
        match ty {
            Type::Ident(ident) => check_type_ident(cx, ident),
            Type::RustBox(ptr) => check_type_box(cx, ptr),
            Type::RustVec(ty) => check_type_rust_vec(cx, ty),
            Type::UniquePtr(ptr) => check_type_unique_ptr(cx, ptr),
            Type::SharedPtr(ptr) => check_type_shared_ptr(cx, ptr),
            Type::WeakPtr(ptr) => check_type_weak_ptr(cx, ptr),
            Type::CxxVector(ptr) => check_type_cxx_vector(cx, ptr),
            Type::Ref(ty) => check_type_ref(cx, ty),
            Type::Ptr(ty) => check_type_ptr(cx, ty),
            Type::Array(array) => check_type_array(cx, array),
            Type::Fn(ty) => check_type_fn(cx, ty),
            Type::SliceRef(ty) => check_type_slice_ref(cx, ty),
            Type::Str(_) | Type::Void(_) => {}
        }
    }

    for api in cx.apis {
        match api {
            Api::Include(_) => {}
            Api::Struct(strct) => check_api_struct(cx, strct),
            Api::Enum(enm) => check_api_enum(cx, enm),
            Api::CxxType(ety) | Api::RustType(ety) => check_api_type(cx, ety),
            Api::CxxFunction(efn) | Api::RustFunction(efn) => check_api_fn(cx, efn),
            Api::TypeAlias(alias) => check_api_type_alias(cx, alias),
            Api::Impl(imp) => check_api_impl(cx, imp),
        }
    }
}

impl Check<'_> {
    pub(crate) fn error(&mut self, sp: impl ToTokens, msg: impl Display) {
        self.errors.error(sp, msg);
    }
}

fn check_type_ident(cx: &mut Check, name: &NamedType) {
    let ident = &name.rust;
    if Atom::from(ident).is_none()
        && !cx.types.structs.contains_key(ident)
        && !cx.types.enums.contains_key(ident)
        && !cx.types.cxx.contains(ident)
        && !cx.types.rust.contains(ident)
    {
        let msg = format!("unsupported type: {}", ident);
        cx.error(ident, msg);
    }
}

fn check_type_box(cx: &mut Check, ptr: &Ty1) {
    if let Type::Ident(ident) = &ptr.inner {
        if cx.types.cxx.contains(&ident.rust)
            && !cx.types.aliases.contains_key(&ident.rust)
            && !cx.types.structs.contains_key(&ident.rust)
            && !cx.types.enums.contains_key(&ident.rust)
        {
            cx.error(ptr, error::BOX_CXX_TYPE.msg);
        }

        if Atom::from(&ident.rust).is_none() {
            return;
        }
    }

    cx.error(ptr, "unsupported target type of Box");
}

fn check_type_rust_vec(cx: &mut Check, ty: &Ty1) {
    match &ty.inner {
        Type::Ident(ident) => {
            if cx.types.cxx.contains(&ident.rust)
                && !cx.types.aliases.contains_key(&ident.rust)
                && !cx.types.structs.contains_key(&ident.rust)
                && !cx.types.enums.contains_key(&ident.rust)
            {
                cx.error(ty, "Rust Vec containing C++ type is not supported yet");
                return;
            }

            match Atom::from(&ident.rust) {
                None
                | Some(
                    Bool | Char | U8 | U16 | U32 | U64 | Usize | I8 | I16 | I32 | I64 | Isize | F32
                    | F64 | RustString,
                ) => return,
                Some(CxxString) => {}
            }
        }
        Type::Str(_) => return,
        _ => {}
    }

    cx.error(ty, "unsupported element type of Vec");
}

fn check_type_unique_ptr(cx: &mut Check, ptr: &Ty1) {
    if let Type::Ident(ident) = &ptr.inner {
        if cx.types.rust.contains(&ident.rust) {
            cx.error(ptr, "unique_ptr of a Rust type is not supported yet");
            return;
        }

        match Atom::from(&ident.rust) {
            None | Some(CxxString) => return,
            _ => {}
        }
    } else if let Type::CxxVector(_) = &ptr.inner {
        return;
    }

    cx.error(ptr, "unsupported unique_ptr target type");
}

fn check_type_shared_ptr(cx: &mut Check, ptr: &Ty1) {
    if let Type::Ident(ident) = &ptr.inner {
        if cx.types.rust.contains(&ident.rust) {
            cx.error(ptr, "shared_ptr of a Rust type is not supported yet");
            return;
        }

        match Atom::from(&ident.rust) {
            None
            | Some(
                Bool | U8 | U16 | U32 | U64 | Usize | I8 | I16 | I32 | I64 | Isize | F32 | F64
                | CxxString,
            ) => return,
            Some(Char | RustString) => {}
        }
    } else if let Type::CxxVector(_) = &ptr.inner {
        cx.error(ptr, "std::shared_ptr<std::vector> is not supported yet");
        return;
    }

    cx.error(ptr, "unsupported shared_ptr target type");
}

fn check_type_weak_ptr(cx: &mut Check, ptr: &Ty1) {
    if let Type::Ident(ident) = &ptr.inner {
        if cx.types.rust.contains(&ident.rust) {
            cx.error(ptr, "weak_ptr of a Rust type is not supported yet");
            return;
        }

        match Atom::from(&ident.rust) {
            None
            | Some(
                Bool | U8 | U16 | U32 | U64 | Usize | I8 | I16 | I32 | I64 | Isize | F32 | F64
                | CxxString,
            ) => return,
            Some(Char | RustString) => {}
        }
    } else if let Type::CxxVector(_) = &ptr.inner {
        cx.error(ptr, "std::weak_ptr<std::vector> is not supported yet");
        return;
    }

    cx.error(ptr, "unsupported weak_ptr target type");
}

fn check_type_cxx_vector(cx: &mut Check, ptr: &Ty1) {
    if let Type::Ident(ident) = &ptr.inner {
        if cx.types.rust.contains(&ident.rust) {
            cx.error(
                ptr,
                "C++ vector containing a Rust type is not supported yet",
            );
            return;
        }

        match Atom::from(&ident.rust) {
            None
            | Some(
                U8 | U16 | U32 | U64 | Usize | I8 | I16 | I32 | I64 | Isize | F32 | F64 | CxxString,
            ) => return,
            Some(Char) => { /* todo */ }
            Some(Bool | RustString) => {}
        }
    }

    cx.error(ptr, "unsupported vector element type");
}

fn check_type_ref(cx: &mut Check, ty: &Ref) {
    if ty.mutable && !ty.pinned {
        if let Some(requires_pin) = match &ty.inner {
            Type::Ident(ident)
                if ident.rust == CxxString
                    || (cx.types.cxx.contains(&ident.rust)
                        && !cx.types.structs.contains_key(&ident.rust)
                        && !cx.types.enums.contains_key(&ident.rust)
                        && !cx.types.aliases.contains_key(&ident.rust)) =>
            {
                Some(ident.rust.to_string())
            }
            Type::CxxVector(_) => Some("CxxVector<...>".to_owned()),
            _ => None,
        } {
            cx.error(
                ty,
                format!(
                    "mutable reference to C++ type requires a pin -- use Pin<&mut {}>",
                    requires_pin,
                ),
            );
        }
    }

    match ty.inner {
        Type::Fn(_) | Type::Void(_) => {}
        Type::Ref(_) => {
            cx.error(ty, "C++ does not allow references to references");
            return;
        }
        _ => return,
    }

    cx.error(ty, "unsupported reference type");
}

fn check_type_ptr(cx: &mut Check, ty: &Ptr) {
    match ty.inner {
        Type::Fn(_) | Type::Void(_) => {}
        Type::Ref(_) => {
            cx.error(ty, "C++ does not allow pointer to reference as a type");
            return;
        }
        _ => return,
    }

    cx.error(ty, "unsupported pointer type");
}

fn check_type_slice_ref(cx: &mut Check, ty: &SliceRef) {
    let supported = !is_unsized(cx.types, &ty.inner)
        || match &ty.inner {
            Type::Ident(ident) => {
                cx.types.rust.contains(&ident.rust) || cx.types.aliases.contains_key(&ident.rust)
            }
            _ => false,
        };

    if !supported {
        let mutable = if ty.mutable { "mut " } else { "" };
        let mut msg = format!("unsupported &{}[T] element type", mutable);
        if let Type::Ident(ident) = &ty.inner {
            if cx.types.cxx.contains(&ident.rust)
                && !cx.types.structs.contains_key(&ident.rust)
                && !cx.types.enums.contains_key(&ident.rust)
            {
                msg += ": opaque C++ type is not supported yet";
            }
        }
        cx.error(ty, msg);
    }
}

fn check_type_array(cx: &mut Check, ty: &Array) {
    let supported = !is_unsized(cx.types, &ty.inner);

    if !supported {
        cx.error(ty, "unsupported array element type");
    }
}

fn check_type_fn(cx: &mut Check, ty: &Signature) {
    if ty.throws {
        cx.error(ty, "function pointer returning Result is not supported yet");
    }

    for arg in &ty.args {
        if let Type::Ptr(_) = arg.ty {
            if ty.unsafety.is_none() {
                cx.error(
                    arg,
                    "pointer argument requires that the function pointer be marked unsafe",
                );
            }
        }
    }
}

fn check_api_struct(cx: &mut Check, strct: &Struct) {
    let name = &strct.name;
    check_reserved_name(cx, &name.rust);
    check_lifetimes(cx, &strct.generics);

    if strct.fields.is_empty() {
        let span = span_for_struct_error(strct);
        cx.error(span, "structs without any fields are not supported");
    }

    if cx.types.cxx.contains(&name.rust) {
        if let Some(ety) = cx.types.untrusted.get(&name.rust) {
            let msg = "extern shared struct must be declared in an `unsafe extern` block";
            cx.error(ety, msg);
        }
    }

    for derive in &strct.derives {
        match derive.what {
            Trait::Clone
            | Trait::Copy
            | Trait::Debug
            | Trait::Default
            | Trait::Eq
            | Trait::Hash
            | Trait::Ord
            | Trait::PartialEq
            | Trait::PartialOrd
            | Trait::Serialize
            | Trait::Deserialize => {}
            Trait::BitAnd | Trait::BitOr | Trait::BitXor => {
                let msg = format!(
                    "derive({}) is currently only supported on enums, not structs",
                    derive,
                );
                cx.error(derive, msg);
            }
            Trait::ExternType => {
                let msg = format!("derive({}) on shared struct is not supported", derive);
                cx.error(derive, msg);
            }
        }
    }

    for field in &strct.fields {
        if let Type::Fn(_) = field.ty {
            cx.error(
                field,
                "function pointers in a struct field are not implemented yet",
            );
        } else if is_unsized(cx.types, &field.ty) {
            let desc = describe(cx.types, &field.ty);
            let msg = format!("using {} by value is not supported", desc);
            cx.error(field, msg);
        }
    }
}

fn check_api_enum(cx: &mut Check, enm: &Enum) {
    check_reserved_name(cx, &enm.name.rust);
    check_lifetimes(cx, &enm.generics);

    if enm.variants.is_empty() && !enm.explicit_repr {
        let span = span_for_enum_error(enm);
        cx.error(
            span,
            "explicit #[repr(...)] is required for enum without any variants",
        );
    }

    for derive in &enm.derives {
        match derive.what {
            Trait::BitAnd
            | Trait::BitOr
            | Trait::BitXor
            | Trait::Clone
            | Trait::Copy
            | Trait::Debug
            | Trait::Eq
            | Trait::Hash
            | Trait::Ord
            | Trait::PartialEq
            | Trait::PartialOrd
            | Trait::Serialize
            | Trait::Deserialize => {}
            Trait::Default => {
                let default_variants = enm.variants.iter().filter(|v| v.default).count();
                if default_variants != 1 {
                    let mut msg = Message::new();
                    write!(msg, "derive(Default) on enum requires exactly one variant to be marked with #[default]");
                    if default_variants > 0 {
                        write!(msg, " (found {})", default_variants);
                    }
                    cx.error(derive, msg);
                }
            }
            Trait::ExternType => {
                let msg = "derive(ExternType) on shared enum is not supported";
                cx.error(derive, msg);
            }
        }
    }
}

fn check_api_type(cx: &mut Check, ety: &ExternType) {
    check_reserved_name(cx, &ety.name.rust);
    check_lifetimes(cx, &ety.generics);

    for derive in &ety.derives {
        if derive.what == Trait::ExternType && ety.lang == Lang::Rust {
            continue;
        }
        let lang = match ety.lang {
            Lang::Rust => "Rust",
            Lang::Cxx | Lang::CxxUnwind => "C++",
        };
        let msg = format!(
            "derive({}) on opaque {} type is not supported yet",
            derive, lang,
        );
        cx.error(derive, msg);
    }

    if !ety.bounds.is_empty() {
        let bounds = &ety.bounds;
        let span = quote!(#(#bounds)*);
        cx.error(span, "extern type bounds are not implemented yet");
    }

    if let Some(reasons) = cx.types.required_trivial.get(&ety.name.rust) {
        let msg = format!(
            "needs a cxx::ExternType impl in order to be used as {}",
            trivial::as_what(&ety.name, reasons),
        );
        cx.error(ety, msg);
    }
}

fn check_api_fn(cx: &mut Check, efn: &ExternFn) {
    match efn.lang {
        Lang::Cxx | Lang::CxxUnwind => {
            if !efn.generics.params.is_empty() && !efn.trusted {
                let ref span = span_for_generics_error(efn);
                cx.error(span, "extern C++ function with lifetimes must be declared in `unsafe extern \"C++\"` block");
            }
        }
        Lang::Rust => {
            if !efn.generics.params.is_empty() && efn.unsafety.is_none() {
                let ref span = span_for_generics_error(efn);
                let message = format!(
                    "must be `unsafe fn {}` in order to expose explicit lifetimes to C++",
                    efn.name.rust,
                );
                cx.error(span, message);
            }
        }
    }

    check_generics(cx, &efn.generics);

    match &efn.kind {
        FnKind::Method(receiver) => {
            let ref span = span_for_receiver_error(receiver);

            if receiver.ty.rust == "Self" {
                let mutability = match receiver.mutable {
                    true => "mut ",
                    false => "",
                };
                let msg = format!(
                    "unnamed receiver type is only allowed if the surrounding extern block contains exactly one extern type; use `self: &{mutability}TheType`",
                    mutability = mutability,
                );
                cx.error(span, msg);
            } else if cx.types.enums.contains_key(&receiver.ty.rust) {
                cx.error(
                    span,
                    "unsupported receiver type; C++ does not allow member functions on enums",
                );
            } else if !cx.types.structs.contains_key(&receiver.ty.rust)
                && !cx.types.cxx.contains(&receiver.ty.rust)
                && !cx.types.rust.contains(&receiver.ty.rust)
            {
                cx.error(span, "unrecognized receiver type");
            } else if receiver.mutable
                && !receiver.pinned
                && cx.types.cxx.contains(&receiver.ty.rust)
                && !cx.types.structs.contains_key(&receiver.ty.rust)
                && !cx.types.aliases.contains_key(&receiver.ty.rust)
            {
                cx.error(
                    span,
                    format!(
                        "mutable reference to opaque C++ type requires a pin -- use `self: Pin<&mut {}>`",
                        receiver.ty.rust,
                    ),
                );
            }
        }
        FnKind::Assoc(self_type) => {
            if cx.types.enums.contains_key(self_type) {
                cx.error(
                    self_type,
                    "unsupported self type; C++ does not allow member functions on enums",
                );
            } else if !cx.types.structs.contains_key(self_type)
                && !cx.types.cxx.contains(self_type)
                && !cx.types.rust.contains(self_type)
            {
                cx.error(self_type, "unrecognized self type");
            }
        }
        FnKind::Free => {}
    }

    for arg in &efn.args {
        if let Type::Fn(_) = arg.ty {
            if efn.lang == Lang::Rust {
                cx.error(
                    arg,
                    "passing a function pointer from C++ to Rust is not implemented yet",
                );
            }
        } else if let Type::Ptr(_) = arg.ty {
            if efn.unsafety.is_none() {
                cx.error(
                    arg,
                    "pointer argument requires that the function be marked unsafe",
                );
            }
        } else if is_unsized(cx.types, &arg.ty) {
            let desc = describe(cx.types, &arg.ty);
            let msg = format!("passing {} by value is not supported", desc);
            cx.error(arg, msg);
        }
    }

    if let Some(ty) = &efn.ret {
        if let Type::Fn(_) = ty {
            cx.error(ty, "returning a function pointer is not implemented yet");
        } else if is_unsized(cx.types, ty) {
            let desc = describe(cx.types, ty);
            let msg = format!("returning {} by value is not supported", desc);
            cx.error(ty, msg);
        }
    }

    if efn.lang == Lang::Cxx {
        check_mut_return_restriction(cx, efn);
    }
}

fn check_api_type_alias(cx: &mut Check, alias: &TypeAlias) {
    check_lifetimes(cx, &alias.generics);

    for derive in &alias.derives {
        let msg = format!("derive({}) on extern type alias is not supported", derive);
        cx.error(derive, msg);
    }
}

fn check_api_impl(cx: &mut Check, imp: &Impl) {
    let ty = &imp.ty;

    check_lifetimes(cx, &imp.impl_generics);

    if let Some(negative) = imp.negative_token {
        let span = quote!(#negative #ty);
        cx.error(span, "negative impl is not supported yet");
        return;
    }

    match ty {
        Type::RustBox(ty)
        | Type::RustVec(ty)
        | Type::UniquePtr(ty)
        | Type::SharedPtr(ty)
        | Type::WeakPtr(ty)
        | Type::CxxVector(ty) => {
            if let Type::Ident(inner) = &ty.inner {
                if Atom::from(&inner.rust).is_none() {
                    return;
                }
            }
        }
        _ => {}
    }

    cx.error(imp, "unsupported Self type of explicit impl");
}

fn check_mut_return_restriction(cx: &mut Check, efn: &ExternFn) {
    if efn.unsafety.is_some() {
        // Unrestricted as long as the function is made unsafe-to-call.
        return;
    }

    match &efn.ret {
        Some(Type::Ref(ty)) if ty.mutable => {}
        Some(Type::SliceRef(slice)) if slice.mutable => {}
        _ => return,
    }

    if let Some(receiver) = efn.receiver() {
        if receiver.mutable {
            return;
        }
        let Some(resolve) = cx.types.try_resolve(&receiver.ty) else {
            return;
        };
        if !resolve.generics.lifetimes.is_empty() {
            return;
        }
    }

    struct FindLifetimeMut<'a> {
        cx: &'a Check<'a>,
        found: bool,
    }

    impl<'t, 'a> Visit<'t> for FindLifetimeMut<'a> {
        fn visit_type(&mut self, ty: &'t Type) {
            self.found |= match ty {
                Type::Ref(ty) => ty.mutable,
                Type::SliceRef(slice) => slice.mutable,
                Type::Ident(ident) if Atom::from(&ident.rust).is_none() => {
                    match self.cx.types.try_resolve(ident) {
                        Some(resolve) => !resolve.generics.lifetimes.is_empty(),
                        None => true,
                    }
                }
                _ => false,
            };
            visit::visit_type(self, ty);
        }
    }

    let mut visitor = FindLifetimeMut { cx, found: false };

    for arg in &efn.args {
        visitor.visit_type(&arg.ty);
    }

    if visitor.found {
        return;
    }

    cx.error(
        efn,
        "&mut return type is not allowed unless there is a &mut argument",
    );
}

fn check_reserved_name(cx: &mut Check, ident: &Ident) {
    if ident == "Box"
        || ident == "UniquePtr"
        || ident == "SharedPtr"
        || ident == "WeakPtr"
        || ident == "Vec"
        || ident == "CxxVector"
        || ident == "str"
        || Atom::from(ident).is_some()
    {
        cx.error(ident, "reserved name");
    }
}

fn check_reserved_lifetime(cx: &mut Check, lifetime: &Lifetime) {
    if lifetime.ident == "static" {
        match cx.generator {
            Generator::Macro => { /* rustc already reports this */ }
            Generator::Build => {
                cx.error(lifetime, error::RESERVED_LIFETIME);
            }
        }
    }
}

fn check_lifetimes(cx: &mut Check, generics: &Lifetimes) {
    for lifetime in &generics.lifetimes {
        check_reserved_lifetime(cx, lifetime);
    }
}

fn check_generics(cx: &mut Check, generics: &Generics) {
    for generic_param in &generics.params {
        if let GenericParam::Lifetime(def) = generic_param {
            check_reserved_lifetime(cx, &def.lifetime);
        }
    }
}

fn is_unsized(types: &Types, ty: &Type) -> bool {
    match ty {
        Type::Ident(ident) => {
            let ident = &ident.rust;
            ident == CxxString
                || (types.cxx.contains(ident)
                    && !types.structs.contains_key(ident)
                    && !types.enums.contains_key(ident)
                    && !(types.aliases.contains_key(ident)
                        && types.required_trivial.contains_key(ident)))
                || types.rust.contains(ident)
        }
        Type::Array(array) => is_unsized(types, &array.inner),
        Type::CxxVector(_) | Type::Fn(_) | Type::Void(_) => true,
        Type::RustBox(_)
        | Type::RustVec(_)
        | Type::UniquePtr(_)
        | Type::SharedPtr(_)
        | Type::WeakPtr(_)
        | Type::Ref(_)
        | Type::Ptr(_)
        | Type::Str(_)
        | Type::SliceRef(_) => false,
    }
}

fn span_for_struct_error(strct: &Struct) -> TokenStream {
    let struct_token = strct.struct_token;
    let mut brace_token = Group::new(Delimiter::Brace, TokenStream::new());
    brace_token.set_span(strct.brace_token.span.join());
    quote!(#struct_token #brace_token)
}

fn span_for_enum_error(enm: &Enum) -> TokenStream {
    let enum_token = enm.enum_token;
    let mut brace_token = Group::new(Delimiter::Brace, TokenStream::new());
    brace_token.set_span(enm.brace_token.span.join());
    quote!(#enum_token #brace_token)
}

fn span_for_receiver_error(receiver: &Receiver) -> TokenStream {
    let ampersand = receiver.ampersand;
    let lifetime = &receiver.lifetime;
    let mutability = receiver.mutability;
    if receiver.shorthand {
        let var = receiver.var;
        quote!(#ampersand #lifetime #mutability #var)
    } else {
        let ty = &receiver.ty;
        quote!(#ampersand #lifetime #mutability #ty)
    }
}

fn span_for_generics_error(efn: &ExternFn) -> TokenStream {
    let unsafety = efn.unsafety;
    let fn_token = efn.fn_token;
    let generics = &efn.generics;
    quote!(#unsafety #fn_token #generics)
}

fn describe(types: &Types, ty: &Type) -> String {
    match ty {
        Type::Ident(ident) => {
            if types.structs.contains_key(&ident.rust) {
                "struct".to_owned()
            } else if types.enums.contains_key(&ident.rust) {
                "enum".to_owned()
            } else if types.aliases.contains_key(&ident.rust) {
                "C++ type".to_owned()
            } else if types.cxx.contains(&ident.rust) {
                "opaque C++ type".to_owned()
            } else if types.rust.contains(&ident.rust) {
                "opaque Rust type".to_owned()
            } else if Atom::from(&ident.rust) == Some(CxxString) {
                "C++ string".to_owned()
            } else if Atom::from(&ident.rust) == Some(Char) {
                "C char".to_owned()
            } else {
                ident.rust.to_string()
            }
        }
        Type::RustBox(_) => "Box".to_owned(),
        Type::RustVec(_) => "Vec".to_owned(),
        Type::UniquePtr(_) => "unique_ptr".to_owned(),
        Type::SharedPtr(_) => "shared_ptr".to_owned(),
        Type::WeakPtr(_) => "weak_ptr".to_owned(),
        Type::Ref(_) => "reference".to_owned(),
        Type::Ptr(_) => "raw pointer".to_owned(),
        Type::Str(_) => "&str".to_owned(),
        Type::CxxVector(_) => "C++ vector".to_owned(),
        Type::SliceRef(_) => "slice".to_owned(),
        Type::Fn(_) => "function pointer".to_owned(),
        Type::Void(_) => "()".to_owned(),
        Type::Array(_) => "array".to_owned(),
    }
}
