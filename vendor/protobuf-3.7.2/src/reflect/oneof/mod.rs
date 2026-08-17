pub(crate) mod generated;

use crate::descriptor::OneofDescriptorProto;
use crate::reflect::file::index::OneofIndices;
use crate::reflect::file::FileDescriptorImpl;
use crate::reflect::oneof::generated::GeneratedOneofDescriptor;
use crate::reflect::FieldDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::MessageDescriptor;

/// Oneof descriptor.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OneofDescriptor {
    pub(crate) file_descriptor: FileDescriptor,
    pub(crate) index: usize,
}

pub(crate) enum OneofDescriptorImplRef {
    #[allow(dead_code)]
    Generated(&'static GeneratedOneofDescriptor),
    Dynamic,
}

impl OneofDescriptor {
    fn index_entry(&self) -> &OneofIndices {
        &self.file_descriptor.common().oneofs[self.index]
    }

    /// `.proto` part associated with this descriptor
    pub fn proto(&self) -> &OneofDescriptorProto {
        let index_entry = self.index_entry();
        let message_descriptor = self
            .file_descriptor
            .message_proto_by_index(index_entry.containing_message);
        &message_descriptor.oneof_decl[index_entry.index_in_containing_message]
    }

    /// Oneof name as specified in `.proto` file.
    pub fn name(&self) -> &str {
        self.proto().name()
    }

    #[allow(dead_code)]
    pub(crate) fn _get_impl(&self) -> OneofDescriptorImplRef {
        match &self.file_descriptor.imp {
            FileDescriptorImpl::Generated(g) => {
                OneofDescriptorImplRef::Generated(&g.oneofs[self.index])
            }
            FileDescriptorImpl::Dynamic(..) => OneofDescriptorImplRef::Dynamic,
        }
    }

    /// Message which contains this oneof.
    pub fn containing_message(&self) -> MessageDescriptor {
        MessageDescriptor {
            file_descriptor: self.file_descriptor.clone(),
            index: self.index_entry().containing_message,
        }
    }

    /// This oneof is not present in sources.
    pub fn is_synthetic(&self) -> bool {
        self.index_entry().synthetic
    }

    /// Fully qualified name of oneof (fully qualified name of enclosing message
    /// followed by oneof name).
    pub fn full_name(&self) -> String {
        format!("{}.{}", self.containing_message(), self.name())
    }

    /// Fields in this oneof.
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = FieldDescriptor> + 'a {
        let message = self.containing_message();
        self.index_entry()
            .fields
            .iter()
            .map(move |&i| message.field_by_index(i))
    }
}
