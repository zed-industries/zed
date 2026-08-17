use std::collections::BTreeSet;

pub(crate) use self::context_module::ContextModule;
pub(crate) use self::context_selector::ContextSelector;
pub(crate) use self::display::{Display, DisplayMatchArm};
pub(crate) use self::error::{Error, ErrorProvideMatchArm, ErrorSourceMatchArm};
pub(crate) use self::error_compat::{ErrorCompat, ErrorCompatBacktraceMatchArm};

pub(crate) struct StaticIdent(&'static str);

impl quote::ToTokens for StaticIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        proc_macro2::Ident::new(self.0, proc_macro2::Span::call_site()).to_tokens(tokens)
    }
}

struct AllFieldNames<'a>(&'a crate::FieldContainer);

impl<'a> AllFieldNames<'a> {
    fn field_names(&self) -> BTreeSet<&'a proc_macro2::Ident> {
        let user_fields = self.0.selector_kind.user_fields();
        let backtrace_field = self.0.backtrace_field.as_ref();
        let implicit_fields = &self.0.implicit_fields;
        let message_field = self.0.selector_kind.message_field();
        let source_field = self.0.selector_kind.source_field();

        user_fields
            .iter()
            .chain(backtrace_field)
            .chain(implicit_fields)
            .chain(message_field)
            .map(crate::Field::name)
            .chain(source_field.map(crate::SourceField::name))
            .collect()
    }
}

pub mod context_module {
    use crate::ModuleName;
    use heck::ToSnakeCase;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use syn::Ident;

    #[derive(Copy, Clone)]
    pub(crate) struct ContextModule<'a, T> {
        pub container_name: &'a Ident,
        pub module_name: &'a ModuleName,
        pub visibility: Option<&'a dyn ToTokens>,
        pub body: &'a T,
    }

    impl<'a, T> ToTokens for ContextModule<'a, T>
    where
        T: ToTokens,
    {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let module_name = match self.module_name {
                ModuleName::Default => {
                    let name_str = self.container_name.to_string().to_snake_case();
                    syn::Ident::new(&name_str, self.container_name.span())
                }
                ModuleName::Custom(name) => name.clone(),
            };

            let visibility = self.visibility;
            let body = self.body;

            let module_tokens = quote! {
                #visibility mod #module_name {
                    use super::*;

                    #body
                }
            };

            stream.extend(module_tokens);
        }
    }
}

pub mod context_selector {
    use crate::{ContextSelectorKind, Field, SuffixKind};
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, ToTokens};

    #[derive(Copy, Clone)]
    pub(crate) struct ContextSelector<'a> {
        pub backtrace_field: Option<&'a Field>,
        pub implicit_fields: &'a [Field],
        pub crate_root: &'a dyn ToTokens,
        pub error_constructor_name: &'a dyn ToTokens,
        pub original_generics_without_defaults: &'a [TokenStream],
        pub parameterized_error_name: &'a dyn ToTokens,
        pub selector_doc_string: &'a str,
        pub selector_kind: &'a ContextSelectorKind,
        pub selector_base_name: &'a proc_macro2::Ident,
        pub user_fields: &'a [Field],
        pub visibility: Option<&'a dyn ToTokens>,
        pub where_clauses: &'a [TokenStream],
        pub default_suffix: &'a SuffixKind,
    }

    impl ToTokens for ContextSelector<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            use self::ContextSelectorKind::*;

            let context_selector = match self.selector_kind {
                Context { source_field, .. } => {
                    let context_selector_type = self.generate_type();
                    let context_selector_impl = match source_field {
                        Some(_) => None,
                        None => Some(self.generate_leaf()),
                    };
                    let context_selector_into_error_impl =
                        self.generate_into_error(source_field.as_ref());

                    quote! {
                        #context_selector_type
                        #context_selector_impl
                        #context_selector_into_error_impl
                    }
                }
                Whatever {
                    source_field,
                    message_field,
                } => self.generate_whatever(source_field.as_ref(), message_field),
                NoContext { source_field } => self.generate_from_source(source_field),
            };

            stream.extend(context_selector)
        }
    }

    impl ContextSelector<'_> {
        fn user_field_generics(&self) -> Vec<proc_macro2::Ident> {
            (0..self.user_fields.len())
                .map(|i| format_ident!("__T{}", i))
                .collect()
        }

        fn user_field_names(&self) -> Vec<&syn::Ident> {
            self.user_fields
                .iter()
                .map(|Field { name, .. }| name)
                .collect()
        }

        fn parameterized_selector_name(&self) -> TokenStream {
            let selector_name = self
                .selector_kind
                .resolve_name(self.default_suffix, self.selector_base_name);
            let user_generics = self.user_field_generics();

            quote! { #selector_name<#(#user_generics,)*> }
        }

        fn extended_where_clauses(&self) -> Vec<TokenStream> {
            let user_fields = self.user_fields;
            let user_field_generics = self.user_field_generics();
            let where_clauses = self.where_clauses;

            let target_types = user_fields
                .iter()
                .map(|Field { ty, .. }| quote! { ::core::convert::Into<#ty>});

            user_field_generics
                .into_iter()
                .zip(target_types)
                .map(|(gen, bound)| quote! { #gen: #bound })
                .chain(where_clauses.iter().cloned())
                .collect()
        }

        fn transfer_user_fields(&self) -> Vec<TokenStream> {
            self.user_field_names()
                .into_iter()
                .map(|name| {
                    quote! { #name: ::core::convert::Into::into(self.#name) }
                })
                .collect()
        }

        fn construct_implicit_fields(&self) -> TokenStream {
            let crate_root = self.crate_root;
            let expression = quote! {
                #crate_root::GenerateImplicitData::generate()
            };

            self.construct_implicit_fields_with_expression(expression)
        }

        fn construct_implicit_fields_with_source(&self) -> TokenStream {
            let crate_root = self.crate_root;
            let expression = quote! { {
                use #crate_root::AsErrorSource;
                let error = error.as_error_source();
                #crate_root::GenerateImplicitData::generate_with_source(error)
            } };

            self.construct_implicit_fields_with_expression(expression)
        }

        fn construct_implicit_fields_with_expression(
            &self,
            expression: TokenStream,
        ) -> TokenStream {
            self.implicit_fields
                .iter()
                .chain(self.backtrace_field)
                .map(|field| {
                    let name = &field.name;
                    quote! { #name: #expression, }
                })
                .collect()
        }

        fn generate_type(self) -> TokenStream {
            let visibility = self.visibility;
            let parameterized_selector_name = self.parameterized_selector_name();
            let user_field_generics = self.user_field_generics();
            let user_field_names = self.user_field_names();
            let selector_doc_string = self.selector_doc_string;

            let body = if user_field_names.is_empty() {
                quote! { ; }
            } else {
                quote! {
                    {
                        #(
                            #[allow(missing_docs)]
                            #visibility #user_field_names: #user_field_generics
                        ),*
                    }
                }
            };

            quote! {
                #[derive(Debug, Copy, Clone)]
                #[doc = #selector_doc_string]
                #visibility struct #parameterized_selector_name #body
            }
        }

        fn generate_leaf(self) -> TokenStream {
            let error_constructor_name = self.error_constructor_name;
            let original_generics_without_defaults = self.original_generics_without_defaults;
            let parameterized_error_name = self.parameterized_error_name;
            let parameterized_selector_name = self.parameterized_selector_name();
            let user_field_generics = self.user_field_generics();
            let visibility = self.visibility;
            let extended_where_clauses = self.extended_where_clauses();
            let transfer_user_fields = self.transfer_user_fields();
            let construct_implicit_fields = self.construct_implicit_fields();

            quote! {
                impl<#(#user_field_generics,)*> #parameterized_selector_name {
                    #[doc = "Consume the selector and return the associated error"]
                    #[must_use]
                    #[track_caller]
                    #visibility fn build<#(#original_generics_without_defaults,)*>(self) -> #parameterized_error_name
                    where
                        #(#extended_where_clauses),*
                    {
                        #error_constructor_name {
                            #construct_implicit_fields
                            #(#transfer_user_fields,)*
                        }
                    }

                    #[doc = "Consume the selector and return a `Result` with the associated error"]
                    #[allow(dead_code)]
                    #[track_caller]
                    #visibility fn fail<#(#original_generics_without_defaults,)* __T>(self) -> ::core::result::Result<__T, #parameterized_error_name>
                    where
                        #(#extended_where_clauses),*
                    {
                        ::core::result::Result::Err(self.build())
                    }
                }
            }
        }

        fn generate_into_error(self, source_field: Option<&crate::SourceField>) -> TokenStream {
            let crate_root = self.crate_root;
            let error_constructor_name = self.error_constructor_name;
            let original_generics_without_defaults = self.original_generics_without_defaults;
            let parameterized_error_name = self.parameterized_error_name;
            let parameterized_selector_name = self.parameterized_selector_name();
            let user_field_generics = self.user_field_generics();
            let extended_where_clauses = self.extended_where_clauses();
            let transfer_user_fields = self.transfer_user_fields();
            let construct_implicit_fields = if source_field.is_some() {
                self.construct_implicit_fields_with_source()
            } else {
                self.construct_implicit_fields()
            };

            let (source_ty, transform_source, transfer_source_field) = match source_field {
                Some(source_field) => {
                    let SourceInfo {
                        source_field_type,
                        transform_source,
                        transfer_source_field,
                    } = build_source_info(source_field);
                    (
                        quote! { #source_field_type },
                        Some(transform_source),
                        Some(transfer_source_field),
                    )
                }
                None => (quote! { #crate_root::NoneError }, None, None),
            };

            quote! {
                impl<#(#original_generics_without_defaults,)* #(#user_field_generics,)*> #crate_root::IntoError<#parameterized_error_name> for #parameterized_selector_name
                where
                    #parameterized_error_name: #crate_root::Error + #crate_root::ErrorCompat,
                    #(#extended_where_clauses),*
                {
                    type Source = #source_ty;

                    #[track_caller]
                    fn into_error(self, error: Self::Source) -> #parameterized_error_name {
                        #transform_source;
                        #error_constructor_name {
                            #construct_implicit_fields
                            #transfer_source_field
                            #(#transfer_user_fields),*
                        }
                    }
                }
            }
        }

        fn generate_whatever(
            self,
            source_field: Option<&crate::SourceField>,
            message_field: &crate::Field,
        ) -> TokenStream {
            let crate_root = self.crate_root;
            let parameterized_error_name = self.parameterized_error_name;
            let error_constructor_name = self.error_constructor_name;
            let construct_implicit_fields = self.construct_implicit_fields();
            let original_generics_without_defaults = self.original_generics_without_defaults;
            let construct_implicit_fields_with_source =
                self.construct_implicit_fields_with_source();
            let extended_where_clauses = self.extended_where_clauses();

            // testme: transform

            let (source_ty, transfer_source_field, empty_source_field) = match source_field {
                Some(f) => {
                    let source_field_type = f.transformation.source_ty();
                    let source_field_name = &f.name;
                    let source_transformation = f.transformation.transformation();

                    (
                        quote! { #source_field_type },
                        Some(quote! { #source_field_name: (#source_transformation)(error), }),
                        Some(quote! { #source_field_name: core::option::Option::None, }),
                    )
                }
                None => (quote! { #crate_root::NoneError }, None, None),
            };

            let message_field_name = &message_field.name;

            quote! {
                impl<#(#original_generics_without_defaults,)*> #crate_root::FromString for #parameterized_error_name
                where
                    #(#extended_where_clauses),*
                {
                    type Source = #source_ty;

                    #[track_caller]
                    fn without_source(message: String) -> Self {
                        #error_constructor_name {
                            #construct_implicit_fields
                            #empty_source_field
                            #message_field_name: message,
                        }
                    }

                    #[track_caller]
                    fn with_source(error: Self::Source, message: String) -> Self {
                        #error_constructor_name {
                            #construct_implicit_fields_with_source
                            #transfer_source_field
                            #message_field_name: message,
                        }
                    }
                }
            }
        }

        fn generate_from_source(self, source_field: &crate::SourceField) -> TokenStream {
            let parameterized_error_name = self.parameterized_error_name;
            let error_constructor_name = self.error_constructor_name;
            let construct_implicit_fields_with_source =
                self.construct_implicit_fields_with_source();
            let original_generics_without_defaults = self.original_generics_without_defaults;
            let user_field_generics = self.user_field_generics();
            let where_clauses = self.where_clauses;

            let SourceInfo {
                source_field_type,
                transform_source,
                transfer_source_field,
            } = build_source_info(source_field);

            quote! {
                impl<#(#original_generics_without_defaults,)* #(#user_field_generics,)*> ::core::convert::From<#source_field_type> for #parameterized_error_name
                where
                    #(#where_clauses),*
                {
                    #[track_caller]
                    fn from(error: #source_field_type) -> Self {
                        #transform_source;
                        #error_constructor_name {
                            #construct_implicit_fields_with_source
                            #transfer_source_field
                        }
                    }
                }
            }
        }
    }

    struct SourceInfo<'a> {
        source_field_type: &'a syn::Type,
        transform_source: TokenStream,
        transfer_source_field: TokenStream,
    }

    // Assumes that the error is in a variable called "error"
    fn build_source_info(source_field: &crate::SourceField) -> SourceInfo<'_> {
        let source_field_name = source_field.name();
        let source_field_type = source_field.transformation.source_ty();
        let target_field_type = source_field.transformation.target_ty();
        let source_transformation = source_field.transformation.transformation();

        let transform_source =
            quote! { let error: #target_field_type = (#source_transformation)(error) };
        let transfer_source_field = quote! { #source_field_name: error, };

        SourceInfo {
            source_field_type,
            transform_source,
            transfer_source_field,
        }
    }
}

pub mod display {
    use super::StaticIdent;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use std::collections::BTreeSet;

    const FORMATTER_ARG: StaticIdent = StaticIdent("__snafu_display_formatter");

    pub(crate) struct Display<'a> {
        pub(crate) arms: &'a [TokenStream],
        pub(crate) original_generics: &'a [TokenStream],
        pub(crate) parameterized_error_name: &'a dyn ToTokens,
        pub(crate) where_clauses: &'a [TokenStream],
    }

    impl ToTokens for Display<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let Self {
                arms,
                original_generics,
                parameterized_error_name,
                where_clauses,
            } = *self;

            let display_impl = quote! {
                #[allow(single_use_lifetimes)]
                impl<#(#original_generics),*> ::core::fmt::Display for #parameterized_error_name
                where
                    #(#where_clauses),*
                {
                    fn fmt(&self, #FORMATTER_ARG: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        #[allow(unused_variables)]
                        match *self {
                            #(#arms),*
                        }
                    }
                }
            };

            stream.extend(display_impl);
        }
    }

    pub(crate) struct DisplayMatchArm<'a> {
        pub(crate) field_container: &'a crate::FieldContainer,
        pub(crate) default_name: &'a dyn ToTokens,
        pub(crate) display_format: Option<&'a crate::Display>,
        pub(crate) doc_comment: Option<&'a crate::DocComment>,
        pub(crate) pattern_ident: &'a dyn ToTokens,
        pub(crate) selector_kind: &'a crate::ContextSelectorKind,
    }

    impl ToTokens for DisplayMatchArm<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let Self {
                field_container,
                default_name,
                display_format,
                doc_comment,
                pattern_ident,
                selector_kind,
            } = *self;

            let source_field = selector_kind.source_field();

            if field_container.is_transparent {
                // transparent errors always have a source field
                let source_field_name = source_field.unwrap().name();

                let match_arm = quote! {
                    #pattern_ident { ref #source_field_name, .. } => {
                        ::core::fmt::Display::fmt(#source_field_name, #FORMATTER_ARG)
                    }
                };

                stream.extend(match_arm);
                return;
            }

            let mut shorthand_names = &BTreeSet::new();
            let mut assigned_names = &BTreeSet::new();

            let format = match (display_format, doc_comment) {
                (Some(v), _) => {
                    let exprs = &v.exprs;
                    shorthand_names = &v.shorthand_names;
                    assigned_names = &v.assigned_names;
                    quote! { #(#exprs),* }
                }
                (_, Some(d)) => {
                    let content = &d.content;
                    shorthand_names = &d.shorthand_names;
                    quote! { #content }
                }
                _ => quote! { stringify!(#default_name) },
            };

            let field_names = super::AllFieldNames(field_container).field_names();

            let shorthand_names = shorthand_names.iter().collect::<BTreeSet<_>>();
            let assigned_names = assigned_names.iter().collect::<BTreeSet<_>>();

            let shorthand_fields = &shorthand_names & &field_names;
            let shorthand_fields = &shorthand_fields - &assigned_names;

            let shorthand_assignments = quote! { #( #shorthand_fields = #shorthand_fields ),* };

            let match_arm = quote! {
                #pattern_ident { #(ref #field_names),* } => {
                    write!(#FORMATTER_ARG, #format, #shorthand_assignments)
                }
            };

            stream.extend(match_arm);
        }
    }
}

pub mod error {
    use super::StaticIdent;
    use crate::{FieldContainer, Provide, SourceField};
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, ToTokens};

    pub(crate) const PROVIDE_ARG: StaticIdent = StaticIdent("__snafu_provide_demand");

    pub(crate) struct Error<'a> {
        pub(crate) crate_root: &'a dyn ToTokens,
        pub(crate) description_arms: &'a [TokenStream],
        pub(crate) original_generics: &'a [TokenStream],
        pub(crate) parameterized_error_name: &'a dyn ToTokens,
        pub(crate) provide_arms: &'a [TokenStream],
        pub(crate) source_arms: &'a [TokenStream],
        pub(crate) where_clauses: &'a [TokenStream],
    }

    impl ToTokens for Error<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let Self {
                crate_root,
                description_arms,
                original_generics,
                parameterized_error_name,
                provide_arms,
                source_arms,
                where_clauses,
            } = *self;

            let description_fn = quote! {
                fn description(&self) -> &str {
                    match *self {
                        #(#description_arms)*
                    }
                }
            };

            let source_body = quote! {
                use #crate_root::AsErrorSource;
                match *self {
                    #(#source_arms)*
                }
            };

            let cause_fn = quote! {
                fn cause(&self) -> ::core::option::Option<&dyn #crate_root::Error> {
                    #source_body
                }
            };

            let source_fn = quote! {
                fn source(&self) -> ::core::option::Option<&(dyn #crate_root::Error + 'static)> {
                    #source_body
                }
            };

            let provide_fn = if cfg!(feature = "unstable-provider-api") {
                Some(quote! {
                    fn provide<'a>(&'a self, #PROVIDE_ARG: &mut #crate_root::error::Request<'a>) {
                        match *self {
                            #(#provide_arms,)*
                        };
                    }
                })
            } else {
                None
            };

            let error = quote! {
                #[allow(single_use_lifetimes)]
                impl<#(#original_generics),*> #crate_root::Error for #parameterized_error_name
                where
                    Self: ::core::fmt::Debug + ::core::fmt::Display,
                    #(#where_clauses),*
                {
                    #description_fn
                    #cause_fn
                    #source_fn
                    #provide_fn
                }
            };

            stream.extend(error);
        }
    }

    pub(crate) struct ErrorSourceMatchArm<'a> {
        pub(crate) field_container: &'a FieldContainer,
        pub(crate) pattern_ident: &'a dyn ToTokens,
    }

    impl ToTokens for ErrorSourceMatchArm<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let Self {
                field_container:
                    FieldContainer {
                        selector_kind,
                        is_transparent,
                        ..
                    },
                pattern_ident,
            } = *self;

            let source_field = selector_kind.source_field();

            let arm = match source_field {
                Some(source_field) => {
                    let SourceField {
                        name: field_name, ..
                    } = source_field;

                    let convert_to_error_source = if selector_kind.is_whatever() {
                        quote! {
                            #field_name.as_ref().map(|e| e.as_error_source())
                        }
                    } else if *is_transparent {
                        quote! {
                            #field_name.as_error_source().source()
                        }
                    } else {
                        quote! {
                            ::core::option::Option::Some(#field_name.as_error_source())
                        }
                    };

                    quote! {
                        #pattern_ident { ref #field_name, .. } => {
                            #convert_to_error_source
                        }
                    }
                }
                None => {
                    quote! {
                        #pattern_ident { .. } => { ::core::option::Option::None }
                    }
                }
            };

            stream.extend(arm);
        }
    }

    pub(crate) struct ProvidePlus<'a> {
        provide: &'a Provide,
        cached_name: proc_macro2::Ident,
    }

    pub(crate) struct ErrorProvideMatchArm<'a> {
        pub(crate) crate_root: &'a dyn ToTokens,
        pub(crate) field_container: &'a FieldContainer,
        pub(crate) pattern_ident: &'a dyn ToTokens,
    }

    impl<'a> ToTokens for ErrorProvideMatchArm<'a> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let Self {
                crate_root,
                field_container,
                pattern_ident,
            } = *self;

            let user_fields = field_container.user_fields();
            let provides = enhance_provider_list(field_container.provides());
            let field_names = super::AllFieldNames(field_container).field_names();

            let (hi_explicit_calls, lo_explicit_calls) = build_explicit_provide_calls(&provides);

            let cached_expressions = quote_cached_expressions(&provides);

            let provide_refs = user_fields
                .iter()
                .chain(&field_container.implicit_fields)
                .chain(field_container.selector_kind.message_field())
                .flat_map(|f| {
                    if f.provide {
                        Some((&f.ty, f.name()))
                    } else {
                        None
                    }
                });

            let provided_source = field_container
                .selector_kind
                .source_field()
                .filter(|f| f.provide);

            let source_provide_ref =
                provided_source.map(|f| (f.transformation.source_ty(), f.name()));

            let provide_refs = provide_refs.chain(source_provide_ref);

            let source_chain = provided_source.map(|f| {
                let name = f.name();
                quote! {
                    #name.provide(#PROVIDE_ARG);
                }
            });

            let user_chained = quote_chained(crate_root, &provides);

            let shorthand_calls = provide_refs.map(|(ty, name)| {
                quote! { #PROVIDE_ARG.provide_ref::<#ty>(#name) }
            });

            let provided_backtrace = field_container
                .backtrace_field
                .as_ref()
                .filter(|f| f.provide);

            let provide_backtrace = provided_backtrace.map(|f| {
                let name = f.name();
                quote! {
                    if #PROVIDE_ARG.would_be_satisfied_by_ref_of::<#crate_root::Backtrace>() {
                        if let ::core::option::Option::Some(bt) = #crate_root::AsBacktrace::as_backtrace(#name) {
                            #PROVIDE_ARG.provide_ref::<#crate_root::Backtrace>(bt);
                        }
                    }
                }
            });

            let arm = quote! {
                #pattern_ident { #(ref #field_names,)* .. } => {
                    #(#cached_expressions;)*
                    #(#hi_explicit_calls;)*
                    #source_chain;
                    #(#user_chained;)*
                    #provide_backtrace;
                    #(#shorthand_calls;)*
                    #(#lo_explicit_calls;)*
                }
            };

            stream.extend(arm);
        }
    }

    pub(crate) fn enhance_provider_list(provides: &[Provide]) -> Vec<ProvidePlus<'_>> {
        provides
            .iter()
            .enumerate()
            .map(|(i, provide)| {
                let cached_name = format_ident!("__snafu_cached_expr_{}", i);
                ProvidePlus {
                    provide,
                    cached_name,
                }
            })
            .collect()
    }

    pub(crate) fn quote_cached_expressions<'a>(
        provides: &'a [ProvidePlus<'a>],
    ) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
        provides.iter().filter(|pp| pp.provide.is_chain).map(|pp| {
            let cached_name = &pp.cached_name;
            let expr = &pp.provide.expr;

            quote! {
                let #cached_name = #expr;
            }
        })
    }

    pub(crate) fn quote_chained<'a>(
        crate_root: &'a dyn ToTokens,
        provides: &'a [ProvidePlus<'a>],
    ) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
        provides
            .iter()
            .filter(|pp| pp.provide.is_chain)
            .map(move |pp| {
                let arm = if pp.provide.is_opt {
                    quote! { ::core::option::Option::Some(chained_item) }
                } else {
                    quote! { chained_item }
                };
                let cached_name = &pp.cached_name;

                quote! {
                    if let #arm = #cached_name {
                        #crate_root::Error::provide(chained_item, #PROVIDE_ARG);
                    }
                }
            })
    }

    fn quote_provides<'a, I>(provides: I) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a
    where
        I: IntoIterator<Item = &'a ProvidePlus<'a>>,
        I::IntoIter: 'a,
    {
        provides.into_iter().map(|pp| {
            let ProvidePlus {
                provide:
                    Provide {
                        is_chain,
                        is_opt,
                        is_priority: _,
                        is_ref,
                        ty,
                        expr,
                    },
                cached_name,
            } = pp;

            let effective_expr = if *is_chain {
                quote! { #cached_name }
            } else {
                quote! { #expr }
            };

            match (is_opt, is_ref) {
                (true, true) => {
                    quote! {
                        if #PROVIDE_ARG.would_be_satisfied_by_ref_of::<#ty>() {
                            if let ::core::option::Option::Some(v) = #effective_expr {
                                #PROVIDE_ARG.provide_ref::<#ty>(v);
                            }
                        }
                    }
                }
                (true, false) => {
                    quote! {
                        if #PROVIDE_ARG.would_be_satisfied_by_value_of::<#ty>() {
                            if let ::core::option::Option::Some(v) = #effective_expr {
                                #PROVIDE_ARG.provide_value::<#ty>(v);
                            }
                        }
                    }
                }
                (false, true) => {
                    quote! { #PROVIDE_ARG.provide_ref_with::<#ty>(|| #effective_expr) }
                }
                (false, false) => {
                    quote! { #PROVIDE_ARG.provide_value_with::<#ty>(|| #effective_expr) }
                }
            }
        })
    }

    pub(crate) fn build_explicit_provide_calls<'a>(
        provides: &'a [ProvidePlus<'a>],
    ) -> (
        impl Iterator<Item = TokenStream> + 'a,
        impl Iterator<Item = TokenStream> + 'a,
    ) {
        let (high_priority, low_priority): (Vec<_>, Vec<_>) =
            provides.iter().partition(|pp| pp.provide.is_priority);

        let hi_explicit_calls = quote_provides(high_priority);
        let lo_explicit_calls = quote_provides(low_priority);

        (hi_explicit_calls, lo_explicit_calls)
    }
}

pub mod error_compat {
    use crate::{Field, FieldContainer, SourceField};
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};

    pub(crate) struct ErrorCompat<'a> {
        pub(crate) crate_root: &'a dyn ToTokens,
        pub(crate) parameterized_error_name: &'a dyn ToTokens,
        pub(crate) backtrace_arms: &'a [TokenStream],
        pub(crate) original_generics: &'a [TokenStream],
        pub(crate) where_clauses: &'a [TokenStream],
    }

    impl ToTokens for ErrorCompat<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let Self {
                crate_root,
                parameterized_error_name,
                backtrace_arms,
                original_generics,
                where_clauses,
            } = *self;

            let backtrace_fn = quote! {
                fn backtrace(&self) -> ::core::option::Option<&#crate_root::Backtrace> {
                    match *self {
                        #(#backtrace_arms),*
                    }
                }
            };

            let error_compat_impl = quote! {
                #[allow(single_use_lifetimes)]
                impl<#(#original_generics),*> #crate_root::ErrorCompat for #parameterized_error_name
                where
                    #(#where_clauses),*
                {
                    #backtrace_fn
                }
            };

            stream.extend(error_compat_impl);
        }
    }

    pub(crate) struct ErrorCompatBacktraceMatchArm<'a> {
        pub(crate) crate_root: &'a dyn ToTokens,
        pub(crate) field_container: &'a FieldContainer,
        pub(crate) pattern_ident: &'a dyn ToTokens,
    }

    impl ToTokens for ErrorCompatBacktraceMatchArm<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let Self {
                crate_root,
                field_container:
                    FieldContainer {
                        backtrace_field,
                        selector_kind,
                        ..
                    },
                pattern_ident,
            } = *self;

            let match_arm = match (selector_kind.source_field(), backtrace_field) {
                (Some(source_field), _) if source_field.backtrace_delegate => {
                    let SourceField {
                        name: field_name, ..
                    } = source_field;
                    quote! {
                        #pattern_ident { ref #field_name, .. } => { #crate_root::ErrorCompat::backtrace(#field_name) }
                    }
                }
                (_, Some(backtrace_field)) => {
                    let Field {
                        name: field_name, ..
                    } = backtrace_field;
                    quote! {
                        #pattern_ident { ref #field_name, .. } => { #crate_root::AsBacktrace::as_backtrace(#field_name) }
                    }
                }
                _ => {
                    quote! {
                        #pattern_ident { .. } => { ::core::option::Option::None }
                    }
                }
            };

            stream.extend(match_arm);
        }
    }
}
