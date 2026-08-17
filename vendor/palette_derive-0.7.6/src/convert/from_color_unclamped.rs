use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_quote, DeriveInput, Generics, Ident, Result, Type};

use crate::{
    color_types::{ColorInfo, MetaTypeSource, XYZ_COLORS},
    convert::util::{InputUser, WhitePointSource},
    meta::{
        parse_field_attributes, parse_namespaced_attributes, FieldAttributes, IdentOrIndex,
        TypeItemAttributes,
    },
    util,
};

use super::util::{component_type, find_nearest_color, get_convert_color_type, white_point_type};

pub fn derive(item: TokenStream) -> ::std::result::Result<TokenStream, Vec<::syn::parse::Error>> {
    let DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    } = syn::parse(item).map_err(|error| vec![error])?;

    let (mut item_meta, item_errors) = parse_namespaced_attributes::<TypeItemAttributes>(attrs);

    let (fields_meta, field_errors) = if let syn::Data::Struct(struct_data) = data {
        parse_field_attributes::<FieldAttributes>(struct_data.fields)
    } else {
        return Err(vec![syn::Error::new(
            Span::call_site(),
            "only structs are supported",
        )]);
    };

    let component = component_type(item_meta.component.clone());
    let white_point = white_point_type(
        item_meta.white_point.as_ref(),
        item_meta.rgb_standard.as_ref(),
        item_meta.luma_standard.as_ref(),
        item_meta.internal,
    );

    let alpha_field = fields_meta.alpha_property;

    // Assume conversion from the root type (Xyz for the base group) by default
    if item_meta.color_groups.is_empty() {
        item_meta.color_groups.insert((&XYZ_COLORS).into());
    }

    if item_meta.skip_derives.is_empty() {
        for group in &item_meta.color_groups {
            item_meta.skip_derives.insert(group.root_type.name.into());
        }
    }

    let (all_from_impl_params, impl_params_errors) =
        prepare_from_impl(&component, white_point, &item_meta, &generics);

    let mut implementations =
        generate_from_implementations(&ident, &generics, &item_meta, &all_from_impl_params);

    if let Some((alpha_property, alpha_type)) = alpha_field {
        implementations.push(generate_from_alpha_implementation_with_internal(
            &ident,
            &generics,
            &item_meta,
            &alpha_property,
            &alpha_type,
        ));
    } else {
        implementations.push(generate_from_alpha_implementation(
            &ident, &generics, &item_meta,
        ));
    }

    let item_errors = item_errors
        .into_iter()
        .map(|error| error.into_compile_error());
    let field_errors = field_errors
        .into_iter()
        .map(|error| error.into_compile_error());
    let impl_params_errors = impl_params_errors
        .into_iter()
        .map(|error| error.into_compile_error());

    Ok(quote! {
        #(#item_errors)*
        #(#field_errors)*
        #(#impl_params_errors)*

        #(#implementations)*
    }
    .into())
}

fn prepare_from_impl(
    component: &Type,
    white_point: Option<(Type, WhitePointSource)>,
    meta: &TypeItemAttributes,
    generics: &Generics,
) -> (Vec<FromImplParameters>, Vec<syn::Error>) {
    let included_colors = meta
        .color_groups
        .iter()
        .flat_map(|group| group.color_names())
        .filter(|&color| !meta.skip_derives.contains(color.name));

    let mut parameters = Vec::new();
    let mut errors = Vec::new();

    for color in included_colors {
        let impl_params = prepare_from_impl_for_pair(
            color,
            component,
            white_point.clone(),
            meta,
            generics.clone(),
        );

        match impl_params {
            Ok(Some(impl_params)) => parameters.push(impl_params),
            Ok(None) => {}
            Err(error) => errors.push(error),
        }
    }

    (parameters, errors)
}

fn prepare_from_impl_for_pair(
    color: &ColorInfo,
    component: &Type,
    white_point: Option<(Type, WhitePointSource)>,
    meta: &TypeItemAttributes,
    mut generics: Generics,
) -> Result<Option<FromImplParameters>> {
    let nearest_color = find_nearest_color(color, meta)?;

    // Figures out which white point the target type prefers, unless it's specified in `white_point`.
    let (white_point, white_point_source) = if let Some((white_point, source)) = white_point {
        (white_point, source)
    } else {
        color.get_default_white_point(meta.internal)
    };

    let (color_ty, mut used_input) =
        get_convert_color_type(color, &white_point, component, meta, &mut generics)?;

    let nearest_color_ty = nearest_color.get_type(
        MetaTypeSource::OtherColor(color),
        component,
        &white_point,
        &mut used_input,
        InputUser::Nearest,
        meta,
    )?;

    // Skip implementing the trait where it wouldn't be able to constrain the
    // white point. This is only happening when certain optional features are
    // enabled.
    if used_input.white_point.is_unconstrained()
        && matches!(white_point_source, WhitePointSource::GeneratedGeneric)
    {
        return Ok(None);
    }

    if used_input.white_point.is_used() {
        match white_point_source {
            WhitePointSource::WhitePoint => {
                let white_point_path = util::path(["white_point", "WhitePoint"], meta.internal);
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#white_point: #white_point_path<#component>))
            }
            WhitePointSource::RgbStandard => {
                let rgb_standard_path = util::path(["rgb", "RgbStandard"], meta.internal);
                let rgb_standard = meta.rgb_standard.as_ref();
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#rgb_standard: #rgb_standard_path));
            }
            WhitePointSource::LumaStandard => {
                let luma_standard_path = util::path(["luma", "LumaStandard"], meta.internal);
                let luma_standard = meta.luma_standard.as_ref();
                generics
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#luma_standard: #luma_standard_path));
            }
            WhitePointSource::ConcreteType => {}
            WhitePointSource::GeneratedGeneric => {
                generics.params.push(parse_quote!(_Wp));
            }
        }
    }

    Ok(Some(FromImplParameters {
        generics,
        color_ty,
        nearest_color_ty,
    }))
}

struct FromImplParameters {
    generics: Generics,
    color_ty: Type,
    nearest_color_ty: Type,
}

fn generate_from_implementations(
    ident: &Ident,
    generics: &Generics,
    meta: &TypeItemAttributes,
    all_parameters: &[FromImplParameters],
) -> Vec<TokenStream2> {
    let from_trait_path = util::path(["convert", "FromColorUnclamped"], meta.internal);
    let into_trait_path = util::path(["convert", "IntoColorUnclamped"], meta.internal);

    let (_, type_generics, _) = generics.split_for_impl();

    let mut implementations = Vec::with_capacity(all_parameters.len());

    for parameters in all_parameters {
        let FromImplParameters {
            color_ty,
            generics,
            nearest_color_ty,
        } = parameters;

        {
            let mut generics = generics.clone();

            {
                let where_clause = generics.make_where_clause();
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #from_trait_path<#color_ty>));
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #into_trait_path<Self>));
            }

            let (impl_generics, _, where_clause) = generics.split_for_impl();

            implementations.push(quote! {
                #[automatically_derived]
                impl #impl_generics #from_trait_path<#color_ty> for #ident #type_generics #where_clause {
                    fn from_color_unclamped(color: #color_ty) -> Self {
                        use #from_trait_path;
                        use #into_trait_path;
                        #nearest_color_ty::from_color_unclamped(color).into_color_unclamped()
                    }
                }
            });
        }

        if !meta.internal || meta.internal_not_base_type {
            let mut generics = generics.clone();

            {
                let where_clause = generics.make_where_clause();
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #from_trait_path<#ident #type_generics>));
                where_clause
                    .predicates
                    .push(parse_quote!(#nearest_color_ty: #into_trait_path<Self>));
            }

            let (impl_generics, _, where_clause) = generics.split_for_impl();

            implementations.push(quote! {
                #[automatically_derived]
                impl #impl_generics #from_trait_path<#ident #type_generics> for #color_ty #where_clause {
                    fn from_color_unclamped(color: #ident #type_generics) -> Self {
                        use #from_trait_path;
                        use #into_trait_path;
                        #nearest_color_ty::from_color_unclamped(color).into_color_unclamped()
                    }
                }
            });
        }
    }

    implementations
}

fn generate_from_alpha_implementation(
    ident: &Ident,
    generics: &Generics,
    meta: &TypeItemAttributes,
) -> TokenStream2 {
    let from_trait_path = util::path(["convert", "FromColorUnclamped"], meta.internal);
    let into_trait_path = util::path(["convert", "IntoColorUnclamped"], meta.internal);
    let alpha_path = util::path(["Alpha"], meta.internal);

    let mut impl_generics = generics.clone();
    impl_generics.params.push(parse_quote!(_C));
    impl_generics.params.push(parse_quote!(_A));
    {
        let where_clause = impl_generics.make_where_clause();
        where_clause
            .predicates
            .push(parse_quote!(_C: #into_trait_path<Self>));
    }

    let (_, type_generics, _) = generics.split_for_impl();
    let (self_impl_generics, _, self_where_clause) = impl_generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #self_impl_generics #from_trait_path<#alpha_path<_C, _A>> for #ident #type_generics #self_where_clause {
            fn from_color_unclamped(color: #alpha_path<_C, _A>) -> Self {
                color.color.into_color_unclamped()
            }
        }
    }
}

fn generate_from_alpha_implementation_with_internal(
    ident: &Ident,
    generics: &Generics,
    meta: &TypeItemAttributes,
    alpha_property: &IdentOrIndex,
    alpha_type: &Type,
) -> TokenStream2 {
    let from_trait_path = util::path(["convert", "FromColorUnclamped"], meta.internal);
    let into_trait_path = util::path(["convert", "IntoColorUnclamped"], meta.internal);
    let alpha_path = util::path(["Alpha"], meta.internal);

    let (_, type_generics, _) = generics.split_for_impl();
    let mut impl_generics = generics.clone();
    impl_generics.params.push(parse_quote!(_C));
    {
        let where_clause = impl_generics.make_where_clause();
        where_clause
            .predicates
            .push(parse_quote!(_C: #into_trait_path<Self>));
    }
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #from_trait_path<#alpha_path<_C, #alpha_type>> for #ident #type_generics #where_clause {
            fn from_color_unclamped(color: #alpha_path<_C, #alpha_type>) -> Self {
                use #from_trait_path;
                use #into_trait_path;

                let #alpha_path { color, alpha } = color;

                let mut result: Self = color.into_color_unclamped();
                result.#alpha_property = alpha;

                result
            }
        }
    }
}
