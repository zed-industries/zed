// Copyright 2020 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::*;

fn main() {
    let device = Device::system_default().expect("No device found");

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    let fence = device.new_fence();

    let blit_encoder = command_buffer.new_blit_command_encoder();
    blit_encoder.update_fence(&fence);
    blit_encoder.end_encoding();

    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.wait_for_fence(&fence);
    compute_encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!("Done");
}
