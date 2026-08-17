//! Header: `sys/file.h`
//!
//! <https://github.com/NetBSD/src/blob/trunk/sys/sys/file.h>

use crate::prelude::*;

pub const DTYPE_VNODE: c_int = 1;
pub const DTYPE_SOCKET: c_int = 2;
pub const DTYPE_PIPE: c_int = 3;
pub const DTYPE_KQUEUE: c_int = 4;
pub const DTYPE_MISC: c_int = 5;
pub const DTYPE_CRYPTO: c_int = 6;
pub const DTYPE_MQUEUE: c_int = 7;
pub const DTYPE_SEM: c_int = 8;
pub const DTYPE_EVENTFD: c_int = 9;
pub const DTYPE_TIMERFD: c_int = 10;
