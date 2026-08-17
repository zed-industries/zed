use crate::syntax::cfg::CfgExpr;
use crate::syntax::namespace::Namespace;
use crate::syntax::report::Errors;
use crate::syntax::repr::Repr;
use crate::syntax::{cfg, Derive, Doc, ForeignName};
use proc_macro2::Ident;
use syn::parse::ParseStream;
use syn::{Attribute, Error, Expr, Lit, LitStr, Meta, Path, Result, Token};

// Intended usage:
//
//     let mut doc = Doc::new();
//     let mut cxx_name = None;
//     let mut rust_name = None;
//     /* ... */
//     let attrs = attrs::parse(
//         cx,
//         item.attrs,
//         attrs::Parser {
//             doc: Some(&mut doc),
//             cxx_name: Some(&mut cxx_name),
//             rust_name: Some(&mut rust_name),
//             /* ... */
//             ..Default::default()
//         },
//     );
//
#[derive(Default)]
pub(crate) struct Parser<'a> {
    pub cfg: Option<&'a mut CfgExpr>,
    pub doc: Option<&'a mut Doc>,
    pub derives: Option<&'a mut Vec<Derive>>,
    pub repr: Option<&'a mut Option<Repr>>,
    pub default: Option<&'a mut bool>,
    pub namespace: Option<&'a mut Namespace>,
    pub cxx_name: Option<&'a mut Option<ForeignName>>,
    pub rust_name: Option<&'a mut Option<Ident>>,
    pub self_type: Option<&'a mut Option<Ident>>,
    pub ignore_unrecognized: bool,

    // Suppress clippy needless_update lint ("struct update has no effect, all
    // the fields in the struct have already been specified") when preemptively
    // writing `..Default::default()`.
    pub(crate) _more: (),
}

#[must_use]
pub(crate) fn parse(cx: &mut Errors, attrs: Vec<Attribute>, mut parser: Parser) -> OtherAttrs {
    let mut other_attrs = OtherAttrs::new();
    for attr in attrs {
        let attr_path = attr.path();
        if attr_path.is_ident("doc") {
            match parse_doc_attribute(&attr.meta) {
                Ok(attr) => {
                    if let Some(doc) = &mut parser.doc {
                        match attr {
                            DocAttribute::Doc(lit) => doc.push(lit),
                            DocAttribute::Hidden => doc.hidden = true,
                        }
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("derive") {
            match attr.parse_args_with(|attr: ParseStream| parse_derive_attribute(cx, attr)) {
                Ok(attr) => {
                    if let Some(derives) = &mut parser.derives {
                        derives.extend(attr);
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("repr") {
            match attr.parse_args::<Repr>() {
                Ok(attr) => {
                    if let Some(repr) = &mut parser.repr {
                        **repr = Some(attr);
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("default") {
            match parse_default_attribute(&attr.meta) {
                Ok(()) => {
                    if let Some(default) = &mut parser.default {
                        **default = true;
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("namespace") {
            match Namespace::parse_meta(&attr.meta) {
                Ok(attr) => {
                    if let Some(namespace) = &mut parser.namespace {
                        **namespace = attr;
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("cxx_name") {
            match parse_cxx_name_attribute(&attr.meta) {
                Ok(attr) => {
                    if let Some(cxx_name) = &mut parser.cxx_name {
                        **cxx_name = Some(attr);
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("rust_name") {
            match parse_rust_ident_attribute(&attr.meta) {
                Ok(attr) => {
                    if let Some(rust_name) = &mut parser.rust_name {
                        **rust_name = Some(attr);
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("Self") {
            match parse_rust_ident_attribute(&attr.meta) {
                Ok(attr) => {
                    if let Some(self_type) = &mut parser.self_type {
                        **self_type = Some(attr);
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("cfg") {
            match cfg::parse_attribute(&attr) {
                Ok(cfg_expr) => {
                    if let Some(cfg) = &mut parser.cfg {
                        cfg.merge_and(cfg_expr);
                        other_attrs.cfg.push(attr);
                        continue;
                    }
                }
                Err(err) => {
                    cx.push(err);
                    break;
                }
            }
        } else if attr_path.is_ident("allow")
            || attr_path.is_ident("warn")
            || attr_path.is_ident("deny")
            || attr_path.is_ident("forbid")
        {
            other_attrs.lint.push(attr);
            continue;
        } else if attr_path.is_ident("deprecated")
            || attr_path.is_ident("must_use")
            || attr_path.is_ident("serde")
        {
            other_attrs.passthrough.push(attr);
            continue;
        } else if attr_path.segments.len() > 1 {
            let tool = &attr_path.segments.first().unwrap().ident;
            if tool == "rustfmt" {
                // Skip, rustfmt only needs to find it in the pre-expansion source file.
                continue;
            } else if tool == "clippy" {
                other_attrs.lint.push(attr);
                continue;
            }
        }
        if !parser.ignore_unrecognized {
            cx.error(attr, "unsupported attribute");
            break;
        }
    }
    other_attrs
}

enum DocAttribute {
    Doc(LitStr),
    Hidden,
}

mod kw {
    syn::custom_keyword!(hidden);
}

fn parse_doc_attribute(meta: &Meta) -> Result<DocAttribute> {
    match meta {
        Meta::NameValue(meta) => {
            if let Expr::Lit(expr) = &meta.value {
                if let Lit::Str(lit) = &expr.lit {
                    return Ok(DocAttribute::Doc(lit.clone()));
                }
            }
        }
        Meta::List(meta) => {
            meta.parse_args::<kw::hidden>()?;
            return Ok(DocAttribute::Hidden);
        }
        Meta::Path(_) => {}
    }
    Err(Error::new_spanned(meta, "unsupported doc attribute"))
}

fn parse_derive_attribute(cx: &mut Errors, input: ParseStream) -> Result<Vec<Derive>> {
    let paths = input.parse_terminated(Path::parse_mod_style, Token![,])?;

    let mut derives = Vec::new();
    for path in paths {
        if let Some(ident) = path.get_ident() {
            if let Some(derive) = Derive::from(ident) {
                derives.push(derive);
                continue;
            }
        }
        cx.error(path, "unsupported derive");
    }
    Ok(derives)
}

fn parse_default_attribute(meta: &Meta) -> Result<()> {
    let error_span = match meta {
        Meta::Path(_) => return Ok(()),
        Meta::List(meta) => meta.delimiter.span().open(),
        Meta::NameValue(meta) => meta.eq_token.span,
    };
    Err(Error::new(
        error_span,
        "#[default] attribute does not accept an argument",
    ))
}

fn parse_cxx_name_attribute(meta: &Meta) -> Result<ForeignName> {
    if let Meta::NameValue(meta) = meta {
        match &meta.value {
            Expr::Lit(expr) => {
                if let Lit::Str(lit) = &expr.lit {
                    return ForeignName::parse(&lit.value(), lit.span());
                }
            }
            Expr::Path(expr) => {
                if let Some(ident) = expr.path.get_ident() {
                    return ForeignName::parse(&ident.to_string(), ident.span());
                }
            }
            _ => {}
        }
    }
    Err(Error::new_spanned(meta, "unsupported cxx_name attribute"))
}

fn parse_rust_ident_attribute(meta: &Meta) -> Result<Ident> {
    if let Meta::NameValue(meta) = meta {
        match &meta.value {
            Expr::Lit(expr) => {
                if let Lit::Str(lit) = &expr.lit {
                    return lit.parse();
                }
            }
            Expr::Path(expr) => {
                if let Some(ident) = expr.path.get_ident() {
                    return Ok(ident.clone());
                }
            }
            _ => {}
        }
    }
    Err(Error::new_spanned(
        meta,
        format!(
            "unsupported `{}` attribute",
            meta.path().get_ident().unwrap(),
        ),
    ))
}

#[derive(Clone)]
pub(crate) struct OtherAttrs {
    pub cfg: Vec<Attribute>,
    pub lint: Vec<Attribute>,
    pub passthrough: Vec<Attribute>,
}

impl OtherAttrs {
    pub(crate) fn new() -> Self {
        OtherAttrs {
            cfg: Vec::new(),
            lint: Vec::new(),
            passthrough: Vec::new(),
        }
    }

    pub(crate) fn extend(&mut self, other: Self) {
        self.cfg.extend(other.cfg);
        self.lint.extend(other.lint);
        self.passthrough.extend(other.passthrough);
    }
}
