// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::crate_ident_new;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::{quote, quote_spanned};
use std::collections::BTreeMap;
use syn::ext::IdentExt;
use syn::parenthesized;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;
use syn::{parse_quote_spanned, Attribute, LitStr};

pub struct PropsMacroInput {
    wrapper_ty: syn::Path,
    ext_trait: Option<Option<syn::Ident>>,
    ident: syn::Ident,
    props: Vec<PropDesc>,
}

pub struct PropertiesAttrs {
    wrapper_ty: syn::Path,
    // None => no ext trait,
    // Some(None) => derive the ext trait from the wrapper type,
    // Some(Some(ident)) => use the given ext trait Ident
    ext_trait: Option<Option<syn::Ident>>,
}

impl Parse for PropertiesAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut wrapper_ty = None;
        let mut ext_trait = None;

        while !input.is_empty() {
            let ident = input.parse::<syn::Ident>()?;
            if ident == "wrapper_type" {
                let _eq = input.parse::<Token![=]>()?;
                wrapper_ty = Some(input.parse::<syn::Path>()?);
            } else if ident == "ext_trait" {
                if input.peek(Token![=]) {
                    let _eq = input.parse::<Token![=]>()?;
                    let ident = input.parse::<syn::Ident>()?;
                    ext_trait = Some(Some(ident));
                } else {
                    ext_trait = Some(None);
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            wrapper_ty: wrapper_ty.ok_or_else(|| {
                syn::Error::new(input.span(), "missing #[properties(wrapper_type = ...)]")
            })?,
            ext_trait,
        })
    }
}

impl Parse for PropsMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input: syn::DeriveInput = input.parse()?;
        let attrs = derive_input
            .attrs
            .iter()
            .find(|x| x.path().is_ident("properties"))
            .ok_or_else(|| {
                syn::Error::new(
                    derive_input.span(),
                    "missing #[properties(wrapper_type = ...)]",
                )
            })?;
        let attrs: PropertiesAttrs = attrs.parse_args()?;
        let props: Vec<_> = match derive_input.data {
            syn::Data::Struct(struct_data) => parse_fields(struct_data.fields)?,
            _ => {
                return Err(syn::Error::new(
                    derive_input.span(),
                    "Properties can only be derived on structs",
                ))
            }
        };
        Ok(Self {
            wrapper_ty: attrs.wrapper_ty,
            ext_trait: attrs.ext_trait,
            ident: derive_input.ident,
            props,
        })
    }
}

enum MaybeCustomFn {
    Custom(Box<syn::Expr>),
    Default,
}

impl std::convert::From<Option<syn::Expr>> for MaybeCustomFn {
    fn from(item: Option<syn::Expr>) -> Self {
        match item {
            Some(expr) => Self::Custom(Box::new(expr)),
            None => Self::Default,
        }
    }
}

enum PropAttr {
    // builder(required_params).parameter(value)
    // becomes
    // Builder(Punctuated(required_params), Optionals(TokenStream))
    Builder(Punctuated<syn::Expr, Token![,]>, TokenStream2),

    // ident
    Nullable,

    // ident [= expr]
    Get(Option<syn::Expr>),
    Set(Option<syn::Expr>),

    // ident = expr
    OverrideClass(syn::Type),
    OverrideInterface(syn::Type),

    // ident = expr
    Type(syn::Type),

    // This will get translated from `ident = value` to `.ident(value)`
    // and will get appended after the `builder(...)` call.
    // ident [= expr]
    BuilderField((syn::Ident, Option<syn::Expr>)),

    // ident = ident
    Member(syn::Ident),

    // ident = "literal"
    Name(syn::LitStr),

    // ident
    Default,
}

impl Parse for PropAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.call(syn::Ident::parse_any)?;
        let name_str = name.to_string();

        let res = if input.peek(Token![=]) {
            let _assign_token: Token![=] = input.parse()?;
            // name = expr | type | ident
            match &*name_str {
                "name" => PropAttr::Name(input.parse()?),
                "get" => PropAttr::Get(Some(input.parse()?)),
                "set" => PropAttr::Set(Some(input.parse()?)),
                "override_class" => PropAttr::OverrideClass(input.parse()?),
                "override_interface" => PropAttr::OverrideInterface(input.parse()?),
                "type" => PropAttr::Type(input.parse()?),
                "member" => PropAttr::Member(input.parse()?),
                // Special case "default = ..." and map it to .default_value(...)
                "default" => PropAttr::BuilderField((
                    syn::Ident::new("default_value", name.span()),
                    Some(input.parse()?),
                )),
                _ => PropAttr::BuilderField((name, Some(input.parse()?))),
            }
        } else if input.peek(syn::token::Paren) {
            match &*name_str {
                "builder" => {
                    let content;
                    parenthesized!(content in input);
                    let required = content.parse_terminated(syn::Expr::parse, Token![,])?;
                    let rest: TokenStream2 = input.parse()?;
                    PropAttr::Builder(required, rest)
                }
                _ => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!("Unsupported attribute list {name_str}(...)"),
                    ))
                }
            }
        } else {
            // attributes with only the identifier name
            match &*name_str {
                "nullable" => PropAttr::Nullable,
                "get" => PropAttr::Get(None),
                "set" => PropAttr::Set(None),
                "readwrite" | "read_only" | "write_only" => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!(
                            "{name} is a flag managed by the Properties macro. \
                            Use `get` and `set` to manage read and write access to a property",
                        ),
                    ))
                }
                "default" => PropAttr::Default,
                _ => PropAttr::BuilderField((name, None)),
            }
        };
        Ok(res)
    }
}

#[derive(Default)]
struct ReceivedAttrs {
    nullable: bool,
    get: Option<MaybeCustomFn>,
    set: Option<MaybeCustomFn>,
    override_class: Option<syn::Type>,
    override_interface: Option<syn::Type>,
    ty: Option<syn::Type>,
    member: Option<syn::Ident>,
    name: Option<syn::LitStr>,
    builder: Option<(Punctuated<syn::Expr, Token![,]>, TokenStream2)>,
    builder_fields: BTreeMap<syn::Ident, Option<syn::Expr>>,
    use_default: bool,
}

impl Parse for ReceivedAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = syn::punctuated::Punctuated::<PropAttr, Token![,]>::parse_terminated(input)?;
        let this = attrs.into_iter().fold(Self::default(), |mut this, attr| {
            this.set_from_attr(attr);
            this
        });

        Ok(this)
    }
}

impl ReceivedAttrs {
    fn set_from_attr(&mut self, attr: PropAttr) {
        match attr {
            PropAttr::Nullable => self.nullable = true,
            PropAttr::Get(some_fn) => self.get = Some(some_fn.into()),
            PropAttr::Set(some_fn) => self.set = Some(some_fn.into()),
            PropAttr::Name(lit) => self.name = Some(lit),
            PropAttr::OverrideClass(ty) => self.override_class = Some(ty),
            PropAttr::OverrideInterface(ty) => self.override_interface = Some(ty),
            PropAttr::Type(ty) => self.ty = Some(ty),
            PropAttr::Member(member) => self.member = Some(member),
            PropAttr::Builder(required_params, optionals) => {
                self.builder = Some((required_params, optionals))
            }
            PropAttr::BuilderField((ident, expr)) => {
                self.builder_fields.insert(ident, expr);
            }
            PropAttr::Default => {
                self.use_default = true;
            }
        }
    }
}

// It's a cleaned up version of `ReceivedAttrs` where some missing attributes get a default,
// generated value.
struct PropDesc {
    attrs_span: proc_macro2::Span,
    field_ident: syn::Ident,
    ty: syn::Type,
    name: syn::LitStr,
    comments: Vec<Attribute>,
    override_class: Option<syn::Type>,
    override_interface: Option<syn::Type>,
    nullable: bool,
    get: Option<MaybeCustomFn>,
    set: Option<MaybeCustomFn>,
    member: Option<syn::Ident>,
    builder: Option<(Punctuated<syn::Expr, Token![,]>, TokenStream2)>,
    builder_fields: BTreeMap<syn::Ident, Option<syn::Expr>>,
    is_construct_only: bool,
    use_default: bool,
}

impl PropDesc {
    fn new(
        attrs_span: proc_macro2::Span,
        field_ident: syn::Ident,
        field_ty: syn::Type,
        comments: Vec<Attribute>,
        attrs: ReceivedAttrs,
    ) -> syn::Result<Self> {
        let ReceivedAttrs {
            nullable,
            get,
            mut set,
            override_class,
            override_interface,
            ty,
            member,
            name,
            builder,
            builder_fields,
            use_default,
        } = attrs;

        let is_construct_only = builder_fields.iter().any(|(k, _)| *k == "construct_only");
        if is_construct_only && set.is_none() {
            // Insert a default internal setter automatically
            set = Some(MaybeCustomFn::Default);
        }

        if get.is_none() && set.is_none() {
            return Err(syn::Error::new(
                attrs_span,
                "No `get` or `set` specified: at least one is required.".to_string(),
            ));
        }

        if override_class.is_some() && override_interface.is_some() {
            return Err(syn::Error::new(
                attrs_span,
                "Both `override_class` and `override_interface` specified.".to_string(),
            ));
        }

        // Fill needed, but missing, attributes with calculated default values
        let name = name.unwrap_or_else(|| {
            syn::LitStr::new(
                &field_ident.to_string().trim_matches('_').replace('_', "-"),
                field_ident.span(),
            )
        });
        let ty = ty.unwrap_or_else(|| field_ty.clone());

        // Now that everything is set and safe, return the final property description
        Ok(Self {
            attrs_span,
            field_ident,
            ty,
            name,
            comments,
            override_class,
            override_interface,
            nullable,
            get,
            set,
            member,
            builder,
            builder_fields,
            is_construct_only,
            use_default,
        })
    }
    fn is_overriding(&self) -> bool {
        self.override_class.is_some() || self.override_interface.is_some()
    }
}

fn expand_param_spec(prop: &PropDesc) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let PropDesc {
        ty,
        name,
        builder,
        use_default,
        ..
    } = prop;
    let stripped_name = strip_raw_prefix_from_name(name);

    match (&prop.override_class, &prop.override_interface) {
        (Some(c), None) => {
            return quote!(#crate_ident::ParamSpecOverride::for_class::<#c>(#stripped_name))
        }
        (None, Some(i)) => {
            return quote!(#crate_ident::ParamSpecOverride::for_interface::<#i>(#stripped_name))
        }
        (Some(_), Some(_)) => {
            unreachable!("Both `override_class` and `override_interface` specified")
        }
        (None, None) => (),
    };

    let rw_flags = match (&prop.get, &prop.set) {
        (Some(_), Some(_)) => quote!(.readwrite()),
        (Some(_), None) => quote!(.read_only()),
        (None, Some(_)) => quote!(.write_only()),
        (None, None) => unreachable!("No `get` or `set` specified"),
    };

    let builder_call = builder
        .as_ref()
        .cloned()
        .map(|(mut required_params, chained_methods)| {
            let name_expr = syn::ExprLit {
                attrs: vec![],
                lit: syn::Lit::Str(stripped_name.to_owned()),
            };
            required_params.insert(0, name_expr.into());
            let required_params = required_params.iter();

            quote!((#(#required_params,)*)#chained_methods)
        })
        .unwrap_or(quote!((#stripped_name)));

    let builder_fields = prop.builder_fields.iter().map(|(k, v)| quote!(.#k(#v)));

    let span = prop.attrs_span;

    // Figure out if we should use the default version or the one that explicitly sets the `Default` value.
    let (trait_name, fn_name) = if *use_default {
        (
            quote!(HasParamSpecDefaulted),
            quote!(param_spec_builder_defaulted),
        )
    } else {
        (quote!(HasParamSpec), quote!(param_spec_builder))
    };

    quote_spanned! {span=>
        <<#ty as #crate_ident::property::Property>::Value as #crate_ident::#trait_name>
            ::#fn_name() #builder_call
            #rw_flags
            #(#builder_fields)*
            .build()
    }
}

fn expand_properties_fn(props: &[PropDesc]) -> TokenStream2 {
    let n_props = props.len();
    let crate_ident = crate_ident_new();
    let param_specs = props.iter().map(expand_param_spec);
    quote!(
        fn derived_properties() -> &'static [#crate_ident::ParamSpec] {
            use #crate_ident::prelude::ParamSpecBuilderExt;
            static PROPERTIES: ::std::sync::OnceLock<[#crate_ident::ParamSpec; #n_props]> = ::std::sync::OnceLock::new();
            PROPERTIES.get_or_init(|| [
                #(#param_specs,)*
            ])
        }
    )
}

fn expand_property_fn(props: &[PropDesc]) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let match_branch_get = props.iter().flat_map(|p| {
        let PropDesc {
            name,
            field_ident,
            member,
            get,
            ty,
            ..
        } = p;

        let enum_ident = name_to_enum_ident(name.value());
        let span = p.attrs_span;
        get.as_ref().map(|get| {
            let body = match (member, get) {
                (_, MaybeCustomFn::Custom(expr)) => quote!(
                    DerivedPropertiesEnum::#enum_ident => {
                        let value: <#ty as #crate_ident::property::Property>::Value = (#expr)(&self);
                        ::std::convert::From::from(value)
                    }
                ),
                (None, MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident =>
                        #crate_ident::property::PropertyGet::get(&self.#field_ident, |v| ::std::convert::From::from(v))

                ),
                (Some(member), MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident =>
                        #crate_ident::property::PropertyGet::get(&self.#field_ident, |v| ::std::convert::From::from(&v.#member))

                ),
            };
            quote_spanned!(span=> #body)
        })
    });
    quote!(
        fn derived_property(
            &self,
            id: usize,
            pspec: &#crate_ident::ParamSpec
        ) -> #crate_ident::Value {
            let prop: DerivedPropertiesEnum = std::convert::TryFrom::try_from(id-1)
                .unwrap_or_else(|_| panic!("property not defined {}", pspec.name()));
            match prop {
                #(#match_branch_get,)*
                _ => panic!("missing getter for property {}", pspec.name()),
            }
        }
    )
}

fn expand_set_property_fn(props: &[PropDesc]) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let match_branch_set = props.iter().flat_map(|p| {
        let PropDesc {
            name,
            field_ident,
            member,
            set,
            ty,
            ..
        } = p;
        let stripped_name = strip_raw_prefix_from_name(name);
        let crate_ident = crate_ident_new();
        let enum_ident = name_to_enum_ident(name.value());
        let span = p.attrs_span;
        let expect = quote!(.unwrap_or_else(
            |err| panic!(
                "Invalid conversion from `glib::value::Value` to `{}` inside setter for property `{}`: {:?}",
                ::std::any::type_name::<<#ty as #crate_ident::property::Property>::Value>(), #stripped_name, err
            )
        ));
        set.as_ref().map(|set| {
            let body = match (member, set) {
                (_, MaybeCustomFn::Custom(expr)) => quote!(
                    DerivedPropertiesEnum::#enum_ident => {
                        (#expr)(&self, #crate_ident::Value::get(value)#expect);
                    }
                ),
                (None, MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident => {
                        #crate_ident::property::PropertySet::set(
                            &self.#field_ident,
                            #crate_ident::Value::get(value)#expect
                        );
                    }
                ),
                (Some(member), MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident => {
                        #crate_ident::property::PropertySetNested::set_nested(
                            &self.#field_ident,
                            move |v| v.#member = #crate_ident::Value::get(value)#expect
                        );
                    }
                ),
            };
            quote_spanned!(span=> #body)
        })
    });
    quote!(
        #[allow(unreachable_code)]
        fn derived_set_property(&self,
            id: usize,
            value: &#crate_ident::Value,
            pspec: &#crate_ident::ParamSpec
        ){
            let prop: DerivedPropertiesEnum = std::convert::TryFrom::try_from(id-1)
                .unwrap_or_else(|_| panic!("property not defined {}", pspec.name()));
            match prop {
                #(#match_branch_set,)*
                _ => panic!("missing setter for property {}", pspec.name()),
            }
        }
    )
}

fn parse_fields(fields: syn::Fields) -> syn::Result<Vec<PropDesc>> {
    let mut properties = vec![];

    for field in fields.into_iter() {
        let syn::Field {
            ident, attrs, ty, ..
        } = field;
        // Store the comments until the next `#[property]` we see and then attach them to it.
        let mut comments: Vec<Attribute> = vec![];
        for prop_attr in attrs.iter() {
            if prop_attr.path().is_ident("doc") {
                comments.push(prop_attr.clone());
            } else if prop_attr.path().is_ident("property") {
                let span = prop_attr.span();
                let existing_comments = comments;
                comments = vec![];
                properties.push(PropDesc::new(
                    span,
                    ident.as_ref().unwrap().clone(),
                    ty.clone(),
                    existing_comments,
                    prop_attr.parse_args()?,
                )?);
            }
        }
    }

    Ok(properties)
}

/// Converts a glib property name to a correct rust ident
fn name_to_ident(name: &syn::LitStr) -> syn::Ident {
    format_ident!("{}", name.value().replace('-', "_"))
}

/// Strips out raw identifier prefix (`r#`) from literal string items
fn strip_raw_prefix_from_name(name: &LitStr) -> LitStr {
    LitStr::new(
        name.value().strip_prefix("r#").unwrap_or(&name.value()),
        name.span(),
    )
}

/// Splits the comments for a property between the getter and setter
///
/// The return tuple is the attributes to copy over into the getter and setter
/// respectively.
fn arrange_property_comments(comments: &[Attribute]) -> (Vec<&Attribute>, Vec<&Attribute>) {
    let mut untagged = vec![];
    let mut getter = vec![];
    let mut setter = vec![];
    let mut saw_section = false;

    // We start with no tags so if the programmer doesn't split the comments we can still arrange them.
    let mut current_section = &mut untagged;
    for attr in comments {
        if let syn::Meta::NameValue(meta) = &attr.meta {
            if let syn::Expr::Lit(expr) = &meta.value {
                if let syn::Lit::Str(lit_str) = &expr.lit {
                    // Now that we have the one line of comment, see if we need
                    // to switch a particular section to be the active one (via
                    // the header syntax) or add the current line to the active
                    // section.
                    match lit_str.value().trim() {
                        "# Getter" => {
                            current_section = &mut getter;
                            saw_section = true;
                        }
                        "# Setter" => {
                            current_section = &mut setter;
                            saw_section = true;
                        }
                        _ => current_section.push(attr),
                    }
                }
            }
        }
    }

    // If no sections were defined then we put the same in both
    if !saw_section {
        return (untagged.clone(), untagged);
    }

    (getter, setter)
}

fn expand_impl_getset_properties(props: &[PropDesc]) -> Vec<syn::ImplItemFn> {
    let crate_ident = crate_ident_new();
    let defs = props.iter().filter(|p| !p.is_overriding()).map(|p| {
        let name = &p.name;
        let stripped_name = strip_raw_prefix_from_name(name);
        let ident = name_to_ident(name);
        let ty = &p.ty;

        let (getter_docs, setter_docs) = arrange_property_comments(&p.comments);

        let getter = p.get.is_some().then(|| {
            let span = p.attrs_span;
            parse_quote_spanned!(span=>
                #(#getter_docs)*
                #[must_use]
                #[allow(dead_code)]
                pub fn #ident(&self) -> <#ty as #crate_ident::property::Property>::Value {
                    self.property::<<#ty as #crate_ident::property::Property>::Value>(#stripped_name)
                }
            )
        });

        let setter = (p.set.is_some() && !p.is_construct_only).then(|| {
            let ident = format_ident!("set_{}", ident);
            let target_ty = quote!(<<#ty as #crate_ident::property::Property>::Value as #crate_ident::prelude::HasParamSpec>::SetValue);
            let set_ty = if p.nullable {
               quote!(::core::option::Option<impl std::borrow::Borrow<#target_ty>>)
            } else {
               quote!(impl std::borrow::Borrow<#target_ty>)
            };
            let upcasted_borrowed_value = if p.nullable {
                quote!(
                    value.as_ref().map(|v| std::borrow::Borrow::borrow(v))
                )
            } else {
                quote!(
                    std::borrow::Borrow::borrow(&value)
                )
            };
            let span = p.attrs_span;
            parse_quote_spanned!(span=>
                #(#setter_docs)*
                #[allow(dead_code)]
                pub fn #ident<'a>(&self, value: #set_ty) {
                    self.set_property_from_value(#stripped_name, &::std::convert::From::from(#upcasted_borrowed_value))
                }
            )
        });

        [getter, setter]
    });
    defs.flatten() // flattens []
        .flatten() // removes None
        .collect::<Vec<_>>()
}

fn expand_impl_connect_prop_notify(props: &[PropDesc]) -> Vec<syn::ImplItemFn> {
    let crate_ident = crate_ident_new();
    let connection_fns = props.iter().filter(|p| !p.is_overriding()).map(|p| -> syn::ImplItemFn {
        let name = &p.name;
        let stripped_name = strip_raw_prefix_from_name(name);
        let fn_ident = format_ident!("connect_{}_notify", name_to_ident(name));
        let span = p.attrs_span;
        let doc = format!("Listen for notifications of a change in the `{}` property", name.value());
        parse_quote_spanned!(span=>
            #[doc = #doc]
            #[allow(dead_code)]
            pub fn #fn_ident<F: Fn(&Self) + 'static>(&self, f: F) -> #crate_ident::SignalHandlerId {
                self.connect_notify_local(::core::option::Option::Some(#stripped_name), move |this, _| {
                    f(this)
                })
            }
        )
    });
    connection_fns.collect::<Vec<_>>()
}

fn expand_impl_notify_prop(wrapper_type: &syn::Path, props: &[PropDesc]) -> Vec<syn::ImplItemFn> {
    let crate_ident = crate_ident_new();
    let emit_fns = props.iter().filter(|p| !p.is_overriding()).map(|p| -> syn::ImplItemFn {
        let name = strip_raw_prefix_from_name(&p.name);
        let fn_ident = format_ident!("notify_{}", name_to_ident(&name));
        let span = p.attrs_span;
        let enum_ident = name_to_enum_ident(name.value());
        let doc = format!("Notify listeners of a change in the `{}` property", name.value());
        parse_quote_spanned!(span=>
            #[doc = #doc]
            #[allow(dead_code)]
            pub fn #fn_ident(&self) {
                self.notify_by_pspec(
                    &<<#wrapper_type as #crate_ident::object::ObjectSubclassIs>::Subclass
                        as #crate_ident::subclass::object::DerivedObjectProperties>::derived_properties()
                    [DerivedPropertiesEnum::#enum_ident as usize]
                );
            }
        )
    });
    emit_fns.collect::<Vec<_>>()
}

fn name_to_enum_ident(name: String) -> syn::Ident {
    let mut name = name.strip_prefix("r#").unwrap_or(&name).to_owned();
    let mut slice = name.as_mut_str();
    while let Some(i) = slice.find('-') {
        let (head, tail) = slice.split_at_mut(i);
        if let Some(c) = head.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        slice = &mut tail[1..];
    }
    if let Some(c) = slice.get_mut(0..1) {
        c.make_ascii_uppercase();
    }
    let enum_member: String = name.split('-').collect();
    format_ident!("{}", enum_member)
}

fn expand_properties_enum(props: &[PropDesc]) -> TokenStream2 {
    if props.is_empty() {
        quote! {
            #[derive(Debug, Copy, Clone)]
            enum DerivedPropertiesEnum {}
            impl std::convert::TryFrom<usize> for DerivedPropertiesEnum {
                type Error = usize;

                fn try_from(item: usize) -> ::core::result::Result<Self, <Self as std::convert::TryFrom<usize>>::Error> {
                    ::core::result::Result::Err(item)
                }
            }
        }
    } else {
        let properties: Vec<syn::Ident> = props
            .iter()
            .map(|p| {
                let name: String = p.name.value();

                name_to_enum_ident(name)
            })
            .collect();
        let props = properties.iter();
        let indices = 0..properties.len();
        quote! {
            #[repr(usize)]
            #[derive(Debug, Copy, Clone)]
            enum DerivedPropertiesEnum {
                #(#props,)*
            }
            impl std::convert::TryFrom<usize> for DerivedPropertiesEnum {
                type Error = usize;

                fn try_from(item: usize) -> ::core::result::Result<Self, <Self as std::convert::TryFrom<usize>>::Error> {
                    match item {
                        #(#indices => ::core::result::Result::Ok(Self::#properties),)*
                        _ => ::core::result::Result::Err(item)
                    }
                }
            }
        }
    }
}

pub fn impl_derive_props(input: PropsMacroInput) -> TokenStream {
    let struct_ident = &input.ident;
    let crate_ident = crate_ident_new();
    let wrapper_type = input.wrapper_ty;
    let fn_properties = expand_properties_fn(&input.props);
    let fn_property = expand_property_fn(&input.props);
    let fn_set_property = expand_set_property_fn(&input.props);
    let getset_properties = expand_impl_getset_properties(&input.props);
    let connect_prop_notify = expand_impl_connect_prop_notify(&input.props);
    let notify_prop = expand_impl_notify_prop(&wrapper_type, &input.props);
    let properties_enum = expand_properties_enum(&input.props);

    let rust_interface = if let Some(ext_trait) = input.ext_trait {
        let trait_ident = if let Some(ext_trait) = ext_trait {
            ext_trait
        } else {
            format_ident!(
                "{}PropertiesExt",
                wrapper_type.segments.last().unwrap().ident
            )
        };
        let fns_without_visibility_modifier = getset_properties
            .into_iter()
            .chain(connect_prop_notify)
            .chain(notify_prop)
            .map(|mut item| {
                item.vis = syn::Visibility::Inherited;
                item
            });
        quote! {
            pub trait #trait_ident: #crate_ident::prelude::IsA<#wrapper_type> {
                #(#fns_without_visibility_modifier)*
            }
            impl<T: #crate_ident::prelude::IsA<#wrapper_type>> #trait_ident for T {}
        }
    } else {
        quote! {
            #[allow(dead_code)]
            impl #wrapper_type {
                #(#getset_properties)*
                #(#connect_prop_notify)*
                #(#notify_prop)*
            }
        }
    };

    let expanded = quote! {
        #properties_enum

        impl #crate_ident::subclass::object::DerivedObjectProperties for #struct_ident {
            #fn_properties
            #fn_property
            #fn_set_property
        }

        #rust_interface
    };
    proc_macro::TokenStream::from(expanded)
}
