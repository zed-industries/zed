#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::oleaut::iterators::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IPropertyStore`](crate::IPropertyStore) virtual table.
#[repr(C)]
pub struct IPropertyStoreVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetAt: fn(COMPTR, u32, PVOID) -> HRES,
	pub GetValue: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub SetValue: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub Commit: fn(COMPTR) -> HRES,
}

com_interface! { IPropertyStore: "886d8eeb-8cf2-4446-8d02-cdba1dbdcf99";
	/// [`IPropertyStore`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nn-propsys-ipropertystore)
	/// COM interface over [`IPropertyStoreVT`](crate::vt::IPropertyStoreVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually, this interface is taken via
	/// [`IShellItem::BindToHandler`](crate::prelude::shell_IShellItem::BindToHandler).
}

impl oleaut_IPropertyStore for IPropertyStore {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`IPropertyStore`](crate::IPropertyStore).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_IPropertyStore: ole_IUnknown {
	/// Returns an iterator over the [`PROPERTYKEY`](crate::PROPERTYKEY)
	/// elements by calling
	/// [`IPropertyStore::GetCount`](crate::prelude::oleaut_IPropertyStore::GetCount)
	/// and
	/// [`IPropertyStore::GetAt`](crate::prelude::oleaut_IPropertyStore::GetAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let pstore: w::IPropertyStore; // initialized somewhere
	/// # let pstore = unsafe { w::IPropertyStore::null() };
	///
	/// for ppk in pstore.iter()? {
	///     let ppk = ppk?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self,
	) -> HrResult<Box<dyn Iterator<Item = HrResult<PROPERTYKEY>> + '_>>
	{
		Ok(Box::new(IpropertystoreIter::new(self)?))
	}

	fn_com_noparm! { Commit: IPropertyStoreVT;
		/// [`IPropertyStore::Commit`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-commit)
		/// method.
	}

	/// [`IPropertyStore::GetAt`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getat)
	/// method.
	#[must_use]
	fn GetAt(&self, index: u32) -> HrResult<PROPERTYKEY> {
		let mut ppk = PROPERTYKEY::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IPropertyStoreVT>(self).GetAt)(
					self.ptr(),
					index,
					&mut ppk as *const _ as _,
				)
			},
		).map(|_| ppk)
	}

	/// [`IPropertyStore::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IPropertyStoreVT>(self).GetCount)(self.ptr(), &mut count)
			},
		).map(|_| count)
	}

	/// [`IPropertyStore::GetValue`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getvalue)
	/// method.
	#[must_use]
	fn GetValue(&self, key: &PROPERTYKEY) -> HrResult<PROPVARIANT> {
		let mut var = PROPVARIANT::default();
		match unsafe {
			co::HRESULT::from_raw(
				(vt::<IPropertyStoreVT>(self).GetValue)(
					self.ptr(),
					key as *const _ as _,
					&mut var as *mut _ as _,
				),
			)
		 } {
			co::HRESULT::S_OK
				| co::HRESULT::INPLACE_S_TRUNCATED => Ok(var),
			hr => Err(hr),
		}
	}
}
