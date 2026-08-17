/// Expands to implement the required methods for the [`crate::EventProperties`] trait.
/// This depends on the struct to have an `item` field of type [`crate::ObjectRef`].
///
/// ```ignore
/// impl_from_interface_event_enum_for_event!(TextCaretMovedEvent);
/// ```
///
/// Expands to:
///
/// ```ignore
/// impl EventProperties for TextCaretMovedEvent {
///     fn sender(&self) -> UniqueName<'_> {
///         self.item
///             .name()
///             .expect("Events are constructed with valid ObjetRef")
///             .as_ref()
///     }
///     fn path(&self) -> ObjectPath<'_> {
///         self.item.path().as_ref()
///     }
/// }
/// ```
macro_rules! impl_event_properties {
	($type:ty) => {
		impl crate::EventProperties for $type {
			fn sender(&self) -> zbus_names::UniqueName<'_> {
				self.item
					.name()
					.expect("Events are received with valid ObjetRef")
					.as_ref()
			}

			fn path(&self) -> zvariant::ObjectPath<'_> {
				self.item.path().as_ref()
			}
		}
	};
}

/// Expands to implement From for [`crate::ObjectRef`].
/// This depends on the struct to have an `item` field of type [`crate::ObjectRef`].
///
/// ```ignore
/// impl_from_object_ref!(TextAttributesChangedEvent);
/// ```
///
/// Exapnds to:
///
/// ```ignore
/// impl From<ObjectRef> for TextAttributesChangedEvent {
///     fn from(obj_ref: ObjectRef) -> Self {
///         Self { item: obj_ref.into() }
///     }
/// }
/// ```
macro_rules! impl_from_object_ref {
	($type:ty) => {
		impl From<crate::ObjectRef<'_>> for $type {
			fn from(obj_ref: crate::ObjectRef) -> Self {
				Self { item: obj_ref.into() }
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Expands to a conversion given the enclosed event type and outer `Event` variant.
///
/// eg
/// ```ignore
/// impl_from_interface_event_enum_for_event!(ObjectEvents, Event::Object);
/// ```
/// expands to:
///
/// ```ignore
/// impl From<ObjectEvents> for Event {
///     fn from(event_variant: ObjectEvents) -> Event {
///         Event::Object(event_variant.into())
///     }
/// }
/// ```
macro_rules! impl_from_interface_event_enum_for_event {
	($outer_type:ty, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl From<$outer_type> for Event {
			fn from(event_variant: $outer_type) -> Event {
				$outer_variant(event_variant.into())
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Expands to a conversion given the enclosed event enum type and outer `Event` variant.
///
/// eg
/// ```ignore
/// impl_try_from_event_for_interface_enum!(ObjectEvents, Event::Object);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<Event> for ObjectEvents {
///     type Error = AtspiError;
///     fn try_from(generic_event: Event) -> Result<ObjectEvents, Self::Error> {
///         if let Event::Object(event_type) = generic_event {
///             Ok(event_type)
///         } else {
///             Err(AtspiError::Conversion("Invalid type"))
///         }
///     }
/// }
/// ```
macro_rules! impl_try_from_event_for_interface_enum {
	($outer_type:ty, $outer_variant:path) => {
		impl TryFrom<Event> for $outer_type {
			type Error = AtspiError;
			fn try_from(generic_event: Event) -> Result<$outer_type, Self::Error> {
				if let $outer_variant(event_type) = generic_event {
					Ok(event_type)
				} else {
					Err(AtspiError::Conversion("Invalid type"))
				}
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Expands to a conversion given the user facing event type,
/// the wrapping interface enum variant, and the outer `Event` variant.
///
/// ```ignore
/// impl_from_user_facing_event_for_interface_event_enum!(StateChangedEvent, ObjectEvents, ObjectEvents::StateChanged);
/// ```
///
/// expands to:
///
/// ```ignore
/// impl From<StateChangedEvent> for ObjectEvents {
///     fn from(specific_event: StateChangedEvent) -> ObjectEvents {
///         ObjectEvents::StateChanged(specific_event)
///     }
/// }
/// ```
macro_rules! impl_from_user_facing_event_for_interface_event_enum {
	($inner_type:ty, $outer_type:ty, $inner_variant:path) => {
		impl From<$inner_type> for $outer_type {
			fn from(specific_event: $inner_type) -> $outer_type {
				$inner_variant(specific_event)
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Expands to a conversion given two arguments,
/// 1. the user facing event type `(inner_type)`
///    which relies on a conversion to its interface variant enum type variant.
/// 2. the outer `Event::<Interface(<InterfaceEnum>)>` wrapper.,
///    the enum type and outtermost variant.
///
/// ```ignore                                   user facing type, outer event variant
/// impl_from_user_facing_type_for_event_enum!(StateChangedEvent, Event::Object);
/// ```
///
/// expands to:
///
/// ```ignore
/// impl From<StateChangedEvent> for Event {
///    fn from(event_variant: StateChangedEvent) -> Event {
///       Event::Object(ObjectEvents::StateChanged(event_variant))
///   }
/// }
/// ```
macro_rules! impl_from_user_facing_type_for_event_enum {
	($inner_type:ty, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl From<$inner_type> for Event {
			fn from(event_variant: $inner_type) -> Event {
				$outer_variant(event_variant.into())
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Expands to a `TryFrom<Event> for T` where T is the user facing type.
/// The macro takes three arguments:
///
/// 1. The user facing type.
/// 2. The inner variant of the user facing type.
/// 3. The outer variant of the `Event` enum.
///
/// ```ignore
/// impl_try_from_event_for_user_facing_type!(
///     StateChangedEvent,
///     ObjectEvents::StateChanged,
///     Event::Object
/// );
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<Event> for StateChangedEvent {
///     type Error = AtspiError;
///     fn try_from(generic_event: Event) -> Result<StateChangedEvent, Self::Error> {
///         if let Event::Object(ObjectEvents::StateChanged(specific_event)) = generic_event {
///             Ok(specific_event)
///         } else {
///             Err(AtspiError::Conversion("Invalid type"))
///         }
///     }
/// }
/// ```
///
macro_rules! impl_try_from_event_for_user_facing_type {
	($inner_type:ty, $inner_variant:path, $outer_variant:path) => {
		#[cfg(feature = "wrappers")]
		impl TryFrom<crate::Event> for $inner_type {
			type Error = AtspiError;
			fn try_from(generic_event: crate::Event) -> Result<$inner_type, Self::Error> {
				if let $outer_variant($inner_variant(specific_event)) = generic_event {
					Ok(specific_event)
				} else {
					Err(AtspiError::Conversion("Invalid type"))
				}
			}
		}
	};
}

/// Implements the `TryFrom` trait for a given event type.
/// Converts a user facing event type into a `zbus::Message`.
///
/// # Example
/// ```ignore
/// impl_to_dbus_message!(StateChangedEvent);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<StateChangedEvent> for zbus::Message {
///   type Error = AtspiError;
///   fn try_from(event: StateChangedEvent) -> Result<Self, Self::Error> {
///     Ok(zbus::Message::signal(
///         event.path(),
///         StateChangedEvent::DBUS_INTERFACE,
///         StateChangedEvent::DBUS_MEMBER,
///     )?
///     .sender(event.sender())?
///     .build(&event.body())?)
///  }
/// }
///
macro_rules! impl_to_dbus_message {
	($type:ty) => {
		#[cfg(feature = "zbus")]
		impl TryFrom<$type> for zbus::Message {
			type Error = AtspiError;
			fn try_from(event: $type) -> Result<Self, Self::Error> {
				use crate::events::{DBusInterface, DBusMember, MessageConversion};
				Ok(zbus::Message::signal(
					event.path(),
					<$type as DBusInterface>::DBUS_INTERFACE,
					<$type as DBusMember>::DBUS_MEMBER,
				)?
				.sender(event.sender().to_string())?
				.build(&event.body())?)
			}
		}
	};
}

/// Implements the `TryFrom` trait for a given event type.
/// Converts a `zbus::Message` into a user facing event type.
///
/// See [`crate::events::MessageConversion`] for details on implementation.
///
/// # Example
/// ```ignore
/// impl_from_dbus_message!(StateChangedEvent);
/// ```
/// expands to:
///
/// ```ignore
/// impl TryFrom<&zbus::Message> for StateChangedEvents {
///   type Error = AtspiError;
///   fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
///    let hdr = msg.header();
///     <$type as MessageConversion>::try_from_message(msg, hdr)
///   }
/// }
/// ```
///
/// There is also a variant that can be used for events whose [`crate::events::MessageConversion::Body`] is not
/// [`crate::events::event_body::EventBodyOwned`]. You can call this by setting the second parameter to `Explicit`.
macro_rules! impl_from_dbus_message {
	($type:ty) => {
		impl_from_dbus_message!($type, Auto);
	};

	($type:ty, Auto) => {
		#[cfg(feature = "zbus")]
		impl<'msg> TryFrom<&'msg zbus::Message> for $type {
			type Error = AtspiError;
			fn try_from(msg: &'msg zbus::Message) -> Result<Self, Self::Error> {
				use crate::events::{EventBody, EventBodyQtBorrowed};
				use crate::events::traits::{MessageConversion, MessageConversionExt};
				use zvariant::Type;
				use crate::ObjectRef;

				let hdr = msg.header();
				<Self as MessageConversionExt<<Self as MessageConversion>::Body<'_>>>::validate_interface(&hdr)?;
				<Self as MessageConversionExt<<Self as MessageConversion>::Body<'_>>>::validate_member(&hdr)?;
				let item = ObjectRef::try_from(&hdr)?;

				let body = msg.body();
				let signature = body.signature();

				if signature == EventBody::SIGNATURE || signature == EventBodyQtBorrowed::SIGNATURE {
					Ok(Self::from_message_unchecked_parts(item, body)?)
				} else {
					Err(AtspiError::SignatureMatch(format!(
						"signature mismatch: expected: {}, signal body: {}",
						msg.body().signature(),
						<Self as MessageConversion>::Body::SIGNATURE,
					)))
				}
			}
		}
	};

	($type:ty, Explicit) => {
		#[cfg(feature = "zbus")]
		impl TryFrom<&zbus::Message> for $type {
			type Error = crate::AtspiError;
			fn try_from(msg: &zbus::Message) -> Result<Self, Self::Error> {
				let hdr = msg.header();
				<$type as crate::events::MessageConversionExt<<$type as MessageConversion>::Body<'_>>>::try_from_message(msg, &hdr)
			}
		}
	};
}

// We decorate the macro with a `#[cfg(test)]` attribute.
// This prevents Clippy from complaining about the macro not being used.
// It is being used, but only in test mode.
//
/// Tests `Default` and `BusProperties::from_message_unchecked` for a given event struct.
///
/// Obtains a default for the given event struct.
/// Asserts that the path and sender are the default.
///
/// Breaks the struct down into item (the associated object) and body.
/// Then tests `BusProperties::from_message_unchecked` with the item and body.
#[cfg(test)]
macro_rules! generic_event_test_case {
	($type:ty) => {
		#[test]
		fn generic_event_uses() {
			use crate::events::traits::MessageConversion;

			let event_struct = <$type>::default();
			assert_eq!(event_struct.path().as_str(), crate::object_ref::TEST_OBJECT_PATH_STR);
			assert_eq!(event_struct.sender().as_str(), crate::object_ref::TEST_OBJECT_BUS_NAME);
			let body = event_struct.body();
			let body2 = Message::method_call(
				event_struct.path().as_str(),
				<$type as crate::events::DBusMember>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(event_struct.sender().as_str())
			.unwrap()
			.build(&(body,))
			.unwrap();
			let header = body2.header();
			let build_struct = <$type>::from_message_unchecked(&body2, &header)
				.expect("<$type as Default>'s parts should build a valid ObjectRef");
			assert_eq!(event_struct, build_struct);
		}
	};
}

// We decorate the macro with a `#[cfg(test)]` attribute.
// This prevents Clippy from complaining about the macro not being used.
// It is being used, but only in test mode.
//
/// Tests conversion to and from the `Event` enum.
///
/// Obtains a default for the given event struct.
/// Converts the struct into the `Event` enum, wrapping the struct.
/// Converts the `Event` enum into the given event struct.
/// Asserts that the original struct and the converted struct are equal.
#[cfg(test)]
macro_rules! event_has_matching_xml_definition {
	($type:ty) => {
		#[test]
		fn event_has_matching_xml_definition() {

	    use zbus_xml;
		use crate::events::{DBusInterface, DBusMember};

		let fname = match <$type>::DBUS_INTERFACE.split(".").last().expect("Has last section") {
			"Cache" => "xml/Cache.xml",
			"Socket" => "xml/Socket.xml",
			"Registry" => "xml/Registry.xml",
			_ => "xml/Event.xml",
		};

		let reader = std::fs::File::open(fname).expect("Valid file path!");
		let xml = zbus_xml::Node::from_reader(reader).expect("Valid DBus XML file!");
		let Some(interface) = xml.interfaces().iter().find(|int| int.name() == <$type>::DBUS_INTERFACE) else {

	    let possible_names: Vec<String> = xml.interfaces().iter().map(|int| int.name().as_str().to_string()).collect();
        panic!("{} has interface name {}, but it was not found in the list of interfaces defined in the XML: {:?}", std::any::type_name::<$type>(), <$type>::DBUS_INTERFACE, possible_names);
      };
        let Some(_member) = interface.signals().iter().find(|mem| mem.name() == <$type>::DBUS_MEMBER) else {
        let possible_names: Vec<String> = interface.signals().iter().map(|mem| mem.name().as_str().to_string()).collect();
        panic!("{} has interface name {} and member name {}, but it was not found in the list of members defined in the corresponding interface in the XML: {:?}", std::any::type_name::<$type>(), <$type>::DBUS_INTERFACE, <$type>::DBUS_MEMBER, possible_names);
      };
		}
	};
}

#[cfg(test)]
macro_rules! zbus_message_qtspi_test_case {
    ($type:ty, Auto) => {
      #[cfg(feature = "zbus")]
     #[test]
    fn zbus_message_conversion_qtspi() {
	  use crate::events::EventTypeProperties;
	  use crate::events::MessageConversion;

	  // in the case that the body type is EventBodyOwned, we need to also check successful
      // conversion from a QSPI-style body.
      let ev = <$type>::default();
      let qt: crate::events::EventBodyQtOwned = ev.body().into();
      let msg = zbus::Message::signal(
        ev.path(),
        ev.interface(),
        ev.member(),
      )
        .unwrap()
        .sender(":0.0")
        .unwrap()
        .build(&(qt,))
        .unwrap();
          <$type>::try_from(&msg).expect("Should be able to use an EventBodyQtOwned for any type whose BusProperties::Body = EventBodyOwned");
    }
    #[cfg(feature = "zbus")]
    #[test]
    fn zbus_message_conversion_qtspi_event_enum() {
	  use crate::events::EventTypeProperties;
	  use crate::events::MessageConversion;
	  use crate::Event;

      // in the case that the body type is EventBodyOwned, we need to also check successful
      // conversion from a QSPI-style body.
      let ev = <$type>::default();
      let qt: crate::events::EventBodyQtOwned = ev.body().into();
      let msg = zbus::Message::signal(
        ev.path(),
        ev.interface(),
        ev.member(),
      )
        .unwrap()
        .sender(":0.0")
        .unwrap()
        .build(&(qt,))
        .unwrap();
        assert_matches!(Event::try_from(&msg), Ok(_));
     }
    };
    ($type:ty, Explicit) => {};
}

// We decorate the macro with a `#[cfg(test)]` attribute.
// This prevents Clippy from complaining about the macro not being used.
// It is being used, but only in test mode.
//
/// As of writing, this macro is expanded only once: in the `event_test_cases!` macro.
#[cfg(test)]
macro_rules! zbus_message_test_case {
  ($type:ty) => {
      zbus_message_test_case!($type, Auto);
    };
	($type:ty, $extra:tt) => {
    zbus_message_qtspi_test_case!($type, $extra);
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_specific_event_type() {
			let struct_event = <$type>::default();
			let msg: zbus::Message = zbus::Message::try_from(<$type>::default()).expect("Should convert a `$type::default()` into a message. Check the `impl_to_dbus_message` macro .");

			let struct_event_back =
				<$type>::try_from(&msg).expect("Should convert from `$type::default()` originated `Message` back into a specific event type. Check the `impl_from_dbus_message` macro.");
        	assert_eq!(struct_event, struct_event_back, "Events converted into a message and back must be the same");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_to_event_enum_type() {
			let struct_event = <$type>::default();
			let msg: zbus::Message = zbus::Message::try_from(struct_event.clone()).expect("Should convert a `$type::default()` into a message. Check the `impl_to_dbus_message` macro.");
			let event_enum_back =
				crate::Event::try_from(&msg).expect("Should convert a from `$type::default()` built `Message` into an event enum. Check the `impl_from_dbus_message` macro.");
			let event_enum: crate::Event = struct_event.into();
			assert_eq!(event_enum, event_enum_back);
		}
		// may want to consider parameterized tests here, no need for fuzz testing, but one level lower than that may be nice
		// try having a matching member, matching interface, path, or body type, but that has some other piece which is not right
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_fake_msg() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.technically.valid",
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
      		assert_matches!(event, Err(_), "This conversion should fail");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_validated_message_with_body() -> () {
			use crate::events::MessageConversion;

			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.technically.valid",
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let hdr = fake_msg.header();
			let event = <$type>::from_message_unchecked(&fake_msg, &hdr);
      event.expect("The from_message_unchecked function should work, despite mismatching interface and member");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_interface() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as crate::events::DBusInterface>::DBUS_INTERFACE,
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
      assert_matches!(event, Err(AtspiError::MemberMatch(_)), "Wrong kind of error");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_interface_and_member() -> () {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as crate::events::DBusInterface>::DBUS_INTERFACE,
				<$type as crate::events::DBusMember>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
      assert_matches!(event, Err(AtspiError::SignatureMatch(_)), "Wrong kind of error");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_interface_and_member_invalid_body() -> () {
      // known invalid body for AT-SPI events
      let invalid_body: (i32, u64, String, String) = (0, 0, String::new(), String::new());
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as crate::events::DBusInterface>::DBUS_INTERFACE,
				<$type as crate::events::DBusMember>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&invalid_body)
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
      assert_matches!(event, Err(AtspiError::SignatureMatch(_)), "Wrong kind of error");
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_body() -> () {
			use crate::events::MessageConversion;
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.accessible.technically.valid",
				"FakeMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
      assert_matches!(event, Err(_));
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_body_and_member() -> () {
			use crate::events::MessageConversion;

			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.accessible.technically.valid",
				<$type as crate::events::DBusMember>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
      assert_matches!(event, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
		}
		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_conversion_failure_correct_body_and_interface() -> () {
			use crate::events::MessageConversion;

			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				<$type as crate::events::DBusInterface>::DBUS_INTERFACE,
				"MadeUpMember",
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$type>::default().body())
			.unwrap();
			let event = <$type>::try_from(&fake_msg);
      assert_matches!(event, Err(AtspiError::MemberMatch(_)), "Wrong kind of error");
		}
	};
}

#[cfg(feature = "wrappers")]
/// Expands to five tests:
///
/// 1. `into_and_try_from_event`
/// 2. `zbus_msg_invalid_interface`
/// 3. `zbus_msg_invalid_member`
/// 4. `zbus_msg_invalid_member_and_interface`
/// 5. `zbus_msg_conversion`
///
/// The macro takes two arguments:
/// 1. The event's interface enum type.
/// 2. Any user facing event type that is wrapped by the interface enum.
///
/// # Examples
///
/// ```ignore
/// event_wrapper_test_cases!(MouseEvents, AbsEvent);
/// ```
///
/// For each of the types, the macro will create a module with the name `events_tests_{foo}`
/// where `{foo}` is the snake case of the 'interface enum' name.
macro_rules! event_wrapper_test_cases {
	// The macro takes two arguments: the interface enum type and the user facing event type (ufet).
	($iface_enum:ty, $ufet:ty) => {
		#[cfg(test)]
		#[rename_item::rename(name($iface_enum), prefix = "events_tests_", case = "snake")]
		mod foo {
		use super::{$ufet, $iface_enum, AtspiError, Event, MessageConversion};

		// TODO: replace with [`std::assert_matches::assert_matches`] when stabilized
		use assert_matches::assert_matches;

		#[test]
		fn into_and_try_from_user_facing_event() {
			// Create a default event struct from its type's `Default::default()` impl.
			let sub_type = <$ufet>::default();

			// Wrap the event struct in the event enum
			let mod_type = <$iface_enum>::from(sub_type);
			let hint_iface = "Check macro `impl_from_user_facing_event_for_interface_event_enum!`";

			// Wrap the inner event enum into the `Event` enum.
			let event = Event::from(mod_type.clone());
			let hint_event = "Check macro `impl_from_interface_event_enum_for_event!`";

			// Unwrap the `Event` enum into the inner event enum.
			let hint_event_try = "Check macro `impl_try_from_event_for_interface_enum!`";
			let mod_type2 = <$iface_enum>::try_from(event.clone())
				.expect("Should convert outer `Event` enum into interface enum. hints: {hint_event} and {hint_event_try}");

			assert_eq!(
				mod_type, mod_type2,
				"Interface enums should match. hint: {hint_iface}, {hint_event} and {hint_event_try}"
			);
		}

		#[cfg(feature = "zbus")]
		#[test]
		fn zbus_msg_invalid_interface() {
			let fake_msg = zbus::Message::signal(
				"/org/a11y/sixtynine/fourtwenty",
				"org.a11y.atspi.technically.valid.lol",
				<$ufet as crate::events::DBusMember>::DBUS_MEMBER,
			)
			.unwrap()
			.sender(":0.0")
			.unwrap()
			.build(&<$ufet>::default().body())
			.unwrap();

			// It is hard to see what eventually is tested here. Let's unravel it:
			//
			// Below we call `TryFrom<&zbus::Message> for $type` where `$type` the interface enum name. (eg. `MouseEvents`, `ObjectEvents`, etc.) and
			// `mod_type` is an 'interface enum' variant (eg. `MouseEvents::Abs(AbsEvent)`).
			// This conversion is found in the `/src/events/{iface_name}.rs`` file.
			// This conversion in turn leans on the `impl_from_dbus_message` macro.
			// In `MouseEvents::Abs(msg.try_into()?)`, it is the `msg.try_into()?` that should fail.
			// The `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.

			// Additioanlly, we check against the same method in `Event`; the overarchive enum that
			// contains all other events as variants.

			let mod_type = <$iface_enum>::try_from(&fake_msg);
			let event_type = Event::try_from(&fake_msg);

			assert_matches!(mod_type, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
			assert_matches!(event_type, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
			}

			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_member() {
				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$ufet as crate::events::DBusInterface>::DBUS_INTERFACE,
					"FakeFunctionLol",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$ufet>::default().body())
				.unwrap();
				// As above, the `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.
				let mod_type = <$iface_enum>::try_from(&fake_msg);
        		assert_matches!(mod_type, Err(AtspiError::MemberMatch(_)), "Wrong kind of error");
			}
			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_invalid_member_and_interface() {
				let fake_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					"org.a11y.atspi.technically.allowed",
					"FakeFunctionLol",
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$ufet>::default().body())
				.unwrap();
				// As above, the `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.
				let mod_type = <$iface_enum>::try_from(&fake_msg);

				// Note that the non-matching interface is the first error, so the member match error is not reached.
        		assert_matches!(mod_type, Err(AtspiError::InterfaceMatch(_)), "Wrong kind of error");
			}

			#[cfg(feature = "zbus")]
			#[test]
			fn zbus_msg_conversion() {
				let valid_msg = zbus::Message::signal(
					"/org/a11y/sixtynine/fourtwenty",
					<$ufet as crate::events::DBusInterface>::DBUS_INTERFACE,
					<$ufet as crate::events::DBusMember>::DBUS_MEMBER,
				)
				.unwrap()
				.sender(":0.0")
				.unwrap()
				.build(&<$ufet>::default().body())
				.unwrap();
				// As above, the `msg.try_into()?` is provided through the `impl_from_dbus_message` macro.
				let mod_type = <$iface_enum>::try_from(&valid_msg);
				mod_type.expect("Should convert from `$ufet::default()` built `Message` back into a interface event enum variant wrapping an inner type. Check the `impl_from_dbus_message` macro.");
			}


		}
	};
}

macro_rules! event_test_cases {
  ($ufet:ty) => {
      event_test_cases!($ufet, Auto);
  };
	($ufet:ty, $qt:tt) => {
		#[cfg(test)]
		#[rename_item::rename(name($ufet), prefix = "event_tests_", case = "snake")]
		mod foo {
		use super::{$ufet, AtspiError, EventProperties };
		use crate::events::traits::EventTypeProperties;
		#[cfg(feature = "zbus")]
        use zbus::Message;
		use crate::Event;

        // TODO: use [`std::assert_matches::assert_matches`] when stabilized
        use assert_matches::assert_matches;

			#[test]
			#[cfg(feature = "wrappers")]
			fn event_enum_conversion() {
				let event_struct = <$ufet>::default();
				let event_enum = Event::from(event_struct.clone());
				let event_struct_back = <$ufet>::try_from(event_enum)
					.expect("Should convert event enum into specific event type because it was created from it. Check the `impl_from_interface_event_enum_for_event` macro");
				assert_eq!(event_struct, event_struct_back);
			}

			#[test]
			#[cfg(feature = "wrappers")]
			fn event_enum_transparency_test_case() {
			let specific_event = <$ufet>::default();

			let generic_event = Event::from(specific_event.clone());
			let hint = "Check macro `impl_from_user_facing_type_for_event_enum!`.";

			assert_eq!(
				specific_event.member(),
				generic_event.member(),
				"DBus members do not match. hint: {hint}"
			);
			assert_eq!(
				specific_event.interface(),
				generic_event.interface(),
				"DBus interfaces do not match. hint: {hint}"
			);
			assert_eq!(
				specific_event.registry_string(),
				generic_event.registry_string(),
				"Registry strings do not match. hint: {hint}"
			);
			assert_eq!(
				specific_event.match_rule(),
				generic_event.match_rule(),
				"Match rule strings do not match. hint: {hint}"
			);
			assert_eq!(specific_event.path(), generic_event.path(), "Paths do not match. hint: {hint}");
			assert_eq!(specific_event.sender(), generic_event.sender(), "Senders do not match. hint: {hint}");
			}

			zbus_message_test_case!($ufet, $qt);
      event_has_matching_xml_definition!($ufet);
			generic_event_test_case!($ufet);
		}
		assert_impl_all!(
			$ufet: Clone,
			std::fmt::Debug,
			serde::Serialize,
			serde::Deserialize<'static>,
			Default,
			PartialEq,
			Eq,
			std::hash::Hash,
			crate::events::traits::EventProperties,
			crate::events::traits::EventTypeProperties,
			crate::events::traits::DBusInterface,
			crate::events::traits::DBusMember,
			crate::events::traits::DBusMatchRule,
			crate::events::traits::RegistryEventString
		);

		#[cfg(feature = "zbus")]
		assert_impl_all!(zbus::Message: TryFrom<$ufet>);
	};
}

/// Implements `MessageConversionExt` for a given target event type with a given body type.
///
/// # Example
/// ```ignore
/// # // 'ignore'd for bevity's sake because `impl`s require that the *example crate* defines traits `MessageConversionExt`,
/// # // `MessageConversion` as well as the body and target types.
///
/// impl_msg_conversion_ext_for_target_type_with_specified_body_type!(target: RemoveAccessibleEvent, body: ObjectRef);
/// ```
/// expands to:
///
/// ```ignore
/// #[cfg(feature = "zbus")]
/// impl<'a> MessageConversionExt<'_, ObjectRef> for RemoveAccessibleEvent {
///     fn try_from_message(msg: &zbus::Message, hdr: &Header) -> Result<Self, AtspiError> {
///         <Self as MessageConversionExt<$body_type>>::validate_interface(hdr)?;
///         <Self as MessageConversionExt<$body_type>>::validate_member(hdr)?;
///         <Self as MessageConversionExt<$body_type>>::validate_body(msg)?;
///         <Self as MessageConversion<'a>>::from_message_unchecked(msg, hdr)
///     }
/// }
/// ```
macro_rules! impl_msg_conversion_ext_for_target_type_with_specified_body_type {
	(target: $target_type:ty, body: $body_type:ty) => {
		#[cfg(feature = "zbus")]
		impl<'a> crate::events::MessageConversionExt<'a, $body_type> for $target_type {
			fn try_from_message(msg: &zbus::Message, hdr: &Header) -> Result<Self, AtspiError> {
				use crate::events::MessageConversionExt;
				<Self as MessageConversionExt<$body_type>>::validate_interface(hdr)?;
				<Self as MessageConversionExt<$body_type>>::validate_member(hdr)?;
				<Self as MessageConversionExt<$body_type>>::validate_body(msg)?;
				<Self as MessageConversion<'a>>::from_message_unchecked(msg, hdr)
			}
		}
	};
}

/// Implements `MessageConversionExt` for a given target event type.
///
/// # Example
///
/// ```ignore
/// impl_msg_conversion_ext_for_target_type!(LoadCompleteEvent);
/// ```
/// expands to:
///
/// ```ignore
/// #[cfg(feature = "zbus")]
/// impl<'msg> MessageConversionExt<'msg, EventBody<'msg>> for LoadCompleteEvent {
///     fn try_from_message(msg: &'msg zbus::Message, header: &Header) -> Result<Self, AtspiError> {
///         Self::validate_interface(header)?;
///         Self::validate_member(header)?;
///
///         let item = ObjectRefOwned::try_from(header)?;
///         let msg_body = msg.body();
///         let signature = msg_body.signature();
///
///         if signature == crate::events::EventBodyOwned::SIGNATURE
///             || signature == crate::events::EventBodyQtOwned::SIGNATURE
///         {
///             Self::from_message_unchecked_parts(item, msg_body)
///         } else {
///             Err(AtspiError::SignatureMatch(format!(
///                 "The message signature {} does not match a valid signal body signature: {} or {}",
///                 msg.body().signature(),
///                 crate::events::EventBodyOwned::SIGNATURE,
///                 crate::events::EventBodyQtOwned::SIGNATURE,
///             )))
///         }
///     }
/// }
/// ```
macro_rules! impl_msg_conversion_ext_for_target_type {
	($target_type:ty) => {
		#[cfg(feature = "zbus")]
		impl<'msg> crate::events::MessageConversionExt<'msg, crate::events::EventBody<'msg>> for $target_type {
			fn try_from_message(msg: &'msg zbus::Message, header: &Header) -> Result<Self, AtspiError> {
				use zvariant::Type;
				use crate::events::traits::MessageConversion;
				Self::validate_interface(header)?;
				Self::validate_member(header)?;

				let item = ObjectRefOwned::try_from(header)?;
				let msg_body = msg.body();
				let signature = msg_body.signature();

				if signature == crate::events::EventBodyOwned::SIGNATURE
					|| signature == crate::events::EventBodyQtOwned::SIGNATURE
				{
					Self::from_message_unchecked_parts(item.into_inner(), msg_body)
				} else {
					Err(AtspiError::SignatureMatch(format!(
						"The message signature {} does not match a valid signal body signature: {} or {}",
						msg.body().signature(),
						crate::events::EventBodyOwned::SIGNATURE,
						crate::events::EventBodyQtOwned::SIGNATURE,
					)))
				}
			}
		}
	};
}

#[cfg(feature = "wrappers")]
/// Implements `TryFromMessage` for a given event wrapper type.
///
/// # Example
/// ```ignore
/// impl_tryfrommessage_for_event_wrapper!(StateChangedEvent);
/// ```
/// expands to:
///
/// ```ignore
/// #[cfg(feature = "zbus")]
/// impl TryFromMessage for StateChangedEvent {
///     fn try_from_message(msg: &zbus::Message) -> Result<StateChangedEvent, AtspiError> {
///        let header = msg.header();
///        let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
///        if interface != Self::DBUS_INTERFACE {
///            return Err(AtspiError::InterfaceMatch(format!(
///                "Interface {} does not match require interface for event: {}",
///                interface,
///                Self::DBUS_INTERFACE
///            )));
///        }
///        Self::try_from_message_interface_checked(msg, &header)
///     }
/// }
/// ```
macro_rules! impl_tryfrommessage_for_event_wrapper {
	($wrapper:ty) => {
		#[cfg(feature = "zbus")]
		impl crate::events::traits::TryFromMessage for $wrapper {
			fn try_from_message(msg: &zbus::Message) -> Result<$wrapper, AtspiError> {
				use crate::events::traits::EventWrapperMessageConversion;

				let header = msg.header();
				let interface = header.interface().ok_or(AtspiError::MissingInterface)?;
				if interface != Self::DBUS_INTERFACE {
					return Err(AtspiError::InterfaceMatch(format!(
						"Interface {} does not match require interface for event: {}",
						interface,
						Self::DBUS_INTERFACE
					)));
				}
				Self::try_from_message_interface_checked(msg, &header)
			}
		}
	};
}

/// Implement the `MessageConversion` trait for the given types.
///
/// This macro is used to implement the `MessageConversion` trait for types that are built from an
/// `ObjectRef` and a `zbus::message::Body` only - no `EventBody` needed.
///
/// # Example
///
/// ```ignore
/// impl_msg_conversion_for_types_built_from_object_ref!(FocusEvent, FocusEvents);
/// ```
///
/// This will generate the following implementations:
///
/// ```ignore
/// #[cfg(feature = "zbus")]
/// impl MessageConversion<'_> for FocusEvent {
///     type Body<'msg> = crate::events::EventBody<'msg>;
///
///     fn from_message_unchecked_parts(
///         obj_ref: ObjectRefOwned,
///         _body: zbus::message::Body,
///     ) -> Result<Self, AtspiError> {
///         Ok(obj_ref.into())
///     }
///
///     fn from_message_unchecked(_: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
///         let obj_ref: ObjectRefOwned = header.try_into()?;
///         Ok(obj_ref.into())
///     }
///
///     fn body(&self) -> Self::Body<'_> {
///         crate::events::EventBodyOwned::default().into()
///     }
/// }
/// ```
macro_rules! impl_msg_conversion_for_types_built_from_object_ref {
	($($type:ty),*) => {
		$(
			#[cfg(feature = "zbus")]
			impl crate::events::MessageConversion<'_> for $type {
				type Body<'msg> = crate::events::EventBody<'msg>;

				fn from_message_unchecked_parts(
					obj_ref: crate::object_ref::ObjectRef<'_>,
					_body: zbus::message::Body,
				) -> Result<Self, AtspiError> {
					Ok(obj_ref.into())
				}

				fn from_message_unchecked(_: &zbus::Message, header: &Header) -> Result<Self, AtspiError> {
					let obj_ref: ObjectRefOwned = header.try_into()?;
					Ok( Self { item: obj_ref })
				}

				fn body(&self) -> Self::Body<'_> {
					crate::events::EventBodyOwned::default().into()
				}
			}
		)*
	};
}

/// Implement `DBusMember`, `DBusInterface`, `DBusMatchRule`, and `RegistryEventString`
/// for a given event type.
///
/// This macro takes 5 arguments in the order:
/// - The target type
/// - The member string
/// - The interface string
/// - The registry string
/// - The match rule string
///
/// # Example
/// ```ignore
/// impl_member_interface_registry_string_and_match_rule_for_event!(
/// FocusEvent, "Focus", "org.a11y.atspi.Event.Focus", "focus",
/// "type='signal',interface='org.a11y.atspi.Event.Focus'");
/// ```
/// expands to:
///
/// ```ignore
/// impl DBusMember for FocusEvent {
///    const DBUS_MEMBER: &'static str = "Focus";
/// }
/// impl DBusInterface for FocusEvent {
///   const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
/// }
/// impl MatchRule for FocusEvent {
///  const MATCH_RULE: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
/// }
/// impl RegistryEventString for FocusEvent {
///  const REGISTRY_STRING: &'static str = "focus";
/// }
/// impl DBusProperties for FocusEvent {}
/// ```
macro_rules! impl_member_interface_registry_string_and_match_rule_for_event {
	($target_type:ty, $member_str:literal, $interface_str:literal, $registry_str:literal, $match_rule_str:literal) => {
		impl crate::events::DBusMember for $target_type {
			const DBUS_MEMBER: &'static str = $member_str;
		}
		impl crate::events::DBusInterface for $target_type {
			const DBUS_INTERFACE: &'static str = $interface_str;
		}
		impl crate::events::DBusMatchRule for $target_type {
			const MATCH_RULE_STRING: &'static str = $match_rule_str;
		}
		impl crate::events::RegistryEventString for $target_type {
			const REGISTRY_EVENT_STRING: &'static str = $registry_str;
		}
		impl crate::events::DBusProperties for $target_type {}
	};
}

/// Implement `EventTypeProperties` for a given event type.
///
/// This macro takes one argument: the target type.
///
/// # Example
/// ```ignore
/// impl_event_type_properties_for_event!(FocusEvent);
/// ```
/// expands to:
///
/// ```ignore
/// impl EventTypeProperties for FocusEvent {
/// fn member(&self) -> &'static str {
///   Self::DBUS_MEMBER
/// }
/// fn interface(&self) -> &'static str {
///   Self::DBUS_INTERFACE
/// }
/// fn registry_string(&self) -> &'static str {
///   Self::REGISTRY_EVENT_STRING
/// }
/// fn match_rule(&self) -> &'static str {
///     Self::MATCH_RULE_STRING
///   }
/// }
///
macro_rules! impl_event_type_properties_for_event {
	($target_type:ty) => {
		impl crate::events::EventTypeProperties for $target_type {
			fn member(&self) -> &'static str {
				Self::DBUS_MEMBER
			}

			fn interface(&self) -> &'static str {
				Self::DBUS_INTERFACE
			}

			fn registry_string(&self) -> &'static str {
				Self::REGISTRY_EVENT_STRING
			}

			fn match_rule(&self) -> &'static str {
				Self::MATCH_RULE_STRING
			}
		}
	};
}
