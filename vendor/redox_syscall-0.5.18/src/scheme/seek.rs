use crate::{error::*, flag::*};
use core::{cmp, convert::TryFrom};

/// Helper for seek calls
/// In most cases it's easier to use a usize to track the offset and buffer size internally,
/// but the seek interface uses isize.  This wrapper ensures EOVERFLOW errors are returned
/// as appropriate if the value in the usize can't fit in the isize.
pub fn calc_seek_offset_usize(
    cur_offset: usize,
    pos: isize,
    whence: usize,
    buf_len: usize,
) -> Result<isize> {
    let cur_offset = isize::try_from(cur_offset).or_else(|_| Err(Error::new(EOVERFLOW)))?;
    let buf_len = isize::try_from(buf_len).or_else(|_| Err(Error::new(EOVERFLOW)))?;
    calc_seek_offset_isize(cur_offset, pos, whence, buf_len)
}

/// Helper for seek calls
/// Result is guaranteed to be positive.
/// EOVERFLOW returned if the arguments would cause an overflow.
/// EINVAL returned if the new offset is out of bounds.
pub fn calc_seek_offset_isize(
    cur_offset: isize,
    pos: isize,
    whence: usize,
    buf_len: isize,
) -> Result<isize> {
    let new_offset = match whence {
        SEEK_CUR => pos.checked_add(cur_offset),
        SEEK_END => pos.checked_add(buf_len),
        SEEK_SET => Some(pos),
        _ => None,
    };

    match new_offset {
        Some(new_offset) if new_offset < 0 => Err(Error::new(EINVAL)),
        Some(new_offset) => Ok(cmp::min(new_offset, buf_len)),
        None => Err(Error::new(EOVERFLOW)),
    }
}
