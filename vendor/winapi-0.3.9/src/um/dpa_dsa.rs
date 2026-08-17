// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::basetsd::INT_PTR;
use shared::minwindef::{BOOL, DWORD, LPARAM, UINT};
use um::winnt::{HANDLE, HRESULT, LPCWSTR, LPWSTR, PVOID, ULONGLONG};
pub const DA_LAST: c_int = 0x7FFFFFFF;
pub const DA_ERR: c_int = -1;
FN!{stdcall PFNDAENUMCALLBACK(
    p: *mut c_void,
    pData: *mut c_void,
) -> c_int}
FN!{stdcall PFNDAENUMCALLBACKCONST(
    p: *const c_void,
    pData: *mut c_void,
) -> c_int}
FN!{stdcall PFNDACOMPARE(
    p1: *mut c_void,
    p2: *mut c_void,
    lParam: LPARAM,
) -> c_int}
FN!{stdcall PFNDACOMPARECONST(
    p1: *const c_void,
    p2: *const c_void,
    lParam: LPARAM,
) -> c_int}
pub enum DSA {}
pub type HDSA = *mut DSA;
extern "system" {
    pub fn DSA_Create(
        cbItem: c_int,
        cItemGrow: c_int,
    ) -> HDSA;
    pub fn DSA_Destroy(
        hdsa: HDSA,
    ) -> BOOL;
    pub fn DSA_DestroyCallback(
        hdsa: HDSA,
        pfnCB: PFNDAENUMCALLBACK,
        pData: *mut c_void,
    );
    pub fn DSA_DeleteItem(
        hdsa: HDSA,
        i: c_int,
    ) -> BOOL;
    pub fn DSA_DeleteAllItems(
        hdsa: HDSA,
    ) -> BOOL;
    pub fn DSA_EnumCallback(
        hdsa: HDSA,
        pfnCB: PFNDAENUMCALLBACK,
        pData: *mut c_void,
    );
    pub fn DSA_InsertItem(
        hdsa: HDSA,
        i: c_int,
        pitem: *const c_void,
    ) -> c_int;
    pub fn DSA_GetItemPtr(
        hdsa: HDSA,
        i: c_int,
    ) -> PVOID;
    pub fn DSA_GetItem(
        hdsa: HDSA,
        i: c_int,
        pitem: *mut c_void,
    ) -> BOOL;
    pub fn DSA_SetItem(
        hdsa: HDSA,
        i: c_int,
        pitem: *const c_void,
    ) -> BOOL;
}
#[inline]
pub unsafe fn DSA_GetItemCount(hdsa: HDSA) -> c_int {
    *(hdsa as *mut c_int)
}
#[inline]
pub unsafe fn DSA_AppendItem(hdsa: HDSA, pitem: *const c_void) -> c_int {
    DSA_InsertItem(hdsa, DA_LAST, pitem)
}
extern "system" {
    pub fn DSA_Clone(
        hdsa: HDSA,
    ) -> HDSA;
    pub fn DSA_GetSize(
        hdsa: HDSA,
    ) -> ULONGLONG;
    pub fn DSA_Sort(
        pdsa: HDSA,
        pfnCompare: PFNDACOMPARE,
        lParam: LPARAM,
    ) -> BOOL;
}
pub const DSA_APPEND: c_int = DA_LAST;
pub const DSA_ERR: c_int = DA_ERR;
pub type PFNDSAENUMCALLBACK = PFNDAENUMCALLBACK;
pub type PFNDSAENUMCALLBACKCONST = PFNDAENUMCALLBACKCONST;
pub type PFNDSACOMPARE = PFNDACOMPARE;
pub type PFNDSACOMPARECONST = PFNDACOMPARECONST;
pub enum DPA {}
pub type HDPA = *mut DPA;
extern "system" {
    pub fn DPA_Create(
        cItemGrow: c_int,
    ) -> HDPA;
    pub fn DPA_CreateEx(
        cpGrow: c_int,
        hheap: HANDLE,
    ) -> HDPA;
    pub fn DPA_Clone(
        hdpa: HDPA,
        hdpaNew: HDPA,
    ) -> HDPA;
    pub fn DPA_Destroy(
        hdpa: HDPA,
    ) -> BOOL;
    pub fn DPA_DestroyCallback(
        hdpa: HDPA,
        pfnCB: PFNDAENUMCALLBACK,
        pData: *mut c_void,
    );
    pub fn DPA_DeletePtr(
        hdpa: HDPA,
        i: c_int,
    ) -> PVOID;
    pub fn DPA_DeleteAllPtrs(
        hdpa: HDPA,
    ) -> BOOL;
    pub fn DPA_EnumCallback(
        hdpa: HDPA,
        pfnCB: PFNDAENUMCALLBACK,
        pData: *mut c_void,
    );
    pub fn DPA_Grow(
        hdpa: HDPA,
        cp: c_int,
    ) -> BOOL;
    pub fn DPA_InsertPtr(
        hdpa: HDPA,
        i: c_int,
        p: *mut c_void,
    ) -> c_int;
    pub fn DPA_SetPtr(
        hdpa: HDPA,
        i: c_int,
        p: *mut c_void,
    ) -> BOOL;
    pub fn DPA_GetPtr(
        hdpa: HDPA,
        i: INT_PTR,
    ) -> PVOID;
    pub fn DPA_GetPtrIndex(
        hdpa: HDPA,
        p: *const c_void,
    ) -> c_int;
}
#[inline]
pub unsafe fn DPA_GetPtrCount(hdpa: HDPA) -> c_int {
    *(hdpa as *mut c_int)
}
#[inline]
pub unsafe fn DPA_SetPtrCount(hdpa: HDPA, cItems: c_int) {
    *(hdpa as *mut c_int) = cItems;
}
#[inline]
pub unsafe fn DPA_FastDeleteLastPtr(hdpa: HDPA) -> c_int {
    *(hdpa as *mut c_int) -= 1;
    *(hdpa as *mut c_int)
}
#[inline]
pub unsafe fn DPA_AppendPtr(hdpa: HDPA, pitem: *mut c_void) -> c_int {
    DPA_InsertPtr(hdpa, DA_LAST, pitem)
}
extern "system" {
    pub fn DPA_GetSize(
        hdpa: HDPA,
    ) -> ULONGLONG;
    pub fn DPA_Sort(
        hdpa: HDPA,
        pfnCompare: PFNDACOMPARE,
        lParam: LPARAM,
    ) -> BOOL;
}
STRUCT!{struct DPASTREAMINFO {
    iPos: c_int,
    pvItem: *mut c_void,
}}
pub enum IStream {}
FN!{stdcall PFNDPASTREAM(
    pinfo: *mut DPASTREAMINFO,
    pstream: *mut IStream,
    pvInstData: *mut c_void,
) -> HRESULT}
extern "system" {
    pub fn DPA_LoadStream(
        phdpa: *mut HDPA,
        pfn: PFNDPASTREAM,
        pstream: *mut IStream,
        pvInstData: *mut c_void,
    ) -> HRESULT;
    pub fn DPA_SaveStream(
        hdpa: HDPA,
        pfn: PFNDPASTREAM,
        pstream: *mut IStream,
        pvInstData: *mut c_void,
    ) -> HRESULT;
}
pub const DPAM_SORTED: DWORD = 0x00000001;
pub const DPAM_NORMAL: DWORD = 0x00000002;
pub const DPAM_UNION: DWORD = 0x00000004;
pub const DPAM_INTERSECT: DWORD = 0x00000008;
FN!{stdcall PFNDPAMERGE(
    uMsg: UINT,
    pvDest: *mut c_void,
    pvSrc: *mut c_void,
    lParam: LPARAM,
) -> *mut c_void}
FN!{stdcall PFNDPAMERGECONST(
    uMsg: UINT,
    pvDest: *const c_void,
    pvSrc: *const c_void,
    lParam: LPARAM,
) -> *const c_void}
pub const DPAMM_MERGE: UINT = 1;
pub const DPAMM_DELETE: UINT = 2;
pub const DPAMM_INSERT: UINT = 3;
extern "system" {
    pub fn DPA_Merge(
        hdpaDest: HDPA,
        hdpaSrc: HDPA,
        dwFlags: DWORD,
        pfnCompare: PFNDACOMPARE,
        pfnMerge: PFNDPAMERGE,
        lParam: LPARAM,
    ) -> BOOL;
}
pub const DPAS_SORTED: UINT = 0x0001;
pub const DPAS_INSERTBEFORE: UINT = 0x0002;
pub const DPAS_INSERTAFTER: UINT = 0x0004;
extern "system" {
    pub fn DPA_Search(
        hdpa: HDPA,
        pFind: *mut c_void,
        iStart: c_int,
        pfnCompare: PFNDACOMPARE,
        lParam: LPARAM,
        options: UINT,
    ) -> c_int;
}
#[inline]
pub unsafe fn DPA_SortedInsertPtr(
    hdpa: HDPA,
    pFind: *mut c_void,
    iStart: c_int,
    pfnCompare: PFNDACOMPARE,
    lParam: LPARAM,
    options: UINT,
    pitem: *mut c_void,
) -> c_int {
    DPA_InsertPtr(
        hdpa,
        DPA_Search(
            hdpa, pFind, iStart, pfnCompare, lParam, DPAS_SORTED | options,
        ),
        pitem,
    )
}
pub const DPA_APPEND: c_int = DA_LAST;
pub const DPA_ERR: c_int = DA_ERR;
pub type PFNDPAENUMCALLBACK = PFNDAENUMCALLBACK;
pub type PFNDPAENUMCALLBACKCONST = PFNDAENUMCALLBACKCONST;
pub type PFNDPACOMPARE = PFNDACOMPARE;
pub type PFNDPACOMPARECONST = PFNDACOMPARECONST;
extern "system" {
    pub fn Str_SetPtrW(
        ppsz: *mut LPWSTR,
        psz: LPCWSTR,
    ) -> BOOL;
}
