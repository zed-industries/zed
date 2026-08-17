mod codegen_settings;
pub mod config;
mod funcs;
mod lifetimes;
mod module_trait;
pub mod names;
mod types;
pub mod wasmtime;

use heck::ToShoutySnakeCase;
use lifetimes::anon_lifetime;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub use codegen_settings::{CodegenSettings, ErrorType, UserErrorType};
pub use config::{Config, WasmtimeConfig};
pub use funcs::define_func;
pub use module_trait::define_module_trait;
pub use types::define_datatype;

pub fn generate(doc: &witx::Document, settings: &CodegenSettings) -> TokenStream {
    let types = doc
        .typenames()
        .map(|t| define_datatype(&t, settings.errors.for_name(&t)));

    let constants = doc.constants().map(|c| {
        let name = quote::format_ident!(
            "{}_{}",
            c.ty.as_str().to_shouty_snake_case(),
            c.name.as_str().to_shouty_snake_case()
        );
        let ty = names::type_(&c.ty);
        let value = Literal::u64_unsuffixed(c.value);
        quote! {
            pub const #name: #ty = #value;
        }
    });

    let user_error_methods = settings.errors.iter().filter_map(|errtype| match errtype {
        ErrorType::User(errtype) => {
            let abi_typename = names::type_ref(&errtype.abi_type(), anon_lifetime());
            let user_typename = errtype.typename();
            let methodname = names::user_error_conversion_method(&errtype);
            Some(quote! {
                fn #methodname(&mut self, e: super::#user_typename)
                    -> wiggle::anyhow::Result<#abi_typename>;
            })
        }
        ErrorType::Generated(_) => None,
    });
    let user_error_conversion = quote! {
        pub trait UserErrorConversion {
            #(#user_error_methods)*
        }
    };
    let modules = doc.modules().map(|module| {
        let modname = names::module(&module.name);
        let fs = module.funcs().map(|f| define_func(&module, &f, &settings));
        let modtrait = define_module_trait(&module, &settings);
        let wasmtime = if settings.wasmtime {
            crate::wasmtime::link_module(&module, None, &settings)
        } else {
            quote! {}
        };
        quote!(
            pub mod #modname {
                use super::types::*;
                pub use super::types::UserErrorConversion;
                #(#fs)*

                #modtrait

                #wasmtime
            }
        )
    });

    quote!(
        pub mod types {
            use std::convert::TryFrom;

            #(#types)*
            #(#constants)*
            #user_error_conversion
        }
        #(#modules)*
    )
}

pub fn generate_metadata(doc: &witx::Document) -> TokenStream {
    let doc_text = &format!("{doc}");
    quote! {
        pub mod metadata {
            pub const DOC_TEXT: &str = #doc_text;
            pub fn document() -> wiggle::witx::Document {
                wiggle::witx::parse(DOC_TEXT).unwrap()
            }
        }
    }
}
