use crate::register_action::generate_register_action;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, LitStr, Token, parse::ParseStream};

/// Implementation of the `Action` derive macro - see docs on the `Action` trait for details.
pub(crate) fn derive_action(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let mut name_argument = None;
    let mut deprecated_aliases = Vec::new();
    let mut internal = false;
    let mut namespace = None;
    let mut deprecated = None;

    for attr in &input.attrs {
        if attr.path().is_ident("action") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    if name_argument.is_some() {
                        return Err(meta.error("'name' argument specified multiple times"));
                    }
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    name_argument = Some(lit.value());
                } else if meta.path.is_ident("namespace") {
                    if namespace.is_some() {
                        return Err(meta.error("'namespace' argument specified multiple times"));
                    }
                    meta.input.parse::<Token![=]>()?;
                    let ident: Ident = meta.input.parse()?;
                    namespace = Some(ident.to_string());
                } else if meta.path.is_ident("internal") {
                    if internal {
                        return Err(meta.error("'internal' argument specified multiple times"));
                    }
                    internal = true;
                } else if meta.path.is_ident("deprecated_aliases") {
                    if !deprecated_aliases.is_empty() {
                        return Err(
                            meta.error("'deprecated_aliases' argument specified multiple times")
                        );
                    }
                    meta.input.parse::<Token![=]>()?;
                    // Parse array of string literals
                    let content;
                    syn::bracketed!(content in meta.input);
                    let aliases = content.parse_terminated(
                        |input: ParseStream| input.parse::<LitStr>(),
                        Token![,],
                    )?;
                    deprecated_aliases.extend(aliases.into_iter().map(|lit| lit.value()));
                } else if meta.path.is_ident("deprecated") {
                    if deprecated.is_some() {
                        return Err(meta.error("'deprecated' argument specified multiple times"));
                    }
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    deprecated = Some(lit.value());
                } else {
                    return Err(meta.error(format!("'{:?}' argument not recognized", meta.path)));
                }
                Ok(())
            })
            .unwrap_or_else(|e| panic!("In #[action] attribute: {}", e));
        }
    }

    let full_name = if let Some(name) = name_argument {
        if name.contains("::") {
            if let Some(namespace) = namespace {
                panic!(
                    "Action derive macro received `name = \"{name}\"`, which contains `::` \
                    and so `namespace = \"{namespace}\"` should not be specified."
                )
            }
            name
        } else {
            if let Some(namespace) = namespace {
                format!("{namespace}::{name}")
            } else {
                panic!(
                    "Action derive macro received `name = \"{name}\"`, \
                    but `namespace` was not specified."
                );
            }
        }
    } else {
        if let Some(namespace) = namespace {
            format!("{namespace}::{struct_name}")
        } else {
            panic!(
                "Action derive macro for {struct_name} \
                must specify #[action(namespace = some_namespace)]"
            );
        }
    };

    let is_unit_struct = matches!(&input.data, Data::Struct(data) if data.fields.is_empty());

    let build_fn = if internal {
        let error_msg = format!(
            "{} is an internal action, so cannot be built from JSON.",
            full_name
        );
        quote! {
            fn build(_: gpui::private::serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
                Err(gpui::private::anyhow::anyhow!(#error_msg))
            }
        }
    } else if is_unit_struct {
        quote! {
            fn build(_: gpui::private::serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
                Ok(Box::new(Self))
            }
        }
    } else {
        quote! {
            fn build(value: gpui::private::serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
                Ok(Box::new(gpui::private::serde_json::from_value::<Self>(value)?))
            }
        }
    };

    let json_schema_fn = if internal || is_unit_struct {
        quote! {
            fn action_json_schema(
                _: &mut gpui::private::schemars::r#gen::SchemaGenerator,
            ) -> Option<gpui::private::schemars::schema::Schema> {
                None
            }
        }
    } else {
        quote! {
            fn action_json_schema(
                generator: &mut gpui::private::schemars::r#gen::SchemaGenerator,
            ) -> Option<gpui::private::schemars::schema::Schema> {
                Some(<Self as gpui::private::schemars::JsonSchema>::json_schema(generator))
            }
        }
    };

    let deprecated_aliases_fn = if deprecated_aliases.is_empty() {
        quote! {
            fn deprecated_aliases() -> &'static [&'static str] {
                &[]
            }
        }
    } else {
        let aliases = deprecated_aliases.iter();
        quote! {
            fn deprecated_aliases() -> &'static [&'static str] {
                &[#(#aliases),*]
            }
        }
    };

    let deprecation_fn = if let Some(message) = deprecated {
        quote! {
            fn deprecation_message() -> Option<&'static str>
            where
                Self: Sized
            {
                Some(#message)
            }
        }
    } else {
        quote! {
            fn deprecation_message() -> Option<&'static str>
            where
                Self: Sized
            {
                None
            }
        }
    };

    let registration = generate_register_action(struct_name);

    TokenStream::from(quote! {
        #registration

        impl gpui::Action for #struct_name {
            fn name(&self) -> &'static str {
                #full_name
            }

            fn name_for_type() -> &'static str
            where
                Self: Sized
            {
                #full_name
            }

            fn partial_eq(&self, action: &dyn gpui::Action) -> bool {
                action
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |a| self == a)
            }

            fn boxed_clone(&self) -> Box<dyn gpui::Action> {
                Box::new(self.clone())
            }

            #build_fn

            #json_schema_fn

            #deprecated_aliases_fn

            #deprecation_fn
        }
    })
}
