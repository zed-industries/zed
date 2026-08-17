// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
extern crate pest;
extern crate pest_derive;

#[cfg(feature = "grammar-extras")]
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../tests/implicit.pest"]
struct TestImplicitParser;

#[test]
#[cfg(feature = "grammar-extras")]
fn test_implicit_whitespace() {
    // this failed to parse due to a bug in the optimizer
    // see: https://github.com/pest-parser/pest/issues/762#issuecomment-1375374868
    let successful_parse = TestImplicitParser::parse(Rule::program, "a a");
    assert!(successful_parse.is_ok());
    // dbg!(&successful_parse);
    let pairs = successful_parse.unwrap();
    assert!(pairs.find_first_tagged("head").is_some());
    assert!(pairs.find_first_tagged("tail").is_some());
    assert!(pairs.find_first_tagged("more_and").is_none());
    assert!(pairs.find_first_tagged("more_comp").is_none());
    assert!(pairs.find_first_tagged("more_array").is_none());
    assert_eq!(pairs.clone().find_tagged("one_and").count(), 2);
    assert_eq!(pairs.clone().find_tagged("one_comp").count(), 2);
    assert_eq!(pairs.find_tagged("one_array").count(), 2);
}

#[test]
#[cfg(feature = "grammar-extras")]
fn test_implicit_whitespace_multitag() {
    let successful_parse = TestImplicitParser::parse(Rule::program, "a a a");
    assert!(successful_parse.is_ok());
    let pairs = successful_parse.unwrap();
    assert_eq!(pairs.clone().find_tagged("tail").count(), 2);
}
