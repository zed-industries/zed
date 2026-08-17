use std::any::{Any, TypeId};
use std::mem;

use runtime::Imp;

extern {
    fn objc_msgSend();
    fn objc_msgSend_fpret();
    fn objc_msgSend_stret();

    fn objc_msgSendSuper();
    fn objc_msgSendSuper_stret();
}

pub fn msg_send_fn<R: Any>() -> Imp {
    // Structures 1 or 2 bytes in size are placed in EAX.
    // Structures 4 or 8 bytes in size are placed in: EAX and EDX.
    // Structures of other sizes are placed at the address supplied by the caller.
    // <https://developer.apple.com/library/mac/documentation/DeveloperTools/Conceptual/LowLevelABI/130-IA-32_Function_Calling_Conventions/IA32.html>

    let type_id = TypeId::of::<R>();
    let size = mem::size_of::<R>();
    if type_id == TypeId::of::<f32>() ||
            type_id == TypeId::of::<f64>() {
        objc_msgSend_fpret
    } else if size == 0 || size == 1 || size == 2 || size == 4 || size == 8 {
        objc_msgSend
    } else {
        objc_msgSend_stret
    }
}

pub fn msg_send_super_fn<R: Any>() -> Imp {
    let size = mem::size_of::<R>();
    if size == 0 || size == 1 || size == 2 || size == 4 || size == 8 {
        objc_msgSendSuper
    } else {
        objc_msgSendSuper_stret
    }
}
