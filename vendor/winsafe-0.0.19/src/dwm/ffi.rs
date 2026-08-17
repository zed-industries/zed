use crate::kernel::ffi_types::*;

extern_sys! { "dwmapi";
	DwmEnableMMCSS(BOOL) -> HRES
	DwmExtendFrameIntoClientArea(HANDLE, PCVOID) -> HRES
	DwmFlush() -> HRES
	DwmGetColorizationColor(*mut u32, *mut BOOL) -> HRES
	DwmInvalidateIconicBitmaps(HANDLE) -> HRES
	DwmIsCompositionEnabled(*mut BOOL) -> HRES
	DwmSetIconicLivePreviewBitmap(HANDLE, HANDLE, PCVOID, u32) -> HRES
	DwmSetIconicThumbnail(HANDLE, HANDLE, u32) -> HRES
}
