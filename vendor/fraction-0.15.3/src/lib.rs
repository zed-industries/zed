#![doc(test(attr(deny(warnings), allow(deprecated))))]

//! Fraction is designed to be a precise lossless drop-in replacement for floating types (f32, f64).
//!
//! It comes with a number of predefined type aliases covering the most common use cases such as
//! [Fraction], [Decimal], [BigFraction], [DynaDecimal] and so on (see [prelude] module for more examples).
//!
//! The public API provides you with the generic types that you may use straightforwardly to build your
//! own types, suiting your needs best (see [prelude] module for the examples).
//!
//! # Library features
//!
//! - Drop in replacement for floats with the exception for NaN == NaN so that it's hashable
//! - It's hashable, so may be used as values in Sets and keys in dictionaries and hash maps
//! - [Display](fraction::display) implementation for fractions and decimals
//! - [Fraction](GenericFraction) type, representing fractions
//! - [Decimal](GenericDecimal) type, based on [Fraction](GenericFraction) type represents floats as lossless decimals
//! - [DynaInt](dynaint) implements dynamically growing integer type that perfarms checked math and avoids stack overflows
//! - PostgreSQL binary protocol integration for both fractions and decimals
//! - Juniper support for both fractions and decimals
//! - [Generic integer conversions](generic), such as `i8 -> u8`, `usize -> u8` and so on
//! - [Lossless division](division) with no allocations and infinite precision
//!
//! # Disclaimer
//! Even though we do our best to keep it well covered with tests, there may be bugs out there.
//! The library API is still in flux. When it gets stable we will release the version 1.0.0.
//! You may find more info about Semantic Versioning on [https://semver.org/](https://semver.org/).
//! Bug reports and contributions are appreciated.
//!
//! # Crate features
//! - `with-bigint` (default) integration with [num::BigInt] and [num::BigUint] data types
//! - `with-decimal` (default) [Decimal] type implemented upon [GenericFraction]
//! - `with-dynaint` (default) dynamically growing integer avoiding stack overflows
//! - `with-unicode` Unicode formatting and parsing options
//! - `with-approx` adds methods for approximate computations (currently `sqrt`)
//! - `with-juniper-support` [Juniper](https://crates.io/crates/juniper) integration
//! - `with-postgres-support` [PostgreSQL](https://crates.io/crates/postgres) integration; Numeric/Decimal type
//! - `with-serde-support` [Serde](https://crates.io/crates/serde) traits implementation
//!
//! # Implementation
//! Basic math implemented upon the [num] crate (in particular the [num::rational] module).
//! The utilised traits from the [num] crate are re-exported, so you don't have to explicitly depend on that crate however,
//! you may import them from either of crates if necessary.
//!
//! # Usage
//! To start using types see the [Prelude](self::prelude) module.
//!
//! # Examples
//!
//! ## Simple use:
//!
//! ```
//! type F = fraction::Fraction; // choose the type accordingly to your needs (see prelude module docs)
//!
//! let two = F::from(0) + F::from(2);   // 0 + 2 = 2
//! let two_third = two / F::from(3);    // 2/3 = 0.666666[...]
//!
//! assert_eq!(F::from(2), two);
//! assert_eq!(F::new(2u64, 3u64), two_third);
//!
//! assert_eq!("2/3", format!("{}", two_third));  // print as Fraction (by default)
//! assert_eq!("0.6666", format!("{:.4}", two_third));  // format as decimal and print up to 4 digits after floating point
//! ```
//!
//! Decimal is implemented as a representation layer on top of Fraction.
//! Thus, it is also lossless and may require explicit control over "precision"
//! for comparison and formatting operations.
//! ```
//! type D = fraction::Decimal;
//!
//! let result = D::from(0.5) / D::from(0.3);
//!
//! assert_eq!(format!("{}", result), "1.6"); // calculation result uses precision of the operands
//! assert_eq!(format!("{:.4}", result), "1.6666");  // explicitly passing precision to format
//!
//! assert_eq!("1.6666", format!("{}", result.set_precision(4))); // the other way to set precision explicitly on Decimal
//! ```
//!
//! ## Construct:
//!
//! Fraction:
//!
//! ```
//! use std::str::FromStr;
//! use fraction::{Fraction, Sign};
//!
//! // fraction crate also re-exports num::{One, Zero} traits for convenience.
//! use fraction::{One, Zero};
//!
//!
//! // There are several ways to construct a fraction, depending on your use case
//!
//! // `new` - construct with numerator/denominator and normalize the fraction.
//! // "Normalization" means it will always find the least common denominator
//! // and convert the input accordingly.
//! let f = Fraction::new(1u8, 2u8);
//!
//! // `new_generic` - construct with numerator/denominator of different integer types
//! assert_eq!(f, Fraction::new_generic(Sign::Plus, 1i32, 2u8).unwrap());
//!
//! // `from` - converts from primitive types such as i32 and f32.
//! assert_eq!(f, Fraction::from(0.5));  // convert from float (f32, f64)
//!
//! // `from_str` - tries parse a string fraction. Supports the usual decimal notation.
//! assert_eq!(f, Fraction::from_str("0.5").unwrap());  // parse a string
//!
//! // `from_str` - also supports _fraction_ notation such as "numerator/denominator" delimited by slash (`/`).
//! assert_eq!(f, Fraction::from_str("1/2").unwrap());  // parse a string
//!
//! // `new_raw` - construct with numerator/denominator but do not normalize the fraction.
//! // This is the most performant constructor, but does not calculate the common denominator,
//! // so may lead to unexpected results in following calculations if the fraction is not normalised.
//! // WARNING: Only use if you are sure numerator/denominator are already normalized.
//! assert_eq!(f, Fraction::new_raw(1u64, 2u64));
//!
//! // `one` - implements num::One trait
//! assert_eq!(f * 2, Fraction::one());
//!
//! // `zero` - implements num::Zero trait
//! assert_eq!(f - f, Fraction::zero());
//! ```
//!
//! Decimal:
//! ```
//! use std::str::FromStr;
//! use fraction::{Decimal, Fraction};
//!
//! // There are similar ways to construct Decimal. Underneath it is always represented as Fraction.
//! // When constructed, Decimal preserves its precision (number of digits after floating point).
//! // When two decimals are calculated, the result takes the biggest precision of both.
//! // The precision is used for visual representation (formatting and printing) and for comparison of two decimals.
//! // Precision is NOT used in any calculations. All calculations are lossless and implemented through Fraction.
//! // To override the precision use Decimal::set_precision.
//!
//! let d = Decimal::from(1);  // from integer, precision = 0
//! assert_eq!(d, Decimal::from_fraction(Fraction::from(1))); // from fraction, precision is calculated from fraction
//!
//! let d = Decimal::from(1.3);  // from float (f32, f64)
//! assert_eq!(d, Decimal::from_str("1.3").unwrap());
//!
//! let d = Decimal::from(0.5);  // from float (f32, f64)
//! assert_eq!(d, Decimal::from_str("1/2").unwrap());
//! ```
//!
//! ## Format (convert to string)
//! Formatting works similar for both Decimal and Fraction (Decimal uses Fraction internally).
//! The format implementation closely follows the rust Format trait documentation.
//!
//! ```
//! type F = fraction::Fraction;
//!
//! let result = F::from(0.7) / F::from(0.4);
//! assert_eq!(format!("{}", result), "7/4");  // Printed as fraction by default
//! assert_eq!(format!("{:.2}", result), "1.75"); // if precision is defined, printed as decimal
//! assert_eq!(format!("{:#.3}", result), "1.750"); // to print leading zeroes, pass hash to the format
//! ```
//!
//! Additionally, there are [methods](GenericFraction::get_unicode_display) available for various unicode display options:
//! See [this SO answer](https://stackoverflow.com/a/77861320/14681457) for a discussion.
//!
//! ```
//! type F = fraction::Fraction;
//!
//! let res = F::from(0.7) / F::from(0.4);
//! assert_eq!("7⁄4",format!("{}", res.get_unicode_display())); // needs font support. Unicode way
//! assert_eq!("1⁤3⁄4",format!("{}", res.get_unicode_display().mixed())); // interpreted wrongly without font support
//! assert_eq!("⁷/₄",format!("{}", res.get_unicode_display().supsub())); // no need for font support
//! assert_eq!("1³/₄",format!("{}", res.get_unicode_display().supsub().mixed()));
//! ```
//!
//! ## Convert into/from other types
//!
//! Both `fraction` and `decimal` types implement
//!  - `from` and `try_into` for all built-in primitive types.
//!  - `from` and `try_into` for `BigInt` and `BigUint` when `with-bigint` feature enabled.
//!
//! ```rust
//! use fraction::{Fraction, One, BigInt, BigUint};
//! use std::convert::TryInto;
//!
//! // Convert from examples (from primitives always succeed)
//! assert_eq!(Fraction::from(1i8), Fraction::one());
//! assert_eq!(Fraction::from(1u8), Fraction::one());
//! assert_eq!(Fraction::from(BigInt::one()), Fraction::one());
//! assert_eq!(Fraction::from(BigUint::one()), Fraction::one());
//! assert_eq!(Fraction::from(1f32), Fraction::one());
//! assert_eq!(Fraction::from(1f64), Fraction::one());
//!
//!
//! // Convert into examples (try_into returns Result<T, ()>)
//! assert_eq!(Ok(1i8), Fraction::one().try_into());
//! assert_eq!(Ok(1u8), Fraction::one().try_into());
//! assert_eq!(Ok(BigInt::one()), Fraction::one().try_into());
//! assert_eq!(Ok(BigUint::one()), Fraction::one().try_into());
//! assert_eq!(Ok(1f32), Fraction::one().try_into());
//! assert_eq!(Ok(1f64), Fraction::one().try_into());
//! ```
//!
//! ### Postgres usage
//! Postgres uses i16 for its binary protocol, so you'll have to use at least u16
//! as the base type for fractions/decimals.
//! Otherwise you may workaround with DynaInt<u8, _something_more_than_u8_>.
//! The safest way to go with would be DynaInt based types
//! such as DynaFraction or DynaDecimal as they would prevent
//! stack overflows for high values.
//!
//! Beware bad numbers such as 1/3, 1/7.
//! Fraction keeps the highest achievable precision (up to 16383 digits after floating point).
//! Decimal uses its own precision.
//! So, if you may end up with bad numbers, it may be preferable to go with Decimals over Fractions.
//!
//! Both types (fractions and decimals) should work transparently
//! in accordance with Postgres crate documentation

extern crate num;

#[cfg(feature = "with-bigint")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "with-bigint")]
pub use num::bigint::{BigInt, BigUint};

pub use num::rational::{ParseRatioError, Ratio};

pub use num::{
    traits::{ConstOne, ConstZero},
    Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Integer, Num, One,
    Signed, ToPrimitive, Zero,
};

#[cfg(test)]
#[macro_use]
mod tests;

pub mod convert;

pub mod division;

pub mod error;

mod fraction;
pub use fraction::*;

pub mod generic;

pub mod prelude;
pub use self::prelude::*;

// ====================================== FEATURES ======================================

#[cfg(feature = "with-juniper-support")]
extern crate juniper;
#[cfg(feature = "with-postgres-support")]
#[macro_use]
extern crate postgres_types;

#[cfg(feature = "with-serde-support")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "with-serde-support")]
extern crate serde;

#[cfg(feature = "with-decimal")]
mod decimal;
#[cfg(feature = "with-dynaint")]
pub mod dynaint;
