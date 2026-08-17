#![cfg(feature = "derive")]

use arbitrary::{Arbitrary, Unstructured};

fn arbitrary_from<'a, T: Arbitrary<'a>>(input: &'a [u8]) -> T {
    let mut buf = Unstructured::new(input);
    T::arbitrary(&mut buf).expect("can create arbitrary instance OK")
}

/// This wrapper trait *implies* `Arbitrary`, but the compiler isn't smart enough to work that out
/// so when using this wrapper we *must* opt-out of the auto-generated `T: Arbitrary` bounds.
pub trait WrapperTrait: for<'a> Arbitrary<'a> {}

impl WrapperTrait for u32 {}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: WrapperTrait")]
struct GenericSingleBound<T: WrapperTrait> {
    t: T,
}

#[test]
fn single_bound() {
    let v: GenericSingleBound<u32> = arbitrary_from(&[0, 0, 0, 0]);
    assert_eq!(v.t, 0);
}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: WrapperTrait, U: WrapperTrait")]
struct GenericMultipleBoundsSingleAttribute<T: WrapperTrait, U: WrapperTrait> {
    t: T,
    u: U,
}

#[test]
fn multiple_bounds_single_attribute() {
    let v: GenericMultipleBoundsSingleAttribute<u32, u32> =
        arbitrary_from(&[1, 0, 0, 0, 2, 0, 0, 0]);
    assert_eq!(v.t, 1);
    assert_eq!(v.u, 2);
}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: WrapperTrait")]
#[arbitrary(bound = "U: Default")]
struct GenericMultipleArbitraryAttributes<T: WrapperTrait, U: Default> {
    t: T,
    #[arbitrary(default)]
    u: U,
}

#[test]
fn multiple_arbitrary_attributes() {
    let v: GenericMultipleArbitraryAttributes<u32, u32> = arbitrary_from(&[1, 0, 0, 0]);
    assert_eq!(v.t, 1);
    assert_eq!(v.u, 0);
}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: WrapperTrait", bound = "U: Default")]
struct GenericMultipleBoundAttributes<T: WrapperTrait, U: Default> {
    t: T,
    #[arbitrary(default)]
    u: U,
}

#[test]
fn multiple_bound_attributes() {
    let v: GenericMultipleBoundAttributes<u32, u32> = arbitrary_from(&[1, 0, 0, 0]);
    assert_eq!(v.t, 1);
    assert_eq!(v.u, 0);
}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: WrapperTrait", bound = "U: Default")]
#[arbitrary(bound = "V: WrapperTrait, W: Default")]
struct GenericMultipleArbitraryAndBoundAttributes<
    T: WrapperTrait,
    U: Default,
    V: WrapperTrait,
    W: Default,
> {
    t: T,
    #[arbitrary(default)]
    u: U,
    v: V,
    #[arbitrary(default)]
    w: W,
}

#[test]
fn multiple_arbitrary_and_bound_attributes() {
    let v: GenericMultipleArbitraryAndBoundAttributes<u32, u32, u32, u32> =
        arbitrary_from(&[1, 0, 0, 0, 2, 0, 0, 0]);
    assert_eq!(v.t, 1);
    assert_eq!(v.u, 0);
    assert_eq!(v.v, 2);
    assert_eq!(v.w, 0);
}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: Default")]
struct GenericDefault<T: Default> {
    #[arbitrary(default)]
    x: T,
}

#[test]
fn default_bound() {
    // We can write a generic func without any `Arbitrary` bound.
    fn generic_default<T: Default>() -> GenericDefault<T> {
        arbitrary_from(&[])
    }

    assert_eq!(generic_default::<u64>().x, 0);
    assert_eq!(generic_default::<String>().x, String::new());
    assert_eq!(generic_default::<Vec<u8>>().x, Vec::new());
}

#[derive(Arbitrary)]
#[arbitrary()]
struct EmptyArbitraryAttribute {
    t: u32,
}

#[test]
fn empty_arbitrary_attribute() {
    let v: EmptyArbitraryAttribute = arbitrary_from(&[1, 0, 0, 0]);
    assert_eq!(v.t, 1);
}

#[derive(Arbitrary)]
#[arbitrary(bound = "")]
struct EmptyBoundAttribute {
    t: u32,
}

#[test]
fn empty_bound_attribute() {
    let v: EmptyBoundAttribute = arbitrary_from(&[1, 0, 0, 0]);
    assert_eq!(v.t, 1);
}
