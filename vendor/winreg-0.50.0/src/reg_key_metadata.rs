// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use std::fmt;
use std::ops::Deref;
use windows_sys::Win32::Foundation::FILETIME;
use windows_sys::Win32::Foundation::SYSTEMTIME;
use windows_sys::Win32::System::Time::FileTimeToSystemTime;

pub struct FileTime(pub(crate) FILETIME);

impl Default for FileTime {
    fn default() -> Self {
        Self(FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        })
    }
}

impl fmt::Debug for FileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FILETIME")
            .field("dwLowDateTime", &self.dwLowDateTime)
            .field("dwHighDateTime", &self.dwHighDateTime)
            .finish()
    }
}

impl Deref for FileTime {
    type Target = FILETIME;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Metadata returned by `RegKey::query_info`
#[derive(Debug, Default)]
pub struct RegKeyMetadata {
    // pub Class: winapi::LPWSTR,
    // pub ClassLen: u32,
    pub sub_keys: u32,
    pub max_sub_key_len: u32,
    pub max_class_len: u32,
    pub values: u32,
    pub max_value_name_len: u32,
    pub max_value_len: u32,
    // pub SecurityDescriptor: u32,
    pub last_write_time: FileTime,
}

impl RegKeyMetadata {
    /// Returns `last_write_time` field as `windows_sys::Win32::Foundation::SYSTEMTIME`
    pub fn get_last_write_time_system(&self) -> SYSTEMTIME {
        let mut st: SYSTEMTIME = unsafe { ::std::mem::zeroed() };
        unsafe {
            FileTimeToSystemTime(&self.last_write_time.0, &mut st);
        }
        st
    }

    /// Returns `last_write_time` field as `chrono::NaiveDateTime`.
    /// Part of `chrono` feature.
    #[cfg(feature = "chrono")]
    pub fn get_last_write_time_chrono(&self) -> chrono::NaiveDateTime {
        let st = self.get_last_write_time_system();

        chrono::NaiveDate::from_ymd_opt(st.wYear.into(), st.wMonth.into(), st.wDay.into())
            .expect("out-of-range date, invalid month and/or day")
            .and_hms_opt(st.wHour.into(), st.wMinute.into(), st.wSecond.into())
            .expect("invalid hour, minute and/or second")
    }
}
