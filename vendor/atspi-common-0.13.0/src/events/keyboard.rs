#[cfg(feature = "zbus")]
use super::event_body::EventBody;
#[cfg(feature = "zbus")]
use crate::error::AtspiError;
use crate::{
	events::{DBusInterface, DBusMatchRule, DBusMember, EventBodyOwned, RegistryEventString},
	object_ref::ObjectRefOwned,
};

#[cfg(feature = "zbus")]
use crate::{events::MessageConversion, EventProperties, ObjectRef};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ModifiersEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
	pub previous_modifiers: i32,
	pub current_modifiers: i32,
}

impl_event_type_properties_for_event!(ModifiersEvent);

impl_member_interface_registry_string_and_match_rule_for_event! {
	ModifiersEvent,
	"Modifiers",
	"org.a11y.atspi.Event.Keyboard",
	"keyboard:modifiers",
	"type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'"
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for ModifiersEvent {
	type Body<'msg> = EventBody<'msg>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self {
			item: item.into(),
			previous_modifiers: body.detail1(),
			current_modifiers: body.detail2(),
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned {
			detail1: self.previous_modifiers,
			detail2: self.current_modifiers,
			..Default::default()
		}
		.into()
	}
}

impl_msg_conversion_ext_for_target_type!(ModifiersEvent);

event_test_cases!(ModifiersEvent);
impl_to_dbus_message!(ModifiersEvent);
impl_from_dbus_message!(ModifiersEvent);
impl_event_properties!(ModifiersEvent);

impl From<ModifiersEvent> for EventBodyOwned {
	fn from(event: ModifiersEvent) -> Self {
		EventBodyOwned {
			detail1: event.previous_modifiers,
			detail2: event.current_modifiers,
			..Default::default()
		}
	}
}
