extern crate test_crate;
use test_crate::*;

result_unwrap_or_abort!(one, two);
result_expect_or_abort!(one, two);

fn main() {}
