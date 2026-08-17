use proc_macro::TokenStream;

mod interp_disable_if_cfg;

#[proc_macro_attribute]
pub fn interp_disable_if_cfg(attrs: TokenStream, item: TokenStream) -> TokenStream {
    interp_disable_if_cfg::run(attrs, item)
}
