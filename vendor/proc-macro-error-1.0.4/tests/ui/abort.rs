extern crate test_crate;
use test_crate::*;

abort_from!(one, two);
abort_to_string!(one, two);
abort_format!(one, two);
direct_abort!(one, two);
abort_notes!(one, two);
abort_call_site_test!(one, two);

fn main() {}
