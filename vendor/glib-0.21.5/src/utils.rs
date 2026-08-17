// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    ffi::{OsStr, OsString},
    mem, ptr,
};

use crate::{ffi, translate::*, GString};

// rustdoc-stripper-ignore-next
/// Same as [`get_prgname()`].
///
/// [`get_prgname()`]: fn.get_prgname.html
#[doc(alias = "get_program_name")]
#[inline]
pub fn program_name() -> Option<GString> {
    prgname()
}

#[doc(alias = "g_get_prgname")]
#[doc(alias = "get_prgname")]
#[inline]
pub fn prgname() -> Option<GString> {
    unsafe { from_glib_none(ffi::g_get_prgname()) }
}

// rustdoc-stripper-ignore-next
/// Same as [`set_prgname()`].
///
/// [`set_prgname()`]: fn.set_prgname.html
#[inline]
pub fn set_program_name(name: Option<impl IntoGStr>) {
    set_prgname(name)
}

#[doc(alias = "g_set_prgname")]
#[inline]
pub fn set_prgname(name: Option<impl IntoGStr>) {
    name.run_with_gstr(|name| unsafe { ffi::g_set_prgname(name.to_glib_none().0) })
}

#[doc(alias = "g_environ_getenv")]
pub fn environ_getenv<K: AsRef<OsStr>>(envp: &[OsString], variable: K) -> Option<OsString> {
    unsafe {
        from_glib_none(ffi::g_environ_getenv(
            envp.to_glib_none().0,
            variable.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_mkstemp")]
pub fn mkstemp<P: AsRef<std::path::Path>>(tmpl: P) -> i32 {
    unsafe {
        // NOTE: This modifies the string in place, which is fine here because
        // to_glib_none() will create a temporary, NUL-terminated copy of the string.
        ffi::g_mkstemp(tmpl.as_ref().to_glib_none().0)
    }
}

#[doc(alias = "g_mkstemp_full")]
pub fn mkstemp_full(tmpl: impl AsRef<std::path::Path>, flags: i32, mode: i32) -> i32 {
    unsafe {
        // NOTE: This modifies the string in place, which is fine here because
        // to_glib_none() will create a temporary, NUL-terminated copy of the string.
        ffi::g_mkstemp_full(tmpl.as_ref().to_glib_none().0, flags, mode)
    }
}

#[doc(alias = "g_mkdtemp")]
pub fn mkdtemp(tmpl: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf> {
    unsafe {
        // NOTE: This modifies the string in place and returns it but does not free it
        // if it returns NULL.
        let tmpl = tmpl.as_ref().to_glib_full();
        let res = ffi::g_mkdtemp(tmpl);
        if res.is_null() {
            ffi::g_free(tmpl as ffi::gpointer);
            None
        } else {
            from_glib_full(res)
        }
    }
}

#[doc(alias = "g_mkdtemp_full")]
pub fn mkdtemp_full(tmpl: impl AsRef<std::path::Path>, mode: i32) -> Option<std::path::PathBuf> {
    unsafe {
        // NOTE: This modifies the string in place and returns it but does not free it
        // if it returns NULL.
        let tmpl = tmpl.as_ref().to_glib_full();
        let res = ffi::g_mkdtemp_full(tmpl, mode);
        if res.is_null() {
            ffi::g_free(tmpl as ffi::gpointer);
            None
        } else {
            from_glib_full(res)
        }
    }
}

#[doc(alias = "g_file_get_contents")]
pub fn file_get_contents(
    filename: impl AsRef<std::path::Path>,
) -> Result<crate::Slice<u8>, crate::Error> {
    unsafe {
        let mut contents = ptr::null_mut();
        let mut length = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = ffi::g_file_get_contents(
            filename.as_ref().to_glib_none().0,
            &mut contents,
            length.as_mut_ptr(),
            &mut error,
        );
        if error.is_null() {
            Ok(crate::Slice::from_glib_full_num(
                contents,
                length.assume_init() as _,
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn is_canonical_pspec_name(name: &str) -> bool {
    name.as_bytes().iter().enumerate().all(|(i, c)| {
        i != 0 && (*c >= b'0' && *c <= b'9' || *c == b'-')
            || (*c >= b'A' && *c <= b'Z')
            || (*c >= b'a' && *c <= b'z')
    })
}

#[doc(alias = "g_uri_escape_string")]
pub fn uri_escape_string(
    unescaped: impl IntoGStr,
    reserved_chars_allowed: Option<impl IntoGStr>,
    allow_utf8: bool,
) -> crate::GString {
    unescaped.run_with_gstr(|unescaped| {
        reserved_chars_allowed.run_with_gstr(|reserved_chars_allowed| unsafe {
            from_glib_full(ffi::g_uri_escape_string(
                unescaped.to_glib_none().0,
                reserved_chars_allowed.to_glib_none().0,
                allow_utf8.into_glib(),
            ))
        })
    })
}

#[doc(alias = "g_uri_unescape_string")]
pub fn uri_unescape_string(
    escaped_string: impl IntoGStr,
    illegal_characters: Option<impl IntoGStr>,
) -> Option<crate::GString> {
    escaped_string.run_with_gstr(|escaped_string| {
        illegal_characters.run_with_gstr(|illegal_characters| unsafe {
            from_glib_full(ffi::g_uri_unescape_string(
                escaped_string.to_glib_none().0,
                illegal_characters.to_glib_none().0,
            ))
        })
    })
}

#[doc(alias = "g_uri_parse_scheme")]
pub fn uri_parse_scheme(uri: impl IntoGStr) -> Option<crate::GString> {
    uri.run_with_gstr(|uri| unsafe {
        from_glib_full(ffi::g_uri_parse_scheme(uri.to_glib_none().0))
    })
}

#[doc(alias = "g_uri_unescape_segment")]
pub fn uri_unescape_segment(
    escaped_string: Option<impl IntoGStr>,
    escaped_string_end: Option<impl IntoGStr>,
    illegal_characters: Option<impl IntoGStr>,
) -> Option<crate::GString> {
    escaped_string.run_with_gstr(|escaped_string| {
        escaped_string_end.run_with_gstr(|escaped_string_end| {
            illegal_characters.run_with_gstr(|illegal_characters| unsafe {
                from_glib_full(ffi::g_uri_unescape_segment(
                    escaped_string.to_glib_none().0,
                    escaped_string_end.to_glib_none().0,
                    illegal_characters.to_glib_none().0,
                ))
            })
        })
    })
}

#[cfg(test)]
mod tests {
    use std::{env, sync::Mutex, sync::OnceLock};

    //Mutex to prevent run environment tests parallel
    fn lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    const VAR_NAME: &str = "function_environment_test";

    fn check_getenv(val: &str) {
        let _data = lock().lock().unwrap();

        env::set_var(VAR_NAME, val);
        assert_eq!(env::var_os(VAR_NAME), Some(val.into()));
        assert_eq!(crate::getenv(VAR_NAME), Some(val.into()));

        let environ = crate::environ();
        assert_eq!(crate::environ_getenv(&environ, VAR_NAME), Some(val.into()));
    }

    fn check_setenv(val: &str) {
        let _data = lock().lock().unwrap();

        crate::setenv(VAR_NAME, val, true).unwrap();
        assert_eq!(env::var_os(VAR_NAME), Some(val.into()));
    }

    #[test]
    fn getenv() {
        check_getenv("Test");
        check_getenv("Тест"); // "Test" in Russian
    }

    #[test]
    fn setenv() {
        check_setenv("Test");
        check_setenv("Тест"); // "Test" in Russian
    }

    #[test]
    fn test_filename_from_uri() {
        use std::path::PathBuf;

        use crate::GString;
        let uri: GString = "file:///foo/bar.txt".into();
        if let Ok((filename, hostname)) = crate::filename_from_uri(&uri) {
            assert_eq!(filename, PathBuf::from(r"/foo/bar.txt"));
            assert_eq!(hostname, None);
        } else {
            unreachable!();
        }

        let uri: GString = "file://host/foo/bar.txt".into();
        if let Ok((filename, hostname)) = crate::filename_from_uri(&uri) {
            assert_eq!(filename, PathBuf::from(r"/foo/bar.txt"));
            assert_eq!(hostname, Some(GString::from("host")));
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_uri_parsing() {
        use crate::GString;
        assert_eq!(
            crate::uri_parse_scheme("foo://bar"),
            Some(GString::from("foo"))
        );
        assert_eq!(crate::uri_parse_scheme("foo"), None);

        let escaped = crate::uri_escape_string("&foo", crate::NONE_STR, true);
        assert_eq!(escaped, GString::from("%26foo"));

        let unescaped = crate::uri_unescape_string(escaped.as_str(), crate::GStr::NONE);
        assert_eq!(unescaped, Some(GString::from("&foo")));

        assert_eq!(
            crate::uri_unescape_segment(Some("/foo"), crate::NONE_STR, crate::NONE_STR),
            Some(GString::from("/foo"))
        );
        assert_eq!(
            crate::uri_unescape_segment(Some("/foo%"), crate::NONE_STR, crate::NONE_STR),
            None
        );
    }
}
