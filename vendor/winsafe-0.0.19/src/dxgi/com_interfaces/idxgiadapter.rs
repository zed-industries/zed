#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::dxgi::iterators::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDXGIAdapter`](crate::IDXGIAdapter) virtual table.
#[repr(C)]
pub struct IDXGIAdapterVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub EnumOutputs: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub CheckInterfaceSupport: fn(COMPTR, PCVOID, *mut i64) -> HRES,
}

com_interface! { IDXGIAdapter: "2411e7e1-12ac-4ccf-bd14-9798e8534dc0";
	/// [`IDXGIAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiadapter)
	/// COM interface over [`IDXGIAdapterVT`](crate::vt::IDXGIAdapterVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIAdapter {}
impl dxgi_IDXGIAdapter for IDXGIAdapter {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIAdapter`](crate::IDXGIAdapter).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIAdapter: dxgi_IDXGIObject {
	/// [`IDXGIAdapter::CheckInterfaceSupport`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-checkinterfacesupport)
	/// method.
	#[must_use]
	fn CheckInterfaceSupport(&self, interface_name: &GUID) -> HrResult<i64> {
		let mut umd_ver = i64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIAdapterVT>(self).CheckInterfaceSupport)(
					self.ptr(),
					interface_name as *const _ as _,
					&mut umd_ver,
				)
			},
		).map(|_| umd_ver)
	}

	/// [`IDXGIAdapter::EnumOutputs`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-enumoutputs)
	/// method.
	///
	/// Returns an iterator over [`IDXGIOutput`](crate::IDXGIOutput) elements.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let adapter: w::IDXGIAdapter; // initialized somewhere
	/// # let adapter = unsafe { w::IDXGIAdapter::null() };
	///
	/// for output in adapter.EnumOutputs() {
	///     let output = output?;
	///     // ...
	/// }
	///
	/// // Collecting into a Vec
	/// let outputs: Vec<w::IDXGIOutput> =
	///     adapter.EnumOutputs()
	///         .collect::<w::HrResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn EnumOutputs(&self,
	) -> Box<dyn Iterator<Item = HrResult<IDXGIOutput>> + '_>
	{
		Box::new(IdxgiadapterEnumoutputsIter::new(self))
	}

	/// [`IDXGIAdapter::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter-getdesc)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let adapter: w::IDXGIAdapter; // initialized somewhere
	/// # let adapter = unsafe { w::IDXGIAdapter::null() };
	/// let mut desc = w::DXGI_ADAPTER_DESC::default();
	///
	/// adapter.GetDesc(&mut desc)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	fn GetDesc(&self, desc: &mut DXGI_ADAPTER_DESC) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIAdapterVT>(self).GetDesc)(
					self.ptr(),
					desc as *mut _ as _,
				)
			},
		)
	}
}
