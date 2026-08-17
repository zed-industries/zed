#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{format, vec::Vec};

#[cfg(feature = "grammar-extras")]
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../tests/oneormore.pest"]
pub struct OneOrMoreParser;

#[test]
#[cfg(feature = "grammar-extras")]
pub fn test_one_or_more() {
    let result = OneOrMoreParser::parse(Rule::assign, "k <- b\n");
    assert!(result.is_ok());
    let mut pairs = result.unwrap();
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::assign);
    let mut inner = pair.into_inner();
    let lhs = inner.next().unwrap();
    assert_eq!(lhs.as_rule(), Rule::identifier);
    assert_eq!(lhs.as_str(), "k");
    let rhs = inner.next().unwrap();
    assert_eq!(rhs.as_rule(), Rule::identifier);
    assert_eq!(rhs.as_str(), "b");
}
