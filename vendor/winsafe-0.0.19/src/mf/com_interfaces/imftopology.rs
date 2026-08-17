#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFTopology`](crate::IMFTopology) virtual table.
#[repr(C)]
pub struct IMFTopologyVT {
	pub IMFAttributesVT: IMFAttributesVT,
	pub GetTopologyID: fn(COMPTR, *mut u64) -> HRES,
	pub AddNode: fn(COMPTR, COMPTR) -> HRES,
	pub RemoveNode: fn(COMPTR, COMPTR) -> HRES,
	pub GetNodeCount: fn(COMPTR, *mut u16) -> HRES,
	pub GetNode: fn(COMPTR, u16, *mut COMPTR) -> HRES,
	pub Clear: fn(COMPTR) -> HRES,
	pub CloneFrom: fn(COMPTR, COMPTR) -> HRES,
	pub GetNodeByID: fn(COMPTR, u64, *mut COMPTR) -> HRES,
	pub GetSourceNodeCollection: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetOutputNodeCollection: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IMFTopology: "83cf873a-f6da-4bc8-823f-bacfd55dc433";
	/// [`IMFTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imftopology)
	/// COM interface over [`IMFTopologyVT`](crate::vt::IMFTopologyVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with [`MFCreateTopology`](crate::MFCreateTopology)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let topology = w::MFCreateTopology()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl mf_IMFAttributes for IMFTopology {}
impl mf_IMFTopology for IMFTopology {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFTopology`](crate::IMFTopology).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFTopology: mf_IMFAttributes {
	/// [`IMFTopology::AddNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-addnode)
	/// method.
	fn AddNode(&self, node: &impl mf_IMFTopologyNode) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IMFTopologyVT>(self).AddNode)(self.ptr(), node.ptr()) },
		)
	}

	fn_com_noparm! { Clear: IMFTopologyVT;
		/// [`IMFTopology::Clear`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-clear)
		/// method.
	}

	/// [`IMFTopology::CloneFrom`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-clonefrom)
	/// method.
	fn CloneFrom(&self, topology: &impl mf_IMFTopology) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).CloneFrom)(self.ptr(), topology.ptr())
			},
		)
	}

	/// [`IMFTopology::GetNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getnode)
	/// method.
	#[must_use]
	fn GetNode(&self, index: u16) -> HrResult<IMFTopologyNode> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).GetNode)(
					self.ptr(),
					index,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFTopology::GetNodeByID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getnodebyid)
	/// method.
	#[must_use]
	fn GetNodeByID(&self, topo_node_id: u64) -> HrResult<IMFTopologyNode> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).GetNodeByID)(
					self.ptr(),
					topo_node_id,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFTopology::GetNodeCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-getnodecount)
	/// method.
	#[must_use]
	fn GetNodeCount(&self) -> HrResult<u16> {
		let mut nodes = u16::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).GetNodeCount)(self.ptr(), &mut nodes)
			},
		).map(|_| nodes)
	}

	/// [`IMFTopology::GetTopologyID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-gettopologyid)
	/// method.
	#[must_use]
	fn GetTopologyID(&self) -> HrResult<u64> {
		let mut id = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).GetTopologyID)(self.ptr(), &mut id)
			},
		).map(|_| id)
	}

	/// [`IMFTopology::RemoveNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopology-removenode)
	/// method.
	fn RemoveNode(&self, node: &impl mf_IMFTopologyNode) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyVT>(self).RemoveNode)(self.ptr(), node.ptr())
			},
		)
	}
}
