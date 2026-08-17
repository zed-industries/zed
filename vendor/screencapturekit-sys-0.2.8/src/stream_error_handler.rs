use std::sync::Once;

use objc::{
    class,
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    Message, *,
};
use objc_foundation::INSObject;
use objc_id::Id;

pub trait UnsafeSCStreamError: Send + Sync + 'static {
    fn handle_error(&self);
}

#[repr(C)]
pub(crate) struct UnsafeSCStreamErrorHandler {}

unsafe impl Message for UnsafeSCStreamErrorHandler {}

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;
static ERROR_HANDLERS: Lazy<RwLock<HashMap<usize, Box<dyn UnsafeSCStreamError + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

impl INSObject for UnsafeSCStreamErrorHandler {
    fn class() -> &'static Class {
        static REGISTER_UNSAFE_SC_ERROR_HANDLER: Once = Once::new();
        REGISTER_UNSAFE_SC_ERROR_HANDLER.call_once(|| {
            let mut decl = ClassDecl::new("SCStreamErrorHandler", class!(NSObject)).unwrap();
            decl.add_ivar::<usize>("_hash");

            extern "C" fn stream_error(
                this: &mut Object,
                _cmd: Sel,
                _stream: *mut Object,
                _error: *mut Object,
            ) {
                unsafe {
                    let hash = this.get_ivar::<usize>("_hash");
                    let lookup = ERROR_HANDLERS.read().unwrap();
                    let error_handler = lookup.get(hash).unwrap();
                    error_handler.handle_error();
                };
            }
            unsafe {
                let stream_error_method: extern "C" fn(&mut Object, Sel, *mut Object, *mut Object) =
                    stream_error;

                decl.add_method(sel!(stream:didStopWithError:), stream_error_method);
            }

            decl.register();
        });
        class!(SCStreamErrorHandler)
    }
}

impl UnsafeSCStreamErrorHandler {
    fn store_error_handler(&mut self, error_handler: impl UnsafeSCStreamError) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let hash = self.hash_code();
            ERROR_HANDLERS
                .write()
                .unwrap()
                .insert(hash, Box::new(error_handler));
            obj.set_ivar("_hash", hash);
        }
    }
    // Error handlers passed into here will currently live forever inside the statically
    // allocated map.
    // TODO: Remove the handler from the HashMap whenever the associated stream is dropped.
    pub fn init(error_handler: impl UnsafeSCStreamError) -> Id<Self> {
        let mut handle = Self::new();
        handle.store_error_handler(error_handler);
        handle
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;
    use std::sync::mpsc::{sync_channel, SyncSender};

    use super::*;

    struct TestHandler {
        error_tx: SyncSender<()>,
    }
    impl UnsafeSCStreamError for TestHandler {
        fn handle_error(&self) {
            eprintln!("ERROR!");
            if let Err(e) = self.error_tx.send(()) {
                panic!("can't send error message back on the channel: {:?}", e);
            }
        }
    }

    #[test]
    fn test_sc_stream_error_handler() {
        let (error_tx, error_rx) = sync_channel(1);
        let handle = UnsafeSCStreamErrorHandler::init(TestHandler { error_tx });
        unsafe {
            msg_send![handle, stream: ptr::null_mut::<Object>() didStopWithError: ptr::null_mut::<Object>()]
        }
        if let Err(e) = error_rx.recv_timeout(std::time::Duration::from_millis(250)) {
            panic!("failed to hear back from the error channel: {:?}", e);
        }
    }
}
