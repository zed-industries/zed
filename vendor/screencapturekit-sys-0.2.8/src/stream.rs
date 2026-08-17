use std::sync::mpsc::{channel, Receiver};

use block::{ConcreteBlock, RcBlock};
use objc::{
    runtime::{Class, Object},
    Message, *,
};

use crate::{
    stream_error_handler::{UnsafeSCStreamError, UnsafeSCStreamErrorHandler},
    stream_output_handler::{UnsafeSCStreamOutput, UnsafeSCStreamOutputHandler},
};

use super::{
    content_filter::UnsafeContentFilter, stream_configuration::UnsafeStreamConfigurationRef,
};
use dispatch::{Queue, QueueAttribute};
use objc_foundation::{INSObject, INSString, NSObject, NSString};
use objc_id::Id;

pub struct UnsafeSCStream {
    _priv: [u8; 0],
}

unsafe impl Message for UnsafeSCStream {}
impl INSObject for UnsafeSCStream {
    fn class() -> &'static Class {
        Class::get("SCStream")
            .expect("Missing SCStream class, check that the binary is linked with ScreenCaptureKit")
    }
}
type CompletionHandlerBlock = RcBlock<(*mut Object,), ()>;
impl UnsafeSCStream {
    unsafe fn new_completion_handler() -> (CompletionHandlerBlock, Receiver<Result<(), String>>) {
        let (tx, rx) = channel();
        let handler = ConcreteBlock::new(move |error: *mut Object| {
            if !error.is_null() {
                let code: *mut NSString = msg_send![error, localizedDescription];
                let err_msg = (*code).as_str();
                tx.send(Err(err_msg.to_string()))
                    .expect("Cannot send error message");
            }

            tx.send(Ok(())).expect("Cannot send message");
        });
        (handler.copy(), rx)
    }

    pub fn init(
        filter: Id<UnsafeContentFilter>,
        config: Id<UnsafeStreamConfigurationRef>,
        error_handler: impl UnsafeSCStreamError,
    ) -> Id<Self> {
        let instance = UnsafeSCStream::new();

        unsafe {
            let _: () = msg_send![instance, initWithFilter: filter  configuration: config delegate: UnsafeSCStreamErrorHandler::init(error_handler)];
        }
        instance
    }
    pub fn start_capture(&self) -> Result<(), String> {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            let _: () = msg_send!(self, startCaptureWithCompletionHandler: handler);
            rx.recv()
                .expect("Should receive a return from completion handler")
        }
    }
    pub fn stop_capture(&self) -> Result<(), String> {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            let _: () = msg_send!(self, stopCaptureWithCompletionHandler: handler);
            rx.recv()
                .expect("Should receive a return from completion handler")
        }
    }
    pub fn add_stream_output(&self, handle: impl UnsafeSCStreamOutput, output_type: u8) {
        let queue = Queue::create("fish.doom.screencapturekit", QueueAttribute::Concurrent);

        let a = UnsafeSCStreamOutputHandler::init(handle);
        unsafe {
            let _: () = msg_send!(self, addStreamOutput: a type: output_type sampleHandlerQueue: queue error: NSObject::new());
        }
    }
}

impl Drop for UnsafeSCStream {
    fn drop(&mut self) {
        if let Err(err) = self.stop_capture() {
            eprintln!("Cannot stop capture: {:?}", err)
        }
    }
}

#[cfg(test)]
mod stream_test {
    use std::sync::mpsc::{sync_channel, SyncSender};

    use objc_id::Id;

    use super::{UnsafeSCStream, UnsafeSCStreamError};
    use crate::{
        cm_sample_buffer_ref::CMSampleBufferRef,
        content_filter::{UnsafeContentFilter, UnsafeInitParams::Display},
        shareable_content::UnsafeSCShareableContent,
        stream_configuration::UnsafeStreamConfiguration,
        stream_output_handler::UnsafeSCStreamOutput,
    };
    struct ErrorHandler {}
    #[repr(C)]
    struct OutputHandler {
        tx: SyncSender<Id<CMSampleBufferRef>>,
    }
    impl Drop for OutputHandler {
        fn drop(&mut self) {
            println!("DROPPP");
        }
    }
    impl UnsafeSCStreamError for ErrorHandler {
        fn handle_error(&self) {
            eprintln!("ERROR!");
        }
    }
    impl UnsafeSCStreamOutput for OutputHandler {
        fn did_output_sample_buffer(&self, sample: Id<CMSampleBufferRef>, _of_type: u8) {
            self.tx.send(sample).unwrap();
        }
    }
    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sc_stream() {
        let display = UnsafeSCShareableContent::get()
            .unwrap()
            .displays()
            .pop()
            .expect("could not get display");

        let filter = UnsafeContentFilter::init(Display(display));
        let config = UnsafeStreamConfiguration {
            width: 100,
            height: 100,
            ..Default::default()
        };
        let (tx, rx) = sync_channel(1);
        let stream = UnsafeSCStream::init(filter, config.into(), ErrorHandler {});
        let a = OutputHandler { tx };

        println!("ADDING OUTPUT");
        stream.add_stream_output(a, 0);
        println!("start capture");
        stream.start_capture().expect("start");
        println!("{:?}", rx.recv().unwrap());
        stream.stop_capture().expect("stop");
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_sc_stream_error_handling() {
        let display = UnsafeSCShareableContent::get()
            .unwrap()
            .displays()
            .pop()
            .expect("could not get display");

        let filter = UnsafeContentFilter::init(Display(display));
        let config = UnsafeStreamConfiguration {
            width: 100,
            height: 100,
            ..Default::default()
        };
        let stream = UnsafeSCStream::init(filter, config.into(), ErrorHandler {});

        println!("start capture");
        assert!(stream.start_capture().is_ok());
        assert!(stream.start_capture().is_err()); // already started error
        assert!(stream.stop_capture().is_ok());
        assert!(stream.stop_capture().is_err()); // already stopped error
    }
}
