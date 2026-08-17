#![allow(clippy::single_match)]

#[macro_use]
extern crate combine;

parser! {
    pub fn test[Input]()(Input) -> ()
        where [Input: ::combine::Stream<Token = char>]
    {

use combine::parser::token::value;

let _ = ();
        fn _test() { }
        match Some(1) {
            Some(_) => (),
            None => (),
        }
        value(())
    }
}

parser! {
    pub fn test_that_parsers_with_unnamed_types_can_be_in_same_scope[Input]()(Input) -> ()
        where [Input: ::combine::Stream<Token = char>]
    {

use combine::parser::token::value;

value(())
    }
}

#[test]
fn test_that_we_dont_need_imports_for_this_macro_to_work() {
    test::<&str>();
    test_that_parsers_with_unnamed_types_can_be_in_same_scope::<&str>();
}
