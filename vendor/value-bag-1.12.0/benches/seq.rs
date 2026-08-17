#![cfg(feature = "seq")]
#![feature(test)]

extern crate test;

use value_bag::ValueBag;

#[cfg(feature = "serde1")]
#[bench]
fn serde1_to_seq_5(b: &mut test::Bencher) {
    let v = ValueBag::from_serde1(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    b.iter(|| v.to_f64_seq::<Vec<Option<f64>>>())
}

#[cfg(feature = "sval2")]
#[bench]
fn sval2_to_seq_5(b: &mut test::Bencher) {
    let v = ValueBag::from_sval2(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    b.iter(|| v.to_f64_seq::<Vec<Option<f64>>>())
}
