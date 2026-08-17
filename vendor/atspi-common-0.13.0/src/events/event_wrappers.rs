use crate::events::registry::socket::AvailableEvent;

use crate::events::registry::{EventListenerDeregisteredEvent, EventListenerRegisteredEvent};
#[cfg(feature = "zbus")]
use crate::events::traits::{EventWrapperMessageConversion, TryFromMessage};
#[cfg(feature = "zbus")]
use crate::events::MessageConversion;
use crate::{
	error::AtspiError,
	events::{
		cache::{AddAccessibleEvent, LegacyAddAccessibleEvent, RemoveAccessibleEvent},
		document::{
			AttributesChangedEvent as DocumentAttributesChangedEvent, ContentChangedEvent,
			LoadCompleteEvent, LoadStoppedEvent, PageChangedEvent, ReloadEvent,
		},
		focus::FocusEvent,
		keyboard::ModifiersEvent,
		mouse::{AbsEvent, ButtonEvent, RelEvent},
		object::{
			ActiveDescendantChangedEvent, AnnouncementEvent,
			AttributesChangedEvent as ObjectAttributesChangedEvent, BoundsChangedEvent,
			ChildrenChangedEvent, ColumnDeletedEvent, ColumnInsertedEvent, ColumnReorderedEvent,
			LinkSelectedEvent, ModelChangedEvent, PropertyChangeEvent as ObjectPropertyChangeEvent,
			RowDeletedEvent, RowInsertedEvent, RowReorderedEvent, SelectionChangedEvent,
			StateChangedEvent, TextAttributesChangedEvent, TextBoundsChangedEvent,
			TextCaretMovedEvent, TextChangedEvent, TextSelectionChangedEvent,
			VisibleDataChangedEvent,
		},
		terminal::{
			ApplicationChangedEvent, CharWidthChangedEvent, ColumnCountChangedEvent,
			LineChangedEvent, LineCountChangedEvent,
		},
		window::{
			ActivateEvent, CloseEvent, CreateEvent, DeactivateEvent, DesktopCreateEvent,
			DesktopDestroyEvent, DestroyEvent, LowerEvent, MaximizeEvent, MinimizeEvent, MoveEvent,
			PropertyChangeEvent as WindowPropertyChangeEvent, RaiseEvent, ReparentEvent,
			ResizeEvent, RestoreEvent, RestyleEvent, ShadeEvent, UUshadeEvent,
		},
		DBusInterface, DBusMatchRule, EventTypeProperties, RegistryEventString,
	},
	EventProperties,
};
#[cfg(feature = "zbus")]
use crate::{events::DBusMember, CacheItem, LegacyCacheItem};
use serde::{Deserialize, Serialize};
#[cfg(feature = "zbus")]
use zbus::message::Header;
use zbus_names::UniqueName;
use zvariant::ObjectPath;
#[cfg(feature = "zbus")]
use zvariant::Type;

impl_from_user_facing_event_for_interface_event_enum!(
	EventListenerRegisteredEvent,
	EventListenerEvents,
	EventListenerEvents::Registered
);

impl_from_user_facing_type_for_event_enum!(EventListenerRegisteredEvent, Event::Listener);

impl_try_from_event_for_user_facing_type!(
	EventListenerRegisteredEvent,
	EventListenerEvents::Registered,
	Event::Listener
);

impl_from_user_facing_event_for_interface_event_enum!(
	EventListenerDeregisteredEvent,
	EventListenerEvents,
	EventListenerEvents::Deregistered
);

impl_from_user_facing_type_for_event_enum!(EventListenerDeregisteredEvent, Event::Listener);
impl_try_from_event_for_user_facing_type!(
	EventListenerDeregisteredEvent,
	EventListenerEvents::Deregistered,
	Event::Listener
);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum KeyboardEvents {
	/// See: [`ModifiersEvent`].
	Modifiers(ModifiersEvent),
}

impl_tryfrommessage_for_event_wrapper!(KeyboardEvents);
impl_try_from_event_for_interface_enum!(KeyboardEvents, Event::Keyboard);
impl_from_interface_event_enum_for_event!(KeyboardEvents, Event::Keyboard);

impl_from_user_facing_event_for_interface_event_enum!(
	ModifiersEvent,
	KeyboardEvents,
	KeyboardEvents::Modifiers
);
impl EventTypeProperties for KeyboardEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.member(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.match_rule(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.interface(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Modifiers(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for KeyboardEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Modifiers(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Modifiers(inner) => inner.sender(),
		}
	}
}

event_wrapper_test_cases!(KeyboardEvents, ModifiersEvent);

impl DBusMatchRule for KeyboardEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Keyboard'";
}

impl DBusInterface for KeyboardEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
}

impl RegistryEventString for KeyboardEvents {
	const REGISTRY_EVENT_STRING: &'static str = "keyboard:";
}

impl_from_user_facing_type_for_event_enum!(ModifiersEvent, Event::Keyboard);

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for KeyboardEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_try_from_event_for_user_facing_type!(
	ModifiersEvent,
	KeyboardEvents::Modifiers,
	Event::Keyboard
);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum MouseEvents {
	/// See: [`AbsEvent`].
	Abs(AbsEvent),

	/// See: [`RelEvent`].
	Rel(RelEvent),

	/// See: [`ButtonEvent`].
	Button(ButtonEvent),
}

impl DBusMatchRule for MouseEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse'";
}

impl DBusInterface for MouseEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
}

impl RegistryEventString for MouseEvents {
	const REGISTRY_EVENT_STRING: &'static str = "mouse:";
}

impl_tryfrommessage_for_event_wrapper!(MouseEvents);

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for KeyboardEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			ModifiersEvent::DBUS_MEMBER => {
				Ok(KeyboardEvents::Modifiers(ModifiersEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
		}
	}
}

impl EventProperties for MouseEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Abs(inner) => inner.path(),
			Self::Rel(inner) => inner.path(),
			Self::Button(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Abs(inner) => inner.sender(),
			Self::Rel(inner) => inner.sender(),
			Self::Button(inner) => inner.sender(),
		}
	}
}

impl EventTypeProperties for MouseEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.member(),
			Self::Rel(inner) => inner.member(),
			Self::Button(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.interface(),
			Self::Rel(inner) => inner.interface(),
			Self::Button(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.match_rule(),
			Self::Rel(inner) => inner.match_rule(),
			Self::Button(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Abs(inner) => inner.registry_string(),
			Self::Rel(inner) => inner.registry_string(),
			Self::Button(inner) => inner.registry_string(),
		}
	}
}

event_wrapper_test_cases!(MouseEvents, AbsEvent);
impl_try_from_event_for_interface_enum!(MouseEvents, Event::Mouse);
impl_from_interface_event_enum_for_event!(MouseEvents, Event::Mouse);

impl_from_user_facing_event_for_interface_event_enum!(RelEvent, MouseEvents, MouseEvents::Rel);
impl_try_from_event_for_user_facing_type!(RelEvent, MouseEvents::Rel, Event::Mouse);
impl_from_user_facing_event_for_interface_event_enum!(
	ButtonEvent,
	MouseEvents,
	MouseEvents::Button
);
impl_try_from_event_for_user_facing_type!(ButtonEvent, MouseEvents::Button, Event::Mouse);

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for MouseEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			AbsEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Abs(AbsEvent::from_message_unchecked(msg, hdr)?))
			}
			RelEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Rel(RelEvent::from_message_unchecked(msg, hdr)?))
			}
			ButtonEvent::DBUS_MEMBER => {
				Ok(MouseEvents::Button(ButtonEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for MouseEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_type_for_event_enum!(ButtonEvent, Event::Mouse);
impl_from_user_facing_type_for_event_enum!(RelEvent, Event::Mouse);
impl_from_user_facing_type_for_event_enum!(AbsEvent, Event::Mouse);

impl_from_user_facing_event_for_interface_event_enum!(AbsEvent, MouseEvents, MouseEvents::Abs);
impl_try_from_event_for_user_facing_type!(AbsEvent, MouseEvents::Abs, Event::Mouse);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ObjectEvents {
	/// See: [`ObjectPropertyChangeEvent`].
	PropertyChange(ObjectPropertyChangeEvent),
	/// See: [`BoundsChangedEvent`].
	BoundsChanged(BoundsChangedEvent),
	/// See: [`LinkSelectedEvent`].
	LinkSelected(LinkSelectedEvent),
	/// See: [`StateChangedEvent`].
	StateChanged(StateChangedEvent),
	/// See: [`ChildrenChangedEvent`].
	ChildrenChanged(ChildrenChangedEvent),
	/// See: [`VisibleDataChangedEvent`].
	VisibleDataChanged(VisibleDataChangedEvent),
	/// See: [`SelectionChangedEvent`].
	SelectionChanged(SelectionChangedEvent),
	/// See: [`ModelChangedEvent`].
	ModelChanged(ModelChangedEvent),
	/// See: [`ActiveDescendantChangedEvent`].
	ActiveDescendantChanged(ActiveDescendantChangedEvent),
	/// See: [`AnnouncementEvent`].
	Announcement(AnnouncementEvent),
	/// See: [`ObjectAttributesChangedEvent`].
	AttributesChanged(ObjectAttributesChangedEvent),
	/// See: [`RowInsertedEvent`].
	RowInserted(RowInsertedEvent),
	/// See: [`RowReorderedEvent`].
	RowReordered(RowReorderedEvent),
	/// See: [`RowDeletedEvent`].
	RowDeleted(RowDeletedEvent),
	/// See: [`ColumnInsertedEvent`].
	ColumnInserted(ColumnInsertedEvent),
	/// See: [`ColumnReorderedEvent`].
	ColumnReordered(ColumnReorderedEvent),
	/// See: [`ColumnDeletedEvent`].
	ColumnDeleted(ColumnDeletedEvent),
	/// See: [`TextBoundsChangedEvent`].
	TextBoundsChanged(TextBoundsChangedEvent),
	/// See: [`TextSelectionChangedEvent`].
	TextSelectionChanged(TextSelectionChangedEvent),
	/// See: [`TextChangedEvent`].
	TextChanged(TextChangedEvent),
	/// See: [`TextAttributesChangedEvent`].
	TextAttributesChanged(TextAttributesChangedEvent),
	/// See: [`TextCaretMovedEvent`].
	TextCaretMoved(TextCaretMovedEvent),
}

impl_tryfrommessage_for_event_wrapper!(ObjectEvents);

impl EventTypeProperties for ObjectEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.member(),
			Self::BoundsChanged(inner) => inner.member(),
			Self::LinkSelected(inner) => inner.member(),
			Self::StateChanged(inner) => inner.member(),
			Self::ChildrenChanged(inner) => inner.member(),
			Self::VisibleDataChanged(inner) => inner.member(),
			Self::SelectionChanged(inner) => inner.member(),
			Self::ModelChanged(inner) => inner.member(),
			Self::ActiveDescendantChanged(inner) => inner.member(),
			Self::Announcement(inner) => inner.member(),
			Self::AttributesChanged(inner) => inner.member(),
			Self::RowInserted(inner) => inner.member(),
			Self::RowReordered(inner) => inner.member(),
			Self::RowDeleted(inner) => inner.member(),
			Self::ColumnInserted(inner) => inner.member(),
			Self::ColumnReordered(inner) => inner.member(),
			Self::ColumnDeleted(inner) => inner.member(),
			Self::TextBoundsChanged(inner) => inner.member(),
			Self::TextSelectionChanged(inner) => inner.member(),
			Self::TextChanged(inner) => inner.member(),
			Self::TextAttributesChanged(inner) => inner.member(),
			Self::TextCaretMoved(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.interface(),
			Self::BoundsChanged(inner) => inner.interface(),
			Self::LinkSelected(inner) => inner.interface(),
			Self::StateChanged(inner) => inner.interface(),
			Self::ChildrenChanged(inner) => inner.interface(),
			Self::VisibleDataChanged(inner) => inner.interface(),
			Self::SelectionChanged(inner) => inner.interface(),
			Self::ModelChanged(inner) => inner.interface(),
			Self::ActiveDescendantChanged(inner) => inner.interface(),
			Self::Announcement(inner) => inner.interface(),
			Self::AttributesChanged(inner) => inner.interface(),
			Self::RowInserted(inner) => inner.interface(),
			Self::RowReordered(inner) => inner.interface(),
			Self::RowDeleted(inner) => inner.interface(),
			Self::ColumnInserted(inner) => inner.interface(),
			Self::ColumnReordered(inner) => inner.interface(),
			Self::ColumnDeleted(inner) => inner.interface(),
			Self::TextBoundsChanged(inner) => inner.interface(),
			Self::TextSelectionChanged(inner) => inner.interface(),
			Self::TextChanged(inner) => inner.interface(),
			Self::TextAttributesChanged(inner) => inner.interface(),
			Self::TextCaretMoved(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.match_rule(),
			Self::BoundsChanged(inner) => inner.match_rule(),
			Self::LinkSelected(inner) => inner.match_rule(),
			Self::StateChanged(inner) => inner.match_rule(),
			Self::ChildrenChanged(inner) => inner.match_rule(),
			Self::VisibleDataChanged(inner) => inner.match_rule(),
			Self::SelectionChanged(inner) => inner.match_rule(),
			Self::ModelChanged(inner) => inner.match_rule(),
			Self::ActiveDescendantChanged(inner) => inner.match_rule(),
			Self::Announcement(inner) => inner.match_rule(),
			Self::AttributesChanged(inner) => inner.match_rule(),
			Self::RowInserted(inner) => inner.match_rule(),
			Self::RowReordered(inner) => inner.match_rule(),
			Self::RowDeleted(inner) => inner.match_rule(),
			Self::ColumnInserted(inner) => inner.match_rule(),
			Self::ColumnReordered(inner) => inner.match_rule(),
			Self::ColumnDeleted(inner) => inner.match_rule(),
			Self::TextBoundsChanged(inner) => inner.match_rule(),
			Self::TextSelectionChanged(inner) => inner.match_rule(),
			Self::TextChanged(inner) => inner.match_rule(),
			Self::TextAttributesChanged(inner) => inner.match_rule(),
			Self::TextCaretMoved(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.registry_string(),
			Self::BoundsChanged(inner) => inner.registry_string(),
			Self::LinkSelected(inner) => inner.registry_string(),
			Self::StateChanged(inner) => inner.registry_string(),
			Self::ChildrenChanged(inner) => inner.registry_string(),
			Self::VisibleDataChanged(inner) => inner.registry_string(),
			Self::SelectionChanged(inner) => inner.registry_string(),
			Self::ModelChanged(inner) => inner.registry_string(),
			Self::ActiveDescendantChanged(inner) => inner.registry_string(),
			Self::Announcement(inner) => inner.registry_string(),
			Self::AttributesChanged(inner) => inner.registry_string(),
			Self::RowInserted(inner) => inner.registry_string(),
			Self::RowReordered(inner) => inner.registry_string(),
			Self::RowDeleted(inner) => inner.registry_string(),
			Self::ColumnInserted(inner) => inner.registry_string(),
			Self::ColumnReordered(inner) => inner.registry_string(),
			Self::ColumnDeleted(inner) => inner.registry_string(),
			Self::TextBoundsChanged(inner) => inner.registry_string(),
			Self::TextSelectionChanged(inner) => inner.registry_string(),
			Self::TextChanged(inner) => inner.registry_string(),
			Self::TextAttributesChanged(inner) => inner.registry_string(),
			Self::TextCaretMoved(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for ObjectEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::PropertyChange(inner) => inner.path(),
			Self::BoundsChanged(inner) => inner.path(),
			Self::LinkSelected(inner) => inner.path(),
			Self::StateChanged(inner) => inner.path(),
			Self::ChildrenChanged(inner) => inner.path(),
			Self::VisibleDataChanged(inner) => inner.path(),
			Self::SelectionChanged(inner) => inner.path(),
			Self::ModelChanged(inner) => inner.path(),
			Self::ActiveDescendantChanged(inner) => inner.path(),
			Self::Announcement(inner) => inner.path(),
			Self::AttributesChanged(inner) => inner.path(),
			Self::RowInserted(inner) => inner.path(),
			Self::RowReordered(inner) => inner.path(),
			Self::RowDeleted(inner) => inner.path(),
			Self::ColumnInserted(inner) => inner.path(),
			Self::ColumnReordered(inner) => inner.path(),
			Self::ColumnDeleted(inner) => inner.path(),
			Self::TextBoundsChanged(inner) => inner.path(),
			Self::TextSelectionChanged(inner) => inner.path(),
			Self::TextChanged(inner) => inner.path(),
			Self::TextAttributesChanged(inner) => inner.path(),
			Self::TextCaretMoved(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::PropertyChange(inner) => inner.sender(),
			Self::BoundsChanged(inner) => inner.sender(),
			Self::LinkSelected(inner) => inner.sender(),
			Self::StateChanged(inner) => inner.sender(),
			Self::ChildrenChanged(inner) => inner.sender(),
			Self::VisibleDataChanged(inner) => inner.sender(),
			Self::SelectionChanged(inner) => inner.sender(),
			Self::ModelChanged(inner) => inner.sender(),
			Self::ActiveDescendantChanged(inner) => inner.sender(),
			Self::Announcement(inner) => inner.sender(),
			Self::AttributesChanged(inner) => inner.sender(),
			Self::RowInserted(inner) => inner.sender(),
			Self::RowReordered(inner) => inner.sender(),
			Self::RowDeleted(inner) => inner.sender(),
			Self::ColumnInserted(inner) => inner.sender(),
			Self::ColumnReordered(inner) => inner.sender(),
			Self::ColumnDeleted(inner) => inner.sender(),
			Self::TextBoundsChanged(inner) => inner.sender(),
			Self::TextSelectionChanged(inner) => inner.sender(),
			Self::TextChanged(inner) => inner.sender(),
			Self::TextAttributesChanged(inner) => inner.sender(),
			Self::TextCaretMoved(inner) => inner.sender(),
		}
	}
}

impl_try_from_event_for_interface_enum!(ObjectEvents, Event::Object);
impl_from_interface_event_enum_for_event!(ObjectEvents, Event::Object);

event_wrapper_test_cases!(ObjectEvents, ObjectPropertyChangeEvent);

impl_from_user_facing_event_for_interface_event_enum!(
	TextChangedEvent,
	ObjectEvents,
	ObjectEvents::TextChanged
);
impl_try_from_event_for_user_facing_type!(
	TextChangedEvent,
	ObjectEvents::TextChanged,
	Event::Object
);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum DocumentEvents {
	/// See: [`LoadCompleteEvent`].
	LoadComplete(LoadCompleteEvent),
	/// See: [`ReloadEvent`].
	Reload(ReloadEvent),
	/// See: [`LoadStoppedEvent`].
	LoadStopped(LoadStoppedEvent),
	/// See: [`ContentChangedEvent`].
	ContentChanged(ContentChangedEvent),
	/// See: [`DocumentAttributesChangedEvent`].
	AttributesChanged(DocumentAttributesChangedEvent),
	/// See: [`PageChangedEvent`].
	PageChanged(PageChangedEvent),
}

impl_tryfrommessage_for_event_wrapper!(DocumentEvents);

impl DBusInterface for DocumentEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
}

impl DBusMatchRule for DocumentEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Document'";
}

impl RegistryEventString for DocumentEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Document:";
}

impl EventTypeProperties for DocumentEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.member(),
			Self::Reload(inner) => inner.member(),
			Self::LoadStopped(inner) => inner.member(),
			Self::ContentChanged(inner) => inner.member(),
			Self::AttributesChanged(inner) => inner.member(),
			Self::PageChanged(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.interface(),
			Self::Reload(inner) => inner.interface(),
			Self::LoadStopped(inner) => inner.interface(),
			Self::ContentChanged(inner) => inner.interface(),
			Self::AttributesChanged(inner) => inner.interface(),
			Self::PageChanged(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.match_rule(),
			Self::Reload(inner) => inner.match_rule(),
			Self::LoadStopped(inner) => inner.match_rule(),
			Self::ContentChanged(inner) => inner.match_rule(),
			Self::AttributesChanged(inner) => inner.match_rule(),
			Self::PageChanged(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::LoadComplete(inner) => inner.registry_string(),
			Self::Reload(inner) => inner.registry_string(),
			Self::LoadStopped(inner) => inner.registry_string(),
			Self::ContentChanged(inner) => inner.registry_string(),
			Self::AttributesChanged(inner) => inner.registry_string(),
			Self::PageChanged(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for DocumentEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::LoadComplete(inner) => inner.path(),
			Self::Reload(inner) => inner.path(),
			Self::LoadStopped(inner) => inner.path(),
			Self::ContentChanged(inner) => inner.path(),
			Self::AttributesChanged(inner) => inner.path(),
			Self::PageChanged(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::LoadComplete(inner) => inner.sender(),
			Self::Reload(inner) => inner.sender(),
			Self::LoadStopped(inner) => inner.sender(),
			Self::ContentChanged(inner) => inner.sender(),
			Self::AttributesChanged(inner) => inner.sender(),
			Self::PageChanged(inner) => inner.sender(),
		}
	}
}

impl_from_user_facing_type_for_event_enum!(PageChangedEvent, Event::Document);
impl_from_user_facing_type_for_event_enum!(DocumentAttributesChangedEvent, Event::Document);
impl_from_user_facing_type_for_event_enum!(ContentChangedEvent, Event::Document);
impl_from_user_facing_type_for_event_enum!(LoadStoppedEvent, Event::Document);
impl_from_user_facing_type_for_event_enum!(ReloadEvent, Event::Document);
impl_from_user_facing_type_for_event_enum!(LoadCompleteEvent, Event::Document);

impl_try_from_event_for_interface_enum!(DocumentEvents, Event::Document);
impl_from_interface_event_enum_for_event!(DocumentEvents, Event::Document);

event_wrapper_test_cases!(DocumentEvents, LoadCompleteEvent);

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for DocumentEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			LoadCompleteEvent::DBUS_MEMBER => Ok(DocumentEvents::LoadComplete(
				LoadCompleteEvent::from_message_unchecked(msg, hdr)?,
			)),
			ReloadEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::Reload(ReloadEvent::from_message_unchecked(msg, hdr)?))
			}
			LoadStoppedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::LoadStopped(LoadStoppedEvent::from_message_unchecked(msg, hdr)?))
			}
			ContentChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::ContentChanged(
				ContentChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			DocumentAttributesChangedEvent::DBUS_MEMBER => Ok(DocumentEvents::AttributesChanged(
				DocumentAttributesChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			PageChangedEvent::DBUS_MEMBER => {
				Ok(DocumentEvents::PageChanged(PageChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for DocumentEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LoadCompleteEvent,
	DocumentEvents,
	DocumentEvents::LoadComplete
);
impl_try_from_event_for_user_facing_type!(
	LoadCompleteEvent,
	DocumentEvents::LoadComplete,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	ReloadEvent,
	DocumentEvents,
	DocumentEvents::Reload
);
impl_try_from_event_for_user_facing_type!(ReloadEvent, DocumentEvents::Reload, Event::Document);

impl_from_user_facing_event_for_interface_event_enum!(
	LoadStoppedEvent,
	DocumentEvents,
	DocumentEvents::LoadStopped
);
impl_try_from_event_for_user_facing_type!(
	LoadStoppedEvent,
	DocumentEvents::LoadStopped,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	ContentChangedEvent,
	DocumentEvents,
	DocumentEvents::ContentChanged
);
impl_try_from_event_for_user_facing_type!(
	ContentChangedEvent,
	DocumentEvents::ContentChanged,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	DocumentAttributesChangedEvent,
	DocumentEvents,
	DocumentEvents::AttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	DocumentAttributesChangedEvent,
	DocumentEvents::AttributesChanged,
	Event::Document
);

impl_from_user_facing_event_for_interface_event_enum!(
	PageChangedEvent,
	DocumentEvents,
	DocumentEvents::PageChanged
);
impl_try_from_event_for_user_facing_type!(
	PageChangedEvent,
	DocumentEvents::PageChanged,
	Event::Document
);

/// Encapsulates the various different accessibility bus signal types.
///
/// Assumes being non exhaustive to allow for future- or custom signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event {
	/// See: [`DocumentEvents`].
	Document(DocumentEvents),
	/// See: [`FocusEvents`].
	Focus(FocusEvents),
	/// See: [`KeyboardEvents`].
	Keyboard(KeyboardEvents),
	/// See: [`MouseEvents`].
	Mouse(MouseEvents),
	/// See: [`ObjectEvents`].
	Object(ObjectEvents),
	/// See: [`TerminalEvents`].
	Terminal(TerminalEvents),
	/// See: [`WindowEvents`].
	Window(WindowEvents),
	/// See: [`AvailableEvent`].
	Available(AvailableEvent),
	/// See: [`CacheEvents`].
	Cache(CacheEvents),
	/// See: [`EventListenerEvents`].
	Listener(EventListenerEvents),
}

impl EventTypeProperties for Event {
	fn member(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.member(),
			Self::Focus(inner) => inner.member(),
			Self::Keyboard(inner) => inner.member(),
			Self::Mouse(inner) => inner.member(),
			Self::Object(inner) => inner.member(),
			Self::Terminal(inner) => inner.member(),
			Self::Window(inner) => inner.member(),
			Self::Available(inner) => inner.member(),
			Self::Cache(inner) => inner.member(),
			Self::Listener(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.interface(),
			Self::Focus(inner) => inner.interface(),
			Self::Keyboard(inner) => inner.interface(),
			Self::Mouse(inner) => inner.interface(),
			Self::Object(inner) => inner.interface(),
			Self::Terminal(inner) => inner.interface(),
			Self::Window(inner) => inner.interface(),
			Self::Available(inner) => inner.interface(),
			Self::Cache(inner) => inner.interface(),
			Self::Listener(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.match_rule(),
			Self::Focus(inner) => inner.match_rule(),
			Self::Keyboard(inner) => inner.match_rule(),
			Self::Mouse(inner) => inner.match_rule(),
			Self::Object(inner) => inner.match_rule(),
			Self::Terminal(inner) => inner.match_rule(),
			Self::Window(inner) => inner.match_rule(),
			Self::Available(inner) => inner.match_rule(),
			Self::Cache(inner) => inner.match_rule(),
			Self::Listener(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Document(inner) => inner.registry_string(),
			Self::Focus(inner) => inner.registry_string(),
			Self::Keyboard(inner) => inner.registry_string(),
			Self::Mouse(inner) => inner.registry_string(),
			Self::Object(inner) => inner.registry_string(),
			Self::Terminal(inner) => inner.registry_string(),
			Self::Window(inner) => inner.registry_string(),
			Self::Available(inner) => inner.registry_string(),
			Self::Cache(inner) => inner.registry_string(),
			Self::Listener(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for Event {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Document(inner) => inner.path(),
			Self::Focus(inner) => inner.path(),
			Self::Keyboard(inner) => inner.path(),
			Self::Mouse(inner) => inner.path(),
			Self::Object(inner) => inner.path(),
			Self::Terminal(inner) => inner.path(),
			Self::Window(inner) => inner.path(),
			Self::Available(inner) => inner.path(),
			Self::Cache(inner) => inner.path(),
			Self::Listener(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Document(inner) => inner.sender(),
			Self::Focus(inner) => inner.sender(),
			Self::Keyboard(inner) => inner.sender(),
			Self::Mouse(inner) => inner.sender(),
			Self::Object(inner) => inner.sender(),
			Self::Terminal(inner) => inner.sender(),
			Self::Window(inner) => inner.sender(),
			Self::Available(inner) => inner.sender(),
			Self::Cache(inner) => inner.sender(),
			Self::Listener(inner) => inner.sender(),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for Event {
	type Error = AtspiError;

	fn try_from(msg: &zbus::Message) -> Result<Event, AtspiError> {
		let header = msg.header();
		let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
		let interface_str = interface.as_str();

		match interface_str {
			<ObjectEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Object(ObjectEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<FocusEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Focus(FocusEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<CacheEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Cache(CacheEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<WindowEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Window(WindowEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<MouseEvents as DBusInterface>::DBUS_INTERFACE => {
				Ok(Event::Mouse(MouseEvents::try_from_message_interface_checked(msg, &header)?))
			}
			<TerminalEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Terminal(
				TerminalEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<DocumentEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Document(
				DocumentEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<KeyboardEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Keyboard(
				KeyboardEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<EventListenerEvents as DBusInterface>::DBUS_INTERFACE => Ok(Event::Listener(
				EventListenerEvents::try_from_message_interface_checked(msg, &header)?,
			)),
			<AvailableEvent as DBusInterface>::DBUS_INTERFACE => {
				Ok(AvailableEvent::try_from(msg)?.into())
			}
			_ => Err(AtspiError::InterfaceMatch(format!(
				"No events found with interface {interface_str}"
			))),
		}
	}
}

impl_from_user_facing_type_for_event_enum!(TextCaretMovedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextAttributesChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextSelectionChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(TextBoundsChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ColumnDeletedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ColumnReorderedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ColumnInsertedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(RowDeletedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(RowReorderedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(RowInsertedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ObjectAttributesChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(AnnouncementEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ActiveDescendantChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ModelChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(SelectionChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(VisibleDataChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ChildrenChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(StateChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(LinkSelectedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(BoundsChangedEvent, Event::Object);
impl_from_user_facing_type_for_event_enum!(ObjectPropertyChangeEvent, Event::Object);

impl_from_user_facing_event_for_interface_event_enum!(
	ObjectPropertyChangeEvent,
	ObjectEvents,
	ObjectEvents::PropertyChange
);
impl_try_from_event_for_user_facing_type!(
	ObjectPropertyChangeEvent,
	ObjectEvents::PropertyChange,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	BoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::BoundsChanged
);
impl_try_from_event_for_user_facing_type!(
	BoundsChangedEvent,
	ObjectEvents::BoundsChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	LinkSelectedEvent,
	ObjectEvents,
	ObjectEvents::LinkSelected
);
impl_try_from_event_for_user_facing_type!(
	LinkSelectedEvent,
	ObjectEvents::LinkSelected,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	StateChangedEvent,
	ObjectEvents,
	ObjectEvents::StateChanged
);
impl_try_from_event_for_user_facing_type!(
	StateChangedEvent,
	ObjectEvents::StateChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ChildrenChangedEvent,
	ObjectEvents,
	ObjectEvents::ChildrenChanged
);
impl_try_from_event_for_user_facing_type!(
	ChildrenChangedEvent,
	ObjectEvents::ChildrenChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	VisibleDataChangedEvent,
	ObjectEvents,
	ObjectEvents::VisibleDataChanged
);
impl_try_from_event_for_user_facing_type!(
	VisibleDataChangedEvent,
	ObjectEvents::VisibleDataChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	SelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::SelectionChanged
);
impl_try_from_event_for_user_facing_type!(
	SelectionChangedEvent,
	ObjectEvents::SelectionChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ModelChangedEvent,
	ObjectEvents,
	ObjectEvents::ModelChanged
);
impl_try_from_event_for_user_facing_type!(
	ModelChangedEvent,
	ObjectEvents::ModelChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ActiveDescendantChangedEvent,
	ObjectEvents,
	ObjectEvents::ActiveDescendantChanged
);
impl_try_from_event_for_user_facing_type!(
	ActiveDescendantChangedEvent,
	ObjectEvents::ActiveDescendantChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	AnnouncementEvent,
	ObjectEvents,
	ObjectEvents::Announcement
);
impl_try_from_event_for_user_facing_type!(
	AnnouncementEvent,
	ObjectEvents::Announcement,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ObjectAttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::AttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	ObjectAttributesChangedEvent,
	ObjectEvents::AttributesChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowInsertedEvent,
	ObjectEvents,
	ObjectEvents::RowInserted
);
impl_try_from_event_for_user_facing_type!(
	RowInsertedEvent,
	ObjectEvents::RowInserted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowReorderedEvent,
	ObjectEvents,
	ObjectEvents::RowReordered
);
impl_try_from_event_for_user_facing_type!(
	RowReorderedEvent,
	ObjectEvents::RowReordered,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	RowDeletedEvent,
	ObjectEvents,
	ObjectEvents::RowDeleted
);
impl_try_from_event_for_user_facing_type!(RowDeletedEvent, ObjectEvents::RowDeleted, Event::Object);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnInsertedEvent,
	ObjectEvents,
	ObjectEvents::ColumnInserted
);
impl_try_from_event_for_user_facing_type!(
	ColumnInsertedEvent,
	ObjectEvents::ColumnInserted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnReorderedEvent,
	ObjectEvents,
	ObjectEvents::ColumnReordered
);
impl_try_from_event_for_user_facing_type!(
	ColumnReorderedEvent,
	ObjectEvents::ColumnReordered,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnDeletedEvent,
	ObjectEvents,
	ObjectEvents::ColumnDeleted
);
impl_try_from_event_for_user_facing_type!(
	ColumnDeletedEvent,
	ObjectEvents::ColumnDeleted,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextBoundsChangedEvent,
	ObjectEvents,
	ObjectEvents::TextBoundsChanged
);
impl_try_from_event_for_user_facing_type!(
	TextBoundsChangedEvent,
	ObjectEvents::TextBoundsChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextSelectionChangedEvent,
	ObjectEvents,
	ObjectEvents::TextSelectionChanged
);
impl_try_from_event_for_user_facing_type!(
	TextSelectionChangedEvent,
	ObjectEvents::TextSelectionChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextAttributesChangedEvent,
	ObjectEvents,
	ObjectEvents::TextAttributesChanged
);
impl_try_from_event_for_user_facing_type!(
	TextAttributesChangedEvent,
	ObjectEvents::TextAttributesChanged,
	Event::Object
);
impl_from_user_facing_event_for_interface_event_enum!(
	TextCaretMovedEvent,
	ObjectEvents,
	ObjectEvents::TextCaretMoved
);
impl_try_from_event_for_user_facing_type!(
	TextCaretMovedEvent,
	ObjectEvents::TextCaretMoved,
	Event::Object
);

impl DBusMatchRule for ObjectEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object'";
}

impl DBusInterface for ObjectEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
}

impl RegistryEventString for ObjectEvents {
	const REGISTRY_EVENT_STRING: &'static str = "object:";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for ObjectEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			ObjectPropertyChangeEvent::DBUS_MEMBER => Ok(ObjectEvents::PropertyChange(
				ObjectPropertyChangeEvent::from_message_unchecked(msg, hdr)?,
			)),
			BoundsChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::BoundsChanged(
				BoundsChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			LinkSelectedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::LinkSelected(LinkSelectedEvent::from_message_unchecked(msg, hdr)?))
			}
			StateChangedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::StateChanged(StateChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			ChildrenChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::ChildrenChanged(
				ChildrenChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			VisibleDataChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::VisibleDataChanged(
				VisibleDataChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			SelectionChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::SelectionChanged(
				SelectionChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ModelChangedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::ModelChanged(ModelChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			ActiveDescendantChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::ActiveDescendantChanged(
				ActiveDescendantChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			AnnouncementEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::Announcement(AnnouncementEvent::from_message_unchecked(msg, hdr)?))
			}
			ObjectAttributesChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::AttributesChanged(
				ObjectAttributesChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			RowInsertedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::RowInserted(RowInsertedEvent::from_message_unchecked(msg, hdr)?))
			}
			RowReorderedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::RowReordered(RowReorderedEvent::from_message_unchecked(msg, hdr)?))
			}
			RowDeletedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::RowDeleted(RowDeletedEvent::from_message_unchecked(msg, hdr)?))
			}
			ColumnInsertedEvent::DBUS_MEMBER => Ok(ObjectEvents::ColumnInserted(
				ColumnInsertedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ColumnReorderedEvent::DBUS_MEMBER => Ok(ObjectEvents::ColumnReordered(
				ColumnReorderedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ColumnDeletedEvent::DBUS_MEMBER => Ok(ObjectEvents::ColumnDeleted(
				ColumnDeletedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextBoundsChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextBoundsChanged(
				TextBoundsChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextSelectionChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextSelectionChanged(
				TextSelectionChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextChangedEvent::DBUS_MEMBER => {
				Ok(ObjectEvents::TextChanged(TextChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			TextAttributesChangedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextAttributesChanged(
				TextAttributesChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			TextCaretMovedEvent::DBUS_MEMBER => Ok(ObjectEvents::TextCaretMoved(
				TextCaretMovedEvent::from_message_unchecked(msg, hdr)?,
			)),
			_ => Err(AtspiError::MemberMatch(format!(
				"No matching member {member} for interface {}",
				Self::DBUS_INTERFACE,
			))),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for ObjectEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

/// All events related to the `org.a11y.atspi.Cache` interface.
/// Note that these are not telling the client that an item *has been added* to a cache.
/// It is telling the client "here is a bunch of information to store it in your cache".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum CacheEvents {
	/// See: [`AddAccessibleEvent`].
	Add(AddAccessibleEvent),
	/// See: [`LegacyAddAccessibleEvent`].
	LegacyAdd(LegacyAddAccessibleEvent),
	/// See: [`RemoveAccessibleEvent`].
	Remove(RemoveAccessibleEvent),
}

impl_from_user_facing_type_for_event_enum!(RemoveAccessibleEvent, Event::Cache);
impl_from_user_facing_type_for_event_enum!(AddAccessibleEvent, Event::Cache);
impl_from_user_facing_type_for_event_enum!(LegacyAddAccessibleEvent, Event::Cache);

impl DBusMatchRule for CacheEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Cache'";
}

impl RegistryEventString for CacheEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Cache";
}

impl DBusInterface for CacheEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Cache";
}

impl EventTypeProperties for CacheEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.member(),
			Self::LegacyAdd(inner) => inner.member(),
			Self::Remove(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.interface(),
			Self::LegacyAdd(inner) => inner.interface(),
			Self::Remove(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.match_rule(),
			Self::LegacyAdd(inner) => inner.match_rule(),
			Self::Remove(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Add(inner) => inner.registry_string(),
			Self::LegacyAdd(inner) => inner.registry_string(),
			Self::Remove(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for CacheEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Add(inner) => inner.path(),
			Self::LegacyAdd(inner) => inner.path(),
			Self::Remove(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Add(inner) => inner.sender(),
			Self::LegacyAdd(inner) => inner.sender(),
			Self::Remove(inner) => inner.sender(),
		}
	}
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for CacheEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			AddAccessibleEvent::DBUS_MEMBER => {
				let body = msg.body();
				let sig = body.signature();
				if sig == CacheItem::SIGNATURE {
					Ok(CacheEvents::Add(AddAccessibleEvent::from_message_unchecked(msg, hdr)?))
				} else if sig == LegacyCacheItem::SIGNATURE {
					Ok(CacheEvents::LegacyAdd(LegacyAddAccessibleEvent::from_message_unchecked(
						msg, hdr,
					)?))
				} else {
					Err(AtspiError::SignatureMatch(format!(
						"No matching event for signature {} in interface {}",
						&sig.to_string(),
						Self::DBUS_INTERFACE
					)))
				}
			}
			RemoveAccessibleEvent::DBUS_MEMBER => {
				Ok(CacheEvents::Remove(RemoveAccessibleEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch(format!(
				"No member {} in {}",
				member.as_str(),
				Self::DBUS_INTERFACE
			))),
		}
	}
}

impl_tryfrommessage_for_event_wrapper!(CacheEvents);

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for CacheEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LegacyAddAccessibleEvent,
	CacheEvents,
	CacheEvents::LegacyAdd
);
impl_try_from_event_for_user_facing_type!(
	LegacyAddAccessibleEvent,
	CacheEvents::LegacyAdd,
	Event::Cache
);
impl_from_user_facing_event_for_interface_event_enum!(
	AddAccessibleEvent,
	CacheEvents,
	CacheEvents::Add
);
impl_try_from_event_for_user_facing_type!(AddAccessibleEvent, CacheEvents::Add, Event::Cache);
impl_from_user_facing_event_for_interface_event_enum!(
	RemoveAccessibleEvent,
	CacheEvents,
	CacheEvents::Remove
);
impl_try_from_event_for_user_facing_type!(RemoveAccessibleEvent, CacheEvents::Remove, Event::Cache);
impl_try_from_event_for_interface_enum!(CacheEvents, Event::Cache);

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum FocusEvents {
	/// See: [`FocusEvent`].
	Focus(FocusEvent),
}

impl_tryfrommessage_for_event_wrapper!(FocusEvents);

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for FocusEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_interface_event_enum_for_event!(FocusEvents, Event::Focus);
impl_try_from_event_for_user_facing_type!(FocusEvent, FocusEvents::Focus, Event::Focus);

impl EventTypeProperties for FocusEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.member(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.match_rule(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.interface(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::Focus(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for FocusEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Focus(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Focus(inner) => inner.sender(),
		}
	}
}

impl_try_from_event_for_interface_enum!(FocusEvents, Event::Focus);

impl_from_user_facing_event_for_interface_event_enum!(FocusEvent, FocusEvents, FocusEvents::Focus);
impl_from_user_facing_type_for_event_enum!(FocusEvent, Event::Focus);
event_wrapper_test_cases!(FocusEvents, FocusEvent);

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for FocusEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			FocusEvent::DBUS_MEMBER => {
				Ok(FocusEvents::Focus(FocusEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch(format!(
				"No matching member {member} for interface {}",
				Self::DBUS_INTERFACE,
			))),
		}
	}
}

impl DBusMatchRule for FocusEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
}

impl RegistryEventString for FocusEvents {
	const REGISTRY_EVENT_STRING: &'static str = "focus:";
}

impl DBusInterface for FocusEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
}

/// All events related to the `org.a11y.atspi.Event.Terminal` interface.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum TerminalEvents {
	/// See: [`LineChangedEvent`].
	LineChanged(LineChangedEvent),
	/// See: [`ColumnCountChangedEvent`].
	ColumnCountChanged(ColumnCountChangedEvent),
	/// See: [`LineCountChangedEvent`].
	LineCountChanged(LineCountChangedEvent),
	/// See: [`ApplicationChangedEvent`].
	ApplicationChanged(ApplicationChangedEvent),
	/// See: [`CharWidthChangedEvent`].
	CharWidthChanged(CharWidthChangedEvent),
}

impl_tryfrommessage_for_event_wrapper!(TerminalEvents);

impl EventTypeProperties for TerminalEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.member(),
			Self::ColumnCountChanged(inner) => inner.member(),
			Self::LineCountChanged(inner) => inner.member(),
			Self::ApplicationChanged(inner) => inner.member(),
			Self::CharWidthChanged(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.interface(),
			Self::ColumnCountChanged(inner) => inner.interface(),
			Self::LineCountChanged(inner) => inner.interface(),
			Self::ApplicationChanged(inner) => inner.interface(),
			Self::CharWidthChanged(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.match_rule(),
			Self::ColumnCountChanged(inner) => inner.match_rule(),
			Self::LineCountChanged(inner) => inner.match_rule(),
			Self::ApplicationChanged(inner) => inner.match_rule(),
			Self::CharWidthChanged(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::LineChanged(inner) => inner.registry_string(),
			Self::ColumnCountChanged(inner) => inner.registry_string(),
			Self::LineCountChanged(inner) => inner.registry_string(),
			Self::ApplicationChanged(inner) => inner.registry_string(),
			Self::CharWidthChanged(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for TerminalEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::LineChanged(inner) => inner.path(),
			Self::ColumnCountChanged(inner) => inner.path(),
			Self::LineCountChanged(inner) => inner.path(),
			Self::ApplicationChanged(inner) => inner.path(),
			Self::CharWidthChanged(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::LineChanged(inner) => inner.sender(),
			Self::ColumnCountChanged(inner) => inner.sender(),
			Self::LineCountChanged(inner) => inner.sender(),
			Self::ApplicationChanged(inner) => inner.sender(),
			Self::CharWidthChanged(inner) => inner.sender(),
		}
	}
}

impl_from_user_facing_type_for_event_enum!(CharWidthChangedEvent, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(ApplicationChangedEvent, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(LineCountChangedEvent, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(ColumnCountChangedEvent, Event::Terminal);
impl_from_user_facing_type_for_event_enum!(LineChangedEvent, Event::Terminal);

impl_try_from_event_for_interface_enum!(TerminalEvents, Event::Terminal);
impl_from_interface_event_enum_for_event!(TerminalEvents, Event::Terminal);

event_wrapper_test_cases!(TerminalEvents, LineChangedEvent);

impl DBusMatchRule for TerminalEvents {
	const MATCH_RULE_STRING: &'static str =
		"type='signal',interface='org.a11y.atspi.Event.Terminal'";
}

impl RegistryEventString for TerminalEvents {
	const REGISTRY_EVENT_STRING: &'static str = "terminal:";
}

impl DBusInterface for TerminalEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for TerminalEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr
			.member()
			.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
		match member.as_str() {
			LineChangedEvent::DBUS_MEMBER => {
				Ok(TerminalEvents::LineChanged(LineChangedEvent::from_message_unchecked(msg, hdr)?))
			}
			ColumnCountChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::ColumnCountChanged(
				ColumnCountChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			LineCountChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::LineCountChanged(
				LineCountChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			ApplicationChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::ApplicationChanged(
				ApplicationChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			CharWidthChangedEvent::DBUS_MEMBER => Ok(TerminalEvents::CharWidthChanged(
				CharWidthChangedEvent::from_message_unchecked(msg, hdr)?,
			)),
			_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for TerminalEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	LineChangedEvent,
	TerminalEvents,
	TerminalEvents::LineChanged
);
impl_try_from_event_for_user_facing_type!(
	LineChangedEvent,
	TerminalEvents::LineChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	ColumnCountChangedEvent,
	TerminalEvents,
	TerminalEvents::ColumnCountChanged
);
impl_try_from_event_for_user_facing_type!(
	ColumnCountChangedEvent,
	TerminalEvents::ColumnCountChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	LineCountChangedEvent,
	TerminalEvents,
	TerminalEvents::LineCountChanged
);
impl_try_from_event_for_user_facing_type!(
	LineCountChangedEvent,
	TerminalEvents::LineCountChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	ApplicationChangedEvent,
	TerminalEvents,
	TerminalEvents::ApplicationChanged
);
impl_try_from_event_for_user_facing_type!(
	ApplicationChangedEvent,
	TerminalEvents::ApplicationChanged,
	Event::Terminal
);
impl_from_user_facing_event_for_interface_event_enum!(
	CharWidthChangedEvent,
	TerminalEvents,
	TerminalEvents::CharWidthChanged
);
impl_try_from_event_for_user_facing_type!(
	CharWidthChangedEvent,
	TerminalEvents::CharWidthChanged,
	Event::Terminal
);

/// All events on the `org.a11y.atspi.Event.Window` interface.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum WindowEvents {
	/// See: [`WindowPropertyChangeEvent`].
	PropertyChange(WindowPropertyChangeEvent),
	/// See: [`MinimizeEvent`].
	Minimize(MinimizeEvent),
	/// See: [`MaximizeEvent`].
	Maximize(MaximizeEvent),
	/// See: [`RestoreEvent`].
	Restore(RestoreEvent),
	/// See: [`CloseEvent`].
	Close(CloseEvent),
	/// See: [`CreateEvent`].
	Create(CreateEvent),
	/// See: [`ReparentEvent`].
	Reparent(ReparentEvent),
	/// See: [`DesktopCreateEvent`].
	DesktopCreate(DesktopCreateEvent),
	/// See: [`DesktopDestroyEvent`].
	DesktopDestroy(DesktopDestroyEvent),
	/// See: [`DestroyEvent`].
	Destroy(DestroyEvent),
	/// See: [`ActivateEvent`].
	Activate(ActivateEvent),
	/// See: [`DeactivateEvent`].
	Deactivate(DeactivateEvent),
	/// See: [`RaiseEvent`].
	Raise(RaiseEvent),
	/// See: [`LowerEvent`].
	Lower(LowerEvent),
	/// See: [`MoveEvent`].
	Move(MoveEvent),
	/// See: [`ResizeEvent`].
	Resize(ResizeEvent),
	/// See: [`ShadeEvent`].
	Shade(ShadeEvent),
	/// See: [`UUshadeEvent`].
	UUshade(UUshadeEvent),
	/// See: [`RestyleEvent`].
	Restyle(RestyleEvent),
}

impl_tryfrommessage_for_event_wrapper!(WindowEvents);

impl EventTypeProperties for WindowEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.member(),
			Self::Minimize(inner) => inner.member(),
			Self::Maximize(inner) => inner.member(),
			Self::Restore(inner) => inner.member(),
			Self::Close(inner) => inner.member(),
			Self::Create(inner) => inner.member(),
			Self::Reparent(inner) => inner.member(),
			Self::DesktopCreate(inner) => inner.member(),
			Self::DesktopDestroy(inner) => inner.member(),
			Self::Destroy(inner) => inner.member(),
			Self::Activate(inner) => inner.member(),
			Self::Deactivate(inner) => inner.member(),
			Self::Raise(inner) => inner.member(),
			Self::Lower(inner) => inner.member(),
			Self::Move(inner) => inner.member(),
			Self::Resize(inner) => inner.member(),
			Self::Shade(inner) => inner.member(),
			Self::UUshade(inner) => inner.member(),
			Self::Restyle(inner) => inner.member(),
		}
	}
	fn interface(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.interface(),
			Self::Minimize(inner) => inner.interface(),
			Self::Maximize(inner) => inner.interface(),
			Self::Restore(inner) => inner.interface(),
			Self::Close(inner) => inner.interface(),
			Self::Create(inner) => inner.interface(),
			Self::Reparent(inner) => inner.interface(),
			Self::DesktopCreate(inner) => inner.interface(),
			Self::DesktopDestroy(inner) => inner.interface(),
			Self::Destroy(inner) => inner.interface(),
			Self::Activate(inner) => inner.interface(),
			Self::Deactivate(inner) => inner.interface(),
			Self::Raise(inner) => inner.interface(),
			Self::Lower(inner) => inner.interface(),
			Self::Move(inner) => inner.interface(),
			Self::Resize(inner) => inner.interface(),
			Self::Shade(inner) => inner.interface(),
			Self::UUshade(inner) => inner.interface(),
			Self::Restyle(inner) => inner.interface(),
		}
	}
	fn match_rule(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.match_rule(),
			Self::Minimize(inner) => inner.match_rule(),
			Self::Maximize(inner) => inner.match_rule(),
			Self::Restore(inner) => inner.match_rule(),
			Self::Close(inner) => inner.match_rule(),
			Self::Create(inner) => inner.match_rule(),
			Self::Reparent(inner) => inner.match_rule(),
			Self::DesktopCreate(inner) => inner.match_rule(),
			Self::DesktopDestroy(inner) => inner.match_rule(),
			Self::Destroy(inner) => inner.match_rule(),
			Self::Activate(inner) => inner.match_rule(),
			Self::Deactivate(inner) => inner.match_rule(),
			Self::Raise(inner) => inner.match_rule(),
			Self::Lower(inner) => inner.match_rule(),
			Self::Move(inner) => inner.match_rule(),
			Self::Resize(inner) => inner.match_rule(),
			Self::Shade(inner) => inner.match_rule(),
			Self::UUshade(inner) => inner.match_rule(),
			Self::Restyle(inner) => inner.match_rule(),
		}
	}
	fn registry_string(&self) -> &'static str {
		match self {
			Self::PropertyChange(inner) => inner.registry_string(),
			Self::Minimize(inner) => inner.registry_string(),
			Self::Maximize(inner) => inner.registry_string(),
			Self::Restore(inner) => inner.registry_string(),
			Self::Close(inner) => inner.registry_string(),
			Self::Create(inner) => inner.registry_string(),
			Self::Reparent(inner) => inner.registry_string(),
			Self::DesktopCreate(inner) => inner.registry_string(),
			Self::DesktopDestroy(inner) => inner.registry_string(),
			Self::Destroy(inner) => inner.registry_string(),
			Self::Activate(inner) => inner.registry_string(),
			Self::Deactivate(inner) => inner.registry_string(),
			Self::Raise(inner) => inner.registry_string(),
			Self::Lower(inner) => inner.registry_string(),
			Self::Move(inner) => inner.registry_string(),
			Self::Resize(inner) => inner.registry_string(),
			Self::Shade(inner) => inner.registry_string(),
			Self::UUshade(inner) => inner.registry_string(),
			Self::Restyle(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for WindowEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::PropertyChange(inner) => inner.path(),
			Self::Minimize(inner) => inner.path(),
			Self::Maximize(inner) => inner.path(),
			Self::Restore(inner) => inner.path(),
			Self::Close(inner) => inner.path(),
			Self::Create(inner) => inner.path(),
			Self::Reparent(inner) => inner.path(),
			Self::DesktopCreate(inner) => inner.path(),
			Self::DesktopDestroy(inner) => inner.path(),
			Self::Destroy(inner) => inner.path(),
			Self::Activate(inner) => inner.path(),
			Self::Deactivate(inner) => inner.path(),
			Self::Raise(inner) => inner.path(),
			Self::Lower(inner) => inner.path(),
			Self::Move(inner) => inner.path(),
			Self::Resize(inner) => inner.path(),
			Self::Shade(inner) => inner.path(),
			Self::UUshade(inner) => inner.path(),
			Self::Restyle(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::PropertyChange(inner) => inner.sender(),
			Self::Minimize(inner) => inner.sender(),
			Self::Maximize(inner) => inner.sender(),
			Self::Restore(inner) => inner.sender(),
			Self::Close(inner) => inner.sender(),
			Self::Create(inner) => inner.sender(),
			Self::Reparent(inner) => inner.sender(),
			Self::DesktopCreate(inner) => inner.sender(),
			Self::DesktopDestroy(inner) => inner.sender(),
			Self::Destroy(inner) => inner.sender(),
			Self::Activate(inner) => inner.sender(),
			Self::Deactivate(inner) => inner.sender(),
			Self::Raise(inner) => inner.sender(),
			Self::Lower(inner) => inner.sender(),
			Self::Move(inner) => inner.sender(),
			Self::Resize(inner) => inner.sender(),
			Self::Shade(inner) => inner.sender(),
			Self::UUshade(inner) => inner.sender(),
			Self::Restyle(inner) => inner.sender(),
		}
	}
}

impl_from_user_facing_type_for_event_enum!(ReparentEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(CloseEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(RestoreEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(MaximizeEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(MinimizeEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(WindowPropertyChangeEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(RestyleEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(UUshadeEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(ShadeEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(ResizeEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(MoveEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(LowerEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(RaiseEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(DeactivateEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(ActivateEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(DestroyEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(DesktopDestroyEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(DesktopCreateEvent, Event::Window);
impl_from_user_facing_type_for_event_enum!(CreateEvent, Event::Window);

impl_try_from_event_for_interface_enum!(WindowEvents, Event::Window);
impl_from_interface_event_enum_for_event!(WindowEvents, Event::Window);

event_wrapper_test_cases!(WindowEvents, MoveEvent);

impl DBusMatchRule for WindowEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window'";
}

impl DBusInterface for WindowEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
}

impl RegistryEventString for WindowEvents {
	const REGISTRY_EVENT_STRING: &'static str = "window:";
}

#[cfg(feature = "zbus")]
impl EventWrapperMessageConversion for WindowEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &Header,
	) -> Result<Self, AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			WindowPropertyChangeEvent::DBUS_MEMBER => Ok(WindowEvents::PropertyChange(
				WindowPropertyChangeEvent::from_message_unchecked(msg, hdr)?,
			)),
			MinimizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Minimize(MinimizeEvent::from_message_unchecked(msg, hdr)?))
			}
			MaximizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Maximize(MaximizeEvent::from_message_unchecked(msg, hdr)?))
			}
			RestoreEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Restore(RestoreEvent::from_message_unchecked(msg, hdr)?))
			}
			CloseEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Close(CloseEvent::from_message_unchecked(msg, hdr)?))
			}
			CreateEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Create(CreateEvent::from_message_unchecked(msg, hdr)?))
			}
			ReparentEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Reparent(ReparentEvent::from_message_unchecked(msg, hdr)?))
			}
			DesktopCreateEvent::DBUS_MEMBER => Ok(WindowEvents::DesktopCreate(
				DesktopCreateEvent::from_message_unchecked(msg, hdr)?,
			)),
			DesktopDestroyEvent::DBUS_MEMBER => Ok(WindowEvents::DesktopDestroy(
				DesktopDestroyEvent::from_message_unchecked(msg, hdr)?,
			)),
			DestroyEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Destroy(DestroyEvent::from_message_unchecked(msg, hdr)?))
			}
			ActivateEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Activate(ActivateEvent::from_message_unchecked(msg, hdr)?))
			}
			DeactivateEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Deactivate(DeactivateEvent::from_message_unchecked(msg, hdr)?))
			}
			RaiseEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Raise(RaiseEvent::from_message_unchecked(msg, hdr)?))
			}
			LowerEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Lower(LowerEvent::from_message_unchecked(msg, hdr)?))
			}
			MoveEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Move(MoveEvent::from_message_unchecked(msg, hdr)?))
			}
			ResizeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Resize(ResizeEvent::from_message_unchecked(msg, hdr)?))
			}
			ShadeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Shade(ShadeEvent::from_message_unchecked(msg, hdr)?))
			}
			UUshadeEvent::DBUS_MEMBER => {
				Ok(WindowEvents::UUshade(UUshadeEvent::from_message_unchecked(msg, hdr)?))
			}
			RestyleEvent::DBUS_MEMBER => {
				Ok(WindowEvents::Restyle(RestyleEvent::from_message_unchecked(msg, hdr)?))
			}
			_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for WindowEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		Self::try_from_message(msg)
	}
}

impl_from_user_facing_event_for_interface_event_enum!(
	WindowPropertyChangeEvent,
	WindowEvents,
	WindowEvents::PropertyChange
);
impl_try_from_event_for_user_facing_type!(
	WindowPropertyChangeEvent,
	WindowEvents::PropertyChange,
	Event::Window
);
impl_from_user_facing_event_for_interface_event_enum!(
	MinimizeEvent,
	WindowEvents,
	WindowEvents::Minimize
);
impl_try_from_event_for_user_facing_type!(MinimizeEvent, WindowEvents::Minimize, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	MaximizeEvent,
	WindowEvents,
	WindowEvents::Maximize
);
impl_try_from_event_for_user_facing_type!(MaximizeEvent, WindowEvents::Maximize, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	RestoreEvent,
	WindowEvents,
	WindowEvents::Restore
);
impl_try_from_event_for_user_facing_type!(RestoreEvent, WindowEvents::Restore, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	CloseEvent,
	WindowEvents,
	WindowEvents::Close
);
impl_try_from_event_for_user_facing_type!(CloseEvent, WindowEvents::Close, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	CreateEvent,
	WindowEvents,
	WindowEvents::Create
);
impl_try_from_event_for_user_facing_type!(CreateEvent, WindowEvents::Create, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ReparentEvent,
	WindowEvents,
	WindowEvents::Reparent
);
impl_try_from_event_for_user_facing_type!(ReparentEvent, WindowEvents::Reparent, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	DesktopCreateEvent,
	WindowEvents,
	WindowEvents::DesktopCreate
);
impl_try_from_event_for_user_facing_type!(
	DesktopCreateEvent,
	WindowEvents::DesktopCreate,
	Event::Window
);
impl_from_user_facing_event_for_interface_event_enum!(
	DesktopDestroyEvent,
	WindowEvents,
	WindowEvents::DesktopDestroy
);
impl_try_from_event_for_user_facing_type!(
	DesktopDestroyEvent,
	WindowEvents::DesktopDestroy,
	Event::Window
);
impl_from_user_facing_event_for_interface_event_enum!(
	DestroyEvent,
	WindowEvents,
	WindowEvents::Destroy
);
impl_try_from_event_for_user_facing_type!(DestroyEvent, WindowEvents::Destroy, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ActivateEvent,
	WindowEvents,
	WindowEvents::Activate
);
impl_try_from_event_for_user_facing_type!(ActivateEvent, WindowEvents::Activate, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	DeactivateEvent,
	WindowEvents,
	WindowEvents::Deactivate
);
impl_try_from_event_for_user_facing_type!(DeactivateEvent, WindowEvents::Deactivate, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	RaiseEvent,
	WindowEvents,
	WindowEvents::Raise
);
impl_try_from_event_for_user_facing_type!(RaiseEvent, WindowEvents::Raise, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	LowerEvent,
	WindowEvents,
	WindowEvents::Lower
);
impl_try_from_event_for_user_facing_type!(LowerEvent, WindowEvents::Lower, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(MoveEvent, WindowEvents, WindowEvents::Move);
impl_try_from_event_for_user_facing_type!(MoveEvent, WindowEvents::Move, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ResizeEvent,
	WindowEvents,
	WindowEvents::Resize
);
impl_try_from_event_for_user_facing_type!(ResizeEvent, WindowEvents::Resize, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	ShadeEvent,
	WindowEvents,
	WindowEvents::Shade
);
impl_try_from_event_for_user_facing_type!(ShadeEvent, WindowEvents::Shade, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	UUshadeEvent,
	WindowEvents,
	WindowEvents::UUshade
);
impl_try_from_event_for_user_facing_type!(UUshadeEvent, WindowEvents::UUshade, Event::Window);
impl_from_user_facing_event_for_interface_event_enum!(
	RestyleEvent,
	WindowEvents,
	WindowEvents::Restyle
);
impl_try_from_event_for_user_facing_type!(RestyleEvent, WindowEvents::Restyle, Event::Window);

/// The events that can be emitted by the registry daemon.
/// This enum is used to wrap the events that are emitted by the registry daemon.
/// The events are [`EventListenerRegisteredEvent`] and [`EventListenerDeregisteredEvent`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum EventListenerEvents {
	/// See: [`EventListenerRegisteredEvent`].
	Registered(EventListenerRegisteredEvent),
	/// See: [`EventListenerDeregisteredEvent`].
	Deregistered(EventListenerDeregisteredEvent),
}

impl_tryfrommessage_for_event_wrapper!(EventListenerEvents);

impl DBusInterface for EventListenerEvents {
	const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Registry";
}

impl DBusMatchRule for EventListenerEvents {
	const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Registry'";
}

impl RegistryEventString for EventListenerEvents {
	const REGISTRY_EVENT_STRING: &'static str = "Event";
}

impl EventTypeProperties for EventListenerEvents {
	fn member(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.member(),
			Self::Deregistered(inner) => inner.member(),
		}
	}

	fn match_rule(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.match_rule(),
			Self::Deregistered(inner) => inner.match_rule(),
		}
	}

	fn interface(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.interface(),
			Self::Deregistered(inner) => inner.interface(),
		}
	}

	fn registry_string(&self) -> &'static str {
		match self {
			Self::Registered(inner) => inner.registry_string(),
			Self::Deregistered(inner) => inner.registry_string(),
		}
	}
}

impl EventProperties for EventListenerEvents {
	fn path(&self) -> ObjectPath<'_> {
		match self {
			Self::Registered(inner) => inner.path(),
			Self::Deregistered(inner) => inner.path(),
		}
	}
	fn sender(&self) -> UniqueName<'_> {
		match self {
			Self::Registered(inner) => inner.sender(),
			Self::Deregistered(inner) => inner.sender(),
		}
	}
}

#[cfg(feature = "zbus")]
impl crate::events::traits::EventWrapperMessageConversion for EventListenerEvents {
	fn try_from_message_interface_checked(
		msg: &zbus::Message,
		hdr: &zbus::message::Header,
	) -> Result<Self, crate::AtspiError> {
		let member = hdr.member().ok_or(AtspiError::MissingMember)?;
		match member.as_str() {
			EventListenerRegisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Registered(
				EventListenerRegisteredEvent::from_message_unchecked(msg, hdr)?,
			)),
			EventListenerDeregisteredEvent::DBUS_MEMBER => Ok(EventListenerEvents::Deregistered(
				EventListenerDeregisteredEvent::from_message_unchecked(msg, hdr)?,
			)),
			_ => Err(AtspiError::MemberMatch(format!(
				"No member {} in {}",
				member.as_str(),
				Self::DBUS_INTERFACE
			))),
		}
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<&zbus::Message> for EventListenerEvents {
	type Error = AtspiError;
	fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
		<Self as crate::events::traits::TryFromMessage>::try_from_message(msg)
	}
}

impl_try_from_event_for_interface_enum!(EventListenerEvents, Event::Listener);
impl_from_interface_event_enum_for_event!(EventListenerEvents, Event::Listener);

impl From<AvailableEvent> for Event {
	fn from(ev: AvailableEvent) -> Event {
		Event::Available(ev)
	}
}

#[cfg(feature = "zbus")]
impl TryFrom<Event> for AvailableEvent {
	type Error = AtspiError;
	fn try_from(generic_event: Event) -> Result<AvailableEvent, Self::Error> {
		if let Event::Available(specific_event) = generic_event {
			Ok(specific_event)
		} else {
			Err(AtspiError::Conversion("Invalid type"))
		}
	}
}

event_wrapper_test_cases!(EventListenerEvents, EventListenerRegisteredEvent);
