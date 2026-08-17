use std::io;

#[allow(dead_code)]
pub(crate) fn cvt_i32(t: i32) -> io::Result<i32> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

#[allow(dead_code)]
pub(crate) fn cvt_i64(t: i64) -> io::Result<i64> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
