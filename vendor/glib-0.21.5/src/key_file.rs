// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, path, ptr};

use crate::{ffi, translate::*, Error, GString, GStringPtr, KeyFile, KeyFileFlags, PtrSlice};

impl KeyFile {
    #[doc(alias = "g_key_file_save_to_file")]
    pub fn save_to_file<T: AsRef<std::path::Path>>(&self, filename: T) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_save_to_file(
                self.to_glib_none().0,
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_load_from_data_dirs")]
    pub fn load_from_data_dirs<T: AsRef<std::path::Path>>(
        &self,
        file: T,
        flags: KeyFileFlags,
    ) -> Result<path::PathBuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut full_path: *mut libc::c_char = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_data_dirs(
                self.to_glib_none().0,
                file.as_ref().to_glib_none().0,
                &mut full_path,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                let path: GString = from_glib_full(full_path);
                Ok(path::PathBuf::from(&path))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_load_from_dirs")]
    pub fn load_from_dirs<T: AsRef<std::path::Path>, U: AsRef<std::path::Path>>(
        &self,
        file: T,
        search_dirs: &[U],
        flags: KeyFileFlags,
    ) -> Result<path::PathBuf, Error> {
        unsafe {
            let search_dirs: Vec<&std::path::Path> =
                search_dirs.iter().map(AsRef::as_ref).collect();
            let mut error = ptr::null_mut();
            let mut full_path: *mut libc::c_char = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_dirs(
                self.to_glib_none().0,
                file.as_ref().to_glib_none().0,
                search_dirs.to_glib_none().0,
                &mut full_path,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                let path: GString = from_glib_full(full_path);
                Ok(path::PathBuf::from(&path))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_to_data")]
    pub fn to_data(&self) -> GString {
        unsafe {
            let ret =
                ffi::g_key_file_to_data(self.to_glib_none().0, ptr::null_mut(), ptr::null_mut());
            from_glib_full(ret)
        }
    }

    #[doc(alias = "g_key_file_get_groups")]
    #[doc(alias = "get_groups")]
    pub fn groups(&self) -> PtrSlice<GStringPtr> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = ffi::g_key_file_get_groups(self.to_glib_none().0, length.as_mut_ptr());
            FromGlibContainer::from_glib_full_num(ret, length.assume_init() as _)
        }
    }

    #[doc(alias = "g_key_file_get_keys")]
    #[doc(alias = "get_keys")]
    pub fn keys(&self, group_name: &str) -> Result<PtrSlice<GStringPtr>, crate::Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_keys(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    length.assume_init() as _,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_boolean")]
    #[doc(alias = "get_boolean")]
    pub fn boolean(&self, group_name: &str, key: &str) -> Result<bool, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_boolean(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_has_key")]
    pub fn has_key(&self, group_name: &str, key: &str) -> Result<bool, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_has_key(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_boolean_list")]
    #[doc(alias = "get_boolean_list")]
    pub fn boolean_list(&self, group_name: &str, key: &str) -> Result<Vec<bool>, Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_boolean_list(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if !error.is_null() {
                return Err(from_glib_full(error));
            }
            Ok(FromGlibContainer::from_glib_container_num(
                ret,
                length.assume_init() as _,
            ))
        }
    }

    #[doc(alias = "g_key_file_get_string")]
    #[doc(alias = "get_string")]
    pub fn string(&self, group_name: &str, key: &str) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_string(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                ffi::g_free(ret as *mut _);
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_string_list")]
    #[doc(alias = "get_string_list")]
    pub fn string_list(&self, group_name: &str, key: &str) -> Result<PtrSlice<GStringPtr>, Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_string_list(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    length.assume_init() as _,
                ))
            } else {
                ffi::g_strfreev(ret);
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_locale_string")]
    #[doc(alias = "get_locale_string")]
    pub fn locale_string(
        &self,
        group_name: &str,
        key: &str,
        locale: Option<&str>,
    ) -> Result<GString, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_locale_string(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                locale.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                ffi::g_free(ret as *mut _);
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_key_file_get_locale_string_list")]
    #[doc(alias = "get_locale_string_list")]
    pub fn locale_string_list(
        &self,
        group_name: &str,
        key: &str,
        locale: Option<&str>,
    ) -> Result<PtrSlice<GStringPtr>, Error> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_locale_string_list(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
                key.to_glib_none().0,
                locale.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    length.assume_init() as _,
                ))
            } else {
                ffi::g_strfreev(ret);
                Err(from_glib_full(error))
            }
        }
    }
}
