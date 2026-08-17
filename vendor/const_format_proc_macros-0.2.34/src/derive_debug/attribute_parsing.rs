use crate::{
    datastructure::{DataStructure, Field, FieldMap},
    utils::LinearResult,
};

use super::{syntax::ImplHeader, type_detection};

use quote::ToTokens;

use syn::{Attribute, Meta, MetaList, NestedMeta};

use std::marker::PhantomData;

pub(crate) struct ConstDebugConfig<'a> {
    pub(crate) debug_print: bool,
    pub(crate) crate_path: Option<syn::Path>,
    pub(crate) impls: Vec<ImplHeader>,
    pub(crate) field_map: FieldMap<FieldConfig<'a>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ConstDebugConfig<'a> {
    fn new(roa: ConstDebugAttrs<'a>) -> Result<Self, crate::Error> {
        let ConstDebugAttrs {
            debug_print,
            crate_path,
            impls,
            field_map,
            errors: _,
            _marker: PhantomData,
        } = roa;

        Ok(Self {
            debug_print,
            crate_path,
            impls,
            field_map,
            _marker: PhantomData,
        })
    }
}

struct ConstDebugAttrs<'a> {
    debug_print: bool,
    crate_path: Option<syn::Path>,
    impls: Vec<ImplHeader>,
    field_map: FieldMap<FieldConfig<'a>>,
    errors: LinearResult,
    _marker: PhantomData<&'a ()>,
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct FieldConfig<'a> {
    pub(crate) how_to_fmt: HowToFmt<'a>,
}

pub(crate) enum HowToFmt<'a> {
    /// coerce_to_fmt!(&field).const_debug_fmt(f)`
    Regular,
    /// Doesn't print the field.
    Ignore,
    /// A slice or an array
    Slice,
    /// A single field tuple struct, èg: `struct Foo(u32);;`
    Option_,
    /// A single field tuple struct, èg: `struct Foo(u32);;`
    /// The path of the field is parsed from the type, erroring if it's not a path.
    Newtype(&'a syn::Ident),
    /// The function used to format the field.
    With(syn::Path),
    //// The macro used to format the field,
    //// it's expected to be callable as `themacro!(var, formatter);`.
    WithMacro(syn::Path),
    /// The newtype used to format the field, taking the field by reference.
    /// eg: `struct Foo<'a>(&'a u32);`.
    WithWrapper(syn::Path),
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
enum ParseContext<'a> {
    TypeAttr,
    Field { field: &'a Field<'a> },
}

pub(crate) fn parse_attrs_for_derive<'a>(
    ds: &'a DataStructure<'a>,
) -> Result<ConstDebugConfig<'a>, crate::Error> {
    let mut this = ConstDebugAttrs {
        debug_print: false,
        crate_path: None,
        impls: Vec::new(),
        field_map: FieldMap::with(ds, |f| FieldConfig {
            how_to_fmt: type_detection::detect_type_formatting(f.ty),
        }),
        errors: LinearResult::ok(),
        _marker: PhantomData,
    };

    let ty_ctx = ParseContext::TypeAttr;
    parse_inner(&mut this, ds.attrs, ty_ctx)?;

    for variant in &ds.variants {
        for field in variant.fields.iter() {
            parse_inner(&mut this, field.attrs, ParseContext::Field { field })?;
        }
    }

    this.errors.take()?;

    ConstDebugConfig::new(this)
}

/// Parses an individual attribute
fn parse_inner<'a, I>(
    this: &mut ConstDebugAttrs<'a>,
    attrs: I,
    pctx: ParseContext<'a>,
) -> Result<(), crate::Error>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    for attr in attrs {
        match attr.parse_meta() {
            Ok(Meta::List(list)) => {
                let x = parse_attr_list(this, pctx, list);
                this.errors.combine_err(x);
            }
            Err(e) => {
                this.errors.push_err(e);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Parses an individual attribute list (A `#[attribute( .. )] attribute`).
fn parse_attr_list<'a>(
    this: &mut ConstDebugAttrs<'a>,
    pctx: ParseContext<'a>,
    list: MetaList,
) -> Result<(), crate::Error> {
    if list.path.is_ident("cdeb") {
        with_nested_meta("cdeb", list.nested, |attr| {
            let x = parse_sabi_attr(this, pctx, attr);
            this.errors.combine_err(x);
            Ok(())
        })?;
    }

    Ok(())
}

fn make_err(tokens: &dyn ToTokens) -> crate::Error {
    spanned_err!(tokens, "unrecognized attribute")
}

/// Parses the contents of a `#[sabi( .. )]` attribute.
fn parse_sabi_attr<'a>(
    this: &mut ConstDebugAttrs<'a>,
    pctx: ParseContext<'a>,
    attr: Meta,
) -> Result<(), crate::Error> {
    match (pctx, attr) {
        (ParseContext::Field { field, .. }, Meta::Path(path)) => {
            let f_config = &mut this.field_map[field.index];

            if path.is_ident("ignore") {
                f_config.how_to_fmt = HowToFmt::Ignore;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::Field { field, .. }, Meta::NameValue(nv)) => {
            let f_config = &mut this.field_map[field.index];

            if nv.path.is_ident("with") {
                f_config.how_to_fmt = HowToFmt::With(parse_lit(&nv.lit)?);
            } else if nv.path.is_ident("with_macro") {
                f_config.how_to_fmt = HowToFmt::WithMacro(parse_lit(&nv.lit)?);
            } else if nv.path.is_ident("with_wrapper") {
                f_config.how_to_fmt = HowToFmt::WithWrapper(parse_lit(&nv.lit)?);
            } else {
                return Err(make_err(&nv));
            }
        }
        (ParseContext::Field { field, .. }, Meta::List(list)) => {
            let f_config = &mut this.field_map[field.index];

            if list.path.is_ident("is_a") {
                match list.nested.len() {
                    0 => return Err(make_err(&list)),
                    1 => (),
                    _ => return_spanned_err!(
                        list,
                        "The `#[cdeb(is_a())` attribute must only specify one kind of type."
                    ),
                }
                with_nested_meta("is_a", list.nested, |attr| {
                    f_config.how_to_fmt = parse_the_is_a_attribute(attr, field)?;
                    Ok(())
                })?;
            } else {
                return Err(make_err(&list));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::Path(path)) => {
            if path.is_ident("debug_print") {
                this.debug_print = true;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::NameValue(nv)) => {
            if nv.path.is_ident("crate") {
                this.crate_path = Some(parse_lit(&nv.lit)?);
            } else {
                return Err(make_err(&nv));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::List(list)) => {
            if list.path.is_ident("impls") {
                for x in list.nested {
                    let lit = match x {
                        NestedMeta::Meta(attr) => return Err(make_err(&attr)),
                        NestedMeta::Lit(lit) => lit,
                    };
                    this.impls.push(parse_lit::<ImplHeader>(&lit)?);
                }
            } else {
                return Err(make_err(&list));
            }
        }
        #[allow(unreachable_patterns)]
        (_, x) => return Err(make_err(&x)),
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn parse_the_is_a_attribute<'a>(
    attr: syn::Meta,
    f: &Field<'a>,
) -> Result<HowToFmt<'a>, crate::Error> {
    match attr {
        Meta::Path(path) => {
            if path.is_ident("array") || path.is_ident("slice") {
                Ok(HowToFmt::Slice)
            } else if path.is_ident("Option") || path.is_ident("option") {
                Ok(HowToFmt::Option_)
            } else if path.is_ident("newtype") {
                let newtype = type_detection::parse_type_as_ident(f.ty)?;
                Ok(HowToFmt::Newtype(newtype))
            } else if path.is_ident("non_std") || path.is_ident("not_std") {
                Ok(HowToFmt::Regular)
            } else {
                Err(make_err(&path))
            }
        }
        _ => Err(make_err(&attr)),
    }
}

///////////////////////////////////////////////////////////////////////////////

fn parse_lit<T>(lit: &syn::Lit) -> Result<T, crate::Error>
where
    T: syn::parse::Parse,
{
    match lit {
        syn::Lit::Str(x) => x.parse().map_err(crate::Error::from),
        _ => Err(spanned_err!(
            lit,
            "Expected string literal containing identifier"
        )),
    }
}

#[allow(dead_code)]
fn parse_expr(lit: syn::Lit) -> Result<syn::Expr, crate::Error> {
    match lit {
        syn::Lit::Str(x) => x.parse(),
        syn::Lit::Int(x) => syn::parse_str(x.base10_digits()),
        _ => return_spanned_err!(lit, "Expected string or integer literal"),
    }
    .map_err(crate::Error::from)
}

///////////////////////////////////////////////////////////////////////////////

pub fn with_nested_meta<F>(
    attr_name: &str,
    iter: syn::punctuated::Punctuated<NestedMeta, syn::Token!(,)>,
    mut f: F,
) -> Result<(), crate::Error>
where
    F: FnMut(Meta) -> Result<(), crate::Error>,
{
    for repr in iter {
        match repr {
            NestedMeta::Meta(attr) => {
                f(attr)?;
            }
            NestedMeta::Lit(lit) => {
                return_spanned_err!(
                    lit,
                    "the #[{}(...)] attribute does not allow literals in the attribute list",
                    attr_name,
                );
            }
        }
    }
    Ok(())
}
