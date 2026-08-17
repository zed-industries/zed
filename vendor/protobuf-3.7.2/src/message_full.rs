use std::fmt;

use crate::message_dyn::MessageDyn;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::MessageDescriptor;
use crate::reflect::ProtobufValue;
use crate::Message;

/// Trait implemented for all the generated messages, except when lite runtime is enabled.
///
/// When lite runtime is enabled, only `MessageLite` is implemented.
///
/// * Generated messages are generated from `.proto` files
/// * Dynamic messages can be created without code generation using only parsed proto files
///   (see [FileDescriptor::new_dynamic](crate::reflect::FileDescriptor::new_dynamic)).
///
/// Also, generated messages implement `Default + PartialEq`
///
/// This trait is sized, there's accompanying [`MessageDyn`](crate::MessageDyn) trait
/// which is implemented for all messages which can be used in functions
/// without making message a function type parameter.
///
/// ## `Display`
///
/// [`Display`](fmt::Display) implementation for messages does protobuf text format.
/// See [`text_format`](crate::text_format) for more details.
pub trait MessageFull: Message + ProtobufValue + fmt::Debug + fmt::Display {
    /// Get message descriptor for message type.
    ///
    /// ```
    /// # use protobuf::MessageFull;
    /// # fn foo<MyMessage: MessageFull>() {
    /// let descriptor = MyMessage::descriptor();
    /// assert_eq!("MyMessage", descriptor.name());
    /// # }
    /// ```
    fn descriptor() -> MessageDescriptor;

    /// Reflective equality.
    ///
    /// # See also
    ///
    /// [`dyn Message::reflect_eq_dyn()`], `dyn` version of this function.
    fn reflect_eq(&self, other: &Self, mode: &ReflectEqMode) -> bool {
        <dyn MessageDyn>::reflect_eq_dyn(self, other, mode)
    }
}
