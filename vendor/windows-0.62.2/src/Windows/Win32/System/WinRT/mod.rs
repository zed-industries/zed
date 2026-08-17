#[cfg(feature = "Win32_System_WinRT_AllJoyn")]
pub mod AllJoyn;
#[cfg(feature = "Win32_System_WinRT_Composition")]
pub mod Composition;
#[cfg(feature = "Win32_System_WinRT_CoreInputView")]
pub mod CoreInputView;
#[cfg(feature = "Win32_System_WinRT_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_System_WinRT_Display")]
pub mod Display;
#[cfg(feature = "Win32_System_WinRT_Graphics")]
pub mod Graphics;
#[cfg(feature = "Win32_System_WinRT_Holographic")]
pub mod Holographic;
#[cfg(feature = "Win32_System_WinRT_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_System_WinRT_ML")]
pub mod ML;
#[cfg(feature = "Win32_System_WinRT_Media")]
pub mod Media;
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub mod Metadata;
#[cfg(feature = "Win32_System_WinRT_Pdf")]
pub mod Pdf;
#[cfg(feature = "Win32_System_WinRT_Printing")]
pub mod Printing;
#[cfg(feature = "Win32_System_WinRT_Shell")]
pub mod Shell;
#[cfg(feature = "Win32_System_WinRT_Storage")]
pub mod Storage;
#[inline]
pub unsafe fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64) -> windows_core::Result<ServerInformation> {
    windows_core::link!("ole32.dll" "system" fn CoDecodeProxy(dwclientpid : u32, ui64proxyaddress : u64, pserverinformation : *mut ServerInformation) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoDecodeProxy(dwclientpid, ui64proxyaddress, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CreateControlInput<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("windows.ui.dll" "C" fn CreateControlInput(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateControlInput(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateControlInputEx<P0, T>(pcorewindow: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("windows.ui.dll" "C" fn CreateControlInputEx(pcorewindow : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateControlInputEx(pcorewindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "System")]
#[inline]
pub unsafe fn CreateDispatcherQueueController(options: DispatcherQueueOptions) -> windows_core::Result<super::super::super::System::DispatcherQueueController> {
    windows_core::link!("coremessaging.dll" "system" fn CreateDispatcherQueueController(options : DispatcherQueueOptions, dispatcherqueuecontroller : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDispatcherQueueController(core::mem::transmute(options), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateRandomAccessStreamOnFile<P0, T>(filepath: P0, accessmode: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOnFile(filepath : windows_core::PCWSTR, accessmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateRandomAccessStreamOnFile(filepath.param().abi(), accessmode, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateRandomAccessStreamOverStream<P0, T>(stream: P0, options: BSOS_OPTIONS) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::Com::IStream>,
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOverStream(stream : * mut core::ffi::c_void, options : BSOS_OPTIONS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateRandomAccessStreamOverStream(stream.param().abi(), options, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateStreamOverRandomAccessStream<P0, T>(randomaccessstream: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateStreamOverRandomAccessStream(randomaccessstream : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateStreamOverRandomAccessStream(randomaccessstream.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> windows_core::Result<IRestrictedErrorInfo> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn GetRestrictedErrorInfo(pprestrictederrorinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetRestrictedErrorInfo(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn HSTRING_UserFree(param0: *const u32, param1: *const windows_core::HSTRING) {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree(param0 : *const u32, param1 : *const * mut core::ffi::c_void));
    unsafe { HSTRING_UserFree(param0, core::mem::transmute(param1)) }
}
#[inline]
pub unsafe fn HSTRING_UserFree64(param0: *const u32, param1: *const windows_core::HSTRING) {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree64(param0 : *const u32, param1 : *const * mut core::ffi::c_void));
    unsafe { HSTRING_UserFree64(param0, core::mem::transmute(param1)) }
}
#[inline]
pub unsafe fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const windows_core::HSTRING) -> *mut u8 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const * mut core::ffi::c_void) -> *mut u8);
    unsafe { HSTRING_UserMarshal(param0, param1 as _, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const windows_core::HSTRING) -> *mut u8 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const * mut core::ffi::c_void) -> *mut u8);
    unsafe { HSTRING_UserMarshal64(param0, param1 as _, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const windows_core::HSTRING) -> u32 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize(param0 : *const u32, param1 : u32, param2 : *const * mut core::ffi::c_void) -> u32);
    unsafe { HSTRING_UserSize(param0, param1, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const windows_core::HSTRING) -> u32 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize64(param0 : *const u32, param1 : u32, param2 : *const * mut core::ffi::c_void) -> u32);
    unsafe { HSTRING_UserSize64(param0, param1, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut windows_core::HSTRING) -> *mut u8 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut * mut core::ffi::c_void) -> *mut u8);
    unsafe { HSTRING_UserUnmarshal(param0, param1, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut windows_core::HSTRING) -> *mut u8 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut * mut core::ffi::c_void) -> *mut u8);
    unsafe { HSTRING_UserUnmarshal64(param0, param1, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn IsErrorPropagationEnabled() -> windows_core::BOOL);
    unsafe { IsErrorPropagationEnabled() }
}
#[inline]
pub unsafe fn RoActivateInstance(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : * mut core::ffi::c_void, instance : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoActivateInstance(core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: windows_core::HRESULT) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoCaptureErrorContext(hr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { RoCaptureErrorContext(hr).ok() }
}
#[inline]
pub unsafe fn RoClearError() {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoClearError());
    unsafe { RoClearError() }
}
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: windows_core::HRESULT) {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoFailFastWithErrorContext(hrerror : windows_core::HRESULT));
    unsafe { RoFailFastWithErrorContext(hrerror) }
}
#[inline]
pub unsafe fn RoGetActivationFactory<T>(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : * mut core::ffi::c_void, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { RoGetActivationFactory(core::mem::transmute_copy(activatableclassid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn RoGetAgileReference<P2>(options: AgileReferenceOptions, riid: *const windows_core::GUID, punk: P2) -> windows_core::Result<IAgileReference>
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("combase.dll" "system" fn RoGetAgileReference(options : AgileReferenceOptions, riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, ppagilereference : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetAgileReference(options, riid, punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> windows_core::Result<u64> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetApartmentIdentifier(apartmentidentifier : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetApartmentIdentifier(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com_Marshal")]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> windows_core::Result<super::Com::Marshal::IMarshal> {
    windows_core::link!("api-ms-win-core-winrt-robuffer-l1-1-0.dll" "system" fn RoGetBufferMarshaler(buffermarshaler : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetBufferMarshaler(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> windows_core::Result<u32> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoGetErrorReportingFlags(pflags : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetErrorReportingFlags(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: windows_core::HRESULT) -> windows_core::Result<IRestrictedErrorInfo> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoGetMatchingRestrictedErrorInfo(hrin : windows_core::HRESULT, pprestrictederrorinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetMatchingRestrictedErrorInfo(hrin, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoGetServerActivatableClasses(servername: &windows_core::HSTRING, activatableclassids: *mut *mut windows_core::HSTRING, count: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-registration-l1-1-0.dll" "system" fn RoGetServerActivatableClasses(servername : * mut core::ffi::c_void, activatableclassids : *mut *mut * mut core::ffi::c_void, count : *mut u32) -> windows_core::HRESULT);
    unsafe { RoGetServerActivatableClasses(core::mem::transmute_copy(servername), activatableclassids as _, count as _).ok() }
}
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
    unsafe { RoInitialize(inittype).ok() }
}
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, framecount : *mut u32, targetbacktraceaddress : *mut usize) -> windows_core::HRESULT);
    unsafe { RoInspectCapturedStackBackTrace(targeterrorinfoaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, framecount as _, targetbacktraceaddress as _).ok() }
}
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>) -> windows_core::Result<usize> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectThreadErrorInfo(targettebaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, targeterrorinfoaddress : *mut usize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoInspectThreadErrorInfo(targettebaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RoOriginateError(error: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateError(error : windows_core::HRESULT, message : * mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoOriginateError(error, core::mem::transmute_copy(message)) }
}
#[inline]
pub unsafe fn RoOriginateErrorW(error: windows_core::HRESULT, cchmax: u32, message: Option<&[u16; 512]>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoOriginateErrorW(error, cchmax, core::mem::transmute(message.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn RoOriginateLanguageException<P2>(error: windows_core::HRESULT, message: &windows_core::HSTRING, languageexception: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoOriginateLanguageException(error : windows_core::HRESULT, message : * mut core::ffi::c_void, languageexception : * mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoOriginateLanguageException(error, core::mem::transmute_copy(message), languageexception.param().abi()) }
}
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const windows_core::HSTRING, activationfactorycallbacks: *const PFNGETACTIVATIONFACTORY, count: u32) -> windows_core::Result<RO_REGISTRATION_COOKIE> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterActivationFactories(activatableclassids : *const * mut core::ffi::c_void, activationfactorycallbacks : *const PFNGETACTIVATIONFACTORY, count : u32, cookie : *mut RO_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoRegisterActivationFactories(core::mem::transmute(activatableclassids), activationfactorycallbacks, count, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<P0>(callbackobject: P0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::Result<()>
where
    P0: windows_core::Param<IApartmentShutdown>,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterForApartmentShutdown(callbackobject : * mut core::ffi::c_void, apartmentidentifier : *mut u64, regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoRegisterForApartmentShutdown(callbackobject.param().abi(), apartmentidentifier as _, regcookie as _).ok() }
}
#[inline]
pub unsafe fn RoReportFailedDelegate<P0, P1>(punkdelegate: P0, prestrictederrorinfo: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportFailedDelegate(punkdelegate : * mut core::ffi::c_void, prestrictederrorinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportFailedDelegate(punkdelegate.param().abi(), prestrictederrorinfo.param().abi()).ok() }
}
#[inline]
pub unsafe fn RoReportUnhandledError<P0>(prestrictederrorinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportUnhandledError(prestrictederrorinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportUnhandledError(prestrictederrorinfo.param().abi()).ok() }
}
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<P0>(reference: P0) -> windows_core::Result<IRestrictedErrorInfo>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoResolveRestrictedErrorInfoReference(reference : windows_core::PCWSTR, pprestrictederrorinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoResolveRestrictedErrorInfoReference(reference.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: RO_REGISTRATION_COOKIE) {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRevokeActivationFactories(cookie : RO_REGISTRATION_COOKIE));
    unsafe { RoRevokeActivationFactories(cookie) }
}
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoSetErrorReportingFlags(flags : u32) -> windows_core::HRESULT);
    unsafe { RoSetErrorReportingFlags(flags).ok() }
}
#[inline]
pub unsafe fn RoTransformError(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformError(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, message : * mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoTransformError(olderror, newerror, core::mem::transmute_copy(message)) }
}
#[inline]
pub unsafe fn RoTransformErrorW(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, cchmax: u32, message: Option<&[u16; 512]>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformErrorW(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoTransformErrorW(olderror, newerror, cchmax, core::mem::transmute(message.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn RoUninitialize() {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUninitialize());
    unsafe { RoUninitialize() }
}
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUnregisterForApartmentShutdown(regcookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoUnregisterForApartmentShutdown(regcookie).ok() }
}
#[inline]
pub unsafe fn SetRestrictedErrorInfo<P0>(prestrictederrorinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn SetRestrictedErrorInfo(prestrictederrorinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SetRestrictedErrorInfo(prestrictederrorinfo.param().abi()).ok() }
}
#[inline]
pub unsafe fn WindowsCompareStringOrdinal(string1: &windows_core::HSTRING, string2: &windows_core::HSTRING) -> windows_core::Result<i32> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCompareStringOrdinal(string1 : * mut core::ffi::c_void, string2 : * mut core::ffi::c_void, result : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsCompareStringOrdinal(core::mem::transmute_copy(string1), core::mem::transmute_copy(string2), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WindowsConcatString(string1: &windows_core::HSTRING, string2: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsConcatString(string1 : * mut core::ffi::c_void, string2 : * mut core::ffi::c_void, newstring : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsConcatString(core::mem::transmute_copy(string1), core::mem::transmute_copy(string2), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsCreateString(sourcestring: Option<&[u16]>) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateString(sourcestring : windows_core::PCWSTR, length : u32, string : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsCreateString(core::mem::transmute(sourcestring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), sourcestring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsCreateStringReference<P0>(sourcestring: P0, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut windows_core::HSTRING) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateStringReference(sourcestring : windows_core::PCWSTR, length : u32, hstringheader : *mut HSTRING_HEADER, string : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WindowsCreateStringReference(sourcestring.param().abi(), length, hstringheader as _, core::mem::transmute(string)).ok() }
}
#[inline]
pub unsafe fn WindowsDeleteString(string: &windows_core::HSTRING) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteString(string : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WindowsDeleteString(core::mem::transmute_copy(string)).ok() }
}
#[inline]
pub unsafe fn WindowsDeleteStringBuffer(bufferhandle: Option<HSTRING_BUFFER>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteStringBuffer(bufferhandle : HSTRING_BUFFER) -> windows_core::HRESULT);
    unsafe { WindowsDeleteStringBuffer(bufferhandle.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WindowsDuplicateString(string: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDuplicateString(string : * mut core::ffi::c_void, newstring : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsDuplicateString(core::mem::transmute_copy(string), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsGetStringLen(string: &windows_core::HSTRING) -> u32 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringLen(string : * mut core::ffi::c_void) -> u32);
    unsafe { WindowsGetStringLen(core::mem::transmute_copy(string)) }
}
#[inline]
pub unsafe fn WindowsGetStringRawBuffer(string: &windows_core::HSTRING, length: Option<*mut u32>) -> windows_core::PCWSTR {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringRawBuffer(string : * mut core::ffi::c_void, length : *mut u32) -> windows_core::PCWSTR);
    unsafe { WindowsGetStringRawBuffer(core::mem::transmute_copy(string), length.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: Option<*const core::ffi::c_void>, length: *mut u32, targetstringaddress: *mut usize) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsInspectString(targethstring : usize, machine : u16, callback : PINSPECT_HSTRING_CALLBACK, context : *const core::ffi::c_void, length : *mut u32, targetstringaddress : *mut usize) -> windows_core::HRESULT);
    unsafe { WindowsInspectString(targethstring, machine, callback, context.unwrap_or(core::mem::zeroed()) as _, length as _, targetstringaddress as _).ok() }
}
#[inline]
pub unsafe fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: Option<*const core::ffi::c_void>, length: *mut u32, targetstringaddress: *mut u64) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-1.dll" "system" fn WindowsInspectString2(targethstring : u64, machine : u16, callback : PINSPECT_HSTRING_CALLBACK2, context : *const core::ffi::c_void, length : *mut u32, targetstringaddress : *mut u64) -> windows_core::HRESULT);
    unsafe { WindowsInspectString2(targethstring, machine, callback, context.unwrap_or(core::mem::zeroed()) as _, length as _, targetstringaddress as _).ok() }
}
#[inline]
pub unsafe fn WindowsIsStringEmpty(string: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsIsStringEmpty(string : * mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WindowsIsStringEmpty(core::mem::transmute_copy(string)) }
}
#[inline]
pub unsafe fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPreallocateStringBuffer(length : u32, charbuffer : *mut *mut u16, bufferhandle : *mut HSTRING_BUFFER) -> windows_core::HRESULT);
    unsafe { WindowsPreallocateStringBuffer(length, charbuffer as _, bufferhandle as _).ok() }
}
#[inline]
pub unsafe fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPromoteStringBuffer(bufferhandle : HSTRING_BUFFER, string : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsPromoteStringBuffer(bufferhandle, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsReplaceString(string: &windows_core::HSTRING, stringreplaced: &windows_core::HSTRING, stringreplacewith: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsReplaceString(string : * mut core::ffi::c_void, stringreplaced : * mut core::ffi::c_void, stringreplacewith : * mut core::ffi::c_void, newstring : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsReplaceString(core::mem::transmute_copy(string), core::mem::transmute_copy(stringreplaced), core::mem::transmute_copy(stringreplacewith), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull(string: &windows_core::HSTRING) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsStringHasEmbeddedNull(string : * mut core::ffi::c_void, hasembednull : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsStringHasEmbeddedNull(core::mem::transmute_copy(string), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WindowsSubstring(string: &windows_core::HSTRING, startindex: u32) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstring(string : * mut core::ffi::c_void, startindex : u32, newstring : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsSubstring(core::mem::transmute_copy(string), startindex, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsSubstringWithSpecifiedLength(string: &windows_core::HSTRING, startindex: u32, length: u32) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstringWithSpecifiedLength(string : * mut core::ffi::c_void, startindex : u32, length : u32, newstring : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsSubstringWithSpecifiedLength(core::mem::transmute_copy(string), startindex, length, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsTrimStringEnd(string: &windows_core::HSTRING, trimstring: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringEnd(string : * mut core::ffi::c_void, trimstring : * mut core::ffi::c_void, newstring : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsTrimStringEnd(core::mem::transmute_copy(string), core::mem::transmute_copy(trimstring), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn WindowsTrimStringStart(string: &windows_core::HSTRING, trimstring: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringStart(string : * mut core::ffi::c_void, trimstring : * mut core::ffi::c_void, newstring : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WindowsTrimStringStart(core::mem::transmute_copy(string), core::mem::transmute_copy(trimstring), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTIVATIONTYPE(pub i32);
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = ACTIVATIONTYPE(2i32);
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = ACTIVATIONTYPE(16i32);
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = ACTIVATIONTYPE(1i32);
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = ACTIVATIONTYPE(4i32);
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = ACTIVATIONTYPE(8i32);
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = ACTIVATIONTYPE(0i32);
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(pub *mut core::ffi::c_void);
impl APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AgileReferenceOptions(pub i32);
pub const BSOS_DEFAULT: BSOS_OPTIONS = BSOS_OPTIONS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BSOS_OPTIONS(pub i32);
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = BSOS_OPTIONS(1i32);
pub const BaseTrust: TrustLevel = TrustLevel(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CASTING_CONNECTION_ERROR_STATUS(pub i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(1i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(2i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(3i32);
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(5i32);
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(4i32);
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(0i32);
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CASTING_CONNECTION_STATE(pub i32);
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(1i32);
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(4i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(0i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(3i32);
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(2i32);
pub const CastingSourceInfo_Property_CastingTypes: windows_core::PCWSTR = windows_core::w!("CastingTypes");
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: windows_core::PCWSTR = windows_core::w!("PreferredSourceUriScheme");
pub const CastingSourceInfo_Property_ProtectedMedia: windows_core::PCWSTR = windows_core::w!("ProtectedMedia");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
pub const FullTrust: TrustLevel = TrustLevel(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSTRING_BUFFER(pub *mut core::ffi::c_void);
impl HSTRING_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HSTRING_BUFFER {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteStringBuffer(bufferhandle : *mut core::ffi::c_void) -> i32);
            unsafe {
                WindowsDeleteStringBuffer(self.0);
            }
        }
    }
}
impl Default for HSTRING_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HSTRING_HEADER {
    pub flags: u32,
    pub length: u32,
    pub padding1: u32,
    pub padding2: u32,
    pub data: isize,
}
windows_core::imp::define_interface!(IAccountsSettingsPaneInterop, IAccountsSettingsPaneInterop_Vtbl, 0xd3ee12ad_3865_4362_9746_b75a682df0e6);
windows_core::imp::interface_hierarchy!(IAccountsSettingsPaneInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IAccountsSettingsPaneInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn ShowManageAccountsForWindowAsync<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowManageAccountsForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn ShowAddAccountForWindowAsync<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowAddAccountForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowManageAccountsForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAddAccountForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAccountsSettingsPaneInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, accountssettingspane: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowManageAccountsForWindowAsync(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowAddAccountForWindowAsync(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IAccountsSettingsPaneInterop_Vtbl {
    pub const fn new<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, accountssettingspane: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccountsSettingsPaneInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&accountssettingspane)).into()
            }
        }
        unsafe extern "system" fn ShowManageAccountsForWindowAsync<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccountsSettingsPaneInterop_Impl::ShowManageAccountsForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncaction)).into()
            }
        }
        unsafe extern "system" fn ShowAddAccountForWindowAsync<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccountsSettingsPaneInterop_Impl::ShowAddAccountForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncaction)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccountsSettingsPaneInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowManageAccountsForWindowAsync: ShowManageAccountsForWindowAsync::<Identity, OFFSET>,
            ShowAddAccountForWindowAsync: ShowAddAccountForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAccountsSettingsPaneInterop {}
windows_core::imp::define_interface!(IActivationFactory, IActivationFactory_Vtbl, 0x00000035_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IActivationFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IActivationFactory {
    pub unsafe fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActivationFactory_Impl: windows_core::IUnknownImpl {
    fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl IActivationFactory_Vtbl {
    pub const fn new<Identity: IActivationFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateInstance<Identity: IActivationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivationFactory_Impl::ActivateInstance(this) {
                    Ok(ok__) => {
                        instance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivationFactory, OFFSET>(), ActivateInstance: ActivateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivationFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivationFactory {}
windows_core::imp::define_interface!(IAgileReference, IAgileReference_Vtbl, 0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
windows_core::imp::interface_hierarchy!(IAgileReference, windows_core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAgileReference_Impl: windows_core::IUnknownImpl {
    fn Resolve(&self, riid: *const windows_core::GUID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IAgileReference_Vtbl {
    pub const fn new<Identity: IAgileReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IAgileReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAgileReference_Impl::Resolve(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobjectreference)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileReference as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAgileReference {}
windows_core::imp::define_interface!(IApartmentShutdown, IApartmentShutdown_Vtbl, 0xa2f05a09_27a2_42b5_bc0e_ac163ef49d9b);
windows_core::imp::interface_hierarchy!(IApartmentShutdown, windows_core::IUnknown);
impl IApartmentShutdown {
    pub unsafe fn OnUninitialize(&self, ui64apartmentidentifier: u64) {
        unsafe { (windows_core::Interface::vtable(self).OnUninitialize)(windows_core::Interface::as_raw(self), ui64apartmentidentifier) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentShutdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
}
pub trait IApartmentShutdown_Impl: windows_core::IUnknownImpl {
    fn OnUninitialize(&self, ui64apartmentidentifier: u64);
}
impl IApartmentShutdown_Vtbl {
    pub const fn new<Identity: IApartmentShutdown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUninitialize<Identity: IApartmentShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ui64apartmentidentifier: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApartmentShutdown_Impl::OnUninitialize(this, core::mem::transmute_copy(&ui64apartmentidentifier))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUninitialize: OnUninitialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApartmentShutdown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IApartmentShutdown {}
windows_core::imp::define_interface!(IAppServiceConnectionExtendedExecution, IAppServiceConnectionExtendedExecution_Vtbl, 0x65219584_f9cb_4ae3_81f9_a28a6ca450d9);
windows_core::imp::interface_hierarchy!(IAppServiceConnectionExtendedExecution, windows_core::IUnknown);
impl IAppServiceConnectionExtendedExecution {
    pub unsafe fn OpenForExtendedExecutionAsync<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenForExtendedExecutionAsync)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionExtendedExecution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenForExtendedExecutionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppServiceConnectionExtendedExecution_Impl: windows_core::IUnknownImpl {
    fn OpenForExtendedExecutionAsync(&self, riid: *const windows_core::GUID, operation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IAppServiceConnectionExtendedExecution_Vtbl {
    pub const fn new<Identity: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenForExtendedExecutionAsync<Identity: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, operation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppServiceConnectionExtendedExecution_Impl::OpenForExtendedExecutionAsync(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&operation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenForExtendedExecutionAsync: OpenForExtendedExecutionAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppServiceConnectionExtendedExecution as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppServiceConnectionExtendedExecution {}
windows_core::imp::define_interface!(IBufferByteAccess, IBufferByteAccess_Vtbl, 0x905a0fef_bc53_11df_8c49_001e4fc686da);
windows_core::imp::interface_hierarchy!(IBufferByteAccess, windows_core::IUnknown);
impl IBufferByteAccess {
    pub unsafe fn Buffer(&self) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IBufferByteAccess_Impl: windows_core::IUnknownImpl {
    fn Buffer(&self) -> windows_core::Result<*mut u8>;
}
impl IBufferByteAccess_Vtbl {
    pub const fn new<Identity: IBufferByteAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Buffer<Identity: IBufferByteAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBufferByteAccess_Impl::Buffer(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Buffer: Buffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBufferByteAccess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBufferByteAccess {}
windows_core::imp::define_interface!(ICastingController, ICastingController_Vtbl, 0xf0a56423_a664_4fbd_8b43_409a45e8d9a1);
windows_core::imp::interface_hierarchy!(ICastingController, windows_core::IUnknown);
impl ICastingController {
    pub unsafe fn Initialize<P0, P1>(&self, castingengine: P0, castingsource: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), castingengine.param().abi(), castingsource.param().abi()).ok() }
    }
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Advise<P0>(&self, eventhandler: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ICastingEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnAdvise(&self, cookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnAdvise)(windows_core::Interface::as_raw(self), cookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingController_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ICastingController_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, castingengine: windows_core::Ref<windows_core::IUnknown>, castingsource: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Advise(&self, eventhandler: windows_core::Ref<ICastingEventHandler>) -> windows_core::Result<u32>;
    fn UnAdvise(&self, cookie: u32) -> windows_core::Result<()>;
}
impl ICastingController_Vtbl {
    pub const fn new<Identity: ICastingController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, castingengine: *mut core::ffi::c_void, castingsource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::Initialize(this, core::mem::transmute_copy(&castingengine), core::mem::transmute_copy(&castingsource)).into()
            }
        }
        unsafe extern "system" fn Connect<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::Connect(this).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICastingController_Impl::Advise(this, core::mem::transmute_copy(&eventhandler)) {
                    Ok(ok__) => {
                        cookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::UnAdvise(this, core::mem::transmute_copy(&cookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            UnAdvise: UnAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingController as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICastingController {}
windows_core::imp::define_interface!(ICastingEventHandler, ICastingEventHandler_Vtbl, 0xc79a6cb7_bebd_47a6_a2ad_4d45ad79c7bc);
windows_core::imp::interface_hierarchy!(ICastingEventHandler, windows_core::IUnknown);
impl ICastingEventHandler {
    pub unsafe fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), newstate).ok() }
    }
    pub unsafe fn OnError<P1>(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), errorstatus, errormessage.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, CASTING_CONNECTION_STATE) -> windows_core::HRESULT,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, CASTING_CONNECTION_ERROR_STATUS, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICastingEventHandler_Impl: windows_core::IUnknownImpl {
    fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> windows_core::Result<()>;
    fn OnError(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICastingEventHandler_Vtbl {
    pub const fn new<Identity: ICastingEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStateChanged<Identity: ICastingEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingEventHandler_Impl::OnStateChanged(this, core::mem::transmute_copy(&newstate)).into()
            }
        }
        unsafe extern "system" fn OnError<Identity: ICastingEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingEventHandler_Impl::OnError(this, core::mem::transmute_copy(&errorstatus), core::mem::transmute(&errormessage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICastingEventHandler {}
windows_core::imp::define_interface!(ICastingSourceInfo, ICastingSourceInfo_Vtbl, 0x45101ab7_7c3a_4bce_9500_12c09024b298);
windows_core::imp::interface_hierarchy!(ICastingSourceInfo, windows_core::IUnknown);
impl ICastingSourceInfo {
    pub unsafe fn GetController(&self) -> windows_core::Result<ICastingController> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetController)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProperties(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSourceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProperties: usize,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ICastingSourceInfo_Impl: windows_core::IUnknownImpl {
    fn GetController(&self) -> windows_core::Result<ICastingController>;
    fn GetProperties(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ICastingSourceInfo_Vtbl {
    pub const fn new<Identity: ICastingSourceInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetController<Identity: ICastingSourceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controller: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICastingSourceInfo_Impl::GetController(this) {
                    Ok(ok__) => {
                        controller.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ICastingSourceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, props: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICastingSourceInfo_Impl::GetProperties(this) {
                    Ok(ok__) => {
                        props.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetController: GetController::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingSourceInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ICastingSourceInfo {}
windows_core::imp::define_interface!(ICoreInputInterop, ICoreInputInterop_Vtbl, 0x40bfe3e3_b75a_4479_ac96_475365749bb8);
windows_core::imp::interface_hierarchy!(ICoreInputInterop, windows_core::IUnknown);
impl ICoreInputInterop {
    pub unsafe fn SetInputSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputSource)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMessageHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInputSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
pub trait ICoreInputInterop_Impl: windows_core::IUnknownImpl {
    fn SetInputSource(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()>;
}
impl ICoreInputInterop_Vtbl {
    pub const fn new<Identity: ICoreInputInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInputSource<Identity: ICoreInputInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop_Impl::SetInputSource(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ICoreInputInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop_Impl::SetMessageHandled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInputSource: SetInputSource::<Identity, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreInputInterop {}
windows_core::imp::define_interface!(ICoreInputInterop2, ICoreInputInterop2_Vtbl, 0xb8a2acd7_a0f0_40ee_8ee7_c82f59cc5cd4);
windows_core::imp::interface_hierarchy!(ICoreInputInterop2, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreInputInterop2 {
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ChangeHostingContext(&self, newparentwindow: super::super::Foundation::HWND, newviewinstanceid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChangeHostingContext)(windows_core::Interface::as_raw(self), newparentwindow, newviewinstanceid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputInterop2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub ChangeHostingContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
}
pub trait ICoreInputInterop2_Impl: windows_core::IUnknownImpl {
    fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn ChangeHostingContext(&self, newparentwindow: super::super::Foundation::HWND, newviewinstanceid: u32) -> windows_core::Result<()>;
}
impl ICoreInputInterop2_Vtbl {
    pub const fn new<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowHandle<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreInputInterop2_Impl::WindowHandle(this) {
                    Ok(ok__) => {
                        window.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeHostingContext<Identity: ICoreInputInterop2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newparentwindow: super::super::Foundation::HWND, newviewinstanceid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreInputInterop2_Impl::ChangeHostingContext(this, core::mem::transmute_copy(&newparentwindow), core::mem::transmute_copy(&newviewinstanceid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreInputInterop2, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            ChangeHostingContext: ChangeHostingContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputInterop2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreInputInterop2 {}
windows_core::imp::define_interface!(ICoreWindowAdapterInterop, ICoreWindowAdapterInterop_Vtbl, 0x7a5b6fd1_cd73_4b6c_9cf4_2e869eaf470a);
windows_core::imp::interface_hierarchy!(ICoreWindowAdapterInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreWindowAdapterInterop {
    pub unsafe fn AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppActivationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoreApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HoloViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PositionerClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemNavigationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TitleBarClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetWindowClientAdapter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWindowClientAdapter)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowAdapterInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppActivationClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CoreApplicationViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HoloViewClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PositionerClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemNavigationClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TitleBarClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWindowClientAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICoreWindowAdapterInterop_Impl: windows_core::IUnknownImpl {
    fn AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetWindowClientAdapter(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ICoreWindowAdapterInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AppActivationClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::AppActivationClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::ApplicationViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CoreApplicationViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::CoreApplicationViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HoloViewClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::HoloViewClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PositionerClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::PositionerClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SystemNavigationClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::SystemNavigationClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TitleBarClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowAdapterInterop_Impl::TitleBarClientAdapter(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWindowClientAdapter<Identity: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowAdapterInterop_Impl::SetWindowClientAdapter(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreWindowAdapterInterop, OFFSET>(),
            AppActivationClientAdapter: AppActivationClientAdapter::<Identity, OFFSET>,
            ApplicationViewClientAdapter: ApplicationViewClientAdapter::<Identity, OFFSET>,
            CoreApplicationViewClientAdapter: CoreApplicationViewClientAdapter::<Identity, OFFSET>,
            HoloViewClientAdapter: HoloViewClientAdapter::<Identity, OFFSET>,
            PositionerClientAdapter: PositionerClientAdapter::<Identity, OFFSET>,
            SystemNavigationClientAdapter: SystemNavigationClientAdapter::<Identity, OFFSET>,
            TitleBarClientAdapter: TitleBarClientAdapter::<Identity, OFFSET>,
            SetWindowClientAdapter: SetWindowClientAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowAdapterInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWindowAdapterInterop {}
windows_core::imp::define_interface!(ICoreWindowComponentInterop, ICoreWindowComponentInterop_Vtbl, 0x0576ab31_a310_4c40_ba31_fd37e0298dfa);
windows_core::imp::interface_hierarchy!(ICoreWindowComponentInterop, windows_core::IUnknown);
impl ICoreWindowComponentInterop {
    pub unsafe fn ConfigureComponentInput<P2>(&self, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConfigureComponentInput)(windows_core::Interface::as_raw(self), hostviewinstanceid, hwndhost, inputsourcevisual.param().abi()).ok() }
    }
    pub unsafe fn GetViewInstanceId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowComponentInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConfigureComponentInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetViewInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ICoreWindowComponentInterop_Impl: windows_core::IUnknownImpl {
    fn ConfigureComponentInput(&self, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetViewInstanceId(&self) -> windows_core::Result<u32>;
}
impl ICoreWindowComponentInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigureComponentInput<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowComponentInterop_Impl::ConfigureComponentInput(this, core::mem::transmute_copy(&hostviewinstanceid), core::mem::transmute_copy(&hwndhost), core::mem::transmute_copy(&inputsourcevisual)).into()
            }
        }
        unsafe extern "system" fn GetViewInstanceId<Identity: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, componentviewinstanceid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowComponentInterop_Impl::GetViewInstanceId(this) {
                    Ok(ok__) => {
                        componentviewinstanceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureComponentInput: ConfigureComponentInput::<Identity, OFFSET>,
            GetViewInstanceId: GetViewInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowComponentInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWindowComponentInterop {}
windows_core::imp::define_interface!(ICoreWindowInterop, ICoreWindowInterop_Vtbl, 0x45d64a29_a63e_4cb6_b498_5781d298cb4f);
windows_core::imp::interface_hierarchy!(ICoreWindowInterop, windows_core::IUnknown);
impl ICoreWindowInterop {
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMessageHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
pub trait ICoreWindowInterop_Impl: windows_core::IUnknownImpl {
    fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()>;
}
impl ICoreWindowInterop_Vtbl {
    pub const fn new<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowHandle<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreWindowInterop_Impl::WindowHandle(this) {
                    Ok(ok__) => {
                        hwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreWindowInterop_Impl::SetMessageHandled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreWindowInterop {}
windows_core::imp::define_interface!(ICorrelationVectorInformation, ICorrelationVectorInformation_Vtbl, 0x83c78b3c_d88b_4950_aa6e_22b8d22aabd3);
windows_core::imp::interface_hierarchy!(ICorrelationVectorInformation, windows_core::IUnknown, windows_core::IInspectable);
impl ICorrelationVectorInformation {
    pub unsafe fn LastCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastCorrelationVectorForThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn NextCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextCorrelationVectorForThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNextCorrelationVectorForThread(&self, cv: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNextCorrelationVectorForThread)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(cv)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LastCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNextCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorrelationVectorInformation_Impl: windows_core::IUnknownImpl {
    fn LastCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn NextCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNextCorrelationVectorForThread(&self, cv: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl ICorrelationVectorInformation_Vtbl {
    pub const fn new<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LastCorrelationVectorForThread<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorrelationVectorInformation_Impl::LastCorrelationVectorForThread(this) {
                    Ok(ok__) => {
                        cv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextCorrelationVectorForThread<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorrelationVectorInformation_Impl::NextCorrelationVectorForThread(this) {
                    Ok(ok__) => {
                        cv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNextCorrelationVectorForThread<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorrelationVectorInformation_Impl::SetNextCorrelationVectorForThread(this, core::mem::transmute(&cv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorrelationVectorInformation, OFFSET>(),
            LastCorrelationVectorForThread: LastCorrelationVectorForThread::<Identity, OFFSET>,
            NextCorrelationVectorForThread: NextCorrelationVectorForThread::<Identity, OFFSET>,
            SetNextCorrelationVectorForThread: SetNextCorrelationVectorForThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorrelationVectorInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorrelationVectorInformation {}
windows_core::imp::define_interface!(ICorrelationVectorSource, ICorrelationVectorSource_Vtbl, 0x152b8a3b_b9b9_4685_b56e_974847bc7545);
windows_core::imp::interface_hierarchy!(ICorrelationVectorSource, windows_core::IUnknown);
impl ICorrelationVectorSource {
    pub unsafe fn CorrelationVector(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CorrelationVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorrelationVectorSource_Impl: windows_core::IUnknownImpl {
    fn CorrelationVector(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ICorrelationVectorSource_Vtbl {
    pub const fn new<Identity: ICorrelationVectorSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CorrelationVector<Identity: ICorrelationVectorSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorrelationVectorSource_Impl::CorrelationVector(this) {
                    Ok(ok__) => {
                        cv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CorrelationVector: CorrelationVector::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorrelationVectorSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorrelationVectorSource {}
windows_core::imp::define_interface!(IDragDropManagerInterop, IDragDropManagerInterop_Vtbl, 0x5ad8cba7_4c01_4dac_9074_827894292d63);
windows_core::imp::interface_hierarchy!(IDragDropManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IDragDropManagerInterop {
    pub unsafe fn GetForWindow<T>(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDropManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDragDropManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDragDropManagerInterop_Vtbl {
    pub const fn new<Identity: IDragDropManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IDragDropManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDragDropManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDragDropManagerInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDragDropManagerInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDragDropManagerInterop {}
windows_core::imp::define_interface!(IHolographicSpaceInterop, IHolographicSpaceInterop_Vtbl, 0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
windows_core::imp::interface_hierarchy!(IHolographicSpaceInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicSpaceInterop {
    pub unsafe fn CreateForWindow<T>(&self, window: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHolographicSpaceInterop_Impl: windows_core::IUnknownImpl {
    fn CreateForWindow(&self, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, holographicspace: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IHolographicSpaceInterop_Vtbl {
    pub const fn new<Identity: IHolographicSpaceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateForWindow<Identity: IHolographicSpaceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, holographicspace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicSpaceInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&holographicspace)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHolographicSpaceInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolographicSpaceInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHolographicSpaceInterop {}
windows_core::imp::define_interface!(IInputPaneInterop, IInputPaneInterop_Vtbl, 0x75cf2c57_9195_4931_8332_f0b409e916af);
windows_core::imp::interface_hierarchy!(IInputPaneInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IInputPaneInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInputPaneInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, inputpane: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IInputPaneInterop_Vtbl {
    pub const fn new<Identity: IInputPaneInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IInputPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, inputpane: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPaneInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&inputpane)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputPaneInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPaneInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInputPaneInterop {}
windows_core::imp::define_interface!(ILanguageExceptionErrorInfo, ILanguageExceptionErrorInfo_Vtbl, 0x04a2dbf3_df83_116c_0946_0812abf6e07d);
windows_core::imp::interface_hierarchy!(ILanguageExceptionErrorInfo, windows_core::IUnknown);
impl ILanguageExceptionErrorInfo {
    pub unsafe fn GetLanguageException(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageException)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLanguageException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetLanguageException(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl ILanguageExceptionErrorInfo_Vtbl {
    pub const fn new<Identity: ILanguageExceptionErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLanguageException<Identity: ILanguageExceptionErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageexception: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionErrorInfo_Impl::GetLanguageException(this) {
                    Ok(ok__) => {
                        languageexception.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLanguageException: GetLanguageException::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionErrorInfo {}
windows_core::imp::define_interface!(ILanguageExceptionErrorInfo2, ILanguageExceptionErrorInfo2_Vtbl, 0x5746e5c4_5b97_424c_b620_2822915734dd);
impl core::ops::Deref for ILanguageExceptionErrorInfo2 {
    type Target = ILanguageExceptionErrorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILanguageExceptionErrorInfo2, windows_core::IUnknown, ILanguageExceptionErrorInfo);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousLanguageExceptionErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CapturePropagationContext<P0>(&self, languageexception: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CapturePropagationContext)(windows_core::Interface::as_raw(self), languageexception.param().abi()).ok() }
    }
    pub unsafe fn GetPropagationContextHead(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropagationContextHead)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionErrorInfo2_Impl: ILanguageExceptionErrorInfo_Impl {
    fn GetPreviousLanguageExceptionErrorInfo(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2>;
    fn CapturePropagationContext(&self, languageexception: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPropagationContextHead(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2>;
}
impl ILanguageExceptionErrorInfo2_Vtbl {
    pub const fn new<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPreviousLanguageExceptionErrorInfo<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionErrorInfo2_Impl::GetPreviousLanguageExceptionErrorInfo(this) {
                    Ok(ok__) => {
                        previouslanguageexceptionerrorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CapturePropagationContext<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageexception: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILanguageExceptionErrorInfo2_Impl::CapturePropagationContext(this, core::mem::transmute_copy(&languageexception)).into()
            }
        }
        unsafe extern "system" fn GetPropagationContextHead<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionErrorInfo2_Impl::GetPropagationContextHead(this) {
                    Ok(ok__) => {
                        propagatedlanguageexceptionerrorinfohead.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ILanguageExceptionErrorInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPreviousLanguageExceptionErrorInfo: GetPreviousLanguageExceptionErrorInfo::<Identity, OFFSET>,
            CapturePropagationContext: CapturePropagationContext::<Identity, OFFSET>,
            GetPropagationContextHead: GetPropagationContextHead::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo2 as windows_core::Interface>::IID || iid == &<ILanguageExceptionErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionErrorInfo2 {}
windows_core::imp::define_interface!(ILanguageExceptionStackBackTrace, ILanguageExceptionStackBackTrace_Vtbl, 0xcbe53fb5_f967_4258_8d34_42f5e25833de);
windows_core::imp::interface_hierarchy!(ILanguageExceptionStackBackTrace, windows_core::IUnknown);
impl ILanguageExceptionStackBackTrace {
    pub unsafe fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStackBackTrace)(windows_core::Interface::as_raw(self), maxframestocapture, stackbacktrace as _, framescaptured as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionStackBackTrace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStackBackTrace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionStackBackTrace_Impl: windows_core::IUnknownImpl {
    fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::Result<()>;
}
impl ILanguageExceptionStackBackTrace_Vtbl {
    pub const fn new<Identity: ILanguageExceptionStackBackTrace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStackBackTrace<Identity: ILanguageExceptionStackBackTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILanguageExceptionStackBackTrace_Impl::GetStackBackTrace(this, core::mem::transmute_copy(&maxframestocapture), core::mem::transmute_copy(&stackbacktrace), core::mem::transmute_copy(&framescaptured)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetStackBackTrace: GetStackBackTrace::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionStackBackTrace as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionStackBackTrace {}
windows_core::imp::define_interface!(ILanguageExceptionTransform, ILanguageExceptionTransform_Vtbl, 0xfeb5a271_a6cd_45ce_880a_696706badc65);
windows_core::imp::interface_hierarchy!(ILanguageExceptionTransform, windows_core::IUnknown);
impl ILanguageExceptionTransform {
    pub unsafe fn GetTransformedRestrictedErrorInfo(&self) -> windows_core::Result<IRestrictedErrorInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformedRestrictedErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTransformedRestrictedErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionTransform_Impl: windows_core::IUnknownImpl {
    fn GetTransformedRestrictedErrorInfo(&self) -> windows_core::Result<IRestrictedErrorInfo>;
}
impl ILanguageExceptionTransform_Vtbl {
    pub const fn new<Identity: ILanguageExceptionTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransformedRestrictedErrorInfo<Identity: ILanguageExceptionTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictederrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionTransform_Impl::GetTransformedRestrictedErrorInfo(this) {
                    Ok(ok__) => {
                        restrictederrorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTransformedRestrictedErrorInfo: GetTransformedRestrictedErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionTransform {}
windows_core::imp::define_interface!(IMemoryBufferByteAccess, IMemoryBufferByteAccess_Vtbl, 0x5b0d3235_4dba_4d44_865e_8f1d0e4fd04d);
windows_core::imp::interface_hierarchy!(IMemoryBufferByteAccess, windows_core::IUnknown);
impl IMemoryBufferByteAccess {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), value as _, capacity as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IMemoryBufferByteAccess_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()>;
}
impl IMemoryBufferByteAccess_Vtbl {
    pub const fn new<Identity: IMemoryBufferByteAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IMemoryBufferByteAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMemoryBufferByteAccess_Impl::GetBuffer(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&capacity)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBuffer: GetBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMemoryBufferByteAccess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMemoryBufferByteAccess {}
windows_core::imp::define_interface!(IMessageDispatcher, IMessageDispatcher_Vtbl, 0xf5f84c8f_cfd0_4cd6_b66b_c5d26ff1689d);
windows_core::imp::interface_hierarchy!(IMessageDispatcher, windows_core::IUnknown, windows_core::IInspectable);
impl IMessageDispatcher {
    pub unsafe fn PumpMessages(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PumpMessages)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDispatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PumpMessages: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMessageDispatcher_Impl: windows_core::IUnknownImpl {
    fn PumpMessages(&self) -> windows_core::Result<()>;
}
impl IMessageDispatcher_Vtbl {
    pub const fn new<Identity: IMessageDispatcher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PumpMessages<Identity: IMessageDispatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageDispatcher_Impl::PumpMessages(this).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMessageDispatcher, OFFSET>(), PumpMessages: PumpMessages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessageDispatcher as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMessageDispatcher {}
windows_core::imp::define_interface!(IPlayToManagerInterop, IPlayToManagerInterop_Vtbl, 0x24394699_1f2c_4eb3_8cd7_0ec1da42a540);
windows_core::imp::interface_hierarchy!(IPlayToManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPlayToManagerInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn ShowPlayToUIForWindow(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowPlayToUIForWindow)(windows_core::Interface::as_raw(self), appwindow).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPlayToUIForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IPlayToManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, playtomanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPlayToUIForWindow(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl IPlayToManagerInterop_Vtbl {
    pub const fn new<Identity: IPlayToManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IPlayToManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, playtomanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayToManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&playtomanager)).into()
            }
        }
        unsafe extern "system" fn ShowPlayToUIForWindow<Identity: IPlayToManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayToManagerInterop_Impl::ShowPlayToUIForWindow(this, core::mem::transmute_copy(&appwindow)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayToManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowPlayToUIForWindow: ShowPlayToUIForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayToManagerInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPlayToManagerInterop {}
windows_core::imp::define_interface!(IRestrictedErrorInfo, IRestrictedErrorInfo_Vtbl, 0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
windows_core::imp::interface_hierarchy!(IRestrictedErrorInfo, windows_core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: *mut windows_core::BSTR, error: *mut windows_core::HRESULT, restricteddescription: *mut windows_core::BSTR, capabilitysid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetErrorDetails)(windows_core::Interface::as_raw(self), core::mem::transmute(description), error as _, core::mem::transmute(restricteddescription), core::mem::transmute(capabilitysid)).ok() }
    }
    pub unsafe fn GetReference(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IRestrictedErrorInfo {}
unsafe impl Sync for IRestrictedErrorInfo {}
pub trait IRestrictedErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetErrorDetails(&self, description: *mut windows_core::BSTR, error: *mut windows_core::HRESULT, restricteddescription: *mut windows_core::BSTR, capabilitysid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetReference(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IRestrictedErrorInfo_Vtbl {
    pub const fn new<Identity: IRestrictedErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorDetails<Identity: IRestrictedErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void, error: *mut windows_core::HRESULT, restricteddescription: *mut *mut core::ffi::c_void, capabilitysid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRestrictedErrorInfo_Impl::GetErrorDetails(this, core::mem::transmute_copy(&description), core::mem::transmute_copy(&error), core::mem::transmute_copy(&restricteddescription), core::mem::transmute_copy(&capabilitysid)).into()
            }
        }
        unsafe extern "system" fn GetReference<Identity: IRestrictedErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRestrictedErrorInfo_Impl::GetReference(this) {
                    Ok(ok__) => {
                        reference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorDetails: GetErrorDetails::<Identity, OFFSET>,
            GetReference: GetReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRestrictedErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRestrictedErrorInfo {}
windows_core::imp::define_interface!(IShareWindowCommandEventArgsInterop, IShareWindowCommandEventArgsInterop_Vtbl, 0x6571a721_643d_43d4_aca4_6b6f5f30f1ad);
windows_core::imp::interface_hierarchy!(IShareWindowCommandEventArgsInterop, windows_core::IUnknown);
impl IShareWindowCommandEventArgsInterop {
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgsInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IShareWindowCommandEventArgsInterop_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
}
impl IShareWindowCommandEventArgsInterop_Vtbl {
    pub const fn new<Identity: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShareWindowCommandEventArgsInterop_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareWindowCommandEventArgsInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IShareWindowCommandEventArgsInterop {}
windows_core::imp::define_interface!(IShareWindowCommandSourceInterop, IShareWindowCommandSourceInterop_Vtbl, 0x461a191f_8424_43a6_a0fa_3451a22f56ab);
windows_core::imp::interface_hierarchy!(IShareWindowCommandSourceInterop, windows_core::IUnknown);
impl IShareWindowCommandSourceInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IShareWindowCommandSourceInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, sharewindowcommandsource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IShareWindowCommandSourceInterop_Vtbl {
    pub const fn new<Identity: IShareWindowCommandSourceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IShareWindowCommandSourceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, sharewindowcommandsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShareWindowCommandSourceInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&sharewindowcommandsource)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareWindowCommandSourceInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IShareWindowCommandSourceInterop {}
windows_core::imp::define_interface!(ISpatialInteractionManagerInterop, ISpatialInteractionManagerInterop_Vtbl, 0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
windows_core::imp::interface_hierarchy!(ISpatialInteractionManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ISpatialInteractionManagerInterop {
    pub unsafe fn GetForWindow<T>(&self, window: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialInteractionManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, spatialinteractionmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ISpatialInteractionManagerInterop_Vtbl {
    pub const fn new<Identity: ISpatialInteractionManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: ISpatialInteractionManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, spatialinteractionmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialInteractionManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&spatialinteractionmanager)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpatialInteractionManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialInteractionManagerInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialInteractionManagerInterop {}
windows_core::imp::define_interface!(ISystemMediaTransportControlsInterop, ISystemMediaTransportControlsInterop_Vtbl, 0xddb0472d_c911_4a1f_86d9_dc3d71a95f5a);
windows_core::imp::interface_hierarchy!(ISystemMediaTransportControlsInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ISystemMediaTransportControlsInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISystemMediaTransportControlsInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, mediatransportcontrol: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ISystemMediaTransportControlsInterop_Vtbl {
    pub const fn new<Identity: ISystemMediaTransportControlsInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: ISystemMediaTransportControlsInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const windows_core::GUID, mediatransportcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemMediaTransportControlsInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&mediatransportcontrol)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISystemMediaTransportControlsInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISystemMediaTransportControlsInterop {}
windows_core::imp::define_interface!(IUIViewSettingsInterop, IUIViewSettingsInterop_Vtbl, 0x3694dbf9_8f68_44be_8ff5_195c98ede8a6);
windows_core::imp::interface_hierarchy!(IUIViewSettingsInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUIViewSettingsInterop {
    pub unsafe fn GetForWindow<T>(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIViewSettingsInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIViewSettingsInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IUIViewSettingsInterop_Vtbl {
    pub const fn new<Identity: IUIViewSettingsInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IUIViewSettingsInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIViewSettingsInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIViewSettingsInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIViewSettingsInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIViewSettingsInterop {}
windows_core::imp::define_interface!(IUserActivityInterop, IUserActivityInterop_Vtbl, 0x1ade314d_0e0a_40d9_824c_9a088a50059f);
windows_core::imp::interface_hierarchy!(IUserActivityInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivityInterop {
    pub unsafe fn CreateSessionForWindow<T>(&self, window: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateSessionForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateSessionForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUserActivityInterop_Impl: windows_core::IUnknownImpl {
    fn CreateSessionForWindow(&self, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IUserActivityInterop_Vtbl {
    pub const fn new<Identity: IUserActivityInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSessionForWindow<Identity: IUserActivityInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserActivityInterop_Impl::CreateSessionForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivityInterop, OFFSET>(),
            CreateSessionForWindow: CreateSessionForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivityInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserActivityInterop {}
windows_core::imp::define_interface!(IUserActivityRequestManagerInterop, IUserActivityRequestManagerInterop_Vtbl, 0xdd69f876_9699_4715_9095_e37ea30dfa1b);
windows_core::imp::interface_hierarchy!(IUserActivityRequestManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivityRequestManagerInterop {
    pub unsafe fn GetForWindow<T>(&self, window: super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUserActivityRequestManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IUserActivityRequestManagerInterop_Vtbl {
    pub const fn new<Identity: IUserActivityRequestManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IUserActivityRequestManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserActivityRequestManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivityRequestManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivityRequestManagerInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserActivityRequestManagerInterop {}
windows_core::imp::define_interface!(IUserActivitySourceHostInterop, IUserActivitySourceHostInterop_Vtbl, 0xc15df8bc_8844_487a_b85b_7578e0f61419);
windows_core::imp::interface_hierarchy!(IUserActivitySourceHostInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivitySourceHostInterop {
    pub unsafe fn SetActivitySourceHost(&self, activitysourcehost: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActivitySourceHost)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(activitysourcehost)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySourceHostInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetActivitySourceHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUserActivitySourceHostInterop_Impl: windows_core::IUnknownImpl {
    fn SetActivitySourceHost(&self, activitysourcehost: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IUserActivitySourceHostInterop_Vtbl {
    pub const fn new<Identity: IUserActivitySourceHostInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetActivitySourceHost<Identity: IUserActivitySourceHostInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitysourcehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserActivitySourceHostInterop_Impl::SetActivitySourceHost(this, core::mem::transmute(&activitysourcehost)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivitySourceHostInterop, OFFSET>(),
            SetActivitySourceHost: SetActivitySourceHost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivitySourceHostInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserActivitySourceHostInterop {}
windows_core::imp::define_interface!(IUserConsentVerifierInterop, IUserConsentVerifierInterop_Vtbl, 0x39e050c3_4e74_441a_8dc0_b81104df949c);
windows_core::imp::interface_hierarchy!(IUserConsentVerifierInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserConsentVerifierInterop {
    pub unsafe fn RequestVerificationForWindowAsync<T>(&self, appwindow: super::super::Foundation::HWND, message: &windows_core::HSTRING) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestVerificationForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(message), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestVerificationForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUserConsentVerifierInterop_Impl: windows_core::IUnknownImpl {
    fn RequestVerificationForWindowAsync(&self, appwindow: super::super::Foundation::HWND, message: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IUserConsentVerifierInterop_Vtbl {
    pub const fn new<Identity: IUserConsentVerifierInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestVerificationForWindowAsync<Identity: IUserConsentVerifierInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserConsentVerifierInterop_Impl::RequestVerificationForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&message), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserConsentVerifierInterop, OFFSET>(),
            RequestVerificationForWindowAsync: RequestVerificationForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserConsentVerifierInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserConsentVerifierInterop {}
windows_core::imp::define_interface!(IWeakReference, IWeakReference_Vtbl, 0x00000037_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IWeakReference, windows_core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWeakReference_Impl: windows_core::IUnknownImpl {
    fn Resolve(&self, riid: *const windows_core::GUID, objectreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IWeakReference_Vtbl {
    pub const fn new<Identity: IWeakReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IWeakReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, objectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeakReference_Impl::Resolve(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&objectreference)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReference as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWeakReference {}
windows_core::imp::define_interface!(IWeakReferenceSource, IWeakReferenceSource_Vtbl, 0x00000038_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IWeakReferenceSource, windows_core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> windows_core::Result<IWeakReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWeakReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWeakReferenceSource_Impl: windows_core::IUnknownImpl {
    fn GetWeakReference(&self) -> windows_core::Result<IWeakReference>;
}
impl IWeakReferenceSource_Vtbl {
    pub const fn new<Identity: IWeakReferenceSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWeakReference<Identity: IWeakReferenceSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weakreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWeakReferenceSource_Impl::GetWeakReference(this) {
                    Ok(ok__) => {
                        weakreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWeakReference: GetWeakReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReferenceSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWeakReferenceSource {}
windows_core::imp::define_interface!(IWebAuthenticationCoreManagerInterop, IWebAuthenticationCoreManagerInterop_Vtbl, 0xf4b8e804_811e_4436_b69c_44cb67b72084);
windows_core::imp::interface_hierarchy!(IWebAuthenticationCoreManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IWebAuthenticationCoreManagerInterop {
    pub unsafe fn RequestTokenForWindowAsync<P1, T>(&self, appwindow: super::super::Foundation::HWND, request: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestTokenForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, request.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RequestTokenWithWebAccountForWindowAsync<P1, P2, T>(&self, appwindow: super::super::Foundation::HWND, request: P1, webaccount: P2) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
        P2: windows_core::Param<windows_core::IInspectable>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestTokenWithWebAccountForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, request.param().abi(), webaccount.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestTokenForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestTokenWithWebAccountForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWebAuthenticationCoreManagerInterop_Impl: windows_core::IUnknownImpl {
    fn RequestTokenForWindowAsync(&self, appwindow: super::super::Foundation::HWND, request: windows_core::Ref<windows_core::IInspectable>, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestTokenWithWebAccountForWindowAsync(&self, appwindow: super::super::Foundation::HWND, request: windows_core::Ref<windows_core::IInspectable>, webaccount: windows_core::Ref<windows_core::IInspectable>, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IWebAuthenticationCoreManagerInterop_Vtbl {
    pub const fn new<Identity: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestTokenForWindowAsync<Identity: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebAuthenticationCoreManagerInterop_Impl::RequestTokenForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&request), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncinfo)).into()
            }
        }
        unsafe extern "system" fn RequestTokenWithWebAccountForWindowAsync<Identity: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut core::ffi::c_void, webaccount: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebAuthenticationCoreManagerInterop_Impl::RequestTokenWithWebAccountForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&request), core::mem::transmute_copy(&webaccount), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncinfo)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAuthenticationCoreManagerInterop, OFFSET>(),
            RequestTokenForWindowAsync: RequestTokenForWindowAsync::<Identity, OFFSET>,
            RequestTokenWithWebAccountForWindowAsync: RequestTokenWithWebAccountForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWebAuthenticationCoreManagerInterop {}
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(param0: windows_core::Ref<windows_core::HSTRING>, param1: windows_core::OutRef<IActivationFactory>) -> windows_core::HRESULT>;
pub type PINSPECT_HSTRING_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
pub type PINSPECT_HSTRING_CALLBACK2 = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
pub type PINSPECT_MEMORY_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
pub const PartialTrust: TrustLevel = TrustLevel(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RO_ERROR_REPORTING_FLAGS(pub i32);
impl RO_ERROR_REPORTING_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(2i32);
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(0i32);
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(1i32);
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(8i32);
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(4i32);
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RO_INIT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RO_REGISTRATION_COOKIE(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TrustLevel(pub i32);
