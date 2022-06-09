use core::panic;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ForeignItemFn, ItemFn, Type, Visibility};

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
                panic!("all arguments must have specified types, no `self` allowed")
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
        pub extern "C" fn #outer_fn_name(ptr: *const u8, len: usize) -> *const ::plugin::__Buffer {
            // setup
            let buffer = ::plugin::__Buffer { ptr, len };
            let data = unsafe { buffer.to_vec() };

            // operation
            let data: #ty = match ::plugin::bincode::deserialize(&data) {
                Ok(d) => d,
                Err(e) => panic!("Data passed to function not deserializable."),
            };
            let result = #inner_fn_name(#args);
            let new_data: Result<Vec<u8>, _> = ::plugin::bincode::serialize(&result);
            let new_data = new_data.unwrap();

            // teardown
            let new_buffer = unsafe { ::plugin::__Buffer::from_vec(new_data) };
            return new_buffer.leak_to_heap();
        }
    })
}

#[proc_macro_attribute]
pub fn import(args: TokenStream, function: TokenStream) -> TokenStream {
    if !args.is_empty() {
        panic!("The import attribute does not take any arguments");
    }

    let fn_declare = parse_macro_input!(function as ForeignItemFn);

    if !fn_declare.sig.generics.params.is_empty() {
        panic!("Exported functions can not take generic parameters");
    }

    dbg!(&fn_declare.sig);

    // let inner_fn = ItemFn {
    //     attrs: fn_declare.attrs,
    //     vis: fn_declare.vis,
    //     sig: fn_declare.sig,
    //     block: todo!(),
    // };

    // let inner_fn_name = format_ident!("{}", inner_fn.sig.ident);
    // let outer_fn_name = format_ident!("__{}", inner_fn_name);

    //     let variadic = inner_fn.sig.inputs.len();
    //     let i = (0..variadic).map(syn::Index::from);
    //     let t: Vec<Type> = inner_fn
    //         .sig
    //         .inputs
    //         .iter()
    //         .map(|x| match x {
    //             FnArg::Receiver(_) => {
    //                 panic!("all arguments must have specified types, no `self` allowed")
    //             }
    //             FnArg::Typed(item) => *item.ty.clone(),
    //         })
    //         .collect();

    //     // this is cursed...
    //     let (args, ty) = if variadic != 1 {
    //         (
    //             quote! {
    //                 #( data.#i ),*
    //             },
    //             quote! {
    //                 ( #( #t ),* )
    //             },
    //         )
    //     } else {
    //         let ty = &t[0];
    //         (quote! { data }, quote! { #ty })
    //     };

    // TokenStream::from(quote! {
    //     extern "C" {
    //         #[no_mangle]
    //         fn #outer_fn_name(ptr: *const u8, len: usize) -> *const ::plugin::__Buffer;
    //     }

    //     #[no_mangle]
    //     fn #inner_fn_name #params -> #output {
    //         println!("executing command: {}", string);
    //         // serialize data
    //         let data = #collect_params;
    //         let data = ::plugin::bincode::serialize(&data).unwrap();
    //         let buffer = unsafe { ::plugin::__Buffer::from_vec(data) };
    //         let ptr = buffer.ptr;
    //         let len = buffer.len;
    //         // leak data to heap
    //         buffer.leak_to_heap();
    //         // call extern function
    //         let result = unsafe { __command(ptr, len) };
    //         // get result
    //         let result = todo!(); // convert into box

    //         // deserialize data
    //         let data: Option<String> = match ::plugin::bincode::deserialize(&data) {
    //             Ok(d) => d,
    //             Err(e) => panic!("Data passed to function not deserializable."),
    //         };
    //         return data;
    //     }
    // })
    todo!()
}
