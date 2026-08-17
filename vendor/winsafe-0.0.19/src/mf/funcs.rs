#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::mf::{ffi, privs::*};
use crate::ole::privs::*;
use crate::prelude::*;

/// [`MFCreateAsyncResult`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfcreateasyncresult)
/// function.
#[must_use]
pub fn MFCreateAsyncResult(
	object: Option<&impl ole_IUnknown>,
	callback: &impl mf_IMFAsyncCallback,
	state: Option<&impl ole_IUnknown>,
) -> HrResult<IMFAsyncResult>
{
	let mut queried = unsafe { IMFAsyncResult::null() };
	ok_to_hrresult(
		unsafe {
			ffi::MFCreateAsyncResult(
				object.map_or(std::ptr::null_mut(), |o| o.ptr()),
				callback.ptr(),
				state.map_or(std::ptr::null_mut(), |s| s.ptr()),
				queried.as_mut(),
			)
		},
	).map(|_| queried)
}

/// [`MFCreateMediaSession`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatemediasession)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let media_session = w::MFCreateMediaSession(None::<&w::IMFAttributes>)?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn MFCreateMediaSession(
	configuration: Option<&impl mf_IMFAttributes>,
) -> HrResult<IMFMediaSession>
{
	let mut queried = unsafe { IMFMediaSession::null() };
	ok_to_hrresult(
		unsafe {
			ffi::MFCreateMediaSession(
				configuration.map_or(std::ptr::null_mut(), |c| c.ptr()),
				queried.as_mut(),
			)
		},
	).map(|_| queried)
}

/// [`MFCreateSourceResolver`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatesourceresolver)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let source_resolver = w::MFCreateSourceResolver()?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn MFCreateSourceResolver() -> HrResult<IMFSourceResolver> {
	let mut queried = unsafe { IMFSourceResolver::null() };
	ok_to_hrresult(unsafe { ffi::MFCreateSourceResolver(queried.as_mut()) })
		.map(|_| queried)
}

/// [`MFCreateTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopology)
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
#[must_use]
pub fn MFCreateTopology() -> HrResult<IMFTopology> {
	let mut queried = unsafe { IMFTopology::null() };
	ok_to_hrresult(unsafe { ffi::MFCreateTopology(queried.as_mut()) })
		.map(|_| queried)
}

/// [`MFCreateTopologyNode`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-mfcreatetopologynode)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let topology_node = w::MFCreateTopologyNode(co::MF_TOPOLOGY::OUTPUT_NODE)?;
/// # Ok::<_, winsafe::co::HRESULT>(())
/// ```
#[must_use]
pub fn MFCreateTopologyNode(
	node_type: co::MF_TOPOLOGY,
) -> HrResult<IMFTopologyNode>
{
	let mut queried = unsafe { IMFTopologyNode::null() };
	ok_to_hrresult(
		unsafe { ffi::MFCreateTopologyNode(node_type.raw(), queried.as_mut()) },
	).map(|_| queried)
}

/// [`MFStartup`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfstartup)
/// function.
pub fn MFStartup(flags: co::MFSTARTUP) -> HrResult<()> {
	ok_to_hrresult(unsafe { ffi::MFStartup(MF_VERSION, flags.raw()) })
}
