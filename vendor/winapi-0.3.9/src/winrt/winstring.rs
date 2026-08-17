// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{INT32, UINT32, UINT_PTR};
use shared::minwindef::{BOOL, BYTE, UCHAR, ULONG, USHORT};
use um::winnt::{HRESULT, PCWSTR, VOID, WCHAR};
use winrt::hstring::{HSTRING, HSTRING_BUFFER, HSTRING_HEADER};
extern "system" {
    pub fn WindowsCreateString(
        sourceString: PCWSTR,
        length: UINT32,
        string: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsCreateStringReference(
        sourceString: PCWSTR,
        length: UINT32,
        hstringHeader: *mut HSTRING_HEADER,
        string: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsDeleteString(
        string: HSTRING,
    ) -> HRESULT;
    pub fn WindowsDuplicateString(
        string: HSTRING,
        newString: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsGetStringLen(
        string: HSTRING,
    ) -> UINT32;
    pub fn WindowsGetStringRawBuffer(
        string: HSTRING,
        length: *mut UINT32,
    ) -> PCWSTR;
    pub fn WindowsIsStringEmpty(
        string: HSTRING,
    ) -> BOOL;
    pub fn WindowsStringHasEmbeddedNull(
        string: HSTRING,
        hasEmbedNull: *mut BOOL,
    ) -> HRESULT;
    pub fn WindowsCompareStringOrdinal(
        string1: HSTRING,
        string2: HSTRING,
        result: *mut INT32,
    ) -> HRESULT;
    pub fn WindowsSubstring(
        string: HSTRING,
        startIndex: UINT32,
        newString: *mut HSTRING,
    ) -> HSTRING;
    pub fn WindowsSubstringWithSpecifiedLength(
        string: HSTRING,
        startIndex: UINT32,
        length: UINT32,
        newString: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsConcatString(
        string1: HSTRING,
        string2: HSTRING,
        newString: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsReplaceString(
        string: HSTRING,
        stringReplaced: HSTRING,
        stringReplaceWith: HSTRING,
        newString: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsTrimStringStart(
        string: HSTRING,
        trimString: HSTRING,
        newString: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsTrimStringEnd(
        string: HSTRING,
        trimString: HSTRING,
        newString: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsPreallocateStringBuffer(
        length: UINT32,
        charBuffer: *mut *mut WCHAR,
        bufferHandle: *mut HSTRING_BUFFER,
    ) -> HRESULT;
    pub fn WindowsPromoteStringBuffer(
        bufferHandle: HSTRING_BUFFER,
        string: *mut HSTRING,
    ) -> HRESULT;
    pub fn WindowsDeleteStringBuffer(
        bufferHandle: HSTRING_BUFFER,
    ) -> HRESULT;
}
FN!{stdcall PINSPECT_HSTRING_CALLBACK(
    *const VOID,
    UINT_PTR,
    UINT32,
    *mut BYTE,
) -> HRESULT}
extern "system" {
    pub fn WindowsInspectString(
        targetHString: UINT_PTR,
        machine: USHORT,
        callback: PINSPECT_HSTRING_CALLBACK,
        context: *const VOID,
        length: *mut UINT32,
        targetStringAddress: *mut UINT_PTR,
    ) -> HRESULT;
    pub fn HSTRING_UserSize(
        pFlags: *const ULONG,
        StartingSize: ULONG,
        ppidl: *const HSTRING,
    ) -> ULONG;
    pub fn HSTRING_UserMarshal(
        pFlags: *const ULONG,
        pBuffer: *mut UCHAR,
        ppidl: *const HSTRING,
    ) -> *mut UCHAR;
    pub fn HSTRING_UserUnmarshal(
        pFlags: *const ULONG,
        pBuffer: *const UCHAR,
        ppidl: *mut HSTRING,
    ) -> *mut UCHAR;
    pub fn HSTRING_UserFree(
        pFlags: *const ULONG,
        ppidl: *const HSTRING,
    );
    #[cfg(target_arch = "x86_64")]
    pub fn HSTRING_UserSize64(
        pFlags: *const ULONG,
        StartingSize: ULONG,
        ppidl: *const HSTRING,
    ) -> ULONG;
    #[cfg(target_arch = "x86_64")]
    pub fn HSTRING_UserMarshal64(
        pFlags: *const ULONG,
        pBuffer: *mut UCHAR,
        ppidl: *const HSTRING,
    ) -> *mut UCHAR;
    #[cfg(target_arch = "x86_64")]
    pub fn HSTRING_UserUnmarshal64(
        pFlags: *const ULONG,
        pBuffer: *const UCHAR,
        ppidl: *mut HSTRING,
    ) -> *mut UCHAR;
    #[cfg(target_arch = "x86_64")]
    pub fn HSTRING_UserFree64(
        pFlags: *const ULONG,
        ppidl: *const HSTRING,
    );
}
