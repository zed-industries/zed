// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "user")]
pub(crate) fn cstr_to_rust(c: *const libc::c_char) -> Option<String> {
    cstr_to_rust_with_size(c, None)
}

#[cfg(any(feature = "disk", feature = "system", feature = "user"))]
#[allow(dead_code)]
pub(crate) fn cstr_to_rust_with_size(
    c: *const libc::c_char,
    size: Option<usize>,
) -> Option<String> {
    if c.is_null() {
        return None;
    }
    let (mut s, max) = match size {
        Some(len) => (Vec::with_capacity(len), len as isize),
        None => (Vec::new(), isize::MAX),
    };
    let mut i = 0;
    unsafe {
        loop {
            let value = *c.offset(i) as u8;
            if value == 0 {
                break;
            }
            s.push(value);
            i += 1;
            if i >= max {
                break;
            }
        }
        String::from_utf8(s).ok()
    }
}
