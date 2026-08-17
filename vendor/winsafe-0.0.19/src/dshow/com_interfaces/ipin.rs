#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IPin`](crate::IPin) virtual table.
#[repr(C)]
pub struct IPinVT {
	pub IUnknownVT: IUnknownVT,
	pub Connect: fn(COMPTR, COMPTR, PCVOID) -> HRES,
	pub ReceiveConnection: fn(COMPTR, COMPTR, PCVOID) -> HRES,
	pub Disconnect: fn(COMPTR) -> HRES,
	pub ConnectedTo: fn(COMPTR, *mut COMPTR) -> HRES,
	pub ConnectionMediaType: fn(COMPTR, PVOID) -> HRES,
	pub QueryPinInfo: fn(COMPTR, PVOID) -> HRES,
	pub QueryDirection: fn(COMPTR, PVOID) -> HRES,
	pub QueryId: fn(COMPTR, *mut PSTR) -> HRES,
	pub QueryAccept: fn(COMPTR, PCVOID) -> HRES,
	pub EnumMediaTypes: fn(COMPTR, *mut COMPTR) -> HRES,
	pub QueryInternalConnections: fn(COMPTR, *mut COMPTR, *mut u32) -> HRES,
	pub EndOfStream: fn(COMPTR) -> HRES,
	pub BeginFlush: fn(COMPTR) -> HRES,
	pub EndFlush: fn(COMPTR) -> HRES,
	pub NewSegment: fn(COMPTR, i64, i64, f64) -> HRES,
}

com_interface! { IPin: "56a86891-0ad4-11ce-b03a-0020af0ba770";
	/// [`IPin`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ipin)
	/// COM interface over [`IPinVT`](crate::vt::IPinVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IPin for IPin {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IPin`](crate::IPin).
pub trait dshow_IPin: ole_IUnknown {
	fn_com_noparm! { BeginFlush: IPinVT;
		/// [`IPin::BeginFlush`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-beginflush)
		/// method.
	}

	/// [`IPin::Connect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connect)
	/// method.
	fn Connect(&self,
		receive_pin: &impl dshow_IPin,
		mt: Option<&AM_MEDIA_TYPE>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).Connect)(
					self.ptr(),
					receive_pin.ptr(),
					mt.map_or(std::ptr::null(), |amt| amt as *const _ as _),
				)
			},
		)
	}

	fn_com_interface_get! { ConnectedTo: IPinVT, IPin;
		/// [`IPin::ConnectedTo`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectedto)
		/// method.
	}

	/// [`IPin::ConnectionMediaType`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-connectionmediatype)
	/// method.
	fn ConnectionMediaType(&self, amt: &mut AM_MEDIA_TYPE) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).ConnectionMediaType)(
					self.ptr(),
					amt as *mut _ as _,
				)
			},
		)
	}

	fn_com_noparm! { Disconnect: IPinVT;
		/// [`IPin::Disconnect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-disconnect)
		/// method.
	}

	fn_com_noparm! { EndFlush: IPinVT;
		/// [`IPin::EndFlush`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endflush)
		/// method.
	}

	fn_com_noparm! { EndOfStream: IPinVT;
		/// [`IPin::EndOfStream`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-endofstream)
		/// method.
	}

	fn_com_interface_get! { EnumMediaTypes: IPinVT, IEnumMediaTypes;
		/// [`IPin::EnumMediaTypes`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-enummediatypes)
		/// method.
	}

	/// [`IPin::NewSegment`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-newsegment)
	/// method.
	fn NewSegment(&self, start: i64, stop: i64, rate: f64) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).NewSegment)(self.ptr(), start, stop, rate)
			},
		)
	}

	/// [`IPin::QueryAccept`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryaccept)
	/// method.
	#[must_use]
	fn QueryAccept(&self, amt: &AM_MEDIA_TYPE) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).QueryAccept)(self.ptr(), amt as *const _ as _)
			},
		)
	}

	/// [`IPin::QueryDirection`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-querydirection)
	/// method.
	#[must_use]
	fn QueryDirection(&self) -> HrResult<co::PIN_DIRECTION> {
		let mut pin_dir = co::PIN_DIRECTION::INPUT;
		ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).QueryDirection)(
					self.ptr(),
					&mut pin_dir as *mut _ as _,
				)
			},
		).map(|_| pin_dir)
	}

	/// [`IPin::QueryId`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryid)
	/// method.
	#[must_use]
	fn QueryId(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		ok_to_hrresult(
			unsafe { (vt::<IPinVT>(self).QueryId)(self.ptr(), &mut pstr) },
		).map(|_| {
			let name = unsafe { WString::from_wchars_nullt(pstr) };
			let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) };
			name.to_string()
		})
	}

	/// [`IPin::QueryInternalConnections`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-queryinternalconnections)
	/// method.
	#[must_use]
	fn QueryInternalConnections(&self) -> HrResult<Vec<IPin>> {
		let mut count = u32::default();
		if let Err(e) = ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).QueryInternalConnections)(
					self.ptr(),
					std::ptr::null_mut(),
					&mut count as *mut _ as _,
				)
			},
		) {
			return Err(e);
		}

		let mut queried = vec![unsafe { IPin::null() }];
		ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).QueryInternalConnections)(
					self.ptr(),
					queried.as_mut_ptr() as _,
					&mut count as *mut _ as _,
				)
			},
		).map(|_| queried)
	}

	/// [`IPin::QueryPinInfo`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-querypininfo)
	/// method.
	fn QueryPinInfo(&self, info: &mut PIN_INFO) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).QueryPinInfo)(self.ptr(), info as *mut _ as _)
			},
		)
	}

	/// [`IPin::ReceiveConnection`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ipin-receiveconnection)
	/// method.
	fn ReceiveConnection(&self,
		connector: &impl dshow_IPin,
		mt: &AM_MEDIA_TYPE,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IPinVT>(self).ReceiveConnection)(
					self.ptr(),
					connector.ptr(),
					mt as *const _ as _,
				)
			},
		)
	}
}
