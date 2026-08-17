#[cfg(feature = "decimal")]
use decimal::d128;
use num::Zero;
use num_complex::Complex;

/// Nested sets and conversions between them (using an injective mapping).
///
/// Useful to work with substructures. In generic code, it is preferable to use `SupersetOf`
/// as trait bound whenever possible instead of `SubsetOf` (because `SupersetOf` is automatically
/// implemented whenever `SubsetOf` is).
///
/// The notion of "nested sets" is very broad and applies to what the types are _supposed to
/// represent_, independently from their actual implementation details and limitations. For
/// example:
/// * f32 and f64 are both supposed to represent reals and are thus considered equal (even if in
///   practice f64 has more elements).
/// * u32 and i8 are respectively supposed to represent natural and relative numbers. Thus, u32 is
///   a subset of i8.
/// * A quaternion and a 3x3 orthogonal matrix with unit determinant are both sets of rotations.
///   They can thus be considered equal.
///
/// In other words, implementation details due to machine limitations are ignored (otherwise we
/// could not even, e.g., convert a u64 to an i64). If considering those limitations are
/// important, other crates allowing you to query the limitations of given types should be used.
pub trait SubsetOf<T>: Sized {
    /// The inclusion map: converts `self` to the equivalent element of its superset.
    fn to_superset(&self) -> T;

    /// The inverse inclusion map: attempts to construct `self` from the equivalent element of its
    /// superset.
    ///
    /// Must return `None` if `element` has no equivalent in `Self`.
    fn from_superset(element: &T) -> Option<Self> {
        if Self::is_in_subset(element) {
            Some(Self::from_superset_unchecked(element))
        } else {
            None
        }
    }

    /// Use with care! Same as `self.to_superset` but without any property checks. Always succeeds.
    fn from_superset_unchecked(element: &T) -> Self;

    /// Checks if `element` is actually part of the subset `Self` (and can be converted to it).
    fn is_in_subset(element: &T) -> bool;
}

/// Nested sets and conversions between them.
///
/// Useful to work with substructures. It is preferable to implement the `SubsetOf` trait instead
/// of `SupersetOf` whenever possible (because `SupersetOf` is automatically implemented whenever
/// `SubsetOf` is).
///
/// The notion of "nested sets" is very broad and applies to what the types are _supposed to
/// represent_, independently from their actual implementation details and limitations. For
/// example:
/// * f32 and f64 are both supposed to represent reals and are thus considered equal (even if in
///   practice f64 has more elements).
/// * u32 and i8 are respectively supposed to represent natural and relative numbers. Thus, i8 is
///   a superset of u32.
/// * A quaternion and a 3x3 orthogonal matrix with unit determinant are both sets of rotations.
///   They can thus be considered equal.
///
/// In other words, implementation details due to machine limitations are ignored (otherwise we
/// could not even, e.g., convert a u64 to an i64). If considering those limitations are
/// important, other crates allowing you to query the limitations of given types should be used.
pub trait SupersetOf<T>: Sized {
    /// The inverse inclusion map: attempts to construct `self` from the equivalent element of its
    /// superset.
    ///
    /// Must return `None` if `element` has no equivalent in `Self`.
    fn to_subset(&self) -> Option<T> {
        if self.is_in_subset() {
            Some(self.to_subset_unchecked())
        } else {
            None
        }
    }

    /// Checks if `self` is actually part of its subset `T` (and can be converted to it).
    fn is_in_subset(&self) -> bool;

    /// Use with care! Same as `self.to_subset` but without any property checks. Always succeeds.
    fn to_subset_unchecked(&self) -> T;

    /// The inclusion map: converts `self` to the equivalent element of its superset.
    fn from_subset(element: &T) -> Self;
}

impl<SS: SubsetOf<SP>, SP> SupersetOf<SS> for SP {
    #[inline]
    fn to_subset(&self) -> Option<SS> {
        SS::from_superset(self)
    }

    #[inline]
    fn is_in_subset(&self) -> bool {
        SS::is_in_subset(self)
    }

    #[inline]
    fn to_subset_unchecked(&self) -> SS {
        SS::from_superset_unchecked(self)
    }

    #[inline]
    fn from_subset(element: &SS) -> Self {
        element.to_superset()
    }
}

macro_rules! impl_subset (
    ($($subset: ty as $( $superset: ty),+ );* $(;)*) => {
        $($(
        impl SubsetOf<$superset> for $subset {
            #[inline]
            fn to_superset(&self) -> $superset {
                *self as $superset
            }

            #[inline]
            fn from_superset_unchecked(element: &$superset) -> $subset {
                *element as $subset
            }

            #[inline]
            fn is_in_subset(_: &$superset) -> bool {
                true
            }
        }
        )+)*
    }
);

impl_subset!(
    u8    as u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    u16   as u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    u32   as u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    u64   as u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    u128  as u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;
    usize as u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64;

    i8    as i8, i16, i32, i64, i128, isize, f32, f64;
    i16   as i8, i16, i32, i64, i128, isize, f32, f64;
    i32   as i8, i16, i32, i64, i128, isize, f32, f64;
    i64   as i8, i16, i32, i64, i128, isize, f32, f64;
    i128  as i8, i16, i32, i64, i128, isize, f32, f64;
    isize as i8, i16, i32, i64, i128, isize, f32, f64;

    f32 as f32, f64;
    f64 as f32, f64;
);
//#[cfg(feature = "decimal")]
//impl_subset!(
//    u8 as d128;
//    u16 as d128;
//    u32 as d128;
//    u64 as d128;
//    usize as d128;
//
//    i8 as d128;
//    i16 as d128;
//    i32 as d128;
//    i64 as d128;
//    isize as d128;
//
//    f32 as d128;
//    f64 as d128;
//    d128 as d128;
//);

impl<N1, N2: SupersetOf<N1>> SubsetOf<Complex<N2>> for Complex<N1> {
    #[inline]
    fn to_superset(&self) -> Complex<N2> {
        Complex {
            re: N2::from_subset(&self.re),
            im: N2::from_subset(&self.im),
        }
    }

    #[inline]
    fn from_superset_unchecked(element: &Complex<N2>) -> Complex<N1> {
        Complex {
            re: element.re.to_subset_unchecked(),
            im: element.im.to_subset_unchecked(),
        }
    }

    #[inline]
    fn is_in_subset(c: &Complex<N2>) -> bool {
        c.re.is_in_subset() && c.im.is_in_subset()
    }
}

macro_rules! impl_scalar_subset_of_complex (
    ($($t: ident),*) => {$(
        impl<N2: Zero + SupersetOf<$t>> SubsetOf<Complex<N2>> for $t {
            #[inline]
            fn to_superset(&self) -> Complex<N2> {
                Complex {
                    re: N2::from_subset(self),
                    im: N2::zero()
                }
            }

            #[inline]
            fn from_superset_unchecked(element: &Complex<N2>) -> $t {
                element.re.to_subset_unchecked()
            }

            #[inline]
            fn is_in_subset(c: &Complex<N2>) -> bool {
                c.re.is_in_subset() && c.im.is_zero()
            }
        }
    )*}
);

impl_scalar_subset_of_complex!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
#[cfg(feature = "decimal")]
impl_scalar_subset_of_complex!(d128);
