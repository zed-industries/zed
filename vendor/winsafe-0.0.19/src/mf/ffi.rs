use crate::kernel::ffi_types::*;

extern_sys! { "mf";
	MFCreateMediaSession(COMPTR, *mut COMPTR) -> HRES
	MFCreateTopology(*mut COMPTR) -> HRES
	MFCreateTopologyNode(u32, *mut COMPTR) -> HRES
}

extern_sys! { "mfplat";
	MFCreateAsyncResult(COMPTR, COMPTR, COMPTR, *mut COMPTR) -> HRES
	MFCreateSourceResolver(*mut COMPTR) -> HRES
	MFStartup(u32, u32) -> HRES
}
