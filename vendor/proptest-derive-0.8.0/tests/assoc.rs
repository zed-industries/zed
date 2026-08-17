// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use proptest::prelude::{prop_assert_eq, proptest, Arbitrary};
use proptest_derive::Arbitrary;

trait Func {
    type Out;
}
trait FuncA {
    type OutA: FuncB;
}
trait FuncB {
    type OutB;
}

#[derive(Debug)]
struct TypeA;

#[derive(Debug)]
struct TypeB;

#[derive(Debug, Arbitrary)]
struct OutTy {
    #[proptest(value = "42")]
    val: usize,
}

impl Func for TypeA {
    type Out = OutTy;
}
impl FuncA for TypeA {
    type OutA = TypeB;
}
impl FuncB for TypeB {
    type OutB = OutTy;
}

#[derive(Debug, Arbitrary)]
struct T0 {
    field: <TypeA as Func>::Out,
}

#[derive(Debug, Arbitrary)]
struct T1 {
    _field: Vec<u8>,
}

#[derive(Debug, Arbitrary)]
struct T2 {
    _field: Vec<Vec<u8>>,
}

#[derive(Debug, Arbitrary)]
struct T3 {
    field: Vec<<TypeA as Func>::Out>,
}

#[derive(Debug, Arbitrary)]
struct T4<Tyvar: FuncB> {
    field: Tyvar::OutB,
}

#[derive(Arbitrary)]
struct T5<Tyvar: FuncB> {
    field: <Tyvar>::OutB,
}

#[derive(Arbitrary)]
struct T6<Tyvar: FuncB> {
    field: <Tyvar as FuncB>::OutB,
}

#[derive(Arbitrary)]
struct T7<Tyvar: FuncA> {
    field: <Tyvar::OutA as FuncB>::OutB,
}

#[derive(Arbitrary)]
struct T8<Tyvar: FuncA> {
    field: <<Tyvar>::OutA as FuncB>::OutB,
}

#[derive(Arbitrary)]
struct T9<Tyvar: FuncA> {
    field: <<Tyvar as FuncA>::OutA as FuncB>::OutB,
}

#[derive(Debug, Arbitrary)]
struct T10<Tyvar: FuncB> {
    field: Vec<Tyvar::OutB>,
}

#[derive(Arbitrary)]
struct T11<Tyvar: FuncB> {
    field: Vec<<Tyvar>::OutB>,
}

#[derive(Arbitrary)]
struct T12<Tyvar: FuncB> {
    field: Vec<<Tyvar as FuncB>::OutB>,
}

#[derive(Arbitrary)]
struct T13<Tyvar: FuncA> {
    field: Vec<<Tyvar::OutA as FuncB>::OutB>,
}

#[derive(Arbitrary)]
struct T14<Tyvar: FuncA> {
    field: Vec<<<Tyvar>::OutA as FuncB>::OutB>,
}

#[derive(Arbitrary)]
struct T15<Tyvar: FuncA> {
    field: Vec<<<Tyvar as FuncA>::OutA as FuncB>::OutB>,
}

macro_rules! debug {
    ($trait: path, $ty: ident) => {
        impl<T: $trait> ::std::fmt::Debug for $ty<T> {
            fn fmt(
                &self,
                fmt: &mut ::std::fmt::Formatter,
            ) -> Result<(), ::std::fmt::Error> {
                fmt.debug_struct(stringify!($ty))
                    .field("field", &"<redacted>")
                    .finish()
            }
        }
    };
}

debug!(FuncB, T5);
debug!(FuncB, T6);
debug!(FuncA, T7);
debug!(FuncA, T8);
debug!(FuncA, T9);

debug!(FuncB, T11);
debug!(FuncB, T12);
debug!(FuncA, T13);
debug!(FuncA, T14);
debug!(FuncA, T15);

#[test]
fn asserting_arbitrary() {
    fn assert_arbitrary<T: Arbitrary>() {}

    assert_arbitrary::<T0>();
    assert_arbitrary::<T1>();
    assert_arbitrary::<T2>();
    assert_arbitrary::<T3>();

    assert_arbitrary::<T4<TypeB>>();
    assert_arbitrary::<T5<TypeB>>();
    assert_arbitrary::<T6<TypeB>>();
    assert_arbitrary::<T7<TypeA>>();
    assert_arbitrary::<T8<TypeA>>();
    assert_arbitrary::<T9<TypeA>>();

    assert_arbitrary::<T10<TypeB>>();
    assert_arbitrary::<T11<TypeB>>();
    assert_arbitrary::<T12<TypeB>>();
    assert_arbitrary::<T13<TypeA>>();
    assert_arbitrary::<T14<TypeA>>();
    assert_arbitrary::<T15<TypeA>>();
}

proptest! {
    #[test]
    fn t0_field_val_42(t: T0) {
        prop_assert_eq!(t.field.val, 42);
    }

    #[test]
    fn t1_no_panic(_: T1) {}

    #[test]
    fn t2_no_panic(_: T2) {}

    #[test]
    fn t3_all_42(t: T3) {
        t.field.iter().for_each(|x| assert_eq!(x.val, 42))
    }

    #[test]
    fn t4_field_val_42(t: T4<TypeB>) {
        prop_assert_eq!(t.field.val, 42);
    }

    #[test]
    fn t5_field_val_42(t: T5<TypeB>) {
        prop_assert_eq!(t.field.val, 42);
    }

    #[test]
    fn t6_field_val_42(t: T6<TypeB>) {
        prop_assert_eq!(t.field.val, 42);
    }

    #[test]
    fn t7_field_val_42(t: T7<TypeA>) {
        prop_assert_eq!(t.field.val, 42);
    }

    #[test]
    fn t8_field_val_42(t: T8<TypeA>) {
        prop_assert_eq!(t.field.val, 42);
    }

    #[test]
    fn t9_field_val_42(t: T9<TypeA>) {
        prop_assert_eq!(t.field.val, 42);
    }

    #[test]
    fn t10_all_42(t: T10<TypeB>) {
        t.field.iter().for_each(|x| assert_eq!(x.val, 42))
    }

    #[test]
    fn t11_all_42(t: T11<TypeB>) {
        t.field.iter().for_each(|x| assert_eq!(x.val, 42))
    }

    #[test]
    fn t12_all_42(t: T12<TypeB>) {
        t.field.iter().for_each(|x| assert_eq!(x.val, 42))
    }

    #[test]
    fn t13_all_42(t: T13<TypeA>) {
        t.field.iter().for_each(|x| assert_eq!(x.val, 42))
    }

    #[test]
    fn t14_all_42(t: T14<TypeA>) {
        t.field.iter().for_each(|x| assert_eq!(x.val, 42))
    }

    #[test]
    fn t15_all_42(t: T15<TypeA>) {
        t.field.iter().for_each(|x| assert_eq!(x.val, 42))
    }
}
