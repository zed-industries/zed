// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_char, c_int, c_void};
use shared::basetsd::{SIZE_T, UINT64};
use shared::guiddef::{GUID, REFIID};
use shared::minwindef::{BOOL, DWORD, UINT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCSTR};
pub const DXGI_DEBUG_BINARY_VERSION: DWORD = 1;
ENUM!{enum DXGI_DEBUG_RLO_FLAGS {
    DXGI_DEBUG_RLO_SUMMARY = 0x1,
    DXGI_DEBUG_RLO_DETAIL = 0x2,
    DXGI_DEBUG_RLO_IGNORE_INTERNAL = 0x4,
    DXGI_DEBUG_RLO_ALL = 0x7,
}}
pub type DXGI_DEBUG_ID = GUID;
DEFINE_GUID!{DXGI_DEBUG_ALL,
    0xe48ae283, 0xda80, 0x490b, 0x87, 0xe6, 0x43, 0xe9, 0xa9, 0xcf, 0xda, 0x08}
DEFINE_GUID!{DXGI_DEBUG_DX,
    0x35cdd7fc, 0x13b2, 0x421d, 0xa5, 0xd7, 0x7e, 0x44, 0x51, 0x28, 0x7d, 0x64}
DEFINE_GUID!{DXGI_DEBUG_DXGI,
    0x25cddaa4, 0xb1c6, 0x47e1, 0xac, 0x3e, 0x98, 0x87, 0x5b, 0x5a, 0x2e, 0x2a}
DEFINE_GUID!{DXGI_DEBUG_APP,
    0x06cd6e01, 0x4219, 0x4ebd, 0x87, 0x09, 0x27, 0xed, 0x23, 0x36, 0x0c, 0x62}
ENUM!{enum DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN = 0,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS = 1,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION = 2,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP = 3,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION = 4,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION = 5,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING = 6,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING = 7,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION = 8,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION = 9,
    DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER = 10,
}}
ENUM!{enum DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION = 0,
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR = 1,
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING = 2,
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO = 3,
    DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE = 4,
}}
pub type DXGI_INFO_QUEUE_MESSAGE_ID = c_int;
STRUCT!{struct DXGI_INFO_QUEUE_MESSAGE {
    Producer: DXGI_DEBUG_ID,
    Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    ID: DXGI_INFO_QUEUE_MESSAGE_ID,
    pDescription: *const c_char,
    DescriptionByteLength: SIZE_T,
}}
STRUCT!{struct DXGI_INFO_QUEUE_FILTER_DESC {
    NumCategories: UINT,
    pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    NumSeverities: UINT,
    pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    NumIDs: UINT,
    pIDList: *mut DXGI_INFO_QUEUE_MESSAGE_ID,
}}
STRUCT!{struct DXGI_INFO_QUEUE_FILTER {
    AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}}
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: DWORD = 1024;
extern "system" {
    pub fn DXGIGetDebugInterface(
        riid: REFIID,
        ppDebug: *mut *mut c_void,
    ) -> HRESULT;
}
RIDL!{#[uuid(0xd67441c7, 0x672a, 0x476f, 0x9e, 0x82, 0xcd, 0x55, 0xb4, 0x49, 0x49, 0xce)]
interface IDXGIInfoQueue(IDXGIInfoQueueVtbl): IUnknown(IUnknownVtbl) {
    fn SetMessageCountLimit(
        Producer: DXGI_DEBUG_ID,
        MessageCountLimit: UINT64,
    ) -> HRESULT,
    fn ClearStoredMessages(
        Producer: DXGI_DEBUG_ID,
    ) -> (),
    fn GetMessage(
        Producer: DXGI_DEBUG_ID,
        MessageIndex: UINT64,
        pMessage: *mut DXGI_INFO_QUEUE_MESSAGE,
        pMessageByteLength: *mut SIZE_T,
    ) -> HRESULT,
    fn GetNumStoredMessagesAllowedByRetrievalFilters(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT64,
    fn GetNumStoredMessages(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT64,
    fn GetNumMessagesDiscardedByMessageCountLimit(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT64,
    fn GetMessageCountLimit(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT64,
    fn GetNumMessagesAllowedByStorageFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT64,
    fn GetNumMessagesDeniedByStorageFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT64,
    fn AddStorageFilterEntries(
        Producer: DXGI_DEBUG_ID,
        pFilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    fn GetStorageFilter(
        Producer: DXGI_DEBUG_ID,
        pFilter: *mut DXGI_INFO_QUEUE_FILTER,
        pFilterByteLength: *mut SIZE_T,
    ) -> HRESULT,
    fn ClearStorageFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> (),
    fn PushEmptyStorageFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> HRESULT,
    fn PushDenyAllStorageFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> HRESULT,
    fn PushCopyOfStorageFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> HRESULT,
    fn PushStorageFilter(
        Producer: DXGI_DEBUG_ID,
        pFilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    fn PopStorageFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> (),
    fn GetStorageFilterStackSize(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT,
    fn AddRetrievalFilterEntries(
        Producer: DXGI_DEBUG_ID,
        pFilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    fn GetRetrievalFilter(
        Producer: DXGI_DEBUG_ID,
        pFilter: *mut DXGI_INFO_QUEUE_FILTER,
        pFilterByteLength: *mut SIZE_T,
    ) -> HRESULT,
    fn ClearRetrievalFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> (),
    fn PushEmptyRetrievalFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> HRESULT,
    fn PushDenyAllRetrievalFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> HRESULT,
    fn PushCopyOfRetrievalFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> HRESULT,
    fn PushRetrievalFilter(
        Producer: DXGI_DEBUG_ID,
        pFilter: *const DXGI_INFO_QUEUE_FILTER,
    ) -> HRESULT,
    fn PopRetrievalFilter(
        Producer: DXGI_DEBUG_ID,
    ) -> (),
    fn GetRetrievalFilterStackSize(
        Producer: DXGI_DEBUG_ID,
    ) -> UINT,
    fn AddMessage(
        Producer: DXGI_DEBUG_ID,
        Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        ID: DXGI_INFO_QUEUE_MESSAGE_ID,
        pDescription: LPCSTR,
    ) -> HRESULT,
    fn AddApplicationMessage(
        Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        pDescription: LPCSTR,
    ) -> HRESULT,
    fn SetBreakOnCategory(
        Producer: DXGI_DEBUG_ID,
        Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
        bEnable: BOOL,
    ) -> HRESULT,
    fn SetBreakOnSeverity(
        Producer: DXGI_DEBUG_ID,
        Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
        bEnable: BOOL,
    ) -> HRESULT,
    fn SetBreakOnID(
        Producer: DXGI_DEBUG_ID,
        ID: DXGI_INFO_QUEUE_MESSAGE_ID,
        bEnable: BOOL,
    ) -> HRESULT,
    fn GetBreakOnCategory(
        Producer: DXGI_DEBUG_ID,
        Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    ) -> BOOL,
    fn GetBreakOnSeverity(
        Producer: DXGI_DEBUG_ID,
        Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    ) -> BOOL,
    fn GetBreakOnID(
        Producer: DXGI_DEBUG_ID,
        ID: DXGI_INFO_QUEUE_MESSAGE_ID,
    ) -> BOOL,
    fn SetMuteDebugOutput(
        Producer: DXGI_DEBUG_ID,
        bMute: BOOL,
    ) -> (),
    fn GetMuteDebugOutput(
        Producer: DXGI_DEBUG_ID,
    ) -> BOOL,
}}
RIDL!{#[uuid(0x119e7452, 0xde9e, 0x40fe, 0x88, 0x06, 0x88, 0xf9, 0x0c, 0x12, 0xb4, 0x41)]
interface IDXGIDebug(IDXGIDebugVtbl): IUnknown(IUnknownVtbl) {
    fn ReportLiveObjects(
        apiid: GUID,
        flags: DXGI_DEBUG_RLO_FLAGS,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x119e7452, 0xde9e, 0x40fe, 0x88, 0x06, 0x88, 0xf9, 0x0c, 0x12, 0xb4, 0x41)]
interface IDXGIDebug1(IDXGIDebug1Vtbl): IDXGIDebug(IDXGIDebugVtbl) {
    fn EnableLeakTrackingForThread() -> (),
    fn DisableLeakTrackingForThread() -> (),
    fn IsLeakTrackingEnabledForThread() -> BOOL,
}}
DEFINE_GUID!{IID_IDXGIInfoQueue,
    0xd67441c7, 0x672a, 0x476f, 0x9e, 0x82, 0xcd, 0x55, 0xb4, 0x49, 0x49, 0xce}
DEFINE_GUID!{IID_IDXGIDebug,
    0x119e7452, 0xde9e, 0x40fe, 0x88, 0x06, 0x88, 0xf9, 0x0c, 0x12, 0xb4, 0x41}
DEFINE_GUID!{IID_IDXGIDebug1,
    0xc5a05f0c, 0x16f2, 0x4adf, 0x9f, 0x4d, 0xa8, 0xc4, 0xd5, 0x8a, 0xc5, 0x50}
