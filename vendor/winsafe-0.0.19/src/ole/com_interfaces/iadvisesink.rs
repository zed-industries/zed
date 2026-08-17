#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IAdviseSink`](crate::IAdviseSink) virtual table.
#[repr(C)]
pub struct IAdviseSinkVT {
	pub IUnknownVT: IUnknownVT,
	pub OnDataChange: fn(COMPTR, PVOID, PVOID),
	pub OnViewChange: fn(COMPTR, u32, i32),
	pub OnRename: fn(COMPTR, COMPTR),
	pub OnSave: fn(COMPTR),
	pub OnClose: fn(COMPTR),
}

com_interface! { IAdviseSink: "0000010f-0000-0000-c000-000000000046";
	/// [`IAdviseSink`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-iadvisesink)
	/// COM interface over [`IAdviseSinkVT`](crate::vt::IAdviseSinkVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IAdviseSink for IAdviseSink {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IAdviseSink`](crate::IAdviseSink).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IAdviseSink: ole_IUnknown {
	fn_com_noparm_noret! { OnClose: IAdviseSinkVT;
		/// [`IAdviseSink::OnClose`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-iadvisesink-onclose)
		/// method.
	}

	/// [`IAdviseSink::OnRename`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-iadvisesink-onrename)
	/// method.
	fn OnRename(&self, mk: &impl ole_IMoniker) {
		unsafe { (vt::<IAdviseSinkVT>(self).OnRename)(self.ptr(), mk.ptr()); }
	}

	fn_com_noparm_noret! { OnSave: IAdviseSinkVT;
		/// [`IAdviseSink::OnSave`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-iadvisesink-onsave)
		/// method.
	}

	/// [`IAdviseSink::OnViewChange`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-iadvisesink-onviewchange)
	/// method.
	fn OnViewChange(&self, aspect: co::DVASPECT, index: i32) {
		unsafe {
			(vt::<IAdviseSinkVT>(self).OnViewChange)(
				self.ptr(),
				aspect.raw(),
				index,
			);
		}
	}
}
