use core::panic;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Visibility};

#[proc_macro_attribute]
pub fn bind(args: TokenStream, function: TokenStream) -> TokenStream {
    if !args.is_empty() {
        panic!("The bind attribute does not take any arguments");
    }

    let inner_fn = parse_macro_input!(function as ItemFn);
    if let Visibility::Public(_) = inner_fn.vis {
    } else {
        panic!("The bind attribute only works for public functions");
    }

    let inner_fn_name = format_ident!("{}", inner_fn.sig.ident);
    let outer_fn_name = format_ident!("__{}", inner_fn_name);

    TokenStream::from(quote! {
        #[no_mangle]
        #inner_fn

        #[no_mangle]
        pub extern "C" fn #outer_fn_name(ptr: *const u8, len: usize) -> *const ::plugin::__Buffer {
            // setup
            let buffer = ::plugin::__Buffer { ptr, len };
            let data = unsafe { buffer.to_vec() };

            // operation
            let argument = ::bincode::deserialize(&data).unwrap();
            let result = #inner_fn_name(argument);
            let new_data: Result<Vec<u8>, _> = ::bincode::serialize(&result);
            let new_data = new_data.unwrap();

            // teardown
            let new_buffer = unsafe { ::plugin::__Buffer::from_vec(new_data) };
            return new_buffer.leak_to_heap();
        }
    })
}
