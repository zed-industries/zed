//! Derives traits from the [palette](https://crates.io/crates/palette) crate.

use proc_macro::TokenStream;

macro_rules! syn_try {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(errors) => {
                trait IntoErrors {
                    fn into_errors(self) -> Vec<::syn::parse::Error>;
                }
                impl IntoErrors for Vec<::syn::parse::Error> {
                    fn into_errors(self) -> Vec<::syn::parse::Error> {
                        self
                    }
                }
                impl IntoErrors for ::syn::parse::Error {
                    fn into_errors(self) -> Vec<::syn::parse::Error> {
                        vec![self]
                    }
                }

                let errors: ::proc_macro2::TokenStream = IntoErrors::into_errors(errors)
                    .iter()
                    .map(::syn::parse::Error::to_compile_error)
                    .collect();
                return ::proc_macro::TokenStream::from(errors);
            }
        }
    };
}

mod alpha;
mod cast;
mod color_types;
mod convert;
mod meta;
mod util;

#[proc_macro_derive(WithAlpha, attributes(palette))]
pub fn derive_with_alpha(tokens: TokenStream) -> TokenStream {
    syn_try!(alpha::derive_with_alpha(tokens))
}

#[proc_macro_derive(FromColorUnclamped, attributes(palette))]
pub fn derive_from_color_unclamped(tokens: TokenStream) -> TokenStream {
    syn_try!(convert::derive_from_color_unclamped(tokens))
}

#[proc_macro_derive(ArrayCast, attributes(palette))]
pub fn derive_array_cast(tokens: TokenStream) -> TokenStream {
    syn_try!(cast::derive_array_cast(tokens))
}
