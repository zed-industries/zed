use alloc::borrow::ToOwned as _;

use wgt::TextureDataOrder;

/// Describes a [Buffer](crate::Buffer) when allocating.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BufferInitDescriptor<'a> {
    /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
    pub label: crate::Label<'a>,
    /// Contents of a buffer on creation.
    pub contents: &'a [u8],
    /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
    /// will panic.
    pub usage: wgt::BufferUsages,
}

/// Utility methods not meant to be in the main API.
pub trait DeviceExt {
    /// Creates a [Buffer](crate::Buffer) with data to initialize it.
    fn create_buffer_init(&self, desc: &BufferInitDescriptor<'_>) -> crate::Buffer;

    /// Upload an entire texture and its mipmaps from a source buffer.
    ///
    /// Expects all mipmaps to be tightly packed in the data buffer.
    ///
    /// See [`TextureDataOrder`] for the order in which the data is laid out in memory.
    ///
    /// Implicitly adds the `COPY_DST` usage if it is not present in the descriptor,
    /// as it is required to be able to upload the data to the gpu.
    fn create_texture_with_data(
        &self,
        queue: &crate::Queue,
        desc: &crate::TextureDescriptor<'_>,
        order: TextureDataOrder,
        data: &[u8],
    ) -> crate::Texture;
}

impl DeviceExt for crate::Device {
    fn create_buffer_init(&self, descriptor: &BufferInitDescriptor<'_>) -> crate::Buffer {
        // Skip mapping if the buffer is zero sized
        if descriptor.contents.is_empty() {
            let wgt_descriptor = crate::BufferDescriptor {
                label: descriptor.label,
                size: 0,
                usage: descriptor.usage,
                mapped_at_creation: false,
            };

            self.create_buffer(&wgt_descriptor)
        } else {
            let unpadded_size = descriptor.contents.len() as crate::BufferAddress;
            // Valid vulkan usage is
            // 1. buffer size must be a multiple of COPY_BUFFER_ALIGNMENT.
            // 2. buffer size must be greater than 0.
            // Therefore we round the value up to the nearest multiple, and ensure it's at least COPY_BUFFER_ALIGNMENT.
            let align_mask = crate::COPY_BUFFER_ALIGNMENT - 1;
            let padded_size =
                ((unpadded_size + align_mask) & !align_mask).max(crate::COPY_BUFFER_ALIGNMENT);

            let wgt_descriptor = crate::BufferDescriptor {
                label: descriptor.label,
                size: padded_size,
                usage: descriptor.usage,
                mapped_at_creation: true,
            };

            let buffer = self.create_buffer(&wgt_descriptor);

            buffer
                .get_mapped_range_mut(..)
                .slice(..unpadded_size as usize)
                .copy_from_slice(descriptor.contents);
            buffer.unmap();

            buffer
        }
    }

    fn create_texture_with_data(
        &self,
        queue: &crate::Queue,
        desc: &crate::TextureDescriptor<'_>,
        order: TextureDataOrder,
        data: &[u8],
    ) -> crate::Texture {
        // Implicitly add the COPY_DST usage
        let mut desc = desc.to_owned();
        desc.usage |= crate::TextureUsages::COPY_DST;
        let texture = self.create_texture(&desc);

        // Will return None only if it's a combined depth-stencil format
        // If so, default to 4, validation will fail later anyway since the depth or stencil
        // aspect needs to be written to individually
        let block_size = desc.format.block_copy_size(None).unwrap_or(4);
        let (block_width, block_height) = desc.format.block_dimensions();
        let layer_iterations = desc.array_layer_count();

        let outer_iteration;
        let inner_iteration;
        match order {
            TextureDataOrder::LayerMajor => {
                outer_iteration = layer_iterations;
                inner_iteration = desc.mip_level_count;
            }
            TextureDataOrder::MipMajor => {
                outer_iteration = desc.mip_level_count;
                inner_iteration = layer_iterations;
            }
        }

        let mut binary_offset = 0;
        for outer in 0..outer_iteration {
            for inner in 0..inner_iteration {
                let (layer, mip) = match order {
                    TextureDataOrder::LayerMajor => (outer, inner),
                    TextureDataOrder::MipMajor => (inner, outer),
                };

                let mut mip_size = desc.mip_level_size(mip).unwrap();
                // copying layers separately
                if desc.dimension != wgt::TextureDimension::D3 {
                    mip_size.depth_or_array_layers = 1;
                }

                // When uploading mips of compressed textures and the mip is supposed to be
                // a size that isn't a multiple of the block size, the mip needs to be uploaded
                // as its "physical size" which is the size rounded up to the nearest block size.
                let mip_physical = mip_size.physical_size(desc.format);

                // All these calculations are performed on the physical size as that's the
                // data that exists in the buffer.
                let width_blocks = mip_physical.width / block_width;
                let height_blocks = mip_physical.height / block_height;

                let bytes_per_row = width_blocks * block_size;
                let data_size = bytes_per_row * height_blocks * mip_size.depth_or_array_layers;

                let end_offset = binary_offset + data_size as usize;

                queue.write_texture(
                    crate::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: mip,
                        origin: crate::Origin3d {
                            x: 0,
                            y: 0,
                            z: layer,
                        },
                        aspect: wgt::TextureAspect::All,
                    },
                    &data[binary_offset..end_offset],
                    crate::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(bytes_per_row),
                        rows_per_image: Some(height_blocks),
                    },
                    mip_physical,
                );

                binary_offset = end_offset;
            }
        }

        texture
    }
}
