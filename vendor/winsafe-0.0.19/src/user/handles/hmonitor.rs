#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

impl_handle! { HMONITOR;
	/// Handle to a
	/// [display monitor](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor).
}

impl user_Hmonitor for HMONITOR {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HMONITOR`](crate::HMONITOR).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hmonitor: Handle {
	/// [`GetMonitorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hmon: w::HMONITOR; // initialized somewhere
	/// # let hmon = w::HMONITOR::NULL;
	///
	/// let mut mi = w::MONITORINFOEX::default();
	/// hmon.GetMonitorInfo(&mut mi)?;
	///
	/// println!("{}", mi.szDevice());
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetMonitorInfo(&self, mi: &mut MONITORINFOEX) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::GetMonitorInfoW(self.ptr(), mi as *mut _ as _) },
		)
	}

	/// [`MonitorFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint)
	/// function.
	#[must_use]
	fn MonitorFromPoint(pt: POINT, flags: co::MONITOR) -> HMONITOR {
		unsafe {
			HMONITOR::from_ptr(ffi::MonitorFromPoint(pt.x, pt.y, flags.raw()))
		}
	}

	/// [`MonitorFromRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromrect)
	/// function.
	#[must_use]
	fn MonitorFromRect(rc: RECT, flags: co::MONITOR) -> HMONITOR {
		unsafe {
			HMONITOR::from_ptr(
				ffi::MonitorFromRect(&rc as *const _ as _, flags.raw()),
			)
		}
	}
}
