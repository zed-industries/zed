use quote::ToTokens;
use syn::{GenericArgument, PathArguments, Type};

use crate::utils::uses_this_lifetime;

const STD_CONTAINER_TYPES: &[&str] = &["Box", "Arc", "Rc"];

/// Returns Some((type_name, element_type)) if the provided type appears to be Box, Arc, or Rc from
/// the standard library. Returns None if not.
pub fn apparent_std_container_type(raw_type: &Type) -> Option<(&'static str, &Type)> {
    let tpath = if let Type::Path(x) = raw_type {
        x
    } else {
        return None;
    };
    let segment = tpath.path.segments.last()?;
    let args = if let PathArguments::AngleBracketed(args) = &segment.arguments {
        args
    } else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    let arg = args.args.first().unwrap();
    let eltype = if let GenericArgument::Type(x) = arg {
        x
    } else {
        return None;
    };
    for type_name in STD_CONTAINER_TYPES {
        if segment.ident == type_name {
            return Some((type_name, eltype));
        }
    }
    None
}

/// Returns Some(true or false) if the type is known to be covariant / not covariant.
pub fn type_is_covariant_over_this_lifetime(ty: &syn::Type) -> Option<bool> {
    use syn::Type::*;
    // If the type never uses the 'this lifetime, we don't have to
    // worry about it not being covariant.
    if !uses_this_lifetime(ty.to_token_stream()) {
        return Some(true);
    }
    match ty {
        Array(arr) => type_is_covariant_over_this_lifetime(&arr.elem),
        BareFn(f) => {
            debug_assert!(uses_this_lifetime(f.to_token_stream()));
            None
        }
        Group(ty) => type_is_covariant_over_this_lifetime(&ty.elem),
        ImplTrait(..) => None, // Unusable in struct definition.
        Infer(..) => None,     // Unusable in struct definition.
        Macro(..) => None,     // We don't know what the macro will resolve to.
        Never(..) => None,
        Paren(ty) => type_is_covariant_over_this_lifetime(&ty.elem),
        Path(path) => {
            if let Some(qself) = &path.qself {
                if !type_is_covariant_over_this_lifetime(&qself.ty)? {
                    return Some(false);
                }
            }
            let mut all_parameters_are_covariant = false;
            // If the type is Box, Arc, or Rc, we can assume it to be covariant.
            if apparent_std_container_type(ty).is_some() {
                all_parameters_are_covariant = true;
            }
            for segment in path.path.segments.iter() {
                let args = &segment.arguments;
                if let syn::PathArguments::AngleBracketed(args) = &args {
                    for arg in args.args.iter() {
                        if let syn::GenericArgument::Type(ty) = arg {
                            if all_parameters_are_covariant {
                                if !type_is_covariant_over_this_lifetime(ty)? {
                                    return Some(false);
                                }
                            } else if uses_this_lifetime(ty.to_token_stream()) {
                                return None;
                            }
                        } else if let syn::GenericArgument::Lifetime(lt) = arg {
                            if lt.ident == "this" && !all_parameters_are_covariant {
                                return None;
                            }
                        }
                    }
                } else if let syn::PathArguments::Parenthesized(args) = &args {
                    for arg in args.inputs.iter() {
                        if uses_this_lifetime(arg.to_token_stream()) {
                            return None;
                        }
                    }
                    if let syn::ReturnType::Type(_, ty) = &args.output {
                        if uses_this_lifetime(ty.to_token_stream()) {
                            return None;
                        }
                    }
                }
            }
            Some(true)
        }
        Ptr(ptr) => {
            if ptr.mutability.is_some() {
                Some(false)
            } else {
                type_is_covariant_over_this_lifetime(&ptr.elem)
            }
        }
        // Ignore the actual lifetime of the reference because Rust can automatically convert those.
        Reference(rf) => {
            if rf.mutability.is_some() {
                Some(!uses_this_lifetime(rf.elem.to_token_stream()))
            } else {
                type_is_covariant_over_this_lifetime(&rf.elem)
            }
        }
        Slice(sl) => type_is_covariant_over_this_lifetime(&sl.elem),
        TraitObject(..) => None,
        Tuple(tup) => {
            let mut result = Some(true);
            for ty in tup.elems.iter() {
                match type_is_covariant_over_this_lifetime(ty) {
                    Some(true) => (),
                    Some(false) => return Some(false),
                    None => result = None,
                }
            }
            result
        }
        // As of writing this, syn parses all the types we could need. However,
        // just to be safe, return that we don't know if it's covariant.
        Verbatim(..) => None,
        _ => None,
    }
}
