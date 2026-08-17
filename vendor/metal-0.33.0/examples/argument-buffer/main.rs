// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::*;
use objc::rc::autoreleasepool;

fn main() {
    autoreleasepool(|| {
        let device = Device::system_default().expect("no device found");

        /*

        // Build encoder for the following MSL argument buffer:
        struct ArgumentBuffer {
            texture2d<float> texture [[id(0)]];
            sampler sampler [[id(1)]];
            array<device float *, 2> buffers [[id(2)]];
        }

         */

        let desc1 = ArgumentDescriptor::new();
        desc1.set_index(0);
        desc1.set_data_type(MTLDataType::Texture);
        desc1.set_texture_type(MTLTextureType::D2);

        let desc2 = ArgumentDescriptor::new();
        desc2.set_index(1);
        desc2.set_data_type(MTLDataType::Sampler);

        let desc3 = ArgumentDescriptor::new();
        desc3.set_index(2);
        desc3.set_data_type(MTLDataType::Pointer);
        desc3.set_array_length(2);

        let encoder = device.new_argument_encoder(Array::from_slice(&[desc1, desc2, desc3]));
        println!("Encoder: {:?}", encoder);

        let argument_buffer =
            device.new_buffer(encoder.encoded_length(), MTLResourceOptions::empty());
        encoder.set_argument_buffer(&argument_buffer, 0);

        let sampler = {
            let descriptor = SamplerDescriptor::new();
            descriptor.set_support_argument_buffers(true);
            device.new_sampler(&descriptor)
        };
        println!("{:?}", sampler);

        let buffer1 = device.new_buffer(1024, MTLResourceOptions::empty());
        println!("Buffer1: {:?}", buffer1);
        let buffer2 = device.new_buffer(1024, MTLResourceOptions::empty());
        println!("Buffer2: {:?}", buffer2);

        encoder.set_sampler_state(1, &sampler);
        encoder.set_buffer(2, &buffer1, 0);
        encoder.set_buffer(3, &buffer2, 0);

        // How to use argument buffer with render encoder.

        let queue = device.new_command_queue();
        let command_buffer = queue.new_command_buffer();

        let render_pass_descriptor = RenderPassDescriptor::new();
        let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);

        // This method makes the array of resources resident for the selected stages of the render pass.
        // Call this method before issuing any draw calls that may access the array of resources.
        encoder.use_resources(
            &[&buffer1, &buffer2],
            MTLResourceUsage::Read,
            MTLRenderStages::Vertex,
        );
        // Bind argument buffer to vertex stage.
        encoder.set_vertex_buffer(0, Some(&argument_buffer), 0);

        // Render pass here...

        encoder.end_encoding();
        println!("Encoder: {:?}", encoder);

        command_buffer.commit();
    });
}
