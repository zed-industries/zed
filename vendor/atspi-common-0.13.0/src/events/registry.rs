//! This module contains the events that are emitted by the registry daemon.
//! The events are [`EventListenerRegisteredEvent`] and [`EventListenerDeregisteredEvent`].

#[cfg(feature = "zbus")]
use crate::object_ref::ObjectRef;
#[cfg(feature = "zbus")]
use crate::{error::AtspiError, events::MessageConversion, EventProperties};
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};
use zbus_lockstep_macros::validate;
use zbus_names::{OwnedUniqueName, UniqueName};
use zvariant::Type;

use crate::{
	events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString},
	object_ref::ObjectRefOwned,
};

/// An event that is emitted by the registry daemon, to inform that an event has been deregistered
/// to no longer listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerDeregisteredEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRefOwned,
	/// A list of events that have been deregistered via the registry interface.
	/// See `atspi-connection`.
	pub deregistered_event: EventListeners,
}

impl_event_type_properties_for_event!(EventListenerDeregisteredEvent);

event_test_cases!(EventListenerDeregisteredEvent, Explicit);

// The registry string cannot be found in upstrream at-spi2-core.

impl_member_interface_registry_string_and_match_rule_for_event!(
	EventListenerDeregisteredEvent,
	"EventListenerDeregistered",
	"org.a11y.atspi.Registry",
	"registry:event-listener-deregistered",
	"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerDeregistered'"
);

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for EventListenerDeregisteredEvent {
	type Body<'a> = EventListeners;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let deregistered_event = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item: item.into(), deregistered_event })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.deregistered_event.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: EventListenerDeregisteredEvent, body: EventListeners);
impl_from_dbus_message!(EventListenerDeregisteredEvent, Explicit);
impl_event_properties!(EventListenerDeregisteredEvent);
impl_to_dbus_message!(EventListenerDeregisteredEvent);

/// An event that is emitted by the regostry daemon to signal that an event has been registered to listen for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
pub struct EventListenerRegisteredEvent {
	/// The [`ObjectRef`] the event applies to.
	pub item: ObjectRefOwned,
	/// A list of events that have been registered via the registry interface.
	/// See `atspi-connection`.
	pub registered_event: EventListeners,
}

impl_event_type_properties_for_event!(EventListenerRegisteredEvent);

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for EventListenerRegisteredEvent {
	type Body<'a> = EventListeners;

	fn from_message_unchecked_parts(
		item: ObjectRef,
		registered_event: DbusBody,
	) -> Result<Self, AtspiError> {
		let registered_event = registered_event.deserialize_unchecked()?;
		Ok(Self { item: item.into(), registered_event })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		self.registered_event.clone()
	}
}

impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: EventListenerRegisteredEvent, body: EventListeners);
impl_from_dbus_message!(EventListenerRegisteredEvent, Explicit);
impl_event_properties!(EventListenerRegisteredEvent);
impl_to_dbus_message!(EventListenerRegisteredEvent);

event_test_cases!(EventListenerRegisteredEvent, Explicit);

// The registry string cannot be found in upstrream at-spi2-core.
impl_member_interface_registry_string_and_match_rule_for_event!(
	EventListenerRegisteredEvent,
	"EventListenerRegistered",
	"org.a11y.atspi.Registry",
	"registry:event-listener-registered",
	"type='signal',interface='org.a11y.atspi.Registry',member='EventListenerRegistered'"
);

/// Signal type emitted by `EventListenerRegistered` and `EventListenerDeregistered` signals,
/// which belong to the `Registry` interface, implemented by the registry-daemon.
#[validate(signal: "EventListenerRegistered")]
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
pub struct EventListeners {
	pub bus_name: OwnedUniqueName,
	// TODO: `path` should be a `zvariant::ObjectPath` but that requires changing the signature with an attribute
	// and `Serialize`/`Deserialize` impls.
	pub path: String,
}

impl Default for EventListeners {
	fn default() -> Self {
		Self {
			bus_name: UniqueName::from_static_str_unchecked(":0.0").into(),
			path: String::from("/org/a11y/atspi/null"),
		}
	}
}

#[cfg(test)]
mod event_listener_tests {
	use super::*;

	#[test]
	fn test_event_listener_default_no_panic() {
		let el = EventListeners::default();
		assert_eq!(el.bus_name.as_str(), ":0.0");
		assert_eq!(el.path.as_str(), "/org/a11y/atspi/null");
	}
}

pub mod socket {
	//! This module contains the event that is emitted by the registry daemon's `Socket` interface.

	#[cfg(feature = "zbus")]
	use crate::events::MessageConversion;
	use crate::events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString};
	use crate::object_ref::ObjectRefOwned;
	#[cfg(feature = "zbus")]
	use crate::AtspiError;
	#[cfg(feature = "zbus")]
	use crate::EventProperties;
	#[cfg(feature = "zbus")]
	use crate::ObjectRef;
	#[cfg(feature = "zbus")]
	use zbus::message::{Body as DbusBody, Header};

	use serde::{Deserialize, Serialize};

	/// An event that is emitted when the registry daemon has started.
	///
	/// The accessibility registry emits this signal early during startup,
	/// when it has registered with the `DBus` daemon and is available for
	/// calls from applications.
	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Eq, Hash)]
	pub struct AvailableEvent {
		/// The emitting [`ObjectRef`].
		pub item: ObjectRefOwned,

		/// The [`ObjectRef`] for the Registry's root object.
		pub socket: ObjectRefOwned,
	}

	impl_event_type_properties_for_event!(AvailableEvent);

	event_test_cases!(AvailableEvent, Explicit);

	// We cannot register at Registry for this event, as it is emitted by the Registry itself at early startup.
	// So, we do not have a registry event string for this event.
	// The `Available` event is unconditionally emitted:
	// [at-spi2-core/registryd/registry.c:1437](https://github.com/GNOME/at-spi2-core/blob/019d1a4013216d7d01040cf4eb3b8647bffc0dc9/registryd/registry.c#L1437)
	impl_member_interface_registry_string_and_match_rule_for_event!(
		AvailableEvent,
		"Available",
		"org.a11y.atspi.Socket",
		"",
		"type='signal',interface='org.a11y.atspi.Socket',member='Available'"
	);

	#[cfg(feature = "zbus")]
	impl MessageConversion<'_> for AvailableEvent {
		type Body<'a> = ObjectRef<'a>;

		fn from_message_unchecked_parts(
			item: ObjectRef<'_>,
			body: DbusBody,
		) -> Result<Self, AtspiError> {
			let socket = body.deserialize_unchecked::<Self::Body<'_>>()?;
			Ok(Self { item: item.into(), socket: socket.into() })
		}

		fn from_message_unchecked(
			msg: &zbus::Message,
			header: &Header,
		) -> Result<Self, AtspiError> {
			let item = header.try_into()?;
			let body = msg.body();
			Self::from_message_unchecked_parts(item, body)
		}

		fn body(&self) -> Self::Body<'_> {
			self.socket.clone().into_inner()
		}
	}

	impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: AvailableEvent, body: ObjectRef<'a>);
	impl_from_dbus_message!(AvailableEvent, Explicit);
	impl_event_properties!(AvailableEvent);
	impl_to_dbus_message!(AvailableEvent);
}
