use std::fmt;
use std::io::Read;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::message_dyn::MessageDyn;
use crate::message_full::MessageFull;
use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::file::index::MessageIndices;
use crate::reflect::file::FileDescriptorImpl;
use crate::reflect::message::generated::GeneratedMessageDescriptor;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::EnumDescriptor;
use crate::reflect::FieldDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::OneofDescriptor;
use crate::CodedInputStream;

pub(crate) mod generated;
pub(crate) mod is_initialized_is_always_true;
pub(crate) mod message_ref;

/// Dynamic representation of message type.
///
/// Used for reflection.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MessageDescriptor {
    pub(crate) file_descriptor: FileDescriptor,
    pub(crate) index: usize,
}

impl fmt::Display for MessageDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl fmt::Debug for MessageDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MessageDescriptor").finish_non_exhaustive()
    }
}

impl MessageDescriptor {
    pub(crate) fn new(file_descriptor: FileDescriptor, index: usize) -> MessageDescriptor {
        MessageDescriptor {
            file_descriptor,
            index,
        }
    }

    /// Get underlying `DescriptorProto` object.
    pub fn proto(&self) -> &DescriptorProto {
        self.file_descriptor.message_proto_by_index(self.index)
    }

    /// Message name as specified in `.proto` file.
    pub fn name(&self) -> &str {
        self.proto().name()
    }

    fn index_entry(&self) -> &MessageIndices {
        self.file_descriptor.message_indices(self.index)
    }

    /// Get a message descriptor for given message type
    pub fn for_type<M: MessageFull>() -> MessageDescriptor {
        M::descriptor()
    }

    /// Messages declared in this messages.
    pub fn nested_messages(&self) -> impl Iterator<Item = MessageDescriptor> + '_ {
        self.index_entry()
            .nested_messages
            .iter()
            .map(|i| MessageDescriptor::new(self.file_descriptor.clone(), *i))
    }

    /// Get enums declared in this message.
    pub fn nested_enums(&self) -> impl Iterator<Item = EnumDescriptor> + '_ {
        self.index_entry()
            .nested_enums
            .clone()
            .map(|i| EnumDescriptor::new(self.file_descriptor.clone(), i))
    }

    /// Get a message containing this message, or `None` if this message is declared at file level.
    pub fn enclosing_message(&self) -> Option<MessageDescriptor> {
        self.index_entry()
            .enclosing_message
            .map(|i| MessageDescriptor::new(self.file_descriptor.clone(), i))
    }

    pub(crate) fn get_impl(&self) -> MessageDescriptorImplRef {
        match &self.file_descriptor.imp {
            FileDescriptorImpl::Generated(g) => {
                MessageDescriptorImplRef::Generated(&g.messages[self.index])
            }
            FileDescriptorImpl::Dynamic(..) => MessageDescriptorImplRef::Dynamic,
        }
    }

    /// [`FileDescriptor`] containing this message.
    pub fn file_descriptor(&self) -> &FileDescriptor {
        &self.file_descriptor
    }

    /// `FileDescriptorProto` containg this message type
    pub fn file_descriptor_proto(&self) -> &FileDescriptorProto {
        self.file_descriptor().proto()
    }

    /// This message descriptor is a map entry.
    pub fn is_map_entry(&self) -> bool {
        self.index().map_entry
    }

    fn assert_not_map_entry(&self) {
        assert!(
            !self.is_map_entry(),
            "message is map entry: {}",
            self.full_name()
        );
    }

    /// Message is considered always initialized.
    #[doc(hidden)]
    pub fn is_initialized_is_always_true(&self) -> bool {
        self.index().is_initialized_is_always_true
    }

    /// New empty message.
    ///
    /// # Panics
    ///
    /// If this message is a map entry message.
    pub fn new_instance(&self) -> Box<dyn MessageDyn> {
        self.assert_not_map_entry();
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.non_map().factory.new_instance(),
            MessageDescriptorImplRef::Dynamic => Box::new(DynamicMessage::new(self.clone())),
        }
    }

    /// Shared immutable empty message.
    ///
    /// Returns `None` for dynamic message.
    ///
    /// # Panics
    ///
    /// If this message is a map entry message.
    pub fn default_instance(&self) -> Option<&'static dyn MessageDyn> {
        self.assert_not_map_entry();
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => Some(g.non_map().factory.default_instance()),
            MessageDescriptorImplRef::Dynamic => None,
        }
    }

    /// Clone a message
    pub(crate) fn clone_message(&self, message: &dyn MessageDyn) -> Box<dyn MessageDyn> {
        assert!(&message.descriptor_dyn() == self);
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.non_map().factory.clone(message),
            MessageDescriptorImplRef::Dynamic => {
                let message: &DynamicMessage = DynamicMessage::downcast_ref(message);
                Box::new(message.clone())
            }
        }
    }

    /// Check if two messages equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub fn eq(&self, a: &dyn MessageDyn, b: &dyn MessageDyn) -> bool {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.non_map().factory.eq(a, b),
            MessageDescriptorImplRef::Dynamic => unimplemented!(),
        }
    }

    /// Similar to `eq`, but considers `NaN` values equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub(crate) fn reflect_eq(
        &self,
        a: &dyn MessageDyn,
        b: &dyn MessageDyn,
        mode: &ReflectEqMode,
    ) -> bool {
        // Explicitly force panic even if field list is empty
        assert_eq!(self, &a.descriptor_dyn());
        assert_eq!(self, &b.descriptor_dyn());

        for field in self.fields() {
            let af = field.get_reflect(a);
            let bf = field.get_reflect(b);
            if !af.reflect_eq(&bf, mode) {
                return false;
            }
        }
        true
    }

    pub(crate) fn reflect_eq_maybe_unrelated(
        a: &dyn MessageDyn,
        b: &dyn MessageDyn,
        mode: &ReflectEqMode,
    ) -> bool {
        let ad = a.descriptor_dyn();
        let bd = b.descriptor_dyn();
        ad == bd && ad.reflect_eq(a, b, mode)
    }

    /// Fully qualified protobuf message name
    pub fn full_name(&self) -> &str {
        &self.index_entry().full_name
    }

    /// Name relative to the package where the message is declared.
    pub fn name_to_package(&self) -> &str {
        &self.index_entry().name_to_package
    }

    /// Nested oneofs including synthetic.
    pub fn all_oneofs<'a>(&'a self) -> impl Iterator<Item = OneofDescriptor> + 'a {
        self.index_entry()
            .oneofs
            .clone()
            .map(move |i| OneofDescriptor {
                file_descriptor: self.file_descriptor.clone(),
                index: i,
            })
    }

    /// Non-synthetic oneofs.
    pub fn oneofs<'a>(&'a self) -> impl Iterator<Item = OneofDescriptor> + 'a {
        self.all_oneofs().filter(|oneof| !oneof.is_synthetic())
    }

    /// Get message oneof by name (**not implemented**).
    pub fn oneof_by_name(&self, name: &str) -> Option<OneofDescriptor> {
        self.all_oneofs().find(|oneof| oneof.name() == name)
    }

    /// Message field descriptors.
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = FieldDescriptor> + 'a {
        self.index()
            .message_index
            .regular_field_range()
            .map(move |index| FieldDescriptor {
                file_descriptor: self.file_descriptor.clone(),
                index,
            })
    }

    /// Extension fields.
    pub fn extensions(&self) -> impl Iterator<Item = FieldDescriptor> + '_ {
        self.index()
            .message_index
            .extension_field_range()
            .map(move |index| FieldDescriptor {
                file_descriptor: self.file_descriptor.clone(),
                index,
            })
    }

    pub(crate) fn index(&self) -> &MessageIndices {
        &self.file_descriptor.common().messages[self.index]
    }

    pub(crate) fn field_by_index(&self, index: usize) -> FieldDescriptor {
        FieldDescriptor {
            file_descriptor: self.file_descriptor.clone(),
            index: self.index().message_index.first_field_index + index,
        }
    }

    /// Find message field by protobuf field name
    ///
    /// Note: protobuf field name might be different for Rust field name.
    // TODO: return value, not pointer, pointer is not compatible with dynamic message
    pub fn field_by_name(&self, name: &str) -> Option<FieldDescriptor> {
        let &index = self.index().message_index.field_index_by_name.get(name)?;
        Some(self.field_by_index(index))
    }

    /// Find message field by field name or field JSON name
    pub fn field_by_name_or_json_name<'a>(&'a self, name: &str) -> Option<FieldDescriptor> {
        let &index = self
            .index()
            .message_index
            .field_index_by_name_or_json_name
            .get(name)?;
        Some(self.field_by_index(index))
    }

    /// Find message field by field name
    pub fn field_by_number(&self, number: u32) -> Option<FieldDescriptor> {
        let &index = self
            .index()
            .message_index
            .field_index_by_number
            .get(&number)?;
        Some(self.field_by_index(index))
    }

    /// Parse message from stream.
    pub fn parse_from(&self, is: &mut CodedInputStream) -> crate::Result<Box<dyn MessageDyn>> {
        let mut r = self.new_instance();
        r.merge_from_dyn(is)?;
        r.check_initialized_dyn()?;
        Ok(r)
    }

    /// Parse message from reader.
    /// Parse stops on EOF or when error encountered.
    pub fn parse_from_reader(&self, reader: &mut dyn Read) -> crate::Result<Box<dyn MessageDyn>> {
        let mut is = CodedInputStream::new(reader);
        let r = self.parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Parse message from byte array.
    pub fn parse_from_bytes(&self, bytes: &[u8]) -> crate::Result<Box<dyn MessageDyn>> {
        let mut is = CodedInputStream::from_bytes(bytes);
        let r = self.parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }
}

pub(crate) enum MessageDescriptorImplRef {
    Generated(&'static GeneratedMessageDescriptor),
    Dynamic,
}

#[cfg(test)]
mod test {
    use crate::descriptor::descriptor_proto::ExtensionRange;
    use crate::descriptor::field_descriptor_proto::Type;
    use crate::descriptor::DescriptorProto;
    use crate::descriptor::FieldDescriptorProto;
    use crate::EnumFull;
    use crate::MessageFull;

    #[test]
    #[cfg_attr(miri, ignore)] // Too slow on Miri.
    fn nested_messages() {
        assert!(DescriptorProto::descriptor()
            .nested_messages()
            .collect::<Vec<_>>()
            .contains(&ExtensionRange::descriptor()));
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Too slow on Miri.
    fn nested_enums() {
        assert!(FieldDescriptorProto::descriptor()
            .nested_enums()
            .collect::<Vec<_>>()
            .contains(&Type::enum_descriptor()));
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Too slow on Miri.
    fn enclosing_message() {
        assert_eq!(
            Some(DescriptorProto::descriptor()),
            ExtensionRange::descriptor().enclosing_message()
        );
        assert_eq!(None, DescriptorProto::descriptor().enclosing_message());
    }
}
