// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

macro_rules! insert_builtin {
    ($builtin: expr, $name: ident, $pattern: expr) => {
        $builtin.push((stringify!($name), generate_rule!($name, $pattern)));
    };
}

#[cfg(feature = "std")]
macro_rules! generate_rule {
    ($name: ident, $pattern: expr) => {
        quote! {
            #[inline]
            #[allow(dead_code, non_snake_case, unused_variables)]
            pub fn $name(state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>) -> ::pest::ParseResult<::std::boxed::Box<::pest::ParserState<'_, Rule>>> {
                $pattern
            }
        }
    }
}

#[cfg(not(feature = "std"))]
macro_rules! generate_rule {
    ($name: ident, $pattern: expr) => {
        quote! {
            #[inline]
            #[allow(dead_code, non_snake_case, unused_variables)]
            pub fn $name(state: ::alloc::boxed::Box<::pest::ParserState<'_, Rule>>) -> ::pest::ParseResult<::alloc::boxed::Box<::pest::ParserState<'_, Rule>>> {
                $pattern
            }
        }
    }
}
