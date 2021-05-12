use std::mem;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let mut inner_fn = parse_macro_input!(function as ItemFn);
    let inner_fn_name = format_ident!("_{}", inner_fn.sig.ident);
    let outer_fn_name = mem::replace(&mut inner_fn.sig.ident, inner_fn_name.clone());
    let outer_fn = if inner_fn.sig.asyncness.is_some() {
        quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                gpui::App::test_async((), move |ctx| async {
                    #inner_fn_name(ctx).await;
                });
            }
        }
    } else {
        quote! {
            #[test]
            fn #outer_fn_name() {
                #inner_fn

                gpui::App::test((), |ctx| {
                    #inner_fn_name(ctx);
                });
            }
        }
    };

    TokenStream::from(outer_fn)
}
