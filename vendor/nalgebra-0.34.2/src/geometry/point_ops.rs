use num::{One, Zero};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use simba::scalar::{
    ClosedAddAssign, ClosedDivAssign, ClosedMulAssign, ClosedNeg, ClosedSubAssign,
};

use crate::base::constraint::{
    AreMultipliable, SameNumberOfColumns, SameNumberOfRows, ShapeConstraint,
};
use crate::base::dimension::{Dim, DimName, U1};
use crate::base::storage::Storage;
use crate::base::{Const, Matrix, OVector, Scalar, Vector};

use crate::DefaultAllocator;
use crate::allocator::Allocator;
use crate::geometry::{OPoint, Point};

/*
 *
 * Indexing.
 *
 */
impl<T: Scalar, D: DimName> Index<usize> for OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    type Output = T;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.coords[i]
    }
}

impl<T: Scalar, D: DimName> IndexMut<usize> for OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.coords[i]
    }
}

/*
 *
 * Neg.
 *
 */
impl<T: Scalar + ClosedNeg, D: DimName> Neg for OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::from(-self.coords)
    }
}

impl<T: Scalar + ClosedNeg, D: DimName> Neg for &OPoint<T, D>
where
    DefaultAllocator: Allocator<D>,
{
    type Output = OPoint<T, D>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::from(-&self.coords)
    }
}

/*
 *
 * Subtraction & Addition.
 *
 */

// Point - Point
add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D, U1), (D, U1) -> (D, U1)
    const; for D; where D: DimName, DefaultAllocator: Allocator<D>;
    self: &'a OPoint<T, D>, right: &'b OPoint<T, D>, Output = OVector<T, D>;
    &self.coords - &right.coords; 'a, 'b);

add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D, U1), (D, U1) -> (D, U1)
    const; for D; where D: DimName, DefaultAllocator: Allocator<D>;
    self: &'a OPoint<T, D>, right: OPoint<T, D>, Output = OVector<T, D>;
    &self.coords - right.coords; 'a);

add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D, U1), (D, U1) -> (D, U1)
    const; for D; where D: DimName, DefaultAllocator: Allocator<D>;
    self: OPoint<T, D>, right: &'b OPoint<T, D>, Output = OVector<T, D>;
    self.coords - &right.coords; 'b);

add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D, U1), (D, U1) -> (D, U1)
    const; for D; where D: DimName, DefaultAllocator: Allocator<D>;
    self: OPoint<T, D>, right: OPoint<T, D>, Output = OVector<T, D>;
    self.coords - right.coords; );

// Point - Vector
add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: &'a OPoint<T, D1>, right: &'b Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(&self.coords - right); 'a, 'b);

add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: &'a OPoint<T, D1>, right: Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(&self.coords - &right); 'a); // TODO: should not be a ref to `right`.

add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: OPoint<T, D1>, right: &'b Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(self.coords - right); 'b);

add_sub_impl!(Sub, sub, ClosedSubAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: OPoint<T, D1>, right: Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(self.coords - right); );

// Point + Vector
add_sub_impl!(Add, add, ClosedAddAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: &'a OPoint<T, D1>, right: &'b Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(&self.coords + right); 'a, 'b);

add_sub_impl!(Add, add, ClosedAddAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: &'a OPoint<T, D1>, right: Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(&self.coords + &right); 'a); // TODO: should not be a ref to `right`.

add_sub_impl!(Add, add, ClosedAddAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: OPoint<T, D1>, right: &'b Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(self.coords + right); 'b);

add_sub_impl!(Add, add, ClosedAddAssign;
    (D1, U1), (D2, U1) -> (D1, U1)
    const;
    for D1, D2, SB;
    where D1: DimName, D2: Dim, SB: Storage<T, D2>, DefaultAllocator: Allocator<D1>;
    self: OPoint<T, D1>, right: Vector<T, D2, SB>, Output = OPoint<T, D1>;
    Self::Output::from(self.coords + right); );

// TODO: replace by the shared macro: add_sub_assign_impl?
macro_rules! op_assign_impl(
    ($($TraitAssign: ident, $method_assign: ident, $bound: ident);* $(;)*) => {$(
        impl<'b, T, D1: DimName, D2: Dim, SB> $TraitAssign<&'b Vector<T, D2, SB>> for OPoint<T, D1>
            where T: Scalar + $bound,
                  SB: Storage<T, D2>,
                  ShapeConstraint: SameNumberOfRows<D1, D2>,
                  DefaultAllocator: Allocator<D1> {

            #[inline]
            fn $method_assign(&mut self, right: &'b Vector<T, D2, SB>) {
                self.coords.$method_assign(right)
            }
        }

        impl<T, D1: DimName, D2: Dim, SB> $TraitAssign<Vector<T, D2, SB>> for OPoint<T, D1>
            where T: Scalar + $bound,
                  SB: Storage<T, D2>,
                  ShapeConstraint: SameNumberOfRows<D1, D2>,
                  DefaultAllocator: Allocator<D1> {

            #[inline]
            fn $method_assign(&mut self, right: Vector<T, D2, SB>) {
                self.coords.$method_assign(right)
            }
        }
    )*}
);

op_assign_impl!(
    AddAssign, add_assign, ClosedAddAssign;
    SubAssign, sub_assign, ClosedSubAssign;
);

/*
 *
 * Matrix × Point
 *
 */
md_impl_all!(
    Mul, mul;
    (Const<R1>, Const<C1>), (Const<D2>, U1)
    const D2, R1, C1;
    for SA;
    where SA: Storage<T, Const<R1>, Const<C1>>,
          ShapeConstraint: AreMultipliable<Const<R1>, Const<C1>, Const<D2>, U1>;
    self: Matrix<T, Const<R1>, Const<C1>, SA>, right: Point<T, D2>, Output = Point<T, R1>;
    [val val] => Point::from(self * right.coords);
    [ref val] => Point::from(self * right.coords);
    [val ref] => Point::from(self * &right.coords);
    [ref ref] => Point::from(self * &right.coords);
);

/*
 *
 * Point ×/÷ Scalar
 *
 */
macro_rules! componentwise_scalarop_impl(
    ($Trait: ident, $method: ident, $bound: ident;
     $TraitAssign: ident, $method_assign: ident) => {
        impl<T: Scalar + $bound, D: DimName> $Trait<T> for OPoint<T, D>
        where DefaultAllocator: Allocator<D>
        {
            type Output = OPoint<T, D>;

            #[inline]
            fn $method(self, right: T) -> Self::Output {
                OPoint::from(self.coords.$method(right))
            }
        }

        impl<'a, T: Scalar + $bound, D: DimName> $Trait<T> for &'a OPoint<T, D>
        where DefaultAllocator: Allocator<D>
        {
            type Output = OPoint<T, D>;

            #[inline]
            fn $method(self, right: T) -> Self::Output {
                OPoint::from((&self.coords).$method(right))
            }
        }

        impl<T: Scalar + $bound, D: DimName> $TraitAssign<T> for OPoint<T, D>
            where DefaultAllocator: Allocator<D>
        {
            #[inline]
            fn $method_assign(&mut self, right: T) {
                self.coords.$method_assign(right)
            }
        }
    }
);

componentwise_scalarop_impl!(Mul, mul, ClosedMulAssign; MulAssign, mul_assign);
componentwise_scalarop_impl!(Div, div, ClosedDivAssign; DivAssign, div_assign);

macro_rules! left_scalar_mul_impl(
    ($($T: ty),* $(,)*) => {$(
        impl<D: DimName> Mul<OPoint<$T, D>> for $T
        where DefaultAllocator: Allocator<D>
        {
            type Output = OPoint<$T, D>;

            #[inline]
            fn mul(self, right: OPoint<$T, D>) -> Self::Output {
                OPoint::from(self * right.coords)
            }
        }

        impl<'b, D: DimName> Mul<&'b OPoint<$T, D>> for $T
        where DefaultAllocator: Allocator<D>
        {
            type Output = OPoint<$T, D>;

            #[inline]
            fn mul(self, right: &'b OPoint<$T, D>) -> Self::Output {
                OPoint::from(self * &right.coords)
            }
        }
    )*}
);

left_scalar_mul_impl!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
