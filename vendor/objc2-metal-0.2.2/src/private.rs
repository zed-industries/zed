//! Private functionality.
//!
//! The credit for finding these belong to the [metal-rs] project.
//!
//! [metal-rs]: https://github.com/gfx-rs/metal-rs
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]
use std::ffi::c_void;

use crate::*;

use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{extern_methods, msg_send_id, Message};

pub unsafe trait MTLDevicePrivate: Message {
    unsafe fn vendorName(&self) -> Retained<objc2_foundation::NSString> {
        unsafe { msg_send_id![self, vendorName] }
    }

    unsafe fn familyName(&self) -> Retained<objc2_foundation::NSString> {
        unsafe { msg_send_id![self, familyName] }
    }
}

#[cfg(feature = "MTLDevice")]
unsafe impl<P: MTLDevice + Message> MTLDevicePrivate for P {}

extern_methods!(
    #[cfg(feature = "MTLRenderPipeline")]
    unsafe impl MTLRenderPipelineReflection {
        #[cfg(feature = "MTLDevice")]
        #[method_id(initWithVertexData:fragmentData:serializedVertexDescriptor:device:options:flags:)]
        pub unsafe fn initWithVertexData(
            this: Allocated<Self>,
            vertex_data: *mut c_void,
            fragment_data: *mut c_void,
            vertex_desc: *mut c_void,
            device: &ProtocolObject<dyn MTLDevice>,
            options: u64,
            flags: u64,
        ) -> Option<Retained<Self>>;

        #[method_id(newSerializedVertexDataWithFlags:error:_)]
        pub unsafe fn newSerializedVertexDataWithFlags_error(
            &self,
            flags: u64,
        ) -> Result<Retained<AnyObject>, Retained<objc2_foundation::NSError>>;

        #[method(serializeFragmentData)]
        pub unsafe fn serializeFragmentData(&self) -> *mut c_void;
    }
);

extern_methods!(
    #[cfg(feature = "MTLSamplerDescriptor")]
    unsafe impl MTLSamplerDescriptor {
        #[method(setLodBias:)]
        pub unsafe fn setLodBias(&self, bias: f32);
    }
);

extern_methods!(
    #[cfg(feature = "MTLVertexDescriptor")]
    unsafe impl MTLVertexDescriptor {
        #[method_id(newSerializedDescriptor)]
        pub unsafe fn newSerializedDescriptor(&self) -> Option<Retained<AnyObject>>;
    }
);
