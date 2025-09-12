use crate::register_action::generate_register_action;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, LitStr, Token, parse::ParseStream};

pub(crate) fn derive_action(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let mut name_argument = None;
    let mut deprecated_aliases = Vec::new();
    let mut no_json = false;
    let mut no_register = false;
    let mut namespace = None;
    let mut deprecated = None;
    let mut doc_str: Option<String> = None;

    /*
    *
    * #[action()]
    * Struct Foo {
    *  bar: bool // is bar considered an attribute
    }
    */
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
                } else if meta.path.is_ident("no_json") {
                    if no_json {
                        return Err(meta.error("'no_json' argument specified multiple times"));
                    }
                    no_json = true;
                } else if meta.path.is_ident("no_register") {
                    if no_register {
                        return Err(meta.error("'no_register' argument specified multiple times"));
                    }
                    no_register = true;
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
                    return Err(meta.error(format!(
                        "'{:?}' argument not recognized, expected \
                        'namespace', 'no_json', 'no_register, 'deprecated_aliases', or 'deprecated'",
                        meta.path
                    )));
                }
                Ok(())
            })
            .unwrap_or_else(|e| panic!("in #[action] attribute: {}", e));
        } else if attr.path().is_ident("doc") {
            use syn::{Expr::Lit, ExprLit, Lit::Str, Meta, MetaNameValue};
            if let Meta::NameValue(MetaNameValue {
                value:
                    Lit(ExprLit {
                        lit: Str(ref lit_str),
                        ..
                    }),
                ..
            }) = attr.meta
            {
                let doc = lit_str.value();
                let doc_str = doc_str.get_or_insert_default();
                doc_str.push_str(doc.trim());
                doc_str.push('\n');
            }
        }
    }

    let name = name_argument.unwrap_or_else(|| struct_name.to_string());

    if name.contains("::") {
        panic!(
            "in #[action] attribute: `name = \"{name}\"` must not contain `::`, \
            also specify `namespace` instead"
        );
    }

    let full_name = if let Some(namespace) = namespace {
        format!("{namespace}::{name}")
    } else {
        name
    };

    let is_unit_struct = matches!(&input.data, Data::Struct(data) if data.fields.is_empty());

    let build_fn_body = if no_json {
        let error_msg = format!("{} cannot be built from JSON", full_name);
        quote! { Err(gpui::private::anyhow::anyhow!(#error_msg)) }
    } else if is_unit_struct {
        quote! { Ok(Box::new(Self)) }
    } else {
        quote! { Ok(Box::new(gpui::private::serde_json::from_value::<Self>(_value)?)) }
    };

    let json_schema_fn_body = if no_json || is_unit_struct {
        quote! { None }
    } else {
        quote! { Some(<Self as gpui::private::schemars::JsonSchema>::json_schema(_generator)) }
    };

    let deprecated_aliases_fn_body = if deprecated_aliases.is_empty() {
        quote! { &[] }
    } else {
        let aliases = deprecated_aliases.iter();
        quote! { &[#(#aliases),*] }
    };

    let deprecation_fn_body = if let Some(message) = deprecated {
        quote! { Some(#message) }
    } else {
        quote! { None }
    };

    let documentation_fn_body = if let Some(doc) = doc_str {
        let doc = doc.trim();
        quote! { Some(#doc) }
    } else {
        quote! { None }
    };

    let registration = if no_register {
        quote! {}
    } else {
        generate_register_action(struct_name)
    };

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

            fn build(_value: gpui::private::serde_json::Value) -> gpui::Result<Box<dyn gpui::Action>> {
                #build_fn_body
            }

            fn action_json_schema(
                _generator: &mut gpui::private::schemars::SchemaGenerator,
            ) -> Option<gpui::private::schemars::Schema> {
                #json_schema_fn_body
            }

            fn deprecated_aliases() -> &'static [&'static str] {
                #deprecated_aliases_fn_body
            }

            fn deprecation_message() -> Option<&'static str> {
                #deprecation_fn_body
            }

            fn documentation() -> Option<&'static str> {
                #documentation_fn_body
            }
        }
    })
}
