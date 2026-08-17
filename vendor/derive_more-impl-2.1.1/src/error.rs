use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned as _, Error, Result};

use crate::utils::{
    self, AttrParams, DeriveType, FullMetaInfo, HashSet, MetaInfo, MultiFieldData,
    State,
};

pub fn expand(
    input: &syn::DeriveInput,
    trait_name: &'static str,
) -> Result<TokenStream> {
    let syn::DeriveInput {
        ident, generics, ..
    } = input;

    let state = State::with_attr_params(
        input,
        trait_name,
        trait_name.to_lowercase(),
        allowed_attr_params(),
    )?;

    let type_params: HashSet<_> = generics
        .params
        .iter()
        .filter_map(|generic| match generic {
            syn::GenericParam::Type(ty) => Some(ty.ident.clone()),
            _ => None,
        })
        .collect();

    let (bounds, source, provide) = match state.derive_type {
        DeriveType::Named | DeriveType::Unnamed => render_struct(&type_params, &state)?,
        DeriveType::Enum => render_enum(&type_params, &state)?,
    };

    let source = source.map(|source| {
        // Not using `#[inline]` here on purpose, since this is almost never part
        // of a hot codepath.
        quote! {
            fn source(&self) -> Option<&(dyn derive_more::core::error::Error + 'static)> {
                use derive_more::__private::AsDynError as _;
                #source
            }
        }
    });

    let provide = provide.map(|provide| {
        // Not using `#[inline]` here on purpose, since this is almost never part
        // of a hot codepath.
        quote! {
            fn provide<'_request>(
                &'_request self,
                request: &mut derive_more::core::error::Request<'_request>,
            ) {
                #provide
            }
        }
    });

    let mut generics = generics.clone();

    if !type_params.is_empty() {
        let (_, ty_generics, _) = generics.split_for_impl();
        generics = utils::add_extra_where_clauses(
            &generics,
            quote! {
                where
                    #ident #ty_generics: derive_more::core::fmt::Debug
                                         + derive_more::core::fmt::Display
            },
        );
    }

    if !bounds.is_empty() {
        let bounds = bounds.iter();
        generics = utils::add_extra_where_clauses(
            &generics,
            quote! {
                where #(
                    #bounds: derive_more::core::fmt::Debug
                             + derive_more::core::fmt::Display
                             + derive_more::core::error::Error
                             + 'static
                ),*
            },
        );
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let render = quote! {
        #[automatically_derived]
        impl #impl_generics derive_more::core::error::Error for #ident #ty_generics #where_clause {
            #source
            #provide
        }
    };

    Ok(render)
}

fn render_struct(
    type_params: &HashSet<syn::Ident>,
    state: &State,
) -> Result<(HashSet<syn::Type>, Option<TokenStream>, Option<TokenStream>)> {
    let parsed_fields = parse_fields(type_params, state)?;

    let source = parsed_fields.render_source_as_struct();
    let provide = cfg!(error_generic_member_access)
        .then(|| parsed_fields.render_provide_as_struct())
        .flatten();

    Ok((parsed_fields.bounds, source, provide))
}

fn render_enum(
    type_params: &HashSet<syn::Ident>,
    state: &State,
) -> Result<(HashSet<syn::Type>, Option<TokenStream>, Option<TokenStream>)> {
    let mut bounds = HashSet::default();
    let mut source_match_arms = Vec::new();
    let mut provide_match_arms = Vec::new();

    for variant in state.enabled_variant_data().variants {
        let default_info = FullMetaInfo {
            enabled: true,
            ..FullMetaInfo::default()
        };

        let state = State::from_variant(
            state.input,
            state.trait_name,
            state.trait_attr.clone(),
            allowed_attr_params(),
            variant,
            default_info,
        )?;

        let parsed_fields = parse_fields(type_params, &state)?;

        if let Some(expr) = parsed_fields.render_source_as_enum_variant_match_arm() {
            source_match_arms.push(expr);
        }

        if let Some(expr) = parsed_fields.render_provide_as_enum_variant_match_arm() {
            provide_match_arms.push(expr);
        }

        bounds.extend(parsed_fields.bounds.into_iter());
    }

    let render = |match_arms: &mut Vec<TokenStream>, unmatched| {
        if !match_arms.is_empty() && match_arms.len() < state.variants.len() {
            match_arms.push(quote! { _ => #unmatched });
        }

        (!match_arms.is_empty()).then(|| {
            quote! {
                match self {
                    #(#match_arms),*
                }
            }
        })
    };

    let source = render(&mut source_match_arms, quote! { None });
    let provide = render(&mut provide_match_arms, quote! { () });

    Ok((bounds, source, provide))
}

fn allowed_attr_params() -> AttrParams {
    AttrParams {
        enum_: vec!["ignore"],
        struct_: vec!["ignore"],
        variant: vec!["ignore"],
        field: vec!["ignore", "source", "optional", "backtrace"],
    }
}

struct ParsedFields<'input, 'state> {
    data: MultiFieldData<'input, 'state>,
    source: Option<usize>,
    backtrace: Option<usize>,
    bounds: HashSet<syn::Type>,
}

impl<'input, 'state> ParsedFields<'input, 'state> {
    fn new(data: MultiFieldData<'input, 'state>) -> Self {
        Self {
            data,
            source: None,
            backtrace: None,
            bounds: HashSet::default(),
        }
    }
}

impl ParsedFields<'_, '_> {
    fn render_source_as_struct(&self) -> Option<TokenStream> {
        let source = self.source?;
        let ident = &self.data.members[source];
        let is_optional = self.data.infos[source].info.source_optional == Some(true)
            || self.data.field_types[source].is_option();

        Some(render_some(quote! { (&#ident) }, is_optional))
    }

    fn render_source_as_enum_variant_match_arm(&self) -> Option<TokenStream> {
        let source = self.source?;
        let pattern = self.data.matcher(&[source], &[quote! { source }]);
        let is_optional = self.data.infos[source].info.source_optional == Some(true)
            || self.data.field_types[source].is_option();

        let expr = render_some(quote! { source }, is_optional);
        Some(quote! { #pattern => #expr })
    }

    fn render_provide_as_struct(&self) -> Option<TokenStream> {
        let backtrace = self.backtrace?;

        let source_provider = self.source.map(|source| {
            let source_expr = &self.data.members[source];
            quote! {
                derive_more::core::error::Error::provide(&#source_expr, request);
            }
        });
        let backtrace_provider = self
            .source
            .filter(|source| *source == backtrace)
            .is_none()
            .then(|| {
                let backtrace_expr = &self.data.members[backtrace];
                quote! {
                    request.provide_ref::<::std::backtrace::Backtrace>(&#backtrace_expr);
                }
            });

        (source_provider.is_some() || backtrace_provider.is_some()).then(|| {
            quote! {
                #backtrace_provider
                #source_provider
            }
        })
    }

    fn render_provide_as_enum_variant_match_arm(&self) -> Option<TokenStream> {
        let backtrace = self.backtrace?;

        match self.source {
            Some(source) if source == backtrace => {
                let pattern = self.data.matcher(&[source], &[quote! { source }]);
                Some(quote! {
                    #pattern => {
                        derive_more::core::error::Error::provide(source, request);
                    }
                })
            }
            Some(source) => {
                let pattern = self.data.matcher(
                    &[source, backtrace],
                    &[quote! { source }, quote! { backtrace }],
                );
                Some(quote! {
                    #pattern => {
                        request.provide_ref::<::std::backtrace::Backtrace>(backtrace);
                        derive_more::core::error::Error::provide(source, request);
                    }
                })
            }
            None => {
                let pattern = self.data.matcher(&[backtrace], &[quote! { backtrace }]);
                Some(quote! {
                    #pattern => {
                        request.provide_ref::<::std::backtrace::Backtrace>(backtrace);
                    }
                })
            }
        }
    }
}

fn render_some(mut expr: TokenStream, unpack: bool) -> TokenStream {
    if unpack {
        expr = quote! { derive_more::core::option::Option::as_ref(#expr)? }
    }
    quote! { Some(#expr.__derive_more_as_dyn_error()) }
}

fn parse_fields<'input, 'state>(
    type_params: &HashSet<syn::Ident>,
    state: &'state State<'input>,
) -> Result<ParsedFields<'input, 'state>> {
    let mut parsed_fields = match state.derive_type {
        DeriveType::Named => {
            parse_fields_impl(state, |attr, field, _| {
                // Unwrapping is safe, cause fields in named struct
                // always have an ident
                let ident = field.ident.as_ref().unwrap();

                match attr {
                    "source" => ident == "source",
                    "backtrace" => {
                        ident == "backtrace"
                            || is_type_path_ends_with_segment(&field.ty, "Backtrace")
                    }
                    _ => unreachable!(),
                }
            })
        }

        DeriveType::Unnamed => {
            let mut parsed_fields =
                parse_fields_impl(state, |attr, field, len| match attr {
                    "source" => {
                        len == 1
                            && !is_type_path_ends_with_segment(&field.ty, "Backtrace")
                    }
                    "backtrace" => {
                        is_type_path_ends_with_segment(&field.ty, "Backtrace")
                    }
                    _ => unreachable!(),
                })?;

            parsed_fields.source = parsed_fields
                .source
                .or_else(|| infer_source_field(&state.fields, &parsed_fields));

            Ok(parsed_fields)
        }

        _ => unreachable!(),
    }?;

    if let Some(source) = parsed_fields.source {
        let is_optional = parsed_fields.data.infos[source].info.source_optional
            == Some(true)
            || state.fields[source].ty.is_option();

        add_bound_if_type_parameter_used_in_type(
            &mut parsed_fields.bounds,
            type_params,
            &state.fields[source].ty,
            is_optional,
        );
    }

    Ok(parsed_fields)
}

/// Checks if `ty` is [`syn::Type::Path`] and ends with segment matching `tail`
/// and doesn't contain any generic parameters.
fn is_type_path_ends_with_segment(ty: &syn::Type, tail: &str) -> bool {
    let syn::Type::Path(ty) = ty else {
        return false;
    };

    // Unwrapping is safe, cause 'syn::TypePath.path.segments'
    // have to have at least one segment
    let segment = ty.path.segments.last().unwrap();

    if !matches!(segment.arguments, syn::PathArguments::None) {
        return false;
    }

    segment.ident == tail
}

fn infer_source_field(
    fields: &[&syn::Field],
    parsed_fields: &ParsedFields,
) -> Option<usize> {
    // if we have exactly two fields
    if fields.len() != 2 {
        return None;
    }

    // no source field was specified/inferred
    if parsed_fields.source.is_some() {
        return None;
    }

    // but one of the fields was specified/inferred as backtrace field
    if let Some(backtrace) = parsed_fields.backtrace {
        // then infer *other field* as source field
        let source = (backtrace + 1) % 2;
        // unless it was explicitly marked as non-source
        if parsed_fields.data.infos[source].info.source != Some(false) {
            return Some(source);
        }
    }

    None
}

fn parse_fields_impl<'input, 'state, P>(
    state: &'state State<'input>,
    is_valid_default_field_for_attr: P,
) -> Result<ParsedFields<'input, 'state>>
where
    P: Fn(&str, &syn::Field, usize) -> bool,
{
    let MultiFieldData { fields, infos, .. } = state.enabled_fields_data();

    let iter = fields
        .iter()
        .zip(infos.iter().map(|info| &info.info))
        .enumerate()
        .map(|(index, (field, info))| (index, *field, info));

    let source = parse_field_impl(
        &is_valid_default_field_for_attr,
        state.fields.len(),
        iter.clone(),
        "source",
        |info| info.source,
    )?;

    let backtrace = parse_field_impl(
        &is_valid_default_field_for_attr,
        state.fields.len(),
        iter.clone(),
        "backtrace",
        |info| info.backtrace,
    )?;

    let mut parsed_fields = ParsedFields::new(state.enabled_fields_data());

    if let Some((index, _, _)) = source {
        parsed_fields.source = Some(index);
    }

    if let Some((index, _, _)) = backtrace {
        parsed_fields.backtrace = Some(index);
    }

    Ok(parsed_fields)
}

fn parse_field_impl<'a, P, V>(
    is_valid_default_field_for_attr: &P,
    len: usize,
    iter: impl Iterator<Item = (usize, &'a syn::Field, &'a MetaInfo)> + Clone,
    attr: &str,
    value: V,
) -> Result<Option<(usize, &'a syn::Field, &'a MetaInfo)>>
where
    P: Fn(&str, &syn::Field, usize) -> bool,
    V: Fn(&MetaInfo) -> Option<bool>,
{
    let explicit_fields = iter
        .clone()
        .filter(|(_, _, info)| matches!(value(info), Some(true)));

    let inferred_fields = iter.filter(|(_, field, info)| match value(info) {
        None => is_valid_default_field_for_attr(attr, field, len),
        _ => false,
    });

    let field = assert_iter_contains_zero_or_one_item(
        explicit_fields,
        &format!(
            "Multiple `{attr}` attributes specified. \
             Single attribute per struct/enum variant allowed.",
        ),
    )?;

    let field = match field {
        field @ Some(_) => field,
        None => assert_iter_contains_zero_or_one_item(
            inferred_fields,
            "Conflicting fields found. Consider specifying some \
             `#[error(...)]` attributes to resolve conflict.",
        )?,
    };

    Ok(field)
}

fn assert_iter_contains_zero_or_one_item<'a>(
    mut iter: impl Iterator<Item = (usize, &'a syn::Field, &'a MetaInfo)>,
    error_msg: &str,
) -> Result<Option<(usize, &'a syn::Field, &'a MetaInfo)>> {
    let Some(item) = iter.next() else {
        return Ok(None);
    };

    if let Some((_, field, _)) = iter.next() {
        return Err(Error::new(field.span(), error_msg));
    }

    Ok(Some(item))
}

fn add_bound_if_type_parameter_used_in_type(
    bounds: &mut HashSet<syn::Type>,
    type_params: &HashSet<syn::Ident>,
    ty: &syn::Type,
    unpack: bool,
) {
    if let Some(ty) = utils::get_if_type_parameter_used_in_type(type_params, ty) {
        bounds.insert(
            unpack
                .then(|| ty.get_inner())
                .flatten()
                .cloned()
                .unwrap_or(ty),
        );
    }
}

/// Extension of a [`syn::Type`] used by this expansion.
trait TypeExt {
    /// Checks syntactically whether this [`syn::Type`] represents an [`Option`].
    fn is_option(&self) -> bool;

    /// Returns the inner [`syn::Type`] if this one represents a wrapper.
    ///
    /// `filter` filters out this [`syn::Type`] by its name.
    fn get_inner_if(&self, filter: impl Fn(&syn::Ident) -> bool) -> Option<&Self>;

    /// Returns the inner [`syn::Type`] if this one represents a wrapper.
    fn get_inner(&self) -> Option<&Self> {
        self.get_inner_if(|_| true)
    }

    /// Returns the inner [`syn::Type`] if this one represents an [`Option`].
    fn get_option_inner(&self) -> Option<&Self> {
        self.get_inner_if(|ident| ident == "Option")
    }
}

impl TypeExt for syn::Type {
    fn is_option(&self) -> bool {
        self.get_option_inner().is_some()
    }

    fn get_inner_if(&self, filter: impl Fn(&syn::Ident) -> bool) -> Option<&Self> {
        match self {
            Self::Group(g) => g.elem.get_option_inner(),
            Self::Paren(p) => p.elem.get_option_inner(),
            Self::Path(p) => p
                .path
                .segments
                .last()
                .filter(|s| filter(&s.ident))
                .and_then(|s| {
                    if let syn::PathArguments::AngleBracketed(a) = &s.arguments {
                        if let Some(syn::GenericArgument::Type(ty)) = a.args.first() {
                            return Some(ty);
                        }
                    }
                    None
                }),
            _ => None,
        }
    }
}
