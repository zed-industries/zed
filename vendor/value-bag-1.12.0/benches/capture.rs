#![feature(test)]

extern crate test;

use value_bag::ValueBag;

#[bench]
fn u8_capture_from(b: &mut test::Bencher) {
    b.iter(|| ValueBag::from(1u8))
}

#[bench]
fn u8_capture_debug(b: &mut test::Bencher) {
    b.iter(|| ValueBag::capture_debug(&1u8))
}

#[bench]
fn str_capture_debug(b: &mut test::Bencher) {
    // Currently at the top of the linear list of types in `cast::primitive`
    b.iter(|| ValueBag::capture_debug(&"a string"))
}

#[bench]
fn bool_capture_debug(b: &mut test::Bencher) {
    // Currently at the bottom of the linear list of types in `cast::primitive`
    b.iter(|| ValueBag::capture_debug(&true))
}

#[bench]
fn custom_capture_debug(b: &mut test::Bencher) {
    #[derive(Debug)]
    struct A;

    b.iter(|| ValueBag::capture_debug(&A))
}

#[bench]
fn fill_debug(b: &mut test::Bencher) {
    b.iter(|| {
        ValueBag::from_fill(&|slot: value_bag::fill::Slot| {
            #[derive(Debug)]
            struct A;

            slot.fill_debug(&A)
        })
    })
}

#[bench]
fn u8_capture_from_to_u64(b: &mut test::Bencher) {
    let v = ValueBag::from(1u8);
    b.iter(|| v.to_u64())
}

#[bench]
fn u8_capture_debug_to_u64(b: &mut test::Bencher) {
    let v = ValueBag::capture_debug(&1u8);
    b.iter(|| v.to_u64())
}

#[bench]
fn u8_fill_to_u64(b: &mut test::Bencher) {
    let v = ValueBag::from_fill(&|slot: value_bag::fill::Slot| slot.fill_any(1u8));

    b.iter(|| v.to_u64())
}

#[bench]
#[cfg(feature = "sval2")]
fn u8_from_sval_to_u64(b: &mut test::Bencher) {
    let v = ValueBag::from_sval2(&1u8);

    b.iter(|| v.to_u64())
}

#[bench]
#[cfg(feature = "sval2")]
fn u8_fill_sval_to_u64(b: &mut test::Bencher) {
    let v = ValueBag::from_fill(&|slot: value_bag::fill::Slot| slot.fill_sval2(&1u8));

    b.iter(|| v.to_u64())
}

#[bench]
fn u8_capture_debug_to_borrowed_str(b: &mut test::Bencher) {
    let v = ValueBag::capture_debug(&1u8);
    b.iter(|| v.to_borrowed_str())
}

#[bench]
fn str_capture_debug_to_borrowed_str(b: &mut test::Bencher) {
    let v = ValueBag::capture_debug(&"a string");
    b.iter(|| v.to_borrowed_str())
}

#[bench]
fn str_capture_debug_to_u64(b: &mut test::Bencher) {
    let v = ValueBag::capture_debug(&"a string");
    b.iter(|| v.to_u64())
}

#[bench]
fn custom_capture_debug_to_str(b: &mut test::Bencher) {
    #[derive(Debug)]
    struct A;

    let v = ValueBag::capture_debug(&A);
    b.iter(|| v.to_borrowed_str())
}
