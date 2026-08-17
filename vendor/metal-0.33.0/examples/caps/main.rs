// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::*;

fn main() {
    let device = Device::system_default().expect("no device found");

    #[cfg(feature = "private")]
    {
        println!("Vendor: {:?}", unsafe { device.vendor() });
        println!("Family: {:?}", unsafe { device.family_name() });
    }
    println!(
        "Max threads per threadgroup: {:?}",
        device.max_threads_per_threadgroup()
    );
    #[cfg(target_os = "macos")]
    {
        println!("Integrated GPU: {:?}", device.is_low_power());
        println!("Headless: {:?}", device.is_headless());
        println!("D24S8: {:?}", device.d24_s8_supported());
    }
    println!("maxBufferLength: {} Mb", device.max_buffer_length() >> 20);
    println!(
        "Indirect argument buffer: {:?}",
        device.argument_buffers_support()
    );
}
