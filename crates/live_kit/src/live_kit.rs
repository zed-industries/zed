use std::ffi::c_void;

extern "C" {
    fn LKRoomCreate() -> *const c_void;
    fn LKRoomDestroy(ptr: *const c_void);
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
}

impl Drop for Room {
    fn drop(&mut self) {
        unsafe { LKRoomDestroy(self.native_room) }
    }
}
