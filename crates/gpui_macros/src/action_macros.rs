use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, LitStr, Path, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// Common code generation for Action trait implementation
fn generate_action_impl(
    name: &Ident,
    full_name: String,
    no_json: bool,
    is_unit_struct: bool,
    deprecated_aliases: Vec<String>,
) -> TokenStream2 {
    // Generate the build function
    let build_fn = if no_json {
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
    let json_schema_fn = if no_json || is_unit_struct {
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

    quote! {
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
    }
}

/// Derive macro for implementing the Action trait
pub fn derive_action(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let mut action_name = None;
    let mut deprecated_aliases = Vec::new();
    let mut no_json = false;
    let mut namespace = None;

    // Parse the #[action(...)] attribute
    for attr in &input.attrs {
        if attr.path().is_ident("action") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
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
                    meta.input.parse::<Token![=]>()?;
                    let lit: LitStr = meta.input.parse()?;
                    namespace = Some(lit.value());
                } else if meta.path.is_ident("no_json") {
                    no_json = true;
                } else if meta.path.is_ident("deprecated_aliases") {
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
        // For unit structs defined by actions! macro, the full name
        // will be provided via the name attribute
        name.to_string()
    };

    // Check if this is a unit struct
    let is_unit_struct = matches!(&input.data, Data::Struct(data) if data.fields.is_empty());

    let action_impl =
        generate_action_impl(name, full_name, no_json, is_unit_struct, deprecated_aliases);

    let registration = register_action(name);

    let output = quote! {
        #action_impl
        #registration
    };

    TokenStream::from(output)
}

/// Parse impl_action! macro with fixed arguments
/// Format: impl_action!(Type, namespace, name, no_json, ["alias1", "alias2"])
pub fn impl_action_macro(input: TokenStream) -> TokenStream {
    let ImplActionInput {
        action_struct,
        action_namespace,
        action_name,
        no_json,
        deprecated_aliases,
    } = parse_macro_input!(input as ImplActionInput);

    let full_name = format!("{}::{}", quote!(#action_namespace), quote!(#action_name));

    let aliases: Vec<String> = deprecated_aliases.iter().map(|lit| lit.value()).collect();

    let action_impl = generate_action_impl(
        &action_struct,
        full_name,
        no_json,
        false, // not a unit struct
        aliases,
    );

    let registration = register_action(&action_struct);

    let output = quote! {
        #action_impl
        #registration
    };

    TokenStream::from(output)
}

struct ImplActionInput {
    action_struct: Ident,
    action_namespace: Path,
    action_name: Ident,
    no_json: bool,
    deprecated_aliases: Vec<LitStr>,
}

impl Parse for ImplActionInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let action_struct = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        let action_namespace = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;

        let action_name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        let no_json = input.parse::<syn::LitBool>()?.value;
        input.parse::<Token![,]>()?;

        let content;
        syn::bracketed!(content in input);
        let deprecated_aliases = content
            .parse_terminated(|input: ParseStream| input.parse::<LitStr>(), Token![,])?
            .into_iter()
            .collect();

        if !input.is_empty() {
            return Err(input.error("Unexpected tokens"));
        }

        Ok(ImplActionInput {
            action_struct,
            action_namespace,
            action_name,
            no_json,
            deprecated_aliases,
        })
    }
}

/// Generate the registration code for an action
pub fn register_action_macro(ident: TokenStream) -> TokenStream {
    let name = parse_macro_input!(ident as Ident);
    let registration = register_action(&name);

    TokenStream::from(quote! {
        #registration
    })
}

fn register_action(type_name: &Ident) -> TokenStream2 {
    let action_builder_fn_name = format_ident!(
        "__gpui_actions_builder_{}",
        type_name.to_string().to_lowercase()
    );

    quote! {
        impl #type_name {
            /// This is an auto generated function, do not use.
            #[automatically_derived]
            #[doc(hidden)]
            fn __autogenerated() {
                /// This is an auto generated function, do not use.
                #[doc(hidden)]
                fn #action_builder_fn_name() -> gpui::MacroActionData {
                    gpui::MacroActionData {
                        name: <#type_name as gpui::Action>::debug_name(),
                        aliases: <#type_name as gpui::Action>::deprecated_aliases(),
                        type_id: ::std::any::TypeId::of::<#type_name>(),
                        build: <#type_name as gpui::Action>::build,
                        json_schema: <#type_name as gpui::Action>::action_json_schema,
                    }
                }

                gpui::private::inventory::submit! {
                    gpui::MacroActionBuilder(#action_builder_fn_name)
                }
            }
        }
    }
}
