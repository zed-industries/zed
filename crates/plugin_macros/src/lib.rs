use core::panic;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Block, FnArg, ForeignItemFn, Ident, ItemFn, Pat, PatIdent, Type, Visibility,
};

/// Attribute macro to be used guest-side within a plugin.
/// ```ignore
/// #[export]
/// pub fn say_hello() -> String {
///     "Hello from Wasm".into()
/// }
/// ```
/// This macro makes a function defined guest-side avaliable host-side.
/// Note that all arguments and return types must be `serde`.
#[proc_macro_attribute]
pub fn export(args: TokenStream, function: TokenStream) -> TokenStream {
    if !args.is_empty() {
        panic!("The export attribute does not take any arguments");
    }

    let inner_fn = parse_macro_input!(function as ItemFn);

    if !inner_fn.sig.generics.params.is_empty() {
        panic!("Exported functions can not take generic parameters");
    }

    if let Visibility::Public(_) = inner_fn.vis {
    } else {
        panic!("The export attribute only works for public functions");
    }

    let inner_fn_name = format_ident!("{}", inner_fn.sig.ident);
    let outer_fn_name = format_ident!("__{}", inner_fn_name);

    let variadic = inner_fn.sig.inputs.len();
    let i = (0..variadic).map(syn::Index::from);
    let t: Vec<Type> = inner_fn
        .sig
        .inputs
        .iter()
        .map(|x| match x {
            FnArg::Receiver(_) => {
                panic!("All arguments must have specified types, no `self` allowed")
            }
            FnArg::Typed(item) => *item.ty.clone(),
        })
        .collect();

    // this is cursed...
    let (args, ty) = if variadic != 1 {
        (
            quote! {
                #( data.#i ),*
            },
            quote! {
                ( #( #t ),* )
            },
        )
    } else {
        let ty = &t[0];
        (quote! { data }, quote! { #ty })
    };

    TokenStream::from(quote! {
        #[no_mangle]
        #inner_fn

        #[no_mangle]
        // TODO: switch len from usize to u32?
        pub extern "C" fn #outer_fn_name(packed_buffer: u64) -> u64 {
            // setup
            let data = unsafe { ::plugin::__Buffer::from_u64(packed_buffer).to_vec() };

            // operation
            let data: #ty = match ::plugin::bincode::deserialize(&data) {
                Ok(d) => d,
                Err(e) => panic!("Data passed to function not deserializable."),
            };
            let result = #inner_fn_name(#args);
            let new_data: Result<Vec<u8>, _> = ::plugin::bincode::serialize(&result);
            let new_data = new_data.unwrap();

            // teardown
            let new_buffer = unsafe { ::plugin::__Buffer::from_vec(new_data) }.into_u64();
            return new_buffer;
        }
    })
}

/// Attribute macro to be used guest-side within a plugin.
/// ```ignore
/// #[import]
/// pub fn operating_system_name() -> String;
/// ```
/// This macro makes a function defined host-side avaliable guest-side.
/// Note that all arguments and return types must be `serde`.
/// All that's provided is a signature, as the function is implemented host-side.
#[proc_macro_attribute]
pub fn import(args: TokenStream, function: TokenStream) -> TokenStream {
    if !args.is_empty() {
        panic!("The import attribute does not take any arguments");
    }

    let fn_declare = parse_macro_input!(function as ForeignItemFn);

    if !fn_declare.sig.generics.params.is_empty() {
        panic!("Exported functions can not take generic parameters");
    }

    // let inner_fn_name = format_ident!("{}", fn_declare.sig.ident);
    let extern_fn_name = format_ident!("__{}", fn_declare.sig.ident);

    let (args, tys): (Vec<Ident>, Vec<Type>) = fn_declare
        .sig
        .inputs
        .clone()
        .into_iter()
        .map(|x| match x {
            FnArg::Receiver(_) => {
                panic!("All arguments must have specified types, no `self` allowed")
            }
            FnArg::Typed(t) => {
                if let Pat::Ident(i) = *t.pat {
                    (i.ident, *t.ty)
                } else {
                    panic!("All function arguments must be identifiers");
                }
            }
        })
        .unzip();

    let body = TokenStream::from(quote! {
        {
            // setup
            let data: (#( #tys ),*) = (#( #args ),*);
            let data = ::plugin::bincode::serialize(&data).unwrap();
            let buffer = unsafe { ::plugin::__Buffer::from_vec(data) };

            // operation
            let new_buffer = unsafe { #extern_fn_name(buffer.into_u64()) };
            let new_data = unsafe { ::plugin::__Buffer::from_u64(new_buffer).to_vec() };

            // teardown
            match ::plugin::bincode::deserialize(&new_data) {
                Ok(d) => d,
                Err(e) => panic!("Data returned from function not deserializable."),
            }
        }
    });

    let block = parse_macro_input!(body as Block);

    let inner_fn = ItemFn {
        attrs: fn_declare.attrs,
        vis: fn_declare.vis,
        sig: fn_declare.sig,
        block: Box::new(block),
    };

    TokenStream::from(quote! {
        extern "C" {
            fn #extern_fn_name(buffer: u64) -> u64;
        }

        #[no_mangle]
        #inner_fn
    })
}
