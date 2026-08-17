// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{UINT32, UINT_PTR};
use shared::minwindef::{BOOL, BYTE, UINT, USHORT};
use um::restrictederrorinfo::IRestrictedErrorInfo;
use um::unknwnbase::IUnknown;
use um::winnt::{HRESULT, PCWSTR, PVOID, VOID};
use winrt::hstring::HSTRING;
ENUM!{enum RO_ERROR_REPORTING_FLAGS {
    RO_ERROR_REPORTING_NONE = 0x00000000,
    RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS = 0x00000001,
    RO_ERROR_REPORTING_FORCEEXCEPTIONS = 0x00000002,
    RO_ERROR_REPORTING_USESETERRORINFO = 0x00000004,
    RO_ERROR_REPORTING_SUPPRESSSETERRORINFO = 0x00000008,
}}
extern "system" {
    pub fn RoGetErrorReportingFlags(
        pflags: *mut UINT32,
    ) -> HRESULT;
    pub fn RoSetErrorReportingFlags(
        flags: UINT32,
    ) -> HRESULT;
    pub fn RoResolveRestrictedErrorInfoReference(
        reference: PCWSTR,
        ppRestrictedErrorInfo: *mut *mut IRestrictedErrorInfo ,
    ) -> HRESULT;
    pub fn SetRestrictedErrorInfo(
        pRestrictedErrorInfo: *const IRestrictedErrorInfo,
    ) -> HRESULT;
    pub fn GetRestrictedErrorInfo(
        ppRestrictedErrorInfo: *mut *mut IRestrictedErrorInfo,
    ) -> HRESULT;
    pub fn RoOriginateErrorW(
        error: HRESULT,
        cchMax: UINT,
        message: PCWSTR,
    ) -> BOOL;
    pub fn RoOriginateError(
        error: HRESULT,
        message: HSTRING,
    ) -> BOOL;
    pub fn RoTransformErrorW(
        oldError: HRESULT,
        newError: HRESULT,
        cchMax: UINT,
        message: PCWSTR,
    ) -> BOOL;
    pub fn RoTransformError(
        oldError: HRESULT,
        newError: HRESULT,
        message: HSTRING,
    ) -> BOOL;
    pub fn RoCaptureErrorContext(
        hr: HRESULT,
    ) -> HRESULT;
    pub fn RoFailFastWithErrorContext(
        hrError: HRESULT,
    );
    pub fn RoOriginateLanguageException(
        error: HRESULT,
        message: HSTRING,
        languageException: *const IUnknown,
    ) -> BOOL;
    pub fn RoClearError();
    pub fn RoReportUnhandledError(
        pRestrictedErrorInfo: *const IRestrictedErrorInfo,
    ) -> HRESULT;
}
FN!{stdcall PINSPECT_MEMORY_CALLBACK(
    *const VOID,
    UINT_PTR,
    UINT32,
    *mut BYTE,
) -> HRESULT}
extern "system" {
    pub fn RoInspectThreadErrorInfo(
        targetTebAddress: UINT_PTR,
        machine: USHORT,
        readMemoryCallback: PINSPECT_MEMORY_CALLBACK,
        context: PVOID,
        targetErrorInfoAddress: *mut UINT_PTR,
    ) -> HRESULT;
    pub fn RoInspectCapturedStackBackTrace(
        targetErrorInfoAddress: UINT_PTR,
        machine: USHORT,
        readMemoryCallback: PINSPECT_MEMORY_CALLBACK,
        context: PVOID,
        frameCount: *mut UINT32,
        targetBackTraceAddress: *mut UINT_PTR,
    ) -> HRESULT;
    pub fn RoGetMatchingRestrictedErrorInfo(
        hrIn: HRESULT,
        ppRestrictedErrorInfo: *mut *mut IRestrictedErrorInfo,
    ) -> HRESULT;
    pub fn RoReportFailedDelegate(
        punkDelegate: *const IUnknown,
        pRestrictedErrorInfo: *const IRestrictedErrorInfo,
    ) -> HRESULT;
    pub fn IsErrorPropagationEnabled() -> BOOL;
}
