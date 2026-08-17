// Copyright 2018 GFX developers
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

        let buffer = device.new_buffer(4, MTLResourceOptions::empty());
        let sampler = {
            let descriptor = SamplerDescriptor::new();
            device.new_sampler(&descriptor)
        };

        let queue = device.new_command_queue();
        let cmd_buf = queue.new_command_buffer();

        let encoder = cmd_buf.new_compute_command_encoder();

        encoder.set_buffers(2, &[Some(&buffer), None], &[4, 0]);
        encoder.set_sampler_states(1, &[Some(&sampler), None]);

        encoder.end_encoding();
        cmd_buf.commit();

        println!("Everything is bound");
    });
}
