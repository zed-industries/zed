use std::ops::*;

use super::{MaybeOwned, MaybeOwnedMut};

macro_rules! impl_op {
    ($([$OP:ident : $op:ident, $OP_ASSIGN:ident : $op_assign: ident]),*) => ($(
        impl<'min, L, R, OUT: 'min> $OP<MaybeOwned<'min, R>> for MaybeOwned<'min, L>
            where L: $OP<R, Output=OUT> + $OP<&'min R, Output=OUT>,
                &'min L: $OP<R, Output=OUT> + $OP<&'min R, Output=OUT>
        {
            type Output = OUT;

            fn $op(self, rhs: MaybeOwned<'min, R>) -> Self::Output {
                use self::MaybeOwned::*;
                match (self, rhs) {
                    (Owned(l), Owned(r)) => l.$op(r),
                    (Owned(l), Borrowed(r)) => l.$op(r),
                    (Borrowed(l), Owned(r)) => l.$op(r),
                    (Borrowed(l), Borrowed(r)) => l.$op(r)
                }
            }
        }

        // Note: With an additional macro level we could fold this with the
        //       previous $OP implementation. But the additional read complexity
        //       isn't really worth it.
        impl<'min, L, R, OUT: 'min> $OP<MaybeOwnedMut<'min, R>> for MaybeOwnedMut<'min, L>
            where L: $OP<R, Output=OUT> + $OP<&'min R, Output=OUT>,
                &'min L: $OP<R, Output=OUT> + $OP<&'min R, Output=OUT>
        {
            type Output = OUT;

            fn $op(self, rhs: MaybeOwnedMut<'min, R>) -> Self::Output {
                use self::MaybeOwnedMut::*;
                match (self, rhs) {
                    (Owned(l), Owned(r)) => l.$op(r),
                    (Owned(l), Borrowed(r)) => l.$op(&*r),
                    (Borrowed(l), Owned(r)) => (&*l).$op(r),
                    (Borrowed(l), Borrowed(r)) => (&*l).$op(&*r)
                }
            }
        }

        impl<'min, L, R> $OP_ASSIGN<MaybeOwned<'min, R>> for MaybeOwned<'min, L>
            where L: Clone + $OP_ASSIGN<R> + $OP_ASSIGN<&'min R>
        {

            fn $op_assign(&mut self, rhs: MaybeOwned<'min, R>) {
                use self::MaybeOwned::*;
                match rhs {
                    Owned(r) => self.make_owned().$op_assign(r),
                    Borrowed(r) => self.make_owned().$op_assign(r)
                }
            }
        }

        impl<'min, L, R> $OP_ASSIGN<MaybeOwnedMut<'min, R>> for MaybeOwnedMut<'min, L>
            where L: $OP_ASSIGN<R> + $OP_ASSIGN<&'min R>
        {

            fn $op_assign(&mut self, rhs: MaybeOwnedMut<'min, R>) {
                use self::MaybeOwnedMut::*;
                match rhs {
                    Owned(r) => self.as_mut().$op_assign(r),
                    Borrowed(r) => self.as_mut().$op_assign(&*r)
                }
            }
        }
    )*);
}

impl_op! {
    [Add: add, AddAssign: add_assign],
    [Sub: sub, SubAssign: sub_assign],
    [Mul: mul, MulAssign: mul_assign],
    [Div: div, DivAssign: div_assign],
    [Shl: shl, ShlAssign: shl_assign],
    [Shr: shr, ShrAssign: shr_assign],
    [BitAnd: bitand, BitAndAssign: bitand_assign],
    [BitOr:  bitor,  BitOrAssign:  bitor_assign ],
    [BitXor: bitxor, BitXorAssign: bitxor_assign]
}

impl<'l, V, OUT> Neg for MaybeOwned<'l, V>
where
    V: Neg<Output = OUT>,
    &'l V: Neg<Output = OUT>,
{
    type Output = OUT;

    fn neg(self) -> Self::Output {
        use self::MaybeOwned::*;

        match self {
            Owned(s) => s.neg(),
            Borrowed(s) => s.neg(),
        }
    }
}

impl<'l, V, OUT> Neg for MaybeOwnedMut<'l, V>
where
    V: Neg<Output = OUT>,
    &'l V: Neg<Output = OUT>,
{
    type Output = OUT;

    fn neg(self) -> Self::Output {
        use self::MaybeOwnedMut::*;

        match self {
            Owned(s) => s.neg(),
            Borrowed(s) => (&*s).neg(),
        }
    }
}

impl<'l, V, OUT> Not for MaybeOwned<'l, V>
where
    V: Not<Output = OUT>,
    &'l V: Not<Output = OUT>,
{
    type Output = V::Output;

    fn not(self) -> Self::Output {
        use self::MaybeOwned::*;

        match self {
            Owned(s) => s.not(),
            Borrowed(s) => s.not(),
        }
    }
}

impl<'l, V, OUT> Not for MaybeOwnedMut<'l, V>
where
    V: Not<Output = OUT>,
    &'l V: Not<Output = OUT>,
{
    type Output = V::Output;

    fn not(self) -> Self::Output {
        use self::MaybeOwnedMut::*;

        match self {
            Owned(s) => s.not(),
            Borrowed(s) => (&*s).not(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::{Add, AddAssign, Neg, Not};

    //FIXME the test might need some cleanup.

    #[derive(Clone, PartialEq)]
    struct Think {
        x: u8,
    }

    impl Add<Think> for Think {
        type Output = u8;

        fn add(self, rhs: Think) -> Self::Output {
            self.x + rhs.x
        }
    }
    impl AddAssign<Think> for Think {
        fn add_assign(&mut self, rhs: Think) {
            self.x += rhs.x
        }
    }
    impl<'a> Add<&'a Think> for Think {
        type Output = u8;

        fn add(self, rhs: &'a Think) -> Self::Output {
            self.x + rhs.x
        }
    }
    impl<'a> AddAssign<&'a Think> for Think {
        fn add_assign(&mut self, rhs: &'a Think) {
            self.x += rhs.x
        }
    }
    impl<'a> Add<Think> for &'a Think {
        type Output = u8;

        fn add(self, rhs: Think) -> Self::Output {
            self.x + rhs.x
        }
    }
    impl<'a, 'b> Add<&'a Think> for &'b Think {
        type Output = u8;

        fn add(self, rhs: &'a Think) -> Self::Output {
            self.x + rhs.x
        }
    }

    impl Not for Think {
        type Output = bool;

        fn not(self) -> Self::Output {
            self.x != 0
        }
    }

    impl<'a> Not for &'a Think {
        type Output = bool;

        fn not(self) -> Self::Output {
            self.x != 0
        }
    }

    impl Neg for Think {
        type Output = i8;

        fn neg(self) -> Self::Output {
            -(self.x as i8)
        }
    }

    impl<'a> Neg for &'a Think {
        type Output = i8;

        fn neg(self) -> Self::Output {
            -(self.x as i8)
        }
    }

    #[test]
    fn op_impls_exist() {
        let a = MaybeOwned::from(Think { x: 12 });
        let b = MaybeOwned::from(Think { x: 13 });
        assert_eq!(a + b, 25u8);

        let c = Think { x: 42 };
        let c1: MaybeOwned<Think> = (&c).into();
        let c2: MaybeOwned<Think> = (&c).into();

        assert_eq!(c1 + c2, 84);
    }

    #[test]
    fn op_impls_exist_for_mut() {
        let a: MaybeOwnedMut<Think> = Think { x: 12 }.into();
        let b: MaybeOwnedMut<Think> = Think { x: 13 }.into();
        assert_eq!(a + b, 25u8);

        let mut c0a = Think { x: 42 };
        let mut c0b = Think { x: 8 };
        let c1: MaybeOwnedMut<Think> = (&mut c0a).into();
        let c2: MaybeOwnedMut<Think> = (&mut c0b).into();
        assert_eq!(c1 + c2, 50);
    }

    #[test]
    fn op_assign_impls_exist() {
        let mut a = MaybeOwned::from(Think { x: 2 });
        a += MaybeOwned::from(Think { x: 3 });
        assert_eq!(a.x, 5);

        let a = Think { x: 2 };
        let mut a: MaybeOwned<Think> = (&a).into();
        assert!(!a.is_owned());
        a += MaybeOwned::from(Think { x: 5 });
        assert!(a.is_owned());
        assert_eq!(a.as_ref().x, 7);
    }

    #[test]
    fn op_assign_impls_exist_mut() {
        let mut a: MaybeOwnedMut<Think> = Think { x: 2 }.into();
        a += MaybeOwnedMut::from(Think { x: 3 });
        assert_eq!(a.x, 5);

        let mut a = Think { x: 2 };
        let mut a: MaybeOwnedMut<Think> = (&mut a).into();
        assert!(!a.is_owned());
        a += MaybeOwnedMut::from(Think { x: 5 });
        assert!(!a.is_owned());
        assert_eq!(a.as_ref().x, 7);
    }

    #[test]
    fn not_and_neg_work_for_think_test_type() {
        assert_eq!(!Think { x: 0 }, false);
        assert_eq!(!Think { x: 1 }, true);
        assert_eq!(!&Think { x: 0 }, false);
        assert_eq!(!&Think { x: 1 }, true);
    }

    #[test]
    fn not_and_neg_are_impl() {
        let a = Think { x: 5 };
        let a1: MaybeOwned<Think> = (&a).into();
        let a2: MaybeOwned<Think> = (&a).into();
        assert_eq!(!a1, true);
        assert_eq!(-a2, -5i8);
    }

    #[test]
    fn not_and_neg_are_impl_mut() {
        let mut a = Think { x: 5 };
        let mut b = Think { x: 0 };
        let a1: MaybeOwnedMut<Think> = (&mut a).into();
        let b1: MaybeOwnedMut<Think> = (&mut b).into();

        assert_eq!(!a1, true);
        assert_eq!(!b1, false);

        let a2: MaybeOwnedMut<Think> = (&mut a).into();
        assert_eq!(-a2, -5i8);
    }
}
