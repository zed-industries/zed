extern crate test_crate;
use test_crate::*;

enum NeedDefault {
    A,
    B
}

dummy!(need_default);

fn main() {
    let _ = NeedDefault::default();
}
