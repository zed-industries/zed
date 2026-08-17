use crate::events::{DBusInterface, DBusMatchRule, DBusMember, RegistryEventString};
use crate::object_ref::ObjectRefOwned;
#[cfg(feature = "zbus")]
use crate::EventProperties;

#[cfg(feature = "zbus")]
use crate::AtspiError;
#[cfg(feature = "zbus")]
use zbus::message::Header;

/// An event triggered by the completion of a document load action.
/// For example: a web page has finished loading its initial payload, or
/// `LibreOffice` has loaded a document from disk.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LoadCompleteEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

/// An event triggered by a reloading of a document.
/// For example: pressing F5, or `Control + r` will reload a page in a web browser.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ReloadEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

/// An event triggered by the cancelling of a document load.
/// For example: during the loading of a large web page, a user may press `Escape` to stop loading the page.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct LoadStoppedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct ContentChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct AttributesChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

/// The focused page has changed.
///
/// This event is usually sent only by document readers, signaling
/// that the _physical page equivalent is now different.
/// This event does not encode _which_ page is the new one, only that a new page is now the primary
/// one.
///
/// See `atspi_proxies::document::DocumentProxy::current_page_number` to actively find the
/// page number.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub struct PageChangedEvent {
	/// The [`crate::ObjectRef`] which the event applies to.
	pub item: ObjectRefOwned,
}

impl_member_interface_registry_string_and_match_rule_for_event!(
	LoadCompleteEvent,
	"LoadComplete",
	"org.a11y.atspi.Event.Document",
	"document:load-complete",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	ReloadEvent,
	"Reload",
	"org.a11y.atspi.Event.Document",
	"document:reload",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	LoadStoppedEvent,
	"LoadStopped",
	"org.a11y.atspi.Event.Document",
	"document:load-stopped",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'"
);

// TODO confirm registry event string, not found in grep at at-spi2-core
impl_member_interface_registry_string_and_match_rule_for_event!(
	ContentChangedEvent,
	"ContentChanged",
	"org.a11y.atspi.Event.Document",
	"document:content-changed",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	AttributesChangedEvent,
	"AttributesChanged",
	"org.a11y.atspi.Event.Document",
	"document:attributes-changed",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'"
);

impl_member_interface_registry_string_and_match_rule_for_event!(
	PageChangedEvent,
	"PageChanged",
	"org.a11y.atspi.Event.Document",
	"document:page-changed",
	"type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'"
);

impl_event_type_properties_for_event!(LoadCompleteEvent);

event_test_cases!(LoadCompleteEvent);
impl_to_dbus_message!(LoadCompleteEvent);
impl_from_dbus_message!(LoadCompleteEvent);
impl_event_properties!(LoadCompleteEvent);
impl_from_object_ref!(LoadCompleteEvent);

impl_event_type_properties_for_event!(ReloadEvent);
event_test_cases!(ReloadEvent);
impl_to_dbus_message!(ReloadEvent);
impl_from_dbus_message!(ReloadEvent);
impl_event_properties!(ReloadEvent);
impl_from_object_ref!(ReloadEvent);

impl_event_type_properties_for_event!(LoadStoppedEvent);
event_test_cases!(LoadStoppedEvent);
impl_to_dbus_message!(LoadStoppedEvent);
impl_from_dbus_message!(LoadStoppedEvent);
impl_event_properties!(LoadStoppedEvent);
impl_from_object_ref!(LoadStoppedEvent);

impl_event_type_properties_for_event!(ContentChangedEvent);
event_test_cases!(ContentChangedEvent);
impl_to_dbus_message!(ContentChangedEvent);
impl_from_dbus_message!(ContentChangedEvent);
impl_event_properties!(ContentChangedEvent);
impl_from_object_ref!(ContentChangedEvent);

impl_event_type_properties_for_event!(AttributesChangedEvent);
event_test_cases!(AttributesChangedEvent);
impl_to_dbus_message!(AttributesChangedEvent);
impl_from_dbus_message!(AttributesChangedEvent);
impl_event_properties!(AttributesChangedEvent);
impl_from_object_ref!(AttributesChangedEvent);

impl_event_type_properties_for_event!(PageChangedEvent);
event_test_cases!(PageChangedEvent);
impl_to_dbus_message!(PageChangedEvent);
impl_from_dbus_message!(PageChangedEvent);
impl_event_properties!(PageChangedEvent);
impl_from_object_ref!(PageChangedEvent);
impl_msg_conversion_ext_for_target_type!(LoadCompleteEvent);
impl_msg_conversion_ext_for_target_type!(ReloadEvent);
impl_msg_conversion_ext_for_target_type!(LoadStoppedEvent);
impl_msg_conversion_ext_for_target_type!(ContentChangedEvent);
impl_msg_conversion_ext_for_target_type!(AttributesChangedEvent);
impl_msg_conversion_ext_for_target_type!(PageChangedEvent);

impl_msg_conversion_for_types_built_from_object_ref!(LoadCompleteEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ReloadEvent);
impl_msg_conversion_for_types_built_from_object_ref!(LoadStoppedEvent);
impl_msg_conversion_for_types_built_from_object_ref!(ContentChangedEvent);
impl_msg_conversion_for_types_built_from_object_ref!(AttributesChangedEvent);
impl_msg_conversion_for_types_built_from_object_ref!(PageChangedEvent);
