use std::fmt;
use std::ops::Deref;

use crate::message_dyn::MessageDyn;
use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::MessageDescriptor;
use crate::MessageFull;

#[derive(Clone, Debug)]
enum MessageRefImpl<'a> {
    Message(&'a dyn MessageDyn),
    EmptyDynamic(DynamicMessage),
}

/// Wrapper around either [`MessageFull`] reference or a container for an empty dynamic message.
#[derive(Clone, Debug)]
pub struct MessageRef<'a> {
    imp: MessageRefImpl<'a>,
}

impl<'a> fmt::Display for MessageRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.deref(), f)
    }
}

impl<'a> From<&'a dyn MessageDyn> for MessageRef<'a> {
    fn from(m: &'a dyn MessageDyn) -> Self {
        MessageRef {
            imp: MessageRefImpl::Message(m),
        }
    }
}

impl<'a, M: MessageFull> From<&'a M> for MessageRef<'a> {
    fn from(m: &'a M) -> Self {
        MessageRef {
            imp: MessageRefImpl::Message(m),
        }
    }
}

impl<'a> ReflectEq for MessageRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        let ad = self.descriptor_dyn();
        let bd = that.descriptor_dyn();
        ad == bd && ad.reflect_eq(&**self, &**that, mode)
    }
}

impl<'a> MessageRef<'a> {
    /// Wrap a message.
    pub fn new(message: &'a dyn MessageDyn) -> MessageRef<'a> {
        MessageRef {
            imp: MessageRefImpl::Message(message),
        }
    }

    /// Default (empty) instance of given message type.
    pub fn default_instance(message: &MessageDescriptor) -> MessageRef<'static> {
        // Note we create a native generated instance for generated types
        // and dynamic message for dynamic types.
        match message.default_instance() {
            Some(m) => MessageRef::new(m),
            None => MessageRef {
                imp: MessageRefImpl::EmptyDynamic(DynamicMessage::new(message.clone())),
            },
        }
    }
}

impl<'a> Deref for MessageRef<'a> {
    type Target = dyn MessageDyn;

    fn deref(&self) -> &dyn MessageDyn {
        match &self.imp {
            MessageRefImpl::Message(m) => *m,
            MessageRefImpl::EmptyDynamic(e) => e,
        }
    }
}
