use crate::OneofFull;

#[doc(hidden)]
pub struct GeneratedOneofDescriptorData {
    pub(crate) name: &'static str,
}

impl GeneratedOneofDescriptorData {
    #[doc(hidden)]
    pub fn new<O>(name: &'static str) -> GeneratedOneofDescriptorData
    where
        O: OneofFull,
    {
        GeneratedOneofDescriptorData { name }
    }
}

#[derive(Debug)]
pub(crate) struct GeneratedOneofDescriptor {}

impl GeneratedOneofDescriptor {
    /// Synthetic oneof for proto3 optional field.
    pub(crate) fn new_synthetic() -> GeneratedOneofDescriptor {
        GeneratedOneofDescriptor {}
    }

    pub(crate) fn new(_data: &GeneratedOneofDescriptorData) -> GeneratedOneofDescriptor {
        GeneratedOneofDescriptor {}
    }
}
