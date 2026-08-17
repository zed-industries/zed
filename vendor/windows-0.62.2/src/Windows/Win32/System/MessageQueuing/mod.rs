#[inline]
pub unsafe fn MQADsPathToFormatName<P0>(lpwcsadspath: P0, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQADsPathToFormatName(lpwcsadspath : windows_core::PCWSTR, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQADsPathToFormatName(lpwcsadspath.param().abi(), core::mem::transmute(lpwcsformatname), lpdwformatnamelength as _).ok() }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
#[inline]
pub unsafe fn MQBeginTransaction() -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction> {
    windows_core::link!("mqrt.dll" "system" fn MQBeginTransaction(pptransaction : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQBeginTransaction(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MQCloseCursor(hcursor: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQCloseCursor(hcursor : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { MQCloseCursor(hcursor).ok() }
}
#[inline]
pub unsafe fn MQCloseQueue(hqueue: isize) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQCloseQueue(hqueue : isize) -> windows_core::HRESULT);
    unsafe { MQCloseQueue(hqueue).ok() }
}
#[inline]
pub unsafe fn MQCreateCursor(hqueue: isize) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("mqrt.dll" "system" fn MQCreateCursor(hqueue : isize, phcursor : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQCreateCursor(hqueue, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQCreateQueue(psecuritydescriptor: Option<super::super::Security::PSECURITY_DESCRIPTOR>, pqueueprops: *mut MQQUEUEPROPS, lpwcsformatname: Option<windows_core::PWSTR>, lpdwformatnamelength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQCreateQueue(psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, pqueueprops : *mut MQQUEUEPROPS, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQCreateQueue(psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, pqueueprops as _, lpwcsformatname.unwrap_or(core::mem::zeroed()) as _, lpdwformatnamelength as _).ok() }
}
#[inline]
pub unsafe fn MQDeleteQueue<P0>(lpwcsformatname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQDeleteQueue(lpwcsformatname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { MQDeleteQueue(lpwcsformatname.param().abi()).ok() }
}
#[inline]
pub unsafe fn MQFreeMemory(pvmemory: *const core::ffi::c_void) {
    windows_core::link!("mqrt.dll" "system" fn MQFreeMemory(pvmemory : *const core::ffi::c_void));
    unsafe { MQFreeMemory(pvmemory) }
}
#[inline]
pub unsafe fn MQFreeSecurityContext(hsecuritycontext: super::super::Foundation::HANDLE) {
    windows_core::link!("mqrt.dll" "system" fn MQFreeSecurityContext(hsecuritycontext : super::super::Foundation:: HANDLE));
    unsafe { MQFreeSecurityContext(hsecuritycontext) }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQGetMachineProperties<P0>(lpwcsmachinename: P0, pguidmachineid: Option<*const windows_core::GUID>, pqmprops: *mut MQQMPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetMachineProperties(lpwcsmachinename : windows_core::PCWSTR, pguidmachineid : *const windows_core::GUID, pqmprops : *mut MQQMPROPS) -> windows_core::HRESULT);
    unsafe { MQGetMachineProperties(lpwcsmachinename.param().abi(), pguidmachineid.unwrap_or(core::mem::zeroed()) as _, pqmprops as _).ok() }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn MQGetOverlappedResult(lpoverlapped: *const super::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQGetOverlappedResult(lpoverlapped : *const super::IO:: OVERLAPPED) -> windows_core::HRESULT);
    unsafe { MQGetOverlappedResult(lpoverlapped).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQGetPrivateComputerInformation<P0>(lpwcscomputername: P0, pprivateprops: *mut MQPRIVATEPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetPrivateComputerInformation(lpwcscomputername : windows_core::PCWSTR, pprivateprops : *mut MQPRIVATEPROPS) -> windows_core::HRESULT);
    unsafe { MQGetPrivateComputerInformation(lpwcscomputername.param().abi(), pprivateprops as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQGetQueueProperties<P0>(lpwcsformatname: P0, pqueueprops: *mut MQQUEUEPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetQueueProperties(lpwcsformatname : windows_core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_core::HRESULT);
    unsafe { MQGetQueueProperties(lpwcsformatname.param().abi(), pqueueprops as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn MQGetQueueSecurity<P0>(lpwcsformatname: P0, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetQueueSecurity(lpwcsformatname : windows_core::PCWSTR, requestedinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::HRESULT);
    unsafe { MQGetQueueSecurity(lpwcsformatname.param().abi(), requestedinformation, psecuritydescriptor as _, nlength, lpnlengthneeded as _).ok() }
}
#[inline]
pub unsafe fn MQGetSecurityContext(lpcertbuffer: Option<*const core::ffi::c_void>, dwcertbufferlength: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("mqrt.dll" "system" fn MQGetSecurityContext(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQGetSecurityContext(lpcertbuffer.unwrap_or(core::mem::zeroed()) as _, dwcertbufferlength, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MQGetSecurityContextEx(lpcertbuffer: Option<*const core::ffi::c_void>, dwcertbufferlength: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("mqrt.dll" "system" fn MQGetSecurityContextEx(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQGetSecurityContextEx(lpcertbuffer.unwrap_or(core::mem::zeroed()) as _, dwcertbufferlength, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MQHandleToFormatName(hqueue: isize, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQHandleToFormatName(hqueue : isize, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQHandleToFormatName(hqueue, core::mem::transmute(lpwcsformatname), lpdwformatnamelength as _).ok() }
}
#[inline]
pub unsafe fn MQInstanceToFormatName(pguid: *const windows_core::GUID, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQInstanceToFormatName(pguid : *const windows_core::GUID, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQInstanceToFormatName(pguid, core::mem::transmute(lpwcsformatname), lpdwformatnamelength as _).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQLocateBegin<P0>(lpwcscontext: P0, prestriction: Option<*const MQRESTRICTION>, pcolumns: *const MQCOLUMNSET, psort: *const MQSORTSET) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQLocateBegin(lpwcscontext : windows_core::PCWSTR, prestriction : *const MQRESTRICTION, pcolumns : *const MQCOLUMNSET, psort : *const MQSORTSET, phenum : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQLocateBegin(lpwcscontext.param().abi(), prestriction.unwrap_or(core::mem::zeroed()) as _, pcolumns, psort, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MQLocateEnd(henum: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQLocateEnd(henum : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { MQLocateEnd(henum).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQLocateNext(henum: super::super::Foundation::HANDLE, pcprops: *mut u32, apropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQLocateNext(henum : super::super::Foundation:: HANDLE, pcprops : *mut u32, apropvar : *mut super::Com::StructuredStorage:: PROPVARIANT) -> windows_core::HRESULT);
    unsafe { MQLocateNext(henum, pcprops as _, core::mem::transmute(apropvar)).ok() }
}
#[inline]
pub unsafe fn MQMarkMessageRejected(hqueue: super::super::Foundation::HANDLE, ulllookupid: u64) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQMarkMessageRejected(hqueue : super::super::Foundation:: HANDLE, ulllookupid : u64) -> windows_core::HRESULT);
    unsafe { MQMarkMessageRejected(hqueue, ulllookupid).ok() }
}
#[inline]
pub unsafe fn MQMgmtAction<P0, P1, P2>(pcomputername: P0, pobjectname: P1, paction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQMgmtAction(pcomputername : windows_core::PCWSTR, pobjectname : windows_core::PCWSTR, paction : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { MQMgmtAction(pcomputername.param().abi(), pobjectname.param().abi(), paction.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQMgmtGetInfo<P0, P1>(pcomputername: P0, pobjectname: P1, pmgmtprops: *mut MQMGMTPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQMgmtGetInfo(pcomputername : windows_core::PCWSTR, pobjectname : windows_core::PCWSTR, pmgmtprops : *mut MQMGMTPROPS) -> windows_core::HRESULT);
    unsafe { MQMgmtGetInfo(pcomputername.param().abi(), pobjectname.param().abi(), pmgmtprops as _).ok() }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
#[inline]
pub unsafe fn MQMoveMessage<P3>(hsourcequeue: isize, hdestinationqueue: isize, ulllookupid: u64, ptransaction: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQMoveMessage(hsourcequeue : isize, hdestinationqueue : isize, ulllookupid : u64, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQMoveMessage(hsourcequeue, hdestinationqueue, ulllookupid, ptransaction.param().abi()).ok() }
}
#[inline]
pub unsafe fn MQOpenQueue<P0>(lpwcsformatname: P0, dwaccess: u32, dwsharemode: u32) -> windows_core::Result<isize>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQOpenQueue(lpwcsformatname : windows_core::PCWSTR, dwaccess : u32, dwsharemode : u32, phqueue : *mut isize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQOpenQueue(lpwcsformatname.param().abi(), dwaccess, dwsharemode, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MQPathNameToFormatName<P0>(lpwcspathname: P0, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQPathNameToFormatName(lpwcspathname : windows_core::PCWSTR, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQPathNameToFormatName(lpwcspathname.param().abi(), core::mem::transmute(lpwcsformatname), lpdwformatnamelength as _).ok() }
}
#[inline]
pub unsafe fn MQPurgeQueue(hqueue: isize) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQPurgeQueue(hqueue : isize) -> windows_core::HRESULT);
    unsafe { MQPurgeQueue(hqueue).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_IO", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQReceiveMessage<P7>(hsource: isize, dwtimeout: u32, dwaction: u32, pmessageprops: Option<*mut MQMSGPROPS>, lpoverlapped: Option<*mut super::IO::OVERLAPPED>, fnreceivecallback: PMQRECEIVECALLBACK, hcursor: Option<super::super::Foundation::HANDLE>, ptransaction: P7) -> windows_core::Result<()>
where
    P7: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQReceiveMessage(hsource : isize, dwtimeout : u32, dwaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::IO:: OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, hcursor : super::super::Foundation:: HANDLE, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQReceiveMessage(hsource, dwtimeout, dwaction, pmessageprops.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, fnreceivecallback, hcursor.unwrap_or(core::mem::zeroed()) as _, ptransaction.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_IO", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQReceiveMessageByLookupId<P6>(hsource: isize, ulllookupid: u64, dwlookupaction: u32, pmessageprops: Option<*mut MQMSGPROPS>, lpoverlapped: Option<*mut super::IO::OVERLAPPED>, fnreceivecallback: PMQRECEIVECALLBACK, ptransaction: P6) -> windows_core::Result<()>
where
    P6: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQReceiveMessageByLookupId(hsource : isize, ulllookupid : u64, dwlookupaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::IO:: OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQReceiveMessageByLookupId(hsource, ulllookupid, dwlookupaction, pmessageprops.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, fnreceivecallback, ptransaction.param().abi()).ok() }
}
#[inline]
pub unsafe fn MQRegisterCertificate(dwflags: u32, lpcertbuffer: *const core::ffi::c_void, dwcertbufferlength: u32) -> windows_core::Result<()> {
    windows_core::link!("mqrt.dll" "system" fn MQRegisterCertificate(dwflags : u32, lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32) -> windows_core::HRESULT);
    unsafe { MQRegisterCertificate(dwflags, lpcertbuffer, dwcertbufferlength).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQSendMessage<P2>(hdestinationqueue: isize, pmessageprops: *const MQMSGPROPS, ptransaction: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQSendMessage(hdestinationqueue : isize, pmessageprops : *const MQMSGPROPS, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQSendMessage(hdestinationqueue, pmessageprops, ptransaction.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn MQSetQueueProperties<P0>(lpwcsformatname: P0, pqueueprops: *mut MQQUEUEPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQSetQueueProperties(lpwcsformatname : windows_core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_core::HRESULT);
    unsafe { MQSetQueueProperties(lpwcsformatname.param().abi(), pqueueprops as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn MQSetQueueSecurity<P0>(lpwcsformatname: P0, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: Option<super::super::Security::PSECURITY_DESCRIPTOR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQSetQueueSecurity(lpwcsformatname : windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_core::HRESULT);
    unsafe { MQSetQueueSecurity(lpwcsformatname.param().abi(), securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _).ok() }
}
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_DELIVERY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_PRIORITY: MQDEFAULT = MQDEFAULT(3i32);
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_Q_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = MQDEFAULT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FOREIGN_STATUS(pub i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQApplication, IMSMQApplication_Vtbl, 0xd7d6e085_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQApplication {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQApplication, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication {
    pub unsafe fn MachineIdOfMachineName(&self, machinename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MachineIdOfMachineName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(machinename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub MachineIdOfMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQApplication_Impl: super::Com::IDispatch_Impl {
    fn MachineIdOfMachineName(&self, machinename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQApplication_Vtbl {
    pub const fn new<Identity: IMSMQApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MachineIdOfMachineName<Identity: IMSMQApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinename: *mut core::ffi::c_void, pbstrguid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication_Impl::MachineIdOfMachineName(this, core::mem::transmute(&machinename)) {
                    Ok(ok__) => {
                        pbstrguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), MachineIdOfMachineName: MachineIdOfMachineName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQApplication as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQApplication {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQApplication2, IMSMQApplication2_Vtbl, 0x12a30900_7300_11d2_b0e6_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQApplication2 {
    type Target = IMSMQApplication;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQApplication2, windows_core::IUnknown, super::Com::IDispatch, IMSMQApplication);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterCertificate(&self, flags: *const super::Variant::VARIANT, externalcertificate: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute(flags), core::mem::transmute(externalcertificate)).ok() }
    }
    pub unsafe fn MachineNameOfMachineId(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MachineNameOfMachineId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MSMQVersionMajor(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MSMQVersionMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MSMQVersionMinor(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MSMQVersionMinor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MSMQVersionBuild(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MSMQVersionBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication2_Vtbl {
    pub base__: IMSMQApplication_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RegisterCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RegisterCertificate: usize,
    pub MachineNameOfMachineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MSMQVersionMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub MSMQVersionMinor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub MSMQVersionBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsDsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQApplication2_Impl: IMSMQApplication_Impl {
    fn RegisterCertificate(&self, flags: *const super::Variant::VARIANT, externalcertificate: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn MachineNameOfMachineId(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn MSMQVersionMajor(&self) -> windows_core::Result<i16>;
    fn MSMQVersionMinor(&self) -> windows_core::Result<i16>;
    fn MSMQVersionBuild(&self) -> windows_core::Result<i16>;
    fn IsDsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQApplication2_Vtbl {
    pub const fn new<Identity: IMSMQApplication2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterCertificate<Identity: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *const super::Variant::VARIANT, externalcertificate: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQApplication2_Impl::RegisterCertificate(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&externalcertificate)).into()
            }
        }
        unsafe extern "system" fn MachineNameOfMachineId<Identity: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: *mut core::ffi::c_void, pbstrmachinename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication2_Impl::MachineNameOfMachineId(this, core::mem::transmute(&bstrguid)) {
                    Ok(ok__) => {
                        pbstrmachinename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MSMQVersionMajor<Identity: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psmsmqversionmajor: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication2_Impl::MSMQVersionMajor(this) {
                    Ok(ok__) => {
                        psmsmqversionmajor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MSMQVersionMinor<Identity: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psmsmqversionminor: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication2_Impl::MSMQVersionMinor(this) {
                    Ok(ok__) => {
                        psmsmqversionminor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MSMQVersionBuild<Identity: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psmsmqversionbuild: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication2_Impl::MSMQVersionBuild(this) {
                    Ok(ok__) => {
                        psmsmqversionbuild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDsEnabled<Identity: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisdsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication2_Impl::IsDsEnabled(this) {
                    Ok(ok__) => {
                        pfisdsenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMSMQApplication_Vtbl::new::<Identity, OFFSET>(),
            RegisterCertificate: RegisterCertificate::<Identity, OFFSET>,
            MachineNameOfMachineId: MachineNameOfMachineId::<Identity, OFFSET>,
            MSMQVersionMajor: MSMQVersionMajor::<Identity, OFFSET>,
            MSMQVersionMinor: MSMQVersionMinor::<Identity, OFFSET>,
            MSMQVersionBuild: MSMQVersionBuild::<Identity, OFFSET>,
            IsDsEnabled: IsDsEnabled::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQApplication2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQApplication as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQApplication2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQApplication3, IMSMQApplication3_Vtbl, 0xeba96b1f_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQApplication3 {
    type Target = IMSMQApplication2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQApplication3, windows_core::IUnknown, super::Com::IDispatch, IMSMQApplication, IMSMQApplication2);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication3 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ActiveQueues(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActiveQueues)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PrivateQueues(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateQueues)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DirectoryServiceServer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DirectoryServiceServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn BytesInAllQueues(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BytesInAllQueues)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMachine(&self, bstrmachine: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMachine)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmachine)).ok() }
    }
    pub unsafe fn Machine(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Machine)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Tidy(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Tidy)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication3_Vtbl {
    pub base__: IMSMQApplication2_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ActiveQueues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ActiveQueues: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PrivateQueues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PrivateQueues: usize,
    pub DirectoryServiceServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub BytesInAllQueues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    BytesInAllQueues: usize,
    pub SetMachine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Machine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tidy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQApplication3_Impl: IMSMQApplication2_Impl {
    fn ActiveQueues(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn PrivateQueues(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn DirectoryServiceServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BytesInAllQueues(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetMachine(&self, bstrmachine: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Machine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Tidy(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQApplication3_Vtbl {
    pub const fn new<Identity: IMSMQApplication3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActiveQueues<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvactivequeues: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication3_Impl::ActiveQueues(this) {
                    Ok(ok__) => {
                        pvactivequeues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateQueues<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvprivatequeues: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication3_Impl::PrivateQueues(this) {
                    Ok(ok__) => {
                        pvprivatequeues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DirectoryServiceServer<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdirectoryserviceserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication3_Impl::DirectoryServiceServer(this) {
                    Ok(ok__) => {
                        pbstrdirectoryserviceserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication3_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        pfisconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BytesInAllQueues<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbytesinallqueues: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication3_Impl::BytesInAllQueues(this) {
                    Ok(ok__) => {
                        pvbytesinallqueues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMachine<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmachine: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQApplication3_Impl::SetMachine(this, core::mem::transmute(&bstrmachine)).into()
            }
        }
        unsafe extern "system" fn Machine<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmachine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQApplication3_Impl::Machine(this) {
                    Ok(ok__) => {
                        pbstrmachine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQApplication3_Impl::Connect(this).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQApplication3_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Tidy<Identity: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQApplication3_Impl::Tidy(this).into()
            }
        }
        Self {
            base__: IMSMQApplication2_Vtbl::new::<Identity, OFFSET>(),
            ActiveQueues: ActiveQueues::<Identity, OFFSET>,
            PrivateQueues: PrivateQueues::<Identity, OFFSET>,
            DirectoryServiceServer: DirectoryServiceServer::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            BytesInAllQueues: BytesInAllQueues::<Identity, OFFSET>,
            SetMachine: SetMachine::<Identity, OFFSET>,
            Machine: Machine::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Tidy: Tidy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQApplication3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQApplication as windows_core::Interface>::IID || iid == &<IMSMQApplication2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQApplication3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQCollection, IMSMQCollection_Vtbl, 0x0188ac2f_ecb3_4173_9779_635ca2039c72);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCollection {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Item(&self, index: *const super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute(index), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCollection_Impl: super::Com::IDispatch_Impl {
    fn Item(&self, index: *const super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQCollection_Vtbl {
    pub const fn new<Identity: IMSMQCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *const super::Variant::VARIANT, pvarret: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pvarret.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQCoordinatedTransactionDispenser, IMSMQCoordinatedTransactionDispenser_Vtbl, 0xd7d6e081_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQCoordinatedTransactionDispenser {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQCoordinatedTransactionDispenser, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser {
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCoordinatedTransactionDispenser_Impl: super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub const fn new<Identity: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginTransaction<Identity: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCoordinatedTransactionDispenser_Impl::BeginTransaction(this) {
                    Ok(ok__) => {
                        ptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQCoordinatedTransactionDispenser {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQCoordinatedTransactionDispenser2, IMSMQCoordinatedTransactionDispenser2_Vtbl, 0xeba96b10_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQCoordinatedTransactionDispenser2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQCoordinatedTransactionDispenser2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser2 {
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCoordinatedTransactionDispenser2_Impl: super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub const fn new<Identity: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginTransaction<Identity: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCoordinatedTransactionDispenser2_Impl::BeginTransaction(this) {
                    Ok(ok__) => {
                        ptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCoordinatedTransactionDispenser2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQCoordinatedTransactionDispenser2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQCoordinatedTransactionDispenser3, IMSMQCoordinatedTransactionDispenser3_Vtbl, 0xeba96b14_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQCoordinatedTransactionDispenser3 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQCoordinatedTransactionDispenser3, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser3 {
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCoordinatedTransactionDispenser3_Impl: super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub const fn new<Identity: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginTransaction<Identity: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCoordinatedTransactionDispenser3_Impl::BeginTransaction(this) {
                    Ok(ok__) => {
                        ptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQCoordinatedTransactionDispenser3_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQCoordinatedTransactionDispenser3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQDestination, IMSMQDestination_Vtbl, 0xeba96b16_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQDestination {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQDestination, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQDestination {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IADs(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IADs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_IADs<P0>(&self, piads: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_IADs)(windows_core::Interface::as_raw(self), piads.param().abi()).ok() }
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetADsPath(&self, bstradspath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetADsPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstradspath)).ok() }
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpathname)).ok() }
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrformatname)).ok() }
    }
    pub unsafe fn Destinations(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Destinations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_Destinations<P0>(&self, pdestinations: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_Destinations)(windows_core::Interface::as_raw(self), pdestinations.param().abi()).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IADs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_IADs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Destinations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_Destinations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQDestination_Impl: super::Com::IDispatch_Impl {
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn IsOpen(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IADs(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_IADs(&self, piads: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetADsPath(&self, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Destinations(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_Destinations(&self, pdestinations: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQDestination_Vtbl {
    pub const fn new<Identity: IMSMQDestination_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQDestination_Impl::Open(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQDestination_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn IsOpen<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisopen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQDestination_Impl::IsOpen(this) {
                    Ok(ok__) => {
                        pfisopen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IADs<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiads: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQDestination_Impl::IADs(this) {
                    Ok(ok__) => {
                        ppiads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_IADs<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piads: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQDestination_Impl::putref_IADs(this, core::mem::transmute_copy(&piads)).into()
            }
        }
        unsafe extern "system" fn ADsPath<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstradspath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQDestination_Impl::ADsPath(this) {
                    Ok(ok__) => {
                        pbstradspath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetADsPath<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradspath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQDestination_Impl::SetADsPath(this, core::mem::transmute(&bstradspath)).into()
            }
        }
        unsafe extern "system" fn PathName<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQDestination_Impl::PathName(this) {
                    Ok(ok__) => {
                        pbstrpathname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPathName<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQDestination_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
            }
        }
        unsafe extern "system" fn FormatName<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQDestination_Impl::FormatName(this) {
                    Ok(ok__) => {
                        pbstrformatname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQDestination_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
            }
        }
        unsafe extern "system" fn Destinations<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestinations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQDestination_Impl::Destinations(this) {
                    Ok(ok__) => {
                        ppdestinations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_Destinations<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinations: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQDestination_Impl::putref_Destinations(this, core::mem::transmute_copy(&pdestinations)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQDestination_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            IADs: IADs::<Identity, OFFSET>,
            putref_IADs: putref_IADs::<Identity, OFFSET>,
            ADsPath: ADsPath::<Identity, OFFSET>,
            SetADsPath: SetADsPath::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            Destinations: Destinations::<Identity, OFFSET>,
            putref_Destinations: putref_Destinations::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQDestination as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQDestination {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQEvent, IMSMQEvent_Vtbl, 0xd7d6e077_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQEvent_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQEvent_Vtbl {
    pub const fn new<Identity: IMSMQEvent_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQEvent {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQEvent2, IMSMQEvent2_Vtbl, 0xeba96b12_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQEvent2 {
    type Target = IMSMQEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQEvent2, windows_core::IUnknown, super::Com::IDispatch, IMSMQEvent);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent2 {
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent2_Vtbl {
    pub base__: IMSMQEvent_Vtbl,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQEvent2_Impl: IMSMQEvent_Impl {
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQEvent2_Vtbl {
    pub const fn new<Identity: IMSMQEvent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Properties<Identity: IMSMQEvent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQEvent2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMSMQEvent_Vtbl::new::<Identity, OFFSET>(), Properties: Properties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQEvent2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQEvent3, IMSMQEvent3_Vtbl, 0xeba96b1c_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQEvent3 {
    type Target = IMSMQEvent2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQEvent3, windows_core::IUnknown, super::Com::IDispatch, IMSMQEvent, IMSMQEvent2);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent3_Vtbl {
    pub base__: IMSMQEvent2_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQEvent3_Impl: IMSMQEvent2_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQEvent3_Vtbl {
    pub const fn new<Identity: IMSMQEvent3_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IMSMQEvent2_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQEvent3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQEvent as windows_core::Interface>::IID || iid == &<IMSMQEvent2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQEvent3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQManagement, IMSMQManagement_Vtbl, 0xbe5f0241_e489_4957_8cc4_a452fcf3e23e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQManagement {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQManagement, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQManagement {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Init(&self, machine: *const super::Variant::VARIANT, pathname: *const super::Variant::VARIANT, formatname: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), core::mem::transmute(machine), core::mem::transmute(pathname), core::mem::transmute(formatname)).ok() }
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Machine(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Machine)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MessageCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ForeignStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForeignStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueueType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransactionalStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionalStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn BytesInQueue(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BytesInQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQManagement_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Init: usize,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Machine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ForeignStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TransactionalStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub BytesInQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    BytesInQueue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQManagement_Impl: super::Com::IDispatch_Impl {
    fn Init(&self, machine: *const super::Variant::VARIANT, pathname: *const super::Variant::VARIANT, formatname: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Machine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MessageCount(&self) -> windows_core::Result<i32>;
    fn ForeignStatus(&self) -> windows_core::Result<i32>;
    fn QueueType(&self) -> windows_core::Result<i32>;
    fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TransactionalStatus(&self) -> windows_core::Result<i32>;
    fn BytesInQueue(&self) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQManagement_Vtbl {
    pub const fn new<Identity: IMSMQManagement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machine: *const super::Variant::VARIANT, pathname: *const super::Variant::VARIANT, formatname: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQManagement_Impl::Init(this, core::mem::transmute_copy(&machine), core::mem::transmute_copy(&pathname), core::mem::transmute_copy(&formatname)).into()
            }
        }
        unsafe extern "system" fn FormatName<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::FormatName(this) {
                    Ok(ok__) => {
                        pbstrformatname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Machine<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmachine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::Machine(this) {
                    Ok(ok__) => {
                        pbstrmachine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MessageCount<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmessagecount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::MessageCount(this) {
                    Ok(ok__) => {
                        plmessagecount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ForeignStatus<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plforeignstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::ForeignStatus(this) {
                    Ok(ok__) => {
                        plforeignstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueType<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plqueuetype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::QueueType(this) {
                    Ok(ok__) => {
                        plqueuetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLocal<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::IsLocal(this) {
                    Ok(ok__) => {
                        pfislocal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransactionalStatus<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltransactionalstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::TransactionalStatus(this) {
                    Ok(ok__) => {
                        pltransactionalstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BytesInQueue<Identity: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbytesinqueue: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQManagement_Impl::BytesInQueue(this) {
                    Ok(ok__) => {
                        pvbytesinqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            Machine: Machine::<Identity, OFFSET>,
            MessageCount: MessageCount::<Identity, OFFSET>,
            ForeignStatus: ForeignStatus::<Identity, OFFSET>,
            QueueType: QueueType::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            TransactionalStatus: TransactionalStatus::<Identity, OFFSET>,
            BytesInQueue: BytesInQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQManagement as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQManagement {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQMessage, IMSMQMessage_Vtbl, 0xd7d6e074_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQMessage {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQMessage, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage {
    pub unsafe fn Class(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok() }
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok() }
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok() }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok() }
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Body(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varbody)).ok() }
    }
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Id(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varmsgid)).ok() }
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok() }
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok() }
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok() }
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsendercert)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok() }
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Body: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetBody: usize,
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Id: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ArrivedTime: usize,
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage_Impl: super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: windows_core::Ref<IMSMQQueue>, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQMessage_Vtbl {
    pub const fn new<Identity: IMSMQMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Class<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Class(this) {
                    Ok(ok__) => {
                        plclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn AuthLevel<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::AuthLevel(this) {
                    Ok(ok__) => {
                        plauthlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::IsAuthenticated(this) {
                    Ok(ok__) => {
                        pisauthenticated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delivery<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Delivery(this) {
                    Ok(ok__) => {
                        pldelivery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
            }
        }
        unsafe extern "system" fn Trace<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Trace(this) {
                    Ok(ok__) => {
                        pltrace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTrace<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Priority(this) {
                    Ok(ok__) => {
                        plpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::ResponseQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::putref_ResponseQueueInfo(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AppSpecific<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::AppSpecific(this) {
                    Ok(ok__) => {
                        plappspecific.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::SourceMachineGuid(this) {
                    Ok(ok__) => {
                        pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BodyLength<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::BodyLength(this) {
                    Ok(ok__) => {
                        pcbbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Body<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Body(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBody<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::AdminQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::putref_AdminQueueInfo(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Id(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::CorrelationId(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
            }
        }
        unsafe extern "system" fn Ack<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Ack(this) {
                    Ok(ok__) => {
                        plack.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAck<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::MaxTimeToReachQueue(this) {
                    Ok(ok__) => {
                        plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::MaxTimeToReceive(this) {
                    Ok(ok__) => {
                        plmaxtimetoreceive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::HashAlgorithm(this) {
                    Ok(ok__) => {
                        plhashalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::EncryptAlgorithm(this) {
                    Ok(ok__) => {
                        plencryptalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
            }
        }
        unsafe extern "system" fn SentTime<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::SentTime(this) {
                    Ok(ok__) => {
                        pvarsenttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::ArrivedTime(this) {
                    Ok(ok__) => {
                        plarrivedtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::DestinationQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfodest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::SenderCertificate(this) {
                    Ok(ok__) => {
                        pvarsendercert.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
            }
        }
        unsafe extern "system" fn SenderId<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::SenderId(this) {
                    Ok(ok__) => {
                        pvarsenderid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage_Impl::SenderIdType(this) {
                    Ok(ok__) => {
                        plsenderidtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
            }
        }
        unsafe extern "system" fn Send<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::Send(this, core::mem::transmute_copy(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage_Impl::AttachCurrentSecurityContext(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQMessage {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQMessage2, IMSMQMessage2_Vtbl, 0xd9933be0_a567_11d2_b0f3_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQMessage2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQMessage2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage2 {
    pub unsafe fn Class(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok() }
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok() }
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok() }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok() }
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Body(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varbody)).ok() }
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo_v1<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Id(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varmsgid)).ok() }
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok() }
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok() }
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok() }
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsendercert)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue2>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok() }
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SenderVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Extension(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Extension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtension(&self, varextension: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varextension)).ok() }
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectorTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectorTypeGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidconnectortype)).ok() }
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationSymmetricKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vardestsymmkey)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Signature(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSignature(&self, varsignature: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSignature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsignature)).ok() }
    }
    pub unsafe fn AuthenticationProviderType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationProviderType)(windows_core::Interface::as_raw(self), lauthprovtype).ok() }
    }
    pub unsafe fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationProviderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationProviderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrauthprovname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSenderId(&self, varsenderid: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsenderid)).ok() }
    }
    pub unsafe fn MsgClass(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MsgClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMsgClass)(windows_core::Interface::as_raw(self), lmsgclass).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TransactionId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsFirstInTransaction(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLastInTransaction(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLastInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Body: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetBody: usize,
    pub AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Id: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ArrivedTime: usize,
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExtension: usize,
    pub ConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSenderId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage2_Impl: super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: windows_core::Ref<IMSMQQueue2>, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
    fn SenderVersion(&self) -> windows_core::Result<i32>;
    fn Extension(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetExtension(&self, varextension: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn DestinationSymmetricKey(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Signature(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AuthenticationProviderType(&self) -> windows_core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()>;
    fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn MsgClass(&self) -> windows_core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn IsFirstInTransaction(&self) -> windows_core::Result<i16>;
    fn IsLastInTransaction(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQMessage2_Vtbl {
    pub const fn new<Identity: IMSMQMessage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Class<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Class(this) {
                    Ok(ok__) => {
                        plclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn AuthLevel<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::AuthLevel(this) {
                    Ok(ok__) => {
                        plauthlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::IsAuthenticated(this) {
                    Ok(ok__) => {
                        pisauthenticated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delivery<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Delivery(this) {
                    Ok(ok__) => {
                        pldelivery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
            }
        }
        unsafe extern "system" fn Trace<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Trace(this) {
                    Ok(ok__) => {
                        pltrace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTrace<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Priority(this) {
                    Ok(ok__) => {
                        plpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::ResponseQueueInfo_v1(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::putref_ResponseQueueInfo_v1(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AppSpecific<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::AppSpecific(this) {
                    Ok(ok__) => {
                        plappspecific.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::SourceMachineGuid(this) {
                    Ok(ok__) => {
                        pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BodyLength<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::BodyLength(this) {
                    Ok(ok__) => {
                        pcbbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Body<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Body(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBody<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::AdminQueueInfo_v1(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::putref_AdminQueueInfo_v1(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Id(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::CorrelationId(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
            }
        }
        unsafe extern "system" fn Ack<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Ack(this) {
                    Ok(ok__) => {
                        plack.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAck<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::MaxTimeToReachQueue(this) {
                    Ok(ok__) => {
                        plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::MaxTimeToReceive(this) {
                    Ok(ok__) => {
                        plmaxtimetoreceive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::HashAlgorithm(this) {
                    Ok(ok__) => {
                        plhashalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::EncryptAlgorithm(this) {
                    Ok(ok__) => {
                        plencryptalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
            }
        }
        unsafe extern "system" fn SentTime<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::SentTime(this) {
                    Ok(ok__) => {
                        pvarsenttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::ArrivedTime(this) {
                    Ok(ok__) => {
                        plarrivedtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::DestinationQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfodest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::SenderCertificate(this) {
                    Ok(ok__) => {
                        pvarsendercert.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
            }
        }
        unsafe extern "system" fn SenderId<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::SenderId(this) {
                    Ok(ok__) => {
                        pvarsenderid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::SenderIdType(this) {
                    Ok(ok__) => {
                        plsenderidtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
            }
        }
        unsafe extern "system" fn Send<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::Send(this, core::mem::transmute_copy(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::AttachCurrentSecurityContext(this).into()
            }
        }
        unsafe extern "system" fn SenderVersion<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::SenderVersion(this) {
                    Ok(ok__) => {
                        plsenderversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Extension<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarextension: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Extension(this) {
                    Ok(ok__) => {
                        pvarextension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtension<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varextension: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetExtension(this, core::mem::transmute(&varextension)).into()
            }
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidconnectortype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::ConnectorTypeGuid(this) {
                    Ok(ok__) => {
                        pbstrguidconnectortype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidconnectortype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetConnectorTypeGuid(this, core::mem::transmute(&bstrguidconnectortype)).into()
            }
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoxactstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::TransactionStatusQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfoxactstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardestsymmkey: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::DestinationSymmetricKey(this) {
                    Ok(ok__) => {
                        pvardestsymmkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestsymmkey: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetDestinationSymmetricKey(this, core::mem::transmute(&vardestsymmkey)).into()
            }
        }
        unsafe extern "system" fn Signature<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsignature: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Signature(this) {
                    Ok(ok__) => {
                        pvarsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignature<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsignature: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetSignature(this, core::mem::transmute(&varsignature)).into()
            }
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthprovtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::AuthenticationProviderType(this) {
                    Ok(ok__) => {
                        plauthprovtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthprovtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetAuthenticationProviderType(this, core::mem::transmute_copy(&lauthprovtype)).into()
            }
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthprovname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::AuthenticationProviderName(this) {
                    Ok(ok__) => {
                        pbstrauthprovname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthprovname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetAuthenticationProviderName(this, core::mem::transmute(&bstrauthprovname)).into()
            }
        }
        unsafe extern "system" fn SetSenderId<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsenderid: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetSenderId(this, core::mem::transmute(&varsenderid)).into()
            }
        }
        unsafe extern "system" fn MsgClass<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmsgclass: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::MsgClass(this) {
                    Ok(ok__) => {
                        plmsgclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmsgclass: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::SetMsgClass(this, core::mem::transmute_copy(&lmsgclass)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransactionId<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarxactid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::TransactionId(this) {
                    Ok(ok__) => {
                        pvarxactid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::IsFirstInTransaction(this) {
                    Ok(ok__) => {
                        pisfirstinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::IsLastInTransaction(this) {
                    Ok(ok__) => {
                        pislastinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::ResponseQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::putref_ResponseQueueInfo(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::AdminQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage2_Impl::putref_AdminQueueInfo(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage2_Impl::ReceivedAuthenticationLevel(this) {
                    Ok(ok__) => {
                        psreceivedauthenticationlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
            SenderVersion: SenderVersion::<Identity, OFFSET>,
            Extension: Extension::<Identity, OFFSET>,
            SetExtension: SetExtension::<Identity, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, OFFSET>,
            Signature: Signature::<Identity, OFFSET>,
            SetSignature: SetSignature::<Identity, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, OFFSET>,
            SetSenderId: SetSenderId::<Identity, OFFSET>,
            MsgClass: MsgClass::<Identity, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            TransactionId: TransactionId::<Identity, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQMessage2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQMessage3, IMSMQMessage3_Vtbl, 0xeba96b1a_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQMessage3 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQMessage3, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage3 {
    pub unsafe fn Class(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok() }
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok() }
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok() }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok() }
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Body(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varbody)).ok() }
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo_v1<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Id(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varmsgid)).ok() }
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok() }
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok() }
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok() }
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsendercert)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok() }
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SenderVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Extension(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Extension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtension(&self, varextension: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varextension)).ok() }
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectorTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectorTypeGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidconnectortype)).ok() }
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationSymmetricKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vardestsymmkey)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Signature(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSignature(&self, varsignature: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSignature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsignature)).ok() }
    }
    pub unsafe fn AuthenticationProviderType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationProviderType)(windows_core::Interface::as_raw(self), lauthprovtype).ok() }
    }
    pub unsafe fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationProviderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationProviderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrauthprovname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSenderId(&self, varsenderid: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsenderid)).ok() }
    }
    pub unsafe fn MsgClass(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MsgClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMsgClass)(windows_core::Interface::as_raw(self), lmsgclass).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TransactionId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsFirstInTransaction(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLastInTransaction(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLastInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo_v2<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo_v2<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo3>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo3>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    pub unsafe fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseDestination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseDestination<P0>(&self, pdestresponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseDestination)(windows_core::Interface::as_raw(self), pdestresponse.param().abi()).ok() }
    }
    pub unsafe fn Destination(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Destination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAuthenticated2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLastInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachCurrentSecurityContext2)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SoapEnvelope)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CompoundMessage(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompoundMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSoapHeader(&self, bstrsoapheader: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSoapHeader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsoapheader)).ok() }
    }
    pub unsafe fn SetSoapBody(&self, bstrsoapbody: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSoapBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsoapbody)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Body: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetBody: usize,
    pub AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Id: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ArrivedTime: usize,
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExtension: usize,
    pub ConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSenderId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SoapEnvelope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CompoundMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CompoundMessage: usize,
    pub SetSoapHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSoapBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage3_Impl: super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: windows_core::Ref<super::Com::IDispatch>, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
    fn SenderVersion(&self) -> windows_core::Result<i32>;
    fn Extension(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetExtension(&self, varextension: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn DestinationSymmetricKey(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Signature(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AuthenticationProviderType(&self) -> windows_core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()>;
    fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn MsgClass(&self) -> windows_core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn IsFirstInTransaction(&self) -> windows_core::Result<i16>;
    fn IsLastInTransaction(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo3>) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo3>) -> windows_core::Result<()>;
    fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Destination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()>;
    fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CompoundMessage(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQMessage3_Vtbl {
    pub const fn new<Identity: IMSMQMessage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Class<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Class(this) {
                    Ok(ok__) => {
                        plclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn AuthLevel<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::AuthLevel(this) {
                    Ok(ok__) => {
                        plauthlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::IsAuthenticated(this) {
                    Ok(ok__) => {
                        pisauthenticated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delivery<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Delivery(this) {
                    Ok(ok__) => {
                        pldelivery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
            }
        }
        unsafe extern "system" fn Trace<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Trace(this) {
                    Ok(ok__) => {
                        pltrace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTrace<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Priority(this) {
                    Ok(ok__) => {
                        plpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::ResponseQueueInfo_v1(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::putref_ResponseQueueInfo_v1(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AppSpecific<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::AppSpecific(this) {
                    Ok(ok__) => {
                        plappspecific.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::SourceMachineGuid(this) {
                    Ok(ok__) => {
                        pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BodyLength<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::BodyLength(this) {
                    Ok(ok__) => {
                        pcbbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Body<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Body(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBody<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::AdminQueueInfo_v1(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::putref_AdminQueueInfo_v1(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Id(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::CorrelationId(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
            }
        }
        unsafe extern "system" fn Ack<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Ack(this) {
                    Ok(ok__) => {
                        plack.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAck<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::MaxTimeToReachQueue(this) {
                    Ok(ok__) => {
                        plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::MaxTimeToReceive(this) {
                    Ok(ok__) => {
                        plmaxtimetoreceive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::HashAlgorithm(this) {
                    Ok(ok__) => {
                        plhashalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::EncryptAlgorithm(this) {
                    Ok(ok__) => {
                        plencryptalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
            }
        }
        unsafe extern "system" fn SentTime<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::SentTime(this) {
                    Ok(ok__) => {
                        pvarsenttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::ArrivedTime(this) {
                    Ok(ok__) => {
                        plarrivedtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::DestinationQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfodest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::SenderCertificate(this) {
                    Ok(ok__) => {
                        pvarsendercert.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
            }
        }
        unsafe extern "system" fn SenderId<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::SenderId(this) {
                    Ok(ok__) => {
                        pvarsenderid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::SenderIdType(this) {
                    Ok(ok__) => {
                        plsenderidtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
            }
        }
        unsafe extern "system" fn Send<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::Send(this, core::mem::transmute_copy(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::AttachCurrentSecurityContext(this).into()
            }
        }
        unsafe extern "system" fn SenderVersion<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::SenderVersion(this) {
                    Ok(ok__) => {
                        plsenderversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Extension<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarextension: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Extension(this) {
                    Ok(ok__) => {
                        pvarextension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtension<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varextension: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetExtension(this, core::mem::transmute(&varextension)).into()
            }
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidconnectortype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::ConnectorTypeGuid(this) {
                    Ok(ok__) => {
                        pbstrguidconnectortype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidconnectortype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetConnectorTypeGuid(this, core::mem::transmute(&bstrguidconnectortype)).into()
            }
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoxactstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::TransactionStatusQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfoxactstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardestsymmkey: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::DestinationSymmetricKey(this) {
                    Ok(ok__) => {
                        pvardestsymmkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestsymmkey: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetDestinationSymmetricKey(this, core::mem::transmute(&vardestsymmkey)).into()
            }
        }
        unsafe extern "system" fn Signature<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsignature: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Signature(this) {
                    Ok(ok__) => {
                        pvarsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignature<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsignature: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetSignature(this, core::mem::transmute(&varsignature)).into()
            }
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthprovtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::AuthenticationProviderType(this) {
                    Ok(ok__) => {
                        plauthprovtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthprovtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetAuthenticationProviderType(this, core::mem::transmute_copy(&lauthprovtype)).into()
            }
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthprovname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::AuthenticationProviderName(this) {
                    Ok(ok__) => {
                        pbstrauthprovname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthprovname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetAuthenticationProviderName(this, core::mem::transmute(&bstrauthprovname)).into()
            }
        }
        unsafe extern "system" fn SetSenderId<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsenderid: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetSenderId(this, core::mem::transmute(&varsenderid)).into()
            }
        }
        unsafe extern "system" fn MsgClass<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmsgclass: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::MsgClass(this) {
                    Ok(ok__) => {
                        plmsgclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmsgclass: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetMsgClass(this, core::mem::transmute_copy(&lmsgclass)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransactionId<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarxactid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::TransactionId(this) {
                    Ok(ok__) => {
                        pvarxactid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::IsFirstInTransaction(this) {
                    Ok(ok__) => {
                        pisfirstinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::IsLastInTransaction(this) {
                    Ok(ok__) => {
                        pislastinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::ResponseQueueInfo_v2(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::putref_ResponseQueueInfo_v2(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::AdminQueueInfo_v2(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::putref_AdminQueueInfo_v2(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::ReceivedAuthenticationLevel(this) {
                    Ok(ok__) => {
                        psreceivedauthenticationlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::ResponseQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::putref_ResponseQueueInfo(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::AdminQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::putref_AdminQueueInfo(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn ResponseDestination<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::ResponseDestination(this) {
                    Ok(ok__) => {
                        ppdestresponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestresponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::putref_ResponseDestination(this, core::mem::transmute_copy(&pdestresponse)).into()
            }
        }
        unsafe extern "system" fn Destination<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::Destination(this) {
                    Ok(ok__) => {
                        ppdestdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LookupId<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarlookupid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::LookupId(this) {
                    Ok(ok__) => {
                        pvarlookupid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::IsAuthenticated2(this) {
                    Ok(ok__) => {
                        pisauthenticated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::IsFirstInTransaction2(this) {
                    Ok(ok__) => {
                        pisfirstinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::IsLastInTransaction2(this) {
                    Ok(ok__) => {
                        pislastinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::AttachCurrentSecurityContext2(this).into()
            }
        }
        unsafe extern "system" fn SoapEnvelope<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsoapenvelope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::SoapEnvelope(this) {
                    Ok(ok__) => {
                        pbstrsoapenvelope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcompoundmessage: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage3_Impl::CompoundMessage(this) {
                    Ok(ok__) => {
                        pvarcompoundmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetSoapHeader(this, core::mem::transmute(&bstrsoapheader)).into()
            }
        }
        unsafe extern "system" fn SetSoapBody<Identity: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapbody: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage3_Impl::SetSoapBody(this, core::mem::transmute(&bstrsoapbody)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
            SenderVersion: SenderVersion::<Identity, OFFSET>,
            Extension: Extension::<Identity, OFFSET>,
            SetExtension: SetExtension::<Identity, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, OFFSET>,
            Signature: Signature::<Identity, OFFSET>,
            SetSignature: SetSignature::<Identity, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, OFFSET>,
            SetSenderId: SetSenderId::<Identity, OFFSET>,
            MsgClass: MsgClass::<Identity, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            TransactionId: TransactionId::<Identity, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, OFFSET>,
            Destination: Destination::<Identity, OFFSET>,
            LookupId: LookupId::<Identity, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQMessage3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQMessage4, IMSMQMessage4_Vtbl, 0xeba96b23_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQMessage4 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQMessage4, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage4 {
    pub unsafe fn Class(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok() }
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok() }
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok() }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok() }
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Body(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varbody)).ok() }
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo_v1<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Id(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varmsgid)).ok() }
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok() }
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok() }
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok() }
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsendercert)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok() }
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SenderVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SenderVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Extension(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Extension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetExtension(&self, varextension: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varextension)).ok() }
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectorTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectorTypeGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidconnectortype)).ok() }
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestinationSymmetricKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vardestsymmkey)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Signature(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSignature(&self, varsignature: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSignature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsignature)).ok() }
    }
    pub unsafe fn AuthenticationProviderType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationProviderType)(windows_core::Interface::as_raw(self), lauthprovtype).ok() }
    }
    pub unsafe fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationProviderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationProviderName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrauthprovname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSenderId(&self, varsenderid: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSenderId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsenderid)).ok() }
    }
    pub unsafe fn MsgClass(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MsgClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMsgClass)(windows_core::Interface::as_raw(self), lmsgclass).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TransactionId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsFirstInTransaction(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLastInTransaction(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLastInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo_v2<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo_v2<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo4>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok() }
    }
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo4>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok() }
    }
    pub unsafe fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResponseDestination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_ResponseDestination<P0>(&self, pdestresponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_ResponseDestination)(windows_core::Interface::as_raw(self), pdestresponse.param().abi()).ok() }
    }
    pub unsafe fn Destination(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Destination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupId(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAuthenticated2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLastInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachCurrentSecurityContext2)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SoapEnvelope)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CompoundMessage(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompoundMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSoapHeader(&self, bstrsoapheader: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSoapHeader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsoapheader)).ok() }
    }
    pub unsafe fn SetSoapBody(&self, bstrsoapbody: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSoapBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsoapbody)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Delivery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Trace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Body: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetBody: usize,
    pub AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Id: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ArrivedTime: usize,
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetExtension: usize,
    pub ConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSenderId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SoapEnvelope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CompoundMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CompoundMessage: usize,
    pub SetSoapHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSoapBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage4_Impl: super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(&self, varbody: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn SenderCertificate(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: windows_core::Ref<super::Com::IDispatch>, transaction: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
    fn SenderVersion(&self) -> windows_core::Result<i32>;
    fn Extension(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetExtension(&self, varextension: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn DestinationSymmetricKey(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Signature(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AuthenticationProviderType(&self) -> windows_core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()>;
    fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn MsgClass(&self) -> windows_core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn IsFirstInTransaction(&self) -> windows_core::Result<i16>;
    fn IsLastInTransaction(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: windows_core::Ref<IMSMQQueueInfo4>) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: windows_core::Ref<IMSMQQueueInfo4>) -> windows_core::Result<()>;
    fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Destination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()>;
    fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CompoundMessage(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQMessage4_Vtbl {
    pub const fn new<Identity: IMSMQMessage4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Class<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Class(this) {
                    Ok(ok__) => {
                        plclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn AuthLevel<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::AuthLevel(this) {
                    Ok(ok__) => {
                        plauthlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::IsAuthenticated(this) {
                    Ok(ok__) => {
                        pisauthenticated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delivery<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Delivery(this) {
                    Ok(ok__) => {
                        pldelivery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
            }
        }
        unsafe extern "system" fn Trace<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Trace(this) {
                    Ok(ok__) => {
                        pltrace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTrace<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Priority(this) {
                    Ok(ok__) => {
                        plpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::ResponseQueueInfo_v1(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::putref_ResponseQueueInfo_v1(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AppSpecific<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::AppSpecific(this) {
                    Ok(ok__) => {
                        plappspecific.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::SourceMachineGuid(this) {
                    Ok(ok__) => {
                        pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BodyLength<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::BodyLength(this) {
                    Ok(ok__) => {
                        pcbbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Body<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Body(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBody<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::AdminQueueInfo_v1(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::putref_AdminQueueInfo_v1(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Id(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::CorrelationId(this) {
                    Ok(ok__) => {
                        pvarmsgid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
            }
        }
        unsafe extern "system" fn Ack<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Ack(this) {
                    Ok(ok__) => {
                        plack.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAck<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::MaxTimeToReachQueue(this) {
                    Ok(ok__) => {
                        plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::MaxTimeToReceive(this) {
                    Ok(ok__) => {
                        plmaxtimetoreceive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::HashAlgorithm(this) {
                    Ok(ok__) => {
                        plhashalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::EncryptAlgorithm(this) {
                    Ok(ok__) => {
                        plencryptalg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
            }
        }
        unsafe extern "system" fn SentTime<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::SentTime(this) {
                    Ok(ok__) => {
                        pvarsenttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::ArrivedTime(this) {
                    Ok(ok__) => {
                        plarrivedtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::DestinationQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfodest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::SenderCertificate(this) {
                    Ok(ok__) => {
                        pvarsendercert.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
            }
        }
        unsafe extern "system" fn SenderId<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::SenderId(this) {
                    Ok(ok__) => {
                        pvarsenderid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::SenderIdType(this) {
                    Ok(ok__) => {
                        plsenderidtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
            }
        }
        unsafe extern "system" fn Send<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::Send(this, core::mem::transmute_copy(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::AttachCurrentSecurityContext(this).into()
            }
        }
        unsafe extern "system" fn SenderVersion<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::SenderVersion(this) {
                    Ok(ok__) => {
                        plsenderversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Extension<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarextension: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Extension(this) {
                    Ok(ok__) => {
                        pvarextension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtension<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varextension: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetExtension(this, core::mem::transmute(&varextension)).into()
            }
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidconnectortype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::ConnectorTypeGuid(this) {
                    Ok(ok__) => {
                        pbstrguidconnectortype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidconnectortype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetConnectorTypeGuid(this, core::mem::transmute(&bstrguidconnectortype)).into()
            }
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoxactstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::TransactionStatusQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfoxactstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardestsymmkey: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::DestinationSymmetricKey(this) {
                    Ok(ok__) => {
                        pvardestsymmkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestsymmkey: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetDestinationSymmetricKey(this, core::mem::transmute(&vardestsymmkey)).into()
            }
        }
        unsafe extern "system" fn Signature<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsignature: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Signature(this) {
                    Ok(ok__) => {
                        pvarsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSignature<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsignature: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetSignature(this, core::mem::transmute(&varsignature)).into()
            }
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthprovtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::AuthenticationProviderType(this) {
                    Ok(ok__) => {
                        plauthprovtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthprovtype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetAuthenticationProviderType(this, core::mem::transmute_copy(&lauthprovtype)).into()
            }
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthprovname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::AuthenticationProviderName(this) {
                    Ok(ok__) => {
                        pbstrauthprovname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthprovname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetAuthenticationProviderName(this, core::mem::transmute(&bstrauthprovname)).into()
            }
        }
        unsafe extern "system" fn SetSenderId<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsenderid: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetSenderId(this, core::mem::transmute(&varsenderid)).into()
            }
        }
        unsafe extern "system" fn MsgClass<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmsgclass: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::MsgClass(this) {
                    Ok(ok__) => {
                        plmsgclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmsgclass: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetMsgClass(this, core::mem::transmute_copy(&lmsgclass)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransactionId<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarxactid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::TransactionId(this) {
                    Ok(ok__) => {
                        pvarxactid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::IsFirstInTransaction(this) {
                    Ok(ok__) => {
                        pisfirstinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::IsLastInTransaction(this) {
                    Ok(ok__) => {
                        pislastinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::ResponseQueueInfo_v2(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::putref_ResponseQueueInfo_v2(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::AdminQueueInfo_v2(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::putref_AdminQueueInfo_v2(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::ReceivedAuthenticationLevel(this) {
                    Ok(ok__) => {
                        psreceivedauthenticationlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::ResponseQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinforesponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::putref_ResponseQueueInfo(this, core::mem::transmute_copy(&pqinforesponse)).into()
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::AdminQueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfoadmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::putref_AdminQueueInfo(this, core::mem::transmute_copy(&pqinfoadmin)).into()
            }
        }
        unsafe extern "system" fn ResponseDestination<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::ResponseDestination(this) {
                    Ok(ok__) => {
                        ppdestresponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestresponse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::putref_ResponseDestination(this, core::mem::transmute_copy(&pdestresponse)).into()
            }
        }
        unsafe extern "system" fn Destination<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::Destination(this) {
                    Ok(ok__) => {
                        ppdestdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LookupId<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarlookupid: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::LookupId(this) {
                    Ok(ok__) => {
                        pvarlookupid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::IsAuthenticated2(this) {
                    Ok(ok__) => {
                        pisauthenticated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::IsFirstInTransaction2(this) {
                    Ok(ok__) => {
                        pisfirstinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::IsLastInTransaction2(this) {
                    Ok(ok__) => {
                        pislastinxact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::AttachCurrentSecurityContext2(this).into()
            }
        }
        unsafe extern "system" fn SoapEnvelope<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsoapenvelope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::SoapEnvelope(this) {
                    Ok(ok__) => {
                        pbstrsoapenvelope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcompoundmessage: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQMessage4_Impl::CompoundMessage(this) {
                    Ok(ok__) => {
                        pvarcompoundmessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetSoapHeader(this, core::mem::transmute(&bstrsoapheader)).into()
            }
        }
        unsafe extern "system" fn SetSoapBody<Identity: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapbody: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQMessage4_Impl::SetSoapBody(this, core::mem::transmute(&bstrsoapbody)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
            SenderVersion: SenderVersion::<Identity, OFFSET>,
            Extension: Extension::<Identity, OFFSET>,
            SetExtension: SetExtension::<Identity, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, OFFSET>,
            Signature: Signature::<Identity, OFFSET>,
            SetSignature: SetSignature::<Identity, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, OFFSET>,
            SetSenderId: SetSenderId::<Identity, OFFSET>,
            MsgClass: MsgClass::<Identity, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            TransactionId: TransactionId::<Identity, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, OFFSET>,
            Destination: Destination::<Identity, OFFSET>,
            LookupId: LookupId::<Identity, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQMessage4 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQOutgoingQueueManagement, IMSMQOutgoingQueueManagement_Vtbl, 0x64c478fb_f9b0_4695_8a7f_439ac94326d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQOutgoingQueueManagement {
    type Target = IMSMQManagement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQOutgoingQueueManagement, windows_core::IUnknown, super::Com::IDispatch, IMSMQManagement);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQOutgoingQueueManagement {
    pub unsafe fn State(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn NextHops(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextHops)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EodGetSendInfo(&self) -> windows_core::Result<IMSMQCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EodGetSendInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EodResend(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EodResend)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQOutgoingQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub NextHops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    NextHops: usize,
    pub EodGetSendInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EodResend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQOutgoingQueueManagement_Impl: IMSMQManagement_Impl {
    fn State(&self) -> windows_core::Result<i32>;
    fn NextHops(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn EodGetSendInfo(&self) -> windows_core::Result<IMSMQCollection>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn EodResend(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQOutgoingQueueManagement_Vtbl {
    pub const fn new<Identity: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn State<Identity: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQOutgoingQueueManagement_Impl::State(this) {
                    Ok(ok__) => {
                        plstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextHops<Identity: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvnexthops: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQOutgoingQueueManagement_Impl::NextHops(this) {
                    Ok(ok__) => {
                        pvnexthops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EodGetSendInfo<Identity: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQOutgoingQueueManagement_Impl::EodGetSendInfo(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resume<Identity: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQOutgoingQueueManagement_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQOutgoingQueueManagement_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn EodResend<Identity: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQOutgoingQueueManagement_Impl::EodResend(this).into()
            }
        }
        Self {
            base__: IMSMQManagement_Vtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, OFFSET>,
            NextHops: NextHops::<Identity, OFFSET>,
            EodGetSendInfo: EodGetSendInfo::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            EodResend: EodResend::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQOutgoingQueueManagement as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQManagement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQOutgoingQueueManagement {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQPrivateDestination, IMSMQPrivateDestination_Vtbl, 0xeba96b17_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQPrivateDestination {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQPrivateDestination, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateDestination {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Handle(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetHandle(&self, varhandle: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varhandle)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Handle: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetHandle: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQPrivateDestination_Impl: super::Com::IDispatch_Impl {
    fn Handle(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetHandle(&self, varhandle: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQPrivateDestination_Vtbl {
    pub const fn new<Identity: IMSMQPrivateDestination_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handle<Identity: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarhandle: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQPrivateDestination_Impl::Handle(this) {
                    Ok(ok__) => {
                        pvarhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandle<Identity: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varhandle: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQPrivateDestination_Impl::SetHandle(this, core::mem::transmute(&varhandle)).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Handle: Handle::<Identity, OFFSET>, SetHandle: SetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQPrivateDestination as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQPrivateDestination {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQPrivateEvent, IMSMQPrivateEvent_Vtbl, 0xd7ab3341_c9d3_11d1_bb47_0080c7c5a2c0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQPrivateEvent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQPrivateEvent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateEvent {
    pub unsafe fn Hwnd(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FireArrivedEvent<P0>(&self, pq: P0, msgcursor: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).FireArrivedEvent)(windows_core::Interface::as_raw(self), pq.param().abi(), msgcursor).ok() }
    }
    pub unsafe fn FireArrivedErrorEvent<P0>(&self, pq: P0, hrstatus: windows_core::HRESULT, msgcursor: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).FireArrivedErrorEvent)(windows_core::Interface::as_raw(self), pq.param().abi(), hrstatus, msgcursor).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Hwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FireArrivedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FireArrivedErrorEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQPrivateEvent_Impl: super::Com::IDispatch_Impl {
    fn Hwnd(&self) -> windows_core::Result<i32>;
    fn FireArrivedEvent(&self, pq: windows_core::Ref<IMSMQQueue>, msgcursor: i32) -> windows_core::Result<()>;
    fn FireArrivedErrorEvent(&self, pq: windows_core::Ref<IMSMQQueue>, hrstatus: windows_core::HRESULT, msgcursor: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQPrivateEvent_Vtbl {
    pub const fn new<Identity: IMSMQPrivateEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Hwnd<Identity: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQPrivateEvent_Impl::Hwnd(this) {
                    Ok(ok__) => {
                        phwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FireArrivedEvent<Identity: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pq: *mut core::ffi::c_void, msgcursor: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQPrivateEvent_Impl::FireArrivedEvent(this, core::mem::transmute_copy(&pq), core::mem::transmute_copy(&msgcursor)).into()
            }
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Identity: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pq: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, msgcursor: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQPrivateEvent_Impl::FireArrivedErrorEvent(this, core::mem::transmute_copy(&pq), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&msgcursor)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Hwnd: Hwnd::<Identity, OFFSET>,
            FireArrivedEvent: FireArrivedEvent::<Identity, OFFSET>,
            FireArrivedErrorEvent: FireArrivedErrorEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQPrivateEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQPrivateEvent {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQuery, IMSMQQuery_Vtbl, 0xd7d6e072_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQuery {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQuery, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupQueue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery_Impl: super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQuery_Vtbl {
    pub const fn new<Identity: IMSMQQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LookupQueue<Identity: IMSMQQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                    Ok(ok__) => {
                        ppqinfos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), LookupQueue: LookupQueue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQuery {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQuery2, IMSMQQuery2_Vtbl, 0xeba96b0e_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQuery2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQuery2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupQueue: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery2_Impl: super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQuery2_Vtbl {
    pub const fn new<Identity: IMSMQQuery2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LookupQueue<Identity: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery2_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                    Ok(ok__) => {
                        ppqinfos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LookupQueue: LookupQueue::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQuery2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQuery3, IMSMQQuery3_Vtbl, 0xeba96b19_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQuery3 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQuery3, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery3 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupQueue_v2)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), core::mem::transmute(multicastaddress), core::mem::transmute(relmulticastaddress), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupQueue_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupQueue_v2: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupQueue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery3_Impl: super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos3>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQuery3_Vtbl {
    pub const fn new<Identity: IMSMQQuery3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LookupQueue_v2<Identity: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery3_Impl::LookupQueue_v2(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                    Ok(ok__) => {
                        ppqinfos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery3_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery3_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime), core::mem::transmute_copy(&multicastaddress), core::mem::transmute_copy(&relmulticastaddress)) {
                    Ok(ok__) => {
                        ppqinfos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            LookupQueue: LookupQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQuery3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQuery4, IMSMQQuery4_Vtbl, 0xeba96b24_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQuery4 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQuery4, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery4 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupQueue_v2)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), core::mem::transmute(multicastaddress), core::mem::transmute(relmulticastaddress), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupQueue_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupQueue_v2: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LookupQueue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery4_Impl: super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos4>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQQueueInfos4>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQuery4_Vtbl {
    pub const fn new<Identity: IMSMQQuery4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LookupQueue_v2<Identity: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery4_Impl::LookupQueue_v2(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                    Ok(ok__) => {
                        ppqinfos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery4_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQuery4_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime), core::mem::transmute_copy(&multicastaddress), core::mem::transmute_copy(&relmulticastaddress)) {
                    Ok(ok__) => {
                        ppqinfos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            LookupQueue: LookupQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQuery4 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueue, IMSMQQueue_Vtbl, 0xd7d6e076_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueue {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueue, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue {
    pub unsafe fn Access(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekCurrent: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue_Impl: super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: windows_core::Ref<IMSMQEvent>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueue_Vtbl {
    pub const fn new<Identity: IMSMQQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Access<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::Access(this) {
                    Ok(ok__) => {
                        placcess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShareMode<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::ShareMode(this) {
                    Ok(ok__) => {
                        plsharemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::QueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::Handle(this) {
                    Ok(ok__) => {
                        plhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOpen<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::IsOpen(this) {
                    Ok(ok__) => {
                        pisopen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Receive<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Peek<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue_Impl::EnableNotification(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNext<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueue2, IMSMQQueue2_Vtbl, 0xef0574e0_06d8_11d3_b100_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueue2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueue2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue2 {
    pub unsafe fn Access(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Receive_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Peek_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent2>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNext_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Receive_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Peek_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNext_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekCurrent: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue2_Impl: super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: windows_core::Ref<IMSMQEvent2>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueue2_Vtbl {
    pub const fn new<Identity: IMSMQQueue2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Access<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::Access(this) {
                    Ok(ok__) => {
                        placcess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShareMode<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::ShareMode(this) {
                    Ok(ok__) => {
                        plsharemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::QueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::Handle(this) {
                    Ok(ok__) => {
                        plhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOpen<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::IsOpen(this) {
                    Ok(ok__) => {
                        pisopen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue2_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Receive_v1<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::Receive_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::Peek_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue2_Impl::EnableNotification(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue2_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::ReceiveCurrent_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::PeekNext_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::PeekCurrent_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Receive<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Peek<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNext<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive_v1: Receive_v1::<Identity, OFFSET>,
            Peek_v1: Peek_v1::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueue2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueue3, IMSMQQueue3_Vtbl, 0xeba96b1b_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueue3 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueue3, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue3 {
    pub unsafe fn Access(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Receive_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Peek_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent3>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNext_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Handle2(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle2)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveNextByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveNextByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceivePreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceivePreviousByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNextByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNextByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekPreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekPreviousByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Purge(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Receive_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Peek_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNext_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekCurrent: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Handle2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Handle2: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue3_Impl: super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: windows_core::Ref<IMSMQEvent3>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Handle2(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ReceiveByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveNextByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceivePreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveFirstByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveLastByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekNextByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekPreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn Purge(&self) -> windows_core::Result<()>;
    fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueue3_Vtbl {
    pub const fn new<Identity: IMSMQQueue3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Access<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Access(this) {
                    Ok(ok__) => {
                        placcess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShareMode<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ShareMode(this) {
                    Ok(ok__) => {
                        plsharemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::QueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Handle(this) {
                    Ok(ok__) => {
                        plhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOpen<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::IsOpen(this) {
                    Ok(ok__) => {
                        pisopen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue3_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Receive_v1<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Receive_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Peek_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue3_Impl::EnableNotification(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue3_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ReceiveCurrent_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekNext_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekCurrent_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Receive<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Peek<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNext<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle2<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarhandle: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::Handle2(this) {
                    Ok(ok__) => {
                        pvarhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ReceiveByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ReceiveNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ReceivePreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ReceiveFirstByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::ReceiveLastByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekPreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekFirstByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::PeekLastByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Purge<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue3_Impl::Purge(this).into()
            }
        }
        unsafe extern "system" fn IsOpen2<Identity: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue3_Impl::IsOpen2(this) {
                    Ok(ok__) => {
                        pisopen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive_v1: Receive_v1::<Identity, OFFSET>,
            Peek_v1: Peek_v1::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Handle2: Handle2::<Identity, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, OFFSET>,
            Purge: Purge::<Identity, OFFSET>,
            IsOpen2: IsOpen2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueue3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueue4, IMSMQQueue4_Vtbl, 0xeba96b20_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueue4 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueue4, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue4 {
    pub unsafe fn Access(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Receive_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Peek_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent3>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNext_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Handle2(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle2)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveNextByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveNextByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceivePreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceivePreviousByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekNextByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekNextByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekPreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekPreviousByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PeekLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Purge(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReceiveByLookupIdAllowPeek(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReceiveByLookupIdAllowPeek)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lookupid), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Receive_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Peek_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNext_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekCurrent: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Handle2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Handle2: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReceiveByLookupIdAllowPeek: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReceiveByLookupIdAllowPeek: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue4_Impl: super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: windows_core::Ref<IMSMQEvent3>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn Peek(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveCurrent(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Handle2(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ReceiveByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveNextByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceivePreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveFirstByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveLastByLookupId(&self, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekNextByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekPreviousByLookupId(&self, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn Purge(&self) -> windows_core::Result<()>;
    fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiveByLookupIdAllowPeek(&self, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> windows_core::Result<IMSMQMessage4>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueue4_Vtbl {
    pub const fn new<Identity: IMSMQQueue4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Access<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Access(this) {
                    Ok(ok__) => {
                        placcess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShareMode<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ShareMode(this) {
                    Ok(ok__) => {
                        plsharemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::QueueInfo(this) {
                    Ok(ok__) => {
                        ppqinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Handle(this) {
                    Ok(ok__) => {
                        plhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOpen<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::IsOpen(this) {
                    Ok(ok__) => {
                        pisopen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue4_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Receive_v1<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Receive_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Peek_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue4_Impl::EnableNotification(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue4_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceiveCurrent_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekNext_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekCurrent_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Receive<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Peek<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNext<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle2<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarhandle: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::Handle2(this) {
                    Ok(ok__) => {
                        pvarhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceiveByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceiveNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceivePreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceiveFirstByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceiveLastByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekPreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekFirstByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::PeekLastByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Purge<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueue4_Impl::Purge(this).into()
            }
        }
        unsafe extern "system" fn IsOpen2<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::IsOpen2(this) {
                    Ok(ok__) => {
                        pisopen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Identity: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueue4_Impl::ReceiveByLookupIdAllowPeek(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                    Ok(ok__) => {
                        ppmsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive_v1: Receive_v1::<Identity, OFFSET>,
            Peek_v1: Peek_v1::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Handle2: Handle2::<Identity, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, OFFSET>,
            Purge: Purge::<Identity, OFFSET>,
            IsOpen2: IsOpen2::<Identity, OFFSET>,
            ReceiveByLookupIdAllowPeek: ReceiveByLookupIdAllowPeek::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueue4 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfo, IMSMQQueueInfo_Vtbl, 0xd7d6e07b_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo {
    pub unsafe fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidservicetype)).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpathname)).ok() }
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrformatname)).ok() }
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok() }
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok() }
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok() }
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo_Impl: super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfo_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueGuid<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::QueueGuid(this) {
                    Ok(ok__) => {
                        pbstrguidqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::ServiceTypeGuid(this) {
                    Ok(ok__) => {
                        pbstrguidservicetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn PathName<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::PathName(this) {
                    Ok(ok__) => {
                        pbstrpathname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPathName<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
            }
        }
        unsafe extern "system" fn FormatName<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::FormatName(this) {
                    Ok(ok__) => {
                        pbstrformatname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
            }
        }
        unsafe extern "system" fn IsTransactional<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::IsTransactional(this) {
                    Ok(ok__) => {
                        pistransactional.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn Quota<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::Quota(this) {
                    Ok(ok__) => {
                        plquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuota<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
            }
        }
        unsafe extern "system" fn BasePriority<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::BasePriority(this) {
                    Ok(ok__) => {
                        plbasepriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
            }
        }
        unsafe extern "system" fn CreateTime<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::CreateTime(this) {
                    Ok(ok__) => {
                        pvarcreatetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::ModifyTime(this) {
                    Ok(ok__) => {
                        pvarmodifytime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Authenticate<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::Authenticate(this) {
                    Ok(ok__) => {
                        plauthenticate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
            }
        }
        unsafe extern "system" fn JournalQuota<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::JournalQuota(this) {
                    Ok(ok__) => {
                        pljournalquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
            }
        }
        unsafe extern "system" fn IsWorldReadable<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::IsWorldReadable(this) {
                    Ok(ok__) => {
                        pisworldreadable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                    Ok(ok__) => {
                        ppq.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo_Impl::Update(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfo2, IMSMQQueueInfo2_Vtbl, 0xfd174a80_89cf_11d2_b0f2_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfo2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfo2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo2 {
    pub unsafe fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidservicetype)).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpathname)).ok() }
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrformatname)).ok() }
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok() }
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok() }
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok() }
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathNameDNS)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Security(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSecurity(&self, varsecurity: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsecurity)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Security: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSecurity: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo2_Impl: super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue2>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
    fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Security(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfo2_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueGuid<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::QueueGuid(this) {
                    Ok(ok__) => {
                        pbstrguidqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::ServiceTypeGuid(this) {
                    Ok(ok__) => {
                        pbstrguidservicetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn PathName<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::PathName(this) {
                    Ok(ok__) => {
                        pbstrpathname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPathName<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
            }
        }
        unsafe extern "system" fn FormatName<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::FormatName(this) {
                    Ok(ok__) => {
                        pbstrformatname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
            }
        }
        unsafe extern "system" fn IsTransactional<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::IsTransactional(this) {
                    Ok(ok__) => {
                        pistransactional.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn Quota<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::Quota(this) {
                    Ok(ok__) => {
                        plquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuota<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
            }
        }
        unsafe extern "system" fn BasePriority<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::BasePriority(this) {
                    Ok(ok__) => {
                        plbasepriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
            }
        }
        unsafe extern "system" fn CreateTime<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::CreateTime(this) {
                    Ok(ok__) => {
                        pvarcreatetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::ModifyTime(this) {
                    Ok(ok__) => {
                        pvarmodifytime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Authenticate<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::Authenticate(this) {
                    Ok(ok__) => {
                        plauthenticate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
            }
        }
        unsafe extern "system" fn JournalQuota<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::JournalQuota(this) {
                    Ok(ok__) => {
                        pljournalquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
            }
        }
        unsafe extern "system" fn IsWorldReadable<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::IsWorldReadable(this) {
                    Ok(ok__) => {
                        pisworldreadable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                    Ok(ok__) => {
                        ppq.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::Update(this).into()
            }
        }
        unsafe extern "system" fn PathNameDNS<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathnamedns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::PathNameDNS(this) {
                    Ok(ok__) => {
                        pbstrpathnamedns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsecurity: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo2_Impl::Security(this) {
                    Ok(ok__) => {
                        pvarsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsecurity: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo2_Impl::SetSecurity(this, core::mem::transmute(&varsecurity)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfo2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfo3, IMSMQQueueInfo3_Vtbl, 0xeba96b1d_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfo3 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfo3, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo3 {
    pub unsafe fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidservicetype)).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpathname)).ok() }
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrformatname)).ok() }
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok() }
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok() }
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok() }
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathNameDNS)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Security(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSecurity(&self, varsecurity: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsecurity)).ok() }
    }
    pub unsafe fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTransactional2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorldReadable2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MulticastAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMulticastAddress(&self, bstrmulticastaddress: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMulticastAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmulticastaddress)).ok() }
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Security: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo3_Impl: super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue3>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
    fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Security(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfo3_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueGuid<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::QueueGuid(this) {
                    Ok(ok__) => {
                        pbstrguidqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::ServiceTypeGuid(this) {
                    Ok(ok__) => {
                        pbstrguidservicetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn PathName<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::PathName(this) {
                    Ok(ok__) => {
                        pbstrpathname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPathName<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
            }
        }
        unsafe extern "system" fn FormatName<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::FormatName(this) {
                    Ok(ok__) => {
                        pbstrformatname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
            }
        }
        unsafe extern "system" fn IsTransactional<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::IsTransactional(this) {
                    Ok(ok__) => {
                        pistransactional.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn Quota<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::Quota(this) {
                    Ok(ok__) => {
                        plquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuota<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
            }
        }
        unsafe extern "system" fn BasePriority<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::BasePriority(this) {
                    Ok(ok__) => {
                        plbasepriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
            }
        }
        unsafe extern "system" fn CreateTime<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::CreateTime(this) {
                    Ok(ok__) => {
                        pvarcreatetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::ModifyTime(this) {
                    Ok(ok__) => {
                        pvarmodifytime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Authenticate<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::Authenticate(this) {
                    Ok(ok__) => {
                        plauthenticate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
            }
        }
        unsafe extern "system" fn JournalQuota<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::JournalQuota(this) {
                    Ok(ok__) => {
                        pljournalquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
            }
        }
        unsafe extern "system" fn IsWorldReadable<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::IsWorldReadable(this) {
                    Ok(ok__) => {
                        pisworldreadable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                    Ok(ok__) => {
                        ppq.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::Update(this).into()
            }
        }
        unsafe extern "system" fn PathNameDNS<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathnamedns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::PathNameDNS(this) {
                    Ok(ok__) => {
                        pbstrpathnamedns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsecurity: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::Security(this) {
                    Ok(ok__) => {
                        pvarsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsecurity: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetSecurity(this, core::mem::transmute(&varsecurity)).into()
            }
        }
        unsafe extern "system" fn IsTransactional2<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::IsTransactional2(this) {
                    Ok(ok__) => {
                        pistransactional.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::IsWorldReadable2(this) {
                    Ok(ok__) => {
                        pisworldreadable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmulticastaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::MulticastAddress(this) {
                    Ok(ok__) => {
                        pbstrmulticastaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmulticastaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo3_Impl::SetMulticastAddress(this, core::mem::transmute(&bstrmulticastaddress)).into()
            }
        }
        unsafe extern "system" fn ADsPath<Identity: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstradspath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo3_Impl::ADsPath(this) {
                    Ok(ok__) => {
                        pbstradspath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, OFFSET>,
            ADsPath: ADsPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfo3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfo4, IMSMQQueueInfo4_Vtbl, 0xeba96b21_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfo4 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfo4, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo4 {
    pub unsafe fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidservicetype)).ok() }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)).ok() }
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpathname)).ok() }
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrformatname)).ok() }
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok() }
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok() }
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok() }
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok() }
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok() }
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathNameDNS)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Security(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSecurity(&self, varsecurity: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varsecurity)).ok() }
    }
    pub unsafe fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTransactional2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorldReadable2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MulticastAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMulticastAddress(&self, bstrmulticastaddress: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMulticastAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmulticastaddress)).ok() }
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Security: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo4_Impl: super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue4>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
    fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Security(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfo4_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueueGuid<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::QueueGuid(this) {
                    Ok(ok__) => {
                        pbstrguidqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::ServiceTypeGuid(this) {
                    Ok(ok__) => {
                        pbstrguidservicetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
            }
        }
        unsafe extern "system" fn Label<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::Label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn PathName<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::PathName(this) {
                    Ok(ok__) => {
                        pbstrpathname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPathName<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
            }
        }
        unsafe extern "system" fn FormatName<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::FormatName(this) {
                    Ok(ok__) => {
                        pbstrformatname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
            }
        }
        unsafe extern "system" fn IsTransactional<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::IsTransactional(this) {
                    Ok(ok__) => {
                        pistransactional.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::PrivLevel(this) {
                    Ok(ok__) => {
                        plprivlevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
            }
        }
        unsafe extern "system" fn Journal<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::Journal(this) {
                    Ok(ok__) => {
                        pljournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournal<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
            }
        }
        unsafe extern "system" fn Quota<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::Quota(this) {
                    Ok(ok__) => {
                        plquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuota<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
            }
        }
        unsafe extern "system" fn BasePriority<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::BasePriority(this) {
                    Ok(ok__) => {
                        plbasepriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
            }
        }
        unsafe extern "system" fn CreateTime<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::CreateTime(this) {
                    Ok(ok__) => {
                        pvarcreatetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::ModifyTime(this) {
                    Ok(ok__) => {
                        pvarmodifytime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Authenticate<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::Authenticate(this) {
                    Ok(ok__) => {
                        plauthenticate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
            }
        }
        unsafe extern "system" fn JournalQuota<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::JournalQuota(this) {
                    Ok(ok__) => {
                        pljournalquota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
            }
        }
        unsafe extern "system" fn IsWorldReadable<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::IsWorldReadable(this) {
                    Ok(ok__) => {
                        pisworldreadable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                    Ok(ok__) => {
                        ppq.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::Update(this).into()
            }
        }
        unsafe extern "system" fn PathNameDNS<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathnamedns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::PathNameDNS(this) {
                    Ok(ok__) => {
                        pbstrpathnamedns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsecurity: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::Security(this) {
                    Ok(ok__) => {
                        pvarsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsecurity: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetSecurity(this, core::mem::transmute(&varsecurity)).into()
            }
        }
        unsafe extern "system" fn IsTransactional2<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::IsTransactional2(this) {
                    Ok(ok__) => {
                        pistransactional.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::IsWorldReadable2(this) {
                    Ok(ok__) => {
                        pisworldreadable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmulticastaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::MulticastAddress(this) {
                    Ok(ok__) => {
                        pbstrmulticastaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmulticastaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfo4_Impl::SetMulticastAddress(this, core::mem::transmute(&bstrmulticastaddress)).into()
            }
        }
        unsafe extern "system" fn ADsPath<Identity: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstradspath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfo4_Impl::ADsPath(this) {
                    Ok(ok__) => {
                        pbstradspath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, OFFSET>,
            ADsPath: ADsPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfo4 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfos, IMSMQQueueInfos_Vtbl, 0xd7d6e07d_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfos {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfos, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos_Impl: super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfos_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfos_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfos_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfos_Impl::Next(this) {
                    Ok(ok__) => {
                        ppqinfonext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Reset: Reset::<Identity, OFFSET>, Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfos {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfos2, IMSMQQueueInfos2_Vtbl, 0xeba96b0f_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfos2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfos2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos2 {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos2_Impl: super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfos2_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfos2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfos2_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfos2_Impl::Next(this) {
                    Ok(ok__) => {
                        ppqinfonext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfos2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfos2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfos3, IMSMQQueueInfos3_Vtbl, 0xeba96b1e_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfos3 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfos3, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos3 {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos3_Impl: super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfos3_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfos3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfos3_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfos3_Impl::Next(this) {
                    Ok(ok__) => {
                        ppqinfonext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfos3_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfos3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueInfos4, IMSMQQueueInfos4_Vtbl, 0xeba96b22_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueInfos4 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueInfos4, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos4 {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos4_Impl: super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueInfos4_Vtbl {
    pub const fn new<Identity: IMSMQQueueInfos4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQQueueInfos4_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfos4_Impl::Next(this) {
                    Ok(ok__) => {
                        ppqinfonext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueInfos4_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueInfos4 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQQueueManagement, IMSMQQueueManagement_Vtbl, 0x7fbe7759_5760_444d_b8a5_5e7ab9a84cce);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQQueueManagement {
    type Target = IMSMQManagement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQQueueManagement, windows_core::IUnknown, super::Com::IDispatch, IMSMQManagement);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueManagement {
    pub unsafe fn JournalMessageCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JournalMessageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn BytesInJournal(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BytesInJournal)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EodGetReceiveInfo(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EodGetReceiveInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub JournalMessageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub BytesInJournal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    BytesInJournal: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EodGetReceiveInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EodGetReceiveInfo: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueManagement_Impl: IMSMQManagement_Impl {
    fn JournalMessageCount(&self) -> windows_core::Result<i32>;
    fn BytesInJournal(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn EodGetReceiveInfo(&self) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQQueueManagement_Vtbl {
    pub const fn new<Identity: IMSMQQueueManagement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn JournalMessageCount<Identity: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalmessagecount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueManagement_Impl::JournalMessageCount(this) {
                    Ok(ok__) => {
                        pljournalmessagecount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BytesInJournal<Identity: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbytesinjournal: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueManagement_Impl::BytesInJournal(this) {
                    Ok(ok__) => {
                        pvbytesinjournal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EodGetReceiveInfo<Identity: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcollection: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQQueueManagement_Impl::EodGetReceiveInfo(this) {
                    Ok(ok__) => {
                        pvcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMSMQManagement_Vtbl::new::<Identity, OFFSET>(),
            JournalMessageCount: JournalMessageCount::<Identity, OFFSET>,
            BytesInJournal: BytesInJournal::<Identity, OFFSET>,
            EodGetReceiveInfo: EodGetReceiveInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueManagement as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQManagement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQQueueManagement {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQTransaction, IMSMQTransaction_Vtbl, 0xd7d6e07f_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQTransaction {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQTransaction, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction {
    pub unsafe fn Transaction(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Transaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Variant::VARIANT, grftc: *const super::Variant::VARIANT, grfrm: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), core::mem::transmute(fretaining), core::mem::transmute(grftc), core::mem::transmute(grfrm)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Variant::VARIANT, fasync: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), core::mem::transmute(fretaining), core::mem::transmute(fasync)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Transaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Commit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Abort: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransaction_Impl: super::Com::IDispatch_Impl {
    fn Transaction(&self) -> windows_core::Result<i32>;
    fn Commit(&self, fretaining: *const super::Variant::VARIANT, grftc: *const super::Variant::VARIANT, grfrm: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Abort(&self, fretaining: *const super::Variant::VARIANT, fasync: *const super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQTransaction_Vtbl {
    pub const fn new<Identity: IMSMQTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Transaction<Identity: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltransaction: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransaction_Impl::Transaction(this) {
                    Ok(ok__) => {
                        pltransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Commit<Identity: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: *const super::Variant::VARIANT, grftc: *const super::Variant::VARIANT, grfrm: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQTransaction_Impl::Commit(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grftc), core::mem::transmute_copy(&grfrm)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: *const super::Variant::VARIANT, fasync: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQTransaction_Impl::Abort(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&fasync)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Transaction: Transaction::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransaction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQTransaction {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQTransaction2, IMSMQTransaction2_Vtbl, 0x2ce0c5b0_6e67_11d2_b0e6_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQTransaction2 {
    type Target = IMSMQTransaction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQTransaction2, windows_core::IUnknown, super::Com::IDispatch, IMSMQTransaction);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn InitNew(&self, vartransaction: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vartransaction)).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction2_Vtbl {
    pub base__: IMSMQTransaction_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    InitNew: usize,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransaction2_Impl: IMSMQTransaction_Impl {
    fn InitNew(&self, vartransaction: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQTransaction2_Vtbl {
    pub const fn new<Identity: IMSMQTransaction2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitNew<Identity: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartransaction: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMSMQTransaction2_Impl::InitNew(this, core::mem::transmute(&vartransaction)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransaction2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMSMQTransaction_Vtbl::new::<Identity, OFFSET>(), InitNew: InitNew::<Identity, OFFSET>, Properties: Properties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransaction2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQTransaction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQTransaction2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQTransaction3, IMSMQTransaction3_Vtbl, 0xeba96b13_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQTransaction3 {
    type Target = IMSMQTransaction2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQTransaction3, windows_core::IUnknown, super::Com::IDispatch, IMSMQTransaction, IMSMQTransaction2);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction3 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ITransaction(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ITransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction3_Vtbl {
    pub base__: IMSMQTransaction2_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ITransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ITransaction: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransaction3_Impl: IMSMQTransaction2_Impl {
    fn ITransaction(&self) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQTransaction3_Vtbl {
    pub const fn new<Identity: IMSMQTransaction3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ITransaction<Identity: IMSMQTransaction3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaritransaction: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransaction3_Impl::ITransaction(this) {
                    Ok(ok__) => {
                        pvaritransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMSMQTransaction2_Vtbl::new::<Identity, OFFSET>(), ITransaction: ITransaction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransaction3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQTransaction as windows_core::Interface>::IID || iid == &<IMSMQTransaction2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQTransaction3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQTransactionDispenser, IMSMQTransactionDispenser_Vtbl, 0xd7d6e083_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQTransactionDispenser {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQTransactionDispenser, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser {
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransactionDispenser_Impl: super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQTransactionDispenser_Vtbl {
    pub const fn new<Identity: IMSMQTransactionDispenser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginTransaction<Identity: IMSMQTransactionDispenser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransactionDispenser_Impl::BeginTransaction(this) {
                    Ok(ok__) => {
                        ptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQTransactionDispenser {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQTransactionDispenser2, IMSMQTransactionDispenser2_Vtbl, 0xeba96b11_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQTransactionDispenser2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQTransactionDispenser2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser2 {
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransactionDispenser2_Impl: super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQTransactionDispenser2_Vtbl {
    pub const fn new<Identity: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginTransaction<Identity: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransactionDispenser2_Impl::BeginTransaction(this) {
                    Ok(ok__) => {
                        ptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransactionDispenser2_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQTransactionDispenser2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMSMQTransactionDispenser3, IMSMQTransactionDispenser3_Vtbl, 0xeba96b15_2168_11d3_898c_00e02c074f6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMSMQTransactionDispenser3 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMSMQTransactionDispenser3, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser3 {
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransactionDispenser3_Impl: super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMSMQTransactionDispenser3_Vtbl {
    pub const fn new<Identity: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginTransaction<Identity: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransactionDispenser3_Impl::BeginTransaction(this) {
                    Ok(ok__) => {
                        ptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMSMQTransactionDispenser3_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppcolproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMSMQTransactionDispenser3 {}
pub const LONG_LIVED: u32 = 4294967294u32;
pub const MACHINE_ACTION_CONNECT: windows_core::PCWSTR = windows_core::w!("CONNECT");
pub const MACHINE_ACTION_DISCONNECT: windows_core::PCWSTR = windows_core::w!("DISCONNECT");
pub const MACHINE_ACTION_TIDY: windows_core::PCWSTR = windows_core::w!("TIDY");
pub const MGMT_QUEUE_CORRECT_TYPE: windows_core::PCWSTR = windows_core::w!("YES");
pub const MGMT_QUEUE_FOREIGN_TYPE: windows_core::PCWSTR = windows_core::w!("YES");
pub const MGMT_QUEUE_INCORRECT_TYPE: windows_core::PCWSTR = windows_core::w!("NO");
pub const MGMT_QUEUE_LOCAL_LOCATION: windows_core::PCWSTR = windows_core::w!("LOCAL");
pub const MGMT_QUEUE_NOT_FOREIGN_TYPE: windows_core::PCWSTR = windows_core::w!("NO");
pub const MGMT_QUEUE_NOT_TRANSACTIONAL_TYPE: windows_core::PCWSTR = windows_core::w!("NO");
pub const MGMT_QUEUE_REMOTE_LOCATION: windows_core::PCWSTR = windows_core::w!("REMOTE");
pub const MGMT_QUEUE_STATE_CONNECTED: windows_core::PCWSTR = windows_core::w!("CONNECTED");
pub const MGMT_QUEUE_STATE_DISCONNECTED: windows_core::PCWSTR = windows_core::w!("DISCONNECTED");
pub const MGMT_QUEUE_STATE_DISCONNECTING: windows_core::PCWSTR = windows_core::w!("DISCONNECTING");
pub const MGMT_QUEUE_STATE_LOCAL: windows_core::PCWSTR = windows_core::w!("LOCAL CONNECTION");
pub const MGMT_QUEUE_STATE_LOCKED: windows_core::PCWSTR = windows_core::w!("LOCKED");
pub const MGMT_QUEUE_STATE_NEED_VALIDATE: windows_core::PCWSTR = windows_core::w!("NEED VALIDATION");
pub const MGMT_QUEUE_STATE_NONACTIVE: windows_core::PCWSTR = windows_core::w!("INACTIVE");
pub const MGMT_QUEUE_STATE_ONHOLD: windows_core::PCWSTR = windows_core::w!("ONHOLD");
pub const MGMT_QUEUE_STATE_WAITING: windows_core::PCWSTR = windows_core::w!("WAITING");
pub const MGMT_QUEUE_TRANSACTIONAL_TYPE: windows_core::PCWSTR = windows_core::w!("YES");
pub const MGMT_QUEUE_TYPE_CONNECTOR: windows_core::PCWSTR = windows_core::w!("CONNECTOR");
pub const MGMT_QUEUE_TYPE_MACHINE: windows_core::PCWSTR = windows_core::w!("MACHINE");
pub const MGMT_QUEUE_TYPE_MULTICAST: windows_core::PCWSTR = windows_core::w!("MULTICAST");
pub const MGMT_QUEUE_TYPE_PRIVATE: windows_core::PCWSTR = windows_core::w!("PRIVATE");
pub const MGMT_QUEUE_TYPE_PUBLIC: windows_core::PCWSTR = windows_core::w!("PUBLIC");
pub const MGMT_QUEUE_UNKNOWN_TYPE: windows_core::PCWSTR = windows_core::w!("UNKNOWN");
pub const MO_MACHINE_TOKEN: windows_core::PCWSTR = windows_core::w!("MACHINE");
pub const MO_QUEUE_TOKEN: windows_core::PCWSTR = windows_core::w!("QUEUE");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQACCESS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQAUTHENTICATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQCALG(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQCERT_REGISTER(pub i32);
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = MQCERT_REGISTER(1i32);
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = MQCERT_REGISTER(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQCOLUMNSET {
    pub cCol: u32,
    pub aCol: *mut u32,
}
impl Default for MQCOLUMNSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MQCONN_BIND_SOCKET_FAILURE: MQConnectionState = MQConnectionState(-2147483645i32);
pub const MQCONN_CONNECT_SOCKET_FAILURE: MQConnectionState = MQConnectionState(-2147483644i32);
pub const MQCONN_CREATE_SOCKET_FAILURE: MQConnectionState = MQConnectionState(-2147483646i32);
pub const MQCONN_ESTABLISH_PACKET_RECEIVED: MQConnectionState = MQConnectionState(1i32);
pub const MQCONN_INVALID_SERVER_CERT: MQConnectionState = MQConnectionState(-2147483639i32);
pub const MQCONN_LIMIT_REACHED: MQConnectionState = MQConnectionState(-2147483638i32);
pub const MQCONN_NAME_RESOLUTION_FAILURE: MQConnectionState = MQConnectionState(-2147483640i32);
pub const MQCONN_NOFAILURE: MQConnectionState = MQConnectionState(0i32);
pub const MQCONN_NOT_READY: MQConnectionState = MQConnectionState(-2147483641i32);
pub const MQCONN_OUT_OF_MEMORY: MQConnectionState = MQConnectionState(-2147483635i32);
pub const MQCONN_PING_FAILURE: MQConnectionState = MQConnectionState(-2147483647i32);
pub const MQCONN_READY: MQConnectionState = MQConnectionState(2i32);
pub const MQCONN_REFUSED_BY_OTHER_SIDE: MQConnectionState = MQConnectionState(-2147483637i32);
pub const MQCONN_ROUTING_FAILURE: MQConnectionState = MQConnectionState(-2147483636i32);
pub const MQCONN_SEND_FAILURE: MQConnectionState = MQConnectionState(-2147483642i32);
pub const MQCONN_TCP_NOT_ENABLED: MQConnectionState = MQConnectionState(-2147483643i32);
pub const MQCONN_UNKNOWN_FAILURE: MQConnectionState = MQConnectionState(-2147483648i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQConnectionState(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQDEFAULT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQERROR(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQJOURNAL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMAX(pub i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQMGMTPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQMGMTPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGACKNOWLEDGEMENT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGAUTHENTICATION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGAUTHLEVEL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGCLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGCURSOR(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGDELIVERY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGIDSIZE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGJOURNAL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGMAX(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGPRIVLEVEL(pub i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQMSGPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQMSGPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGSENDERIDTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGTRACE(pub i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(5i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(14i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(12i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(8i32);
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(0i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(1i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(2i32);
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(5i32);
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(9i32);
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(0i32);
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(1i32);
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(0i32);
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(8i32);
pub const MQMSG_CALG_DES: MQCALG = MQCALG(26113i32);
pub const MQMSG_CALG_DSS_SIGN: MQCALG = MQCALG(8704i32);
pub const MQMSG_CALG_MAC: MQCALG = MQCALG(32773i32);
pub const MQMSG_CALG_MD2: MQCALG = MQCALG(32769i32);
pub const MQMSG_CALG_MD4: MQCALG = MQCALG(32770i32);
pub const MQMSG_CALG_MD5: MQCALG = MQCALG(32771i32);
pub const MQMSG_CALG_RC2: MQCALG = MQCALG(26114i32);
pub const MQMSG_CALG_RC4: MQCALG = MQCALG(26625i32);
pub const MQMSG_CALG_RSA_KEYX: MQCALG = MQCALG(41984i32);
pub const MQMSG_CALG_RSA_SIGN: MQCALG = MQCALG(9216i32);
pub const MQMSG_CALG_SEAL: MQCALG = MQCALG(26626i32);
pub const MQMSG_CALG_SHA: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_SHA1: MQCALG = MQCALG(32772i32);
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = MQMSGCLASS(2i32);
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = MQMSGCLASS(16384i32);
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = MQMSGCLASS(32772i32);
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = MQMSGCLASS(32768i32);
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = MQMSGCLASS(32775i32);
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = MQMSGCLASS(32774i32);
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = MQMSGCLASS(32776i32);
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = MQMSGCLASS(32773i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = MQMSGCLASS(32778i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = MQMSGCLASS(32777i32);
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = MQMSGCLASS(32769i32);
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = MQMSGCLASS(49152i32);
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = MQMSGCLASS(32771i32);
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = MQMSGCLASS(49153i32);
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(32770i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(49154i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = MQMSGCLASS(49155i32);
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = MQMSGCLASS(32780i32);
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = MQMSGCLASS(32779i32);
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = MQMSGCLASS(0i32);
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = MQMSGCLASS(1i32);
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_CURRENT: MQMSGCURSOR = MQMSGCURSOR(1i32);
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = MQMSGJOURNAL(1i32);
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = MQMSGDELIVERY(0i32);
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = MQMSGDELIVERY(1i32);
pub const MQMSG_FIRST: MQMSGCURSOR = MQMSGCURSOR(0i32);
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
pub const MQMSG_JOURNAL: MQMSGJOURNAL = MQMSGJOURNAL(2i32);
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = MQMSGJOURNAL(0i32);
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_NEXT: MQMSGCURSOR = MQMSGCURSOR(2i32);
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(1i32);
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(3i32);
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(0i32);
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(0i32);
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(1i32);
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = MQMSGTRACE(1i32);
pub const MQMSG_TRACE_NONE: MQMSGTRACE = MQMSGTRACE(0i32);
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQPRIORITY(pub i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQPRIVATEPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQPRIVATEPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQPRIVLEVEL(pub i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct MQPROPERTYRESTRICTION {
    pub rel: u32,
    pub prop: u32,
    pub prval: super::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Clone for MQPROPERTYRESTRICTION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQPROPERTYRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQQMPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQQMPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQQUEUEACCESSMASK(pub u32);
impl MQQUEUEACCESSMASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MQQUEUEACCESSMASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MQQUEUEACCESSMASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MQQUEUEACCESSMASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MQQUEUEACCESSMASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MQQUEUEACCESSMASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQQUEUEPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQQUEUEPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQRESTRICTION {
    pub cRes: u32,
    pub paPropRes: *mut MQPROPERTYRESTRICTION,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(262144u32);
pub const MQSEC_DELETE_JOURNAL_MESSAGE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(8u32);
pub const MQSEC_DELETE_MESSAGE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(1u32);
pub const MQSEC_DELETE_QUEUE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(65536u32);
pub const MQSEC_GET_QUEUE_PERMISSIONS: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(131072u32);
pub const MQSEC_GET_QUEUE_PROPERTIES: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(32u32);
pub const MQSEC_PEEK_MESSAGE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(2u32);
pub const MQSEC_QUEUE_GENERIC_ALL: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(983103u32);
pub const MQSEC_QUEUE_GENERIC_EXECUTE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(0u32);
pub const MQSEC_QUEUE_GENERIC_READ: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(131115u32);
pub const MQSEC_QUEUE_GENERIC_WRITE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(131108u32);
pub const MQSEC_RECEIVE_JOURNAL_MESSAGE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(10u32);
pub const MQSEC_RECEIVE_MESSAGE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(3u32);
pub const MQSEC_SET_QUEUE_PROPERTIES: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(16u32);
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(524288u32);
pub const MQSEC_WRITE_MESSAGE: MQQUEUEACCESSMASK = MQQUEUEACCESSMASK(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQSHARE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MQSORTKEY {
    pub propColumn: u32,
    pub dwOrder: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MQSORTSET {
    pub cCol: u32,
    pub aCol: *mut MQSORTKEY,
}
impl Default for MQSORTSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQTRANSACTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQTRANSACTIONAL(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQWARNING(pub i32);
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648u32;
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649u32;
pub const MQ_ACTION_RECEIVE: u32 = 0u32;
pub const MQ_ADMIN_ACCESS: MQACCESS = MQACCESS(128i32);
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = MQAUTHENTICATE(1i32);
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = MQAUTHENTICATE(0i32);
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = MQERROR(-1072824216i32);
pub const MQ_DENY_NONE: MQSHARE = MQSHARE(0i32);
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = MQSHARE(1i32);
pub const MQ_ERROR: MQERROR = MQERROR(-1072824319i32);
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = MQERROR(-1072824283i32);
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = MQERROR(-1072824267i32);
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = MQERROR(-1072824174i32);
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = MQERROR(-1072824294i32);
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = MQERROR(-1072824191i32);
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = MQERROR(-1072824201i32);
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824171i32);
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = MQERROR(-1072824189i32);
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = MQERROR(-1072824194i32);
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = MQERROR(-1072824206i32);
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = MQERROR(-1072824193i32);
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = MQERROR(-1072824284i32);
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = MQERROR(-1072824202i32);
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = MQERROR(-1072824187i32);
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = MQERROR(-1072824186i32);
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = MQERROR(-1072824205i32);
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = MQERROR(-1072824212i32);
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = MQERROR(-1072824192i32);
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824170i32);
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = MQERROR(-1072824183i32);
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = MQERROR(-1072824211i32);
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = MQERROR(-1072824269i32);
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = MQERROR(-1072824275i32);
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = MQERROR(-1072824271i32);
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = MQERROR(-1072824272i32);
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = MQERROR(-1072824265i32);
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = MQERROR(-1072824266i32);
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = MQERROR(-1072824248i32);
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = MQERROR(-1072824217i32);
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = MQERROR(-1072824177i32);
pub const MQ_ERROR_DS_ERROR: MQERROR = MQERROR(-1072824253i32);
pub const MQ_ERROR_DS_IS_FULL: MQERROR = MQERROR(-1072824254i32);
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = MQERROR(-1072824176i32);
pub const MQ_ERROR_DTC_CONNECT: MQERROR = MQERROR(-1072824244i32);
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = MQERROR(-1072824213i32);
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = MQERROR(-1072824190i32);
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824289i32);
pub const MQ_ERROR_GC_NEEDED: MQERROR = MQERROR(-1072824178i32);
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = MQERROR(-1072824200i32);
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = MQERROR(-1072824229i32);
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = MQERROR(-1072824292i32);
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = MQERROR(-1072824207i32);
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = MQERROR(-1072824290i32);
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = MQERROR(-1072824264i32);
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = MQERROR(-1072824197i32);
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = MQERROR(-1072824255i32);
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = MQERROR(-1072824259i32);
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = MQERROR(-1072824220i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = MQERROR(-1072824261i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = MQERROR(-1072824296i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = MQERROR(-1072824295i32);
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = MQERROR(-1072824263i32);
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = MQERROR(-1072824300i32);
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = MQERROR(-1072824262i32);
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = MQERROR(-1072824260i32);
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = MQERROR(-1072824287i32);
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = MQERROR(-1072824304i32);
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = MQERROR(-1072824228i32);
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = MQERROR(-1072824303i32);
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = MQERROR(-1072824257i32);
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = MQERROR(-1072824281i32);
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = MQERROR(-1072824274i32);
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = MQERROR(-1072824276i32);
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = MQERROR(-1072824313i32);
pub const MQ_ERROR_INVALID_OWNER: MQERROR = MQERROR(-1072824252i32);
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = MQERROR(-1072824314i32);
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = MQERROR(-1072824293i32);
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824226i32);
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = MQERROR(-1072824227i32);
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = MQERROR(-1072824256i32);
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = MQERROR(-1072824307i32);
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = MQERROR(-1072824291i32);
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: windows_core::HRESULT = windows_core::HRESULT(0xC00E009C_u32 as _);
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: windows_core::HRESULT = windows_core::HRESULT(0xC00E009B_u32 as _);
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = MQERROR(-1072824184i32);
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = MQERROR(-1072824278i32);
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = MQERROR(-1072824235i32);
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = MQERROR(-1072824224i32);
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = MQERROR(-1072824225i32);
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = MQERROR(-1072824179i32);
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = MQERROR(-1072824180i32);
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = MQERROR(-1072824182i32);
pub const MQ_ERROR_NO_DS: MQERROR = MQERROR(-1072824301i32);
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = MQERROR(-1072824204i32);
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = MQERROR(-1072824196i32);
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = MQERROR(-1072824273i32);
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = MQERROR(-1072824188i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = MQERROR(-1072824203i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = MQERROR(-1072824195i32);
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = MQERROR(-1072824247i32);
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = MQERROR(-1072824246i32);
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = MQERROR(-1072824312i32);
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = MQERROR(-1072824181i32);
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = MQERROR(-1072824282i32);
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = MQERROR(-1072824185i32);
pub const MQ_ERROR_PROPERTY: MQERROR = MQERROR(-1072824318i32);
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = MQERROR(-1072824258i32);
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824221i32);
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = MQERROR(-1072824198i32);
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = MQERROR(-1072824199i32);
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = MQERROR(-1072824230i32);
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = MQERROR(-1072824315i32);
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = MQERROR(-1072824316i32);
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824245i32);
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = MQERROR(-1072824317i32);
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824175i32);
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824210i32);
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824215i32);
pub const MQ_ERROR_RESOLVE_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0099_u32 as _);
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824250i32);
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = MQERROR(-1072824285i32);
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824286i32);
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824277i32);
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824309i32);
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = MQERROR(-1072824311i32);
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824222i32);
pub const MQ_ERROR_STALE_HANDLE: MQERROR = MQERROR(-1072824234i32);
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824223i32);
pub const MQ_ERROR_TOO_MANY_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0xC00E009A_u32 as _);
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = MQERROR(-1072824232i32);
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = MQERROR(-1072824242i32);
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = MQERROR(-1072824239i32);
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = MQERROR(-1072824240i32);
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = MQERROR(-1072824172i32);
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = MQERROR(-1072824251i32);
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = MQERROR(-1072824173i32);
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = MQERROR(-1072824288i32);
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = MQERROR(-1072824214i32);
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824280i32);
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = MQERROR(-1072824218i32);
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = MQERROR(-1072824219i32);
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = MQWARNING(1074659333i32);
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = MQWARNING(1074659337i32);
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = MQWARNING(1074659330i32);
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = MQWARNING(1074659338i32);
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = MQWARNING(1074659334i32);
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = MQWARNING(1074659339i32);
pub const MQ_INFORMATION_PROPERTY: MQWARNING = MQWARNING(1074659329i32);
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = MQWARNING(1074659331i32);
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = MQWARNING(1074659332i32);
pub const MQ_JOURNAL: MQJOURNAL = MQJOURNAL(1i32);
pub const MQ_JOURNAL_NONE: MQJOURNAL = MQJOURNAL(0i32);
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840u32;
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844u32;
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848u32;
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841u32;
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842u32;
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112u32;
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856u32;
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860u32;
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864u32;
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857u32;
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858u32;
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = MQMSGMAX(249i32);
pub const MQ_MAX_PRIORITY: MQPRIORITY = MQPRIORITY(7i32);
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = MQMAX(124i32);
pub const MQ_MAX_Q_NAME_LEN: MQMAX = MQMAX(124i32);
pub const MQ_MIN_PRIORITY: MQPRIORITY = MQPRIORITY(0i32);
pub const MQ_MOVE_ACCESS: u32 = 4u32;
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = MQTRANSACTION(1i32);
pub const MQ_NO_TRANSACTION: MQTRANSACTION = MQTRANSACTION(0i32);
pub const MQ_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const MQ_PEEK_ACCESS: MQACCESS = MQACCESS(32i32);
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = MQPRIVLEVEL(2i32);
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = MQPRIVLEVEL(0i32);
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = MQPRIVLEVEL(1i32);
pub const MQ_QTYPE_REPORT: windows_core::GUID = windows_core::GUID::from_u128(0x55ee8f32_cce9_11cf_b108_0020afd61ce9);
pub const MQ_QTYPE_TEST: windows_core::GUID = windows_core::GUID::from_u128(0x55ee8f33_cce9_11cf_b108_0020afd61ce9);
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = QUEUE_STATE(6i32);
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = QUEUE_STATE(1i32);
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = QUEUE_STATE(7i32);
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = QUEUE_STATE(0i32);
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = QUEUE_STATE(8i32);
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = QUEUE_STATE(3i32);
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = QUEUE_STATE(5i32);
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = QUEUE_STATE(4i32);
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = QUEUE_STATE(2i32);
pub const MQ_RECEIVE_ACCESS: MQACCESS = MQACCESS(1i32);
pub const MQ_SEND_ACCESS: MQACCESS = MQACCESS(2i32);
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = MQTRANSACTION(3i32);
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(0i32);
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(1i32);
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = FOREIGN_STATUS(2i32);
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = MQTRANSACTIONAL(1i32);
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = MQTRANSACTIONAL(0i32);
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = QUEUE_TYPE(3i32);
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = QUEUE_TYPE(2i32);
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = QUEUE_TYPE(4i32);
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = QUEUE_TYPE(1i32);
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = QUEUE_TYPE(0i32);
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = XACT_STATUS(1i32);
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = XACT_STATUS(2i32);
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = XACT_STATUS(0i32);
pub const MQ_XA_TRANSACTION: MQTRANSACTION = MQTRANSACTION(2i32);
pub const MSMQApplication: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e086_dccd_11d0_aa4b_0060970debae);
pub const MSMQCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf72b9031_2f0c_43e8_924e_e6052cdc493f);
pub const MSMQCoordinatedTransactionDispenser: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e082_dccd_11d0_aa4b_0060970debae);
pub const MSMQDestination: windows_core::GUID = windows_core::GUID::from_u128(0xeba96b18_2168_11d3_898c_00e02c074f6b);
pub const MSMQEvent: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e07a_dccd_11d0_aa4b_0060970debae);
pub const MSMQManagement: windows_core::GUID = windows_core::GUID::from_u128(0x39ce96fe_f4c5_4484_a143_4c2d5d324229);
pub const MSMQMessage: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e075_dccd_11d0_aa4b_0060970debae);
pub const MSMQOutgoingQueueManagement: windows_core::GUID = windows_core::GUID::from_u128(0x0188401c_247a_4fed_99c6_bf14119d7055);
pub const MSMQQuery: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e073_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueue: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e079_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfo: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e07c_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfos: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e07e_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueManagement: windows_core::GUID = windows_core::GUID::from_u128(0x33b6d07e_f27d_42fa_b2d7_bf82e11e9374);
pub const MSMQTransaction: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e080_dccd_11d0_aa4b_0060970debae);
pub const MSMQTransactionDispenser: windows_core::GUID = windows_core::GUID::from_u128(0xd7d6e084_dccd_11d0_aa4b_0060970debae);
pub const MSMQ_CONNECTED: windows_core::PCWSTR = windows_core::w!("CONNECTED");
pub const MSMQ_DISCONNECTED: windows_core::PCWSTR = windows_core::w!("DISCONNECTED");
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_IO", feature = "Win32_System_Variant"))]
pub type PMQRECEIVECALLBACK = Option<unsafe extern "system" fn(hrstatus: windows_core::HRESULT, hsource: isize, dwtimeout: u32, dwaction: u32, pmessageprops: *mut MQMSGPROPS, lpoverlapped: *mut super::IO::OVERLAPPED, hcursor: super::super::Foundation::HANDLE)>;
pub const PREQ: u32 = 4u32;
pub const PRGE: u32 = 3u32;
pub const PRGT: u32 = 2u32;
pub const PRLE: u32 = 1u32;
pub const PRLT: u32 = 0u32;
pub const PRNE: u32 = 5u32;
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1u32;
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0u32;
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6u32;
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4u32;
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3u32;
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2u32;
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5u32;
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25u32;
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17u32;
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21u32;
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24u32;
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6u32;
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4u32;
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7u32;
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12u32;
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1u32;
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27u32;
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3u32;
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5u32;
pub const PROPID_M_ABORT_COUNT: u32 = 69u32;
pub const PROPID_M_ACKNOWLEDGE: u32 = 6u32;
pub const PROPID_M_ADMIN_QUEUE: u32 = 17u32;
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18u32;
pub const PROPID_M_APPSPECIFIC: u32 = 8u32;
pub const PROPID_M_ARRIVEDTIME: u32 = 32u32;
pub const PROPID_M_AUTHENTICATED: u32 = 25u32;
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53u32;
pub const PROPID_M_AUTH_LEVEL: u32 = 24u32;
pub const PROPID_M_BASE: u32 = 0u32;
pub const PROPID_M_BODY: u32 = 9u32;
pub const PROPID_M_BODY_SIZE: u32 = 10u32;
pub const PROPID_M_BODY_TYPE: u32 = 42u32;
pub const PROPID_M_CLASS: u32 = 1u32;
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63u32;
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64u32;
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38u32;
pub const PROPID_M_CORRELATIONID: u32 = 3u32;
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20u32;
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67u32;
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68u32;
pub const PROPID_M_DELIVERY: u32 = 5u32;
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58u32;
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59u32;
pub const PROPID_M_DEST_QUEUE: u32 = 33u32;
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34u32;
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43u32;
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44u32;
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27u32;
pub const PROPID_M_EXTENSION: u32 = 35u32;
pub const PROPID_M_EXTENSION_LEN: u32 = 36u32;
pub const PROPID_M_FIRST_IN_XACT: u32 = 50u32;
pub const PROPID_M_HASH_ALG: u32 = 26u32;
pub const PROPID_M_JOURNAL: u32 = 7u32;
pub const PROPID_M_LABEL: u32 = 11u32;
pub const PROPID_M_LABEL_LEN: u32 = 12u32;
pub const PROPID_M_LAST_IN_XACT: u32 = 51u32;
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75u32;
pub const PROPID_M_LOOKUPID: u32 = 60u32;
pub const PROPID_M_MOVE_COUNT: u32 = 70u32;
pub const PROPID_M_MSGID: u32 = 2u32;
pub const PROPID_M_MSGID_SIZE: u32 = 20u32;
pub const PROPID_M_PRIORITY: u32 = 4u32;
pub const PROPID_M_PRIV_LEVEL: u32 = 23u32;
pub const PROPID_M_PROV_NAME: u32 = 48u32;
pub const PROPID_M_PROV_NAME_LEN: u32 = 49u32;
pub const PROPID_M_PROV_TYPE: u32 = 47u32;
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54u32;
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55u32;
pub const PROPID_M_RESP_QUEUE: u32 = 15u32;
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16u32;
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37u32;
pub const PROPID_M_SENDERID: u32 = 20u32;
pub const PROPID_M_SENDERID_LEN: u32 = 21u32;
pub const PROPID_M_SENDERID_TYPE: u32 = 22u32;
pub const PROPID_M_SENDER_CERT: u32 = 28u32;
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29u32;
pub const PROPID_M_SENTTIME: u32 = 31u32;
pub const PROPID_M_SIGNATURE: u32 = 45u32;
pub const PROPID_M_SIGNATURE_LEN: u32 = 46u32;
pub const PROPID_M_SOAP_BODY: u32 = 66u32;
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61u32;
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62u32;
pub const PROPID_M_SOAP_HEADER: u32 = 65u32;
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30u32;
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14u32;
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13u32;
pub const PROPID_M_TRACE: u32 = 41u32;
pub const PROPID_M_VERSION: u32 = 19u32;
pub const PROPID_M_XACTID: u32 = 52u32;
pub const PROPID_M_XACTID_SIZE: u32 = 20u32;
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39u32;
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40u32;
pub const PROPID_PC_BASE: u32 = 5800u32;
pub const PROPID_PC_DS_ENABLED: u32 = 5802u32;
pub const PROPID_PC_VERSION: u32 = 5801u32;
pub const PROPID_QM_BASE: u32 = 200u32;
pub const PROPID_QM_CONNECTION: u32 = 204u32;
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205u32;
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244u32;
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231u32;
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232u32;
pub const PROPID_QM_MACHINE_ID: u32 = 202u32;
pub const PROPID_QM_PATHNAME: u32 = 203u32;
pub const PROPID_QM_PATHNAME_DNS: u32 = 233u32;
pub const PROPID_QM_SITE_ID: u32 = 201u32;
pub const PROPID_Q_ADS_PATH: u32 = 126u32;
pub const PROPID_Q_AUTHENTICATE: u32 = 111u32;
pub const PROPID_Q_BASE: u32 = 100u32;
pub const PROPID_Q_BASEPRIORITY: u32 = 106u32;
pub const PROPID_Q_CREATE_TIME: u32 = 109u32;
pub const PROPID_Q_INSTANCE: u32 = 101u32;
pub const PROPID_Q_JOURNAL: u32 = 104u32;
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107u32;
pub const PROPID_Q_LABEL: u32 = 108u32;
pub const PROPID_Q_MODIFY_TIME: u32 = 110u32;
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125u32;
pub const PROPID_Q_PATHNAME: u32 = 103u32;
pub const PROPID_Q_PATHNAME_DNS: u32 = 124u32;
pub const PROPID_Q_PRIV_LEVEL: u32 = 112u32;
pub const PROPID_Q_QUOTA: u32 = 105u32;
pub const PROPID_Q_TRANSACTION: u32 = 113u32;
pub const PROPID_Q_TYPE: u32 = 102u32;
pub const QUERY_SORTASCEND: u32 = 0u32;
pub const QUERY_SORTDESCEND: u32 = 1u32;
pub const QUEUE_ACTION_EOD_RESEND: windows_core::PCWSTR = windows_core::w!("EOD_RESEND");
pub const QUEUE_ACTION_PAUSE: windows_core::PCWSTR = windows_core::w!("PAUSE");
pub const QUEUE_ACTION_RESUME: windows_core::PCWSTR = windows_core::w!("RESUME");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUEUE_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUEUE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RELOPS(pub i32);
pub const REL_EQ: RELOPS = RELOPS(1i32);
pub const REL_GE: RELOPS = RELOPS(6i32);
pub const REL_GT: RELOPS = RELOPS(4i32);
pub const REL_LE: RELOPS = RELOPS(5i32);
pub const REL_LT: RELOPS = RELOPS(3i32);
pub const REL_NEQ: RELOPS = RELOPS(2i32);
pub const REL_NOP: RELOPS = RELOPS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEQUENCE_INFO {
    pub SeqID: i64,
    pub SeqNo: u32,
    pub PrevNo: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XACT_STATUS(pub i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_DMSMQEventEvents, _DMSMQEventEvents_Vtbl, 0xd7d6e078_dccd_11d0_aa4b_0060970debae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _DMSMQEventEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_DMSMQEventEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _DMSMQEventEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _DMSMQEventEvents_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _DMSMQEventEvents_Vtbl {
    pub const fn new<Identity: _DMSMQEventEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_DMSMQEventEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _DMSMQEventEvents {}
