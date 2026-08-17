// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::*;
use objc::rc::autoreleasepool;
use std::mem::size_of_val;

static LIBRARY_SRC: &str = include_str!("compute-argument-buffer.metal");

fn main() {
    autoreleasepool(|| {
        let device = Device::system_default().expect("no device found");
        let command_queue = device.new_command_queue();

        let data = [
            1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30,
        ];

        let buffer = device.new_buffer_with_data(
            data.as_ptr().cast(),
            size_of_val(&data) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );

        let sum = {
            let data = [0u32];
            device.new_buffer_with_data(
                data.as_ptr().cast(),
                size_of_val(&data) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            )
        };

        let command_buffer = command_queue.new_command_buffer();
        let encoder = command_buffer.new_compute_command_encoder();

        let library = device
            .new_library_with_source(LIBRARY_SRC, &CompileOptions::new())
            .unwrap();
        let kernel = library.get_function("sum", None).unwrap();

        let argument_encoder = kernel.new_argument_encoder(0);
        let arg_buffer = device.new_buffer(
            argument_encoder.encoded_length(),
            MTLResourceOptions::empty(),
        );
        argument_encoder.set_argument_buffer(&arg_buffer, 0);
        argument_encoder.set_buffer(0, &buffer, 0);
        argument_encoder.set_buffer(1, &sum, 0);

        let pipeline_state_descriptor = ComputePipelineDescriptor::new();
        pipeline_state_descriptor.set_compute_function(Some(&kernel));

        let pipeline_state = device
            .new_compute_pipeline_state_with_function(
                pipeline_state_descriptor.compute_function().unwrap(),
            )
            .unwrap();

        encoder.set_compute_pipeline_state(&pipeline_state);
        encoder.set_buffer(0, Some(&arg_buffer), 0);

        encoder.use_resource(&buffer, MTLResourceUsage::Read);
        encoder.use_resource(&sum, MTLResourceUsage::Write);

        let width = 16;

        let thread_group_count = MTLSize {
            width,
            height: 1,
            depth: 1,
        };

        let thread_group_size = MTLSize {
            width: (data.len() as u64 + width) / width,
            height: 1,
            depth: 1,
        };

        encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
        encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let sum = unsafe { *sum.contents().cast::<u32>() };
        assert_eq!(465, sum);
    });
}
