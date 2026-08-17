use proc_macro2::TokenStream;
use syn::parse2;

use self::validate::validate;

mod codegen;
mod options;
mod utils;
mod validate;

#[cfg(test)]
mod tests;

/// try to parse an item, or return the error as a token stream
macro_rules! parse {
    ($e:expr) => {
        match parse2($e) {
            Ok(item) => item,
            Err(e) => return e.into_compile_error(),
        }
    };
}

pub fn property_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse!(item);
    let options = parse!(attr);

    if let Err(compile_error) = validate(&mut item_fn) {
        return compile_error;
    }

    codegen::generate(item_fn, options)
}
