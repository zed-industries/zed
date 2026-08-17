//! Low-level bindings to the [RenderDoc](https://renderdoc.org/) in-application API.
//!
//! RenderDoc is a free and open source debugger for real-time graphics providing quick and easy
//! frame captures and detailed introspection of any application using [Vulkan], [Direct3D 11],
//! [Direct3D 12], [OpenGL], and [OpenGL ES].
//!
//! [Vulkan]: https://www.vulkan.org/
//! [Direct3D 11]: https://learn.microsoft.com/en-us/windows/win32/direct3d11/atoc-dx-graphics-direct3d-11
//! [Direct3D 12]: https://learn.microsoft.com/en-us/windows/win32/direct3d12/direct3d-12-graphics
//! [OpenGL]: https://www.khronos.org/opengl/
//! [OpenGL ES]: https://www.khronos.org/opengles/
//!
//! These bindings are automatically generated from [`renderdoc_app.h`] with [`bindgen`]. This
//! crate does not provide nor link to the `renderdoc.dll` or `librenderdoc.so` libraries on its
//! own; it only contains FFI symbols. Refer to the official [In-Application API][api]
//! documentation for correct usage.
//!
//! [`renderdoc_app.h`]: https://github.com/baldurk/renderdoc/blob/v1.x/renderdoc/api/app/renderdoc_app.h
//! [`bindgen`]: https://github.com/rust-lang/rust-bindgen
//! [api]: https://renderdoc.org/docs/in_application_api.html
//!
//! For a safe wrapper, see the [`renderdoc`](https://docs.rs/renderdoc) crate.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!("./bindings.rs");
