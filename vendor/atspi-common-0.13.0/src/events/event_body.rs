use crate::AtspiError;
use serde::{
	ser::{SerializeMap, SerializeTuple},
	Deserialize, Serialize,
};
use zbus_lockstep_macros::validate;
use zvariant::{ObjectPath, OwnedValue, Type, Value};

/// Event body as used exclusively by 'Qt' toolkit.
///
/// Signature:  "siiv(so)"
#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
pub struct EventBodyQtOwned {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: String,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	pub any_data: OwnedValue,

	/// Not in use.
	/// See: [`QtProperties`].
	#[serde(skip_deserializing)]
	pub(crate) properties: QtProperties,
}

impl Clone for EventBodyQtOwned {
	/// # Safety
	///
	/// This implementation of [`Clone`] *can panic!* although chances are slim.
	///
	/// If the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Then this function panic.
	/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the current
	/// specification describes that.
	/// See [`zvariant::Value::try_clone`] for more information.
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		Self {
			kind: self.kind.clone(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: QtProperties,
		}
	}
}

/// Unit struct placeholder for `EventBodyQtOwned.properties`
///
/// AT-SPI2 never reads or writes to `EventBodyQT.properties`.
/// `QtProperties` has the appropriate implementations for `Serialize` and `Deserialize`
/// to make it serialize as an a valid tuple and valid bytes deserialize as placeholder.
#[derive(Debug, Copy, Clone, Deserialize, Type, Default, PartialEq)]
#[zvariant(signature = "(so)")]
pub(crate) struct QtProperties;

impl Serialize for QtProperties {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		let mut tup = serializer.serialize_tuple(2)?;
		tup.serialize_element(":0.0")?;
		tup.serialize_element(&ObjectPath::from_static_str_unchecked("/"))?;
		tup.end()
	}
}

impl Default for EventBodyQtOwned {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0_u32.into(),
			properties: QtProperties,
		}
	}
}

/// Unit struct placeholder for `EventBody.properties`
///
/// AT-SPI2 never reads or writes to `EventBody.properties`.
/// `Properties` has the appropriate implementations for `Serialize` and `Deserialize`
/// to make it serialize as an a valid dictionary and valid bytes deserialize as placeholder.
#[derive(Debug, Copy, Clone, Type, Default, Deserialize, PartialEq)]
#[zvariant(signature = "a{sv}")]
pub(crate) struct Properties;

impl Serialize for Properties {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		serializer.serialize_map(Some(0))?.end()
	}
}

/// AT-SPI2 protocol native event body type.
///
/// All of the various signals in the AT-SPI2 protocol share this shape.
/// Most toolkits and implementors emit this type, except for `Qt`, which has has its
/// own type: [`EventBodyQtOwned`].
///
/// Signature `(siiva{sv})`,
#[validate(signal: "PropertyChange")]
#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
pub struct EventBodyOwned {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: String,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	///
	pub any_data: OwnedValue,

	/// Not in use.
	/// See: [`Properties`].
	pub(crate) properties: Properties,
}

impl Default for EventBodyOwned {
	fn default() -> Self {
		Self {
			kind: String::new(),
			detail1: 0,
			detail2: 0,
			any_data: 0_u32.into(),
			properties: Properties,
		}
	}
}

impl Clone for EventBodyOwned {
	/// # Safety
	///
	/// This implementation of [`Clone`] *can panic!* although chances are slim.
	///
	/// If the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Then this function panic.
	/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the current
	/// specification describes that.
	/// See [`zvariant::Value::try_clone`] for more information.
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		Self {
			kind: self.kind.clone(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: Properties,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
pub struct EventBodyBorrowed<'a> {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	#[serde(borrow)]
	pub kind: &'a str,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	#[serde(borrow)]
	pub any_data: Value<'a>,

	/// Not in use.
	/// See: [`Properties`].
	#[serde(skip_deserializing)]
	pub(crate) properties: Properties,
}

impl Default for EventBodyBorrowed<'_> {
	fn default() -> Self {
		Self {
			kind: "",
			detail1: 0,
			detail2: 0,
			any_data: Value::new(0_u32),
			properties: Properties,
		}
	}
}

impl EventBodyBorrowed<'_> {
	/// Convert this borrowed event body to an owned event body.
	///
	/// # Errors
	///
	/// This will error if the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Chances are slim because none of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// See [`zvariant::Value::try_clone`] for more information.
	pub fn to_fully_owned(&self) -> Result<EventBodyOwned, AtspiError> {
		let owned_any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyOwned {
			kind: self.kind.into(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: owned_any_data,
			properties: Properties,
		})
	}
}

impl Clone for EventBodyBorrowed<'_> {
	/// # Safety
	///
	/// This implementation of [`Clone`] *can panic!* although chances are slim.
	///
	/// If the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Then this function panic.
	/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the current
	/// specification describes that.
	/// See [`zvariant::Value::try_clone`] for more information.
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		Self {
			kind: self.kind,
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: Properties,
		}
	}
}

#[derive(Debug, Type, Deserialize, PartialEq)]
pub struct EventBodyQtBorrowed<'m> {
	/// kind variant, used for specifying an event triple "object:state-changed:focused",
	/// the "focus" part of this event is what is contained within the kind.
	#[serde(rename = "type")]
	pub kind: &'m str,

	/// Generic detail1 value described by AT-SPI.
	pub detail1: i32,

	/// Generic detail2 value described by AT-SPI.
	pub detail2: i32,

	/// Generic `any_data` value described by AT-SPI.
	/// This can be any type.
	#[serde(borrow)]
	pub any_data: Value<'m>,

	/// Not in use.
	/// See: [`QtProperties`].
	#[serde(skip_deserializing)]
	pub(crate) properties: QtProperties,
}

impl Default for EventBodyQtBorrowed<'_> {
	fn default() -> Self {
		Self {
			kind: "",
			detail1: 0,
			detail2: 0,
			any_data: Value::new(0_u32),
			properties: QtProperties,
		}
	}
}

impl Clone for EventBodyQtBorrowed<'_> {
	/// # Safety
	///
	/// This implementation of [`Clone`] *can panic!* although chances are slim.
	///
	/// If the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	///
	/// Then this function panics.
	/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the current
	/// specification describes that.
	/// See [`zvariant::Value::try_clone`] for more information.
	fn clone(&self) -> Self {
		let cloned_any_data = self.any_data.try_clone().unwrap_or_else(|err| {
			panic!("Failure cloning 'any_data' field: {err:?}");
		});

		Self {
			kind: self.kind,
			detail1: self.detail1,
			detail2: self.detail2,
			any_data: cloned_any_data,
			properties: QtProperties,
		}
	}
}

impl EventBodyQtBorrowed<'_> {
	/// Convert partially borrowed Qt event body to an owned event body.
	///
	/// # Errors
	///
	/// This will error if the following conditions are met:
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn try_to_owned(&self) -> Result<EventBodyQtOwned, AtspiError> {
		let any_data = self.any_data.try_to_owned()?;

		Ok(EventBodyQtOwned {
			kind: self.kind.to_owned(),
			detail1: self.detail1,
			detail2: self.detail2,
			any_data,
			properties: self.properties,
		})
	}
}

impl<'de> From<EventBodyQtBorrowed<'de>> for EventBodyBorrowed<'de> {
	fn from(borrow: EventBodyQtBorrowed<'de>) -> Self {
		let EventBodyQtBorrowed { kind, detail1, detail2, any_data, properties: _ } = borrow;

		Self { kind, detail1, detail2, any_data, properties: Properties }
	}
}

impl From<EventBodyQtOwned> for EventBodyOwned {
	fn from(body: EventBodyQtOwned) -> Self {
		Self {
			kind: body.kind,
			detail1: body.detail1,
			detail2: body.detail2,
			any_data: body.any_data,
			properties: Properties,
		}
	}
}

/// Common event body that can be either owned or borrowed.
///
/// This is useful for APIs that can return either owned or borrowed event bodies.
/// Having this type allows to be generic over the event body type.
#[derive(Debug, Clone, PartialEq)]
pub enum EventBody<'a> {
	Owned(EventBodyOwned),
	Borrowed(EventBodyBorrowed<'a>),
}

impl Default for EventBody<'_> {
	fn default() -> Self {
		Self::Borrowed(EventBodyBorrowed::default())
	}
}

impl<'a> EventBody<'_> {
	/// Non-consuming conversion to an owned event body.
	///
	/// Does cloning.
	///
	/// # Errors
	/// The borrowed variant will error if the following conditions are met:  
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and  
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn as_owned(&self) -> Result<EventBodyOwned, AtspiError> {
		match self {
			Self::Owned(owned) => Ok(owned.clone()),
			Self::Borrowed(borrowed) => borrowed.to_fully_owned(),
		}
	}

	/// Consuming conversion to an owned event body.
	///
	/// Does cloning.
	///
	/// # Errors
	/// The borrowed variant will error if the following conditions are met:  
	/// 1. the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and  
	/// 2. the maximum number of open files for the process is exceeded.
	pub fn into_owned(self) -> Result<EventBodyOwned, AtspiError> {
		match self {
			Self::Owned(owned) => Ok(owned),
			Self::Borrowed(borrowed) => borrowed.to_fully_owned(),
		}
	}

	/// The `kind` field as `&str`.
	///
	/// With both variants, this method returns a reference to the `kind` field.
	#[must_use]
	pub fn kind(&'a self) -> &'a str {
		match self {
			Self::Owned(owned) => owned.kind.as_str(),
			Self::Borrowed(borrowed) => borrowed.kind,
		}
	}

	/// Take or convert the `kind` field as `String`.
	///
	/// With the owned variant, this method takes the `kind` field and replaces it with an empty string.
	/// With the borrowed variant, this method clones and allocates the `kind` field.
	pub fn take_kind(&mut self) -> String {
		match self {
			Self::Owned(owned) => std::mem::take(&mut owned.kind),
			Self::Borrowed(borrowed) => borrowed.kind.to_owned(),
		}
	}

	/// The `detail1` field.
	#[must_use]
	pub fn detail1(&self) -> i32 {
		match self {
			Self::Owned(owned) => owned.detail1,
			Self::Borrowed(borrowed) => borrowed.detail1,
		}
	}

	/// The `detail2` field.
	#[must_use]
	pub fn detail2(&self) -> i32 {
		match self {
			Self::Owned(owned) => owned.detail2,
			Self::Borrowed(borrowed) => borrowed.detail2,
		}
	}

	/// The `any_data` field as `&Value`.
	/// With both variants, this method returns a reference to the `any_data` field.
	#[must_use]
	pub fn any_data(&'a self) -> &'a Value<'a> {
		match self {
			Self::Owned(owned) => &owned.any_data,
			Self::Borrowed(borrowed) => &borrowed.any_data,
		}
	}

	/// Take or convert the `any_data` field as `OwnedValue`.
	/// With the owned variant, this method takes the `any_data` field and replaces it with a default value.
	/// As `Value` does not have a default value, we will replace with `0_u32`, a nbon-allocating value.
	///
	/// With the borrowed variant, this method clones and allocates the `any_data` field.
	///
	/// # Panics
	/// This method will panic if the `any_data` field contains an [`std::os::fd::OwnedFd`] type, and
	/// the maximum number of open files for the process is exceeded.
	///
	/// None of the types in [`crate::events`] use [`std::os::fd::OwnedFd`].
	/// Events on the AT-SPI bus *could, theoretically* send a file descriptor, but nothing in the current
	/// specification describes that.
	pub fn take_any_data(&mut self) -> OwnedValue {
		match self {
			Self::Owned(owned) => std::mem::replace(&mut owned.any_data, 0_u32.into()),
			Self::Borrowed(borrowed) => borrowed.any_data.try_to_owned().expect("cloning 'any_data' field should not fail because we do not expect it to hold an fd"),
		}
	}
}

impl Type for EventBody<'_> {
	const SIGNATURE: &'static zvariant::Signature = EventBodyOwned::SIGNATURE;
}

impl<'de> Deserialize<'de> for EventBody<'de> {
	fn deserialize<D>(deserializer: D) -> Result<EventBody<'de>, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		let borrowed = EventBodyBorrowed::deserialize(deserializer)?;
		Ok(borrowed.into())
	}
}

impl Serialize for EventBody<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		match self {
			EventBody::Owned(owned) => owned.serialize(serializer),
			EventBody::Borrowed(borrowed) => borrowed.serialize(serializer),
		}
	}
}

impl From<EventBodyOwned> for EventBody<'_> {
	fn from(owned: EventBodyOwned) -> Self {
		EventBody::Owned(owned)
	}
}

impl<'b> From<EventBodyBorrowed<'b>> for EventBody<'b> {
	fn from(borrowed: EventBodyBorrowed<'b>) -> Self {
		EventBody::Borrowed(borrowed)
	}
}

impl From<EventBodyQtOwned> for EventBody<'_> {
	fn from(qt_owned: EventBodyQtOwned) -> Self {
		EventBody::Owned(qt_owned.into())
	}
}

impl<'a> From<EventBodyQtBorrowed<'a>> for EventBody<'a> {
	fn from(qt_borrowed: EventBodyQtBorrowed<'a>) -> Self {
		EventBody::Borrowed(qt_borrowed.into())
	}
}

impl From<EventBodyOwned> for EventBodyQtOwned {
	fn from(owned: EventBodyOwned) -> Self {
		Self {
			kind: owned.kind,
			detail1: owned.detail1,
			detail2: owned.detail2,
			any_data: owned.any_data,
			properties: QtProperties,
		}
	}
}

impl<'a> From<EventBodyBorrowed<'a>> for EventBodyQtOwned {
	fn from(borrowed: EventBodyBorrowed<'a>) -> Self {
		Self {
			kind: borrowed.kind.to_owned(),
			detail1: borrowed.detail1,
			detail2: borrowed.detail2,
			any_data: borrowed
				.any_data
				.try_to_owned()
				.expect("converting borrowed to owned should not fail"),
			properties: QtProperties,
		}
	}
}

impl From<EventBody<'_>> for EventBodyQtOwned {
	fn from(event: EventBody) -> Self {
		match event {
			EventBody::Owned(owned) => owned.into(),
			EventBody::Borrowed(borrowed) => borrowed.into(),
		}
	}
}

impl PartialEq<EventBodyOwned> for EventBodyQtOwned {
	fn eq(&self, other: &EventBodyOwned) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl PartialEq<EventBodyQtOwned> for EventBodyOwned {
	fn eq(&self, other: &EventBodyQtOwned) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl PartialEq<EventBodyBorrowed<'_>> for EventBodyQtBorrowed<'_> {
	fn eq(&self, other: &EventBodyBorrowed<'_>) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

impl PartialEq<EventBodyQtBorrowed<'_>> for EventBodyBorrowed<'_> {
	fn eq(&self, other: &EventBodyQtBorrowed<'_>) -> bool {
		self.kind == other.kind
			&& self.detail1 == other.detail1
			&& self.detail2 == other.detail2
			&& self.any_data == other.any_data
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::ObjectRef;
	use std::collections::HashMap;
	use zvariant::{serialized::Context, LE};
	use zvariant::{Array, ObjectPath, Value};

	#[test]
	fn owned_event_body_clone() {
		let event = EventBodyOwned::default();
		let cloned = event.clone();

		assert_eq!(event, cloned);
	}

	#[test]
	fn event_body_qt_clone() {
		let event = EventBodyQtOwned::default();
		let cloned = event.clone();

		assert_eq!(event, cloned);
	}

	#[test]
	fn event_body_borrowed_clone() {
		let event = EventBodyBorrowed::default();
		let cloned = event.clone();

		assert_eq!(event, cloned);
	}

	#[test]
	fn event_body_qt_borrowed_clone() {
		let event = EventBodyQtBorrowed::default();
		let cloned = event.clone();

		assert_eq!(event, cloned);
	}

	#[test]
	fn owned_event_body_default() {
		let event = EventBodyOwned::default();

		assert_eq!(event.kind, "");
		assert_eq!(event.detail1, 0);
		assert_eq!(event.detail2, 0);
		assert_eq!(event.any_data, 0_u32.into());
	}

	#[test]
	fn qt_event_body_default() {
		let event = EventBodyQtOwned::default();

		assert_eq!(event.kind, "");
		assert_eq!(event.detail1, 0);
		assert_eq!(event.detail2, 0);
		assert_eq!(event.any_data, 0_u32.into());
		assert_eq!(event.properties, QtProperties);
	}

	#[test]
	fn event_body_borrowed_default() {
		let event = EventBodyBorrowed::default();

		assert_eq!(event.kind, "");
		assert_eq!(event.detail1, 0);
		assert_eq!(event.detail2, 0);
		assert_eq!(event.any_data, Value::new(0_u32));
	}

	#[test]
	fn qt_event_body_borrowed_default() {
		let event = EventBodyQtBorrowed::default();

		assert_eq!(event.kind, "");
		assert_eq!(event.detail1, 0);
		assert_eq!(event.detail2, 0);
		assert_eq!(event.any_data, Value::new(0_u32));
		assert_eq!(event.properties, QtProperties);
	}

	#[test]
	fn event_body_default() {
		let event = EventBody::default();

		assert_eq!(event, EventBody::Borrowed(EventBodyBorrowed::default()));
	}

	#[test]
	fn qt_to_owned() {
		let qt = EventBodyQtOwned::default();
		let owned: EventBodyOwned = EventBodyQtOwned::default().into();

		assert_eq!(owned, qt);
	}

	#[test]
	fn borrowed_to_qt() {
		let borrowed: EventBodyBorrowed = EventBodyQtBorrowed::default().into();

		assert_eq!(borrowed, EventBodyBorrowed::default());
	}

	#[test]
	fn event_body_deserialize_as_owned() {
		let event = EventBodyOwned::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<EventBodyOwned>(ctxt, &event).unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBodyOwned>().unwrap();

		assert_eq!(deserialized, event);
	}

	#[test]
	fn owned_event_body_deserialize_as_borrowed() {
		let event = EventBodyOwned::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<EventBodyOwned>(ctxt, &event).unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBodyBorrowed>().unwrap();

		assert_eq!(deserialized, EventBodyBorrowed::default());
		assert_eq!(deserialized.kind, event.kind.as_str());
		assert_eq!(deserialized.detail1, event.detail1);
		assert_eq!(deserialized.detail2, event.detail2);
		assert_eq!(deserialized.any_data, *event.any_data);
	}

	#[test]
	fn qt_owned_event_body_deserialize_as_borrowed() {
		let event = EventBodyQtOwned::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<EventBodyQtOwned>(ctxt, &event).unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBodyBorrowed>().unwrap();

		assert_eq!(deserialized, EventBodyBorrowed::default());
		assert_eq!(deserialized.kind, event.kind.as_str());
		assert_eq!(deserialized.detail1, event.detail1);
		assert_eq!(deserialized.detail2, event.detail2);
		assert_eq!(deserialized.any_data, *event.any_data);
	}

	#[test]
	fn event_body_default_deserialize_as_event_body() {
		let event = EventBody::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<EventBody>(ctxt, &event).unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBody>().unwrap();

		assert_eq!(deserialized, event);
	}

	#[test]
	fn event_body_owned_default_deserialize_as_event_body() {
		let event = EventBodyOwned::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<EventBodyOwned>(ctxt, &event).unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBody>().unwrap();

		assert_eq!(deserialized.kind(), event.kind.as_str());
		assert_eq!(deserialized.detail1(), event.detail1);
		assert_eq!(deserialized.detail2(), event.detail2);
		assert_eq!(*deserialized.any_data(), *event.any_data);
	}

	#[test]
	fn complex_body_deserialize_as_event_body() {
		let boots = Array::from(vec!["these", "boots", "are", "made", "for", "walking"]);
		let boots = Value::from(boots);
		let event = (
			"object:state-changed:focused",
			1,
			2,
			boots.clone(),
			HashMap::from([("key", Value::from(55_u32)), ("key2", Value::from(56_u32))]),
		);

		let ctxt = Context::new_dbus(LE, 0);
		let bytes =
			zvariant::to_bytes::<(&str, i32, i32, Value, HashMap<&str, Value>)>(ctxt, &event)
				.unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBody>().unwrap();

		assert_eq!(deserialized.kind(), "object:state-changed:focused");
		assert_eq!(deserialized.detail1(), 1);
		assert_eq!(deserialized.detail2(), 2);
		assert_eq!(*deserialized.any_data(), boots);
	}

	#[test]
	fn complex_body_deserialize_as_owned_event_body() {
		let boots = Array::from(vec!["these", "boots", "are", "made", "for", "walking"]);
		let boots = Value::from(boots);
		let event = (
			"object:state-changed:focused",
			1,
			2,
			boots.clone(),
			HashMap::from([("key", Value::from(55_u32)), ("key2", Value::from(56_u32))]),
		);

		let ctxt = Context::new_dbus(LE, 0);
		let bytes =
			zvariant::to_bytes::<(&str, i32, i32, Value, HashMap<&str, Value>)>(ctxt, &event)
				.unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBodyOwned>().unwrap();

		assert_eq!(deserialized.kind, "object:state-changed:focused");
		assert_eq!(deserialized.detail1, 1);
		assert_eq!(deserialized.detail2, 2);
		assert_eq!(*deserialized.any_data, boots);
	}

	#[test]
	fn complex_qt_body_as_bytes_deserialize_as_event_body() {
		let boots = Array::from(vec!["these", "boots", "are", "made", "for", "walking"]);
		let boots = Value::from(boots);
		let event = (
			"and that is what they'll do",
			1,
			2,
			boots.clone(),
			(":0.0", ObjectPath::from_static_str_unchecked("/")),
		);

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<(&str, i32, i32, Value, (&str, ObjectPath))>(ctxt, &event)
			.unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBody>().unwrap();

		assert_eq!(deserialized.kind(), "and that is what they'll do");
		assert_eq!(deserialized.detail1(), 1);
		assert_eq!(deserialized.detail2(), 2);
		assert_eq!(*deserialized.any_data(), boots);
	}

	#[test]
	fn complex_qt_body_as_message_deserialize_as_event_body() {
		let boots = Array::from(vec!["these", "boots", "are", "made", "for", "walking"]);
		let boots = Value::from(boots);
		let event = (
			"and that is what they'll do",
			1,
			2,
			boots.clone(),
			(":0.0", ObjectPath::from_static_str_unchecked("/")),
		);

		let msg = zbus::Message::signal("/", "org.a11y.atspi.Object", "StateChange")
			.unwrap()
			.build(&event)
			.unwrap();

		let body = msg.body();
		let deserialized = body.deserialize_unchecked::<EventBody>().unwrap();

		assert_eq!(deserialized.kind(), "and that is what they'll do");
		assert_eq!(deserialized.detail1(), 1);
		assert_eq!(deserialized.detail2(), 2);
		assert_eq!(*deserialized.any_data(), boots);
	}

	#[test]
	fn illegal_body_as_bytes_deserialize_as_event_body() {
		let boots = Array::from(vec!["these", "boots", "are", "made", "for", "walking"]);
		let boots = Value::from(boots);
		let event = (
			"and that is what they'll do",
			1,
			2,
			4_u32,
			boots.clone(),
			(":0.0", ObjectPath::from_static_str_unchecked("/")),
		);

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<(&str, i32, i32, u32, Value<'_>, (&str, ObjectPath<'_>))>(
			ctxt, &event,
		)
		.unwrap();

		assert!(bytes.deserialize::<EventBody>().is_err());
	}

	#[test]
	fn complex_body_deserialize_as_borrowed_event_body() {
		let boots = Array::from(vec!["these", "boots", "are", "made", "for", "walking"]);
		let boots = Value::from(boots);
		let event = (
			"object:state-changed:focused",
			1,
			2,
			boots.clone(),
			HashMap::from([("key", Value::from(55_u32)), ("key2", Value::from(56_u32))]),
		);

		let ctxt = Context::new_dbus(LE, 0);
		let bytes =
			zvariant::to_bytes::<(&str, i32, i32, Value, HashMap<&str, Value>)>(ctxt, &event)
				.unwrap();

		let (deserialized, _) = bytes.deserialize::<EventBodyBorrowed>().unwrap();

		assert_eq!(deserialized.kind, "object:state-changed:focused");
		assert_eq!(deserialized.detail1, 1);
		assert_eq!(deserialized.detail2, 2);
		assert_eq!(deserialized.any_data, boots);
	}

	#[test]
	fn deserialize_message_from_complex_message_data() {
		let boots = Array::from(vec!["these", "boots", "are", "made", "for", "walking"]);
		let boots = Value::from(boots);
		let body = (
			"object:state-changed:focused",
			1,
			2,
			boots.clone(),
			HashMap::from([("key", Value::from(55_u32)), ("key2", Value::from(56_u32))]),
		);

		let message = zbus::Message::signal("/", "org.a11y.atspi.Object", "StateChange")
			.unwrap()
			.build(&body)
			.unwrap();

		let msg_body = message.body();

		let deserialized = msg_body.deserialize::<EventBodyOwned>().unwrap();

		assert_eq!(deserialized.kind, "object:state-changed:focused");
		assert_eq!(deserialized.detail1, 1);
		assert_eq!(deserialized.detail2, 2);
		assert_eq!(*deserialized.any_data, boots);
	}

	#[test]
	fn simple_data_deserialization() {
		let body = "hello";

		let message = zbus::Message::signal("/bus/driver/zeenix", "org.Zbus", "TicketCheck")
			.unwrap()
			.build(&body)
			.unwrap();

		let msg_body = message.body();
		let deserialized = msg_body.deserialize::<&str>().unwrap();

		assert_eq!(deserialized, body);
	}

	#[test]
	fn test_valid_hashmap_of_string_value_deserializes_as_properties() {
		let val = Value::from(0_u32);
		let key = "test";
		let map = HashMap::from([(key, val)]);

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<HashMap<&str, Value>>(ctxt, &map).unwrap();

		let (properties, _) = bytes.deserialize::<Properties>().unwrap();

		assert_eq!(properties, Properties);
	}

	#[test]
	fn test_object_ref_deserializes_as_qt_properties() {
		let object_ref = ObjectRef::default();

		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<ObjectRef>(ctxt, &object_ref).unwrap();

		let (qt_props, _) = bytes.deserialize::<QtProperties>().unwrap();

		assert_eq!(qt_props, QtProperties);
	}

	#[test]
	fn test_properties_serializes_as_valid_hashmap() {
		let properties = Properties;
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<Properties>(ctxt, &properties).unwrap();

		let (map, _) = bytes.deserialize::<HashMap<&str, Value>>().unwrap();

		assert_eq!(map, HashMap::new());
	}

	#[test]
	fn test_qt_properties_serializes_as_valid_string_objpath_tuple() {
		let qt_properties = QtProperties;
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<QtProperties>(ctxt, &qt_properties).unwrap();

		let (tuple, _) = bytes.deserialize::<(&str, ObjectPath)>().unwrap();

		assert_eq!(tuple, (":0.0", ObjectPath::from_static_str_unchecked("/")));
	}

	#[test]
	fn test_qt_properties_serializes_as_valid_object_ref() {
		let qt_properties = QtProperties;
		let ctxt = Context::new_dbus(LE, 0);
		let bytes = zvariant::to_bytes::<QtProperties>(ctxt, &qt_properties).unwrap();

		let (objectref, _) = bytes.deserialize::<ObjectRef>().unwrap();

		assert_eq!(objectref.name().unwrap().as_str(), ":0.0");
		assert_eq!(objectref.path(), &ObjectPath::from_static_str_unchecked("/"));
	}

	#[cfg(test)]
	mod signatures {
		use crate::events::EventBodyQtOwned;
		use zvariant::{signature::Fields, Signature, Type};

		#[test]
		fn test_event_body_signature_equals_borrowed_event_body_signature() {
			use super::*;
			use zvariant::Type;

			let borrowed = EventBodyBorrowed::SIGNATURE;
			let owned = EventBodyOwned::SIGNATURE;

			assert_eq!(borrowed, owned);
		}

		// We have no definition of the Qt event body, so we can't compare the type
		// to a signature definition in XML.
		// That is why we keep a copy here.
		const QSPI_EVENT_SIGNATURE: &Signature = &Signature::static_structure(&[
			&Signature::Str,
			&Signature::I32,
			&Signature::I32,
			&Signature::Variant,
			&Signature::Structure(Fields::Static {
				fields: &[&Signature::Str, &Signature::ObjectPath],
			}),
		]);

		#[test]
		fn check_event_body_qt_signature() {
			assert_eq!(<EventBodyQtOwned as Type>::SIGNATURE, QSPI_EVENT_SIGNATURE);
		}
	}
}
