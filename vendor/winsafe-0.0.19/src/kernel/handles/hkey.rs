#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, ffi_types::*, iterators::*, privs::*};
use crate::prelude::*;

impl_handle! { HKEY;
	/// Handle to a
	/// [registry key](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hkey).
	///
	/// This handle also exposes several
	/// [predefined registry keys](https://learn.microsoft.com/en-us/windows/win32/sysinfo/predefined-keys),
	/// like `HKEY::CURRENT_USER`, which are always open and ready to be used.
	/// Usually, they are the starting point to open a registry key.
}

impl kernel_Hkey for HKEY {}

macro_rules! predef_key {
	($name:ident, $val:expr) => {
		/// Predefined registry key, always open.
		const $name: HKEY = HKEY($val as *mut _);
	};
}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HKEY`](crate::HKEY).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hkey: Handle {
	predef_key!(CLASSES_ROOT, 0x8000_0000);
	predef_key!(CURRENT_USER, 0x8000_0001);
	predef_key!(LOCAL_MACHINE, 0x8000_0002);
	predef_key!(USERS, 0x8000_0003);
	predef_key!(PERFORMANCE_DATA, 0x8000_0004);
	predef_key!(CURRENT_CONFIG, 0x8000_0005);
	predef_key!(DYN_DATA, 0x8000_0006);
	predef_key!(CURRENT_USER_LOCAL_SETTINGS, 0x8000_0007);
	predef_key!(PERFORMANCE_TEXT, 0x8000_0050);
	predef_key!(PERFORMANCE_NLSTEXT, 0x8000_0060);

	/// [`RegConnectRegistry`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regconnectregistryw)
	/// function.
	///
	/// # Panics
	///
	/// Panics if `predef_key` is different from:
	///
	/// - [`HKEY::LOCAL_MACHINE`](crate::prelude::kernel_Hkey::LOCAL_MACHINE);
	/// - [`HKEY::PERFORMANCE_DATA`](crate::prelude::kernel_Hkey::PERFORMANCE_DATA);
	/// - [`HKEY::USERS`](crate::prelude::kernel_Hkey::USERS).
	#[must_use]
	fn RegConnectRegistry(
		machine_name: Option<&str>,
		predef_hkey: &HKEY,
	) -> SysResult<RegCloseKeyGuard>
	{
		if *predef_hkey != HKEY::LOCAL_MACHINE
			&& *predef_hkey != HKEY::PERFORMANCE_DATA
			&& *predef_hkey != HKEY::USERS
		{
			panic!("Invalid predef_key.");
		}

		let mut hkey = HKEY::NULL;
		unsafe {
			error_to_sysresult(
				ffi::RegConnectRegistryW(
					WString::from_opt_str(machine_name).as_ptr(),
					predef_hkey.ptr(),
					hkey.as_mut(),
				),
			).map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegCopyTree`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcopytreew)
	/// function.
	fn RegCopyTree(&self, sub_key: Option<&str>, dest: &HKEY) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				ffi::RegCopyTreeW(
					self.ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					dest.ptr(),
				)
			},
		)
	}

	/// [`RegCreateKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexw)
	/// function.
	#[must_use]
	fn RegCreateKeyEx(&self,
		sub_key: &str,
		class: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<(RegCloseKeyGuard, co::REG_DISPOSITION)>
	{
		let mut hkey = HKEY::NULL;
		let mut disposition = co::REG_DISPOSITION::default();

		unsafe {
			error_to_sysresult(
				ffi::RegCreateKeyExW(
					self.ptr(),
					WString::from_str(sub_key).as_ptr(),
					0,
					WString::from_opt_str(class).as_ptr(),
					options.raw(),
					access_rights.raw(),
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					hkey.as_mut(),
					disposition.as_mut(),
				),
			).map(|_| (RegCloseKeyGuard::new(hkey), disposition))
		}
	}

	/// [`RegCreateKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeytransactedw)
	/// function.
	#[must_use]
	fn RegCreateKeyTransacted(&self,
		sub_key: &str,
		class: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		htransaction: &HTRANSACTION,
	) -> SysResult<(RegCloseKeyGuard, co::REG_DISPOSITION)>
	{
		let mut hkey = HKEY::NULL;
		let mut disposition = co::REG_DISPOSITION::default();

		unsafe {
			error_to_sysresult(
				ffi::RegCreateKeyTransactedW(
					self.ptr(),
					WString::from_str(sub_key).as_ptr(),
					0,
					WString::from_opt_str(class).as_ptr(),
					options.raw(),
					access_rights.raw(),
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					hkey.as_mut(),
					disposition.as_mut(),
					htransaction.ptr(),
					std::ptr::null_mut(),
				),
			).map(|_| (RegCloseKeyGuard::new(hkey), disposition))
		}
	}

	/// [`RegDeleteKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyw)
	/// function.
	fn RegDeleteKey(&self, sub_key: &str) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				ffi::RegDeleteKeyW(
					self.ptr(),
					WString::from_str(sub_key).as_ptr(),
				)
			},
		)
	}

	/// [`RegDeleteKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyexw)
	/// function.
	///
	/// # Panics
	///
	/// Panics if `platform_view` is different from
	/// [`co::KEY::WOW64_32KEY`](crate::co::KEY::WOW64_32KEY) and
	/// [`co::KEY::WOW64_64KEY`](crate::co::KEY::WOW64_64KEY).
	fn RegDeleteKeyEx(&self,
		sub_key: &str,
		platform_view: co::KEY,
	) -> SysResult<()>
	{
		if platform_view != co::KEY::WOW64_32KEY
			&& platform_view != co::KEY::WOW64_64KEY
		{
			panic!("Platform view must be co::KEY::WOW64_32KEY or co::KEY::WOW64_64KEY");
		}

		error_to_sysresult(
			unsafe {
				ffi::RegDeleteKeyExW(
					self.ptr(),
					WString::from_str(sub_key).as_ptr(),
					platform_view.raw(),
					0,
				)
			},
		)
	}

	/// [`RegDeleteKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeytransactedw)
	/// function.
	fn RegDeleteKeyTransacted(&self,
		sub_key: &str,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				ffi::RegDeleteKeyTransactedW(
					self.ptr(),
					WString::from_str(sub_key).as_ptr(),
					access_rights.raw(),
					0,
					htransaction.ptr(),
					std::ptr::null_mut(),
				)
			},
		)
	}

	/// [`RegDeleteTree`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletetreew)
	/// function.
	fn RegDeleteTree(&self, sub_key: Option<&str>) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				ffi::RegDeleteTreeW(
					self.ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
				)
			},
		)
	}

	/// [`RegDeleteValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletevaluew)
	/// function.
	fn RegDeleteValue(&self, value_name: Option<&str>) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				ffi::RegDeleteValueW(
					self.ptr(),
					WString::from_opt_str(value_name).as_ptr(),
				)
			},
		)
	}

	/// [`RegDisablePredefinedCache`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablepredefinedcache)
	/// function.
	fn RegDisablePredefinedCache() -> SysResult<()> {
		error_to_sysresult(unsafe { ffi::RegDisablePredefinedCache() })
	}

	/// [`RegDisablePredefinedCacheEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablepredefinedcacheex)
	/// function.
	fn RegDisablePredefinedCacheEx() -> SysResult<()> {
		error_to_sysresult(unsafe { ffi::RegDisablePredefinedCacheEx() })
	}

	/// [`RegDisableReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablereflectionkey)
	/// function.
	fn RegDisableReflectionKey(&self) -> SysResult<()> {
		error_to_sysresult(unsafe { ffi::RegDisableReflectionKey(self.ptr()) })
	}

	/// [`RegEnableReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenablereflectionkey)
	/// function.
	fn RegEnableReflectionKey(&self) -> SysResult<()> {
		error_to_sysresult(unsafe { ffi::RegEnableReflectionKey(self.ptr()) })
	}

	/// Returns an iterator over the names of the keys, which calls
	/// [`RegEnumKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyexw)
	/// repeatedly.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for key_name in hkey.RegEnumKeyEx()? {
	///     let key_name = key_name?;
	///     println!("{}", key_name);
	/// }
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegEnumKeyEx(&self,
	) -> SysResult<Box<dyn Iterator<Item = SysResult<String>> + '_>>
	{
		Ok(Box::new(HkeyKeyIter::new(self)?))
	}

	/// Returns an iterator of the names and types of the values, which calls
	/// [`RegEnumValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumvaluew)
	/// repeatedly.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Appearance"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for value_and_type in hkey.RegEnumValue()? {
	///     let (value, reg_type) = value_and_type?;
	///     println!("{}, {}", value, reg_type);
	/// }
	///
	/// // Collecting into a Vec
	/// let values_and_types: Vec<(String, co::REG)> =
	///     hkey.RegEnumValue()?
	///         .collect::<w::SysResult<Vec<_>>>()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegEnumValue(&self,
	) -> SysResult<Box<dyn Iterator<Item = SysResult<(String, co::REG)>> + '_>>
	{
		Ok(Box::new(HkeyValueIter::new(self)?))
	}

	/// [`RegFlushKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regflushkey)
	/// function.
	fn RegFlushKey(&self) -> SysResult<()> {
		error_to_sysresult(unsafe { ffi::RegFlushKey(self.ptr()) })
	}

	/// [`RegGetValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew)
	/// function.
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID)
	/// and [`co::ERROR::INVALID_DATA`](crate::co::ERROR::INVALID_DATA).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let val = w::HKEY::CURRENT_USER.RegGetValue(
	///     Some("Control Panel\\Mouse"),
	///     Some("Beep"),
	/// )?;
	///
	/// match val {
	///     w::RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     w::RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     w::RegistryValue::Sz(s) => println!("String: {}", s),
	///     w::RegistryValue::ExpandSz(s) => {
	///         println!("Env string: {}", w::ExpandEnvironmentStrings(&s)?);
	///     },
	///     w::RegistryValue::MultiSz(strs) => {
	///        println!("Multi string:");
	///        for s in strs.iter() {
	///            print!("[{}] ", s);
	///        }
	///        println!("");
	///     },
	///     w::RegistryValue::Binary(bin) => {
	///         println!("Binary:");
	///         for b in bin.iter() {
	///             print!("{:02x} ", b);
	///         }
	///         println!("");
	///     },
	///     w::RegistryValue::None => println!("No value"),
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn RegGetValue(&self,
		sub_key: Option<&str>,
		value_name: Option<&str>,
	) -> SysResult<RegistryValue>
	{
		let sub_key_w = WString::from_opt_str(sub_key);
		let value_name_w = WString::from_opt_str(value_name);
		let mut raw_data_type1 = u32::default();
		let mut data_len1 = u32::default();

		// Query data type and length.
		error_to_sysresult(
			unsafe {
				ffi::RegGetValueW(
					self.ptr(),
					sub_key_w.as_ptr(),
					value_name_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).raw(),
					&mut raw_data_type1,
					std::ptr::null_mut(),
					&mut data_len1,
				)
			},
		)?;

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut raw_data_type2 = u32::default();
		let mut data_len2 = data_len1;

		// Retrieve the value content.
		error_to_sysresult(
			unsafe {
				ffi::RegGetValueW(
					self.ptr(),
					sub_key_w.as_ptr(),
					value_name_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).raw(),
					&mut raw_data_type2,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			},
		)?;

		validate_retrieved_reg_val(
			unsafe { co::REG::from_raw(raw_data_type1) }, data_len1,
			unsafe { co::REG::from_raw(raw_data_type2) }, data_len2,
			buf,
		)
	}

	/// [`RegLoadKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regloadkeyw)
	/// function.
	fn RegLoadKey(&self,
		sub_key: Option<&str>,
		file_path: &str,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				ffi::RegLoadKeyW(
					self.ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					WString::from_str(file_path).as_ptr(),
				)
			},
		)
	}

	/// [`RegOpenCurrentUser`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopencurrentuser)
	/// function.
	#[must_use]
	fn RegOpenCurrentUser(
		access_rights: co::KEY,
	) -> SysResult<RegCloseKeyGuard>
	{
		let mut hkey = HKEY::NULL;
		unsafe {
			error_to_sysresult(
				ffi::RegOpenCurrentUser(access_rights.raw(), hkey.as_mut()),
			).map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegOpenKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Mouse"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegOpenKeyEx(&self,
		sub_key: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
	) -> SysResult<RegCloseKeyGuard>
	{
		let mut hkey = HKEY::NULL;
		unsafe {
			error_to_sysresult(
				ffi::RegOpenKeyExW(
					self.ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					options.raw(),
					access_rights.raw(),
					hkey.as_mut(),
				),
			).map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegOpenKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeytransactedw)
	/// function.
	#[must_use]
	fn RegOpenKeyTransacted(&self,
		sub_key: &str,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION,
	) -> SysResult<RegCloseKeyGuard>
	{
		let mut hkey = HKEY::NULL;
		unsafe {
			error_to_sysresult(
				ffi::RegOpenKeyTransactedW(
					self.ptr(),
					WString::from_str(sub_key).as_ptr(),
					options.raw(),
					access_rights.raw(),
					hkey.as_mut(),
					htransaction.ptr(),
					std::ptr::null_mut(),
				),
			).map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegQueryInfoKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryinfokeyw)
	/// function.
	fn RegQueryInfoKey(&self,
		mut class: Option<&mut WString>,
		num_sub_keys: Option<&mut u32>,
		max_sub_key_name_len: Option<&mut u32>,
		max_class_len: Option<&mut u32>,
		num_values: Option<&mut u32>,
		max_value_name_len: Option<&mut u32>,
		max_value_len: Option<&mut u32>,
		security_descr_len: Option<&mut u32>,
		last_write_time: Option<&mut FILETIME>,
	) -> SysResult<()>
	{
		let (mut class_ptr, mut class_len) = match &mut class {
			Some(class) => {
				if class.buf_len() < SSO_LEN { // start with no string heap allocation
					**class = WString::new_alloc_buf(SSO_LEN); // make buffer at least this length
				}
				(unsafe { class.as_mut_ptr() }, class.buf_len() as u32)
			},
			None => (std::ptr::null_mut(), 0),
		};

		let num_sub_keys = num_sub_keys.map_or(std::ptr::null_mut(), |re| re as _);
		let max_sub_key_name_len = max_sub_key_name_len.map_or(std::ptr::null_mut(), |re| re as _);
		let max_class_len = max_class_len.map_or(std::ptr::null_mut(), |re| re as _);
		let num_values = num_values.map_or(std::ptr::null_mut(), |re| re as _);
		let max_value_name_len = max_value_name_len.map_or(std::ptr::null_mut(), |re| re as _);
		let max_value_len = max_value_len.map_or(std::ptr::null_mut(), |re| re as _);
		let security_descr_len = security_descr_len.map_or(std::ptr::null_mut(), |re| re as _);
		let last_write_time = last_write_time.map_or(std::ptr::null_mut(), |re| re as *mut _ as _);

		loop { // until class is large enough
			match unsafe {
				co::ERROR::from_raw(
					ffi::RegQueryInfoKeyW(
						self.ptr(),
						class_ptr,
						&mut class_len,
						std::ptr::null_mut(),
						num_sub_keys,
						max_sub_key_name_len,
						max_class_len,
						num_values,
						max_value_name_len,
						max_value_len,
						security_descr_len,
						last_write_time,
					) as _,
				 )
			} {
				co::ERROR::MORE_DATA => match &mut class {
					Some(class) => {
						**class = WString::new_alloc_buf(class.buf_len() * 2); // double the buffer size to try again
						class_ptr = unsafe { class.as_mut_ptr() };
						class_len = class.buf_len() as _;
					},
					None => return Err(co::ERROR::MORE_DATA),
				},
				co::ERROR::SUCCESS => return Ok(()),
				err => return Err(err),
			}
		}
	}

	/// [`RegQueryMultipleValues`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regquerymultiplevaluesw)
	/// function.
	///
	/// This method is a multi-value version of
	/// [`HKEY::RegQueryValueEx`](crate::prelude::kernel_Hkey::RegQueryValueEx).
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Desktop"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for val in hkey.RegQueryMultipleValues(&["DpiScalingVer", "WallPaper"])? {
	///     match val {
	///         w::RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///         w::RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///         w::RegistryValue::Sz(s) => println!("String: {}", s),
	///         w::RegistryValue::ExpandSz(s) => {
	///             println!("Env string: {}", w::ExpandEnvironmentStrings(&s)?);
	///         },
	///         w::RegistryValue::MultiSz(strs) => {
	///            println!("Multi string:");
	///            for s in strs.iter() {
	///                print!("[{}] ", s);
	///            }
	///            println!("");
	///         },
	///         w::RegistryValue::Binary(bin) => {
	///             println!("Binary:");
	///             for b in bin.iter() {
	///                 print!("{:02x} ", b);
	///             }
	///             println!("");
	///         },
	///         w::RegistryValue::None => println!("No value"),
	///     }
	/// }
	///
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegQueryMultipleValues(&self,
		value_names: &[impl AsRef<str>],
	) -> SysResult<Vec<RegistryValue>>
	{
		let mut valents1 = vec![VALENT::default(); value_names.len()];
		let value_names_w = value_names.iter()
			.zip(valents1.iter_mut())
			.map(|(value_name, valent)| {
				let value_name_w = WString::from_str(value_name.as_ref());
				valent.ve_valuename = value_name_w.as_ptr() as _;
				value_name_w
			})
			.collect::<Vec<_>>();
		let mut data_len1 = u32::default();

		// Query data types and lenghts.
		match unsafe {
			co::ERROR::from_raw(
				ffi::RegQueryMultipleValuesW(
					self.ptr(),
					valents1.as_mut_ptr() as _,
					value_names.len() as _,
					std::ptr::null_mut(),
					&mut data_len1,
				) as _,
			 )
		} {
			co::ERROR::MORE_DATA => {},
			err => return Err(err),
		}

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut valents2 = value_names_w.iter()
			.map(|value_name_w| {
				let mut valent = VALENT::default();
				valent.ve_valuename = value_name_w.as_ptr() as _;
				valent
			})
			.collect::<Vec<_>>();
		let mut data_len2 = data_len1;

		// Retrieve the values content.
		error_to_sysresult(
			unsafe {
				ffi::RegQueryMultipleValuesW(
					self.ptr(),
					valents2.as_mut_ptr() as _,
					value_names.len() as _,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			},
		)?;

		if data_len1 != data_len2 {
			// Race condition: someone modified the data content in between our calls.
			return Err(co::ERROR::TRANSACTION_REQUEST_NOT_VALID);
		}

		Ok(
			valents2.iter() // first VALENT array is not filled with len/type values
				.map(|v2| unsafe {
					RegistryValue::from_raw(
						v2.buf_projection(&buf).to_vec(),
						v2.ve_type,
					)
				})
				.collect::<Vec<_>>()
		)
	}

	/// [`RegQueryReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryreflectionkey)
	/// function.
	#[must_use]
	fn RegQueryReflectionKey(&self) -> SysResult<bool> {
		let mut is_disabled: BOOL = 0;
		error_to_sysresult(
			unsafe { ffi::RegQueryReflectionKey(self.ptr(), &mut is_disabled) },
		).map(|_| is_disabled != 0)
	}

	/// [`RegQueryValueEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryvalueexw)
	/// function.
	///
	/// This method is a single-value version of
	/// [`HKEY::RegQueryMultipleValues`](crate::prelude::kernel_Hkey::RegQueryMultipleValues).
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID)
	/// and [`co::ERROR::INVALID_DATA`](crate::co::ERROR::INVALID_DATA).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Mouse"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// let val = hkey.RegQueryValueEx(Some("Beep"))?;
	///
	/// match val {
	///     w::RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     w::RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     w::RegistryValue::Sz(s) => println!("String: {}", s),
	///     w::RegistryValue::ExpandSz(s) => {
	///         println!("Env string: {}", w::ExpandEnvironmentStrings(&s)?);
	///     },
	///     w::RegistryValue::MultiSz(strs) => {
	///        println!("Multi string:");
	///        for s in strs.iter() {
	///            print!("[{}] ", s);
	///        }
	///        println!("");
	///     },
	///     w::RegistryValue::Binary(bin) => {
	///         println!("Binary:");
	///         for b in bin.iter() {
	///             print!("{:02x} ", b);
	///         }
	///         println!("");
	///     },
	///     w::RegistryValue::None => println!("No value"),
	/// }
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegQueryValueEx(&self,
		value_name: Option<&str>,
	) -> SysResult<RegistryValue>
	{
		let value_name_w = WString::from_opt_str(value_name);
		let mut raw_data_type1 = u32::default();
		let mut data_len1 = u32::default();

		// Query data type and length.
		error_to_sysresult(
			unsafe {
				ffi::RegQueryValueExW(
					self.ptr(),
					value_name_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type1,
					std::ptr::null_mut(),
					&mut data_len1,
				)
			},
		)?;

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut raw_data_type2 = u32::default();
		let mut data_len2 = data_len1;

		// Retrieve the value content.
		error_to_sysresult(
			unsafe {
				ffi::RegQueryValueExW(
					self.ptr(),
					value_name_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type2,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			},
		)?;

		validate_retrieved_reg_val(
			unsafe { co::REG::from_raw(raw_data_type1) }, data_len1,
			unsafe { co::REG::from_raw(raw_data_type2) }, data_len2,
			buf,
		)
	}

	/// [`RegRenameKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrenamekey)
	/// function.
	fn RegRenameKey(&self,
		sub_key_name: &str,
		new_key_name: &str,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				ffi::RegRenameKey(
					self.ptr(),
					WString::from_str(sub_key_name).as_ptr(),
					WString::from_str(new_key_name).as_ptr(),
				)
			},
		)
	}

	/// [`RegReplaceKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regreplacekeyw)
	/// function.
	fn RegReplaceKey(&self,
		sub_key: Option<&str>,
		new_src_file: &str,
		old_file_backup: &str,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				ffi::RegReplaceKeyW(
					self.ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					WString::from_str(new_src_file).as_ptr(),
					WString::from_str(old_file_backup).as_ptr(),
				)
			},
		)
	}

	/// [`RegRestoreKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrestorekeyw)
	/// function.
	fn RegRestoreKey(&self,
		file_path: &str,
		flags: co::REG_RESTORE,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				ffi::RegRestoreKeyW(
					self.ptr(),
					WString::from_str(file_path).as_ptr(),
					flags.raw(),
				)
			},
		)
	}

	/// [`RegSaveKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyw)
	/// function.
	fn RegSaveKey(&self,
		dest_file_path: &str,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				ffi::RegSaveKeyW(
					self.ptr(),
					WString::from_str(dest_file_path).as_ptr(),
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
				)
			},
		)
	}

	/// [`RegSaveKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyexw)
	/// function.
	fn RegSaveKeyEx(&self,
		dest_file_path: &str,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		flags: co::REG_SAVE,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				ffi::RegSaveKeyExW(
					self.ptr(),
					WString::from_str(dest_file_path).as_ptr(),
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					flags.raw(),
				)
			},
		)
	}

	/// [`RegSetKeyValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetkeyvaluew)
	/// function.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will take over.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// w::HKEY::CURRENT_USER.RegSetKeyValue(
	///     Some("Software\\My Company"),
	///     Some("Color"),
	///     w::RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn RegSetKeyValue(&self,
		sub_key: Option<&str>,
		value_name: Option<&str>,
		data: RegistryValue,
	) -> SysResult<()>
	{
		let mut str_buf = WString::default();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		error_to_sysresult(
			unsafe {
				ffi::RegSetKeyValueW(
					self.ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					WString::from_opt_str(value_name).as_ptr(),
					data.reg_type().raw(),
					data_ptr,
					data_len,
				)
			},
		)
	}

	/// [`RegSetValueEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw)
	/// function.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will prevail.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Console\\Git Bash"),
	///     co::REG_OPTION::default(),
	///     co::KEY::ALL_ACCESS,
	/// )?;
	///
	/// hkey.RegSetValueEx(
	///     Some("Color"),
	///     w::RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn RegSetValueEx(&self,
		value_name: Option<&str>,
		data: RegistryValue,
	) -> SysResult<()>
	{
		let mut str_buf = WString::default();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		error_to_sysresult(
			unsafe {
				ffi::RegSetValueExW(
					self.ptr(),
					WString::from_opt_str(value_name).as_ptr(),
					0,
					data.reg_type().raw(),
					data_ptr as _,
					data_len,
				)
			},
		)
	}

	/// [`RegUnLoadKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regunloadkeyw)
	/// function.
	fn RegUnLoadKey(&self, sub_key: Option<&str>) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				ffi::RegUnLoadKeyW(
					self.ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
				)
			},
		)
	}
}

impl HKEY {
	pub(in crate::kernel) fn is_predef_key(&self) -> bool {
		// Note that we are not constructing HKEY objects, so no drop() is called.
		(self.0 as usize) >= (Self::CLASSES_ROOT.0 as usize)
			&& (self.0 as usize) <= (Self::PERFORMANCE_NLSTEXT.0 as usize)
	}
}

//------------------------------------------------------------------------------

fn validate_retrieved_reg_val(
	data_type1: co::REG,
	data_len1: u32,
	data_type2: co::REG,
	data_len2: u32,
	buf: Vec<u8>,
) -> SysResult<RegistryValue>
{
	if data_type1 != data_type2 {
		// Race condition: someone modified the data type in between our calls.
		return Err(co::ERROR::TRANSACTION_REQUEST_NOT_VALID);
	}

	if data_len1 != data_len2 {
		// Race condition: someone modified the data content in between our calls.
		return Err(co::ERROR::TRANSACTION_REQUEST_NOT_VALID);
	}

	if data_type1 == co::REG::DWORD && data_len1 != 4
		|| data_type1 == co::REG::QWORD && data_len1 != 8
	{
		// Data length makes no sense, possibly corrupted.
		return Err(co::ERROR::INVALID_DATA);
	}

	if data_type1 == co::REG::SZ {
		if data_len1 == 0 // empty data
			|| data_len1 % 2 != 0 // odd number of bytes
			|| buf[data_len1 as usize - 2] != 0 // terminating null
			|| buf[data_len1 as usize - 1] != 0
		{
			// Bad string.
			return Err(co::ERROR::INVALID_DATA);
		}
	}

	Ok(unsafe { RegistryValue::from_raw(buf, data_type1) })
}
