#[inline]
pub unsafe fn SnmpCancelMsg(session: isize, reqid: i32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCancelMsg(session : isize, reqid : i32) -> u32);
    SnmpCancelMsg(session, reqid)
}
#[inline]
pub unsafe fn SnmpCleanup() -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCleanup() -> u32);
    SnmpCleanup()
}
#[inline]
pub unsafe fn SnmpCleanupEx() -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCleanupEx() -> u32);
    SnmpCleanupEx()
}
#[inline]
pub unsafe fn SnmpClose(session: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpClose(session : isize) -> u32);
    SnmpClose(session)
}
#[inline]
pub unsafe fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpContextToStr(context : isize, string : *mut smiOCTETS) -> u32);
    SnmpContextToStr(context, string)
}
#[inline]
pub unsafe fn SnmpCountVbl(vbl: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCountVbl(vbl : isize) -> u32);
    SnmpCountVbl(vbl)
}
#[inline]
pub unsafe fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCreatePdu(session : isize, pdu_type : SNMP_PDU_TYPE, request_id : i32, error_status : i32, error_index : i32, varbindlist : isize) -> isize);
    SnmpCreatePdu(session, pdu_type, request_id, error_status, error_index, varbindlist)
}
#[inline]
pub unsafe fn SnmpCreateSession<P0>(hwnd: P0, wmsg: u32, fcallback: SNMPAPI_CALLBACK, lpclientdata: *mut core::ffi::c_void) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCreateSession(hwnd : super::super::Foundation:: HWND, wmsg : u32, fcallback : SNMPAPI_CALLBACK, lpclientdata : *mut core::ffi::c_void) -> isize);
    SnmpCreateSession(hwnd.param().abi(), wmsg, fcallback, lpclientdata)
}
#[inline]
pub unsafe fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCreateVbl(session : isize, name : *mut smiOID, value : *mut smiVALUE) -> isize);
    SnmpCreateVbl(session, name, value)
}
#[inline]
pub unsafe fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDecodeMsg(session : isize, srcentity : *mut isize, dstentity : *mut isize, context : *mut isize, pdu : *mut isize, msgbufdesc : *mut smiOCTETS) -> u32);
    SnmpDecodeMsg(session, srcentity, dstentity, context, pdu, msgbufdesc)
}
#[inline]
pub unsafe fn SnmpDeleteVb(vbl: isize, index: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDeleteVb(vbl : isize, index : u32) -> u32);
    SnmpDeleteVb(vbl, index)
}
#[inline]
pub unsafe fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDuplicatePdu(session : isize, pdu : isize) -> isize);
    SnmpDuplicatePdu(session, pdu)
}
#[inline]
pub unsafe fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDuplicateVbl(session : isize, vbl : isize) -> isize);
    SnmpDuplicateVbl(session, vbl)
}
#[inline]
pub unsafe fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpEncodeMsg(session : isize, srcentity : isize, dstentity : isize, context : isize, pdu : isize, msgbufdesc : *mut smiOCTETS) -> u32);
    SnmpEncodeMsg(session, srcentity, dstentity, context, pdu, msgbufdesc)
}
#[inline]
pub unsafe fn SnmpEntityToStr(entity: isize, string: &mut [u8]) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpEntityToStr(entity : isize, size : u32, string : windows_core::PSTR) -> u32);
    SnmpEntityToStr(entity, string.len().try_into().unwrap(), core::mem::transmute(string.as_ptr()))
}
#[inline]
pub unsafe fn SnmpFreeContext(context: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeContext(context : isize) -> u32);
    SnmpFreeContext(context)
}
#[inline]
pub unsafe fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeDescriptor(syntax : u32, descriptor : *mut smiOCTETS) -> u32);
    SnmpFreeDescriptor(syntax, descriptor)
}
#[inline]
pub unsafe fn SnmpFreeEntity(entity: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeEntity(entity : isize) -> u32);
    SnmpFreeEntity(entity)
}
#[inline]
pub unsafe fn SnmpFreePdu(pdu: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreePdu(pdu : isize) -> u32);
    SnmpFreePdu(pdu)
}
#[inline]
pub unsafe fn SnmpFreeVbl(vbl: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeVbl(vbl : isize) -> u32);
    SnmpFreeVbl(vbl)
}
#[inline]
pub unsafe fn SnmpGetLastError(session: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetLastError(session : isize) -> u32);
    SnmpGetLastError(session)
}
#[inline]
pub unsafe fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetPduData(pdu : isize, pdu_type : *mut SNMP_PDU_TYPE, request_id : *mut i32, error_status : *mut SNMP_ERROR, error_index : *mut i32, varbindlist : *mut isize) -> u32);
    SnmpGetPduData(pdu, pdu_type, request_id, error_status, error_index, varbindlist)
}
#[inline]
pub unsafe fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetRetransmitMode(nretransmitmode : *mut SNMP_STATUS) -> u32);
    SnmpGetRetransmitMode(nretransmitmode)
}
#[inline]
pub unsafe fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetRetry(hentity : isize, npolicyretry : *mut u32, nactualretry : *mut u32) -> u32);
    SnmpGetRetry(hentity, npolicyretry, nactualretry)
}
#[inline]
pub unsafe fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetTimeout(hentity : isize, npolicytimeout : *mut u32, nactualtimeout : *mut u32) -> u32);
    SnmpGetTimeout(hentity, npolicytimeout, nactualtimeout)
}
#[inline]
pub unsafe fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetTranslateMode(ntranslatemode : *mut SNMP_API_TRANSLATE_MODE) -> u32);
    SnmpGetTranslateMode(ntranslatemode)
}
#[inline]
pub unsafe fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetVb(vbl : isize, index : u32, name : *mut smiOID, value : *mut smiVALUE) -> u32);
    SnmpGetVb(vbl, index, name, value)
}
#[inline]
pub unsafe fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetVendorInfo(vendorinfo : *mut smiVENDORINFO) -> u32);
    SnmpGetVendorInfo(vendorinfo)
}
#[inline]
pub unsafe fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpListen(hentity : isize, lstatus : SNMP_STATUS) -> u32);
    SnmpListen(hentity, lstatus)
}
#[inline]
pub unsafe fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpListenEx(hentity : isize, lstatus : u32, nuseentityaddr : u32) -> u32);
    SnmpListenEx(hentity, lstatus, nuseentityaddr)
}
#[inline]
pub unsafe fn SnmpMgrClose(session: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrClose(session : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SnmpMgrClose(session)
}
#[inline]
pub unsafe fn SnmpMgrCtl(session: *mut core::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrCtl(session : *mut core::ffi::c_void, dwctlcode : u32, lpvinbuffer : *mut core::ffi::c_void, cbinbuffer : u32, lpvoutbuffer : *mut core::ffi::c_void, cboutbuffer : u32, lpcbbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    SnmpMgrCtl(session, dwctlcode, lpvinbuffer, cbinbuffer, lpvoutbuffer, cboutbuffer, lpcbbytesreturned).ok()
}
#[inline]
pub unsafe fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrGetTrap(enterprise : *mut AsnObjectIdentifier, ipaddress : *mut AsnOctetString, generictrap : *mut SNMP_GENERICTRAP, specifictrap : *mut i32, timestamp : *mut u32, variablebindings : *mut SnmpVarBindList) -> super::super::Foundation:: BOOL);
    SnmpMgrGetTrap(enterprise, ipaddress, generictrap, specifictrap, timestamp, variablebindings)
}
#[inline]
pub unsafe fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrGetTrapEx(enterprise : *mut AsnObjectIdentifier, agentaddress : *mut AsnOctetString, sourceaddress : *mut AsnOctetString, generictrap : *mut SNMP_GENERICTRAP, specifictrap : *mut i32, community : *mut AsnOctetString, timestamp : *mut u32, variablebindings : *mut SnmpVarBindList) -> super::super::Foundation:: BOOL);
    SnmpMgrGetTrapEx(enterprise, agentaddress, sourceaddress, generictrap, specifictrap, community, timestamp, variablebindings)
}
#[inline]
pub unsafe fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: Option<*mut windows_core::PSTR>) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrOidToStr(oid : *mut AsnObjectIdentifier, string : *mut windows_core::PSTR) -> super::super::Foundation:: BOOL);
    SnmpMgrOidToStr(oid, core::mem::transmute(string.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn SnmpMgrOpen<P0, P1>(lpagentaddress: P0, lpagentcommunity: P1, ntimeout: i32, nretries: i32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrOpen(lpagentaddress : windows_core::PCSTR, lpagentcommunity : windows_core::PCSTR, ntimeout : i32, nretries : i32) -> *mut core::ffi::c_void);
    SnmpMgrOpen(lpagentaddress.param().abi(), lpagentcommunity.param().abi(), ntimeout, nretries)
}
#[inline]
pub unsafe fn SnmpMgrRequest(session: *mut core::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32 {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrRequest(session : *mut core::ffi::c_void, requesttype : u8, variablebindings : *mut SnmpVarBindList, errorstatus : *mut SNMP_ERROR_STATUS, errorindex : *mut i32) -> i32);
    SnmpMgrRequest(session, requesttype, variablebindings, errorstatus, errorindex)
}
#[inline]
pub unsafe fn SnmpMgrStrToOid<P0>(string: P0, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrStrToOid(string : windows_core::PCSTR, oid : *mut AsnObjectIdentifier) -> super::super::Foundation:: BOOL);
    SnmpMgrStrToOid(string.param().abi(), oid)
}
#[inline]
pub unsafe fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrTrapListen(phtrapavailable : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SnmpMgrTrapListen(phtrapavailable).ok()
}
#[inline]
pub unsafe fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOidCompare(xoid : *mut smiOID, yoid : *mut smiOID, maxlen : u32, result : *mut i32) -> u32);
    SnmpOidCompare(xoid, yoid, maxlen, result)
}
#[inline]
pub unsafe fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOidCopy(srcoid : *mut smiOID, dstoid : *mut smiOID) -> u32);
    SnmpOidCopy(srcoid, dstoid)
}
#[inline]
pub unsafe fn SnmpOidToStr(srcoid: *const smiOID, string: &mut [u8]) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOidToStr(srcoid : *const smiOID, size : u32, string : windows_core::PSTR) -> u32);
    SnmpOidToStr(srcoid, string.len().try_into().unwrap(), core::mem::transmute(string.as_ptr()))
}
#[inline]
pub unsafe fn SnmpOpen<P0>(hwnd: P0, wmsg: u32) -> isize
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOpen(hwnd : super::super::Foundation:: HWND, wmsg : u32) -> isize);
    SnmpOpen(hwnd.param().abi(), wmsg)
}
#[inline]
pub unsafe fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpRecvMsg(session : isize, srcentity : *mut isize, dstentity : *mut isize, context : *mut isize, pdu : *mut isize) -> u32);
    SnmpRecvMsg(session, srcentity, dstentity, context, pdu)
}
#[inline]
pub unsafe fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpRegister(session : isize, srcentity : isize, dstentity : isize, context : isize, notification : *mut smiOID, state : SNMP_STATUS) -> u32);
    SnmpRegister(session, srcentity, dstentity, context, notification, state)
}
#[inline]
pub unsafe fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSendMsg(session : isize, srcentity : isize, dstentity : isize, context : isize, pdu : isize) -> u32);
    SnmpSendMsg(session, srcentity, dstentity, context, pdu)
}
#[inline]
pub unsafe fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetPduData(pdu : isize, pdu_type : *const i32, request_id : *const i32, non_repeaters : *const i32, max_repetitions : *const i32, varbindlist : *const isize) -> u32);
    SnmpSetPduData(pdu, pdu_type, request_id, non_repeaters, max_repetitions, varbindlist)
}
#[inline]
pub unsafe fn SnmpSetPort(hentity: isize, nport: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetPort(hentity : isize, nport : u32) -> u32);
    SnmpSetPort(hentity, nport)
}
#[inline]
pub unsafe fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetRetransmitMode(nretransmitmode : SNMP_STATUS) -> u32);
    SnmpSetRetransmitMode(nretransmitmode)
}
#[inline]
pub unsafe fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetRetry(hentity : isize, npolicyretry : u32) -> u32);
    SnmpSetRetry(hentity, npolicyretry)
}
#[inline]
pub unsafe fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetTimeout(hentity : isize, npolicytimeout : u32) -> u32);
    SnmpSetTimeout(hentity, npolicytimeout)
}
#[inline]
pub unsafe fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetTranslateMode(ntranslatemode : SNMP_API_TRANSLATE_MODE) -> u32);
    SnmpSetTranslateMode(ntranslatemode)
}
#[inline]
pub unsafe fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetVb(vbl : isize, index : u32, name : *mut smiOID, value : *mut smiVALUE) -> u32);
    SnmpSetVb(vbl, index, name, value)
}
#[inline]
pub unsafe fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStartup(nmajorversion : *mut u32, nminorversion : *mut u32, nlevel : *mut u32, ntranslatemode : *mut SNMP_API_TRANSLATE_MODE, nretransmitmode : *mut SNMP_STATUS) -> u32);
    SnmpStartup(nmajorversion, nminorversion, nlevel, ntranslatemode, nretransmitmode)
}
#[inline]
pub unsafe fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStartupEx(nmajorversion : *mut u32, nminorversion : *mut u32, nlevel : *mut u32, ntranslatemode : *mut SNMP_API_TRANSLATE_MODE, nretransmitmode : *mut SNMP_STATUS) -> u32);
    SnmpStartupEx(nmajorversion, nminorversion, nlevel, ntranslatemode, nretransmitmode)
}
#[inline]
pub unsafe fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStrToContext(session : isize, string : *mut smiOCTETS) -> isize);
    SnmpStrToContext(session, string)
}
#[inline]
pub unsafe fn SnmpStrToEntity<P0>(session: isize, string: P0) -> isize
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStrToEntity(session : isize, string : windows_core::PCSTR) -> isize);
    SnmpStrToEntity(session, string.param().abi())
}
#[inline]
pub unsafe fn SnmpStrToOid<P0>(string: P0, dstoid: *mut smiOID) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStrToOid(string : windows_core::PCSTR, dstoid : *mut smiOID) -> u32);
    SnmpStrToOid(string.param().abi(), dstoid)
}
#[inline]
pub unsafe fn SnmpSvcGetUptime() -> u32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpSvcGetUptime() -> u32);
    SnmpSvcGetUptime()
}
#[inline]
pub unsafe fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpSvcSetLogLevel(nloglevel : SNMP_LOG));
    SnmpSvcSetLogLevel(nloglevel)
}
#[inline]
pub unsafe fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpSvcSetLogType(nlogtype : i32));
    SnmpSvcSetLogType(nlogtype.0 as _)
}
#[inline]
pub unsafe fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilAsnAnyCpy(panydst : *mut AsnAny, panysrc : *mut AsnAny) -> i32);
    SnmpUtilAsnAnyCpy(panydst, panysrc)
}
#[inline]
pub unsafe fn SnmpUtilAsnAnyFree(pany: *mut AsnAny) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilAsnAnyFree(pany : *mut AsnAny));
    SnmpUtilAsnAnyFree(pany)
}
#[inline]
pub unsafe fn SnmpUtilDbgPrint<P0>(nloglevel: SNMP_LOG, szformat: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("snmpapi.dll" "cdecl" fn SnmpUtilDbgPrint(nloglevel : SNMP_LOG, szformat : windows_core::PCSTR));
    SnmpUtilDbgPrint(nloglevel, szformat.param().abi())
}
#[inline]
pub unsafe fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> windows_core::PSTR {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilIdsToA(ids : *mut u32, idlength : u32) -> windows_core::PSTR);
    SnmpUtilIdsToA(ids, idlength)
}
#[inline]
pub unsafe fn SnmpUtilMemAlloc(nbytes: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilMemAlloc(nbytes : u32) -> *mut core::ffi::c_void);
    SnmpUtilMemAlloc(nbytes)
}
#[inline]
pub unsafe fn SnmpUtilMemFree(pmem: *mut core::ffi::c_void) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilMemFree(pmem : *mut core::ffi::c_void));
    SnmpUtilMemFree(pmem)
}
#[inline]
pub unsafe fn SnmpUtilMemReAlloc(pmem: *mut core::ffi::c_void, nbytes: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilMemReAlloc(pmem : *mut core::ffi::c_void, nbytes : u32) -> *mut core::ffi::c_void);
    SnmpUtilMemReAlloc(pmem, nbytes)
}
#[inline]
pub unsafe fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsCmp(poctets1 : *mut AsnOctetString, poctets2 : *mut AsnOctetString) -> i32);
    SnmpUtilOctetsCmp(poctets1, poctets2)
}
#[inline]
pub unsafe fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsCpy(poctetsdst : *mut AsnOctetString, poctetssrc : *mut AsnOctetString) -> i32);
    SnmpUtilOctetsCpy(poctetsdst, poctetssrc)
}
#[inline]
pub unsafe fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsFree(poctets : *mut AsnOctetString));
    SnmpUtilOctetsFree(poctets)
}
#[inline]
pub unsafe fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsNCmp(poctets1 : *mut AsnOctetString, poctets2 : *mut AsnOctetString, nchars : u32) -> i32);
    SnmpUtilOctetsNCmp(poctets1, poctets2, nchars)
}
#[inline]
pub unsafe fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidAppend(poiddst : *mut AsnObjectIdentifier, poidsrc : *mut AsnObjectIdentifier) -> i32);
    SnmpUtilOidAppend(poiddst, poidsrc)
}
#[inline]
pub unsafe fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidCmp(poid1 : *mut AsnObjectIdentifier, poid2 : *mut AsnObjectIdentifier) -> i32);
    SnmpUtilOidCmp(poid1, poid2)
}
#[inline]
pub unsafe fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidCpy(poiddst : *mut AsnObjectIdentifier, poidsrc : *mut AsnObjectIdentifier) -> i32);
    SnmpUtilOidCpy(poiddst, poidsrc)
}
#[inline]
pub unsafe fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidFree(poid : *mut AsnObjectIdentifier));
    SnmpUtilOidFree(poid)
}
#[inline]
pub unsafe fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidNCmp(poid1 : *mut AsnObjectIdentifier, poid2 : *mut AsnObjectIdentifier, nsubids : u32) -> i32);
    SnmpUtilOidNCmp(poid1, poid2, nsubids)
}
#[inline]
pub unsafe fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> windows_core::PSTR {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidToA(oid : *mut AsnObjectIdentifier) -> windows_core::PSTR);
    SnmpUtilOidToA(oid)
}
#[inline]
pub unsafe fn SnmpUtilPrintAsnAny(pany: *mut AsnAny) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilPrintAsnAny(pany : *mut AsnAny));
    SnmpUtilPrintAsnAny(pany)
}
#[inline]
pub unsafe fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilPrintOid(oid : *mut AsnObjectIdentifier));
    SnmpUtilPrintOid(oid)
}
#[inline]
pub unsafe fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindCpy(pvbdst : *mut SnmpVarBind, pvbsrc : *mut SnmpVarBind) -> i32);
    SnmpUtilVarBindCpy(pvbdst, pvbsrc)
}
#[inline]
pub unsafe fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindFree(pvb : *mut SnmpVarBind));
    SnmpUtilVarBindFree(pvb)
}
#[inline]
pub unsafe fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindListCpy(pvbldst : *mut SnmpVarBindList, pvblsrc : *mut SnmpVarBindList) -> i32);
    SnmpUtilVarBindListCpy(pvbldst, pvblsrc)
}
#[inline]
pub unsafe fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindListFree(pvbl : *mut SnmpVarBindList));
    SnmpUtilVarBindListFree(pvbl)
}
pub const ASN_APPLICATION: u32 = 64u32;
pub const ASN_CONSTRUCTOR: u32 = 32u32;
pub const ASN_CONTEXT: u32 = 128u32;
pub const ASN_CONTEXTSPECIFIC: u32 = 128u32;
pub const ASN_PRIMATIVE: u32 = 0u32;
pub const ASN_PRIMITIVE: u32 = 0u32;
pub const ASN_PRIVATE: u32 = 192u32;
pub const ASN_UNIVERSAL: u32 = 0u32;
pub const DEFAULT_SNMPTRAP_PORT_IPX: u32 = 36880u32;
pub const DEFAULT_SNMPTRAP_PORT_UDP: u32 = 162u32;
pub const DEFAULT_SNMP_PORT_IPX: u32 = 36879u32;
pub const DEFAULT_SNMP_PORT_UDP: u32 = 161u32;
pub const MAXOBJIDSIZE: u32 = 128u32;
pub const MAXOBJIDSTRSIZE: u32 = 1408u32;
pub const MAXVENDORINFO: u32 = 32u32;
pub const MGMCTL_SETAGENTPORT: u32 = 1u32;
pub const SNMPAPI_ALLOC_ERROR: u32 = 2u32;
pub const SNMPAPI_CONTEXT_INVALID: u32 = 3u32;
pub const SNMPAPI_CONTEXT_UNKNOWN: u32 = 4u32;
pub const SNMPAPI_ENTITY_INVALID: u32 = 5u32;
pub const SNMPAPI_ENTITY_UNKNOWN: u32 = 6u32;
pub const SNMPAPI_ERROR: u32 = 0u32;
pub const SNMPAPI_FAILURE: u32 = 0u32;
pub const SNMPAPI_HWND_INVALID: u32 = 20u32;
pub const SNMPAPI_INDEX_INVALID: u32 = 7u32;
pub const SNMPAPI_M2M_SUPPORT: u32 = 3u32;
pub const SNMPAPI_MESSAGE_INVALID: u32 = 19u32;
pub const SNMPAPI_MODE_INVALID: u32 = 16u32;
pub const SNMPAPI_NOERROR: u32 = 1u32;
pub const SNMPAPI_NOOP: u32 = 8u32;
pub const SNMPAPI_NOT_INITIALIZED: u32 = 18u32;
pub const SNMPAPI_NO_SUPPORT: u32 = 0u32;
pub const SNMPAPI_OFF: SNMP_STATUS = SNMP_STATUS(0u32);
pub const SNMPAPI_OID_INVALID: u32 = 9u32;
pub const SNMPAPI_ON: SNMP_STATUS = SNMP_STATUS(1u32);
pub const SNMPAPI_OPERATION_INVALID: u32 = 10u32;
pub const SNMPAPI_OTHER_ERROR: u32 = 99u32;
pub const SNMPAPI_OUTPUT_TRUNCATED: u32 = 11u32;
pub const SNMPAPI_PDU_INVALID: u32 = 12u32;
pub const SNMPAPI_SESSION_INVALID: u32 = 13u32;
pub const SNMPAPI_SIZE_INVALID: u32 = 17u32;
pub const SNMPAPI_SUCCESS: u32 = 1u32;
pub const SNMPAPI_SYNTAX_INVALID: u32 = 14u32;
pub const SNMPAPI_TL_INVALID_PARAM: u32 = 106u32;
pub const SNMPAPI_TL_IN_USE: u32 = 107u32;
pub const SNMPAPI_TL_NOT_AVAILABLE: u32 = 102u32;
pub const SNMPAPI_TL_NOT_INITIALIZED: u32 = 100u32;
pub const SNMPAPI_TL_NOT_SUPPORTED: u32 = 101u32;
pub const SNMPAPI_TL_OTHER: u32 = 199u32;
pub const SNMPAPI_TL_PDU_TOO_BIG: u32 = 109u32;
pub const SNMPAPI_TL_RESOURCE_ERROR: u32 = 103u32;
pub const SNMPAPI_TL_SRC_INVALID: u32 = 105u32;
pub const SNMPAPI_TL_TIMEOUT: u32 = 108u32;
pub const SNMPAPI_TL_UNDELIVERABLE: u32 = 104u32;
pub const SNMPAPI_TRANSLATED: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(0u32);
pub const SNMPAPI_UNTRANSLATED_V1: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(1u32);
pub const SNMPAPI_UNTRANSLATED_V2: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(2u32);
pub const SNMPAPI_V1_SUPPORT: u32 = 1u32;
pub const SNMPAPI_V2_SUPPORT: u32 = 2u32;
pub const SNMPAPI_VBL_INVALID: u32 = 15u32;
pub const SNMPLISTEN_ALL_ADDR: u32 = 1u32;
pub const SNMPLISTEN_USEENTITY_ADDR: u32 = 0u32;
pub const SNMP_ACCESS_NONE: u32 = 0u32;
pub const SNMP_ACCESS_NOTIFY: u32 = 1u32;
pub const SNMP_ACCESS_READ_CREATE: u32 = 4u32;
pub const SNMP_ACCESS_READ_ONLY: u32 = 2u32;
pub const SNMP_ACCESS_READ_WRITE: u32 = 3u32;
pub const SNMP_AUTHAPI_INVALID_MSG_TYPE: u32 = 31u32;
pub const SNMP_AUTHAPI_INVALID_VERSION: u32 = 30u32;
pub const SNMP_AUTHAPI_TRIV_AUTH_FAILED: u32 = 32u32;
pub const SNMP_BERAPI_INVALID_LENGTH: u32 = 10u32;
pub const SNMP_BERAPI_INVALID_OBJELEM: u32 = 14u32;
pub const SNMP_BERAPI_INVALID_TAG: u32 = 11u32;
pub const SNMP_BERAPI_OVERFLOW: u32 = 12u32;
pub const SNMP_BERAPI_SHORT_BUFFER: u32 = 13u32;
pub const SNMP_ERRORSTATUS_AUTHORIZATIONERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(16u32);
pub const SNMP_ERRORSTATUS_BADVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(3u32);
pub const SNMP_ERRORSTATUS_COMMITFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(14u32);
pub const SNMP_ERRORSTATUS_GENERR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(5u32);
pub const SNMP_ERRORSTATUS_INCONSISTENTNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(18u32);
pub const SNMP_ERRORSTATUS_INCONSISTENTVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(12u32);
pub const SNMP_ERRORSTATUS_NOACCESS: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(6u32);
pub const SNMP_ERRORSTATUS_NOCREATION: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(11u32);
pub const SNMP_ERRORSTATUS_NOERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(0u32);
pub const SNMP_ERRORSTATUS_NOSUCHNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(2u32);
pub const SNMP_ERRORSTATUS_NOTWRITABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(17u32);
pub const SNMP_ERRORSTATUS_READONLY: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(4u32);
pub const SNMP_ERRORSTATUS_RESOURCEUNAVAILABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(13u32);
pub const SNMP_ERRORSTATUS_TOOBIG: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(1u32);
pub const SNMP_ERRORSTATUS_UNDOFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(15u32);
pub const SNMP_ERRORSTATUS_WRONGENCODING: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(9u32);
pub const SNMP_ERRORSTATUS_WRONGLENGTH: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(8u32);
pub const SNMP_ERRORSTATUS_WRONGTYPE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(7u32);
pub const SNMP_ERRORSTATUS_WRONGVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(10u32);
pub const SNMP_ERROR_AUTHORIZATIONERROR: SNMP_ERROR = SNMP_ERROR(16u32);
pub const SNMP_ERROR_BADVALUE: SNMP_ERROR = SNMP_ERROR(3u32);
pub const SNMP_ERROR_COMMITFAILED: SNMP_ERROR = SNMP_ERROR(14u32);
pub const SNMP_ERROR_GENERR: SNMP_ERROR = SNMP_ERROR(5u32);
pub const SNMP_ERROR_INCONSISTENTNAME: SNMP_ERROR = SNMP_ERROR(18u32);
pub const SNMP_ERROR_INCONSISTENTVALUE: SNMP_ERROR = SNMP_ERROR(12u32);
pub const SNMP_ERROR_NOACCESS: SNMP_ERROR = SNMP_ERROR(6u32);
pub const SNMP_ERROR_NOCREATION: SNMP_ERROR = SNMP_ERROR(11u32);
pub const SNMP_ERROR_NOERROR: SNMP_ERROR = SNMP_ERROR(0u32);
pub const SNMP_ERROR_NOSUCHNAME: SNMP_ERROR = SNMP_ERROR(2u32);
pub const SNMP_ERROR_NOTWRITABLE: SNMP_ERROR = SNMP_ERROR(17u32);
pub const SNMP_ERROR_READONLY: SNMP_ERROR = SNMP_ERROR(4u32);
pub const SNMP_ERROR_RESOURCEUNAVAILABLE: SNMP_ERROR = SNMP_ERROR(13u32);
pub const SNMP_ERROR_TOOBIG: SNMP_ERROR = SNMP_ERROR(1u32);
pub const SNMP_ERROR_UNDOFAILED: SNMP_ERROR = SNMP_ERROR(15u32);
pub const SNMP_ERROR_WRONGENCODING: SNMP_ERROR = SNMP_ERROR(9u32);
pub const SNMP_ERROR_WRONGLENGTH: SNMP_ERROR = SNMP_ERROR(8u32);
pub const SNMP_ERROR_WRONGTYPE: SNMP_ERROR = SNMP_ERROR(7u32);
pub const SNMP_ERROR_WRONGVALUE: SNMP_ERROR = SNMP_ERROR(10u32);
pub const SNMP_EXTENSION_GET: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(160u32);
pub const SNMP_EXTENSION_GET_NEXT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(161u32);
pub const SNMP_EXTENSION_SET_CLEANUP: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(226u32);
pub const SNMP_EXTENSION_SET_COMMIT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(163u32);
pub const SNMP_EXTENSION_SET_TEST: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(224u32);
pub const SNMP_EXTENSION_SET_UNDO: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(225u32);
pub const SNMP_GENERICTRAP_AUTHFAILURE: SNMP_GENERICTRAP = SNMP_GENERICTRAP(4u32);
pub const SNMP_GENERICTRAP_COLDSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(0u32);
pub const SNMP_GENERICTRAP_EGPNEIGHLOSS: SNMP_GENERICTRAP = SNMP_GENERICTRAP(5u32);
pub const SNMP_GENERICTRAP_ENTERSPECIFIC: SNMP_GENERICTRAP = SNMP_GENERICTRAP(6u32);
pub const SNMP_GENERICTRAP_LINKDOWN: SNMP_GENERICTRAP = SNMP_GENERICTRAP(2u32);
pub const SNMP_GENERICTRAP_LINKUP: SNMP_GENERICTRAP = SNMP_GENERICTRAP(3u32);
pub const SNMP_GENERICTRAP_WARMSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(1u32);
pub const SNMP_LOG_ERROR: SNMP_LOG = SNMP_LOG(2i32);
pub const SNMP_LOG_FATAL: SNMP_LOG = SNMP_LOG(1i32);
pub const SNMP_LOG_SILENT: SNMP_LOG = SNMP_LOG(0i32);
pub const SNMP_LOG_TRACE: SNMP_LOG = SNMP_LOG(4i32);
pub const SNMP_LOG_VERBOSE: SNMP_LOG = SNMP_LOG(5i32);
pub const SNMP_LOG_WARNING: SNMP_LOG = SNMP_LOG(3i32);
pub const SNMP_MAX_OID_LEN: u32 = 128u32;
pub const SNMP_MEM_ALLOC_ERROR: u32 = 1u32;
pub const SNMP_MGMTAPI_AGAIN: u32 = 45u32;
pub const SNMP_MGMTAPI_INVALID_BUFFER: u32 = 48u32;
pub const SNMP_MGMTAPI_INVALID_CTL: u32 = 46u32;
pub const SNMP_MGMTAPI_INVALID_SESSION: u32 = 47u32;
pub const SNMP_MGMTAPI_NOTRAPS: u32 = 44u32;
pub const SNMP_MGMTAPI_SELECT_FDERRORS: u32 = 41u32;
pub const SNMP_MGMTAPI_TIMEOUT: u32 = 40u32;
pub const SNMP_MGMTAPI_TRAP_DUPINIT: u32 = 43u32;
pub const SNMP_MGMTAPI_TRAP_ERRORS: u32 = 42u32;
pub const SNMP_OUTPUT_TO_CONSOLE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(1u32);
pub const SNMP_OUTPUT_TO_DEBUGGER: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(8u32);
pub const SNMP_OUTPUT_TO_EVENTLOG: u32 = 4u32;
pub const SNMP_OUTPUT_TO_LOGFILE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(2u32);
pub const SNMP_PDUAPI_INVALID_ES: u32 = 21u32;
pub const SNMP_PDUAPI_INVALID_GT: u32 = 22u32;
pub const SNMP_PDUAPI_UNRECOGNIZED_PDU: u32 = 20u32;
pub const SNMP_PDU_GET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(160u32);
pub const SNMP_PDU_GETBULK: SNMP_PDU_TYPE = SNMP_PDU_TYPE(165u32);
pub const SNMP_PDU_GETNEXT: SNMP_PDU_TYPE = SNMP_PDU_TYPE(161u32);
pub const SNMP_PDU_RESPONSE: SNMP_PDU_TYPE = SNMP_PDU_TYPE(162u32);
pub const SNMP_PDU_SET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(163u32);
pub const SNMP_PDU_TRAP: SNMP_PDU_TYPE = SNMP_PDU_TYPE(167u32);
pub const SNMP_TRAP_AUTHFAIL: u32 = 4u32;
pub const SNMP_TRAP_COLDSTART: u32 = 0u32;
pub const SNMP_TRAP_EGPNEIGHBORLOSS: u32 = 5u32;
pub const SNMP_TRAP_ENTERPRISESPECIFIC: u32 = 6u32;
pub const SNMP_TRAP_LINKDOWN: u32 = 2u32;
pub const SNMP_TRAP_LINKUP: u32 = 3u32;
pub const SNMP_TRAP_WARMSTART: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_API_TRANSLATE_MODE(pub u32);
impl windows_core::TypeKind for SNMP_API_TRANSLATE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_API_TRANSLATE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_API_TRANSLATE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_ERROR(pub u32);
impl windows_core::TypeKind for SNMP_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_ERROR_STATUS(pub u32);
impl windows_core::TypeKind for SNMP_ERROR_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_ERROR_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_ERROR_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_EXTENSION_REQUEST_TYPE(pub u32);
impl windows_core::TypeKind for SNMP_EXTENSION_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_EXTENSION_REQUEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_EXTENSION_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_GENERICTRAP(pub u32);
impl windows_core::TypeKind for SNMP_GENERICTRAP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_GENERICTRAP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_GENERICTRAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_LOG(pub i32);
impl windows_core::TypeKind for SNMP_LOG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_LOG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_LOG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_OUTPUT_LOG_TYPE(pub u32);
impl windows_core::TypeKind for SNMP_OUTPUT_LOG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_OUTPUT_LOG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_OUTPUT_LOG_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_PDU_TYPE(pub u32);
impl windows_core::TypeKind for SNMP_PDU_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_PDU_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_PDU_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SNMP_STATUS(pub u32);
impl windows_core::TypeKind for SNMP_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SNMP_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SNMP_STATUS").field(&self.0).finish()
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct AsnAny {
    pub asnType: u8,
    pub asnValue: AsnAny_0,
}
impl windows_core::TypeKind for AsnAny {
    type TypeKind = windows_core::CopyType;
}
impl Default for AsnAny {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union AsnAny_0 {
    pub number: i32,
    pub unsigned32: u32,
    pub counter64: u64,
    pub string: AsnOctetString,
    pub bits: AsnOctetString,
    pub object: AsnObjectIdentifier,
    pub sequence: AsnOctetString,
    pub address: AsnOctetString,
    pub counter: u32,
    pub gauge: u32,
    pub ticks: u32,
    pub arbitrary: AsnOctetString,
}
impl windows_core::TypeKind for AsnAny_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AsnAny_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for AsnObjectIdentifier {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for AsnObjectIdentifier {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for AsnOctetString {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for AsnOctetString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for AsnOctetString {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for AsnOctetString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct SnmpVarBind {
    pub name: AsnObjectIdentifier,
    pub value: AsnAny,
}
impl windows_core::TypeKind for SnmpVarBind {
    type TypeKind = windows_core::CopyType;
}
impl Default for SnmpVarBind {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SnmpVarBindList {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SnmpVarBindList {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct smiCNTR64 {
    pub hipart: u32,
    pub lopart: u32,
}
impl windows_core::TypeKind for smiCNTR64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for smiCNTR64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct smiOCTETS {
    pub len: u32,
    pub ptr: *mut u8,
}
impl windows_core::TypeKind for smiOCTETS {
    type TypeKind = windows_core::CopyType;
}
impl Default for smiOCTETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct smiOID {
    pub len: u32,
    pub ptr: *mut u32,
}
impl windows_core::TypeKind for smiOID {
    type TypeKind = windows_core::CopyType;
}
impl Default for smiOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct smiVALUE {
    pub syntax: u32,
    pub value: smiVALUE_0,
}
impl windows_core::TypeKind for smiVALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for smiVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union smiVALUE_0 {
    pub sNumber: i32,
    pub uNumber: u32,
    pub hNumber: smiCNTR64,
    pub string: smiOCTETS,
    pub oid: smiOID,
    pub empty: u8,
}
impl windows_core::TypeKind for smiVALUE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for smiVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct smiVENDORINFO {
    pub vendorName: [i8; 64],
    pub vendorContact: [i8; 64],
    pub vendorVersionId: [i8; 32],
    pub vendorVersionDate: [i8; 32],
    pub vendorEnterprise: u32,
}
impl windows_core::TypeKind for smiVENDORINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for smiVENDORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFNSNMPCLEANUPEX = Option<unsafe extern "system" fn() -> u32>;
pub type PFNSNMPEXTENSIONCLOSE = Option<unsafe extern "system" fn()>;
pub type PFNSNMPEXTENSIONINIT = Option<unsafe extern "system" fn(dwuptimereference: u32, phsubagenttrapevent: *mut super::super::Foundation::HANDLE, pfirstsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONINITEX = Option<unsafe extern "system" fn(pnextsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONMONITOR = Option<unsafe extern "system" fn(pagentmgmtdata: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONQUERY = Option<unsafe extern "system" fn(bpdutype: u8, pvarbindlist: *mut SnmpVarBindList, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONQUERYEX = Option<unsafe extern "system" fn(nrequesttype: u32, ntransactionid: u32, pvarbindlist: *mut SnmpVarBindList, pcontextinfo: *mut AsnOctetString, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONTRAP = Option<unsafe extern "system" fn(penterpriseoid: *mut AsnObjectIdentifier, pgenerictrapid: *mut i32, pspecifictrapid: *mut i32, ptimestamp: *mut u32, pvarbindlist: *mut SnmpVarBindList) -> super::super::Foundation::BOOL>;
pub type PFNSNMPSTARTUPEX = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u32, param2: *mut u32, param3: *mut u32, param4: *mut u32) -> u32>;
pub type SNMPAPI_CALLBACK = Option<unsafe extern "system" fn(hsession: isize, hwnd: super::super::Foundation::HWND, wmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpclientdata: *mut core::ffi::c_void) -> u32>;
