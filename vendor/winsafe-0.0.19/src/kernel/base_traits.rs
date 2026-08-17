#![allow(non_snake_case)]

use std::{fmt, hash, ops};

use crate::co;
use crate::decl::*;

/// A struct natively defined with its last element being an 1-element array.
/// The struct is dynamically allocated to accomodate a variable number of
/// elements, whose amount is stored in a specific field. The elements are
/// accessed by reaching the past-struct memory room.
///
/// Structs of this kind are managed through a [guard](crate::guard).
pub trait VariableSized {}

/// A type which has a primitive integer as its underlying type.
pub trait IntUnderlying: Sized
	+ Clone + Copy + PartialEq + Eq + hash::Hash + Send
	+ Into<Self::Raw> + AsRef<Self::Raw>
{
	/// The underlying raw integer type.
	type Raw;

	/// Returns a mutable reference to the underlying raw value.
	///
	/// # Safety
	///
	/// Be sure the integer being set is meaningful for the actual type.
	#[must_use]
	unsafe fn as_mut(&mut self) -> &mut Self::Raw;
}

/// A native typed constant.
///
/// If the values of this constant type can be combined as bitflags, it will
/// also implement the [`NativeBitflag`](crate::prelude::NativeBitflag) trait.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait NativeConst: IntUnderlying
	+ Default + fmt::Debug + fmt::Display
	+ fmt::LowerHex + fmt::UpperHex
	+ fmt::Binary + fmt::Octal
{}

/// A native typed bitflag constant.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait NativeBitflag: NativeConst
	+ ops::BitAnd + ops::BitAndAssign
	+ ops::BitOr + ops::BitOrAssign
	+ ops::BitXor + ops::BitXorAssign
	+ ops::Not
{
	/// Tells whether other bitflag style is present.
	///
	/// Equivalent to `(val & other) != 0`.
	#[must_use]
	fn has(&self, other: Self) -> bool;
}

/// A native typed constant mapped to a string.
pub trait NativeStrConst: PartialEq + Eq
	+ for<'a> TryFrom<&'a str, Error = co::ERROR>
	+ Into<WString>
	+ fmt::Debug + fmt::Display
{}

/// A system error which can be formatted with
/// [`FormatMessage`](crate::FormatMessage).
pub trait FormattedError: Into<u32> {
	/// Returns the textual description of the system error, by calling
	/// [`FormatMessage`](crate::FormatMessage).
	/// function.
	#[must_use]
	fn FormatMessage(self) -> String {
		let err_code: u32 = self.into();
		match unsafe {
			FormatMessage(
				co::FORMAT_MESSAGE::ALLOCATE_BUFFER
					| co::FORMAT_MESSAGE::FROM_SYSTEM
					| co::FORMAT_MESSAGE::IGNORE_INSERTS,
				None,
				err_code,
				LANGID::USER_DEFAULT,
				None,
			)
		} {
			Err(err_fmt) => format!( // never fails, returns a message instead
				"FormatMessage failed to format error {:#06x}: error {:#06x}.",
				err_code, err_fmt,
			),
			Ok(s) => s,
		}
	}
}
