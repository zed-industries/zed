use super::{char_encoding_generic::*, JvmError};
use std::{
    borrow::Cow,
    convert::TryInto,
    ffi::{c_int, c_uint, CStr},
    io,
    mem::MaybeUninit,
    ptr,
};
use windows_sys::Win32::Globalization as winnls;

// The integer type used by `WideCharToMultiByte` for string lengths.
type WSize = c_int;

// The type of Windows codepage numbers.
type WCodepage = c_uint;

// The maximum length, in UTF-8 bytes, of strings that will be accepted for transcoding.
//
// The purpose of this limit is to prevent overflow. `WideCharToMultiByte` behaves rather badly
// (see https://github.com/jni-rs/jni-rs/pull/414 for discussion) if the string is long enough to
// overflow its counters.
//
// Although it is possible to transcode a string of any length by splitting it into smaller
// substrings, the code complexity needed to do so isn't worthwhile just for transcoding JVM
// options. Also, `test_overflow` would take a very long time to run, which was deemed unacceptable
// (see https://github.com/jni-rs/jni-rs/pull/414#issuecomment-1419130483). We set this arbitrary
// limit instead.
const MAX_INPUT_LEN: usize = 1048576;

/// Converts `s` into a `Cow<CStr>` encoded in the specified Windows code page.
pub(super) fn str_to_cstr_win32<'a>(
    s: Cow<'a, str>,
    needed_codepage: WCodepage,
) -> Result<Cow<'static, CStr>, JvmError> {
    // First, check if the input string (UTF-8) is too long to transcode. Bail early if so.
    if s.len() > MAX_INPUT_LEN {
        return Err(JvmError::OptStringTooLong {
            opt_string: s.into_owned(),
        });
    }

    // This function will generate an error if `WideCharToMultiByte` fails.
    fn convert_error(s: Cow<str>) -> JvmError {
        JvmError::OptStringTranscodeFailure {
            opt_string: s.into_owned(),
            error: io::Error::last_os_error(),
        }
    }

    // Convert the string to UTF-16 first.
    let s_utf16: Vec<u16> = s.encode_utf16().collect();

    // Determine how long the string is, in UTF-16 units, in the integer type that Win32 expects.
    // Overflow should be impossible; panic if it happens.
    let s_utf16_len: WSize = s_utf16
        .len()
        .try_into()
        .expect("UTF-16 form of input string is too long");

    // Decide which flags we're going to use.
    let conversion_flags = match needed_codepage {
        // No flags may be given for the following code pages.
        // https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-widechartomultibyte
        42
        | 50220
        | 50221
        | 50222
        | 50225
        | 50227
        | 50229
        | 54936
        | 57002..=57011
        | 65000
        | 65001 => 0,

        _ => winnls::WC_COMPOSITECHECK | winnls::WC_NO_BEST_FIT_CHARS,
    };

    // Find out how much buffer space will be needed for the output and whether the string is
    // fully representable.
    let mut is_non_representable: Option<MaybeUninit<_>> = match needed_codepage {
        // All characters are representable in UTF-7 and UTF-8, and moreover
        // `WideCharToMultiByte` will fail if the target encoding is UTF-7 or UTF-8 and this is not
        // `None`.
        winnls::CP_UTF7 | winnls::CP_UTF8 => None,
        _ => Some(MaybeUninit::uninit()),
    };

    // Safety: `s_utf16.as_ptr()` is a valid pointer to a UTF-16 string, and `s_utf16_len` is its
    // length. `lpDefaultChar` is null. `lpUsedDefaultChar` is either null or valid. `cbMultiByte`
    // is zero.
    let required_buffer_space = unsafe {
        winnls::WideCharToMultiByte(
            needed_codepage,
            conversion_flags,
            s_utf16.as_ptr(),
            s_utf16_len,
            ptr::null_mut(),
            0,
            ptr::null(),
            match &mut is_non_representable {
                Some(x) => x.as_mut_ptr(),
                None => ptr::null_mut(),
            },
        )
    };

    // Bail on error.
    if required_buffer_space == 0 {
        drop(s_utf16);

        return Err(convert_error(s));
    }

    // Check if the string is not fully representable.
    if let Some(is_non_representable) = is_non_representable {
        // Safety: `is_non_representable` has been initialized by `WideCharToMultiByte`.
        let is_non_representable = unsafe { is_non_representable.assume_init() };

        if is_non_representable != 0 {
            drop(s_utf16);

            return Err(JvmError::OptStringNotRepresentable {
                opt_string: s.into_owned(),
            });
        }
    }

    // Convert the required buffer space to `usize`, and increment it by one for the null
    // terminator.
    //
    // This shouldn't overflow (see the comment on `MAX_INPUT_LEN` above), so we won't check for
    // overflow here.
    let required_buffer_space_usize: usize = required_buffer_space as _;
    let required_buffer_space_usize_with_nul: usize = required_buffer_space_usize + 1;

    // Allocate enough buffer space, including one byte for the null terminator.
    let mut output = Vec::<u8>::with_capacity(required_buffer_space_usize_with_nul);

    // Perform the actual conversion.
    //
    // Safety: `chunk.as_ptr()` is a valid pointer, and `chunk_len_i32` is its length.
    // `chunk_output_ptr` is a valid pointer, and `required_buffer_space` is its length.
    // All other raw pointers are null.
    let used_buffer_space = unsafe {
        winnls::WideCharToMultiByte(
            needed_codepage,
            conversion_flags,
            s_utf16.as_ptr(),
            s_utf16_len,
            output.as_mut_ptr(),
            required_buffer_space,
            ptr::null(),
            ptr::null_mut(),
        )
    };

    drop(s_utf16);

    // Bail on error.
    if used_buffer_space == 0 {
        drop(output);

        return Err(convert_error(s));
    }

    let used_buffer_space_usize: usize = used_buffer_space as usize;

    // Set the new length of the output buffer. Don't use `required_buffer_space`, just in case
    // `WideCharToMultiByte` changes its mind about how much buffer space it's actually going to
    // use.
    //
    // Safety: `used_buffer_space_usize` is the number of bytes that `WideCharToMultiByte` has
    // just initialized.
    unsafe {
        output.set_len(used_buffer_space_usize);
    }

    // That's it, it's converted. Now turn it into a `CString`. This will add a null terminator if
    // there isn't one already and check for null bytes in the middle.
    unsafe { bytes_to_cstr(Cow::Owned(output), Some(s.into())) }
}

/// Converts `s` into the Windows default character encoding.
pub(super) fn str_to_cstr_win32_default_codepage<'a>(
    s: Cow<'a, str>,
) -> Result<Cow<'a, CStr>, JvmError> {
    // Get the code page. There is a remote possibility that it is UTF-8. If so, pass the
    // string through unchanged (other than adding a null terminator). If not, we need to have
    // Windows convert the string to the expected code page first.

    // Safety: This function isn't actually unsafe.
    let needed_codepage = unsafe { winnls::GetACP() };

    if needed_codepage == winnls::CP_UTF8 {
        // The code page is UTF-8! Lucky us.
        return utf8_to_cstr(s);
    }

    // The code page is not UTF-8, so do the transcoding.
    str_to_cstr_win32(s, needed_codepage)
}

/// Transcodes text in an arbitrary Windows codepage into a Rust `String`. Used to test
/// round-tripping.
#[cfg(test)]
fn codepage_to_string_win32(
    codepage_string: impl AsRef<[u8]>,
    codepage: WCodepage,
    max_expected_utf16_len: WSize,
) -> io::Result<String> {
    let codepage_string_slice = codepage_string.as_ref();

    let codepage_string_slice_len: WSize = codepage_string_slice
        .len()
        .try_into()
        .expect("`codepage_string`'s length is too large to transcode with Win32");

    let mut buf = Vec::<u16>::with_capacity(
        max_expected_utf16_len
            .try_into()
            .expect("expected_utf16_len is negative or exceeds address space"),
    );

    // Safety: All of these pointers and lengths are valid and checked for overflow.
    let utf16_units_transcoded = unsafe {
        winnls::MultiByteToWideChar(
            codepage,
            0,
            codepage_string_slice.as_ptr() as *const _,
            codepage_string_slice_len,
            buf.as_mut_ptr(),
            max_expected_utf16_len,
        )
    };

    if utf16_units_transcoded == 0 {
        return Err(io::Error::last_os_error());
    }

    // Safety: `MultiByteToWideChar` claims to have initialized this many UTF-16 units.
    unsafe {
        buf.set_len(utf16_units_transcoded as _);
    }

    drop(codepage_string);

    let string =
        String::from_utf16(buf.as_slice()).expect("`MultiByteToWideChar` generated invalid UTF-16");

    Ok(string)
}

#[test]
fn test() {
    use assert_matches::assert_matches;

    {
        let result = str_to_cstr_win32("Hello, world 😎".into(), winnls::CP_UTF8).unwrap();
        assert_eq!(
            result.to_bytes_with_nul(),
            b"Hello, world \xf0\x9f\x98\x8e\0"
        );
        assert_matches!(result, Cow::Owned(_));
    }

    {
        let result = str_to_cstr_win32("Hello, world 😎\0".into(), winnls::CP_UTF8).unwrap();
        assert_eq!(
            result.to_bytes_with_nul(),
            b"Hello, world \xf0\x9f\x98\x8e\0"
        );
    }

    {
        let result = str_to_cstr_win32("Hello, world 😎".into(), 1252).unwrap_err();
        let error_string = assert_matches!(result, JvmError::OptStringNotRepresentable { opt_string } => opt_string);
        assert_eq!(error_string, "Hello, world 😎");
    }

    {
        let result = str_to_cstr_win32("Hello, world™".into(), 1252).unwrap();
        assert_eq!(result.to_bytes_with_nul(), b"Hello, world\x99\0");
        assert_matches!(result, Cow::Owned(_));
    }
}

#[test]
fn test_overflow() {
    use assert_matches::assert_matches;

    // Note: We avoid naïvely using `assert` here, because assertion failure will dump millions of
    // characters to the console. Instead, here are some functions for handling errors without
    // doing that.

    #[track_caller]
    fn check_and_clear_error_opt_string(expected_opt_string: &str, error: &mut JvmError) {
        if let Some(actual_opt_string) = error.opt_string_mut() {
            if actual_opt_string != expected_opt_string {
                panic!("opt_string was mangled in moving it to an error");
            }

            *actual_opt_string = String::new();
        }
    }

    #[track_caller]
    fn expect_success(
        expected_opt_string: &str,
        result: Result<Cow<'static, CStr>, JvmError>,
    ) -> Cow<'static, CStr> {
        match result {
            Ok(ok) => ok,
            Err(mut error) => {
                check_and_clear_error_opt_string(expected_opt_string, &mut error);
                panic!("unexpected transcoding failure: {}", error)
            }
        }
    }

    #[track_caller]
    fn expect_successful_roundtrip(
        expected_opt_string: &str,
        result: Result<Cow<'static, CStr>, JvmError>,
    ) -> Cow<'static, CStr> {
        let string = expect_success(expected_opt_string, result);
        assert!(
            expected_opt_string.as_bytes() == string.to_bytes(),
            "opt_string was transcoded successfully but mangled"
        );
        string
    }

    #[track_caller]
    fn expect_opt_string_too_long(
        expected_opt_string: &str,
        result: Result<Cow<'static, CStr>, JvmError>,
    ) {
        let mut error = match result {
            Err(err) => err,
            Ok(ok) => {
                assert!(
                    expected_opt_string.as_bytes() == ok.to_bytes(),
                    "transcoding unexpectedly succeeded and resulted in mangled output"
                );
                panic!("transcoding unexpectedly succeeded")
            }
        };

        check_and_clear_error_opt_string(expected_opt_string, &mut error);

        assert_matches!(error, JvmError::OptStringTooLong { .. });
    }

    {
        // Try transcoding a plain ASCII string.

        // First, allocate enough space to completely fill the maximum allowed length, plus one
        // more.
        //eprintln!("Allocating & filling ASCII");
        let string = vec![b'H'; MAX_INPUT_LEN.checked_add(1).unwrap()];

        //eprintln!("Checking UTF-8 correctness");
        let mut string = String::from_utf8(string).unwrap();

        // This string is currently one character too long to transcode, so there should be an
        // overflow error.
        //eprintln!("Transcoding ASCII string that's too long");
        expect_opt_string_too_long(
            &string,
            str_to_cstr_win32(string.as_str().into(), winnls::CP_UTF8),
        );

        // But if we remove one character…
        assert_eq!(string.pop(), Some('H'));

        // …then it should transcode fine.
        //eprintln!("Transcoding ASCII string that's not too long");
        expect_successful_roundtrip(
            &string,
            str_to_cstr_win32(string.as_str().into(), winnls::CP_UTF8),
        );
    }

    {
        // Try transcoding a non-ASCII string.

        // U+07FF is the highest code point that can be represnted in UTF-8 with only two bytes, so
        // we'll use that. The UTF-8 encoding is `df bf`. We fill it this way because it's much
        // faster than the naïve character-by-character approach (at least unless some future Rust
        // compiler performs this optimization on its own, but 1.66 doesn't).
        //eprintln!("Allocating & filling non-ASCII for UTF-8 and UTF-7");
        let string_byte_pairs = vec![u16::from_be(0xdfbf); MAX_INPUT_LEN / 2];

        //eprintln!("Checking UTF-8 correctness");
        let string: &str =
            std::str::from_utf8(bytemuck::cast_slice(string_byte_pairs.as_slice())).unwrap();

        // Again, the string should transcode without overflow.
        //eprintln!("Transcoding non-ASCII to UTF-8");
        expect_successful_roundtrip(string, str_to_cstr_win32(string.into(), winnls::CP_UTF8));

        // This should work even with UTF-7. This is the real reason we're using U+07FF: we need
        // to check that the highest code point that fits under the limit will not overflow even
        // with the worst-case code page.
        {
            //eprintln!("Transcoding non-ASCII to UTF-7");
            let result = expect_success(string, str_to_cstr_win32(string.into(), winnls::CP_UTF7));

            // *And* it should roundtrip back to UTF-8.
            //eprintln!("Transcoding UTF-7 back to UTF-8");
            let result: String = codepage_to_string_win32(
                result.to_bytes(),
                winnls::CP_UTF7,
                (string.len() / 2).try_into().unwrap(),
            )
            .unwrap();

            assert!(result == string, "didn't roundtrip via UTF-7");
        }
    }

    {
        // Try transcoding to Windows-1252. This is the slowest part of the test
        // (`WideCharToMultiByte` is very slow at this, for some reason), so it's done last.
        //eprintln!("Allocating & filling non-ASCII for Windows-1252");
        let string_byte_pairs = vec![u16::from_be(0xc2ae); MAX_INPUT_LEN / 2];

        //eprintln!("Checking UTF-8 correctness");
        let string: &str =
            std::str::from_utf8(bytemuck::cast_slice(string_byte_pairs.as_slice())).unwrap();

        //eprintln!("Transcoding non-ASCII to Windows-1252");
        let result = expect_success(string, str_to_cstr_win32(string.into(), 1252));

        //eprintln!("Checking Windows-1252 for correctness");
        assert!(
            result.to_bytes().iter().all(|byte| *byte == 0xae),
            "string didn't transcode to Windows-1252 properly"
        );
    }
}
