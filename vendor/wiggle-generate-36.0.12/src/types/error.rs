use crate::codegen_settings::TrappableErrorType;
use crate::names;

use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn define_error(
    name: &witx::Id,
    _v: &witx::Variant,
    e: &TrappableErrorType,
) -> TokenStream {
    let abi_error = names::type_(name);
    let rich_error = e.typename();

    quote! {
        #[derive(Debug)]
        pub struct #rich_error {
            inner: anyhow::Error,
        }

        impl std::fmt::Display for #rich_error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.inner)
            }
        }
        impl std::error::Error for #rich_error {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.inner.source()
            }
        }

        impl #rich_error {
            pub fn trap(inner: anyhow::Error) -> #rich_error {
                Self { inner }
            }
            pub fn downcast(self) -> Result<#abi_error, anyhow::Error> {
                self.inner.downcast()
            }
            pub fn downcast_ref(&self) -> Option<&#abi_error> {
                self.inner.downcast_ref()
            }
            pub fn context(self, s: impl Into<String>) -> Self {
                Self { inner: self.inner.context(s.into()) }
            }
        }

        impl From<#abi_error> for #rich_error {
            fn from(abi: #abi_error) -> #rich_error {
                #rich_error { inner: anyhow::Error::from(abi) }
            }
        }
    }
}
