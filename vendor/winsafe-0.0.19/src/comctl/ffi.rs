use crate::kernel::ffi_types::*;

extern_sys! { "comctl32";
	DefSubclassProc(HANDLE, u32, usize, isize) -> isize
	ImageList_Add(HANDLE, HANDLE, HANDLE) -> i32
	ImageList_AddMasked(HANDLE, HANDLE, u32) -> i32
	ImageList_BeginDrag(HANDLE, i32, i32, i32) -> BOOL
	ImageList_Create(i32, i32, u32, i32, i32) -> HANDLE
	ImageList_Destroy(HANDLE) -> BOOL
	ImageList_DragMove(HANDLE, i32, i32) -> BOOL
	ImageList_DragShowNolock(BOOL) -> BOOL
	ImageList_EndDrag()
	ImageList_GetIconSize(HANDLE, *mut i32, *mut i32) -> BOOL
	ImageList_GetImageCount(HANDLE) -> i32
	ImageList_Remove(HANDLE, i32) -> BOOL
	ImageList_ReplaceIcon(HANDLE, i32, HANDLE) -> i32
	ImageList_SetImageCount(HANDLE, u32) -> BOOL
	InitCommonControls()
	InitCommonControlsEx(PVOID) -> BOOL
	InitializeFlatSB(HANDLE) -> HRES
	InitMUILanguage(u16)
	RemoveWindowSubclass(HANDLE, PFUNC, usize) -> BOOL
	SetWindowSubclass(HANDLE, PFUNC, usize, usize) -> BOOL
	TaskDialog(HANDLE, HANDLE, PCSTR, PCSTR, PCSTR, i32, PCSTR, *mut i32) -> HRES
	TaskDialogIndirect(PCVOID, *mut i32, *mut i32, *mut BOOL) -> HRES
	UninitializeFlatSB(HANDLE) -> HRES
}
