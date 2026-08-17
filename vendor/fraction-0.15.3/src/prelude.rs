//! Predefines some types for the most common use cases
//!
//! You should consider this module as a list of shortcuts, and not
//! as the list of only available types.
//! The actual workhorses are a part of the Public API and you are
//! encouraged to use them straightforwardly whenever you may
//! feel necessary.
//!
//! Long story short, you may compose your own types with these:
//! - [`GenericFraction`] for fractions
//! - [`GenericDecimal`] for decimals
//! - [`DynaInt`] integers on stack, but dynamically growing into heap when necessary
//!
//! [`GenericFraction`]: super::GenericFraction
//! [`GenericDecimal`]: super::GenericDecimal
//! [`DynaInt`]: super::dynaint

pub use super::fraction::GenericFraction;

#[cfg(feature = "with-bigint")]
pub use super::{BigInt, BigUint};

#[cfg(feature = "with-decimal")]
pub use super::decimal::GenericDecimal;

#[cfg(feature = "with-dynaint")]
pub use super::dynaint::DynaInt;

/// Fraction consisting from two `u64` numbers
///
/// Allows to keep and work with fractions on stack.
///
/// Be aware of possible stack overflows that might be caused by
/// exceeding `u64` limits in some math operations, which will make thread to panic.
///
/// # Examples
///
/// ```
/// use fraction::Fraction;
///
/// let first = Fraction::new (1u8, 2u8);
/// let second = Fraction::new (2u8, 8u8);
///
/// assert_eq! (first + second, Fraction::new (3u8, 4u8));
/// ```
pub type Fraction = GenericFraction<u64>;

/// Fraction consisting from two [`BigUint`] numbers
///
/// Allows to keep and work with fractions on heap.
///
/// BigUint number is based on heap and does not have any limits, which makes
/// BigFraction safe from stack overflows. However, it comes with a price of
/// making allocations on every math operation.
///
/// # Examples
///
/// ```
/// use fraction::BigFraction;
///
/// let first = BigFraction::new (2u8, 3u8);
/// let second = BigFraction::new (1u8, 6u8);
///
/// assert_eq! (first + second, BigFraction::new (5u8, 6u8));
/// ```
///
/// [`BigUint`]: https://docs.rs/num-bigint/*/num_bigint/
#[cfg(feature = "with-bigint")]
pub type BigFraction = GenericFraction<BigUint>;

/// Stack allocated, but dynamically growing into heap if necessary
///
/// Fraction using T as the base type for numerator and denominator  
/// Whenever possible keeps data in T, but on math overflows
/// automatically casts values into BigUint, which allocates memory on heap.
///
/// Allows to use fractions without memory allocations wherever possible.
/// For unexpectedly big values performs on heap and doesn't suffer from stack overflows.
/// Automatically uses T if an operation on two BigUint numbers produces the result
/// that may fit within T.
///
/// # Examples
///
/// ```
/// use fraction::{DynaFraction, BigUint};
///
/// type D = DynaFraction<u32>;
///
/// let max_u32 = u32::max_value();
///
/// let one = D::from (1u32);
/// let mut val = D::from (max_u32);
///
/// assert_eq!(  // check we still use u32 for the numerator
///     max_u32,
///     val.numer().unwrap().clone().unpack().ok().unwrap()
/// );
///
///
/// val += &one;
/// assert_eq!(  // BigUint allocated for the result instead of stack overflow on u32
///     BigUint::from(max_u32) + BigUint::from(1u8),
///     val.numer().unwrap().clone().unpack().err().unwrap()
/// );
///
///
/// val -= one;
/// assert_eq!(  // check we use u32 again
///     max_u32,
///     val.numer().unwrap().clone().unpack().ok().unwrap()
/// );
/// ```
#[cfg(all(feature = "with-bigint", feature = "with-dynaint"))]
pub type DynaFraction<T> = GenericFraction<DynaInt<T, BigUint>>;

/// Basic Decimal based on 2 u64 numbers and one u8 for precision.
/// Able to keep up to 19 digits in the number (including
/// both sides across the floating point).
///
/// # Examples
/// ```
/// use fraction::Decimal;
///
/// let one = Decimal::from(152.568);
/// let two = Decimal::from(328.76842);
///
/// assert_eq!(one + two, Decimal::from("481.33642"));
/// assert_eq!(two - one, Decimal::from("176.20042"));
/// assert_eq!(one * two, Decimal::from("50159.5403"));
/// assert_eq!(two / one, Decimal::from("2.15489"));
/// // the result takes the max precision (between 5 and 8 it goes with 8)
/// assert_eq!(two / one.set_precision(8), Decimal::from("2.15489761"))
/// ```
#[cfg(feature = "with-decimal")]
pub type Decimal = GenericDecimal<u64, u8>;

/// Heap allocated [`BigUint`] for numerics and `usize` for precision
///
/// # Examples
///
/// ```
/// use fraction::BigDecimal;
///
/// let one = BigDecimal::from(11.5);
/// let two = BigDecimal::from(30.5);
///
/// assert_eq!(one + two, BigDecimal::from(42));
/// ```
///
/// [`BigUint`]: https://docs.rs/num-bigint/*/num_bigint/
#[cfg(all(feature = "with-decimal", feature = "with-bigint"))]
pub type BigDecimal = GenericDecimal<BigUint, usize>;

/// Stack allocated, but dynamically growing into heap if necessary
///
/// Allows to use decimals without memory allocations wherever possible.
/// For unexpectedly big values performs on heap and doesn't suffer from stack overflows.
/// Automatically goes back onto T if an operation with [`BigUint`] numbers produces the result
/// that may fit within T.
///
/// # Examples
///
/// ```
/// use fraction::DynaDecimal;
///
/// type D = DynaDecimal<usize, u8>;
///
/// let d1 = D::from("0.462046206206402");
/// let d2 = D::from(12042002442022044usize);
///
/// let d3 = d2 / d1 * D::from(240);
///
/// assert_eq!(d3, D::from("6254960104129183747.885873163232639"));
/// ```
///
/// [`BigUint`]: https://docs.rs/num-bigint/*/num_bigint/
#[cfg(all(
    feature = "with-decimal",
    feature = "with-bigint",
    feature = "with-dynaint"
))]
pub type DynaDecimal<T, P> = GenericDecimal<DynaInt<T, BigUint>, P>;
