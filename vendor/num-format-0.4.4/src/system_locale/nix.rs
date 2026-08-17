#![cfg(all(feature = "with-system-locale", unix))]

mod bsd;
mod encoding;
mod linux;

pub(crate) use self::encoding::{Encoding, UTF_8};

cfg_if! {
    if #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "netbsd"
    ))] {
        use self::bsd::{get_encoding, get_lconv, get_name};
    } else {
        use self::linux::{get_encoding, get_lconv, get_name};
    }
}

use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::process::Command;
use std::ptr::{self, NonNull};

use libc::{c_char, c_int, c_void};

use crate::error::Error;
use crate::grouping::Grouping;
use crate::locale::Locale;
use crate::strings::{DecString, InfString, MinString, NanString, PlusString, SepString};
use crate::system_locale::SystemLocale;

extern "C" {
    pub fn freelocale(locale: *const c_void);
    pub fn newlocale(mask: c_int, name: *const c_char, base: *const c_void) -> *const c_void;
    pub fn uselocale(locale: *const c_void) -> *const c_void;
}

pub(crate) fn available_names() -> HashSet<String> {
    let inner = |bin| {
        let output = Command::new(bin).arg("-a").output().ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = std::str::from_utf8(&output.stdout).ok()?;
        let set = stdout
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<HashSet<String>>();
        Some(set)
    };
    if let Some(set) = inner("locale") {
        return set;
    }
    HashSet::default()
}

pub(crate) fn new(maybe_name: Option<String>) -> Result<SystemLocale, Error> {
    // create a new locale object
    let new = new_locale(&maybe_name)?;

    let inner = || {
        // use the new locale object, while saving the initial one
        let initial = use_locale(new)?;

        // get the encoding
        let encoding = get_encoding(new)?;

        // get the lconv
        let lconv = get_lconv(new, encoding)?;

        // get the name
        let mut name = match maybe_name {
            Some(name) => name,
            None => get_name(new, encoding)?,
        };
        if &name == "POSIX" {
            name = "C".to_string();
        }

        // reset to the initial locale object
        let _ = use_locale(initial);

        let system_locale = SystemLocale {
            dec: lconv.dec,
            grp: lconv.grp,
            inf: InfString::new(Locale::en.infinity()).unwrap(),
            min: lconv.min,
            name,
            nan: NanString::new(Locale::en.nan()).unwrap(),
            plus: lconv.plus,
            sep: lconv.sep,
        };

        Ok(system_locale)
    };

    let output = inner();

    // free the new locale object
    free_locale(new);

    output
}

fn free_locale(locale: *const c_void) {
    unsafe { freelocale(locale) };
}

fn new_locale(name: &Option<String>) -> Result<*const c_void, Error> {
    let name_cstring = match name {
        Some(ref name) => {
            CString::new(name.as_bytes()).map_err(|_| Error::interior_nul_byte(name.to_string()))?
        }
        None => CString::new("").unwrap(),
    };
    let mask = libc::LC_CTYPE_MASK | libc::LC_MONETARY_MASK | libc::LC_NUMERIC_MASK;
    let new_locale = unsafe { newlocale(mask, name_cstring.as_ptr(), ptr::null()) };
    if new_locale.is_null() {
        match name {
            Some(name) => return Err(Error::parse_locale(name)),
            None => {
                return Err(Error::system_invalid_return(
                    "newlocale",
                    "newlocale unexpectedly returned a null pointer.",
                ));
            }
        }
    }
    Ok(new_locale)
}

fn use_locale(locale: *const c_void) -> Result<*const c_void, Error> {
    let old_locale = unsafe { uselocale(locale) };
    if old_locale.is_null() {
        return Err(Error::system_invalid_return(
            "uselocale",
            "uselocale unexpectedly returned a null pointer.",
        ));
    }
    Ok(old_locale)
}

pub(crate) struct Lconv {
    pub(crate) dec: DecString,
    pub(crate) grp: Grouping,
    pub(crate) min: MinString,
    pub(crate) plus: PlusString,
    pub(crate) sep: SepString,
}

impl Lconv {
    pub(crate) fn new(lconv: &libc::lconv, encoding: Encoding) -> Result<Lconv, Error> {
        let dec = {
            let s = StaticCString::new(lconv.decimal_point, encoding, "lconv.decimal_point")?
                .to_string()?;
            DecString::new(&s)?
        };

        let grp = StaticCString::new(lconv.grouping, encoding, "lconv.grouping")?.to_grouping()?;

        let min = {
            let s = StaticCString::new(lconv.negative_sign, encoding, "lconv.negative_sign")?
                .to_string()?;
            MinString::new(&s)?
        };

        let plus = {
            let s = StaticCString::new(lconv.positive_sign, encoding, "lconv.positive_sign")?
                .to_string()?;
            PlusString::new(&s)?
        };

        let sep = {
            let s = StaticCString::new(lconv.thousands_sep, encoding, "lconv.thousands_sep")?
                .to_string()?;
            SepString::new(&s)?
        };

        Ok(Lconv {
            dec,
            grp,
            min,
            plus,
            sep,
        })
    }
}

/// Invariants: nul terminated, static lifetime
pub(crate) struct StaticCString {
    encoding: Encoding,
    non_null: NonNull<c_char>,
}

impl StaticCString {
    pub(crate) fn new(
        ptr: *const std::os::raw::c_char,
        encoding: Encoding,
        function_name: &str,
    ) -> Result<StaticCString, Error> {
        let non_null = NonNull::new(ptr as *mut c_char).ok_or_else(|| {
            Error::system_invalid_return(
                function_name,
                format!("{} unexpectedly returned a null pointer.", function_name),
            )
        })?;
        Ok(StaticCString { encoding, non_null })
    }

    pub(crate) fn to_grouping(&self) -> Result<Grouping, Error> {
        let ptr = self.non_null.as_ptr();
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let bytes = cstr.to_bytes();
        let grouping = match bytes {
            [3, 2] | [2, 3] => Grouping::Indian,
            [] | [127] => Grouping::Posix,
            [3] | [3, 3] => Grouping::Standard,
            _ => return Err(Error::system_unsupported_grouping(bytes)),
        };
        Ok(grouping)
    }

    pub(crate) fn to_string(&self) -> Result<String, Error> {
        let ptr = self.non_null.as_ptr();
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let bytes = cstr.to_bytes();
        self.encoding.decode(bytes)
    }
}
