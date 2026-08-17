# gpu-descriptor

[![crates](https://img.shields.io/crates/v/gpu-descriptor.svg?style=for-the-badge&label=gpu-descriptor)](https://crates.io/crates/gpu-descriptor)
[![docs](https://img.shields.io/badge/docs.rs-gpu--descriptor-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white)](https://docs.rs/gpu-descriptor)
[![actions](https://img.shields.io/github/workflow/status/zakarumych/gpu-descriptor/badge/master?style=for-the-badge)](https://github.com/zakarumych/gpu-descriptor/actions?query=workflow%3ARust)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](COPYING)
![loc](https://img.shields.io/tokei/lines/github/zakarumych/gpu-descriptor?style=for-the-badge)


Library for Vulkan-like APIs to allocated descriptor sets
from descriptor pools fast, with least overhead and zero fragmentation.

Straightforward usage:
```rust
use gpu_descriptor::DescriptorAllocator;

let mut allocator = DescriptorAllocator::new(max_update_after_bind_descriptors_in_all_pools); // Limit as dictated by API for selected hardware

let result = allocator.allocate(
    device, // Implementation of `gpu_descriptor::DescriptorDevice`. Comes from plugins.
    layout, // Descriptor set layout recognized by device's type.
    flags,  // Flags specified when layout was created.
    layout_descriptor_count, // Descriptors count in the layout.
    count, // count of sets to allocated.
);
```


## License

Licensed under either of

* Apache License, Version 2.0, ([license/APACHE](license/APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([license/MIT](license/MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Support me on Patreon

[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dzakarum%26type%3Dpatrons&style=for-the-badge)](https://patreon.com/zakarum)
