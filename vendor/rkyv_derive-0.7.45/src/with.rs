use syn::{parse_quote, punctuated::Punctuated, token::Comma, Error, Expr, Field, Path, Type};

#[inline]
pub fn with<B, F: FnMut(B, &Type) -> B>(field: &Field, init: B, f: F) -> Result<B, Error> {
    let fields = field
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path.is_ident("with") {
                Some(attr.parse_args_with(Punctuated::<Type, Comma>::parse_separated_nonempty))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(fields.iter().flatten().rev().fold(init, f))
}

#[inline]
pub fn make_with_ty(rkyv_path: &Path) -> impl '_ + Fn(&Field) -> Result<Type, Error> {
    move |field| {
        with(
            field,
            field.ty.clone(),
            |ty, wrapper| parse_quote! { #rkyv_path::with::With<#ty, #wrapper> },
        )
    }
}

#[inline]
pub fn make_with_cast(rkyv_path: &Path) -> impl '_ + Fn(&Field, Expr) -> Result<Expr, Error> {
    move |field, expr| {
        with(
            field,
            expr,
            |expr, wrapper| parse_quote! { #rkyv_path::with::With::<_, #wrapper>::cast(#expr) },
        )
    }
}

#[inline]
pub fn with_inner(field: &Field, expr: Expr) -> Result<Expr, Error> {
    with(field, expr, |expr, _| parse_quote! { #expr.into_inner() })
}
