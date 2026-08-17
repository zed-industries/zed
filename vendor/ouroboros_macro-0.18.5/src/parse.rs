use proc_macro2::{Span, TokenTree};
use quote::format_ident;
use syn::{
    spanned::Spanned, Attribute, Error, Fields, GenericParam, ItemStruct, MacroDelimiter, Meta,
};

use crate::{
    covariance_detection::type_is_covariant_over_this_lifetime,
    info_structures::{BorrowRequest, Derive, FieldType, StructFieldInfo, StructInfo},
    utils::submodule_contents_visibility,
};

fn handle_borrows_attr(
    field_info: &mut [StructFieldInfo],
    attr: &Attribute,
    borrows: &mut Vec<BorrowRequest>,
) -> Result<(), Error> {
    let mut borrow_mut = false;
    let mut waiting_for_comma = false;
    let tokens = match &attr.meta {
        Meta::List(ml) => ml.tokens.clone(),
        _ => {
            return Err(Error::new_spanned(
                &attr.meta,
                "Invalid syntax for borrows() macro.",
            ))
        }
    };
    for token in tokens {
        if let TokenTree::Ident(ident) = token {
            if waiting_for_comma {
                return Err(Error::new_spanned(&ident, "Expected comma."));
            }
            let istr = ident.to_string();
            if istr == "mut" {
                if borrow_mut {
                    return Err(Error::new_spanned(&ident, "Unexpected double 'mut'"));
                }
                borrow_mut = true;
            } else {
                let index = field_info.iter().position(|item| item.name == istr);
                let index = if let Some(v) = index {
                    v
                } else {
                    return Err(Error::new_spanned(
                        &ident,
                        concat!(
                            "Unknown identifier, make sure that it is spelled ",
                            "correctly and defined above the location it is borrowed."
                        ),
                    ));
                };
                if borrow_mut {
                    if field_info[index].field_type == FieldType::Borrowed {
                        return Err(Error::new_spanned(
                            &ident,
                            "Cannot borrow mutably, this field was previously borrowed immutably.",
                        ));
                    }
                    if field_info[index].field_type == FieldType::BorrowedMut {
                        return Err(Error::new_spanned(&ident, "Cannot borrow mutably twice."));
                    }
                    field_info[index].field_type = FieldType::BorrowedMut;
                } else {
                    if field_info[index].field_type == FieldType::BorrowedMut {
                        return Err(Error::new_spanned(
                            &ident,
                            "Cannot borrow as immutable as it was previously borrowed mutably.",
                        ));
                    }
                    field_info[index].field_type = FieldType::Borrowed;
                }
                borrows.push(BorrowRequest {
                    index,
                    mutable: borrow_mut,
                });
                waiting_for_comma = true;
                borrow_mut = false;
            }
        } else if let TokenTree::Punct(punct) = token {
            if punct.as_char() == ',' {
                if waiting_for_comma {
                    waiting_for_comma = false;
                } else {
                    return Err(Error::new_spanned(&punct, "Unexpected extra comma."));
                }
            } else {
                return Err(Error::new_spanned(
                    &punct,
                    "Unexpected punctuation, expected comma or identifier.",
                ));
            }
        } else {
            return Err(Error::new_spanned(
                &token,
                "Unexpected token, expected comma or identifier.",
            ));
        }
    }
    Ok(())
}

fn parse_derive_token(token: &TokenTree) -> Result<Option<Derive>, Error> {
    match token {
        TokenTree::Ident(ident) => match &ident.to_string()[..] {
            "Debug" => Ok(Some(Derive::Debug)),
            "PartialEq" => Ok(Some(Derive::PartialEq)),
            "Eq" => Ok(Some(Derive::Eq)),
            _ => Err(Error::new(
                ident.span(),
                format!("{} cannot be derived for self-referencing structs", ident),
            )),
        },
        TokenTree::Punct(..) => Ok(None),
        _ => Err(Error::new(token.span(), "bad syntax")),
    }
}

fn parse_derive_attribute(attr: &Attribute) -> Result<Vec<Derive>, Error> {
    let body = match &attr.meta {
        Meta::List(ml) => ml,
        _ => unreachable!(),
    };
    if !matches!(body.delimiter, MacroDelimiter::Paren(_)) {
        return Err(Error::new(
            attr.span(),
            format!(
                "malformed derive input, derive attributes are of the form `#[derive({})]`",
                body.tokens
            ),
        ));
    }
    let mut derives = Vec::new();
    for token in body.tokens.clone().into_iter() {
        if let Some(derive) = parse_derive_token(&token)? {
            derives.push(derive);
        }
    }
    Ok(derives)
}

pub fn parse_struct(def: &ItemStruct) -> Result<StructInfo, Error> {
    let vis = def.vis.clone();
    let generics = def.generics.clone();
    let mut actual_struct_def = def.clone();
    actual_struct_def.vis = vis.clone();
    let mut fields = Vec::new();
    match &mut actual_struct_def.fields {
        Fields::Named(def_fields) => {
            for field in &mut def_fields.named {
                let mut borrows = Vec::new();
                let mut self_referencing = false;
                let mut covariant = type_is_covariant_over_this_lifetime(&field.ty);
                let mut remove_attrs = Vec::new();
                for (index, attr) in field.attrs.iter().enumerate() {
                    let path = &attr.path();
                    if path.leading_colon.is_some() {
                        continue;
                    }
                    if path.segments.len() != 1 {
                        continue;
                    }
                    if path.segments.first().unwrap().ident == "borrows" {
                        if self_referencing {
                            panic!("TODO: Nice error, used #[borrows()] twice.");
                        }
                        self_referencing = true;
                        handle_borrows_attr(&mut fields[..], attr, &mut borrows)?;
                        remove_attrs.push(index);
                    }
                    if path.segments.first().unwrap().ident == "covariant" {
                        if covariant.is_some() {
                            panic!("TODO: Nice error, covariance specified twice.");
                        }
                        covariant = Some(true);
                        remove_attrs.push(index);
                    }
                    if path.segments.first().unwrap().ident == "not_covariant" {
                        if covariant.is_some() {
                            panic!("TODO: Nice error, covariance specified twice.");
                        }
                        covariant = Some(false);
                        remove_attrs.push(index);
                    }
                }
                // We should not be able to access the field outside of the hidden module where
                // everything is generated.
                let with_vis = submodule_contents_visibility(&field.vis.clone());
                fields.push(StructFieldInfo {
                    name: field.ident.clone().expect("Named field has no name."),
                    typ: field.ty.clone(),
                    field_type: FieldType::Tail,
                    vis: with_vis,
                    borrows,
                    self_referencing,
                    covariant,
                });
            }
        }
        Fields::Unnamed(_fields) => {
            return Err(Error::new(
                Span::call_site(),
                "Tuple structs are not supported yet.",
            ))
        }
        Fields::Unit => {
            return Err(Error::new(
                Span::call_site(),
                "Unit structs cannot be self-referential.",
            ))
        }
    }
    if fields.len() < 2 {
        return Err(Error::new(
            Span::call_site(),
            "Self-referencing structs must have at least 2 fields.",
        ));
    }
    let mut has_non_tail = false;
    for field in &fields {
        if !field.field_type.is_tail() {
            has_non_tail = true;
            break;
        }
    }
    if !has_non_tail {
        return Err(Error::new(
            Span::call_site(),
            format!(
                concat!(
                    "Self-referencing struct cannot be made entirely of tail fields, try adding ",
                    "#[borrows({0})] to a field defined after {0}."
                ),
                fields[0].name
            ),
        ));
    }
    let first_lifetime = if let Some(GenericParam::Lifetime(param)) = generics.params.first() {
        param.lifetime.ident.clone()
    } else {
        format_ident!("static")
    };
    let mut attributes = Vec::new();
    let mut derives = Vec::new();
    for attr in &def.attrs {
        let p = &attr.path().segments;
        if p.is_empty() {
            return Err(Error::new(p.span(), "Unsupported attribute".to_string()));
        }
        let name = p[0].ident.to_string();
        let good = matches!(&name[..], "clippy" | "allow" | "deny" | "doc");
        if good {
            attributes.push(attr.clone())
        } else if name == "derive" {
            if !derives.is_empty() {
                return Err(Error::new(
                    attr.span(),
                    "Multiple derive attributes not allowed",
                ));
            } else {
                derives = parse_derive_attribute(attr)?;
            }
        } else {
            return Err(Error::new(p.span(), "Unsupported attribute".to_string()));
        }
    }

    Ok(StructInfo {
        derives,
        ident: def.ident.clone(),
        internal_ident: format_ident!("{}Internal", def.ident),
        generics: def.generics.clone(),
        fields,
        vis,
        first_lifetime,
        attributes,
    })
}
