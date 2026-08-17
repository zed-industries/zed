// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::{TimeType, TimeZone};

impl TimeZone {
    #[doc(alias = "g_time_zone_adjust_time")]
    pub fn adjust_time(&self, type_: TimeType, mut time: i64) -> (i32, i64) {
        unsafe {
            let res = crate::ffi::g_time_zone_adjust_time(
                self.to_glib_none().0,
                type_.into_glib(),
                &mut time,
            );
            (res, time)
        }
    }
}
