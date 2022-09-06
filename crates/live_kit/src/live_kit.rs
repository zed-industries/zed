use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};
use futures::{channel::oneshot, Future};
use std::ffi::c_void;

extern "C" {
    fn LKRoomCreate() -> *const c_void;
    fn LKRoomDestroy(room: *const c_void);
    fn LKRoomConnect(
        room: *const c_void,
        url: CFStringRef,
        token: CFStringRef,
        callback: extern "C" fn(*mut c_void) -> (),
        callback_data: *mut c_void,
    );
}

pub struct Room {
    native_room: *const c_void,
}

impl Room {
    pub fn new() -> Self {
        Self {
            native_room: unsafe { LKRoomCreate() },
        }
    }

    pub fn connect(&self, url: &str, token: &str) -> impl Future<Output = ()> {
        let url = CFString::new(url);
        let token = CFString::new(token);

        let (tx, rx) = oneshot::channel();
        extern "C" fn did_connect(tx: *mut c_void) {
            let tx = unsafe { Box::from_raw(tx as *mut oneshot::Sender<()>) };
            let _ = tx.send(());
        }

        unsafe {
            LKRoomConnect(
                self.native_room,
                url.as_concrete_TypeRef(),
                token.as_concrete_TypeRef(),
                did_connect,
                Box::into_raw(Box::new(tx)) as *mut c_void,
            )
        }

        async { rx.await.unwrap() }
    }
}

impl Drop for Room {
    fn drop(&mut self) {
        unsafe { LKRoomDestroy(self.native_room) }
    }
}
