use crate::Error;
use core::mem::MaybeUninit;

mod get_errno;
mod sanitizer;

pub(crate) use get_errno::get_errno;

/// Fill a buffer by repeatedly invoking `sys_fill`.
///
/// The `sys_fill` function:
///   - should return -1 and set errno on failure
///   - should return the number of bytes written on success
pub(crate) fn sys_fill_exact(
    mut buf: &mut [MaybeUninit<u8>],
    sys_fill: impl Fn(&mut [MaybeUninit<u8>]) -> libc::ssize_t,
) -> Result<(), Error> {
    while !buf.is_empty() {
        let res = sys_fill(buf);
        match res {
            res if res > 0 => {
                let len = usize::try_from(res).map_err(|_| Error::UNEXPECTED)?;
                let (l, r) = buf.split_at_mut_checked(len).ok_or(Error::UNEXPECTED)?;
                unsafe { sanitizer::unpoison(l) };
                buf = r;
            }
            -1 => {
                let errno = get_errno();
                // We should try again if the call was interrupted.
                if errno != libc::EINTR {
                    return Err(Error::from_errno(errno));
                }
            }
            // Negative return codes not equal to -1 should be impossible.
            // EOF (ret = 0) should be impossible, as the data we are reading
            // should be an infinite stream of random bytes.
            _ => return Err(Error::UNEXPECTED),
        }
    }
    Ok(())
}
