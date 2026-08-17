#[cfg(feature = "zbus")]
use crate::error::AtspiError;
#[cfg(feature = "zbus")]
use crate::EventProperties;
use crate::{
	events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString},
	object_ref::ObjectRefOwned,
};

#[cfg(feature = "zbus")]
use zbus::message::Header;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct FocusEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_event_type_properties_for_event!(FocusEvent);

impl_member_interface_registry_string_and_match_rule_for_event! {
	FocusEvent,
	"Focus",
	"org.a11y.atspi.Event.Focus",
	"focus:",
	"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'"
}

impl_msg_conversion_ext_for_target_type!(FocusEvent);
impl_msg_conversion_for_types_built_from_object_ref!(FocusEvent);

event_test_cases!(FocusEvent);
impl_to_dbus_message!(FocusEvent);
impl_from_dbus_message!(FocusEvent);
impl_event_properties!(FocusEvent);
impl_from_object_ref!(FocusEvent);
