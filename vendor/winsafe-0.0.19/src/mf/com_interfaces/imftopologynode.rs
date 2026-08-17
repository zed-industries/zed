#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFTopologyNode`](crate::IMFTopologyNode) virtual table.
#[repr(C)]
pub struct IMFTopologyNodeVT {
	pub IMFAttributesVT: IMFAttributesVT,
	pub SetObject: fn(COMPTR, COMPTR) -> HRES,
	pub GetObject: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetNodeType: fn(COMPTR, *mut u32) -> HRES,
	pub GetTopoNodeID: fn(COMPTR,*mut u64) -> HRES,
	pub SetTopoNodeID: fn(COMPTR, u64) -> HRES,
	pub GetInputCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetOutputCount: fn(COMPTR, *mut u32) -> HRES,
	pub ConnectOutput: fn(COMPTR, u32, COMPTR, u32) -> HRES,
	pub DisconnectOutput: fn(COMPTR, u32) -> HRES,
	pub GetInput: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub GetOutput: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub SetOutputPrefType: fn(COMPTR, u32, COMPTR) -> HRES,
	pub GetOutputPrefType: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub SetInputPrefType: fn(COMPTR, u32, COMPTR) -> HRES,
	pub GetInputPrefType: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub CloneFrom: fn(COMPTR, COMPTR) -> HRES,
}

com_interface! { IMFTopologyNode: "83cf873a-f6da-4bc8-823f-bacfd55dc430";
	/// [`IMFTopologyNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imftopologynode)
	/// COM interface over
	/// [`IMFTopologyNodeVT`](crate::vt::IMFTopologyNodeVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`MFCreateTopologyNode`](crate::MFCreateTopologyNode) function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let topology_node = w::MFCreateTopologyNode(co::MF_TOPOLOGY::OUTPUT_NODE)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl mf_IMFAttributes for IMFTopologyNode {}
impl mf_IMFTopologyNode for IMFTopologyNode {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFTopologyNode`](crate::IMFTopologyNode).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFTopologyNode: mf_IMFAttributes {
	/// [`IMFTopologyNode::CloneFrom`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-clonefrom)
	/// method.
	fn CloneFrom(&self, node: &impl mf_IMFTopologyNode) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).CloneFrom)(self.ptr(), node.ptr())
			},
		)
	}

	/// [`IMFTopologyNode::ConnectOutput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-connectoutput)
	/// method.
	fn ConnectOutput(&self,
		output_index: u32,
		downstream_node: &impl mf_IMFTopologyNode,
		input_index_on_downstream_node: u32,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).ConnectOutput)(
					self.ptr(),
					output_index,
					downstream_node.ptr(),
					input_index_on_downstream_node,
				)
			},
		)
	}

	/// [`IMFTopologyNode::DisconnectOutput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-disconnectoutput)
	/// method.
	#[must_use]
	fn DisconnectOutput(&self, output_index: u32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).DisconnectOutput)(
					self.ptr(),
					output_index,
				)
			},
		)
	}

	/// [`IMFTopologyNode::GetInput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getinput)
	/// method.
	///
	/// Returns the node and the index of the output stream that is connected to
	/// this node's input stream.
	#[must_use]
	fn GetInput(&self, input_index: u32) -> HrResult<(IMFTopologyNode, u32)> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		let mut output_index_downstream_node = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).GetInput)(
					self.ptr(),
					input_index,
					queried.as_mut(),
					&mut output_index_downstream_node,
				)
			},
		).map(|_| (queried, output_index_downstream_node))
	}

	/// [`IMFTopologyNode::GetInputCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getinputcount)
	/// method.
	#[must_use]
	fn GetInputCount(&self) -> HrResult<u32> {
		let mut c = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).GetInputCount)(self.ptr(), &mut c)
			},
		).map(|_| c)
	}

	/// [`IMFTopologyNode::GetNodeType`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getnodetype)
	/// method.
	#[must_use]
	fn GetNodeType(&self) -> HrResult<co::MF_TOPOLOGY> {
		let mut ty = co::MF_TOPOLOGY::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).GetNodeType)(self.ptr(), ty.as_mut())
			},
		).map(|_| ty)
	}

	/// [`IMFTopologyNode::GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getobject)
	/// method.
	#[must_use]
	fn GetObject<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).GetObject)(
					self.ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFTopologyNode::GetOutput`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getoutput)
	/// method.
	///
	/// Returns the node and the index of the input stream that is connected to
	/// this node's output stream.
	#[must_use]
	fn GetOutput(&self, output_index: u32) -> HrResult<(IMFTopologyNode, u32)> {
		let mut queried = unsafe { IMFTopologyNode::null() };
		let mut input_index_downstream_node = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).GetOutput)(
					self.ptr(),
					output_index,
					queried.as_mut(),
					&mut input_index_downstream_node,
				)
			},
		).map(|_| (queried, input_index_downstream_node))
	}

	/// [`IMFTopologyNode::GetOutputCount`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-getoutputcount)
	/// method.
	#[must_use]
	fn GetOutputCount(&self) -> HrResult<u32> {
		let mut c = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).GetOutputCount)(self.ptr(), &mut c)
			},
		).map(|_| c)
	}

	/// [`IMFTopologyNode::GetTopoNodeID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-gettoponodeid)
	/// method.
	#[must_use]
	fn GetTopoNodeID(&self) -> HrResult<u64> {
		let mut id = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).GetTopoNodeID)(self.ptr(), &mut id)
			},
		).map(|_| id)
	}

	/// [`IMFTopologyNode::SetObject`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-setobject)
	/// method
	fn SetObject(&self, object: &impl ole_IUnknown) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).SetObject)(self.ptr(), object.ptr())
			},
		)
	}

	/// [`IMFTopologyNode::SetTopoNodeID`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imftopologynode-settoponodeid)
	/// method.
	fn SetTopoNodeID(&self, topo_id: u64) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFTopologyNodeVT>(self).SetTopoNodeID)(self.ptr(), topo_id)
			},
		)
	}
}
