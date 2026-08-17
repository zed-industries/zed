extern crate proc_macro;

use proc_macro::*;

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn serialize(_items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn deserialize(_items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

#[proc_macro_attribute]
pub fn hawktracer(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn scoped_tracepoint(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// https://docs.rs/wasm-bindgen-macro/0.2.63/src/wasm_bindgen_macro/lib.rs.html#9-19
#[proc_macro_attribute]
pub fn wasm_bindgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
