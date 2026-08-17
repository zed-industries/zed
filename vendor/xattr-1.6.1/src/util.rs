use std::io;
use std::mem::MaybeUninit;

pub fn extract_noattr(result: io::Result<Vec<u8>>) -> io::Result<Option<Vec<u8>>> {
    result.map(Some).or_else(|e| {
        if e.raw_os_error() == Some(crate::sys::ENOATTR) {
            Ok(None)
        } else {
            Err(e)
        }
    })
}

/// Calls `get_value` to with a buffer and `get_size` to estimate the size of the buffer if/when
/// `get_value` returns ERANGE.
#[allow(dead_code)]
pub fn allocate_loop<F, S>(mut get_value: F, mut get_size: S) -> io::Result<Vec<u8>>
where
    F: for<'a> FnMut(&'a mut [MaybeUninit<u8>]) -> io::Result<&'a mut [u8]>,
    S: FnMut() -> io::Result<usize>,
{
    // Start by assuming the return value is <= 4KiB. If it is, we can do this in one syscall.
    const INITIAL_BUFFER_SIZE: usize = 4096;
    match get_value(&mut [MaybeUninit::<u8>::uninit(); INITIAL_BUFFER_SIZE]) {
        Ok(val) => return Ok(val.to_vec()),
        Err(e) if e.raw_os_error() != Some(crate::sys::ERANGE) => return Err(e),
        _ => {}
    }

    // If that fails, we ask for the size and try again with a buffer of the correct size.
    let mut vec: Vec<u8> = Vec::new();
    loop {
        vec.reserve_exact(get_size()?);
        match get_value(vec.spare_capacity_mut()) {
            Ok(initialized) => {
                unsafe {
                    let len = initialized.len();
                    assert_eq!(
                        initialized.as_ptr(),
                        vec.as_ptr(),
                        "expected the same buffer"
                    );
                    vec.set_len(len);
                }
                // Only shrink to fit if we've over-allocated by MORE than one byte. Unfortunately,
                // on FreeBSD, we have to over-allocate by one byte to determine if we've read all
                // the attributes.
                if vec.capacity() > vec.len() + 1 {
                    vec.shrink_to_fit();
                }
                return Ok(vec);
            }
            Err(e) if e.raw_os_error() != Some(crate::sys::ERANGE) => return Err(e),
            _ => {} // try again
        }
    }
}
