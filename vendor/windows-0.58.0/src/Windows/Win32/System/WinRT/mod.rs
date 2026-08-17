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
    windows_targets::link!("ole32.dll" "system" fn CoDecodeProxy(dwclientpid : u32, ui64proxyaddress : u64, pserverinformation : *mut ServerInformation) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoDecodeProxy(dwclientpid, ui64proxyaddress, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CreateControlInput<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("windows.ui.dll" "cdecl" fn CreateControlInput(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CreateControlInput(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateControlInputEx<P0, T>(pcorewindow: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_targets::link!("windows.ui.dll" "cdecl" fn CreateControlInputEx(pcorewindow : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CreateControlInputEx(pcorewindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "System")]
#[inline]
pub unsafe fn CreateDispatcherQueueController(options: DispatcherQueueOptions) -> windows_core::Result<super::super::super::System::DispatcherQueueController> {
    windows_targets::link!("coremessaging.dll" "system" fn CreateDispatcherQueueController(options : DispatcherQueueOptions, dispatcherqueuecontroller : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateDispatcherQueueController(core::mem::transmute(options), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateRandomAccessStreamOnFile<P0, T>(filepath: P0, accessmode: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_targets::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOnFile(filepath : windows_core::PCWSTR, accessmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CreateRandomAccessStreamOnFile(filepath.param().abi(), accessmode, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateRandomAccessStreamOverStream<P0, T>(stream: P0, options: BSOS_OPTIONS) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::Com::IStream>,
    T: windows_core::Interface,
{
    windows_targets::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateRandomAccessStreamOverStream(stream : * mut core::ffi::c_void, options : BSOS_OPTIONS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CreateRandomAccessStreamOverStream(stream.param().abi(), options, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateStreamOverRandomAccessStream<P0, T>(randomaccessstream: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_targets::link!("api-ms-win-shcore-stream-winrt-l1-1-0.dll" "system" fn CreateStreamOverRandomAccessStream(randomaccessstream : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CreateStreamOverRandomAccessStream(randomaccessstream.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> windows_core::Result<IRestrictedErrorInfo> {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn GetRestrictedErrorInfo(pprestrictederrorinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetRestrictedErrorInfo(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn HSTRING_UserFree(param0: *const u32, param1: *const windows_core::HSTRING) {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree(param0 : *const u32, param1 : *const core::mem::MaybeUninit < windows_core::HSTRING >));
    HSTRING_UserFree(param0, core::mem::transmute(param1))
}
#[inline]
pub unsafe fn HSTRING_UserFree64(param0: *const u32, param1: *const windows_core::HSTRING) {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree64(param0 : *const u32, param1 : *const core::mem::MaybeUninit < windows_core::HSTRING >));
    HSTRING_UserFree64(param0, core::mem::transmute(param1))
}
#[inline]
pub unsafe fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const windows_core::HSTRING) -> *mut u8 {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const core::mem::MaybeUninit < windows_core::HSTRING >) -> *mut u8);
    HSTRING_UserMarshal(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const windows_core::HSTRING) -> *mut u8 {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const core::mem::MaybeUninit < windows_core::HSTRING >) -> *mut u8);
    HSTRING_UserMarshal64(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const windows_core::HSTRING) -> u32 {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize(param0 : *const u32, param1 : u32, param2 : *const core::mem::MaybeUninit < windows_core::HSTRING >) -> u32);
    HSTRING_UserSize(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const windows_core::HSTRING) -> u32 {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize64(param0 : *const u32, param1 : u32, param2 : *const core::mem::MaybeUninit < windows_core::HSTRING >) -> u32);
    HSTRING_UserSize64(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut windows_core::HSTRING) -> *mut u8 {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> *mut u8);
    HSTRING_UserUnmarshal(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut windows_core::HSTRING) -> *mut u8 {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> *mut u8);
    HSTRING_UserUnmarshal64(param0, param1, core::mem::transmute(param2))
}
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn IsErrorPropagationEnabled() -> super::super::Foundation:: BOOL);
    IsErrorPropagationEnabled()
}
#[inline]
pub unsafe fn RoActivateInstance(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : core::mem::MaybeUninit < windows_core::HSTRING >, instance : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoActivateInstance(core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: windows_core::HRESULT) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoCaptureErrorContext(hr : windows_core::HRESULT) -> windows_core::HRESULT);
    RoCaptureErrorContext(hr).ok()
}
#[inline]
pub unsafe fn RoClearError() {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoClearError());
    RoClearError()
}
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: windows_core::HRESULT) {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoFailFastWithErrorContext(hrerror : windows_core::HRESULT));
    RoFailFastWithErrorContext(hrerror)
}
#[inline]
pub unsafe fn RoGetActivationFactory<T>(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : core::mem::MaybeUninit < windows_core::HSTRING >, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    RoGetActivationFactory(core::mem::transmute_copy(activatableclassid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RoGetAgileReference<P0>(options: AgileReferenceOptions, riid: *const windows_core::GUID, punk: P0) -> windows_core::Result<IAgileReference>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn RoGetAgileReference(options : AgileReferenceOptions, riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, ppagilereference : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoGetAgileReference(options, riid, punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> windows_core::Result<u64> {
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetApartmentIdentifier(apartmentidentifier : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoGetApartmentIdentifier(&mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com_Marshal")]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> windows_core::Result<super::Com::Marshal::IMarshal> {
    windows_targets::link!("api-ms-win-core-winrt-robuffer-l1-1-0.dll" "system" fn RoGetBufferMarshaler(buffermarshaler : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoGetBufferMarshaler(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> windows_core::Result<u32> {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoGetErrorReportingFlags(pflags : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoGetErrorReportingFlags(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: windows_core::HRESULT) -> windows_core::Result<IRestrictedErrorInfo> {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoGetMatchingRestrictedErrorInfo(hrin : windows_core::HRESULT, pprestrictederrorinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoGetMatchingRestrictedErrorInfo(hrin, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RoGetServerActivatableClasses(servername: &windows_core::HSTRING, activatableclassids: *mut *mut windows_core::HSTRING, count: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-registration-l1-1-0.dll" "system" fn RoGetServerActivatableClasses(servername : core::mem::MaybeUninit < windows_core::HSTRING >, activatableclassids : *mut *mut windows_core::HSTRING, count : *mut u32) -> windows_core::HRESULT);
    RoGetServerActivatableClasses(core::mem::transmute_copy(servername), activatableclassids, count).ok()
}
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
    RoInitialize(inittype).ok()
}
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, framecount : *mut u32, targetbacktraceaddress : *mut usize) -> windows_core::HRESULT);
    RoInspectCapturedStackBackTrace(targeterrorinfoaddress, machine, readmemorycallback, core::mem::transmute(context.unwrap_or(std::ptr::null())), framecount, targetbacktraceaddress).ok()
}
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>) -> windows_core::Result<usize> {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectThreadErrorInfo(targettebaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, targeterrorinfoaddress : *mut usize) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoInspectThreadErrorInfo(targettebaddress, machine, readmemorycallback, core::mem::transmute(context.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RoOriginateError(error: windows_core::HRESULT, message: &windows_core::HSTRING) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateError(error : windows_core::HRESULT, message : core::mem::MaybeUninit < windows_core::HSTRING >) -> super::super::Foundation:: BOOL);
    RoOriginateError(error, core::mem::transmute_copy(message))
}
#[inline]
pub unsafe fn RoOriginateErrorW(error: windows_core::HRESULT, cchmax: u32, message: Option<&[u16; 512]>) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    RoOriginateErrorW(error, cchmax, core::mem::transmute(message.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn RoOriginateLanguageException<P0>(error: windows_core::HRESULT, message: &windows_core::HSTRING, languageexception: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoOriginateLanguageException(error : windows_core::HRESULT, message : core::mem::MaybeUninit < windows_core::HSTRING >, languageexception : * mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    RoOriginateLanguageException(error, core::mem::transmute_copy(message), languageexception.param().abi())
}
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const windows_core::HSTRING, activationfactorycallbacks: *const PFNGETACTIVATIONFACTORY, count: u32) -> windows_core::Result<RO_REGISTRATION_COOKIE> {
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterActivationFactories(activatableclassids : *const core::mem::MaybeUninit < windows_core::HSTRING >, activationfactorycallbacks : *const PFNGETACTIVATIONFACTORY, count : u32, cookie : *mut RO_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoRegisterActivationFactories(core::mem::transmute(activatableclassids), activationfactorycallbacks, count, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<P0>(callbackobject: P0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::Result<()>
where
    P0: windows_core::Param<IApartmentShutdown>,
{
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterForApartmentShutdown(callbackobject : * mut core::ffi::c_void, apartmentidentifier : *mut u64, regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    RoRegisterForApartmentShutdown(callbackobject.param().abi(), apartmentidentifier, regcookie).ok()
}
#[inline]
pub unsafe fn RoReportFailedDelegate<P0, P1>(punkdelegate: P0, prestrictederrorinfo: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<IRestrictedErrorInfo>,
{
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportFailedDelegate(punkdelegate : * mut core::ffi::c_void, prestrictederrorinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RoReportFailedDelegate(punkdelegate.param().abi(), prestrictederrorinfo.param().abi()).ok()
}
#[inline]
pub unsafe fn RoReportUnhandledError<P0>(prestrictederrorinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRestrictedErrorInfo>,
{
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportUnhandledError(prestrictederrorinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RoReportUnhandledError(prestrictederrorinfo.param().abi()).ok()
}
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<P0>(reference: P0) -> windows_core::Result<IRestrictedErrorInfo>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoResolveRestrictedErrorInfoReference(reference : windows_core::PCWSTR, pprestrictederrorinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoResolveRestrictedErrorInfoReference(reference.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RoRevokeActivationFactories<P0>(cookie: P0)
where
    P0: windows_core::Param<RO_REGISTRATION_COOKIE>,
{
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRevokeActivationFactories(cookie : RO_REGISTRATION_COOKIE));
    RoRevokeActivationFactories(cookie.param().abi())
}
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoSetErrorReportingFlags(flags : u32) -> windows_core::HRESULT);
    RoSetErrorReportingFlags(flags).ok()
}
#[inline]
pub unsafe fn RoTransformError(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, message: &windows_core::HSTRING) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformError(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, message : core::mem::MaybeUninit < windows_core::HSTRING >) -> super::super::Foundation:: BOOL);
    RoTransformError(olderror, newerror, core::mem::transmute_copy(message))
}
#[inline]
pub unsafe fn RoTransformErrorW(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, cchmax: u32, message: Option<&[u16; 512]>) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformErrorW(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    RoTransformErrorW(olderror, newerror, cchmax, core::mem::transmute(message.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())))
}
#[inline]
pub unsafe fn RoUninitialize() {
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUninitialize());
    RoUninitialize()
}
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown<P0>(regcookie: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>,
{
    windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUnregisterForApartmentShutdown(regcookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    RoUnregisterForApartmentShutdown(regcookie.param().abi()).ok()
}
#[inline]
pub unsafe fn SetRestrictedErrorInfo<P0>(prestrictederrorinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRestrictedErrorInfo>,
{
    windows_targets::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn SetRestrictedErrorInfo(prestrictederrorinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    SetRestrictedErrorInfo(prestrictederrorinfo.param().abi()).ok()
}
#[inline]
pub unsafe fn WindowsCompareStringOrdinal(string1: &windows_core::HSTRING, string2: &windows_core::HSTRING) -> windows_core::Result<i32> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCompareStringOrdinal(string1 : core::mem::MaybeUninit < windows_core::HSTRING >, string2 : core::mem::MaybeUninit < windows_core::HSTRING >, result : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsCompareStringOrdinal(core::mem::transmute_copy(string1), core::mem::transmute_copy(string2), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WindowsConcatString(string1: &windows_core::HSTRING, string2: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsConcatString(string1 : core::mem::MaybeUninit < windows_core::HSTRING >, string2 : core::mem::MaybeUninit < windows_core::HSTRING >, newstring : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsConcatString(core::mem::transmute_copy(string1), core::mem::transmute_copy(string2), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsCreateString(sourcestring: Option<&[u16]>) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateString(sourcestring : windows_core::PCWSTR, length : u32, string : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsCreateString(core::mem::transmute(sourcestring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), sourcestring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsCreateStringReference<P0>(sourcestring: P0, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut windows_core::HSTRING) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateStringReference(sourcestring : windows_core::PCWSTR, length : u32, hstringheader : *mut HSTRING_HEADER, string : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    WindowsCreateStringReference(sourcestring.param().abi(), length, hstringheader, core::mem::transmute(string)).ok()
}
#[inline]
pub unsafe fn WindowsDeleteString(string: &windows_core::HSTRING) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteString(string : core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    WindowsDeleteString(core::mem::transmute_copy(string)).ok()
}
#[inline]
pub unsafe fn WindowsDeleteStringBuffer<P0>(bufferhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HSTRING_BUFFER>,
{
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteStringBuffer(bufferhandle : HSTRING_BUFFER) -> windows_core::HRESULT);
    WindowsDeleteStringBuffer(bufferhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn WindowsDuplicateString(string: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDuplicateString(string : core::mem::MaybeUninit < windows_core::HSTRING >, newstring : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsDuplicateString(core::mem::transmute_copy(string), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsGetStringLen(string: &windows_core::HSTRING) -> u32 {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringLen(string : core::mem::MaybeUninit < windows_core::HSTRING >) -> u32);
    WindowsGetStringLen(core::mem::transmute_copy(string))
}
#[inline]
pub unsafe fn WindowsGetStringRawBuffer(string: &windows_core::HSTRING, length: Option<*mut u32>) -> windows_core::PCWSTR {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringRawBuffer(string : core::mem::MaybeUninit < windows_core::HSTRING >, length : *mut u32) -> windows_core::PCWSTR);
    WindowsGetStringRawBuffer(core::mem::transmute_copy(string), core::mem::transmute(length.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn WindowsInspectString(targethstring: usize, machine: u16, callback: PINSPECT_HSTRING_CALLBACK, context: Option<*const core::ffi::c_void>, length: *mut u32, targetstringaddress: *mut usize) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsInspectString(targethstring : usize, machine : u16, callback : PINSPECT_HSTRING_CALLBACK, context : *const core::ffi::c_void, length : *mut u32, targetstringaddress : *mut usize) -> windows_core::HRESULT);
    WindowsInspectString(targethstring, machine, callback, core::mem::transmute(context.unwrap_or(std::ptr::null())), length, targetstringaddress).ok()
}
#[inline]
pub unsafe fn WindowsInspectString2(targethstring: u64, machine: u16, callback: PINSPECT_HSTRING_CALLBACK2, context: Option<*const core::ffi::c_void>, length: *mut u32, targetstringaddress: *mut u64) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-1.dll" "system" fn WindowsInspectString2(targethstring : u64, machine : u16, callback : PINSPECT_HSTRING_CALLBACK2, context : *const core::ffi::c_void, length : *mut u32, targetstringaddress : *mut u64) -> windows_core::HRESULT);
    WindowsInspectString2(targethstring, machine, callback, core::mem::transmute(context.unwrap_or(std::ptr::null())), length, targetstringaddress).ok()
}
#[inline]
pub unsafe fn WindowsIsStringEmpty(string: &windows_core::HSTRING) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsIsStringEmpty(string : core::mem::MaybeUninit < windows_core::HSTRING >) -> super::super::Foundation:: BOOL);
    WindowsIsStringEmpty(core::mem::transmute_copy(string))
}
#[inline]
pub unsafe fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPreallocateStringBuffer(length : u32, charbuffer : *mut *mut u16, bufferhandle : *mut HSTRING_BUFFER) -> windows_core::HRESULT);
    WindowsPreallocateStringBuffer(length, charbuffer, bufferhandle).ok()
}
#[inline]
pub unsafe fn WindowsPromoteStringBuffer<P0>(bufferhandle: P0) -> windows_core::Result<windows_core::HSTRING>
where
    P0: windows_core::Param<HSTRING_BUFFER>,
{
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsPromoteStringBuffer(bufferhandle : HSTRING_BUFFER, string : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsPromoteStringBuffer(bufferhandle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsReplaceString(string: &windows_core::HSTRING, stringreplaced: &windows_core::HSTRING, stringreplacewith: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsReplaceString(string : core::mem::MaybeUninit < windows_core::HSTRING >, stringreplaced : core::mem::MaybeUninit < windows_core::HSTRING >, stringreplacewith : core::mem::MaybeUninit < windows_core::HSTRING >, newstring : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsReplaceString(core::mem::transmute_copy(string), core::mem::transmute_copy(stringreplaced), core::mem::transmute_copy(stringreplacewith), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull(string: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::BOOL> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsStringHasEmbeddedNull(string : core::mem::MaybeUninit < windows_core::HSTRING >, hasembednull : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsStringHasEmbeddedNull(core::mem::transmute_copy(string), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WindowsSubstring(string: &windows_core::HSTRING, startindex: u32) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstring(string : core::mem::MaybeUninit < windows_core::HSTRING >, startindex : u32, newstring : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsSubstring(core::mem::transmute_copy(string), startindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsSubstringWithSpecifiedLength(string: &windows_core::HSTRING, startindex: u32, length: u32) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsSubstringWithSpecifiedLength(string : core::mem::MaybeUninit < windows_core::HSTRING >, startindex : u32, length : u32, newstring : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsSubstringWithSpecifiedLength(core::mem::transmute_copy(string), startindex, length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsTrimStringEnd(string: &windows_core::HSTRING, trimstring: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringEnd(string : core::mem::MaybeUninit < windows_core::HSTRING >, trimstring : core::mem::MaybeUninit < windows_core::HSTRING >, newstring : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsTrimStringEnd(core::mem::transmute_copy(string), core::mem::transmute_copy(trimstring), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WindowsTrimStringStart(string: &windows_core::HSTRING, trimstring: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
    windows_targets::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsTrimStringStart(string : core::mem::MaybeUninit < windows_core::HSTRING >, trimstring : core::mem::MaybeUninit < windows_core::HSTRING >, newstring : *mut core::mem::MaybeUninit < windows_core::HSTRING >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WindowsTrimStringStart(core::mem::transmute_copy(string), core::mem::transmute_copy(trimstring), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(IAccountsSettingsPaneInterop, IAccountsSettingsPaneInterop_Vtbl, 0xd3ee12ad_3865_4362_9746_b75a682df0e6);
impl core::ops::Deref for IAccountsSettingsPaneInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccountsSettingsPaneInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IAccountsSettingsPaneInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShowManageAccountsForWindowAsync<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).ShowManageAccountsForWindowAsync)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShowAddAccountForWindowAsync<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).ShowAddAccountForWindowAsync)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAccountsSettingsPaneInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowManageAccountsForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAddAccountForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivationFactory, IActivationFactory_Vtbl, 0x00000035_0000_0000_c000_000000000046);
impl core::ops::Deref for IActivationFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActivationFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IActivationFactory {
    pub unsafe fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateInstance)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IActivationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAgileReference, IAgileReference_Vtbl, 0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
impl core::ops::Deref for IAgileReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAgileReference, windows_core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAgileReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApartmentShutdown, IApartmentShutdown_Vtbl, 0xa2f05a09_27a2_42b5_bc0e_ac163ef49d9b);
impl core::ops::Deref for IApartmentShutdown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IApartmentShutdown, windows_core::IUnknown);
impl IApartmentShutdown {
    pub unsafe fn OnUninitialize(&self, ui64apartmentidentifier: u64) {
        (windows_core::Interface::vtable(self).OnUninitialize)(windows_core::Interface::as_raw(self), ui64apartmentidentifier)
    }
}
#[repr(C)]
pub struct IApartmentShutdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
}
windows_core::imp::define_interface!(IAppServiceConnectionExtendedExecution, IAppServiceConnectionExtendedExecution_Vtbl, 0x65219584_f9cb_4ae3_81f9_a28a6ca450d9);
impl core::ops::Deref for IAppServiceConnectionExtendedExecution {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppServiceConnectionExtendedExecution, windows_core::IUnknown);
impl IAppServiceConnectionExtendedExecution {
    pub unsafe fn OpenForExtendedExecutionAsync<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).OpenForExtendedExecutionAsync)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppServiceConnectionExtendedExecution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenForExtendedExecutionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBufferByteAccess, IBufferByteAccess_Vtbl, 0x905a0fef_bc53_11df_8c49_001e4fc686da);
impl core::ops::Deref for IBufferByteAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBufferByteAccess, windows_core::IUnknown);
impl IBufferByteAccess {
    pub unsafe fn Buffer(&self) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Buffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICastingController, ICastingController_Vtbl, 0xf0a56423_a664_4fbd_8b43_409a45e8d9a1);
impl core::ops::Deref for ICastingController {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICastingController, windows_core::IUnknown);
impl ICastingController {
    pub unsafe fn Initialize<P0, P1>(&self, castingengine: P0, castingsource: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), castingengine.param().abi(), castingsource.param().abi()).ok()
    }
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<P0>(&self, eventhandler: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ICastingEventHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), eventhandler.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UnAdvise(&self, cookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnAdvise)(windows_core::Interface::as_raw(self), cookie).ok()
    }
}
#[repr(C)]
pub struct ICastingController_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICastingEventHandler, ICastingEventHandler_Vtbl, 0xc79a6cb7_bebd_47a6_a2ad_4d45ad79c7bc);
impl core::ops::Deref for ICastingEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICastingEventHandler, windows_core::IUnknown);
impl ICastingEventHandler {
    pub unsafe fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn OnError<P0>(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), errorstatus, errormessage.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICastingEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, CASTING_CONNECTION_STATE) -> windows_core::HRESULT,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, CASTING_CONNECTION_ERROR_STATUS, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICastingSourceInfo, ICastingSourceInfo_Vtbl, 0x45101ab7_7c3a_4bce_9500_12c09024b298);
impl core::ops::Deref for ICastingSourceInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICastingSourceInfo, windows_core::IUnknown);
impl ICastingSourceInfo {
    pub unsafe fn GetController(&self) -> windows_core::Result<ICastingController> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetController)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetProperties(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICastingSourceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProperties: usize,
}
windows_core::imp::define_interface!(ICoreInputInterop, ICoreInputInterop_Vtbl, 0x40bfe3e3_b75a_4479_ac96_475365749bb8);
impl core::ops::Deref for ICoreInputInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreInputInterop, windows_core::IUnknown);
impl ICoreInputInterop {
    pub unsafe fn SetInputSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetInputSource)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMessageHandled)(windows_core::Interface::as_raw(self), value).ok()
    }
}
#[repr(C)]
pub struct ICoreInputInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInputSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputInterop2, ICoreInputInterop2_Vtbl, 0xb8a2acd7_a0f0_40ee_8ee7_c82f59cc5cd4);
impl core::ops::Deref for ICoreInputInterop2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreInputInterop2, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreInputInterop2 {
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ChangeHostingContext<P0>(&self, newparentwindow: P0, newviewinstanceid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ChangeHostingContext)(windows_core::Interface::as_raw(self), newparentwindow.param().abi(), newviewinstanceid).ok()
    }
}
#[repr(C)]
pub struct ICoreInputInterop2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub ChangeHostingContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowAdapterInterop, ICoreWindowAdapterInterop_Vtbl, 0x7a5b6fd1_cd73_4b6c_9cf4_2e869eaf470a);
impl core::ops::Deref for ICoreWindowAdapterInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreWindowAdapterInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreWindowAdapterInterop {
    pub unsafe fn AppActivationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppActivationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CoreApplicationViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CoreApplicationViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HoloViewClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HoloViewClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PositionerClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PositionerClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SystemNavigationClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SystemNavigationClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TitleBarClientAdapter(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TitleBarClientAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetWindowClientAdapter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetWindowClientAdapter)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(ICoreWindowComponentInterop, ICoreWindowComponentInterop_Vtbl, 0x0576ab31_a310_4c40_ba31_fd37e0298dfa);
impl core::ops::Deref for ICoreWindowComponentInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreWindowComponentInterop, windows_core::IUnknown);
impl ICoreWindowComponentInterop {
    pub unsafe fn ConfigureComponentInput<P0, P1>(&self, hostviewinstanceid: u32, hwndhost: P0, inputsourcevisual: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ConfigureComponentInput)(windows_core::Interface::as_raw(self), hostviewinstanceid, hwndhost.param().abi(), inputsourcevisual.param().abi()).ok()
    }
    pub unsafe fn GetViewInstanceId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICoreWindowComponentInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConfigureComponentInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetViewInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWindowInterop, ICoreWindowInterop_Vtbl, 0x45d64a29_a63e_4cb6_b498_5781d298cb4f);
impl core::ops::Deref for ICoreWindowInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreWindowInterop, windows_core::IUnknown);
impl ICoreWindowInterop {
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMessageHandled(&self, value: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMessageHandled)(windows_core::Interface::as_raw(self), value).ok()
    }
}
#[repr(C)]
pub struct ICoreWindowInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub SetMessageHandled: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorrelationVectorInformation, ICorrelationVectorInformation_Vtbl, 0x83c78b3c_d88b_4950_aa6e_22b8d22aabd3);
impl core::ops::Deref for ICorrelationVectorInformation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorrelationVectorInformation, windows_core::IUnknown, windows_core::IInspectable);
impl ICorrelationVectorInformation {
    pub unsafe fn LastCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastCorrelationVectorForThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NextCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextCorrelationVectorForThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNextCorrelationVectorForThread(&self, cv: &windows_core::HSTRING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNextCorrelationVectorForThread)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(cv)).ok()
    }
}
#[repr(C)]
pub struct ICorrelationVectorInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LastCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NextCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetNextCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorrelationVectorSource, ICorrelationVectorSource_Vtbl, 0x152b8a3b_b9b9_4685_b56e_974847bc7545);
impl core::ops::Deref for ICorrelationVectorSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorrelationVectorSource, windows_core::IUnknown);
impl ICorrelationVectorSource {
    pub unsafe fn CorrelationVector(&self) -> windows_core::Result<windows_core::HSTRING> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorrelationVector)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICorrelationVectorSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CorrelationVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDragDropManagerInterop, IDragDropManagerInterop_Vtbl, 0x5ad8cba7_4c01_4dac_9074_827894292d63);
impl core::ops::Deref for IDragDropManagerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDragDropManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IDragDropManagerInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, hwnd: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDragDropManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicSpaceInterop, IHolographicSpaceInterop_Vtbl, 0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
impl core::ops::Deref for IHolographicSpaceInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHolographicSpaceInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicSpaceInterop {
    pub unsafe fn CreateForWindow<P0, T>(&self, window: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), window.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IHolographicSpaceInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPaneInterop, IInputPaneInterop_Vtbl, 0x75cf2c57_9195_4931_8332_f0b409e916af);
impl core::ops::Deref for IInputPaneInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInputPaneInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IInputPaneInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IInputPaneInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILanguageExceptionErrorInfo, ILanguageExceptionErrorInfo_Vtbl, 0x04a2dbf3_df83_116c_0946_0812abf6e07d);
impl core::ops::Deref for ILanguageExceptionErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILanguageExceptionErrorInfo, windows_core::IUnknown);
impl ILanguageExceptionErrorInfo {
    pub unsafe fn GetLanguageException(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguageException)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLanguageException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousLanguageExceptionErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CapturePropagationContext<P0>(&self, languageexception: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CapturePropagationContext)(windows_core::Interface::as_raw(self), languageexception.param().abi()).ok()
    }
    pub unsafe fn GetPropagationContextHead(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropagationContextHead)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILanguageExceptionStackBackTrace, ILanguageExceptionStackBackTrace_Vtbl, 0xcbe53fb5_f967_4258_8d34_42f5e25833de);
impl core::ops::Deref for ILanguageExceptionStackBackTrace {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILanguageExceptionStackBackTrace, windows_core::IUnknown);
impl ILanguageExceptionStackBackTrace {
    pub unsafe fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStackBackTrace)(windows_core::Interface::as_raw(self), maxframestocapture, stackbacktrace, framescaptured).ok()
    }
}
#[repr(C)]
pub struct ILanguageExceptionStackBackTrace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStackBackTrace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILanguageExceptionTransform, ILanguageExceptionTransform_Vtbl, 0xfeb5a271_a6cd_45ce_880a_696706badc65);
impl core::ops::Deref for ILanguageExceptionTransform {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILanguageExceptionTransform, windows_core::IUnknown);
impl ILanguageExceptionTransform {
    pub unsafe fn GetTransformedRestrictedErrorInfo(&self) -> windows_core::Result<IRestrictedErrorInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransformedRestrictedErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ILanguageExceptionTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTransformedRestrictedErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMemoryBufferByteAccess, IMemoryBufferByteAccess_Vtbl, 0x5b0d3235_4dba_4d44_865e_8f1d0e4fd04d);
impl core::ops::Deref for IMemoryBufferByteAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMemoryBufferByteAccess, windows_core::IUnknown);
impl IMemoryBufferByteAccess {
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), value, capacity).ok()
    }
}
#[repr(C)]
pub struct IMemoryBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMessageDispatcher, IMessageDispatcher_Vtbl, 0xf5f84c8f_cfd0_4cd6_b66b_c5d26ff1689d);
impl core::ops::Deref for IMessageDispatcher {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMessageDispatcher, windows_core::IUnknown, windows_core::IInspectable);
impl IMessageDispatcher {
    pub unsafe fn PumpMessages(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PumpMessages)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMessageDispatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PumpMessages: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayToManagerInterop, IPlayToManagerInterop_Vtbl, 0x24394699_1f2c_4eb3_8cd7_0ec1da42a540);
impl core::ops::Deref for IPlayToManagerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPlayToManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPlayToManagerInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShowPlayToUIForWindow<P0>(&self, appwindow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ShowPlayToUIForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPlayToManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPlayToUIForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRestrictedErrorInfo, IRestrictedErrorInfo_Vtbl, 0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
impl core::ops::Deref for IRestrictedErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRestrictedErrorInfo, windows_core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: *mut windows_core::BSTR, error: *mut windows_core::HRESULT, restricteddescription: *mut windows_core::BSTR, capabilitysid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetErrorDetails)(windows_core::Interface::as_raw(self), core::mem::transmute(description), error, core::mem::transmute(restricteddescription), core::mem::transmute(capabilitysid)).ok()
    }
    pub unsafe fn GetReference(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
unsafe impl Send for IRestrictedErrorInfo {}
unsafe impl Sync for IRestrictedErrorInfo {}
#[repr(C)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut windows_core::HRESULT, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShareWindowCommandEventArgsInterop, IShareWindowCommandEventArgsInterop_Vtbl, 0x6571a721_643d_43d4_aca4_6b6f5f30f1ad);
impl core::ops::Deref for IShareWindowCommandEventArgsInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IShareWindowCommandEventArgsInterop, windows_core::IUnknown);
impl IShareWindowCommandEventArgsInterop {
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IShareWindowCommandEventArgsInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShareWindowCommandSourceInterop, IShareWindowCommandSourceInterop_Vtbl, 0x461a191f_8424_43a6_a0fa_3451a22f56ab);
impl core::ops::Deref for IShareWindowCommandSourceInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IShareWindowCommandSourceInterop, windows_core::IUnknown);
impl IShareWindowCommandSourceInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IShareWindowCommandSourceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialInteractionManagerInterop, ISpatialInteractionManagerInterop_Vtbl, 0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
impl core::ops::Deref for ISpatialInteractionManagerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialInteractionManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ISpatialInteractionManagerInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, window: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), window.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpatialInteractionManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemMediaTransportControlsInterop, ISystemMediaTransportControlsInterop_Vtbl, 0xddb0472d_c911_4a1f_86d9_dc3d71a95f5a);
impl core::ops::Deref for ISystemMediaTransportControlsInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISystemMediaTransportControlsInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ISystemMediaTransportControlsInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISystemMediaTransportControlsInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIViewSettingsInterop, IUIViewSettingsInterop_Vtbl, 0x3694dbf9_8f68_44be_8ff5_195c98ede8a6);
impl core::ops::Deref for IUIViewSettingsInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIViewSettingsInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUIViewSettingsInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, hwnd: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIViewSettingsInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityInterop, IUserActivityInterop_Vtbl, 0x1ade314d_0e0a_40d9_824c_9a088a50059f);
impl core::ops::Deref for IUserActivityInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserActivityInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivityInterop {
    pub unsafe fn CreateSessionForWindow<P0, T>(&self, window: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateSessionForWindow)(windows_core::Interface::as_raw(self), window.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUserActivityInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateSessionForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityRequestManagerInterop, IUserActivityRequestManagerInterop_Vtbl, 0xdd69f876_9699_4715_9095_e37ea30dfa1b);
impl core::ops::Deref for IUserActivityRequestManagerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserActivityRequestManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivityRequestManagerInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, window: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), window.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUserActivityRequestManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivitySourceHostInterop, IUserActivitySourceHostInterop_Vtbl, 0xc15df8bc_8844_487a_b85b_7578e0f61419);
impl core::ops::Deref for IUserActivitySourceHostInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserActivitySourceHostInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivitySourceHostInterop {
    pub unsafe fn SetActivitySourceHost(&self, activitysourcehost: &windows_core::HSTRING) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActivitySourceHost)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(activitysourcehost)).ok()
    }
}
#[repr(C)]
pub struct IUserActivitySourceHostInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetActivitySourceHost: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserConsentVerifierInterop, IUserConsentVerifierInterop_Vtbl, 0x39e050c3_4e74_441a_8dc0_b81104df949c);
impl core::ops::Deref for IUserConsentVerifierInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserConsentVerifierInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserConsentVerifierInterop {
    pub unsafe fn RequestVerificationForWindowAsync<P0, T>(&self, appwindow: P0, message: &windows_core::HSTRING) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).RequestVerificationForWindowAsync)(windows_core::Interface::as_raw(self), appwindow.param().abi(), core::mem::transmute_copy(message), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUserConsentVerifierInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestVerificationForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, core::mem::MaybeUninit<windows_core::HSTRING>, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWeakReference, IWeakReference_Vtbl, 0x00000037_0000_0000_c000_000000000046);
impl core::ops::Deref for IWeakReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWeakReference, windows_core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWeakReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWeakReferenceSource, IWeakReferenceSource_Vtbl, 0x00000038_0000_0000_c000_000000000046);
impl core::ops::Deref for IWeakReferenceSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWeakReferenceSource, windows_core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> windows_core::Result<IWeakReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWeakReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAuthenticationCoreManagerInterop, IWebAuthenticationCoreManagerInterop_Vtbl, 0xf4b8e804_811e_4436_b69c_44cb67b72084);
impl core::ops::Deref for IWebAuthenticationCoreManagerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAuthenticationCoreManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IWebAuthenticationCoreManagerInterop {
    pub unsafe fn RequestTokenForWindowAsync<P0, P1, T>(&self, appwindow: P0, request: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::IInspectable>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).RequestTokenForWindowAsync)(windows_core::Interface::as_raw(self), appwindow.param().abi(), request.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestTokenWithWebAccountForWindowAsync<P0, P1, P2, T>(&self, appwindow: P0, request: P1, webaccount: P2) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::IInspectable>,
        P2: windows_core::Param<windows_core::IInspectable>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).RequestTokenWithWebAccountForWindowAsync)(windows_core::Interface::as_raw(self), appwindow.param().abi(), request.param().abi(), webaccount.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestTokenForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestTokenWithWebAccountForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = ACTIVATIONTYPE(2i32);
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = ACTIVATIONTYPE(16i32);
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = ACTIVATIONTYPE(1i32);
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = ACTIVATIONTYPE(4i32);
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = ACTIVATIONTYPE(8i32);
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = ACTIVATIONTYPE(0i32);
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
pub const BSOS_DEFAULT: BSOS_OPTIONS = BSOS_OPTIONS(0i32);
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = BSOS_OPTIONS(1i32);
pub const BaseTrust: TrustLevel = TrustLevel(0i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(1i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(2i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(3i32);
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(5i32);
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(4i32);
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(0i32);
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(6i32);
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(1i32);
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(4i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(0i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(3i32);
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(2i32);
pub const CastingSourceInfo_Property_CastingTypes: windows_core::PCWSTR = windows_core::w!("CastingTypes");
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: windows_core::PCWSTR = windows_core::w!("PreferredSourceUriScheme");
pub const CastingSourceInfo_Property_ProtectedMedia: windows_core::PCWSTR = windows_core::w!("ProtectedMedia");
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
pub const FullTrust: TrustLevel = TrustLevel(2i32);
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
pub const PartialTrust: TrustLevel = TrustLevel(1i32);
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(2i32);
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(0i32);
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(1i32);
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(8i32);
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(4i32);
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACTIVATIONTYPE(pub i32);
impl windows_core::TypeKind for ACTIVATIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACTIVATIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACTIVATIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AgileReferenceOptions(pub i32);
impl windows_core::TypeKind for AgileReferenceOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AgileReferenceOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AgileReferenceOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BSOS_OPTIONS(pub i32);
impl windows_core::TypeKind for BSOS_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BSOS_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BSOS_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CASTING_CONNECTION_ERROR_STATUS(pub i32);
impl windows_core::TypeKind for CASTING_CONNECTION_ERROR_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CASTING_CONNECTION_ERROR_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_ERROR_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CASTING_CONNECTION_STATE(pub i32);
impl windows_core::TypeKind for CASTING_CONNECTION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CASTING_CONNECTION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CASTING_CONNECTION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
impl windows_core::TypeKind for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_APARTMENTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
impl windows_core::TypeKind for DISPATCHERQUEUE_THREAD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPATCHERQUEUE_THREAD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPATCHERQUEUE_THREAD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RO_ERROR_REPORTING_FLAGS(pub i32);
impl windows_core::TypeKind for RO_ERROR_REPORTING_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RO_ERROR_REPORTING_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RO_ERROR_REPORTING_FLAGS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RO_INIT_TYPE(pub i32);
impl windows_core::TypeKind for RO_INIT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RO_INIT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RO_INIT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TrustLevel(pub i32);
impl windows_core::TypeKind for TrustLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TrustLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TrustLevel").field(&self.0).finish()
    }
}
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
impl windows_core::TypeKind for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
impl windows_core::TypeKind for DispatcherQueueOptions {
    type TypeKind = windows_core::CopyType;
}
impl Default for DispatcherQueueOptions {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventRegistrationToken {
    pub value: i64,
}
impl windows_core::TypeKind for EventRegistrationToken {
    type TypeKind = windows_core::CopyType;
}
impl Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
            _ = WindowsDeleteStringBuffer(*self);
        }
    }
}
impl Default for HSTRING_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HSTRING_BUFFER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HSTRING_HEADER {
    pub flags: u32,
    pub length: u32,
    pub padding1: u32,
    pub padding2: u32,
    pub data: isize,
}
impl windows_core::TypeKind for HSTRING_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for HSTRING_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RO_REGISTRATION_COOKIE(pub isize);
impl Default for RO_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RO_REGISTRATION_COOKIE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
impl windows_core::TypeKind for ServerInformation {
    type TypeKind = windows_core::CopyType;
}
impl Default for ServerInformation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(param0: windows_core::HSTRING, param1: *mut Option<IActivationFactory>) -> windows_core::HRESULT>;
pub type PINSPECT_HSTRING_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
pub type PINSPECT_HSTRING_CALLBACK2 = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
pub type PINSPECT_MEMORY_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
