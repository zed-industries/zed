use crate::descriptor::MethodDescriptorProto;
use crate::descriptor::ServiceDescriptorProto;
use crate::reflect::service::index::MethodIndex;
use crate::reflect::service::index::ServiceIndex;
use crate::reflect::FileDescriptor;
use crate::reflect::MessageDescriptor;

pub(crate) mod index;

/// Dynamic representation of service type.
///
/// Rust-protobuf does not support services (it is not an RPC library),
/// but it support querying service description. Which might be useful
/// for example to generate source files for the services.
/// or to perform invocations dynamically.
#[derive(Clone, Eq, PartialEq)]
pub struct ServiceDescriptor {
    file_descriptor: FileDescriptor,
    index: usize,
}

impl ServiceDescriptor {
    pub(crate) fn new(file_descriptor: FileDescriptor, index: usize) -> ServiceDescriptor {
        ServiceDescriptor {
            file_descriptor,
            index,
        }
    }

    fn index(&self) -> &ServiceIndex {
        &self.file_descriptor.common().services[self.index]
    }

    /// Proto snippet describing this service.
    pub fn proto(&self) -> &ServiceDescriptorProto {
        &self.file_descriptor.proto().service[self.index]
    }

    /// Method descriptors of this service.
    pub fn methods(&self) -> impl Iterator<Item = MethodDescriptor> + '_ {
        let value_len = self.proto().method.len();
        (0..value_len).map(move |index| MethodDescriptor {
            service_descriptor: self.clone(),
            index,
        })
    }
}

/// Service method descriptor.
pub struct MethodDescriptor {
    service_descriptor: ServiceDescriptor,
    index: usize,
}

impl MethodDescriptor {
    fn index(&self) -> &MethodIndex {
        &self.service_descriptor.index().methods[self.index]
    }

    /// Proto snippet describing this method.
    pub fn proto(&self) -> &MethodDescriptorProto {
        &self.service_descriptor.proto().method[self.index]
    }

    /// Method input type.
    pub fn input_type(&self) -> MessageDescriptor {
        self.index()
            .input_type
            .resolve_message(&self.service_descriptor.file_descriptor)
    }

    /// Method output type.
    pub fn output_type(&self) -> MessageDescriptor {
        self.index()
            .output_type
            .resolve_message(&self.service_descriptor.file_descriptor)
    }
}
