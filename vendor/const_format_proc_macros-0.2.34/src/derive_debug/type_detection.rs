use super::HowToFmt;

use syn::Type;

pub(super) fn detect_type_formatting(ty: &Type) -> HowToFmt {
    let ty = unwrap_reference(ty);

    // println!("{:?} {}", ty, ty.to_token_stream());

    match ty {
        Type::Array { .. } | Type::Slice { .. } => HowToFmt::Slice,
        Type::Path(ty) if ty.qself.is_none() && is_path_an_option(&ty.path) => HowToFmt::Option_,
        _ => HowToFmt::Regular,
    }
}

fn unwrap_reference(mut ty: &Type) -> &Type {
    loop {
        match ty {
            Type::Reference(next) => ty = &*next.elem,
            Type::Group(next) => ty = &*next.elem,
            Type::Paren(next) => ty = &*next.elem,
            _ => break,
        }
    }
    ty
}

fn is_path_an_option(path: &syn::Path) -> bool {
    let segments = &path.segments;

    if segments[0].ident == "Option" {
        return true;
    }

    if segments.len() < 3 {
        return false;
    }

    let thecrate = &segments[0].ident;

    (thecrate == "core" || thecrate == "alloc" || thecrate == "std")
        && segments[1].ident == "option"
        && segments[2].ident == "Option"
}

/// Parses the type as an identifier, or the last identifier of a path.
pub(super) fn parse_type_as_ident(ty: &Type) -> Result<&syn::Ident, crate::Error> {
    let ty = unwrap_reference(ty);
    match ty {
        Type::Path(typath) if typath.qself.is_none() && !typath.path.segments.is_empty() => {
            Ok(&typath.path.segments.last().unwrap().ident)
        }
        _ => Err(spanned_err!(ty, "expected a struct or enum")),
    }
}
