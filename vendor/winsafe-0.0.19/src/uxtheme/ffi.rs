use crate::kernel::ffi_types::*;

#[cfg(target_pointer_width = "64")]
extern_sys! { "uxtheme";
	IsThemeDialogTextureEnabled() -> BOOL
}

extern_sys! { "uxtheme";
	CloseThemeData(HANDLE) -> HRES
	DrawThemeBackground(HANDLE, HANDLE, i32, i32, PCVOID, PCVOID) -> HRES
	GetThemeAppProperties() -> u32
	GetThemeBackgroundContentRect(HANDLE, HANDLE, i32, i32, PCVOID, PVOID) -> HRES
	GetThemeBackgroundExtent(HANDLE, HANDLE, i32, i32, PCVOID, PVOID) -> HRES
	GetThemeBackgroundRegion(HANDLE, HANDLE, i32, i32, PCVOID, *mut HANDLE) -> HRES
	GetThemeColor(HANDLE, i32, i32, i32, *mut u32) -> HRES
	GetThemeMargins(HANDLE, HANDLE, i32, i32, i32, PCVOID, PVOID) -> HRES
	GetThemeMetric(HANDLE, HANDLE, i32, i32, i32, *mut i32) -> HRES
	GetThemePartSize(HANDLE, HANDLE, i32, i32, PCVOID, u32, PVOID) -> HRES
	GetThemePosition(HANDLE, i32, i32, i32, PVOID) -> HRES
	GetThemePropertyOrigin(HANDLE, i32, i32, i32, *mut u32) -> HRES
	GetThemeRect(HANDLE, i32, i32, i32, PVOID) -> HRES
	IsAppThemed() -> BOOL
	IsCompositionActive() -> BOOL
	IsThemeActive() -> BOOL
	IsThemeBackgroundPartiallyTransparent(HANDLE, i32, i32) -> BOOL
	IsThemePartDefined(HANDLE, i32, i32) -> BOOL
	OpenThemeData(HANDLE, PCSTR) -> HANDLE
}
