// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! This module is inefficient and should not be used by Rust programs except for
//! compatibility with GLib.Regex based APIs.

use crate::{
    ffi, translate::*, GStr, GStringPtr, MatchInfo, PtrSlice, Regex, RegexCompileFlags,
    RegexMatchFlags,
};
use std::{mem, ptr};

impl Regex {
    #[doc(alias = "g_regex_get_string_number")]
    #[doc(alias = "get_string_number")]
    pub fn string_number(&self, name: impl IntoGStr) -> i32 {
        name.run_with_gstr(|name| unsafe {
            ffi::g_regex_get_string_number(self.to_glib_none().0, name.to_glib_none().0)
        })
    }

    #[doc(alias = "g_regex_escape_nul")]
    pub fn escape_nul(string: impl IntoGStr) -> crate::GString {
        unsafe {
            string.run_with_gstr(|string| {
                from_glib_full(ffi::g_regex_escape_nul(
                    string.to_glib_none().0,
                    string.len() as _,
                ))
            })
        }
    }

    #[doc(alias = "g_regex_escape_string")]
    pub fn escape_string(string: impl IntoGStr) -> crate::GString {
        unsafe {
            string.run_with_gstr(|string| {
                from_glib_full(ffi::g_regex_escape_string(
                    string.to_glib_none().0,
                    string.len() as _,
                ))
            })
        }
    }

    #[doc(alias = "g_regex_check_replacement")]
    pub fn check_replacement(replacement: impl IntoGStr) -> Result<bool, crate::Error> {
        replacement.run_with_gstr(|replacement| unsafe {
            let mut has_references = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_regex_check_replacement(
                replacement.to_glib_none().0,
                has_references.as_mut_ptr(),
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib(has_references.assume_init()))
            } else {
                Err(from_glib_full(error))
            }
        })
    }

    #[doc(alias = "g_regex_match_simple")]
    pub fn match_simple(
        pattern: impl IntoGStr,
        string: impl IntoGStr,
        compile_options: RegexCompileFlags,
        match_options: RegexMatchFlags,
    ) -> bool {
        pattern.run_with_gstr(|pattern| {
            string.run_with_gstr(|string| unsafe {
                from_glib(ffi::g_regex_match_simple(
                    pattern.to_glib_none().0,
                    string.to_glib_none().0,
                    compile_options.into_glib(),
                    match_options.into_glib(),
                ))
            })
        })
    }

    #[doc(alias = "g_regex_replace")]
    pub fn replace(
        &self,
        string: impl IntoGStr,
        start_position: i32,
        replacement: impl IntoGStr,
        match_options: RegexMatchFlags,
    ) -> Result<crate::GString, crate::Error> {
        unsafe {
            string.run_with_gstr(|string| {
                replacement.run_with_gstr(|replacement| {
                    let mut error = ptr::null_mut();
                    let ret = ffi::g_regex_replace(
                        self.to_glib_none().0,
                        string.as_ptr() as *const _,
                        string.len() as _,
                        start_position,
                        replacement.to_glib_none().0,
                        match_options.into_glib(),
                        &mut error,
                    );
                    debug_assert_eq!(ret.is_null(), !error.is_null());
                    if error.is_null() {
                        Ok(from_glib_full(ret))
                    } else {
                        Err(from_glib_full(error))
                    }
                })
            })
        }
    }

    #[doc(alias = "g_regex_match_all")]
    pub fn match_all<'input>(
        &self,
        string: &'input GStr,
        match_options: RegexMatchFlags,
    ) -> Result<MatchInfo<'input>, crate::Error> {
        self.match_all_full(string, 0, match_options)
    }

    #[doc(alias = "g_regex_match_all_full")]
    pub fn match_all_full<'input>(
        &self,
        string: &'input GStr,
        start_position: i32,
        match_options: RegexMatchFlags,
    ) -> Result<MatchInfo<'input>, crate::Error> {
        unsafe {
            let mut match_info = ptr::null_mut();
            let mut error = ptr::null_mut();
            let res = ffi::g_regex_match_all_full(
                self.to_glib_none().0,
                string.to_glib_none().0,
                string.len() as _,
                start_position,
                match_options.into_glib(),
                &mut match_info,
                &mut error,
            );
            if error.is_null() {
                let match_info = MatchInfo::from_glib_full(match_info);
                debug_assert_eq!(match_info.matches(), from_glib(res));
                Ok(match_info)
            } else {
                debug_assert!(match_info.is_null());
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_regex_match")]
    pub fn match_<'input>(
        &self,
        string: &'input GStr,
        match_options: RegexMatchFlags,
    ) -> Result<MatchInfo<'input>, crate::Error> {
        self.match_full(string, 0, match_options)
    }

    #[doc(alias = "g_regex_match_full")]
    pub fn match_full<'input>(
        &self,
        string: &'input GStr,
        start_position: i32,
        match_options: RegexMatchFlags,
    ) -> Result<MatchInfo<'input>, crate::Error> {
        unsafe {
            let mut match_info = ptr::null_mut();
            let mut error = ptr::null_mut();
            let res = ffi::g_regex_match_full(
                self.to_glib_none().0,
                string.to_glib_none().0,
                string.len() as _,
                start_position,
                match_options.into_glib(),
                &mut match_info,
                &mut error,
            );
            if error.is_null() {
                let match_info = MatchInfo::from_glib_full(match_info);
                debug_assert_eq!(match_info.matches(), from_glib(res));
                Ok(match_info)
            } else {
                debug_assert!(match_info.is_null());
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_regex_replace_literal")]
    pub fn replace_literal(
        &self,
        string: impl IntoGStr,
        start_position: i32,
        replacement: impl IntoGStr,
        match_options: RegexMatchFlags,
    ) -> Result<crate::GString, crate::Error> {
        unsafe {
            string.run_with_gstr(|string| {
                replacement.run_with_gstr(|replacement| {
                    let mut error = ptr::null_mut();
                    let ret = ffi::g_regex_replace_literal(
                        self.to_glib_none().0,
                        string.to_glib_none().0,
                        string.len() as _,
                        start_position,
                        replacement.to_glib_none().0,
                        match_options.into_glib(),
                        &mut error,
                    );
                    debug_assert_eq!(ret.is_null(), !error.is_null());
                    if error.is_null() {
                        Ok(from_glib_full(ret))
                    } else {
                        Err(from_glib_full(error))
                    }
                })
            })
        }
    }

    #[doc(alias = "g_regex_split")]
    pub fn split(
        &self,
        string: impl IntoGStr,
        match_options: RegexMatchFlags,
    ) -> PtrSlice<GStringPtr> {
        self.split_full(string, 0, match_options, 0)
            .unwrap_or_default()
    }

    #[doc(alias = "g_regex_split_full")]
    pub fn split_full(
        &self,
        string: impl IntoGStr,
        start_position: i32,
        match_options: RegexMatchFlags,
        max_tokens: i32,
    ) -> Result<PtrSlice<GStringPtr>, crate::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            string.run_with_gstr(|string| {
                let ret = ffi::g_regex_split_full(
                    self.to_glib_none().0,
                    string.to_glib_none().0,
                    string.len() as _,
                    start_position,
                    match_options.into_glib(),
                    max_tokens,
                    &mut error,
                );
                debug_assert_eq!(ret.is_null(), !error.is_null());
                if error.is_null() {
                    Ok(FromGlibPtrContainer::from_glib_full(ret))
                } else {
                    Err(from_glib_full(error))
                }
            })
        }
    }

    #[doc(alias = "g_regex_split_simple")]
    pub fn split_simple(
        pattern: impl IntoGStr,
        string: impl IntoGStr,
        compile_options: RegexCompileFlags,
        match_options: RegexMatchFlags,
    ) -> PtrSlice<GStringPtr> {
        pattern.run_with_gstr(|pattern| {
            string.run_with_gstr(|string| unsafe {
                FromGlibPtrContainer::from_glib_full(ffi::g_regex_split_simple(
                    pattern.to_glib_none().0,
                    string.to_glib_none().0,
                    compile_options.into_glib(),
                    match_options.into_glib(),
                ))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_literal() {
        let regex = Regex::new(
            "s[ai]mple",
            RegexCompileFlags::OPTIMIZE,
            RegexMatchFlags::DEFAULT,
        )
        .expect("Regex new")
        .expect("Null regex");

        let quote = "This is a simple sample.";
        let result = regex
            .replace_literal(quote, 0, "XXX", RegexMatchFlags::DEFAULT)
            .expect("regex replace");

        assert_eq!(result, "This is a XXX XXX.");
    }

    #[test]
    fn test_split() {
        let regex = Regex::new(
            "s[ai]mple",
            RegexCompileFlags::OPTIMIZE,
            RegexMatchFlags::DEFAULT,
        )
        .expect("Regex new")
        .expect("Null regex");

        let quote = "This is a simple sample.";
        let result = regex.split(quote, RegexMatchFlags::DEFAULT);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "This is a ");
        assert_eq!(result[1], " ");
        assert_eq!(result[2], ".");
    }

    #[test]
    fn test_match() {
        let regex = Regex::new(r"\d", RegexCompileFlags::DEFAULT, RegexMatchFlags::DEFAULT)
            .expect("Regex new")
            .expect("Null regex");

        let input = crate::GString::from("87");
        let m = regex.match_(input.as_gstr(), RegexMatchFlags::DEFAULT);
        let m = m.unwrap();
        assert!(m.matches());
        assert_eq!(m.match_count(), 1);
        assert_eq!(m.fetch(0).as_deref(), Some("8"));
        assert!(m.next().unwrap());
        assert_eq!(m.fetch(0).as_deref(), Some("7"));
        assert!(!m.next().unwrap());
        assert!(m.fetch(0).is_none());

        let input = crate::GString::from("a");
        let m = regex.match_(input.as_gstr(), RegexMatchFlags::DEFAULT);
        let m = m.unwrap();
        assert!(!m.matches());
        assert_eq!(m.match_count(), 0);
        assert!(m.fetch(0).is_none());
    }
}
