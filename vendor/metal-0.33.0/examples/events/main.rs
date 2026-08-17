// Copyright 2020 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use dispatch::{Queue, QueueAttribute};
use metal::*;

/// This example replicates `Synchronizing Events Between a GPU and the CPU` article.
/// See https://developer.apple.com/documentation/metal/synchronization/synchronizing_events_between_a_gpu_and_the_cpu
fn main() {
    let device = Device::system_default().expect("No device found");

    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();

    // Shareable event
    let shared_event = device.new_shared_event();

    // Shareable event listener
    let my_queue = Queue::create(
        "com.example.apple-samplecode.MyQueue",
        QueueAttribute::Serial,
    );

    // Enable `dispatch` feature to use dispatch queues,
    // otherwise unsafe `from_queue_handle` is available for use with native APIs.
    let shared_event_listener = SharedEventListener::from_queue(&my_queue);

    // Register CPU work
    let notify_block = block::ConcreteBlock::new(move |evt: &SharedEventRef, val: u64| {
        println!("Got notification from GPU: {}", val);
        evt.set_signaled_value(3);
    });

    shared_event.notify(&shared_event_listener, 2, notify_block.copy());

    // Encode GPU work
    command_buffer.encode_signal_event(&shared_event, 1);
    command_buffer.encode_signal_event(&shared_event, 2);
    command_buffer.encode_wait_for_event(&shared_event, 3);

    command_buffer.commit();

    command_buffer.wait_until_completed();

    println!("Done");
}
