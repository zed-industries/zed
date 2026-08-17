#[inline]
pub unsafe fn DnsAcquireContextHandle_A(credentialflags: u32, credentials: Option<*const core::ffi::c_void>, pcontext: *mut super::super::Foundation::HANDLE) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsAcquireContextHandle_A(credentialflags : u32, credentials : *const core::ffi::c_void, pcontext : *mut super::super::Foundation:: HANDLE) -> i32);
    DnsAcquireContextHandle_A(credentialflags, core::mem::transmute(credentials.unwrap_or(std::ptr::null())), pcontext)
}
#[inline]
pub unsafe fn DnsAcquireContextHandle_W(credentialflags: u32, credentials: Option<*const core::ffi::c_void>, pcontext: *mut super::super::Foundation::HANDLE) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsAcquireContextHandle_W(credentialflags : u32, credentials : *const core::ffi::c_void, pcontext : *mut super::super::Foundation:: HANDLE) -> i32);
    DnsAcquireContextHandle_W(credentialflags, core::mem::transmute(credentials.unwrap_or(std::ptr::null())), pcontext)
}
#[inline]
pub unsafe fn DnsCancelQuery(pcancelhandle: *const DNS_QUERY_CANCEL) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsCancelQuery(pcancelhandle : *const DNS_QUERY_CANCEL) -> i32);
    DnsCancelQuery(pcancelhandle)
}
#[inline]
pub unsafe fn DnsCancelQueryRaw(cancelhandle: *const DNS_QUERY_RAW_CANCEL) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsCancelQueryRaw(cancelhandle : *const DNS_QUERY_RAW_CANCEL) -> i32);
    DnsCancelQueryRaw(cancelhandle)
}
#[inline]
pub unsafe fn DnsConnectionDeletePolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionDeletePolicyEntries(policyentrytag : DNS_CONNECTION_POLICY_TAG) -> u32);
    DnsConnectionDeletePolicyEntries(policyentrytag)
}
#[inline]
pub unsafe fn DnsConnectionDeleteProxyInfo<P0>(pwszconnectionname: P0, r#type: DNS_CONNECTION_PROXY_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionDeleteProxyInfo(pwszconnectionname : windows_core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE) -> u32);
    DnsConnectionDeleteProxyInfo(pwszconnectionname.param().abi(), r#type)
}
#[inline]
pub unsafe fn DnsConnectionFreeNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionFreeNameList(pnamelist : *mut DNS_CONNECTION_NAME_LIST));
    DnsConnectionFreeNameList(pnamelist)
}
#[inline]
pub unsafe fn DnsConnectionFreeProxyInfo(pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionFreeProxyInfo(pproxyinfo : *mut DNS_CONNECTION_PROXY_INFO));
    DnsConnectionFreeProxyInfo(pproxyinfo)
}
#[inline]
pub unsafe fn DnsConnectionFreeProxyInfoEx(pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionFreeProxyInfoEx(pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX));
    DnsConnectionFreeProxyInfoEx(pproxyinfoex)
}
#[inline]
pub unsafe fn DnsConnectionFreeProxyList(pproxylist: *mut DNS_CONNECTION_PROXY_LIST) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionFreeProxyList(pproxylist : *mut DNS_CONNECTION_PROXY_LIST));
    DnsConnectionFreeProxyList(pproxylist)
}
#[inline]
pub unsafe fn DnsConnectionGetNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionGetNameList(pnamelist : *mut DNS_CONNECTION_NAME_LIST) -> u32);
    DnsConnectionGetNameList(pnamelist)
}
#[inline]
pub unsafe fn DnsConnectionGetProxyInfo<P0>(pwszconnectionname: P0, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionGetProxyInfo(pwszconnectionname : windows_core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE, pproxyinfo : *mut DNS_CONNECTION_PROXY_INFO) -> u32);
    DnsConnectionGetProxyInfo(pwszconnectionname.param().abi(), r#type, pproxyinfo)
}
#[inline]
pub unsafe fn DnsConnectionGetProxyInfoForHostUrl<P0>(pwszhosturl: P0, pselectioncontext: Option<&[u8]>, dwexplicitinterfaceindex: u32, pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionGetProxyInfoForHostUrl(pwszhosturl : windows_core::PCWSTR, pselectioncontext : *const u8, dwselectioncontextlength : u32, dwexplicitinterfaceindex : u32, pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32);
    DnsConnectionGetProxyInfoForHostUrl(pwszhosturl.param().abi(), core::mem::transmute(pselectioncontext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pselectioncontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwexplicitinterfaceindex, pproxyinfoex)
}
#[inline]
pub unsafe fn DnsConnectionGetProxyInfoForHostUrlEx<P0, P1>(pwszhosturl: P0, pselectioncontext: Option<&[u8]>, dwexplicitinterfaceindex: u32, pwszconnectionname: P1, pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionGetProxyInfoForHostUrlEx(pwszhosturl : windows_core::PCWSTR, pselectioncontext : *const u8, dwselectioncontextlength : u32, dwexplicitinterfaceindex : u32, pwszconnectionname : windows_core::PCWSTR, pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32);
    DnsConnectionGetProxyInfoForHostUrlEx(pwszhosturl.param().abi(), core::mem::transmute(pselectioncontext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pselectioncontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwexplicitinterfaceindex, pwszconnectionname.param().abi(), pproxyinfoex)
}
#[inline]
pub unsafe fn DnsConnectionGetProxyList<P0>(pwszconnectionname: P0, pproxylist: *mut DNS_CONNECTION_PROXY_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionGetProxyList(pwszconnectionname : windows_core::PCWSTR, pproxylist : *mut DNS_CONNECTION_PROXY_LIST) -> u32);
    DnsConnectionGetProxyList(pwszconnectionname.param().abi(), pproxylist)
}
#[inline]
pub unsafe fn DnsConnectionSetPolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG, ppolicyentrylist: *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionSetPolicyEntries(policyentrytag : DNS_CONNECTION_POLICY_TAG, ppolicyentrylist : *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32);
    DnsConnectionSetPolicyEntries(policyentrytag, ppolicyentrylist)
}
#[inline]
pub unsafe fn DnsConnectionSetProxyInfo<P0>(pwszconnectionname: P0, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *const DNS_CONNECTION_PROXY_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionSetProxyInfo(pwszconnectionname : windows_core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE, pproxyinfo : *const DNS_CONNECTION_PROXY_INFO) -> u32);
    DnsConnectionSetProxyInfo(pwszconnectionname.param().abi(), r#type, pproxyinfo)
}
#[inline]
pub unsafe fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries: *const DNS_CONNECTION_IFINDEX_LIST) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries : *const DNS_CONNECTION_IFINDEX_LIST) -> u32);
    DnsConnectionUpdateIfIndexTable(pconnectionifindexentries)
}
#[inline]
pub unsafe fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer : *const DNS_MESSAGE_BUFFER, wmessagelength : u16, pprecord : *mut *mut DNS_RECORDA) -> i32);
    DnsExtractRecordsFromMessage_UTF8(pdnsbuffer, wmessagelength, pprecord)
}
#[inline]
pub unsafe fn DnsExtractRecordsFromMessage_W(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsExtractRecordsFromMessage_W(pdnsbuffer : *const DNS_MESSAGE_BUFFER, wmessagelength : u16, pprecord : *mut *mut DNS_RECORDA) -> i32);
    DnsExtractRecordsFromMessage_W(pdnsbuffer, wmessagelength, pprecord)
}
#[inline]
pub unsafe fn DnsFree(pdata: Option<*const core::ffi::c_void>, freetype: DNS_FREE_TYPE) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsFree(pdata : *const core::ffi::c_void, freetype : DNS_FREE_TYPE));
    DnsFree(core::mem::transmute(pdata.unwrap_or(std::ptr::null())), freetype)
}
#[inline]
pub unsafe fn DnsFreeCustomServers(pcservers: *mut u32, ppservers: *mut *mut DNS_CUSTOM_SERVER) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsFreeCustomServers(pcservers : *mut u32, ppservers : *mut *mut DNS_CUSTOM_SERVER));
    DnsFreeCustomServers(pcservers, ppservers)
}
#[inline]
pub unsafe fn DnsFreeProxyName<P0>(proxyname: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsFreeProxyName(proxyname : windows_core::PCWSTR));
    DnsFreeProxyName(proxyname.param().abi())
}
#[inline]
pub unsafe fn DnsGetApplicationSettings(pcservers: *mut u32, ppdefaultservers: *mut *mut DNS_CUSTOM_SERVER, psettings: Option<*mut DNS_APPLICATION_SETTINGS>) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsGetApplicationSettings(pcservers : *mut u32, ppdefaultservers : *mut *mut DNS_CUSTOM_SERVER, psettings : *mut DNS_APPLICATION_SETTINGS) -> u32);
    DnsGetApplicationSettings(pcservers, ppdefaultservers, core::mem::transmute(psettings.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsGetProxyInformation<P0>(hostname: P0, proxyinformation: *mut DNS_PROXY_INFORMATION, defaultproxyinformation: Option<*mut DNS_PROXY_INFORMATION>, completionroutine: DNS_PROXY_COMPLETION_ROUTINE, completioncontext: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsGetProxyInformation(hostname : windows_core::PCWSTR, proxyinformation : *mut DNS_PROXY_INFORMATION, defaultproxyinformation : *mut DNS_PROXY_INFORMATION, completionroutine : DNS_PROXY_COMPLETION_ROUTINE, completioncontext : *const core::ffi::c_void) -> u32);
    DnsGetProxyInformation(hostname.param().abi(), proxyinformation, core::mem::transmute(defaultproxyinformation.unwrap_or(std::ptr::null_mut())), completionroutine, core::mem::transmute(completioncontext.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn DnsModifyRecordsInSet_A<P0>(paddrecords: Option<*const DNS_RECORDA>, pdeleterecords: Option<*const DNS_RECORDA>, options: u32, hcredentials: P0, pextralist: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_A(paddrecords : *const DNS_RECORDA, pdeleterecords : *const DNS_RECORDA, options : u32, hcredentials : super::super::Foundation:: HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> i32);
    DnsModifyRecordsInSet_A(core::mem::transmute(paddrecords.unwrap_or(std::ptr::null())), core::mem::transmute(pdeleterecords.unwrap_or(std::ptr::null())), options, hcredentials.param().abi(), core::mem::transmute(pextralist.unwrap_or(std::ptr::null_mut())), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsModifyRecordsInSet_UTF8<P0>(paddrecords: Option<*const DNS_RECORDA>, pdeleterecords: Option<*const DNS_RECORDA>, options: u32, hcredentials: P0, pextralist: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_UTF8(paddrecords : *const DNS_RECORDA, pdeleterecords : *const DNS_RECORDA, options : u32, hcredentials : super::super::Foundation:: HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> i32);
    DnsModifyRecordsInSet_UTF8(core::mem::transmute(paddrecords.unwrap_or(std::ptr::null())), core::mem::transmute(pdeleterecords.unwrap_or(std::ptr::null())), options, hcredentials.param().abi(), core::mem::transmute(pextralist.unwrap_or(std::ptr::null_mut())), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsModifyRecordsInSet_W<P0>(paddrecords: Option<*const DNS_RECORDA>, pdeleterecords: Option<*const DNS_RECORDA>, options: u32, hcredentials: P0, pextralist: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_W(paddrecords : *const DNS_RECORDA, pdeleterecords : *const DNS_RECORDA, options : u32, hcredentials : super::super::Foundation:: HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> i32);
    DnsModifyRecordsInSet_W(core::mem::transmute(paddrecords.unwrap_or(std::ptr::null())), core::mem::transmute(pdeleterecords.unwrap_or(std::ptr::null())), options, hcredentials.param().abi(), core::mem::transmute(pextralist.unwrap_or(std::ptr::null_mut())), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsNameCompare_A<P0, P1>(pname1: P0, pname2: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsNameCompare_A(pname1 : windows_core::PCSTR, pname2 : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    DnsNameCompare_A(pname1.param().abi(), pname2.param().abi())
}
#[inline]
pub unsafe fn DnsNameCompare_W<P0, P1>(pname1: P0, pname2: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsNameCompare_W(pname1 : windows_core::PCWSTR, pname2 : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    DnsNameCompare_W(pname1.param().abi(), pname2.param().abi())
}
#[inline]
pub unsafe fn DnsQueryConfig<P0>(config: DNS_CONFIG_TYPE, flag: u32, pwsadaptername: P0, preserved: Option<*const core::ffi::c_void>, pbuffer: Option<*mut core::ffi::c_void>, pbuflen: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsQueryConfig(config : DNS_CONFIG_TYPE, flag : u32, pwsadaptername : windows_core::PCWSTR, preserved : *const core::ffi::c_void, pbuffer : *mut core::ffi::c_void, pbuflen : *mut u32) -> i32);
    DnsQueryConfig(config, flag, pwsadaptername.param().abi(), core::mem::transmute(preserved.unwrap_or(std::ptr::null())), core::mem::transmute(pbuffer.unwrap_or(std::ptr::null_mut())), pbuflen)
}
#[inline]
pub unsafe fn DnsQueryEx(pqueryrequest: *const DNS_QUERY_REQUEST, pqueryresults: *mut DNS_QUERY_RESULT, pcancelhandle: Option<*mut DNS_QUERY_CANCEL>) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsQueryEx(pqueryrequest : *const DNS_QUERY_REQUEST, pqueryresults : *mut DNS_QUERY_RESULT, pcancelhandle : *mut DNS_QUERY_CANCEL) -> i32);
    DnsQueryEx(pqueryrequest, pqueryresults, core::mem::transmute(pcancelhandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsQueryRaw(queryrequest: *const DNS_QUERY_RAW_REQUEST, cancelhandle: *mut DNS_QUERY_RAW_CANCEL) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsQueryRaw(queryrequest : *const DNS_QUERY_RAW_REQUEST, cancelhandle : *mut DNS_QUERY_RAW_CANCEL) -> i32);
    DnsQueryRaw(queryrequest, cancelhandle)
}
#[inline]
pub unsafe fn DnsQueryRawResultFree(queryresults: Option<*const DNS_QUERY_RAW_RESULT>) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsQueryRawResultFree(queryresults : *const DNS_QUERY_RAW_RESULT));
    DnsQueryRawResultFree(core::mem::transmute(queryresults.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn DnsQuery_A<P0>(pszname: P0, wtype: DNS_TYPE, options: DNS_QUERY_OPTIONS, pextra: Option<*mut core::ffi::c_void>, ppqueryresults: *mut *mut DNS_RECORDA, preserved: Option<*mut *mut core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsQuery_A(pszname : windows_core::PCSTR, wtype : DNS_TYPE, options : DNS_QUERY_OPTIONS, pextra : *mut core::ffi::c_void, ppqueryresults : *mut *mut DNS_RECORDA, preserved : *mut *mut core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    DnsQuery_A(pszname.param().abi(), wtype, options, core::mem::transmute(pextra.unwrap_or(std::ptr::null_mut())), ppqueryresults, core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsQuery_UTF8<P0>(pszname: P0, wtype: DNS_TYPE, options: DNS_QUERY_OPTIONS, pextra: Option<*mut core::ffi::c_void>, ppqueryresults: *mut *mut DNS_RECORDA, preserved: Option<*mut *mut core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsQuery_UTF8(pszname : windows_core::PCSTR, wtype : DNS_TYPE, options : DNS_QUERY_OPTIONS, pextra : *mut core::ffi::c_void, ppqueryresults : *mut *mut DNS_RECORDA, preserved : *mut *mut core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    DnsQuery_UTF8(pszname.param().abi(), wtype, options, core::mem::transmute(pextra.unwrap_or(std::ptr::null_mut())), ppqueryresults, core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsQuery_W<P0>(pszname: P0, wtype: DNS_TYPE, options: DNS_QUERY_OPTIONS, pextra: Option<*mut core::ffi::c_void>, ppqueryresults: *mut *mut DNS_RECORDA, preserved: Option<*mut *mut core::ffi::c_void>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsQuery_W(pszname : windows_core::PCWSTR, wtype : DNS_TYPE, options : DNS_QUERY_OPTIONS, pextra : *mut core::ffi::c_void, ppqueryresults : *mut *mut DNS_RECORDA, preserved : *mut *mut core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    DnsQuery_W(pszname.param().abi(), wtype, options, core::mem::transmute(pextra.unwrap_or(std::ptr::null_mut())), ppqueryresults, core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsRecordCompare(precord1: *const DNS_RECORDA, precord2: *const DNS_RECORDA) -> super::super::Foundation::BOOL {
    windows_targets::link!("dnsapi.dll" "system" fn DnsRecordCompare(precord1 : *const DNS_RECORDA, precord2 : *const DNS_RECORDA) -> super::super::Foundation:: BOOL);
    DnsRecordCompare(precord1, precord2)
}
#[inline]
pub unsafe fn DnsRecordCopyEx(precord: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA {
    windows_targets::link!("dnsapi.dll" "system" fn DnsRecordCopyEx(precord : *const DNS_RECORDA, charsetin : DNS_CHARSET, charsetout : DNS_CHARSET) -> *mut DNS_RECORDA);
    DnsRecordCopyEx(precord, charsetin, charsetout)
}
#[inline]
pub unsafe fn DnsRecordSetCompare(prr1: *mut DNS_RECORDA, prr2: *mut DNS_RECORDA, ppdiff1: Option<*mut *mut DNS_RECORDA>, ppdiff2: Option<*mut *mut DNS_RECORDA>) -> super::super::Foundation::BOOL {
    windows_targets::link!("dnsapi.dll" "system" fn DnsRecordSetCompare(prr1 : *mut DNS_RECORDA, prr2 : *mut DNS_RECORDA, ppdiff1 : *mut *mut DNS_RECORDA, ppdiff2 : *mut *mut DNS_RECORDA) -> super::super::Foundation:: BOOL);
    DnsRecordSetCompare(prr1, prr2, core::mem::transmute(ppdiff1.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppdiff2.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsRecordSetCopyEx(precordset: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA {
    windows_targets::link!("dnsapi.dll" "system" fn DnsRecordSetCopyEx(precordset : *const DNS_RECORDA, charsetin : DNS_CHARSET, charsetout : DNS_CHARSET) -> *mut DNS_RECORDA);
    DnsRecordSetCopyEx(precordset, charsetin, charsetout)
}
#[inline]
pub unsafe fn DnsRecordSetDetach(precordlist: *mut DNS_RECORDA) -> *mut DNS_RECORDA {
    windows_targets::link!("dnsapi.dll" "system" fn DnsRecordSetDetach(precordlist : *mut DNS_RECORDA) -> *mut DNS_RECORDA);
    DnsRecordSetDetach(precordlist)
}
#[inline]
pub unsafe fn DnsReleaseContextHandle<P0>(hcontext: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsReleaseContextHandle(hcontext : super::super::Foundation:: HANDLE));
    DnsReleaseContextHandle(hcontext.param().abi())
}
#[inline]
pub unsafe fn DnsReplaceRecordSetA<P0>(preplaceset: *const DNS_RECORDA, options: u32, hcontext: P0, pextrainfo: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetA(preplaceset : *const DNS_RECORDA, options : u32, hcontext : super::super::Foundation:: HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> i32);
    DnsReplaceRecordSetA(preplaceset, options, hcontext.param().abi(), core::mem::transmute(pextrainfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsReplaceRecordSetUTF8<P0>(preplaceset: *const DNS_RECORDA, options: u32, hcontext: P0, pextrainfo: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetUTF8(preplaceset : *const DNS_RECORDA, options : u32, hcontext : super::super::Foundation:: HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> i32);
    DnsReplaceRecordSetUTF8(preplaceset, options, hcontext.param().abi(), core::mem::transmute(pextrainfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsReplaceRecordSetW<P0>(preplaceset: *const DNS_RECORDA, options: u32, hcontext: P0, pextrainfo: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetW(preplaceset : *const DNS_RECORDA, options : u32, hcontext : super::super::Foundation:: HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> i32);
    DnsReplaceRecordSetW(preplaceset, options, hcontext.param().abi(), core::mem::transmute(pextrainfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(preserved.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsServiceBrowse(prequest: *const DNS_SERVICE_BROWSE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceBrowse(prequest : *const DNS_SERVICE_BROWSE_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> i32);
    DnsServiceBrowse(prequest, pcancel)
}
#[inline]
pub unsafe fn DnsServiceBrowseCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceBrowseCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> i32);
    DnsServiceBrowseCancel(pcancelhandle)
}
#[inline]
pub unsafe fn DnsServiceConstructInstance<P0, P1>(pservicename: P0, phostname: P1, pip4: Option<*const u32>, pip6: Option<*const IP6_ADDRESS>, wport: u16, wpriority: u16, wweight: u16, dwpropertiescount: u32, keys: *const windows_core::PCWSTR, values: *const windows_core::PCWSTR) -> *mut DNS_SERVICE_INSTANCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceConstructInstance(pservicename : windows_core::PCWSTR, phostname : windows_core::PCWSTR, pip4 : *const u32, pip6 : *const IP6_ADDRESS, wport : u16, wpriority : u16, wweight : u16, dwpropertiescount : u32, keys : *const windows_core::PCWSTR, values : *const windows_core::PCWSTR) -> *mut DNS_SERVICE_INSTANCE);
    DnsServiceConstructInstance(pservicename.param().abi(), phostname.param().abi(), core::mem::transmute(pip4.unwrap_or(std::ptr::null())), core::mem::transmute(pip6.unwrap_or(std::ptr::null())), wport, wpriority, wweight, dwpropertiescount, keys, values)
}
#[inline]
pub unsafe fn DnsServiceCopyInstance(porig: *const DNS_SERVICE_INSTANCE) -> *mut DNS_SERVICE_INSTANCE {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceCopyInstance(porig : *const DNS_SERVICE_INSTANCE) -> *mut DNS_SERVICE_INSTANCE);
    DnsServiceCopyInstance(porig)
}
#[inline]
pub unsafe fn DnsServiceDeRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: Option<*mut DNS_SERVICE_CANCEL>) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceDeRegister(prequest : *const DNS_SERVICE_REGISTER_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> u32);
    DnsServiceDeRegister(prequest, core::mem::transmute(pcancel.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsServiceFreeInstance(pinstance: *const DNS_SERVICE_INSTANCE) {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceFreeInstance(pinstance : *const DNS_SERVICE_INSTANCE));
    DnsServiceFreeInstance(pinstance)
}
#[inline]
pub unsafe fn DnsServiceRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: Option<*mut DNS_SERVICE_CANCEL>) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceRegister(prequest : *const DNS_SERVICE_REGISTER_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> u32);
    DnsServiceRegister(prequest, core::mem::transmute(pcancel.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DnsServiceRegisterCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceRegisterCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> u32);
    DnsServiceRegisterCancel(pcancelhandle)
}
#[inline]
pub unsafe fn DnsServiceResolve(prequest: *const DNS_SERVICE_RESOLVE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceResolve(prequest : *const DNS_SERVICE_RESOLVE_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> i32);
    DnsServiceResolve(prequest, pcancel)
}
#[inline]
pub unsafe fn DnsServiceResolveCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsServiceResolveCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> i32);
    DnsServiceResolveCancel(pcancelhandle)
}
#[inline]
pub unsafe fn DnsSetApplicationSettings(pservers: &[DNS_CUSTOM_SERVER], psettings: Option<*const DNS_APPLICATION_SETTINGS>) -> u32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsSetApplicationSettings(cservers : u32, pservers : *const DNS_CUSTOM_SERVER, psettings : *const DNS_APPLICATION_SETTINGS) -> u32);
    DnsSetApplicationSettings(pservers.len().try_into().unwrap(), core::mem::transmute(pservers.as_ptr()), core::mem::transmute(psettings.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn DnsStartMulticastQuery(pqueryrequest: *const MDNS_QUERY_REQUEST, phandle: *mut MDNS_QUERY_HANDLE) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsStartMulticastQuery(pqueryrequest : *const MDNS_QUERY_REQUEST, phandle : *mut MDNS_QUERY_HANDLE) -> i32);
    DnsStartMulticastQuery(pqueryrequest, phandle)
}
#[inline]
pub unsafe fn DnsStopMulticastQuery(phandle: *mut MDNS_QUERY_HANDLE) -> i32 {
    windows_targets::link!("dnsapi.dll" "system" fn DnsStopMulticastQuery(phandle : *mut MDNS_QUERY_HANDLE) -> i32);
    DnsStopMulticastQuery(phandle)
}
#[inline]
pub unsafe fn DnsValidateName_A<P0>(pszname: P0, format: DNS_NAME_FORMAT) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsValidateName_A(pszname : windows_core::PCSTR, format : DNS_NAME_FORMAT) -> i32);
    DnsValidateName_A(pszname.param().abi(), format)
}
#[inline]
pub unsafe fn DnsValidateName_UTF8<P0>(pszname: P0, format: DNS_NAME_FORMAT) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsValidateName_UTF8(pszname : windows_core::PCSTR, format : DNS_NAME_FORMAT) -> i32);
    DnsValidateName_UTF8(pszname.param().abi(), format)
}
#[inline]
pub unsafe fn DnsValidateName_W<P0>(pszname: P0, format: DNS_NAME_FORMAT) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsValidateName_W(pszname : windows_core::PCWSTR, format : DNS_NAME_FORMAT) -> i32);
    DnsValidateName_W(pszname.param().abi(), format)
}
#[inline]
pub unsafe fn DnsWriteQuestionToBuffer_UTF8<P0, P1>(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: P0, wtype: u16, xid: u16, frecursiondesired: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsWriteQuestionToBuffer_UTF8(pdnsbuffer : *mut DNS_MESSAGE_BUFFER, pdwbuffersize : *mut u32, pszname : windows_core::PCSTR, wtype : u16, xid : u16, frecursiondesired : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    DnsWriteQuestionToBuffer_UTF8(pdnsbuffer, pdwbuffersize, pszname.param().abi(), wtype, xid, frecursiondesired.param().abi())
}
#[inline]
pub unsafe fn DnsWriteQuestionToBuffer_W<P0, P1>(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: P0, wtype: u16, xid: u16, frecursiondesired: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dnsapi.dll" "system" fn DnsWriteQuestionToBuffer_W(pdnsbuffer : *mut DNS_MESSAGE_BUFFER, pdwbuffersize : *mut u32, pszname : windows_core::PCWSTR, wtype : u16, xid : u16, frecursiondesired : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    DnsWriteQuestionToBuffer_W(pdnsbuffer, pdwbuffersize, pszname.param().abi(), wtype, xid, frecursiondesired.param().abi())
}
pub const DDR_MAX_IP_HINTS: u32 = 4u32;
pub const DNSREC_ADDITIONAL: u32 = 3u32;
pub const DNSREC_ANSWER: u32 = 1u32;
pub const DNSREC_AUTHORITY: u32 = 2u32;
pub const DNSREC_DELETE: u32 = 4u32;
pub const DNSREC_NOEXIST: u32 = 4u32;
pub const DNSREC_PREREQ: u32 = 1u32;
pub const DNSREC_QUESTION: u32 = 0u32;
pub const DNSREC_SECTION: u32 = 3u32;
pub const DNSREC_UPDATE: u32 = 2u32;
pub const DNSREC_ZONE: u32 = 0u32;
pub const DNSSEC_ALGORITHM_ECDSAP256_SHA256: u32 = 13u32;
pub const DNSSEC_ALGORITHM_ECDSAP384_SHA384: u32 = 14u32;
pub const DNSSEC_ALGORITHM_NULL: u32 = 253u32;
pub const DNSSEC_ALGORITHM_PRIVATE: u32 = 254u32;
pub const DNSSEC_ALGORITHM_RSAMD5: u32 = 1u32;
pub const DNSSEC_ALGORITHM_RSASHA1: u32 = 5u32;
pub const DNSSEC_ALGORITHM_RSASHA1_NSEC3: u32 = 7u32;
pub const DNSSEC_ALGORITHM_RSASHA256: u32 = 8u32;
pub const DNSSEC_ALGORITHM_RSASHA512: u32 = 10u32;
pub const DNSSEC_DIGEST_ALGORITHM_SHA1: u32 = 1u32;
pub const DNSSEC_DIGEST_ALGORITHM_SHA256: u32 = 2u32;
pub const DNSSEC_DIGEST_ALGORITHM_SHA384: u32 = 4u32;
pub const DNSSEC_KEY_FLAG_EXTEND: u32 = 8u32;
pub const DNSSEC_KEY_FLAG_FLAG10: u32 = 1024u32;
pub const DNSSEC_KEY_FLAG_FLAG11: u32 = 2048u32;
pub const DNSSEC_KEY_FLAG_FLAG2: u32 = 4u32;
pub const DNSSEC_KEY_FLAG_FLAG4: u32 = 16u32;
pub const DNSSEC_KEY_FLAG_FLAG5: u32 = 32u32;
pub const DNSSEC_KEY_FLAG_FLAG8: u32 = 256u32;
pub const DNSSEC_KEY_FLAG_FLAG9: u32 = 512u32;
pub const DNSSEC_KEY_FLAG_HOST: u32 = 128u32;
pub const DNSSEC_KEY_FLAG_NOAUTH: u32 = 1u32;
pub const DNSSEC_KEY_FLAG_NOCONF: u32 = 2u32;
pub const DNSSEC_KEY_FLAG_NTPE3: u32 = 192u32;
pub const DNSSEC_KEY_FLAG_SIG0: u32 = 0u32;
pub const DNSSEC_KEY_FLAG_SIG1: u32 = 4096u32;
pub const DNSSEC_KEY_FLAG_SIG10: u32 = 40960u32;
pub const DNSSEC_KEY_FLAG_SIG11: u32 = 45056u32;
pub const DNSSEC_KEY_FLAG_SIG12: u32 = 49152u32;
pub const DNSSEC_KEY_FLAG_SIG13: u32 = 53248u32;
pub const DNSSEC_KEY_FLAG_SIG14: u32 = 57344u32;
pub const DNSSEC_KEY_FLAG_SIG15: u32 = 61440u32;
pub const DNSSEC_KEY_FLAG_SIG2: u32 = 8192u32;
pub const DNSSEC_KEY_FLAG_SIG3: u32 = 12288u32;
pub const DNSSEC_KEY_FLAG_SIG4: u32 = 16384u32;
pub const DNSSEC_KEY_FLAG_SIG5: u32 = 20480u32;
pub const DNSSEC_KEY_FLAG_SIG6: u32 = 24576u32;
pub const DNSSEC_KEY_FLAG_SIG7: u32 = 28672u32;
pub const DNSSEC_KEY_FLAG_SIG8: u32 = 32768u32;
pub const DNSSEC_KEY_FLAG_SIG9: u32 = 36864u32;
pub const DNSSEC_KEY_FLAG_USER: u32 = 0u32;
pub const DNSSEC_KEY_FLAG_ZONE: u32 = 64u32;
pub const DNSSEC_PROTOCOL_DNSSEC: u32 = 3u32;
pub const DNSSEC_PROTOCOL_EMAIL: u32 = 2u32;
pub const DNSSEC_PROTOCOL_IPSEC: u32 = 4u32;
pub const DNSSEC_PROTOCOL_NONE: u32 = 0u32;
pub const DNSSEC_PROTOCOL_TLS: u32 = 1u32;
pub const DNS_ADDRESS_STRING_LENGTH: u32 = 65u32;
pub const DNS_ADDR_MAX_SOCKADDR_LENGTH: u32 = 32u32;
pub const DNS_APP_SETTINGS_EXCLUSIVE_SERVERS: u32 = 1u32;
pub const DNS_APP_SETTINGS_VERSION1: u32 = 1u32;
pub const DNS_ATMA_AESA_ADDR_LENGTH: u32 = 20u32;
pub const DNS_ATMA_FORMAT_AESA: u32 = 2u32;
pub const DNS_ATMA_FORMAT_E164: u32 = 1u32;
pub const DNS_ATMA_MAX_ADDR_LENGTH: u32 = 20u32;
pub const DNS_ATMA_MAX_RECORD_LENGTH: u32 = 21u32;
pub const DNS_CLASS_ALL: u32 = 255u32;
pub const DNS_CLASS_ANY: u32 = 255u32;
pub const DNS_CLASS_CHAOS: u32 = 3u32;
pub const DNS_CLASS_CSNET: u32 = 2u32;
pub const DNS_CLASS_HESIOD: u32 = 4u32;
pub const DNS_CLASS_INTERNET: u32 = 1u32;
pub const DNS_CLASS_NONE: u32 = 254u32;
pub const DNS_CLASS_UNICAST_RESPONSE: u32 = 32768u32;
pub const DNS_COMPRESSED_QUESTION_NAME: u32 = 49164u32;
pub const DNS_CONFIG_FLAG_ALLOC: u32 = 1u32;
pub const DNS_CONNECTION_NAME_MAX_LENGTH: u32 = 64u32;
pub const DNS_CONNECTION_POLICY_ENTRY_ONDEMAND: u32 = 1u32;
pub const DNS_CONNECTION_PROXY_INFO_CURRENT_VERSION: u32 = 1u32;
pub const DNS_CONNECTION_PROXY_INFO_EXCEPTION_MAX_LENGTH: u32 = 1024u32;
pub const DNS_CONNECTION_PROXY_INFO_EXTRA_INFO_MAX_LENGTH: u32 = 1024u32;
pub const DNS_CONNECTION_PROXY_INFO_FLAG_BYPASSLOCAL: u32 = 2u32;
pub const DNS_CONNECTION_PROXY_INFO_FLAG_DISABLED: u32 = 1u32;
pub const DNS_CONNECTION_PROXY_INFO_FRIENDLY_NAME_MAX_LENGTH: u32 = 64u32;
pub const DNS_CONNECTION_PROXY_INFO_PASSWORD_MAX_LENGTH: u32 = 128u32;
pub const DNS_CONNECTION_PROXY_INFO_SERVER_MAX_LENGTH: u32 = 256u32;
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_CONFIG: DNS_CONNECTION_PROXY_INFO_SWITCH = DNS_CONNECTION_PROXY_INFO_SWITCH(0i32);
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_SCRIPT: DNS_CONNECTION_PROXY_INFO_SWITCH = DNS_CONNECTION_PROXY_INFO_SWITCH(1i32);
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_WPAD: DNS_CONNECTION_PROXY_INFO_SWITCH = DNS_CONNECTION_PROXY_INFO_SWITCH(2i32);
pub const DNS_CONNECTION_PROXY_INFO_USERNAME_MAX_LENGTH: u32 = 128u32;
pub const DNS_CONNECTION_PROXY_TYPE_HTTP: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(1i32);
pub const DNS_CONNECTION_PROXY_TYPE_NULL: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(0i32);
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS4: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(4i32);
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS5: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(5i32);
pub const DNS_CONNECTION_PROXY_TYPE_WAP: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(2i32);
pub const DNS_CUSTOM_SERVER_TYPE_DOH: u32 = 2u32;
pub const DNS_CUSTOM_SERVER_TYPE_UDP: u32 = 1u32;
pub const DNS_CUSTOM_SERVER_UDP_FALLBACK: u32 = 1u32;
pub const DNS_MAX_IP4_REVERSE_NAME_BUFFER_LENGTH: u32 = 31u32;
pub const DNS_MAX_IP4_REVERSE_NAME_LENGTH: u32 = 31u32;
pub const DNS_MAX_IP6_REVERSE_NAME_BUFFER_LENGTH: u32 = 75u32;
pub const DNS_MAX_IP6_REVERSE_NAME_LENGTH: u32 = 75u32;
pub const DNS_MAX_LABEL_BUFFER_LENGTH: u32 = 64u32;
pub const DNS_MAX_LABEL_LENGTH: u32 = 63u32;
pub const DNS_MAX_NAME_BUFFER_LENGTH: u32 = 256u32;
pub const DNS_MAX_NAME_LENGTH: u32 = 255u32;
pub const DNS_MAX_REVERSE_NAME_BUFFER_LENGTH: u32 = 75u32;
pub const DNS_MAX_REVERSE_NAME_LENGTH: u32 = 75u32;
pub const DNS_MAX_TEXT_STRING_LENGTH: u32 = 255u32;
pub const DNS_OPCODE_IQUERY: u32 = 1u32;
pub const DNS_OPCODE_NOTIFY: u32 = 4u32;
pub const DNS_OPCODE_QUERY: u32 = 0u32;
pub const DNS_OPCODE_SERVER_STATUS: u32 = 2u32;
pub const DNS_OPCODE_UNKNOWN: u32 = 3u32;
pub const DNS_OPCODE_UPDATE: u32 = 5u32;
pub const DNS_PORT_HOST_ORDER: u32 = 53u32;
pub const DNS_PORT_NET_ORDER: u32 = 13568u32;
pub const DNS_PROTOCOL_DOH: u32 = 3u32;
pub const DNS_PROTOCOL_NO_WIRE: u32 = 5u32;
pub const DNS_PROTOCOL_TCP: u32 = 2u32;
pub const DNS_PROTOCOL_UDP: u32 = 1u32;
pub const DNS_PROTOCOL_UNSPECIFIED: u32 = 0u32;
pub const DNS_PROXY_INFORMATION_DEFAULT_SETTINGS: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(1i32);
pub const DNS_PROXY_INFORMATION_DIRECT: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(0i32);
pub const DNS_PROXY_INFORMATION_DOES_NOT_EXIST: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(3i32);
pub const DNS_PROXY_INFORMATION_PROXY_NAME: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(2i32);
pub const DNS_QUERY_ACCEPT_TRUNCATED_RESPONSE: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1u32);
pub const DNS_QUERY_ADDRCONFIG: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(8192u32);
pub const DNS_QUERY_APPEND_MULTILABEL: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(8388608u32);
pub const DNS_QUERY_BYPASS_CACHE: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(8u32);
pub const DNS_QUERY_CACHE_ONLY: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(16u32);
pub const DNS_QUERY_DISABLE_IDN_ENCODING: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(2097152u32);
pub const DNS_QUERY_DNSSEC_CHECKING_DISABLED: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(33554432u32);
pub const DNS_QUERY_DNSSEC_OK: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(16777216u32);
pub const DNS_QUERY_DONT_RESET_TTL_VALUES: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1048576u32);
pub const DNS_QUERY_DUAL_ADDR: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(16384u32);
pub const DNS_QUERY_MULTICAST_ONLY: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1024u32);
pub const DNS_QUERY_NO_HOSTS_FILE: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(64u32);
pub const DNS_QUERY_NO_LOCAL_NAME: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(32u32);
pub const DNS_QUERY_NO_MULTICAST: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(2048u32);
pub const DNS_QUERY_NO_NETBT: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(128u32);
pub const DNS_QUERY_NO_RECURSION: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(4u32);
pub const DNS_QUERY_NO_WIRE_QUERY: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(16u32);
pub const DNS_QUERY_RAW_OPTION_BEST_EFFORT_PARSE: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1u32);
pub const DNS_QUERY_RAW_REQUEST_VERSION1: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1u32);
pub const DNS_QUERY_RAW_RESULTS_VERSION1: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1u32);
pub const DNS_QUERY_REQUEST_VERSION1: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1u32);
pub const DNS_QUERY_REQUEST_VERSION2: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(2u32);
pub const DNS_QUERY_REQUEST_VERSION3: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(3u32);
pub const DNS_QUERY_RESERVED: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(4026531840u32);
pub const DNS_QUERY_RESULTS_VERSION1: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(1u32);
pub const DNS_QUERY_RETURN_MESSAGE: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(512u32);
pub const DNS_QUERY_STANDARD: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(0u32);
pub const DNS_QUERY_TREAT_AS_FQDN: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(4096u32);
pub const DNS_QUERY_USE_TCP_ONLY: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(2u32);
pub const DNS_QUERY_WIRE_ONLY: DNS_QUERY_OPTIONS = DNS_QUERY_OPTIONS(256u32);
pub const DNS_RCLASS_ALL: u32 = 65280u32;
pub const DNS_RCLASS_ANY: u32 = 65280u32;
pub const DNS_RCLASS_CHAOS: u32 = 768u32;
pub const DNS_RCLASS_CSNET: u32 = 512u32;
pub const DNS_RCLASS_HESIOD: u32 = 1024u32;
pub const DNS_RCLASS_INTERNET: u32 = 256u32;
pub const DNS_RCLASS_MDNS_CACHE_FLUSH: u32 = 128u32;
pub const DNS_RCLASS_NONE: u32 = 65024u32;
pub const DNS_RCLASS_UNICAST_RESPONSE: u32 = 128u32;
pub const DNS_RCODE_BADKEY: u32 = 17u32;
pub const DNS_RCODE_BADSIG: u32 = 16u32;
pub const DNS_RCODE_BADTIME: u32 = 18u32;
pub const DNS_RCODE_BADVERS: u32 = 16u32;
pub const DNS_RCODE_FORMAT_ERROR: u32 = 1u32;
pub const DNS_RCODE_FORMERR: u32 = 1u32;
pub const DNS_RCODE_MAX: u32 = 15u32;
pub const DNS_RCODE_NAME_ERROR: u32 = 3u32;
pub const DNS_RCODE_NOERROR: u32 = 0u32;
pub const DNS_RCODE_NOTAUTH: u32 = 9u32;
pub const DNS_RCODE_NOTIMPL: u32 = 4u32;
pub const DNS_RCODE_NOTZONE: u32 = 10u32;
pub const DNS_RCODE_NOT_IMPLEMENTED: u32 = 4u32;
pub const DNS_RCODE_NO_ERROR: u32 = 0u32;
pub const DNS_RCODE_NXDOMAIN: u32 = 3u32;
pub const DNS_RCODE_NXRRSET: u32 = 8u32;
pub const DNS_RCODE_REFUSED: u32 = 5u32;
pub const DNS_RCODE_SERVER_FAILURE: u32 = 2u32;
pub const DNS_RCODE_SERVFAIL: u32 = 2u32;
pub const DNS_RCODE_YXDOMAIN: u32 = 6u32;
pub const DNS_RCODE_YXRRSET: u32 = 7u32;
pub const DNS_RFC_MAX_UDP_PACKET_LENGTH: u32 = 512u32;
pub const DNS_RTYPE_A: u32 = 256u32;
pub const DNS_RTYPE_A6: u32 = 9728u32;
pub const DNS_RTYPE_AAAA: u32 = 7168u32;
pub const DNS_RTYPE_AFSDB: u32 = 4608u32;
pub const DNS_RTYPE_ALL: u32 = 65280u32;
pub const DNS_RTYPE_ANY: u32 = 65280u32;
pub const DNS_RTYPE_ATMA: u32 = 8704u32;
pub const DNS_RTYPE_AXFR: u32 = 64512u32;
pub const DNS_RTYPE_CERT: u32 = 9472u32;
pub const DNS_RTYPE_CNAME: u32 = 1280u32;
pub const DNS_RTYPE_DHCID: u32 = 12544u32;
pub const DNS_RTYPE_DNAME: u32 = 9984u32;
pub const DNS_RTYPE_DNSKEY: u32 = 12288u32;
pub const DNS_RTYPE_DS: u32 = 11008u32;
pub const DNS_RTYPE_EID: u32 = 7936u32;
pub const DNS_RTYPE_GID: u32 = 26112u32;
pub const DNS_RTYPE_GPOS: u32 = 6912u32;
pub const DNS_RTYPE_HINFO: u32 = 3328u32;
pub const DNS_RTYPE_ISDN: u32 = 5120u32;
pub const DNS_RTYPE_IXFR: u32 = 64256u32;
pub const DNS_RTYPE_KEY: u32 = 6400u32;
pub const DNS_RTYPE_KX: u32 = 9216u32;
pub const DNS_RTYPE_LOC: u32 = 7424u32;
pub const DNS_RTYPE_MAILA: u32 = 65024u32;
pub const DNS_RTYPE_MAILB: u32 = 64768u32;
pub const DNS_RTYPE_MB: u32 = 1792u32;
pub const DNS_RTYPE_MD: u32 = 768u32;
pub const DNS_RTYPE_MF: u32 = 1024u32;
pub const DNS_RTYPE_MG: u32 = 2048u32;
pub const DNS_RTYPE_MINFO: u32 = 3584u32;
pub const DNS_RTYPE_MR: u32 = 2304u32;
pub const DNS_RTYPE_MX: u32 = 3840u32;
pub const DNS_RTYPE_NAPTR: u32 = 8960u32;
pub const DNS_RTYPE_NIMLOC: u32 = 8192u32;
pub const DNS_RTYPE_NS: u32 = 512u32;
pub const DNS_RTYPE_NSAP: u32 = 5632u32;
pub const DNS_RTYPE_NSAPPTR: u32 = 5888u32;
pub const DNS_RTYPE_NSEC: u32 = 12032u32;
pub const DNS_RTYPE_NSEC3: u32 = 12800u32;
pub const DNS_RTYPE_NSEC3PARAM: u32 = 13056u32;
pub const DNS_RTYPE_NULL: u32 = 2560u32;
pub const DNS_RTYPE_NXT: u32 = 7680u32;
pub const DNS_RTYPE_OPT: u32 = 10496u32;
pub const DNS_RTYPE_PTR: u32 = 3072u32;
pub const DNS_RTYPE_PX: u32 = 6656u32;
pub const DNS_RTYPE_RP: u32 = 4352u32;
pub const DNS_RTYPE_RRSIG: u32 = 11776u32;
pub const DNS_RTYPE_RT: u32 = 5376u32;
pub const DNS_RTYPE_SIG: u32 = 6144u32;
pub const DNS_RTYPE_SINK: u32 = 10240u32;
pub const DNS_RTYPE_SOA: u32 = 1536u32;
pub const DNS_RTYPE_SRV: u32 = 8448u32;
pub const DNS_RTYPE_TEXT: u32 = 4096u32;
pub const DNS_RTYPE_TKEY: u32 = 63744u32;
pub const DNS_RTYPE_TLSA: u32 = 13312u32;
pub const DNS_RTYPE_TSIG: u32 = 64000u32;
pub const DNS_RTYPE_UID: u32 = 25856u32;
pub const DNS_RTYPE_UINFO: u32 = 25600u32;
pub const DNS_RTYPE_UNSPEC: u32 = 26368u32;
pub const DNS_RTYPE_WINS: u32 = 511u32;
pub const DNS_RTYPE_WINSR: u32 = 767u32;
pub const DNS_RTYPE_WKS: u32 = 2816u32;
pub const DNS_RTYPE_X25: u32 = 4864u32;
pub const DNS_TKEY_MODE_DIFFIE_HELLMAN: u32 = 2u32;
pub const DNS_TKEY_MODE_GSS: u32 = 3u32;
pub const DNS_TKEY_MODE_RESOLVER_ASSIGN: u32 = 4u32;
pub const DNS_TKEY_MODE_SERVER_ASSIGN: u32 = 1u32;
pub const DNS_TYPE_A: DNS_TYPE = DNS_TYPE(1u16);
pub const DNS_TYPE_A6: DNS_TYPE = DNS_TYPE(38u16);
pub const DNS_TYPE_AAAA: DNS_TYPE = DNS_TYPE(28u16);
pub const DNS_TYPE_ADDRS: DNS_TYPE = DNS_TYPE(248u16);
pub const DNS_TYPE_AFSDB: DNS_TYPE = DNS_TYPE(18u16);
pub const DNS_TYPE_ALL: DNS_TYPE = DNS_TYPE(255u16);
pub const DNS_TYPE_ANY: DNS_TYPE = DNS_TYPE(255u16);
pub const DNS_TYPE_ATMA: DNS_TYPE = DNS_TYPE(34u16);
pub const DNS_TYPE_AXFR: DNS_TYPE = DNS_TYPE(252u16);
pub const DNS_TYPE_CERT: DNS_TYPE = DNS_TYPE(37u16);
pub const DNS_TYPE_CNAME: DNS_TYPE = DNS_TYPE(5u16);
pub const DNS_TYPE_DHCID: DNS_TYPE = DNS_TYPE(49u16);
pub const DNS_TYPE_DNAME: DNS_TYPE = DNS_TYPE(39u16);
pub const DNS_TYPE_DNSKEY: DNS_TYPE = DNS_TYPE(48u16);
pub const DNS_TYPE_DS: DNS_TYPE = DNS_TYPE(43u16);
pub const DNS_TYPE_EID: DNS_TYPE = DNS_TYPE(31u16);
pub const DNS_TYPE_GID: DNS_TYPE = DNS_TYPE(102u16);
pub const DNS_TYPE_GPOS: DNS_TYPE = DNS_TYPE(27u16);
pub const DNS_TYPE_HINFO: DNS_TYPE = DNS_TYPE(13u16);
pub const DNS_TYPE_HTTPS: DNS_TYPE = DNS_TYPE(65u16);
pub const DNS_TYPE_ISDN: DNS_TYPE = DNS_TYPE(20u16);
pub const DNS_TYPE_IXFR: DNS_TYPE = DNS_TYPE(251u16);
pub const DNS_TYPE_KEY: DNS_TYPE = DNS_TYPE(25u16);
pub const DNS_TYPE_KX: DNS_TYPE = DNS_TYPE(36u16);
pub const DNS_TYPE_LOC: DNS_TYPE = DNS_TYPE(29u16);
pub const DNS_TYPE_MAILA: DNS_TYPE = DNS_TYPE(254u16);
pub const DNS_TYPE_MAILB: DNS_TYPE = DNS_TYPE(253u16);
pub const DNS_TYPE_MB: DNS_TYPE = DNS_TYPE(7u16);
pub const DNS_TYPE_MD: DNS_TYPE = DNS_TYPE(3u16);
pub const DNS_TYPE_MF: DNS_TYPE = DNS_TYPE(4u16);
pub const DNS_TYPE_MG: DNS_TYPE = DNS_TYPE(8u16);
pub const DNS_TYPE_MINFO: DNS_TYPE = DNS_TYPE(14u16);
pub const DNS_TYPE_MR: DNS_TYPE = DNS_TYPE(9u16);
pub const DNS_TYPE_MX: DNS_TYPE = DNS_TYPE(15u16);
pub const DNS_TYPE_NAPTR: DNS_TYPE = DNS_TYPE(35u16);
pub const DNS_TYPE_NBSTAT: DNS_TYPE = DNS_TYPE(65282u16);
pub const DNS_TYPE_NIMLOC: DNS_TYPE = DNS_TYPE(32u16);
pub const DNS_TYPE_NS: DNS_TYPE = DNS_TYPE(2u16);
pub const DNS_TYPE_NSAP: DNS_TYPE = DNS_TYPE(22u16);
pub const DNS_TYPE_NSAPPTR: DNS_TYPE = DNS_TYPE(23u16);
pub const DNS_TYPE_NSEC: DNS_TYPE = DNS_TYPE(47u16);
pub const DNS_TYPE_NSEC3: DNS_TYPE = DNS_TYPE(50u16);
pub const DNS_TYPE_NSEC3PARAM: DNS_TYPE = DNS_TYPE(51u16);
pub const DNS_TYPE_NULL: DNS_TYPE = DNS_TYPE(10u16);
pub const DNS_TYPE_NXT: DNS_TYPE = DNS_TYPE(30u16);
pub const DNS_TYPE_OPT: DNS_TYPE = DNS_TYPE(41u16);
pub const DNS_TYPE_PTR: DNS_TYPE = DNS_TYPE(12u16);
pub const DNS_TYPE_PX: DNS_TYPE = DNS_TYPE(26u16);
pub const DNS_TYPE_RP: DNS_TYPE = DNS_TYPE(17u16);
pub const DNS_TYPE_RRSIG: DNS_TYPE = DNS_TYPE(46u16);
pub const DNS_TYPE_RT: DNS_TYPE = DNS_TYPE(21u16);
pub const DNS_TYPE_SIG: DNS_TYPE = DNS_TYPE(24u16);
pub const DNS_TYPE_SINK: DNS_TYPE = DNS_TYPE(40u16);
pub const DNS_TYPE_SOA: DNS_TYPE = DNS_TYPE(6u16);
pub const DNS_TYPE_SRV: DNS_TYPE = DNS_TYPE(33u16);
pub const DNS_TYPE_SVCB: DNS_TYPE = DNS_TYPE(64u16);
pub const DNS_TYPE_TEXT: DNS_TYPE = DNS_TYPE(16u16);
pub const DNS_TYPE_TKEY: DNS_TYPE = DNS_TYPE(249u16);
pub const DNS_TYPE_TLSA: DNS_TYPE = DNS_TYPE(52u16);
pub const DNS_TYPE_TSIG: DNS_TYPE = DNS_TYPE(250u16);
pub const DNS_TYPE_UID: DNS_TYPE = DNS_TYPE(101u16);
pub const DNS_TYPE_UINFO: DNS_TYPE = DNS_TYPE(100u16);
pub const DNS_TYPE_UNSPEC: DNS_TYPE = DNS_TYPE(103u16);
pub const DNS_TYPE_WINS: DNS_TYPE = DNS_TYPE(65281u16);
pub const DNS_TYPE_WINSR: DNS_TYPE = DNS_TYPE(65282u16);
pub const DNS_TYPE_WKS: DNS_TYPE = DNS_TYPE(11u16);
pub const DNS_TYPE_X25: DNS_TYPE = DNS_TYPE(19u16);
pub const DNS_TYPE_ZERO: DNS_TYPE = DNS_TYPE(0u16);
pub const DNS_UPDATE_CACHE_SECURITY_CONTEXT: u32 = 512u32;
pub const DNS_UPDATE_FORCE_SECURITY_NEGO: u32 = 2048u32;
pub const DNS_UPDATE_REMOTE_SERVER: u32 = 16384u32;
pub const DNS_UPDATE_RESERVED: u32 = 4294901760u32;
pub const DNS_UPDATE_SECURITY_OFF: u32 = 16u32;
pub const DNS_UPDATE_SECURITY_ON: u32 = 32u32;
pub const DNS_UPDATE_SECURITY_ONLY: u32 = 256u32;
pub const DNS_UPDATE_SECURITY_USE_DEFAULT: u32 = 0u32;
pub const DNS_UPDATE_SKIP_NO_UPDATE_ADAPTERS: u32 = 8192u32;
pub const DNS_UPDATE_TEST_USE_LOCAL_SYS_ACCT: u32 = 1024u32;
pub const DNS_UPDATE_TRY_ALL_MASTER_SERVERS: u32 = 4096u32;
pub const DNS_VALSVR_ERROR_INVALID_ADDR: u32 = 1u32;
pub const DNS_VALSVR_ERROR_INVALID_NAME: u32 = 2u32;
pub const DNS_VALSVR_ERROR_NO_AUTH: u32 = 5u32;
pub const DNS_VALSVR_ERROR_NO_RESPONSE: u32 = 4u32;
pub const DNS_VALSVR_ERROR_NO_TCP: u32 = 16u32;
pub const DNS_VALSVR_ERROR_REFUSED: u32 = 6u32;
pub const DNS_VALSVR_ERROR_UNKNOWN: u32 = 255u32;
pub const DNS_VALSVR_ERROR_UNREACHABLE: u32 = 3u32;
pub const DNS_WINS_FLAG_LOCAL: u32 = 65536u32;
pub const DNS_WINS_FLAG_SCOPE: u32 = 2147483648u32;
pub const DnsCharSetAnsi: DNS_CHARSET = DNS_CHARSET(3i32);
pub const DnsCharSetUnicode: DNS_CHARSET = DNS_CHARSET(1i32);
pub const DnsCharSetUnknown: DNS_CHARSET = DNS_CHARSET(0i32);
pub const DnsCharSetUtf8: DNS_CHARSET = DNS_CHARSET(2i32);
pub const DnsConfigAdapterDomainName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(4i32);
pub const DnsConfigAdapterDomainName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(5i32);
pub const DnsConfigAdapterDomainName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(3i32);
pub const DnsConfigAdapterHostNameRegistrationEnabled: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(10i32);
pub const DnsConfigAdapterInfo: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(8i32);
pub const DnsConfigAddressRegistrationMaxCount: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(11i32);
pub const DnsConfigDnsServerList: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(6i32);
pub const DnsConfigFullHostName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(16i32);
pub const DnsConfigFullHostName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(17i32);
pub const DnsConfigFullHostName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(15i32);
pub const DnsConfigHostName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(13i32);
pub const DnsConfigHostName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(14i32);
pub const DnsConfigHostName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(12i32);
pub const DnsConfigNameServer: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(18i32);
pub const DnsConfigPrimaryDomainName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(1i32);
pub const DnsConfigPrimaryDomainName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(2i32);
pub const DnsConfigPrimaryDomainName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(0i32);
pub const DnsConfigPrimaryHostNameRegistrationEnabled: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(9i32);
pub const DnsConfigSearchList: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(7i32);
pub const DnsFreeFlat: DNS_FREE_TYPE = DNS_FREE_TYPE(0i32);
pub const DnsFreeParsedMessageFields: DNS_FREE_TYPE = DNS_FREE_TYPE(2i32);
pub const DnsFreeRecordList: DNS_FREE_TYPE = DNS_FREE_TYPE(1i32);
pub const DnsNameDomain: DNS_NAME_FORMAT = DNS_NAME_FORMAT(0i32);
pub const DnsNameDomainLabel: DNS_NAME_FORMAT = DNS_NAME_FORMAT(1i32);
pub const DnsNameHostnameFull: DNS_NAME_FORMAT = DNS_NAME_FORMAT(2i32);
pub const DnsNameHostnameLabel: DNS_NAME_FORMAT = DNS_NAME_FORMAT(3i32);
pub const DnsNameSrvRecord: DNS_NAME_FORMAT = DNS_NAME_FORMAT(5i32);
pub const DnsNameValidateTld: DNS_NAME_FORMAT = DNS_NAME_FORMAT(6i32);
pub const DnsNameWildcard: DNS_NAME_FORMAT = DNS_NAME_FORMAT(4i32);
pub const DnsSectionAddtional: DNS_SECTION = DNS_SECTION(3i32);
pub const DnsSectionAnswer: DNS_SECTION = DNS_SECTION(1i32);
pub const DnsSectionAuthority: DNS_SECTION = DNS_SECTION(2i32);
pub const DnsSectionQuestion: DNS_SECTION = DNS_SECTION(0i32);
pub const DnsSvcbParamAlpn: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(1i32);
pub const DnsSvcbParamDohPath: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(7i32);
pub const DnsSvcbParamDohPathOpenDns: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(65432i32);
pub const DnsSvcbParamDohPathQuad9: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(65380i32);
pub const DnsSvcbParamEch: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(5i32);
pub const DnsSvcbParamIpv4Hint: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(4i32);
pub const DnsSvcbParamIpv6Hint: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(6i32);
pub const DnsSvcbParamMandatory: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(0i32);
pub const DnsSvcbParamNoDefaultAlpn: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(2i32);
pub const DnsSvcbParamPort: DNS_SVCB_PARAM_TYPE = DNS_SVCB_PARAM_TYPE(3i32);
pub const IP4_ADDRESS_STRING_BUFFER_LENGTH: u32 = 16u32;
pub const IP4_ADDRESS_STRING_LENGTH: u32 = 16u32;
pub const IP6_ADDRESS_STRING_BUFFER_LENGTH: u32 = 65u32;
pub const IP6_ADDRESS_STRING_LENGTH: u32 = 65u32;
pub const SIZEOF_IP4_ADDRESS: u32 = 4u32;
pub const TAG_DNS_CONNECTION_POLICY_TAG_CONNECTION_MANAGER: DNS_CONNECTION_POLICY_TAG = DNS_CONNECTION_POLICY_TAG(1i32);
pub const TAG_DNS_CONNECTION_POLICY_TAG_DEFAULT: DNS_CONNECTION_POLICY_TAG = DNS_CONNECTION_POLICY_TAG(0i32);
pub const TAG_DNS_CONNECTION_POLICY_TAG_WWWPT: DNS_CONNECTION_POLICY_TAG = DNS_CONNECTION_POLICY_TAG(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_CHARSET(pub i32);
impl windows_core::TypeKind for DNS_CHARSET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_CHARSET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_CHARSET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_CONFIG_TYPE(pub i32);
impl windows_core::TypeKind for DNS_CONFIG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_CONFIG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_CONFIG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_CONNECTION_POLICY_TAG(pub i32);
impl windows_core::TypeKind for DNS_CONNECTION_POLICY_TAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_CONNECTION_POLICY_TAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_CONNECTION_POLICY_TAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_CONNECTION_PROXY_INFO_SWITCH(pub i32);
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_INFO_SWITCH {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_CONNECTION_PROXY_INFO_SWITCH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_CONNECTION_PROXY_INFO_SWITCH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_CONNECTION_PROXY_TYPE(pub i32);
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_CONNECTION_PROXY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_CONNECTION_PROXY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_FREE_TYPE(pub i32);
impl windows_core::TypeKind for DNS_FREE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_FREE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_FREE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_NAME_FORMAT(pub i32);
impl windows_core::TypeKind for DNS_NAME_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_NAME_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_NAME_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_PROXY_INFORMATION_TYPE(pub i32);
impl windows_core::TypeKind for DNS_PROXY_INFORMATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_PROXY_INFORMATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_PROXY_INFORMATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_QUERY_OPTIONS(pub u32);
impl windows_core::TypeKind for DNS_QUERY_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_QUERY_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_QUERY_OPTIONS").field(&self.0).finish()
    }
}
impl DNS_QUERY_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DNS_QUERY_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DNS_QUERY_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DNS_QUERY_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DNS_QUERY_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DNS_QUERY_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_SECTION(pub i32);
impl windows_core::TypeKind for DNS_SECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_SECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_SECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_SVCB_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for DNS_SVCB_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_SVCB_PARAM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_SVCB_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DNS_TYPE(pub u16);
impl windows_core::TypeKind for DNS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DNS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DNS_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DNS_AAAA_DATA {
    pub Ip6Address: IP6_ADDRESS,
}
impl Copy for DNS_AAAA_DATA {}
impl Clone for DNS_AAAA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_AAAA_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_AAAA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_ADDR {
    pub MaxSa: [i8; 32],
    pub Data: DNS_ADDR_0,
}
impl Copy for DNS_ADDR {}
impl Clone for DNS_ADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_ADDR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union DNS_ADDR_0 {
    pub DnsAddrUserDword: [u32; 8],
}
impl Copy for DNS_ADDR_0 {}
impl Clone for DNS_ADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_ADDR_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_ADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DNS_ADDR_ARRAY {
    pub MaxCount: u32,
    pub AddrCount: u32,
    pub Tag: u32,
    pub Family: u16,
    pub WordReserved: u16,
    pub Flags: u32,
    pub MatchFlag: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub AddrArray: [DNS_ADDR; 1],
}
impl Copy for DNS_ADDR_ARRAY {}
impl Clone for DNS_ADDR_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_ADDR_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_ADDR_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_APPLICATION_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
}
impl Copy for DNS_APPLICATION_SETTINGS {}
impl Clone for DNS_APPLICATION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_APPLICATION_SETTINGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_APPLICATION_SETTINGS").field("Version", &self.Version).field("Flags", &self.Flags).finish()
    }
}
impl windows_core::TypeKind for DNS_APPLICATION_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_APPLICATION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags
    }
}
impl Eq for DNS_APPLICATION_SETTINGS {}
impl Default for DNS_APPLICATION_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_ATMA_DATA {
    pub AddressType: u8,
    pub Address: [u8; 20],
}
impl Copy for DNS_ATMA_DATA {}
impl Clone for DNS_ATMA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_ATMA_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_ATMA_DATA").field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl windows_core::TypeKind for DNS_ATMA_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_ATMA_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl Eq for DNS_ATMA_DATA {}
impl Default for DNS_ATMA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_A_DATA {
    pub IpAddress: u32,
}
impl Copy for DNS_A_DATA {}
impl Clone for DNS_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_A_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_A_DATA").field("IpAddress", &self.IpAddress).finish()
    }
}
impl windows_core::TypeKind for DNS_A_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress
    }
}
impl Eq for DNS_A_DATA {}
impl Default for DNS_A_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_IFINDEX_ENTRY {
    pub pwszConnectionName: windows_core::PCWSTR,
    pub dwIfIndex: u32,
}
impl Copy for DNS_CONNECTION_IFINDEX_ENTRY {}
impl Clone for DNS_CONNECTION_IFINDEX_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_IFINDEX_ENTRY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_IFINDEX_ENTRY").field("pwszConnectionName", &self.pwszConnectionName).field("dwIfIndex", &self.dwIfIndex).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_IFINDEX_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_IFINDEX_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszConnectionName == other.pwszConnectionName && self.dwIfIndex == other.dwIfIndex
    }
}
impl Eq for DNS_CONNECTION_IFINDEX_ENTRY {}
impl Default for DNS_CONNECTION_IFINDEX_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_IFINDEX_LIST {
    pub pConnectionIfIndexEntries: *mut DNS_CONNECTION_IFINDEX_ENTRY,
    pub nEntries: u32,
}
impl Copy for DNS_CONNECTION_IFINDEX_LIST {}
impl Clone for DNS_CONNECTION_IFINDEX_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_IFINDEX_LIST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_IFINDEX_LIST").field("pConnectionIfIndexEntries", &self.pConnectionIfIndexEntries).field("nEntries", &self.nEntries).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_IFINDEX_LIST {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_IFINDEX_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.pConnectionIfIndexEntries == other.pConnectionIfIndexEntries && self.nEntries == other.nEntries
    }
}
impl Eq for DNS_CONNECTION_IFINDEX_LIST {}
impl Default for DNS_CONNECTION_IFINDEX_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_NAME {
    pub wszName: [u16; 65],
}
impl Copy for DNS_CONNECTION_NAME {}
impl Clone for DNS_CONNECTION_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_NAME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_NAME").field("wszName", &self.wszName).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_NAME {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.wszName == other.wszName
    }
}
impl Eq for DNS_CONNECTION_NAME {}
impl Default for DNS_CONNECTION_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_NAME_LIST {
    pub cNames: u32,
    pub pNames: *mut DNS_CONNECTION_NAME,
}
impl Copy for DNS_CONNECTION_NAME_LIST {}
impl Clone for DNS_CONNECTION_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_NAME_LIST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_NAME_LIST").field("cNames", &self.cNames).field("pNames", &self.pNames).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_NAME_LIST {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cNames == other.cNames && self.pNames == other.pNames
    }
}
impl Eq for DNS_CONNECTION_NAME_LIST {}
impl Default for DNS_CONNECTION_NAME_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_POLICY_ENTRY {
    pub pwszHost: windows_core::PCWSTR,
    pub pwszAppId: windows_core::PCWSTR,
    pub cbAppSid: u32,
    pub pbAppSid: *mut u8,
    pub nConnections: u32,
    pub ppwszConnections: *const windows_core::PCWSTR,
    pub dwPolicyEntryFlags: u32,
}
impl Copy for DNS_CONNECTION_POLICY_ENTRY {}
impl Clone for DNS_CONNECTION_POLICY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_POLICY_ENTRY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_POLICY_ENTRY").field("pwszHost", &self.pwszHost).field("pwszAppId", &self.pwszAppId).field("cbAppSid", &self.cbAppSid).field("pbAppSid", &self.pbAppSid).field("nConnections", &self.nConnections).field("ppwszConnections", &self.ppwszConnections).field("dwPolicyEntryFlags", &self.dwPolicyEntryFlags).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_POLICY_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_POLICY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszHost == other.pwszHost && self.pwszAppId == other.pwszAppId && self.cbAppSid == other.cbAppSid && self.pbAppSid == other.pbAppSid && self.nConnections == other.nConnections && self.ppwszConnections == other.ppwszConnections && self.dwPolicyEntryFlags == other.dwPolicyEntryFlags
    }
}
impl Eq for DNS_CONNECTION_POLICY_ENTRY {}
impl Default for DNS_CONNECTION_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_POLICY_ENTRY_LIST {
    pub pPolicyEntries: *mut DNS_CONNECTION_POLICY_ENTRY,
    pub nEntries: u32,
}
impl Copy for DNS_CONNECTION_POLICY_ENTRY_LIST {}
impl Clone for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_POLICY_ENTRY_LIST").field("pPolicyEntries", &self.pPolicyEntries).field("nEntries", &self.nEntries).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_POLICY_ENTRY_LIST {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.pPolicyEntries == other.pPolicyEntries && self.nEntries == other.nEntries
    }
}
impl Eq for DNS_CONNECTION_POLICY_ENTRY_LIST {}
impl Default for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_ELEMENT {
    pub Type: DNS_CONNECTION_PROXY_TYPE,
    pub Info: DNS_CONNECTION_PROXY_INFO,
}
impl Copy for DNS_CONNECTION_PROXY_ELEMENT {}
impl Clone for DNS_CONNECTION_PROXY_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_ELEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_CONNECTION_PROXY_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_INFO {
    pub Version: u32,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub Flags: u32,
    pub Switch: DNS_CONNECTION_PROXY_INFO_SWITCH,
    pub Anonymous: DNS_CONNECTION_PROXY_INFO_0,
}
impl Copy for DNS_CONNECTION_PROXY_INFO {}
impl Clone for DNS_CONNECTION_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_CONNECTION_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_CONNECTION_PROXY_INFO_0 {
    pub Config: DNS_CONNECTION_PROXY_INFO_0_0,
    pub Script: DNS_CONNECTION_PROXY_INFO_0_1,
}
impl Copy for DNS_CONNECTION_PROXY_INFO_0 {}
impl Clone for DNS_CONNECTION_PROXY_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_CONNECTION_PROXY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_INFO_0_0 {
    pub pwszServer: windows_core::PWSTR,
    pub pwszUsername: windows_core::PWSTR,
    pub pwszPassword: windows_core::PWSTR,
    pub pwszException: windows_core::PWSTR,
    pub pwszExtraInfo: windows_core::PWSTR,
    pub Port: u16,
}
impl Copy for DNS_CONNECTION_PROXY_INFO_0_0 {}
impl Clone for DNS_CONNECTION_PROXY_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_PROXY_INFO_0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_PROXY_INFO_0_0").field("pwszServer", &self.pwszServer).field("pwszUsername", &self.pwszUsername).field("pwszPassword", &self.pwszPassword).field("pwszException", &self.pwszException).field("pwszExtraInfo", &self.pwszExtraInfo).field("Port", &self.Port).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_INFO_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_PROXY_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pwszServer == other.pwszServer && self.pwszUsername == other.pwszUsername && self.pwszPassword == other.pwszPassword && self.pwszException == other.pwszException && self.pwszExtraInfo == other.pwszExtraInfo && self.Port == other.Port
    }
}
impl Eq for DNS_CONNECTION_PROXY_INFO_0_0 {}
impl Default for DNS_CONNECTION_PROXY_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_INFO_0_1 {
    pub pwszScript: windows_core::PWSTR,
    pub pwszUsername: windows_core::PWSTR,
    pub pwszPassword: windows_core::PWSTR,
}
impl Copy for DNS_CONNECTION_PROXY_INFO_0_1 {}
impl Clone for DNS_CONNECTION_PROXY_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_PROXY_INFO_0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_PROXY_INFO_0_1").field("pwszScript", &self.pwszScript).field("pwszUsername", &self.pwszUsername).field("pwszPassword", &self.pwszPassword).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_INFO_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_PROXY_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pwszScript == other.pwszScript && self.pwszUsername == other.pwszUsername && self.pwszPassword == other.pwszPassword
    }
}
impl Eq for DNS_CONNECTION_PROXY_INFO_0_1 {}
impl Default for DNS_CONNECTION_PROXY_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_INFO_EX {
    pub ProxyInfo: DNS_CONNECTION_PROXY_INFO,
    pub dwInterfaceIndex: u32,
    pub pwszConnectionName: windows_core::PWSTR,
    pub fDirectConfiguration: super::super::Foundation::BOOL,
    pub hConnection: super::super::Foundation::HANDLE,
}
impl Copy for DNS_CONNECTION_PROXY_INFO_EX {}
impl Clone for DNS_CONNECTION_PROXY_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_INFO_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_CONNECTION_PROXY_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_LIST {
    pub cProxies: u32,
    pub pProxies: *mut DNS_CONNECTION_PROXY_ELEMENT,
}
impl Copy for DNS_CONNECTION_PROXY_LIST {}
impl Clone for DNS_CONNECTION_PROXY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_CONNECTION_PROXY_LIST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_CONNECTION_PROXY_LIST").field("cProxies", &self.cProxies).field("pProxies", &self.pProxies).finish()
    }
}
impl windows_core::TypeKind for DNS_CONNECTION_PROXY_LIST {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_CONNECTION_PROXY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cProxies == other.cProxies && self.pProxies == other.pProxies
    }
}
impl Eq for DNS_CONNECTION_PROXY_LIST {}
impl Default for DNS_CONNECTION_PROXY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_CUSTOM_SERVER {
    pub dwServerType: u32,
    pub ullFlags: u64,
    pub Anonymous1: DNS_CUSTOM_SERVER_0,
    pub Anonymous2: DNS_CUSTOM_SERVER_1,
}
impl Copy for DNS_CUSTOM_SERVER {}
impl Clone for DNS_CUSTOM_SERVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_CUSTOM_SERVER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_CUSTOM_SERVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_CUSTOM_SERVER_0 {
    pub pwszTemplate: windows_core::PWSTR,
}
impl Copy for DNS_CUSTOM_SERVER_0 {}
impl Clone for DNS_CUSTOM_SERVER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_CUSTOM_SERVER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_CUSTOM_SERVER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_CUSTOM_SERVER_1 {
    pub MaxSa: [i8; 32],
}
impl Copy for DNS_CUSTOM_SERVER_1 {}
impl Clone for DNS_CUSTOM_SERVER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_CUSTOM_SERVER_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_CUSTOM_SERVER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_DHCID_DATA {
    pub dwByteCount: u32,
    pub DHCID: [u8; 1],
}
impl Copy for DNS_DHCID_DATA {}
impl Clone for DNS_DHCID_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_DHCID_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_DHCID_DATA").field("dwByteCount", &self.dwByteCount).field("DHCID", &self.DHCID).finish()
    }
}
impl windows_core::TypeKind for DNS_DHCID_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_DHCID_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwByteCount == other.dwByteCount && self.DHCID == other.DHCID
    }
}
impl Eq for DNS_DHCID_DATA {}
impl Default for DNS_DHCID_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_DS_DATA {
    pub wKeyTag: u16,
    pub chAlgorithm: u8,
    pub chDigestType: u8,
    pub wDigestLength: u16,
    pub wPad: u16,
    pub Digest: [u8; 1],
}
impl Copy for DNS_DS_DATA {}
impl Clone for DNS_DS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_DS_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_DS_DATA").field("wKeyTag", &self.wKeyTag).field("chAlgorithm", &self.chAlgorithm).field("chDigestType", &self.chDigestType).field("wDigestLength", &self.wDigestLength).field("wPad", &self.wPad).field("Digest", &self.Digest).finish()
    }
}
impl windows_core::TypeKind for DNS_DS_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_DS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wKeyTag == other.wKeyTag && self.chAlgorithm == other.chAlgorithm && self.chDigestType == other.chDigestType && self.wDigestLength == other.wDigestLength && self.wPad == other.wPad && self.Digest == other.Digest
    }
}
impl Eq for DNS_DS_DATA {}
impl Default for DNS_DS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DNS_HEADER {
    pub Xid: u16,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub QuestionCount: u16,
    pub AnswerCount: u16,
    pub NameServerCount: u16,
    pub AdditionalCount: u16,
}
impl Copy for DNS_HEADER {}
impl Clone for DNS_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DNS_HEADER_EXT {
    pub _bitfield: u16,
    pub chRcode: u8,
    pub chVersion: u8,
}
impl Copy for DNS_HEADER_EXT {}
impl Clone for DNS_HEADER_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_HEADER_EXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_HEADER_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_KEY_DATA {
    pub wFlags: u16,
    pub chProtocol: u8,
    pub chAlgorithm: u8,
    pub wKeyLength: u16,
    pub wPad: u16,
    pub Key: [u8; 1],
}
impl Copy for DNS_KEY_DATA {}
impl Clone for DNS_KEY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_KEY_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_KEY_DATA").field("wFlags", &self.wFlags).field("chProtocol", &self.chProtocol).field("chAlgorithm", &self.chAlgorithm).field("wKeyLength", &self.wKeyLength).field("wPad", &self.wPad).field("Key", &self.Key).finish()
    }
}
impl windows_core::TypeKind for DNS_KEY_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_KEY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wFlags == other.wFlags && self.chProtocol == other.chProtocol && self.chAlgorithm == other.chAlgorithm && self.wKeyLength == other.wKeyLength && self.wPad == other.wPad && self.Key == other.Key
    }
}
impl Eq for DNS_KEY_DATA {}
impl Default for DNS_KEY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_LOC_DATA {
    pub wVersion: u16,
    pub wSize: u16,
    pub wHorPrec: u16,
    pub wVerPrec: u16,
    pub dwLatitude: u32,
    pub dwLongitude: u32,
    pub dwAltitude: u32,
}
impl Copy for DNS_LOC_DATA {}
impl Clone for DNS_LOC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_LOC_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_LOC_DATA").field("wVersion", &self.wVersion).field("wSize", &self.wSize).field("wHorPrec", &self.wHorPrec).field("wVerPrec", &self.wVerPrec).field("dwLatitude", &self.dwLatitude).field("dwLongitude", &self.dwLongitude).field("dwAltitude", &self.dwAltitude).finish()
    }
}
impl windows_core::TypeKind for DNS_LOC_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_LOC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wSize == other.wSize && self.wHorPrec == other.wHorPrec && self.wVerPrec == other.wVerPrec && self.dwLatitude == other.dwLatitude && self.dwLongitude == other.dwLongitude && self.dwAltitude == other.dwAltitude
    }
}
impl Eq for DNS_LOC_DATA {}
impl Default for DNS_LOC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_MESSAGE_BUFFER {
    pub MessageHead: DNS_HEADER,
    pub MessageBody: [i8; 1],
}
impl Copy for DNS_MESSAGE_BUFFER {}
impl Clone for DNS_MESSAGE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_MESSAGE_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_MESSAGE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_MINFO_DATAA {
    pub pNameMailbox: windows_core::PSTR,
    pub pNameErrorsMailbox: windows_core::PSTR,
}
impl Copy for DNS_MINFO_DATAA {}
impl Clone for DNS_MINFO_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_MINFO_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_MINFO_DATAA").field("pNameMailbox", &self.pNameMailbox).field("pNameErrorsMailbox", &self.pNameErrorsMailbox).finish()
    }
}
impl windows_core::TypeKind for DNS_MINFO_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_MINFO_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameMailbox == other.pNameMailbox && self.pNameErrorsMailbox == other.pNameErrorsMailbox
    }
}
impl Eq for DNS_MINFO_DATAA {}
impl Default for DNS_MINFO_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_MINFO_DATAW {
    pub pNameMailbox: windows_core::PWSTR,
    pub pNameErrorsMailbox: windows_core::PWSTR,
}
impl Copy for DNS_MINFO_DATAW {}
impl Clone for DNS_MINFO_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_MINFO_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_MINFO_DATAW").field("pNameMailbox", &self.pNameMailbox).field("pNameErrorsMailbox", &self.pNameErrorsMailbox).finish()
    }
}
impl windows_core::TypeKind for DNS_MINFO_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_MINFO_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameMailbox == other.pNameMailbox && self.pNameErrorsMailbox == other.pNameErrorsMailbox
    }
}
impl Eq for DNS_MINFO_DATAW {}
impl Default for DNS_MINFO_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_MX_DATAA {
    pub pNameExchange: windows_core::PSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
impl Copy for DNS_MX_DATAA {}
impl Clone for DNS_MX_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_MX_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_MX_DATAA").field("pNameExchange", &self.pNameExchange).field("wPreference", &self.wPreference).field("Pad", &self.Pad).finish()
    }
}
impl windows_core::TypeKind for DNS_MX_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_MX_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameExchange == other.pNameExchange && self.wPreference == other.wPreference && self.Pad == other.Pad
    }
}
impl Eq for DNS_MX_DATAA {}
impl Default for DNS_MX_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_MX_DATAW {
    pub pNameExchange: windows_core::PWSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
impl Copy for DNS_MX_DATAW {}
impl Clone for DNS_MX_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_MX_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_MX_DATAW").field("pNameExchange", &self.pNameExchange).field("wPreference", &self.wPreference).field("Pad", &self.Pad).finish()
    }
}
impl windows_core::TypeKind for DNS_MX_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_MX_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameExchange == other.pNameExchange && self.wPreference == other.wPreference && self.Pad == other.Pad
    }
}
impl Eq for DNS_MX_DATAW {}
impl Default for DNS_MX_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NAPTR_DATAA {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: windows_core::PSTR,
    pub pService: windows_core::PSTR,
    pub pRegularExpression: windows_core::PSTR,
    pub pReplacement: windows_core::PSTR,
}
impl Copy for DNS_NAPTR_DATAA {}
impl Clone for DNS_NAPTR_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NAPTR_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NAPTR_DATAA").field("wOrder", &self.wOrder).field("wPreference", &self.wPreference).field("pFlags", &self.pFlags).field("pService", &self.pService).field("pRegularExpression", &self.pRegularExpression).field("pReplacement", &self.pReplacement).finish()
    }
}
impl windows_core::TypeKind for DNS_NAPTR_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NAPTR_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.wOrder == other.wOrder && self.wPreference == other.wPreference && self.pFlags == other.pFlags && self.pService == other.pService && self.pRegularExpression == other.pRegularExpression && self.pReplacement == other.pReplacement
    }
}
impl Eq for DNS_NAPTR_DATAA {}
impl Default for DNS_NAPTR_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NAPTR_DATAW {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: windows_core::PWSTR,
    pub pService: windows_core::PWSTR,
    pub pRegularExpression: windows_core::PWSTR,
    pub pReplacement: windows_core::PWSTR,
}
impl Copy for DNS_NAPTR_DATAW {}
impl Clone for DNS_NAPTR_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NAPTR_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NAPTR_DATAW").field("wOrder", &self.wOrder).field("wPreference", &self.wPreference).field("pFlags", &self.pFlags).field("pService", &self.pService).field("pRegularExpression", &self.pRegularExpression).field("pReplacement", &self.pReplacement).finish()
    }
}
impl windows_core::TypeKind for DNS_NAPTR_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NAPTR_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.wOrder == other.wOrder && self.wPreference == other.wPreference && self.pFlags == other.pFlags && self.pService == other.pService && self.pRegularExpression == other.pRegularExpression && self.pReplacement == other.pReplacement
    }
}
impl Eq for DNS_NAPTR_DATAW {}
impl Default for DNS_NAPTR_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NSEC3PARAM_DATA {
    pub chAlgorithm: u8,
    pub bFlags: u8,
    pub wIterations: u16,
    pub bSaltLength: u8,
    pub bPad: [u8; 3],
    pub pbSalt: [u8; 1],
}
impl Copy for DNS_NSEC3PARAM_DATA {}
impl Clone for DNS_NSEC3PARAM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NSEC3PARAM_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NSEC3PARAM_DATA").field("chAlgorithm", &self.chAlgorithm).field("bFlags", &self.bFlags).field("wIterations", &self.wIterations).field("bSaltLength", &self.bSaltLength).field("bPad", &self.bPad).field("pbSalt", &self.pbSalt).finish()
    }
}
impl windows_core::TypeKind for DNS_NSEC3PARAM_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NSEC3PARAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.chAlgorithm == other.chAlgorithm && self.bFlags == other.bFlags && self.wIterations == other.wIterations && self.bSaltLength == other.bSaltLength && self.bPad == other.bPad && self.pbSalt == other.pbSalt
    }
}
impl Eq for DNS_NSEC3PARAM_DATA {}
impl Default for DNS_NSEC3PARAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NSEC3_DATA {
    pub chAlgorithm: u8,
    pub bFlags: u8,
    pub wIterations: u16,
    pub bSaltLength: u8,
    pub bHashLength: u8,
    pub wTypeBitMapsLength: u16,
    pub chData: [u8; 1],
}
impl Copy for DNS_NSEC3_DATA {}
impl Clone for DNS_NSEC3_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NSEC3_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NSEC3_DATA").field("chAlgorithm", &self.chAlgorithm).field("bFlags", &self.bFlags).field("wIterations", &self.wIterations).field("bSaltLength", &self.bSaltLength).field("bHashLength", &self.bHashLength).field("wTypeBitMapsLength", &self.wTypeBitMapsLength).field("chData", &self.chData).finish()
    }
}
impl windows_core::TypeKind for DNS_NSEC3_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NSEC3_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.chAlgorithm == other.chAlgorithm && self.bFlags == other.bFlags && self.wIterations == other.wIterations && self.bSaltLength == other.bSaltLength && self.bHashLength == other.bHashLength && self.wTypeBitMapsLength == other.wTypeBitMapsLength && self.chData == other.chData
    }
}
impl Eq for DNS_NSEC3_DATA {}
impl Default for DNS_NSEC3_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NSEC_DATAA {
    pub pNextDomainName: windows_core::PSTR,
    pub wTypeBitMapsLength: u16,
    pub wPad: u16,
    pub TypeBitMaps: [u8; 1],
}
impl Copy for DNS_NSEC_DATAA {}
impl Clone for DNS_NSEC_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NSEC_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NSEC_DATAA").field("pNextDomainName", &self.pNextDomainName).field("wTypeBitMapsLength", &self.wTypeBitMapsLength).field("wPad", &self.wPad).field("TypeBitMaps", &self.TypeBitMaps).finish()
    }
}
impl windows_core::TypeKind for DNS_NSEC_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NSEC_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNextDomainName == other.pNextDomainName && self.wTypeBitMapsLength == other.wTypeBitMapsLength && self.wPad == other.wPad && self.TypeBitMaps == other.TypeBitMaps
    }
}
impl Eq for DNS_NSEC_DATAA {}
impl Default for DNS_NSEC_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NSEC_DATAW {
    pub pNextDomainName: windows_core::PWSTR,
    pub wTypeBitMapsLength: u16,
    pub wPad: u16,
    pub TypeBitMaps: [u8; 1],
}
impl Copy for DNS_NSEC_DATAW {}
impl Clone for DNS_NSEC_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NSEC_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NSEC_DATAW").field("pNextDomainName", &self.pNextDomainName).field("wTypeBitMapsLength", &self.wTypeBitMapsLength).field("wPad", &self.wPad).field("TypeBitMaps", &self.TypeBitMaps).finish()
    }
}
impl windows_core::TypeKind for DNS_NSEC_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NSEC_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNextDomainName == other.pNextDomainName && self.wTypeBitMapsLength == other.wTypeBitMapsLength && self.wPad == other.wPad && self.TypeBitMaps == other.TypeBitMaps
    }
}
impl Eq for DNS_NSEC_DATAW {}
impl Default for DNS_NSEC_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NULL_DATA {
    pub dwByteCount: u32,
    pub Data: [u8; 1],
}
impl Copy for DNS_NULL_DATA {}
impl Clone for DNS_NULL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NULL_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NULL_DATA").field("dwByteCount", &self.dwByteCount).field("Data", &self.Data).finish()
    }
}
impl windows_core::TypeKind for DNS_NULL_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NULL_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwByteCount == other.dwByteCount && self.Data == other.Data
    }
}
impl Eq for DNS_NULL_DATA {}
impl Default for DNS_NULL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NXT_DATAA {
    pub pNameNext: windows_core::PSTR,
    pub wNumTypes: u16,
    pub wTypes: [u16; 1],
}
impl Copy for DNS_NXT_DATAA {}
impl Clone for DNS_NXT_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NXT_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NXT_DATAA").field("pNameNext", &self.pNameNext).field("wNumTypes", &self.wNumTypes).field("wTypes", &self.wTypes).finish()
    }
}
impl windows_core::TypeKind for DNS_NXT_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NXT_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameNext == other.pNameNext && self.wNumTypes == other.wNumTypes && self.wTypes == other.wTypes
    }
}
impl Eq for DNS_NXT_DATAA {}
impl Default for DNS_NXT_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_NXT_DATAW {
    pub pNameNext: windows_core::PWSTR,
    pub wNumTypes: u16,
    pub wTypes: [u16; 1],
}
impl Copy for DNS_NXT_DATAW {}
impl Clone for DNS_NXT_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_NXT_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_NXT_DATAW").field("pNameNext", &self.pNameNext).field("wNumTypes", &self.wNumTypes).field("wTypes", &self.wTypes).finish()
    }
}
impl windows_core::TypeKind for DNS_NXT_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_NXT_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameNext == other.pNameNext && self.wNumTypes == other.wNumTypes && self.wTypes == other.wTypes
    }
}
impl Eq for DNS_NXT_DATAW {}
impl Default for DNS_NXT_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_OPT_DATA {
    pub wDataLength: u16,
    pub wPad: u16,
    pub Data: [u8; 1],
}
impl Copy for DNS_OPT_DATA {}
impl Clone for DNS_OPT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_OPT_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_OPT_DATA").field("wDataLength", &self.wDataLength).field("wPad", &self.wPad).field("Data", &self.Data).finish()
    }
}
impl windows_core::TypeKind for DNS_OPT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_OPT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wDataLength == other.wDataLength && self.wPad == other.wPad && self.Data == other.Data
    }
}
impl Eq for DNS_OPT_DATA {}
impl Default for DNS_OPT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_PROXY_INFORMATION {
    pub version: u32,
    pub proxyInformationType: DNS_PROXY_INFORMATION_TYPE,
    pub proxyName: windows_core::PWSTR,
}
impl Copy for DNS_PROXY_INFORMATION {}
impl Clone for DNS_PROXY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_PROXY_INFORMATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_PROXY_INFORMATION").field("version", &self.version).field("proxyInformationType", &self.proxyInformationType).field("proxyName", &self.proxyName).finish()
    }
}
impl windows_core::TypeKind for DNS_PROXY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_PROXY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.proxyInformationType == other.proxyInformationType && self.proxyName == other.proxyName
    }
}
impl Eq for DNS_PROXY_INFORMATION {}
impl Default for DNS_PROXY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_PTR_DATAA {
    pub pNameHost: windows_core::PSTR,
}
impl Copy for DNS_PTR_DATAA {}
impl Clone for DNS_PTR_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_PTR_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_PTR_DATAA").field("pNameHost", &self.pNameHost).finish()
    }
}
impl windows_core::TypeKind for DNS_PTR_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_PTR_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameHost == other.pNameHost
    }
}
impl Eq for DNS_PTR_DATAA {}
impl Default for DNS_PTR_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_PTR_DATAW {
    pub pNameHost: windows_core::PWSTR,
}
impl Copy for DNS_PTR_DATAW {}
impl Clone for DNS_PTR_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_PTR_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_PTR_DATAW").field("pNameHost", &self.pNameHost).finish()
    }
}
impl windows_core::TypeKind for DNS_PTR_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_PTR_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameHost == other.pNameHost
    }
}
impl Eq for DNS_PTR_DATAW {}
impl Default for DNS_PTR_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_QUERY_CANCEL {
    pub Reserved: [i8; 32],
}
impl Copy for DNS_QUERY_CANCEL {}
impl Clone for DNS_QUERY_CANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_QUERY_CANCEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_QUERY_CANCEL").field("Reserved", &self.Reserved).finish()
    }
}
impl windows_core::TypeKind for DNS_QUERY_CANCEL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_QUERY_CANCEL {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl Eq for DNS_QUERY_CANCEL {}
impl Default for DNS_QUERY_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_QUERY_RAW_CANCEL {
    pub reserved: [i8; 32],
}
impl Copy for DNS_QUERY_RAW_CANCEL {}
impl Clone for DNS_QUERY_RAW_CANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_QUERY_RAW_CANCEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_QUERY_RAW_CANCEL").field("reserved", &self.reserved).finish()
    }
}
impl windows_core::TypeKind for DNS_QUERY_RAW_CANCEL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_QUERY_RAW_CANCEL {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl Eq for DNS_QUERY_RAW_CANCEL {}
impl Default for DNS_QUERY_RAW_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_QUERY_RAW_REQUEST {
    pub version: u32,
    pub resultsVersion: u32,
    pub dnsQueryRawSize: u32,
    pub dnsQueryRaw: *mut u8,
    pub dnsQueryName: windows_core::PWSTR,
    pub dnsQueryType: u16,
    pub queryOptions: u64,
    pub interfaceIndex: u32,
    pub queryCompletionCallback: DNS_QUERY_RAW_COMPLETION_ROUTINE,
    pub queryContext: *mut core::ffi::c_void,
    pub queryRawOptions: u64,
    pub customServersSize: u32,
    pub customServers: *mut DNS_CUSTOM_SERVER,
    pub protocol: u32,
    pub Anonymous: DNS_QUERY_RAW_REQUEST_0,
}
impl Copy for DNS_QUERY_RAW_REQUEST {}
impl Clone for DNS_QUERY_RAW_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_QUERY_RAW_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_QUERY_RAW_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_QUERY_RAW_REQUEST_0 {
    pub maxSa: [i8; 32],
}
impl Copy for DNS_QUERY_RAW_REQUEST_0 {}
impl Clone for DNS_QUERY_RAW_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_QUERY_RAW_REQUEST_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_QUERY_RAW_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_QUERY_RAW_RESULT {
    pub version: u32,
    pub queryStatus: i32,
    pub queryOptions: u64,
    pub queryRawOptions: u64,
    pub responseFlags: u64,
    pub queryRawResponseSize: u32,
    pub queryRawResponse: *mut u8,
    pub queryRecords: *mut DNS_RECORDA,
    pub protocol: u32,
    pub Anonymous: DNS_QUERY_RAW_RESULT_0,
}
impl Copy for DNS_QUERY_RAW_RESULT {}
impl Clone for DNS_QUERY_RAW_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_QUERY_RAW_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_QUERY_RAW_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_QUERY_RAW_RESULT_0 {
    pub maxSa: [i8; 32],
}
impl Copy for DNS_QUERY_RAW_RESULT_0 {}
impl Clone for DNS_QUERY_RAW_RESULT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_QUERY_RAW_RESULT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_QUERY_RAW_RESULT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_QUERY_REQUEST {
    pub Version: u32,
    pub QueryName: windows_core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: *mut DNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut core::ffi::c_void,
}
impl Copy for DNS_QUERY_REQUEST {}
impl Clone for DNS_QUERY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_QUERY_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_QUERY_REQUEST").field("Version", &self.Version).field("QueryName", &self.QueryName).field("QueryType", &self.QueryType).field("QueryOptions", &self.QueryOptions).field("pDnsServerList", &self.pDnsServerList).field("InterfaceIndex", &self.InterfaceIndex).field("pQueryContext", &self.pQueryContext).finish()
    }
}
impl windows_core::TypeKind for DNS_QUERY_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_QUERY_REQUEST3 {
    pub Version: u32,
    pub QueryName: windows_core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: *mut DNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut core::ffi::c_void,
    pub IsNetworkQueryRequired: super::super::Foundation::BOOL,
    pub RequiredNetworkIndex: u32,
    pub cCustomServers: u32,
    pub pCustomServers: *mut DNS_CUSTOM_SERVER,
}
impl Copy for DNS_QUERY_REQUEST3 {}
impl Clone for DNS_QUERY_REQUEST3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_QUERY_REQUEST3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_QUERY_REQUEST3")
            .field("Version", &self.Version)
            .field("QueryName", &self.QueryName)
            .field("QueryType", &self.QueryType)
            .field("QueryOptions", &self.QueryOptions)
            .field("pDnsServerList", &self.pDnsServerList)
            .field("InterfaceIndex", &self.InterfaceIndex)
            .field("pQueryContext", &self.pQueryContext)
            .field("IsNetworkQueryRequired", &self.IsNetworkQueryRequired)
            .field("RequiredNetworkIndex", &self.RequiredNetworkIndex)
            .field("cCustomServers", &self.cCustomServers)
            .field("pCustomServers", &self.pCustomServers)
            .finish()
    }
}
impl windows_core::TypeKind for DNS_QUERY_REQUEST3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_QUERY_REQUEST3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_QUERY_RESULT {
    pub Version: u32,
    pub QueryStatus: i32,
    pub QueryOptions: u64,
    pub pQueryRecords: *mut DNS_RECORDA,
    pub Reserved: *mut core::ffi::c_void,
}
impl Copy for DNS_QUERY_RESULT {}
impl Clone for DNS_QUERY_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_QUERY_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_QUERY_RESULT").field("Version", &self.Version).field("QueryStatus", &self.QueryStatus).field("QueryOptions", &self.QueryOptions).field("pQueryRecords", &self.pQueryRecords).field("Reserved", &self.Reserved).finish()
    }
}
impl windows_core::TypeKind for DNS_QUERY_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_QUERY_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.QueryStatus == other.QueryStatus && self.QueryOptions == other.QueryOptions && self.pQueryRecords == other.pQueryRecords && self.Reserved == other.Reserved
    }
}
impl Eq for DNS_QUERY_RESULT {}
impl Default for DNS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_RECORDA {
    pub pNext: *mut DNS_RECORDA,
    pub pName: windows_core::PSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORDA_1,
    pub dwTtl: u32,
    pub dwReserved: u32,
    pub Data: DNS_RECORDA_0,
}
impl Copy for DNS_RECORDA {}
impl Clone for DNS_RECORDA {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORDA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_RECORDA_0 {
    pub A: DNS_A_DATA,
    pub SOA: DNS_SOA_DATAA,
    pub Soa: DNS_SOA_DATAA,
    pub PTR: DNS_PTR_DATAA,
    pub Ptr: DNS_PTR_DATAA,
    pub NS: DNS_PTR_DATAA,
    pub Ns: DNS_PTR_DATAA,
    pub CNAME: DNS_PTR_DATAA,
    pub Cname: DNS_PTR_DATAA,
    pub DNAME: DNS_PTR_DATAA,
    pub Dname: DNS_PTR_DATAA,
    pub MB: DNS_PTR_DATAA,
    pub Mb: DNS_PTR_DATAA,
    pub MD: DNS_PTR_DATAA,
    pub Md: DNS_PTR_DATAA,
    pub MF: DNS_PTR_DATAA,
    pub Mf: DNS_PTR_DATAA,
    pub MG: DNS_PTR_DATAA,
    pub Mg: DNS_PTR_DATAA,
    pub MR: DNS_PTR_DATAA,
    pub Mr: DNS_PTR_DATAA,
    pub MINFO: DNS_MINFO_DATAA,
    pub Minfo: DNS_MINFO_DATAA,
    pub RP: DNS_MINFO_DATAA,
    pub Rp: DNS_MINFO_DATAA,
    pub MX: DNS_MX_DATAA,
    pub Mx: DNS_MX_DATAA,
    pub AFSDB: DNS_MX_DATAA,
    pub Afsdb: DNS_MX_DATAA,
    pub RT: DNS_MX_DATAA,
    pub Rt: DNS_MX_DATAA,
    pub HINFO: DNS_TXT_DATAA,
    pub Hinfo: DNS_TXT_DATAA,
    pub ISDN: DNS_TXT_DATAA,
    pub Isdn: DNS_TXT_DATAA,
    pub TXT: DNS_TXT_DATAA,
    pub Txt: DNS_TXT_DATAA,
    pub X25: DNS_TXT_DATAA,
    pub Null: DNS_NULL_DATA,
    pub WKS: DNS_WKS_DATA,
    pub Wks: DNS_WKS_DATA,
    pub AAAA: DNS_AAAA_DATA,
    pub KEY: DNS_KEY_DATA,
    pub Key: DNS_KEY_DATA,
    pub SIG: DNS_SIG_DATAA,
    pub Sig: DNS_SIG_DATAA,
    pub ATMA: DNS_ATMA_DATA,
    pub Atma: DNS_ATMA_DATA,
    pub NXT: DNS_NXT_DATAA,
    pub Nxt: DNS_NXT_DATAA,
    pub SRV: DNS_SRV_DATAA,
    pub Srv: DNS_SRV_DATAA,
    pub NAPTR: DNS_NAPTR_DATAA,
    pub Naptr: DNS_NAPTR_DATAA,
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
    pub DS: DNS_DS_DATA,
    pub Ds: DNS_DS_DATA,
    pub RRSIG: DNS_SIG_DATAA,
    pub Rrsig: DNS_SIG_DATAA,
    pub NSEC: DNS_NSEC_DATAA,
    pub Nsec: DNS_NSEC_DATAA,
    pub DNSKEY: DNS_KEY_DATA,
    pub Dnskey: DNS_KEY_DATA,
    pub TKEY: DNS_TKEY_DATAA,
    pub Tkey: DNS_TKEY_DATAA,
    pub TSIG: DNS_TSIG_DATAA,
    pub Tsig: DNS_TSIG_DATAA,
    pub WINS: DNS_WINS_DATA,
    pub Wins: DNS_WINS_DATA,
    pub WINSR: DNS_WINSR_DATAA,
    pub WinsR: DNS_WINSR_DATAA,
    pub NBSTAT: DNS_WINSR_DATAA,
    pub Nbstat: DNS_WINSR_DATAA,
    pub DHCID: DNS_DHCID_DATA,
    pub NSEC3: DNS_NSEC3_DATA,
    pub Nsec3: DNS_NSEC3_DATA,
    pub NSEC3PARAM: DNS_NSEC3PARAM_DATA,
    pub Nsec3Param: DNS_NSEC3PARAM_DATA,
    pub TLSA: DNS_TLSA_DATA,
    pub Tlsa: DNS_TLSA_DATA,
    pub SVCB: DNS_SVCB_DATA,
    pub Svcb: DNS_SVCB_DATA,
    pub UNKNOWN: DNS_UNKNOWN_DATA,
    pub Unknown: DNS_UNKNOWN_DATA,
    pub pDataPtr: *mut u8,
}
impl Copy for DNS_RECORDA_0 {}
impl Clone for DNS_RECORDA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORDA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORDA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_RECORDA_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
impl Copy for DNS_RECORDA_1 {}
impl Clone for DNS_RECORDA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORDA_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORDA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_RECORDW {
    pub pNext: *mut DNS_RECORDW,
    pub pName: windows_core::PWSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORDW_1,
    pub dwTtl: u32,
    pub dwReserved: u32,
    pub Data: DNS_RECORDW_0,
}
impl Copy for DNS_RECORDW {}
impl Clone for DNS_RECORDW {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORDW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_RECORDW_0 {
    pub A: DNS_A_DATA,
    pub SOA: DNS_SOA_DATAW,
    pub Soa: DNS_SOA_DATAW,
    pub PTR: DNS_PTR_DATAW,
    pub Ptr: DNS_PTR_DATAW,
    pub NS: DNS_PTR_DATAW,
    pub Ns: DNS_PTR_DATAW,
    pub CNAME: DNS_PTR_DATAW,
    pub Cname: DNS_PTR_DATAW,
    pub DNAME: DNS_PTR_DATAW,
    pub Dname: DNS_PTR_DATAW,
    pub MB: DNS_PTR_DATAW,
    pub Mb: DNS_PTR_DATAW,
    pub MD: DNS_PTR_DATAW,
    pub Md: DNS_PTR_DATAW,
    pub MF: DNS_PTR_DATAW,
    pub Mf: DNS_PTR_DATAW,
    pub MG: DNS_PTR_DATAW,
    pub Mg: DNS_PTR_DATAW,
    pub MR: DNS_PTR_DATAW,
    pub Mr: DNS_PTR_DATAW,
    pub MINFO: DNS_MINFO_DATAW,
    pub Minfo: DNS_MINFO_DATAW,
    pub RP: DNS_MINFO_DATAW,
    pub Rp: DNS_MINFO_DATAW,
    pub MX: DNS_MX_DATAW,
    pub Mx: DNS_MX_DATAW,
    pub AFSDB: DNS_MX_DATAW,
    pub Afsdb: DNS_MX_DATAW,
    pub RT: DNS_MX_DATAW,
    pub Rt: DNS_MX_DATAW,
    pub HINFO: DNS_TXT_DATAW,
    pub Hinfo: DNS_TXT_DATAW,
    pub ISDN: DNS_TXT_DATAW,
    pub Isdn: DNS_TXT_DATAW,
    pub TXT: DNS_TXT_DATAW,
    pub Txt: DNS_TXT_DATAW,
    pub X25: DNS_TXT_DATAW,
    pub Null: DNS_NULL_DATA,
    pub WKS: DNS_WKS_DATA,
    pub Wks: DNS_WKS_DATA,
    pub AAAA: DNS_AAAA_DATA,
    pub KEY: DNS_KEY_DATA,
    pub Key: DNS_KEY_DATA,
    pub SIG: DNS_SIG_DATAW,
    pub Sig: DNS_SIG_DATAW,
    pub ATMA: DNS_ATMA_DATA,
    pub Atma: DNS_ATMA_DATA,
    pub NXT: DNS_NXT_DATAW,
    pub Nxt: DNS_NXT_DATAW,
    pub SRV: DNS_SRV_DATAW,
    pub Srv: DNS_SRV_DATAW,
    pub NAPTR: DNS_NAPTR_DATAW,
    pub Naptr: DNS_NAPTR_DATAW,
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
    pub DS: DNS_DS_DATA,
    pub Ds: DNS_DS_DATA,
    pub RRSIG: DNS_SIG_DATAW,
    pub Rrsig: DNS_SIG_DATAW,
    pub NSEC: DNS_NSEC_DATAW,
    pub Nsec: DNS_NSEC_DATAW,
    pub DNSKEY: DNS_KEY_DATA,
    pub Dnskey: DNS_KEY_DATA,
    pub TKEY: DNS_TKEY_DATAW,
    pub Tkey: DNS_TKEY_DATAW,
    pub TSIG: DNS_TSIG_DATAW,
    pub Tsig: DNS_TSIG_DATAW,
    pub WINS: DNS_WINS_DATA,
    pub Wins: DNS_WINS_DATA,
    pub WINSR: DNS_WINSR_DATAW,
    pub WinsR: DNS_WINSR_DATAW,
    pub NBSTAT: DNS_WINSR_DATAW,
    pub Nbstat: DNS_WINSR_DATAW,
    pub DHCID: DNS_DHCID_DATA,
    pub NSEC3: DNS_NSEC3_DATA,
    pub Nsec3: DNS_NSEC3_DATA,
    pub NSEC3PARAM: DNS_NSEC3PARAM_DATA,
    pub Nsec3Param: DNS_NSEC3PARAM_DATA,
    pub TLSA: DNS_TLSA_DATA,
    pub Tlsa: DNS_TLSA_DATA,
    pub SVCB: DNS_SVCB_DATA,
    pub Svcb: DNS_SVCB_DATA,
    pub UNKNOWN: DNS_UNKNOWN_DATA,
    pub Unknown: DNS_UNKNOWN_DATA,
    pub pDataPtr: *mut u8,
}
impl Copy for DNS_RECORDW_0 {}
impl Clone for DNS_RECORDW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORDW_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORDW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_RECORDW_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
impl Copy for DNS_RECORDW_1 {}
impl Clone for DNS_RECORDW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORDW_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORDW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_RECORD_FLAGS {
    pub _bitfield: u32,
}
impl Copy for DNS_RECORD_FLAGS {}
impl Clone for DNS_RECORD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_RECORD_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_RECORD_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl windows_core::TypeKind for DNS_RECORD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_RECORD_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl Eq for DNS_RECORD_FLAGS {}
impl Default for DNS_RECORD_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_RECORD_OPTW {
    pub pNext: *mut DNS_RECORDW,
    pub pName: windows_core::PWSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORD_OPTW_1,
    pub ExtHeader: DNS_HEADER_EXT,
    pub wPayloadSize: u16,
    pub wReserved: u16,
    pub Data: DNS_RECORD_OPTW_0,
}
impl Copy for DNS_RECORD_OPTW {}
impl Clone for DNS_RECORD_OPTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORD_OPTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORD_OPTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_RECORD_OPTW_0 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
impl Copy for DNS_RECORD_OPTW_0 {}
impl Clone for DNS_RECORD_OPTW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORD_OPTW_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORD_OPTW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_RECORD_OPTW_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
impl Copy for DNS_RECORD_OPTW_1 {}
impl Clone for DNS_RECORD_OPTW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_RECORD_OPTW_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_RECORD_OPTW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_RRSET {
    pub pFirstRR: *mut DNS_RECORDA,
    pub pLastRR: *mut DNS_RECORDA,
}
impl Copy for DNS_RRSET {}
impl Clone for DNS_RRSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_RRSET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_RRSET").field("pFirstRR", &self.pFirstRR).field("pLastRR", &self.pLastRR).finish()
    }
}
impl windows_core::TypeKind for DNS_RRSET {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_RRSET {
    fn eq(&self, other: &Self) -> bool {
        self.pFirstRR == other.pFirstRR && self.pLastRR == other.pLastRR
    }
}
impl Eq for DNS_RRSET {}
impl Default for DNS_RRSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SERVICE_BROWSE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: windows_core::PCWSTR,
    pub Anonymous: DNS_SERVICE_BROWSE_REQUEST_0,
    pub pQueryContext: *mut core::ffi::c_void,
}
impl Copy for DNS_SERVICE_BROWSE_REQUEST {}
impl Clone for DNS_SERVICE_BROWSE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_SERVICE_BROWSE_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_SERVICE_BROWSE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_SERVICE_BROWSE_REQUEST_0 {
    pub pBrowseCallback: PDNS_SERVICE_BROWSE_CALLBACK,
    pub pBrowseCallbackV2: PDNS_QUERY_COMPLETION_ROUTINE,
}
impl Copy for DNS_SERVICE_BROWSE_REQUEST_0 {}
impl Clone for DNS_SERVICE_BROWSE_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_SERVICE_BROWSE_REQUEST_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_SERVICE_BROWSE_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SERVICE_CANCEL {
    pub reserved: *mut core::ffi::c_void,
}
impl Copy for DNS_SERVICE_CANCEL {}
impl Clone for DNS_SERVICE_CANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SERVICE_CANCEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SERVICE_CANCEL").field("reserved", &self.reserved).finish()
    }
}
impl windows_core::TypeKind for DNS_SERVICE_CANCEL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SERVICE_CANCEL {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl Eq for DNS_SERVICE_CANCEL {}
impl Default for DNS_SERVICE_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SERVICE_INSTANCE {
    pub pszInstanceName: windows_core::PWSTR,
    pub pszHostName: windows_core::PWSTR,
    pub ip4Address: *mut u32,
    pub ip6Address: *mut IP6_ADDRESS,
    pub wPort: u16,
    pub wPriority: u16,
    pub wWeight: u16,
    pub dwPropertyCount: u32,
    pub keys: *mut windows_core::PWSTR,
    pub values: *mut windows_core::PWSTR,
    pub dwInterfaceIndex: u32,
}
impl Copy for DNS_SERVICE_INSTANCE {}
impl Clone for DNS_SERVICE_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SERVICE_INSTANCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SERVICE_INSTANCE").field("pszInstanceName", &self.pszInstanceName).field("pszHostName", &self.pszHostName).field("ip4Address", &self.ip4Address).field("ip6Address", &self.ip6Address).field("wPort", &self.wPort).field("wPriority", &self.wPriority).field("wWeight", &self.wWeight).field("dwPropertyCount", &self.dwPropertyCount).field("keys", &self.keys).field("values", &self.values).field("dwInterfaceIndex", &self.dwInterfaceIndex).finish()
    }
}
impl windows_core::TypeKind for DNS_SERVICE_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SERVICE_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.pszInstanceName == other.pszInstanceName && self.pszHostName == other.pszHostName && self.ip4Address == other.ip4Address && self.ip6Address == other.ip6Address && self.wPort == other.wPort && self.wPriority == other.wPriority && self.wWeight == other.wWeight && self.dwPropertyCount == other.dwPropertyCount && self.keys == other.keys && self.values == other.values && self.dwInterfaceIndex == other.dwInterfaceIndex
    }
}
impl Eq for DNS_SERVICE_INSTANCE {}
impl Default for DNS_SERVICE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SERVICE_REGISTER_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub pServiceInstance: *mut DNS_SERVICE_INSTANCE,
    pub pRegisterCompletionCallback: PDNS_SERVICE_REGISTER_COMPLETE,
    pub pQueryContext: *mut core::ffi::c_void,
    pub hCredentials: super::super::Foundation::HANDLE,
    pub unicastEnabled: super::super::Foundation::BOOL,
}
impl Copy for DNS_SERVICE_REGISTER_REQUEST {}
impl Clone for DNS_SERVICE_REGISTER_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SERVICE_REGISTER_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SERVICE_REGISTER_REQUEST").field("Version", &self.Version).field("InterfaceIndex", &self.InterfaceIndex).field("pServiceInstance", &self.pServiceInstance).field("pQueryContext", &self.pQueryContext).field("hCredentials", &self.hCredentials).field("unicastEnabled", &self.unicastEnabled).finish()
    }
}
impl windows_core::TypeKind for DNS_SERVICE_REGISTER_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_SERVICE_REGISTER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SERVICE_RESOLVE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: windows_core::PWSTR,
    pub pResolveCompletionCallback: PDNS_SERVICE_RESOLVE_COMPLETE,
    pub pQueryContext: *mut core::ffi::c_void,
}
impl Copy for DNS_SERVICE_RESOLVE_REQUEST {}
impl Clone for DNS_SERVICE_RESOLVE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SERVICE_RESOLVE_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SERVICE_RESOLVE_REQUEST").field("Version", &self.Version).field("InterfaceIndex", &self.InterfaceIndex).field("QueryName", &self.QueryName).field("pQueryContext", &self.pQueryContext).finish()
    }
}
impl windows_core::TypeKind for DNS_SERVICE_RESOLVE_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_SERVICE_RESOLVE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SIG_DATAA {
    pub wTypeCovered: u16,
    pub chAlgorithm: u8,
    pub chLabelCount: u8,
    pub dwOriginalTtl: u32,
    pub dwExpiration: u32,
    pub dwTimeSigned: u32,
    pub wKeyTag: u16,
    pub wSignatureLength: u16,
    pub pNameSigner: windows_core::PSTR,
    pub Signature: [u8; 1],
}
impl Copy for DNS_SIG_DATAA {}
impl Clone for DNS_SIG_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SIG_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SIG_DATAA").field("wTypeCovered", &self.wTypeCovered).field("chAlgorithm", &self.chAlgorithm).field("chLabelCount", &self.chLabelCount).field("dwOriginalTtl", &self.dwOriginalTtl).field("dwExpiration", &self.dwExpiration).field("dwTimeSigned", &self.dwTimeSigned).field("wKeyTag", &self.wKeyTag).field("wSignatureLength", &self.wSignatureLength).field("pNameSigner", &self.pNameSigner).field("Signature", &self.Signature).finish()
    }
}
impl windows_core::TypeKind for DNS_SIG_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SIG_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.wTypeCovered == other.wTypeCovered && self.chAlgorithm == other.chAlgorithm && self.chLabelCount == other.chLabelCount && self.dwOriginalTtl == other.dwOriginalTtl && self.dwExpiration == other.dwExpiration && self.dwTimeSigned == other.dwTimeSigned && self.wKeyTag == other.wKeyTag && self.wSignatureLength == other.wSignatureLength && self.pNameSigner == other.pNameSigner && self.Signature == other.Signature
    }
}
impl Eq for DNS_SIG_DATAA {}
impl Default for DNS_SIG_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SIG_DATAW {
    pub wTypeCovered: u16,
    pub chAlgorithm: u8,
    pub chLabelCount: u8,
    pub dwOriginalTtl: u32,
    pub dwExpiration: u32,
    pub dwTimeSigned: u32,
    pub wKeyTag: u16,
    pub wSignatureLength: u16,
    pub pNameSigner: windows_core::PWSTR,
    pub Signature: [u8; 1],
}
impl Copy for DNS_SIG_DATAW {}
impl Clone for DNS_SIG_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SIG_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SIG_DATAW").field("wTypeCovered", &self.wTypeCovered).field("chAlgorithm", &self.chAlgorithm).field("chLabelCount", &self.chLabelCount).field("dwOriginalTtl", &self.dwOriginalTtl).field("dwExpiration", &self.dwExpiration).field("dwTimeSigned", &self.dwTimeSigned).field("wKeyTag", &self.wKeyTag).field("wSignatureLength", &self.wSignatureLength).field("pNameSigner", &self.pNameSigner).field("Signature", &self.Signature).finish()
    }
}
impl windows_core::TypeKind for DNS_SIG_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SIG_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.wTypeCovered == other.wTypeCovered && self.chAlgorithm == other.chAlgorithm && self.chLabelCount == other.chLabelCount && self.dwOriginalTtl == other.dwOriginalTtl && self.dwExpiration == other.dwExpiration && self.dwTimeSigned == other.dwTimeSigned && self.wKeyTag == other.wKeyTag && self.wSignatureLength == other.wSignatureLength && self.pNameSigner == other.pNameSigner && self.Signature == other.Signature
    }
}
impl Eq for DNS_SIG_DATAW {}
impl Default for DNS_SIG_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SOA_DATAA {
    pub pNamePrimaryServer: windows_core::PSTR,
    pub pNameAdministrator: windows_core::PSTR,
    pub dwSerialNo: u32,
    pub dwRefresh: u32,
    pub dwRetry: u32,
    pub dwExpire: u32,
    pub dwDefaultTtl: u32,
}
impl Copy for DNS_SOA_DATAA {}
impl Clone for DNS_SOA_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SOA_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SOA_DATAA").field("pNamePrimaryServer", &self.pNamePrimaryServer).field("pNameAdministrator", &self.pNameAdministrator).field("dwSerialNo", &self.dwSerialNo).field("dwRefresh", &self.dwRefresh).field("dwRetry", &self.dwRetry).field("dwExpire", &self.dwExpire).field("dwDefaultTtl", &self.dwDefaultTtl).finish()
    }
}
impl windows_core::TypeKind for DNS_SOA_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SOA_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNamePrimaryServer == other.pNamePrimaryServer && self.pNameAdministrator == other.pNameAdministrator && self.dwSerialNo == other.dwSerialNo && self.dwRefresh == other.dwRefresh && self.dwRetry == other.dwRetry && self.dwExpire == other.dwExpire && self.dwDefaultTtl == other.dwDefaultTtl
    }
}
impl Eq for DNS_SOA_DATAA {}
impl Default for DNS_SOA_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SOA_DATAW {
    pub pNamePrimaryServer: windows_core::PWSTR,
    pub pNameAdministrator: windows_core::PWSTR,
    pub dwSerialNo: u32,
    pub dwRefresh: u32,
    pub dwRetry: u32,
    pub dwExpire: u32,
    pub dwDefaultTtl: u32,
}
impl Copy for DNS_SOA_DATAW {}
impl Clone for DNS_SOA_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SOA_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SOA_DATAW").field("pNamePrimaryServer", &self.pNamePrimaryServer).field("pNameAdministrator", &self.pNameAdministrator).field("dwSerialNo", &self.dwSerialNo).field("dwRefresh", &self.dwRefresh).field("dwRetry", &self.dwRetry).field("dwExpire", &self.dwExpire).field("dwDefaultTtl", &self.dwDefaultTtl).finish()
    }
}
impl windows_core::TypeKind for DNS_SOA_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SOA_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNamePrimaryServer == other.pNamePrimaryServer && self.pNameAdministrator == other.pNameAdministrator && self.dwSerialNo == other.dwSerialNo && self.dwRefresh == other.dwRefresh && self.dwRetry == other.dwRetry && self.dwExpire == other.dwExpire && self.dwDefaultTtl == other.dwDefaultTtl
    }
}
impl Eq for DNS_SOA_DATAW {}
impl Default for DNS_SOA_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SRV_DATAA {
    pub pNameTarget: windows_core::PSTR,
    pub wPriority: u16,
    pub wWeight: u16,
    pub wPort: u16,
    pub Pad: u16,
}
impl Copy for DNS_SRV_DATAA {}
impl Clone for DNS_SRV_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SRV_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SRV_DATAA").field("pNameTarget", &self.pNameTarget).field("wPriority", &self.wPriority).field("wWeight", &self.wWeight).field("wPort", &self.wPort).field("Pad", &self.Pad).finish()
    }
}
impl windows_core::TypeKind for DNS_SRV_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SRV_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameTarget == other.pNameTarget && self.wPriority == other.wPriority && self.wWeight == other.wWeight && self.wPort == other.wPort && self.Pad == other.Pad
    }
}
impl Eq for DNS_SRV_DATAA {}
impl Default for DNS_SRV_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SRV_DATAW {
    pub pNameTarget: windows_core::PWSTR,
    pub wPriority: u16,
    pub wWeight: u16,
    pub wPort: u16,
    pub Pad: u16,
}
impl Copy for DNS_SRV_DATAW {}
impl Clone for DNS_SRV_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SRV_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SRV_DATAW").field("pNameTarget", &self.pNameTarget).field("wPriority", &self.wPriority).field("wWeight", &self.wWeight).field("wPort", &self.wPort).field("Pad", &self.Pad).finish()
    }
}
impl windows_core::TypeKind for DNS_SRV_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SRV_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameTarget == other.pNameTarget && self.wPriority == other.wPriority && self.wWeight == other.wWeight && self.wPort == other.wPort && self.Pad == other.Pad
    }
}
impl Eq for DNS_SRV_DATAW {}
impl Default for DNS_SRV_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_DATA {
    pub wSvcPriority: u16,
    pub pszTargetName: windows_core::PSTR,
    pub cSvcParams: u16,
    pub pSvcParams: *mut DNS_SVCB_PARAM,
}
impl Copy for DNS_SVCB_DATA {}
impl Clone for DNS_SVCB_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SVCB_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SVCB_DATA").field("wSvcPriority", &self.wSvcPriority).field("pszTargetName", &self.pszTargetName).field("cSvcParams", &self.cSvcParams).field("pSvcParams", &self.pSvcParams).finish()
    }
}
impl windows_core::TypeKind for DNS_SVCB_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SVCB_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wSvcPriority == other.wSvcPriority && self.pszTargetName == other.pszTargetName && self.cSvcParams == other.cSvcParams && self.pSvcParams == other.pSvcParams
    }
}
impl Eq for DNS_SVCB_DATA {}
impl Default for DNS_SVCB_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_PARAM {
    pub wSvcParamKey: u16,
    pub Anonymous: DNS_SVCB_PARAM_0,
}
impl Copy for DNS_SVCB_PARAM {}
impl Clone for DNS_SVCB_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_SVCB_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DNS_SVCB_PARAM_0 {
    pub pIpv4Hints: *mut DNS_SVCB_PARAM_IPV4,
    pub pIpv6Hints: *mut DNS_SVCB_PARAM_IPV6,
    pub pMandatory: *mut DNS_SVCB_PARAM_MANDATORY,
    pub pAlpn: *mut DNS_SVCB_PARAM_ALPN,
    pub wPort: u16,
    pub pUnknown: *mut DNS_SVCB_PARAM_UNKNOWN,
    pub pszDohPath: windows_core::PSTR,
    pub pReserved: *mut core::ffi::c_void,
}
impl Copy for DNS_SVCB_PARAM_0 {}
impl Clone for DNS_SVCB_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_SVCB_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_PARAM_ALPN {
    pub cIds: u16,
    pub rgIds: [DNS_SVCB_PARAM_ALPN_ID; 1],
}
impl Copy for DNS_SVCB_PARAM_ALPN {}
impl Clone for DNS_SVCB_PARAM_ALPN {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SVCB_PARAM_ALPN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SVCB_PARAM_ALPN").field("cIds", &self.cIds).field("rgIds", &self.rgIds).finish()
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM_ALPN {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SVCB_PARAM_ALPN {
    fn eq(&self, other: &Self) -> bool {
        self.cIds == other.cIds && self.rgIds == other.rgIds
    }
}
impl Eq for DNS_SVCB_PARAM_ALPN {}
impl Default for DNS_SVCB_PARAM_ALPN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_PARAM_ALPN_ID {
    pub cBytes: u8,
    pub pbId: *mut u8,
}
impl Copy for DNS_SVCB_PARAM_ALPN_ID {}
impl Clone for DNS_SVCB_PARAM_ALPN_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SVCB_PARAM_ALPN_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SVCB_PARAM_ALPN_ID").field("cBytes", &self.cBytes).field("pbId", &self.pbId).finish()
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM_ALPN_ID {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SVCB_PARAM_ALPN_ID {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes && self.pbId == other.pbId
    }
}
impl Eq for DNS_SVCB_PARAM_ALPN_ID {}
impl Default for DNS_SVCB_PARAM_ALPN_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_PARAM_IPV4 {
    pub cIps: u16,
    pub rgIps: [u32; 1],
}
impl Copy for DNS_SVCB_PARAM_IPV4 {}
impl Clone for DNS_SVCB_PARAM_IPV4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SVCB_PARAM_IPV4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SVCB_PARAM_IPV4").field("cIps", &self.cIps).field("rgIps", &self.rgIps).finish()
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM_IPV4 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SVCB_PARAM_IPV4 {
    fn eq(&self, other: &Self) -> bool {
        self.cIps == other.cIps && self.rgIps == other.rgIps
    }
}
impl Eq for DNS_SVCB_PARAM_IPV4 {}
impl Default for DNS_SVCB_PARAM_IPV4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_PARAM_IPV6 {
    pub cIps: u16,
    pub rgIps: [IP6_ADDRESS; 1],
}
impl Copy for DNS_SVCB_PARAM_IPV6 {}
impl Clone for DNS_SVCB_PARAM_IPV6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM_IPV6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_SVCB_PARAM_IPV6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_PARAM_MANDATORY {
    pub cMandatoryKeys: u16,
    pub rgwMandatoryKeys: [u16; 1],
}
impl Copy for DNS_SVCB_PARAM_MANDATORY {}
impl Clone for DNS_SVCB_PARAM_MANDATORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SVCB_PARAM_MANDATORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SVCB_PARAM_MANDATORY").field("cMandatoryKeys", &self.cMandatoryKeys).field("rgwMandatoryKeys", &self.rgwMandatoryKeys).finish()
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM_MANDATORY {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SVCB_PARAM_MANDATORY {
    fn eq(&self, other: &Self) -> bool {
        self.cMandatoryKeys == other.cMandatoryKeys && self.rgwMandatoryKeys == other.rgwMandatoryKeys
    }
}
impl Eq for DNS_SVCB_PARAM_MANDATORY {}
impl Default for DNS_SVCB_PARAM_MANDATORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_SVCB_PARAM_UNKNOWN {
    pub cBytes: u16,
    pub pbSvcParamValue: [u8; 1],
}
impl Copy for DNS_SVCB_PARAM_UNKNOWN {}
impl Clone for DNS_SVCB_PARAM_UNKNOWN {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_SVCB_PARAM_UNKNOWN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_SVCB_PARAM_UNKNOWN").field("cBytes", &self.cBytes).field("pbSvcParamValue", &self.pbSvcParamValue).finish()
    }
}
impl windows_core::TypeKind for DNS_SVCB_PARAM_UNKNOWN {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_SVCB_PARAM_UNKNOWN {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes && self.pbSvcParamValue == other.pbSvcParamValue
    }
}
impl Eq for DNS_SVCB_PARAM_UNKNOWN {}
impl Default for DNS_SVCB_PARAM_UNKNOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_TKEY_DATAA {
    pub pNameAlgorithm: windows_core::PSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pKey: *mut u8,
    pub pOtherData: *mut u8,
    pub dwCreateTime: u32,
    pub dwExpireTime: u32,
    pub wMode: u16,
    pub wError: u16,
    pub wKeyLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
impl Copy for DNS_TKEY_DATAA {}
impl Clone for DNS_TKEY_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_TKEY_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_TKEY_DATAA")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pKey", &self.pKey)
            .field("pOtherData", &self.pOtherData)
            .field("dwCreateTime", &self.dwCreateTime)
            .field("dwExpireTime", &self.dwExpireTime)
            .field("wMode", &self.wMode)
            .field("wError", &self.wError)
            .field("wKeyLength", &self.wKeyLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
impl windows_core::TypeKind for DNS_TKEY_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_TKEY_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pKey == other.pKey && self.pOtherData == other.pOtherData && self.dwCreateTime == other.dwCreateTime && self.dwExpireTime == other.dwExpireTime && self.wMode == other.wMode && self.wError == other.wError && self.wKeyLength == other.wKeyLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
impl Eq for DNS_TKEY_DATAA {}
impl Default for DNS_TKEY_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_TKEY_DATAW {
    pub pNameAlgorithm: windows_core::PWSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pKey: *mut u8,
    pub pOtherData: *mut u8,
    pub dwCreateTime: u32,
    pub dwExpireTime: u32,
    pub wMode: u16,
    pub wError: u16,
    pub wKeyLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
impl Copy for DNS_TKEY_DATAW {}
impl Clone for DNS_TKEY_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_TKEY_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_TKEY_DATAW")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pKey", &self.pKey)
            .field("pOtherData", &self.pOtherData)
            .field("dwCreateTime", &self.dwCreateTime)
            .field("dwExpireTime", &self.dwExpireTime)
            .field("wMode", &self.wMode)
            .field("wError", &self.wError)
            .field("wKeyLength", &self.wKeyLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
impl windows_core::TypeKind for DNS_TKEY_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_TKEY_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pKey == other.pKey && self.pOtherData == other.pOtherData && self.dwCreateTime == other.dwCreateTime && self.dwExpireTime == other.dwExpireTime && self.wMode == other.wMode && self.wError == other.wError && self.wKeyLength == other.wKeyLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
impl Eq for DNS_TKEY_DATAW {}
impl Default for DNS_TKEY_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_TLSA_DATA {
    pub bCertUsage: u8,
    pub bSelector: u8,
    pub bMatchingType: u8,
    pub bCertificateAssociationDataLength: u16,
    pub bPad: [u8; 3],
    pub bCertificateAssociationData: [u8; 1],
}
impl Copy for DNS_TLSA_DATA {}
impl Clone for DNS_TLSA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_TLSA_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_TLSA_DATA").field("bCertUsage", &self.bCertUsage).field("bSelector", &self.bSelector).field("bMatchingType", &self.bMatchingType).field("bCertificateAssociationDataLength", &self.bCertificateAssociationDataLength).field("bPad", &self.bPad).field("bCertificateAssociationData", &self.bCertificateAssociationData).finish()
    }
}
impl windows_core::TypeKind for DNS_TLSA_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_TLSA_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.bCertUsage == other.bCertUsage && self.bSelector == other.bSelector && self.bMatchingType == other.bMatchingType && self.bCertificateAssociationDataLength == other.bCertificateAssociationDataLength && self.bPad == other.bPad && self.bCertificateAssociationData == other.bCertificateAssociationData
    }
}
impl Eq for DNS_TLSA_DATA {}
impl Default for DNS_TLSA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_TSIG_DATAA {
    pub pNameAlgorithm: windows_core::PSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pSignature: *mut u8,
    pub pOtherData: *mut u8,
    pub i64CreateTime: i64,
    pub wFudgeTime: u16,
    pub wOriginalXid: u16,
    pub wError: u16,
    pub wSigLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
impl Copy for DNS_TSIG_DATAA {}
impl Clone for DNS_TSIG_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_TSIG_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_TSIG_DATAA")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pSignature", &self.pSignature)
            .field("pOtherData", &self.pOtherData)
            .field("i64CreateTime", &self.i64CreateTime)
            .field("wFudgeTime", &self.wFudgeTime)
            .field("wOriginalXid", &self.wOriginalXid)
            .field("wError", &self.wError)
            .field("wSigLength", &self.wSigLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
impl windows_core::TypeKind for DNS_TSIG_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_TSIG_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pSignature == other.pSignature && self.pOtherData == other.pOtherData && self.i64CreateTime == other.i64CreateTime && self.wFudgeTime == other.wFudgeTime && self.wOriginalXid == other.wOriginalXid && self.wError == other.wError && self.wSigLength == other.wSigLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
impl Eq for DNS_TSIG_DATAA {}
impl Default for DNS_TSIG_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_TSIG_DATAW {
    pub pNameAlgorithm: windows_core::PWSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pSignature: *mut u8,
    pub pOtherData: *mut u8,
    pub i64CreateTime: i64,
    pub wFudgeTime: u16,
    pub wOriginalXid: u16,
    pub wError: u16,
    pub wSigLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
impl Copy for DNS_TSIG_DATAW {}
impl Clone for DNS_TSIG_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_TSIG_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_TSIG_DATAW")
            .field("pNameAlgorithm", &self.pNameAlgorithm)
            .field("pAlgorithmPacket", &self.pAlgorithmPacket)
            .field("pSignature", &self.pSignature)
            .field("pOtherData", &self.pOtherData)
            .field("i64CreateTime", &self.i64CreateTime)
            .field("wFudgeTime", &self.wFudgeTime)
            .field("wOriginalXid", &self.wOriginalXid)
            .field("wError", &self.wError)
            .field("wSigLength", &self.wSigLength)
            .field("wOtherLength", &self.wOtherLength)
            .field("cAlgNameLength", &self.cAlgNameLength)
            .field("bPacketPointers", &self.bPacketPointers)
            .finish()
    }
}
impl windows_core::TypeKind for DNS_TSIG_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_TSIG_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.pNameAlgorithm == other.pNameAlgorithm && self.pAlgorithmPacket == other.pAlgorithmPacket && self.pSignature == other.pSignature && self.pOtherData == other.pOtherData && self.i64CreateTime == other.i64CreateTime && self.wFudgeTime == other.wFudgeTime && self.wOriginalXid == other.wOriginalXid && self.wError == other.wError && self.wSigLength == other.wSigLength && self.wOtherLength == other.wOtherLength && self.cAlgNameLength == other.cAlgNameLength && self.bPacketPointers == other.bPacketPointers
    }
}
impl Eq for DNS_TSIG_DATAW {}
impl Default for DNS_TSIG_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_TXT_DATAA {
    pub dwStringCount: u32,
    pub pStringArray: [windows_core::PSTR; 1],
}
impl Copy for DNS_TXT_DATAA {}
impl Clone for DNS_TXT_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_TXT_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_TXT_DATAA").field("dwStringCount", &self.dwStringCount).field("pStringArray", &self.pStringArray).finish()
    }
}
impl windows_core::TypeKind for DNS_TXT_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_TXT_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStringCount == other.dwStringCount && self.pStringArray == other.pStringArray
    }
}
impl Eq for DNS_TXT_DATAA {}
impl Default for DNS_TXT_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_TXT_DATAW {
    pub dwStringCount: u32,
    pub pStringArray: [windows_core::PWSTR; 1],
}
impl Copy for DNS_TXT_DATAW {}
impl Clone for DNS_TXT_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_TXT_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_TXT_DATAW").field("dwStringCount", &self.dwStringCount).field("pStringArray", &self.pStringArray).finish()
    }
}
impl windows_core::TypeKind for DNS_TXT_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_TXT_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStringCount == other.dwStringCount && self.pStringArray == other.pStringArray
    }
}
impl Eq for DNS_TXT_DATAW {}
impl Default for DNS_TXT_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_UNKNOWN_DATA {
    pub dwByteCount: u32,
    pub bData: [u8; 1],
}
impl Copy for DNS_UNKNOWN_DATA {}
impl Clone for DNS_UNKNOWN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_UNKNOWN_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_UNKNOWN_DATA").field("dwByteCount", &self.dwByteCount).field("bData", &self.bData).finish()
    }
}
impl windows_core::TypeKind for DNS_UNKNOWN_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_UNKNOWN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwByteCount == other.dwByteCount && self.bData == other.bData
    }
}
impl Eq for DNS_UNKNOWN_DATA {}
impl Default for DNS_UNKNOWN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_WINSR_DATAA {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: windows_core::PSTR,
}
impl Copy for DNS_WINSR_DATAA {}
impl Clone for DNS_WINSR_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_WINSR_DATAA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_WINSR_DATAA").field("dwMappingFlag", &self.dwMappingFlag).field("dwLookupTimeout", &self.dwLookupTimeout).field("dwCacheTimeout", &self.dwCacheTimeout).field("pNameResultDomain", &self.pNameResultDomain).finish()
    }
}
impl windows_core::TypeKind for DNS_WINSR_DATAA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_WINSR_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMappingFlag == other.dwMappingFlag && self.dwLookupTimeout == other.dwLookupTimeout && self.dwCacheTimeout == other.dwCacheTimeout && self.pNameResultDomain == other.pNameResultDomain
    }
}
impl Eq for DNS_WINSR_DATAA {}
impl Default for DNS_WINSR_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_WINSR_DATAW {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: windows_core::PWSTR,
}
impl Copy for DNS_WINSR_DATAW {}
impl Clone for DNS_WINSR_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_WINSR_DATAW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_WINSR_DATAW").field("dwMappingFlag", &self.dwMappingFlag).field("dwLookupTimeout", &self.dwLookupTimeout).field("dwCacheTimeout", &self.dwCacheTimeout).field("pNameResultDomain", &self.pNameResultDomain).finish()
    }
}
impl windows_core::TypeKind for DNS_WINSR_DATAW {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_WINSR_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwMappingFlag == other.dwMappingFlag && self.dwLookupTimeout == other.dwLookupTimeout && self.dwCacheTimeout == other.dwCacheTimeout && self.pNameResultDomain == other.pNameResultDomain
    }
}
impl Eq for DNS_WINSR_DATAW {}
impl Default for DNS_WINSR_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_WINS_DATA {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub cWinsServerCount: u32,
    pub WinsServers: [u32; 1],
}
impl Copy for DNS_WINS_DATA {}
impl Clone for DNS_WINS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_WINS_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_WINS_DATA").field("dwMappingFlag", &self.dwMappingFlag).field("dwLookupTimeout", &self.dwLookupTimeout).field("dwCacheTimeout", &self.dwCacheTimeout).field("cWinsServerCount", &self.cWinsServerCount).field("WinsServers", &self.WinsServers).finish()
    }
}
impl windows_core::TypeKind for DNS_WINS_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_WINS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMappingFlag == other.dwMappingFlag && self.dwLookupTimeout == other.dwLookupTimeout && self.dwCacheTimeout == other.dwCacheTimeout && self.cWinsServerCount == other.cWinsServerCount && self.WinsServers == other.WinsServers
    }
}
impl Eq for DNS_WINS_DATA {}
impl Default for DNS_WINS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DNS_WIRE_QUESTION {
    pub QuestionType: u16,
    pub QuestionClass: u16,
}
impl Copy for DNS_WIRE_QUESTION {}
impl Clone for DNS_WIRE_QUESTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_WIRE_QUESTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_WIRE_QUESTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DNS_WIRE_RECORD {
    pub RecordType: u16,
    pub RecordClass: u16,
    pub TimeToLive: u32,
    pub DataLength: u16,
}
impl Copy for DNS_WIRE_RECORD {}
impl Clone for DNS_WIRE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DNS_WIRE_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for DNS_WIRE_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DNS_WKS_DATA {
    pub IpAddress: u32,
    pub chProtocol: u8,
    pub BitMask: [u8; 1],
}
impl Copy for DNS_WKS_DATA {}
impl Clone for DNS_WKS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DNS_WKS_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DNS_WKS_DATA").field("IpAddress", &self.IpAddress).field("chProtocol", &self.chProtocol).field("BitMask", &self.BitMask).finish()
    }
}
impl windows_core::TypeKind for DNS_WKS_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DNS_WKS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.IpAddress == other.IpAddress && self.chProtocol == other.chProtocol && self.BitMask == other.BitMask
    }
}
impl Eq for DNS_WKS_DATA {}
impl Default for DNS_WKS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IP4_ARRAY {
    pub AddrCount: u32,
    pub AddrArray: [u32; 1],
}
impl Copy for IP4_ARRAY {}
impl Clone for IP4_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for IP4_ARRAY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IP4_ARRAY").field("AddrCount", &self.AddrCount).field("AddrArray", &self.AddrArray).finish()
    }
}
impl windows_core::TypeKind for IP4_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for IP4_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.AddrCount == other.AddrCount && self.AddrArray == other.AddrArray
    }
}
impl Eq for IP4_ARRAY {}
impl Default for IP4_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub union IP6_ADDRESS {
    pub IP6Qword: [u64; 2],
    pub IP6Dword: [u32; 4],
    pub IP6Word: [u16; 8],
    pub IP6Byte: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Copy for IP6_ADDRESS {}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Clone for IP6_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for IP6_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IP6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
pub union IP6_ADDRESS {
    pub IP6Dword: [u32; 4],
    pub IP6Word: [u16; 8],
    pub IP6Byte: [u8; 16],
}
#[cfg(target_arch = "x86")]
impl Copy for IP6_ADDRESS {}
#[cfg(target_arch = "x86")]
impl Clone for IP6_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for IP6_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for IP6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MDNS_QUERY_HANDLE {
    pub nameBuf: [u16; 256],
    pub wType: u16,
    pub pSubscription: *mut core::ffi::c_void,
    pub pWnfCallbackParams: *mut core::ffi::c_void,
    pub stateNameData: [u32; 2],
}
impl Copy for MDNS_QUERY_HANDLE {}
impl Clone for MDNS_QUERY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MDNS_QUERY_HANDLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MDNS_QUERY_HANDLE").field("nameBuf", &self.nameBuf).field("wType", &self.wType).field("pSubscription", &self.pSubscription).field("pWnfCallbackParams", &self.pWnfCallbackParams).field("stateNameData", &self.stateNameData).finish()
    }
}
impl windows_core::TypeKind for MDNS_QUERY_HANDLE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MDNS_QUERY_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.nameBuf == other.nameBuf && self.wType == other.wType && self.pSubscription == other.pSubscription && self.pWnfCallbackParams == other.pWnfCallbackParams && self.stateNameData == other.stateNameData
    }
}
impl Eq for MDNS_QUERY_HANDLE {}
impl Default for MDNS_QUERY_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MDNS_QUERY_REQUEST {
    pub Version: u32,
    pub ulRefCount: u32,
    pub Query: windows_core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub InterfaceIndex: u32,
    pub pQueryCallback: PMDNS_QUERY_CALLBACK,
    pub pQueryContext: *mut core::ffi::c_void,
    pub fAnswerReceived: super::super::Foundation::BOOL,
    pub ulResendCount: u32,
}
impl Copy for MDNS_QUERY_REQUEST {}
impl Clone for MDNS_QUERY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MDNS_QUERY_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MDNS_QUERY_REQUEST").field("Version", &self.Version).field("ulRefCount", &self.ulRefCount).field("Query", &self.Query).field("QueryType", &self.QueryType).field("QueryOptions", &self.QueryOptions).field("InterfaceIndex", &self.InterfaceIndex).field("pQueryContext", &self.pQueryContext).field("fAnswerReceived", &self.fAnswerReceived).field("ulResendCount", &self.ulResendCount).finish()
    }
}
impl windows_core::TypeKind for MDNS_QUERY_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for MDNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _DnsRecordOptA {
    pub pNext: *mut DNS_RECORDA,
    pub pName: windows_core::PSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: _DnsRecordOptA_1,
    pub ExtHeader: DNS_HEADER_EXT,
    pub wPayloadSize: u16,
    pub wReserved: u16,
    pub Data: _DnsRecordOptA_0,
}
impl Copy for _DnsRecordOptA {}
impl Clone for _DnsRecordOptA {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for _DnsRecordOptA {
    type TypeKind = windows_core::CopyType;
}
impl Default for _DnsRecordOptA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union _DnsRecordOptA_0 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
impl Copy for _DnsRecordOptA_0 {}
impl Clone for _DnsRecordOptA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for _DnsRecordOptA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for _DnsRecordOptA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union _DnsRecordOptA_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
impl Copy for _DnsRecordOptA_1 {}
impl Clone for _DnsRecordOptA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for _DnsRecordOptA_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for _DnsRecordOptA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_PROXY_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(completioncontext: *const core::ffi::c_void, status: i32)>;
pub type DNS_QUERY_RAW_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(querycontext: *const core::ffi::c_void, queryresults: *const DNS_QUERY_RAW_RESULT)>;
pub type PDNS_QUERY_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryresults: *mut DNS_QUERY_RESULT)>;
pub type PDNS_SERVICE_BROWSE_CALLBACK = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pdnsrecord: *const DNS_RECORDW)>;
pub type PDNS_SERVICE_REGISTER_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
pub type PDNS_SERVICE_RESOLVE_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
pub type PMDNS_QUERY_CALLBACK = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryhandle: *mut MDNS_QUERY_HANDLE, pqueryresults: *mut DNS_QUERY_RESULT)>;
