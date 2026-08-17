#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::events::{
	DBusInterface, DBusMatchRule, DBusMember, EventBody, EventBodyOwned, RegistryEventString,
};
use crate::object_ref::ObjectRefOwned;
#[cfg(feature = "zbus")]
use crate::EventProperties;
#[cfg(feature = "zbus")]
use crate::{error::AtspiError, ObjectRef};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AbsEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
	pub x: i32,
	pub y: i32,
}

impl_event_type_properties_for_event!(AbsEvent);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RelEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
	pub x: i32,
	pub y: i32,
}

impl_event_type_properties_for_event!(RelEvent);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ButtonEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
	pub detail: String,
	pub mouse_x: i32,
	pub mouse_y: i32,
}

impl_event_type_properties_for_event!(ButtonEvent);

impl_member_interface_registry_string_and_match_rule_for_event! {
	AbsEvent,
	"Abs",
	"org.a11y.atspi.Event.Mouse",
	"mouse:abs",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'"
}

impl_member_interface_registry_string_and_match_rule_for_event! {
	RelEvent,
	"Rel",
	"org.a11y.atspi.Event.Mouse",
	"mouse:rel",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'"
}

impl_member_interface_registry_string_and_match_rule_for_event! {
	ButtonEvent,
	"Button",
	"org.a11y.atspi.Event.Mouse",
	"mouse:button",
	"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'"
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for AbsEvent {
	type Body<'a> = EventBody<'a>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item: item.into(), x: body.detail1(), y: body.detail2() })
	}
	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}
	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned { detail1: self.x, detail2: self.y, ..Default::default() }.into()
	}
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for RelEvent {
	type Body<'a> = EventBody<'a>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item: item.into(), x: body.detail1(), y: body.detail2() })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned { detail1: self.x, detail2: self.y, ..Default::default() }.into()
	}
}

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for ButtonEvent {
	type Body<'a> = EventBody<'a>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self {
			item: item.into(),
			detail: body.take_kind(),
			mouse_x: body.detail1(),
			mouse_y: body.detail2(),
		})
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBodyOwned::from(self).into()
	}
}

event_test_cases!(AbsEvent);
impl_to_dbus_message!(AbsEvent);
impl_from_dbus_message!(AbsEvent);
impl_event_properties!(AbsEvent);

impl From<AbsEvent> for EventBodyOwned {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<&AbsEvent> for EventBodyOwned {
	fn from(event: &AbsEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<AbsEvent> for EventBody<'_> {
	fn from(event: AbsEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

event_test_cases!(RelEvent);
impl_to_dbus_message!(RelEvent);
impl_from_dbus_message!(RelEvent);
impl_event_properties!(RelEvent);

impl From<RelEvent> for EventBodyOwned {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<&RelEvent> for EventBodyOwned {
	fn from(event: &RelEvent) -> Self {
		EventBodyOwned { detail1: event.x, detail2: event.y, ..Default::default() }
	}
}

impl From<RelEvent> for EventBody<'_> {
	fn from(event: RelEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

event_test_cases!(ButtonEvent);
impl_to_dbus_message!(ButtonEvent);
impl_from_dbus_message!(ButtonEvent);

impl_event_properties!(ButtonEvent);
impl From<ButtonEvent> for EventBodyOwned {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned {
			kind: event.detail,
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			..Default::default()
		}
	}
}

impl From<ButtonEvent> for EventBody<'_> {
	fn from(event: ButtonEvent) -> Self {
		EventBodyOwned::from(event).into()
	}
}

impl From<&ButtonEvent> for EventBodyOwned {
	fn from(event: &ButtonEvent) -> Self {
		EventBodyOwned {
			kind: event.detail.clone(),
			detail1: event.mouse_x,
			detail2: event.mouse_y,
			..Default::default()
		}
	}
}

impl_msg_conversion_ext_for_target_type!(AbsEvent);
impl_msg_conversion_ext_for_target_type!(RelEvent);
impl_msg_conversion_ext_for_target_type!(ButtonEvent);
