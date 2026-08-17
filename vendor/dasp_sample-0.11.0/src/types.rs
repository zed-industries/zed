//! A collection of custom, non-std **Sample** types.

pub use self::i11::I11;
pub use self::i20::I20;
pub use self::i24::I24;
pub use self::i48::I48;
pub use self::u11::U11;
pub use self::u20::U20;
pub use self::u24::U24;
pub use self::u48::U48;

macro_rules! impl_from {
    ($T:ident: $Rep:ident from {$U:ident : $URep:ty}) => {
        impl From<$U> for $T {
            #[inline]
            fn from(other: $U) -> Self {
                $T(other.inner() as $Rep)
            }
        }
    };
    ($T:ident: $Rep:ident from $U:ident) => {
        impl From<$U> for $T {
            #[inline]
            fn from(other: $U) -> Self {
                $T(other as $Rep)
            }
        }
    };
}

macro_rules! impl_froms {
    ($T:ident: $Rep:ident, {$U:ident : $URep:ty}, $($rest:tt)*) => {
        impl_from!($T: $Rep from {$U: $URep});
        impl_froms!($T: $Rep, $($rest)*);
    };
    ($T:ident: $Rep:ident, {$U:ident : $URep:ty}) => {
        impl_from!($T: $Rep from {$U: $URep});
    };
    ($T:ident: $Rep:ident, $U:ident, $($rest:tt)*) => {
        impl_from!($T: $Rep from $U);
        impl_froms!($T: $Rep, $($rest)*);
    };
    ($T:ident: $Rep:ident, $U:ident) => {
        impl_from!($T: $Rep from $U);
    };
    ($T:ident: $Rep:ident,) => {};
}

macro_rules! impl_neg {
    ($T:ident) => {
        impl ::core::ops::Neg for $T {
            type Output = $T;
            #[inline]
            fn neg(self) -> $T {
                $T(-self.0)
            }
        }
    };
}

macro_rules! new_sample_type {
    ($T:ident: $Rep:ident, eq: $EQ:expr, min: $MIN:expr, max: $MAX:expr, total: $TOTAL:expr, from: $($rest:tt)*) => {
        pub const MIN: $T = $T($MIN);
        pub const MAX: $T = $T($MAX);
        pub const EQUILIBRIUM: $T = $T($EQ);
        const MIN_REP: $Rep = $MIN;
        const MAX_REP: $Rep = $MAX;
        const TOTAL: $Rep = $TOTAL;

        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
        pub struct $T($Rep);

        impl From<$Rep> for $T {
            #[inline]
            fn from(val: $Rep) -> Self {
                $T(val).wrap_overflow()
            }
        }

        impl $T {
            /// Construct a new sample if the given value is within range.
            ///
            /// Returns `None` if `val` is out of range.
            #[inline]
            pub fn new(val: $Rep) -> Option<Self> {
                if val > MAX_REP || val < MIN_REP {
                    None
                } else {
                    Some($T(val))
                }
            }

            /// Constructs a new sample without checking for overflowing.
            ///
            /// This should *only* be used if the user can guarantee the sample will be within
            /// range and they require the extra performance.
            ///
            /// If this function is used, the sample crate can't guarantee that the returned sample
            /// or any interacting samples will remain within their MIN and MAX bounds.
            pub fn new_unchecked(s: $Rep) -> Self {
                $T(s)
            }

            /// Return the internal value used to represent the sample type.
            #[inline]
            pub fn inner(self) -> $Rep {
                self.0
            }

            /// Wraps self once in the case that self has overflowed.
            #[inline]
            fn wrap_overflow_once(self) -> Self {
                if      self.0 > MAX_REP { $T(self.0 - TOTAL) }
                else if self.0 < MIN_REP { $T(self.0 + TOTAL) }
                else                     { self }
            }

            /// Wraps self in the case that self has overflowed.
            #[inline]
            fn wrap_overflow(mut self) -> Self {
                while self.0 > MAX_REP {
                    self.0 -= TOTAL;
                }
                while self.0 < MIN_REP {
                    self.0 += TOTAL;
                }
                self
            }
        }

        impl ::core::ops::Add<$T> for $T {
            type Output = $T;
            #[inline]
            fn add(self, other: Self) -> Self {
                if cfg!(debug_assertions) {
                    $T::new(self.0 + other.0).expect("arithmetic operation overflowed")
                } else {
                    $T(self.0 + other.0).wrap_overflow_once()
                }
            }
        }

        impl ::core::ops::Sub<$T> for $T {
            type Output = $T;
            #[inline]
            fn sub(self, other: Self) -> Self {
                if cfg!(debug_assertions) {
                    $T::new(self.0 - other.0).expect("arithmetic operation overflowed")
                } else {
                    $T(self.0 - other.0).wrap_overflow_once()
                }
            }
        }

        impl ::core::ops::Mul<$T> for $T {
            type Output = $T;
            #[inline]
            fn mul(self, other: Self) -> Self {
                if cfg!(debug_assertions) {
                    $T::new(self.0 * other.0).expect("arithmetic operation overflowed")
                } else {
                    $T::from(self.0 * other.0)
                }
            }
        }

        impl ::core::ops::Div<$T> for $T {
            type Output = $T;
            #[inline]
            fn div(self, other: Self) -> Self {
                $T(self.0 / other.0)
            }
        }

        impl ::core::ops::Not for $T {
            type Output = $T;
            #[inline]
            fn not(self) -> $T {
                $T(!self.0)
            }
        }

        impl ::core::ops::Rem<$T> for $T {
            type Output = $T;
            #[inline]
            fn rem(self, other: Self) -> Self {
                $T(self.0 % other.0)
            }
        }

        impl ::core::ops::Shl<$T> for $T {
            type Output = $T;
            #[inline]
            fn shl(self, other: Self) -> Self {
                // TODO: Needs review
                $T(self.0 << other.0)
            }
        }

        impl ::core::ops::Shr<$T> for $T {
            type Output = $T;
            #[inline]
            fn shr(self, other: Self) -> Self {
                // TODO: Needs review
                $T(self.0 >> other.0)
            }
        }

        impl ::core::ops::BitAnd<$T> for $T {
            type Output = $T;
            #[inline]
            fn bitand(self, other: Self) -> Self {
                $T(self.0 & other.0)
            }
        }

        impl ::core::ops::BitOr<$T> for $T {
            type Output = $T;
            #[inline]
            fn bitor(self, other: Self) -> Self {
                $T(self.0 | other.0)
            }
        }

        impl ::core::ops::BitXor<$T> for $T {
            type Output = $T;
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                $T(self.0 ^ other.0)
            }
        }

        impl_froms!($T: $Rep, $($rest)*);
    };
}

pub mod i11 {
    new_sample_type!(I11: i16, eq: 0, min: -1024, max: 1023, total: 2048,
                     from: i8, u8);
    impl_neg!(I11);
}

pub mod i20 {
    use super::{I11, U11};
    new_sample_type!(I20: i32, eq: 0, min: -524_288, max: 524_287, total: 1_048_576,
                     from: i8, {I11:i16}, i16, u8, {U11:i16}, u16);
}

pub mod i24 {
    use super::{I20, U20};
    new_sample_type!(I24: i32, eq: 0, min: -8_388_608, max: 8_388_607, total: 16_777_216,
                     from: i8, i16, {I20:i32}, u8, u16, {U20:i32});
    impl_neg!(I24);
}

pub mod i48 {
    use super::{I20, I24, U20, U24};
    new_sample_type!(I48: i64, eq: 0, min: -140_737_488_355_328, max: 140_737_488_355_327, total: 281_474_976_710_656,
                     from: i8, i16, {I20:i32}, {I24:i32}, i32, u8, u16, {U20:i32}, {U24:i32}, u32);
    impl_neg!(I48);
}

pub mod u11 {
    new_sample_type!(U11: i16, eq: 1024, min: 0, max: 2047, total: 2048,
                     from: u8);
    impl_neg!(U11);
}

pub mod u20 {
    new_sample_type!(U20: i32, eq: 524_288, min: 0, max: 1_048_575, total: 1_048_576,
                     from: u8, u16);
}

pub mod u24 {
    use super::U20;
    new_sample_type!(U24: i32, eq: 8_388_608, min: 0, max: 16_777_215, total: 16_777_216,
                     from: u8, u16, {U20:i32});
}

pub mod u48 {
    use super::{U20, U24};
    new_sample_type!(U48: i64, eq: 140_737_488_355_328, min: 0, max: 281_474_976_710_655, total: 281_474_976_710_656,
                     from: u8, u16, {U20:i32}, {U24:i32}, u32);
}
