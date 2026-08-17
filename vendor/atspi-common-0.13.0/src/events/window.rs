#[cfg(feature = "zbus")]
use crate::error::AtspiError;
#[cfg(feature = "zbus")]
use crate::events::EventBody;
#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::events::{
	DBusInterface, DBusMatchRule, DBusMember, EventBodyOwned, RegistryEventString,
};
use crate::object_ref::ObjectRefOwned;
#[cfg(feature = "zbus")]
use crate::{EventProperties, ObjectRef};
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "zbus")]
use zbus::message::{Body as DbusBody, Header};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct PropertyChangeEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
	pub property: String,
}

impl_event_type_properties_for_event!(PropertyChangeEvent);

/// The window has been minimized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MinimizeEvent {
	/// The application which has been minimized.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(MinimizeEvent);

/// The window has been maximized.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct MaximizeEvent {
	/// The application which has been maximized.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(MaximizeEvent);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct RestoreEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(RestoreEvent);

/// A window has been closed.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CloseEvent {
	/// The application which has been closed.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(CloseEvent);

/// A new window has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct CreateEvent {
	/// An application to query for additional events from.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(CreateEvent);

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReparentEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(ReparentEvent);

/// A new virtual desktop has been created.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct DesktopCreateEvent {
	/// A reference to a new desktop
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(DesktopCreateEvent);

/// A virtual desktop has been deleted.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct DesktopDestroyEvent {
	/// A reference to the destroyed desktop.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(DesktopDestroyEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct DestroyEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}
impl_event_type_properties_for_event!(DestroyEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct ActivateEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(ActivateEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct DeactivateEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(DeactivateEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct RaiseEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(RaiseEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct LowerEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(LowerEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct MoveEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(MoveEvent);

/// A window has been resized.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct ResizeEvent {
	/// The application which has been resized.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(ResizeEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct ShadeEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(ShadeEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct UUshadeEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(UUshadeEvent);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash, Default)]
pub struct RestyleEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(RestyleEvent);

impl_member_interface_registry_string_and_match_rule_for_event!(
	PropertyChangeEvent,
	"PropertyChange",
	"org.a11y.atspi.Event.Window",
	"window:property-change",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'"
);

#[cfg(feature = "zbus")]
impl MessageConversion<'_> for PropertyChangeEvent {
	type Body<'a> = EventBody<'a>;

	fn from_message_unchecked_parts(item: ObjectRef, body: DbusBody) -> Result<Self, AtspiError> {
		let mut body = body.deserialize_unchecked::<Self::Body<'_>>()?;
		Ok(Self { item: item.into(), property: body.take_kind() })
	}

	fn from_message_unchecked(msg: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
		let item = header.try_into()?;
		let body = msg.body();
		Self::from_message_unchecked_parts(item, body)
	}

	fn body(&self) -> Self::Body<'_> {
		EventBody::Owned(EventBodyOwned { kind: self.property.clone(), ..Default::default() })
	}
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	MinimizeEvent,
	"Minimize",
	"org.a11y.atspi.Event.Window",
	"window:minimize",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	MaximizeEvent,
	"Maximize",
	"org.a11y.atspi.Event.Window",
	"window:maximize",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RestoreEvent,
	"Restore",
	"org.a11y.atspi.Event.Window",
	"window:restore",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	CloseEvent,
	"Close",
	"org.a11y.atspi.Event.Window",
	"window:close",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Close'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	CreateEvent,
	"Create",
	"org.a11y.atspi.Event.Window",
	"window:create",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Create'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ReparentEvent,
	"Reparent",
	"org.a11y.atspi.Event.Window",
	"window:reparent",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DesktopCreateEvent,
	"DesktopCreate",
	"org.a11y.atspi.Event.Window",
	"window:desktop-create",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DesktopDestroyEvent,
	"DesktopDestroy",
	"org.a11y.atspi.Event.Window",
	"window:desktop-destroy",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DestroyEvent,
	"Destroy",
	"org.a11y.atspi.Event.Window",
	"window:destroy",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ActivateEvent,
	"Activate",
	"org.a11y.atspi.Event.Window",
	"window:activate",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	DeactivateEvent,
	"Deactivate",
	"org.a11y.atspi.Event.Window",
	"window:deactivate",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RaiseEvent,
	"Raise",
	"org.a11y.atspi.Event.Window",
	"window:raise",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LowerEvent,
	"Lower",
	"org.a11y.atspi.Event.Window",
	"window:lower",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	MoveEvent,
	"Move",
	"org.a11y.atspi.Event.Window",
	"window:move",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Move'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ResizeEvent,
	"Resize",
	"org.a11y.atspi.Event.Window",
	"window:resize",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ShadeEvent,
	"Shade",
	"org.a11y.atspi.Event.Window",
	"window:shade",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	UUshadeEvent,
	"uUshade",
	"org.a11y.atspi.Event.Window",
	"window:uushade",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	RestyleEvent,
	"Restyle",
	"org.a11y.atspi.Event.Window",
	"window:restyle",
	"type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'"
);

event_test_cases!(PropertyChangeEvent);
impl_to_dbus_message!(PropertyChangeEvent);
impl_from_dbus_message!(PropertyChangeEvent);
impl_event_properties!(PropertyChangeEvent);
impl From<PropertyChangeEvent> for EventBodyOwned {
	fn from(event: PropertyChangeEvent) -> Self {
		EventBodyOwned { kind: event.property, ..Default::default() }
	}
}

event_test_cases!(MinimizeEvent);
impl_to_dbus_message!(MinimizeEvent);
impl_from_dbus_message!(MinimizeEvent);
impl_event_properties!(MinimizeEvent);
impl_from_object_ref!(MinimizeEvent);

event_test_cases!(MaximizeEvent);
impl_to_dbus_message!(MaximizeEvent);
impl_from_dbus_message!(MaximizeEvent);
impl_event_properties!(MaximizeEvent);
impl_from_object_ref!(MaximizeEvent);

event_test_cases!(RestoreEvent);
impl_to_dbus_message!(RestoreEvent);
impl_from_dbus_message!(RestoreEvent);
impl_event_properties!(RestoreEvent);
impl_from_object_ref!(RestoreEvent);

event_test_cases!(CloseEvent);
impl_to_dbus_message!(CloseEvent);
impl_from_dbus_message!(CloseEvent);
impl_event_properties!(CloseEvent);
impl_from_object_ref!(CloseEvent);

event_test_cases!(CreateEvent);
impl_to_dbus_message!(CreateEvent);
impl_from_dbus_message!(CreateEvent);
impl_event_properties!(CreateEvent);
impl_from_object_ref!(CreateEvent);

event_test_cases!(ReparentEvent);
impl_to_dbus_message!(ReparentEvent);
impl_from_dbus_message!(ReparentEvent);
impl_event_properties!(ReparentEvent);
impl_from_object_ref!(ReparentEvent);

event_test_cases!(DesktopCreateEvent);
impl_to_dbus_message!(DesktopCreateEvent);
impl_from_dbus_message!(DesktopCreateEvent);
impl_event_properties!(DesktopCreateEvent);
impl_from_object_ref!(DesktopCreateEvent);

event_test_cases!(DesktopDestroyEvent);
impl_to_dbus_message!(DesktopDestroyEvent);
impl_from_dbus_message!(DesktopDestroyEvent);
impl_event_properties!(DesktopDestroyEvent);
impl_from_object_ref!(DesktopDestroyEvent);

event_test_cases!(DestroyEvent);
impl_to_dbus_message!(DestroyEvent);
impl_from_dbus_message!(DestroyEvent);
impl_event_properties!(DestroyEvent);
impl_from_object_ref!(DestroyEvent);

event_test_cases!(ActivateEvent);
impl_to_dbus_message!(ActivateEvent);
impl_from_dbus_message!(ActivateEvent);
impl_event_properties!(ActivateEvent);
impl_from_object_ref!(ActivateEvent);

event_test_cases!(DeactivateEvent);
impl_to_dbus_message!(DeactivateEvent);
impl_from_dbus_message!(DeactivateEvent);
impl_event_properties!(DeactivateEvent);
impl_from_object_ref!(DeactivateEvent);

event_test_cases!(RaiseEvent);
impl_to_dbus_message!(RaiseEvent);
impl_from_dbus_message!(RaiseEvent);
impl_event_properties!(RaiseEvent);
impl_from_object_ref!(RaiseEvent);

event_test_cases!(LowerEvent);
impl_to_dbus_message!(LowerEvent);
impl_from_dbus_message!(LowerEvent);
impl_event_properties!(LowerEvent);
impl_from_object_ref!(LowerEvent);

event_test_cases!(MoveEvent);
impl_to_dbus_message!(MoveEvent);
impl_from_dbus_message!(MoveEvent);
impl_event_properties!(MoveEvent);
impl_from_object_ref!(MoveEvent);

event_test_cases!(ResizeEvent);
impl_to_dbus_message!(ResizeEvent);
impl_from_dbus_message!(ResizeEvent);
impl_event_properties!(ResizeEvent);
impl_from_object_ref!(ResizeEvent);

event_test_cases!(ShadeEvent);
impl_to_dbus_message!(ShadeEvent);
impl_from_dbus_message!(ShadeEvent);
impl_event_properties!(ShadeEvent);
impl_from_object_ref!(ShadeEvent);

event_test_cases!(UUshadeEvent);
impl_to_dbus_message!(UUshadeEvent);
impl_from_dbus_message!(UUshadeEvent);
impl_event_properties!(UUshadeEvent);
impl_from_object_ref!(UUshadeEvent);

event_test_cases!(RestyleEvent);
impl_to_dbus_message!(RestyleEvent);
impl_from_dbus_message!(RestyleEvent);
impl_event_properties!(RestyleEvent);
impl_from_object_ref!(RestyleEvent);

impl_msg_conversion_ext_for_target_type!(PropertyChangeEvent);
impl_msg_conversion_ext_for_target_type!(MinimizeEvent);
impl_msg_conversion_ext_for_target_type!(MaximizeEvent);
impl_msg_conversion_ext_for_target_type!(RestoreEvent);
impl_msg_conversion_ext_for_target_type!(CloseEvent);
impl_msg_conversion_ext_for_target_type!(CreateEvent);
impl_msg_conversion_ext_for_target_type!(ReparentEvent);
impl_msg_conversion_ext_for_target_type!(DesktopCreateEvent);
impl_msg_conversion_ext_for_target_type!(DesktopDestroyEvent);
impl_msg_conversion_ext_for_target_type!(DestroyEvent);
impl_msg_conversion_ext_for_target_type!(ActivateEvent);
impl_msg_conversion_ext_for_target_type!(DeactivateEvent);
impl_msg_conversion_ext_for_target_type!(RaiseEvent);
impl_msg_conversion_ext_for_target_type!(LowerEvent);
impl_msg_conversion_ext_for_target_type!(MoveEvent);
impl_msg_conversion_ext_for_target_type!(ResizeEvent);
impl_msg_conversion_ext_for_target_type!(ShadeEvent);
impl_msg_conversion_ext_for_target_type!(UUshadeEvent);
impl_msg_conversion_ext_for_target_type!(RestyleEvent);

impl_msg_conversion_for_types_built_from_object_ref!(MinimizeEvent);
impl_msg_conversion_for_types_built_from_object_ref!(MaximizeEvent);
impl_msg_conversion_for_types_built_from_object_ref!(RestoreEvent);
impl_msg_conversion_for_types_built_from_object_ref!(CloseEvent);
impl_msg_conversion_for_types_built_from_object_ref!(CreateEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ReparentEvent);
impl_msg_conversion_for_types_built_from_object_ref!(DesktopCreateEvent);
impl_msg_conversion_for_types_built_from_object_ref!(DesktopDestroyEvent);
impl_msg_conversion_for_types_built_from_object_ref!(DestroyEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ActivateEvent);
impl_msg_conversion_for_types_built_from_object_ref!(DeactivateEvent);
impl_msg_conversion_for_types_built_from_object_ref!(RaiseEvent);
impl_msg_conversion_for_types_built_from_object_ref!(LowerEvent);
impl_msg_conversion_for_types_built_from_object_ref!(MoveEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ResizeEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ShadeEvent);
impl_msg_conversion_for_types_built_from_object_ref!(UUshadeEvent);
impl_msg_conversion_for_types_built_from_object_ref!(RestyleEvent);
