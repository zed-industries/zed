bitflags::bitflags! {
    /// Flags to augment descriptor pool creation.
    ///
    /// Match corresponding bits in Vulkan.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct DescriptorPoolCreateFlags: u32 {
        /// Allows freeing individual sets.
        const FREE_DESCRIPTOR_SET = 0x1;

        /// Allows allocating sets with layout created with matching backend-specific flag.
        const UPDATE_AFTER_BIND = 0x2;
    }
}

/// Number of descriptors of each type.
///
/// For `InlineUniformBlock` this value is number of bytes instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DescriptorTotalCount {
    pub sampler: u32,
    pub combined_image_sampler: u32,
    pub sampled_image: u32,
    pub storage_image: u32,
    pub uniform_texel_buffer: u32,
    pub storage_texel_buffer: u32,
    pub uniform_buffer: u32,
    pub storage_buffer: u32,
    pub uniform_buffer_dynamic: u32,
    pub storage_buffer_dynamic: u32,
    pub input_attachment: u32,
    pub acceleration_structure: u32,
    pub inline_uniform_block_bytes: u32,
    pub inline_uniform_block_bindings: u32,
}

impl DescriptorTotalCount {
    pub fn total(&self) -> u32 {
        self.sampler
            + self.combined_image_sampler
            + self.sampled_image
            + self.storage_image
            + self.uniform_texel_buffer
            + self.storage_texel_buffer
            + self.uniform_buffer
            + self.storage_buffer
            + self.uniform_buffer_dynamic
            + self.storage_buffer_dynamic
            + self.input_attachment
            + self.acceleration_structure
            + self.inline_uniform_block_bytes
            + self.inline_uniform_block_bindings
    }
}
