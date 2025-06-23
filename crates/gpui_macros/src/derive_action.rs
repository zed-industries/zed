use crate::register_action::generate_register_action;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, LitStr, Token, parse::ParseStream};

/// Implementation of the `Action` derive macro - see docs on the `Action` trait for details.
pub(crate) fn derive_action(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let mut action_name = None;
    let mut deprecated_aliases = Vec::new();
    let mut internal = false;
    let mut namespace = None;

    let mut has_name = false;
    let mut has_namespace = false;
    let mut has_internal = false;
    let mut has_deprecated_aliases = false;

    for attr in &input.attrs {
        if attr.path().is_ident("action") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    if has_name {
                        return Err(meta.error("'name' argument specified multiple times"));
                    }
                    has_name = true;
                    meta.input.parse::<Token![=]>()?;
                    // Handle either a string literal or concat! macro
                    if meta.input.peek(syn::token::Paren) {
                        // Skip concat! for now, will be handled by the macro expansion
                        let _content;
                        syn::parenthesized!(_content in meta.input);
                    } else {
                        let lit: LitStr = meta.input.parse()?;
                        action_name = Some(lit.value());
                    }
                } else if meta.path.is_ident("namespace") {
                    if has_namespace {
                        return Err(meta.error("'namespace' argument specified multiple times"));
                    }
                    has_namespace = true;
                    meta.input.parse::<Token![=]>()?;
                    let ident: Ident = meta.input.parse()?;
                    namespace = Some(ident.to_string());
                } else if meta.path.is_ident("internal") {
                    if has_internal {
                        return Err(meta.error("'internal' argument specified multiple times"));
                    }
                    has_internal = true;
                    internal = true;
                } else if meta.path.is_ident("deprecated_aliases") {
                    if has_deprecated_aliases {
                        return Err(
                            meta.error("'deprecated_aliases' argument specified multiple times")
                        );
                    }
                    has_deprecated_aliases = true;
                    meta.input.parse::<Token![=]>()?;
                    // Parse array of string literals
                    let content;
                    syn::bracketed!(content in meta.input);
                    let aliases = content.parse_terminated(
                        |input: ParseStream| input.parse::<LitStr>(),
                        Token![,],
                    )?;
                    deprecated_aliases.extend(aliases.into_iter().map(|lit| lit.value()));
                }
                Ok(())
            })
            .unwrap_or_else(|e| panic!("Failed to parse action attribute: {}", e));
        }
    }

    // Determine the full action name
    let full_name = if let Some(name) = action_name {
        name
    } else if let Some(ns) = namespace {
        format!("{}::{}", ns, name)
    } else {
        // No name or namespace provided, just use the struct name
        name.to_string()
    };

    // Check if this is a unit struct
    let is_unit_struct = matches!(&input.data, Data::Struct(data) if data.fields.is_empty());

    // Generate the build function
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

    // Generate the JSON schema function
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

    // Generate deprecated aliases function
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

    let registration = generate_register_action(name);

    TokenStream::from(quote! {
        #registration

        impl gpui::Action for #name {
            fn name(&self) -> &'static str {
                #full_name
            }

            fn debug_name() -> &'static str
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
        }
    })
}
