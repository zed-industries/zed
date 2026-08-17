// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use crate::common::*;
use crate::enums::{self, *};
use crate::reg_key_metadata::RegKeyMetadata;
use crate::reg_value::RegValue;
#[cfg(feature = "transactions")]
use crate::transaction::Transaction;
use crate::types::{FromRegValue, ToRegValue};
use std::default::Default;
use std::ffi::OsStr;
use std::io;
use std::mem::transmute;
use std::ptr;
use windows_sys::Win32::Foundation;
use windows_sys::Win32::System::Registry;
pub use windows_sys::Win32::System::Registry::HKEY;

/// Handle of opened registry key
#[derive(Debug)]
pub struct RegKey {
    hkey: HKEY,
}

unsafe impl Send for RegKey {}

impl RegKey {
    /// Open one of predefined keys:
    ///
    /// * `HKEY_CLASSES_ROOT`
    /// * `HKEY_CURRENT_USER`
    /// * `HKEY_LOCAL_MACHINE`
    /// * `HKEY_USERS`
    /// * `HKEY_PERFORMANCE_DATA`
    /// * `HKEY_PERFORMANCE_TEXT`
    /// * `HKEY_PERFORMANCE_NLSTEXT`
    /// * `HKEY_CURRENT_CONFIG`
    /// * `HKEY_DYN_DATA`
    /// * `HKEY_CURRENT_USER_LOCAL_SETTINGS`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    /// ```
    pub const fn predef(hkey: HKEY) -> RegKey {
        RegKey { hkey }
    }

    /// Load a registry hive from a file as an application hive.
    /// If `lock` is set to `true`, then the hive cannot be loaded again until
    /// it's unloaded (i.e. all keys from it go out of scope).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let handle = RegKey::load_app_key("C:\\myhive.dat", false)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn load_app_key<N: AsRef<OsStr>>(filename: N, lock: bool) -> io::Result<RegKey> {
        let options = if lock {
            Registry::REG_PROCESS_APPKEY
        } else {
            0
        };
        RegKey::load_app_key_with_flags(filename, enums::KEY_ALL_ACCESS, options)
    }

    /// Load a registry hive from a file as an application hive with desired
    /// permissions and options. If `options` is set to `REG_PROCESS_APPKEY`,
    /// then the hive cannot be loaded again until it's unloaded (i.e. all keys
    /// from it go out of scope).
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let handle = RegKey::load_app_key_with_flags("C:\\myhive.dat", KEY_READ, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn load_app_key_with_flags<N: AsRef<OsStr>>(
        filename: N,
        perms: Registry::REG_SAM_FLAGS,
        options: u32,
    ) -> io::Result<RegKey> {
        let c_filename = to_utf16(filename);
        let mut new_hkey: HKEY = 0;
        match unsafe {
            Registry::RegLoadAppKeyW(c_filename.as_ptr(), &mut new_hkey, perms, options, 0)
        } {
            0 => Ok(RegKey { hkey: new_hkey }),
            err => werr!(err),
        }
    }

    /// Return inner winapi HKEY of a key:
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    /// let soft = hklm.open_subkey("SOFTWARE")?;
    /// let handle = soft.raw_handle();
    /// # Ok(())
    /// # }
    /// ```
    pub const fn raw_handle(&self) -> HKEY {
        self.hkey
    }

    /// Open subkey with `KEY_READ` permissions.
    /// Will open another handle to itself if `path` is an empty string.
    /// To open with different permissions use `open_subkey_with_flags`.
    /// You can also use `create_subkey` to open with `KEY_ALL_ACCESS` permissions.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let soft = RegKey::predef(HKEY_CURRENT_USER)
    ///     .open_subkey("Software")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open_subkey<P: AsRef<OsStr>>(&self, path: P) -> io::Result<RegKey> {
        self.open_subkey_with_flags(path, enums::KEY_READ)
    }

    /// Open subkey with desired permissions.
    /// Will open another handle to itself if `path` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    /// hklm.open_subkey_with_flags("SOFTWARE\\Microsoft", KEY_READ)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open_subkey_with_flags<P: AsRef<OsStr>>(
        &self,
        path: P,
        perms: Registry::REG_SAM_FLAGS,
    ) -> io::Result<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = 0;
        match unsafe {
            Registry::RegOpenKeyExW(self.hkey, c_path.as_ptr(), 0, perms, &mut new_hkey)
        } {
            0 => Ok(RegKey { hkey: new_hkey }),
            err => werr!(err),
        }
    }

    /// Part of `transactions` feature.
    #[cfg(feature = "transactions")]
    pub fn open_subkey_transacted<P: AsRef<OsStr>>(
        &self,
        path: P,
        t: &Transaction,
    ) -> io::Result<RegKey> {
        self.open_subkey_transacted_with_flags(path, t, Registry::KEY_READ)
    }

    /// Part of `transactions` feature.
    #[cfg(feature = "transactions")]
    pub fn open_subkey_transacted_with_flags<P: AsRef<OsStr>>(
        &self,
        path: P,
        t: &Transaction,
        perms: Registry::REG_SAM_FLAGS,
    ) -> io::Result<RegKey> {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = 0;
        match unsafe {
            Registry::RegOpenKeyTransactedW(
                self.hkey,
                c_path.as_ptr(),
                0,
                perms,
                &mut new_hkey,
                t.handle,
                ptr::null_mut(),
            )
        } {
            0 => Ok(RegKey { hkey: new_hkey }),
            err => werr!(err),
        }
    }

    /// Create subkey (and all missing parent keys)
    /// and open it with `KEY_ALL_ACCESS` permissions.
    /// Will just open key if it already exists.
    /// If succeeds returns a tuple with the created subkey and its disposition,
    /// which can be `REG_CREATED_NEW_KEY` or `REG_OPENED_EXISTING_KEY`.
    /// Will open another handle to itself if `path` is an empty string.
    /// To create with different permissions use `create_subkey_with_flags`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let (settings, disp) = hkcu.create_subkey("Software\\MyProduct\\Settings")?;
    ///
    /// match disp {
    ///     REG_CREATED_NEW_KEY => println!("A new key has been created"),
    ///     REG_OPENED_EXISTING_KEY => println!("An existing key has been opened")
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_subkey<P: AsRef<OsStr>>(&self, path: P) -> io::Result<(RegKey, RegDisposition)> {
        self.create_subkey_with_flags(path, enums::KEY_ALL_ACCESS)
    }

    pub fn create_subkey_with_flags<P: AsRef<OsStr>>(
        &self,
        path: P,
        perms: Registry::REG_SAM_FLAGS,
    ) -> io::Result<(RegKey, RegDisposition)> {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = 0;
        let mut disp_buf: u32 = 0;
        match unsafe {
            Registry::RegCreateKeyExW(
                self.hkey,
                c_path.as_ptr(),
                0,
                ptr::null_mut(),
                Registry::REG_OPTION_NON_VOLATILE,
                perms,
                ptr::null_mut(),
                &mut new_hkey,
                &mut disp_buf,
            )
        } {
            0 => {
                let disp: RegDisposition = unsafe { transmute(disp_buf as u8) };
                Ok((RegKey { hkey: new_hkey }, disp))
            }
            err => werr!(err),
        }
    }

    /// Part of `transactions` feature.
    #[cfg(feature = "transactions")]
    pub fn create_subkey_transacted<P: AsRef<OsStr>>(
        &self,
        path: P,
        t: &Transaction,
    ) -> io::Result<(RegKey, RegDisposition)> {
        self.create_subkey_transacted_with_flags(path, t, Registry::KEY_ALL_ACCESS)
    }

    /// Part of `transactions` feature.
    #[cfg(feature = "transactions")]
    pub fn create_subkey_transacted_with_flags<P: AsRef<OsStr>>(
        &self,
        path: P,
        t: &Transaction,
        perms: Registry::REG_SAM_FLAGS,
    ) -> io::Result<(RegKey, RegDisposition)> {
        let c_path = to_utf16(path);
        let mut new_hkey: HKEY = 0;
        let mut disp_buf: u32 = 0;
        match unsafe {
            Registry::RegCreateKeyTransactedW(
                self.hkey,
                c_path.as_ptr(),
                0,
                ptr::null_mut(),
                Registry::REG_OPTION_NON_VOLATILE,
                perms,
                ptr::null_mut(),
                &mut new_hkey,
                &mut disp_buf,
                t.handle,
                ptr::null_mut(),
            )
        } {
            0 => {
                let disp: RegDisposition = unsafe { transmute(disp_buf as u8) };
                Ok((RegKey { hkey: new_hkey }, disp))
            }
            err => werr!(err),
        }
    }

    /// Copy all the values and subkeys from `path` to `dest` key.
    /// Will copy the content of `self` if `path` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let src = hkcu.open_subkey_with_flags("Software\\MyProduct", KEY_READ)?;
    /// let (dst, dst_disp) = hkcu.create_subkey("Software\\MyProduct\\Section2")?;
    /// src.copy_tree("Section1", &dst)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy_tree<P: AsRef<OsStr>>(&self, path: P, dest: &RegKey) -> io::Result<()> {
        let c_path = to_utf16(path);
        match unsafe { Registry::RegCopyTreeW(self.hkey, c_path.as_ptr(), dest.hkey) } {
            0 => Ok(()),
            err => werr!(err),
        }
    }

    pub fn query_info(&self) -> io::Result<RegKeyMetadata> {
        let mut info: RegKeyMetadata = RegKeyMetadata::default();
        match unsafe {
            Registry::RegQueryInfoKeyW(
                self.hkey,
                ptr::null_mut(), // Class: winapi::LPWSTR,
                ptr::null_mut(), // ClassLen: u32,
                ptr::null_mut(), // Reserved
                &mut info.sub_keys,
                &mut info.max_sub_key_len,
                &mut info.max_class_len,
                &mut info.values,
                &mut info.max_value_name_len,
                &mut info.max_value_len,
                ptr::null_mut(), // lpcbSecurityDescriptor: winapi::LPDWORD,
                &mut info.last_write_time.0,
            )
        } {
            0 => Ok(info),
            err => werr!(err),
        }
    }

    /// Return an iterator over subkeys names.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// println!("File extensions, registered in this system:");
    /// for i in RegKey::predef(HKEY_CLASSES_ROOT)
    ///     .enum_keys().map(|x| x.unwrap())
    ///     .filter(|x| x.starts_with("."))
    /// {
    ///     println!("{}", i);
    /// }
    /// ```
    pub const fn enum_keys(&self) -> EnumKeys {
        EnumKeys {
            key: self,
            index: 0,
        }
    }

    /// Return an iterator over values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let system = RegKey::predef(HKEY_LOCAL_MACHINE)
    ///     .open_subkey_with_flags("HARDWARE\\DESCRIPTION\\System", KEY_READ)?;
    /// for (name, value) in system.enum_values().map(|x| x.unwrap()) {
    ///     println!("{} = {:?}", name, value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub const fn enum_values(&self) -> EnumValues {
        EnumValues {
            key: self,
            index: 0,
        }
    }

    /// Delete key. Key names are not case sensitive.
    /// Cannot delete if it has subkeys.
    /// Use `delete_subkey_all` for that.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// RegKey::predef(HKEY_CURRENT_USER)
    ///     .delete_subkey(r"Software\MyProduct\History")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_subkey<P: AsRef<OsStr>>(&self, path: P) -> io::Result<()> {
        self.delete_subkey_with_flags(path, 0)
    }

    /// Delete key from the desired platform-specific view of the registry.
    /// Key names are not case sensitive.
    ///
    /// # Examples
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // delete the key from the 32-bit registry view
    /// RegKey::predef(HKEY_LOCAL_MACHINE)
    ///     .delete_subkey_with_flags(r"Software\MyProduct\History", KEY_WOW64_32KEY)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_subkey_with_flags<P: AsRef<OsStr>>(
        &self,
        path: P,
        perms: Registry::REG_SAM_FLAGS,
    ) -> io::Result<()> {
        let c_path = to_utf16(path);
        match unsafe {
            Registry::RegDeleteKeyExW(
                self.hkey,
                c_path.as_ptr(), // This parameter cannot be NULL.
                perms,
                0,
            )
        } {
            0 => Ok(()),
            err => werr!(err),
        }
    }

    /// Part of `transactions` feature.
    #[cfg(feature = "transactions")]
    pub fn delete_subkey_transacted<P: AsRef<OsStr>>(
        &self,
        path: P,
        t: &Transaction,
    ) -> io::Result<()> {
        self.delete_subkey_transacted_with_flags(path, t, 0)
    }

    /// Part of `transactions` feature.
    #[cfg(feature = "transactions")]
    pub fn delete_subkey_transacted_with_flags<P: AsRef<OsStr>>(
        &self,
        path: P,
        t: &Transaction,
        perms: Registry::REG_SAM_FLAGS,
    ) -> io::Result<()> {
        let c_path = to_utf16(path);
        match unsafe {
            Registry::RegDeleteKeyTransactedW(
                self.hkey,
                c_path.as_ptr(), // This parameter cannot be NULL.
                perms,
                0,
                t.handle,
                ptr::null_mut(),
            )
        } {
            0 => Ok(()),
            err => werr!(err),
        }
    }

    /// Recursively delete subkey with all its subkeys and values.
    /// If `path` is an empty string, the subkeys and values of this key are deleted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// RegKey::predef(HKEY_CURRENT_USER)
    ///     .delete_subkey_all("Software\\MyProduct")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_subkey_all<P: AsRef<OsStr>>(&self, path: P) -> io::Result<()> {
        let c_path;
        let path_ptr = if path.as_ref().is_empty() {
            ptr::null()
        } else {
            c_path = to_utf16(path);
            c_path.as_ptr()
        };
        match unsafe {
            Registry::RegDeleteTreeW(
                self.hkey,
                path_ptr, //If this parameter is NULL, the subkeys and values of this key are deleted.
            )
        } {
            0 => Ok(()),
            err => werr!(err),
        }
    }

    /// Get a value from registry and seamlessly convert it to the specified rust type
    /// with `FromRegValue` implemented (currently `String`, `u32` and `u64`).
    /// Will get the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings")?;
    /// let server: String = settings.get_value("server")?;
    /// let port: u32 = settings.get_value("port")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_value<T: FromRegValue, N: AsRef<OsStr>>(&self, name: N) -> io::Result<T> {
        match self.get_raw_value(name) {
            Ok(ref val) => FromRegValue::from_reg_value(val),
            Err(err) => Err(err),
        }
    }

    /// Get raw bytes from registry value.
    /// Will get the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings")?;
    /// let data = settings.get_raw_value("data")?;
    /// println!("Bytes: {:?}", data.bytes);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_raw_value<N: AsRef<OsStr>>(&self, name: N) -> io::Result<RegValue> {
        let c_name = to_utf16(name);
        let mut buf_len: u32 = 2048;
        let mut buf_type: u32 = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
        loop {
            match unsafe {
                Registry::RegQueryValueExW(
                    self.hkey,
                    c_name.as_ptr() as *const u16,
                    ptr::null_mut(),
                    &mut buf_type,
                    buf.as_mut_ptr() as *mut u8,
                    &mut buf_len,
                )
            } {
                0 => {
                    unsafe {
                        buf.set_len(buf_len as usize);
                    }
                    // minimal check before transmute to RegType
                    if buf_type > Registry::REG_QWORD {
                        return werr!(Foundation::ERROR_BAD_FILE_TYPE);
                    }
                    let t: RegType = unsafe { transmute(buf_type as u8) };
                    return Ok(RegValue {
                        bytes: buf,
                        vtype: t,
                    });
                }
                Foundation::ERROR_MORE_DATA => {
                    buf.reserve(buf_len as usize);
                }
                err => return werr!(err),
            }
        }
    }

    /// Seamlessly convert a value from a rust type and write it to the registry value
    /// with `ToRegValue` trait implemented (currently `String`, `&str`, `u32` and `u64`).
    /// Will set the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let (settings, disp) = hkcu.create_subkey("Software\\MyProduct\\Settings")?;
    /// settings.set_value("server", &"www.example.com")?;
    /// settings.set_value("port", &8080u32)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_value<T: ToRegValue, N: AsRef<OsStr>>(&self, name: N, value: &T) -> io::Result<()> {
        self.set_raw_value(name, &value.to_reg_value())
    }

    /// Write raw bytes from `RegValue` struct to a registry value.
    /// Will set the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// use winreg::{RegKey, RegValue};
    /// use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings")?;
    /// let bytes: Vec<u8> = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    /// let data = RegValue{ vtype: REG_BINARY, bytes: bytes};
    /// settings.set_raw_value("data", &data)?;
    /// println!("Bytes: {:?}", data.bytes);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_raw_value<N: AsRef<OsStr>>(&self, name: N, value: &RegValue) -> io::Result<()> {
        let c_name = to_utf16(name);
        let t = value.vtype.clone() as u32;
        match unsafe {
            Registry::RegSetValueExW(
                self.hkey,
                c_name.as_ptr(),
                0,
                t,
                value.bytes.as_ptr() as *const u8,
                value.bytes.len() as u32,
            )
        } {
            0 => Ok(()),
            err => werr!(err),
        }
    }

    /// Delete specified value from registry.
    /// Will delete the `Default` value if `name` is an empty string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use winreg::RegKey;
    /// # use winreg::enums::*;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    /// let settings = hkcu.open_subkey("Software\\MyProduct\\Settings")?;
    /// settings.delete_value("data")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_value<N: AsRef<OsStr>>(&self, name: N) -> io::Result<()> {
        let c_name = to_utf16(name);
        match unsafe { Registry::RegDeleteValueW(self.hkey, c_name.as_ptr()) } {
            0 => Ok(()),
            err => werr!(err),
        }
    }

    /// Save `Encodable` type to a registry key.
    /// Part of `serialization-serde` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// use serde_derive::Serialize;
    /// use winreg::RegKey;
    /// use winreg::enums::*;
    ///
    /// #[derive(Serialize)]
    /// struct Rectangle{
    ///     x: u32,
    ///     y: u32,
    ///     w: u32,
    ///     h: u32,
    /// }
    ///
    /// #[derive(Serialize)]
    /// struct Settings{
    ///     current_dir: String,
    ///     window_pos: Rectangle,
    ///     show_in_tray: bool,
    /// }
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let s: Settings = Settings{
    ///     current_dir: "C:\\".to_owned(),
    ///     window_pos: Rectangle{ x:200, y: 100, w: 800, h: 500 },
    ///     show_in_tray: false,
    /// };
    /// let s_key = RegKey::predef(HKEY_CURRENT_USER)
    ///     .open_subkey("Software\\MyProduct\\Settings")?;
    /// s_key.encode(&s)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "serialization-serde")]
    pub fn encode<T: serde::Serialize>(&self, value: &T) -> crate::encoder::EncodeResult<()> {
        let mut encoder = crate::encoder::Encoder::from_key(self)?;
        value.serialize(&mut encoder)?;
        encoder.commit()
    }

    /// Load `Decodable` type from a registry key.
    /// Part of `serialization-serde` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// use serde_derive::Deserialize;
    /// use winreg::RegKey;
    /// use winreg::enums::*;
    ///
    /// #[derive(Deserialize)]
    /// struct Rectangle{
    ///     x: u32,
    ///     y: u32,
    ///     w: u32,
    ///     h: u32,
    /// }
    ///
    /// #[derive(Deserialize)]
    /// struct Settings{
    ///     current_dir: String,
    ///     window_pos: Rectangle,
    ///     show_in_tray: bool,
    /// }
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let s_key = RegKey::predef(HKEY_CURRENT_USER)
    ///     .open_subkey("Software\\MyProduct\\Settings")?;
    /// let s: Settings = s_key.decode()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "serialization-serde")]
    pub fn decode<'de, T: serde::Deserialize<'de>>(&self) -> crate::decoder::DecodeResult<T> {
        let mut decoder = crate::decoder::Decoder::from_key(self)?;
        T::deserialize(&mut decoder)
    }

    fn close_(&mut self) -> io::Result<()> {
        // don't try to close predefined keys
        // The root hkey overflows with windows-sys, where HKEY is an alias for isize.
        // Cast to u32 to keep comparisons intact.
        if self.hkey as usize >= enums::HKEY_CLASSES_ROOT as usize {
            return Ok(());
        };
        match unsafe { Registry::RegCloseKey(self.hkey) } {
            0 => Ok(()),
            err => werr!(err),
        }
    }

    pub(crate) fn enum_key(&self, index: u32) -> Option<io::Result<String>> {
        let mut name_len = 2048;
        #[allow(clippy::unnecessary_cast)]
        let mut name = [0 as u16; 2048];
        match unsafe {
            Registry::RegEnumKeyExW(
                self.hkey,
                index,
                name.as_mut_ptr(),
                &mut name_len,
                ptr::null_mut(), // reserved
                ptr::null_mut(), // lpClass: LPWSTR,
                ptr::null_mut(), // lpcClass: LPDWORD,
                ptr::null_mut(), // lpftLastWriteTime: PFILETIME,
            )
        } {
            0 => match String::from_utf16(&name[..name_len as usize]) {
                Ok(s) => Some(Ok(s)),
                Err(_) => Some(werr!(Foundation::ERROR_INVALID_BLOCK)),
            },
            Foundation::ERROR_NO_MORE_ITEMS => None,
            err => Some(werr!(err)),
        }
    }

    pub(crate) fn enum_value(&self, index: u32) -> Option<io::Result<(String, RegValue)>> {
        let mut name_len = 2048;
        #[allow(clippy::unnecessary_cast)]
        let mut name = [0 as u16; 2048];

        let mut buf_len: u32 = 2048;
        let mut buf_type: u32 = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
        loop {
            match unsafe {
                Registry::RegEnumValueW(
                    self.hkey,
                    index,
                    name.as_mut_ptr(),
                    &mut name_len,
                    ptr::null_mut(), // reserved
                    &mut buf_type,
                    buf.as_mut_ptr() as *mut u8,
                    &mut buf_len,
                )
            } {
                0 => {
                    let name = match String::from_utf16(&name[..name_len as usize]) {
                        Ok(s) => s,
                        Err(_) => return Some(werr!(Foundation::ERROR_INVALID_DATA)),
                    };
                    unsafe {
                        buf.set_len(buf_len as usize);
                    }
                    // minimal check before transmute to RegType
                    if buf_type > Registry::REG_QWORD {
                        return Some(werr!(Foundation::ERROR_BAD_FILE_TYPE));
                    }
                    let t: RegType = unsafe { transmute(buf_type as u8) };
                    let value = RegValue {
                        bytes: buf,
                        vtype: t,
                    };
                    return Some(Ok((name, value)));
                }
                Foundation::ERROR_MORE_DATA => {
                    name_len += 1; //for NULL char
                    buf.reserve(buf_len as usize);
                }
                Foundation::ERROR_NO_MORE_ITEMS => return None,
                err => return Some(werr!(err)),
            }
        }
    }
}

impl Drop for RegKey {
    fn drop(&mut self) {
        self.close_().unwrap_or(());
    }
}

/// Iterator over subkeys names
pub struct EnumKeys<'key> {
    key: &'key RegKey,
    index: u32,
}

impl<'key> Iterator for EnumKeys<'key> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        match self.key.enum_key(self.index) {
            v @ Some(_) => {
                self.index += 1;
                v
            }
            e @ None => e,
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n as u32;
        self.next()
    }
}

/// Iterator over values
pub struct EnumValues<'key> {
    key: &'key RegKey,
    index: u32,
}

impl<'key> Iterator for EnumValues<'key> {
    type Item = io::Result<(String, RegValue)>;

    fn next(&mut self) -> Option<io::Result<(String, RegValue)>> {
        match self.key.enum_value(self.index) {
            v @ Some(_) => {
                self.index += 1;
                v
            }
            e @ None => e,
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index += n as u32;
        self.next()
    }
}
