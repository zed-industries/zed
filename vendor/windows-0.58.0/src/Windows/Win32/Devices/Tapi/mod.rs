#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GetTnefStreamCodepage<P0>(lpstream: P0, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("mapi32.dll" "system" fn GetTnefStreamCodepage(lpstream : * mut core::ffi::c_void, lpulcodepage : *mut u32, lpulsubcodepage : *mut u32) -> windows_core::HRESULT);
    GetTnefStreamCodepage(lpstream.param().abi(), lpulcodepage, lpulsubcodepage).ok()
}
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OpenTnefStream<P0, P1>(lpvsupport: *mut core::ffi::c_void, lpstream: P0, lpszstreamname: *const i8, ulflags: u32, lpmessage: P1, wkeyval: u16, lpptnef: *mut Option<ITnef>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<super::super::System::AddressBook::IMessage>,
{
    windows_targets::link!("mapi32.dll" "system" fn OpenTnefStream(lpvsupport : *mut core::ffi::c_void, lpstream : * mut core::ffi::c_void, lpszstreamname : *const i8, ulflags : u32, lpmessage : * mut core::ffi::c_void, wkeyval : u16, lpptnef : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    OpenTnefStream(lpvsupport, lpstream.param().abi(), lpszstreamname, ulflags, lpmessage.param().abi(), wkeyval, core::mem::transmute(lpptnef)).ok()
}
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OpenTnefStreamEx<P0, P1, P2>(lpvsupport: *mut core::ffi::c_void, lpstream: P0, lpszstreamname: *const i8, ulflags: u32, lpmessage: P1, wkeyval: u16, lpadressbook: P2, lpptnef: *mut Option<ITnef>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<super::super::System::AddressBook::IMessage>,
    P2: windows_core::Param<super::super::System::AddressBook::IAddrBook>,
{
    windows_targets::link!("mapi32.dll" "system" fn OpenTnefStreamEx(lpvsupport : *mut core::ffi::c_void, lpstream : * mut core::ffi::c_void, lpszstreamname : *const i8, ulflags : u32, lpmessage : * mut core::ffi::c_void, wkeyval : u16, lpadressbook : * mut core::ffi::c_void, lpptnef : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    OpenTnefStreamEx(lpvsupport, lpstream.param().abi(), lpszstreamname, ulflags, lpmessage.param().abi(), wkeyval, lpadressbook.param().abi(), core::mem::transmute(lpptnef)).ok()
}
#[inline]
pub unsafe fn lineAccept<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineAccept(hcall : u32, lpsuseruserinfo : windows_core::PCSTR, dwsize : u32) -> i32);
    lineAccept(hcall, lpsuseruserinfo.param().abi(), dwsize)
}
#[inline]
pub unsafe fn lineAddProvider<P0, P1>(lpszproviderfilename: P0, hwndowner: P1, lpdwpermanentproviderid: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineAddProvider(lpszproviderfilename : windows_core::PCSTR, hwndowner : super::super::Foundation:: HWND, lpdwpermanentproviderid : *mut u32) -> i32);
    lineAddProvider(lpszproviderfilename.param().abi(), hwndowner.param().abi(), lpdwpermanentproviderid)
}
#[inline]
pub unsafe fn lineAddProviderA<P0, P1>(lpszproviderfilename: P0, hwndowner: P1, lpdwpermanentproviderid: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineAddProviderA(lpszproviderfilename : windows_core::PCSTR, hwndowner : super::super::Foundation:: HWND, lpdwpermanentproviderid : *mut u32) -> i32);
    lineAddProviderA(lpszproviderfilename.param().abi(), hwndowner.param().abi(), lpdwpermanentproviderid)
}
#[inline]
pub unsafe fn lineAddProviderW<P0, P1>(lpszproviderfilename: P0, hwndowner: P1, lpdwpermanentproviderid: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineAddProviderW(lpszproviderfilename : windows_core::PCWSTR, hwndowner : super::super::Foundation:: HWND, lpdwpermanentproviderid : *mut u32) -> i32);
    lineAddProviderW(lpszproviderfilename.param().abi(), hwndowner.param().abi(), lpdwpermanentproviderid)
}
#[inline]
pub unsafe fn lineAddToConference(hconfcall: u32, hconsultcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineAddToConference(hconfcall : u32, hconsultcall : u32) -> i32);
    lineAddToConference(hconfcall, hconsultcall)
}
#[inline]
pub unsafe fn lineAgentSpecific(hline: u32, dwaddressid: u32, dwagentextensionidindex: u32, lpparams: *mut core::ffi::c_void, dwsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineAgentSpecific(hline : u32, dwaddressid : u32, dwagentextensionidindex : u32, lpparams : *mut core::ffi::c_void, dwsize : u32) -> i32);
    lineAgentSpecific(hline, dwaddressid, dwagentextensionidindex, lpparams, dwsize)
}
#[inline]
pub unsafe fn lineAnswer<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineAnswer(hcall : u32, lpsuseruserinfo : windows_core::PCSTR, dwsize : u32) -> i32);
    lineAnswer(hcall, lpsuseruserinfo.param().abi(), dwsize)
}
#[inline]
pub unsafe fn lineBlindTransfer<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineBlindTransfer(hcall : u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32) -> i32);
    lineBlindTransfer(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineBlindTransferA<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineBlindTransferA(hcall : u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32) -> i32);
    lineBlindTransferA(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineBlindTransferW<P0>(hcall: u32, lpszdestaddressw: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineBlindTransferW(hcall : u32, lpszdestaddressw : windows_core::PCWSTR, dwcountrycode : u32) -> i32);
    lineBlindTransferW(hcall, lpszdestaddressw.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineClose(hline: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineClose(hline : u32) -> i32);
    lineClose(hline)
}
#[inline]
pub unsafe fn lineCompleteCall(hcall: u32, lpdwcompletionid: *mut u32, dwcompletionmode: u32, dwmessageid: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineCompleteCall(hcall : u32, lpdwcompletionid : *mut u32, dwcompletionmode : u32, dwmessageid : u32) -> i32);
    lineCompleteCall(hcall, lpdwcompletionid, dwcompletionmode, dwmessageid)
}
#[inline]
pub unsafe fn lineCompleteTransfer(hcall: u32, hconsultcall: u32, lphconfcall: *mut u32, dwtransfermode: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineCompleteTransfer(hcall : u32, hconsultcall : u32, lphconfcall : *mut u32, dwtransfermode : u32) -> i32);
    lineCompleteTransfer(hcall, hconsultcall, lphconfcall, dwtransfermode)
}
#[inline]
pub unsafe fn lineConfigDialog<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineConfigDialog(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineConfigDialog(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineConfigDialogA<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineConfigDialogA(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineConfigDialogA(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineConfigDialogEdit<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1, lpdeviceconfigin: *const core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineConfigDialogEdit(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCSTR, lpdeviceconfigin : *const core::ffi::c_void, dwsize : u32, lpdeviceconfigout : *mut VARSTRING) -> i32);
    lineConfigDialogEdit(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi(), lpdeviceconfigin, dwsize, lpdeviceconfigout)
}
#[inline]
pub unsafe fn lineConfigDialogEditA<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1, lpdeviceconfigin: *const core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineConfigDialogEditA(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCSTR, lpdeviceconfigin : *const core::ffi::c_void, dwsize : u32, lpdeviceconfigout : *mut VARSTRING) -> i32);
    lineConfigDialogEditA(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi(), lpdeviceconfigin, dwsize, lpdeviceconfigout)
}
#[inline]
pub unsafe fn lineConfigDialogEditW<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1, lpdeviceconfigin: *const core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineConfigDialogEditW(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCWSTR, lpdeviceconfigin : *const core::ffi::c_void, dwsize : u32, lpdeviceconfigout : *mut VARSTRING) -> i32);
    lineConfigDialogEditW(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi(), lpdeviceconfigin, dwsize, lpdeviceconfigout)
}
#[inline]
pub unsafe fn lineConfigDialogW<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineConfigDialogW(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCWSTR) -> i32);
    lineConfigDialogW(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineConfigProvider<P0>(hwndowner: P0, dwpermanentproviderid: u32) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineConfigProvider(hwndowner : super::super::Foundation:: HWND, dwpermanentproviderid : u32) -> i32);
    lineConfigProvider(hwndowner.param().abi(), dwpermanentproviderid)
}
#[inline]
pub unsafe fn lineCreateAgentA<P0, P1>(hline: u32, lpszagentid: P0, lpszagentpin: P1, lphagent: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineCreateAgentA(hline : u32, lpszagentid : windows_core::PCSTR, lpszagentpin : windows_core::PCSTR, lphagent : *mut u32) -> i32);
    lineCreateAgentA(hline, lpszagentid.param().abi(), lpszagentpin.param().abi(), lphagent)
}
#[inline]
pub unsafe fn lineCreateAgentSessionA<P0>(hline: u32, hagent: u32, lpszagentpin: P0, dwworkingaddressid: u32, lpgroupid: *mut windows_core::GUID, lphagentsession: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineCreateAgentSessionA(hline : u32, hagent : u32, lpszagentpin : windows_core::PCSTR, dwworkingaddressid : u32, lpgroupid : *mut windows_core::GUID, lphagentsession : *mut u32) -> i32);
    lineCreateAgentSessionA(hline, hagent, lpszagentpin.param().abi(), dwworkingaddressid, lpgroupid, lphagentsession)
}
#[inline]
pub unsafe fn lineCreateAgentSessionW<P0>(hline: u32, hagent: u32, lpszagentpin: P0, dwworkingaddressid: u32, lpgroupid: *mut windows_core::GUID, lphagentsession: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineCreateAgentSessionW(hline : u32, hagent : u32, lpszagentpin : windows_core::PCWSTR, dwworkingaddressid : u32, lpgroupid : *mut windows_core::GUID, lphagentsession : *mut u32) -> i32);
    lineCreateAgentSessionW(hline, hagent, lpszagentpin.param().abi(), dwworkingaddressid, lpgroupid, lphagentsession)
}
#[inline]
pub unsafe fn lineCreateAgentW<P0, P1>(hline: u32, lpszagentid: P0, lpszagentpin: P1, lphagent: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineCreateAgentW(hline : u32, lpszagentid : windows_core::PCWSTR, lpszagentpin : windows_core::PCWSTR, lphagent : *mut u32) -> i32);
    lineCreateAgentW(hline, lpszagentid.param().abi(), lpszagentpin.param().abi(), lphagent)
}
#[inline]
pub unsafe fn lineDeallocateCall(hcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineDeallocateCall(hcall : u32) -> i32);
    lineDeallocateCall(hcall)
}
#[inline]
pub unsafe fn lineDevSpecific(hline: u32, dwaddressid: u32, hcall: u32, lpparams: *mut core::ffi::c_void, dwsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineDevSpecific(hline : u32, dwaddressid : u32, hcall : u32, lpparams : *mut core::ffi::c_void, dwsize : u32) -> i32);
    lineDevSpecific(hline, dwaddressid, hcall, lpparams, dwsize)
}
#[inline]
pub unsafe fn lineDevSpecificFeature(hline: u32, dwfeature: u32, lpparams: *mut core::ffi::c_void, dwsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineDevSpecificFeature(hline : u32, dwfeature : u32, lpparams : *mut core::ffi::c_void, dwsize : u32) -> i32);
    lineDevSpecificFeature(hline, dwfeature, lpparams, dwsize)
}
#[inline]
pub unsafe fn lineDial<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineDial(hcall : u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32) -> i32);
    lineDial(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineDialA<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineDialA(hcall : u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32) -> i32);
    lineDialA(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineDialW<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineDialW(hcall : u32, lpszdestaddress : windows_core::PCWSTR, dwcountrycode : u32) -> i32);
    lineDialW(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineDrop<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineDrop(hcall : u32, lpsuseruserinfo : windows_core::PCSTR, dwsize : u32) -> i32);
    lineDrop(hcall, lpsuseruserinfo.param().abi(), dwsize)
}
#[inline]
pub unsafe fn lineForward(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineForward(hline : u32, balladdresses : u32, dwaddressid : u32, lpforwardlist : *const LINEFORWARDLIST, dwnumringsnoanswer : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineForward(hline, balladdresses, dwaddressid, lpforwardlist, dwnumringsnoanswer, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn lineForwardA(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineForwardA(hline : u32, balladdresses : u32, dwaddressid : u32, lpforwardlist : *const LINEFORWARDLIST, dwnumringsnoanswer : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineForwardA(hline, balladdresses, dwaddressid, lpforwardlist, dwnumringsnoanswer, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn lineForwardW(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineForwardW(hline : u32, balladdresses : u32, dwaddressid : u32, lpforwardlist : *const LINEFORWARDLIST, dwnumringsnoanswer : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineForwardW(hline, balladdresses, dwaddressid, lpforwardlist, dwnumringsnoanswer, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn lineGatherDigits<P0>(hcall: u32, dwdigitmodes: u32, lpsdigits: Option<&mut [u8]>, lpszterminationdigits: P0, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGatherDigits(hcall : u32, dwdigitmodes : u32, lpsdigits : windows_core::PSTR, dwnumdigits : u32, lpszterminationdigits : windows_core::PCSTR, dwfirstdigittimeout : u32, dwinterdigittimeout : u32) -> i32);
    lineGatherDigits(hcall, dwdigitmodes, core::mem::transmute(lpsdigits.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpsdigits.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszterminationdigits.param().abi(), dwfirstdigittimeout, dwinterdigittimeout)
}
#[inline]
pub unsafe fn lineGatherDigitsA<P0>(hcall: u32, dwdigitmodes: u32, lpsdigits: Option<&mut [u8]>, lpszterminationdigits: P0, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGatherDigitsA(hcall : u32, dwdigitmodes : u32, lpsdigits : windows_core::PSTR, dwnumdigits : u32, lpszterminationdigits : windows_core::PCSTR, dwfirstdigittimeout : u32, dwinterdigittimeout : u32) -> i32);
    lineGatherDigitsA(hcall, dwdigitmodes, core::mem::transmute(lpsdigits.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpsdigits.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszterminationdigits.param().abi(), dwfirstdigittimeout, dwinterdigittimeout)
}
#[inline]
pub unsafe fn lineGatherDigitsW<P0>(hcall: u32, dwdigitmodes: u32, lpsdigits: Option<&mut [u16]>, lpszterminationdigits: P0, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGatherDigitsW(hcall : u32, dwdigitmodes : u32, lpsdigits : windows_core::PWSTR, dwnumdigits : u32, lpszterminationdigits : windows_core::PCWSTR, dwfirstdigittimeout : u32, dwinterdigittimeout : u32) -> i32);
    lineGatherDigitsW(hcall, dwdigitmodes, core::mem::transmute(lpsdigits.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpsdigits.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszterminationdigits.param().abi(), dwfirstdigittimeout, dwinterdigittimeout)
}
#[inline]
pub unsafe fn lineGenerateDigits<P0>(hcall: u32, dwdigitmode: u32, lpszdigits: P0, dwduration: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGenerateDigits(hcall : u32, dwdigitmode : u32, lpszdigits : windows_core::PCSTR, dwduration : u32) -> i32);
    lineGenerateDigits(hcall, dwdigitmode, lpszdigits.param().abi(), dwduration)
}
#[inline]
pub unsafe fn lineGenerateDigitsA<P0>(hcall: u32, dwdigitmode: u32, lpszdigits: P0, dwduration: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGenerateDigitsA(hcall : u32, dwdigitmode : u32, lpszdigits : windows_core::PCSTR, dwduration : u32) -> i32);
    lineGenerateDigitsA(hcall, dwdigitmode, lpszdigits.param().abi(), dwduration)
}
#[inline]
pub unsafe fn lineGenerateDigitsW<P0>(hcall: u32, dwdigitmode: u32, lpszdigits: P0, dwduration: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGenerateDigitsW(hcall : u32, dwdigitmode : u32, lpszdigits : windows_core::PCWSTR, dwduration : u32) -> i32);
    lineGenerateDigitsW(hcall, dwdigitmode, lpszdigits.param().abi(), dwduration)
}
#[inline]
pub unsafe fn lineGenerateTone(hcall: u32, dwtonemode: u32, dwduration: u32, dwnumtones: u32, lptones: *const LINEGENERATETONE) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGenerateTone(hcall : u32, dwtonemode : u32, dwduration : u32, dwnumtones : u32, lptones : *const LINEGENERATETONE) -> i32);
    lineGenerateTone(hcall, dwtonemode, dwduration, dwnumtones, lptones)
}
#[inline]
pub unsafe fn lineGetAddressCaps(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressCaps(hlineapp : u32, dwdeviceid : u32, dwaddressid : u32, dwapiversion : u32, dwextversion : u32, lpaddresscaps : *mut LINEADDRESSCAPS) -> i32);
    lineGetAddressCaps(hlineapp, dwdeviceid, dwaddressid, dwapiversion, dwextversion, lpaddresscaps)
}
#[inline]
pub unsafe fn lineGetAddressCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressCapsA(hlineapp : u32, dwdeviceid : u32, dwaddressid : u32, dwapiversion : u32, dwextversion : u32, lpaddresscaps : *mut LINEADDRESSCAPS) -> i32);
    lineGetAddressCapsA(hlineapp, dwdeviceid, dwaddressid, dwapiversion, dwextversion, lpaddresscaps)
}
#[inline]
pub unsafe fn lineGetAddressCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressCapsW(hlineapp : u32, dwdeviceid : u32, dwaddressid : u32, dwapiversion : u32, dwextversion : u32, lpaddresscaps : *mut LINEADDRESSCAPS) -> i32);
    lineGetAddressCapsW(hlineapp, dwdeviceid, dwaddressid, dwapiversion, dwextversion, lpaddresscaps)
}
#[inline]
pub unsafe fn lineGetAddressID<P0>(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressID(hline : u32, lpdwaddressid : *mut u32, dwaddressmode : u32, lpsaddress : windows_core::PCSTR, dwsize : u32) -> i32);
    lineGetAddressID(hline, lpdwaddressid, dwaddressmode, lpsaddress.param().abi(), dwsize)
}
#[inline]
pub unsafe fn lineGetAddressIDA<P0>(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressIDA(hline : u32, lpdwaddressid : *mut u32, dwaddressmode : u32, lpsaddress : windows_core::PCSTR, dwsize : u32) -> i32);
    lineGetAddressIDA(hline, lpdwaddressid, dwaddressmode, lpsaddress.param().abi(), dwsize)
}
#[inline]
pub unsafe fn lineGetAddressIDW<P0>(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressIDW(hline : u32, lpdwaddressid : *mut u32, dwaddressmode : u32, lpsaddress : windows_core::PCWSTR, dwsize : u32) -> i32);
    lineGetAddressIDW(hline, lpdwaddressid, dwaddressmode, lpsaddress.param().abi(), dwsize)
}
#[inline]
pub unsafe fn lineGetAddressStatus(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressStatus(hline : u32, dwaddressid : u32, lpaddressstatus : *mut LINEADDRESSSTATUS) -> i32);
    lineGetAddressStatus(hline, dwaddressid, lpaddressstatus)
}
#[inline]
pub unsafe fn lineGetAddressStatusA(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressStatusA(hline : u32, dwaddressid : u32, lpaddressstatus : *mut LINEADDRESSSTATUS) -> i32);
    lineGetAddressStatusA(hline, dwaddressid, lpaddressstatus)
}
#[inline]
pub unsafe fn lineGetAddressStatusW(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAddressStatusW(hline : u32, dwaddressid : u32, lpaddressstatus : *mut LINEADDRESSSTATUS) -> i32);
    lineGetAddressStatusW(hline, dwaddressid, lpaddressstatus)
}
#[inline]
pub unsafe fn lineGetAgentActivityListA(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentActivityListA(hline : u32, dwaddressid : u32, lpagentactivitylist : *mut LINEAGENTACTIVITYLIST) -> i32);
    lineGetAgentActivityListA(hline, dwaddressid, lpagentactivitylist)
}
#[inline]
pub unsafe fn lineGetAgentActivityListW(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentActivityListW(hline : u32, dwaddressid : u32, lpagentactivitylist : *mut LINEAGENTACTIVITYLIST) -> i32);
    lineGetAgentActivityListW(hline, dwaddressid, lpagentactivitylist)
}
#[inline]
pub unsafe fn lineGetAgentCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentCapsA(hlineapp : u32, dwdeviceid : u32, dwaddressid : u32, dwappapiversion : u32, lpagentcaps : *mut LINEAGENTCAPS) -> i32);
    lineGetAgentCapsA(hlineapp, dwdeviceid, dwaddressid, dwappapiversion, lpagentcaps)
}
#[inline]
pub unsafe fn lineGetAgentCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentCapsW(hlineapp : u32, dwdeviceid : u32, dwaddressid : u32, dwappapiversion : u32, lpagentcaps : *mut LINEAGENTCAPS) -> i32);
    lineGetAgentCapsW(hlineapp, dwdeviceid, dwaddressid, dwappapiversion, lpagentcaps)
}
#[inline]
pub unsafe fn lineGetAgentGroupListA(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentGroupListA(hline : u32, dwaddressid : u32, lpagentgrouplist : *mut LINEAGENTGROUPLIST) -> i32);
    lineGetAgentGroupListA(hline, dwaddressid, lpagentgrouplist)
}
#[inline]
pub unsafe fn lineGetAgentGroupListW(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentGroupListW(hline : u32, dwaddressid : u32, lpagentgrouplist : *mut LINEAGENTGROUPLIST) -> i32);
    lineGetAgentGroupListW(hline, dwaddressid, lpagentgrouplist)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn lineGetAgentInfo(hline: u32, hagent: u32, lpagentinfo: *mut LINEAGENTINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentInfo(hline : u32, hagent : u32, lpagentinfo : *mut LINEAGENTINFO) -> i32);
    lineGetAgentInfo(hline, hagent, lpagentinfo)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn lineGetAgentSessionInfo(hline: u32, hagentsession: u32, lpagentsessioninfo: *mut LINEAGENTSESSIONINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentSessionInfo(hline : u32, hagentsession : u32, lpagentsessioninfo : *mut LINEAGENTSESSIONINFO) -> i32);
    lineGetAgentSessionInfo(hline, hagentsession, lpagentsessioninfo)
}
#[inline]
pub unsafe fn lineGetAgentSessionList(hline: u32, hagent: u32, lpagentsessionlist: *mut LINEAGENTSESSIONLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentSessionList(hline : u32, hagent : u32, lpagentsessionlist : *mut LINEAGENTSESSIONLIST) -> i32);
    lineGetAgentSessionList(hline, hagent, lpagentsessionlist)
}
#[inline]
pub unsafe fn lineGetAgentStatusA(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentStatusA(hline : u32, dwaddressid : u32, lpagentstatus : *mut LINEAGENTSTATUS) -> i32);
    lineGetAgentStatusA(hline, dwaddressid, lpagentstatus)
}
#[inline]
pub unsafe fn lineGetAgentStatusW(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetAgentStatusW(hline : u32, dwaddressid : u32, lpagentstatus : *mut LINEAGENTSTATUS) -> i32);
    lineGetAgentStatusW(hline, dwaddressid, lpagentstatus)
}
#[inline]
pub unsafe fn lineGetAppPriority<P0>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetAppPriority(lpszappfilename : windows_core::PCSTR, dwmediamode : u32, lpextensionid : *mut LINEEXTENSIONID, dwrequestmode : u32, lpextensionname : *mut VARSTRING, lpdwpriority : *mut u32) -> i32);
    lineGetAppPriority(lpszappfilename.param().abi(), dwmediamode, lpextensionid, dwrequestmode, lpextensionname, lpdwpriority)
}
#[inline]
pub unsafe fn lineGetAppPriorityA<P0>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetAppPriorityA(lpszappfilename : windows_core::PCSTR, dwmediamode : u32, lpextensionid : *mut LINEEXTENSIONID, dwrequestmode : u32, lpextensionname : *mut VARSTRING, lpdwpriority : *mut u32) -> i32);
    lineGetAppPriorityA(lpszappfilename.param().abi(), dwmediamode, lpextensionid, dwrequestmode, lpextensionname, lpdwpriority)
}
#[inline]
pub unsafe fn lineGetAppPriorityW<P0>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetAppPriorityW(lpszappfilename : windows_core::PCWSTR, dwmediamode : u32, lpextensionid : *mut LINEEXTENSIONID, dwrequestmode : u32, lpextensionname : *mut VARSTRING, lpdwpriority : *mut u32) -> i32);
    lineGetAppPriorityW(lpszappfilename.param().abi(), dwmediamode, lpextensionid, dwrequestmode, lpextensionname, lpdwpriority)
}
#[inline]
pub unsafe fn lineGetCallInfo(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetCallInfo(hcall : u32, lpcallinfo : *mut LINECALLINFO) -> i32);
    lineGetCallInfo(hcall, lpcallinfo)
}
#[inline]
pub unsafe fn lineGetCallInfoA(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetCallInfoA(hcall : u32, lpcallinfo : *mut LINECALLINFO) -> i32);
    lineGetCallInfoA(hcall, lpcallinfo)
}
#[inline]
pub unsafe fn lineGetCallInfoW(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetCallInfoW(hcall : u32, lpcallinfo : *mut LINECALLINFO) -> i32);
    lineGetCallInfoW(hcall, lpcallinfo)
}
#[inline]
pub unsafe fn lineGetCallStatus(hcall: u32, lpcallstatus: *mut LINECALLSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetCallStatus(hcall : u32, lpcallstatus : *mut LINECALLSTATUS) -> i32);
    lineGetCallStatus(hcall, lpcallstatus)
}
#[inline]
pub unsafe fn lineGetConfRelatedCalls(hcall: u32, lpcalllist: *mut LINECALLLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetConfRelatedCalls(hcall : u32, lpcalllist : *mut LINECALLLIST) -> i32);
    lineGetConfRelatedCalls(hcall, lpcalllist)
}
#[inline]
pub unsafe fn lineGetCountry(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetCountry(dwcountryid : u32, dwapiversion : u32, lplinecountrylist : *mut LINECOUNTRYLIST) -> i32);
    lineGetCountry(dwcountryid, dwapiversion, lplinecountrylist)
}
#[inline]
pub unsafe fn lineGetCountryA(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetCountryA(dwcountryid : u32, dwapiversion : u32, lplinecountrylist : *mut LINECOUNTRYLIST) -> i32);
    lineGetCountryA(dwcountryid, dwapiversion, lplinecountrylist)
}
#[inline]
pub unsafe fn lineGetCountryW(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetCountryW(dwcountryid : u32, dwapiversion : u32, lplinecountrylist : *mut LINECOUNTRYLIST) -> i32);
    lineGetCountryW(dwcountryid, dwapiversion, lplinecountrylist)
}
#[inline]
pub unsafe fn lineGetDevCaps(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetDevCaps(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextversion : u32, lplinedevcaps : *mut LINEDEVCAPS) -> i32);
    lineGetDevCaps(hlineapp, dwdeviceid, dwapiversion, dwextversion, lplinedevcaps)
}
#[inline]
pub unsafe fn lineGetDevCapsA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetDevCapsA(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextversion : u32, lplinedevcaps : *mut LINEDEVCAPS) -> i32);
    lineGetDevCapsA(hlineapp, dwdeviceid, dwapiversion, dwextversion, lplinedevcaps)
}
#[inline]
pub unsafe fn lineGetDevCapsW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetDevCapsW(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextversion : u32, lplinedevcaps : *mut LINEDEVCAPS) -> i32);
    lineGetDevCapsW(hlineapp, dwdeviceid, dwapiversion, dwextversion, lplinedevcaps)
}
#[inline]
pub unsafe fn lineGetDevConfig<P0>(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetDevConfig(dwdeviceid : u32, lpdeviceconfig : *mut VARSTRING, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineGetDevConfig(dwdeviceid, lpdeviceconfig, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineGetDevConfigA<P0>(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetDevConfigA(dwdeviceid : u32, lpdeviceconfig : *mut VARSTRING, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineGetDevConfigA(dwdeviceid, lpdeviceconfig, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineGetDevConfigW<P0>(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetDevConfigW(dwdeviceid : u32, lpdeviceconfig : *mut VARSTRING, lpszdeviceclass : windows_core::PCWSTR) -> i32);
    lineGetDevConfigW(dwdeviceid, lpdeviceconfig, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineGetGroupListA(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetGroupListA(hline : u32, lpgrouplist : *mut LINEAGENTGROUPLIST) -> i32);
    lineGetGroupListA(hline, lpgrouplist)
}
#[inline]
pub unsafe fn lineGetGroupListW(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetGroupListW(hline : u32, lpgrouplist : *mut LINEAGENTGROUPLIST) -> i32);
    lineGetGroupListW(hline, lpgrouplist)
}
#[inline]
pub unsafe fn lineGetID<P0>(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetID(hline : u32, dwaddressid : u32, hcall : u32, dwselect : u32, lpdeviceid : *mut VARSTRING, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineGetID(hline, dwaddressid, hcall, dwselect, lpdeviceid, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineGetIDA<P0>(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetIDA(hline : u32, dwaddressid : u32, hcall : u32, dwselect : u32, lpdeviceid : *mut VARSTRING, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineGetIDA(hline, dwaddressid, hcall, dwselect, lpdeviceid, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineGetIDW<P0>(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetIDW(hline : u32, dwaddressid : u32, hcall : u32, dwselect : u32, lpdeviceid : *mut VARSTRING, lpszdeviceclass : windows_core::PCWSTR) -> i32);
    lineGetIDW(hline, dwaddressid, hcall, dwselect, lpdeviceid, lpszdeviceclass.param().abi())
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn lineGetIcon<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetIcon(dwdeviceid : u32, lpszdeviceclass : windows_core::PCSTR, lphicon : *mut super::super::UI::WindowsAndMessaging:: HICON) -> i32);
    lineGetIcon(dwdeviceid, lpszdeviceclass.param().abi(), lphicon)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn lineGetIconA<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetIconA(dwdeviceid : u32, lpszdeviceclass : windows_core::PCSTR, lphicon : *mut super::super::UI::WindowsAndMessaging:: HICON) -> i32);
    lineGetIconA(dwdeviceid, lpszdeviceclass.param().abi(), lphicon)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn lineGetIconW<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineGetIconW(dwdeviceid : u32, lpszdeviceclass : windows_core::PCWSTR, lphicon : *mut super::super::UI::WindowsAndMessaging:: HICON) -> i32);
    lineGetIconW(dwdeviceid, lpszdeviceclass.param().abi(), lphicon)
}
#[inline]
pub unsafe fn lineGetLineDevStatus(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetLineDevStatus(hline : u32, lplinedevstatus : *mut LINEDEVSTATUS) -> i32);
    lineGetLineDevStatus(hline, lplinedevstatus)
}
#[inline]
pub unsafe fn lineGetLineDevStatusA(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetLineDevStatusA(hline : u32, lplinedevstatus : *mut LINEDEVSTATUS) -> i32);
    lineGetLineDevStatusA(hline, lplinedevstatus)
}
#[inline]
pub unsafe fn lineGetLineDevStatusW(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetLineDevStatusW(hline : u32, lplinedevstatus : *mut LINEDEVSTATUS) -> i32);
    lineGetLineDevStatusW(hline, lplinedevstatus)
}
#[inline]
pub unsafe fn lineGetMessage(hlineapp: u32, lpmessage: *mut LINEMESSAGE, dwtimeout: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetMessage(hlineapp : u32, lpmessage : *mut LINEMESSAGE, dwtimeout : u32) -> i32);
    lineGetMessage(hlineapp, lpmessage, dwtimeout)
}
#[inline]
pub unsafe fn lineGetNewCalls(hline: u32, dwaddressid: u32, dwselect: u32, lpcalllist: *mut LINECALLLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetNewCalls(hline : u32, dwaddressid : u32, dwselect : u32, lpcalllist : *mut LINECALLLIST) -> i32);
    lineGetNewCalls(hline, dwaddressid, dwselect, lpcalllist)
}
#[inline]
pub unsafe fn lineGetNumRings(hline: u32, dwaddressid: u32, lpdwnumrings: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetNumRings(hline : u32, dwaddressid : u32, lpdwnumrings : *mut u32) -> i32);
    lineGetNumRings(hline, dwaddressid, lpdwnumrings)
}
#[inline]
pub unsafe fn lineGetProviderList(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetProviderList(dwapiversion : u32, lpproviderlist : *mut LINEPROVIDERLIST) -> i32);
    lineGetProviderList(dwapiversion, lpproviderlist)
}
#[inline]
pub unsafe fn lineGetProviderListA(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetProviderListA(dwapiversion : u32, lpproviderlist : *mut LINEPROVIDERLIST) -> i32);
    lineGetProviderListA(dwapiversion, lpproviderlist)
}
#[inline]
pub unsafe fn lineGetProviderListW(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetProviderListW(dwapiversion : u32, lpproviderlist : *mut LINEPROVIDERLIST) -> i32);
    lineGetProviderListW(dwapiversion, lpproviderlist)
}
#[inline]
pub unsafe fn lineGetProxyStatus(hlineapp: u32, dwdeviceid: u32, dwappapiversion: u32, lplineproxyreqestlist: *mut LINEPROXYREQUESTLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetProxyStatus(hlineapp : u32, dwdeviceid : u32, dwappapiversion : u32, lplineproxyreqestlist : *mut LINEPROXYREQUESTLIST) -> i32);
    lineGetProxyStatus(hlineapp, dwdeviceid, dwappapiversion, lplineproxyreqestlist)
}
#[inline]
pub unsafe fn lineGetQueueInfo(hline: u32, dwqueueid: u32, lplinequeueinfo: *mut LINEQUEUEINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetQueueInfo(hline : u32, dwqueueid : u32, lplinequeueinfo : *mut LINEQUEUEINFO) -> i32);
    lineGetQueueInfo(hline, dwqueueid, lplinequeueinfo)
}
#[inline]
pub unsafe fn lineGetQueueListA(hline: u32, lpgroupid: *mut windows_core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetQueueListA(hline : u32, lpgroupid : *mut windows_core::GUID, lpqueuelist : *mut LINEQUEUELIST) -> i32);
    lineGetQueueListA(hline, lpgroupid, lpqueuelist)
}
#[inline]
pub unsafe fn lineGetQueueListW(hline: u32, lpgroupid: *mut windows_core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetQueueListW(hline : u32, lpgroupid : *mut windows_core::GUID, lpqueuelist : *mut LINEQUEUELIST) -> i32);
    lineGetQueueListW(hline, lpgroupid, lpqueuelist)
}
#[inline]
pub unsafe fn lineGetRequest(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetRequest(hlineapp : u32, dwrequestmode : u32, lprequestbuffer : *mut core::ffi::c_void) -> i32);
    lineGetRequest(hlineapp, dwrequestmode, lprequestbuffer)
}
#[inline]
pub unsafe fn lineGetRequestA(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetRequestA(hlineapp : u32, dwrequestmode : u32, lprequestbuffer : *mut core::ffi::c_void) -> i32);
    lineGetRequestA(hlineapp, dwrequestmode, lprequestbuffer)
}
#[inline]
pub unsafe fn lineGetRequestW(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetRequestW(hlineapp : u32, dwrequestmode : u32, lprequestbuffer : *mut core::ffi::c_void) -> i32);
    lineGetRequestW(hlineapp, dwrequestmode, lprequestbuffer)
}
#[inline]
pub unsafe fn lineGetStatusMessages(hline: u32, lpdwlinestates: *mut u32, lpdwaddressstates: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetStatusMessages(hline : u32, lpdwlinestates : *mut u32, lpdwaddressstates : *mut u32) -> i32);
    lineGetStatusMessages(hline, lpdwlinestates, lpdwaddressstates)
}
#[inline]
pub unsafe fn lineGetTranslateCaps(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetTranslateCaps(hlineapp : u32, dwapiversion : u32, lptranslatecaps : *mut LINETRANSLATECAPS) -> i32);
    lineGetTranslateCaps(hlineapp, dwapiversion, lptranslatecaps)
}
#[inline]
pub unsafe fn lineGetTranslateCapsA(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetTranslateCapsA(hlineapp : u32, dwapiversion : u32, lptranslatecaps : *mut LINETRANSLATECAPS) -> i32);
    lineGetTranslateCapsA(hlineapp, dwapiversion, lptranslatecaps)
}
#[inline]
pub unsafe fn lineGetTranslateCapsW(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineGetTranslateCapsW(hlineapp : u32, dwapiversion : u32, lptranslatecaps : *mut LINETRANSLATECAPS) -> i32);
    lineGetTranslateCapsW(hlineapp, dwapiversion, lptranslatecaps)
}
#[inline]
pub unsafe fn lineHandoff<P0>(hcall: u32, lpszfilename: P0, dwmediamode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineHandoff(hcall : u32, lpszfilename : windows_core::PCSTR, dwmediamode : u32) -> i32);
    lineHandoff(hcall, lpszfilename.param().abi(), dwmediamode)
}
#[inline]
pub unsafe fn lineHandoffA<P0>(hcall: u32, lpszfilename: P0, dwmediamode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineHandoffA(hcall : u32, lpszfilename : windows_core::PCSTR, dwmediamode : u32) -> i32);
    lineHandoffA(hcall, lpszfilename.param().abi(), dwmediamode)
}
#[inline]
pub unsafe fn lineHandoffW<P0>(hcall: u32, lpszfilename: P0, dwmediamode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineHandoffW(hcall : u32, lpszfilename : windows_core::PCWSTR, dwmediamode : u32) -> i32);
    lineHandoffW(hcall, lpszfilename.param().abi(), dwmediamode)
}
#[inline]
pub unsafe fn lineHold(hcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineHold(hcall : u32) -> i32);
    lineHold(hcall)
}
#[inline]
pub unsafe fn lineInitialize<P0, P1>(lphlineapp: *mut u32, hinstance: P0, lpfncallback: LINECALLBACK, lpszappname: P1, lpdwnumdevs: *mut u32) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineInitialize(lphlineapp : *mut u32, hinstance : super::super::Foundation:: HINSTANCE, lpfncallback : LINECALLBACK, lpszappname : windows_core::PCSTR, lpdwnumdevs : *mut u32) -> i32);
    lineInitialize(lphlineapp, hinstance.param().abi(), lpfncallback, lpszappname.param().abi(), lpdwnumdevs)
}
#[inline]
pub unsafe fn lineInitializeExA<P0, P1>(lphlineapp: *mut u32, hinstance: P0, lpfncallback: LINECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineInitializeExA(lphlineapp : *mut u32, hinstance : super::super::Foundation:: HINSTANCE, lpfncallback : LINECALLBACK, lpszfriendlyappname : windows_core::PCSTR, lpdwnumdevs : *mut u32, lpdwapiversion : *mut u32, lplineinitializeexparams : *mut LINEINITIALIZEEXPARAMS) -> i32);
    lineInitializeExA(lphlineapp, hinstance.param().abi(), lpfncallback, lpszfriendlyappname.param().abi(), lpdwnumdevs, lpdwapiversion, lplineinitializeexparams)
}
#[inline]
pub unsafe fn lineInitializeExW<P0, P1>(lphlineapp: *mut u32, hinstance: P0, lpfncallback: LINECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineInitializeExW(lphlineapp : *mut u32, hinstance : super::super::Foundation:: HINSTANCE, lpfncallback : LINECALLBACK, lpszfriendlyappname : windows_core::PCWSTR, lpdwnumdevs : *mut u32, lpdwapiversion : *mut u32, lplineinitializeexparams : *mut LINEINITIALIZEEXPARAMS) -> i32);
    lineInitializeExW(lphlineapp, hinstance.param().abi(), lpfncallback, lpszfriendlyappname.param().abi(), lpdwnumdevs, lpdwapiversion, lplineinitializeexparams)
}
#[inline]
pub unsafe fn lineMakeCall<P0>(hline: u32, lphcall: *mut u32, lpszdestaddress: P0, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineMakeCall(hline : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineMakeCall(hline, lphcall, lpszdestaddress.param().abi(), dwcountrycode, lpcallparams)
}
#[inline]
pub unsafe fn lineMakeCallA<P0>(hline: u32, lphcall: *mut u32, lpszdestaddress: P0, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineMakeCallA(hline : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineMakeCallA(hline, lphcall, lpszdestaddress.param().abi(), dwcountrycode, lpcallparams)
}
#[inline]
pub unsafe fn lineMakeCallW<P0>(hline: u32, lphcall: *mut u32, lpszdestaddress: P0, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineMakeCallW(hline : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCWSTR, dwcountrycode : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineMakeCallW(hline, lphcall, lpszdestaddress.param().abi(), dwcountrycode, lpcallparams)
}
#[inline]
pub unsafe fn lineMonitorDigits(hcall: u32, dwdigitmodes: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineMonitorDigits(hcall : u32, dwdigitmodes : u32) -> i32);
    lineMonitorDigits(hcall, dwdigitmodes)
}
#[inline]
pub unsafe fn lineMonitorMedia(hcall: u32, dwmediamodes: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineMonitorMedia(hcall : u32, dwmediamodes : u32) -> i32);
    lineMonitorMedia(hcall, dwmediamodes)
}
#[inline]
pub unsafe fn lineMonitorTones(hcall: u32, lptonelist: *const LINEMONITORTONE, dwnumentries: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineMonitorTones(hcall : u32, lptonelist : *const LINEMONITORTONE, dwnumentries : u32) -> i32);
    lineMonitorTones(hcall, lptonelist, dwnumentries)
}
#[inline]
pub unsafe fn lineNegotiateAPIVersion(hlineapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut LINEEXTENSIONID) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineNegotiateAPIVersion(hlineapp : u32, dwdeviceid : u32, dwapilowversion : u32, dwapihighversion : u32, lpdwapiversion : *mut u32, lpextensionid : *mut LINEEXTENSIONID) -> i32);
    lineNegotiateAPIVersion(hlineapp, dwdeviceid, dwapilowversion, dwapihighversion, lpdwapiversion, lpextensionid)
}
#[inline]
pub unsafe fn lineNegotiateExtVersion(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineNegotiateExtVersion(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextlowversion : u32, dwexthighversion : u32, lpdwextversion : *mut u32) -> i32);
    lineNegotiateExtVersion(hlineapp, dwdeviceid, dwapiversion, dwextlowversion, dwexthighversion, lpdwextversion)
}
#[inline]
pub unsafe fn lineOpen(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineOpen(hlineapp : u32, dwdeviceid : u32, lphline : *mut u32, dwapiversion : u32, dwextversion : u32, dwcallbackinstance : usize, dwprivileges : u32, dwmediamodes : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineOpen(hlineapp, dwdeviceid, lphline, dwapiversion, dwextversion, dwcallbackinstance, dwprivileges, dwmediamodes, lpcallparams)
}
#[inline]
pub unsafe fn lineOpenA(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineOpenA(hlineapp : u32, dwdeviceid : u32, lphline : *mut u32, dwapiversion : u32, dwextversion : u32, dwcallbackinstance : usize, dwprivileges : u32, dwmediamodes : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineOpenA(hlineapp, dwdeviceid, lphline, dwapiversion, dwextversion, dwcallbackinstance, dwprivileges, dwmediamodes, lpcallparams)
}
#[inline]
pub unsafe fn lineOpenW(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineOpenW(hlineapp : u32, dwdeviceid : u32, lphline : *mut u32, dwapiversion : u32, dwextversion : u32, dwcallbackinstance : usize, dwprivileges : u32, dwmediamodes : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineOpenW(hlineapp, dwdeviceid, lphline, dwapiversion, dwextversion, dwcallbackinstance, dwprivileges, dwmediamodes, lpcallparams)
}
#[inline]
pub unsafe fn linePark<P0>(hcall: u32, dwparkmode: u32, lpszdiraddress: P0, lpnondiraddress: *mut VARSTRING) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn linePark(hcall : u32, dwparkmode : u32, lpszdiraddress : windows_core::PCSTR, lpnondiraddress : *mut VARSTRING) -> i32);
    linePark(hcall, dwparkmode, lpszdiraddress.param().abi(), lpnondiraddress)
}
#[inline]
pub unsafe fn lineParkA<P0>(hcall: u32, dwparkmode: u32, lpszdiraddress: P0, lpnondiraddress: *mut VARSTRING) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineParkA(hcall : u32, dwparkmode : u32, lpszdiraddress : windows_core::PCSTR, lpnondiraddress : *mut VARSTRING) -> i32);
    lineParkA(hcall, dwparkmode, lpszdiraddress.param().abi(), lpnondiraddress)
}
#[inline]
pub unsafe fn lineParkW<P0>(hcall: u32, dwparkmode: u32, lpszdiraddress: P0, lpnondiraddress: *mut VARSTRING) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineParkW(hcall : u32, dwparkmode : u32, lpszdiraddress : windows_core::PCWSTR, lpnondiraddress : *mut VARSTRING) -> i32);
    lineParkW(hcall, dwparkmode, lpszdiraddress.param().abi(), lpnondiraddress)
}
#[inline]
pub unsafe fn linePickup<P0, P1>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0, lpszgroupid: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn linePickup(hline : u32, dwaddressid : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCSTR, lpszgroupid : windows_core::PCSTR) -> i32);
    linePickup(hline, dwaddressid, lphcall, lpszdestaddress.param().abi(), lpszgroupid.param().abi())
}
#[inline]
pub unsafe fn linePickupA<P0, P1>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0, lpszgroupid: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn linePickupA(hline : u32, dwaddressid : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCSTR, lpszgroupid : windows_core::PCSTR) -> i32);
    linePickupA(hline, dwaddressid, lphcall, lpszdestaddress.param().abi(), lpszgroupid.param().abi())
}
#[inline]
pub unsafe fn linePickupW<P0, P1>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0, lpszgroupid: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn linePickupW(hline : u32, dwaddressid : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCWSTR, lpszgroupid : windows_core::PCWSTR) -> i32);
    linePickupW(hline, dwaddressid, lphcall, lpszdestaddress.param().abi(), lpszgroupid.param().abi())
}
#[inline]
pub unsafe fn linePrepareAddToConference(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn linePrepareAddToConference(hconfcall : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    linePrepareAddToConference(hconfcall, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn linePrepareAddToConferenceA(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn linePrepareAddToConferenceA(hconfcall : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    linePrepareAddToConferenceA(hconfcall, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn linePrepareAddToConferenceW(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn linePrepareAddToConferenceW(hconfcall : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    linePrepareAddToConferenceW(hconfcall, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn lineProxyMessage(hline: u32, hcall: u32, dwmsg: u32, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineProxyMessage(hline : u32, hcall : u32, dwmsg : u32, dwparam1 : u32, dwparam2 : u32, dwparam3 : u32) -> i32);
    lineProxyMessage(hline, hcall, dwmsg, dwparam1, dwparam2, dwparam3)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn lineProxyResponse(hline: u32, lpproxyrequest: *mut LINEPROXYREQUEST, dwresult: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineProxyResponse(hline : u32, lpproxyrequest : *mut LINEPROXYREQUEST, dwresult : u32) -> i32);
    lineProxyResponse(hline, lpproxyrequest, dwresult)
}
#[inline]
pub unsafe fn lineRedirect<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineRedirect(hcall : u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32) -> i32);
    lineRedirect(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineRedirectA<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineRedirectA(hcall : u32, lpszdestaddress : windows_core::PCSTR, dwcountrycode : u32) -> i32);
    lineRedirectA(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineRedirectW<P0>(hcall: u32, lpszdestaddress: P0, dwcountrycode: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineRedirectW(hcall : u32, lpszdestaddress : windows_core::PCWSTR, dwcountrycode : u32) -> i32);
    lineRedirectW(hcall, lpszdestaddress.param().abi(), dwcountrycode)
}
#[inline]
pub unsafe fn lineRegisterRequestRecipient(hlineapp: u32, dwregistrationinstance: u32, dwrequestmode: u32, benable: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineRegisterRequestRecipient(hlineapp : u32, dwregistrationinstance : u32, dwrequestmode : u32, benable : u32) -> i32);
    lineRegisterRequestRecipient(hlineapp, dwregistrationinstance, dwrequestmode, benable)
}
#[inline]
pub unsafe fn lineReleaseUserUserInfo(hcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineReleaseUserUserInfo(hcall : u32) -> i32);
    lineReleaseUserUserInfo(hcall)
}
#[inline]
pub unsafe fn lineRemoveFromConference(hcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineRemoveFromConference(hcall : u32) -> i32);
    lineRemoveFromConference(hcall)
}
#[inline]
pub unsafe fn lineRemoveProvider<P0>(dwpermanentproviderid: u32, hwndowner: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineRemoveProvider(dwpermanentproviderid : u32, hwndowner : super::super::Foundation:: HWND) -> i32);
    lineRemoveProvider(dwpermanentproviderid, hwndowner.param().abi())
}
#[inline]
pub unsafe fn lineSecureCall(hcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSecureCall(hcall : u32) -> i32);
    lineSecureCall(hcall)
}
#[inline]
pub unsafe fn lineSendUserUserInfo<P0>(hcall: u32, lpsuseruserinfo: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSendUserUserInfo(hcall : u32, lpsuseruserinfo : windows_core::PCSTR, dwsize : u32) -> i32);
    lineSendUserUserInfo(hcall, lpsuseruserinfo.param().abi(), dwsize)
}
#[inline]
pub unsafe fn lineSetAgentActivity(hline: u32, dwaddressid: u32, dwactivityid: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetAgentActivity(hline : u32, dwaddressid : u32, dwactivityid : u32) -> i32);
    lineSetAgentActivity(hline, dwaddressid, dwactivityid)
}
#[inline]
pub unsafe fn lineSetAgentGroup(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetAgentGroup(hline : u32, dwaddressid : u32, lpagentgrouplist : *mut LINEAGENTGROUPLIST) -> i32);
    lineSetAgentGroup(hline, dwaddressid, lpagentgrouplist)
}
#[inline]
pub unsafe fn lineSetAgentMeasurementPeriod(hline: u32, hagent: u32, dwmeasurementperiod: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetAgentMeasurementPeriod(hline : u32, hagent : u32, dwmeasurementperiod : u32) -> i32);
    lineSetAgentMeasurementPeriod(hline, hagent, dwmeasurementperiod)
}
#[inline]
pub unsafe fn lineSetAgentSessionState(hline: u32, hagentsession: u32, dwagentsessionstate: u32, dwnextagentsessionstate: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetAgentSessionState(hline : u32, hagentsession : u32, dwagentsessionstate : u32, dwnextagentsessionstate : u32) -> i32);
    lineSetAgentSessionState(hline, hagentsession, dwagentsessionstate, dwnextagentsessionstate)
}
#[inline]
pub unsafe fn lineSetAgentState(hline: u32, dwaddressid: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetAgentState(hline : u32, dwaddressid : u32, dwagentstate : u32, dwnextagentstate : u32) -> i32);
    lineSetAgentState(hline, dwaddressid, dwagentstate, dwnextagentstate)
}
#[inline]
pub unsafe fn lineSetAgentStateEx(hline: u32, hagent: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetAgentStateEx(hline : u32, hagent : u32, dwagentstate : u32, dwnextagentstate : u32) -> i32);
    lineSetAgentStateEx(hline, hagent, dwagentstate, dwnextagentstate)
}
#[inline]
pub unsafe fn lineSetAppPriority<P0, P1>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: P1, dwpriority: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetAppPriority(lpszappfilename : windows_core::PCSTR, dwmediamode : u32, lpextensionid : *mut LINEEXTENSIONID, dwrequestmode : u32, lpszextensionname : windows_core::PCSTR, dwpriority : u32) -> i32);
    lineSetAppPriority(lpszappfilename.param().abi(), dwmediamode, lpextensionid, dwrequestmode, lpszextensionname.param().abi(), dwpriority)
}
#[inline]
pub unsafe fn lineSetAppPriorityA<P0, P1>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: P1, dwpriority: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetAppPriorityA(lpszappfilename : windows_core::PCSTR, dwmediamode : u32, lpextensionid : *mut LINEEXTENSIONID, dwrequestmode : u32, lpszextensionname : windows_core::PCSTR, dwpriority : u32) -> i32);
    lineSetAppPriorityA(lpszappfilename.param().abi(), dwmediamode, lpextensionid, dwrequestmode, lpszextensionname.param().abi(), dwpriority)
}
#[inline]
pub unsafe fn lineSetAppPriorityW<P0, P1>(lpszappfilename: P0, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: P1, dwpriority: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetAppPriorityW(lpszappfilename : windows_core::PCWSTR, dwmediamode : u32, lpextensionid : *mut LINEEXTENSIONID, dwrequestmode : u32, lpszextensionname : windows_core::PCWSTR, dwpriority : u32) -> i32);
    lineSetAppPriorityW(lpszappfilename.param().abi(), dwmediamode, lpextensionid, dwrequestmode, lpszextensionname.param().abi(), dwpriority)
}
#[inline]
pub unsafe fn lineSetAppSpecific(hcall: u32, dwappspecific: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetAppSpecific(hcall : u32, dwappspecific : u32) -> i32);
    lineSetAppSpecific(hcall, dwappspecific)
}
#[inline]
pub unsafe fn lineSetCallData(hcall: u32, lpcalldata: *mut core::ffi::c_void, dwsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetCallData(hcall : u32, lpcalldata : *mut core::ffi::c_void, dwsize : u32) -> i32);
    lineSetCallData(hcall, lpcalldata, dwsize)
}
#[inline]
pub unsafe fn lineSetCallParams(hcall: u32, dwbearermode: u32, dwminrate: u32, dwmaxrate: u32, lpdialparams: *const LINEDIALPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetCallParams(hcall : u32, dwbearermode : u32, dwminrate : u32, dwmaxrate : u32, lpdialparams : *const LINEDIALPARAMS) -> i32);
    lineSetCallParams(hcall, dwbearermode, dwminrate, dwmaxrate, lpdialparams)
}
#[inline]
pub unsafe fn lineSetCallPrivilege(hcall: u32, dwcallprivilege: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetCallPrivilege(hcall : u32, dwcallprivilege : u32) -> i32);
    lineSetCallPrivilege(hcall, dwcallprivilege)
}
#[inline]
pub unsafe fn lineSetCallQualityOfService(hcall: u32, lpsendingflowspec: *mut core::ffi::c_void, dwsendingflowspecsize: u32, lpreceivingflowspec: *mut core::ffi::c_void, dwreceivingflowspecsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetCallQualityOfService(hcall : u32, lpsendingflowspec : *mut core::ffi::c_void, dwsendingflowspecsize : u32, lpreceivingflowspec : *mut core::ffi::c_void, dwreceivingflowspecsize : u32) -> i32);
    lineSetCallQualityOfService(hcall, lpsendingflowspec, dwsendingflowspecsize, lpreceivingflowspec, dwreceivingflowspecsize)
}
#[inline]
pub unsafe fn lineSetCallTreatment(hcall: u32, dwtreatment: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetCallTreatment(hcall : u32, dwtreatment : u32) -> i32);
    lineSetCallTreatment(hcall, dwtreatment)
}
#[inline]
pub unsafe fn lineSetCurrentLocation(hlineapp: u32, dwlocation: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetCurrentLocation(hlineapp : u32, dwlocation : u32) -> i32);
    lineSetCurrentLocation(hlineapp, dwlocation)
}
#[inline]
pub unsafe fn lineSetDevConfig<P0>(dwdeviceid: u32, lpdeviceconfig: *const core::ffi::c_void, dwsize: u32, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetDevConfig(dwdeviceid : u32, lpdeviceconfig : *const core::ffi::c_void, dwsize : u32, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineSetDevConfig(dwdeviceid, lpdeviceconfig, dwsize, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineSetDevConfigA<P0>(dwdeviceid: u32, lpdeviceconfig: *const core::ffi::c_void, dwsize: u32, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetDevConfigA(dwdeviceid : u32, lpdeviceconfig : *const core::ffi::c_void, dwsize : u32, lpszdeviceclass : windows_core::PCSTR) -> i32);
    lineSetDevConfigA(dwdeviceid, lpdeviceconfig, dwsize, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineSetDevConfigW<P0>(dwdeviceid: u32, lpdeviceconfig: *const core::ffi::c_void, dwsize: u32, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetDevConfigW(dwdeviceid : u32, lpdeviceconfig : *const core::ffi::c_void, dwsize : u32, lpszdeviceclass : windows_core::PCWSTR) -> i32);
    lineSetDevConfigW(dwdeviceid, lpdeviceconfig, dwsize, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn lineSetLineDevStatus(hline: u32, dwstatustochange: u32, fstatus: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetLineDevStatus(hline : u32, dwstatustochange : u32, fstatus : u32) -> i32);
    lineSetLineDevStatus(hline, dwstatustochange, fstatus)
}
#[inline]
pub unsafe fn lineSetMediaControl(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdigitlist: *const LINEMEDIACONTROLDIGIT, dwdigitnumentries: u32, lpmedialist: *const LINEMEDIACONTROLMEDIA, dwmedianumentries: u32, lptonelist: *const LINEMEDIACONTROLTONE, dwtonenumentries: u32, lpcallstatelist: *const LINEMEDIACONTROLCALLSTATE, dwcallstatenumentries: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetMediaControl(hline : u32, dwaddressid : u32, hcall : u32, dwselect : u32, lpdigitlist : *const LINEMEDIACONTROLDIGIT, dwdigitnumentries : u32, lpmedialist : *const LINEMEDIACONTROLMEDIA, dwmedianumentries : u32, lptonelist : *const LINEMEDIACONTROLTONE, dwtonenumentries : u32, lpcallstatelist : *const LINEMEDIACONTROLCALLSTATE, dwcallstatenumentries : u32) -> i32);
    lineSetMediaControl(hline, dwaddressid, hcall, dwselect, lpdigitlist, dwdigitnumentries, lpmedialist, dwmedianumentries, lptonelist, dwtonenumentries, lpcallstatelist, dwcallstatenumentries)
}
#[inline]
pub unsafe fn lineSetMediaMode(hcall: u32, dwmediamodes: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetMediaMode(hcall : u32, dwmediamodes : u32) -> i32);
    lineSetMediaMode(hcall, dwmediamodes)
}
#[inline]
pub unsafe fn lineSetNumRings(hline: u32, dwaddressid: u32, dwnumrings: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetNumRings(hline : u32, dwaddressid : u32, dwnumrings : u32) -> i32);
    lineSetNumRings(hline, dwaddressid, dwnumrings)
}
#[inline]
pub unsafe fn lineSetQueueMeasurementPeriod(hline: u32, dwqueueid: u32, dwmeasurementperiod: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetQueueMeasurementPeriod(hline : u32, dwqueueid : u32, dwmeasurementperiod : u32) -> i32);
    lineSetQueueMeasurementPeriod(hline, dwqueueid, dwmeasurementperiod)
}
#[inline]
pub unsafe fn lineSetStatusMessages(hline: u32, dwlinestates: u32, dwaddressstates: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetStatusMessages(hline : u32, dwlinestates : u32, dwaddressstates : u32) -> i32);
    lineSetStatusMessages(hline, dwlinestates, dwaddressstates)
}
#[inline]
pub unsafe fn lineSetTerminal(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, dwterminalmodes: u32, dwterminalid: u32, benable: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetTerminal(hline : u32, dwaddressid : u32, hcall : u32, dwselect : u32, dwterminalmodes : u32, dwterminalid : u32, benable : u32) -> i32);
    lineSetTerminal(hline, dwaddressid, hcall, dwselect, dwterminalmodes, dwterminalid, benable)
}
#[inline]
pub unsafe fn lineSetTollList<P0>(hlineapp: u32, dwdeviceid: u32, lpszaddressin: P0, dwtolllistoption: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetTollList(hlineapp : u32, dwdeviceid : u32, lpszaddressin : windows_core::PCSTR, dwtolllistoption : u32) -> i32);
    lineSetTollList(hlineapp, dwdeviceid, lpszaddressin.param().abi(), dwtolllistoption)
}
#[inline]
pub unsafe fn lineSetTollListA<P0>(hlineapp: u32, dwdeviceid: u32, lpszaddressin: P0, dwtolllistoption: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetTollListA(hlineapp : u32, dwdeviceid : u32, lpszaddressin : windows_core::PCSTR, dwtolllistoption : u32) -> i32);
    lineSetTollListA(hlineapp, dwdeviceid, lpszaddressin.param().abi(), dwtolllistoption)
}
#[inline]
pub unsafe fn lineSetTollListW<P0>(hlineapp: u32, dwdeviceid: u32, lpszaddressinw: P0, dwtolllistoption: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineSetTollListW(hlineapp : u32, dwdeviceid : u32, lpszaddressinw : windows_core::PCWSTR, dwtolllistoption : u32) -> i32);
    lineSetTollListW(hlineapp, dwdeviceid, lpszaddressinw.param().abi(), dwtolllistoption)
}
#[inline]
pub unsafe fn lineSetupConference(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetupConference(hcall : u32, hline : u32, lphconfcall : *mut u32, lphconsultcall : *mut u32, dwnumparties : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineSetupConference(hcall, hline, lphconfcall, lphconsultcall, dwnumparties, lpcallparams)
}
#[inline]
pub unsafe fn lineSetupConferenceA(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetupConferenceA(hcall : u32, hline : u32, lphconfcall : *mut u32, lphconsultcall : *mut u32, dwnumparties : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineSetupConferenceA(hcall, hline, lphconfcall, lphconsultcall, dwnumparties, lpcallparams)
}
#[inline]
pub unsafe fn lineSetupConferenceW(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetupConferenceW(hcall : u32, hline : u32, lphconfcall : *mut u32, lphconsultcall : *mut u32, dwnumparties : u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineSetupConferenceW(hcall, hline, lphconfcall, lphconsultcall, dwnumparties, lpcallparams)
}
#[inline]
pub unsafe fn lineSetupTransfer(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetupTransfer(hcall : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineSetupTransfer(hcall, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn lineSetupTransferA(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetupTransferA(hcall : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineSetupTransferA(hcall, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn lineSetupTransferW(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSetupTransferW(hcall : u32, lphconsultcall : *mut u32, lpcallparams : *const LINECALLPARAMS) -> i32);
    lineSetupTransferW(hcall, lphconsultcall, lpcallparams)
}
#[inline]
pub unsafe fn lineShutdown(hlineapp: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineShutdown(hlineapp : u32) -> i32);
    lineShutdown(hlineapp)
}
#[inline]
pub unsafe fn lineSwapHold(hactivecall: u32, hheldcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineSwapHold(hactivecall : u32, hheldcall : u32) -> i32);
    lineSwapHold(hactivecall, hheldcall)
}
#[inline]
pub unsafe fn lineTranslateAddress<P0>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: P0, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineTranslateAddress(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, lpszaddressin : windows_core::PCSTR, dwcard : u32, dwtranslateoptions : u32, lptranslateoutput : *mut LINETRANSLATEOUTPUT) -> i32);
    lineTranslateAddress(hlineapp, dwdeviceid, dwapiversion, lpszaddressin.param().abi(), dwcard, dwtranslateoptions, lptranslateoutput)
}
#[inline]
pub unsafe fn lineTranslateAddressA<P0>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: P0, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineTranslateAddressA(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, lpszaddressin : windows_core::PCSTR, dwcard : u32, dwtranslateoptions : u32, lptranslateoutput : *mut LINETRANSLATEOUTPUT) -> i32);
    lineTranslateAddressA(hlineapp, dwdeviceid, dwapiversion, lpszaddressin.param().abi(), dwcard, dwtranslateoptions, lptranslateoutput)
}
#[inline]
pub unsafe fn lineTranslateAddressW<P0>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: P0, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineTranslateAddressW(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, lpszaddressin : windows_core::PCWSTR, dwcard : u32, dwtranslateoptions : u32, lptranslateoutput : *mut LINETRANSLATEOUTPUT) -> i32);
    lineTranslateAddressW(hlineapp, dwdeviceid, dwapiversion, lpszaddressin.param().abi(), dwcard, dwtranslateoptions, lptranslateoutput)
}
#[inline]
pub unsafe fn lineTranslateDialog<P0, P1>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: P0, lpszaddressin: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineTranslateDialog(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, hwndowner : super::super::Foundation:: HWND, lpszaddressin : windows_core::PCSTR) -> i32);
    lineTranslateDialog(hlineapp, dwdeviceid, dwapiversion, hwndowner.param().abi(), lpszaddressin.param().abi())
}
#[inline]
pub unsafe fn lineTranslateDialogA<P0, P1>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: P0, lpszaddressin: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineTranslateDialogA(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, hwndowner : super::super::Foundation:: HWND, lpszaddressin : windows_core::PCSTR) -> i32);
    lineTranslateDialogA(hlineapp, dwdeviceid, dwapiversion, hwndowner.param().abi(), lpszaddressin.param().abi())
}
#[inline]
pub unsafe fn lineTranslateDialogW<P0, P1>(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: P0, lpszaddressin: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineTranslateDialogW(hlineapp : u32, dwdeviceid : u32, dwapiversion : u32, hwndowner : super::super::Foundation:: HWND, lpszaddressin : windows_core::PCWSTR) -> i32);
    lineTranslateDialogW(hlineapp, dwdeviceid, dwapiversion, hwndowner.param().abi(), lpszaddressin.param().abi())
}
#[inline]
pub unsafe fn lineUncompleteCall(hline: u32, dwcompletionid: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineUncompleteCall(hline : u32, dwcompletionid : u32) -> i32);
    lineUncompleteCall(hline, dwcompletionid)
}
#[inline]
pub unsafe fn lineUnhold(hcall: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn lineUnhold(hcall : u32) -> i32);
    lineUnhold(hcall)
}
#[inline]
pub unsafe fn lineUnpark<P0>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineUnpark(hline : u32, dwaddressid : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCSTR) -> i32);
    lineUnpark(hline, dwaddressid, lphcall, lpszdestaddress.param().abi())
}
#[inline]
pub unsafe fn lineUnparkA<P0>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineUnparkA(hline : u32, dwaddressid : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCSTR) -> i32);
    lineUnparkA(hline, dwaddressid, lphcall, lpszdestaddress.param().abi())
}
#[inline]
pub unsafe fn lineUnparkW<P0>(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn lineUnparkW(hline : u32, dwaddressid : u32, lphcall : *mut u32, lpszdestaddress : windows_core::PCWSTR) -> i32);
    lineUnparkW(hline, dwaddressid, lphcall, lpszdestaddress.param().abi())
}
#[inline]
pub unsafe fn phoneClose(hphone: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneClose(hphone : u32) -> i32);
    phoneClose(hphone)
}
#[inline]
pub unsafe fn phoneConfigDialog<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneConfigDialog(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCSTR) -> i32);
    phoneConfigDialog(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn phoneConfigDialogA<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneConfigDialogA(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCSTR) -> i32);
    phoneConfigDialogA(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn phoneConfigDialogW<P0, P1>(dwdeviceid: u32, hwndowner: P0, lpszdeviceclass: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneConfigDialogW(dwdeviceid : u32, hwndowner : super::super::Foundation:: HWND, lpszdeviceclass : windows_core::PCWSTR) -> i32);
    phoneConfigDialogW(dwdeviceid, hwndowner.param().abi(), lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn phoneDevSpecific(hphone: u32, lpparams: *mut core::ffi::c_void, dwsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneDevSpecific(hphone : u32, lpparams : *mut core::ffi::c_void, dwsize : u32) -> i32);
    phoneDevSpecific(hphone, lpparams, dwsize)
}
#[inline]
pub unsafe fn phoneGetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetButtonInfo(hphone : u32, dwbuttonlampid : u32, lpbuttoninfo : *mut PHONEBUTTONINFO) -> i32);
    phoneGetButtonInfo(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[inline]
pub unsafe fn phoneGetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetButtonInfoA(hphone : u32, dwbuttonlampid : u32, lpbuttoninfo : *mut PHONEBUTTONINFO) -> i32);
    phoneGetButtonInfoA(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[inline]
pub unsafe fn phoneGetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetButtonInfoW(hphone : u32, dwbuttonlampid : u32, lpbuttoninfo : *mut PHONEBUTTONINFO) -> i32);
    phoneGetButtonInfoW(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[inline]
pub unsafe fn phoneGetData(hphone: u32, dwdataid: u32, lpdata: *mut core::ffi::c_void, dwsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetData(hphone : u32, dwdataid : u32, lpdata : *mut core::ffi::c_void, dwsize : u32) -> i32);
    phoneGetData(hphone, dwdataid, lpdata, dwsize)
}
#[inline]
pub unsafe fn phoneGetDevCaps(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetDevCaps(hphoneapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextversion : u32, lpphonecaps : *mut PHONECAPS) -> i32);
    phoneGetDevCaps(hphoneapp, dwdeviceid, dwapiversion, dwextversion, lpphonecaps)
}
#[inline]
pub unsafe fn phoneGetDevCapsA(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetDevCapsA(hphoneapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextversion : u32, lpphonecaps : *mut PHONECAPS) -> i32);
    phoneGetDevCapsA(hphoneapp, dwdeviceid, dwapiversion, dwextversion, lpphonecaps)
}
#[inline]
pub unsafe fn phoneGetDevCapsW(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetDevCapsW(hphoneapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextversion : u32, lpphonecaps : *mut PHONECAPS) -> i32);
    phoneGetDevCapsW(hphoneapp, dwdeviceid, dwapiversion, dwextversion, lpphonecaps)
}
#[inline]
pub unsafe fn phoneGetDisplay(hphone: u32, lpdisplay: *mut VARSTRING) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetDisplay(hphone : u32, lpdisplay : *mut VARSTRING) -> i32);
    phoneGetDisplay(hphone, lpdisplay)
}
#[inline]
pub unsafe fn phoneGetGain(hphone: u32, dwhookswitchdev: u32, lpdwgain: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetGain(hphone : u32, dwhookswitchdev : u32, lpdwgain : *mut u32) -> i32);
    phoneGetGain(hphone, dwhookswitchdev, lpdwgain)
}
#[inline]
pub unsafe fn phoneGetHookSwitch(hphone: u32, lpdwhookswitchdevs: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetHookSwitch(hphone : u32, lpdwhookswitchdevs : *mut u32) -> i32);
    phoneGetHookSwitch(hphone, lpdwhookswitchdevs)
}
#[inline]
pub unsafe fn phoneGetID<P0>(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneGetID(hphone : u32, lpdeviceid : *mut VARSTRING, lpszdeviceclass : windows_core::PCSTR) -> i32);
    phoneGetID(hphone, lpdeviceid, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn phoneGetIDA<P0>(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneGetIDA(hphone : u32, lpdeviceid : *mut VARSTRING, lpszdeviceclass : windows_core::PCSTR) -> i32);
    phoneGetIDA(hphone, lpdeviceid, lpszdeviceclass.param().abi())
}
#[inline]
pub unsafe fn phoneGetIDW<P0>(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneGetIDW(hphone : u32, lpdeviceid : *mut VARSTRING, lpszdeviceclass : windows_core::PCWSTR) -> i32);
    phoneGetIDW(hphone, lpdeviceid, lpszdeviceclass.param().abi())
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn phoneGetIcon<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneGetIcon(dwdeviceid : u32, lpszdeviceclass : windows_core::PCSTR, lphicon : *mut super::super::UI::WindowsAndMessaging:: HICON) -> i32);
    phoneGetIcon(dwdeviceid, lpszdeviceclass.param().abi(), lphicon)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn phoneGetIconA<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneGetIconA(dwdeviceid : u32, lpszdeviceclass : windows_core::PCSTR, lphicon : *mut super::super::UI::WindowsAndMessaging:: HICON) -> i32);
    phoneGetIconA(dwdeviceid, lpszdeviceclass.param().abi(), lphicon)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn phoneGetIconW<P0>(dwdeviceid: u32, lpszdeviceclass: P0, lphicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneGetIconW(dwdeviceid : u32, lpszdeviceclass : windows_core::PCWSTR, lphicon : *mut super::super::UI::WindowsAndMessaging:: HICON) -> i32);
    phoneGetIconW(dwdeviceid, lpszdeviceclass.param().abi(), lphicon)
}
#[inline]
pub unsafe fn phoneGetLamp(hphone: u32, dwbuttonlampid: u32, lpdwlampmode: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetLamp(hphone : u32, dwbuttonlampid : u32, lpdwlampmode : *mut u32) -> i32);
    phoneGetLamp(hphone, dwbuttonlampid, lpdwlampmode)
}
#[inline]
pub unsafe fn phoneGetMessage(hphoneapp: u32, lpmessage: *mut PHONEMESSAGE, dwtimeout: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetMessage(hphoneapp : u32, lpmessage : *mut PHONEMESSAGE, dwtimeout : u32) -> i32);
    phoneGetMessage(hphoneapp, lpmessage, dwtimeout)
}
#[inline]
pub unsafe fn phoneGetRing(hphone: u32, lpdwringmode: *mut u32, lpdwvolume: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetRing(hphone : u32, lpdwringmode : *mut u32, lpdwvolume : *mut u32) -> i32);
    phoneGetRing(hphone, lpdwringmode, lpdwvolume)
}
#[inline]
pub unsafe fn phoneGetStatus(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetStatus(hphone : u32, lpphonestatus : *mut PHONESTATUS) -> i32);
    phoneGetStatus(hphone, lpphonestatus)
}
#[inline]
pub unsafe fn phoneGetStatusA(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetStatusA(hphone : u32, lpphonestatus : *mut PHONESTATUS) -> i32);
    phoneGetStatusA(hphone, lpphonestatus)
}
#[inline]
pub unsafe fn phoneGetStatusMessages(hphone: u32, lpdwphonestates: *mut u32, lpdwbuttonmodes: *mut u32, lpdwbuttonstates: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetStatusMessages(hphone : u32, lpdwphonestates : *mut u32, lpdwbuttonmodes : *mut u32, lpdwbuttonstates : *mut u32) -> i32);
    phoneGetStatusMessages(hphone, lpdwphonestates, lpdwbuttonmodes, lpdwbuttonstates)
}
#[inline]
pub unsafe fn phoneGetStatusW(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetStatusW(hphone : u32, lpphonestatus : *mut PHONESTATUS) -> i32);
    phoneGetStatusW(hphone, lpphonestatus)
}
#[inline]
pub unsafe fn phoneGetVolume(hphone: u32, dwhookswitchdev: u32, lpdwvolume: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneGetVolume(hphone : u32, dwhookswitchdev : u32, lpdwvolume : *mut u32) -> i32);
    phoneGetVolume(hphone, dwhookswitchdev, lpdwvolume)
}
#[inline]
pub unsafe fn phoneInitialize<P0, P1>(lphphoneapp: *mut u32, hinstance: P0, lpfncallback: PHONECALLBACK, lpszappname: P1, lpdwnumdevs: *mut u32) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneInitialize(lphphoneapp : *mut u32, hinstance : super::super::Foundation:: HINSTANCE, lpfncallback : PHONECALLBACK, lpszappname : windows_core::PCSTR, lpdwnumdevs : *mut u32) -> i32);
    phoneInitialize(lphphoneapp, hinstance.param().abi(), lpfncallback, lpszappname.param().abi(), lpdwnumdevs)
}
#[inline]
pub unsafe fn phoneInitializeExA<P0, P1>(lphphoneapp: *mut u32, hinstance: P0, lpfncallback: PHONECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneInitializeExA(lphphoneapp : *mut u32, hinstance : super::super::Foundation:: HINSTANCE, lpfncallback : PHONECALLBACK, lpszfriendlyappname : windows_core::PCSTR, lpdwnumdevs : *mut u32, lpdwapiversion : *mut u32, lpphoneinitializeexparams : *mut PHONEINITIALIZEEXPARAMS) -> i32);
    phoneInitializeExA(lphphoneapp, hinstance.param().abi(), lpfncallback, lpszfriendlyappname.param().abi(), lpdwnumdevs, lpdwapiversion, lpphoneinitializeexparams)
}
#[inline]
pub unsafe fn phoneInitializeExW<P0, P1>(lphphoneapp: *mut u32, hinstance: P0, lpfncallback: PHONECALLBACK, lpszfriendlyappname: P1, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneInitializeExW(lphphoneapp : *mut u32, hinstance : super::super::Foundation:: HINSTANCE, lpfncallback : PHONECALLBACK, lpszfriendlyappname : windows_core::PCWSTR, lpdwnumdevs : *mut u32, lpdwapiversion : *mut u32, lpphoneinitializeexparams : *mut PHONEINITIALIZEEXPARAMS) -> i32);
    phoneInitializeExW(lphphoneapp, hinstance.param().abi(), lpfncallback, lpszfriendlyappname.param().abi(), lpdwnumdevs, lpdwapiversion, lpphoneinitializeexparams)
}
#[inline]
pub unsafe fn phoneNegotiateAPIVersion(hphoneapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut PHONEEXTENSIONID) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneNegotiateAPIVersion(hphoneapp : u32, dwdeviceid : u32, dwapilowversion : u32, dwapihighversion : u32, lpdwapiversion : *mut u32, lpextensionid : *mut PHONEEXTENSIONID) -> i32);
    phoneNegotiateAPIVersion(hphoneapp, dwdeviceid, dwapilowversion, dwapihighversion, lpdwapiversion, lpextensionid)
}
#[inline]
pub unsafe fn phoneNegotiateExtVersion(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneNegotiateExtVersion(hphoneapp : u32, dwdeviceid : u32, dwapiversion : u32, dwextlowversion : u32, dwexthighversion : u32, lpdwextversion : *mut u32) -> i32);
    phoneNegotiateExtVersion(hphoneapp, dwdeviceid, dwapiversion, dwextlowversion, dwexthighversion, lpdwextversion)
}
#[inline]
pub unsafe fn phoneOpen(hphoneapp: u32, dwdeviceid: u32, lphphone: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivilege: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneOpen(hphoneapp : u32, dwdeviceid : u32, lphphone : *mut u32, dwapiversion : u32, dwextversion : u32, dwcallbackinstance : usize, dwprivilege : u32) -> i32);
    phoneOpen(hphoneapp, dwdeviceid, lphphone, dwapiversion, dwextversion, dwcallbackinstance, dwprivilege)
}
#[inline]
pub unsafe fn phoneSetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetButtonInfo(hphone : u32, dwbuttonlampid : u32, lpbuttoninfo : *const PHONEBUTTONINFO) -> i32);
    phoneSetButtonInfo(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[inline]
pub unsafe fn phoneSetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetButtonInfoA(hphone : u32, dwbuttonlampid : u32, lpbuttoninfo : *const PHONEBUTTONINFO) -> i32);
    phoneSetButtonInfoA(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[inline]
pub unsafe fn phoneSetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetButtonInfoW(hphone : u32, dwbuttonlampid : u32, lpbuttoninfo : *const PHONEBUTTONINFO) -> i32);
    phoneSetButtonInfoW(hphone, dwbuttonlampid, lpbuttoninfo)
}
#[inline]
pub unsafe fn phoneSetData(hphone: u32, dwdataid: u32, lpdata: *const core::ffi::c_void, dwsize: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetData(hphone : u32, dwdataid : u32, lpdata : *const core::ffi::c_void, dwsize : u32) -> i32);
    phoneSetData(hphone, dwdataid, lpdata, dwsize)
}
#[inline]
pub unsafe fn phoneSetDisplay<P0>(hphone: u32, dwrow: u32, dwcolumn: u32, lpsdisplay: P0, dwsize: u32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn phoneSetDisplay(hphone : u32, dwrow : u32, dwcolumn : u32, lpsdisplay : windows_core::PCSTR, dwsize : u32) -> i32);
    phoneSetDisplay(hphone, dwrow, dwcolumn, lpsdisplay.param().abi(), dwsize)
}
#[inline]
pub unsafe fn phoneSetGain(hphone: u32, dwhookswitchdev: u32, dwgain: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetGain(hphone : u32, dwhookswitchdev : u32, dwgain : u32) -> i32);
    phoneSetGain(hphone, dwhookswitchdev, dwgain)
}
#[inline]
pub unsafe fn phoneSetHookSwitch(hphone: u32, dwhookswitchdevs: u32, dwhookswitchmode: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetHookSwitch(hphone : u32, dwhookswitchdevs : u32, dwhookswitchmode : u32) -> i32);
    phoneSetHookSwitch(hphone, dwhookswitchdevs, dwhookswitchmode)
}
#[inline]
pub unsafe fn phoneSetLamp(hphone: u32, dwbuttonlampid: u32, dwlampmode: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetLamp(hphone : u32, dwbuttonlampid : u32, dwlampmode : u32) -> i32);
    phoneSetLamp(hphone, dwbuttonlampid, dwlampmode)
}
#[inline]
pub unsafe fn phoneSetRing(hphone: u32, dwringmode: u32, dwvolume: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetRing(hphone : u32, dwringmode : u32, dwvolume : u32) -> i32);
    phoneSetRing(hphone, dwringmode, dwvolume)
}
#[inline]
pub unsafe fn phoneSetStatusMessages(hphone: u32, dwphonestates: u32, dwbuttonmodes: u32, dwbuttonstates: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetStatusMessages(hphone : u32, dwphonestates : u32, dwbuttonmodes : u32, dwbuttonstates : u32) -> i32);
    phoneSetStatusMessages(hphone, dwphonestates, dwbuttonmodes, dwbuttonstates)
}
#[inline]
pub unsafe fn phoneSetVolume(hphone: u32, dwhookswitchdev: u32, dwvolume: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneSetVolume(hphone : u32, dwhookswitchdev : u32, dwvolume : u32) -> i32);
    phoneSetVolume(hphone, dwhookswitchdev, dwvolume)
}
#[inline]
pub unsafe fn phoneShutdown(hphoneapp: u32) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn phoneShutdown(hphoneapp : u32) -> i32);
    phoneShutdown(hphoneapp)
}
#[inline]
pub unsafe fn tapiGetLocationInfo(lpszcountrycode: &mut [u8; 8], lpszcitycode: &mut [u8; 8]) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn tapiGetLocationInfo(lpszcountrycode : windows_core::PSTR, lpszcitycode : windows_core::PSTR) -> i32);
    tapiGetLocationInfo(core::mem::transmute(lpszcountrycode.as_ptr()), core::mem::transmute(lpszcitycode.as_ptr()))
}
#[inline]
pub unsafe fn tapiGetLocationInfoA(lpszcountrycode: &mut [u8; 8], lpszcitycode: &mut [u8; 8]) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn tapiGetLocationInfoA(lpszcountrycode : windows_core::PSTR, lpszcitycode : windows_core::PSTR) -> i32);
    tapiGetLocationInfoA(core::mem::transmute(lpszcountrycode.as_ptr()), core::mem::transmute(lpszcitycode.as_ptr()))
}
#[inline]
pub unsafe fn tapiGetLocationInfoW(lpszcountrycodew: &mut [u16; 8], lpszcitycodew: &mut [u16; 8]) -> i32 {
    windows_targets::link!("tapi32.dll" "system" fn tapiGetLocationInfoW(lpszcountrycodew : windows_core::PWSTR, lpszcitycodew : windows_core::PWSTR) -> i32);
    tapiGetLocationInfoW(core::mem::transmute(lpszcountrycodew.as_ptr()), core::mem::transmute(lpszcitycodew.as_ptr()))
}
#[inline]
pub unsafe fn tapiRequestDrop<P0, P1>(hwnd: P0, wrequestid: P1) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
{
    windows_targets::link!("tapi32.dll" "system" fn tapiRequestDrop(hwnd : super::super::Foundation:: HWND, wrequestid : super::super::Foundation:: WPARAM) -> i32);
    tapiRequestDrop(hwnd.param().abi(), wrequestid.param().abi())
}
#[inline]
pub unsafe fn tapiRequestMakeCall<P0, P1, P2, P3>(lpszdestaddress: P0, lpszappname: P1, lpszcalledparty: P2, lpszcomment: P3) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn tapiRequestMakeCall(lpszdestaddress : windows_core::PCSTR, lpszappname : windows_core::PCSTR, lpszcalledparty : windows_core::PCSTR, lpszcomment : windows_core::PCSTR) -> i32);
    tapiRequestMakeCall(lpszdestaddress.param().abi(), lpszappname.param().abi(), lpszcalledparty.param().abi(), lpszcomment.param().abi())
}
#[inline]
pub unsafe fn tapiRequestMakeCallA<P0, P1, P2, P3>(lpszdestaddress: P0, lpszappname: P1, lpszcalledparty: P2, lpszcomment: P3) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn tapiRequestMakeCallA(lpszdestaddress : windows_core::PCSTR, lpszappname : windows_core::PCSTR, lpszcalledparty : windows_core::PCSTR, lpszcomment : windows_core::PCSTR) -> i32);
    tapiRequestMakeCallA(lpszdestaddress.param().abi(), lpszappname.param().abi(), lpszcalledparty.param().abi(), lpszcomment.param().abi())
}
#[inline]
pub unsafe fn tapiRequestMakeCallW<P0, P1, P2, P3>(lpszdestaddress: P0, lpszappname: P1, lpszcalledparty: P2, lpszcomment: P3) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn tapiRequestMakeCallW(lpszdestaddress : windows_core::PCWSTR, lpszappname : windows_core::PCWSTR, lpszcalledparty : windows_core::PCWSTR, lpszcomment : windows_core::PCWSTR) -> i32);
    tapiRequestMakeCallW(lpszdestaddress.param().abi(), lpszappname.param().abi(), lpszcalledparty.param().abi(), lpszcomment.param().abi())
}
#[inline]
pub unsafe fn tapiRequestMediaCall<P0, P1, P2, P3, P4, P5, P6, P7>(hwnd: P0, wrequestid: P1, lpszdeviceclass: P2, lpdeviceid: P3, dwsize: u32, dwsecure: u32, lpszdestaddress: P4, lpszappname: P5, lpszcalledparty: P6, lpszcomment: P7) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn tapiRequestMediaCall(hwnd : super::super::Foundation:: HWND, wrequestid : super::super::Foundation:: WPARAM, lpszdeviceclass : windows_core::PCSTR, lpdeviceid : windows_core::PCSTR, dwsize : u32, dwsecure : u32, lpszdestaddress : windows_core::PCSTR, lpszappname : windows_core::PCSTR, lpszcalledparty : windows_core::PCSTR, lpszcomment : windows_core::PCSTR) -> i32);
    tapiRequestMediaCall(hwnd.param().abi(), wrequestid.param().abi(), lpszdeviceclass.param().abi(), lpdeviceid.param().abi(), dwsize, dwsecure, lpszdestaddress.param().abi(), lpszappname.param().abi(), lpszcalledparty.param().abi(), lpszcomment.param().abi())
}
#[inline]
pub unsafe fn tapiRequestMediaCallA<P0, P1, P2, P3, P4, P5, P6, P7>(hwnd: P0, wrequestid: P1, lpszdeviceclass: P2, lpdeviceid: P3, dwsize: u32, dwsecure: u32, lpszdestaddress: P4, lpszappname: P5, lpszcalledparty: P6, lpszcomment: P7) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn tapiRequestMediaCallA(hwnd : super::super::Foundation:: HWND, wrequestid : super::super::Foundation:: WPARAM, lpszdeviceclass : windows_core::PCSTR, lpdeviceid : windows_core::PCSTR, dwsize : u32, dwsecure : u32, lpszdestaddress : windows_core::PCSTR, lpszappname : windows_core::PCSTR, lpszcalledparty : windows_core::PCSTR, lpszcomment : windows_core::PCSTR) -> i32);
    tapiRequestMediaCallA(hwnd.param().abi(), wrequestid.param().abi(), lpszdeviceclass.param().abi(), lpdeviceid.param().abi(), dwsize, dwsecure, lpszdestaddress.param().abi(), lpszappname.param().abi(), lpszcalledparty.param().abi(), lpszcomment.param().abi())
}
#[inline]
pub unsafe fn tapiRequestMediaCallW<P0, P1, P2, P3, P4, P5, P6, P7>(hwnd: P0, wrequestid: P1, lpszdeviceclass: P2, lpdeviceid: P3, dwsize: u32, dwsecure: u32, lpszdestaddress: P4, lpszappname: P5, lpszcalledparty: P6, lpszcomment: P7) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("tapi32.dll" "system" fn tapiRequestMediaCallW(hwnd : super::super::Foundation:: HWND, wrequestid : super::super::Foundation:: WPARAM, lpszdeviceclass : windows_core::PCWSTR, lpdeviceid : windows_core::PCWSTR, dwsize : u32, dwsecure : u32, lpszdestaddress : windows_core::PCWSTR, lpszappname : windows_core::PCWSTR, lpszcalledparty : windows_core::PCWSTR, lpszcomment : windows_core::PCWSTR) -> i32);
    tapiRequestMediaCallW(hwnd.param().abi(), wrequestid.param().abi(), lpszdeviceclass.param().abi(), lpdeviceid.param().abi(), dwsize, dwsecure, lpszdestaddress.param().abi(), lpszappname.param().abi(), lpszcalledparty.param().abi(), lpszcomment.param().abi())
}
windows_core::imp::define_interface!(IEnumACDGroup, IEnumACDGroup_Vtbl, 0x5afc3157_4bcc_11d1_bf80_00805fc147d3);
impl core::ops::Deref for IEnumACDGroup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumACDGroup, windows_core::IUnknown);
impl IEnumACDGroup {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITACDGroup>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumACDGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumACDGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumAddress, IEnumAddress_Vtbl, 0x1666fca1_9363_11d0_835c_00aa003ccabd);
impl core::ops::Deref for IEnumAddress {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumAddress, windows_core::IUnknown);
impl IEnumAddress {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [Option<ITAddress>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumAddress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumAgent, IEnumAgent_Vtbl, 0x5afc314d_4bcc_11d1_bf80_00805fc147d3);
impl core::ops::Deref for IEnumAgent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumAgent, windows_core::IUnknown);
impl IEnumAgent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITAgent>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumAgent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumAgent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumAgentHandler, IEnumAgentHandler_Vtbl, 0x587e8c28_9802_11d1_a0a4_00805fc147d3);
impl core::ops::Deref for IEnumAgentHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumAgentHandler, windows_core::IUnknown);
impl IEnumAgentHandler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITAgentHandler>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumAgentHandler> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumAgentHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumAgentSession, IEnumAgentSession_Vtbl, 0x5afc314e_4bcc_11d1_bf80_00805fc147d3);
impl core::ops::Deref for IEnumAgentSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumAgentSession, windows_core::IUnknown);
impl IEnumAgentSession {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITAgentSession>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumAgentSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumAgentSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumBstr, IEnumBstr_Vtbl, 0x35372049_0bc6_11d2_a033_00c04fb6809f);
impl core::ops::Deref for IEnumBstr {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumBstr, windows_core::IUnknown);
impl IEnumBstr {
    pub unsafe fn Next(&self, ppstrings: &mut [windows_core::BSTR], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppstrings.len().try_into().unwrap(), core::mem::transmute(ppstrings.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumBstr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumBstr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumCall, IEnumCall_Vtbl, 0xae269cf6_935e_11d0_835c_00aa003ccabd);
impl core::ops::Deref for IEnumCall {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumCall, windows_core::IUnknown);
impl IEnumCall {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITCallInfo>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCall> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumCall_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumCallHub, IEnumCallHub_Vtbl, 0xa3c15450_5b92_11d1_8f4e_00c04fb6809f);
impl core::ops::Deref for IEnumCallHub {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumCallHub, windows_core::IUnknown);
impl IEnumCallHub {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [Option<ITCallHub>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCallHub> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumCallHub_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumCallingCard, IEnumCallingCard_Vtbl, 0x0c4d8f02_8ddb_11d1_a09e_00805fc147d3);
impl core::ops::Deref for IEnumCallingCard {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumCallingCard, windows_core::IUnknown);
impl IEnumCallingCard {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITCallingCard>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCallingCard> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumCallingCard_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumDialableAddrs, IEnumDialableAddrs_Vtbl, 0x34621d70_6cff_11d1_aff7_00c04fc31fee);
impl core::ops::Deref for IEnumDialableAddrs {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDialableAddrs, windows_core::IUnknown);
impl IEnumDialableAddrs {
    pub unsafe fn Next(&self, ppelements: &mut [windows_core::BSTR], pcfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pcfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDialableAddrs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumDialableAddrs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumDirectory, IEnumDirectory_Vtbl, 0x34621d6d_6cff_11d1_aff7_00c04fc31fee);
impl core::ops::Deref for IEnumDirectory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDirectory, windows_core::IUnknown);
impl IEnumDirectory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [Option<ITDirectory>], pcfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pcfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDirectory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumDirectory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumDirectoryObject, IEnumDirectoryObject_Vtbl, 0x06c9b64a_306d_11d1_9774_00c04fd91ac0);
impl core::ops::Deref for IEnumDirectoryObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDirectoryObject, windows_core::IUnknown);
impl IEnumDirectoryObject {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, pval: &mut [Option<ITDirectoryObject>], pcfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pval.len().try_into().unwrap(), core::mem::transmute(pval.as_ptr()), core::mem::transmute(pcfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDirectoryObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumDirectoryObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumLocation, IEnumLocation_Vtbl, 0x0c4d8f01_8ddb_11d1_a09e_00805fc147d3);
impl core::ops::Deref for IEnumLocation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumLocation, windows_core::IUnknown);
impl IEnumLocation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITLocationInfo>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumLocation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumLocation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumMcastScope, IEnumMcastScope_Vtbl, 0xdf0daf09_a289_11d1_8697_006008b0e5d2);
impl core::ops::Deref for IEnumMcastScope {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumMcastScope, windows_core::IUnknown);
impl IEnumMcastScope {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppscopes: *mut Option<IMcastScope>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppscopes), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumMcastScope> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumMcastScope_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumPhone, IEnumPhone_Vtbl, 0xf15b7669_4780_4595_8c89_fb369c8cf7aa);
impl core::ops::Deref for IEnumPhone {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumPhone, windows_core::IUnknown);
impl IEnumPhone {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [Option<ITPhone>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumPhone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumPhone_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumPluggableSuperclassInfo, IEnumPluggableSuperclassInfo_Vtbl, 0xe9586a80_89e6_4cff_931d_478d5751f4c0);
impl core::ops::Deref for IEnumPluggableSuperclassInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumPluggableSuperclassInfo, windows_core::IUnknown);
impl IEnumPluggableSuperclassInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [Option<ITPluggableTerminalSuperclassInfo>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumPluggableSuperclassInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumPluggableSuperclassInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumPluggableTerminalClassInfo, IEnumPluggableTerminalClassInfo_Vtbl, 0x4567450c_dbee_4e3f_aaf5_37bf9ebf5e29);
impl core::ops::Deref for IEnumPluggableTerminalClassInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumPluggableTerminalClassInfo, windows_core::IUnknown);
impl IEnumPluggableTerminalClassInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, ppelements: &mut [Option<ITPluggableTerminalClassInfo>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppelements.len().try_into().unwrap(), core::mem::transmute(ppelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumPluggableTerminalClassInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumPluggableTerminalClassInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumQueue, IEnumQueue_Vtbl, 0x5afc3158_4bcc_11d1_bf80_00805fc147d3);
impl core::ops::Deref for IEnumQueue {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumQueue, windows_core::IUnknown);
impl IEnumQueue {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITQueue>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), pceltfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumQueue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumStream, IEnumStream_Vtbl, 0xee3bd606_3868_11d2_a045_00c04fb6809f);
impl core::ops::Deref for IEnumStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumStream, windows_core::IUnknown);
impl IEnumStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITStream>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumSubStream, IEnumSubStream_Vtbl, 0xee3bd609_3868_11d2_a045_00c04fb6809f);
impl core::ops::Deref for IEnumSubStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSubStream, windows_core::IUnknown);
impl IEnumSubStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITSubStream>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSubStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSubStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumTerminal, IEnumTerminal_Vtbl, 0xae269cf4_935e_11d0_835c_00aa003ccabd);
impl core::ops::Deref for IEnumTerminal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumTerminal, windows_core::IUnknown);
impl IEnumTerminal {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut Option<ITTerminal>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppelements), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumTerminal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumTerminalClass, IEnumTerminalClass_Vtbl, 0xae269cf5_935e_11d0_835c_00aa003ccabd);
impl core::ops::Deref for IEnumTerminalClass {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumTerminalClass, windows_core::IUnknown);
impl IEnumTerminalClass {
    pub unsafe fn Next(&self, pelements: &mut [windows_core::GUID], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pelements.len().try_into().unwrap(), core::mem::transmute(pelements.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumTerminalClass> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumTerminalClass_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMcastAddressAllocation, IMcastAddressAllocation_Vtbl, 0xdf0daef1_a289_11d1_8697_006008b0e5d2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMcastAddressAllocation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMcastAddressAllocation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMcastAddressAllocation {
    pub unsafe fn Scopes(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Scopes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateScopes(&self) -> windows_core::Result<IEnumMcastScope> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateScopes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestAddress<P0>(&self, pscope: P0, leasestarttime: f64, leasestoptime: f64, numaddresses: i32) -> windows_core::Result<IMcastLeaseInfo>
    where
        P0: windows_core::Param<IMcastScope>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestAddress)(windows_core::Interface::as_raw(self), pscope.param().abi(), leasestarttime, leasestoptime, numaddresses, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RenewAddress<P0>(&self, lreserved: i32, prenewrequest: P0) -> windows_core::Result<IMcastLeaseInfo>
    where
        P0: windows_core::Param<IMcastLeaseInfo>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RenewAddress)(windows_core::Interface::as_raw(self), lreserved, prenewrequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReleaseAddress<P0>(&self, preleaserequest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMcastLeaseInfo>,
    {
        (windows_core::Interface::vtable(self).ReleaseAddress)(windows_core::Interface::as_raw(self), preleaserequest.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateLeaseInfo<P0, P1>(&self, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const windows_core::PCWSTR, prequestid: P0, pserveraddress: P1) -> windows_core::Result<IMcastLeaseInfo>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLeaseInfo)(windows_core::Interface::as_raw(self), leasestarttime, leasestoptime, dwnumaddresses, ppaddresses, prequestid.param().abi(), pserveraddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateLeaseInfoFromVariant<P0, P1, P2>(&self, leasestarttime: f64, leasestoptime: f64, vaddresses: P0, prequestid: P1, pserveraddress: P2) -> windows_core::Result<IMcastLeaseInfo>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLeaseInfoFromVariant)(windows_core::Interface::as_raw(self), leasestarttime, leasestoptime, vaddresses.param().abi(), prequestid.param().abi(), pserveraddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMcastAddressAllocation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Scopes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateScopes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, f64, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RenewAddress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RenewAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReleaseAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReleaseAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateLeaseInfo: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, u32, *const windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateLeaseInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateLeaseInfoFromVariant: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateLeaseInfoFromVariant: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMcastLeaseInfo, IMcastLeaseInfo_Vtbl, 0xdf0daefd_a289_11d1_8697_006008b0e5d2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMcastLeaseInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMcastLeaseInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMcastLeaseInfo {
    pub unsafe fn RequestID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LeaseStartTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LeaseStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLeaseStartTime(&self, time: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLeaseStartTime)(windows_core::Interface::as_raw(self), time).ok()
    }
    pub unsafe fn LeaseStopTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LeaseStopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLeaseStopTime(&self, time: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLeaseStopTime)(windows_core::Interface::as_raw(self), time).ok()
    }
    pub unsafe fn AddressCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddressCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ServerAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServerAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TTL(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TTL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Addresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Addresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateAddresses(&self) -> windows_core::Result<IEnumBstr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMcastLeaseInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RequestID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LeaseStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLeaseStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LeaseStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLeaseStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub AddressCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ServerAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Addresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMcastScope, IMcastScope_Vtbl, 0xdf0daef4_a289_11d1_8697_006008b0e5d2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMcastScope {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMcastScope, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMcastScope {
    pub unsafe fn ScopeID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScopeID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ServerID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServerID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ScopeDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScopeDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TTL(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TTL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMcastScope_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ScopeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ServerID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ScopeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITACDGroup, ITACDGroup_Vtbl, 0x5afc3148_4bcc_11d1_bf80_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITACDGroup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITACDGroup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroup {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateQueues(&self) -> windows_core::Result<IEnumQueue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateQueues)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Queues(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Queues)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITACDGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumerateQueues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Queues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITACDGroupEvent, ITACDGroupEvent_Vtbl, 0x297f3032_bd11_11d1_a0a7_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITACDGroupEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITACDGroupEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroupEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Group(&self) -> windows_core::Result<ITACDGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Group)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<ACDGROUP_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITACDGroupEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Group: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ACDGROUP_EVENT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITAMMediaFormat, ITAMMediaFormat_Vtbl, 0x0364eb00_4a77_11d1_a671_006097c9a2e8);
impl core::ops::Deref for ITAMMediaFormat {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITAMMediaFormat, windows_core::IUnknown);
impl ITAMMediaFormat {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn MediaFormat(&self) -> windows_core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetMediaFormat(&self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMediaFormat)(windows_core::Interface::as_raw(self), pmt).ok()
    }
}
#[repr(C)]
pub struct ITAMMediaFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub MediaFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    MediaFormat: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub SetMediaFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    SetMediaFormat: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITASRTerminalEvent, ITASRTerminalEvent_Vtbl, 0xee016a02_4fa9_467c_933f_5a15b12377d7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITASRTerminalEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITASRTerminalEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITASRTerminalEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITASRTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAddress, ITAddress_Vtbl, 0xb1efc386_9355_11d0_835c_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAddress {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAddress, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAddress {
    pub unsafe fn State(&self) -> windows_core::Result<ADDRESS_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AddressName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddressName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TAPIObject(&self) -> windows_core::Result<ITTAPI> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TAPIObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCall<P0>(&self, pdestaddress: P0, laddresstype: i32, lmediatypes: i32) -> windows_core::Result<ITBasicCallControl>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCall)(windows_core::Interface::as_raw(self), pdestaddress.param().abi(), laddresstype, lmediatypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Calls(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Calls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateCalls(&self) -> windows_core::Result<IEnumCall> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateCalls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DialableAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DialableAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateForwardInfoObject(&self) -> windows_core::Result<ITForwardInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateForwardInfoObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Forward<P0, P1>(&self, pforwardinfo: P0, pcall: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITForwardInformation>,
        P1: windows_core::Param<ITBasicCallControl>,
    {
        (windows_core::Interface::vtable(self).Forward)(windows_core::Interface::as_raw(self), pforwardinfo.param().abi(), pcall.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentForwardInfo(&self) -> windows_core::Result<ITForwardInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentForwardInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMessageWaiting<P0>(&self, fmessagewaiting: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMessageWaiting)(windows_core::Interface::as_raw(self), fmessagewaiting.param().abi()).ok()
    }
    pub unsafe fn MessageWaiting(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MessageWaiting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDoNotDisturb<P0>(&self, fdonotdisturb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDoNotDisturb)(windows_core::Interface::as_raw(self), fdonotdisturb.param().abi()).ok()
    }
    pub unsafe fn DoNotDisturb(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DoNotDisturb)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ADDRESS_STATE) -> windows_core::HRESULT,
    pub AddressName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TAPIObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TAPIObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCall: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCall: usize,
    pub Calls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DialableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateForwardInfoObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateForwardInfoObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Forward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Forward: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentForwardInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentForwardInfo: usize,
    pub SetMessageWaiting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MessageWaiting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDoNotDisturb: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DoNotDisturb: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAddress2, ITAddress2_Vtbl, 0xb0ae5d9b_be51_46c9_b0f7_dfa8a22a8bc4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAddress2 {
    type Target = ITAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAddress2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITAddress);
#[cfg(feature = "Win32_System_Com")]
impl ITAddress2 {
    pub unsafe fn Phones(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Phones)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePhones(&self) -> windows_core::Result<IEnumPhone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePhones)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPhoneFromTerminal<P0>(&self, pterminal: P0) -> windows_core::Result<ITPhone>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPhoneFromTerminal)(windows_core::Interface::as_raw(self), pterminal.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PreferredPhones(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreferredPhones)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePreferredPhones(&self) -> windows_core::Result<IEnumPhone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePreferredPhones)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EventFilter)(windows_core::Interface::as_raw(self), tapievent, lsubevent, &mut result__).map(|| result__)
    }
    pub unsafe fn put_EventFilter<P0>(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).put_EventFilter)(windows_core::Interface::as_raw(self), tapievent, lsubevent, benable.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSpecific<P0>(&self, pcall: P0, pparams: *const u8, dwsize: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITCallInfo>,
    {
        (windows_core::Interface::vtable(self).DeviceSpecific)(windows_core::Interface::as_raw(self), pcall.param().abi(), pparams, dwsize).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceSpecificVariant<P0, P1>(&self, pcall: P0, vardevspecificbytearray: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITCallInfo>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeviceSpecificVariant)(windows_core::Interface::as_raw(self), pcall.param().abi(), vardevspecificbytearray.param().abi()).ok()
    }
    pub unsafe fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NegotiateExtVersion)(windows_core::Interface::as_raw(self), llowversion, lhighversion, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddress2_Vtbl {
    pub base__: ITAddress_Vtbl,
    pub Phones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumeratePhones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPhoneFromTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPhoneFromTerminal: usize,
    pub PreferredPhones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumeratePreferredPhones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, TAPI_EVENT, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_EventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, TAPI_EVENT, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceSpecific: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceSpecificVariant: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceSpecificVariant: usize,
    pub NegotiateExtVersion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAddressCapabilities, ITAddressCapabilities_Vtbl, 0x8df232f5_821b_11d1_bb5c_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAddressCapabilities {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAddressCapabilities, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressCapabilities {
    pub unsafe fn get_AddressCapability(&self, addresscap: ADDRESS_CAPABILITY) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AddressCapability)(windows_core::Interface::as_raw(self), addresscap, &mut result__).map(|| result__)
    }
    pub unsafe fn get_AddressCapabilityString(&self, addresscapstring: ADDRESS_CAPABILITY_STRING) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_AddressCapabilityString)(windows_core::Interface::as_raw(self), addresscapstring, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CallTreatments(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallTreatments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateCallTreatments(&self) -> windows_core::Result<IEnumBstr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateCallTreatments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CompletionMessages(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CompletionMessages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateCompletionMessages(&self) -> windows_core::Result<IEnumBstr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateCompletionMessages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeviceClasses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceClasses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateDeviceClasses(&self) -> windows_core::Result<IEnumBstr> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateDeviceClasses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressCapabilities_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_AddressCapability: unsafe extern "system" fn(*mut core::ffi::c_void, ADDRESS_CAPABILITY, *mut i32) -> windows_core::HRESULT,
    pub get_AddressCapabilityString: unsafe extern "system" fn(*mut core::ffi::c_void, ADDRESS_CAPABILITY_STRING, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CallTreatments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateCallTreatments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompletionMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateCompletionMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateDeviceClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAddressDeviceSpecificEvent, ITAddressDeviceSpecificEvent_Vtbl, 0x3acb216b_40bd_487a_8672_5ce77bd7e3a3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAddressDeviceSpecificEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAddressDeviceSpecificEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressDeviceSpecificEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> windows_core::Result<ITAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn lParam1(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).lParam1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn lParam2(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).lParam2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn lParam3(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).lParam3)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressDeviceSpecificEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub lParam1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub lParam2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub lParam3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAddressEvent, ITAddressEvent_Vtbl, 0x831ce2d1_83b5_11d1_bb5c_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAddressEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAddressEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> windows_core::Result<ITAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<ADDRESS_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ADDRESS_EVENT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAddressTranslation, ITAddressTranslation_Vtbl, 0x0c4d8f03_8ddb_11d1_a09e_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAddressTranslation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAddressTranslation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TranslateAddress<P0>(&self, paddresstotranslate: P0, lcard: i32, ltranslateoptions: i32) -> windows_core::Result<ITAddressTranslationInfo>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TranslateAddress)(windows_core::Interface::as_raw(self), paddresstotranslate.param().abi(), lcard, ltranslateoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TranslateDialog<P0>(&self, hwndowner: isize, paddressin: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).TranslateDialog)(windows_core::Interface::as_raw(self), hwndowner, paddressin.param().abi()).ok()
    }
    pub unsafe fn EnumerateLocations(&self) -> windows_core::Result<IEnumLocation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateLocations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Locations(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Locations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateCallingCards(&self) -> windows_core::Result<IEnumCallingCard> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateCallingCards)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CallingCards(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallingCards)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressTranslation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub TranslateAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TranslateAddress: usize,
    pub TranslateDialog: unsafe extern "system" fn(*mut core::ffi::c_void, isize, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumerateLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Locations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateCallingCards: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CallingCards: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAddressTranslationInfo, ITAddressTranslationInfo_Vtbl, 0xafc15945_8d40_11d1_a09e_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAddressTranslationInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAddressTranslationInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslationInfo {
    pub unsafe fn DialableString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DialableString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayableString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayableString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentCountryCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCountryCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DestinationCountryCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestinationCountryCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TranslationResults(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TranslationResults)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAddressTranslationInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DialableString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayableString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CurrentCountryCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DestinationCountryCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TranslationResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAgent, ITAgent_Vtbl, 0x5770ece5_4b27_11d1_bf80_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAgent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAgent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAgent {
    pub unsafe fn EnumerateAgentSessions(&self) -> windows_core::Result<IEnumAgentSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateAgentSessions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0, P1>(&self, pacdgroup: P0, paddress: P1) -> windows_core::Result<ITAgentSession>
    where
        P0: windows_core::Param<ITACDGroup>,
        P1: windows_core::Param<ITAddress>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), pacdgroup.param().abi(), paddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSessionWithPIN<P0, P1, P2>(&self, pacdgroup: P0, paddress: P1, ppin: P2) -> windows_core::Result<ITAgentSession>
    where
        P0: windows_core::Param<ITACDGroup>,
        P1: windows_core::Param<ITAddress>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSessionWithPIN)(windows_core::Interface::as_raw(self), pacdgroup.param().abi(), paddress.param().abi(), ppin.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetState(&self, agentstate: AGENT_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), agentstate).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<AGENT_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMeasurementPeriod(&self, lperiod: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMeasurementPeriod)(windows_core::Interface::as_raw(self), lperiod).ok()
    }
    pub unsafe fn MeasurementPeriod(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MeasurementPeriod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OverallCallRate(&self) -> windows_core::Result<super::super::System::Com::CY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OverallCallRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfACDCalls(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfACDCalls)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfIncomingCalls(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfIncomingCalls)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfOutgoingCalls(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfOutgoingCalls)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalACDTalkTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalACDTalkTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalACDCallTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalACDCallTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalWrapUpTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalWrapUpTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AgentSessions(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AgentSessions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EnumerateAgentSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSession: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSessionWithPIN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSessionWithPIN: usize,
    pub ID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, AGENT_STATE) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AGENT_STATE) -> windows_core::HRESULT,
    pub SetMeasurementPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MeasurementPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OverallCallRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Com::CY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OverallCallRate: usize,
    pub NumberOfACDCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfIncomingCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfOutgoingCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalACDTalkTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalACDCallTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalWrapUpTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AgentSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAgentEvent, ITAgentEvent_Vtbl, 0x5afc314a_4bcc_11d1_bf80_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAgentEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAgentEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Agent(&self) -> windows_core::Result<ITAgent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Agent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<AGENT_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Agent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Agent: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AGENT_EVENT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAgentHandler, ITAgentHandler_Vtbl, 0x587e8c22_9802_11d1_a0a4_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAgentHandler {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAgentHandler, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandler {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAgent(&self) -> windows_core::Result<ITAgent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAgent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAgentWithID<P0, P1>(&self, pid: P0, ppin: P1) -> windows_core::Result<ITAgent>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAgentWithID)(windows_core::Interface::as_raw(self), pid.param().abi(), ppin.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateACDGroups(&self) -> windows_core::Result<IEnumACDGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateACDGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateUsableAddresses(&self) -> windows_core::Result<IEnumAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateUsableAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ACDGroups(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ACDGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UsableAddresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UsableAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentHandler_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAgent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAgent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAgentWithID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAgentWithID: usize,
    pub EnumerateACDGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateUsableAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ACDGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub UsableAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAgentHandlerEvent, ITAgentHandlerEvent_Vtbl, 0x297f3034_bd11_11d1_a0a7_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAgentHandlerEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAgentHandlerEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandlerEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AgentHandler(&self) -> windows_core::Result<ITAgentHandler> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AgentHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<AGENTHANDLER_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentHandlerEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AgentHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AgentHandler: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AGENTHANDLER_EVENT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAgentSession, ITAgentSession_Vtbl, 0x5afc3147_4bcc_11d1_bf80_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAgentSession {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAgentSession, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSession {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Agent(&self) -> windows_core::Result<ITAgent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Agent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> windows_core::Result<ITAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ACDGroup(&self) -> windows_core::Result<ITACDGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ACDGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetState(&self, sessionstate: AGENT_SESSION_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), sessionstate).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<AGENT_SESSION_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SessionStartTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionStartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SessionDuration(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfCalls(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfCalls)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalTalkTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalTalkTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AverageTalkTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AverageTalkTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalCallTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalCallTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AverageCallTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AverageCallTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalWrapUpTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalWrapUpTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AverageWrapUpTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AverageWrapUpTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ACDCallRate(&self) -> windows_core::Result<super::super::System::Com::CY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ACDCallRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LongestTimeToAnswer(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LongestTimeToAnswer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AverageTimeToAnswer(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AverageTimeToAnswer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentSession_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Agent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Agent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ACDGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ACDGroup: usize,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, AGENT_SESSION_STATE) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AGENT_SESSION_STATE) -> windows_core::HRESULT,
    pub SessionStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SessionDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalTalkTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AverageTalkTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalCallTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AverageCallTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalWrapUpTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AverageWrapUpTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ACDCallRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Com::CY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ACDCallRate: usize,
    pub LongestTimeToAnswer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AverageTimeToAnswer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAgentSessionEvent, ITAgentSessionEvent_Vtbl, 0x5afc314b_4bcc_11d1_bf80_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAgentSessionEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAgentSessionEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSessionEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Session(&self) -> windows_core::Result<ITAgentSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<AGENT_SESSION_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAgentSessionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Session: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AGENT_SESSION_EVENT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITAllocatorProperties, ITAllocatorProperties_Vtbl, 0xc1bc3c90_bcfe_11d1_9745_00c04fd91ac0);
impl core::ops::Deref for ITAllocatorProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITAllocatorProperties, windows_core::IUnknown);
impl ITAllocatorProperties {
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn SetAllocatorProperties(&self, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllocatorProperties)(windows_core::Interface::as_raw(self), pallocproperties).ok()
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn GetAllocatorProperties(&self) -> windows_core::Result<super::super::Media::DirectShow::ALLOCATOR_PROPERTIES> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllocatorProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllocateBuffers<P0>(&self, ballocbuffers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllocateBuffers)(windows_core::Interface::as_raw(self), ballocbuffers.param().abi()).ok()
    }
    pub unsafe fn GetAllocateBuffers(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllocateBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBufferSize(&self, buffersize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferSize)(windows_core::Interface::as_raw(self), buffersize).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITAllocatorProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub SetAllocatorProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    SetAllocatorProperties: usize,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub GetAllocatorProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    GetAllocatorProperties: usize,
    pub SetAllocateBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAllocateBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITAutomatedPhoneControl, ITAutomatedPhoneControl_Vtbl, 0x1ee1af0e_6159_4a61_b79b_6a4ba3fc9dfc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITAutomatedPhoneControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITAutomatedPhoneControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITAutomatedPhoneControl {
    pub unsafe fn StartTone(&self, tone: PHONE_TONE, lduration: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartTone)(windows_core::Interface::as_raw(self), tone, lduration).ok()
    }
    pub unsafe fn StopTone(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopTone)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Tone(&self) -> windows_core::Result<PHONE_TONE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Tone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StartRinger(&self, lringmode: i32, lduration: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartRinger)(windows_core::Interface::as_raw(self), lringmode, lduration).ok()
    }
    pub unsafe fn StopRinger(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopRinger)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Ringer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Ringer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPhoneHandlingEnabled<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPhoneHandlingEnabled)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn PhoneHandlingEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhoneHandlingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoEndOfNumberTimeout(&self, ltimeout: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutoEndOfNumberTimeout)(windows_core::Interface::as_raw(self), ltimeout).ok()
    }
    pub unsafe fn AutoEndOfNumberTimeout(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoEndOfNumberTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoDialtone<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoDialtone)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn AutoDialtone(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoDialtone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoStopTonesOnOnHook<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoStopTonesOnOnHook)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn AutoStopTonesOnOnHook(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoStopTonesOnOnHook)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoStopRingOnOffHook<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoStopRingOnOffHook)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn AutoStopRingOnOffHook(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoStopRingOnOffHook)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoKeypadTones<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoKeypadTones)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn AutoKeypadTones(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoKeypadTones)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoKeypadTonesMinimumDuration(&self, lduration: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutoKeypadTonesMinimumDuration)(windows_core::Interface::as_raw(self), lduration).ok()
    }
    pub unsafe fn AutoKeypadTonesMinimumDuration(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoKeypadTonesMinimumDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoVolumeControl<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoVolumeControl)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn AutoVolumeControl(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoVolumeControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoVolumeControlStep(&self, lstepsize: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutoVolumeControlStep)(windows_core::Interface::as_raw(self), lstepsize).ok()
    }
    pub unsafe fn AutoVolumeControlStep(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoVolumeControlStep)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoVolumeControlRepeatDelay(&self, ldelay: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutoVolumeControlRepeatDelay)(windows_core::Interface::as_raw(self), ldelay).ok()
    }
    pub unsafe fn AutoVolumeControlRepeatDelay(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoVolumeControlRepeatDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoVolumeControlRepeatPeriod(&self, lperiod: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutoVolumeControlRepeatPeriod)(windows_core::Interface::as_raw(self), lperiod).ok()
    }
    pub unsafe fn AutoVolumeControlRepeatPeriod(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoVolumeControlRepeatPeriod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectCall<P0, P1>(&self, pcall: P0, fselectdefaultterminals: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITCallInfo>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SelectCall)(windows_core::Interface::as_raw(self), pcall.param().abi(), fselectdefaultterminals.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectCall<P0>(&self, pcall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITCallInfo>,
    {
        (windows_core::Interface::vtable(self).UnselectCall)(windows_core::Interface::as_raw(self), pcall.param().abi()).ok()
    }
    pub unsafe fn EnumerateSelectedCalls(&self) -> windows_core::Result<IEnumCall> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateSelectedCalls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SelectedCalls(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SelectedCalls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITAutomatedPhoneControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartTone: unsafe extern "system" fn(*mut core::ffi::c_void, PHONE_TONE, i32) -> windows_core::HRESULT,
    pub StopTone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PHONE_TONE) -> windows_core::HRESULT,
    pub StartRinger: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub StopRinger: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Ringer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPhoneHandlingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub PhoneHandlingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoEndOfNumberTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoEndOfNumberTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoDialtone: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AutoDialtone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoStopTonesOnOnHook: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AutoStopTonesOnOnHook: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoStopRingOnOffHook: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AutoStopRingOnOffHook: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoKeypadTones: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AutoKeypadTones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoKeypadTonesMinimumDuration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoKeypadTonesMinimumDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoVolumeControl: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AutoVolumeControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoVolumeControlStep: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoVolumeControlStep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoVolumeControlRepeatDelay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoVolumeControlRepeatDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoVolumeControlRepeatPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoVolumeControlRepeatPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectCall: usize,
    pub EnumerateSelectedCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectedCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITBasicAudioTerminal, ITBasicAudioTerminal_Vtbl, 0xb1efc38d_9355_11d0_835c_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITBasicAudioTerminal {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITBasicAudioTerminal, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITBasicAudioTerminal {
    pub unsafe fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), lvolume).ok()
    }
    pub unsafe fn Volume(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBalance(&self, lbalance: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBalance)(windows_core::Interface::as_raw(self), lbalance).ok()
    }
    pub unsafe fn Balance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Balance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITBasicAudioTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBalance: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Balance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITBasicCallControl, ITBasicCallControl_Vtbl, 0xb1efc389_9355_11d0_835c_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITBasicCallControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITBasicCallControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl {
    pub unsafe fn Connect<P0>(&self, fsync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), fsync.param().abi()).ok()
    }
    pub unsafe fn Answer(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Answer)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self, code: DISCONNECT_CODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), code).ok()
    }
    pub unsafe fn Hold<P0>(&self, fhold: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Hold)(windows_core::Interface::as_raw(self), fhold.param().abi()).ok()
    }
    pub unsafe fn HandoffDirect<P0>(&self, papplicationname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).HandoffDirect)(windows_core::Interface::as_raw(self), papplicationname.param().abi()).ok()
    }
    pub unsafe fn HandoffIndirect(&self, lmediatype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandoffIndirect)(windows_core::Interface::as_raw(self), lmediatype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Conference<P0, P1>(&self, pcall: P0, fsync: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITBasicCallControl>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Conference)(windows_core::Interface::as_raw(self), pcall.param().abi(), fsync.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Transfer<P0, P1>(&self, pcall: P0, fsync: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITBasicCallControl>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Transfer)(windows_core::Interface::as_raw(self), pcall.param().abi(), fsync.param().abi()).ok()
    }
    pub unsafe fn BlindTransfer<P0>(&self, pdestaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).BlindTransfer)(windows_core::Interface::as_raw(self), pdestaddress.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SwapHold<P0>(&self, pcall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITBasicCallControl>,
    {
        (windows_core::Interface::vtable(self).SwapHold)(windows_core::Interface::as_raw(self), pcall.param().abi()).ok()
    }
    pub unsafe fn ParkDirect<P0>(&self, pparkaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ParkDirect)(windows_core::Interface::as_raw(self), pparkaddress.param().abi()).ok()
    }
    pub unsafe fn ParkIndirect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParkIndirect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Unpark(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unpark)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetQOS(&self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQOS)(windows_core::Interface::as_raw(self), lmediatype, servicelevel).ok()
    }
    pub unsafe fn Pickup<P0>(&self, pgroupid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Pickup)(windows_core::Interface::as_raw(self), pgroupid.param().abi()).ok()
    }
    pub unsafe fn Dial<P0>(&self, pdestaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Dial)(windows_core::Interface::as_raw(self), pdestaddress.param().abi()).ok()
    }
    pub unsafe fn Finish(&self, finishmode: FINISH_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish)(windows_core::Interface::as_raw(self), finishmode).ok()
    }
    pub unsafe fn RemoveFromConference(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveFromConference)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITBasicCallControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Answer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, DISCONNECT_CODE) -> windows_core::HRESULT,
    pub Hold: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub HandoffDirect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HandoffIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Conference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Conference: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Transfer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Transfer: usize,
    pub BlindTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SwapHold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SwapHold: usize,
    pub ParkDirect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ParkIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Unpark: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQOS: unsafe extern "system" fn(*mut core::ffi::c_void, i32, QOS_SERVICE_LEVEL) -> windows_core::HRESULT,
    pub Pickup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Dial: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Finish: unsafe extern "system" fn(*mut core::ffi::c_void, FINISH_MODE) -> windows_core::HRESULT,
    pub RemoveFromConference: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITBasicCallControl2, ITBasicCallControl2_Vtbl, 0x161a4a56_1e99_4b3f_a46a_168f38a5ee4c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITBasicCallControl2 {
    type Target = ITBasicCallControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITBasicCallControl2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITBasicCallControl);
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestTerminal<P0>(&self, bstrterminalclassguid: P0, lmediatype: i32, direction: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestTerminal)(windows_core::Interface::as_raw(self), bstrterminalclassguid.param().abi(), lmediatype, direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectTerminalOnCall<P0>(&self, pterminal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        (windows_core::Interface::vtable(self).SelectTerminalOnCall)(windows_core::Interface::as_raw(self), pterminal.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectTerminalOnCall<P0>(&self, pterminal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        (windows_core::Interface::vtable(self).UnselectTerminalOnCall)(windows_core::Interface::as_raw(self), pterminal.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITBasicCallControl2_Vtbl {
    pub base__: ITBasicCallControl_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, TERMINAL_DIRECTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminalOnCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminalOnCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminalOnCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminalOnCall: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallHub, ITCallHub_Vtbl, 0xa3c1544e_5b92_11d1_8f4e_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallHub {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallHub, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallHub {
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumerateCalls(&self) -> windows_core::Result<IEnumCall> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateCalls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Calls(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Calls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NumCalls(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumCalls)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn State(&self) -> windows_core::Result<CALLHUB_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallHub_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Calls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub NumCalls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALLHUB_STATE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallHubEvent, ITCallHubEvent_Vtbl, 0xa3c15451_5b92_11d1_8f4e_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallHubEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallHubEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallHubEvent {
    pub unsafe fn Event(&self) -> windows_core::Result<CALLHUB_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CallHub(&self) -> windows_core::Result<ITCallHub> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallHub)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallHubEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALLHUB_EVENT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallInfo, ITCallInfo_Vtbl, 0x350f85d1_1227_11d3_83d4_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> windows_core::Result<ITAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CallState(&self) -> windows_core::Result<CALL_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Privilege(&self) -> windows_core::Result<CALL_PRIVILEGE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Privilege)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CallHub(&self) -> windows_core::Result<ITCallHub> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallHub)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_CallInfoLong(&self, callinfolong: CALLINFO_LONG) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_CallInfoLong)(windows_core::Interface::as_raw(self), callinfolong, &mut result__).map(|| result__)
    }
    pub unsafe fn put_CallInfoLong(&self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_CallInfoLong)(windows_core::Interface::as_raw(self), callinfolong, lcallinfolongval).ok()
    }
    pub unsafe fn get_CallInfoString(&self, callinfostring: CALLINFO_STRING) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_CallInfoString)(windows_core::Interface::as_raw(self), callinfostring, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_CallInfoString<P0>(&self, callinfostring: CALLINFO_STRING, pcallinfostring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_CallInfoString)(windows_core::Interface::as_raw(self), callinfostring, pcallinfostring.param().abi()).ok()
    }
    pub unsafe fn get_CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_CallInfoBuffer)(windows_core::Interface::as_raw(self), callinfobuffer, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_CallInfoBuffer<P0>(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).put_CallInfoBuffer)(windows_core::Interface::as_raw(self), callinfobuffer, pcallinfobuffer.param().abi()).ok()
    }
    pub unsafe fn GetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCallInfoBuffer)(windows_core::Interface::as_raw(self), callinfobuffer, pdwsize, ppcallinfobuffer).ok()
    }
    pub unsafe fn SetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCallInfoBuffer)(windows_core::Interface::as_raw(self), callinfobuffer, pcallinfobuffer.len().try_into().unwrap(), core::mem::transmute(pcallinfobuffer.as_ptr())).ok()
    }
    pub unsafe fn ReleaseUserUserInfo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseUserUserInfo)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub CallState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALL_STATE) -> windows_core::HRESULT,
    pub Privilege: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALL_PRIVILEGE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    pub get_CallInfoLong: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_LONG, *mut i32) -> windows_core::HRESULT,
    pub put_CallInfoLong: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_LONG, i32) -> windows_core::HRESULT,
    pub get_CallInfoString: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_STRING, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_CallInfoString: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_STRING, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_CallInfoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_BUFFER, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub put_CallInfoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_BUFFER, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetCallInfoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_BUFFER, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetCallInfoBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, CALLINFO_BUFFER, u32, *const u8) -> windows_core::HRESULT,
    pub ReleaseUserUserInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallInfo2, ITCallInfo2_Vtbl, 0x94d70ca6_7ab0_4daa_81ca_b8f8643faec1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallInfo2 {
    type Target = ITCallInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallInfo2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITCallInfo);
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo2 {
    pub unsafe fn get_EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_EventFilter)(windows_core::Interface::as_raw(self), tapievent, lsubevent, &mut result__).map(|| result__)
    }
    pub unsafe fn put_EventFilter<P0>(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).put_EventFilter)(windows_core::Interface::as_raw(self), tapievent, lsubevent, benable.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallInfo2_Vtbl {
    pub base__: ITCallInfo_Vtbl,
    pub get_EventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, TAPI_EVENT, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub put_EventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, TAPI_EVENT, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallInfoChangeEvent, ITCallInfoChangeEvent_Vtbl, 0x5d4b65f9_e51c_11d1_a02f_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallInfoChangeEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallInfoChangeEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfoChangeEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cause(&self) -> windows_core::Result<CALLINFOCHANGE_CAUSE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cause)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallInfoChangeEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Cause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALLINFOCHANGE_CAUSE) -> windows_core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallMediaEvent, ITCallMediaEvent_Vtbl, 0xff36b87f_ec3a_11d0_8ee4_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallMediaEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallMediaEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallMediaEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<CALL_MEDIA_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Stream(&self) -> windows_core::Result<ITStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Cause(&self) -> windows_core::Result<CALL_MEDIA_EVENT_CAUSE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cause)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallMediaEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALL_MEDIA_EVENT) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
    pub Cause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALL_MEDIA_EVENT_CAUSE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallNotificationEvent, ITCallNotificationEvent_Vtbl, 0x895801df_3dd6_11d1_8f30_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallNotificationEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallNotificationEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallNotificationEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<CALL_NOTIFICATION_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallNotificationEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALL_NOTIFICATION_EVENT) -> windows_core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallStateEvent, ITCallStateEvent_Vtbl, 0x62f47097_95c9_11d0_835d_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallStateEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallStateEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallStateEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<CALL_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Cause(&self) -> windows_core::Result<CALL_STATE_EVENT_CAUSE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cause)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallStateEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALL_STATE) -> windows_core::HRESULT,
    pub Cause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALL_STATE_EVENT_CAUSE) -> windows_core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCallingCard, ITCallingCard_Vtbl, 0x0c4d8f00_8ddb_11d1_a09e_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCallingCard {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCallingCard, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCallingCard {
    pub unsafe fn PermanentCardID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PermanentCardID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfDigits(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfDigits)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Options(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Options)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CardName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CardName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SameAreaDialingRule(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SameAreaDialingRule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LongDistanceDialingRule(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LongDistanceDialingRule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InternationalDialingRule(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InternationalDialingRule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCallingCard_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PermanentCardID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CardName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SameAreaDialingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LongDistanceDialingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InternationalDialingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCollection, ITCollection_Vtbl, 0x5ec5acf2_9c02_11d0_8362_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCollection2, ITCollection2_Vtbl, 0xe6dddda5_a6d3_48ff_8737_d32fc4d95477);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCollection2 {
    type Target = ITCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCollection2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITCollection);
#[cfg(feature = "Win32_System_Com")]
impl ITCollection2 {
    pub unsafe fn Add(&self, index: i32, pvariant: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), index, core::mem::transmute(pvariant)).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCollection2_Vtbl {
    pub base__: ITCollection_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITCustomTone, ITCustomTone_Vtbl, 0x357ad764_b3c6_4b2a_8fa5_0722827a9254);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITCustomTone {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITCustomTone, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITCustomTone {
    pub unsafe fn Frequency(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Frequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFrequency(&self, lfrequency: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFrequency)(windows_core::Interface::as_raw(self), lfrequency).ok()
    }
    pub unsafe fn CadenceOn(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CadenceOn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCadenceOn(&self, cadenceon: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCadenceOn)(windows_core::Interface::as_raw(self), cadenceon).ok()
    }
    pub unsafe fn CadenceOff(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CadenceOff)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCadenceOff(&self, lcadenceoff: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCadenceOff)(windows_core::Interface::as_raw(self), lcadenceoff).ok()
    }
    pub unsafe fn Volume(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), lvolume).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITCustomTone_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Frequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CadenceOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCadenceOn: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CadenceOff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCadenceOff: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDetectTone, ITDetectTone_Vtbl, 0x961f79bd_3097_49df_a1d6_909b77e89ca0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDetectTone {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDetectTone, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDetectTone {
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppSpecific)(windows_core::Interface::as_raw(self), lappspecific).ok()
    }
    pub unsafe fn Duration(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDuration(&self, lduration: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), lduration).ok()
    }
    pub unsafe fn get_Frequency(&self, index: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Frequency)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn put_Frequency(&self, index: i32, lfrequency: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_Frequency)(windows_core::Interface::as_raw(self), index, lfrequency).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDetectTone_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Frequency: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub put_Frequency: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDigitDetectionEvent, ITDigitDetectionEvent_Vtbl, 0x80d3bfac_57d9_11d2_a04a_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDigitDetectionEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDigitDetectionEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDigitDetectionEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Digit(&self) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Digit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DigitMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DigitMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TickCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TickCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDigitDetectionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Digit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub DigitMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TickCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDigitGenerationEvent, ITDigitGenerationEvent_Vtbl, 0x80d3bfad_57d9_11d2_a04a_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDigitGenerationEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDigitGenerationEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDigitGenerationEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GenerationTermination(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerationTermination)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TickCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TickCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDigitGenerationEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub GenerationTermination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TickCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDigitsGatheredEvent, ITDigitsGatheredEvent_Vtbl, 0xe52ec4c1_cba3_441a_9e6a_93cb909e9724);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDigitsGatheredEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDigitsGatheredEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDigitsGatheredEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Digits(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Digits)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GatherTermination(&self) -> windows_core::Result<TAPI_GATHERTERM> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GatherTermination)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TickCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TickCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDigitsGatheredEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Digits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GatherTermination: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TAPI_GATHERTERM) -> windows_core::HRESULT,
    pub TickCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDirectory, ITDirectory_Vtbl, 0x34621d6c_6cff_11d1_aff7_00c04fc31fee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDirectory {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDirectory, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectory {
    pub unsafe fn DirectoryType(&self) -> windows_core::Result<DIRECTORY_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DirectoryType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsDynamic(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDynamic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DefaultObjectTTL(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultObjectTTL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDefaultObjectTTL(&self, ttl: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultObjectTTL)(windows_core::Interface::as_raw(self), ttl).ok()
    }
    pub unsafe fn EnableAutoRefresh<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableAutoRefresh)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    pub unsafe fn Connect<P0>(&self, fsecure: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), fsecure.param().abi()).ok()
    }
    pub unsafe fn Bind<P0, P1, P2>(&self, pdomainname: P0, pusername: P1, ppassword: P2, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), pdomainname.param().abi(), pusername.param().abi(), ppassword.param().abi(), lflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddDirectoryObject<P0>(&self, pdirectoryobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITDirectoryObject>,
    {
        (windows_core::Interface::vtable(self).AddDirectoryObject)(windows_core::Interface::as_raw(self), pdirectoryobject.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModifyDirectoryObject<P0>(&self, pdirectoryobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITDirectoryObject>,
    {
        (windows_core::Interface::vtable(self).ModifyDirectoryObject)(windows_core::Interface::as_raw(self), pdirectoryobject.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RefreshDirectoryObject<P0>(&self, pdirectoryobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITDirectoryObject>,
    {
        (windows_core::Interface::vtable(self).RefreshDirectoryObject)(windows_core::Interface::as_raw(self), pdirectoryobject.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteDirectoryObject<P0>(&self, pdirectoryobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITDirectoryObject>,
    {
        (windows_core::Interface::vtable(self).DeleteDirectoryObject)(windows_core::Interface::as_raw(self), pdirectoryobject.param().abi()).ok()
    }
    pub unsafe fn get_DirectoryObjects<P0>(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_DirectoryObjects)(windows_core::Interface::as_raw(self), directoryobjecttype, pname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateDirectoryObjects<P0>(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: P0) -> windows_core::Result<IEnumDirectoryObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateDirectoryObjects)(windows_core::Interface::as_raw(self), directoryobjecttype, pname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DirectoryType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIRECTORY_TYPE) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsDynamic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DefaultObjectTTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDefaultObjectTTL: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnableAutoRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddDirectoryObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifyDirectoryObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifyDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RefreshDirectoryObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RefreshDirectoryObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteDirectoryObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteDirectoryObject: usize,
    pub get_DirectoryObjects: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTORY_OBJECT_TYPE, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateDirectoryObjects: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTORY_OBJECT_TYPE, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDirectoryObject, ITDirectoryObject_Vtbl, 0x34621d6e_6cff_11d1_aff7_00c04fc31fee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDirectoryObject {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDirectoryObject, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObject {
    pub unsafe fn ObjectType(&self) -> windows_core::Result<DIRECTORY_OBJECT_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, pname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pname.param().abi()).ok()
    }
    pub unsafe fn get_DialableAddrs(&self, dwaddresstype: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_DialableAddrs)(windows_core::Interface::as_raw(self), dwaddresstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateDialableAddrs(&self, dwaddresstype: u32) -> windows_core::Result<IEnumDialableAddrs> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateDialableAddrs)(windows_core::Interface::as_raw(self), dwaddresstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityDescriptor(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, psecdes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), psecdes.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectoryObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIRECTORY_OBJECT_TYPE) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_DialableAddrs: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateDialableAddrs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDirectoryObjectConference, ITDirectoryObjectConference_Vtbl, 0xf1029e5d_cb5b_11d0_8d59_00c04fd91ac0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDirectoryObjectConference {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDirectoryObjectConference, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectConference {
    pub unsafe fn Protocol(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Originator(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Originator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOriginator<P0>(&self, poriginator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOriginator)(windows_core::Interface::as_raw(self), poriginator.param().abi()).ok()
    }
    pub unsafe fn AdvertisingScope(&self) -> windows_core::Result<RND_ADVERTISING_SCOPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdvertisingScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAdvertisingScope(&self, advertisingscope: RND_ADVERTISING_SCOPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAdvertisingScope)(windows_core::Interface::as_raw(self), advertisingscope).ok()
    }
    pub unsafe fn Url(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Url)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUrl<P0>(&self, purl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUrl)(windows_core::Interface::as_raw(self), purl.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, pdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), pdescription.param().abi()).ok()
    }
    pub unsafe fn IsEncrypted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEncrypted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsEncrypted<P0>(&self, fencrypted: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsEncrypted)(windows_core::Interface::as_raw(self), fencrypted.param().abi()).ok()
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartTime(&self, date: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), date).ok()
    }
    pub unsafe fn StopTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StopTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStopTime(&self, date: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStopTime)(windows_core::Interface::as_raw(self), date).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectoryObjectConference_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Originator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOriginator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AdvertisingScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RND_ADVERTISING_SCOPE) -> windows_core::HRESULT,
    pub SetAdvertisingScope: unsafe extern "system" fn(*mut core::ffi::c_void, RND_ADVERTISING_SCOPE) -> windows_core::HRESULT,
    pub Url: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUrl: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub StopTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStopTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDirectoryObjectUser, ITDirectoryObjectUser_Vtbl, 0x34621d6f_6cff_11d1_aff7_00c04fc31fee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDirectoryObjectUser {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDirectoryObjectUser, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectUser {
    pub unsafe fn IPPhonePrimary(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IPPhonePrimary)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetIPPhonePrimary<P0>(&self, pname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetIPPhonePrimary)(windows_core::Interface::as_raw(self), pname.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDirectoryObjectUser_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IPPhonePrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetIPPhonePrimary: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITDispatchMapper, ITDispatchMapper_Vtbl, 0xe9225295_c759_11d1_a02b_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITDispatchMapper {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITDispatchMapper, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITDispatchMapper {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatchInterface<P0, P1>(&self, piid: P0, pinterfacetomap: P1) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDispatchInterface)(windows_core::Interface::as_raw(self), piid.param().abi(), pinterfacetomap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITDispatchMapper_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatchInterface: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatchInterface: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITFileTerminalEvent, ITFileTerminalEvent_Vtbl, 0xe4a7fbac_8c17_4427_9f55_9f589ac8af00);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITFileTerminalEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITFileTerminalEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITFileTerminalEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Track(&self) -> windows_core::Result<ITFileTrack> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Track)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<TERMINAL_MEDIA_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Cause(&self) -> windows_core::Result<FT_STATE_EVENT_CAUSE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cause)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITFileTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Track: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Track: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_MEDIA_STATE) -> windows_core::HRESULT,
    pub Cause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FT_STATE_EVENT_CAUSE) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITFileTrack, ITFileTrack_Vtbl, 0x31ca6ea9_c08a_4bea_8811_8e9c1ba3ea3a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITFileTrack {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITFileTrack, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITFileTrack {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn Format(&self) -> windows_core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetFormat(&self, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), pmt).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ControllingTerminal(&self) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ControllingTerminal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormatForScripting(&self) -> windows_core::Result<ITScriptableAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioFormatForScripting)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAudioFormatForScripting<P0>(&self, paudioformat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITScriptableAudioFormat>,
    {
        (windows_core::Interface::vtable(self).SetAudioFormatForScripting)(windows_core::Interface::as_raw(self), paudioformat.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EmptyAudioFormatForScripting(&self) -> windows_core::Result<ITScriptableAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EmptyAudioFormatForScripting)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITFileTrack_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    Format: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    SetFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ControllingTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ControllingTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormatForScripting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormatForScripting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAudioFormatForScripting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAudioFormatForScripting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EmptyAudioFormatForScripting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EmptyAudioFormatForScripting: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITForwardInformation, ITForwardInformation_Vtbl, 0x449f659e_88a3_11d1_bb5d_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITForwardInformation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITForwardInformation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation {
    pub unsafe fn SetNumRingsNoAnswer(&self, lnumrings: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNumRingsNoAnswer)(windows_core::Interface::as_raw(self), lnumrings).ok()
    }
    pub unsafe fn NumRingsNoAnswer(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumRingsNoAnswer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetForwardType<P0, P1>(&self, forwardtype: i32, pdestaddress: P0, pcalleraddress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetForwardType)(windows_core::Interface::as_raw(self), forwardtype, pdestaddress.param().abi(), pcalleraddress.param().abi()).ok()
    }
    pub unsafe fn get_ForwardTypeDestination(&self, forwardtype: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ForwardTypeDestination)(windows_core::Interface::as_raw(self), forwardtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_ForwardTypeCaller(&self, forwardtype: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ForwardTypeCaller)(windows_core::Interface::as_raw(self), forwardtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetForwardType(&self, forwardtype: i32, ppdestinationaddress: *mut windows_core::BSTR, ppcalleraddress: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetForwardType)(windows_core::Interface::as_raw(self), forwardtype, core::mem::transmute(ppdestinationaddress), core::mem::transmute(ppcalleraddress)).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITForwardInformation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetNumRingsNoAnswer: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NumRingsNoAnswer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetForwardType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_ForwardTypeDestination: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_ForwardTypeCaller: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetForwardType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITForwardInformation2, ITForwardInformation2_Vtbl, 0x5229b4ed_b260_4382_8e1a_5df3a8a4ccc0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITForwardInformation2 {
    type Target = ITForwardInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITForwardInformation2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITForwardInformation);
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation2 {
    pub unsafe fn SetForwardType2<P0, P1>(&self, forwardtype: i32, pdestaddress: P0, destaddresstype: i32, pcalleraddress: P1, calleraddresstype: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetForwardType2)(windows_core::Interface::as_raw(self), forwardtype, pdestaddress.param().abi(), destaddresstype, pcalleraddress.param().abi(), calleraddresstype).ok()
    }
    pub unsafe fn GetForwardType2(&self, forwardtype: i32, ppdestinationaddress: *mut windows_core::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut windows_core::BSTR, pcalleraddresstype: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetForwardType2)(windows_core::Interface::as_raw(self), forwardtype, core::mem::transmute(ppdestinationaddress), pdestaddresstype, core::mem::transmute(ppcalleraddress), pcalleraddresstype).ok()
    }
    pub unsafe fn get_ForwardTypeDestinationAddressType(&self, forwardtype: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ForwardTypeDestinationAddressType)(windows_core::Interface::as_raw(self), forwardtype, &mut result__).map(|| result__)
    }
    pub unsafe fn get_ForwardTypeCallerAddressType(&self, forwardtype: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ForwardTypeCallerAddressType)(windows_core::Interface::as_raw(self), forwardtype, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITForwardInformation2_Vtbl {
    pub base__: ITForwardInformation_Vtbl,
    pub SetForwardType2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub GetForwardType2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub get_ForwardTypeDestinationAddressType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub get_ForwardTypeCallerAddressType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITILSConfig, ITILSConfig_Vtbl, 0x34621d72_6cff_11d1_aff7_00c04fc31fee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITILSConfig {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITILSConfig, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITILSConfig {
    pub unsafe fn Port(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Port)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPort(&self, port: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), port).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITILSConfig_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITLegacyAddressMediaControl, ITLegacyAddressMediaControl_Vtbl, 0xab493640_4c0b_11d2_a046_00c04fb6809f);
impl core::ops::Deref for ITLegacyAddressMediaControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITLegacyAddressMediaControl, windows_core::IUnknown);
impl ITLegacyAddressMediaControl {
    pub unsafe fn GetID<P0>(&self, pdeviceclass: P0, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), pdeviceclass.param().abi(), pdwsize, ppdeviceid).ok()
    }
    pub unsafe fn GetDevConfig<P0>(&self, pdeviceclass: P0, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetDevConfig)(windows_core::Interface::as_raw(self), pdeviceclass.param().abi(), pdwsize, ppdeviceconfig).ok()
    }
    pub unsafe fn SetDevConfig<P0>(&self, pdeviceclass: P0, pdeviceconfig: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDevConfig)(windows_core::Interface::as_raw(self), pdeviceclass.param().abi(), pdeviceconfig.len().try_into().unwrap(), core::mem::transmute(pdeviceconfig.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ITLegacyAddressMediaControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetDevConfig: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetDevConfig: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITLegacyAddressMediaControl2, ITLegacyAddressMediaControl2_Vtbl, 0xb0ee512b_a531_409e_9dd9_4099fe86c738);
impl core::ops::Deref for ITLegacyAddressMediaControl2 {
    type Target = ITLegacyAddressMediaControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITLegacyAddressMediaControl2, windows_core::IUnknown, ITLegacyAddressMediaControl);
impl ITLegacyAddressMediaControl2 {
    pub unsafe fn ConfigDialog<P0, P1>(&self, hwndowner: P0, pdeviceclass: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ConfigDialog)(windows_core::Interface::as_raw(self), hwndowner.param().abi(), pdeviceclass.param().abi()).ok()
    }
    pub unsafe fn ConfigDialogEdit<P0, P1>(&self, hwndowner: P0, pdeviceclass: P1, pdeviceconfigin: &[u8], pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ConfigDialogEdit)(windows_core::Interface::as_raw(self), hwndowner.param().abi(), pdeviceclass.param().abi(), pdeviceconfigin.len().try_into().unwrap(), core::mem::transmute(pdeviceconfigin.as_ptr()), pdwsizeout, ppdeviceconfigout).ok()
    }
}
#[repr(C)]
pub struct ITLegacyAddressMediaControl2_Vtbl {
    pub base__: ITLegacyAddressMediaControl_Vtbl,
    pub ConfigDialog: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ConfigDialogEdit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const u8, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITLegacyCallMediaControl, ITLegacyCallMediaControl_Vtbl, 0xd624582f_cc23_4436_b8a5_47c625c8045d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITLegacyCallMediaControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITLegacyCallMediaControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl {
    pub unsafe fn DetectDigits(&self, digitmode: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DetectDigits)(windows_core::Interface::as_raw(self), digitmode).ok()
    }
    pub unsafe fn GenerateDigits<P0>(&self, pdigits: P0, digitmode: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GenerateDigits)(windows_core::Interface::as_raw(self), pdigits.param().abi(), digitmode).ok()
    }
    pub unsafe fn GetID<P0>(&self, pdeviceclass: P0, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), pdeviceclass.param().abi(), pdwsize, ppdeviceid).ok()
    }
    pub unsafe fn SetMediaType(&self, lmediatype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMediaType)(windows_core::Interface::as_raw(self), lmediatype).ok()
    }
    pub unsafe fn MonitorMedia(&self, lmediatype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MonitorMedia)(windows_core::Interface::as_raw(self), lmediatype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLegacyCallMediaControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DetectDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GenerateDigits: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MonitorMedia: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITLegacyCallMediaControl2, ITLegacyCallMediaControl2_Vtbl, 0x57ca332d_7bc2_44f1_a60c_936fe8d7ce73);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITLegacyCallMediaControl2 {
    type Target = ITLegacyCallMediaControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITLegacyCallMediaControl2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITLegacyCallMediaControl);
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl2 {
    pub unsafe fn GenerateDigits2<P0>(&self, pdigits: P0, digitmode: i32, lduration: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GenerateDigits2)(windows_core::Interface::as_raw(self), pdigits.param().abi(), digitmode, lduration).ok()
    }
    pub unsafe fn GatherDigits<P0>(&self, digitmode: i32, lnumdigits: i32, pterminationdigits: P0, lfirstdigittimeout: i32, linterdigittimeout: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GatherDigits)(windows_core::Interface::as_raw(self), digitmode, lnumdigits, pterminationdigits.param().abi(), lfirstdigittimeout, linterdigittimeout).ok()
    }
    pub unsafe fn DetectTones(&self, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DetectTones)(windows_core::Interface::as_raw(self), ptonelist, lnumtones).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectTonesByCollection<P0>(&self, pdetecttonecollection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITCollection2>,
    {
        (windows_core::Interface::vtable(self).DetectTonesByCollection)(windows_core::Interface::as_raw(self), pdetecttonecollection.param().abi()).ok()
    }
    pub unsafe fn GenerateTone(&self, tonemode: TAPI_TONEMODE, lduration: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenerateTone)(windows_core::Interface::as_raw(self), tonemode, lduration).ok()
    }
    pub unsafe fn GenerateCustomTones(&self, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenerateCustomTones)(windows_core::Interface::as_raw(self), ptonelist, lnumtones, lduration).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateCustomTonesByCollection<P0>(&self, pcustomtonecollection: P0, lduration: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITCollection2>,
    {
        (windows_core::Interface::vtable(self).GenerateCustomTonesByCollection)(windows_core::Interface::as_raw(self), pcustomtonecollection.param().abi(), lduration).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDetectToneObject(&self) -> windows_core::Result<ITDetectTone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDetectToneObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCustomToneObject(&self) -> windows_core::Result<ITCustomTone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomToneObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIDAsVariant<P0>(&self, bstrdeviceclass: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIDAsVariant)(windows_core::Interface::as_raw(self), bstrdeviceclass.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLegacyCallMediaControl2_Vtbl {
    pub base__: ITLegacyCallMediaControl_Vtbl,
    pub GenerateDigits2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32) -> windows_core::HRESULT,
    pub GatherDigits: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32) -> windows_core::HRESULT,
    pub DetectTones: unsafe extern "system" fn(*mut core::ffi::c_void, *const TAPI_DETECTTONE, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectTonesByCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectTonesByCollection: usize,
    pub GenerateTone: unsafe extern "system" fn(*mut core::ffi::c_void, TAPI_TONEMODE, i32) -> windows_core::HRESULT,
    pub GenerateCustomTones: unsafe extern "system" fn(*mut core::ffi::c_void, *const TAPI_CUSTOMTONE, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateCustomTonesByCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateCustomTonesByCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDetectToneObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDetectToneObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCustomToneObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCustomToneObject: usize,
    pub GetIDAsVariant: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITLegacyWaveSupport, ITLegacyWaveSupport_Vtbl, 0x207823ea_e252_11d2_b77e_0080c7135381);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITLegacyWaveSupport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITLegacyWaveSupport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyWaveSupport {
    pub unsafe fn IsFullDuplex(&self) -> windows_core::Result<FULLDUPLEX_SUPPORT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFullDuplex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLegacyWaveSupport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IsFullDuplex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FULLDUPLEX_SUPPORT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITLocationInfo, ITLocationInfo_Vtbl, 0x0c4d8eff_8ddb_11d1_a09e_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITLocationInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITLocationInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITLocationInfo {
    pub unsafe fn PermanentLocationID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PermanentLocationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CountryCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CountryCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CountryID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CountryID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Options(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Options)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PreferredCardID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreferredCardID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocationName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocationName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CityCode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CityCode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LocalAccessCode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalAccessCode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LongDistanceAccessCode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LongDistanceAccessCode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TollPrefixList(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TollPrefixList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CancelCallWaitingCode(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CancelCallWaitingCode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITLocationInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PermanentLocationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CountryCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CountryID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PreferredCardID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LocationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CityCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalAccessCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LongDistanceAccessCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TollPrefixList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CancelCallWaitingCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITMSPAddress, ITMSPAddress_Vtbl, 0xee3bd600_3868_11d2_a045_00c04fb6809f);
impl core::ops::Deref for ITMSPAddress {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITMSPAddress, windows_core::IUnknown);
impl ITMSPAddress {
    pub unsafe fn Initialize(&self, hevent: *const i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hevent).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateMSPCall<P0>(&self, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMSPCall)(windows_core::Interface::as_raw(self), hcall, dwreserved, dwmediatype, pouterunknown.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShutdownMSPCall<P0>(&self, pstreamcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ShutdownMSPCall)(windows_core::Interface::as_raw(self), pstreamcontrol.param().abi()).ok()
    }
    pub unsafe fn ReceiveTSPData<P0>(&self, pmspcall: P0, pbuffer: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ReceiveTSPData)(windows_core::Interface::as_raw(self), pmspcall.param().abi(), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetEvent(&self, pdwsize: *mut u32, peventbuffer: *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetEvent)(windows_core::Interface::as_raw(self), pdwsize, peventbuffer).ok()
    }
}
#[repr(C)]
pub struct ITMSPAddress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMSPCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownMSPCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReceiveTSPData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITMediaControl, ITMediaControl_Vtbl, 0xc445dde8_5199_4bc7_9807_5ffb92e42e09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITMediaControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITMediaControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaControl {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MediaState(&self) -> windows_core::Result<TERMINAL_MEDIA_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_MEDIA_STATE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITMediaPlayback, ITMediaPlayback_Vtbl, 0x627e8ae6_ae4c_4a69_bb63_2ad625404b77);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITMediaPlayback {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITMediaPlayback, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaPlayback {
    pub unsafe fn SetPlayList<P0>(&self, playlistvariant: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPlayList)(windows_core::Interface::as_raw(self), playlistvariant.param().abi()).ok()
    }
    pub unsafe fn PlayList(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PlayList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaPlayback_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetPlayList: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PlayList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITMediaRecord, ITMediaRecord_Vtbl, 0xf5dd4592_5476_4cc1_9d4d_fad3eefe7db2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITMediaRecord {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITMediaRecord, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaRecord {
    pub unsafe fn SetFileName<P0>(&self, bstrfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFileName)(windows_core::Interface::as_raw(self), bstrfilename.param().abi()).ok()
    }
    pub unsafe fn FileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaRecord_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITMediaSupport, ITMediaSupport_Vtbl, 0xb1efc384_9355_11d0_835c_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITMediaSupport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITMediaSupport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITMediaSupport {
    pub unsafe fn MediaTypes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryMediaType(&self, lmediatype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMediaType)(windows_core::Interface::as_raw(self), lmediatype, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMediaSupport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub QueryMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITMultiTrackTerminal, ITMultiTrackTerminal_Vtbl, 0xfe040091_ade8_4072_95c9_bf7de8c54b44);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITMultiTrackTerminal {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITMultiTrackTerminal, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITMultiTrackTerminal {
    pub unsafe fn TrackTerminals(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TrackTerminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateTrackTerminals(&self) -> windows_core::Result<IEnumTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateTrackTerminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTrackTerminal(&self, mediatype: i32, terminaldirection: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTrackTerminal)(windows_core::Interface::as_raw(self), mediatype, terminaldirection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MediaTypesInUse(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaTypesInUse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DirectionsInUse(&self) -> windows_core::Result<TERMINAL_DIRECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DirectionsInUse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveTrackTerminal<P0>(&self, ptrackterminaltoremove: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        (windows_core::Interface::vtable(self).RemoveTrackTerminal)(windows_core::Interface::as_raw(self), ptrackterminaltoremove.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITMultiTrackTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub TrackTerminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateTrackTerminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTrackTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, i32, TERMINAL_DIRECTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTrackTerminal: usize,
    pub MediaTypesInUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DirectionsInUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_DIRECTION) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveTrackTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveTrackTerminal: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITPhone, ITPhone_Vtbl, 0x09d48db4_10cc_4388_9de7_a8465618975a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITPhone {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITPhone, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITPhone {
    pub unsafe fn Open(&self, privilege: PHONE_PRIVILEGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), privilege).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Addresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Addresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateAddresses(&self) -> windows_core::Result<IEnumAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_PhoneCapsLong(&self, pclcap: PHONECAPS_LONG) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PhoneCapsLong)(windows_core::Interface::as_raw(self), pclcap, &mut result__).map(|| result__)
    }
    pub unsafe fn get_PhoneCapsString(&self, pcscap: PHONECAPS_STRING) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PhoneCapsString)(windows_core::Interface::as_raw(self), pcscap, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Terminals<P0>(&self, paddress: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<ITAddress>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Terminals)(windows_core::Interface::as_raw(self), paddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateTerminals<P0>(&self, paddress: P0) -> windows_core::Result<IEnumTerminal>
    where
        P0: windows_core::Param<ITAddress>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateTerminals)(windows_core::Interface::as_raw(self), paddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_ButtonMode(&self, lbuttonid: i32) -> windows_core::Result<PHONE_BUTTON_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ButtonMode)(windows_core::Interface::as_raw(self), lbuttonid, &mut result__).map(|| result__)
    }
    pub unsafe fn put_ButtonMode(&self, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_ButtonMode)(windows_core::Interface::as_raw(self), lbuttonid, buttonmode).ok()
    }
    pub unsafe fn get_ButtonFunction(&self, lbuttonid: i32) -> windows_core::Result<PHONE_BUTTON_FUNCTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ButtonFunction)(windows_core::Interface::as_raw(self), lbuttonid, &mut result__).map(|| result__)
    }
    pub unsafe fn put_ButtonFunction(&self, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_ButtonFunction)(windows_core::Interface::as_raw(self), lbuttonid, buttonfunction).ok()
    }
    pub unsafe fn get_ButtonText(&self, lbuttonid: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ButtonText)(windows_core::Interface::as_raw(self), lbuttonid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_ButtonText<P0>(&self, lbuttonid: i32, bstrbuttontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_ButtonText)(windows_core::Interface::as_raw(self), lbuttonid, bstrbuttontext.param().abi()).ok()
    }
    pub unsafe fn get_ButtonState(&self, lbuttonid: i32) -> windows_core::Result<PHONE_BUTTON_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_ButtonState)(windows_core::Interface::as_raw(self), lbuttonid, &mut result__).map(|| result__)
    }
    pub unsafe fn get_HookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE) -> windows_core::Result<PHONE_HOOK_SWITCH_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_HookSwitchState)(windows_core::Interface::as_raw(self), hookswitchdevice, &mut result__).map(|| result__)
    }
    pub unsafe fn put_HookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_HookSwitchState)(windows_core::Interface::as_raw(self), hookswitchdevice, hookswitchstate).ok()
    }
    pub unsafe fn SetRingMode(&self, lringmode: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRingMode)(windows_core::Interface::as_raw(self), lringmode).ok()
    }
    pub unsafe fn RingMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RingMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRingVolume(&self, lringvolume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRingVolume)(windows_core::Interface::as_raw(self), lringvolume).ok()
    }
    pub unsafe fn RingVolume(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RingVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Privilege(&self) -> windows_core::Result<PHONE_PRIVILEGE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Privilege)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPhoneCapsBuffer)(windows_core::Interface::as_raw(self), pcbcaps, pdwsize, ppphonecapsbuffer).ok()
    }
    pub unsafe fn get_PhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PhoneCapsBuffer)(windows_core::Interface::as_raw(self), pcbcaps, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_LampMode(&self, llampid: i32) -> windows_core::Result<PHONE_LAMP_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_LampMode)(windows_core::Interface::as_raw(self), llampid, &mut result__).map(|| result__)
    }
    pub unsafe fn put_LampMode(&self, llampid: i32, lampmode: PHONE_LAMP_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_LampMode)(windows_core::Interface::as_raw(self), llampid, lampmode).ok()
    }
    pub unsafe fn Display(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Display)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDisplay<P0>(&self, lrow: i32, lcolumn: i32, bstrdisplay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplay)(windows_core::Interface::as_raw(self), lrow, lcolumn, bstrdisplay.param().abi()).ok()
    }
    pub unsafe fn PreferredAddresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PreferredAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePreferredAddresses(&self) -> windows_core::Result<IEnumAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePreferredAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeviceSpecific(&self, pparams: *const u8, dwsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeviceSpecific)(windows_core::Interface::as_raw(self), pparams, dwsize).ok()
    }
    pub unsafe fn DeviceSpecificVariant<P0>(&self, vardevspecificbytearray: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).DeviceSpecificVariant)(windows_core::Interface::as_raw(self), vardevspecificbytearray.param().abi()).ok()
    }
    pub unsafe fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NegotiateExtVersion)(windows_core::Interface::as_raw(self), llowversion, lhighversion, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPhone_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, PHONE_PRIVILEGE) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Addresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PhoneCapsLong: unsafe extern "system" fn(*mut core::ffi::c_void, PHONECAPS_LONG, *mut i32) -> windows_core::HRESULT,
    pub get_PhoneCapsString: unsafe extern "system" fn(*mut core::ffi::c_void, PHONECAPS_STRING, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Terminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Terminals: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateTerminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateTerminals: usize,
    pub get_ButtonMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut PHONE_BUTTON_MODE) -> windows_core::HRESULT,
    pub put_ButtonMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PHONE_BUTTON_MODE) -> windows_core::HRESULT,
    pub get_ButtonFunction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut PHONE_BUTTON_FUNCTION) -> windows_core::HRESULT,
    pub put_ButtonFunction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PHONE_BUTTON_FUNCTION) -> windows_core::HRESULT,
    pub get_ButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_ButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub get_ButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut PHONE_BUTTON_STATE) -> windows_core::HRESULT,
    pub get_HookSwitchState: unsafe extern "system" fn(*mut core::ffi::c_void, PHONE_HOOK_SWITCH_DEVICE, *mut PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT,
    pub put_HookSwitchState: unsafe extern "system" fn(*mut core::ffi::c_void, PHONE_HOOK_SWITCH_DEVICE, PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT,
    pub SetRingMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRingVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RingVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Privilege: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PHONE_PRIVILEGE) -> windows_core::HRESULT,
    pub GetPhoneCapsBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, PHONECAPS_BUFFER, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub get_PhoneCapsBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, PHONECAPS_BUFFER, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub get_LampMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut PHONE_LAMP_MODE) -> windows_core::HRESULT,
    pub put_LampMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PHONE_LAMP_MODE) -> windows_core::HRESULT,
    pub Display: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PreferredAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumeratePreferredAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub DeviceSpecificVariant: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub NegotiateExtVersion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITPhoneDeviceSpecificEvent, ITPhoneDeviceSpecificEvent_Vtbl, 0x63ffb2a6_872b_4cd3_a501_326e8fb40af7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITPhoneDeviceSpecificEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITPhoneDeviceSpecificEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneDeviceSpecificEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Phone(&self) -> windows_core::Result<ITPhone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Phone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn lParam1(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).lParam1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn lParam2(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).lParam2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn lParam3(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).lParam3)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPhoneDeviceSpecificEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
    pub lParam1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub lParam2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub lParam3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITPhoneEvent, ITPhoneEvent_Vtbl, 0x8f942dd8_64ed_4aaf_a77d_b23db0837ead);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITPhoneEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITPhoneEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Phone(&self) -> windows_core::Result<ITPhone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Phone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<PHONE_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ButtonState(&self) -> windows_core::Result<PHONE_BUTTON_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ButtonState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HookSwitchState(&self) -> windows_core::Result<PHONE_HOOK_SWITCH_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HookSwitchState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HookSwitchDevice(&self) -> windows_core::Result<PHONE_HOOK_SWITCH_DEVICE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HookSwitchDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RingMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RingMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ButtonLampId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ButtonLampId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberGathered(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberGathered)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPhoneEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PHONE_EVENT) -> windows_core::HRESULT,
    pub ButtonState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PHONE_BUTTON_STATE) -> windows_core::HRESULT,
    pub HookSwitchState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PHONE_HOOK_SWITCH_STATE) -> windows_core::HRESULT,
    pub HookSwitchDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PHONE_HOOK_SWITCH_DEVICE) -> windows_core::HRESULT,
    pub RingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ButtonLampId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberGathered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITPluggableTerminalClassInfo, ITPluggableTerminalClassInfo_Vtbl, 0x41757f4a_cf09_4b34_bc96_0a79d2390076);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITPluggableTerminalClassInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITPluggableTerminalClassInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalClassInfo {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Company(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Company)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TerminalClass(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalClass)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<TERMINAL_DIRECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MediaTypes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPluggableTerminalClassInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Company: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TerminalClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_DIRECTION) -> windows_core::HRESULT,
    pub MediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITPluggableTerminalEventSink, ITPluggableTerminalEventSink_Vtbl, 0x6e0887be_ba1a_492e_bd10_4020ec5e33e0);
impl core::ops::Deref for ITPluggableTerminalEventSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITPluggableTerminalEventSink, windows_core::IUnknown);
impl ITPluggableTerminalEventSink {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireEvent(&self, pmspeventinfo: *const MSP_EVENT_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FireEvent)(windows_core::Interface::as_raw(self), pmspeventinfo).ok()
    }
}
#[repr(C)]
pub struct ITPluggableTerminalEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FireEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const MSP_EVENT_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireEvent: usize,
}
windows_core::imp::define_interface!(ITPluggableTerminalEventSinkRegistration, ITPluggableTerminalEventSinkRegistration_Vtbl, 0xf7115709_a216_4957_a759_060ab32a90d1);
impl core::ops::Deref for ITPluggableTerminalEventSinkRegistration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITPluggableTerminalEventSinkRegistration, windows_core::IUnknown);
impl ITPluggableTerminalEventSinkRegistration {
    pub unsafe fn RegisterSink<P0>(&self, peventsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITPluggableTerminalEventSink>,
    {
        (windows_core::Interface::vtable(self).RegisterSink)(windows_core::Interface::as_raw(self), peventsink.param().abi()).ok()
    }
    pub unsafe fn UnregisterSink(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterSink)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITPluggableTerminalEventSinkRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterSink: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITPluggableTerminalSuperclassInfo, ITPluggableTerminalSuperclassInfo_Vtbl, 0x6d54e42c_4625_4359_a6f7_631999107e05);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITPluggableTerminalSuperclassInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITPluggableTerminalSuperclassInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalSuperclassInfo {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPluggableTerminalSuperclassInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITPrivateEvent, ITPrivateEvent_Vtbl, 0x0e269cd0_10d4_4121_9c22_9c85d625650d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITPrivateEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITPrivateEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITPrivateEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> windows_core::Result<ITAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CallHub(&self) -> windows_core::Result<ITCallHub> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallHub)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EventCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EventInterface(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITPrivateEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CallHub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CallHub: usize,
    pub EventCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EventInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EventInterface: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITQOSEvent, ITQOSEvent_Vtbl, 0xcfa3357c_ad77_11d1_bb68_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITQOSEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITQOSEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITQOSEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<QOS_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MediaType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITQOSEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut QOS_EVENT) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITQueue, ITQueue_Vtbl, 0x5afc3149_4bcc_11d1_bf80_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITQueue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITQueue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITQueue {
    pub unsafe fn SetMeasurementPeriod(&self, lperiod: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMeasurementPeriod)(windows_core::Interface::as_raw(self), lperiod).ok()
    }
    pub unsafe fn MeasurementPeriod(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MeasurementPeriod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalCallsQueued(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalCallsQueued)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentCallsQueued(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentCallsQueued)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalCallsAbandoned(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalCallsAbandoned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalCallsFlowedIn(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalCallsFlowedIn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalCallsFlowedOut(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalCallsFlowedOut)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LongestEverWaitTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LongestEverWaitTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentLongestWaitTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentLongestWaitTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AverageWaitTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AverageWaitTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FinalDisposition(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FinalDisposition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITQueue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetMeasurementPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MeasurementPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalCallsQueued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentCallsQueued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalCallsAbandoned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalCallsFlowedIn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalCallsFlowedOut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LongestEverWaitTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentLongestWaitTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AverageWaitTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FinalDisposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITQueueEvent, ITQueueEvent_Vtbl, 0x297f3033_bd11_11d1_a0a7_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITQueueEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITQueueEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITQueueEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Queue(&self) -> windows_core::Result<ITQueue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Queue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<ACDQUEUE_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITQueueEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Queue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Queue: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ACDQUEUE_EVENT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITRendezvous, ITRendezvous_Vtbl, 0x34621d6b_6cff_11d1_aff7_00c04fc31fee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITRendezvous {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITRendezvous, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITRendezvous {
    pub unsafe fn DefaultDirectories(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultDirectories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateDefaultDirectories(&self) -> windows_core::Result<IEnumDirectory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateDefaultDirectories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectory<P0>(&self, directorytype: DIRECTORY_TYPE, pname: P0) -> windows_core::Result<ITDirectory>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDirectory)(windows_core::Interface::as_raw(self), directorytype, pname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDirectoryObject<P0>(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: P0) -> windows_core::Result<ITDirectoryObject>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDirectoryObject)(windows_core::Interface::as_raw(self), directoryobjecttype, pname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITRendezvous_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DefaultDirectories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateDefaultDirectories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTORY_TYPE, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDirectory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDirectoryObject: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTORY_OBJECT_TYPE, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDirectoryObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITRequest, ITRequest_Vtbl, 0xac48ffdf_f8c4_11d1_a030_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITRequest {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITRequest, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITRequest {
    pub unsafe fn MakeCall<P0, P1, P2, P3>(&self, pdestaddress: P0, pappname: P1, pcalledparty: P2, pcomment: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).MakeCall)(windows_core::Interface::as_raw(self), pdestaddress.param().abi(), pappname.param().abi(), pcalledparty.param().abi(), pcomment.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITRequest_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MakeCall: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITRequestEvent, ITRequestEvent_Vtbl, 0xac48ffde_f8c4_11d1_a030_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITRequestEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITRequestEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITRequestEvent {
    pub unsafe fn RegistrationInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegistrationInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DestAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DestAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AppName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CalledParty(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CalledParty)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Comment(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Comment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITRequestEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RegistrationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RequestMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DestAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AppName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CalledParty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITScriptableAudioFormat, ITScriptableAudioFormat_Vtbl, 0xb87658bd_3c59_4f64_be74_aede3e86a81e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITScriptableAudioFormat {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITScriptableAudioFormat, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITScriptableAudioFormat {
    pub unsafe fn Channels(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Channels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetChannels(&self, nnewval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannels)(windows_core::Interface::as_raw(self), nnewval).ok()
    }
    pub unsafe fn SamplesPerSec(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SamplesPerSec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSamplesPerSec(&self, nnewval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSamplesPerSec)(windows_core::Interface::as_raw(self), nnewval).ok()
    }
    pub unsafe fn AvgBytesPerSec(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AvgBytesPerSec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAvgBytesPerSec(&self, nnewval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAvgBytesPerSec)(windows_core::Interface::as_raw(self), nnewval).ok()
    }
    pub unsafe fn BlockAlign(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BlockAlign)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBlockAlign(&self, nnewval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlockAlign)(windows_core::Interface::as_raw(self), nnewval).ok()
    }
    pub unsafe fn BitsPerSample(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BitsPerSample)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBitsPerSample(&self, nnewval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBitsPerSample)(windows_core::Interface::as_raw(self), nnewval).ok()
    }
    pub unsafe fn FormatTag(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFormatTag(&self, nnewval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormatTag)(windows_core::Interface::as_raw(self), nnewval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITScriptableAudioFormat_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Channels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetChannels: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SamplesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSamplesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AvgBytesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAvgBytesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BlockAlign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBlockAlign: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FormatTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFormatTag: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITStaticAudioTerminal, ITStaticAudioTerminal_Vtbl, 0xa86b7871_d14c_48e6_922e_a8d15f984800);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITStaticAudioTerminal {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITStaticAudioTerminal, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITStaticAudioTerminal {
    pub unsafe fn WaveId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaveId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITStaticAudioTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WaveId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITStream, ITStream_Vtbl, 0xee3bd605_3868_11d2_a045_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITStream {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITStream, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITStream {
    pub unsafe fn MediaType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<TERMINAL_DIRECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StartStream(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartStream)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PauseStream(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PauseStream)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StopStream(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopStream)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectTerminal<P0>(&self, pterminal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        (windows_core::Interface::vtable(self).SelectTerminal)(windows_core::Interface::as_raw(self), pterminal.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectTerminal<P0>(&self, pterminal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        (windows_core::Interface::vtable(self).UnselectTerminal)(windows_core::Interface::as_raw(self), pterminal.param().abi()).ok()
    }
    pub unsafe fn EnumerateTerminals(&self) -> windows_core::Result<IEnumTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateTerminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Terminals(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_DIRECTION) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminal: usize,
    pub EnumerateTerminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITStreamControl, ITStreamControl_Vtbl, 0xee3bd604_3868_11d2_a045_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITStreamControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITStreamControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITStreamControl {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self, lmediatype: i32, td: TERMINAL_DIRECTION) -> windows_core::Result<ITStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), lmediatype, td, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveStream<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITStream>,
    {
        (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok()
    }
    pub unsafe fn EnumerateStreams(&self) -> windows_core::Result<IEnumStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Streams(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Streams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITStreamControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, TERMINAL_DIRECTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveStream: usize,
    pub EnumerateStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Streams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITSubStream, ITSubStream_Vtbl, 0xee3bd608_3868_11d2_a045_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITSubStream {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITSubStream, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITSubStream {
    pub unsafe fn StartSubStream(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartSubStream)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PauseSubStream(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PauseSubStream)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StopSubStream(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopSubStream)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectTerminal<P0>(&self, pterminal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        (windows_core::Interface::vtable(self).SelectTerminal)(windows_core::Interface::as_raw(self), pterminal.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnselectTerminal<P0>(&self, pterminal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITTerminal>,
    {
        (windows_core::Interface::vtable(self).UnselectTerminal)(windows_core::Interface::as_raw(self), pterminal.param().abi()).ok()
    }
    pub unsafe fn EnumerateTerminals(&self) -> windows_core::Result<IEnumTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateTerminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Terminals(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Stream(&self) -> windows_core::Result<ITStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITSubStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartSubStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseSubStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopSubStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UnselectTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UnselectTerminal: usize,
    pub EnumerateTerminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITSubStreamControl, ITSubStreamControl_Vtbl, 0xee3bd607_3868_11d2_a045_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITSubStreamControl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITSubStreamControl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITSubStreamControl {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSubStream(&self) -> windows_core::Result<ITSubStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSubStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveSubStream<P0>(&self, psubstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITSubStream>,
    {
        (windows_core::Interface::vtable(self).RemoveSubStream)(windows_core::Interface::as_raw(self), psubstream.param().abi()).ok()
    }
    pub unsafe fn EnumerateSubStreams(&self) -> windows_core::Result<IEnumSubStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateSubStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SubStreams(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITSubStreamControl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSubStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSubStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveSubStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveSubStream: usize,
    pub EnumerateSubStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTAPI, ITTAPI_Vtbl, 0xb1efc382_9355_11d0_835c_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTAPI {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTAPI, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI {
    pub unsafe fn Initialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Addresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Addresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateAddresses(&self) -> windows_core::Result<IEnumAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterCallNotifications<P0, P1, P2>(&self, paddress: P0, fmonitor: P1, fowner: P2, lmediatypes: i32, lcallbackinstance: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITAddress>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterCallNotifications)(windows_core::Interface::as_raw(self), paddress.param().abi(), fmonitor.param().abi(), fowner.param().abi(), lmediatypes, lcallbackinstance, &mut result__).map(|| result__)
    }
    pub unsafe fn UnregisterNotifications(&self, lregister: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterNotifications)(windows_core::Interface::as_raw(self), lregister).ok()
    }
    pub unsafe fn CallHubs(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallHubs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateCallHubs(&self) -> windows_core::Result<IEnumCallHub> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateCallHubs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCallHubTracking<P0, P1>(&self, paddresses: P0, btracking: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCallHubTracking)(windows_core::Interface::as_raw(self), paddresses.param().abi(), btracking.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumeratePrivateTAPIObjects(&self) -> windows_core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePrivateTAPIObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PrivateTAPIObjects(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateTAPIObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterRequestRecipient<P0>(&self, lregistrationinstance: i32, lrequestmode: i32, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).RegisterRequestRecipient)(windows_core::Interface::as_raw(self), lregistrationinstance, lrequestmode, fenable.param().abi()).ok()
    }
    pub unsafe fn SetAssistedTelephonyPriority<P0, P1>(&self, pappfilename: P0, fpriority: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAssistedTelephonyPriority)(windows_core::Interface::as_raw(self), pappfilename.param().abi(), fpriority.param().abi()).ok()
    }
    pub unsafe fn SetApplicationPriority<P0, P1>(&self, pappfilename: P0, lmediatype: i32, fpriority: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetApplicationPriority)(windows_core::Interface::as_raw(self), pappfilename.param().abi(), lmediatype, fpriority.param().abi()).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfiltermask: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventFilter)(windows_core::Interface::as_raw(self), lfiltermask).ok()
    }
    pub unsafe fn EventFilter(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventFilter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPI_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Addresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterCallNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, i32, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterCallNotifications: usize,
    pub UnregisterNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CallHubs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateCallHubs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCallHubTracking: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumeratePrivateTAPIObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumeratePrivateTAPIObjects: usize,
    pub PrivateTAPIObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RegisterRequestRecipient: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAssistedTelephonyPriority: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetApplicationPriority: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EventFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTAPI2, ITTAPI2_Vtbl, 0x54fbdc8c_d90f_4dad_9695_b373097f094b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTAPI2 {
    type Target = ITTAPI;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTAPI2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITTAPI);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI2 {
    pub unsafe fn Phones(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Phones)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePhones(&self) -> windows_core::Result<IEnumPhone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePhones)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEmptyCollectionObject(&self) -> windows_core::Result<ITCollection2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEmptyCollectionObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPI2_Vtbl {
    pub base__: ITTAPI_Vtbl,
    pub Phones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumeratePhones: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEmptyCollectionObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEmptyCollectionObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTAPICallCenter, ITTAPICallCenter_Vtbl, 0x5afc3154_4bcc_11d1_bf80_00805fc147d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTAPICallCenter {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTAPICallCenter, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPICallCenter {
    pub unsafe fn EnumerateAgentHandlers(&self) -> windows_core::Result<IEnumAgentHandler> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateAgentHandlers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AgentHandlers(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AgentHandlers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPICallCenter_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EnumerateAgentHandlers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AgentHandlers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTAPIDispatchEventNotification, ITTAPIDispatchEventNotification_Vtbl, 0x9f34325b_7e62_11d2_9457_00c04f8ec888);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTAPIDispatchEventNotification {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTAPIDispatchEventNotification, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPIDispatchEventNotification_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(ITTAPIEventNotification, ITTAPIEventNotification_Vtbl, 0xeddb9426_3b91_11d1_8f30_00c04fb6809f);
impl core::ops::Deref for ITTAPIEventNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITTAPIEventNotification, windows_core::IUnknown);
impl ITTAPIEventNotification {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Event<P0>(&self, tapievent: TAPI_EVENT, pevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), tapievent, pevent.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITTAPIEventNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, TAPI_EVENT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Event: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTAPIObjectEvent, ITTAPIObjectEvent_Vtbl, 0xf4854d48_937a_11d1_bb58_00c04fb6809f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTAPIObjectEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTAPIObjectEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TAPIObject(&self) -> windows_core::Result<ITTAPI> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TAPIObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Event(&self) -> windows_core::Result<TAPIOBJECT_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> windows_core::Result<ITAddress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPIObjectEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub TAPIObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TAPIObject: usize,
    pub Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TAPIOBJECT_EVENT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Address: usize,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTAPIObjectEvent2, ITTAPIObjectEvent2_Vtbl, 0x359dda6e_68ce_4383_bf0b_169133c41b46);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTAPIObjectEvent2 {
    type Target = ITTAPIObjectEvent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTAPIObjectEvent2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITTAPIObjectEvent);
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Phone(&self) -> windows_core::Result<ITPhone> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Phone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTAPIObjectEvent2_Vtbl {
    pub base__: ITTAPIObjectEvent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Phone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Phone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTTSTerminalEvent, ITTTSTerminalEvent_Vtbl, 0xd964788f_95a5_461d_ab0c_b9900a6c2713);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTTSTerminalEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTTSTerminalEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITTTSTerminalEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTTSTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTerminal, ITTerminal_Vtbl, 0xb1efc38a_9355_11d0_835c_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTerminal {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTerminal, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITTerminal {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<TERMINAL_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TerminalType(&self) -> windows_core::Result<TERMINAL_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TerminalClass(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalClass)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MediaType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Direction(&self) -> windows_core::Result<TERMINAL_DIRECTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTerminal_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_STATE) -> windows_core::HRESULT,
    pub TerminalType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_TYPE) -> windows_core::HRESULT,
    pub TerminalClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TERMINAL_DIRECTION) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTerminalSupport, ITTerminalSupport_Vtbl, 0xb1efc385_9355_11d0_835c_00aa003ccabd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTerminalSupport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTerminalSupport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport {
    pub unsafe fn StaticTerminals(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StaticTerminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateStaticTerminals(&self) -> windows_core::Result<IEnumTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateStaticTerminals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DynamicTerminalClasses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DynamicTerminalClasses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateDynamicTerminalClasses(&self) -> windows_core::Result<IEnumTerminalClass> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateDynamicTerminalClasses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTerminal<P0>(&self, pterminalclass: P0, lmediatype: i32, direction: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTerminal)(windows_core::Interface::as_raw(self), pterminalclass.param().abi(), lmediatype, direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDefaultStaticTerminal(&self, lmediatype: i32, direction: TERMINAL_DIRECTION) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultStaticTerminal)(windows_core::Interface::as_raw(self), lmediatype, direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTerminalSupport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StaticTerminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateStaticTerminals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DynamicTerminalClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumerateDynamicTerminalClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, TERMINAL_DIRECTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTerminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDefaultStaticTerminal: unsafe extern "system" fn(*mut core::ffi::c_void, i32, TERMINAL_DIRECTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDefaultStaticTerminal: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITTerminalSupport2, ITTerminalSupport2_Vtbl, 0xf3eb39bc_1b1f_4e99_a0c0_56305c4dd591);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITTerminalSupport2 {
    type Target = ITTerminalSupport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITTerminalSupport2, windows_core::IUnknown, super::super::System::Com::IDispatch, ITTerminalSupport);
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport2 {
    pub unsafe fn PluggableSuperclasses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PluggableSuperclasses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePluggableSuperclasses(&self) -> windows_core::Result<IEnumPluggableSuperclassInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePluggableSuperclasses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_PluggableTerminalClasses<P0>(&self, bstrterminalsuperclass: P0, lmediatype: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_PluggableTerminalClasses)(windows_core::Interface::as_raw(self), bstrterminalsuperclass.param().abi(), lmediatype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumeratePluggableTerminalClasses(&self, iidterminalsuperclass: windows_core::GUID, lmediatype: i32) -> windows_core::Result<IEnumPluggableTerminalClassInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumeratePluggableTerminalClasses)(windows_core::Interface::as_raw(self), core::mem::transmute(iidterminalsuperclass), lmediatype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITTerminalSupport2_Vtbl {
    pub base__: ITTerminalSupport_Vtbl,
    pub PluggableSuperclasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumeratePluggableSuperclasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PluggableTerminalClasses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub EnumeratePluggableTerminalClasses: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITToneDetectionEvent, ITToneDetectionEvent_Vtbl, 0x407e0faf_d047_4753_b0c6_8e060373fecd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITToneDetectionEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITToneDetectionEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITToneDetectionEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AppSpecific(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppSpecific)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TickCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TickCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CallbackInstance(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CallbackInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITToneDetectionEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub AppSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TickCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CallbackInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITToneTerminalEvent, ITToneTerminalEvent_Vtbl, 0xe6f56009_611f_4945_bbd2_2d0ce5612056);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITToneTerminalEvent {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITToneTerminalEvent, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITToneTerminalEvent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Terminal(&self) -> windows_core::Result<ITTerminal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Terminal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Call(&self) -> windows_core::Result<ITCallInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITToneTerminalEvent_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Terminal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Terminal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Call: usize,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITnef, ITnef_Vtbl, 0);
impl core::ops::Deref for ITnef {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITnef, windows_core::IUnknown);
impl ITnef {
    #[cfg(feature = "Win32_System_AddressBook")]
    pub unsafe fn AddProps(&self, ulflags: u32, ulelemid: u32, lpvdata: *mut core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddProps)(windows_core::Interface::as_raw(self), ulflags, ulelemid, lpvdata, lpproplist).ok()
    }
    #[cfg(feature = "Win32_System_AddressBook")]
    pub unsafe fn ExtractProps(&self, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExtractProps)(windows_core::Interface::as_raw(self), ulflags, lpproplist, lpproblems).ok()
    }
    pub unsafe fn Finish(&self, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish)(windows_core::Interface::as_raw(self), ulflags, lpkey, lpproblems).ok()
    }
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub unsafe fn OpenTaggedBody<P0>(&self, lpmessage: P0, ulflags: u32) -> windows_core::Result<super::super::System::Com::IStream>
    where
        P0: windows_core::Param<super::super::System::AddressBook::IMessage>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenTaggedBody)(windows_core::Interface::as_raw(self), lpmessage.param().abi(), ulflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub unsafe fn SetProps(&self, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProps)(windows_core::Interface::as_raw(self), ulflags, ulelemid, cvalues, lpprops).ok()
    }
    #[cfg(feature = "Win32_System_AddressBook")]
    pub unsafe fn EncodeRecips<P0>(&self, ulflags: u32, lprecipienttable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::AddressBook::IMAPITable>,
    {
        (windows_core::Interface::vtable(self).EncodeRecips)(windows_core::Interface::as_raw(self), ulflags, lprecipienttable.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub unsafe fn FinishComponent(&self, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinishComponent)(windows_core::Interface::as_raw(self), ulflags, ulcomponentid, lpcustomproplist, lpcustomprops, lpproplist, lpproblems).ok()
    }
}
#[repr(C)]
pub struct ITnef_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub AddProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut super::super::System::AddressBook::SPropTagArray) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    AddProps: usize,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub ExtractProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::System::AddressBook::SPropTagArray, *mut *mut STnefProblemArray) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    ExtractProps: usize,
    pub Finish: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, *mut *mut STnefProblemArray) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub OpenTaggedBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    OpenTaggedBody: usize,
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub SetProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut super::super::System::AddressBook::SPropValue) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    SetProps: usize,
    #[cfg(feature = "Win32_System_AddressBook")]
    pub EncodeRecips: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_AddressBook"))]
    EncodeRecips: usize,
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub FinishComponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::super::System::AddressBook::SPropTagArray, *mut super::super::System::AddressBook::SPropValue, *mut super::super::System::AddressBook::SPropTagArray, *mut *mut STnefProblemArray) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com")))]
    FinishComponent: usize,
}
pub const ACDGE_GROUP_REMOVED: ACDGROUP_EVENT = ACDGROUP_EVENT(1i32);
pub const ACDGE_NEW_GROUP: ACDGROUP_EVENT = ACDGROUP_EVENT(0i32);
pub const ACDQE_NEW_QUEUE: ACDQUEUE_EVENT = ACDQUEUE_EVENT(0i32);
pub const ACDQE_QUEUE_REMOVED: ACDQUEUE_EVENT = ACDQUEUE_EVENT(1i32);
pub const ACS_ADDRESSDEVICESPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(1i32);
pub const ACS_LINEDEVICESPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(2i32);
pub const ACS_PERMANENTDEVICEGUID: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(5i32);
pub const ACS_PROTOCOL: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(0i32);
pub const ACS_PROVIDERSPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(3i32);
pub const ACS_SWITCHSPECIFIC: ADDRESS_CAPABILITY_STRING = ADDRESS_CAPABILITY_STRING(4i32);
pub const AC_ADDRESSCAPFLAGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(23i32);
pub const AC_ADDRESSFEATURES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(29i32);
pub const AC_ADDRESSID: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(33i32);
pub const AC_ADDRESSTYPES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(0i32);
pub const AC_ANSWERMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(14i32);
pub const AC_BEARERMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(1i32);
pub const AC_CALLCOMPLETIONCONDITIONS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(40i32);
pub const AC_CALLCOMPLETIONMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(41i32);
pub const AC_CALLEDIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(19i32);
pub const AC_CALLERIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(18i32);
pub const AC_CALLFEATURES1: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(24i32);
pub const AC_CALLFEATURES2: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(25i32);
pub const AC_CONNECTEDIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(20i32);
pub const AC_DEVCAPFLAGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(13i32);
pub const AC_FORWARDMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(34i32);
pub const AC_GATHERDIGITSMAXTIMEOUT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(44i32);
pub const AC_GATHERDIGITSMINTIMEOUT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(43i32);
pub const AC_GENERATEDIGITDEFAULTDURATION: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(47i32);
pub const AC_GENERATEDIGITMAXDURATION: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(46i32);
pub const AC_GENERATEDIGITMINDURATION: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(45i32);
pub const AC_GENERATEDIGITSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(8i32);
pub const AC_GENERATETONEMAXNUMFREQ: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(10i32);
pub const AC_GENERATETONEMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(9i32);
pub const AC_LINEFEATURES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(15i32);
pub const AC_LINEID: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(32i32);
pub const AC_MAXACTIVECALLS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(2i32);
pub const AC_MAXCALLCOMPLETIONS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(39i32);
pub const AC_MAXCALLDATASIZE: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(31i32);
pub const AC_MAXFORWARDENTRIES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(35i32);
pub const AC_MAXFWDNUMRINGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(38i32);
pub const AC_MAXNUMCONFERENCE: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(5i32);
pub const AC_MAXNUMTRANSCONF: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(6i32);
pub const AC_MAXONHOLDCALLS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(3i32);
pub const AC_MAXONHOLDPENDINGCALLS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(4i32);
pub const AC_MAXSPECIFICENTRIES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(36i32);
pub const AC_MINFWDNUMRINGS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(37i32);
pub const AC_MONITORDIGITSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(7i32);
pub const AC_MONITORTONEMAXNUMENTRIES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(12i32);
pub const AC_MONITORTONEMAXNUMFREQ: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(11i32);
pub const AC_PARKSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(17i32);
pub const AC_PERMANENTDEVICEID: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(42i32);
pub const AC_PREDICTIVEAUTOTRANSFERSTATES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(30i32);
pub const AC_REDIRECTINGIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(22i32);
pub const AC_REDIRECTIONIDSUPPORT: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(21i32);
pub const AC_REMOVEFROMCONFCAPS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(26i32);
pub const AC_REMOVEFROMCONFSTATE: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(27i32);
pub const AC_SETTABLEDEVSTATUS: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(16i32);
pub const AC_TRANSFERMODES: ADDRESS_CAPABILITY = ADDRESS_CAPABILITY(28i32);
pub const ADDRESS_TERMINAL_AVAILABLE: MSP_ADDRESS_EVENT = MSP_ADDRESS_EVENT(0i32);
pub const ADDRESS_TERMINAL_UNAVAILABLE: MSP_ADDRESS_EVENT = MSP_ADDRESS_EVENT(1i32);
pub const AE_BUSY_ACD: AGENT_EVENT = AGENT_EVENT(2i32);
pub const AE_BUSY_INCOMING: AGENT_EVENT = AGENT_EVENT(3i32);
pub const AE_BUSY_OUTGOING: AGENT_EVENT = AGENT_EVENT(4i32);
pub const AE_CAPSCHANGE: ADDRESS_EVENT = ADDRESS_EVENT(1i32);
pub const AE_CONFIGCHANGE: ADDRESS_EVENT = ADDRESS_EVENT(3i32);
pub const AE_FORWARD: ADDRESS_EVENT = ADDRESS_EVENT(4i32);
pub const AE_LASTITEM: ADDRESS_EVENT = ADDRESS_EVENT(8i32);
pub const AE_MSGWAITOFF: ADDRESS_EVENT = ADDRESS_EVENT(8i32);
pub const AE_MSGWAITON: ADDRESS_EVENT = ADDRESS_EVENT(7i32);
pub const AE_NEWTERMINAL: ADDRESS_EVENT = ADDRESS_EVENT(5i32);
pub const AE_NOT_READY: AGENT_EVENT = AGENT_EVENT(0i32);
pub const AE_READY: AGENT_EVENT = AGENT_EVENT(1i32);
pub const AE_REMOVETERMINAL: ADDRESS_EVENT = ADDRESS_EVENT(6i32);
pub const AE_RINGING: ADDRESS_EVENT = ADDRESS_EVENT(2i32);
pub const AE_STATE: ADDRESS_EVENT = ADDRESS_EVENT(0i32);
pub const AE_UNKNOWN: AGENT_EVENT = AGENT_EVENT(5i32);
pub const AHE_AGENTHANDLER_REMOVED: AGENTHANDLER_EVENT = AGENTHANDLER_EVENT(1i32);
pub const AHE_NEW_AGENTHANDLER: AGENTHANDLER_EVENT = AGENTHANDLER_EVENT(0i32);
pub const ASE_BUSY: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(3i32);
pub const ASE_END: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(5i32);
pub const ASE_NEW_SESSION: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(0i32);
pub const ASE_NOT_READY: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(1i32);
pub const ASE_READY: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(2i32);
pub const ASE_WRAPUP: AGENT_SESSION_EVENT = AGENT_SESSION_EVENT(4i32);
pub const ASST_BUSY_ON_CALL: AGENT_SESSION_STATE = AGENT_SESSION_STATE(2i32);
pub const ASST_BUSY_WRAPUP: AGENT_SESSION_STATE = AGENT_SESSION_STATE(3i32);
pub const ASST_NOT_READY: AGENT_SESSION_STATE = AGENT_SESSION_STATE(0i32);
pub const ASST_READY: AGENT_SESSION_STATE = AGENT_SESSION_STATE(1i32);
pub const ASST_SESSION_ENDED: AGENT_SESSION_STATE = AGENT_SESSION_STATE(4i32);
pub const AS_BUSY_ACD: AGENT_STATE = AGENT_STATE(2i32);
pub const AS_BUSY_INCOMING: AGENT_STATE = AGENT_STATE(3i32);
pub const AS_BUSY_OUTGOING: AGENT_STATE = AGENT_STATE(4i32);
pub const AS_INSERVICE: ADDRESS_STATE = ADDRESS_STATE(0i32);
pub const AS_NOT_READY: AGENT_STATE = AGENT_STATE(0i32);
pub const AS_OUTOFSERVICE: ADDRESS_STATE = ADDRESS_STATE(1i32);
pub const AS_READY: AGENT_STATE = AGENT_STATE(1i32);
pub const AS_UNKNOWN: AGENT_STATE = AGENT_STATE(5i32);
pub const CALL_CAUSE_BAD_DEVICE: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(1i32);
pub const CALL_CAUSE_CONNECT_FAIL: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(2i32);
pub const CALL_CAUSE_LOCAL_REQUEST: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(3i32);
pub const CALL_CAUSE_MEDIA_RECOVERED: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(6i32);
pub const CALL_CAUSE_MEDIA_TIMEOUT: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(5i32);
pub const CALL_CAUSE_QUALITY_OF_SERVICE: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(7i32);
pub const CALL_CAUSE_REMOTE_REQUEST: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(4i32);
pub const CALL_CAUSE_UNKNOWN: MSP_CALL_EVENT_CAUSE = MSP_CALL_EVENT_CAUSE(0i32);
pub const CALL_NEW_STREAM: MSP_CALL_EVENT = MSP_CALL_EVENT(0i32);
pub const CALL_STREAM_ACTIVE: MSP_CALL_EVENT = MSP_CALL_EVENT(4i32);
pub const CALL_STREAM_FAIL: MSP_CALL_EVENT = MSP_CALL_EVENT(1i32);
pub const CALL_STREAM_INACTIVE: MSP_CALL_EVENT = MSP_CALL_EVENT(5i32);
pub const CALL_STREAM_NOT_USED: MSP_CALL_EVENT = MSP_CALL_EVENT(3i32);
pub const CALL_TERMINAL_FAIL: MSP_CALL_EVENT = MSP_CALL_EVENT(2i32);
pub const CEC_DISCONNECT_BADADDRESS: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(3i32);
pub const CEC_DISCONNECT_BLOCKED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(8i32);
pub const CEC_DISCONNECT_BUSY: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(2i32);
pub const CEC_DISCONNECT_CANCELLED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(5i32);
pub const CEC_DISCONNECT_FAILED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(7i32);
pub const CEC_DISCONNECT_NOANSWER: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(4i32);
pub const CEC_DISCONNECT_NORMAL: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(1i32);
pub const CEC_DISCONNECT_REJECTED: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(6i32);
pub const CEC_NONE: CALL_STATE_EVENT_CAUSE = CALL_STATE_EVENT_CAUSE(0i32);
pub const CHE_CALLHUBIDLE: CALLHUB_EVENT = CALLHUB_EVENT(3i32);
pub const CHE_CALLHUBNEW: CALLHUB_EVENT = CALLHUB_EVENT(2i32);
pub const CHE_CALLJOIN: CALLHUB_EVENT = CALLHUB_EVENT(0i32);
pub const CHE_CALLLEAVE: CALLHUB_EVENT = CALLHUB_EVENT(1i32);
pub const CHE_LASTITEM: CALLHUB_EVENT = CALLHUB_EVENT(3i32);
pub const CHS_ACTIVE: CALLHUB_STATE = CALLHUB_STATE(0i32);
pub const CHS_IDLE: CALLHUB_STATE = CALLHUB_STATE(1i32);
pub const CIB_CALLDATABUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(2i32);
pub const CIB_CHARGINGINFOBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(3i32);
pub const CIB_DEVSPECIFICBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(1i32);
pub const CIB_HIGHLEVELCOMPATIBILITYBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(4i32);
pub const CIB_LOWLEVELCOMPATIBILITYBUFFER: CALLINFO_BUFFER = CALLINFO_BUFFER(5i32);
pub const CIB_USERUSERINFO: CALLINFO_BUFFER = CALLINFO_BUFFER(0i32);
pub const CIC_APPSPECIFIC: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(4i32);
pub const CIC_BEARERMODE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(2i32);
pub const CIC_CALLDATA: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(24i32);
pub const CIC_CALLEDID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(15i32);
pub const CIC_CALLERID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(14i32);
pub const CIC_CALLID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(5i32);
pub const CIC_CHARGINGINFO: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(22i32);
pub const CIC_COMPLETIONID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(9i32);
pub const CIC_CONNECTEDID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(16i32);
pub const CIC_DEVSPECIFIC: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(1i32);
pub const CIC_HIGHLEVELCOMP: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(20i32);
pub const CIC_LASTITEM: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(26i32);
pub const CIC_LOWLEVELCOMP: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(21i32);
pub const CIC_MEDIATYPE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(26i32);
pub const CIC_NUMMONITORS: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(12i32);
pub const CIC_NUMOWNERDECR: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(11i32);
pub const CIC_NUMOWNERINCR: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(10i32);
pub const CIC_ORIGIN: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(7i32);
pub const CIC_OTHER: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(0i32);
pub const CIC_PRIVILEGE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(25i32);
pub const CIC_RATE: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(3i32);
pub const CIC_REASON: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(8i32);
pub const CIC_REDIRECTINGID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(18i32);
pub const CIC_REDIRECTIONID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(17i32);
pub const CIC_RELATEDCALLID: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(6i32);
pub const CIC_TREATMENT: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(23i32);
pub const CIC_TRUNK: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(13i32);
pub const CIC_USERUSERINFO: CALLINFOCHANGE_CAUSE = CALLINFOCHANGE_CAUSE(19i32);
pub const CIL_APPSPECIFIC: CALLINFO_LONG = CALLINFO_LONG(9i32);
pub const CIL_BEARERMODE: CALLINFO_LONG = CALLINFO_LONG(1i32);
pub const CIL_CALLEDIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(3i32);
pub const CIL_CALLERIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(2i32);
pub const CIL_CALLID: CALLINFO_LONG = CALLINFO_LONG(15i32);
pub const CIL_CALLPARAMSFLAGS: CALLINFO_LONG = CALLINFO_LONG(10i32);
pub const CIL_CALLTREATMENT: CALLINFO_LONG = CALLINFO_LONG(11i32);
pub const CIL_COMPLETIONID: CALLINFO_LONG = CALLINFO_LONG(17i32);
pub const CIL_CONNECTEDIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(4i32);
pub const CIL_COUNTRYCODE: CALLINFO_LONG = CALLINFO_LONG(14i32);
pub const CIL_GENERATEDIGITDURATION: CALLINFO_LONG = CALLINFO_LONG(22i32);
pub const CIL_MAXRATE: CALLINFO_LONG = CALLINFO_LONG(13i32);
pub const CIL_MEDIATYPESAVAILABLE: CALLINFO_LONG = CALLINFO_LONG(0i32);
pub const CIL_MINRATE: CALLINFO_LONG = CALLINFO_LONG(12i32);
pub const CIL_MONITORDIGITMODES: CALLINFO_LONG = CALLINFO_LONG(23i32);
pub const CIL_MONITORMEDIAMODES: CALLINFO_LONG = CALLINFO_LONG(24i32);
pub const CIL_NUMBEROFMONITORS: CALLINFO_LONG = CALLINFO_LONG(19i32);
pub const CIL_NUMBEROFOWNERS: CALLINFO_LONG = CALLINFO_LONG(18i32);
pub const CIL_ORIGIN: CALLINFO_LONG = CALLINFO_LONG(7i32);
pub const CIL_RATE: CALLINFO_LONG = CALLINFO_LONG(21i32);
pub const CIL_REASON: CALLINFO_LONG = CALLINFO_LONG(8i32);
pub const CIL_REDIRECTINGIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(6i32);
pub const CIL_REDIRECTIONIDADDRESSTYPE: CALLINFO_LONG = CALLINFO_LONG(5i32);
pub const CIL_RELATEDCALLID: CALLINFO_LONG = CALLINFO_LONG(16i32);
pub const CIL_TRUNK: CALLINFO_LONG = CALLINFO_LONG(20i32);
pub const CIS_CALLEDIDNAME: CALLINFO_STRING = CALLINFO_STRING(2i32);
pub const CIS_CALLEDIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(3i32);
pub const CIS_CALLEDPARTYFRIENDLYNAME: CALLINFO_STRING = CALLINFO_STRING(10i32);
pub const CIS_CALLERIDNAME: CALLINFO_STRING = CALLINFO_STRING(0i32);
pub const CIS_CALLERIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(1i32);
pub const CIS_CALLINGPARTYID: CALLINFO_STRING = CALLINFO_STRING(13i32);
pub const CIS_COMMENT: CALLINFO_STRING = CALLINFO_STRING(11i32);
pub const CIS_CONNECTEDIDNAME: CALLINFO_STRING = CALLINFO_STRING(4i32);
pub const CIS_CONNECTEDIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(5i32);
pub const CIS_DISPLAYABLEADDRESS: CALLINFO_STRING = CALLINFO_STRING(12i32);
pub const CIS_REDIRECTINGIDNAME: CALLINFO_STRING = CALLINFO_STRING(8i32);
pub const CIS_REDIRECTINGIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(9i32);
pub const CIS_REDIRECTIONIDNAME: CALLINFO_STRING = CALLINFO_STRING(6i32);
pub const CIS_REDIRECTIONIDNUMBER: CALLINFO_STRING = CALLINFO_STRING(7i32);
pub const CMC_BAD_DEVICE: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(1i32);
pub const CMC_CONNECT_FAIL: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(2i32);
pub const CMC_LOCAL_REQUEST: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(3i32);
pub const CMC_MEDIA_RECOVERED: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(6i32);
pub const CMC_MEDIA_TIMEOUT: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(5i32);
pub const CMC_QUALITY_OF_SERVICE: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(7i32);
pub const CMC_REMOTE_REQUEST: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(4i32);
pub const CMC_UNKNOWN: CALL_MEDIA_EVENT_CAUSE = CALL_MEDIA_EVENT_CAUSE(0i32);
pub const CME_LASTITEM: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(5i32);
pub const CME_NEW_STREAM: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(0i32);
pub const CME_STREAM_ACTIVE: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(4i32);
pub const CME_STREAM_FAIL: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(1i32);
pub const CME_STREAM_INACTIVE: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(5i32);
pub const CME_STREAM_NOT_USED: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(3i32);
pub const CME_TERMINAL_FAIL: CALL_MEDIA_EVENT = CALL_MEDIA_EVENT(2i32);
pub const CNE_LASTITEM: CALL_NOTIFICATION_EVENT = CALL_NOTIFICATION_EVENT(1i32);
pub const CNE_MONITOR: CALL_NOTIFICATION_EVENT = CALL_NOTIFICATION_EVENT(1i32);
pub const CNE_OWNER: CALL_NOTIFICATION_EVENT = CALL_NOTIFICATION_EVENT(0i32);
pub const CP_MONITOR: CALL_PRIVILEGE = CALL_PRIVILEGE(1i32);
pub const CP_OWNER: CALL_PRIVILEGE = CALL_PRIVILEGE(0i32);
pub const CS_CONNECTED: CALL_STATE = CALL_STATE(2i32);
pub const CS_DISCONNECTED: CALL_STATE = CALL_STATE(3i32);
pub const CS_HOLD: CALL_STATE = CALL_STATE(5i32);
pub const CS_IDLE: CALL_STATE = CALL_STATE(0i32);
pub const CS_INPROGRESS: CALL_STATE = CALL_STATE(1i32);
pub const CS_LASTITEM: CALL_STATE = CALL_STATE(6i32);
pub const CS_OFFERING: CALL_STATE = CALL_STATE(4i32);
pub const CS_QUEUED: CALL_STATE = CALL_STATE(6i32);
pub const DC_NOANSWER: DISCONNECT_CODE = DISCONNECT_CODE(1i32);
pub const DC_NORMAL: DISCONNECT_CODE = DISCONNECT_CODE(0i32);
pub const DC_REJECTED: DISCONNECT_CODE = DISCONNECT_CODE(2i32);
pub const DISPIDMASK: u32 = 65535u32;
pub const DT_ILS: DIRECTORY_TYPE = DIRECTORY_TYPE(2i32);
pub const DT_NTDS: DIRECTORY_TYPE = DIRECTORY_TYPE(1i32);
pub const FDS_NOTSUPPORTED: FULLDUPLEX_SUPPORT = FULLDUPLEX_SUPPORT(1i32);
pub const FDS_SUPPORTED: FULLDUPLEX_SUPPORT = FULLDUPLEX_SUPPORT(0i32);
pub const FDS_UNKNOWN: FULLDUPLEX_SUPPORT = FULLDUPLEX_SUPPORT(2i32);
pub const FM_ASCONFERENCE: FINISH_MODE = FINISH_MODE(1i32);
pub const FM_ASTRANSFER: FINISH_MODE = FINISH_MODE(0i32);
pub const FTEC_END_OF_FILE: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(1i32);
pub const FTEC_NORMAL: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(0i32);
pub const FTEC_READ_ERROR: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(2i32);
pub const FTEC_WRITE_ERROR: FT_STATE_EVENT_CAUSE = FT_STATE_EVENT_CAUSE(3i32);
pub const GETTNEFSTREAMCODEPAGE: windows_core::PCSTR = windows_core::s!("GetTnefStreamCodePage");
pub const IDISPADDRESS: u32 = 65536u32;
pub const IDISPADDRESSCAPABILITIES: u32 = 131072u32;
pub const IDISPADDRESSTRANSLATION: u32 = 262144u32;
pub const IDISPAGGREGATEDMSPADDRESSOBJ: u32 = 393216u32;
pub const IDISPAGGREGATEDMSPCALLOBJ: u32 = 262144u32;
pub const IDISPAPC: u32 = 131072u32;
pub const IDISPBASICCALLCONTROL: u32 = 131072u32;
pub const IDISPCALLINFO: u32 = 65536u32;
pub const IDISPDIRECTORY: u32 = 65536u32;
pub const IDISPDIROBJCONFERENCE: u32 = 131072u32;
pub const IDISPDIROBJECT: u32 = 65536u32;
pub const IDISPDIROBJUSER: u32 = 196608u32;
pub const IDISPFILETRACK: u32 = 65536u32;
pub const IDISPILSCONFIG: u32 = 131072u32;
pub const IDISPLEGACYADDRESSMEDIACONTROL: u32 = 327680u32;
pub const IDISPLEGACYCALLMEDIACONTROL: u32 = 196608u32;
pub const IDISPMEDIACONTROL: u32 = 131072u32;
pub const IDISPMEDIAPLAYBACK: u32 = 262144u32;
pub const IDISPMEDIARECORD: u32 = 196608u32;
pub const IDISPMEDIASUPPORT: u32 = 196608u32;
pub const IDISPMULTITRACK: u32 = 65536u32;
pub const IDISPPHONE: u32 = 65536u32;
pub const IDISPTAPI: u32 = 65536u32;
pub const IDISPTAPICALLCENTER: u32 = 131072u32;
pub const INITIALIZE_NEGOTIATION: u32 = 4294967295u32;
pub const INTERFACEMASK: u32 = 16711680u32;
pub const LAST_LINEMEDIAMODE: u32 = 32768u32;
pub const LAST_LINEREQUESTMODE: u32 = 2u32;
pub const LINEADDRCAPFLAGS_ACCEPTTOALERT: u32 = 1048576u32;
pub const LINEADDRCAPFLAGS_ACDGROUP: u32 = 1073741824u32;
pub const LINEADDRCAPFLAGS_AUTORECONNECT: u32 = 1024u32;
pub const LINEADDRCAPFLAGS_BLOCKIDDEFAULT: u32 = 8u32;
pub const LINEADDRCAPFLAGS_BLOCKIDOVERRIDE: u32 = 16u32;
pub const LINEADDRCAPFLAGS_COMPLETIONID: u32 = 2048u32;
pub const LINEADDRCAPFLAGS_CONFDROP: u32 = 2097152u32;
pub const LINEADDRCAPFLAGS_CONFERENCEHELD: u32 = 16384u32;
pub const LINEADDRCAPFLAGS_CONFERENCEMAKE: u32 = 32768u32;
pub const LINEADDRCAPFLAGS_DESTOFFHOOK: u32 = 128u32;
pub const LINEADDRCAPFLAGS_DIALED: u32 = 32u32;
pub const LINEADDRCAPFLAGS_FWDBUSYNAADDR: u32 = 524288u32;
pub const LINEADDRCAPFLAGS_FWDCONSULT: u32 = 256u32;
pub const LINEADDRCAPFLAGS_FWDINTEXTADDR: u32 = 262144u32;
pub const LINEADDRCAPFLAGS_FWDNUMRINGS: u32 = 1u32;
pub const LINEADDRCAPFLAGS_FWDSTATUSVALID: u32 = 131072u32;
pub const LINEADDRCAPFLAGS_HOLDMAKESNEW: u32 = 67108864u32;
pub const LINEADDRCAPFLAGS_NOEXTERNALCALLS: u32 = 268435456u32;
pub const LINEADDRCAPFLAGS_NOINTERNALCALLS: u32 = 134217728u32;
pub const LINEADDRCAPFLAGS_NOPSTNADDRESSTRANSLATION: u32 = 2147483648u32;
pub const LINEADDRCAPFLAGS_ORIGOFFHOOK: u32 = 64u32;
pub const LINEADDRCAPFLAGS_PARTIALDIAL: u32 = 65536u32;
pub const LINEADDRCAPFLAGS_PICKUPCALLWAIT: u32 = 4194304u32;
pub const LINEADDRCAPFLAGS_PICKUPGROUPID: u32 = 2u32;
pub const LINEADDRCAPFLAGS_PREDICTIVEDIALER: u32 = 8388608u32;
pub const LINEADDRCAPFLAGS_QUEUE: u32 = 16777216u32;
pub const LINEADDRCAPFLAGS_ROUTEPOINT: u32 = 33554432u32;
pub const LINEADDRCAPFLAGS_SECURE: u32 = 4u32;
pub const LINEADDRCAPFLAGS_SETCALLINGID: u32 = 536870912u32;
pub const LINEADDRCAPFLAGS_SETUPCONFNULL: u32 = 512u32;
pub const LINEADDRCAPFLAGS_TRANSFERHELD: u32 = 4096u32;
pub const LINEADDRCAPFLAGS_TRANSFERMAKE: u32 = 8192u32;
pub const LINEADDRESSMODE_ADDRESSID: u32 = 1u32;
pub const LINEADDRESSMODE_DIALABLEADDR: u32 = 2u32;
pub const LINEADDRESSSHARING_BRIDGEDEXCL: u32 = 2u32;
pub const LINEADDRESSSHARING_BRIDGEDNEW: u32 = 4u32;
pub const LINEADDRESSSHARING_BRIDGEDSHARED: u32 = 8u32;
pub const LINEADDRESSSHARING_MONITORED: u32 = 16u32;
pub const LINEADDRESSSHARING_PRIVATE: u32 = 1u32;
pub const LINEADDRESSSTATE_CAPSCHANGE: u32 = 256u32;
pub const LINEADDRESSSTATE_DEVSPECIFIC: u32 = 2u32;
pub const LINEADDRESSSTATE_FORWARD: u32 = 64u32;
pub const LINEADDRESSSTATE_INUSEMANY: u32 = 16u32;
pub const LINEADDRESSSTATE_INUSEONE: u32 = 8u32;
pub const LINEADDRESSSTATE_INUSEZERO: u32 = 4u32;
pub const LINEADDRESSSTATE_NUMCALLS: u32 = 32u32;
pub const LINEADDRESSSTATE_OTHER: u32 = 1u32;
pub const LINEADDRESSSTATE_TERMINALS: u32 = 128u32;
pub const LINEADDRESSTYPE_DOMAINNAME: u32 = 8u32;
pub const LINEADDRESSTYPE_EMAILNAME: u32 = 4u32;
pub const LINEADDRESSTYPE_IPADDRESS: u32 = 16u32;
pub const LINEADDRESSTYPE_PHONENUMBER: u32 = 1u32;
pub const LINEADDRESSTYPE_SDP: u32 = 2u32;
pub const LINEADDRFEATURE_FORWARD: u32 = 1u32;
pub const LINEADDRFEATURE_FORWARDDND: u32 = 8192u32;
pub const LINEADDRFEATURE_FORWARDFWD: u32 = 4096u32;
pub const LINEADDRFEATURE_MAKECALL: u32 = 2u32;
pub const LINEADDRFEATURE_PICKUP: u32 = 4u32;
pub const LINEADDRFEATURE_PICKUPDIRECT: u32 = 1024u32;
pub const LINEADDRFEATURE_PICKUPGROUP: u32 = 512u32;
pub const LINEADDRFEATURE_PICKUPHELD: u32 = 256u32;
pub const LINEADDRFEATURE_PICKUPWAITING: u32 = 2048u32;
pub const LINEADDRFEATURE_SETMEDIACONTROL: u32 = 8u32;
pub const LINEADDRFEATURE_SETTERMINAL: u32 = 16u32;
pub const LINEADDRFEATURE_SETUPCONF: u32 = 32u32;
pub const LINEADDRFEATURE_UNCOMPLETECALL: u32 = 64u32;
pub const LINEADDRFEATURE_UNPARK: u32 = 128u32;
pub const LINEAGENTFEATURE_AGENTSPECIFIC: u32 = 8u32;
pub const LINEAGENTFEATURE_GETAGENTACTIVITYLIST: u32 = 16u32;
pub const LINEAGENTFEATURE_GETAGENTGROUP: u32 = 32u32;
pub const LINEAGENTFEATURE_SETAGENTACTIVITY: u32 = 4u32;
pub const LINEAGENTFEATURE_SETAGENTGROUP: u32 = 1u32;
pub const LINEAGENTFEATURE_SETAGENTSTATE: u32 = 2u32;
pub const LINEAGENTSESSIONSTATE_BUSYONCALL: u32 = 4u32;
pub const LINEAGENTSESSIONSTATE_BUSYWRAPUP: u32 = 8u32;
pub const LINEAGENTSESSIONSTATE_ENDED: u32 = 16u32;
pub const LINEAGENTSESSIONSTATE_NOTREADY: u32 = 1u32;
pub const LINEAGENTSESSIONSTATE_READY: u32 = 2u32;
pub const LINEAGENTSESSIONSTATE_RELEASED: u32 = 32u32;
pub const LINEAGENTSESSIONSTATUS_NEWSESSION: u32 = 1u32;
pub const LINEAGENTSESSIONSTATUS_STATE: u32 = 2u32;
pub const LINEAGENTSESSIONSTATUS_UPDATEINFO: u32 = 4u32;
pub const LINEAGENTSTATEEX_BUSYACD: u32 = 4u32;
pub const LINEAGENTSTATEEX_BUSYINCOMING: u32 = 8u32;
pub const LINEAGENTSTATEEX_BUSYOUTGOING: u32 = 16u32;
pub const LINEAGENTSTATEEX_NOTREADY: u32 = 1u32;
pub const LINEAGENTSTATEEX_READY: u32 = 2u32;
pub const LINEAGENTSTATEEX_RELEASED: u32 = 64u32;
pub const LINEAGENTSTATEEX_UNKNOWN: u32 = 32u32;
pub const LINEAGENTSTATE_BUSYACD: u32 = 8u32;
pub const LINEAGENTSTATE_BUSYINCOMING: u32 = 16u32;
pub const LINEAGENTSTATE_BUSYOTHER: u32 = 64u32;
pub const LINEAGENTSTATE_BUSYOUTBOUND: u32 = 32u32;
pub const LINEAGENTSTATE_LOGGEDOFF: u32 = 1u32;
pub const LINEAGENTSTATE_NOTREADY: u32 = 2u32;
pub const LINEAGENTSTATE_READY: u32 = 4u32;
pub const LINEAGENTSTATE_UNAVAIL: u32 = 512u32;
pub const LINEAGENTSTATE_UNKNOWN: u32 = 256u32;
pub const LINEAGENTSTATE_WORKINGAFTERCALL: u32 = 128u32;
pub const LINEAGENTSTATUSEX_NEWAGENT: u32 = 1u32;
pub const LINEAGENTSTATUSEX_STATE: u32 = 2u32;
pub const LINEAGENTSTATUSEX_UPDATEINFO: u32 = 4u32;
pub const LINEAGENTSTATUS_ACTIVITY: u32 = 8u32;
pub const LINEAGENTSTATUS_ACTIVITYLIST: u32 = 16u32;
pub const LINEAGENTSTATUS_CAPSCHANGE: u32 = 64u32;
pub const LINEAGENTSTATUS_GROUP: u32 = 1u32;
pub const LINEAGENTSTATUS_GROUPLIST: u32 = 32u32;
pub const LINEAGENTSTATUS_NEXTSTATE: u32 = 4u32;
pub const LINEAGENTSTATUS_STATE: u32 = 2u32;
pub const LINEAGENTSTATUS_VALIDNEXTSTATES: u32 = 256u32;
pub const LINEAGENTSTATUS_VALIDSTATES: u32 = 128u32;
pub const LINEANSWERMODE_DROP: u32 = 2u32;
pub const LINEANSWERMODE_HOLD: u32 = 4u32;
pub const LINEANSWERMODE_NONE: u32 = 1u32;
pub const LINEBEARERMODE_ALTSPEECHDATA: u32 = 16u32;
pub const LINEBEARERMODE_DATA: u32 = 8u32;
pub const LINEBEARERMODE_MULTIUSE: u32 = 4u32;
pub const LINEBEARERMODE_NONCALLSIGNALING: u32 = 32u32;
pub const LINEBEARERMODE_PASSTHROUGH: u32 = 64u32;
pub const LINEBEARERMODE_RESTRICTEDDATA: u32 = 128u32;
pub const LINEBEARERMODE_SPEECH: u32 = 2u32;
pub const LINEBEARERMODE_VOICE: u32 = 1u32;
pub const LINEBUSYMODE_STATION: u32 = 1u32;
pub const LINEBUSYMODE_TRUNK: u32 = 2u32;
pub const LINEBUSYMODE_UNAVAIL: u32 = 8u32;
pub const LINEBUSYMODE_UNKNOWN: u32 = 4u32;
pub const LINECALLCOMPLCOND_BUSY: u32 = 1u32;
pub const LINECALLCOMPLCOND_NOANSWER: u32 = 2u32;
pub const LINECALLCOMPLMODE_CALLBACK: u32 = 2u32;
pub const LINECALLCOMPLMODE_CAMPON: u32 = 1u32;
pub const LINECALLCOMPLMODE_INTRUDE: u32 = 4u32;
pub const LINECALLCOMPLMODE_MESSAGE: u32 = 8u32;
pub const LINECALLFEATURE2_COMPLCALLBACK: u32 = 8u32;
pub const LINECALLFEATURE2_COMPLCAMPON: u32 = 4u32;
pub const LINECALLFEATURE2_COMPLINTRUDE: u32 = 16u32;
pub const LINECALLFEATURE2_COMPLMESSAGE: u32 = 32u32;
pub const LINECALLFEATURE2_NOHOLDCONFERENCE: u32 = 1u32;
pub const LINECALLFEATURE2_ONESTEPTRANSFER: u32 = 2u32;
pub const LINECALLFEATURE2_PARKDIRECT: u32 = 256u32;
pub const LINECALLFEATURE2_PARKNONDIRECT: u32 = 512u32;
pub const LINECALLFEATURE2_TRANSFERCONF: u32 = 128u32;
pub const LINECALLFEATURE2_TRANSFERNORM: u32 = 64u32;
pub const LINECALLFEATURE_ACCEPT: u32 = 1u32;
pub const LINECALLFEATURE_ADDTOCONF: u32 = 2u32;
pub const LINECALLFEATURE_ANSWER: u32 = 4u32;
pub const LINECALLFEATURE_BLINDTRANSFER: u32 = 8u32;
pub const LINECALLFEATURE_COMPLETECALL: u32 = 16u32;
pub const LINECALLFEATURE_COMPLETETRANSF: u32 = 32u32;
pub const LINECALLFEATURE_DIAL: u32 = 64u32;
pub const LINECALLFEATURE_DROP: u32 = 128u32;
pub const LINECALLFEATURE_GATHERDIGITS: u32 = 256u32;
pub const LINECALLFEATURE_GENERATEDIGITS: u32 = 512u32;
pub const LINECALLFEATURE_GENERATETONE: u32 = 1024u32;
pub const LINECALLFEATURE_HOLD: u32 = 2048u32;
pub const LINECALLFEATURE_MONITORDIGITS: u32 = 4096u32;
pub const LINECALLFEATURE_MONITORMEDIA: u32 = 8192u32;
pub const LINECALLFEATURE_MONITORTONES: u32 = 16384u32;
pub const LINECALLFEATURE_PARK: u32 = 32768u32;
pub const LINECALLFEATURE_PREPAREADDCONF: u32 = 65536u32;
pub const LINECALLFEATURE_REDIRECT: u32 = 131072u32;
pub const LINECALLFEATURE_RELEASEUSERUSERINFO: u32 = 268435456u32;
pub const LINECALLFEATURE_REMOVEFROMCONF: u32 = 262144u32;
pub const LINECALLFEATURE_SECURECALL: u32 = 524288u32;
pub const LINECALLFEATURE_SENDUSERUSER: u32 = 1048576u32;
pub const LINECALLFEATURE_SETCALLDATA: u32 = 2147483648u32;
pub const LINECALLFEATURE_SETCALLPARAMS: u32 = 2097152u32;
pub const LINECALLFEATURE_SETMEDIACONTROL: u32 = 4194304u32;
pub const LINECALLFEATURE_SETQOS: u32 = 1073741824u32;
pub const LINECALLFEATURE_SETTERMINAL: u32 = 8388608u32;
pub const LINECALLFEATURE_SETTREATMENT: u32 = 536870912u32;
pub const LINECALLFEATURE_SETUPCONF: u32 = 16777216u32;
pub const LINECALLFEATURE_SETUPTRANSFER: u32 = 33554432u32;
pub const LINECALLFEATURE_SWAPHOLD: u32 = 67108864u32;
pub const LINECALLFEATURE_UNHOLD: u32 = 134217728u32;
pub const LINECALLHUBTRACKING_ALLCALLS: u32 = 2u32;
pub const LINECALLHUBTRACKING_NONE: u32 = 0u32;
pub const LINECALLHUBTRACKING_PROVIDERLEVEL: u32 = 1u32;
pub const LINECALLINFOSTATE_APPSPECIFIC: u32 = 32u32;
pub const LINECALLINFOSTATE_BEARERMODE: u32 = 4u32;
pub const LINECALLINFOSTATE_CALLDATA: u32 = 1073741824u32;
pub const LINECALLINFOSTATE_CALLEDID: u32 = 65536u32;
pub const LINECALLINFOSTATE_CALLERID: u32 = 32768u32;
pub const LINECALLINFOSTATE_CALLID: u32 = 64u32;
pub const LINECALLINFOSTATE_CHARGINGINFO: u32 = 16777216u32;
pub const LINECALLINFOSTATE_COMPLETIONID: u32 = 1024u32;
pub const LINECALLINFOSTATE_CONNECTEDID: u32 = 131072u32;
pub const LINECALLINFOSTATE_DEVSPECIFIC: u32 = 2u32;
pub const LINECALLINFOSTATE_DIALPARAMS: u32 = 67108864u32;
pub const LINECALLINFOSTATE_DISPLAY: u32 = 1048576u32;
pub const LINECALLINFOSTATE_HIGHLEVELCOMP: u32 = 4194304u32;
pub const LINECALLINFOSTATE_LOWLEVELCOMP: u32 = 8388608u32;
pub const LINECALLINFOSTATE_MEDIAMODE: u32 = 16u32;
pub const LINECALLINFOSTATE_MONITORMODES: u32 = 134217728u32;
pub const LINECALLINFOSTATE_NUMMONITORS: u32 = 8192u32;
pub const LINECALLINFOSTATE_NUMOWNERDECR: u32 = 4096u32;
pub const LINECALLINFOSTATE_NUMOWNERINCR: u32 = 2048u32;
pub const LINECALLINFOSTATE_ORIGIN: u32 = 256u32;
pub const LINECALLINFOSTATE_OTHER: u32 = 1u32;
pub const LINECALLINFOSTATE_QOS: u32 = 536870912u32;
pub const LINECALLINFOSTATE_RATE: u32 = 8u32;
pub const LINECALLINFOSTATE_REASON: u32 = 512u32;
pub const LINECALLINFOSTATE_REDIRECTINGID: u32 = 524288u32;
pub const LINECALLINFOSTATE_REDIRECTIONID: u32 = 262144u32;
pub const LINECALLINFOSTATE_RELATEDCALLID: u32 = 128u32;
pub const LINECALLINFOSTATE_TERMINAL: u32 = 33554432u32;
pub const LINECALLINFOSTATE_TREATMENT: u32 = 268435456u32;
pub const LINECALLINFOSTATE_TRUNK: u32 = 16384u32;
pub const LINECALLINFOSTATE_USERUSERINFO: u32 = 2097152u32;
pub const LINECALLORIGIN_CONFERENCE: u32 = 64u32;
pub const LINECALLORIGIN_EXTERNAL: u32 = 4u32;
pub const LINECALLORIGIN_INBOUND: u32 = 128u32;
pub const LINECALLORIGIN_INTERNAL: u32 = 2u32;
pub const LINECALLORIGIN_OUTBOUND: u32 = 1u32;
pub const LINECALLORIGIN_UNAVAIL: u32 = 32u32;
pub const LINECALLORIGIN_UNKNOWN: u32 = 16u32;
pub const LINECALLPARAMFLAGS_BLOCKID: u32 = 4u32;
pub const LINECALLPARAMFLAGS_DESTOFFHOOK: u32 = 16u32;
pub const LINECALLPARAMFLAGS_IDLE: u32 = 2u32;
pub const LINECALLPARAMFLAGS_NOHOLDCONFERENCE: u32 = 32u32;
pub const LINECALLPARAMFLAGS_ONESTEPTRANSFER: u32 = 128u32;
pub const LINECALLPARAMFLAGS_ORIGOFFHOOK: u32 = 8u32;
pub const LINECALLPARAMFLAGS_PREDICTIVEDIAL: u32 = 64u32;
pub const LINECALLPARAMFLAGS_SECURE: u32 = 1u32;
pub const LINECALLPARTYID_ADDRESS: u32 = 8u32;
pub const LINECALLPARTYID_BLOCKED: u32 = 1u32;
pub const LINECALLPARTYID_NAME: u32 = 4u32;
pub const LINECALLPARTYID_OUTOFAREA: u32 = 2u32;
pub const LINECALLPARTYID_PARTIAL: u32 = 16u32;
pub const LINECALLPARTYID_UNAVAIL: u32 = 64u32;
pub const LINECALLPARTYID_UNKNOWN: u32 = 32u32;
pub const LINECALLPRIVILEGE_MONITOR: u32 = 2u32;
pub const LINECALLPRIVILEGE_NONE: u32 = 1u32;
pub const LINECALLPRIVILEGE_OWNER: u32 = 4u32;
pub const LINECALLREASON_CALLCOMPLETION: u32 = 128u32;
pub const LINECALLREASON_CAMPEDON: u32 = 16384u32;
pub const LINECALLREASON_DIRECT: u32 = 1u32;
pub const LINECALLREASON_FWDBUSY: u32 = 2u32;
pub const LINECALLREASON_FWDNOANSWER: u32 = 4u32;
pub const LINECALLREASON_FWDUNCOND: u32 = 8u32;
pub const LINECALLREASON_INTRUDE: u32 = 4096u32;
pub const LINECALLREASON_PARKED: u32 = 8192u32;
pub const LINECALLREASON_PICKUP: u32 = 16u32;
pub const LINECALLREASON_REDIRECT: u32 = 64u32;
pub const LINECALLREASON_REMINDER: u32 = 512u32;
pub const LINECALLREASON_ROUTEREQUEST: u32 = 32768u32;
pub const LINECALLREASON_TRANSFER: u32 = 256u32;
pub const LINECALLREASON_UNAVAIL: u32 = 2048u32;
pub const LINECALLREASON_UNKNOWN: u32 = 1024u32;
pub const LINECALLREASON_UNPARK: u32 = 32u32;
pub const LINECALLSELECT_ADDRESS: u32 = 2u32;
pub const LINECALLSELECT_CALL: u32 = 4u32;
pub const LINECALLSELECT_CALLID: u32 = 16u32;
pub const LINECALLSELECT_DEVICEID: u32 = 8u32;
pub const LINECALLSELECT_LINE: u32 = 1u32;
pub const LINECALLSTATE_ACCEPTED: u32 = 4u32;
pub const LINECALLSTATE_BUSY: u32 = 64u32;
pub const LINECALLSTATE_CONFERENCED: u32 = 2048u32;
pub const LINECALLSTATE_CONNECTED: u32 = 256u32;
pub const LINECALLSTATE_DIALING: u32 = 16u32;
pub const LINECALLSTATE_DIALTONE: u32 = 8u32;
pub const LINECALLSTATE_DISCONNECTED: u32 = 16384u32;
pub const LINECALLSTATE_IDLE: u32 = 1u32;
pub const LINECALLSTATE_OFFERING: u32 = 2u32;
pub const LINECALLSTATE_ONHOLD: u32 = 1024u32;
pub const LINECALLSTATE_ONHOLDPENDCONF: u32 = 4096u32;
pub const LINECALLSTATE_ONHOLDPENDTRANSFER: u32 = 8192u32;
pub const LINECALLSTATE_PROCEEDING: u32 = 512u32;
pub const LINECALLSTATE_RINGBACK: u32 = 32u32;
pub const LINECALLSTATE_SPECIALINFO: u32 = 128u32;
pub const LINECALLSTATE_UNKNOWN: u32 = 32768u32;
pub const LINECALLTREATMENT_BUSY: u32 = 3u32;
pub const LINECALLTREATMENT_MUSIC: u32 = 4u32;
pub const LINECALLTREATMENT_RINGBACK: u32 = 2u32;
pub const LINECALLTREATMENT_SILENCE: u32 = 1u32;
pub const LINECARDOPTION_HIDDEN: u32 = 2u32;
pub const LINECARDOPTION_PREDEFINED: u32 = 1u32;
pub const LINECONNECTEDMODE_ACTIVE: u32 = 1u32;
pub const LINECONNECTEDMODE_ACTIVEHELD: u32 = 4u32;
pub const LINECONNECTEDMODE_CONFIRMED: u32 = 16u32;
pub const LINECONNECTEDMODE_INACTIVE: u32 = 2u32;
pub const LINECONNECTEDMODE_INACTIVEHELD: u32 = 8u32;
pub const LINEDEVCAPFLAGS_CALLHUB: u32 = 1024u32;
pub const LINEDEVCAPFLAGS_CALLHUBTRACKING: u32 = 2048u32;
pub const LINEDEVCAPFLAGS_CLOSEDROP: u32 = 32u32;
pub const LINEDEVCAPFLAGS_CROSSADDRCONF: u32 = 1u32;
pub const LINEDEVCAPFLAGS_DIALBILLING: u32 = 64u32;
pub const LINEDEVCAPFLAGS_DIALDIALTONE: u32 = 256u32;
pub const LINEDEVCAPFLAGS_DIALQUIET: u32 = 128u32;
pub const LINEDEVCAPFLAGS_HIGHLEVCOMP: u32 = 2u32;
pub const LINEDEVCAPFLAGS_LOCAL: u32 = 8192u32;
pub const LINEDEVCAPFLAGS_LOWLEVCOMP: u32 = 4u32;
pub const LINEDEVCAPFLAGS_MEDIACONTROL: u32 = 8u32;
pub const LINEDEVCAPFLAGS_MSP: u32 = 512u32;
pub const LINEDEVCAPFLAGS_MULTIPLEADDR: u32 = 16u32;
pub const LINEDEVCAPFLAGS_PRIVATEOBJECTS: u32 = 4096u32;
pub const LINEDEVSTATE_BATTERY: u32 = 32768u32;
pub const LINEDEVSTATE_CAPSCHANGE: u32 = 1048576u32;
pub const LINEDEVSTATE_CLOSE: u32 = 1024u32;
pub const LINEDEVSTATE_COMPLCANCEL: u32 = 8388608u32;
pub const LINEDEVSTATE_CONFIGCHANGE: u32 = 2097152u32;
pub const LINEDEVSTATE_CONNECTED: u32 = 4u32;
pub const LINEDEVSTATE_DEVSPECIFIC: u32 = 131072u32;
pub const LINEDEVSTATE_DISCONNECTED: u32 = 8u32;
pub const LINEDEVSTATE_INSERVICE: u32 = 64u32;
pub const LINEDEVSTATE_LOCK: u32 = 524288u32;
pub const LINEDEVSTATE_MAINTENANCE: u32 = 256u32;
pub const LINEDEVSTATE_MSGWAITOFF: u32 = 32u32;
pub const LINEDEVSTATE_MSGWAITON: u32 = 16u32;
pub const LINEDEVSTATE_NUMCALLS: u32 = 2048u32;
pub const LINEDEVSTATE_NUMCOMPLETIONS: u32 = 4096u32;
pub const LINEDEVSTATE_OPEN: u32 = 512u32;
pub const LINEDEVSTATE_OTHER: u32 = 1u32;
pub const LINEDEVSTATE_OUTOFSERVICE: u32 = 128u32;
pub const LINEDEVSTATE_REINIT: u32 = 262144u32;
pub const LINEDEVSTATE_REMOVED: u32 = 16777216u32;
pub const LINEDEVSTATE_RINGING: u32 = 2u32;
pub const LINEDEVSTATE_ROAMMODE: u32 = 16384u32;
pub const LINEDEVSTATE_SIGNAL: u32 = 65536u32;
pub const LINEDEVSTATE_TERMINALS: u32 = 8192u32;
pub const LINEDEVSTATE_TRANSLATECHANGE: u32 = 4194304u32;
pub const LINEDEVSTATUSFLAGS_CONNECTED: u32 = 1u32;
pub const LINEDEVSTATUSFLAGS_INSERVICE: u32 = 4u32;
pub const LINEDEVSTATUSFLAGS_LOCKED: u32 = 8u32;
pub const LINEDEVSTATUSFLAGS_MSGWAIT: u32 = 2u32;
pub const LINEDIALTONEMODE_EXTERNAL: u32 = 8u32;
pub const LINEDIALTONEMODE_INTERNAL: u32 = 4u32;
pub const LINEDIALTONEMODE_NORMAL: u32 = 1u32;
pub const LINEDIALTONEMODE_SPECIAL: u32 = 2u32;
pub const LINEDIALTONEMODE_UNAVAIL: u32 = 32u32;
pub const LINEDIALTONEMODE_UNKNOWN: u32 = 16u32;
pub const LINEDIGITMODE_DTMF: u32 = 2u32;
pub const LINEDIGITMODE_DTMFEND: u32 = 4u32;
pub const LINEDIGITMODE_PULSE: u32 = 1u32;
pub const LINEDISCONNECTMODE_BADADDRESS: u32 = 128u32;
pub const LINEDISCONNECTMODE_BLOCKED: u32 = 131072u32;
pub const LINEDISCONNECTMODE_BUSY: u32 = 32u32;
pub const LINEDISCONNECTMODE_CANCELLED: u32 = 524288u32;
pub const LINEDISCONNECTMODE_CONGESTION: u32 = 512u32;
pub const LINEDISCONNECTMODE_DESTINATIONBARRED: u32 = 1048576u32;
pub const LINEDISCONNECTMODE_DONOTDISTURB: u32 = 262144u32;
pub const LINEDISCONNECTMODE_FDNRESTRICT: u32 = 2097152u32;
pub const LINEDISCONNECTMODE_FORWARDED: u32 = 16u32;
pub const LINEDISCONNECTMODE_INCOMPATIBLE: u32 = 1024u32;
pub const LINEDISCONNECTMODE_NOANSWER: u32 = 64u32;
pub const LINEDISCONNECTMODE_NODIALTONE: u32 = 4096u32;
pub const LINEDISCONNECTMODE_NORMAL: u32 = 1u32;
pub const LINEDISCONNECTMODE_NUMBERCHANGED: u32 = 8192u32;
pub const LINEDISCONNECTMODE_OUTOFORDER: u32 = 16384u32;
pub const LINEDISCONNECTMODE_PICKUP: u32 = 8u32;
pub const LINEDISCONNECTMODE_QOSUNAVAIL: u32 = 65536u32;
pub const LINEDISCONNECTMODE_REJECT: u32 = 4u32;
pub const LINEDISCONNECTMODE_TEMPFAILURE: u32 = 32768u32;
pub const LINEDISCONNECTMODE_UNAVAIL: u32 = 2048u32;
pub const LINEDISCONNECTMODE_UNKNOWN: u32 = 2u32;
pub const LINEDISCONNECTMODE_UNREACHABLE: u32 = 256u32;
pub const LINEEQOSINFO_ADMISSIONFAILURE: u32 = 2u32;
pub const LINEEQOSINFO_GENERICERROR: u32 = 4u32;
pub const LINEEQOSINFO_NOQOS: u32 = 1u32;
pub const LINEEQOSINFO_POLICYFAILURE: u32 = 3u32;
pub const LINEERR_ADDRESSBLOCKED: u32 = 2147483731u32;
pub const LINEERR_ALLOCATED: u32 = 2147483649u32;
pub const LINEERR_BADDEVICEID: u32 = 2147483650u32;
pub const LINEERR_BEARERMODEUNAVAIL: u32 = 2147483651u32;
pub const LINEERR_BILLINGREJECTED: u32 = 2147483732u32;
pub const LINEERR_CALLUNAVAIL: u32 = 2147483653u32;
pub const LINEERR_COMPLETIONOVERRUN: u32 = 2147483654u32;
pub const LINEERR_CONFERENCEFULL: u32 = 2147483655u32;
pub const LINEERR_DIALBILLING: u32 = 2147483656u32;
pub const LINEERR_DIALDIALTONE: u32 = 2147483657u32;
pub const LINEERR_DIALPROMPT: u32 = 2147483658u32;
pub const LINEERR_DIALQUIET: u32 = 2147483659u32;
pub const LINEERR_DIALVOICEDETECT: u32 = 2147483740u32;
pub const LINEERR_DISCONNECTED: u32 = 2147483744u32;
pub const LINEERR_INCOMPATIBLEAPIVERSION: u32 = 2147483660u32;
pub const LINEERR_INCOMPATIBLEEXTVERSION: u32 = 2147483661u32;
pub const LINEERR_INIFILECORRUPT: u32 = 2147483662u32;
pub const LINEERR_INUSE: u32 = 2147483663u32;
pub const LINEERR_INVALADDRESS: u32 = 2147483664u32;
pub const LINEERR_INVALADDRESSID: u32 = 2147483665u32;
pub const LINEERR_INVALADDRESSMODE: u32 = 2147483666u32;
pub const LINEERR_INVALADDRESSSTATE: u32 = 2147483667u32;
pub const LINEERR_INVALADDRESSTYPE: u32 = 2147483742u32;
pub const LINEERR_INVALAGENTACTIVITY: u32 = 2147483739u32;
pub const LINEERR_INVALAGENTGROUP: u32 = 2147483736u32;
pub const LINEERR_INVALAGENTID: u32 = 2147483735u32;
pub const LINEERR_INVALAGENTSESSIONSTATE: u32 = 2147483743u32;
pub const LINEERR_INVALAGENTSTATE: u32 = 2147483738u32;
pub const LINEERR_INVALAPPHANDLE: u32 = 2147483668u32;
pub const LINEERR_INVALAPPNAME: u32 = 2147483669u32;
pub const LINEERR_INVALBEARERMODE: u32 = 2147483670u32;
pub const LINEERR_INVALCALLCOMPLMODE: u32 = 2147483671u32;
pub const LINEERR_INVALCALLHANDLE: u32 = 2147483672u32;
pub const LINEERR_INVALCALLPARAMS: u32 = 2147483673u32;
pub const LINEERR_INVALCALLPRIVILEGE: u32 = 2147483674u32;
pub const LINEERR_INVALCALLSELECT: u32 = 2147483675u32;
pub const LINEERR_INVALCALLSTATE: u32 = 2147483676u32;
pub const LINEERR_INVALCALLSTATELIST: u32 = 2147483677u32;
pub const LINEERR_INVALCARD: u32 = 2147483678u32;
pub const LINEERR_INVALCOMPLETIONID: u32 = 2147483679u32;
pub const LINEERR_INVALCONFCALLHANDLE: u32 = 2147483680u32;
pub const LINEERR_INVALCONSULTCALLHANDLE: u32 = 2147483681u32;
pub const LINEERR_INVALCOUNTRYCODE: u32 = 2147483682u32;
pub const LINEERR_INVALDEVICECLASS: u32 = 2147483683u32;
pub const LINEERR_INVALDEVICEHANDLE: u32 = 2147483684u32;
pub const LINEERR_INVALDIALPARAMS: u32 = 2147483685u32;
pub const LINEERR_INVALDIGITLIST: u32 = 2147483686u32;
pub const LINEERR_INVALDIGITMODE: u32 = 2147483687u32;
pub const LINEERR_INVALDIGITS: u32 = 2147483688u32;
pub const LINEERR_INVALEXTVERSION: u32 = 2147483689u32;
pub const LINEERR_INVALFEATURE: u32 = 2147483733u32;
pub const LINEERR_INVALGROUPID: u32 = 2147483690u32;
pub const LINEERR_INVALLINEHANDLE: u32 = 2147483691u32;
pub const LINEERR_INVALLINESTATE: u32 = 2147483692u32;
pub const LINEERR_INVALLOCATION: u32 = 2147483693u32;
pub const LINEERR_INVALMEDIALIST: u32 = 2147483694u32;
pub const LINEERR_INVALMEDIAMODE: u32 = 2147483695u32;
pub const LINEERR_INVALMESSAGEID: u32 = 2147483696u32;
pub const LINEERR_INVALPARAM: u32 = 2147483698u32;
pub const LINEERR_INVALPARKID: u32 = 2147483699u32;
pub const LINEERR_INVALPARKMODE: u32 = 2147483700u32;
pub const LINEERR_INVALPASSWORD: u32 = 2147483737u32;
pub const LINEERR_INVALPOINTER: u32 = 2147483701u32;
pub const LINEERR_INVALPRIVSELECT: u32 = 2147483702u32;
pub const LINEERR_INVALRATE: u32 = 2147483703u32;
pub const LINEERR_INVALREQUESTMODE: u32 = 2147483704u32;
pub const LINEERR_INVALTERMINALID: u32 = 2147483705u32;
pub const LINEERR_INVALTERMINALMODE: u32 = 2147483706u32;
pub const LINEERR_INVALTIMEOUT: u32 = 2147483707u32;
pub const LINEERR_INVALTONE: u32 = 2147483708u32;
pub const LINEERR_INVALTONELIST: u32 = 2147483709u32;
pub const LINEERR_INVALTONEMODE: u32 = 2147483710u32;
pub const LINEERR_INVALTRANSFERMODE: u32 = 2147483711u32;
pub const LINEERR_LINEMAPPERFAILED: u32 = 2147483712u32;
pub const LINEERR_NOCONFERENCE: u32 = 2147483713u32;
pub const LINEERR_NODEVICE: u32 = 2147483714u32;
pub const LINEERR_NODRIVER: u32 = 2147483715u32;
pub const LINEERR_NOMEM: u32 = 2147483716u32;
pub const LINEERR_NOMULTIPLEINSTANCE: u32 = 2147483734u32;
pub const LINEERR_NOREQUEST: u32 = 2147483717u32;
pub const LINEERR_NOTOWNER: u32 = 2147483718u32;
pub const LINEERR_NOTREGISTERED: u32 = 2147483719u32;
pub const LINEERR_OPERATIONFAILED: u32 = 2147483720u32;
pub const LINEERR_OPERATIONUNAVAIL: u32 = 2147483721u32;
pub const LINEERR_RATEUNAVAIL: u32 = 2147483722u32;
pub const LINEERR_REINIT: u32 = 2147483730u32;
pub const LINEERR_REQUESTOVERRUN: u32 = 2147483724u32;
pub const LINEERR_RESOURCEUNAVAIL: u32 = 2147483723u32;
pub const LINEERR_SERVICE_NOT_RUNNING: u32 = 2147483745u32;
pub const LINEERR_STRUCTURETOOSMALL: u32 = 2147483725u32;
pub const LINEERR_TARGETNOTFOUND: u32 = 2147483726u32;
pub const LINEERR_TARGETSELF: u32 = 2147483727u32;
pub const LINEERR_UNINITIALIZED: u32 = 2147483728u32;
pub const LINEERR_USERCANCELLED: u32 = 2147483741u32;
pub const LINEERR_USERUSERINFOTOOBIG: u32 = 2147483729u32;
pub const LINEFEATURE_DEVSPECIFIC: u32 = 1u32;
pub const LINEFEATURE_DEVSPECIFICFEAT: u32 = 2u32;
pub const LINEFEATURE_FORWARD: u32 = 4u32;
pub const LINEFEATURE_FORWARDDND: u32 = 256u32;
pub const LINEFEATURE_FORWARDFWD: u32 = 128u32;
pub const LINEFEATURE_MAKECALL: u32 = 8u32;
pub const LINEFEATURE_SETDEVSTATUS: u32 = 64u32;
pub const LINEFEATURE_SETMEDIACONTROL: u32 = 16u32;
pub const LINEFEATURE_SETTERMINAL: u32 = 32u32;
pub const LINEFORWARDMODE_BUSY: u32 = 16u32;
pub const LINEFORWARDMODE_BUSYEXTERNAL: u32 = 64u32;
pub const LINEFORWARDMODE_BUSYINTERNAL: u32 = 32u32;
pub const LINEFORWARDMODE_BUSYNA: u32 = 4096u32;
pub const LINEFORWARDMODE_BUSYNAEXTERNAL: u32 = 16384u32;
pub const LINEFORWARDMODE_BUSYNAINTERNAL: u32 = 8192u32;
pub const LINEFORWARDMODE_BUSYNASPECIFIC: u32 = 32768u32;
pub const LINEFORWARDMODE_BUSYSPECIFIC: u32 = 128u32;
pub const LINEFORWARDMODE_NOANSW: u32 = 256u32;
pub const LINEFORWARDMODE_NOANSWEXTERNAL: u32 = 1024u32;
pub const LINEFORWARDMODE_NOANSWINTERNAL: u32 = 512u32;
pub const LINEFORWARDMODE_NOANSWSPECIFIC: u32 = 2048u32;
pub const LINEFORWARDMODE_UNAVAIL: u32 = 131072u32;
pub const LINEFORWARDMODE_UNCOND: u32 = 1u32;
pub const LINEFORWARDMODE_UNCONDEXTERNAL: u32 = 4u32;
pub const LINEFORWARDMODE_UNCONDINTERNAL: u32 = 2u32;
pub const LINEFORWARDMODE_UNCONDSPECIFIC: u32 = 8u32;
pub const LINEFORWARDMODE_UNKNOWN: u32 = 65536u32;
pub const LINEGATHERTERM_BUFFERFULL: u32 = 1u32;
pub const LINEGATHERTERM_CANCEL: u32 = 16u32;
pub const LINEGATHERTERM_FIRSTTIMEOUT: u32 = 4u32;
pub const LINEGATHERTERM_INTERTIMEOUT: u32 = 8u32;
pub const LINEGATHERTERM_TERMDIGIT: u32 = 2u32;
pub const LINEGENERATETERM_CANCEL: u32 = 2u32;
pub const LINEGENERATETERM_DONE: u32 = 1u32;
pub const LINEGROUPSTATUS_GROUPREMOVED: u32 = 2u32;
pub const LINEGROUPSTATUS_NEWGROUP: u32 = 1u32;
pub const LINEINITIALIZEEXOPTION_CALLHUBTRACKING: u32 = 2147483648u32;
pub const LINEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
pub const LINEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
pub const LINEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
pub const LINELOCATIONOPTION_PULSEDIAL: u32 = 1u32;
pub const LINEMAPPER: u32 = 4294967295u32;
pub const LINEMEDIACONTROL_NONE: u32 = 1u32;
pub const LINEMEDIACONTROL_PAUSE: u32 = 8u32;
pub const LINEMEDIACONTROL_RATEDOWN: u32 = 64u32;
pub const LINEMEDIACONTROL_RATENORMAL: u32 = 128u32;
pub const LINEMEDIACONTROL_RATEUP: u32 = 32u32;
pub const LINEMEDIACONTROL_RESET: u32 = 4u32;
pub const LINEMEDIACONTROL_RESUME: u32 = 16u32;
pub const LINEMEDIACONTROL_START: u32 = 2u32;
pub const LINEMEDIACONTROL_VOLUMEDOWN: u32 = 512u32;
pub const LINEMEDIACONTROL_VOLUMENORMAL: u32 = 1024u32;
pub const LINEMEDIACONTROL_VOLUMEUP: u32 = 256u32;
pub const LINEMEDIAMODE_ADSI: u32 = 8192u32;
pub const LINEMEDIAMODE_AUTOMATEDVOICE: u32 = 8u32;
pub const LINEMEDIAMODE_DATAMODEM: u32 = 16u32;
pub const LINEMEDIAMODE_DIGITALDATA: u32 = 256u32;
pub const LINEMEDIAMODE_G3FAX: u32 = 32u32;
pub const LINEMEDIAMODE_G4FAX: u32 = 128u32;
pub const LINEMEDIAMODE_INTERACTIVEVOICE: u32 = 4u32;
pub const LINEMEDIAMODE_MIXED: u32 = 4096u32;
pub const LINEMEDIAMODE_TDD: u32 = 64u32;
pub const LINEMEDIAMODE_TELETEX: u32 = 512u32;
pub const LINEMEDIAMODE_TELEX: u32 = 2048u32;
pub const LINEMEDIAMODE_UNKNOWN: u32 = 2u32;
pub const LINEMEDIAMODE_VIDEO: u32 = 32768u32;
pub const LINEMEDIAMODE_VIDEOTEX: u32 = 1024u32;
pub const LINEMEDIAMODE_VOICEVIEW: u32 = 16384u32;
pub const LINEOFFERINGMODE_ACTIVE: u32 = 1u32;
pub const LINEOFFERINGMODE_INACTIVE: u32 = 2u32;
pub const LINEOPENOPTION_PROXY: u32 = 1073741824u32;
pub const LINEOPENOPTION_SINGLEADDRESS: u32 = 2147483648u32;
pub const LINEPARKMODE_DIRECTED: u32 = 1u32;
pub const LINEPARKMODE_NONDIRECTED: u32 = 2u32;
pub const LINEPROXYREQUEST_AGENTSPECIFIC: u32 = 6u32;
pub const LINEPROXYREQUEST_CREATEAGENT: u32 = 9u32;
pub const LINEPROXYREQUEST_CREATEAGENTSESSION: u32 = 12u32;
pub const LINEPROXYREQUEST_GETAGENTACTIVITYLIST: u32 = 7u32;
pub const LINEPROXYREQUEST_GETAGENTCAPS: u32 = 4u32;
pub const LINEPROXYREQUEST_GETAGENTGROUPLIST: u32 = 8u32;
pub const LINEPROXYREQUEST_GETAGENTINFO: u32 = 11u32;
pub const LINEPROXYREQUEST_GETAGENTSESSIONINFO: u32 = 15u32;
pub const LINEPROXYREQUEST_GETAGENTSESSIONLIST: u32 = 13u32;
pub const LINEPROXYREQUEST_GETAGENTSTATUS: u32 = 5u32;
pub const LINEPROXYREQUEST_GETGROUPLIST: u32 = 19u32;
pub const LINEPROXYREQUEST_GETQUEUEINFO: u32 = 18u32;
pub const LINEPROXYREQUEST_GETQUEUELIST: u32 = 16u32;
pub const LINEPROXYREQUEST_SETAGENTACTIVITY: u32 = 3u32;
pub const LINEPROXYREQUEST_SETAGENTGROUP: u32 = 1u32;
pub const LINEPROXYREQUEST_SETAGENTMEASUREMENTPERIOD: u32 = 10u32;
pub const LINEPROXYREQUEST_SETAGENTSESSIONSTATE: u32 = 14u32;
pub const LINEPROXYREQUEST_SETAGENTSTATE: u32 = 2u32;
pub const LINEPROXYREQUEST_SETAGENTSTATEEX: u32 = 20u32;
pub const LINEPROXYREQUEST_SETQUEUEMEASUREMENTPERIOD: u32 = 17u32;
pub const LINEPROXYSTATUS_ALLOPENFORACD: u32 = 4u32;
pub const LINEPROXYSTATUS_CLOSE: u32 = 2u32;
pub const LINEPROXYSTATUS_OPEN: u32 = 1u32;
pub const LINEQOSREQUESTTYPE_SERVICELEVEL: u32 = 1u32;
pub const LINEQOSSERVICELEVEL_BESTEFFORT: u32 = 3u32;
pub const LINEQOSSERVICELEVEL_IFAVAILABLE: u32 = 2u32;
pub const LINEQOSSERVICELEVEL_NEEDED: u32 = 1u32;
pub const LINEQUEUESTATUS_NEWQUEUE: u32 = 2u32;
pub const LINEQUEUESTATUS_QUEUEREMOVED: u32 = 4u32;
pub const LINEQUEUESTATUS_UPDATEINFO: u32 = 1u32;
pub const LINEREMOVEFROMCONF_ANY: u32 = 3u32;
pub const LINEREMOVEFROMCONF_LAST: u32 = 2u32;
pub const LINEREMOVEFROMCONF_NONE: u32 = 1u32;
pub const LINEREQUESTMODE_DROP: u32 = 4u32;
pub const LINEREQUESTMODE_MAKECALL: u32 = 1u32;
pub const LINEREQUESTMODE_MEDIACALL: u32 = 2u32;
pub const LINEROAMMODE_HOME: u32 = 4u32;
pub const LINEROAMMODE_ROAMA: u32 = 8u32;
pub const LINEROAMMODE_ROAMB: u32 = 16u32;
pub const LINEROAMMODE_UNAVAIL: u32 = 2u32;
pub const LINEROAMMODE_UNKNOWN: u32 = 1u32;
pub const LINESPECIALINFO_CUSTIRREG: u32 = 2u32;
pub const LINESPECIALINFO_NOCIRCUIT: u32 = 1u32;
pub const LINESPECIALINFO_REORDER: u32 = 4u32;
pub const LINESPECIALINFO_UNAVAIL: u32 = 16u32;
pub const LINESPECIALINFO_UNKNOWN: u32 = 8u32;
pub const LINETERMDEV_HEADSET: u32 = 2u32;
pub const LINETERMDEV_PHONE: u32 = 1u32;
pub const LINETERMDEV_SPEAKER: u32 = 4u32;
pub const LINETERMMODE_BUTTONS: u32 = 1u32;
pub const LINETERMMODE_DISPLAY: u32 = 4u32;
pub const LINETERMMODE_HOOKSWITCH: u32 = 16u32;
pub const LINETERMMODE_LAMPS: u32 = 2u32;
pub const LINETERMMODE_MEDIABIDIRECT: u32 = 128u32;
pub const LINETERMMODE_MEDIAFROMLINE: u32 = 64u32;
pub const LINETERMMODE_MEDIATOLINE: u32 = 32u32;
pub const LINETERMMODE_RINGER: u32 = 8u32;
pub const LINETERMSHARING_PRIVATE: u32 = 1u32;
pub const LINETERMSHARING_SHAREDCONF: u32 = 4u32;
pub const LINETERMSHARING_SHAREDEXCL: u32 = 2u32;
pub const LINETOLLLISTOPTION_ADD: u32 = 1u32;
pub const LINETOLLLISTOPTION_REMOVE: u32 = 2u32;
pub const LINETONEMODE_BEEP: u32 = 8u32;
pub const LINETONEMODE_BILLING: u32 = 16u32;
pub const LINETONEMODE_BUSY: u32 = 4u32;
pub const LINETONEMODE_CUSTOM: u32 = 1u32;
pub const LINETONEMODE_RINGBACK: u32 = 2u32;
pub const LINETRANSFERMODE_CONFERENCE: u32 = 2u32;
pub const LINETRANSFERMODE_TRANSFER: u32 = 1u32;
pub const LINETRANSLATEOPTION_CANCELCALLWAITING: u32 = 2u32;
pub const LINETRANSLATEOPTION_CARDOVERRIDE: u32 = 1u32;
pub const LINETRANSLATEOPTION_FORCELD: u32 = 8u32;
pub const LINETRANSLATEOPTION_FORCELOCAL: u32 = 4u32;
pub const LINETRANSLATERESULT_CANONICAL: u32 = 1u32;
pub const LINETRANSLATERESULT_DIALBILLING: u32 = 64u32;
pub const LINETRANSLATERESULT_DIALDIALTONE: u32 = 256u32;
pub const LINETRANSLATERESULT_DIALPROMPT: u32 = 512u32;
pub const LINETRANSLATERESULT_DIALQUIET: u32 = 128u32;
pub const LINETRANSLATERESULT_INTERNATIONAL: u32 = 2u32;
pub const LINETRANSLATERESULT_INTOLLLIST: u32 = 16u32;
pub const LINETRANSLATERESULT_LOCAL: u32 = 8u32;
pub const LINETRANSLATERESULT_LONGDISTANCE: u32 = 4u32;
pub const LINETRANSLATERESULT_NOTINTOLLLIST: u32 = 32u32;
pub const LINETRANSLATERESULT_NOTRANSLATION: u32 = 2048u32;
pub const LINETRANSLATERESULT_VOICEDETECT: u32 = 1024u32;
pub const LINETSPIOPTION_NONREENTRANT: u32 = 1u32;
pub const LINE_ADDRESSSTATE: i32 = 0i32;
pub const LINE_AGENTSESSIONSTATUS: i32 = 27i32;
pub const LINE_AGENTSPECIFIC: i32 = 21i32;
pub const LINE_AGENTSTATUS: i32 = 22i32;
pub const LINE_AGENTSTATUSEX: i32 = 29i32;
pub const LINE_APPNEWCALL: i32 = 23i32;
pub const LINE_APPNEWCALLHUB: i32 = 32i32;
pub const LINE_CALLHUBCLOSE: i32 = 33i32;
pub const LINE_CALLINFO: i32 = 1i32;
pub const LINE_CALLSTATE: i32 = 2i32;
pub const LINE_CLOSE: i32 = 3i32;
pub const LINE_CREATE: i32 = 19i32;
pub const LINE_DEVSPECIFIC: i32 = 4i32;
pub const LINE_DEVSPECIFICEX: i32 = 34i32;
pub const LINE_DEVSPECIFICFEATURE: i32 = 5i32;
pub const LINE_GATHERDIGITS: i32 = 6i32;
pub const LINE_GENERATE: i32 = 7i32;
pub const LINE_GROUPSTATUS: i32 = 30i32;
pub const LINE_LINEDEVSTATE: i32 = 8i32;
pub const LINE_MONITORDIGITS: i32 = 9i32;
pub const LINE_MONITORMEDIA: i32 = 10i32;
pub const LINE_MONITORTONE: i32 = 11i32;
pub const LINE_PROXYREQUEST: i32 = 24i32;
pub const LINE_PROXYSTATUS: i32 = 31i32;
pub const LINE_QUEUESTATUS: i32 = 28i32;
pub const LINE_REMOVE: i32 = 25i32;
pub const LINE_REPLY: i32 = 12i32;
pub const LINE_REQUEST: i32 = 13i32;
pub const LM_BROKENFLUTTER: PHONE_LAMP_MODE = PHONE_LAMP_MODE(64i32);
pub const LM_DUMMY: PHONE_LAMP_MODE = PHONE_LAMP_MODE(1i32);
pub const LM_FLASH: PHONE_LAMP_MODE = PHONE_LAMP_MODE(16i32);
pub const LM_FLUTTER: PHONE_LAMP_MODE = PHONE_LAMP_MODE(32i32);
pub const LM_OFF: PHONE_LAMP_MODE = PHONE_LAMP_MODE(2i32);
pub const LM_STEADY: PHONE_LAMP_MODE = PHONE_LAMP_MODE(4i32);
pub const LM_UNKNOWN: PHONE_LAMP_MODE = PHONE_LAMP_MODE(128i32);
pub const LM_WINK: PHONE_LAMP_MODE = PHONE_LAMP_MODE(8i32);
pub const ME_ADDRESS_EVENT: MSP_EVENT = MSP_EVENT(0i32);
pub const ME_ASR_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(4i32);
pub const ME_CALL_EVENT: MSP_EVENT = MSP_EVENT(1i32);
pub const ME_FILE_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(6i32);
pub const ME_PRIVATE_EVENT: MSP_EVENT = MSP_EVENT(3i32);
pub const ME_TONE_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(7i32);
pub const ME_TSP_DATA: MSP_EVENT = MSP_EVENT(2i32);
pub const ME_TTS_TERMINAL_EVENT: MSP_EVENT = MSP_EVENT(5i32);
pub const OPENTNEFSTREAM: windows_core::PCSTR = windows_core::s!("OpenTnefStream");
pub const OPENTNEFSTREAMEX: windows_core::PCSTR = windows_core::s!("OpenTnefStreamEx");
pub const OT_CONFERENCE: DIRECTORY_OBJECT_TYPE = DIRECTORY_OBJECT_TYPE(1i32);
pub const OT_USER: DIRECTORY_OBJECT_TYPE = DIRECTORY_OBJECT_TYPE(2i32);
pub const PBF_ABBREVDIAL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(11i32);
pub const PBF_BRIDGEDAPP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(28i32);
pub const PBF_BUSY: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(29i32);
pub const PBF_CALLAPP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(30i32);
pub const PBF_CALLID: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(34i32);
pub const PBF_CAMPON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(43i32);
pub const PBF_CONFERENCE: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(1i32);
pub const PBF_CONNECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(7i32);
pub const PBF_COVER: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(33i32);
pub const PBF_DATAOFF: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(25i32);
pub const PBF_DATAON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(24i32);
pub const PBF_DATETIME: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(31i32);
pub const PBF_DIRECTORY: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(32i32);
pub const PBF_DISCONNECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(6i32);
pub const PBF_DONOTDISTURB: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(26i32);
pub const PBF_DROP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(3i32);
pub const PBF_FLASH: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(23i32);
pub const PBF_FORWARD: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(12i32);
pub const PBF_HOLD: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(4i32);
pub const PBF_INTERCOM: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(27i32);
pub const PBF_LASTNUM: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(35i32);
pub const PBF_MSGINDICATOR: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(38i32);
pub const PBF_MSGWAITOFF: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(9i32);
pub const PBF_MSGWAITON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(8i32);
pub const PBF_MUTE: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(18i32);
pub const PBF_NIGHTSRV: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(36i32);
pub const PBF_NONE: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(46i32);
pub const PBF_PARK: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(15i32);
pub const PBF_PICKUP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(13i32);
pub const PBF_QUEUECALL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(45i32);
pub const PBF_RECALL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(5i32);
pub const PBF_REDIRECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(17i32);
pub const PBF_REJECT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(16i32);
pub const PBF_REPDIAL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(39i32);
pub const PBF_RINGAGAIN: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(14i32);
pub const PBF_SAVEREPEAT: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(44i32);
pub const PBF_SELECTRING: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(10i32);
pub const PBF_SEND: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(47i32);
pub const PBF_SENDCALLS: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(37i32);
pub const PBF_SETREPDIAL: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(40i32);
pub const PBF_SPEAKEROFF: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(22i32);
pub const PBF_SPEAKERON: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(21i32);
pub const PBF_STATIONSPEED: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(42i32);
pub const PBF_SYSTEMSPEED: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(41i32);
pub const PBF_TRANSFER: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(2i32);
pub const PBF_UNKNOWN: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(0i32);
pub const PBF_VOLUMEDOWN: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(20i32);
pub const PBF_VOLUMEUP: PHONE_BUTTON_FUNCTION = PHONE_BUTTON_FUNCTION(19i32);
pub const PBM_CALL: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(1i32);
pub const PBM_DISPLAY: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(5i32);
pub const PBM_DUMMY: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(0i32);
pub const PBM_FEATURE: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(2i32);
pub const PBM_KEYPAD: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(3i32);
pub const PBM_LOCAL: PHONE_BUTTON_MODE = PHONE_BUTTON_MODE(4i32);
pub const PBS_DOWN: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(2i32);
pub const PBS_UNAVAIL: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(8i32);
pub const PBS_UNKNOWN: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(4i32);
pub const PBS_UP: PHONE_BUTTON_STATE = PHONE_BUTTON_STATE(1i32);
pub const PCB_DEVSPECIFICBUFFER: PHONECAPS_BUFFER = PHONECAPS_BUFFER(0i32);
pub const PCL_DISPLAYNUMCOLUMNS: PHONECAPS_LONG = PHONECAPS_LONG(5i32);
pub const PCL_DISPLAYNUMROWS: PHONECAPS_LONG = PHONECAPS_LONG(4i32);
pub const PCL_GENERICPHONE: PHONECAPS_LONG = PHONECAPS_LONG(8i32);
pub const PCL_HANDSETHOOKSWITCHMODES: PHONECAPS_LONG = PHONECAPS_LONG(1i32);
pub const PCL_HEADSETHOOKSWITCHMODES: PHONECAPS_LONG = PHONECAPS_LONG(2i32);
pub const PCL_HOOKSWITCHES: PHONECAPS_LONG = PHONECAPS_LONG(0i32);
pub const PCL_NUMBUTTONLAMPS: PHONECAPS_LONG = PHONECAPS_LONG(7i32);
pub const PCL_NUMRINGMODES: PHONECAPS_LONG = PHONECAPS_LONG(6i32);
pub const PCL_SPEAKERPHONEHOOKSWITCHMODES: PHONECAPS_LONG = PHONECAPS_LONG(3i32);
pub const PCS_PHONEINFO: PHONECAPS_STRING = PHONECAPS_STRING(1i32);
pub const PCS_PHONENAME: PHONECAPS_STRING = PHONECAPS_STRING(0i32);
pub const PCS_PROVIDERINFO: PHONECAPS_STRING = PHONECAPS_STRING(2i32);
pub const PE_ANSWER: PHONE_EVENT = PHONE_EVENT(10i32);
pub const PE_BUTTON: PHONE_EVENT = PHONE_EVENT(6i32);
pub const PE_CAPSCHANGE: PHONE_EVENT = PHONE_EVENT(5i32);
pub const PE_CLOSE: PHONE_EVENT = PHONE_EVENT(7i32);
pub const PE_DIALING: PHONE_EVENT = PHONE_EVENT(9i32);
pub const PE_DISCONNECT: PHONE_EVENT = PHONE_EVENT(11i32);
pub const PE_DISPLAY: PHONE_EVENT = PHONE_EVENT(0i32);
pub const PE_HOOKSWITCH: PHONE_EVENT = PHONE_EVENT(4i32);
pub const PE_LAMPMODE: PHONE_EVENT = PHONE_EVENT(1i32);
pub const PE_LASTITEM: PHONE_EVENT = PHONE_EVENT(11i32);
pub const PE_NUMBERGATHERED: PHONE_EVENT = PHONE_EVENT(8i32);
pub const PE_RINGMODE: PHONE_EVENT = PHONE_EVENT(2i32);
pub const PE_RINGVOLUME: PHONE_EVENT = PHONE_EVENT(3i32);
pub const PHONEBUTTONFUNCTION_ABBREVDIAL: u32 = 11u32;
pub const PHONEBUTTONFUNCTION_BRIDGEDAPP: u32 = 28u32;
pub const PHONEBUTTONFUNCTION_BUSY: u32 = 29u32;
pub const PHONEBUTTONFUNCTION_CALLAPP: u32 = 30u32;
pub const PHONEBUTTONFUNCTION_CALLID: u32 = 34u32;
pub const PHONEBUTTONFUNCTION_CAMPON: u32 = 43u32;
pub const PHONEBUTTONFUNCTION_CONFERENCE: u32 = 1u32;
pub const PHONEBUTTONFUNCTION_CONNECT: u32 = 7u32;
pub const PHONEBUTTONFUNCTION_COVER: u32 = 33u32;
pub const PHONEBUTTONFUNCTION_DATAOFF: u32 = 25u32;
pub const PHONEBUTTONFUNCTION_DATAON: u32 = 24u32;
pub const PHONEBUTTONFUNCTION_DATETIME: u32 = 31u32;
pub const PHONEBUTTONFUNCTION_DIRECTORY: u32 = 32u32;
pub const PHONEBUTTONFUNCTION_DISCONNECT: u32 = 6u32;
pub const PHONEBUTTONFUNCTION_DONOTDISTURB: u32 = 26u32;
pub const PHONEBUTTONFUNCTION_DROP: u32 = 3u32;
pub const PHONEBUTTONFUNCTION_FLASH: u32 = 23u32;
pub const PHONEBUTTONFUNCTION_FORWARD: u32 = 12u32;
pub const PHONEBUTTONFUNCTION_HOLD: u32 = 4u32;
pub const PHONEBUTTONFUNCTION_INTERCOM: u32 = 27u32;
pub const PHONEBUTTONFUNCTION_LASTNUM: u32 = 35u32;
pub const PHONEBUTTONFUNCTION_MSGINDICATOR: u32 = 38u32;
pub const PHONEBUTTONFUNCTION_MSGWAITOFF: u32 = 9u32;
pub const PHONEBUTTONFUNCTION_MSGWAITON: u32 = 8u32;
pub const PHONEBUTTONFUNCTION_MUTE: u32 = 18u32;
pub const PHONEBUTTONFUNCTION_NIGHTSRV: u32 = 36u32;
pub const PHONEBUTTONFUNCTION_NONE: u32 = 46u32;
pub const PHONEBUTTONFUNCTION_PARK: u32 = 15u32;
pub const PHONEBUTTONFUNCTION_PICKUP: u32 = 13u32;
pub const PHONEBUTTONFUNCTION_QUEUECALL: u32 = 45u32;
pub const PHONEBUTTONFUNCTION_RECALL: u32 = 5u32;
pub const PHONEBUTTONFUNCTION_REDIRECT: u32 = 17u32;
pub const PHONEBUTTONFUNCTION_REJECT: u32 = 16u32;
pub const PHONEBUTTONFUNCTION_REPDIAL: u32 = 39u32;
pub const PHONEBUTTONFUNCTION_RINGAGAIN: u32 = 14u32;
pub const PHONEBUTTONFUNCTION_SAVEREPEAT: u32 = 44u32;
pub const PHONEBUTTONFUNCTION_SELECTRING: u32 = 10u32;
pub const PHONEBUTTONFUNCTION_SEND: u32 = 47u32;
pub const PHONEBUTTONFUNCTION_SENDCALLS: u32 = 37u32;
pub const PHONEBUTTONFUNCTION_SETREPDIAL: u32 = 40u32;
pub const PHONEBUTTONFUNCTION_SPEAKEROFF: u32 = 22u32;
pub const PHONEBUTTONFUNCTION_SPEAKERON: u32 = 21u32;
pub const PHONEBUTTONFUNCTION_STATIONSPEED: u32 = 42u32;
pub const PHONEBUTTONFUNCTION_SYSTEMSPEED: u32 = 41u32;
pub const PHONEBUTTONFUNCTION_TRANSFER: u32 = 2u32;
pub const PHONEBUTTONFUNCTION_UNKNOWN: u32 = 0u32;
pub const PHONEBUTTONFUNCTION_VOLUMEDOWN: u32 = 20u32;
pub const PHONEBUTTONFUNCTION_VOLUMEUP: u32 = 19u32;
pub const PHONEBUTTONMODE_CALL: u32 = 2u32;
pub const PHONEBUTTONMODE_DISPLAY: u32 = 32u32;
pub const PHONEBUTTONMODE_DUMMY: u32 = 1u32;
pub const PHONEBUTTONMODE_FEATURE: u32 = 4u32;
pub const PHONEBUTTONMODE_KEYPAD: u32 = 8u32;
pub const PHONEBUTTONMODE_LOCAL: u32 = 16u32;
pub const PHONEBUTTONSTATE_DOWN: u32 = 2u32;
pub const PHONEBUTTONSTATE_UNAVAIL: u32 = 8u32;
pub const PHONEBUTTONSTATE_UNKNOWN: u32 = 4u32;
pub const PHONEBUTTONSTATE_UP: u32 = 1u32;
pub const PHONEERR_ALLOCATED: u32 = 2415919105u32;
pub const PHONEERR_BADDEVICEID: u32 = 2415919106u32;
pub const PHONEERR_DISCONNECTED: u32 = 2415919140u32;
pub const PHONEERR_INCOMPATIBLEAPIVERSION: u32 = 2415919107u32;
pub const PHONEERR_INCOMPATIBLEEXTVERSION: u32 = 2415919108u32;
pub const PHONEERR_INIFILECORRUPT: u32 = 2415919109u32;
pub const PHONEERR_INUSE: u32 = 2415919110u32;
pub const PHONEERR_INVALAPPHANDLE: u32 = 2415919111u32;
pub const PHONEERR_INVALAPPNAME: u32 = 2415919112u32;
pub const PHONEERR_INVALBUTTONLAMPID: u32 = 2415919113u32;
pub const PHONEERR_INVALBUTTONMODE: u32 = 2415919114u32;
pub const PHONEERR_INVALBUTTONSTATE: u32 = 2415919115u32;
pub const PHONEERR_INVALDATAID: u32 = 2415919116u32;
pub const PHONEERR_INVALDEVICECLASS: u32 = 2415919117u32;
pub const PHONEERR_INVALEXTVERSION: u32 = 2415919118u32;
pub const PHONEERR_INVALHOOKSWITCHDEV: u32 = 2415919119u32;
pub const PHONEERR_INVALHOOKSWITCHMODE: u32 = 2415919120u32;
pub const PHONEERR_INVALLAMPMODE: u32 = 2415919121u32;
pub const PHONEERR_INVALPARAM: u32 = 2415919122u32;
pub const PHONEERR_INVALPHONEHANDLE: u32 = 2415919123u32;
pub const PHONEERR_INVALPHONESTATE: u32 = 2415919124u32;
pub const PHONEERR_INVALPOINTER: u32 = 2415919125u32;
pub const PHONEERR_INVALPRIVILEGE: u32 = 2415919126u32;
pub const PHONEERR_INVALRINGMODE: u32 = 2415919127u32;
pub const PHONEERR_NODEVICE: u32 = 2415919128u32;
pub const PHONEERR_NODRIVER: u32 = 2415919129u32;
pub const PHONEERR_NOMEM: u32 = 2415919130u32;
pub const PHONEERR_NOTOWNER: u32 = 2415919131u32;
pub const PHONEERR_OPERATIONFAILED: u32 = 2415919132u32;
pub const PHONEERR_OPERATIONUNAVAIL: u32 = 2415919133u32;
pub const PHONEERR_REINIT: u32 = 2415919139u32;
pub const PHONEERR_REQUESTOVERRUN: u32 = 2415919136u32;
pub const PHONEERR_RESOURCEUNAVAIL: u32 = 2415919135u32;
pub const PHONEERR_SERVICE_NOT_RUNNING: u32 = 2415919141u32;
pub const PHONEERR_STRUCTURETOOSMALL: u32 = 2415919137u32;
pub const PHONEERR_UNINITIALIZED: u32 = 2415919138u32;
pub const PHONEFEATURE_GENERICPHONE: u32 = 268435456u32;
pub const PHONEFEATURE_GETBUTTONINFO: u32 = 1u32;
pub const PHONEFEATURE_GETDATA: u32 = 2u32;
pub const PHONEFEATURE_GETDISPLAY: u32 = 4u32;
pub const PHONEFEATURE_GETGAINHANDSET: u32 = 8u32;
pub const PHONEFEATURE_GETGAINHEADSET: u32 = 32u32;
pub const PHONEFEATURE_GETGAINSPEAKER: u32 = 16u32;
pub const PHONEFEATURE_GETHOOKSWITCHHANDSET: u32 = 64u32;
pub const PHONEFEATURE_GETHOOKSWITCHHEADSET: u32 = 256u32;
pub const PHONEFEATURE_GETHOOKSWITCHSPEAKER: u32 = 128u32;
pub const PHONEFEATURE_GETLAMP: u32 = 512u32;
pub const PHONEFEATURE_GETRING: u32 = 1024u32;
pub const PHONEFEATURE_GETVOLUMEHANDSET: u32 = 2048u32;
pub const PHONEFEATURE_GETVOLUMEHEADSET: u32 = 8192u32;
pub const PHONEFEATURE_GETVOLUMESPEAKER: u32 = 4096u32;
pub const PHONEFEATURE_SETBUTTONINFO: u32 = 16384u32;
pub const PHONEFEATURE_SETDATA: u32 = 32768u32;
pub const PHONEFEATURE_SETDISPLAY: u32 = 65536u32;
pub const PHONEFEATURE_SETGAINHANDSET: u32 = 131072u32;
pub const PHONEFEATURE_SETGAINHEADSET: u32 = 524288u32;
pub const PHONEFEATURE_SETGAINSPEAKER: u32 = 262144u32;
pub const PHONEFEATURE_SETHOOKSWITCHHANDSET: u32 = 1048576u32;
pub const PHONEFEATURE_SETHOOKSWITCHHEADSET: u32 = 4194304u32;
pub const PHONEFEATURE_SETHOOKSWITCHSPEAKER: u32 = 2097152u32;
pub const PHONEFEATURE_SETLAMP: u32 = 8388608u32;
pub const PHONEFEATURE_SETRING: u32 = 16777216u32;
pub const PHONEFEATURE_SETVOLUMEHANDSET: u32 = 33554432u32;
pub const PHONEFEATURE_SETVOLUMEHEADSET: u32 = 134217728u32;
pub const PHONEFEATURE_SETVOLUMESPEAKER: u32 = 67108864u32;
pub const PHONEHOOKSWITCHDEV_HANDSET: u32 = 1u32;
pub const PHONEHOOKSWITCHDEV_HEADSET: u32 = 4u32;
pub const PHONEHOOKSWITCHDEV_SPEAKER: u32 = 2u32;
pub const PHONEHOOKSWITCHMODE_MIC: u32 = 2u32;
pub const PHONEHOOKSWITCHMODE_MICSPEAKER: u32 = 8u32;
pub const PHONEHOOKSWITCHMODE_ONHOOK: u32 = 1u32;
pub const PHONEHOOKSWITCHMODE_SPEAKER: u32 = 4u32;
pub const PHONEHOOKSWITCHMODE_UNKNOWN: u32 = 16u32;
pub const PHONEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
pub const PHONEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
pub const PHONEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
pub const PHONELAMPMODE_BROKENFLUTTER: u32 = 64u32;
pub const PHONELAMPMODE_DUMMY: u32 = 1u32;
pub const PHONELAMPMODE_FLASH: u32 = 16u32;
pub const PHONELAMPMODE_FLUTTER: u32 = 32u32;
pub const PHONELAMPMODE_OFF: u32 = 2u32;
pub const PHONELAMPMODE_STEADY: u32 = 4u32;
pub const PHONELAMPMODE_UNKNOWN: u32 = 128u32;
pub const PHONELAMPMODE_WINK: u32 = 8u32;
pub const PHONEPRIVILEGE_MONITOR: u32 = 1u32;
pub const PHONEPRIVILEGE_OWNER: u32 = 2u32;
pub const PHONESTATE_CAPSCHANGE: u32 = 4194304u32;
pub const PHONESTATE_CONNECTED: u32 = 2u32;
pub const PHONESTATE_DEVSPECIFIC: u32 = 1048576u32;
pub const PHONESTATE_DISCONNECTED: u32 = 4u32;
pub const PHONESTATE_DISPLAY: u32 = 32u32;
pub const PHONESTATE_HANDSETGAIN: u32 = 2048u32;
pub const PHONESTATE_HANDSETHOOKSWITCH: u32 = 512u32;
pub const PHONESTATE_HANDSETVOLUME: u32 = 1024u32;
pub const PHONESTATE_HEADSETGAIN: u32 = 131072u32;
pub const PHONESTATE_HEADSETHOOKSWITCH: u32 = 32768u32;
pub const PHONESTATE_HEADSETVOLUME: u32 = 65536u32;
pub const PHONESTATE_LAMP: u32 = 64u32;
pub const PHONESTATE_MONITORS: u32 = 16u32;
pub const PHONESTATE_OTHER: u32 = 1u32;
pub const PHONESTATE_OWNER: u32 = 8u32;
pub const PHONESTATE_REINIT: u32 = 2097152u32;
pub const PHONESTATE_REMOVED: u32 = 8388608u32;
pub const PHONESTATE_RESUME: u32 = 524288u32;
pub const PHONESTATE_RINGMODE: u32 = 128u32;
pub const PHONESTATE_RINGVOLUME: u32 = 256u32;
pub const PHONESTATE_SPEAKERGAIN: u32 = 16384u32;
pub const PHONESTATE_SPEAKERHOOKSWITCH: u32 = 4096u32;
pub const PHONESTATE_SPEAKERVOLUME: u32 = 8192u32;
pub const PHONESTATE_SUSPEND: u32 = 262144u32;
pub const PHONESTATUSFLAGS_CONNECTED: u32 = 1u32;
pub const PHONESTATUSFLAGS_SUSPENDED: u32 = 2u32;
pub const PHONE_BUTTON: i32 = 14i32;
pub const PHONE_CLOSE: i32 = 15i32;
pub const PHONE_CREATE: i32 = 20i32;
pub const PHONE_DEVSPECIFIC: i32 = 16i32;
pub const PHONE_REMOVE: i32 = 26i32;
pub const PHONE_REPLY: i32 = 17i32;
pub const PHONE_STATE: i32 = 18i32;
pub const PHSD_HANDSET: PHONE_HOOK_SWITCH_DEVICE = PHONE_HOOK_SWITCH_DEVICE(1i32);
pub const PHSD_HEADSET: PHONE_HOOK_SWITCH_DEVICE = PHONE_HOOK_SWITCH_DEVICE(4i32);
pub const PHSD_SPEAKERPHONE: PHONE_HOOK_SWITCH_DEVICE = PHONE_HOOK_SWITCH_DEVICE(2i32);
pub const PHSS_OFFHOOK: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(8i32);
pub const PHSS_OFFHOOK_MIC_ONLY: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(2i32);
pub const PHSS_OFFHOOK_SPEAKER_ONLY: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(4i32);
pub const PHSS_ONHOOK: PHONE_HOOK_SWITCH_STATE = PHONE_HOOK_SWITCH_STATE(1i32);
pub const PP_MONITOR: PHONE_PRIVILEGE = PHONE_PRIVILEGE(1i32);
pub const PP_OWNER: PHONE_PRIVILEGE = PHONE_PRIVILEGE(0i32);
pub const PRIVATEOBJECT_ADDRESS: u32 = 6u32;
pub const PRIVATEOBJECT_CALL: u32 = 4u32;
pub const PRIVATEOBJECT_CALLID: u32 = 2u32;
pub const PRIVATEOBJECT_LINE: u32 = 3u32;
pub const PRIVATEOBJECT_NONE: u32 = 1u32;
pub const PRIVATEOBJECT_PHONE: u32 = 5u32;
pub const PT_BUSY: PHONE_TONE = PHONE_TONE(18i32);
pub const PT_ERRORTONE: PHONE_TONE = PHONE_TONE(20i32);
pub const PT_EXTERNALDIALTONE: PHONE_TONE = PHONE_TONE(17i32);
pub const PT_KEYPADA: PHONE_TONE = PHONE_TONE(12i32);
pub const PT_KEYPADB: PHONE_TONE = PHONE_TONE(13i32);
pub const PT_KEYPADC: PHONE_TONE = PHONE_TONE(14i32);
pub const PT_KEYPADD: PHONE_TONE = PHONE_TONE(15i32);
pub const PT_KEYPADEIGHT: PHONE_TONE = PHONE_TONE(8i32);
pub const PT_KEYPADFIVE: PHONE_TONE = PHONE_TONE(5i32);
pub const PT_KEYPADFOUR: PHONE_TONE = PHONE_TONE(4i32);
pub const PT_KEYPADNINE: PHONE_TONE = PHONE_TONE(9i32);
pub const PT_KEYPADONE: PHONE_TONE = PHONE_TONE(1i32);
pub const PT_KEYPADPOUND: PHONE_TONE = PHONE_TONE(11i32);
pub const PT_KEYPADSEVEN: PHONE_TONE = PHONE_TONE(7i32);
pub const PT_KEYPADSIX: PHONE_TONE = PHONE_TONE(6i32);
pub const PT_KEYPADSTAR: PHONE_TONE = PHONE_TONE(10i32);
pub const PT_KEYPADTHREE: PHONE_TONE = PHONE_TONE(3i32);
pub const PT_KEYPADTWO: PHONE_TONE = PHONE_TONE(2i32);
pub const PT_KEYPADZERO: PHONE_TONE = PHONE_TONE(0i32);
pub const PT_NORMALDIALTONE: PHONE_TONE = PHONE_TONE(16i32);
pub const PT_RINGBACK: PHONE_TONE = PHONE_TONE(19i32);
pub const PT_SILENCE: PHONE_TONE = PHONE_TONE(21i32);
pub const QE_ADMISSIONFAILURE: QOS_EVENT = QOS_EVENT(2i32);
pub const QE_GENERICERROR: QOS_EVENT = QOS_EVENT(4i32);
pub const QE_LASTITEM: QOS_EVENT = QOS_EVENT(4i32);
pub const QE_NOQOS: QOS_EVENT = QOS_EVENT(1i32);
pub const QE_POLICYFAILURE: QOS_EVENT = QOS_EVENT(3i32);
pub const QSL_BEST_EFFORT: QOS_SERVICE_LEVEL = QOS_SERVICE_LEVEL(3i32);
pub const QSL_IF_AVAILABLE: QOS_SERVICE_LEVEL = QOS_SERVICE_LEVEL(2i32);
pub const QSL_NEEDED: QOS_SERVICE_LEVEL = QOS_SERVICE_LEVEL(1i32);
pub const RAS_LOCAL: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(1i32);
pub const RAS_REGION: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(3i32);
pub const RAS_SITE: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(2i32);
pub const RAS_WORLD: RND_ADVERTISING_SCOPE = RND_ADVERTISING_SCOPE(4i32);
pub const RENDBIND_AUTHENTICATE: u32 = 1u32;
pub const RENDBIND_DEFAULTCREDENTIALS: u32 = 14u32;
pub const RENDBIND_DEFAULTDOMAINNAME: u32 = 2u32;
pub const RENDBIND_DEFAULTPASSWORD: u32 = 8u32;
pub const RENDBIND_DEFAULTUSERNAME: u32 = 4u32;
pub const STRINGFORMAT_ASCII: u32 = 1u32;
pub const STRINGFORMAT_BINARY: u32 = 4u32;
pub const STRINGFORMAT_DBCS: u32 = 2u32;
pub const STRINGFORMAT_UNICODE: u32 = 3u32;
pub const STRM_CONFIGURED: u32 = 2u32;
pub const STRM_INITIAL: u32 = 0u32;
pub const STRM_PAUSED: u32 = 8u32;
pub const STRM_RUNNING: u32 = 4u32;
pub const STRM_STOPPED: u32 = 16u32;
pub const STRM_TERMINALSELECTED: u32 = 1u32;
pub const TAPIERR_CONNECTED: i32 = 0i32;
pub const TAPIERR_DESTBUSY: i32 = -11i32;
pub const TAPIERR_DESTNOANSWER: i32 = -12i32;
pub const TAPIERR_DESTUNAVAIL: i32 = -13i32;
pub const TAPIERR_DEVICECLASSUNAVAIL: i32 = -8i32;
pub const TAPIERR_DEVICEIDUNAVAIL: i32 = -9i32;
pub const TAPIERR_DEVICEINUSE: i32 = -10i32;
pub const TAPIERR_DROPPED: i32 = -1i32;
pub const TAPIERR_INVALDESTADDRESS: i32 = -4i32;
pub const TAPIERR_INVALDEVICECLASS: i32 = -6i32;
pub const TAPIERR_INVALDEVICEID: i32 = -7i32;
pub const TAPIERR_INVALPOINTER: i32 = -18i32;
pub const TAPIERR_INVALWINDOWHANDLE: i32 = -5i32;
pub const TAPIERR_MMCWRITELOCKED: i32 = -20i32;
pub const TAPIERR_NOREQUESTRECIPIENT: i32 = -2i32;
pub const TAPIERR_NOTADMIN: i32 = -19i32;
pub const TAPIERR_PROVIDERALREADYINSTALLED: i32 = -21i32;
pub const TAPIERR_REQUESTCANCELLED: i32 = -17i32;
pub const TAPIERR_REQUESTFAILED: i32 = -16i32;
pub const TAPIERR_REQUESTQUEUEFULL: i32 = -3i32;
pub const TAPIERR_SCP_ALREADY_EXISTS: i32 = -22i32;
pub const TAPIERR_SCP_DOES_NOT_EXIST: i32 = -23i32;
pub const TAPIERR_UNKNOWNREQUESTID: i32 = -15i32;
pub const TAPIERR_UNKNOWNWINHANDLE: i32 = -14i32;
pub const TAPIMAXAPPNAMESIZE: i32 = 40i32;
pub const TAPIMAXCALLEDPARTYSIZE: i32 = 40i32;
pub const TAPIMAXCOMMENTSIZE: i32 = 80i32;
pub const TAPIMAXDESTADDRESSSIZE: i32 = 80i32;
pub const TAPIMAXDEVICECLASSSIZE: i32 = 40i32;
pub const TAPIMAXDEVICEIDSIZE: i32 = 40i32;
pub const TAPIMEDIATYPE_AUDIO: u32 = 8u32;
pub const TAPIMEDIATYPE_DATAMODEM: u32 = 16u32;
pub const TAPIMEDIATYPE_G3FAX: u32 = 32u32;
pub const TAPIMEDIATYPE_MULTITRACK: u32 = 65536u32;
pub const TAPIMEDIATYPE_VIDEO: u32 = 32768u32;
pub const TAPI_CURRENT_VERSION: u32 = 131074u32;
pub const TAPI_E_ADDRESSBLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8004002A_u32 as _);
pub const TAPI_E_ALLOCATED: windows_core::HRESULT = windows_core::HRESULT(0x80040006_u32 as _);
pub const TAPI_E_BILLINGREJECTED: windows_core::HRESULT = windows_core::HRESULT(0x8004002B_u32 as _);
pub const TAPI_E_CALLCENTER_GROUP_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x80040045_u32 as _);
pub const TAPI_E_CALLCENTER_INVALAGENTACTIVITY: windows_core::HRESULT = windows_core::HRESULT(0x8004004C_u32 as _);
pub const TAPI_E_CALLCENTER_INVALAGENTGROUP: windows_core::HRESULT = windows_core::HRESULT(0x80040049_u32 as _);
pub const TAPI_E_CALLCENTER_INVALAGENTID: windows_core::HRESULT = windows_core::HRESULT(0x80040048_u32 as _);
pub const TAPI_E_CALLCENTER_INVALAGENTSTATE: windows_core::HRESULT = windows_core::HRESULT(0x8004004B_u32 as _);
pub const TAPI_E_CALLCENTER_INVALPASSWORD: windows_core::HRESULT = windows_core::HRESULT(0x8004004A_u32 as _);
pub const TAPI_E_CALLCENTER_NO_AGENT_ID: windows_core::HRESULT = windows_core::HRESULT(0x80040047_u32 as _);
pub const TAPI_E_CALLCENTER_QUEUE_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x80040046_u32 as _);
pub const TAPI_E_CALLNOTSELECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040054_u32 as _);
pub const TAPI_E_CALLUNAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x80040007_u32 as _);
pub const TAPI_E_COMPLETIONOVERRUN: windows_core::HRESULT = windows_core::HRESULT(0x80040008_u32 as _);
pub const TAPI_E_CONFERENCEFULL: windows_core::HRESULT = windows_core::HRESULT(0x80040009_u32 as _);
pub const TAPI_E_DESTBUSY: windows_core::HRESULT = windows_core::HRESULT(0x80040034_u32 as _);
pub const TAPI_E_DESTNOANSWER: windows_core::HRESULT = windows_core::HRESULT(0x80040035_u32 as _);
pub const TAPI_E_DESTUNAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x80040036_u32 as _);
pub const TAPI_E_DIALMODIFIERNOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004000A_u32 as _);
pub const TAPI_E_DROPPED: windows_core::HRESULT = windows_core::HRESULT(0x80040031_u32 as _);
pub const TAPI_E_INUSE: windows_core::HRESULT = windows_core::HRESULT(0x8004000B_u32 as _);
pub const TAPI_E_INVALADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004000C_u32 as _);
pub const TAPI_E_INVALADDRESSSTATE: windows_core::HRESULT = windows_core::HRESULT(0x8004000D_u32 as _);
pub const TAPI_E_INVALADDRESSTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040051_u32 as _);
pub const TAPI_E_INVALBUTTONLAMPID: windows_core::HRESULT = windows_core::HRESULT(0x8004002D_u32 as _);
pub const TAPI_E_INVALBUTTONSTATE: windows_core::HRESULT = windows_core::HRESULT(0x8004002E_u32 as _);
pub const TAPI_E_INVALCALLPARAMS: windows_core::HRESULT = windows_core::HRESULT(0x8004000E_u32 as _);
pub const TAPI_E_INVALCALLPRIVILEGE: windows_core::HRESULT = windows_core::HRESULT(0x8004000F_u32 as _);
pub const TAPI_E_INVALCALLSTATE: windows_core::HRESULT = windows_core::HRESULT(0x80040010_u32 as _);
pub const TAPI_E_INVALCARD: windows_core::HRESULT = windows_core::HRESULT(0x80040011_u32 as _);
pub const TAPI_E_INVALCOMPLETIONID: windows_core::HRESULT = windows_core::HRESULT(0x80040012_u32 as _);
pub const TAPI_E_INVALCOUNTRYCODE: windows_core::HRESULT = windows_core::HRESULT(0x80040013_u32 as _);
pub const TAPI_E_INVALDATAID: windows_core::HRESULT = windows_core::HRESULT(0x8004002F_u32 as _);
pub const TAPI_E_INVALDEVICECLASS: windows_core::HRESULT = windows_core::HRESULT(0x80040014_u32 as _);
pub const TAPI_E_INVALDIALPARAMS: windows_core::HRESULT = windows_core::HRESULT(0x80040015_u32 as _);
pub const TAPI_E_INVALDIGITS: windows_core::HRESULT = windows_core::HRESULT(0x80040016_u32 as _);
pub const TAPI_E_INVALFEATURE: windows_core::HRESULT = windows_core::HRESULT(0x8004002C_u32 as _);
pub const TAPI_E_INVALGROUPID: windows_core::HRESULT = windows_core::HRESULT(0x80040017_u32 as _);
pub const TAPI_E_INVALHOOKSWITCHDEV: windows_core::HRESULT = windows_core::HRESULT(0x80040030_u32 as _);
pub const TAPI_E_INVALIDDIRECTION: windows_core::HRESULT = windows_core::HRESULT(0x8004003A_u32 as _);
pub const TAPI_E_INVALIDMEDIATYPE: windows_core::HRESULT = windows_core::HRESULT(0x80040004_u32 as _);
pub const TAPI_E_INVALIDSTREAM: windows_core::HRESULT = windows_core::HRESULT(0x80040043_u32 as _);
pub const TAPI_E_INVALIDSTREAMSTATE: windows_core::HRESULT = windows_core::HRESULT(0x80040057_u32 as _);
pub const TAPI_E_INVALIDTERMINAL: windows_core::HRESULT = windows_core::HRESULT(0x8004003B_u32 as _);
pub const TAPI_E_INVALIDTERMINALCLASS: windows_core::HRESULT = windows_core::HRESULT(0x8004003C_u32 as _);
pub const TAPI_E_INVALLIST: windows_core::HRESULT = windows_core::HRESULT(0x8004001E_u32 as _);
pub const TAPI_E_INVALLOCATION: windows_core::HRESULT = windows_core::HRESULT(0x80040018_u32 as _);
pub const TAPI_E_INVALMESSAGEID: windows_core::HRESULT = windows_core::HRESULT(0x80040019_u32 as _);
pub const TAPI_E_INVALMODE: windows_core::HRESULT = windows_core::HRESULT(0x8004001F_u32 as _);
pub const TAPI_E_INVALPARKID: windows_core::HRESULT = windows_core::HRESULT(0x8004001A_u32 as _);
pub const TAPI_E_INVALPRIVILEGE: windows_core::HRESULT = windows_core::HRESULT(0x80040039_u32 as _);
pub const TAPI_E_INVALRATE: windows_core::HRESULT = windows_core::HRESULT(0x8004001B_u32 as _);
pub const TAPI_E_INVALTIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004001C_u32 as _);
pub const TAPI_E_INVALTONE: windows_core::HRESULT = windows_core::HRESULT(0x8004001D_u32 as _);
pub const TAPI_E_MAXSTREAMS: windows_core::HRESULT = windows_core::HRESULT(0x8004003E_u32 as _);
pub const TAPI_E_MAXTERMINALS: windows_core::HRESULT = windows_core::HRESULT(0x80040042_u32 as _);
pub const TAPI_E_NOCONFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x80040020_u32 as _);
pub const TAPI_E_NODEVICE: windows_core::HRESULT = windows_core::HRESULT(0x80040021_u32 as _);
pub const TAPI_E_NODRIVER: windows_core::HRESULT = windows_core::HRESULT(0x8004003D_u32 as _);
pub const TAPI_E_NOEVENT: windows_core::HRESULT = windows_core::HRESULT(0x80040050_u32 as _);
pub const TAPI_E_NOFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80040056_u32 as _);
pub const TAPI_E_NOITEMS: windows_core::HRESULT = windows_core::HRESULT(0x80040002_u32 as _);
pub const TAPI_E_NOREQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80040022_u32 as _);
pub const TAPI_E_NOREQUESTRECIPIENT: windows_core::HRESULT = windows_core::HRESULT(0x80040032_u32 as _);
pub const TAPI_E_NOTENOUGHMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80040001_u32 as _);
pub const TAPI_E_NOTERMINALSELECTED: windows_core::HRESULT = windows_core::HRESULT(0x8004003F_u32 as _);
pub const TAPI_E_NOTOWNER: windows_core::HRESULT = windows_core::HRESULT(0x80040023_u32 as _);
pub const TAPI_E_NOTREGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80040024_u32 as _);
pub const TAPI_E_NOTSTOPPED: windows_core::HRESULT = windows_core::HRESULT(0x80040041_u32 as _);
pub const TAPI_E_NOTSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040003_u32 as _);
pub const TAPI_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80040059_u32 as _);
pub const TAPI_E_OPERATIONFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80040005_u32 as _);
pub const TAPI_E_PEER_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x8004004F_u32 as _);
pub const TAPI_E_PHONENOTOPEN: windows_core::HRESULT = windows_core::HRESULT(0x80040053_u32 as _);
pub const TAPI_E_REGISTRY_SETTING_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x8004004D_u32 as _);
pub const TAPI_E_REINIT: windows_core::HRESULT = windows_core::HRESULT(0x80040029_u32 as _);
pub const TAPI_E_REQUESTCANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80040038_u32 as _);
pub const TAPI_E_REQUESTFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80040037_u32 as _);
pub const TAPI_E_REQUESTOVERRUN: windows_core::HRESULT = windows_core::HRESULT(0x80040025_u32 as _);
pub const TAPI_E_REQUESTQUEUEFULL: windows_core::HRESULT = windows_core::HRESULT(0x80040033_u32 as _);
pub const TAPI_E_RESOURCEUNAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x80040052_u32 as _);
pub const TAPI_E_SERVICE_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004005A_u32 as _);
pub const TAPI_E_TARGETNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040026_u32 as _);
pub const TAPI_E_TARGETSELF: windows_core::HRESULT = windows_core::HRESULT(0x80040027_u32 as _);
pub const TAPI_E_TERMINALINUSE: windows_core::HRESULT = windows_core::HRESULT(0x80040040_u32 as _);
pub const TAPI_E_TERMINAL_PEER: windows_core::HRESULT = windows_core::HRESULT(0x8004004E_u32 as _);
pub const TAPI_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80040044_u32 as _);
pub const TAPI_E_USERUSERINFOTOOBIG: windows_core::HRESULT = windows_core::HRESULT(0x80040028_u32 as _);
pub const TAPI_E_WRONGEVENT: windows_core::HRESULT = windows_core::HRESULT(0x80040055_u32 as _);
pub const TAPI_E_WRONG_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80040058_u32 as _);
pub const TAPI_REPLY: u32 = 1123u32;
pub const TD_BIDIRECTIONAL: TERMINAL_DIRECTION = TERMINAL_DIRECTION(2i32);
pub const TD_CAPTURE: TERMINAL_DIRECTION = TERMINAL_DIRECTION(0i32);
pub const TD_MULTITRACK_MIXED: TERMINAL_DIRECTION = TERMINAL_DIRECTION(3i32);
pub const TD_NONE: TERMINAL_DIRECTION = TERMINAL_DIRECTION(4i32);
pub const TD_RENDER: TERMINAL_DIRECTION = TERMINAL_DIRECTION(1i32);
pub const TE_ACDGROUP: TAPI_EVENT = TAPI_EVENT(8192i32);
pub const TE_ADDRESS: TAPI_EVENT = TAPI_EVENT(2i32);
pub const TE_ADDRESSCLOSE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(4i32);
pub const TE_ADDRESSCREATE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(0i32);
pub const TE_ADDRESSDEVSPECIFIC: TAPI_EVENT = TAPI_EVENT(16777216i32);
pub const TE_ADDRESSREMOVE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(1i32);
pub const TE_AGENT: TAPI_EVENT = TAPI_EVENT(512i32);
pub const TE_AGENTHANDLER: TAPI_EVENT = TAPI_EVENT(4096i32);
pub const TE_AGENTSESSION: TAPI_EVENT = TAPI_EVENT(1024i32);
pub const TE_ASRTERMINAL: TAPI_EVENT = TAPI_EVENT(131072i32);
pub const TE_CALLHUB: TAPI_EVENT = TAPI_EVENT(32i32);
pub const TE_CALLINFOCHANGE: TAPI_EVENT = TAPI_EVENT(64i32);
pub const TE_CALLMEDIA: TAPI_EVENT = TAPI_EVENT(16i32);
pub const TE_CALLNOTIFICATION: TAPI_EVENT = TAPI_EVENT(4i32);
pub const TE_CALLSTATE: TAPI_EVENT = TAPI_EVENT(8i32);
pub const TE_DIGITEVENT: TAPI_EVENT = TAPI_EVENT(32768i32);
pub const TE_FILETERMINAL: TAPI_EVENT = TAPI_EVENT(524288i32);
pub const TE_GATHERDIGITS: TAPI_EVENT = TAPI_EVENT(8388608i32);
pub const TE_GENERATEEVENT: TAPI_EVENT = TAPI_EVENT(65536i32);
pub const TE_PHONECREATE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(5i32);
pub const TE_PHONEDEVSPECIFIC: TAPI_EVENT = TAPI_EVENT(33554432i32);
pub const TE_PHONEEVENT: TAPI_EVENT = TAPI_EVENT(2097152i32);
pub const TE_PHONEREMOVE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(6i32);
pub const TE_PRIVATE: TAPI_EVENT = TAPI_EVENT(128i32);
pub const TE_QOSEVENT: TAPI_EVENT = TAPI_EVENT(2048i32);
pub const TE_QUEUE: TAPI_EVENT = TAPI_EVENT(16384i32);
pub const TE_REINIT: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(2i32);
pub const TE_REQUEST: TAPI_EVENT = TAPI_EVENT(256i32);
pub const TE_TAPIOBJECT: TAPI_EVENT = TAPI_EVENT(1i32);
pub const TE_TONEEVENT: TAPI_EVENT = TAPI_EVENT(4194304i32);
pub const TE_TONETERMINAL: TAPI_EVENT = TAPI_EVENT(1048576i32);
pub const TE_TRANSLATECHANGE: TAPIOBJECT_EVENT = TAPIOBJECT_EVENT(3i32);
pub const TE_TTSTERMINAL: TAPI_EVENT = TAPI_EVENT(262144i32);
pub const TGT_BUFFERFULL: TAPI_GATHERTERM = TAPI_GATHERTERM(1i32);
pub const TGT_CANCEL: TAPI_GATHERTERM = TAPI_GATHERTERM(16i32);
pub const TGT_FIRSTTIMEOUT: TAPI_GATHERTERM = TAPI_GATHERTERM(4i32);
pub const TGT_INTERTIMEOUT: TAPI_GATHERTERM = TAPI_GATHERTERM(8i32);
pub const TGT_TERMDIGIT: TAPI_GATHERTERM = TAPI_GATHERTERM(2i32);
pub const TMS_ACTIVE: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(1i32);
pub const TMS_IDLE: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(0i32);
pub const TMS_LASTITEM: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(2i32);
pub const TMS_PAUSED: TERMINAL_MEDIA_STATE = TERMINAL_MEDIA_STATE(2i32);
pub const TOT_ADDRESS: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(2i32);
pub const TOT_CALL: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(4i32);
pub const TOT_CALLHUB: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(5i32);
pub const TOT_NONE: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(0i32);
pub const TOT_PHONE: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(6i32);
pub const TOT_TAPI: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(1i32);
pub const TOT_TERMINAL: TAPI_OBJECT_TYPE = TAPI_OBJECT_TYPE(3i32);
pub const TSPI_LINEACCEPT: u32 = 500u32;
pub const TSPI_LINEADDTOCONFERENCE: u32 = 501u32;
pub const TSPI_LINEANSWER: u32 = 502u32;
pub const TSPI_LINEBLINDTRANSFER: u32 = 503u32;
pub const TSPI_LINECLOSE: u32 = 504u32;
pub const TSPI_LINECLOSECALL: u32 = 505u32;
pub const TSPI_LINECLOSEMSPINSTANCE: u32 = 609u32;
pub const TSPI_LINECOMPLETECALL: u32 = 506u32;
pub const TSPI_LINECOMPLETETRANSFER: u32 = 507u32;
pub const TSPI_LINECONDITIONALMEDIADETECTION: u32 = 508u32;
pub const TSPI_LINECONFIGDIALOG: u32 = 509u32;
pub const TSPI_LINECONFIGDIALOGEDIT: u32 = 601u32;
pub const TSPI_LINECREATEMSPINSTANCE: u32 = 608u32;
pub const TSPI_LINEDEVSPECIFIC: u32 = 510u32;
pub const TSPI_LINEDEVSPECIFICFEATURE: u32 = 511u32;
pub const TSPI_LINEDIAL: u32 = 512u32;
pub const TSPI_LINEDROP: u32 = 513u32;
pub const TSPI_LINEDROPNOOWNER: u32 = 597u32;
pub const TSPI_LINEDROPONCLOSE: u32 = 596u32;
pub const TSPI_LINEFORWARD: u32 = 514u32;
pub const TSPI_LINEGATHERDIGITS: u32 = 515u32;
pub const TSPI_LINEGENERATEDIGITS: u32 = 516u32;
pub const TSPI_LINEGENERATETONE: u32 = 517u32;
pub const TSPI_LINEGETADDRESSCAPS: u32 = 518u32;
pub const TSPI_LINEGETADDRESSID: u32 = 519u32;
pub const TSPI_LINEGETADDRESSSTATUS: u32 = 520u32;
pub const TSPI_LINEGETCALLADDRESSID: u32 = 521u32;
pub const TSPI_LINEGETCALLHUBTRACKING: u32 = 604u32;
pub const TSPI_LINEGETCALLID: u32 = 603u32;
pub const TSPI_LINEGETCALLINFO: u32 = 522u32;
pub const TSPI_LINEGETCALLSTATUS: u32 = 523u32;
pub const TSPI_LINEGETDEVCAPS: u32 = 524u32;
pub const TSPI_LINEGETDEVCONFIG: u32 = 525u32;
pub const TSPI_LINEGETEXTENSIONID: u32 = 526u32;
pub const TSPI_LINEGETICON: u32 = 527u32;
pub const TSPI_LINEGETID: u32 = 528u32;
pub const TSPI_LINEGETLINEDEVSTATUS: u32 = 529u32;
pub const TSPI_LINEGETNUMADDRESSIDS: u32 = 530u32;
pub const TSPI_LINEHOLD: u32 = 531u32;
pub const TSPI_LINEMAKECALL: u32 = 532u32;
pub const TSPI_LINEMONITORDIGITS: u32 = 533u32;
pub const TSPI_LINEMONITORMEDIA: u32 = 534u32;
pub const TSPI_LINEMONITORTONES: u32 = 535u32;
pub const TSPI_LINEMSPIDENTIFY: u32 = 607u32;
pub const TSPI_LINENEGOTIATEEXTVERSION: u32 = 536u32;
pub const TSPI_LINENEGOTIATETSPIVERSION: u32 = 537u32;
pub const TSPI_LINEOPEN: u32 = 538u32;
pub const TSPI_LINEPARK: u32 = 539u32;
pub const TSPI_LINEPICKUP: u32 = 540u32;
pub const TSPI_LINEPREPAREADDTOCONFERENCE: u32 = 541u32;
pub const TSPI_LINERECEIVEMSPDATA: u32 = 606u32;
pub const TSPI_LINEREDIRECT: u32 = 542u32;
pub const TSPI_LINERELEASEUSERUSERINFO: u32 = 602u32;
pub const TSPI_LINEREMOVEFROMCONFERENCE: u32 = 543u32;
pub const TSPI_LINESECURECALL: u32 = 544u32;
pub const TSPI_LINESELECTEXTVERSION: u32 = 545u32;
pub const TSPI_LINESENDUSERUSERINFO: u32 = 546u32;
pub const TSPI_LINESETAPPSPECIFIC: u32 = 547u32;
pub const TSPI_LINESETCALLHUBTRACKING: u32 = 605u32;
pub const TSPI_LINESETCALLPARAMS: u32 = 548u32;
pub const TSPI_LINESETCURRENTLOCATION: u32 = 600u32;
pub const TSPI_LINESETDEFAULTMEDIADETECTION: u32 = 549u32;
pub const TSPI_LINESETDEVCONFIG: u32 = 550u32;
pub const TSPI_LINESETMEDIACONTROL: u32 = 551u32;
pub const TSPI_LINESETMEDIAMODE: u32 = 552u32;
pub const TSPI_LINESETSTATUSMESSAGES: u32 = 553u32;
pub const TSPI_LINESETTERMINAL: u32 = 554u32;
pub const TSPI_LINESETUPCONFERENCE: u32 = 555u32;
pub const TSPI_LINESETUPTRANSFER: u32 = 556u32;
pub const TSPI_LINESWAPHOLD: u32 = 557u32;
pub const TSPI_LINEUNCOMPLETECALL: u32 = 558u32;
pub const TSPI_LINEUNHOLD: u32 = 559u32;
pub const TSPI_LINEUNPARK: u32 = 560u32;
pub const TSPI_MESSAGE_BASE: u32 = 500u32;
pub const TSPI_PHONECLOSE: u32 = 561u32;
pub const TSPI_PHONECONFIGDIALOG: u32 = 562u32;
pub const TSPI_PHONEDEVSPECIFIC: u32 = 563u32;
pub const TSPI_PHONEGETBUTTONINFO: u32 = 564u32;
pub const TSPI_PHONEGETDATA: u32 = 565u32;
pub const TSPI_PHONEGETDEVCAPS: u32 = 566u32;
pub const TSPI_PHONEGETDISPLAY: u32 = 567u32;
pub const TSPI_PHONEGETEXTENSIONID: u32 = 568u32;
pub const TSPI_PHONEGETGAIN: u32 = 569u32;
pub const TSPI_PHONEGETHOOKSWITCH: u32 = 570u32;
pub const TSPI_PHONEGETICON: u32 = 571u32;
pub const TSPI_PHONEGETID: u32 = 572u32;
pub const TSPI_PHONEGETLAMP: u32 = 573u32;
pub const TSPI_PHONEGETRING: u32 = 574u32;
pub const TSPI_PHONEGETSTATUS: u32 = 575u32;
pub const TSPI_PHONEGETVOLUME: u32 = 576u32;
pub const TSPI_PHONENEGOTIATEEXTVERSION: u32 = 577u32;
pub const TSPI_PHONENEGOTIATETSPIVERSION: u32 = 578u32;
pub const TSPI_PHONEOPEN: u32 = 579u32;
pub const TSPI_PHONESELECTEXTVERSION: u32 = 580u32;
pub const TSPI_PHONESETBUTTONINFO: u32 = 581u32;
pub const TSPI_PHONESETDATA: u32 = 582u32;
pub const TSPI_PHONESETDISPLAY: u32 = 583u32;
pub const TSPI_PHONESETGAIN: u32 = 584u32;
pub const TSPI_PHONESETHOOKSWITCH: u32 = 585u32;
pub const TSPI_PHONESETLAMP: u32 = 586u32;
pub const TSPI_PHONESETRING: u32 = 587u32;
pub const TSPI_PHONESETSTATUSMESSAGES: u32 = 588u32;
pub const TSPI_PHONESETVOLUME: u32 = 589u32;
pub const TSPI_PROC_BASE: u32 = 500u32;
pub const TSPI_PROVIDERCONFIG: u32 = 590u32;
pub const TSPI_PROVIDERCREATELINEDEVICE: u32 = 598u32;
pub const TSPI_PROVIDERCREATEPHONEDEVICE: u32 = 599u32;
pub const TSPI_PROVIDERENUMDEVICES: u32 = 595u32;
pub const TSPI_PROVIDERINIT: u32 = 591u32;
pub const TSPI_PROVIDERINSTALL: u32 = 592u32;
pub const TSPI_PROVIDERREMOVE: u32 = 593u32;
pub const TSPI_PROVIDERSHUTDOWN: u32 = 594u32;
pub const TS_INUSE: TERMINAL_STATE = TERMINAL_STATE(0i32);
pub const TS_NOTINUSE: TERMINAL_STATE = TERMINAL_STATE(1i32);
pub const TTM_BEEP: TAPI_TONEMODE = TAPI_TONEMODE(8i32);
pub const TTM_BILLING: TAPI_TONEMODE = TAPI_TONEMODE(16i32);
pub const TTM_BUSY: TAPI_TONEMODE = TAPI_TONEMODE(4i32);
pub const TTM_RINGBACK: TAPI_TONEMODE = TAPI_TONEMODE(2i32);
pub const TT_DYNAMIC: TERMINAL_TYPE = TERMINAL_TYPE(1i32);
pub const TT_STATIC: TERMINAL_TYPE = TERMINAL_TYPE(0i32);
pub const TUISPIDLL_OBJECT_DIALOGINSTANCE: i32 = 4i32;
pub const TUISPIDLL_OBJECT_LINEID: i32 = 1i32;
pub const TUISPIDLL_OBJECT_PHONEID: i32 = 2i32;
pub const TUISPIDLL_OBJECT_PROVIDERID: i32 = 3i32;
pub const atypFile: i32 = 1i32;
pub const atypMax: i32 = 4i32;
pub const atypNull: i32 = 0i32;
pub const atypOle: i32 = 2i32;
pub const atypPicture: i32 = 3i32;
pub const cbDisplayName: u32 = 41u32;
pub const cbEmailName: u32 = 11u32;
pub const cbMaxIdData: u32 = 200u32;
pub const cbSeverName: u32 = 12u32;
pub const cbTYPE: u32 = 16u32;
pub const prioHigh: u32 = 1u32;
pub const prioLow: u32 = 3u32;
pub const prioNorm: u32 = 2u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACDGROUP_EVENT(pub i32);
impl windows_core::TypeKind for ACDGROUP_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACDGROUP_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACDGROUP_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACDQUEUE_EVENT(pub i32);
impl windows_core::TypeKind for ACDQUEUE_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACDQUEUE_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACDQUEUE_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADDRESS_CAPABILITY(pub i32);
impl windows_core::TypeKind for ADDRESS_CAPABILITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADDRESS_CAPABILITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADDRESS_CAPABILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADDRESS_CAPABILITY_STRING(pub i32);
impl windows_core::TypeKind for ADDRESS_CAPABILITY_STRING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADDRESS_CAPABILITY_STRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADDRESS_CAPABILITY_STRING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADDRESS_EVENT(pub i32);
impl windows_core::TypeKind for ADDRESS_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADDRESS_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADDRESS_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADDRESS_STATE(pub i32);
impl windows_core::TypeKind for ADDRESS_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADDRESS_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADDRESS_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AGENTHANDLER_EVENT(pub i32);
impl windows_core::TypeKind for AGENTHANDLER_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AGENTHANDLER_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AGENTHANDLER_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AGENT_EVENT(pub i32);
impl windows_core::TypeKind for AGENT_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AGENT_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AGENT_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AGENT_SESSION_EVENT(pub i32);
impl windows_core::TypeKind for AGENT_SESSION_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AGENT_SESSION_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AGENT_SESSION_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AGENT_SESSION_STATE(pub i32);
impl windows_core::TypeKind for AGENT_SESSION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AGENT_SESSION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AGENT_SESSION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AGENT_STATE(pub i32);
impl windows_core::TypeKind for AGENT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AGENT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AGENT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLHUB_EVENT(pub i32);
impl windows_core::TypeKind for CALLHUB_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLHUB_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLHUB_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLHUB_STATE(pub i32);
impl windows_core::TypeKind for CALLHUB_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLHUB_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLHUB_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLINFOCHANGE_CAUSE(pub i32);
impl windows_core::TypeKind for CALLINFOCHANGE_CAUSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLINFOCHANGE_CAUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLINFOCHANGE_CAUSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLINFO_BUFFER(pub i32);
impl windows_core::TypeKind for CALLINFO_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLINFO_BUFFER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLINFO_BUFFER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLINFO_LONG(pub i32);
impl windows_core::TypeKind for CALLINFO_LONG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLINFO_LONG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLINFO_LONG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLINFO_STRING(pub i32);
impl windows_core::TypeKind for CALLINFO_STRING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLINFO_STRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLINFO_STRING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALL_MEDIA_EVENT(pub i32);
impl windows_core::TypeKind for CALL_MEDIA_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALL_MEDIA_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALL_MEDIA_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALL_MEDIA_EVENT_CAUSE(pub i32);
impl windows_core::TypeKind for CALL_MEDIA_EVENT_CAUSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALL_MEDIA_EVENT_CAUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALL_MEDIA_EVENT_CAUSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALL_NOTIFICATION_EVENT(pub i32);
impl windows_core::TypeKind for CALL_NOTIFICATION_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALL_NOTIFICATION_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALL_NOTIFICATION_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALL_PRIVILEGE(pub i32);
impl windows_core::TypeKind for CALL_PRIVILEGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALL_PRIVILEGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALL_PRIVILEGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALL_STATE(pub i32);
impl windows_core::TypeKind for CALL_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALL_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALL_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALL_STATE_EVENT_CAUSE(pub i32);
impl windows_core::TypeKind for CALL_STATE_EVENT_CAUSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALL_STATE_EVENT_CAUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALL_STATE_EVENT_CAUSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTORY_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for DIRECTORY_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTORY_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTORY_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTORY_TYPE(pub i32);
impl windows_core::TypeKind for DIRECTORY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTORY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTORY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISCONNECT_CODE(pub i32);
impl windows_core::TypeKind for DISCONNECT_CODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISCONNECT_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISCONNECT_CODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FINISH_MODE(pub i32);
impl windows_core::TypeKind for FINISH_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FINISH_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FINISH_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FT_STATE_EVENT_CAUSE(pub i32);
impl windows_core::TypeKind for FT_STATE_EVENT_CAUSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FT_STATE_EVENT_CAUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FT_STATE_EVENT_CAUSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FULLDUPLEX_SUPPORT(pub i32);
impl windows_core::TypeKind for FULLDUPLEX_SUPPORT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FULLDUPLEX_SUPPORT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FULLDUPLEX_SUPPORT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSP_ADDRESS_EVENT(pub i32);
impl windows_core::TypeKind for MSP_ADDRESS_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSP_ADDRESS_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSP_ADDRESS_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSP_CALL_EVENT(pub i32);
impl windows_core::TypeKind for MSP_CALL_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSP_CALL_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSP_CALL_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSP_CALL_EVENT_CAUSE(pub i32);
impl windows_core::TypeKind for MSP_CALL_EVENT_CAUSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSP_CALL_EVENT_CAUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSP_CALL_EVENT_CAUSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSP_EVENT(pub i32);
impl windows_core::TypeKind for MSP_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSP_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSP_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONECAPS_BUFFER(pub i32);
impl windows_core::TypeKind for PHONECAPS_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONECAPS_BUFFER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONECAPS_BUFFER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONECAPS_LONG(pub i32);
impl windows_core::TypeKind for PHONECAPS_LONG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONECAPS_LONG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONECAPS_LONG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONECAPS_STRING(pub i32);
impl windows_core::TypeKind for PHONECAPS_STRING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONECAPS_STRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONECAPS_STRING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_BUTTON_FUNCTION(pub i32);
impl windows_core::TypeKind for PHONE_BUTTON_FUNCTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_BUTTON_FUNCTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_BUTTON_FUNCTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_BUTTON_MODE(pub i32);
impl windows_core::TypeKind for PHONE_BUTTON_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_BUTTON_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_BUTTON_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_BUTTON_STATE(pub i32);
impl windows_core::TypeKind for PHONE_BUTTON_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_BUTTON_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_BUTTON_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_EVENT(pub i32);
impl windows_core::TypeKind for PHONE_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_HOOK_SWITCH_DEVICE(pub i32);
impl windows_core::TypeKind for PHONE_HOOK_SWITCH_DEVICE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_HOOK_SWITCH_DEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_HOOK_SWITCH_DEVICE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_HOOK_SWITCH_STATE(pub i32);
impl windows_core::TypeKind for PHONE_HOOK_SWITCH_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_HOOK_SWITCH_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_HOOK_SWITCH_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_LAMP_MODE(pub i32);
impl windows_core::TypeKind for PHONE_LAMP_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_LAMP_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_LAMP_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_PRIVILEGE(pub i32);
impl windows_core::TypeKind for PHONE_PRIVILEGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_PRIVILEGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_PRIVILEGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONE_TONE(pub i32);
impl windows_core::TypeKind for PHONE_TONE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONE_TONE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONE_TONE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QOS_EVENT(pub i32);
impl windows_core::TypeKind for QOS_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QOS_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QOS_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QOS_SERVICE_LEVEL(pub i32);
impl windows_core::TypeKind for QOS_SERVICE_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QOS_SERVICE_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QOS_SERVICE_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RND_ADVERTISING_SCOPE(pub i32);
impl windows_core::TypeKind for RND_ADVERTISING_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RND_ADVERTISING_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RND_ADVERTISING_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPIOBJECT_EVENT(pub i32);
impl windows_core::TypeKind for TAPIOBJECT_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPIOBJECT_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPIOBJECT_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPI_EVENT(pub i32);
impl windows_core::TypeKind for TAPI_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPI_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPI_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPI_GATHERTERM(pub i32);
impl windows_core::TypeKind for TAPI_GATHERTERM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPI_GATHERTERM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPI_GATHERTERM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPI_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for TAPI_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPI_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPI_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAPI_TONEMODE(pub i32);
impl windows_core::TypeKind for TAPI_TONEMODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAPI_TONEMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAPI_TONEMODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TERMINAL_DIRECTION(pub i32);
impl windows_core::TypeKind for TERMINAL_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TERMINAL_DIRECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TERMINAL_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TERMINAL_MEDIA_STATE(pub i32);
impl windows_core::TypeKind for TERMINAL_MEDIA_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TERMINAL_MEDIA_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TERMINAL_MEDIA_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TERMINAL_STATE(pub i32);
impl windows_core::TypeKind for TERMINAL_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TERMINAL_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TERMINAL_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TERMINAL_TYPE(pub i32);
impl windows_core::TypeKind for TERMINAL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TERMINAL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TERMINAL_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRALIAS {
    pub rgchName: [i8; 41],
    pub rgchEName: [i8; 11],
    pub rgchSrvr: [i8; 12],
    pub dibDetail: u32,
    pub r#type: u16,
}
impl windows_core::TypeKind for ADDRALIAS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADDRALIAS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DTR {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wDayOfWeek: u16,
}
impl windows_core::TypeKind for DTR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DispatchMapper: windows_core::GUID = windows_core::GUID::from_u128(0xe9225296_c759_11d1_a02b_00c04fb6809f);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVCALL(pub *mut core::ffi::c_void);
impl HDRVCALL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDRVCALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDRVCALL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVDIALOGINSTANCE(pub *mut core::ffi::c_void);
impl HDRVDIALOGINSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDRVDIALOGINSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDRVDIALOGINSTANCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVLINE(pub *mut core::ffi::c_void);
impl HDRVLINE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDRVLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDRVLINE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVMSPLINE(pub *mut core::ffi::c_void);
impl HDRVMSPLINE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDRVMSPLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDRVMSPLINE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVPHONE(pub *mut core::ffi::c_void);
impl HDRVPHONE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDRVPHONE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HDRVPHONE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPROVIDER(pub *mut core::ffi::c_void);
impl HPROVIDER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HPROVIDER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTAPICALL(pub *mut core::ffi::c_void);
impl HTAPICALL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HTAPICALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HTAPICALL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTAPILINE(pub *mut core::ffi::c_void);
impl HTAPILINE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HTAPILINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HTAPILINE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HTAPIPHONE(pub *mut core::ffi::c_void);
impl HTAPIPHONE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HTAPIPHONE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HTAPIPHONE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEADDRESSCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwLineDeviceID: u32,
    pub dwAddressSize: u32,
    pub dwAddressOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwAddressSharing: u32,
    pub dwAddressStates: u32,
    pub dwCallInfoStates: u32,
    pub dwCallerIDFlags: u32,
    pub dwCalledIDFlags: u32,
    pub dwConnectedIDFlags: u32,
    pub dwRedirectionIDFlags: u32,
    pub dwRedirectingIDFlags: u32,
    pub dwCallStates: u32,
    pub dwDialToneModes: u32,
    pub dwBusyModes: u32,
    pub dwSpecialInfo: u32,
    pub dwDisconnectModes: u32,
    pub dwMaxNumActiveCalls: u32,
    pub dwMaxNumOnHoldCalls: u32,
    pub dwMaxNumOnHoldPendingCalls: u32,
    pub dwMaxNumConference: u32,
    pub dwMaxNumTransConf: u32,
    pub dwAddrCapFlags: u32,
    pub dwCallFeatures: u32,
    pub dwRemoveFromConfCaps: u32,
    pub dwRemoveFromConfState: u32,
    pub dwTransferModes: u32,
    pub dwParkModes: u32,
    pub dwForwardModes: u32,
    pub dwMaxForwardEntries: u32,
    pub dwMaxSpecificEntries: u32,
    pub dwMinFwdNumRings: u32,
    pub dwMaxFwdNumRings: u32,
    pub dwMaxCallCompletions: u32,
    pub dwCallCompletionConds: u32,
    pub dwCallCompletionModes: u32,
    pub dwNumCompletionMessages: u32,
    pub dwCompletionMsgTextEntrySize: u32,
    pub dwCompletionMsgTextSize: u32,
    pub dwCompletionMsgTextOffset: u32,
    pub dwAddressFeatures: u32,
    pub dwPredictiveAutoTransferStates: u32,
    pub dwNumCallTreatments: u32,
    pub dwCallTreatmentListSize: u32,
    pub dwCallTreatmentListOffset: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub dwMaxCallDataSize: u32,
    pub dwCallFeatures2: u32,
    pub dwMaxNoAnswerTimeout: u32,
    pub dwConnectedModes: u32,
    pub dwOfferingModes: u32,
    pub dwAvailableMediaModes: u32,
}
impl windows_core::TypeKind for LINEADDRESSCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEADDRESSCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEADDRESSSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumInUse: u32,
    pub dwNumActiveCalls: u32,
    pub dwNumOnHoldCalls: u32,
    pub dwNumOnHoldPendCalls: u32,
    pub dwAddressFeatures: u32,
    pub dwNumRingsNoAnswer: u32,
    pub dwForwardNumEntries: u32,
    pub dwForwardSize: u32,
    pub dwForwardOffset: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
}
impl windows_core::TypeKind for LINEADDRESSSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEADDRESSSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTACTIVITYENTRY {
    pub dwID: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl windows_core::TypeKind for LINEAGENTACTIVITYENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTACTIVITYENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTACTIVITYLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl windows_core::TypeKind for LINEAGENTACTIVITYLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTACTIVITYLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentHandlerInfoSize: u32,
    pub dwAgentHandlerInfoOffset: u32,
    pub dwCapsVersion: u32,
    pub dwFeatures: u32,
    pub dwStates: u32,
    pub dwNextStates: u32,
    pub dwMaxNumGroupEntries: u32,
    pub dwAgentStatusMessages: u32,
    pub dwNumAgentExtensionIDs: u32,
    pub dwAgentExtensionIDListSize: u32,
    pub dwAgentExtensionIDListOffset: u32,
    pub ProxyGUID: windows_core::GUID,
}
impl windows_core::TypeKind for LINEAGENTCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTENTRY {
    pub hAgent: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
    pub dwIDSize: u32,
    pub dwIDOffset: u32,
    pub dwPINSize: u32,
    pub dwPINOffset: u32,
}
impl windows_core::TypeKind for LINEAGENTENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTGROUPENTRY {
    pub GroupID: LINEAGENTGROUPENTRY_0,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl windows_core::TypeKind for LINEAGENTGROUPENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTGROUPENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTGROUPENTRY_0 {
    pub dwGroupID1: u32,
    pub dwGroupID2: u32,
    pub dwGroupID3: u32,
    pub dwGroupID4: u32,
}
impl windows_core::TypeKind for LINEAGENTGROUPENTRY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTGROUPENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTGROUPLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl windows_core::TypeKind for LINEAGENTGROUPLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTGROUPLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEAGENTINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
    pub dwMeasurementPeriod: u32,
    pub cyOverallCallRate: super::super::System::Com::CY,
    pub dwNumberOfACDCalls: u32,
    pub dwNumberOfIncomingCalls: u32,
    pub dwNumberOfOutgoingCalls: u32,
    pub dwTotalACDTalkTime: u32,
    pub dwTotalACDCallTime: u32,
    pub dwTotalACDWrapUpTime: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEAGENTINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEAGENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl windows_core::TypeKind for LINEAGENTLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTSESSIONENTRY {
    pub hAgentSession: u32,
    pub hAgent: u32,
    pub GroupID: windows_core::GUID,
    pub dwWorkingAddressID: u32,
}
impl windows_core::TypeKind for LINEAGENTSESSIONENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTSESSIONENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEAGENTSESSIONINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwAgentSessionState: u32,
    pub dwNextAgentSessionState: u32,
    pub dateSessionStartTime: f64,
    pub dwSessionDuration: u32,
    pub dwNumberOfCalls: u32,
    pub dwTotalTalkTime: u32,
    pub dwAverageTalkTime: u32,
    pub dwTotalCallTime: u32,
    pub dwAverageCallTime: u32,
    pub dwTotalWrapUpTime: u32,
    pub dwAverageWrapUpTime: u32,
    pub cyACDCallRate: super::super::System::Com::CY,
    pub dwLongestTimeToAnswer: u32,
    pub dwAverageTimeToAnswer: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEAGENTSESSIONINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEAGENTSESSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTSESSIONLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl windows_core::TypeKind for LINEAGENTSESSIONLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTSESSIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAGENTSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwGroupListSize: u32,
    pub dwGroupListOffset: u32,
    pub dwState: u32,
    pub dwNextState: u32,
    pub dwActivityID: u32,
    pub dwActivitySize: u32,
    pub dwActivityOffset: u32,
    pub dwAgentFeatures: u32,
    pub dwValidStates: u32,
    pub dwValidNextStates: u32,
}
impl windows_core::TypeKind for LINEAGENTSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAGENTSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEAPPINFO {
    pub dwMachineNameSize: u32,
    pub dwMachineNameOffset: u32,
    pub dwUserNameSize: u32,
    pub dwUserNameOffset: u32,
    pub dwModuleFilenameSize: u32,
    pub dwModuleFilenameOffset: u32,
    pub dwFriendlyNameSize: u32,
    pub dwFriendlyNameOffset: u32,
    pub dwMediaModes: u32,
    pub dwAddressID: u32,
}
impl windows_core::TypeKind for LINEAPPINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEAPPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECALLINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub hLine: u32,
    pub dwLineDeviceID: u32,
    pub dwAddressID: u32,
    pub dwBearerMode: u32,
    pub dwRate: u32,
    pub dwMediaMode: u32,
    pub dwAppSpecific: u32,
    pub dwCallID: u32,
    pub dwRelatedCallID: u32,
    pub dwCallParamFlags: u32,
    pub dwCallStates: u32,
    pub dwMonitorDigitModes: u32,
    pub dwMonitorMediaModes: u32,
    pub DialParams: LINEDIALPARAMS,
    pub dwOrigin: u32,
    pub dwReason: u32,
    pub dwCompletionID: u32,
    pub dwNumOwners: u32,
    pub dwNumMonitors: u32,
    pub dwCountryCode: u32,
    pub dwTrunk: u32,
    pub dwCallerIDFlags: u32,
    pub dwCallerIDSize: u32,
    pub dwCallerIDOffset: u32,
    pub dwCallerIDNameSize: u32,
    pub dwCallerIDNameOffset: u32,
    pub dwCalledIDFlags: u32,
    pub dwCalledIDSize: u32,
    pub dwCalledIDOffset: u32,
    pub dwCalledIDNameSize: u32,
    pub dwCalledIDNameOffset: u32,
    pub dwConnectedIDFlags: u32,
    pub dwConnectedIDSize: u32,
    pub dwConnectedIDOffset: u32,
    pub dwConnectedIDNameSize: u32,
    pub dwConnectedIDNameOffset: u32,
    pub dwRedirectionIDFlags: u32,
    pub dwRedirectionIDSize: u32,
    pub dwRedirectionIDOffset: u32,
    pub dwRedirectionIDNameSize: u32,
    pub dwRedirectionIDNameOffset: u32,
    pub dwRedirectingIDFlags: u32,
    pub dwRedirectingIDSize: u32,
    pub dwRedirectingIDOffset: u32,
    pub dwRedirectingIDNameSize: u32,
    pub dwRedirectingIDNameOffset: u32,
    pub dwAppNameSize: u32,
    pub dwAppNameOffset: u32,
    pub dwDisplayableAddressSize: u32,
    pub dwDisplayableAddressOffset: u32,
    pub dwCalledPartySize: u32,
    pub dwCalledPartyOffset: u32,
    pub dwCommentSize: u32,
    pub dwCommentOffset: u32,
    pub dwDisplaySize: u32,
    pub dwDisplayOffset: u32,
    pub dwUserUserInfoSize: u32,
    pub dwUserUserInfoOffset: u32,
    pub dwHighLevelCompSize: u32,
    pub dwHighLevelCompOffset: u32,
    pub dwLowLevelCompSize: u32,
    pub dwLowLevelCompOffset: u32,
    pub dwChargingInfoSize: u32,
    pub dwChargingInfoOffset: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwCallTreatment: u32,
    pub dwCallDataSize: u32,
    pub dwCallDataOffset: u32,
    pub dwSendingFlowspecSize: u32,
    pub dwSendingFlowspecOffset: u32,
    pub dwReceivingFlowspecSize: u32,
    pub dwReceivingFlowspecOffset: u32,
}
impl windows_core::TypeKind for LINECALLINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECALLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECALLLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwCallsNumEntries: u32,
    pub dwCallsSize: u32,
    pub dwCallsOffset: u32,
}
impl windows_core::TypeKind for LINECALLLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECALLLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECALLPARAMS {
    pub dwTotalSize: u32,
    pub dwBearerMode: u32,
    pub dwMinRate: u32,
    pub dwMaxRate: u32,
    pub dwMediaMode: u32,
    pub dwCallParamFlags: u32,
    pub dwAddressMode: u32,
    pub dwAddressID: u32,
    pub DialParams: LINEDIALPARAMS,
    pub dwOrigAddressSize: u32,
    pub dwOrigAddressOffset: u32,
    pub dwDisplayableAddressSize: u32,
    pub dwDisplayableAddressOffset: u32,
    pub dwCalledPartySize: u32,
    pub dwCalledPartyOffset: u32,
    pub dwCommentSize: u32,
    pub dwCommentOffset: u32,
    pub dwUserUserInfoSize: u32,
    pub dwUserUserInfoOffset: u32,
    pub dwHighLevelCompSize: u32,
    pub dwHighLevelCompOffset: u32,
    pub dwLowLevelCompSize: u32,
    pub dwLowLevelCompOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwPredictiveAutoTransferStates: u32,
    pub dwTargetAddressSize: u32,
    pub dwTargetAddressOffset: u32,
    pub dwSendingFlowspecSize: u32,
    pub dwSendingFlowspecOffset: u32,
    pub dwReceivingFlowspecSize: u32,
    pub dwReceivingFlowspecOffset: u32,
    pub dwDeviceClassSize: u32,
    pub dwDeviceClassOffset: u32,
    pub dwDeviceConfigSize: u32,
    pub dwDeviceConfigOffset: u32,
    pub dwCallDataSize: u32,
    pub dwCallDataOffset: u32,
    pub dwNoAnswerTimeout: u32,
    pub dwCallingPartyIDSize: u32,
    pub dwCallingPartyIDOffset: u32,
}
impl windows_core::TypeKind for LINECALLPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECALLPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECALLSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwCallState: u32,
    pub dwCallStateMode: u32,
    pub dwCallPrivilege: u32,
    pub dwCallFeatures: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwCallFeatures2: u32,
    pub tStateEntryTime: super::super::Foundation::SYSTEMTIME,
}
impl windows_core::TypeKind for LINECALLSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECALLSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECALLTREATMENTENTRY {
    pub dwCallTreatmentID: u32,
    pub dwCallTreatmentNameSize: u32,
    pub dwCallTreatmentNameOffset: u32,
}
impl windows_core::TypeKind for LINECALLTREATMENTENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECALLTREATMENTENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECARDENTRY {
    pub dwPermanentCardID: u32,
    pub dwCardNameSize: u32,
    pub dwCardNameOffset: u32,
    pub dwCardNumberDigits: u32,
    pub dwSameAreaRuleSize: u32,
    pub dwSameAreaRuleOffset: u32,
    pub dwLongDistanceRuleSize: u32,
    pub dwLongDistanceRuleOffset: u32,
    pub dwInternationalRuleSize: u32,
    pub dwInternationalRuleOffset: u32,
    pub dwOptions: u32,
}
impl windows_core::TypeKind for LINECARDENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECARDENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECOUNTRYENTRY {
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub dwNextCountryID: u32,
    pub dwCountryNameSize: u32,
    pub dwCountryNameOffset: u32,
    pub dwSameAreaRuleSize: u32,
    pub dwSameAreaRuleOffset: u32,
    pub dwLongDistanceRuleSize: u32,
    pub dwLongDistanceRuleOffset: u32,
    pub dwInternationalRuleSize: u32,
    pub dwInternationalRuleOffset: u32,
}
impl windows_core::TypeKind for LINECOUNTRYENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECOUNTRYENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINECOUNTRYLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumCountries: u32,
    pub dwCountryListSize: u32,
    pub dwCountryListOffset: u32,
}
impl windows_core::TypeKind for LINECOUNTRYLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINECOUNTRYLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEDEVCAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwProviderInfoSize: u32,
    pub dwProviderInfoOffset: u32,
    pub dwSwitchInfoSize: u32,
    pub dwSwitchInfoOffset: u32,
    pub dwPermanentLineID: u32,
    pub dwLineNameSize: u32,
    pub dwLineNameOffset: u32,
    pub dwStringFormat: u32,
    pub dwAddressModes: u32,
    pub dwNumAddresses: u32,
    pub dwBearerModes: u32,
    pub dwMaxRate: u32,
    pub dwMediaModes: u32,
    pub dwGenerateToneModes: u32,
    pub dwGenerateToneMaxNumFreq: u32,
    pub dwGenerateDigitModes: u32,
    pub dwMonitorToneMaxNumFreq: u32,
    pub dwMonitorToneMaxNumEntries: u32,
    pub dwMonitorDigitModes: u32,
    pub dwGatherDigitsMinTimeout: u32,
    pub dwGatherDigitsMaxTimeout: u32,
    pub dwMedCtlDigitMaxListSize: u32,
    pub dwMedCtlMediaMaxListSize: u32,
    pub dwMedCtlToneMaxListSize: u32,
    pub dwMedCtlCallStateMaxListSize: u32,
    pub dwDevCapFlags: u32,
    pub dwMaxNumActiveCalls: u32,
    pub dwAnswerMode: u32,
    pub dwRingModes: u32,
    pub dwLineStates: u32,
    pub dwUUIAcceptSize: u32,
    pub dwUUIAnswerSize: u32,
    pub dwUUIMakeCallSize: u32,
    pub dwUUIDropSize: u32,
    pub dwUUISendUserUserInfoSize: u32,
    pub dwUUICallInfoSize: u32,
    pub MinDialParams: LINEDIALPARAMS,
    pub MaxDialParams: LINEDIALPARAMS,
    pub DefaultDialParams: LINEDIALPARAMS,
    pub dwNumTerminals: u32,
    pub dwTerminalCapsSize: u32,
    pub dwTerminalCapsOffset: u32,
    pub dwTerminalTextEntrySize: u32,
    pub dwTerminalTextSize: u32,
    pub dwTerminalTextOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwLineFeatures: u32,
    pub dwSettableDevStatus: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub PermanentLineGuid: windows_core::GUID,
}
impl windows_core::TypeKind for LINEDEVCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEDEVCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEDEVSTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumOpens: u32,
    pub dwOpenMediaModes: u32,
    pub dwNumActiveCalls: u32,
    pub dwNumOnHoldCalls: u32,
    pub dwNumOnHoldPendCalls: u32,
    pub dwLineFeatures: u32,
    pub dwNumCallCompletions: u32,
    pub dwRingMode: u32,
    pub dwSignalLevel: u32,
    pub dwBatteryLevel: u32,
    pub dwRoamMode: u32,
    pub dwDevStatusFlags: u32,
    pub dwTerminalModesSize: u32,
    pub dwTerminalModesOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwAvailableMediaModes: u32,
    pub dwAppInfoSize: u32,
    pub dwAppInfoOffset: u32,
}
impl windows_core::TypeKind for LINEDEVSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEDEVSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEDIALPARAMS {
    pub dwDialPause: u32,
    pub dwDialSpeed: u32,
    pub dwDigitDuration: u32,
    pub dwWaitForDialtone: u32,
}
impl windows_core::TypeKind for LINEDIALPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEDIALPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEEXTENSIONID {
    pub dwExtensionID0: u32,
    pub dwExtensionID1: u32,
    pub dwExtensionID2: u32,
    pub dwExtensionID3: u32,
}
impl windows_core::TypeKind for LINEEXTENSIONID {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEEXTENSIONID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEFORWARD {
    pub dwForwardMode: u32,
    pub dwCallerAddressSize: u32,
    pub dwCallerAddressOffset: u32,
    pub dwDestCountryCode: u32,
    pub dwDestAddressSize: u32,
    pub dwDestAddressOffset: u32,
}
impl windows_core::TypeKind for LINEFORWARD {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEFORWARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEFORWARDLIST {
    pub dwTotalSize: u32,
    pub dwNumEntries: u32,
    pub ForwardList: [LINEFORWARD; 1],
}
impl windows_core::TypeKind for LINEFORWARDLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEFORWARDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEGENERATETONE {
    pub dwFrequency: u32,
    pub dwCadenceOn: u32,
    pub dwCadenceOff: u32,
    pub dwVolume: u32,
}
impl windows_core::TypeKind for LINEGENERATETONE {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEGENERATETONE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEINITIALIZEEXPARAMS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwOptions: u32,
    pub Handles: LINEINITIALIZEEXPARAMS_0,
    pub dwCompletionKey: u32,
}
impl windows_core::TypeKind for LINEINITIALIZEEXPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEINITIALIZEEXPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union LINEINITIALIZEEXPARAMS_0 {
    pub hEvent: super::super::Foundation::HANDLE,
    pub hCompletionPort: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for LINEINITIALIZEEXPARAMS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEINITIALIZEEXPARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINELOCATIONENTRY {
    pub dwPermanentLocationID: u32,
    pub dwLocationNameSize: u32,
    pub dwLocationNameOffset: u32,
    pub dwCountryCode: u32,
    pub dwCityCodeSize: u32,
    pub dwCityCodeOffset: u32,
    pub dwPreferredCardID: u32,
    pub dwLocalAccessCodeSize: u32,
    pub dwLocalAccessCodeOffset: u32,
    pub dwLongDistanceAccessCodeSize: u32,
    pub dwLongDistanceAccessCodeOffset: u32,
    pub dwTollPrefixListSize: u32,
    pub dwTollPrefixListOffset: u32,
    pub dwCountryID: u32,
    pub dwOptions: u32,
    pub dwCancelCallWaitingSize: u32,
    pub dwCancelCallWaitingOffset: u32,
}
impl windows_core::TypeKind for LINELOCATIONENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINELOCATIONENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEMEDIACONTROLCALLSTATE {
    pub dwCallStates: u32,
    pub dwMediaControl: u32,
}
impl windows_core::TypeKind for LINEMEDIACONTROLCALLSTATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEMEDIACONTROLCALLSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEMEDIACONTROLDIGIT {
    pub dwDigit: u32,
    pub dwDigitModes: u32,
    pub dwMediaControl: u32,
}
impl windows_core::TypeKind for LINEMEDIACONTROLDIGIT {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEMEDIACONTROLDIGIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEMEDIACONTROLMEDIA {
    pub dwMediaModes: u32,
    pub dwDuration: u32,
    pub dwMediaControl: u32,
}
impl windows_core::TypeKind for LINEMEDIACONTROLMEDIA {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEMEDIACONTROLMEDIA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEMEDIACONTROLTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
    pub dwMediaControl: u32,
}
impl windows_core::TypeKind for LINEMEDIACONTROLTONE {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEMEDIACONTROLTONE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEMESSAGE {
    pub hDevice: u32,
    pub dwMessageID: u32,
    pub dwCallbackInstance: usize,
    pub dwParam1: usize,
    pub dwParam2: usize,
    pub dwParam3: usize,
}
impl windows_core::TypeKind for LINEMESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEMONITORTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
}
impl windows_core::TypeKind for LINEMONITORTONE {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEMONITORTONE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEPROVIDERENTRY {
    pub dwPermanentProviderID: u32,
    pub dwProviderFilenameSize: u32,
    pub dwProviderFilenameOffset: u32,
}
impl windows_core::TypeKind for LINEPROVIDERENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEPROVIDERENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEPROVIDERLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumProviders: u32,
    pub dwProviderListSize: u32,
    pub dwProviderListOffset: u32,
}
impl windows_core::TypeKind for LINEPROVIDERLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEPROVIDERLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST {
    pub dwSize: u32,
    pub dwClientMachineNameSize: u32,
    pub dwClientMachineNameOffset: u32,
    pub dwClientUserNameSize: u32,
    pub dwClientUserNameOffset: u32,
    pub dwClientAppAPIVersion: u32,
    pub dwRequestType: u32,
    pub Anonymous: LINEPROXYREQUEST_0,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union LINEPROXYREQUEST_0 {
    pub SetAgentGroup: LINEPROXYREQUEST_0_14,
    pub SetAgentState: LINEPROXYREQUEST_0_18,
    pub SetAgentActivity: LINEPROXYREQUEST_0_13,
    pub GetAgentCaps: LINEPROXYREQUEST_0_4,
    pub GetAgentStatus: LINEPROXYREQUEST_0_9,
    pub AgentSpecific: LINEPROXYREQUEST_0_0,
    pub GetAgentActivityList: LINEPROXYREQUEST_0_3,
    pub GetAgentGroupList: LINEPROXYREQUEST_0_5,
    pub CreateAgent: LINEPROXYREQUEST_0_2,
    pub SetAgentStateEx: LINEPROXYREQUEST_0_17,
    pub SetAgentMeasurementPeriod: LINEPROXYREQUEST_0_15,
    pub GetAgentInfo: LINEPROXYREQUEST_0_6,
    pub CreateAgentSession: LINEPROXYREQUEST_0_1,
    pub GetAgentSessionList: LINEPROXYREQUEST_0_8,
    pub GetAgentSessionInfo: LINEPROXYREQUEST_0_7,
    pub SetAgentSessionState: LINEPROXYREQUEST_0_16,
    pub GetQueueList: LINEPROXYREQUEST_0_12,
    pub SetQueueMeasurementPeriod: LINEPROXYREQUEST_0_19,
    pub GetQueueInfo: LINEPROXYREQUEST_0_11,
    pub GetGroupList: LINEPROXYREQUEST_0_10,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_0 {
    pub dwAddressID: u32,
    pub dwAgentExtensionIDIndex: u32,
    pub dwSize: u32,
    pub Params: [u8; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_1 {
    pub hAgentSession: u32,
    pub dwAgentPINSize: u32,
    pub dwAgentPINOffset: u32,
    pub hAgent: u32,
    pub GroupID: windows_core::GUID,
    pub dwWorkingAddressID: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_2 {
    pub hAgent: u32,
    pub dwAgentIDSize: u32,
    pub dwAgentIDOffset: u32,
    pub dwAgentPINSize: u32,
    pub dwAgentPINOffset: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_3 {
    pub dwAddressID: u32,
    pub ActivityList: LINEAGENTACTIVITYLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_4 {
    pub dwAddressID: u32,
    pub AgentCaps: LINEAGENTCAPS,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_5 {
    pub dwAddressID: u32,
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_5 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_6 {
    pub hAgent: u32,
    pub AgentInfo: LINEAGENTINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_6 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_7 {
    pub hAgentSession: u32,
    pub SessionInfo: LINEAGENTSESSIONINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_7 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_8 {
    pub hAgent: u32,
    pub SessionList: LINEAGENTSESSIONLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_8 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_9 {
    pub dwAddressID: u32,
    pub AgentStatus: LINEAGENTSTATUS,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_9 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_10 {
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_10 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_11 {
    pub dwQueueID: u32,
    pub QueueInfo: LINEQUEUEINFO,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_11 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_12 {
    pub GroupID: windows_core::GUID,
    pub QueueList: LINEQUEUELIST,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_12 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_13 {
    pub dwAddressID: u32,
    pub dwActivityID: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_13 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_14 {
    pub dwAddressID: u32,
    pub GroupList: LINEAGENTGROUPLIST,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_14 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_15 {
    pub hAgent: u32,
    pub dwMeasurementPeriod: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_15 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_16 {
    pub hAgentSession: u32,
    pub dwAgentSessionState: u32,
    pub dwNextAgentSessionState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_16 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_17 {
    pub hAgent: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_17 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_18 {
    pub dwAddressID: u32,
    pub dwAgentState: u32,
    pub dwNextAgentState: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_18 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUEST_0_19 {
    pub dwQueueID: u32,
    pub dwMeasurementPeriod: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for LINEPROXYREQUEST_0_19 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for LINEPROXYREQUEST_0_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEPROXYREQUESTLIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl windows_core::TypeKind for LINEPROXYREQUESTLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEPROXYREQUESTLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEQUEUEENTRY {
    pub dwQueueID: u32,
    pub dwNameSize: u32,
    pub dwNameOffset: u32,
}
impl windows_core::TypeKind for LINEQUEUEENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEQUEUEENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEQUEUEINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwMeasurementPeriod: u32,
    pub dwTotalCallsQueued: u32,
    pub dwCurrentCallsQueued: u32,
    pub dwTotalCallsAbandoned: u32,
    pub dwTotalCallsFlowedIn: u32,
    pub dwTotalCallsFlowedOut: u32,
    pub dwLongestEverWaitTime: u32,
    pub dwCurrentLongestWaitTime: u32,
    pub dwAverageWaitTime: u32,
    pub dwFinalDisposition: u32,
}
impl windows_core::TypeKind for LINEQUEUEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEQUEUEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEQUEUELIST {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumEntries: u32,
    pub dwListSize: u32,
    pub dwListOffset: u32,
}
impl windows_core::TypeKind for LINEQUEUELIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEQUEUELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LINEREQMAKECALL {
    pub szDestAddress: [i8; 80],
    pub szAppName: [i8; 40],
    pub szCalledParty: [i8; 40],
    pub szComment: [i8; 80],
}
impl windows_core::TypeKind for LINEREQMAKECALL {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEREQMAKECALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEREQMAKECALLW {
    pub szDestAddress: [u16; 80],
    pub szAppName: [u16; 40],
    pub szCalledParty: [u16; 40],
    pub szComment: [u16; 80],
}
impl windows_core::TypeKind for LINEREQMAKECALLW {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEREQMAKECALLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEREQMEDIACALL {
    pub hWnd: super::super::Foundation::HWND,
    pub wRequestID: super::super::Foundation::WPARAM,
    pub szDeviceClass: [i8; 40],
    pub ucDeviceID: [u8; 40],
    pub dwSize: u32,
    pub dwSecure: u32,
    pub szDestAddress: [i8; 80],
    pub szAppName: [i8; 40],
    pub szCalledParty: [i8; 40],
    pub szComment: [i8; 80],
}
impl windows_core::TypeKind for LINEREQMEDIACALL {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEREQMEDIACALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINEREQMEDIACALLW {
    pub hWnd: super::super::Foundation::HWND,
    pub wRequestID: super::super::Foundation::WPARAM,
    pub szDeviceClass: [u16; 40],
    pub ucDeviceID: [u8; 40],
    pub dwSize: u32,
    pub dwSecure: u32,
    pub szDestAddress: [u16; 80],
    pub szAppName: [u16; 40],
    pub szCalledParty: [u16; 40],
    pub szComment: [u16; 80],
}
impl windows_core::TypeKind for LINEREQMEDIACALLW {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINEREQMEDIACALLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINETERMCAPS {
    pub dwTermDev: u32,
    pub dwTermModes: u32,
    pub dwTermSharing: u32,
}
impl windows_core::TypeKind for LINETERMCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINETERMCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINETRANSLATECAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwNumLocations: u32,
    pub dwLocationListSize: u32,
    pub dwLocationListOffset: u32,
    pub dwCurrentLocationID: u32,
    pub dwNumCards: u32,
    pub dwCardListSize: u32,
    pub dwCardListOffset: u32,
    pub dwCurrentPreferredCardID: u32,
}
impl windows_core::TypeKind for LINETRANSLATECAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINETRANSLATECAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LINETRANSLATEOUTPUT {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwDialableStringSize: u32,
    pub dwDialableStringOffset: u32,
    pub dwDisplayableStringSize: u32,
    pub dwDisplayableStringOffset: u32,
    pub dwCurrentCountry: u32,
    pub dwDestCountry: u32,
    pub dwTranslateResults: u32,
}
impl windows_core::TypeKind for LINETRANSLATEOUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for LINETRANSLATEOUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct MSP_EVENT_INFO {
    pub dwSize: u32,
    pub Event: MSP_EVENT,
    pub hCall: *mut i32,
    pub Anonymous: MSP_EVENT_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union MSP_EVENT_INFO_0 {
    pub MSP_ADDRESS_EVENT_INFO: core::mem::ManuallyDrop<MSP_EVENT_INFO_0_0>,
    pub MSP_CALL_EVENT_INFO: core::mem::ManuallyDrop<MSP_EVENT_INFO_0_2>,
    pub MSP_TSP_DATA: MSP_EVENT_INFO_0_6,
    pub MSP_PRIVATE_EVENT_INFO: core::mem::ManuallyDrop<MSP_EVENT_INFO_0_4>,
    pub MSP_FILE_TERMINAL_EVENT_INFO: core::mem::ManuallyDrop<MSP_EVENT_INFO_0_3>,
    pub MSP_ASR_TERMINAL_EVENT_INFO: core::mem::ManuallyDrop<MSP_EVENT_INFO_0_1>,
    pub MSP_TTS_TERMINAL_EVENT_INFO: core::mem::ManuallyDrop<MSP_EVENT_INFO_0_7>,
    pub MSP_TONE_TERMINAL_EVENT_INFO: core::mem::ManuallyDrop<MSP_EVENT_INFO_0_5>,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_0 {
    pub Type: MSP_ADDRESS_EVENT,
    pub pTerminal: core::mem::ManuallyDrop<Option<ITTerminal>>,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_1 {
    pub pASRTerminal: core::mem::ManuallyDrop<Option<ITTerminal>>,
    pub hrErrorCode: windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0_1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_2 {
    pub Type: MSP_CALL_EVENT,
    pub Cause: MSP_CALL_EVENT_CAUSE,
    pub pStream: core::mem::ManuallyDrop<Option<ITStream>>,
    pub pTerminal: core::mem::ManuallyDrop<Option<ITTerminal>>,
    pub hrError: windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0_2 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_3 {
    pub pParentFileTerminal: core::mem::ManuallyDrop<Option<ITTerminal>>,
    pub pFileTrack: core::mem::ManuallyDrop<Option<ITFileTrack>>,
    pub TerminalMediaState: TERMINAL_MEDIA_STATE,
    pub ftecEventCause: FT_STATE_EVENT_CAUSE,
    pub hrErrorCode: windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0_3 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_4 {
    pub pEvent: core::mem::ManuallyDrop<Option<super::super::System::Com::IDispatch>>,
    pub lEventCode: i32,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0_4 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_5 {
    pub pToneTerminal: core::mem::ManuallyDrop<Option<ITTerminal>>,
    pub hrErrorCode: windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0_5 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_5 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_6 {
    pub dwBufferSize: u32,
    pub pBuffer: [u8; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_6 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct MSP_EVENT_INFO_0_7 {
    pub pTTSTerminal: core::mem::ManuallyDrop<Option<ITTerminal>>,
    pub hrErrorCode: windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for MSP_EVENT_INFO_0_7 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MSP_EVENT_INFO_0_7 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MSP_EVENT_INFO_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const McastAddressAllocation: windows_core::GUID = windows_core::GUID::from_u128(0xdf0daef2_a289_11d1_8697_006008b0e5d2);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NSID {
    pub dwSize: u32,
    pub uchType: [u8; 16],
    pub xtype: u32,
    pub lTime: i32,
    pub address: NSID_0,
}
impl windows_core::TypeKind for NSID {
    type TypeKind = windows_core::CopyType;
}
impl Default for NSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NSID_0 {
    pub alias: ADDRALIAS,
    pub rgchInterNet: [i8; 1],
}
impl windows_core::TypeKind for NSID_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NSID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PHONEBUTTONINFO {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwButtonMode: u32,
    pub dwButtonFunction: u32,
    pub dwButtonTextSize: u32,
    pub dwButtonTextOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwButtonState: u32,
}
impl windows_core::TypeKind for PHONEBUTTONINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHONEBUTTONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PHONECAPS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwProviderInfoSize: u32,
    pub dwProviderInfoOffset: u32,
    pub dwPhoneInfoSize: u32,
    pub dwPhoneInfoOffset: u32,
    pub dwPermanentPhoneID: u32,
    pub dwPhoneNameSize: u32,
    pub dwPhoneNameOffset: u32,
    pub dwStringFormat: u32,
    pub dwPhoneStates: u32,
    pub dwHookSwitchDevs: u32,
    pub dwHandsetHookSwitchModes: u32,
    pub dwSpeakerHookSwitchModes: u32,
    pub dwHeadsetHookSwitchModes: u32,
    pub dwVolumeFlags: u32,
    pub dwGainFlags: u32,
    pub dwDisplayNumRows: u32,
    pub dwDisplayNumColumns: u32,
    pub dwNumRingModes: u32,
    pub dwNumButtonLamps: u32,
    pub dwButtonModesSize: u32,
    pub dwButtonModesOffset: u32,
    pub dwButtonFunctionsSize: u32,
    pub dwButtonFunctionsOffset: u32,
    pub dwLampModesSize: u32,
    pub dwLampModesOffset: u32,
    pub dwNumSetData: u32,
    pub dwSetDataSize: u32,
    pub dwSetDataOffset: u32,
    pub dwNumGetData: u32,
    pub dwGetDataSize: u32,
    pub dwGetDataOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDeviceClassesSize: u32,
    pub dwDeviceClassesOffset: u32,
    pub dwPhoneFeatures: u32,
    pub dwSettableHandsetHookSwitchModes: u32,
    pub dwSettableSpeakerHookSwitchModes: u32,
    pub dwSettableHeadsetHookSwitchModes: u32,
    pub dwMonitoredHandsetHookSwitchModes: u32,
    pub dwMonitoredSpeakerHookSwitchModes: u32,
    pub dwMonitoredHeadsetHookSwitchModes: u32,
    pub PermanentPhoneGuid: windows_core::GUID,
}
impl windows_core::TypeKind for PHONECAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHONECAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PHONEEXTENSIONID {
    pub dwExtensionID0: u32,
    pub dwExtensionID1: u32,
    pub dwExtensionID2: u32,
    pub dwExtensionID3: u32,
}
impl windows_core::TypeKind for PHONEEXTENSIONID {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHONEEXTENSIONID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PHONEINITIALIZEEXPARAMS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwOptions: u32,
    pub Handles: PHONEINITIALIZEEXPARAMS_0,
    pub dwCompletionKey: u32,
}
impl windows_core::TypeKind for PHONEINITIALIZEEXPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHONEINITIALIZEEXPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union PHONEINITIALIZEEXPARAMS_0 {
    pub hEvent: super::super::Foundation::HANDLE,
    pub hCompletionPort: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for PHONEINITIALIZEEXPARAMS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHONEINITIALIZEEXPARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PHONEMESSAGE {
    pub hDevice: u32,
    pub dwMessageID: u32,
    pub dwCallbackInstance: usize,
    pub dwParam1: usize,
    pub dwParam2: usize,
    pub dwParam3: usize,
}
impl windows_core::TypeKind for PHONEMESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHONEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PHONESTATUS {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwStatusFlags: u32,
    pub dwNumOwners: u32,
    pub dwNumMonitors: u32,
    pub dwRingMode: u32,
    pub dwRingVolume: u32,
    pub dwHandsetHookSwitchMode: u32,
    pub dwHandsetVolume: u32,
    pub dwHandsetGain: u32,
    pub dwSpeakerHookSwitchMode: u32,
    pub dwSpeakerVolume: u32,
    pub dwSpeakerGain: u32,
    pub dwHeadsetHookSwitchMode: u32,
    pub dwHeadsetVolume: u32,
    pub dwHeadsetGain: u32,
    pub dwDisplaySize: u32,
    pub dwDisplayOffset: u32,
    pub dwLampModesSize: u32,
    pub dwLampModesOffset: u32,
    pub dwOwnerNameSize: u32,
    pub dwOwnerNameOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwPhoneFeatures: u32,
}
impl windows_core::TypeKind for PHONESTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PHONESTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RENDDATA {
    pub atyp: u16,
    pub ulPosition: u32,
    pub dxWidth: u16,
    pub dyHeight: u16,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for RENDDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RENDDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Rendezvous: windows_core::GUID = windows_core::GUID::from_u128(0xf1029e5b_cb5b_11d0_8d59_00c04fd91ac0);
pub const RequestMakeCall: windows_core::GUID = windows_core::GUID::from_u128(0xac48ffe0_f8c4_11d1_a030_00c04fb6809f);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STnefProblem {
    pub ulComponent: u32,
    pub ulAttribute: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
impl windows_core::TypeKind for STnefProblem {
    type TypeKind = windows_core::CopyType;
}
impl Default for STnefProblem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STnefProblemArray {
    pub cProblem: u32,
    pub aProblem: [STnefProblem; 1],
}
impl windows_core::TypeKind for STnefProblemArray {
    type TypeKind = windows_core::CopyType;
}
impl Default for STnefProblemArray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TAPI: windows_core::GUID = windows_core::GUID::from_u128(0x21d6d48e_a88b_11d0_83dd_00aa003ccabd);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAPI_CUSTOMTONE {
    pub dwFrequency: u32,
    pub dwCadenceOn: u32,
    pub dwCadenceOff: u32,
    pub dwVolume: u32,
}
impl windows_core::TypeKind for TAPI_CUSTOMTONE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAPI_CUSTOMTONE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAPI_DETECTTONE {
    pub dwAppSpecific: u32,
    pub dwDuration: u32,
    pub dwFrequency1: u32,
    pub dwFrequency2: u32,
    pub dwFrequency3: u32,
}
impl windows_core::TypeKind for TAPI_DETECTTONE {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAPI_DETECTTONE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRP {
    pub trpid: u16,
    pub cbgrtrp: u16,
    pub cch: u16,
    pub cbRgb: u16,
}
impl windows_core::TypeKind for TRP {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TUISPICREATEDIALOGINSTANCEPARAMS {
    pub dwRequestID: u32,
    pub hdDlgInst: HDRVDIALOGINSTANCE,
    pub htDlgInst: u32,
    pub lpszUIDLLName: windows_core::PCWSTR,
    pub lpParams: *mut core::ffi::c_void,
    pub dwSize: u32,
}
impl windows_core::TypeKind for TUISPICREATEDIALOGINSTANCEPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TUISPICREATEDIALOGINSTANCEPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VARSTRING {
    pub dwTotalSize: u32,
    pub dwNeededSize: u32,
    pub dwUsedSize: u32,
    pub dwStringFormat: u32,
    pub dwStringSize: u32,
    pub dwStringOffset: u32,
}
impl windows_core::TypeKind for VARSTRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for VARSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ASYNC_COMPLETION = Option<unsafe extern "system" fn(dwrequestid: u32, lresult: i32)>;
pub type LINECALLBACK = Option<unsafe extern "system" fn(hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
pub type LINEEVENT = Option<unsafe extern "system" fn(htline: HTAPILINE, htcall: HTAPICALL, dwmsg: u32, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
#[cfg(feature = "Win32_System_Com")]
pub type LPGETTNEFSTREAMCODEPAGE = Option<unsafe extern "system" fn(lpstream: Option<super::super::System::Com::IStream>, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub type LPOPENTNEFSTREAM = Option<unsafe extern "system" fn(lpvsupport: *mut core::ffi::c_void, lpstream: Option<super::super::System::Com::IStream>, lpszstreamname: *const i8, ulflags: u32, lpmessage: Option<super::super::System::AddressBook::IMessage>, wkeyval: u16, lpptnef: *mut Option<ITnef>) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub type LPOPENTNEFSTREAMEX = Option<unsafe extern "system" fn(lpvsupport: *mut core::ffi::c_void, lpstream: Option<super::super::System::Com::IStream>, lpszstreamname: *const i8, ulflags: u32, lpmessage: Option<super::super::System::AddressBook::IMessage>, wkeyval: u16, lpadressbook: Option<super::super::System::AddressBook::IAddrBook>, lpptnef: *mut Option<ITnef>) -> windows_core::HRESULT>;
pub type PHONECALLBACK = Option<unsafe extern "system" fn(hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
pub type PHONEEVENT = Option<unsafe extern "system" fn(htphone: HTAPIPHONE, dwmsg: u32, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
pub type TUISPIDLLCALLBACK = Option<unsafe extern "system" fn(dwobjectid: usize, dwobjecttype: u32, lpparams: *mut core::ffi::c_void, dwsize: u32) -> i32>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
