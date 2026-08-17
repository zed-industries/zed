#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::shell::ffi;

impl shell_Hwnd for HWND {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_Hwnd: ole_Hwnd {
	/// [`DragAcceptFiles`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragacceptfiles)
	/// function.
	fn DragAcceptFiles(&self, accept: bool) {
		unsafe { ffi::DragAcceptFiles(self.ptr(), accept as _); }
	}

	/// [`ShellAbout`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellaboutw)
	/// function.
	fn ShellAbout(&self,
		title_bar: &str,
		first_line: Option<&str>,
		other_stuff: Option<&str>,
		hicon: Option<&HICON>,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::ShellAboutW(
					self.ptr(),
					WString::from_str(
						&match first_line {
							Some(line) => format!("{}#{}", title_bar, line),
							None => title_bar.to_owned(),
						},
					).as_ptr(),
					WString::from_opt_str(other_stuff).as_ptr(),
					hicon.map_or(std::ptr::null_mut(), |h| h.ptr()),
				)
			},
		)
	}

	/// [`ShellExecute`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew)
	/// function.
	fn ShellExecute(&self,
		operation: &str,
		file: &str,
		parameters: Option<&str>,
		directory: Option<&str>,
		show_cmd: co::SW,
	) -> Result<HINSTANCE, co::SE_ERR>
	{
		let ret = unsafe {
			ffi::ShellExecuteW(
				self.ptr(),
				WString::from_str(operation).as_ptr(),
				WString::from_str(file).as_ptr(),
				parameters.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				directory.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				show_cmd.raw(),
			)
		};

		unsafe {
			if ret <= 32 as _ {
				Err(co::SE_ERR::from_raw(ret as _))
			} else {
				Ok(HINSTANCE::from_ptr(ret as _))
			}
		}
	}
}
