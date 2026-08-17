use std::{
    collections::HashMap,
    sync::{Once, RwLock},
};

use crate::cm_sample_buffer_ref::CMSampleBufferRef;
use objc::{
    class,
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    *,
};
use objc_foundation::INSObject;
use objc_id::Id;
use once_cell::sync::Lazy;

static OUTPUT_HANDLERS: Lazy<RwLock<HashMap<usize, Box<dyn UnsafeSCStreamOutput + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[repr(C)]
pub(crate) struct UnsafeSCStreamOutputHandler {
    _priv: [u8; 0],
}

unsafe impl Message for UnsafeSCStreamOutputHandler {}

pub trait UnsafeSCStreamOutput: Send + Sync + 'static {
    fn did_output_sample_buffer(&self, sample_buffer_ref: Id<CMSampleBufferRef>, of_type: u8);
}

impl INSObject for UnsafeSCStreamOutputHandler {
    fn class() -> &'static Class {
        static REGISTER_UNSAFE_SC_OUTPUT_HANDLER: Once = Once::new();
        REGISTER_UNSAFE_SC_OUTPUT_HANDLER.call_once(|| {
            let mut decl = ClassDecl::new("SCStreamOutputHandler", class!(NSObject)).unwrap();
            decl.add_ivar::<usize>("_output_handler");

            extern "C" fn stream_output(
                this: &mut Object,
                _cmd: Sel,
                _stream: *mut Object,
                sample_ref: *mut Object,
                of_type: u8,
            ) {
                unsafe {
                    if sample_ref.is_null() {
                        return;
                    }
                    let sample: Id<CMSampleBufferRef> = Id::from_ptr(sample_ref.cast());
                    let handler_trait_ptr_address = this.get_ivar::<usize>("_output_handler");
                    let lookup = OUTPUT_HANDLERS.read().unwrap();
                    let output_handler_trait = lookup.get(handler_trait_ptr_address).unwrap();
                    output_handler_trait.did_output_sample_buffer(sample, of_type)
                };
            }
            unsafe {
                let stream_output_method: for<'a> extern "C" fn(
                    &mut Object,
                    Sel,
                    *mut Object,
                    *mut Object,
                    u8,
                ) = stream_output;

                decl.add_method(
                    sel!(stream:didOutputSampleBuffer:ofType:),
                    stream_output_method,
                );
            }

            decl.register();
        });
        class!(SCStreamOutputHandler)
    }
}

impl UnsafeSCStreamOutputHandler {
    fn store_output_handler(&mut self, output_handler: impl UnsafeSCStreamOutput) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let hash = self.hash_code();
            OUTPUT_HANDLERS
                .write()
                .unwrap()
                .insert(hash, Box::new(output_handler));
            obj.set_ivar("_output_handler", hash);
        }
    }
    pub fn init(output_handler: impl UnsafeSCStreamOutput) -> Id<Self> {
        let mut handle = Self::new();
        handle.store_output_handler(output_handler);
        handle
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[repr(C)]
    struct TestHandler {}
    impl UnsafeSCStreamOutput for TestHandler {
        fn did_output_sample_buffer(&self, sample: Id<CMSampleBufferRef>, of_type: u8) {
            println!("GOT SAMPLE! {:?} {}", sample, of_type);
        }
    }

    #[test]
    fn test_sc_stream_output_handler() {
        let handle = TestHandler {};
        let handle = UnsafeSCStreamOutputHandler::init(handle);
        let _: () = unsafe {
            msg_send![handle, stream: ptr::null_mut::<Object>() didOutputSampleBuffer: ptr::null_mut::<Object>() ofType: 0]
        };
        let _: () = unsafe {
            msg_send![handle, stream: ptr::null_mut::<Object>() didOutputSampleBuffer: ptr::null_mut::<Object>() ofType: 1]
        };
    }
}
