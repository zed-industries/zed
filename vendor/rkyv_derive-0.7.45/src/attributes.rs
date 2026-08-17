use crate::repr::Repr;
use quote::ToTokens;
use syn::{AttrStyle, DeriveInput, Error, Ident, Lit, LitStr, Meta, NestedMeta, Path};

#[derive(Default)]
pub struct Attributes {
    pub archive_as: Option<LitStr>,
    pub archived: Option<Ident>,
    pub resolver: Option<Ident>,
    pub attrs: Vec<Meta>,
    pub archived_repr: Repr,
    pub compares: Option<(Path, Vec<Path>)>,
    pub archive_bound: Option<LitStr>,
    pub serialize_bound: Option<LitStr>,
    pub deserialize_bound: Option<LitStr>,
    pub check_bytes: Option<Path>,
    pub copy_safe: Option<Path>,
    pub rkyv_path: Option<Path>,
    pub rkyv_path_str: Option<LitStr>,
}

fn try_set_attribute<T: ToTokens>(
    attribute: &mut Option<T>,
    value: T,
    name: &'static str,
) -> Result<(), Error> {
    if attribute.is_none() {
        *attribute = Some(value);
        Ok(())
    } else {
        Err(Error::new_spanned(
            value,
            format!("{} already specified", name),
        ))
    }
}

fn parse_archive_attributes(attributes: &mut Attributes, meta: &Meta) -> Result<(), Error> {
    match meta {
        Meta::Path(path) => {
            if path.is_ident("check_bytes") {
                try_set_attribute(&mut attributes.check_bytes, path.clone(), "check_bytes")
            } else if path.is_ident("copy_safe") {
                try_set_attribute(&mut attributes.copy_safe, path.clone(), "copy_safe")
            } else {
                Err(Error::new_spanned(meta, "unrecognized archive argument"))
            }
        }
        Meta::List(list) => {
            if list.path.is_ident("compare") {
                if attributes.compares.is_none() {
                    let mut compares = Vec::new();
                    for compare in list.nested.iter() {
                        if let NestedMeta::Meta(Meta::Path(path)) = compare {
                            compares.push(path.clone());
                        } else {
                            return Err(Error::new_spanned(
                                compare,
                                "compare arguments must be compare traits to derive",
                            ));
                        }
                    }
                    attributes.compares = Some((list.path.clone(), compares));
                    Ok(())
                } else {
                    Err(Error::new_spanned(list, "compares already specified"))
                }
            } else if list.path.is_ident("bound") {
                for bound in list.nested.iter() {
                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = bound {
                        if let Lit::Str(ref lit_str) = name_value.lit {
                            if name_value.path.is_ident("archive") {
                                try_set_attribute(
                                    &mut attributes.archive_bound,
                                    lit_str.clone(),
                                    "archive bound",
                                )?;
                            } else if name_value.path.is_ident("serialize") {
                                try_set_attribute(
                                    &mut attributes.serialize_bound,
                                    lit_str.clone(),
                                    "serialize bound",
                                )?;
                            } else if name_value.path.is_ident("deserialize") {
                                try_set_attribute(
                                    &mut attributes.deserialize_bound,
                                    lit_str.clone(),
                                    "deserialize bound",
                                )?;
                            } else {
                                return Err(Error::new_spanned(
                                    bound,
                                    "bound must be either serialize or deserialize",
                                ));
                            }
                        } else {
                            return Err(Error::new_spanned(
                                bound,
                                "bound arguments must be a string",
                            ));
                        }
                    } else {
                        return Err(Error::new_spanned(
                            bound,
                            "bound arguments must be serialize or deserialize bounds to apply",
                        ));
                    }
                }
                Ok(())
            } else if list.path.is_ident("repr") {
                // TODO: remove `archive(repr(...))` syntax
                attributes.archived_repr.parse_args(list.nested.iter())
            } else {
                Err(Error::new_spanned(
                    &list.path,
                    "unrecognized archive argument",
                ))
            }
        }
        Meta::NameValue(meta) => {
            if meta.path.is_ident("archived") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    try_set_attribute(
                        &mut attributes.archived,
                        Ident::new(&lit_str.value(), lit_str.span()),
                        "archived",
                    )
                } else {
                    Err(Error::new_spanned(meta, "archived must be a string"))
                }
            } else if meta.path.is_ident("resolver") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    try_set_attribute(
                        &mut attributes.resolver,
                        Ident::new(&lit_str.value(), lit_str.span()),
                        "resolver",
                    )
                } else {
                    Err(Error::new_spanned(meta, "resolver must be a string"))
                }
            } else if meta.path.is_ident("as") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    try_set_attribute(&mut attributes.archive_as, lit_str.clone(), "archive as")
                } else {
                    Err(Error::new_spanned(meta, "archive as must be a string"))
                }
            } else if meta.path.is_ident("crate") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    let stream = syn::parse_str(&lit_str.value())?;
                    let tokens = crate::serde::respan::respan(stream, lit_str.span());
                    let path = syn::parse2(tokens)?;
                    try_set_attribute(&mut attributes.rkyv_path, path, "crate")?;
                    attributes.rkyv_path_str = Some(lit_str.clone());
                    Ok(())
                } else {
                    Err(Error::new_spanned(meta, "crate must be a string"))
                }
            } else {
                Err(Error::new_spanned(meta, "unrecognized archive argument"))
            }
        }
    }
}

pub fn parse_attributes(input: &DeriveInput) -> Result<Attributes, Error> {
    let mut result = Attributes::default();
    for attr in input.attrs.iter() {
        if let AttrStyle::Outer = attr.style {
            if attr.path.is_ident("archive") || attr.path.is_ident("archive_attr") {
                if let Meta::List(list) = attr.parse_meta()? {
                    if list.path.is_ident("archive") {
                        for nested in list.nested.iter() {
                            if let NestedMeta::Meta(meta) = nested {
                                parse_archive_attributes(&mut result, meta)?;
                            } else {
                                return Err(Error::new_spanned(
                                    nested,
                                    "archive arguments must be metas",
                                ));
                            }
                        }
                    } else if list.path.is_ident("archive_attr") {
                        for nested in list.nested.iter() {
                            if let NestedMeta::Meta(meta) = nested {
                                if let Meta::List(list) = meta {
                                    if list.path.is_ident("repr") {
                                        result.archived_repr.parse_args(list.nested.iter())?;
                                    } else {
                                        result.attrs.push(meta.clone());
                                    }
                                } else {
                                    result.attrs.push(meta.clone());
                                }
                            } else {
                                return Err(Error::new_spanned(
                                    nested,
                                    "archive_attr arguments must be metas",
                                ));
                            }
                        }
                    }
                } else {
                    return Err(Error::new_spanned(
                        attr,
                        "archive and archive_attr may only be structured list attributes",
                    ));
                }
            }
        }
    }

    Ok(result)
}
