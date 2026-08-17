# renderdoc-sys

[![Build Status][build-badge]][build-url]
[![Crates.io][crate-badge]][crate-url]
[![Documentation][docs-badge]][docs-url]

[build-badge]: https://github.com/ebkalderon/renderdoc-rs/actions/workflows/ci.yml/badge.svg
[build-url]: https://github.com/ebkalderon/renderdoc-rs/actions
[crate-badge]: https://img.shields.io/crates/v/renderdoc-sys.svg
[crate-url]: https://crates.io/crates/renderdoc-sys
[docs-badge]: https://docs.rs/renderdoc-sys/badge.svg
[docs-url]: https://docs.rs/renderdoc-sys

Low-level bindings to the [RenderDoc] in-application API.

[RenderDoc]: https://renderdoc.org/

RenderDoc is a free and open source debugger for real-time graphics that allows
quick and easy frame captures and detailed introspection of any application
using [Vulkan], [Direct3D 11], [Direct3D 12], [OpenGL], and [OpenGL ES].

[Vulkan]: https://www.vulkan.org/
[Direct3D 11]: https://learn.microsoft.com/en-us/windows/win32/direct3d11/atoc-dx-graphics-direct3d-11
[Direct3D 12]: https://learn.microsoft.com/en-us/windows/win32/direct3d12/direct3d-12-graphics
[OpenGL]: https://www.khronos.org/opengl/
[OpenGL ES]: https://www.khronos.org/opengles/

These bindings are automatically generated from [`renderdoc_app.h`] with
[`bindgen`]. This crate does not provide nor link to `renderdoc.dll` nor
`librenderdoc.so` by itself; it only contains the FFI symbols. Refer to the
upstream [In-Application API][api] documentation for correct usage details.

[`renderdoc_app.h`]: https://github.com/baldurk/renderdoc/blob/v1.x/renderdoc/api/app/renderdoc_app.h
[bindgen]: https://github.com/rust-lang/rust-bindgen
[api]: https://renderdoc.org/docs/in_application_api.html

For a safe wrapper, see the [`renderdoc`](https://docs.rs/renderdoc) crate.

## License

`renderdoc-sys` is free and open source software distributed under the terms of
either the [MIT](LICENSE-MIT) or the [Apache 2.0](LICENSE-APACHE) license, at
your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
