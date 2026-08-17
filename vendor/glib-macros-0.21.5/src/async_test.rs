// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use quote::ToTokens;

pub(crate) fn async_test(_args: TokenStream, mut item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => {
            item.extend(TokenStream::from(e.into_compile_error()));
            return item;
        }
    };

    if item_fn.sig.asyncness.is_none() {
        item.extend(TokenStream::from(
            syn::Error::new_spanned(
                item_fn.sig.ident,
                "The 'async' keyword is missing from the test function declaration",
            )
            .into_compile_error(),
        ));
        return item;
    }

    item_fn.sig.asyncness = None;

    let gen_attr = quote::quote! {
        #[::core::prelude::v1::test]
    };

    let body = &item_fn.block;

    item_fn.block = syn::parse2(quote::quote! {
        {
            let main_ctx = glib::MainContext::new();
            main_ctx.with_thread_default(|| main_ctx.block_on(async #body))
                .expect("cannot set thread default main context for test")
        }
    })
    .expect("Body parsing failure");

    let mut tokens = TokenStream::new();
    tokens.extend(TokenStream::from(gen_attr.to_token_stream()));
    tokens.extend(TokenStream::from(item_fn.into_token_stream()));

    tokens
}
