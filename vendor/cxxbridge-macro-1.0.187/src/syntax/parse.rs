use crate::syntax::attrs::OtherAttrs;
use crate::syntax::cfg::CfgExpr;
use crate::syntax::discriminant::DiscriminantSet;
use crate::syntax::file::{Item, ItemForeignMod};
use crate::syntax::report::Errors;
use crate::syntax::repr::Repr;
use crate::syntax::Atom::*;
use crate::syntax::{
    attrs, error, Api, Array, Derive, Doc, Enum, EnumRepr, ExternFn, ExternType, FnKind,
    ForeignName, Impl, Include, IncludeKind, Lang, Lifetimes, NamedType, Namespace, Pair, Ptr,
    Receiver, Ref, Signature, SliceRef, Struct, Ty1, Type, TypeAlias, Var, Variant,
};
use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned};
use std::mem;
use syn::parse::{ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{
    Abi, Attribute, Error, Expr, Fields, FnArg, ForeignItem, ForeignItemFn, ForeignItemType,
    GenericArgument, GenericParam, Generics, Ident, ItemEnum, ItemImpl, ItemStruct, Lit, LitStr,
    Pat, PathArguments, Result, ReturnType, Signature as RustSignature, Token, TraitBound,
    TraitBoundModifier, Type as RustType, TypeArray, TypeBareFn, TypeParamBound, TypePath, TypePtr,
    TypeReference, Variant as RustVariant, Visibility,
};

pub(crate) mod kw {
    syn::custom_keyword!(Pin);
    syn::custom_keyword!(Result);
}

pub(crate) fn parse_items(
    cx: &mut Errors,
    items: Vec<Item>,
    trusted: bool,
    namespace: &Namespace,
) -> Vec<Api> {
    let mut apis = Vec::new();
    for item in items {
        match item {
            Item::Struct(item) => match parse_struct(cx, item, namespace) {
                Ok(strct) => apis.push(strct),
                Err(err) => cx.push(err),
            },
            Item::Enum(item) => apis.push(parse_enum(cx, item, namespace)),
            Item::ForeignMod(foreign_mod) => {
                parse_foreign_mod(cx, foreign_mod, &mut apis, trusted, namespace);
            }
            Item::Impl(item) => match parse_impl(cx, item) {
                Ok(imp) => apis.push(imp),
                Err(err) => cx.push(err),
            },
            Item::Use(item) => cx.error(item, error::USE_NOT_ALLOWED),
            Item::Other(item) => cx.error(item, "unsupported item"),
        }
    }
    apis
}

fn parse_struct(cx: &mut Errors, mut item: ItemStruct, namespace: &Namespace) -> Result<Api> {
    let mut cfg = CfgExpr::Unconditional;
    let mut doc = Doc::new();
    let mut derives = Vec::new();
    let mut repr = None;
    let mut namespace = namespace.clone();
    let mut cxx_name = None;
    let mut rust_name = None;
    let attrs = attrs::parse(
        cx,
        mem::take(&mut item.attrs),
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            derives: Some(&mut derives),
            repr: Some(&mut repr),
            namespace: Some(&mut namespace),
            cxx_name: Some(&mut cxx_name),
            rust_name: Some(&mut rust_name),
            ..Default::default()
        },
    );

    let align = match repr {
        Some(Repr::Align(align)) => Some(align),
        Some(Repr::Atom(_atom, span)) => {
            cx.push(Error::new(span, "unsupported alignment on a struct"));
            None
        }
        None => None,
    };

    let named_fields = match item.fields {
        Fields::Named(fields) => fields,
        Fields::Unit => return Err(Error::new_spanned(item, "unit structs are not supported")),
        Fields::Unnamed(_) => {
            return Err(Error::new_spanned(item, "tuple structs are not supported"));
        }
    };

    let mut lifetimes = Punctuated::new();
    let mut has_unsupported_generic_param = false;
    for pair in item.generics.params.into_pairs() {
        let (param, punct) = pair.into_tuple();
        match param {
            GenericParam::Lifetime(param) => {
                if !param.bounds.is_empty() && !has_unsupported_generic_param {
                    let msg = "lifetime parameter with bounds is not supported yet";
                    cx.error(&param, msg);
                    has_unsupported_generic_param = true;
                }
                lifetimes.push_value(param.lifetime);
                if let Some(punct) = punct {
                    lifetimes.push_punct(punct);
                }
            }
            GenericParam::Type(param) => {
                if !has_unsupported_generic_param {
                    let msg = "struct with generic type parameter is not supported yet";
                    cx.error(&param, msg);
                    has_unsupported_generic_param = true;
                }
            }
            GenericParam::Const(param) => {
                if !has_unsupported_generic_param {
                    let msg = "struct with const generic parameter is not supported yet";
                    cx.error(&param, msg);
                    has_unsupported_generic_param = true;
                }
            }
        }
    }

    if let Some(where_clause) = &item.generics.where_clause {
        cx.error(
            where_clause,
            "struct with where-clause is not supported yet",
        );
    }

    let mut fields = Vec::new();
    for field in named_fields.named {
        let ident = field.ident.unwrap();
        let mut cfg = CfgExpr::Unconditional;
        let mut doc = Doc::new();
        let mut cxx_name = None;
        let mut rust_name = None;
        let attrs = attrs::parse(
            cx,
            field.attrs,
            attrs::Parser {
                cfg: Some(&mut cfg),
                doc: Some(&mut doc),
                cxx_name: Some(&mut cxx_name),
                rust_name: Some(&mut rust_name),
                ..Default::default()
            },
        );
        let ty = match parse_type(&field.ty) {
            Ok(ty) => ty,
            Err(err) => {
                cx.push(err);
                continue;
            }
        };
        let visibility = visibility_pub(&field.vis, ident.span());
        let name = pair(Namespace::default(), &ident, cxx_name, rust_name);
        let colon_token = field.colon_token.unwrap();
        fields.push(Var {
            cfg,
            doc,
            attrs,
            visibility,
            name,
            colon_token,
            ty,
        });
    }

    let struct_token = item.struct_token;
    let visibility = visibility_pub(&item.vis, struct_token.span);
    let name = pair(namespace, &item.ident, cxx_name, rust_name);
    let generics = Lifetimes {
        lt_token: item.generics.lt_token,
        lifetimes,
        gt_token: item.generics.gt_token,
    };
    let brace_token = named_fields.brace_token;

    Ok(Api::Struct(Struct {
        cfg,
        doc,
        derives,
        align,
        attrs,
        visibility,
        struct_token,
        name,
        generics,
        brace_token,
        fields,
    }))
}

fn parse_enum(cx: &mut Errors, item: ItemEnum, namespace: &Namespace) -> Api {
    let mut cfg = CfgExpr::Unconditional;
    let mut doc = Doc::new();
    let mut derives = Vec::new();
    let mut repr = None;
    let mut namespace = namespace.clone();
    let mut cxx_name = None;
    let mut rust_name = None;
    let attrs = attrs::parse(
        cx,
        item.attrs,
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            derives: Some(&mut derives),
            repr: Some(&mut repr),
            namespace: Some(&mut namespace),
            cxx_name: Some(&mut cxx_name),
            rust_name: Some(&mut rust_name),
            ..Default::default()
        },
    );

    if !item.generics.params.is_empty() {
        let vis = &item.vis;
        let enum_token = item.enum_token;
        let ident = &item.ident;
        let generics = &item.generics;
        let span = quote!(#vis #enum_token #ident #generics);
        cx.error(span, "enum with generic parameters is not supported");
    } else if let Some(where_clause) = &item.generics.where_clause {
        cx.error(where_clause, "enum with where-clause is not supported");
    }

    let repr = match repr {
        Some(Repr::Atom(atom, _span)) => Some(atom),
        Some(Repr::Align(align)) => {
            cx.error(align, "C++ does not support custom alignment on an enum");
            None
        }
        None => None,
    };

    let mut variants = Vec::new();
    let mut discriminants = DiscriminantSet::new(repr);
    for variant in item.variants {
        match parse_variant(cx, variant, &mut discriminants) {
            Ok(variant) => variants.push(variant),
            Err(err) => cx.push(err),
        }
    }

    let enum_token = item.enum_token;
    let visibility = visibility_pub(&item.vis, enum_token.span);
    let brace_token = item.brace_token;

    let explicit_repr = repr.is_some();
    let mut repr = U8;
    match discriminants.inferred_repr() {
        Ok(inferred) => repr = inferred,
        Err(err) => {
            let span = quote_spanned!(brace_token.span=> #enum_token {});
            cx.error(span, err);
            variants.clear();
        }
    }

    let name = pair(namespace, &item.ident, cxx_name, rust_name);
    let repr_ident = Ident::new(repr.as_ref(), Span::call_site());
    let repr_type = Type::Ident(NamedType::new(repr_ident));
    let repr = EnumRepr {
        atom: repr,
        repr_type,
    };
    let generics = Lifetimes {
        lt_token: None,
        lifetimes: Punctuated::new(),
        gt_token: None,
    };

    Api::Enum(Enum {
        cfg,
        doc,
        derives,
        attrs,
        visibility,
        enum_token,
        name,
        generics,
        brace_token,
        variants,
        repr,
        explicit_repr,
    })
}

fn parse_variant(
    cx: &mut Errors,
    mut variant: RustVariant,
    discriminants: &mut DiscriminantSet,
) -> Result<Variant> {
    let mut cfg = CfgExpr::Unconditional;
    let mut doc = Doc::new();
    let mut default = false;
    let mut cxx_name = None;
    let mut rust_name = None;
    let attrs = attrs::parse(
        cx,
        mem::take(&mut variant.attrs),
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            default: Some(&mut default),
            cxx_name: Some(&mut cxx_name),
            rust_name: Some(&mut rust_name),
            ..Default::default()
        },
    );

    match variant.fields {
        Fields::Unit => {}
        _ => {
            let msg = "enums with data are not supported yet";
            return Err(Error::new_spanned(variant, msg));
        }
    }

    let expr = variant.discriminant.as_ref().map(|(_, expr)| expr);
    let try_discriminant = match &expr {
        Some(lit) => discriminants.insert(lit),
        None => discriminants.insert_next(),
    };
    let discriminant = match try_discriminant {
        Ok(discriminant) => discriminant,
        Err(err) => return Err(Error::new_spanned(variant, err)),
    };

    let name = pair(Namespace::ROOT, &variant.ident, cxx_name, rust_name);
    let expr = variant.discriminant.map(|(_, expr)| expr);

    Ok(Variant {
        cfg,
        doc,
        default,
        attrs,
        name,
        discriminant,
        expr,
    })
}

fn parse_foreign_mod(
    cx: &mut Errors,
    foreign_mod: ItemForeignMod,
    out: &mut Vec<Api>,
    trusted: bool,
    namespace: &Namespace,
) {
    let lang = match parse_lang(&foreign_mod.abi) {
        Ok(lang) => lang,
        Err(err) => return cx.push(err),
    };

    match lang {
        Lang::Rust => {
            if foreign_mod.unsafety.is_some() {
                let unsafety = foreign_mod.unsafety;
                let abi = &foreign_mod.abi;
                let span = quote!(#unsafety #abi);
                cx.error(span, "extern \"Rust\" block does not need to be unsafe");
            }
        }
        Lang::Cxx | Lang::CxxUnwind => {}
    }

    let trusted = trusted || foreign_mod.unsafety.is_some();

    let mut cfg = CfgExpr::Unconditional;
    let mut namespace = namespace.clone();
    let attrs = attrs::parse(
        cx,
        foreign_mod.attrs,
        attrs::Parser {
            cfg: Some(&mut cfg),
            namespace: Some(&mut namespace),
            ..Default::default()
        },
    );

    let mut items = Vec::new();
    for foreign in foreign_mod.items {
        match foreign {
            ForeignItem::Type(foreign) => {
                let ety = parse_extern_type(cx, foreign, lang, trusted, &cfg, &namespace, &attrs);
                items.push(ety);
            }
            ForeignItem::Fn(foreign) => {
                match parse_extern_fn(cx, foreign, lang, trusted, &cfg, &namespace, &attrs) {
                    Ok(efn) => items.push(efn),
                    Err(err) => cx.push(err),
                }
            }
            ForeignItem::Macro(foreign) if foreign.mac.path.is_ident("include") => {
                match foreign.mac.parse_body_with(parse_include) {
                    Ok(mut include) => {
                        include.cfg = cfg.clone();
                        items.push(Api::Include(include));
                    }
                    Err(err) => cx.push(err),
                }
            }
            ForeignItem::Verbatim(tokens) => {
                match parse_extern_verbatim(cx, tokens, lang, trusted, &cfg, &namespace, &attrs) {
                    Ok(api) => items.push(api),
                    Err(err) => cx.push(err),
                }
            }
            _ => cx.error(foreign, "unsupported foreign item"),
        }
    }

    if !trusted
        && items.iter().any(|api| match api {
            Api::CxxFunction(efn) => efn.unsafety.is_none(),
            _ => false,
        })
    {
        cx.error(
            foreign_mod.abi,
            "block must be declared `unsafe extern \"C++\"` if it contains any safe-to-call C++ functions",
        );
    }

    let mut types = items.iter().filter_map(|item| match item {
        Api::CxxType(ety) | Api::RustType(ety) => Some(&ety.name),
        Api::TypeAlias(alias) => Some(&alias.name),
        _ => None,
    });
    if let (Some(single_type), None) = (types.next(), types.next()) {
        let single_type = single_type.clone();
        for item in &mut items {
            if let Api::CxxFunction(efn) | Api::RustFunction(efn) = item {
                if let Some(receiver) = efn.sig.receiver_mut() {
                    if receiver.ty.rust == "Self" {
                        receiver.ty.rust = single_type.rust.clone();
                    }
                }
            }
        }
    }

    out.extend(items);
}

fn parse_lang(abi: &Abi) -> Result<Lang> {
    let Some(name) = &abi.name else {
        return Err(Error::new_spanned(
            abi,
            "ABI name is required, extern \"C++\" or extern \"Rust\"",
        ));
    };

    match name.value().as_str() {
        "C++" => Ok(Lang::Cxx),
        "C++-unwind" => Ok(Lang::CxxUnwind),
        "Rust" => Ok(Lang::Rust),
        _ => Err(Error::new_spanned(
            abi,
            "unrecognized ABI, requires either \"C++\" or \"Rust\"",
        )),
    }
}

fn parse_extern_type(
    cx: &mut Errors,
    foreign_type: ForeignItemType,
    lang: Lang,
    trusted: bool,
    extern_block_cfg: &CfgExpr,
    namespace: &Namespace,
    attrs: &OtherAttrs,
) -> Api {
    let mut cfg = extern_block_cfg.clone();
    let mut doc = Doc::new();
    let mut derives = Vec::new();
    let mut namespace = namespace.clone();
    let mut cxx_name = None;
    let mut rust_name = None;
    let mut attrs = attrs.clone();
    attrs.extend(attrs::parse(
        cx,
        foreign_type.attrs,
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            derives: Some(&mut derives),
            namespace: Some(&mut namespace),
            cxx_name: Some(&mut cxx_name),
            rust_name: Some(&mut rust_name),
            ..Default::default()
        },
    ));

    let type_token = foreign_type.type_token;
    let visibility = visibility_pub(&foreign_type.vis, type_token.span);
    let name = pair(namespace, &foreign_type.ident, cxx_name, rust_name);
    let generics = extern_type_lifetimes(cx, foreign_type.generics);
    let colon_token = None;
    let bounds = Vec::new();
    let semi_token = foreign_type.semi_token;

    (match lang {
        Lang::Cxx | Lang::CxxUnwind => Api::CxxType,
        Lang::Rust => Api::RustType,
    })(ExternType {
        cfg,
        lang,
        doc,
        derives,
        attrs,
        visibility,
        type_token,
        name,
        generics,
        colon_token,
        bounds,
        semi_token,
        trusted,
    })
}

fn parse_extern_fn(
    cx: &mut Errors,
    mut foreign_fn: ForeignItemFn,
    lang: Lang,
    trusted: bool,
    extern_block_cfg: &CfgExpr,
    namespace: &Namespace,
    attrs: &OtherAttrs,
) -> Result<Api> {
    let mut cfg = extern_block_cfg.clone();
    let mut doc = Doc::new();
    let mut namespace = namespace.clone();
    let mut cxx_name = None;
    let mut rust_name = None;
    let mut self_type = None;
    let mut attrs = attrs.clone();
    attrs.extend(attrs::parse(
        cx,
        mem::take(&mut foreign_fn.attrs),
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            namespace: Some(&mut namespace),
            cxx_name: Some(&mut cxx_name),
            rust_name: Some(&mut rust_name),
            self_type: Some(&mut self_type),
            ..Default::default()
        },
    ));

    let generics = &foreign_fn.sig.generics;
    if generics.where_clause.is_some()
        || generics.params.iter().any(|param| match param {
            GenericParam::Lifetime(lifetime) => !lifetime.bounds.is_empty(),
            GenericParam::Type(_) | GenericParam::Const(_) => true,
        })
    {
        return Err(Error::new_spanned(
            foreign_fn,
            "extern function with generic parameters is not supported yet",
        ));
    }

    if let Some(variadic) = &foreign_fn.sig.variadic {
        return Err(Error::new_spanned(
            variadic,
            "variadic function is not supported yet",
        ));
    }

    if foreign_fn.sig.asyncness.is_some() {
        return Err(Error::new_spanned(
            foreign_fn,
            "async function is not directly supported yet, but see https://cxx.rs/async.html \
            for a working approach, and https://github.com/pcwalton/cxx-async for some helpers; \
            eventually what you wrote will work but it isn't integrated into the cxx::bridge \
            macro yet",
        ));
    }

    if foreign_fn.sig.constness.is_some() {
        return Err(Error::new_spanned(
            foreign_fn,
            "const extern function is not supported",
        ));
    }

    if let Some(abi) = &foreign_fn.sig.abi {
        return Err(Error::new_spanned(
            abi,
            "explicit ABI on extern function is not supported",
        ));
    }

    let mut receiver = None;
    let mut args = Punctuated::new();
    for arg in foreign_fn.sig.inputs.pairs() {
        let (arg, comma) = arg.into_tuple();
        match arg {
            FnArg::Receiver(arg) => {
                if let Some((ampersand, lifetime)) = &arg.reference {
                    receiver = Some(Receiver {
                        pinned: false,
                        ampersand: *ampersand,
                        lifetime: lifetime.clone(),
                        mutable: arg.mutability.is_some(),
                        var: arg.self_token,
                        colon_token: Token![:](arg.self_token.span),
                        ty: NamedType::new(Ident::new("Self", arg.self_token.span)),
                        shorthand: true,
                        pin_tokens: None,
                        mutability: arg.mutability,
                    });
                    continue;
                }
                if let Some(colon_token) = arg.colon_token {
                    let ty = parse_type(&arg.ty)?;
                    if let Type::Ref(reference) = ty {
                        if let Type::Ident(ident) = reference.inner {
                            receiver = Some(Receiver {
                                pinned: reference.pinned,
                                ampersand: reference.ampersand,
                                lifetime: reference.lifetime,
                                mutable: reference.mutable,
                                var: Token![self](ident.rust.span()),
                                colon_token,
                                ty: ident,
                                shorthand: false,
                                pin_tokens: reference.pin_tokens,
                                mutability: reference.mutability,
                            });
                            continue;
                        }
                    }
                }
                return Err(Error::new_spanned(arg, "unsupported method receiver"));
            }
            FnArg::Typed(arg) => {
                let ident = match arg.pat.as_ref() {
                    Pat::Ident(pat) => pat.ident.clone(),
                    Pat::Wild(pat) => {
                        Ident::new(&format!("arg{}", args.len()), pat.underscore_token.span)
                    }
                    _ => return Err(Error::new_spanned(arg, "unsupported signature")),
                };
                let ty = parse_type(&arg.ty)?;
                let cfg = CfgExpr::Unconditional;
                let doc = Doc::new();
                let attrs = OtherAttrs::new();
                let visibility = Token![pub](ident.span());
                let name = pair(Namespace::default(), &ident, None, None);
                let colon_token = arg.colon_token;
                args.push_value(Var {
                    cfg,
                    doc,
                    attrs,
                    visibility,
                    name,
                    colon_token,
                    ty,
                });
                if let Some(comma) = comma {
                    args.push_punct(*comma);
                }
            }
        }
    }

    let kind = match (self_type, receiver) {
        (None, None) => FnKind::Free,
        (Some(self_type), None) => FnKind::Assoc(self_type),
        (None, Some(receiver)) => FnKind::Method(receiver),
        (Some(self_type), Some(receiver)) => {
            let msg = "function with Self type must not have a `self` argument";
            cx.error(self_type, msg);
            FnKind::Method(receiver)
        }
    };

    let mut throws_tokens = None;
    let ret = parse_return_type(&foreign_fn.sig.output, &mut throws_tokens)?;
    let throws = throws_tokens.is_some();
    let asyncness = foreign_fn.sig.asyncness;
    let unsafety = foreign_fn.sig.unsafety;
    let fn_token = foreign_fn.sig.fn_token;
    let inherited_span = unsafety.map_or(fn_token.span, |unsafety| unsafety.span);
    let visibility = visibility_pub(&foreign_fn.vis, inherited_span);
    let name = pair(namespace, &foreign_fn.sig.ident, cxx_name, rust_name);
    let generics = generics.clone();
    let paren_token = foreign_fn.sig.paren_token;
    let semi_token = foreign_fn.semi_token;

    Ok(match lang {
        Lang::Cxx | Lang::CxxUnwind => Api::CxxFunction,
        Lang::Rust => Api::RustFunction,
    }(ExternFn {
        cfg,
        lang,
        doc,
        attrs,
        visibility,
        name,
        sig: Signature {
            asyncness,
            unsafety,
            fn_token,
            generics,
            kind,
            args,
            ret,
            throws,
            paren_token,
            throws_tokens,
        },
        semi_token,
        trusted,
    }))
}

fn parse_extern_verbatim(
    cx: &mut Errors,
    tokens: TokenStream,
    lang: Lang,
    trusted: bool,
    extern_block_cfg: &CfgExpr,
    namespace: &Namespace,
    attrs: &OtherAttrs,
) -> Result<Api> {
    |input: ParseStream| -> Result<Api> {
        let unparsed_attrs = input.call(Attribute::parse_outer)?;
        let visibility: Visibility = input.parse()?;
        if input.peek(Token![type]) {
            parse_extern_verbatim_type(
                cx,
                unparsed_attrs,
                visibility,
                input,
                lang,
                trusted,
                extern_block_cfg,
                namespace,
                attrs,
            )
        } else if input.peek(Token![fn]) {
            parse_extern_verbatim_fn(input)
        } else {
            let span = input.cursor().token_stream();
            Err(Error::new_spanned(
                span,
                "unsupported foreign item, expected `type` or `fn`",
            ))
        }
    }
    .parse2(tokens)
}

fn parse_extern_verbatim_type(
    cx: &mut Errors,
    unparsed_attrs: Vec<Attribute>,
    visibility: Visibility,
    input: ParseStream,
    lang: Lang,
    trusted: bool,
    extern_block_cfg: &CfgExpr,
    namespace: &Namespace,
    attrs: &OtherAttrs,
) -> Result<Api> {
    let type_token: Token![type] = input.parse()?;
    let ident: Ident = input.parse()?;
    let generics: Generics = input.parse()?;
    let lifetimes = extern_type_lifetimes(cx, generics);
    let lookahead = input.lookahead1();
    if lookahead.peek(Token![=]) {
        // type Alias = crate::path::to::Type;
        parse_type_alias(
            cx,
            unparsed_attrs,
            visibility,
            type_token,
            ident,
            lifetimes,
            input,
            lang,
            extern_block_cfg,
            namespace,
            attrs,
        )
    } else if lookahead.peek(Token![:]) {
        // type Opaque: Bound2 + Bound2;
        parse_extern_type_bounded(
            cx,
            unparsed_attrs,
            visibility,
            type_token,
            ident,
            lifetimes,
            input,
            lang,
            trusted,
            extern_block_cfg,
            namespace,
            attrs,
        )
    } else {
        Err(lookahead.error())
    }
}

fn extern_type_lifetimes(cx: &mut Errors, generics: Generics) -> Lifetimes {
    let mut lifetimes = Punctuated::new();
    let mut has_unsupported_generic_param = false;
    for pair in generics.params.into_pairs() {
        let (param, punct) = pair.into_tuple();
        match param {
            GenericParam::Lifetime(param) => {
                if !param.bounds.is_empty() && !has_unsupported_generic_param {
                    let msg = "lifetime parameter with bounds is not supported yet";
                    cx.error(&param, msg);
                    has_unsupported_generic_param = true;
                }
                lifetimes.push_value(param.lifetime);
                if let Some(punct) = punct {
                    lifetimes.push_punct(punct);
                }
            }
            GenericParam::Type(param) => {
                if !has_unsupported_generic_param {
                    let msg = "extern type with generic type parameter is not supported yet";
                    cx.error(&param, msg);
                    has_unsupported_generic_param = true;
                }
            }
            GenericParam::Const(param) => {
                if !has_unsupported_generic_param {
                    let msg = "extern type with const generic parameter is not supported yet";
                    cx.error(&param, msg);
                    has_unsupported_generic_param = true;
                }
            }
        }
    }
    Lifetimes {
        lt_token: generics.lt_token,
        lifetimes,
        gt_token: generics.gt_token,
    }
}

fn parse_extern_verbatim_fn(input: ParseStream) -> Result<Api> {
    input.parse::<RustSignature>()?;
    input.parse::<Token![;]>()?;
    unreachable!()
}

fn parse_type_alias(
    cx: &mut Errors,
    unparsed_attrs: Vec<Attribute>,
    visibility: Visibility,
    type_token: Token![type],
    ident: Ident,
    generics: Lifetimes,
    input: ParseStream,
    lang: Lang,
    extern_block_cfg: &CfgExpr,
    namespace: &Namespace,
    attrs: &OtherAttrs,
) -> Result<Api> {
    let eq_token: Token![=] = input.parse()?;
    let ty: RustType = input.parse()?;
    let semi_token: Token![;] = input.parse()?;

    let mut cfg = extern_block_cfg.clone();
    let mut doc = Doc::new();
    let mut derives = Vec::new();
    let mut namespace = namespace.clone();
    let mut cxx_name = None;
    let mut rust_name = None;
    let mut attrs = attrs.clone();
    attrs.extend(attrs::parse(
        cx,
        unparsed_attrs,
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            derives: Some(&mut derives),
            namespace: Some(&mut namespace),
            cxx_name: Some(&mut cxx_name),
            rust_name: Some(&mut rust_name),
            ..Default::default()
        },
    ));

    if lang == Lang::Rust {
        let span = quote!(#type_token #semi_token);
        let msg = "type alias in extern \"Rust\" block is not supported";
        return Err(Error::new_spanned(span, msg));
    }

    let visibility = visibility_pub(&visibility, type_token.span);
    let name = pair(namespace, &ident, cxx_name, rust_name);

    Ok(Api::TypeAlias(TypeAlias {
        cfg,
        doc,
        derives,
        attrs,
        visibility,
        type_token,
        name,
        generics,
        eq_token,
        ty,
        semi_token,
    }))
}

fn parse_extern_type_bounded(
    cx: &mut Errors,
    unparsed_attrs: Vec<Attribute>,
    visibility: Visibility,
    type_token: Token![type],
    ident: Ident,
    generics: Lifetimes,
    input: ParseStream,
    lang: Lang,
    trusted: bool,
    extern_block_cfg: &CfgExpr,
    namespace: &Namespace,
    attrs: &OtherAttrs,
) -> Result<Api> {
    let mut bounds = Vec::new();
    let colon_token: Option<Token![:]> = input.parse()?;
    if colon_token.is_some() {
        loop {
            match input.parse()? {
                TypeParamBound::Trait(TraitBound {
                    paren_token: None,
                    modifier: TraitBoundModifier::None,
                    lifetimes: None,
                    path,
                }) if if let Some(derive) = path.get_ident().and_then(Derive::from) {
                    bounds.push(derive);
                    true
                } else {
                    false
                } => {}
                bound => cx.error(bound, "unsupported trait"),
            }

            let lookahead = input.lookahead1();
            if lookahead.peek(Token![+]) {
                input.parse::<Token![+]>()?;
            } else if lookahead.peek(Token![;]) {
                break;
            } else {
                return Err(lookahead.error());
            }
        }
    }
    let semi_token: Token![;] = input.parse()?;

    let mut cfg = extern_block_cfg.clone();
    let mut doc = Doc::new();
    let mut derives = Vec::new();
    let mut namespace = namespace.clone();
    let mut cxx_name = None;
    let mut rust_name = None;
    let mut attrs = attrs.clone();
    attrs.extend(attrs::parse(
        cx,
        unparsed_attrs,
        attrs::Parser {
            cfg: Some(&mut cfg),
            doc: Some(&mut doc),
            derives: Some(&mut derives),
            namespace: Some(&mut namespace),
            cxx_name: Some(&mut cxx_name),
            rust_name: Some(&mut rust_name),
            ..Default::default()
        },
    ));

    let visibility = visibility_pub(&visibility, type_token.span);
    let name = pair(namespace, &ident, cxx_name, rust_name);

    Ok(match lang {
        Lang::Cxx | Lang::CxxUnwind => Api::CxxType,
        Lang::Rust => Api::RustType,
    }(ExternType {
        cfg,
        lang,
        doc,
        derives,
        attrs,
        visibility,
        type_token,
        name,
        generics,
        colon_token,
        bounds,
        semi_token,
        trusted,
    }))
}

fn parse_impl(cx: &mut Errors, imp: ItemImpl) -> Result<Api> {
    let impl_token = imp.impl_token;

    let mut cfg = CfgExpr::Unconditional;
    let attrs = attrs::parse(
        cx,
        imp.attrs,
        attrs::Parser {
            cfg: Some(&mut cfg),
            ..Default::default()
        },
    );

    if !imp.items.is_empty() {
        let mut span = Group::new(Delimiter::Brace, TokenStream::new());
        span.set_span(imp.brace_token.span.join());
        return Err(Error::new_spanned(span, "expected an empty impl block"));
    }

    if let Some((bang, path, for_token)) = &imp.trait_ {
        let self_ty = &imp.self_ty;
        let span = quote!(#bang #path #for_token #self_ty);
        return Err(Error::new_spanned(
            span,
            "unexpected impl, expected something like `impl UniquePtr<T> {}`",
        ));
    }

    if let Some(where_clause) = imp.generics.where_clause {
        return Err(Error::new_spanned(
            where_clause,
            "where-clause on an impl is not supported yet",
        ));
    }
    let mut impl_generics = Lifetimes {
        lt_token: imp.generics.lt_token,
        lifetimes: Punctuated::new(),
        gt_token: imp.generics.gt_token,
    };
    for pair in imp.generics.params.into_pairs() {
        let (param, punct) = pair.into_tuple();
        match param {
            GenericParam::Lifetime(def) if def.bounds.is_empty() => {
                impl_generics.lifetimes.push_value(def.lifetime);
                if let Some(punct) = punct {
                    impl_generics.lifetimes.push_punct(punct);
                }
            }
            _ => {
                let span = quote!(#impl_token #impl_generics);
                return Err(Error::new_spanned(
                    span,
                    "generic parameter on an impl is not supported yet",
                ));
            }
        }
    }

    let mut negative_token = None;
    let mut self_ty = *imp.self_ty;
    if let RustType::Verbatim(ty) = &self_ty {
        let mut iter = ty.clone().into_iter();
        if let Some(TokenTree::Punct(punct)) = iter.next() {
            if punct.as_char() == '!' {
                let ty = iter.collect::<TokenStream>();
                if !ty.is_empty() {
                    negative_token = Some(Token![!](punct.span()));
                    self_ty = syn::parse2(ty)?;
                }
            }
        }
    }

    let ty = parse_type(&self_ty)?;
    let ty_generics = match &ty {
        Type::RustBox(ty)
        | Type::RustVec(ty)
        | Type::UniquePtr(ty)
        | Type::SharedPtr(ty)
        | Type::WeakPtr(ty)
        | Type::CxxVector(ty) => match &ty.inner {
            Type::Ident(ident) => ident.generics.clone(),
            _ => Lifetimes::default(),
        },
        Type::Ident(_)
        | Type::Ref(_)
        | Type::Ptr(_)
        | Type::Str(_)
        | Type::Fn(_)
        | Type::Void(_)
        | Type::SliceRef(_)
        | Type::Array(_) => Lifetimes::default(),
    };

    let negative = negative_token.is_some();
    let brace_token = imp.brace_token;

    Ok(Api::Impl(Impl {
        cfg,
        attrs,
        impl_token,
        impl_generics,
        negative,
        ty,
        ty_generics,
        brace_token,
        negative_token,
    }))
}

fn parse_include(input: ParseStream) -> Result<Include> {
    if input.peek(LitStr) {
        let lit: LitStr = input.parse()?;
        let span = lit.span();
        return Ok(Include {
            cfg: CfgExpr::Unconditional,
            path: lit.value(),
            kind: IncludeKind::Quoted,
            begin_span: span,
            end_span: span,
        });
    }

    if input.peek(Token![<]) {
        let mut path = String::new();

        let langle: Token![<] = input.parse()?;
        while !input.is_empty() && !input.peek(Token![>]) {
            let token: TokenTree = input.parse()?;
            match token {
                TokenTree::Ident(token) => path += &token.to_string(),
                TokenTree::Literal(token)
                    if token
                        .to_string()
                        .starts_with(|ch: char| ch.is_ascii_digit()) =>
                {
                    path += &token.to_string();
                }
                TokenTree::Punct(token) => path.push(token.as_char()),
                _ => return Err(Error::new(token.span(), "unexpected token in include path")),
            }
        }
        let rangle: Token![>] = input.parse()?;

        return Ok(Include {
            cfg: CfgExpr::Unconditional,
            path,
            kind: IncludeKind::Bracketed,
            begin_span: langle.span,
            end_span: rangle.span,
        });
    }

    Err(input.error("expected \"quoted/path/to\" or <bracketed/path/to>"))
}

fn parse_type(ty: &RustType) -> Result<Type> {
    match ty {
        RustType::Reference(ty) => parse_type_reference(ty),
        RustType::Ptr(ty) => parse_type_ptr(ty),
        RustType::Path(ty) => parse_type_path(ty),
        RustType::Array(ty) => parse_type_array(ty),
        RustType::BareFn(ty) => parse_type_fn(ty),
        RustType::Tuple(ty) if ty.elems.is_empty() => Ok(Type::Void(ty.paren_token.span.join())),
        _ => Err(Error::new_spanned(ty, "unsupported type")),
    }
}

fn parse_type_reference(ty: &TypeReference) -> Result<Type> {
    let ampersand = ty.and_token;
    let lifetime = ty.lifetime.clone();
    let mutable = ty.mutability.is_some();
    let mutability = ty.mutability;

    if let RustType::Slice(slice) = ty.elem.as_ref() {
        let inner = parse_type(&slice.elem)?;
        let bracket = slice.bracket_token;
        return Ok(Type::SliceRef(Box::new(SliceRef {
            ampersand,
            lifetime,
            mutable,
            bracket,
            inner,
            mutability,
        })));
    }

    let inner = parse_type(&ty.elem)?;
    let pinned = false;
    let pin_tokens = None;

    Ok(match &inner {
        Type::Ident(ident) if ident.rust == "str" => {
            if ty.mutability.is_some() {
                return Err(Error::new_spanned(ty, "unsupported type"));
            } else {
                Type::Str
            }
        }
        _ => Type::Ref,
    }(Box::new(Ref {
        pinned,
        ampersand,
        lifetime,
        mutable,
        inner,
        pin_tokens,
        mutability,
    })))
}

fn parse_type_ptr(ty: &TypePtr) -> Result<Type> {
    let star = ty.star_token;
    let mutable = ty.mutability.is_some();
    let constness = ty.const_token;
    let mutability = ty.mutability;

    let inner = parse_type(&ty.elem)?;

    Ok(Type::Ptr(Box::new(Ptr {
        star,
        mutable,
        inner,
        mutability,
        constness,
    })))
}

fn parse_type_path(ty: &TypePath) -> Result<Type> {
    let path = &ty.path;
    if ty.qself.is_none() && path.leading_colon.is_none() && path.segments.len() == 1 {
        let segment = &path.segments[0];
        let ident = segment.ident.clone();
        match &segment.arguments {
            PathArguments::None => return Ok(Type::Ident(NamedType::new(ident))),
            PathArguments::AngleBracketed(generic) => {
                if ident == "UniquePtr" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        let inner = parse_type(arg)?;
                        return Ok(Type::UniquePtr(Box::new(Ty1 {
                            name: ident,
                            langle: generic.lt_token,
                            inner,
                            rangle: generic.gt_token,
                        })));
                    }
                } else if ident == "SharedPtr" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        let inner = parse_type(arg)?;
                        return Ok(Type::SharedPtr(Box::new(Ty1 {
                            name: ident,
                            langle: generic.lt_token,
                            inner,
                            rangle: generic.gt_token,
                        })));
                    }
                } else if ident == "WeakPtr" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        let inner = parse_type(arg)?;
                        return Ok(Type::WeakPtr(Box::new(Ty1 {
                            name: ident,
                            langle: generic.lt_token,
                            inner,
                            rangle: generic.gt_token,
                        })));
                    }
                } else if ident == "CxxVector" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        let inner = parse_type(arg)?;
                        return Ok(Type::CxxVector(Box::new(Ty1 {
                            name: ident,
                            langle: generic.lt_token,
                            inner,
                            rangle: generic.gt_token,
                        })));
                    }
                } else if ident == "Box" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        let inner = parse_type(arg)?;
                        return Ok(Type::RustBox(Box::new(Ty1 {
                            name: ident,
                            langle: generic.lt_token,
                            inner,
                            rangle: generic.gt_token,
                        })));
                    }
                } else if ident == "Vec" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        let inner = parse_type(arg)?;
                        return Ok(Type::RustVec(Box::new(Ty1 {
                            name: ident,
                            langle: generic.lt_token,
                            inner,
                            rangle: generic.gt_token,
                        })));
                    }
                } else if ident == "Pin" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        let inner = parse_type(arg)?;
                        let pin_token = kw::Pin(ident.span());
                        if let Type::Ref(mut inner) = inner {
                            inner.pinned = true;
                            inner.pin_tokens =
                                Some((pin_token, generic.lt_token, generic.gt_token));
                            return Ok(Type::Ref(inner));
                        }
                    }
                } else {
                    let mut lifetimes = Punctuated::new();
                    let mut only_lifetimes = true;
                    for pair in generic.args.pairs() {
                        let (param, punct) = pair.into_tuple();
                        if let GenericArgument::Lifetime(param) = param {
                            lifetimes.push_value(param.clone());
                            if let Some(punct) = punct {
                                lifetimes.push_punct(*punct);
                            }
                        } else {
                            only_lifetimes = false;
                            break;
                        }
                    }
                    if only_lifetimes {
                        return Ok(Type::Ident(NamedType {
                            rust: ident,
                            generics: Lifetimes {
                                lt_token: Some(generic.lt_token),
                                lifetimes,
                                gt_token: Some(generic.gt_token),
                            },
                        }));
                    }
                }
            }
            PathArguments::Parenthesized(_) => {}
        }
    }

    if ty.qself.is_none() && path.segments.len() == 2 && path.segments[0].ident == "cxx" {
        return Err(Error::new_spanned(
            ty,
            "unexpected `cxx::` qualifier found in a `#[cxx::bridge]`",
        ));
    }

    Err(Error::new_spanned(ty, "unsupported type"))
}

fn parse_type_array(ty: &TypeArray) -> Result<Type> {
    let inner = parse_type(&ty.elem)?;

    let Expr::Lit(len_expr) = &ty.len else {
        let msg = "unsupported expression, array length must be an integer literal";
        return Err(Error::new_spanned(&ty.len, msg));
    };

    let Lit::Int(len_token) = &len_expr.lit else {
        let msg = "array length must be an integer literal";
        return Err(Error::new_spanned(len_expr, msg));
    };

    let len = len_token.base10_parse::<usize>()?;
    if len == 0 {
        let msg = "array with zero size is not supported";
        return Err(Error::new_spanned(ty, msg));
    }

    let bracket = ty.bracket_token;
    let semi_token = ty.semi_token;

    Ok(Type::Array(Box::new(Array {
        bracket,
        inner,
        semi_token,
        len,
        len_token: len_token.clone(),
    })))
}

fn parse_type_fn(ty: &TypeBareFn) -> Result<Type> {
    if ty.lifetimes.is_some() {
        return Err(Error::new_spanned(
            ty,
            "function pointer with lifetime parameters is not supported yet",
        ));
    }

    if ty.variadic.is_some() {
        return Err(Error::new_spanned(
            ty,
            "variadic function pointer is not supported yet",
        ));
    }

    let args = ty
        .inputs
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let (ident, colon_token) = match &arg.name {
                Some((ident, colon_token)) => (ident.clone(), *colon_token),
                None => {
                    let fn_span = ty.paren_token.span.join();
                    let ident = format_ident!("arg{}", i, span = fn_span);
                    let colon_token = Token![:](fn_span);
                    (ident, colon_token)
                }
            };
            let ty = parse_type(&arg.ty)?;
            let cfg = CfgExpr::Unconditional;
            let doc = Doc::new();
            let attrs = OtherAttrs::new();
            let visibility = Token![pub](ident.span());
            let name = pair(Namespace::default(), &ident, None, None);
            Ok(Var {
                cfg,
                doc,
                attrs,
                visibility,
                name,
                colon_token,
                ty,
            })
        })
        .collect::<Result<_>>()?;

    let mut throws_tokens = None;
    let ret = parse_return_type(&ty.output, &mut throws_tokens)?;
    let throws = throws_tokens.is_some();

    let asyncness = None;
    let unsafety = ty.unsafety;
    let fn_token = ty.fn_token;
    let generics = Generics::default();
    let kind = FnKind::Free;
    let paren_token = ty.paren_token;

    Ok(Type::Fn(Box::new(Signature {
        asyncness,
        unsafety,
        fn_token,
        generics,
        kind,
        args,
        ret,
        throws,
        paren_token,
        throws_tokens,
    })))
}

fn parse_return_type(
    ty: &ReturnType,
    throws_tokens: &mut Option<(kw::Result, Token![<], Token![>])>,
) -> Result<Option<Type>> {
    let mut ret = match ty {
        ReturnType::Default => return Ok(None),
        ReturnType::Type(_, ret) => ret.as_ref(),
    };

    if let RustType::Path(ty) = ret {
        let path = &ty.path;
        if ty.qself.is_none() && path.leading_colon.is_none() && path.segments.len() == 1 {
            let segment = &path.segments[0];
            let ident = segment.ident.clone();
            if let PathArguments::AngleBracketed(generic) = &segment.arguments {
                if ident == "Result" && generic.args.len() == 1 {
                    if let GenericArgument::Type(arg) = &generic.args[0] {
                        ret = arg;
                        *throws_tokens =
                            Some((kw::Result(ident.span()), generic.lt_token, generic.gt_token));
                    }
                }
            }
        }
    }

    match parse_type(ret)? {
        Type::Void(_) => Ok(None),
        ty => Ok(Some(ty)),
    }
}

fn visibility_pub(vis: &Visibility, inherited: Span) -> Token![pub] {
    Token![pub](match vis {
        Visibility::Public(vis) => vis.span,
        Visibility::Restricted(vis) => vis.pub_token.span,
        Visibility::Inherited => inherited,
    })
}

fn pair(
    namespace: Namespace,
    default: &Ident,
    cxx: Option<ForeignName>,
    rust: Option<Ident>,
) -> Pair {
    Pair {
        namespace,
        cxx: cxx
            .unwrap_or_else(|| ForeignName::parse(&default.to_string(), default.span()).unwrap()),
        rust: rust.unwrap_or_else(|| default.clone()),
    }
}
