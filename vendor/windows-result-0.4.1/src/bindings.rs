windows_link::link!("kernel32.dll" "system" fn FormatMessageW(dwflags : FORMAT_MESSAGE_OPTIONS, lpsource : *const core::ffi::c_void, dwmessageid : u32, dwlanguageid : u32, lpbuffer : PWSTR, nsize : u32, arguments : *const *const i8) -> u32);
windows_link::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : u32, pperrinfo : *mut * mut core::ffi::c_void) -> HRESULT);
windows_link::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : HRESULT, cchmax : u32, message : PCWSTR) -> BOOL);
windows_link::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : u32, perrinfo : * mut core::ffi::c_void) -> HRESULT);
windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR));
windows_link::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> u32);
pub type BOOL = i32;
pub type BSTR = *const u16;
pub const ERROR_INVALID_DATA: WIN32_ERROR = 13u32;
pub const ERROR_NO_UNICODE_TRANSLATION: WIN32_ERROR = 1113u32;
pub const E_UNEXPECTED: HRESULT = 0x8000FFFF_u32 as _;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS = 256u32;
pub const FORMAT_MESSAGE_FROM_HMODULE: FORMAT_MESSAGE_OPTIONS = 2048u32;
pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS = 4096u32;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS = 512u32;
pub type FORMAT_MESSAGE_OPTIONS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HANDLE = *mut core::ffi::c_void;
pub type HEAP_FLAGS = u32;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type HRESULT = i32;
pub const IID_IErrorInfo: GUID = GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
#[repr(C)]
pub struct IErrorInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GUID) -> HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
}
pub const IID_IRestrictedErrorInfo: GUID = GUID::from_u128(0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
#[repr(C)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut BSTR,
        *mut HRESULT,
        *mut BSTR,
        *mut BSTR,
    ) -> HRESULT,
    pub GetReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
}
pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
}
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
pub type WIN32_ERROR = u32;
