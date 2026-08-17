#![allow(non_snake_case)]
use crate::{CFIndex, CFOptionFlags, CFUserNotification};

impl CFUserNotification {
    #[doc(alias = "CFUserNotificationCheckBoxChecked")]
    pub extern "C" fn check_box_checked(i: CFIndex) -> CFOptionFlags {
        (1usize << (8 + i)) as CFOptionFlags
    }

    #[doc(alias = "CFUserNotificationSecureTextField")]
    pub extern "C" fn secure_text_field(i: CFIndex) -> CFOptionFlags {
        (1usize << (16 + i)) as CFOptionFlags
    }

    #[doc(alias = "CFUserNotificationPopUpSelection")]
    pub fn pop_up_selection(n: CFIndex) -> CFOptionFlags {
        (n << 24) as CFOptionFlags
    }
}
