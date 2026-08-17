#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpAddFragmentToCache<P0, P1>(requestqueuehandle: P0, urlprefix: P1, datachunk: *const HTTP_DATA_CHUNK, cachepolicy: *const HTTP_CACHE_POLICY, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpAddFragmentToCache(requestqueuehandle : super::super::Foundation:: HANDLE, urlprefix : windows_core::PCWSTR, datachunk : *const HTTP_DATA_CHUNK, cachepolicy : *const HTTP_CACHE_POLICY, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpAddFragmentToCache(requestqueuehandle.param().abi(), urlprefix.param().abi(), datachunk, cachepolicy, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpAddUrl<P0, P1>(requestqueuehandle: P0, fullyqualifiedurl: P1, reserved: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpAddUrl(requestqueuehandle : super::super::Foundation:: HANDLE, fullyqualifiedurl : windows_core::PCWSTR, reserved : *const core::ffi::c_void) -> u32);
    HttpAddUrl(requestqueuehandle.param().abi(), fullyqualifiedurl.param().abi(), core::mem::transmute(reserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpAddUrlToUrlGroup<P0>(urlgroupid: u64, pfullyqualifiedurl: P0, urlcontext: u64, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpAddUrlToUrlGroup(urlgroupid : u64, pfullyqualifiedurl : windows_core::PCWSTR, urlcontext : u64, reserved : u32) -> u32);
    HttpAddUrlToUrlGroup(urlgroupid, pfullyqualifiedurl.param().abi(), urlcontext, reserved)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpCancelHttpRequest<P0>(requestqueuehandle: P0, requestid: u64, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpCancelHttpRequest(requestqueuehandle : super::super::Foundation:: HANDLE, requestid : u64, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpCancelHttpRequest(requestqueuehandle.param().abi(), requestid, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpCloseRequestQueue<P0>(requestqueuehandle: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpCloseRequestQueue(requestqueuehandle : super::super::Foundation:: HANDLE) -> u32);
    HttpCloseRequestQueue(requestqueuehandle.param().abi())
}
#[inline]
pub unsafe fn HttpCloseServerSession(serversessionid: u64) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpCloseServerSession(serversessionid : u64) -> u32);
    HttpCloseServerSession(serversessionid)
}
#[inline]
pub unsafe fn HttpCloseUrlGroup(urlgroupid: u64) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpCloseUrlGroup(urlgroupid : u64) -> u32);
    HttpCloseUrlGroup(urlgroupid)
}
#[inline]
pub unsafe fn HttpCreateHttpHandle(requestqueuehandle: *mut super::super::Foundation::HANDLE, reserved: u32) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpCreateHttpHandle(requestqueuehandle : *mut super::super::Foundation:: HANDLE, reserved : u32) -> u32);
    HttpCreateHttpHandle(requestqueuehandle, reserved)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn HttpCreateRequestQueue<P0>(version: HTTPAPI_VERSION, name: P0, securityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flags: u32, requestqueuehandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpCreateRequestQueue(version : HTTPAPI_VERSION, name : windows_core::PCWSTR, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flags : u32, requestqueuehandle : *mut super::super::Foundation:: HANDLE) -> u32);
    HttpCreateRequestQueue(core::mem::transmute(version), name.param().abi(), core::mem::transmute(securityattributes.unwrap_or(std::ptr::null())), flags, requestqueuehandle)
}
#[inline]
pub unsafe fn HttpCreateServerSession(version: HTTPAPI_VERSION, serversessionid: *mut u64, reserved: u32) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpCreateServerSession(version : HTTPAPI_VERSION, serversessionid : *mut u64, reserved : u32) -> u32);
    HttpCreateServerSession(core::mem::transmute(version), serversessionid, reserved)
}
#[inline]
pub unsafe fn HttpCreateUrlGroup(serversessionid: u64, purlgroupid: *mut u64, reserved: u32) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpCreateUrlGroup(serversessionid : u64, purlgroupid : *mut u64, reserved : u32) -> u32);
    HttpCreateUrlGroup(serversessionid, purlgroupid, reserved)
}
#[inline]
pub unsafe fn HttpDeclarePush<P0, P1, P2>(requestqueuehandle: P0, requestid: u64, verb: HTTP_VERB, path: P1, query: P2, headers: Option<*const HTTP_REQUEST_HEADERS>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpDeclarePush(requestqueuehandle : super::super::Foundation:: HANDLE, requestid : u64, verb : HTTP_VERB, path : windows_core::PCWSTR, query : windows_core::PCSTR, headers : *const HTTP_REQUEST_HEADERS) -> u32);
    HttpDeclarePush(requestqueuehandle.param().abi(), requestid, verb, path.param().abi(), query.param().abi(), core::mem::transmute(headers.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpDelegateRequestEx<P0, P1>(requestqueuehandle: P0, delegatequeuehandle: P1, requestid: u64, delegateurlgroupid: u64, propertyinfosetsize: u32, propertyinfoset: *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpDelegateRequestEx(requestqueuehandle : super::super::Foundation:: HANDLE, delegatequeuehandle : super::super::Foundation:: HANDLE, requestid : u64, delegateurlgroupid : u64, propertyinfosetsize : u32, propertyinfoset : *const HTTP_DELEGATE_REQUEST_PROPERTY_INFO) -> u32);
    HttpDelegateRequestEx(requestqueuehandle.param().abi(), delegatequeuehandle.param().abi(), requestid, delegateurlgroupid, propertyinfosetsize, propertyinfoset)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpDeleteServiceConfiguration<P0>(servicehandle: P0, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const core::ffi::c_void, configinformationlength: u32, poverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpDeleteServiceConfiguration(servicehandle : super::super::Foundation:: HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pconfiginformation : *const core::ffi::c_void, configinformationlength : u32, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpDeleteServiceConfiguration(servicehandle.param().abi(), configid, pconfiginformation, configinformationlength, core::mem::transmute(poverlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpFindUrlGroupId<P0, P1>(fullyqualifiedurl: P0, requestqueuehandle: P1, urlgroupid: *mut u64) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpFindUrlGroupId(fullyqualifiedurl : windows_core::PCWSTR, requestqueuehandle : super::super::Foundation:: HANDLE, urlgroupid : *mut u64) -> u32);
    HttpFindUrlGroupId(fullyqualifiedurl.param().abi(), requestqueuehandle.param().abi(), urlgroupid)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpFlushResponseCache<P0, P1>(requestqueuehandle: P0, urlprefix: P1, flags: u32, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpFlushResponseCache(requestqueuehandle : super::super::Foundation:: HANDLE, urlprefix : windows_core::PCWSTR, flags : u32, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpFlushResponseCache(requestqueuehandle.param().abi(), urlprefix.param().abi(), flags, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpGetExtension(version: HTTPAPI_VERSION, extension: u32, buffer: *mut core::ffi::c_void, buffersize: u32) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpGetExtension(version : HTTPAPI_VERSION, extension : u32, buffer : *mut core::ffi::c_void, buffersize : u32) -> u32);
    HttpGetExtension(core::mem::transmute(version), extension, buffer, buffersize)
}
#[inline]
pub unsafe fn HttpInitialize(version: HTTPAPI_VERSION, flags: HTTP_INITIALIZE, preserved: Option<*mut core::ffi::c_void>) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpInitialize(version : HTTPAPI_VERSION, flags : HTTP_INITIALIZE, preserved : *mut core::ffi::c_void) -> u32);
    HttpInitialize(core::mem::transmute(version), flags, core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn HttpIsFeatureSupported(featureid: HTTP_FEATURE_ID) -> super::super::Foundation::BOOL {
    windows_targets::link!("httpapi.dll" "system" fn HttpIsFeatureSupported(featureid : HTTP_FEATURE_ID) -> super::super::Foundation:: BOOL);
    HttpIsFeatureSupported(featureid)
}
#[inline]
pub unsafe fn HttpPrepareUrl<P0>(reserved: Option<*const core::ffi::c_void>, flags: u32, url: P0, preparedurl: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpPrepareUrl(reserved : *const core::ffi::c_void, flags : u32, url : windows_core::PCWSTR, preparedurl : *mut windows_core::PWSTR) -> u32);
    HttpPrepareUrl(core::mem::transmute(reserved.unwrap_or(std::ptr::null())), flags, url.param().abi(), preparedurl)
}
#[inline]
pub unsafe fn HttpQueryRequestQueueProperty<P0>(requestqueuehandle: P0, property: HTTP_SERVER_PROPERTY, propertyinformation: Option<*mut core::ffi::c_void>, propertyinformationlength: u32, reserved1: u32, returnlength: Option<*mut u32>, reserved2: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpQueryRequestQueueProperty(requestqueuehandle : super::super::Foundation:: HANDLE, property : HTTP_SERVER_PROPERTY, propertyinformation : *mut core::ffi::c_void, propertyinformationlength : u32, reserved1 : u32, returnlength : *mut u32, reserved2 : *const core::ffi::c_void) -> u32);
    HttpQueryRequestQueueProperty(requestqueuehandle.param().abi(), property, core::mem::transmute(propertyinformation.unwrap_or(std::ptr::null_mut())), propertyinformationlength, reserved1, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(reserved2.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpQueryServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: Option<*mut core::ffi::c_void>, propertyinformationlength: u32, returnlength: Option<*mut u32>) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpQueryServerSessionProperty(serversessionid : u64, property : HTTP_SERVER_PROPERTY, propertyinformation : *mut core::ffi::c_void, propertyinformationlength : u32, returnlength : *mut u32) -> u32);
    HttpQueryServerSessionProperty(serversessionid, property, core::mem::transmute(propertyinformation.unwrap_or(std::ptr::null_mut())), propertyinformationlength, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpQueryServiceConfiguration<P0>(servicehandle: P0, configid: HTTP_SERVICE_CONFIG_ID, pinput: Option<*const core::ffi::c_void>, inputlength: u32, poutput: Option<*mut core::ffi::c_void>, outputlength: u32, preturnlength: Option<*mut u32>, poverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpQueryServiceConfiguration(servicehandle : super::super::Foundation:: HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pinput : *const core::ffi::c_void, inputlength : u32, poutput : *mut core::ffi::c_void, outputlength : u32, preturnlength : *mut u32, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpQueryServiceConfiguration(servicehandle.param().abi(), configid, core::mem::transmute(pinput.unwrap_or(std::ptr::null())), inputlength, core::mem::transmute(poutput.unwrap_or(std::ptr::null_mut())), outputlength, core::mem::transmute(preturnlength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(poverlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpQueryUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: Option<*mut core::ffi::c_void>, propertyinformationlength: u32, returnlength: Option<*mut u32>) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpQueryUrlGroupProperty(urlgroupid : u64, property : HTTP_SERVER_PROPERTY, propertyinformation : *mut core::ffi::c_void, propertyinformationlength : u32, returnlength : *mut u32) -> u32);
    HttpQueryUrlGroupProperty(urlgroupid, property, core::mem::transmute(propertyinformation.unwrap_or(std::ptr::null_mut())), propertyinformationlength, core::mem::transmute(returnlength.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpReadFragmentFromCache<P0, P1>(requestqueuehandle: P0, urlprefix: P1, byterange: Option<*const HTTP_BYTE_RANGE>, buffer: *mut core::ffi::c_void, bufferlength: u32, bytesread: Option<*mut u32>, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpReadFragmentFromCache(requestqueuehandle : super::super::Foundation:: HANDLE, urlprefix : windows_core::PCWSTR, byterange : *const HTTP_BYTE_RANGE, buffer : *mut core::ffi::c_void, bufferlength : u32, bytesread : *mut u32, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpReadFragmentFromCache(requestqueuehandle.param().abi(), urlprefix.param().abi(), core::mem::transmute(byterange.unwrap_or(std::ptr::null())), buffer, bufferlength, core::mem::transmute(bytesread.unwrap_or(std::ptr::null_mut())), core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpReceiveClientCertificate<P0>(requestqueuehandle: P0, connectionid: u64, flags: u32, sslclientcertinfo: *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize: u32, bytesreceived: Option<*mut u32>, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpReceiveClientCertificate(requestqueuehandle : super::super::Foundation:: HANDLE, connectionid : u64, flags : u32, sslclientcertinfo : *mut HTTP_SSL_CLIENT_CERT_INFO, sslclientcertinfosize : u32, bytesreceived : *mut u32, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpReceiveClientCertificate(requestqueuehandle.param().abi(), connectionid, flags, sslclientcertinfo, sslclientcertinfosize, core::mem::transmute(bytesreceived.unwrap_or(std::ptr::null_mut())), core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn HttpReceiveHttpRequest<P0>(requestqueuehandle: P0, requestid: u64, flags: HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer: *mut HTTP_REQUEST_V2, requestbufferlength: u32, bytesreturned: Option<*mut u32>, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpReceiveHttpRequest(requestqueuehandle : super::super::Foundation:: HANDLE, requestid : u64, flags : HTTP_RECEIVE_HTTP_REQUEST_FLAGS, requestbuffer : *mut HTTP_REQUEST_V2, requestbufferlength : u32, bytesreturned : *mut u32, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> u32);
    HttpReceiveHttpRequest(requestqueuehandle.param().abi(), requestid, flags, requestbuffer, requestbufferlength, core::mem::transmute(bytesreturned.unwrap_or(std::ptr::null_mut())), core::mem::transmute(overlapped.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpReceiveRequestEntityBody<P0>(requestqueuehandle: P0, requestid: u64, flags: u32, entitybuffer: *mut core::ffi::c_void, entitybufferlength: u32, bytesreturned: Option<*mut u32>, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpReceiveRequestEntityBody(requestqueuehandle : super::super::Foundation:: HANDLE, requestid : u64, flags : u32, entitybuffer : *mut core::ffi::c_void, entitybufferlength : u32, bytesreturned : *mut u32, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpReceiveRequestEntityBody(requestqueuehandle.param().abi(), requestid, flags, entitybuffer, entitybufferlength, core::mem::transmute(bytesreturned.unwrap_or(std::ptr::null_mut())), core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpRemoveUrl<P0, P1>(requestqueuehandle: P0, fullyqualifiedurl: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpRemoveUrl(requestqueuehandle : super::super::Foundation:: HANDLE, fullyqualifiedurl : windows_core::PCWSTR) -> u32);
    HttpRemoveUrl(requestqueuehandle.param().abi(), fullyqualifiedurl.param().abi())
}
#[inline]
pub unsafe fn HttpRemoveUrlFromUrlGroup<P0>(urlgroupid: u64, pfullyqualifiedurl: P0, flags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpRemoveUrlFromUrlGroup(urlgroupid : u64, pfullyqualifiedurl : windows_core::PCWSTR, flags : u32) -> u32);
    HttpRemoveUrlFromUrlGroup(urlgroupid, pfullyqualifiedurl.param().abi(), flags)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpSendHttpResponse<P0>(requestqueuehandle: P0, requestid: u64, flags: u32, httpresponse: *const HTTP_RESPONSE_V2, cachepolicy: Option<*const HTTP_CACHE_POLICY>, bytessent: Option<*mut u32>, reserved1: Option<*const core::ffi::c_void>, reserved2: u32, overlapped: Option<*const super::super::System::IO::OVERLAPPED>, logdata: Option<*const HTTP_LOG_DATA>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpSendHttpResponse(requestqueuehandle : super::super::Foundation:: HANDLE, requestid : u64, flags : u32, httpresponse : *const HTTP_RESPONSE_V2, cachepolicy : *const HTTP_CACHE_POLICY, bytessent : *mut u32, reserved1 : *const core::ffi::c_void, reserved2 : u32, overlapped : *const super::super::System::IO:: OVERLAPPED, logdata : *const HTTP_LOG_DATA) -> u32);
    HttpSendHttpResponse(requestqueuehandle.param().abi(), requestid, flags, httpresponse, core::mem::transmute(cachepolicy.unwrap_or(std::ptr::null())), core::mem::transmute(bytessent.unwrap_or(std::ptr::null_mut())), core::mem::transmute(reserved1.unwrap_or(std::ptr::null())), reserved2, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())), core::mem::transmute(logdata.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpSendResponseEntityBody<P0>(requestqueuehandle: P0, requestid: u64, flags: u32, entitychunks: Option<&[HTTP_DATA_CHUNK]>, bytessent: Option<*mut u32>, reserved1: Option<*const core::ffi::c_void>, reserved2: u32, overlapped: Option<*const super::super::System::IO::OVERLAPPED>, logdata: Option<*const HTTP_LOG_DATA>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpSendResponseEntityBody(requestqueuehandle : super::super::Foundation:: HANDLE, requestid : u64, flags : u32, entitychunkcount : u16, entitychunks : *const HTTP_DATA_CHUNK, bytessent : *mut u32, reserved1 : *const core::ffi::c_void, reserved2 : u32, overlapped : *const super::super::System::IO:: OVERLAPPED, logdata : *const HTTP_LOG_DATA) -> u32);
    HttpSendResponseEntityBody(
        requestqueuehandle.param().abi(),
        requestid,
        flags,
        entitychunks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(entitychunks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        core::mem::transmute(bytessent.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(reserved1.unwrap_or(std::ptr::null())),
        reserved2,
        core::mem::transmute(overlapped.unwrap_or(std::ptr::null())),
        core::mem::transmute(logdata.unwrap_or(std::ptr::null())),
    )
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpSetRequestProperty<P0>(requestqueuehandle: P0, id: u64, propertyid: HTTP_REQUEST_PROPERTY, input: Option<*const core::ffi::c_void>, inputpropertysize: u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpSetRequestProperty(requestqueuehandle : super::super::Foundation:: HANDLE, id : u64, propertyid : HTTP_REQUEST_PROPERTY, input : *const core::ffi::c_void, inputpropertysize : u32, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpSetRequestProperty(requestqueuehandle.param().abi(), id, propertyid, core::mem::transmute(input.unwrap_or(std::ptr::null())), inputpropertysize, overlapped)
}
#[inline]
pub unsafe fn HttpSetRequestQueueProperty<P0>(requestqueuehandle: P0, property: HTTP_SERVER_PROPERTY, propertyinformation: *const core::ffi::c_void, propertyinformationlength: u32, reserved1: u32, reserved2: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpSetRequestQueueProperty(requestqueuehandle : super::super::Foundation:: HANDLE, property : HTTP_SERVER_PROPERTY, propertyinformation : *const core::ffi::c_void, propertyinformationlength : u32, reserved1 : u32, reserved2 : *const core::ffi::c_void) -> u32);
    HttpSetRequestQueueProperty(requestqueuehandle.param().abi(), property, propertyinformation, propertyinformationlength, reserved1, core::mem::transmute(reserved2.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpSetServerSessionProperty(serversessionid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const core::ffi::c_void, propertyinformationlength: u32) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpSetServerSessionProperty(serversessionid : u64, property : HTTP_SERVER_PROPERTY, propertyinformation : *const core::ffi::c_void, propertyinformationlength : u32) -> u32);
    HttpSetServerSessionProperty(serversessionid, property, propertyinformation, propertyinformationlength)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpSetServiceConfiguration<P0>(servicehandle: P0, configid: HTTP_SERVICE_CONFIG_ID, pconfiginformation: *const core::ffi::c_void, configinformationlength: u32, poverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpSetServiceConfiguration(servicehandle : super::super::Foundation:: HANDLE, configid : HTTP_SERVICE_CONFIG_ID, pconfiginformation : *const core::ffi::c_void, configinformationlength : u32, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpSetServiceConfiguration(servicehandle.param().abi(), configid, pconfiginformation, configinformationlength, core::mem::transmute(poverlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn HttpSetUrlGroupProperty(urlgroupid: u64, property: HTTP_SERVER_PROPERTY, propertyinformation: *const core::ffi::c_void, propertyinformationlength: u32) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpSetUrlGroupProperty(urlgroupid : u64, property : HTTP_SERVER_PROPERTY, propertyinformation : *const core::ffi::c_void, propertyinformationlength : u32) -> u32);
    HttpSetUrlGroupProperty(urlgroupid, property, propertyinformation, propertyinformationlength)
}
#[inline]
pub unsafe fn HttpShutdownRequestQueue<P0>(requestqueuehandle: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpShutdownRequestQueue(requestqueuehandle : super::super::Foundation:: HANDLE) -> u32);
    HttpShutdownRequestQueue(requestqueuehandle.param().abi())
}
#[inline]
pub unsafe fn HttpTerminate(flags: HTTP_INITIALIZE, preserved: Option<*mut core::ffi::c_void>) -> u32 {
    windows_targets::link!("httpapi.dll" "system" fn HttpTerminate(flags : HTTP_INITIALIZE, preserved : *mut core::ffi::c_void) -> u32);
    HttpTerminate(flags, core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpUpdateServiceConfiguration<P0>(handle: P0, configid: HTTP_SERVICE_CONFIG_ID, configinfo: *const core::ffi::c_void, configinfolength: u32, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpUpdateServiceConfiguration(handle : super::super::Foundation:: HANDLE, configid : HTTP_SERVICE_CONFIG_ID, configinfo : *const core::ffi::c_void, configinfolength : u32, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpUpdateServiceConfiguration(handle.param().abi(), configid, configinfo, configinfolength, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpWaitForDemandStart<P0>(requestqueuehandle: P0, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpWaitForDemandStart(requestqueuehandle : super::super::Foundation:: HANDLE, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpWaitForDemandStart(requestqueuehandle.param().abi(), core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpWaitForDisconnect<P0>(requestqueuehandle: P0, connectionid: u64, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpWaitForDisconnect(requestqueuehandle : super::super::Foundation:: HANDLE, connectionid : u64, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpWaitForDisconnect(requestqueuehandle.param().abi(), connectionid, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn HttpWaitForDisconnectEx<P0>(requestqueuehandle: P0, connectionid: u64, reserved: u32, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("httpapi.dll" "system" fn HttpWaitForDisconnectEx(requestqueuehandle : super::super::Foundation:: HANDLE, connectionid : u64, reserved : u32, overlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    HttpWaitForDisconnectEx(requestqueuehandle.param().abi(), connectionid, reserved, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
pub const CacheRangeChunkSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(1i32);
pub const CreateRequestQueueExternalIdProperty: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(1i32);
pub const CreateRequestQueueMax: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID = HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(2i32);
pub const DelegateRequestDelegateUrlProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(1i32);
pub const DelegateRequestReservedProperty: HTTP_DELEGATE_REQUEST_PROPERTY_ID = HTTP_DELEGATE_REQUEST_PROPERTY_ID(0i32);
pub const ExParamTypeErrorHeaders: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(4i32);
pub const ExParamTypeHttp2SettingsLimits: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(1i32);
pub const ExParamTypeHttp2Window: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(0i32);
pub const ExParamTypeHttpPerformance: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(2i32);
pub const ExParamTypeMax: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(6i32);
pub const ExParamTypeTlsRestrictions: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(3i32);
pub const ExParamTypeTlsSessionTicketKeys: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE = HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(5i32);
pub const HTTP_AUTH_ENABLE_BASIC: u32 = 1u32;
pub const HTTP_AUTH_ENABLE_DIGEST: u32 = 2u32;
pub const HTTP_AUTH_ENABLE_KERBEROS: u32 = 16u32;
pub const HTTP_AUTH_ENABLE_NEGOTIATE: u32 = 8u32;
pub const HTTP_AUTH_ENABLE_NTLM: u32 = 4u32;
pub const HTTP_AUTH_EX_FLAG_CAPTURE_CREDENTIAL: u32 = 2u32;
pub const HTTP_AUTH_EX_FLAG_ENABLE_KERBEROS_CREDENTIAL_CACHING: u32 = 1u32;
pub const HTTP_CHANNEL_BIND_CLIENT_SERVICE: u32 = 16u32;
pub const HTTP_CHANNEL_BIND_DOTLESS_SERVICE: u32 = 4u32;
pub const HTTP_CHANNEL_BIND_NO_SERVICE_NAME_CHECK: u32 = 2u32;
pub const HTTP_CHANNEL_BIND_PROXY: u32 = 1u32;
pub const HTTP_CHANNEL_BIND_PROXY_COHOSTING: u32 = 32u32;
pub const HTTP_CHANNEL_BIND_SECURE_CHANNEL_TOKEN: u32 = 8u32;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_CONTROLLER: u32 = 2u32;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_DELEGATION: u32 = 8u32;
pub const HTTP_CREATE_REQUEST_QUEUE_FLAG_OPEN_EXISTING: u32 = 1u32;
pub const HTTP_DEMAND_CBT: u32 = 4u32;
pub const HTTP_FLUSH_RESPONSE_FLAG_RECURSIVE: u32 = 1u32;
pub const HTTP_INITIALIZE_CONFIG: HTTP_INITIALIZE = HTTP_INITIALIZE(2u32);
pub const HTTP_INITIALIZE_SERVER: HTTP_INITIALIZE = HTTP_INITIALIZE(1u32);
pub const HTTP_LOGGING_FLAG_LOCAL_TIME_ROLLOVER: u32 = 1u32;
pub const HTTP_LOGGING_FLAG_LOG_ERRORS_ONLY: u32 = 4u32;
pub const HTTP_LOGGING_FLAG_LOG_SUCCESS_ONLY: u32 = 8u32;
pub const HTTP_LOGGING_FLAG_USE_UTF8_CONVERSION: u32 = 2u32;
pub const HTTP_LOG_FIELD_BYTES_RECV: u32 = 8192u32;
pub const HTTP_LOG_FIELD_BYTES_SENT: u32 = 4096u32;
pub const HTTP_LOG_FIELD_CLIENT_IP: u32 = 4u32;
pub const HTTP_LOG_FIELD_CLIENT_PORT: u32 = 4194304u32;
pub const HTTP_LOG_FIELD_COMPUTER_NAME: u32 = 32u32;
pub const HTTP_LOG_FIELD_COOKIE: u32 = 131072u32;
pub const HTTP_LOG_FIELD_CORRELATION_ID: u32 = 1073741824u32;
pub const HTTP_LOG_FIELD_DATE: u32 = 1u32;
pub const HTTP_LOG_FIELD_FAULT_CODE: u32 = 2147483648u32;
pub const HTTP_LOG_FIELD_HOST: u32 = 1048576u32;
pub const HTTP_LOG_FIELD_METHOD: u32 = 128u32;
pub const HTTP_LOG_FIELD_QUEUE_NAME: u32 = 67108864u32;
pub const HTTP_LOG_FIELD_REASON: u32 = 33554432u32;
pub const HTTP_LOG_FIELD_REFERER: u32 = 262144u32;
pub const HTTP_LOG_FIELD_SERVER_IP: u32 = 64u32;
pub const HTTP_LOG_FIELD_SERVER_PORT: u32 = 32768u32;
pub const HTTP_LOG_FIELD_SITE_ID: u32 = 16777216u32;
pub const HTTP_LOG_FIELD_SITE_NAME: u32 = 16u32;
pub const HTTP_LOG_FIELD_STATUS: u32 = 1024u32;
pub const HTTP_LOG_FIELD_STREAM_ID: u32 = 134217728u32;
pub const HTTP_LOG_FIELD_STREAM_ID_EX: u32 = 268435456u32;
pub const HTTP_LOG_FIELD_SUB_STATUS: u32 = 2097152u32;
pub const HTTP_LOG_FIELD_TIME: u32 = 2u32;
pub const HTTP_LOG_FIELD_TIME_TAKEN: u32 = 16384u32;
pub const HTTP_LOG_FIELD_TRANSPORT_TYPE: u32 = 536870912u32;
pub const HTTP_LOG_FIELD_URI: u32 = 8388608u32;
pub const HTTP_LOG_FIELD_URI_QUERY: u32 = 512u32;
pub const HTTP_LOG_FIELD_URI_STEM: u32 = 256u32;
pub const HTTP_LOG_FIELD_USER_AGENT: u32 = 65536u32;
pub const HTTP_LOG_FIELD_USER_NAME: u32 = 8u32;
pub const HTTP_LOG_FIELD_VERSION: u32 = 524288u32;
pub const HTTP_LOG_FIELD_WIN32_STATUS: u32 = 2048u32;
pub const HTTP_MAX_SERVER_QUEUE_LENGTH: u32 = 2147483647u32;
pub const HTTP_MIN_SERVER_QUEUE_LENGTH: u32 = 1u32;
pub const HTTP_RECEIVE_FULL_CHAIN: u32 = 2u32;
pub const HTTP_RECEIVE_REQUEST_ENTITY_BODY_FLAG_FILL_BUFFER: u32 = 1u32;
pub const HTTP_RECEIVE_REQUEST_FLAG_COPY_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(1u32);
pub const HTTP_RECEIVE_REQUEST_FLAG_FLUSH_BODY: HTTP_RECEIVE_HTTP_REQUEST_FLAGS = HTTP_RECEIVE_HTTP_REQUEST_FLAGS(2u32);
pub const HTTP_RECEIVE_SECURE_CHANNEL_TOKEN: u32 = 1u32;
pub const HTTP_REQUEST_AUTH_FLAG_TOKEN_FOR_CACHED_CRED: u32 = 1u32;
pub const HTTP_REQUEST_FLAG_HTTP2: u32 = 4u32;
pub const HTTP_REQUEST_FLAG_HTTP3: u32 = 8u32;
pub const HTTP_REQUEST_FLAG_IP_ROUTED: u32 = 2u32;
pub const HTTP_REQUEST_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 1u32;
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_NO_SNI: u32 = 2u32;
pub const HTTP_REQUEST_PROPERTY_SNI_FLAG_SNI_USED: u32 = 1u32;
pub const HTTP_REQUEST_PROPERTY_SNI_HOST_MAX_LENGTH: u32 = 255u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_FIRST_REQUEST: u32 = 8u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TCP_FAST_OPEN: u32 = 1u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_FALSE_START: u32 = 4u32;
pub const HTTP_REQUEST_SIZING_INFO_FLAG_TLS_SESSION_RESUMPTION: u32 = 2u32;
pub const HTTP_RESPONSE_FLAG_MORE_ENTITY_BODY_EXISTS: u32 = 2u32;
pub const HTTP_RESPONSE_FLAG_MULTIPLE_ENCODINGS_AVAILABLE: u32 = 1u32;
pub const HTTP_RESPONSE_INFO_FLAGS_PRESERVE_ORDER: u32 = 1u32;
pub const HTTP_SEND_RESPONSE_FLAG_BUFFER_DATA: u32 = 4u32;
pub const HTTP_SEND_RESPONSE_FLAG_DISCONNECT: u32 = 1u32;
pub const HTTP_SEND_RESPONSE_FLAG_ENABLE_NAGLING: u32 = 8u32;
pub const HTTP_SEND_RESPONSE_FLAG_GOAWAY: u32 = 256u32;
pub const HTTP_SEND_RESPONSE_FLAG_MORE_DATA: u32 = 2u32;
pub const HTTP_SEND_RESPONSE_FLAG_OPAQUE: u32 = 64u32;
pub const HTTP_SEND_RESPONSE_FLAG_PROCESS_RANGES: u32 = 32u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_HTTP2: u32 = 16u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_LEGACY_TLS: u32 = 1024u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_OCSP_STAPLING: u32 = 128u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_QUIC: u32 = 32u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_SESSION_ID: u32 = 16384u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS12: u32 = 4096u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_DISABLE_TLS13: u32 = 64u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_CLIENT_CORRELATION: u32 = 8192u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_SESSION_TICKET: u32 = 2048u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_ENABLE_TOKEN_BINDING: u32 = 256u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_LOG_EXTENDED_EVENTS: u32 = 512u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NEGOTIATE_CLIENT_CERT: u32 = 2u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_NO_RAW_FILTER: u32 = 4u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_REJECT: u32 = 8u32;
pub const HTTP_SERVICE_CONFIG_SSL_FLAG_USE_DS_MAPPER: u32 = 1u32;
pub const HTTP_URL_FLAG_REMOVE_ALL: u32 = 1u32;
pub const HTTP_VERSION: windows_core::PCWSTR = windows_core::w!("HTTP/1.0");
pub const HeaderWaitTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(1i32);
pub const Http503ResponseVerbosityBasic: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(0i32);
pub const Http503ResponseVerbosityFull: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(2i32);
pub const Http503ResponseVerbosityLimited: HTTP_503_RESPONSE_VERBOSITY = HTTP_503_RESPONSE_VERBOSITY(1i32);
pub const HttpAuthStatusFailure: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(2i32);
pub const HttpAuthStatusNotAuthenticated: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(1i32);
pub const HttpAuthStatusSuccess: HTTP_AUTH_STATUS = HTTP_AUTH_STATUS(0i32);
pub const HttpAuthenticationHardeningLegacy: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(0i32);
pub const HttpAuthenticationHardeningMedium: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(1i32);
pub const HttpAuthenticationHardeningStrict: HTTP_AUTHENTICATION_HARDENING_LEVELS = HTTP_AUTHENTICATION_HARDENING_LEVELS(2i32);
pub const HttpCachePolicyMaximum: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(3i32);
pub const HttpCachePolicyNocache: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(0i32);
pub const HttpCachePolicyTimeToLive: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(2i32);
pub const HttpCachePolicyUserInvalidates: HTTP_CACHE_POLICY_TYPE = HTTP_CACHE_POLICY_TYPE(1i32);
pub const HttpDataChunkFromFileHandle: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(1i32);
pub const HttpDataChunkFromFragmentCache: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(2i32);
pub const HttpDataChunkFromFragmentCacheEx: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(3i32);
pub const HttpDataChunkFromMemory: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(0i32);
pub const HttpDataChunkMaximum: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(5i32);
pub const HttpDataChunkTrailers: HTTP_DATA_CHUNK_TYPE = HTTP_DATA_CHUNK_TYPE(4i32);
pub const HttpEnabledStateActive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(0i32);
pub const HttpEnabledStateInactive: HTTP_ENABLED_STATE = HTTP_ENABLED_STATE(1i32);
pub const HttpFeatureApiTimings: HTTP_FEATURE_ID = HTTP_FEATURE_ID(2i32);
pub const HttpFeatureDelegateEx: HTTP_FEATURE_ID = HTTP_FEATURE_ID(3i32);
pub const HttpFeatureHttp3: HTTP_FEATURE_ID = HTTP_FEATURE_ID(4i32);
pub const HttpFeatureLast: HTTP_FEATURE_ID = HTTP_FEATURE_ID(5i32);
pub const HttpFeatureResponseTrailers: HTTP_FEATURE_ID = HTTP_FEATURE_ID(1i32);
pub const HttpFeatureUnknown: HTTP_FEATURE_ID = HTTP_FEATURE_ID(0i32);
pub const HttpFeaturemax: HTTP_FEATURE_ID = HTTP_FEATURE_ID(-1i32);
pub const HttpHeaderAccept: HTTP_HEADER_ID = HTTP_HEADER_ID(20i32);
pub const HttpHeaderAcceptCharset: HTTP_HEADER_ID = HTTP_HEADER_ID(21i32);
pub const HttpHeaderAcceptEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(22i32);
pub const HttpHeaderAcceptLanguage: HTTP_HEADER_ID = HTTP_HEADER_ID(23i32);
pub const HttpHeaderAcceptRanges: HTTP_HEADER_ID = HTTP_HEADER_ID(20i32);
pub const HttpHeaderAge: HTTP_HEADER_ID = HTTP_HEADER_ID(21i32);
pub const HttpHeaderAllow: HTTP_HEADER_ID = HTTP_HEADER_ID(10i32);
pub const HttpHeaderAuthorization: HTTP_HEADER_ID = HTTP_HEADER_ID(24i32);
pub const HttpHeaderCacheControl: HTTP_HEADER_ID = HTTP_HEADER_ID(0i32);
pub const HttpHeaderConnection: HTTP_HEADER_ID = HTTP_HEADER_ID(1i32);
pub const HttpHeaderContentEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(13i32);
pub const HttpHeaderContentLanguage: HTTP_HEADER_ID = HTTP_HEADER_ID(14i32);
pub const HttpHeaderContentLength: HTTP_HEADER_ID = HTTP_HEADER_ID(11i32);
pub const HttpHeaderContentLocation: HTTP_HEADER_ID = HTTP_HEADER_ID(15i32);
pub const HttpHeaderContentMd5: HTTP_HEADER_ID = HTTP_HEADER_ID(16i32);
pub const HttpHeaderContentRange: HTTP_HEADER_ID = HTTP_HEADER_ID(17i32);
pub const HttpHeaderContentType: HTTP_HEADER_ID = HTTP_HEADER_ID(12i32);
pub const HttpHeaderCookie: HTTP_HEADER_ID = HTTP_HEADER_ID(25i32);
pub const HttpHeaderDate: HTTP_HEADER_ID = HTTP_HEADER_ID(2i32);
pub const HttpHeaderEtag: HTTP_HEADER_ID = HTTP_HEADER_ID(22i32);
pub const HttpHeaderExpect: HTTP_HEADER_ID = HTTP_HEADER_ID(26i32);
pub const HttpHeaderExpires: HTTP_HEADER_ID = HTTP_HEADER_ID(18i32);
pub const HttpHeaderFrom: HTTP_HEADER_ID = HTTP_HEADER_ID(27i32);
pub const HttpHeaderHost: HTTP_HEADER_ID = HTTP_HEADER_ID(28i32);
pub const HttpHeaderIfMatch: HTTP_HEADER_ID = HTTP_HEADER_ID(29i32);
pub const HttpHeaderIfModifiedSince: HTTP_HEADER_ID = HTTP_HEADER_ID(30i32);
pub const HttpHeaderIfNoneMatch: HTTP_HEADER_ID = HTTP_HEADER_ID(31i32);
pub const HttpHeaderIfRange: HTTP_HEADER_ID = HTTP_HEADER_ID(32i32);
pub const HttpHeaderIfUnmodifiedSince: HTTP_HEADER_ID = HTTP_HEADER_ID(33i32);
pub const HttpHeaderKeepAlive: HTTP_HEADER_ID = HTTP_HEADER_ID(3i32);
pub const HttpHeaderLastModified: HTTP_HEADER_ID = HTTP_HEADER_ID(19i32);
pub const HttpHeaderLocation: HTTP_HEADER_ID = HTTP_HEADER_ID(23i32);
pub const HttpHeaderMaxForwards: HTTP_HEADER_ID = HTTP_HEADER_ID(34i32);
pub const HttpHeaderMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(41i32);
pub const HttpHeaderPragma: HTTP_HEADER_ID = HTTP_HEADER_ID(4i32);
pub const HttpHeaderProxyAuthenticate: HTTP_HEADER_ID = HTTP_HEADER_ID(24i32);
pub const HttpHeaderProxyAuthorization: HTTP_HEADER_ID = HTTP_HEADER_ID(35i32);
pub const HttpHeaderRange: HTTP_HEADER_ID = HTTP_HEADER_ID(37i32);
pub const HttpHeaderReferer: HTTP_HEADER_ID = HTTP_HEADER_ID(36i32);
pub const HttpHeaderRequestMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(41i32);
pub const HttpHeaderResponseMaximum: HTTP_HEADER_ID = HTTP_HEADER_ID(30i32);
pub const HttpHeaderRetryAfter: HTTP_HEADER_ID = HTTP_HEADER_ID(25i32);
pub const HttpHeaderServer: HTTP_HEADER_ID = HTTP_HEADER_ID(26i32);
pub const HttpHeaderSetCookie: HTTP_HEADER_ID = HTTP_HEADER_ID(27i32);
pub const HttpHeaderTe: HTTP_HEADER_ID = HTTP_HEADER_ID(38i32);
pub const HttpHeaderTrailer: HTTP_HEADER_ID = HTTP_HEADER_ID(5i32);
pub const HttpHeaderTransferEncoding: HTTP_HEADER_ID = HTTP_HEADER_ID(6i32);
pub const HttpHeaderTranslate: HTTP_HEADER_ID = HTTP_HEADER_ID(39i32);
pub const HttpHeaderUpgrade: HTTP_HEADER_ID = HTTP_HEADER_ID(7i32);
pub const HttpHeaderUserAgent: HTTP_HEADER_ID = HTTP_HEADER_ID(40i32);
pub const HttpHeaderVary: HTTP_HEADER_ID = HTTP_HEADER_ID(28i32);
pub const HttpHeaderVia: HTTP_HEADER_ID = HTTP_HEADER_ID(8i32);
pub const HttpHeaderWarning: HTTP_HEADER_ID = HTTP_HEADER_ID(9i32);
pub const HttpHeaderWwwAuthenticate: HTTP_HEADER_ID = HTTP_HEADER_ID(29i32);
pub const HttpLogDataTypeFields: HTTP_LOG_DATA_TYPE = HTTP_LOG_DATA_TYPE(0i32);
pub const HttpLoggingRolloverDaily: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(1i32);
pub const HttpLoggingRolloverHourly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(4i32);
pub const HttpLoggingRolloverMonthly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(3i32);
pub const HttpLoggingRolloverSize: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(0i32);
pub const HttpLoggingRolloverWeekly: HTTP_LOGGING_ROLLOVER_TYPE = HTTP_LOGGING_ROLLOVER_TYPE(2i32);
pub const HttpLoggingTypeIIS: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(1i32);
pub const HttpLoggingTypeNCSA: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(2i32);
pub const HttpLoggingTypeRaw: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(3i32);
pub const HttpLoggingTypeW3C: HTTP_LOGGING_TYPE = HTTP_LOGGING_TYPE(0i32);
pub const HttpNone: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(0i32);
pub const HttpProtectionLevelEdgeRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(1i32);
pub const HttpProtectionLevelRestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(2i32);
pub const HttpProtectionLevelUnrestricted: HTTP_PROTECTION_LEVEL_TYPE = HTTP_PROTECTION_LEVEL_TYPE(0i32);
pub const HttpQosSettingTypeBandwidth: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(0i32);
pub const HttpQosSettingTypeConnectionLimit: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(1i32);
pub const HttpQosSettingTypeFlowRate: HTTP_QOS_SETTING_TYPE = HTTP_QOS_SETTING_TYPE(2i32);
pub const HttpRequestAuthTypeBasic: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(1i32);
pub const HttpRequestAuthTypeDigest: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(2i32);
pub const HttpRequestAuthTypeKerberos: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(5i32);
pub const HttpRequestAuthTypeNTLM: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(3i32);
pub const HttpRequestAuthTypeNegotiate: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(4i32);
pub const HttpRequestAuthTypeNone: HTTP_REQUEST_AUTH_TYPE = HTTP_REQUEST_AUTH_TYPE(0i32);
pub const HttpRequestInfoTypeAuth: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(0i32);
pub const HttpRequestInfoTypeChannelBind: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(1i32);
pub const HttpRequestInfoTypeQuicStats: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(8i32);
pub const HttpRequestInfoTypeRequestSizing: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(7i32);
pub const HttpRequestInfoTypeRequestTiming: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(5i32);
pub const HttpRequestInfoTypeSslProtocol: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(2i32);
pub const HttpRequestInfoTypeSslTokenBinding: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(4i32);
pub const HttpRequestInfoTypeSslTokenBindingDraft: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(3i32);
pub const HttpRequestInfoTypeTcpInfoV0: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(6i32);
pub const HttpRequestInfoTypeTcpInfoV1: HTTP_REQUEST_INFO_TYPE = HTTP_REQUEST_INFO_TYPE(9i32);
pub const HttpRequestPropertyIsb: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(0i32);
pub const HttpRequestPropertyQuicApiTimings: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(7i32);
pub const HttpRequestPropertyQuicStats: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(2i32);
pub const HttpRequestPropertySni: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(4i32);
pub const HttpRequestPropertyStreamError: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(5i32);
pub const HttpRequestPropertyTcpInfoV0: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(1i32);
pub const HttpRequestPropertyTcpInfoV1: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(3i32);
pub const HttpRequestPropertyWskApiTimings: HTTP_REQUEST_PROPERTY = HTTP_REQUEST_PROPERTY(6i32);
pub const HttpRequestSizingTypeHeaders: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(4i32);
pub const HttpRequestSizingTypeMax: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(5i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg1ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(0i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg1ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(1i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg2ClientData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(2i32);
pub const HttpRequestSizingTypeTlsHandshakeLeg2ServerData: HTTP_REQUEST_SIZING_TYPE = HTTP_REQUEST_SIZING_TYPE(3i32);
pub const HttpRequestTimingTypeConnectionStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(0i32);
pub const HttpRequestTimingTypeDataStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(1i32);
pub const HttpRequestTimingTypeHttp2HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(14i32);
pub const HttpRequestTimingTypeHttp2HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(13i32);
pub const HttpRequestTimingTypeHttp2StreamStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(12i32);
pub const HttpRequestTimingTypeHttp3HeaderDecodeEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(29i32);
pub const HttpRequestTimingTypeHttp3HeaderDecodeStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(28i32);
pub const HttpRequestTimingTypeHttp3StreamStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(27i32);
pub const HttpRequestTimingTypeMax: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(30i32);
pub const HttpRequestTimingTypeRequestDeliveredForDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(23i32);
pub const HttpRequestTimingTypeRequestDeliveredForIO: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(26i32);
pub const HttpRequestTimingTypeRequestDeliveredForInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(20i32);
pub const HttpRequestTimingTypeRequestHeaderParseEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(16i32);
pub const HttpRequestTimingTypeRequestHeaderParseStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(15i32);
pub const HttpRequestTimingTypeRequestQueuedForDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(22i32);
pub const HttpRequestTimingTypeRequestQueuedForIO: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(25i32);
pub const HttpRequestTimingTypeRequestQueuedForInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(19i32);
pub const HttpRequestTimingTypeRequestReturnedAfterDelegation: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(24i32);
pub const HttpRequestTimingTypeRequestReturnedAfterInspection: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(21i32);
pub const HttpRequestTimingTypeRequestRoutingEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(18i32);
pub const HttpRequestTimingTypeRequestRoutingStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(17i32);
pub const HttpRequestTimingTypeTlsAttributesQueryEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(9i32);
pub const HttpRequestTimingTypeTlsAttributesQueryStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(8i32);
pub const HttpRequestTimingTypeTlsCertificateLoadEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(3i32);
pub const HttpRequestTimingTypeTlsCertificateLoadStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(2i32);
pub const HttpRequestTimingTypeTlsClientCertQueryEnd: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(11i32);
pub const HttpRequestTimingTypeTlsClientCertQueryStart: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(10i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg1End: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(5i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg1Start: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(4i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg2End: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(7i32);
pub const HttpRequestTimingTypeTlsHandshakeLeg2Start: HTTP_REQUEST_TIMING_TYPE = HTTP_REQUEST_TIMING_TYPE(6i32);
pub const HttpResponseInfoTypeAuthenticationProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(1i32);
pub const HttpResponseInfoTypeChannelBind: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(3i32);
pub const HttpResponseInfoTypeMultipleKnownHeaders: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(0i32);
pub const HttpResponseInfoTypeQoSProperty: HTTP_RESPONSE_INFO_TYPE = HTTP_RESPONSE_INFO_TYPE(2i32);
pub const HttpSchemeHttp: HTTP_SCHEME = HTTP_SCHEME(0i32);
pub const HttpSchemeHttps: HTTP_SCHEME = HTTP_SCHEME(1i32);
pub const HttpSchemeMaximum: HTTP_SCHEME = HTTP_SCHEME(2i32);
pub const HttpServer503VerbosityProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(6i32);
pub const HttpServerAuthenticationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(0i32);
pub const HttpServerBindingProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(7i32);
pub const HttpServerChannelBindProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(10i32);
pub const HttpServerDelegationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(16i32);
pub const HttpServerExtendedAuthenticationProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(8i32);
pub const HttpServerListenEndpointProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(9i32);
pub const HttpServerLoggingProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(1i32);
pub const HttpServerProtectionLevelProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(11i32);
pub const HttpServerQosProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(2i32);
pub const HttpServerQueueLengthProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(4i32);
pub const HttpServerStateProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(5i32);
pub const HttpServerTimeoutsProperty: HTTP_SERVER_PROPERTY = HTTP_SERVER_PROPERTY(3i32);
pub const HttpServiceBindingTypeA: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(2i32);
pub const HttpServiceBindingTypeNone: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(0i32);
pub const HttpServiceBindingTypeW: HTTP_SERVICE_BINDING_TYPE = HTTP_SERVICE_BINDING_TYPE(1i32);
pub const HttpServiceConfigCache: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(4i32);
pub const HttpServiceConfigIPListenList: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(0i32);
pub const HttpServiceConfigMax: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(13i32);
pub const HttpServiceConfigQueryExact: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(0i32);
pub const HttpServiceConfigQueryMax: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(2i32);
pub const HttpServiceConfigQueryNext: HTTP_SERVICE_CONFIG_QUERY_TYPE = HTTP_SERVICE_CONFIG_QUERY_TYPE(1i32);
pub const HttpServiceConfigSSLCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(1i32);
pub const HttpServiceConfigSetting: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(7i32);
pub const HttpServiceConfigSslCcsCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(6i32);
pub const HttpServiceConfigSslCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(10i32);
pub const HttpServiceConfigSslCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(8i32);
pub const HttpServiceConfigSslScopedCcsCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(11i32);
pub const HttpServiceConfigSslScopedCcsCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(12i32);
pub const HttpServiceConfigSslSniCertInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(5i32);
pub const HttpServiceConfigSslSniCertInfoEx: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(9i32);
pub const HttpServiceConfigTimeout: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(3i32);
pub const HttpServiceConfigUrlAclInfo: HTTP_SERVICE_CONFIG_ID = HTTP_SERVICE_CONFIG_ID(2i32);
pub const HttpTlsThrottle: HTTP_SERVICE_CONFIG_SETTING_KEY = HTTP_SERVICE_CONFIG_SETTING_KEY(1i32);
pub const HttpVerbCONNECT: HTTP_VERB = HTTP_VERB(10i32);
pub const HttpVerbCOPY: HTTP_VERB = HTTP_VERB(13i32);
pub const HttpVerbDELETE: HTTP_VERB = HTTP_VERB(8i32);
pub const HttpVerbGET: HTTP_VERB = HTTP_VERB(4i32);
pub const HttpVerbHEAD: HTTP_VERB = HTTP_VERB(5i32);
pub const HttpVerbInvalid: HTTP_VERB = HTTP_VERB(2i32);
pub const HttpVerbLOCK: HTTP_VERB = HTTP_VERB(17i32);
pub const HttpVerbMKCOL: HTTP_VERB = HTTP_VERB(16i32);
pub const HttpVerbMOVE: HTTP_VERB = HTTP_VERB(12i32);
pub const HttpVerbMaximum: HTTP_VERB = HTTP_VERB(20i32);
pub const HttpVerbOPTIONS: HTTP_VERB = HTTP_VERB(3i32);
pub const HttpVerbPOST: HTTP_VERB = HTTP_VERB(6i32);
pub const HttpVerbPROPFIND: HTTP_VERB = HTTP_VERB(14i32);
pub const HttpVerbPROPPATCH: HTTP_VERB = HTTP_VERB(15i32);
pub const HttpVerbPUT: HTTP_VERB = HTTP_VERB(7i32);
pub const HttpVerbSEARCH: HTTP_VERB = HTTP_VERB(19i32);
pub const HttpVerbTRACE: HTTP_VERB = HTTP_VERB(9i32);
pub const HttpVerbTRACK: HTTP_VERB = HTTP_VERB(11i32);
pub const HttpVerbUNLOCK: HTTP_VERB = HTTP_VERB(18i32);
pub const HttpVerbUnknown: HTTP_VERB = HTTP_VERB(1i32);
pub const HttpVerbUnparsed: HTTP_VERB = HTTP_VERB(0i32);
pub const IdleConnectionTimeout: HTTP_SERVICE_CONFIG_TIMEOUT_KEY = HTTP_SERVICE_CONFIG_TIMEOUT_KEY(0i32);
pub const MaxCacheResponseSize: HTTP_SERVICE_CONFIG_CACHE_KEY = HTTP_SERVICE_CONFIG_CACHE_KEY(0i32);
pub const PerformanceParamAggressiveICW: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(1i32);
pub const PerformanceParamDecryptOnSspiThread: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(5i32);
pub const PerformanceParamMax: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(6i32);
pub const PerformanceParamMaxConcurrentClientStreams: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(3i32);
pub const PerformanceParamMaxReceiveBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(4i32);
pub const PerformanceParamMaxSendBufferSize: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(2i32);
pub const PerformanceParamSendBufferingFlags: HTTP_PERFORMANCE_PARAM_TYPE = HTTP_PERFORMANCE_PARAM_TYPE(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_503_RESPONSE_VERBOSITY(pub i32);
impl windows_core::TypeKind for HTTP_503_RESPONSE_VERBOSITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_503_RESPONSE_VERBOSITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_503_RESPONSE_VERBOSITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_AUTHENTICATION_HARDENING_LEVELS(pub i32);
impl windows_core::TypeKind for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_AUTHENTICATION_HARDENING_LEVELS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_AUTHENTICATION_HARDENING_LEVELS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_AUTH_STATUS(pub i32);
impl windows_core::TypeKind for HTTP_AUTH_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_AUTH_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_AUTH_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_CACHE_POLICY_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_CACHE_POLICY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_CACHE_POLICY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_CACHE_POLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_DATA_CHUNK_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_DATA_CHUNK_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_DATA_CHUNK_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_DATA_CHUNK_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_DELEGATE_REQUEST_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_DELEGATE_REQUEST_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_ENABLED_STATE(pub i32);
impl windows_core::TypeKind for HTTP_ENABLED_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_ENABLED_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_ENABLED_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_FEATURE_ID(pub i32);
impl windows_core::TypeKind for HTTP_FEATURE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_FEATURE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_FEATURE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_HEADER_ID(pub i32);
impl windows_core::TypeKind for HTTP_HEADER_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_HEADER_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_HEADER_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_INITIALIZE(pub u32);
impl windows_core::TypeKind for HTTP_INITIALIZE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_INITIALIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_INITIALIZE").field(&self.0).finish()
    }
}
impl HTTP_INITIALIZE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HTTP_INITIALIZE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HTTP_INITIALIZE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HTTP_INITIALIZE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HTTP_INITIALIZE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HTTP_INITIALIZE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_LOGGING_ROLLOVER_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_LOGGING_ROLLOVER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_LOGGING_ROLLOVER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_LOGGING_ROLLOVER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_LOGGING_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_LOGGING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_LOGGING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_LOGGING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_LOG_DATA_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_LOG_DATA_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_LOG_DATA_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_LOG_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_PERFORMANCE_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_PERFORMANCE_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_PERFORMANCE_PARAM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_PERFORMANCE_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_PROTECTION_LEVEL_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_PROTECTION_LEVEL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_PROTECTION_LEVEL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_PROTECTION_LEVEL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_QOS_SETTING_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_QOS_SETTING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_QOS_SETTING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_QOS_SETTING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_RECEIVE_HTTP_REQUEST_FLAGS(pub u32);
impl windows_core::TypeKind for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_RECEIVE_HTTP_REQUEST_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_RECEIVE_HTTP_REQUEST_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_REQUEST_AUTH_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_REQUEST_AUTH_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_REQUEST_AUTH_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_AUTH_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_REQUEST_INFO_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_REQUEST_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_REQUEST_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_REQUEST_PROPERTY(pub i32);
impl windows_core::TypeKind for HTTP_REQUEST_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_REQUEST_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_REQUEST_SIZING_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_REQUEST_SIZING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_REQUEST_SIZING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_SIZING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_REQUEST_TIMING_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_REQUEST_TIMING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_REQUEST_TIMING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_REQUEST_TIMING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_RESPONSE_INFO_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_RESPONSE_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_RESPONSE_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_RESPONSE_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SCHEME(pub i32);
impl windows_core::TypeKind for HTTP_SCHEME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SCHEME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SCHEME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SERVER_PROPERTY(pub i32);
impl windows_core::TypeKind for HTTP_SERVER_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SERVER_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SERVER_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SERVICE_BINDING_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_SERVICE_BINDING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SERVICE_BINDING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_BINDING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SERVICE_CONFIG_CACHE_KEY(pub i32);
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_CACHE_KEY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SERVICE_CONFIG_CACHE_KEY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_CACHE_KEY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SERVICE_CONFIG_ID(pub i32);
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SERVICE_CONFIG_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SERVICE_CONFIG_QUERY_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SERVICE_CONFIG_QUERY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_QUERY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SERVICE_CONFIG_SETTING_KEY(pub i32);
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SETTING_KEY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SERVICE_CONFIG_SETTING_KEY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_SETTING_KEY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_KEY(pub i32);
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SERVICE_CONFIG_TIMEOUT_KEY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SERVICE_CONFIG_TIMEOUT_KEY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HTTP_VERB(pub i32);
impl windows_core::TypeKind for HTTP_VERB {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HTTP_VERB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HTTP_VERB").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP2_SETTINGS_LIMITS_PARAM {
    pub Http2MaxSettingsPerFrame: u32,
    pub Http2MaxSettingsPerMinute: u32,
}
impl windows_core::TypeKind for HTTP2_SETTINGS_LIMITS_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP2_SETTINGS_LIMITS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP2_WINDOW_SIZE_PARAM {
    pub Http2ReceiveWindowSize: u32,
}
impl windows_core::TypeKind for HTTP2_WINDOW_SIZE_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP2_WINDOW_SIZE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTPAPI_VERSION {
    pub HttpApiMajorVersion: u16,
    pub HttpApiMinorVersion: u16,
}
impl windows_core::TypeKind for HTTPAPI_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTPAPI_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_BANDWIDTH_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
}
impl windows_core::TypeKind for HTTP_BANDWIDTH_LIMIT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_BANDWIDTH_LIMIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_BINDING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub RequestQueueHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for HTTP_BINDING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_BYTE_RANGE {
    pub StartingOffset: u64,
    pub Length: u64,
}
impl windows_core::TypeKind for HTTP_BYTE_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_BYTE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_CACHE_POLICY {
    pub Policy: HTTP_CACHE_POLICY_TYPE,
    pub SecondsToLive: u32,
}
impl windows_core::TypeKind for HTTP_CACHE_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_CACHE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_CHANNEL_BIND_INFO {
    pub Hardening: HTTP_AUTHENTICATION_HARDENING_LEVELS,
    pub Flags: u32,
    pub ServiceNames: *mut *mut HTTP_SERVICE_BINDING_BASE,
    pub NumberOfServiceNames: u32,
}
impl windows_core::TypeKind for HTTP_CHANNEL_BIND_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_CHANNEL_BIND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_CONNECTION_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxConnections: u32,
}
impl windows_core::TypeKind for HTTP_CONNECTION_LIMIT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_CONNECTION_LIMIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_COOKED_URL {
    pub FullUrlLength: u16,
    pub HostLength: u16,
    pub AbsPathLength: u16,
    pub QueryStringLength: u16,
    pub pFullUrl: windows_core::PCWSTR,
    pub pHost: windows_core::PCWSTR,
    pub pAbsPath: windows_core::PCWSTR,
    pub pQueryString: windows_core::PCWSTR,
}
impl windows_core::TypeKind for HTTP_COOKED_URL {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_COOKED_URL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    pub PropertyId: HTTP_CREATE_REQUEST_QUEUE_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_CREATE_REQUEST_QUEUE_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_DATA_CHUNK {
    pub DataChunkType: HTTP_DATA_CHUNK_TYPE,
    pub Anonymous: HTTP_DATA_CHUNK_0,
}
impl windows_core::TypeKind for HTTP_DATA_CHUNK {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DATA_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HTTP_DATA_CHUNK_0 {
    pub FromMemory: HTTP_DATA_CHUNK_0_3,
    pub FromFileHandle: HTTP_DATA_CHUNK_0_0,
    pub FromFragmentCache: HTTP_DATA_CHUNK_0_2,
    pub FromFragmentCacheEx: HTTP_DATA_CHUNK_0_1,
    pub Trailers: HTTP_DATA_CHUNK_0_4,
}
impl windows_core::TypeKind for HTTP_DATA_CHUNK_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DATA_CHUNK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_0 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub FileHandle: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for HTTP_DATA_CHUNK_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DATA_CHUNK_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_1 {
    pub ByteRange: HTTP_BYTE_RANGE,
    pub pFragmentName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for HTTP_DATA_CHUNK_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DATA_CHUNK_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_2 {
    pub FragmentNameLength: u16,
    pub pFragmentName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for HTTP_DATA_CHUNK_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DATA_CHUNK_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_3 {
    pub pBuffer: *mut core::ffi::c_void,
    pub BufferLength: u32,
}
impl windows_core::TypeKind for HTTP_DATA_CHUNK_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DATA_CHUNK_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_DATA_CHUNK_0_4 {
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
}
impl windows_core::TypeKind for HTTP_DATA_CHUNK_0_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DATA_CHUNK_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    pub PropertyId: HTTP_DELEGATE_REQUEST_PROPERTY_ID,
    pub PropertyInfoLength: u32,
    pub PropertyInfo: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_DELEGATE_REQUEST_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_ERROR_HEADERS_PARAM {
    pub StatusCode: u16,
    pub HeaderCount: u16,
    pub Headers: *mut HTTP_UNKNOWN_HEADER,
}
impl windows_core::TypeKind for HTTP_ERROR_HEADERS_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_ERROR_HEADERS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_FLOWRATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub MaxBandwidth: u32,
    pub MaxPeakBandwidth: u32,
    pub BurstSize: u32,
}
impl windows_core::TypeKind for HTTP_FLOWRATE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_FLOWRATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_KNOWN_HEADER {
    pub RawValueLength: u16,
    pub pRawValue: windows_core::PCSTR,
}
impl windows_core::TypeKind for HTTP_KNOWN_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_KNOWN_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_LISTEN_ENDPOINT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EnableSharing: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for HTTP_LISTEN_ENDPOINT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_LISTEN_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_LOGGING_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub LoggingFlags: u32,
    pub SoftwareName: windows_core::PCWSTR,
    pub SoftwareNameLength: u16,
    pub DirectoryNameLength: u16,
    pub DirectoryName: windows_core::PCWSTR,
    pub Format: HTTP_LOGGING_TYPE,
    pub Fields: u32,
    pub pExtFields: *mut core::ffi::c_void,
    pub NumOfExtFields: u16,
    pub MaxRecordSize: u16,
    pub RolloverType: HTTP_LOGGING_ROLLOVER_TYPE,
    pub RolloverSize: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for HTTP_LOGGING_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for HTTP_LOGGING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_LOG_DATA {
    pub Type: HTTP_LOG_DATA_TYPE,
}
impl windows_core::TypeKind for HTTP_LOG_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_LOG_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_LOG_FIELDS_DATA {
    pub Base: HTTP_LOG_DATA,
    pub UserNameLength: u16,
    pub UriStemLength: u16,
    pub ClientIpLength: u16,
    pub ServerNameLength: u16,
    pub ServiceNameLength: u16,
    pub ServerIpLength: u16,
    pub MethodLength: u16,
    pub UriQueryLength: u16,
    pub HostLength: u16,
    pub UserAgentLength: u16,
    pub CookieLength: u16,
    pub ReferrerLength: u16,
    pub UserName: windows_core::PWSTR,
    pub UriStem: windows_core::PWSTR,
    pub ClientIp: windows_core::PSTR,
    pub ServerName: windows_core::PSTR,
    pub ServiceName: windows_core::PSTR,
    pub ServerIp: windows_core::PSTR,
    pub Method: windows_core::PSTR,
    pub UriQuery: windows_core::PSTR,
    pub Host: windows_core::PSTR,
    pub UserAgent: windows_core::PSTR,
    pub Cookie: windows_core::PSTR,
    pub Referrer: windows_core::PSTR,
    pub ServerPort: u16,
    pub ProtocolStatus: u16,
    pub Win32Status: u32,
    pub MethodNum: HTTP_VERB,
    pub SubStatus: u16,
}
impl windows_core::TypeKind for HTTP_LOG_FIELDS_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_LOG_FIELDS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_MULTIPLE_KNOWN_HEADERS {
    pub HeaderId: HTTP_HEADER_ID,
    pub Flags: u32,
    pub KnownHeaderCount: u16,
    pub KnownHeaders: *mut HTTP_KNOWN_HEADER,
}
impl windows_core::TypeKind for HTTP_MULTIPLE_KNOWN_HEADERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_MULTIPLE_KNOWN_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_PERFORMANCE_PARAM {
    pub Type: HTTP_PERFORMANCE_PARAM_TYPE,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_PERFORMANCE_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_PERFORMANCE_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_PROPERTY_FLAGS {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for HTTP_PROPERTY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_PROPERTY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_PROTECTION_LEVEL_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub Level: HTTP_PROTECTION_LEVEL_TYPE,
}
impl windows_core::TypeKind for HTTP_PROTECTION_LEVEL_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_PROTECTION_LEVEL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_QOS_SETTING_INFO {
    pub QosType: HTTP_QOS_SETTING_TYPE,
    pub QosSetting: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_QOS_SETTING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_QOS_SETTING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    pub Freshness: u64,
}
impl windows_core::TypeKind for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_QUERY_REQUEST_QUALIFIER_QUIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    pub Freshness: u64,
}
impl windows_core::TypeKind for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_QUERY_REQUEST_QUALIFIER_TCP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_QUIC_API_TIMINGS {
    pub ConnectionTimings: HTTP_QUIC_CONNECTION_API_TIMINGS,
    pub StreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
impl windows_core::TypeKind for HTTP_QUIC_API_TIMINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_QUIC_API_TIMINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_QUIC_CONNECTION_API_TIMINGS {
    pub OpenTime: u64,
    pub CloseTime: u64,
    pub StartTime: u64,
    pub ShutdownTime: u64,
    pub SecConfigCreateTime: u64,
    pub SecConfigDeleteTime: u64,
    pub GetParamCount: u64,
    pub GetParamSum: u64,
    pub SetParamCount: u64,
    pub SetParamSum: u64,
    pub SetCallbackHandlerCount: u64,
    pub SetCallbackHandlerSum: u64,
    pub ControlStreamTimings: HTTP_QUIC_STREAM_API_TIMINGS,
}
impl windows_core::TypeKind for HTTP_QUIC_CONNECTION_API_TIMINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_QUIC_CONNECTION_API_TIMINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_QUIC_STREAM_API_TIMINGS {
    pub OpenCount: u64,
    pub OpenSum: u64,
    pub CloseCount: u64,
    pub CloseSum: u64,
    pub StartCount: u64,
    pub StartSum: u64,
    pub ShutdownCount: u64,
    pub ShutdownSum: u64,
    pub SendCount: u64,
    pub SendSum: u64,
    pub ReceiveSetEnabledCount: u64,
    pub ReceiveSetEnabledSum: u64,
    pub GetParamCount: u64,
    pub GetParamSum: u64,
    pub SetParamCount: u64,
    pub SetParamSum: u64,
    pub SetCallbackHandlerCount: u64,
    pub SetCallbackHandlerSum: u64,
}
impl windows_core::TypeKind for HTTP_QUIC_STREAM_API_TIMINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_QUIC_STREAM_API_TIMINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_QUIC_STREAM_REQUEST_STATS {
    pub StreamWaitStart: u64,
    pub StreamWaitEnd: u64,
    pub RequestHeadersCompressionStart: u64,
    pub RequestHeadersCompressionEnd: u64,
    pub ResponseHeadersDecompressionStart: u64,
    pub ResponseHeadersDecompressionEnd: u64,
    pub RequestHeadersCompressedSize: u64,
    pub ResponseHeadersCompressedSize: u64,
}
impl windows_core::TypeKind for HTTP_QUIC_STREAM_REQUEST_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_QUIC_STREAM_REQUEST_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_AUTH_INFO {
    pub AuthStatus: HTTP_AUTH_STATUS,
    pub SecStatus: windows_core::HRESULT,
    pub Flags: u32,
    pub AuthType: HTTP_REQUEST_AUTH_TYPE,
    pub AccessToken: super::super::Foundation::HANDLE,
    pub ContextAttributes: u32,
    pub PackedContextLength: u32,
    pub PackedContextType: u32,
    pub PackedContext: *mut core::ffi::c_void,
    pub MutualAuthDataLength: u32,
    pub pMutualAuthData: windows_core::PSTR,
    pub PackageNameLength: u16,
    pub pPackageName: windows_core::PWSTR,
}
impl windows_core::TypeKind for HTTP_REQUEST_AUTH_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_AUTH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_CHANNEL_BIND_STATUS {
    pub ServiceName: *mut HTTP_SERVICE_BINDING_BASE,
    pub ChannelToken: *mut u8,
    pub ChannelTokenSize: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_CHANNEL_BIND_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 41],
}
impl windows_core::TypeKind for HTTP_REQUEST_HEADERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_INFO {
    pub InfoType: HTTP_REQUEST_INFO_TYPE,
    pub InfoLength: u32,
    pub pInfo: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_REQUEST_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_PROPERTY_SNI {
    pub Hostname: [u16; 256],
    pub Flags: u32,
}
impl windows_core::TypeKind for HTTP_REQUEST_PROPERTY_SNI {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_PROPERTY_SNI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    pub ErrorCode: u32,
}
impl windows_core::TypeKind for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_PROPERTY_STREAM_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_SIZING_INFO {
    pub Flags: u64,
    pub RequestIndex: u32,
    pub RequestSizingCount: u32,
    pub RequestSizing: [u64; 5],
}
impl windows_core::TypeKind for HTTP_REQUEST_SIZING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_SIZING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_TIMING_INFO {
    pub RequestTimingCount: u32,
    pub RequestTiming: [u64; 30],
}
impl windows_core::TypeKind for HTTP_REQUEST_TIMING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_TIMING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_TOKEN_BINDING_INFO {
    pub TokenBinding: *mut u8,
    pub TokenBindingSize: u32,
    pub EKM: *mut u8,
    pub EKMSize: u32,
    pub KeyType: u8,
}
impl windows_core::TypeKind for HTTP_REQUEST_TOKEN_BINDING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_REQUEST_TOKEN_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_V1 {
    pub Flags: u32,
    pub ConnectionId: u64,
    pub RequestId: u64,
    pub UrlContext: u64,
    pub Version: HTTP_VERSION,
    pub Verb: HTTP_VERB,
    pub UnknownVerbLength: u16,
    pub RawUrlLength: u16,
    pub pUnknownVerb: windows_core::PCSTR,
    pub pRawUrl: windows_core::PCSTR,
    pub CookedUrl: HTTP_COOKED_URL,
    pub Address: HTTP_TRANSPORT_ADDRESS,
    pub Headers: HTTP_REQUEST_HEADERS,
    pub BytesReceived: u64,
    pub EntityChunkCount: u16,
    pub pEntityChunks: *mut HTTP_DATA_CHUNK,
    pub RawConnectionId: u64,
    pub pSslInfo: *mut HTTP_SSL_INFO,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_REQUEST_V1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_REQUEST_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_REQUEST_V2 {
    pub Base: HTTP_REQUEST_V1,
    pub RequestInfoCount: u16,
    pub pRequestInfo: *mut HTTP_REQUEST_INFO,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_REQUEST_V2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_REQUEST_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_RESPONSE_HEADERS {
    pub UnknownHeaderCount: u16,
    pub pUnknownHeaders: *mut HTTP_UNKNOWN_HEADER,
    pub TrailerCount: u16,
    pub pTrailers: *mut HTTP_UNKNOWN_HEADER,
    pub KnownHeaders: [HTTP_KNOWN_HEADER; 30],
}
impl windows_core::TypeKind for HTTP_RESPONSE_HEADERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_RESPONSE_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_RESPONSE_INFO {
    pub Type: HTTP_RESPONSE_INFO_TYPE,
    pub Length: u32,
    pub pInfo: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_RESPONSE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_RESPONSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_RESPONSE_V1 {
    pub Flags: u32,
    pub Version: HTTP_VERSION,
    pub StatusCode: u16,
    pub ReasonLength: u16,
    pub pReason: windows_core::PCSTR,
    pub Headers: HTTP_RESPONSE_HEADERS,
    pub EntityChunkCount: u16,
    pub pEntityChunks: *mut HTTP_DATA_CHUNK,
}
impl windows_core::TypeKind for HTTP_RESPONSE_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_RESPONSE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_RESPONSE_V2 {
    pub Base: HTTP_RESPONSE_V1,
    pub ResponseInfoCount: u16,
    pub pResponseInfo: *mut HTTP_RESPONSE_INFO,
}
impl windows_core::TypeKind for HTTP_RESPONSE_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_RESPONSE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    pub RealmLength: u16,
    pub Realm: windows_core::PWSTR,
}
impl windows_core::TypeKind for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    pub DomainNameLength: u16,
    pub DomainName: windows_core::PWSTR,
    pub RealmLength: u16,
    pub Realm: windows_core::PWSTR,
}
impl windows_core::TypeKind for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVER_AUTHENTICATION_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub AuthSchemes: u32,
    pub ReceiveMutualAuth: super::super::Foundation::BOOLEAN,
    pub ReceiveContextHandle: super::super::Foundation::BOOLEAN,
    pub DisableNTLMCredentialCaching: super::super::Foundation::BOOLEAN,
    pub ExFlags: u8,
    pub DigestParams: HTTP_SERVER_AUTHENTICATION_DIGEST_PARAMS,
    pub BasicParams: HTTP_SERVER_AUTHENTICATION_BASIC_PARAMS,
}
impl windows_core::TypeKind for HTTP_SERVER_AUTHENTICATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVER_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_BINDING_A {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: windows_core::PSTR,
    pub BufferSize: u32,
}
impl windows_core::TypeKind for HTTP_SERVICE_BINDING_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_BINDING_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_BINDING_BASE {
    pub Type: HTTP_SERVICE_BINDING_TYPE,
}
impl windows_core::TypeKind for HTTP_SERVICE_BINDING_BASE {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_BINDING_BASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_BINDING_W {
    pub Base: HTTP_SERVICE_BINDING_BASE,
    pub Buffer: windows_core::PWSTR,
    pub BufferSize: u32,
}
impl windows_core::TypeKind for HTTP_SERVICE_BINDING_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_BINDING_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_CACHE_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_CACHE_KEY,
    pub ParamDesc: u32,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_CACHE_SET {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_CACHE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    pub AddrLength: u16,
    pub pAddress: *mut super::WinSock::SOCKADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_IP_LISTEN_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    pub AddrCount: u32,
    pub AddrList: [super::WinSock::SOCKADDR_STORAGE; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_IP_LISTEN_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SETTING_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SETTING_KEY,
    pub ParamDesc: u32,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SETTING_SET {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_SETTING_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    pub LocalAddress: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_CCS_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_CCS_QUERY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_CCS_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_CCS_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY {
    pub pIpPort: *mut super::WinSock::SOCKADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_KEY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    pub IpPort: super::WinSock::SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_KEY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM {
    pub SslHashLength: u32,
    pub pSslHash: *mut core::ffi::c_void,
    pub AppId: windows_core::GUID,
    pub pSslCertStoreName: windows_core::PWSTR,
    pub DefaultCertCheckMode: u32,
    pub DefaultRevocationFreshnessTime: u32,
    pub DefaultRevocationUrlRetrievalTimeout: u32,
    pub pDefaultSslCtlIdentifier: windows_core::PWSTR,
    pub pDefaultSslCtlStoreName: windows_core::PWSTR,
    pub DefaultFlags: u32,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
    pub Flags: u64,
    pub Anonymous: HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    pub Http2WindowSizeParam: HTTP2_WINDOW_SIZE_PARAM,
    pub Http2SettingsLimitsParam: HTTP2_SETTINGS_LIMITS_PARAM,
    pub HttpPerformanceParam: HTTP_PERFORMANCE_PARAM,
    pub HttpTlsRestrictionsParam: HTTP_TLS_RESTRICTIONS_PARAM,
    pub HttpErrorHeadersParam: HTTP_ERROR_HEADERS_PARAM,
    pub HttpTlsSessionTicketKeysParam: HTTP_TLS_SESSION_TICKET_KEYS_PARAM,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_SSL_PARAM_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub dwToken: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_QUERY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_QUERY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_SET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_KEY_EX,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    pub IpPort: super::WinSock::SOCKADDR_STORAGE,
    pub Host: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub dwToken: u32,
    pub ParamType: HTTP_SSL_SERVICE_CONFIG_EX_PARAM_TYPE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_QUERY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    pub KeyDesc: HTTP_SERVICE_CONFIG_SSL_SNI_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_SSL_PARAM_EX,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_SERVICE_CONFIG_SSL_SNI_SET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_TIMEOUT_KEY,
    pub ParamDesc: u16,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_TIMEOUT_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_KEY {
    pub pUrlPrefix: windows_core::PWSTR,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_URLACL_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_URLACL_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_PARAM {
    pub pStringSecurityDescriptor: windows_core::PWSTR,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_URLACL_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_QUERY {
    pub QueryDesc: HTTP_SERVICE_CONFIG_QUERY_TYPE,
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub dwToken: u32,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_URLACL_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SERVICE_CONFIG_URLACL_SET {
    pub KeyDesc: HTTP_SERVICE_CONFIG_URLACL_KEY,
    pub ParamDesc: HTTP_SERVICE_CONFIG_URLACL_PARAM,
}
impl windows_core::TypeKind for HTTP_SERVICE_CONFIG_URLACL_SET {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SERVICE_CONFIG_URLACL_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SSL_CLIENT_CERT_INFO {
    pub CertFlags: u32,
    pub CertEncodedSize: u32,
    pub pCertEncoded: *mut u8,
    pub Token: super::super::Foundation::HANDLE,
    pub CertDeniedByMapper: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for HTTP_SSL_CLIENT_CERT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SSL_CLIENT_CERT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SSL_INFO {
    pub ServerCertKeySize: u16,
    pub ConnectionKeySize: u16,
    pub ServerCertIssuerSize: u32,
    pub ServerCertSubjectSize: u32,
    pub pServerCertIssuer: windows_core::PCSTR,
    pub pServerCertSubject: windows_core::PCSTR,
    pub pClientCertInfo: *mut HTTP_SSL_CLIENT_CERT_INFO,
    pub SslClientCertNegotiated: u32,
}
impl windows_core::TypeKind for HTTP_SSL_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SSL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_SSL_PROTOCOL_INFO {
    pub Protocol: u32,
    pub CipherType: u32,
    pub CipherStrength: u32,
    pub HashType: u32,
    pub HashStrength: u32,
    pub KeyExchangeType: u32,
    pub KeyExchangeStrength: u32,
}
impl windows_core::TypeKind for HTTP_SSL_PROTOCOL_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_SSL_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_STATE_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub State: HTTP_ENABLED_STATE,
}
impl windows_core::TypeKind for HTTP_STATE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_STATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_TIMEOUT_LIMIT_INFO {
    pub Flags: HTTP_PROPERTY_FLAGS,
    pub EntityBody: u16,
    pub DrainEntityBody: u16,
    pub RequestQueue: u16,
    pub IdleConnection: u16,
    pub HeaderWait: u16,
    pub MinSendRate: u32,
}
impl windows_core::TypeKind for HTTP_TIMEOUT_LIMIT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_TIMEOUT_LIMIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_TLS_RESTRICTIONS_PARAM {
    pub RestrictionCount: u32,
    pub TlsRestrictions: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_TLS_RESTRICTIONS_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_TLS_RESTRICTIONS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    pub SessionTicketKeyCount: u32,
    pub SessionTicketKeys: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_TLS_SESSION_TICKET_KEYS_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_TRANSPORT_ADDRESS {
    pub pRemoteAddress: *mut super::WinSock::SOCKADDR,
    pub pLocalAddress: *mut super::WinSock::SOCKADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for HTTP_TRANSPORT_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for HTTP_TRANSPORT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_UNKNOWN_HEADER {
    pub NameLength: u16,
    pub RawValueLength: u16,
    pub pName: windows_core::PCSTR,
    pub pRawValue: windows_core::PCSTR,
}
impl windows_core::TypeKind for HTTP_UNKNOWN_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_UNKNOWN_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl windows_core::TypeKind for HTTP_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HTTP_WSK_API_TIMINGS {
    pub ConnectCount: u64,
    pub ConnectSum: u64,
    pub DisconnectCount: u64,
    pub DisconnectSum: u64,
    pub SendCount: u64,
    pub SendSum: u64,
    pub ReceiveCount: u64,
    pub ReceiveSum: u64,
    pub ReleaseCount: u64,
    pub ReleaseSum: u64,
    pub ControlSocketCount: u64,
    pub ControlSocketSum: u64,
}
impl windows_core::TypeKind for HTTP_WSK_API_TIMINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HTTP_WSK_API_TIMINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
