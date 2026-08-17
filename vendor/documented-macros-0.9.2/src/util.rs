use syn::{
    parse_quote, spanned::Spanned, Attribute, Error, Expr, ExprLit, Item, Lit, Meta, Path,
    Visibility,
};

pub fn crate_module_path() -> Path {
    parse_quote!(::documented)
}

pub fn get_vis_name_attrs(item: &Item) -> syn::Result<(Visibility, String, &[Attribute])> {
    match item {
        Item::Const(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Enum(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::ExternCrate(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Fn(item) => Ok((item.vis.clone(), item.sig.ident.to_string(), &item.attrs)),
        Item::Mod(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Static(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Struct(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Trait(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::TraitAlias(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Type(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Union(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Macro(item) => {
            let Some(ref ident) = item.ident else {
                Err(Error::new(
                    item.span(),
                    "Doc comments are not supported on macro invocations",
                ))?
            };
            Ok((Visibility::Inherited, ident.to_string(), &item.attrs))
        }
        Item::ForeignMod(_) | Item::Impl(_) | Item::Use(_) => Err(Error::new(
            item.span(),
            "Doc comments are not supported on this item",
        )),
        Item::Verbatim(_) => Err(Error::new(
            item.span(),
            "Doc comments are not supported on items unknown to syn",
        )),
        _ => Err(Error::new(
            item.span(),
            "This item is unknown to documented\n\
            If this item supports doc comments, consider submitting an issue or PR",
        )),
    }
}

pub fn get_docs(attrs: &[Attribute], trim: bool) -> syn::Result<Option<String>> {
    let string_literals = attrs
        .iter()
        .filter_map(|attr| match attr.meta {
            Meta::NameValue(ref name_value) if name_value.path.is_ident("doc") => {
                Some(&name_value.value)
            }
            _ => None,
        })
        .map(|expr| match expr {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
            other => Err(Error::new(
                other.span(),
                "Doc comment is not a string literal",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;

    if string_literals.is_empty() {
        return Ok(None);
    }

    let docs = if trim {
        string_literals
            .iter()
            .flat_map(|lit| lit.split('\n').collect::<Vec<_>>())
            .map(|line| line.trim().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        string_literals.join("\n")
    };

    Ok(Some(docs))
}
