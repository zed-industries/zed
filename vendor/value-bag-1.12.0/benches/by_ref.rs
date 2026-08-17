#![feature(test)]

extern crate test;

use value_bag::ValueBag;

#[bench]
fn u8_by_ref(b: &mut test::Bencher) {
    let v = ValueBag::from(1u8);
    b.iter(|| v.by_ref())
}
