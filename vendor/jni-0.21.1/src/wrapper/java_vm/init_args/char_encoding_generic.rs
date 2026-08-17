use super::JvmError;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};

/// Converts `s: Cow<[u8]>` into a `Cow<CStr>`, adding a null byte if necessary.
///
/// `original`, if present, is the original string, which will be moved into a [`JvmError]`
/// in the event of failure. If `original` is absent, then `s` *is* the original
/// string (i.e. is encoded in UTF-8), and is to be moved into the `JvmError` upon failure.
///
/// # Errors
///
/// This will fail if `s` contains any null bytes other than a single null byte at the end.
///
/// # Safety
///
/// If `original` is `None`, then `s` must contain valid UTF-8.
pub(super) unsafe fn bytes_to_cstr<'a>(
    mut s: Cow<'a, [u8]>,
    original: Option<Cow<'_, str>>,
) -> Result<Cow<'a, CStr>, JvmError> {
    // Check if it has a null byte at the end already. If not, add one.
    let mut null_byte_added = false;

    if s.last() != Some(&0) {
        s.to_mut().push(0);
        null_byte_added = true;
    }

    // This function is called if conversion fails because the string has a null byte
    // in the middle.
    let convert_error = move |s: Cow<'a, [u8]>| -> JvmError {
        // We need to get back to a `String` in order to insert it into the error. How
        // to do that depends on whether we were given a separate original or not.
        let s: String = {
            if let Some(original) = original {
                // Yes, there is a separate original. Use that.
                original.into_owned()
            } else {
                // No, `s` *is* the original. Strip off the null byte if we
                // added one, then assume the rest is valid UTF-8.
                let mut s: Vec<u8> = s.into_owned();

                if null_byte_added {
                    let _removed_null_byte: Option<u8> = s.pop();
                    debug_assert_eq!(_removed_null_byte, Some(0));
                }

                // Safety: The caller of this function asserts that this is valid UTF-8. We
                // have not changed it other than adding a null byte at the end.
                unsafe { String::from_utf8_unchecked(s) }
            }
        };

        JvmError::NullOptString(s)
    };

    // Now, try to convert. Exactly how to do this, and exactly how to handle errors, depends
    // on whether it's borrowed or owned.
    let s: Cow<'a, CStr> = match s {
        Cow::Owned(s) => Cow::Owned({
            CString::from_vec_with_nul(s)
                .map_err(|error| convert_error(Cow::Owned(error.into_bytes())))?
        }),

        Cow::Borrowed(s) => Cow::Borrowed({
            CStr::from_bytes_with_nul(s).map_err(|_error| convert_error(Cow::Borrowed(s)))?
        }),
    };

    // Done.
    Ok(s)
}

/// Converts `s: Cow<str>` into a `Cow<CStr>`, still in UTF-8 encoding, adding a null byte if
/// necessary.
pub(super) fn utf8_to_cstr<'a>(s: Cow<'a, str>) -> Result<Cow<'a, CStr>, JvmError> {
    let s: Cow<'a, [u8]> = match s {
        Cow::Owned(s) => Cow::Owned(s.into_bytes()),
        Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
    };

    // Safety: `s` was just converted from type `str`, so it's already known to contain valid
    // UTF-8.
    unsafe { bytes_to_cstr(s, None) }
}

#[test]
fn test() {
    use assert_matches::assert_matches;

    {
        let result = utf8_to_cstr("Hello, world 😎".into()).unwrap();
        assert_eq!(
            result.to_bytes_with_nul(),
            b"Hello, world \xf0\x9f\x98\x8e\0"
        );
        assert_matches!(result, Cow::Owned(_));
    }

    {
        let result = utf8_to_cstr("Hello, world 😎\0".into()).unwrap();
        assert_eq!(
            result.to_bytes_with_nul(),
            b"Hello, world \xf0\x9f\x98\x8e\0"
        );
        assert_matches!(result, Cow::Borrowed(_));
    }

    {
        let result = utf8_to_cstr("Hello,\0world".into()).unwrap_err();
        let error_string = assert_matches!(result, JvmError::NullOptString(string) => string);
        assert_eq!(error_string, "Hello,\0world");
    }
}
