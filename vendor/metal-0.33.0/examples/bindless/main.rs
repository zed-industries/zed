// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::*;
use objc::rc::autoreleasepool;

const BINDLESS_TEXTURE_COUNT: NSUInteger = 100_000; // ~25Mb

/// This example demonstrates:
/// - How to create a heap
/// - How to allocate textures from heap.
/// - How to create bindless resources via Metal's argument buffers.
/// - How to bind argument buffer to render encoder
fn main() {
    autoreleasepool(|| {
        let device = Device::system_default().expect("no device found");

        /*

        MSL

        struct Textures {
            texture2d<float> texture;
        };
        struct BindlessTextures {
            device Textures *textures;
        };

         */

        // Tier 2 argument buffers are supported by macOS devices with a discrete GPU and by the A13 GPU.
        // The maximum per-app resources available at any given time are:
        // - 500,000 buffers or textures
        // - 2048 unique samplers
        let tier = device.argument_buffers_support();
        println!("Argument buffer support: {:?}", tier);
        assert_eq!(MTLArgumentBuffersTier::Tier2, tier);

        let texture_descriptor = TextureDescriptor::new();
        texture_descriptor.set_width(1);
        texture_descriptor.set_height(1);
        texture_descriptor.set_depth(1);
        texture_descriptor.set_texture_type(MTLTextureType::D2);
        texture_descriptor.set_pixel_format(MTLPixelFormat::R8Uint);
        texture_descriptor.set_storage_mode(MTLStorageMode::Private); // GPU only.
        println!("Texture descriptor: {:?}", texture_descriptor);

        // Determine the size required for the heap for the given descriptor
        let size_and_align = device.heap_texture_size_and_align(&texture_descriptor);

        // Align the size so that more resources will fit in the heap after this texture
        // See https://developer.apple.com/documentation/metal/buffers/using_argument_buffers_with_resource_heaps
        let texture_size =
            (size_and_align.size & (size_and_align.align - 1)) + size_and_align.align;
        let heap_size = texture_size * BINDLESS_TEXTURE_COUNT;

        let heap_descriptor = HeapDescriptor::new();
        heap_descriptor.set_storage_mode(texture_descriptor.storage_mode()); // Must be compatible
        heap_descriptor.set_size(heap_size);
        println!("Heap descriptor: {:?}", heap_descriptor);

        let heap = device.new_heap(&heap_descriptor);
        println!("Heap: {:?}", heap);

        // Allocate textures from heap
        let textures = (0..BINDLESS_TEXTURE_COUNT)
            .map(|i| {
                heap.new_texture(&texture_descriptor)
                    .unwrap_or_else(|| panic!("Failed to allocate texture {}", i))
            })
            .collect::<Vec<_>>();

        // Crate argument encoder that knows how to encode single texture
        let descriptor = ArgumentDescriptor::new();
        descriptor.set_index(0);
        descriptor.set_data_type(MTLDataType::Texture);
        descriptor.set_texture_type(MTLTextureType::D2);
        descriptor.set_access(MTLArgumentAccess::ReadOnly);
        println!("Argument descriptor: {:?}", descriptor);

        let encoder = device.new_argument_encoder(Array::from_slice(&[descriptor]));
        println!("Encoder: {:?}", encoder);

        // Determinate argument buffer size to allocate.
        // Size needed to encode one texture * total number of bindless textures.
        let argument_buffer_size = encoder.encoded_length() * BINDLESS_TEXTURE_COUNT;
        let argument_buffer = device.new_buffer(argument_buffer_size, MTLResourceOptions::empty());

        // Encode textures to the argument buffer.
        textures.iter().enumerate().for_each(|(index, texture)| {
            // Offset encoder to a proper texture slot
            let offset = index as NSUInteger * encoder.encoded_length();
            encoder.set_argument_buffer(&argument_buffer, offset);
            encoder.set_texture(0, texture);
        });

        // How to use bindless argument buffer when drawing

        let queue = device.new_command_queue();
        let command_buffer = queue.new_command_buffer();

        let render_pass_descriptor = RenderPassDescriptor::new();
        let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);

        // Bind argument buffer.
        encoder.set_fragment_buffer(0, Some(&argument_buffer), 0);
        // Make sure all textures are available to the pass.
        encoder.use_heap_at(&heap, MTLRenderStages::Fragment);

        // Bind material buffer at index 1
        // Draw

        /*

        // Now instead of binding individual textures each draw call,
        // you can just bind material information instead:

        MSL

        struct Material {
            int diffuse_texture_index;
            int normal_texture_index;
            // ...
        }

        fragment float4 pixel(
            VertexOut v [[stage_in]],
            constant const BindlessTextures * textures [[buffer(0)]],
            constant Material * material [[buffer(1)]]
        ) {
            if (material->base_color_texture_index != -1) {
                textures[material->diffuse_texture_index].texture.sampler(...)
            }
            if (material->normal_texture_index != -1) {
                ...
            }
            ...
        }

         */

        encoder.end_encoding();
        command_buffer.commit();
    });
}
