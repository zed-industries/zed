#[inline]
pub unsafe fn MQADsPathToFormatName<P0>(lpwcsadspath: P0, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQADsPathToFormatName(lpwcsadspath : windows_core::PCWSTR, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    MQADsPathToFormatName(lpwcsadspath.param().abi(), core::mem::transmute(lpwcsformatname), lpdwformatnamelength).ok()
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
#[inline]
pub unsafe fn MQBeginTransaction() -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction> {
    windows_targets::link!("mqrt.dll" "system" fn MQBeginTransaction(pptransaction : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MQBeginTransaction(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn MQCloseCursor<P0>(hcursor: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQCloseCursor(hcursor : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    MQCloseCursor(hcursor.param().abi()).ok()
}
#[inline]
pub unsafe fn MQCloseQueue(hqueue: isize) -> windows_core::Result<()> {
    windows_targets::link!("mqrt.dll" "system" fn MQCloseQueue(hqueue : isize) -> windows_core::HRESULT);
    MQCloseQueue(hqueue).ok()
}
#[inline]
pub unsafe fn MQCreateCursor(hqueue: isize) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("mqrt.dll" "system" fn MQCreateCursor(hqueue : isize, phcursor : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MQCreateCursor(hqueue, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn MQCreateQueue<P0>(psecuritydescriptor: P0, pqueueprops: *mut MQQUEUEPROPS, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQCreateQueue(psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, pqueueprops : *mut MQQUEUEPROPS, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    MQCreateQueue(psecuritydescriptor.param().abi(), pqueueprops, core::mem::transmute(lpwcsformatname), lpdwformatnamelength).ok()
}
#[inline]
pub unsafe fn MQDeleteQueue<P0>(lpwcsformatname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQDeleteQueue(lpwcsformatname : windows_core::PCWSTR) -> windows_core::HRESULT);
    MQDeleteQueue(lpwcsformatname.param().abi()).ok()
}
#[inline]
pub unsafe fn MQFreeMemory(pvmemory: *const core::ffi::c_void) {
    windows_targets::link!("mqrt.dll" "system" fn MQFreeMemory(pvmemory : *const core::ffi::c_void));
    MQFreeMemory(pvmemory)
}
#[inline]
pub unsafe fn MQFreeSecurityContext<P0>(hsecuritycontext: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQFreeSecurityContext(hsecuritycontext : super::super::Foundation:: HANDLE));
    MQFreeSecurityContext(hsecuritycontext.param().abi())
}
#[inline]
pub unsafe fn MQGetMachineProperties<P0>(lpwcsmachinename: P0, pguidmachineid: Option<*const windows_core::GUID>, pqmprops: *mut MQQMPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQGetMachineProperties(lpwcsmachinename : windows_core::PCWSTR, pguidmachineid : *const windows_core::GUID, pqmprops : *mut MQQMPROPS) -> windows_core::HRESULT);
    MQGetMachineProperties(lpwcsmachinename.param().abi(), core::mem::transmute(pguidmachineid.unwrap_or(std::ptr::null())), pqmprops).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn MQGetOverlappedResult(lpoverlapped: *const super::IO::OVERLAPPED) -> windows_core::Result<()> {
    windows_targets::link!("mqrt.dll" "system" fn MQGetOverlappedResult(lpoverlapped : *const super::IO:: OVERLAPPED) -> windows_core::HRESULT);
    MQGetOverlappedResult(lpoverlapped).ok()
}
#[inline]
pub unsafe fn MQGetPrivateComputerInformation<P0>(lpwcscomputername: P0, pprivateprops: *mut MQPRIVATEPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQGetPrivateComputerInformation(lpwcscomputername : windows_core::PCWSTR, pprivateprops : *mut MQPRIVATEPROPS) -> windows_core::HRESULT);
    MQGetPrivateComputerInformation(lpwcscomputername.param().abi(), pprivateprops).ok()
}
#[inline]
pub unsafe fn MQGetQueueProperties<P0>(lpwcsformatname: P0, pqueueprops: *mut MQQUEUEPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQGetQueueProperties(lpwcsformatname : windows_core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_core::HRESULT);
    MQGetQueueProperties(lpwcsformatname.param().abi(), pqueueprops).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn MQGetQueueSecurity<P0>(lpwcsformatname: P0, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQGetQueueSecurity(lpwcsformatname : windows_core::PCWSTR, requestedinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::HRESULT);
    MQGetQueueSecurity(lpwcsformatname.param().abi(), requestedinformation, psecuritydescriptor, nlength, lpnlengthneeded).ok()
}
#[inline]
pub unsafe fn MQGetSecurityContext(lpcertbuffer: Option<*const core::ffi::c_void>, dwcertbufferlength: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("mqrt.dll" "system" fn MQGetSecurityContext(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MQGetSecurityContext(core::mem::transmute(lpcertbuffer.unwrap_or(std::ptr::null())), dwcertbufferlength, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn MQGetSecurityContextEx(lpcertbuffer: Option<*const core::ffi::c_void>, dwcertbufferlength: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("mqrt.dll" "system" fn MQGetSecurityContextEx(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MQGetSecurityContextEx(core::mem::transmute(lpcertbuffer.unwrap_or(std::ptr::null())), dwcertbufferlength, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn MQHandleToFormatName(hqueue: isize, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mqrt.dll" "system" fn MQHandleToFormatName(hqueue : isize, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    MQHandleToFormatName(hqueue, core::mem::transmute(lpwcsformatname), lpdwformatnamelength).ok()
}
#[inline]
pub unsafe fn MQInstanceToFormatName(pguid: *const windows_core::GUID, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mqrt.dll" "system" fn MQInstanceToFormatName(pguid : *const windows_core::GUID, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    MQInstanceToFormatName(pguid, core::mem::transmute(lpwcsformatname), lpdwformatnamelength).ok()
}
#[inline]
pub unsafe fn MQLocateBegin<P0>(lpwcscontext: P0, prestriction: Option<*const MQRESTRICTION>, pcolumns: *const MQCOLUMNSET, psort: *const MQSORTSET) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQLocateBegin(lpwcscontext : windows_core::PCWSTR, prestriction : *const MQRESTRICTION, pcolumns : *const MQCOLUMNSET, psort : *const MQSORTSET, phenum : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MQLocateBegin(lpwcscontext.param().abi(), core::mem::transmute(prestriction.unwrap_or(std::ptr::null())), pcolumns, psort, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn MQLocateEnd<P0>(henum: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQLocateEnd(henum : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    MQLocateEnd(henum.param().abi()).ok()
}
#[inline]
pub unsafe fn MQLocateNext<P0>(henum: P0, pcprops: *mut u32, apropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQLocateNext(henum : super::super::Foundation:: HANDLE, pcprops : *mut u32, apropvar : *mut core::mem::MaybeUninit < windows_core::PROPVARIANT >) -> windows_core::HRESULT);
    MQLocateNext(henum.param().abi(), pcprops, core::mem::transmute(apropvar)).ok()
}
#[inline]
pub unsafe fn MQMarkMessageRejected<P0>(hqueue: P0, ulllookupid: u64) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQMarkMessageRejected(hqueue : super::super::Foundation:: HANDLE, ulllookupid : u64) -> windows_core::HRESULT);
    MQMarkMessageRejected(hqueue.param().abi(), ulllookupid).ok()
}
#[inline]
pub unsafe fn MQMgmtAction<P0, P1, P2>(pcomputername: P0, pobjectname: P1, paction: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQMgmtAction(pcomputername : windows_core::PCWSTR, pobjectname : windows_core::PCWSTR, paction : windows_core::PCWSTR) -> windows_core::HRESULT);
    MQMgmtAction(pcomputername.param().abi(), pobjectname.param().abi(), paction.param().abi()).ok()
}
#[inline]
pub unsafe fn MQMgmtGetInfo<P0, P1>(pcomputername: P0, pobjectname: P1, pmgmtprops: *mut MQMGMTPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQMgmtGetInfo(pcomputername : windows_core::PCWSTR, pobjectname : windows_core::PCWSTR, pmgmtprops : *mut MQMGMTPROPS) -> windows_core::HRESULT);
    MQMgmtGetInfo(pcomputername.param().abi(), pobjectname.param().abi(), pmgmtprops).ok()
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
#[inline]
pub unsafe fn MQMoveMessage<P0>(hsourcequeue: isize, hdestinationqueue: isize, ulllookupid: u64, ptransaction: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQMoveMessage(hsourcequeue : isize, hdestinationqueue : isize, ulllookupid : u64, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    MQMoveMessage(hsourcequeue, hdestinationqueue, ulllookupid, ptransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn MQOpenQueue<P0>(lpwcsformatname: P0, dwaccess: u32, dwsharemode: u32) -> windows_core::Result<isize>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQOpenQueue(lpwcsformatname : windows_core::PCWSTR, dwaccess : u32, dwsharemode : u32, phqueue : *mut isize) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MQOpenQueue(lpwcsformatname.param().abi(), dwaccess, dwsharemode, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn MQPathNameToFormatName<P0>(lpwcspathname: P0, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQPathNameToFormatName(lpwcspathname : windows_core::PCWSTR, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    MQPathNameToFormatName(lpwcspathname.param().abi(), core::mem::transmute(lpwcsformatname), lpdwformatnamelength).ok()
}
#[inline]
pub unsafe fn MQPurgeQueue(hqueue: isize) -> windows_core::Result<()> {
    windows_targets::link!("mqrt.dll" "system" fn MQPurgeQueue(hqueue : isize) -> windows_core::HRESULT);
    MQPurgeQueue(hqueue).ok()
}
#[cfg(all(feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn MQReceiveMessage<P0, P1>(hsource: isize, dwtimeout: u32, dwaction: u32, pmessageprops: Option<*mut MQMSGPROPS>, lpoverlapped: Option<*mut super::IO::OVERLAPPED>, fnreceivecallback: PMQRECEIVECALLBACK, hcursor: P0, ptransaction: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQReceiveMessage(hsource : isize, dwtimeout : u32, dwaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::IO:: OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, hcursor : super::super::Foundation:: HANDLE, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    MQReceiveMessage(hsource, dwtimeout, dwaction, core::mem::transmute(pmessageprops.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut())), fnreceivecallback, hcursor.param().abi(), ptransaction.param().abi()).ok()
}
#[cfg(all(feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn MQReceiveMessageByLookupId<P0>(hsource: isize, ulllookupid: u64, dwlookupaction: u32, pmessageprops: Option<*mut MQMSGPROPS>, lpoverlapped: Option<*mut super::IO::OVERLAPPED>, fnreceivecallback: PMQRECEIVECALLBACK, ptransaction: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQReceiveMessageByLookupId(hsource : isize, ulllookupid : u64, dwlookupaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::IO:: OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    MQReceiveMessageByLookupId(hsource, ulllookupid, dwlookupaction, core::mem::transmute(pmessageprops.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut())), fnreceivecallback, ptransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn MQRegisterCertificate(dwflags: u32, lpcertbuffer: *const core::ffi::c_void, dwcertbufferlength: u32) -> windows_core::Result<()> {
    windows_targets::link!("mqrt.dll" "system" fn MQRegisterCertificate(dwflags : u32, lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32) -> windows_core::HRESULT);
    MQRegisterCertificate(dwflags, lpcertbuffer, dwcertbufferlength).ok()
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
#[inline]
pub unsafe fn MQSendMessage<P0>(hdestinationqueue: isize, pmessageprops: *const MQMSGPROPS, ptransaction: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQSendMessage(hdestinationqueue : isize, pmessageprops : *const MQMSGPROPS, ptransaction : * mut core::ffi::c_void) -> windows_core::HRESULT);
    MQSendMessage(hdestinationqueue, pmessageprops, ptransaction.param().abi()).ok()
}
#[inline]
pub unsafe fn MQSetQueueProperties<P0>(lpwcsformatname: P0, pqueueprops: *mut MQQUEUEPROPS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQSetQueueProperties(lpwcsformatname : windows_core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_core::HRESULT);
    MQSetQueueProperties(lpwcsformatname.param().abi(), pqueueprops).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn MQSetQueueSecurity<P0, P1>(lpwcsformatname: P0, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("mqrt.dll" "system" fn MQSetQueueSecurity(lpwcsformatname : windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_core::HRESULT);
    MQSetQueueSecurity(lpwcsformatname.param().abi(), securityinformation, psecuritydescriptor.param().abi()).ok()
}
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
    pub unsafe fn MachineIdOfMachineName<P0>(&self, machinename: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MachineIdOfMachineName)(windows_core::Interface::as_raw(self), machinename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQApplication_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub MachineIdOfMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn RegisterCertificate(&self, flags: *const windows_core::VARIANT, externalcertificate: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterCertificate)(windows_core::Interface::as_raw(self), core::mem::transmute(flags), core::mem::transmute(externalcertificate)).ok()
    }
    pub unsafe fn MachineNameOfMachineId<P0>(&self, bstrguid: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MachineNameOfMachineId)(windows_core::Interface::as_raw(self), bstrguid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MSMQVersionMajor(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MSMQVersionMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MSMQVersionMinor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MSMQVersionBuild)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQApplication2_Vtbl {
    pub base__: IMSMQApplication_Vtbl,
    pub RegisterCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MachineNameOfMachineId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MSMQVersionMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub MSMQVersionMinor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub MSMQVersionBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsDsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
    pub unsafe fn ActiveQueues(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActiveQueues)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PrivateQueues(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateQueues)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DirectoryServiceServer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DirectoryServiceServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BytesInAllQueues(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BytesInAllQueues)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMachine<P0>(&self, bstrmachine: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMachine)(windows_core::Interface::as_raw(self), bstrmachine.param().abi()).ok()
    }
    pub unsafe fn Machine(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Machine)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Tidy(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Tidy)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQApplication3_Vtbl {
    pub base__: IMSMQApplication2_Vtbl,
    pub ActiveQueues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PrivateQueues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DirectoryServiceServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BytesInAllQueues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetMachine: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Machine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tidy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
    pub unsafe fn Item(&self, index: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IADs(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IADs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_IADs<P0>(&self, piads: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).putref_IADs)(windows_core::Interface::as_raw(self), piads.param().abi()).ok()
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetADsPath<P0>(&self, bstradspath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetADsPath)(windows_core::Interface::as_raw(self), bstradspath.param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPathName<P0>(&self, bstrpathname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), bstrpathname.param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFormatName<P0>(&self, bstrformatname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), bstrformatname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destinations(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Destinations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Destinations<P0>(&self, pdestinations: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).putref_Destinations)(windows_core::Interface::as_raw(self), pdestinations.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IADs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IADs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_IADs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_IADs: usize,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Destinations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destinations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Destinations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Destinations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
impl IMSMQEvent {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQEvent2_Vtbl {
    pub base__: IMSMQEvent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
impl IMSMQEvent3 {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQEvent3_Vtbl {
    pub base__: IMSMQEvent2_Vtbl,
}
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
    pub unsafe fn Init(&self, machine: *const windows_core::VARIANT, pathname: *const windows_core::VARIANT, formatname: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), core::mem::transmute(machine), core::mem::transmute(pathname), core::mem::transmute(formatname)).ok()
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Machine(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Machine)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MessageCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MessageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ForeignStatus(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ForeignStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueueType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionalStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BytesInQueue(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BytesInQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQManagement_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Machine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ForeignStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TransactionalStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BytesInQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Body(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBody<P0>(&self, varbody: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), varbody.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCorrelationId<P0>(&self, varmsgid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), varmsgid.param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok()
    }
    pub unsafe fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSenderCertificate<P0>(&self, varsendercert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), varsendercert.param().abi()).ok()
    }
    pub unsafe fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue>,
    {
        (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v1<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Body(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBody<P0>(&self, varbody: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), varbody.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v1<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCorrelationId<P0>(&self, varmsgid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), varmsgid.param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok()
    }
    pub unsafe fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSenderCertificate<P0>(&self, varsendercert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), varsendercert.param().abi()).ok()
    }
    pub unsafe fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue2>,
    {
        (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Extension(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Extension)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExtension<P0>(&self, varextension: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetExtension)(windows_core::Interface::as_raw(self), varextension.param().abi()).ok()
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectorTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetConnectorTypeGuid<P0>(&self, bstrguidconnectortype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetConnectorTypeGuid)(windows_core::Interface::as_raw(self), bstrguidconnectortype.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DestinationSymmetricKey(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationSymmetricKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDestinationSymmetricKey<P0>(&self, vardestsymmkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(windows_core::Interface::as_raw(self), vardestsymmkey.param().abi()).ok()
    }
    pub unsafe fn Signature(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSignature<P0>(&self, varsignature: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSignature)(windows_core::Interface::as_raw(self), varsignature.param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticationProviderType)(windows_core::Interface::as_raw(self), lauthprovtype).ok()
    }
    pub unsafe fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAuthenticationProviderName<P0>(&self, bstrauthprovname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAuthenticationProviderName)(windows_core::Interface::as_raw(self), bstrauthprovname.param().abi()).ok()
    }
    pub unsafe fn SetSenderId<P0>(&self, varsenderid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSenderId)(windows_core::Interface::as_raw(self), varsenderid.param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MsgClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMsgClass)(windows_core::Interface::as_raw(self), lmsgclass).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TransactionId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsFirstInTransaction(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFirstInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLastInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    pub DestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSenderId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsFirstInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v1<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Body(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBody<P0>(&self, varbody: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), varbody.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v1<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCorrelationId<P0>(&self, varmsgid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), varmsgid.param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok()
    }
    pub unsafe fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSenderCertificate<P0>(&self, varsendercert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), varsendercert.param().abi()).ok()
    }
    pub unsafe fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Extension(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Extension)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExtension<P0>(&self, varextension: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetExtension)(windows_core::Interface::as_raw(self), varextension.param().abi()).ok()
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectorTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetConnectorTypeGuid<P0>(&self, bstrguidconnectortype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetConnectorTypeGuid)(windows_core::Interface::as_raw(self), bstrguidconnectortype.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DestinationSymmetricKey(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationSymmetricKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDestinationSymmetricKey<P0>(&self, vardestsymmkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(windows_core::Interface::as_raw(self), vardestsymmkey.param().abi()).ok()
    }
    pub unsafe fn Signature(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSignature<P0>(&self, varsignature: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSignature)(windows_core::Interface::as_raw(self), varsignature.param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticationProviderType)(windows_core::Interface::as_raw(self), lauthprovtype).ok()
    }
    pub unsafe fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAuthenticationProviderName<P0>(&self, bstrauthprovname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAuthenticationProviderName)(windows_core::Interface::as_raw(self), bstrauthprovname.param().abi()).ok()
    }
    pub unsafe fn SetSenderId<P0>(&self, varsenderid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSenderId)(windows_core::Interface::as_raw(self), varsenderid.param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MsgClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMsgClass)(windows_core::Interface::as_raw(self), lmsgclass).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TransactionId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsFirstInTransaction(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFirstInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLastInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v2<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v2<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo3>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo3>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseDestination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseDestination<P0>(&self, pdestresponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseDestination)(windows_core::Interface::as_raw(self), pdestresponse.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destination(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Destination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LookupId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAuthenticated2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFirstInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLastInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachCurrentSecurityContext2)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SoapEnvelope)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CompoundMessage(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompoundMessage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSoapHeader<P0>(&self, bstrsoapheader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSoapHeader)(windows_core::Interface::as_raw(self), bstrsoapheader.param().abi()).ok()
    }
    pub unsafe fn SetSoapBody<P0>(&self, bstrsoapbody: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSoapBody)(windows_core::Interface::as_raw(self), bstrsoapbody.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    pub DestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSenderId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsFirstInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destination: usize,
    pub LookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsAuthenticated2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SoapEnvelope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CompoundMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSoapHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSoapBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthLevel)(windows_core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Delivery(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Delivery)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDelivery)(windows_core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Trace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrace)(windows_core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v1<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok()
    }
    pub unsafe fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourceMachineGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BodyLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BodyLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Body(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBody<P0>(&self, varbody: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), varbody.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v1<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v1)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCorrelationId<P0>(&self, varmsgid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetCorrelationId)(windows_core::Interface::as_raw(self), varmsgid.param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Ack)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAck)(windows_core::Interface::as_raw(self), lack).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReachQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReachQueue)(windows_core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxTimeToReceive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxTimeToReceive)(windows_core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HashAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHashAlgorithm)(windows_core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncryptAlgorithm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEncryptAlgorithm)(windows_core::Interface::as_raw(self), lencryptalg).ok()
    }
    pub unsafe fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SentTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ArrivedTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderCertificate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSenderCertificate<P0>(&self, varsendercert: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSenderCertificate)(windows_core::Interface::as_raw(self), varsendercert.param().abi()).ok()
    }
    pub unsafe fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SenderIdType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderIdType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSenderIdType)(windows_core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Send<P0>(&self, destinationqueue: P0, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), destinationqueue.param().abi(), core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachCurrentSecurityContext)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SenderVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Extension(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Extension)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExtension<P0>(&self, varextension: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetExtension)(windows_core::Interface::as_raw(self), varextension.param().abi()).ok()
    }
    pub unsafe fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectorTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetConnectorTypeGuid<P0>(&self, bstrguidconnectortype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetConnectorTypeGuid)(windows_core::Interface::as_raw(self), bstrguidconnectortype.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionStatusQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DestinationSymmetricKey(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationSymmetricKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDestinationSymmetricKey<P0>(&self, vardestsymmkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetDestinationSymmetricKey)(windows_core::Interface::as_raw(self), vardestsymmkey.param().abi()).ok()
    }
    pub unsafe fn Signature(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSignature<P0>(&self, varsignature: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSignature)(windows_core::Interface::as_raw(self), varsignature.param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationProviderType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticationProviderType)(windows_core::Interface::as_raw(self), lauthprovtype).ok()
    }
    pub unsafe fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuthenticationProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAuthenticationProviderName<P0>(&self, bstrauthprovname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAuthenticationProviderName)(windows_core::Interface::as_raw(self), bstrauthprovname.param().abi()).ok()
    }
    pub unsafe fn SetSenderId<P0>(&self, varsenderid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSenderId)(windows_core::Interface::as_raw(self), varsenderid.param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MsgClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMsgClass)(windows_core::Interface::as_raw(self), lmsgclass).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TransactionId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransactionId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsFirstInTransaction(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFirstInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLastInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v2<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v2<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo2>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo_v2)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceivedAuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<P0>(&self, pqinforesponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo4>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseQueueInfo)(windows_core::Interface::as_raw(self), pqinforesponse.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminQueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<P0>(&self, pqinfoadmin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueueInfo4>,
    {
        (windows_core::Interface::vtable(self).putref_AdminQueueInfo)(windows_core::Interface::as_raw(self), pqinfoadmin.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResponseDestination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseDestination<P0>(&self, pdestresponse: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).putref_ResponseDestination)(windows_core::Interface::as_raw(self), pdestresponse.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destination(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Destination)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LookupId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAuthenticated2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFirstInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsLastInTransaction2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachCurrentSecurityContext2)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SoapEnvelope)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CompoundMessage(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompoundMessage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSoapHeader<P0>(&self, bstrsoapheader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSoapHeader)(windows_core::Interface::as_raw(self), bstrsoapheader.param().abi()).ok()
    }
    pub unsafe fn SetSoapBody<P0>(&self, bstrsoapbody: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSoapBody)(windows_core::Interface::as_raw(self), bstrsoapbody.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SourceMachineGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BodyLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Ack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAck: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SentTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ArrivedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    pub SenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSenderCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetConnectorTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    pub DestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAuthenticationProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSenderId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub TransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsFirstInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseDestination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destination: usize,
    pub LookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsAuthenticated2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SoapEnvelope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CompoundMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSoapHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSoapBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NextHops(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextHops)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EodGetSendInfo(&self) -> windows_core::Result<IMSMQCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EodGetSendInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EodResend(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EodResend)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQOutgoingQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextHops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EodGetSendInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EodGetSendInfo: usize,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EodResend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
    pub unsafe fn Handle(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHandle<P0>(&self, varhandle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetHandle)(windows_core::Interface::as_raw(self), varhandle.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQPrivateDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Hwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireArrivedEvent<P0>(&self, pq: P0, msgcursor: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue>,
    {
        (windows_core::Interface::vtable(self).FireArrivedEvent)(windows_core::Interface::as_raw(self), pq.param().abi(), msgcursor).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireArrivedErrorEvent<P0>(&self, pq: P0, hrstatus: windows_core::HRESULT, msgcursor: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQQueue>,
    {
        (windows_core::Interface::vtable(self).FireArrivedErrorEvent)(windows_core::Interface::as_raw(self), pq.param().abi(), hrstatus, msgcursor).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQPrivateEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Hwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FireArrivedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireArrivedEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FireArrivedErrorEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireArrivedErrorEvent: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupQueue: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupQueue_v2)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT, multicastaddress: *const windows_core::VARIANT, relmulticastaddress: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), core::mem::transmute(multicastaddress), core::mem::transmute(relmulticastaddress), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupQueue_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupQueue_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupQueue: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupQueue_v2)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT, multicastaddress: *const windows_core::VARIANT, relmulticastaddress: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupQueue)(windows_core::Interface::as_raw(self), core::mem::transmute(queueguid), core::mem::transmute(servicetypeguid), core::mem::transmute(label), core::mem::transmute(createtime), core::mem::transmute(modifytime), core::mem::transmute(relservicetype), core::mem::transmute(rellabel), core::mem::transmute(relcreatetime), core::mem::transmute(relmodifytime), core::mem::transmute(multicastaddress), core::mem::transmute(relmulticastaddress), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQuery4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupQueue_v2: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupQueue_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupQueue: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent>,
    {
        (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Receive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Peek: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekCurrent: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Receive_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Receive_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Peek_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent2>,
    {
        (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNext_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Receive_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Receive_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Peek_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Peek_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveCurrent_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNext_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNext_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekCurrent_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Receive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Peek: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Receive_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Receive_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Peek_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent3>,
    {
        (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNext_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle2(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveByLookupId<P0>(&self, lookupid: P0, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveNextByLookupId<P0>(&self, lookupid: P0, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveNextByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceivePreviousByLookupId<P0>(&self, lookupid: P0, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceivePreviousByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekByLookupId<P0>(&self, lookupid: P0, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNextByLookupId<P0>(&self, lookupid: P0, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNextByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekPreviousByLookupId<P0>(&self, lookupid: P0, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekPreviousByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Purge(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpen2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Receive_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Receive_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Peek_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Peek_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveCurrent_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNext_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNext_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekCurrent_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Receive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Peek: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Handle2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveNextByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceivePreviousByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveFirstByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveLastByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNextByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekPreviousByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekFirstByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Access)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ShareMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShareMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsOpen(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Receive_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Receive_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Peek_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnableNotification<P0>(&self, event: P0, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMSMQEvent3>,
    {
        (windows_core::Interface::vtable(self).EnableNotification)(windows_core::Interface::as_raw(self), event.param().abi(), core::mem::transmute(cursor), core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNext_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekCurrent_v1)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNext)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekCurrent)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(receivetimeout), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Handle2(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveByLookupId<P0>(&self, lookupid: P0, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveNextByLookupId<P0>(&self, lookupid: P0, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveNextByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceivePreviousByLookupId<P0>(&self, lookupid: P0, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceivePreviousByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekByLookupId<P0>(&self, lookupid: P0, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekNextByLookupId<P0>(&self, lookupid: P0, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekNextByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekPreviousByLookupId<P0>(&self, lookupid: P0, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekPreviousByLookupId)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekFirstByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeekLastByLookupId)(windows_core::Interface::as_raw(self), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Purge(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOpen2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiveByLookupIdAllowPeek<P0>(&self, lookupid: P0, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReceiveByLookupIdAllowPeek)(windows_core::Interface::as_raw(self), lookupid.param().abi(), core::mem::transmute(transaction), core::mem::transmute(wantdestinationqueue), core::mem::transmute(wantbody), core::mem::transmute(wantconnectortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueue4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Receive_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Receive_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Peek_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Peek_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnableNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveCurrent_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNext_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNext_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekCurrent_v1: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekCurrent_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Receive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Peek: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Handle2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveNextByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceivePreviousByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveFirstByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveLastByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekNextByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekNextByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekPreviousByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekFirstByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekFirstByLookupId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PeekLastByLookupId: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiveByLookupIdAllowPeek: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiveByLookupIdAllowPeek: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceTypeGuid<P0>(&self, bstrguidservicetype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), bstrguidservicetype.param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPathName<P0>(&self, bstrpathname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), bstrpathname.param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFormatName<P0>(&self, bstrformatname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), bstrformatname.param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok()
    }
    pub unsafe fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceTypeGuid<P0>(&self, bstrguidservicetype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), bstrguidservicetype.param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPathName<P0>(&self, bstrpathname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), bstrpathname.param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFormatName<P0>(&self, bstrformatname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), bstrformatname.param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok()
    }
    pub unsafe fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathNameDNS)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Security(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurity<P0>(&self, varsecurity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), varsecurity.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceTypeGuid<P0>(&self, bstrguidservicetype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), bstrguidservicetype.param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPathName<P0>(&self, bstrpathname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), bstrpathname.param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFormatName<P0>(&self, bstrformatname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), bstrformatname.param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok()
    }
    pub unsafe fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathNameDNS)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Security(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurity<P0>(&self, varsecurity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), varsecurity.param().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTransactional2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorldReadable2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MulticastAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMulticastAddress<P0>(&self, bstrmulticastaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMulticastAddress)(windows_core::Interface::as_raw(self), bstrmulticastaddress.param().abi()).ok()
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsTransactional2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueueGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceTypeGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceTypeGuid<P0>(&self, bstrguidservicetype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceTypeGuid)(windows_core::Interface::as_raw(self), bstrguidservicetype.param().abi()).ok()
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLabel<P0>(&self, bstrlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), bstrlabel.param().abi()).ok()
    }
    pub unsafe fn PathName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPathName<P0>(&self, bstrpathname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPathName)(windows_core::Interface::as_raw(self), bstrpathname.param().abi()).ok()
    }
    pub unsafe fn FormatName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFormatName<P0>(&self, bstrformatname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFormatName)(windows_core::Interface::as_raw(self), bstrformatname.param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTransactional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PrivLevel(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivLevel)(windows_core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Journal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournal)(windows_core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Quota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuota)(windows_core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BasePriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBasePriority)(windows_core::Interface::as_raw(self), lbasepriority).ok()
    }
    pub unsafe fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModifyTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Authenticate(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAuthenticate)(windows_core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JournalQuota)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetJournalQuota)(windows_core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorldReadable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute(istransactional), core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), access, sharemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PathNameDNS)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Security(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurity<P0>(&self, varsecurity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), varsecurity.param().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsTransactional2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorldReadable2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MulticastAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMulticastAddress<P0>(&self, bstrmulticastaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMulticastAddress)(windows_core::Interface::as_raw(self), bstrmulticastaddress.param().abi()).ok()
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfo4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub QueueGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceTypeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FormatName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFormatName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsTransactional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Journal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Quota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathNameDNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsTransactional2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMulticastAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
}
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
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> windows_core::Result<IMSMQQueueInfo4> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueInfos4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JournalMessageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn BytesInJournal(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BytesInJournal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EodGetReceiveInfo(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EodGetReceiveInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub JournalMessageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BytesInJournal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EodGetReceiveInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Transaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Commit(&self, fretaining: *const windows_core::VARIANT, grftc: *const windows_core::VARIANT, grfrm: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), core::mem::transmute(fretaining), core::mem::transmute(grftc), core::mem::transmute(grfrm)).ok()
    }
    pub unsafe fn Abort(&self, fretaining: *const windows_core::VARIANT, fasync: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), core::mem::transmute(fretaining), core::mem::transmute(fasync)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransaction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Transaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
    pub unsafe fn InitNew<P0>(&self, vartransaction: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self), vartransaction.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransaction2_Vtbl {
    pub base__: IMSMQTransaction_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
    pub unsafe fn ITransaction(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ITransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransaction3_Vtbl {
    pub base__: IMSMQTransaction2_Vtbl,
    pub ITransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMSMQTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
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
impl _DMSMQEventEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _DMSMQEventEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
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
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = MQCERT_REGISTER(1i32);
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = MQCERT_REGISTER(2i32);
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
pub const MSMQ_CONNECTED: windows_core::PCWSTR = windows_core::w!("CONNECTED");
pub const MSMQ_DISCONNECTED: windows_core::PCWSTR = windows_core::w!("DISCONNECTED");
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
pub const REL_EQ: RELOPS = RELOPS(1i32);
pub const REL_GE: RELOPS = RELOPS(6i32);
pub const REL_GT: RELOPS = RELOPS(4i32);
pub const REL_LE: RELOPS = RELOPS(5i32);
pub const REL_LT: RELOPS = RELOPS(3i32);
pub const REL_NEQ: RELOPS = RELOPS(2i32);
pub const REL_NOP: RELOPS = RELOPS(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FOREIGN_STATUS(pub i32);
impl windows_core::TypeKind for FOREIGN_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FOREIGN_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FOREIGN_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQACCESS(pub i32);
impl windows_core::TypeKind for MQACCESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQACCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQACCESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQAUTHENTICATE(pub i32);
impl windows_core::TypeKind for MQAUTHENTICATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQAUTHENTICATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQAUTHENTICATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQCALG(pub i32);
impl windows_core::TypeKind for MQCALG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQCALG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQCALG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQCERT_REGISTER(pub i32);
impl windows_core::TypeKind for MQCERT_REGISTER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQCERT_REGISTER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQCERT_REGISTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQConnectionState(pub i32);
impl windows_core::TypeKind for MQConnectionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQConnectionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQConnectionState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQDEFAULT(pub i32);
impl windows_core::TypeKind for MQDEFAULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQDEFAULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQDEFAULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQERROR(pub i32);
impl windows_core::TypeKind for MQERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQJOURNAL(pub i32);
impl windows_core::TypeKind for MQJOURNAL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQJOURNAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQJOURNAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMAX(pub i32);
impl windows_core::TypeKind for MQMAX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMAX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMAX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGACKNOWLEDGEMENT(pub i32);
impl windows_core::TypeKind for MQMSGACKNOWLEDGEMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGACKNOWLEDGEMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGACKNOWLEDGEMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGAUTHENTICATION(pub i32);
impl windows_core::TypeKind for MQMSGAUTHENTICATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGAUTHENTICATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGAUTHENTICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGAUTHLEVEL(pub i32);
impl windows_core::TypeKind for MQMSGAUTHLEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGAUTHLEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGAUTHLEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGCLASS(pub i32);
impl windows_core::TypeKind for MQMSGCLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGCLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGCLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGCURSOR(pub i32);
impl windows_core::TypeKind for MQMSGCURSOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGCURSOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGCURSOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGDELIVERY(pub i32);
impl windows_core::TypeKind for MQMSGDELIVERY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGDELIVERY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGDELIVERY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGIDSIZE(pub i32);
impl windows_core::TypeKind for MQMSGIDSIZE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGIDSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGIDSIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGJOURNAL(pub i32);
impl windows_core::TypeKind for MQMSGJOURNAL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGJOURNAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGJOURNAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGMAX(pub i32);
impl windows_core::TypeKind for MQMSGMAX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGMAX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGMAX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGPRIVLEVEL(pub i32);
impl windows_core::TypeKind for MQMSGPRIVLEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGPRIVLEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGPRIVLEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGSENDERIDTYPE(pub i32);
impl windows_core::TypeKind for MQMSGSENDERIDTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGSENDERIDTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGSENDERIDTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQMSGTRACE(pub i32);
impl windows_core::TypeKind for MQMSGTRACE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQMSGTRACE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQMSGTRACE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQPRIORITY(pub i32);
impl windows_core::TypeKind for MQPRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQPRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQPRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQPRIVLEVEL(pub i32);
impl windows_core::TypeKind for MQPRIVLEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQPRIVLEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQPRIVLEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQQUEUEACCESSMASK(pub u32);
impl windows_core::TypeKind for MQQUEUEACCESSMASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQQUEUEACCESSMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQQUEUEACCESSMASK").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQSHARE(pub i32);
impl windows_core::TypeKind for MQSHARE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQSHARE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQSHARE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQTRANSACTION(pub i32);
impl windows_core::TypeKind for MQTRANSACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQTRANSACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQTRANSACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQTRANSACTIONAL(pub i32);
impl windows_core::TypeKind for MQTRANSACTIONAL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQTRANSACTIONAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQTRANSACTIONAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MQWARNING(pub i32);
impl windows_core::TypeKind for MQWARNING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MQWARNING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MQWARNING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QUEUE_STATE(pub i32);
impl windows_core::TypeKind for QUEUE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QUEUE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QUEUE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QUEUE_TYPE(pub i32);
impl windows_core::TypeKind for QUEUE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QUEUE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QUEUE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RELOPS(pub i32);
impl windows_core::TypeKind for RELOPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RELOPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RELOPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XACT_STATUS(pub i32);
impl windows_core::TypeKind for XACT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XACT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XACT_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQCOLUMNSET {
    pub cCol: u32,
    pub aCol: *mut u32,
}
impl windows_core::TypeKind for MQCOLUMNSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQCOLUMNSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQMGMTPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut windows_core::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
impl windows_core::TypeKind for MQMGMTPROPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQMGMTPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQMSGPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut windows_core::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
impl windows_core::TypeKind for MQMSGPROPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQMSGPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQPRIVATEPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut windows_core::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
impl windows_core::TypeKind for MQPRIVATEPROPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQPRIVATEPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct MQPROPERTYRESTRICTION {
    pub rel: u32,
    pub prop: u32,
    pub prval: core::mem::ManuallyDrop<windows_core::PROPVARIANT>,
}
impl Clone for MQPROPERTYRESTRICTION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MQPROPERTYRESTRICTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQPROPERTYRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQQMPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut windows_core::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
impl windows_core::TypeKind for MQQMPROPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQQMPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQQUEUEPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut windows_core::PROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
impl windows_core::TypeKind for MQQUEUEPROPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQQUEUEPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQRESTRICTION {
    pub cRes: u32,
    pub paPropRes: *mut MQPROPERTYRESTRICTION,
}
impl windows_core::TypeKind for MQRESTRICTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQSORTKEY {
    pub propColumn: u32,
    pub dwOrder: u32,
}
impl windows_core::TypeKind for MQSORTKEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQSORTKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MQSORTSET {
    pub cCol: u32,
    pub aCol: *mut MQSORTKEY,
}
impl windows_core::TypeKind for MQSORTSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for MQSORTSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SEQUENCE_INFO {
    pub SeqID: i64,
    pub SeqNo: u32,
    pub PrevNo: u32,
}
impl windows_core::TypeKind for SEQUENCE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SEQUENCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_IO")]
pub type PMQRECEIVECALLBACK = Option<unsafe extern "system" fn(hrstatus: windows_core::HRESULT, hsource: isize, dwtimeout: u32, dwaction: u32, pmessageprops: *mut MQMSGPROPS, lpoverlapped: *mut super::IO::OVERLAPPED, hcursor: super::super::Foundation::HANDLE)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
