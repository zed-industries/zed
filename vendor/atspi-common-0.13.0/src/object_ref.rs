use crate::AtspiError;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use zbus_lockstep_macros::validate;
use zbus_names::{BusName, UniqueName};
use zvariant::{ObjectPath, Structure, Type};

const NULL_PATH_STR: &str = "/org/a11y/atspi/null";
const NULL_OBJECT_PATH: &ObjectPath<'static> =
	&ObjectPath::from_static_str_unchecked(NULL_PATH_STR);

#[cfg(test)]
pub(crate) const TEST_OBJECT_BUS_NAME: &str = ":0.0";
#[cfg(test)]
pub(crate) const TEST_OBJECT_PATH_STR: &str = "/org/a11y/atspi/test/default";
#[cfg(test)]
pub(crate) const TEST_DEFAULT_OBJECT_REF: ObjectRef<'static> =
	ObjectRef::from_static_str_unchecked(TEST_OBJECT_BUS_NAME, TEST_OBJECT_PATH_STR);

/// A unique identifier for an object in the accessibility tree.
///
/// A ubiquitous type used to refer to an object in the accessibility tree.
///
/// In AT-SPI2, objects in the applications' UI object tree are uniquely identified
/// using an application's bus name and object path. "(so)"
///
/// Emitted by `RemoveAccessible` and `Available`
#[validate(signal: "Available")]
#[derive(Clone, Debug, Eq, Type)]
#[zvariant(signature = "(so)")]
pub enum ObjectRef<'o> {
	Null,
	Owned { name: UniqueName<'static>, path: ObjectPath<'static> },
	Borrowed { name: UniqueName<'o>, path: ObjectPath<'o> },
}

impl<'o> ObjectRef<'o> {
	/// Create a new `ObjectRef::Borrowed` from a `UniqueName` and `ObjectPath`.
	#[must_use]
	pub fn new(name: UniqueName<'o>, path: ObjectPath<'o>) -> Self {
		Self::new_borrowed(name, path)
	}

	/// Create a new, owned `ObjectRef`.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	///
	/// let object_ref = ObjectRef::new_owned(name, path);
	/// # assert_eq!(object_ref.name_as_str(), Some(":1.23"));
	/// # assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	pub fn new_owned<N, P>(name: N, path: P) -> ObjectRefOwned
	where
		N: Into<UniqueName<'static>>,
		P: Into<ObjectPath<'static>>,
	{
		let name: UniqueName<'static> = name.into();
		let path: ObjectPath<'static> = path.into();

		ObjectRefOwned(ObjectRef::Owned { name, path })
	}

	/// Create a new, borrowed `ObjectRef`.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	///
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	/// # assert_eq!(object_ref.name_as_str(), Some(":1.23"));
	/// # assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	pub fn new_borrowed<N, P>(name: N, path: P) -> ObjectRef<'o>
	where
		N: Into<UniqueName<'o>>,
		P: Into<ObjectPath<'o>>,
	{
		let name: UniqueName<'o> = name.into();
		let path: ObjectPath<'o> = path.into();

		ObjectRef::Borrowed { name, path }
	}

	/// Create a new `ObjectRef`, from `BusName` and `ObjectPath`.
	///
	/// # Errors
	/// Will fail if the `sender` is not a `UniqueName`.
	pub fn try_from_bus_name_and_path(
		sender: BusName<'o>,
		path: ObjectPath<'o>,
	) -> Result<Self, AtspiError> {
		// Check whether `BusName` matches `UniqueName`
		if let BusName::Unique(unique_sender) = sender {
			Ok(ObjectRef::new(unique_sender, path))
		} else {
			Err(AtspiError::ParseError("Expected UniqueName"))
		}
	}

	/// Create a new `ObjectRef`, unchecked.
	///
	/// # Safety
	/// The caller must ensure that the strings are valid.
	#[must_use]
	pub const fn from_static_str_unchecked(name: &'static str, path: &'static str) -> Self {
		let name = UniqueName::from_static_str_unchecked(name);
		let path = ObjectPath::from_static_str_unchecked(path);

		ObjectRef::Owned { name, path }
	}

	/// Returns `true` if the object reference is `Null`, otherwise returns `false`.
	///
	/// Toolkits may use the `Null` object reference to indicate that an object is not available or does not exist.
	/// For example, when calling `Accessible::get_parent` on an object that has no parent,
	/// it may return a `Null` object reference.
	#[must_use]
	pub fn is_null(&self) -> bool {
		matches!(self, Self::Null)
	}

	/// Returns the name of the object reference.
	/// If the object reference is `Null`, it returns `None`.
	/// If the object reference is `Owned` or `Borrowed`, it returns the name.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check the name of the object reference
	/// assert!(object_ref.name().is_some());
	/// assert_eq!(object_ref.name().unwrap().as_str(), ":1.23");
	/// ```
	#[must_use]
	pub fn name(&self) -> Option<&UniqueName<'o>> {
		match self {
			Self::Owned { name: own_name, .. } => Some(own_name),
			Self::Borrowed { name: borrow_name, .. } => Some(borrow_name),
			Self::Null => None,
		}
	}

	/// Returns the path of the object reference.\
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check the path of the object reference
	/// assert_eq!(object_ref.path().as_str(), "/org/a11y/example/path/007");
	/// ```
	#[must_use]
	pub fn path(&self) -> &ObjectPath<'o> {
		match self {
			Self::Owned { path: own_path, .. } => own_path,
			Self::Borrowed { path: borrow_path, .. } => borrow_path,
			Self::Null => NULL_OBJECT_PATH,
		}
	}

	/// Converts the `ObjectRef` into an owned instance, consuming `self`.\
	/// If the object reference is `Null`, it returns `ObjectRef::Null`.\
	/// If the object reference is `Owned`, it returns the same `ObjectRef::Owned`.\
	/// If the object reference is `Borrowed`, it converts the name and path to owned versions and returns `ObjectRef::Owned`.
	///
	/// # Extending lifetime 'magic' (from 'o -> 'static')
	///
	/// `ObjectRef<'_>` leans on the implementation of `UniqueName` and `ObjectPath` to
	/// convert the inner types to `'static`.
	/// These types have an `Inner` enum that can contain an `Owned`, `Borrowed`, or `Static` `Str` type.
	/// The `Str`type is either a `&'static str` (static), `&str` (borrowed), or an `Arc<str>` (owned).
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check whether the object reference can be converted to an owned version
	/// assert!(!object_ref.is_null());
	/// let object_ref = object_ref.into_owned();
	/// assert!(matches!(object_ref, ObjectRef::Owned { .. }));
	/// ```
	#[must_use]
	pub fn into_owned(self) -> ObjectRef<'static> {
		match self {
			Self::Null => ObjectRef::Null,
			Self::Owned { name, path } => ObjectRef::Owned { name, path },
			Self::Borrowed { name, path } => {
				ObjectRef::Owned { name: name.to_owned(), path: path.to_owned() }
			}
		}
	}

	/// Returns the name of the object reference as a string slice.
	#[must_use]
	pub fn name_as_str(&self) -> Option<&str> {
		match self {
			ObjectRef::Null => None,
			ObjectRef::Owned { name, .. } | ObjectRef::Borrowed { name, .. } => Some(name.as_str()),
		}
	}

	/// Returns the path of the object reference as a string slice.
	#[must_use]
	pub fn path_as_str(&self) -> &str {
		match self {
			ObjectRef::Null => NULL_PATH_STR,
			ObjectRef::Owned { path, .. } | ObjectRef::Borrowed { path, .. } => path.as_str(),
		}
	}
}

// Event tests lean on the `Default` implementation of `ObjectRef`.
// This is a workaround for the fact that `ObjectRef::Null` in
// `#[cfg(test)]` context is inconvenient.
// Events are guaranteed to have a non-null `ObjectRef` on their `item` field, because we receive signals over
// regular (non-p2p) DBus. Which means the `Message` `Header` has valid `Sender` and `Path` fields which
// are used to construct the `ObjectRef` from a `Message`.
#[cfg(test)]
impl Default for ObjectRef<'_> {
	/// Returns a non-Null object reference. (test implementation)
	fn default() -> Self {
		TEST_DEFAULT_OBJECT_REF
	}
}

#[cfg(not(test))]
impl Default for ObjectRef<'_> {
	/// Returns a `Null` object reference.
	fn default() -> Self {
		ObjectRef::Null
	}
}

/// A wrapper around the static variant of `ObjectRef`.
/// This is guaranteed to have a `'static` lifetime.
#[validate(signal: "Available")]
#[derive(Clone, Debug, Default, Eq, Type)]
pub struct ObjectRefOwned(pub(crate) ObjectRef<'static>);

impl From<ObjectRef<'_>> for ObjectRefOwned {
	/// Convert an `ObjectRef<'_>` into an `ObjectRefOwned`.
	///
	/// # Extending lifetime 'magic' (from 'o -> 'static')
	///
	/// `ObjectRef<'_>` leans on the implementation of `UniqueName` and `ObjectPath` to
	/// convert the inner types to `'static`.
	/// These types have an `Inner` enum that can contain an `Owned`, `Borrowed`, or `Static` `Str` type.
	/// The `Str`type is either a `&'static str` (static), `&str` (borrowed), or an `Arc<str>` (owned).
	fn from(object_ref: ObjectRef<'_>) -> Self {
		ObjectRefOwned(object_ref.into_owned())
	}
}

impl ObjectRefOwned {
	/// Create a new `ObjectRefOwned` from an `ObjectRef<'static>`.
	#[must_use]
	pub const fn new(object_ref: ObjectRef<'static>) -> Self {
		Self(object_ref)
	}

	/// Create a new `ObjectRefOwned` from `&'static str` unchecked.
	///
	/// # Safety
	/// The caller must ensure that the strings are valid.
	#[must_use]
	pub const fn from_static_str_unchecked(name: &'static str, path: &'static str) -> Self {
		let name = UniqueName::from_static_str_unchecked(name);
		let path = ObjectPath::from_static_str_unchecked(path);

		ObjectRefOwned(ObjectRef::Owned { name, path })
	}

	/// Returns `true` if the object reference is `Null`, otherwise returns `false`.
	#[must_use]
	pub fn is_null(&self) -> bool {
		matches!(self.0, ObjectRef::Null)
	}

	/// Returns the inner `ObjectRef`, consuming `self`.
	#[must_use]
	pub fn into_inner(self) -> ObjectRef<'static> {
		self.0
	}

	/// Returns the name of the object reference.
	/// If the object reference is `Null`, it returns `None`.
	/// If the object reference is `Owned` or `Borrowed`, it returns the name.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// // Check the name of the object reference
	/// assert!(object_ref.name().is_some());
	/// assert_eq!(object_ref.name_as_str().unwrap(), ":1.23");
	/// ```
	#[must_use]
	pub fn name(&self) -> Option<&UniqueName<'static>> {
		match &self.0 {
			ObjectRef::Owned { name, .. } | ObjectRef::Borrowed { name, .. } => Some(name),
			ObjectRef::Null => None,
		}
	}

	/// Returns the path of the object reference.\
	/// If the object reference is `Null`, it returns the null-path.
	///
	/// # Example
	/// ```rust
	/// use zbus::names::UniqueName;
	/// use zbus::zvariant::ObjectPath;
	/// use atspi_common::ObjectRef;
	///
	/// let name = UniqueName::from_static_str_unchecked(":1.23");
	/// let path = ObjectPath::from_static_str_unchecked("/org/a11y/example/path/007");
	/// let object_ref = ObjectRef::new_borrowed(name, path);
	///
	/// assert_eq!(object_ref.path_as_str(), "/org/a11y/example/path/007");
	/// ```
	#[must_use]
	pub fn path(&self) -> &ObjectPath<'static> {
		match &self.0 {
			ObjectRef::Owned { path, .. } | ObjectRef::Borrowed { path, .. } => path,
			ObjectRef::Null => NULL_OBJECT_PATH,
		}
	}

	/// Returns the name of the object reference as a string slice.
	#[must_use]
	pub fn name_as_str(&self) -> Option<&str> {
		match &self.0 {
			ObjectRef::Null => None,
			ObjectRef::Owned { name, .. } | ObjectRef::Borrowed { name, .. } => Some(name.as_str()),
		}
	}

	/// Returns the path of the object reference as a string slice.
	#[must_use]
	pub fn path_as_str(&self) -> &str {
		match &self.0 {
			ObjectRef::Null => NULL_PATH_STR,
			ObjectRef::Owned { path, .. } | ObjectRef::Borrowed { path, .. } => path.as_str(),
		}
	}
}

impl Serialize for ObjectRef<'_> {
	/// `ObjectRef`'s wire format is `(&str, ObjectPath)`.
	/// The `Null` variant, the "Null object", is serialized as `("", ObjectPath("/org/a11y/atspi/null"))`.
	/// Both `Owned` and `Borrowed` variants are serialized as `(&str, ObjectPath)` with the object's\
	/// unique name and path.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self {
			ObjectRef::Null => ("", NULL_OBJECT_PATH).serialize(serializer),
			ObjectRef::Owned { name, path } | ObjectRef::Borrowed { name, path } => {
				(name.as_str(), path).serialize(serializer)
			}
		}
	}
}

impl Serialize for ObjectRefOwned {
	/// `ObjectRefOwned` is serialized as the inner `ObjectRef`.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

impl<'de: 'o, 'o> Deserialize<'de> for ObjectRef<'o> {
	/// `ObjectRef`'s wire format is `(&str, ObjectPath)`.
	/// An empty `&str` with a "/org/a11y/atspi/null" path is considered a `Null` object,
	/// this is deserialized as `ObjectRef::Null`.\
	/// Any other valid `(&str, ObjectPath)`  will deserialize into `ObjectRef::Borrowed`.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct ObjectRefVisitor;

		impl<'de> serde::de::Visitor<'de> for ObjectRefVisitor {
			type Value = ObjectRef<'de>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a tuple of (&str, ObjectPath)")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let name: &str = seq
					.next_element()?
					.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
				let path: ObjectPath<'de> = seq
					.next_element()?
					.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

				if path == *NULL_OBJECT_PATH {
					Ok(ObjectRef::Null)
				} else {
					assert!(
						!name.is_empty(),
						"A non-null ObjectRef requires a name and a path but got: (\"\", {path})"
					);
					Ok(ObjectRef::Borrowed {
						name: UniqueName::try_from(name).map_err(serde::de::Error::custom)?,
						path,
					})
				}
			}
		}

		deserializer.deserialize_tuple(2, ObjectRefVisitor)
	}
}

impl<'de> Deserialize<'de> for ObjectRefOwned {
	/// `ObjectRefOwned` is deserialized as "Owned" variant `ObjectRef<'static>`.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let object_ref: ObjectRef<'_> = Deserialize::deserialize(deserializer)?;
		Ok(object_ref.into())
	}
}

impl PartialEq for ObjectRef<'_> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			// Match order is relevant here. Null == Null, but Null != any other object.
			(ObjectRef::Null, ObjectRef::Null) => true,
			(ObjectRef::Null, _) | (_, ObjectRef::Null) => false,
			_ => self.name() == other.name() && self.path() == other.path(),
		}
	}
}

// `Hash` requires that hashes are equal if values are equal.
// If a == b, then a.hash() == b.hash() must hold true.
//
// Because PartialEq treats Owned and Borrowed variants with identical (name, path) as equal,
// we must implement Hash manually to ignore the variant discriminant and preserve hash/equality
// consistency.
impl Hash for ObjectRef<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			ObjectRef::Null => {
				// Hashing a Null object reference.
				"Null".hash(state);
			}
			ObjectRef::Owned { name, path } | ObjectRef::Borrowed { name, path } => {
				name.as_str().hash(state);
				path.as_str().hash(state);
			}
		}
	}
}

impl Hash for ObjectRefOwned {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.hash(state);
	}
}

impl PartialEq for ObjectRefOwned {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl PartialEq<ObjectRef<'_>> for ObjectRefOwned {
	fn eq(&self, other: &ObjectRef<'_>) -> bool {
		self.0 == *other
	}
}

impl PartialEq<ObjectRefOwned> for ObjectRef<'_> {
	fn eq(&self, other: &ObjectRefOwned) -> bool {
		*self == other.0
	}
}

#[cfg(feature = "zbus")]
impl<'m: 'o, 'o> TryFrom<&'m zbus::message::Header<'_>> for ObjectRef<'o> {
	type Error = crate::AtspiError;

	// Construct an ObjectRef<'o> by reborrowing from the Header’s data.
	// 'm: 'o, 'm outlives 'o, so the references returned by this function
	// are guaranteed to be valid for the lifetime of the header.

	/// Construct an `ObjectRef` from a `zbus::message::Header`.
	///
	/// # Header fields
	///
	/// `Path` is a mandatory field on method calls and signals,
	/// `Sender` is an optional field, see:
	/// [DBus specification - header fields](<https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-header-fields>)).,
	///
	/// ```quote
	///  On a message bus, this header field is controlled by the message bus,
	///  so it is as reliable and trustworthy as the message bus itself.
	///  Otherwise, (eg. P2P) this header field is controlled by the message sender,
	///  unless there is out-of-band information that indicates otherwise.
	/// ```
	///
	/// While unlikely, it is possible that `Sender` or `Path` are not set on the header.
	/// This could happen if the server implementation does not set these fields for any reason.
	///
	/// # Errors
	/// Will return an `AtspiError::ParseError` if the header does not contain a valid path or sender.
	fn try_from(header: &'m zbus::message::Header) -> Result<Self, Self::Error> {
		let path = header.path().ok_or(crate::AtspiError::MissingPath)?;
		let name = header.sender().ok_or(crate::AtspiError::MissingName)?;

		Ok(ObjectRef::Borrowed { name: name.clone(), path: path.clone() })
	}
}

#[cfg(feature = "zbus")]
impl<'m> TryFrom<&'m zbus::message::Header<'_>> for ObjectRefOwned {
	type Error = crate::AtspiError;

	/// Construct an `ObjectRefOwned` from a `zbus::message::Header`.
	fn try_from(header: &'m zbus::message::Header) -> Result<Self, Self::Error> {
		let path = header.path().ok_or(crate::AtspiError::MissingPath)?;
		let name = header.sender().ok_or(crate::AtspiError::MissingName)?;

		let object_ref =
			ObjectRef::Owned { name: name.clone().into_owned(), path: path.clone().into_owned() };
		Ok(ObjectRefOwned(object_ref))
	}
}

impl<'v> TryFrom<zvariant::Value<'v>> for ObjectRef<'v> {
	type Error = zvariant::Error;

	fn try_from(value: zvariant::Value<'v>) -> Result<Self, Self::Error> {
		// Relies on the generic `Value` to tuple conversion `(UniqueName, ObjectPath)`.
		let (name, path): (UniqueName, ObjectPath) = value.try_into()?;
		Ok(ObjectRef::new_borrowed(name, path))
	}
}

impl<'v> TryFrom<zvariant::Value<'v>> for ObjectRefOwned {
	type Error = zvariant::Error;

	fn try_from(value: zvariant::Value<'v>) -> Result<Self, Self::Error> {
		// Relies on the generic `Value` to tuple conversion `(UniqueName, ObjectPath)`.
		let (name, path): (UniqueName, ObjectPath) = value.try_into()?;
		Ok(ObjectRef::new_borrowed(name, path).into())
	}
}

impl TryFrom<zvariant::OwnedValue> for ObjectRef<'static> {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
		let (name, path): (UniqueName<'static>, ObjectPath<'static>) = value.try_into()?;
		Ok(ObjectRef::Owned { name, path })
	}
}

impl TryFrom<zvariant::OwnedValue> for ObjectRefOwned {
	type Error = zvariant::Error;
	fn try_from(value: zvariant::OwnedValue) -> Result<Self, Self::Error> {
		let (name, path): (UniqueName<'static>, ObjectPath<'static>) = value.try_into()?;
		let obj = ObjectRef::Owned { name, path };
		Ok(ObjectRefOwned(obj))
	}
}

impl<'reference: 'structure, 'object: 'structure, 'structure> From<&'reference ObjectRef<'object>>
	for zvariant::Structure<'structure>
{
	fn from(obj: &'reference ObjectRef<'object>) -> Self {
		match obj {
			ObjectRef::Null => ("", NULL_OBJECT_PATH).into(),
			ObjectRef::Borrowed { name, path } => Structure::from((name.clone(), path)),
			ObjectRef::Owned { name, path } => Structure::from((name.as_str(), path.as_ref())),
		}
	}
}

impl<'o> From<ObjectRef<'o>> for zvariant::Structure<'o> {
	fn from(obj: ObjectRef<'o>) -> Self {
		match obj {
			ObjectRef::Null => Structure::from(("", NULL_OBJECT_PATH)),
			ObjectRef::Borrowed { name, path } | ObjectRef::Owned { name, path } => {
				Structure::from((name, path))
			}
		}
	}
}

impl From<ObjectRefOwned> for zvariant::Structure<'_> {
	fn from(obj: ObjectRefOwned) -> Self {
		let object_ref = obj.into_inner();
		object_ref.into()
	}
}

#[cfg(test)]
mod tests {
	use std::hash::{DefaultHasher, Hash, Hasher};

	use super::ObjectRef;
	use crate::object_ref::{NULL_OBJECT_PATH, NULL_PATH_STR};
	use zbus::zvariant;
	use zbus::{names::UniqueName, zvariant::ObjectPath};
	use zvariant::{serialized::Context, to_bytes, OwnedValue, Value, LE};

	const TEST_OBJECT_PATH: &str = "/org/a11y/atspi/path/007";

	#[test]
	fn owned_object_ref_creation() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref = ObjectRef::new_owned(name, path);

		assert_eq!(object_ref.name_as_str(), Some(":1.23"));
		assert_eq!(object_ref.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn borrowed_object_ref_creation() {
		let object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_eq!(object_ref.name_as_str(), Some(":1.23"));
		assert_eq!(object_ref.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn null_object_ref() {
		let null_object_ref: ObjectRef = ObjectRef::Null;
		assert!(null_object_ref.is_null());
		assert!(null_object_ref.name().is_none());
		assert_eq!(null_object_ref.path(), NULL_OBJECT_PATH);
	}

	#[test]
	fn object_ref_into_owned() {
		let borrowed_object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		let owned_object_ref = borrowed_object_ref.into_owned();
		assert!(matches!(owned_object_ref, ObjectRef::Owned { .. }));
		assert_eq!(owned_object_ref.name_as_str(), Some(":1.23"));
		assert_eq!(owned_object_ref.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn object_ref_into_name_and_path() {
		let object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		let name = object_ref.name().unwrap();
		let path = object_ref.path();
		assert_eq!(name.as_str(), ":1.23");
		assert_eq!(path.as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn serialization_null_object_ref() {
		let null_object_ref: ObjectRef = ObjectRef::Null;
		assert!(null_object_ref.is_null());

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &null_object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();

		assert!(obj.is_null());
		assert!(obj.name().is_none());
		assert_eq!(obj.path(), NULL_OBJECT_PATH);
	}

	#[test]
	fn serialization_owned_object_ref() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref = ObjectRef::new_owned(name, path);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();

		// Deserialization alwayys results in a borrowed object reference.
		// On the wire the distinction between owned and borrowed is not preserved.
		// As borrowed is the cheaper option, we always deserialize to that.
		assert!(matches!(obj, ObjectRef::Borrowed { .. }));
		assert_eq!(obj.name().unwrap().as_str(), ":1.23");
		assert_eq!(obj.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn serialization_borrowed_object_ref() {
		let object_ref = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();
		assert!(matches!(obj, ObjectRef::Borrowed { .. }));

		assert_eq!(obj.name().unwrap().as_str(), ":1.23");
		assert_eq!(obj.path_as_str(), TEST_OBJECT_PATH);
	}

	#[test]
	fn object_ref_equality() {
		let object_ref1 = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		let object_ref2 = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.23").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_eq!(object_ref1, object_ref2);

		let object_ref3 = ObjectRef::new_borrowed(
			UniqueName::from_static_str(":1.24").unwrap(),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_ne!(object_ref1, object_ref3);

		let object_ref4 = ObjectRef::new_owned(
			UniqueName::from_static_str_unchecked(":1.23"),
			ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH),
		);
		assert_eq!(object_ref1, object_ref4);

		let null_object_ref: ObjectRef = ObjectRef::Null;
		assert_ne!(object_ref1, null_object_ref);
		assert_ne!(null_object_ref, object_ref1);

		let null_object_ref2: ObjectRef = ObjectRef::Null;
		assert_eq!(null_object_ref, null_object_ref2);
	}

	#[test]
	fn try_from_value_for_objectref() {
		let name = UniqueName::from_static_str_unchecked(":0.0");
		let path = ObjectPath::from_static_str_unchecked("/org/a11y/atspi/testpath");

		let objref = ObjectRef::new_borrowed(name, path);
		let value: Value = objref.into();

		let objref_2: ObjectRef = value.try_into().unwrap();

		assert_eq!(objref_2.name().unwrap().as_str(), ":0.0");
		assert_eq!(objref_2.path_as_str(), "/org/a11y/atspi/testpath");
	}

	#[test]
	fn try_from_owned_value_for_objectref() {
		let name = UniqueName::from_static_str_unchecked(":0.0");
		let path = ObjectPath::from_static_str_unchecked("/org/a11y/atspi/testpath");

		let objref = ObjectRef::new_borrowed(name, path);

		let value: Value = objref.into();
		let value: OwnedValue = value.try_into().unwrap();
		let objref_2: ObjectRef = value.try_into().unwrap();

		assert_eq!(objref_2.name_as_str(), Some(":0.0"));
		assert_eq!(objref_2.path_as_str(), "/org/a11y/atspi/testpath");
	}

	// Must fail test:

	#[test]
	fn must_fail_test_try_from_invalid_value_for_object_ref() {
		let value = zvariant::Value::from((42, true));
		let obj: Result<ObjectRef, _> = value.try_into();
		assert!(obj.is_err());
	}

	#[test]
	fn hash_and_object_coherence() {
		let name = UniqueName::from_static_str_unchecked(":1.23");
		let path = ObjectPath::from_static_str_unchecked(TEST_OBJECT_PATH);

		let object_ref1 = ObjectRef::new_borrowed(&name, &path);
		let object_ref2 = ObjectRef::new_borrowed(name, path);

		let mut hasher1 = DefaultHasher::new();
		let mut hasher2 = DefaultHasher::new();
		assert_eq!(object_ref1, object_ref2);
		object_ref1.hash(&mut hasher1);
		object_ref2.hash(&mut hasher2);
		assert_eq!(hasher1.finish(), hasher2.finish());
	}

	#[test]
	#[should_panic(expected = "assertion failed: matches!(obj, ObjectRef::Borrowed { .. })")]
	fn valid_name_null_path_object_ref() {
		let object_ref = ObjectRef::from_static_str_unchecked("1.23", NULL_PATH_STR);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (obj, _) = encoded.deserialize::<ObjectRef>().unwrap();
		assert!(matches!(obj, ObjectRef::Borrowed { .. }));
	}

	// Check that the Deserialize implementation correctly panics
	#[test]
	#[should_panic(
		expected = r#"A non-null ObjectRef requires a name and a path but got: ("", /org/a11y/atspi/path/007)"#
	)]
	fn empty_name_valid_path_object_ref() {
		let object_ref = ObjectRef::from_static_str_unchecked("", TEST_OBJECT_PATH);

		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &object_ref).unwrap();

		let (_obj, _) = encoded.deserialize::<ObjectRef>().unwrap();
	}
}
