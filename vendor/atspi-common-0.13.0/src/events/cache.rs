use crate::cache::{CacheItem, LegacyCacheItem};
#[cfg(feature = "zbus")]
use crate::error::AtspiError;
#[cfg(feature = "zbus")]
use crate::object_ref::ObjectRef;
#[cfg(feature = "zbus")]
use crate::EventProperties;
use crate::{
	events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString},
	object_ref::ObjectRefOwned,
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[cfg(feature = "zbus")]
use super::{MessageConversion, MessageConversionExt};

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::LegacyCacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct LegacyAddAccessibleEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRefOwned,
	/// A cache item to add to the internal cache.
	pub node_added: LegacyCacheItem,
}

impl_event_type_properties_for_event!(LegacyAddAccessibleEvent);

event_test_cases!(LegacyAddAccessibleEvent, Explicit);
impl_from_dbus_message!(LegacyAddAccessibleEvent, Explicit);
impl_event_properties!(LegacyAddAccessibleEvent);
impl_to_dbus_message!(LegacyAddAccessibleEvent);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LegacyAddAccessibleEvent,
	"AddAccessible",
	"org.a11y.atspi.Cache",
	"cache:add",
	"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'"
);

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for LegacyAddAccessibleEvent {
	type Body<'msg> = LegacyCacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		Ok(Self { item: item.into(), node_added: body.deserialize_unchecked::<Self::Body<'_>>()? })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.node_added.clone()
	}
}

/// Type that contains the `zbus::Message` for meta information and
/// the [`crate::cache::CacheItem`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct AddAccessibleEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRefOwned,
	/// A cache item to add to the internal cache.
	pub node_added: CacheItem,
}

impl_event_type_properties_for_event!(AddAccessibleEvent);

event_test_cases!(AddAccessibleEvent, Explicit);

impl_member_interface_registry_string_and_match_rule_for_event!(
	AddAccessibleEvent,
	"AddAccessible",
	"org.a11y.atspi.Cache",
	"cache:add",
	"type='signal',interface='org.a11y.atspi.Cache',member='AddAccessible'"
);

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for AddAccessibleEvent {
	type Body<'msg> = CacheItem;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		Ok(Self { item: item.into(), node_added: body.deserialize_unchecked::<Self::Body<'_>>()? })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.node_added.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: AddAccessibleEvent, body: CacheItem);
impl_from_dbus_message!(AddAccessibleEvent, Explicit);
impl_event_properties!(AddAccessibleEvent);
impl_to_dbus_message!(AddAccessibleEvent);

/// `Cache::RemoveAccessible` signal event type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct RemoveAccessibleEvent {
	/// The application that emitted the signal TODO Check Me
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRefOwned,
	/// The node that was removed from the application tree  TODO Check Me
	pub node_removed: ObjectRefOwned,
}

impl_event_type_properties_for_event!(RemoveAccessibleEvent);

event_test_cases!(RemoveAccessibleEvent, Explicit);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RemoveAccessibleEvent,
	"RemoveAccessible",
	"org.a11y.atspi.Cache",
	"cache:remove",
	"type='signal',interface='org.a11y.atspi.Cache',member='RemoveAccessible'"
);

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for RemoveAccessibleEvent {
	type Body<'msg> = ObjectRefOwned;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		Ok(Self {
			item: item.into(),
			node_removed: body.deserialize_unchecked::<Self::Body<'_>>()?,
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.node_removed.clone()
	}
}

#[cfg(feature = "zbus")]
impl MessageConversionExt<'_, LegacyCacheItem> for LegacyAddAccessibleEvent {
	fn try_from_message(msg: &zbus::Message, hdr: &Header) -> Result<Self, AtspiError> {
		<LegacyAddAccessibleEvent as MessageConversionExt<crate::LegacyCacheItem>>::validate_interface(hdr)?;
		<LegacyAddAccessibleEvent as MessageConversionExt<crate::LegacyCacheItem>>::validate_member(hdr)?;
		<LegacyAddAccessibleEvent as MessageConversionExt<crate::LegacyCacheItem>>::validate_body(
			msg,
		)?;
		<LegacyAddAccessibleEvent as MessageConversion>::from_message_unchecked(msg, hdr)
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: RemoveAccessibleEvent, body: ObjectRefOwned);
impl_from_dbus_message!(RemoveAccessibleEvent, Explicit);
impl_event_properties!(RemoveAccessibleEvent);
impl_to_dbus_message!(RemoveAccessibleEvent);
