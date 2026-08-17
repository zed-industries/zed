use std::mem;

use objc::{Message, *};
use objc_foundation::{INSString, INSValue, NSString, NSValue};
#[derive(Debug)]
#[repr(C)]
pub struct SCStreamFrameInfo {
    _priv: [u8; 0],
}

// TODO: Documnent using comment docs matching apple
#[derive(Debug)]
#[repr(i32)]
pub enum SCFrameStatus {
    // A status that indicates the system successfully generated a new frame.
    Complete,
    // A status that indicates the system didn’t generate a new frame because the display didn’t change.
    Idle,
    // A status that indicates the system didn’t generate a new frame because the display is blank.
    Blank,
    // A status that indicates the system didn’t generate a new frame because you suspended updates.
    Suspended,
    // A status that indicates the frame is the first one sent after the stream starts.
    Started,
    // A status that indicates the frame is in a stopped state.
    Stopped,
}

unsafe impl Message for SCStreamFrameInfo {}
impl SCStreamFrameInfo {
    pub fn status(&self) -> SCFrameStatus {
        unsafe {
            let key = NSString::from_str("SCStreamUpdateFrameStatus");
            let raw_status: *mut NSValue<i32> = msg_send!(self, objectForKey: key);
            if raw_status.is_null() {
                return SCFrameStatus::Idle;
            }
            mem::transmute((*raw_status).value())
        }
    }
}
