//! Conversion functions and types representing a set of [`Interface`]s.
//!
//! Each `AccessibleProxy` will implement some set of these interfaces,
//! represented by a [`InterfaceSet`].

use enumflags2::{bitflags, BitFlag, BitFlags};
use serde::{
	de::{self, Deserializer, Visitor},
	ser::{self, Serializer},
	Deserialize, Serialize,
};
use std::fmt;
use zvariant::{Signature, Type};

/// AT-SPI interfaces an accessible object can implement.
#[bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interface {
	/// Interface to indicate implementation of `AccessibleProxy`.
	#[serde(rename = "org.a11y.atspi.Accessible")]
	Accessible,
	/// Interface to indicate implementation of `ActionProxy`.
	#[serde(rename = "org.a11y.atspi.Action")]
	Action,
	/// Interface to indicate implementation of `ApplicationProxy`.
	#[serde(rename = "org.a11y.atspi.Application")]
	Application,
	/// Interface to indicate implementation of `CacheProxy`.
	#[serde(rename = "org.a11y.atspi.Cache")]
	Cache,
	/// Interface to indicate implementation of `CollectionProxy`.
	#[serde(rename = "org.a11y.atspi.Collection")]
	Collection,
	/// Interface to indicate implementation of `ComponentProxy`.
	#[serde(rename = "org.a11y.atspi.Component")]
	Component,
	/// Interface to indicate implementation of `DocumentProxy`.
	#[serde(rename = "org.a11y.atspi.Document")]
	Document,
	/// Interface to indicate implementation of `DeviceEventControllerProxy`.
	#[serde(rename = "org.a11y.atspi.DeviceEventController")]
	DeviceEventController,
	/// Interface to indicate implementation of `DeviceEventListenerProxy`.
	#[serde(rename = "org.a11y.atspi.DeviceEventListener")]
	DeviceEventListener,
	/// Interface to indicate implementation of `EditableTextProxy`.
	#[serde(rename = "org.a11y.atspi.EditableText")]
	EditableText,
	/// Interface to indicate implementation of `HyperlinkProxy`.
	#[serde(rename = "org.a11y.atspi.Hyperlink")]
	Hyperlink,
	/// Interface to indicate implementation of `HypertextProxy`.
	#[serde(rename = "org.a11y.atspi.Hypertext")]
	Hypertext,
	/// Interface to indicate implementation of `ImageProxy`.
	#[serde(rename = "org.a11y.atspi.Image")]
	Image,
	/// Interface to indicate implementation of `RegistryProxy`.
	#[serde(rename = "org.a11y.atspi.Registry")]
	Registry,
	/// Interface to indicate implementation of `SelectionProxy`.
	#[serde(rename = "org.a11y.atspi.Selection")]
	Selection,
	/// Interface to indicate implementation of `SocketProxy`.
	#[serde(rename = "org.a11y.atspi.Socket")]
	Socket,
	/// Interface to indicate implementation of `TableProxy`.
	#[serde(rename = "org.a11y.atspi.Table")]
	Table,
	/// Interface to indicate implementation of `TableCellProxy`.
	#[serde(rename = "org.a11y.atspi.TableCell")]
	TableCell,
	/// Interface to indicate implementation of `TextProxy`.
	#[serde(rename = "org.a11y.atspi.Text")]
	Text,
	/// Interface to indicate implementation of `ValueProxy`.
	#[serde(rename = "org.a11y.atspi.Value")]
	Value,
}

/// A collection type which encodes the AT-SPI interfaces an accessible object has implemented.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceSet(BitFlags<Interface>);

impl InterfaceSet {
	pub fn new<B: Into<BitFlags<Interface>>>(value: B) -> Self {
		Self(value.into())
	}

	#[must_use]
	pub fn empty() -> InterfaceSet {
		InterfaceSet(Interface::empty())
	}

	#[must_use]
	pub fn bits(&self) -> u32 {
		self.0.bits()
	}

	#[must_use]
	pub fn all() -> InterfaceSet {
		InterfaceSet(Interface::all())
	}

	pub fn contains<B: Into<BitFlags<Interface>>>(self, other: B) -> bool {
		self.0.contains(other)
	}

	pub fn insert<B: Into<BitFlags<Interface>>>(&mut self, other: B) {
		self.0.insert(other);
	}

	#[must_use]
	pub fn iter(&self) -> enumflags2::Iter<Interface> {
		self.0.iter()
	}
}

impl IntoIterator for InterfaceSet {
	type IntoIter = enumflags2::Iter<Interface>;
	type Item = Interface;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl IntoIterator for &InterfaceSet {
	type IntoIter = enumflags2::Iter<Interface>;
	type Item = Interface;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl Default for InterfaceSet {
	fn default() -> Self {
		Self::empty()
	}
}

impl<'de> de::Deserialize<'de> for InterfaceSet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct InterfaceSetVisitor;

		impl<'de> Visitor<'de> for InterfaceSetVisitor {
			type Value = InterfaceSet;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a sequence comprised of valid AT-SPI interface names")
			}

			fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
			where
				D: Deserializer<'de>,
			{
				match <Vec<Interface> as Deserialize>::deserialize(deserializer) {
					Ok(interfaces) => Ok(InterfaceSet(BitFlags::from_iter(interfaces))),
					Err(e) => Err(e),
				}
			}
		}

		deserializer.deserialize_newtype_struct("InterfaceSet", InterfaceSetVisitor)
	}
}

impl ser::Serialize for InterfaceSet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer
			.serialize_newtype_struct("InterfaceSet", &self.0.iter().collect::<Vec<Interface>>())
	}
}

impl Type for InterfaceSet {
	const SIGNATURE: &'static Signature = <Vec<String> as Type>::SIGNATURE;
}

impl FromIterator<Interface> for InterfaceSet {
	fn from_iter<T: IntoIterator<Item = Interface>>(iter: T) -> Self {
		Self(BitFlags::from_iter(iter))
	}
}

impl<'a> FromIterator<&'a Interface> for InterfaceSet {
	fn from_iter<I: IntoIterator<Item = &'a Interface>>(iter: I) -> Self {
		InterfaceSet(iter.into_iter().copied().collect())
	}
}

impl From<Interface> for InterfaceSet {
	fn from(value: Interface) -> Self {
		Self(value.into())
	}
}

impl std::ops::BitAnd for InterfaceSet {
	type Output = InterfaceSet;

	fn bitand(self, other: Self) -> Self::Output {
		InterfaceSet(self.0 & other.0)
	}
}

impl std::ops::BitXor for InterfaceSet {
	type Output = InterfaceSet;

	fn bitxor(self, other: Self) -> Self::Output {
		InterfaceSet(self.0 ^ other.0)
	}
}

impl std::ops::BitOr for InterfaceSet {
	type Output = InterfaceSet;

	fn bitor(self, other: Self) -> Self::Output {
		InterfaceSet(self.0 | other.0)
	}
}

#[cfg(test)]
mod tests {
	use super::{Interface, InterfaceSet};
	use zvariant::serialized::Data;
	use zvariant::{serialized::Context, to_bytes, LE};

	#[test]
	fn serialize_empty_interface_set() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &InterfaceSet::empty()).unwrap();
		assert_eq!(encoded.bytes(), &[0, 0, 0, 0]);
	}

	#[test]
	fn deserialize_empty_interface_set() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &InterfaceSet::empty()).unwrap();
		let (decoded, _) = encoded.deserialize::<InterfaceSet>().unwrap();
		assert_eq!(decoded, InterfaceSet::empty());
	}

	#[test]
	fn serialize_interface_set_accessible() {
		let ctxt = Context::new_dbus(LE, 0);
		let encoded = to_bytes(ctxt, &InterfaceSet::new(Interface::Accessible)).unwrap();
		assert_eq!(
			encoded.bytes(),
			&[
				30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
				112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0
			]
		);
	}

	#[test]
	fn deserialize_interface_set_accessible() {
		let ctxt = Context::new_dbus(LE, 0);
		let data = Data::new::<&[u8]>(
			&[
				30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
				112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0,
			],
			ctxt,
		);

		let (ifaceset, _) = data.deserialize::<InterfaceSet>().unwrap();
		assert_eq!(ifaceset, InterfaceSet::new(Interface::Accessible));
	}

	#[test]
	fn can_handle_multiple_interfaces() {
		let ctxt = Context::new_dbus(LE, 0);
		let object =
			InterfaceSet::new(Interface::Accessible | Interface::Action | Interface::Component);
		let encoded = to_bytes(ctxt, &object).unwrap();
		let (decoded, _) = encoded.deserialize::<InterfaceSet>().unwrap();
		assert!(object == decoded);
	}

	// The order of appearance of the interfaces is equal to the order in the enum.
	#[test]
	fn iterator_on_interface_set() {
		let set =
			InterfaceSet::new(Interface::Accessible | Interface::Action | Interface::Component);
		let mut iter = set.into_iter();
		assert_eq!(iter.next(), Some(Interface::Accessible));
		assert_eq!(iter.next(), Some(Interface::Action));
		assert_eq!(iter.next(), Some(Interface::Component));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn iterator_on_interface_set_ref() {
		let set = InterfaceSet::new(Interface::Text | Interface::Collection | Interface::Component);
		let mut iter = (&set).into_iter();
		assert_eq!(iter.next(), Some(Interface::Collection));
		assert_eq!(iter.next(), Some(Interface::Component));
		assert_eq!(iter.next(), Some(Interface::Text));
	}
}
