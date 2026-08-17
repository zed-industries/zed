#![deny(unsafe_op_in_unsafe_fn, missing_docs)]
#![cfg_attr(
    not(feature = "enable"),
    allow(unused_variables, unused_imports, unused_mut, dead_code)
)]
// TODO https://github.com/rust-lang/rust-clippy/issues/12017
#![allow(clippy::let_unit_value)]
//! This crate is a set of safe bindings to the client library of the [Tracy profiler].
//!
//! If you have already instrumented your application with `tracing`, consider the `tracing-tracy`
//! crate.
//!
//! [Tracy profiler]: https://github.com/wolfpld/tracy
//!
//! # Important note
//!
//! Depending on the configuration Tracy may broadcast discovery packets to the local network and
//! expose the data it collects in the background to that same network. Traces collected by Tracy
//! may include source and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracy-client` crate conditionally, via
//! the `enable` feature flag provided by this crate.
//!
//! # Features
//!
//! The following crate features are provided to customize the functionality of the Tracy client:
//!
#![doc = include_str!("../FEATURES.mkd")]

pub use crate::frame::{frame_image, frame_mark, Frame, FrameName};
pub use crate::gpu::{
    GpuContext, GpuContextCreationError, GpuContextType, GpuSpan, GpuSpanCreationError,
};
pub use crate::plot::{PlotConfiguration, PlotFormat, PlotLineStyle, PlotName};
pub use crate::span::{Span, SpanLocation};
use std::alloc;
use std::ffi::CString;
pub use sys;

mod frame;
mod gpu;
mod plot;
mod span;
mod state;

#[cfg(feature = "demangle")]
pub mod demangle;

/// /!\ /!\ Please don't rely on anything in this module T_T /!\ /!\
#[doc(hidden)]
pub mod internal {
    pub use crate::{span::SpanLocation, sys};
    pub use once_cell::sync::Lazy;
    pub use std::any::type_name;
    use std::ffi::CString;
    pub use std::ptr::null;

    #[cfg(feature = "demangle")]
    pub mod demangle {
        pub use crate::demangle::{default, internal::implementation};
    }

    #[inline(always)]
    #[must_use]
    pub fn make_span_location(
        type_name: &'static str,
        span_name: *const u8,
        file: *const u8,
        line: u32,
    ) -> SpanLocation {
        #[cfg(feature = "enable")]
        {
            let function_name = CString::new(&type_name[..type_name.len() - 3]).unwrap();
            SpanLocation {
                data: sys::___tracy_source_location_data {
                    name: span_name.cast(),
                    function: function_name.as_ptr(),
                    file: file.cast(),
                    line,
                    color: 0,
                },
                _function_name: function_name,
            }
        }
        #[cfg(not(feature = "enable"))]
        crate::SpanLocation { _internal: () }
    }

    #[inline(always)]
    #[must_use]
    pub const unsafe fn create_frame_name(name: &'static str) -> crate::frame::FrameName {
        crate::frame::FrameName(name)
    }

    #[inline(always)]
    #[must_use]
    pub const unsafe fn create_plot(name: &'static str) -> crate::plot::PlotName {
        crate::plot::PlotName(name)
    }

    /// Safety: `name` must be null-terminated, and a `Client` must be enabled
    #[inline(always)]
    pub unsafe fn set_thread_name(name: *const u8) {
        #[cfg(feature = "enable")]
        unsafe {
            let () = sys::___tracy_set_thread_name(name.cast());
        }
    }
}

/// A type representing an enabled Tracy client.
///
/// Obtaining a `Client` is required in order to instrument the application.
///
/// Multiple copies of a Client may be live at once. As long as at least one `Client` value lives,
/// the `Tracy` client is enabled globally. In addition to collecting information through the
/// instrumentation inserted by you, the Tracy client may automatically collect information about
/// execution of the program while it is enabled. All this information may be stored in memory
/// until a profiler application connects to the client to read the data.
///
/// Depending on the build configuration, the client may collect and make available machine
/// and source code of the application as well as other potentially sensitive information.
///
/// When all of the `Client` values are dropped, the underlying Tracy client will be shut down as
/// well. Shutting down the `Client` will discard any information gathered up to that point that
/// still hasn't been delivered to the profiler application.
pub struct Client(());

/// Instrumentation methods for outputting events occurring at a specific instant.
///
/// Data provided by this instrumentation can largely be considered to be equivalent to logs.
impl Client {
    /// Output a message.
    ///
    /// Specifying a non-zero `callstack_depth` will enable collection of callstack for this
    /// message. The number provided will limit the number of call frames collected. Note that
    /// enabling callstack collection introduces a non-trivial amount of overhead to this call.
    pub fn message(&self, message: &str, callstack_depth: u16) {
        #[cfg(feature = "enable")]
        unsafe {
            let stack_depth = adjust_stack_depth(callstack_depth).into();
            let () =
                sys::___tracy_emit_message(message.as_ptr().cast(), message.len(), stack_depth);
        }
    }

    /// Output a message with an associated color.
    ///
    /// Specifying a non-zero `callstack_depth` will enable collection of callstack for this
    /// message. The number provided will limit the number of call frames collected. Note that
    /// enabling callstack collection introduces a non-trivial amount of overhead to this call.
    ///
    /// The colour shall be provided as RGBA, where the least significant 8 bits represent the alpha
    /// component and most significant 8 bits represent the red component.
    pub fn color_message(&self, message: &str, rgba: u32, callstack_depth: u16) {
        #[cfg(feature = "enable")]
        unsafe {
            let depth = adjust_stack_depth(callstack_depth).into();
            let () = sys::___tracy_emit_messageC(
                message.as_ptr().cast(),
                message.len(),
                rgba >> 8,
                depth,
            );
        }
    }
}

impl Client {
    /// Set the current thread name to the provided value.
    ///
    /// # Panics
    ///
    /// This function will panic if the name contains interior null characters.
    pub fn set_thread_name(&self, name: &str) {
        #[cfg(feature = "enable")]
        unsafe {
            let name = CString::new(name).unwrap();
            // SAFE: `name` is a valid null-terminated string.
            internal::set_thread_name(name.as_ptr().cast());
        }
    }
}

/// Convenience macro for [`Client::set_thread_name`] on the current client.
///
/// Note that any interior null characters terminate the name early. This is not checked for.
///
/// # Panics
///
/// - If a `Client` isn't currently running.
#[macro_export]
macro_rules! set_thread_name {
    ($name: literal) => {{
        $crate::Client::running().expect("set_thread_name! without a running Client");
        unsafe {
            // SAFE: `name` is a valid null-terminated string.
            $crate::internal::set_thread_name(concat!($name, "\0").as_ptr().cast())
        }
    }};
}

/// A profiling wrapper around another allocator.
///
/// See documentation for [`std::alloc`] for more information about global allocators.
///
/// Note that this wrapper will start up the client on the first allocation, if not enabled
/// already.
///
/// # Examples
///
/// In your executable, add:
///
/// ```rust
/// # use tracy_client::*;
/// #[global_allocator]
/// static GLOBAL: ProfiledAllocator<std::alloc::System> =
///     ProfiledAllocator::new(std::alloc::System, 100);
/// ```
pub struct ProfiledAllocator<T>(T, u16);

impl<T> ProfiledAllocator<T> {
    /// Construct a new `ProfiledAllocator`.
    ///
    /// Specifying a non-zero `callstack_depth` will enable collection of callstack for this
    /// message. The number provided will limit the number of call frames collected. Note that
    /// enabling callstack collection introduces a non-trivial amount of overhead to each
    /// allocation and deallocation.
    pub const fn new(inner_allocator: T, callstack_depth: u16) -> Self {
        Self(inner_allocator, adjust_stack_depth(callstack_depth))
    }

    fn emit_alloc(&self, ptr: *mut u8, size: usize) {
        #[cfg(feature = "enable")]
        unsafe {
            Client::start();
            if self.1 == 0 {
                let () = sys::___tracy_emit_memory_alloc(ptr.cast(), size, 1);
            } else {
                let () =
                    sys::___tracy_emit_memory_alloc_callstack(ptr.cast(), size, self.1.into(), 1);
            }
        }
    }

    fn emit_free(&self, ptr: *mut u8) {
        #[cfg(feature = "enable")]
        unsafe {
            if self.1 == 0 {
                let () = sys::___tracy_emit_memory_free(ptr.cast(), 1);
            } else {
                let () = sys::___tracy_emit_memory_free_callstack(ptr.cast(), self.1.into(), 1);
            }
        }
    }
}

unsafe impl<T: alloc::GlobalAlloc> alloc::GlobalAlloc for ProfiledAllocator<T> {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        let alloc = unsafe {
            // SAFE: all invariants satisfied by the caller.
            self.0.alloc(layout)
        };
        self.emit_alloc(alloc, layout.size());
        alloc
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        self.emit_free(ptr);
        unsafe {
            // SAFE: all invariants satisfied by the caller.
            self.0.dealloc(ptr, layout);
        }
    }

    unsafe fn alloc_zeroed(&self, layout: alloc::Layout) -> *mut u8 {
        let alloc = unsafe {
            // SAFE: all invariants satisfied by the caller.
            self.0.alloc_zeroed(layout)
        };
        self.emit_alloc(alloc, layout.size());
        alloc
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: alloc::Layout, new_size: usize) -> *mut u8 {
        self.emit_free(ptr);
        let alloc = unsafe {
            // SAFE: all invariants satisfied by the caller.
            self.0.realloc(ptr, layout, new_size)
        };
        self.emit_alloc(alloc, new_size);
        alloc
    }
}

/// Clamp the stack depth to the maximum supported by Tracy.
pub(crate) const fn adjust_stack_depth(depth: u16) -> u16 {
    #[cfg(windows)]
    {
        62 ^ ((depth ^ 62) & 0u16.wrapping_sub((depth < 62) as _))
    }
    #[cfg(not(windows))]
    {
        depth
    }
}
