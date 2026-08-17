#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
#[non_exhaustive]
/// An error type that can describe atspi and `std` and different `zbus` errors.
pub enum AtspiError {
	/// Converting one type into another failure
	Conversion(&'static str),

	/// When testing on either variant, we might find the we are not interested in.
	CacheVariantMismatch,

	/// On specific types, if the event / message member does not match the Event's name.
	MemberMatch(String),

	/// On specific types, if the event / message member does not match the Event's name.
	InterfaceMatch(String),

	/// On specific types, if the kind (string variant) does not match the Event's kind.
	KindMatch(String),

	/// When an interface is not available.
	InterfaceNotAvailable(&'static str),

	/// To indicate a match or equality test on a signal body signature failed.
	SignatureMatch(String),

	/// When matching on an unknown interface
	UnknownInterface,

	/// No interface on event.
	MissingInterface,

	/// No member on event.
	MissingMember,

	/// No path on event.
	MissingPath,

	/// When matching on an unknown role
	UnknownRole(u32),

	/// No name on bus.
	MissingName,

	/// The signal that was encountered is unknown.
	UnknownSignal,

	/// Other errors.
	Owned(String),

	/// Null-reference error. This is used when an `ObjectRef` is expected to be non-null, but it is null.
	NullRef(&'static str),

	/// A `zbus` or `zbus::Fdo` error. variant.
	Zbus(String),

	/// A `zbus_names` error variant
	ZBusNames(zbus_names::Error),

	/// A `zbus_names` error variant
	Zvariant(zvariant::Error),

	/// Failed to parse a string into an enum variant
	ParseError(&'static str),

	/// Failed to get the ID of a path.
	PathConversionError(ObjectPathConversionError),

	/// Std i/o error variant.
	IO(std::io::Error),

	/// Failed to convert an integer into another type of integer (usually i32 -> usize).
	IntConversionError(std::num::TryFromIntError),

	/// An infallible error; this is just something to satisfy the compiler.
	Infallible,
}

impl std::error::Error for AtspiError {}

impl std::fmt::Display for AtspiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Conversion(e) => f.write_str(&format!("atspi: conversion failure: {e}")),
			Self::MemberMatch(e) => {
				f.write_str("atspi: member mismatch in conversion: ")?;
				e.fmt(f)
			}
			Self::InterfaceMatch(e) => {
				f.write_str("atspi: interface mismatch in conversion: ")?;
				e.fmt(f)
			}
			Self::KindMatch(e) => {
				f.write_str(format!("atspi: kind mismatch in conversion: {e}").as_str())
			}
			Self::SignatureMatch(e) => {
				f.write_str(format!("atspi: body signature mismatch in conversion: {e:?}").as_str())
			}
			Self::InterfaceNotAvailable(e) => {
				f.write_str(format!("atspi: interface not available: {e}").as_str())
			}
			Self::UnknownInterface => f.write_str("Unknown interface."),
			Self::MissingInterface => f.write_str("Missing interface."),
			Self::MissingMember => f.write_str("Missing member."),
			Self::MissingPath => f.write_str("Missing path."),
			Self::UnknownRole(e) => {
				f.write_str("atspi: Unknown role: ")?;
				e.fmt(f)
			}
			Self::UnknownSignal => f.write_str("atspi: Unknown signal"),
			Self::CacheVariantMismatch => f.write_str("atspi: Cache variant mismatch"),
			Self::Owned(e) => {
				f.write_str("atspi: other error: ")?;
				e.fmt(f)
			}
			Self::NullRef(e) => {
				f.write_str("atspi: null reference: ")?;
				f.write_str(e)
			}
			Self::Zbus(e) => {
				f.write_str("ZBus Error: ")?;
				e.fmt(f)
			}
			Self::Zvariant(e) => {
				f.write_str("Zvariant error: ")?;
				e.fmt(f)
			}
			Self::ZBusNames(e) => {
				f.write_str("ZBus_names Error: ")?;
				e.fmt(f)
			}
			Self::ParseError(e) => f.write_str(e),
			Self::PathConversionError(e) => {
				f.write_str("ID cannot be extracted from the path: ")?;
				e.fmt(f)
			}
			Self::IO(e) => {
				f.write_str("std IO Error: ")?;
				e.fmt(f)
			}
			Self::IntConversionError(e) => {
				f.write_str("Integer conversion error: ")?;
				e.fmt(f)
			}
			Self::MissingName => f.write_str("Missing name for a bus."),
			Self::Infallible => {
				f.write_str("Infallible; only to trick the compiler. This should never happen.")
			}
		}
	}
}

impl From<std::convert::Infallible> for AtspiError {
	fn from(_e: std::convert::Infallible) -> Self {
		Self::Infallible
	}
}
impl From<std::num::TryFromIntError> for AtspiError {
	fn from(e: std::num::TryFromIntError) -> Self {
		Self::IntConversionError(e)
	}
}

#[cfg(feature = "zbus")]
impl From<zbus::fdo::Error> for AtspiError {
	fn from(e: zbus::fdo::Error) -> Self {
		Self::Zbus(format!("{e:?}"))
	}
}

#[cfg(feature = "zbus")]
impl From<zbus::Error> for AtspiError {
	fn from(e: zbus::Error) -> Self {
		Self::Zbus(format!("{e:?}"))
	}
}

impl From<zbus_names::Error> for AtspiError {
	fn from(e: zbus_names::Error) -> Self {
		Self::ZBusNames(e)
	}
}

impl From<zvariant::Error> for AtspiError {
	fn from(e: zvariant::Error) -> Self {
		Self::Zvariant(e)
	}
}

impl From<std::io::Error> for AtspiError {
	fn from(e: std::io::Error) -> Self {
		Self::IO(e)
	}
}

impl From<ObjectPathConversionError> for AtspiError {
	fn from(e: ObjectPathConversionError) -> AtspiError {
		Self::PathConversionError(e)
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub enum ObjectPathConversionError {
	NoIdAvailable,
	ParseError(<i64 as std::str::FromStr>::Err),
}
impl std::fmt::Display for ObjectPathConversionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NoIdAvailable => f.write_str("No ID available in the path."),
			Self::ParseError(e) => {
				f.write_str("Failure to parse: ")?;
				e.fmt(f)
			}
		}
	}
}
impl std::error::Error for ObjectPathConversionError {}
