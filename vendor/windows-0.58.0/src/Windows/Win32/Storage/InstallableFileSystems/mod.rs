#[inline]
pub unsafe fn FilterAttach<P0, P1, P2>(lpfiltername: P0, lpvolumename: P1, lpinstancename: P2, dwcreatedinstancenamelength: u32, lpcreatedinstancename: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterAttach(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_core::PWSTR) -> windows_core::HRESULT);
    FilterAttach(lpfiltername.param().abi(), lpvolumename.param().abi(), lpinstancename.param().abi(), dwcreatedinstancenamelength, core::mem::transmute(lpcreatedinstancename)).ok()
}
#[inline]
pub unsafe fn FilterAttachAtAltitude<P0, P1, P2, P3>(lpfiltername: P0, lpvolumename: P1, lpaltitude: P2, lpinstancename: P3, dwcreatedinstancenamelength: u32, lpcreatedinstancename: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterAttachAtAltitude(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpaltitude : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR, dwcreatedinstancenamelength : u32, lpcreatedinstancename : windows_core::PWSTR) -> windows_core::HRESULT);
    FilterAttachAtAltitude(lpfiltername.param().abi(), lpvolumename.param().abi(), lpaltitude.param().abi(), lpinstancename.param().abi(), dwcreatedinstancenamelength, core::mem::transmute(lpcreatedinstancename)).ok()
}
#[inline]
pub unsafe fn FilterClose<P0>(hfilter: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HFILTER>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterClose(hfilter : HFILTER) -> windows_core::HRESULT);
    FilterClose(hfilter.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FilterConnectCommunicationPort<P0>(lpportname: P0, dwoptions: u32, lpcontext: Option<*const core::ffi::c_void>, wsizeofcontext: u16, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterConnectCommunicationPort(lpportname : windows_core::PCWSTR, dwoptions : u32, lpcontext : *const core::ffi::c_void, wsizeofcontext : u16, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, hport : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    FilterConnectCommunicationPort(lpportname.param().abi(), dwoptions, core::mem::transmute(lpcontext.unwrap_or(std::ptr::null())), wsizeofcontext, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FilterCreate<P0>(lpfiltername: P0) -> windows_core::Result<HFILTER>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterCreate(lpfiltername : windows_core::PCWSTR, hfilter : *mut HFILTER) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    FilterCreate(lpfiltername.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FilterDetach<P0, P1, P2>(lpfiltername: P0, lpvolumename: P1, lpinstancename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterDetach(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR) -> windows_core::HRESULT);
    FilterDetach(lpfiltername.param().abi(), lpvolumename.param().abi(), lpinstancename.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterFindClose<P0>(hfilterfind: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterFindClose(hfilterfind : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterFindClose(hfilterfind.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterFindFirst(dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterfind: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("fltlib.dll" "system" fn FilterFindFirst(dwinformationclass : FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterfind : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterFindFirst(dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned, lpfilterfind).ok()
}
#[inline]
pub unsafe fn FilterFindNext<P0>(hfilterfind: P0, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterFindNext(hfilterfind : super::super::Foundation:: HANDLE, dwinformationclass : FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    FilterFindNext(hfilterfind.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned).ok()
}
#[inline]
pub unsafe fn FilterGetDosName<P0>(lpvolumename: P0, lpdosname: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterGetDosName(lpvolumename : windows_core::PCWSTR, lpdosname : windows_core::PWSTR, dwdosnamebuffersize : u32) -> windows_core::HRESULT);
    FilterGetDosName(lpvolumename.param().abi(), core::mem::transmute(lpdosname.as_ptr()), lpdosname.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn FilterGetInformation<P0>(hfilter: P0, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HFILTER>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterGetInformation(hfilter : HFILTER, dwinformationclass : FILTER_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    FilterGetInformation(hfilter.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn FilterGetMessage<P0>(hport: P0, lpmessagebuffer: *mut FILTER_MESSAGE_HEADER, dwmessagebuffersize: u32, lpoverlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterGetMessage(hport : super::super::Foundation:: HANDLE, lpmessagebuffer : *mut FILTER_MESSAGE_HEADER, dwmessagebuffersize : u32, lpoverlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::HRESULT);
    FilterGetMessage(hport.param().abi(), lpmessagebuffer, dwmessagebuffersize, core::mem::transmute(lpoverlapped.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn FilterInstanceClose<P0>(hinstance: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<HFILTER_INSTANCE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterInstanceClose(hinstance : HFILTER_INSTANCE) -> windows_core::HRESULT);
    FilterInstanceClose(hinstance.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterInstanceCreate<P0, P1, P2>(lpfiltername: P0, lpvolumename: P1, lpinstancename: P2) -> windows_core::Result<HFILTER_INSTANCE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterInstanceCreate(lpfiltername : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR, lpinstancename : windows_core::PCWSTR, hinstance : *mut HFILTER_INSTANCE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    FilterInstanceCreate(lpfiltername.param().abi(), lpvolumename.param().abi(), lpinstancename.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FilterInstanceFindClose<P0>(hfilterinstancefind: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterInstanceFindClose(hfilterinstancefind : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterInstanceFindClose(hfilterinstancefind.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterInstanceFindFirst<P0>(lpfiltername: P0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterinstancefind: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterInstanceFindFirst(lpfiltername : windows_core::PCWSTR, dwinformationclass : INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpfilterinstancefind : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterInstanceFindFirst(lpfiltername.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned, lpfilterinstancefind).ok()
}
#[inline]
pub unsafe fn FilterInstanceFindNext<P0>(hfilterinstancefind: P0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterInstanceFindNext(hfilterinstancefind : super::super::Foundation:: HANDLE, dwinformationclass : INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    FilterInstanceFindNext(hfilterinstancefind.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned).ok()
}
#[inline]
pub unsafe fn FilterInstanceGetInformation<P0>(hinstance: P0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<HFILTER_INSTANCE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterInstanceGetInformation(hinstance : HFILTER_INSTANCE, dwinformationclass : INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    FilterInstanceGetInformation(hinstance.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned).ok()
}
#[inline]
pub unsafe fn FilterLoad<P0>(lpfiltername: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterLoad(lpfiltername : windows_core::PCWSTR) -> windows_core::HRESULT);
    FilterLoad(lpfiltername.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterReplyMessage<P0>(hport: P0, lpreplybuffer: *const FILTER_REPLY_HEADER, dwreplybuffersize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterReplyMessage(hport : super::super::Foundation:: HANDLE, lpreplybuffer : *const FILTER_REPLY_HEADER, dwreplybuffersize : u32) -> windows_core::HRESULT);
    FilterReplyMessage(hport.param().abi(), lpreplybuffer, dwreplybuffersize).ok()
}
#[inline]
pub unsafe fn FilterSendMessage<P0>(hport: P0, lpinbuffer: *const core::ffi::c_void, dwinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, dwoutbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterSendMessage(hport : super::super::Foundation:: HANDLE, lpinbuffer : *const core::ffi::c_void, dwinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, dwoutbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    FilterSendMessage(hport.param().abi(), lpinbuffer, dwinbuffersize, core::mem::transmute(lpoutbuffer.unwrap_or(std::ptr::null_mut())), dwoutbuffersize, lpbytesreturned).ok()
}
#[inline]
pub unsafe fn FilterUnload<P0>(lpfiltername: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterUnload(lpfiltername : windows_core::PCWSTR) -> windows_core::HRESULT);
    FilterUnload(lpfiltername.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterVolumeFindClose<P0>(hvolumefind: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterVolumeFindClose(hvolumefind : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterVolumeFindClose(hvolumefind.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterVolumeFindFirst(dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumefind: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("fltlib.dll" "system" fn FilterVolumeFindFirst(dwinformationclass : FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumefind : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterVolumeFindFirst(dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned, lpvolumefind).ok()
}
#[inline]
pub unsafe fn FilterVolumeFindNext<P0>(hvolumefind: P0, dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterVolumeFindNext(hvolumefind : super::super::Foundation:: HANDLE, dwinformationclass : FILTER_VOLUME_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    FilterVolumeFindNext(hvolumefind.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned).ok()
}
#[inline]
pub unsafe fn FilterVolumeInstanceFindClose<P0>(hvolumeinstancefind: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindClose(hvolumeinstancefind : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterVolumeInstanceFindClose(hvolumeinstancefind.param().abi()).ok()
}
#[inline]
pub unsafe fn FilterVolumeInstanceFindFirst<P0>(lpvolumename: P0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumeinstancefind: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindFirst(lpvolumename : windows_core::PCWSTR, dwinformationclass : INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32, lpvolumeinstancefind : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    FilterVolumeInstanceFindFirst(lpvolumename.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned, lpvolumeinstancefind).ok()
}
#[inline]
pub unsafe fn FilterVolumeInstanceFindNext<P0>(hvolumeinstancefind: P0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("fltlib.dll" "system" fn FilterVolumeInstanceFindNext(hvolumeinstancefind : super::super::Foundation:: HANDLE, dwinformationclass : INSTANCE_INFORMATION_CLASS, lpbuffer : *mut core::ffi::c_void, dwbuffersize : u32, lpbytesreturned : *mut u32) -> windows_core::HRESULT);
    FilterVolumeInstanceFindNext(hvolumeinstancefind.param().abi(), dwinformationclass, lpbuffer, dwbuffersize, lpbytesreturned).ok()
}
pub const FILTER_NAME_MAX_CHARS: u32 = 255u32;
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_ASI_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_ASI_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_IASIL_DETACHED_VOLUME: u32 = 1u32;
pub const FLTFL_IASIM_DETACHED_VOLUME: u32 = 1u32;
pub const FLTFL_IASI_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_IASI_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_VSI_DETACHED_VOLUME: u32 = 1u32;
pub const FLT_FSTYPE_BSUDF: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(12i32);
pub const FLT_FSTYPE_CDFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(4i32);
pub const FLT_FSTYPE_CIMFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(30i32);
pub const FLT_FSTYPE_CSVFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(27i32);
pub const FLT_FSTYPE_EXFAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(22i32);
pub const FLT_FSTYPE_FAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(3i32);
pub const FLT_FSTYPE_FS_REC: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(19i32);
pub const FLT_FSTYPE_GPFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(24i32);
pub const FLT_FSTYPE_INCD: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(20i32);
pub const FLT_FSTYPE_INCD_FAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(21i32);
pub const FLT_FSTYPE_LANMAN: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(6i32);
pub const FLT_FSTYPE_MSFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(26i32);
pub const FLT_FSTYPE_MS_NETWARE: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(10i32);
pub const FLT_FSTYPE_MUP: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(13i32);
pub const FLT_FSTYPE_NETWARE: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(11i32);
pub const FLT_FSTYPE_NFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(9i32);
pub const FLT_FSTYPE_NPFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(25i32);
pub const FLT_FSTYPE_NTFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(2i32);
pub const FLT_FSTYPE_OPENAFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(29i32);
pub const FLT_FSTYPE_PSFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(23i32);
pub const FLT_FSTYPE_RAW: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(1i32);
pub const FLT_FSTYPE_RDPDR: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(8i32);
pub const FLT_FSTYPE_REFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(28i32);
pub const FLT_FSTYPE_ROXIO_UDF1: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(15i32);
pub const FLT_FSTYPE_ROXIO_UDF2: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(16i32);
pub const FLT_FSTYPE_ROXIO_UDF3: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(17i32);
pub const FLT_FSTYPE_RSFX: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(14i32);
pub const FLT_FSTYPE_TACIT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(18i32);
pub const FLT_FSTYPE_UDFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(5i32);
pub const FLT_FSTYPE_UNKNOWN: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(0i32);
pub const FLT_FSTYPE_WEBDAV: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(7i32);
pub const FLT_PORT_FLAG_SYNC_HANDLE: u32 = 1u32;
pub const FilterAggregateBasicInformation: FILTER_INFORMATION_CLASS = FILTER_INFORMATION_CLASS(1i32);
pub const FilterAggregateStandardInformation: FILTER_INFORMATION_CLASS = FILTER_INFORMATION_CLASS(2i32);
pub const FilterFullInformation: FILTER_INFORMATION_CLASS = FILTER_INFORMATION_CLASS(0i32);
pub const FilterVolumeBasicInformation: FILTER_VOLUME_INFORMATION_CLASS = FILTER_VOLUME_INFORMATION_CLASS(0i32);
pub const FilterVolumeStandardInformation: FILTER_VOLUME_INFORMATION_CLASS = FILTER_VOLUME_INFORMATION_CLASS(1i32);
pub const INSTANCE_NAME_MAX_CHARS: u32 = 255u32;
pub const InstanceAggregateStandardInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(3i32);
pub const InstanceBasicInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(0i32);
pub const InstanceFullInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(2i32);
pub const InstancePartialInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(1i32);
pub const VOLUME_NAME_MAX_CHARS: u32 = 1024u32;
pub const WNNC_CRED_MANAGER: u32 = 4294901760u32;
pub const WNNC_NET_10NET: u32 = 327680u32;
pub const WNNC_NET_3IN1: u32 = 2555904u32;
pub const WNNC_NET_9P: u32 = 4718592u32;
pub const WNNC_NET_9TILES: u32 = 589824u32;
pub const WNNC_NET_APPLETALK: u32 = 1245184u32;
pub const WNNC_NET_AS400: u32 = 720896u32;
pub const WNNC_NET_AURISTOR_FS: u32 = 4587520u32;
pub const WNNC_NET_AVID: u32 = 1703936u32;
pub const WNNC_NET_AVID1: u32 = 3801088u32;
pub const WNNC_NET_BMC: u32 = 1572864u32;
pub const WNNC_NET_BWNFS: u32 = 1048576u32;
pub const WNNC_NET_CLEARCASE: u32 = 1441792u32;
pub const WNNC_NET_COGENT: u32 = 1114112u32;
pub const WNNC_NET_CSC: u32 = 2490368u32;
pub const WNNC_NET_DAV: u32 = 3014656u32;
pub const WNNC_NET_DCE: u32 = 1638400u32;
pub const WNNC_NET_DECORB: u32 = 2097152u32;
pub const WNNC_NET_DFS: u32 = 3866624u32;
pub const WNNC_NET_DISTINCT: u32 = 2293760u32;
pub const WNNC_NET_DOCUSHARE: u32 = 4521984u32;
pub const WNNC_NET_DOCUSPACE: u32 = 1769472u32;
pub const WNNC_NET_DRIVEONWEB: u32 = 4063232u32;
pub const WNNC_NET_EXIFS: u32 = 2949120u32;
pub const WNNC_NET_EXTENDNET: u32 = 2686976u32;
pub const WNNC_NET_FARALLON: u32 = 1179648u32;
pub const WNNC_NET_FJ_REDIR: u32 = 2228224u32;
pub const WNNC_NET_FOXBAT: u32 = 2818048u32;
pub const WNNC_NET_FRONTIER: u32 = 1507328u32;
pub const WNNC_NET_FTP_NFS: u32 = 786432u32;
pub const WNNC_NET_GOOGLE: u32 = 4390912u32;
pub const WNNC_NET_HOB_NFS: u32 = 3276800u32;
pub const WNNC_NET_IBMAL: u32 = 3407872u32;
pub const WNNC_NET_INTERGRAPH: u32 = 1310720u32;
pub const WNNC_NET_KNOWARE: u32 = 3080192u32;
pub const WNNC_NET_KWNP: u32 = 3932160u32;
pub const WNNC_NET_LANMAN: u32 = 131072u32;
pub const WNNC_NET_LANSTEP: u32 = 524288u32;
pub const WNNC_NET_LANTASTIC: u32 = 655360u32;
pub const WNNC_NET_LIFENET: u32 = 917504u32;
pub const WNNC_NET_LOCK: u32 = 3473408u32;
pub const WNNC_NET_LOCUS: u32 = 393216u32;
pub const WNNC_NET_MANGOSOFT: u32 = 1835008u32;
pub const WNNC_NET_MASFAX: u32 = 3211264u32;
pub const WNNC_NET_MFILES: u32 = 4259840u32;
pub const WNNC_NET_MSNET: u32 = 65536u32;
pub const WNNC_NET_MS_NFS: u32 = 4325376u32;
pub const WNNC_NET_NDFS: u32 = 4456448u32;
pub const WNNC_NET_NETWARE: u32 = 196608u32;
pub const WNNC_NET_OBJECT_DIRE: u32 = 3145728u32;
pub const WNNC_NET_OPENAFS: u32 = 3735552u32;
pub const WNNC_NET_PATHWORKS: u32 = 851968u32;
pub const WNNC_NET_POWERLAN: u32 = 983040u32;
pub const WNNC_NET_PROTSTOR: u32 = 2162688u32;
pub const WNNC_NET_QUINCY: u32 = 3670016u32;
pub const WNNC_NET_RDR2SAMPLE: u32 = 2424832u32;
pub const WNNC_NET_RIVERFRONT1: u32 = 1966080u32;
pub const WNNC_NET_RIVERFRONT2: u32 = 2031616u32;
pub const WNNC_NET_RSFX: u32 = 4194304u32;
pub const WNNC_NET_SECUREAGENT: u32 = 4653056u32;
pub const WNNC_NET_SERNET: u32 = 1900544u32;
pub const WNNC_NET_SHIVA: u32 = 3342336u32;
pub const WNNC_NET_SMB: u32 = 131072u32;
pub const WNNC_NET_SRT: u32 = 3604480u32;
pub const WNNC_NET_STAC: u32 = 2752512u32;
pub const WNNC_NET_SUN_PC_NFS: u32 = 458752u32;
pub const WNNC_NET_SYMFONET: u32 = 1376256u32;
pub const WNNC_NET_TERMSRV: u32 = 3538944u32;
pub const WNNC_NET_TWINS: u32 = 2359296u32;
pub const WNNC_NET_VINES: u32 = 262144u32;
pub const WNNC_NET_VMWARE: u32 = 4128768u32;
pub const WNNC_NET_YAHOO: u32 = 2883584u32;
pub const WNNC_NET_ZENWORKS: u32 = 3997696u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILTER_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for FILTER_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILTER_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILTER_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILTER_VOLUME_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for FILTER_VOLUME_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILTER_VOLUME_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILTER_VOLUME_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FLT_FILESYSTEM_TYPE(pub i32);
impl windows_core::TypeKind for FLT_FILESYSTEM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FLT_FILESYSTEM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FLT_FILESYSTEM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INSTANCE_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for INSTANCE_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INSTANCE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INSTANCE_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_BASIC_INFORMATION_0,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_BASIC_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_1,
    pub LegacyFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_0,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_STANDARD_INFORMATION_0,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_STANDARD_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_1,
    pub LegacyFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_0,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl windows_core::TypeKind for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBuffer: [u16; 1],
}
impl windows_core::TypeKind for FILTER_FULL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_MESSAGE_HEADER {
    pub ReplyLength: u32,
    pub MessageId: u64,
}
impl windows_core::TypeKind for FILTER_MESSAGE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_REPLY_HEADER {
    pub Status: super::super::Foundation::NTSTATUS,
    pub MessageId: u64,
}
impl windows_core::TypeKind for FILTER_REPLY_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_REPLY_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_VOLUME_BASIC_INFORMATION {
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl windows_core::TypeKind for FILTER_VOLUME_BASIC_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_VOLUME_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_VOLUME_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub FrameID: u32,
    pub FileSystemType: FLT_FILESYSTEM_TYPE,
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl windows_core::TypeKind for FILTER_VOLUME_STANDARD_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FILTER_VOLUME_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HFILTER(pub isize);
impl HFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HFILTER {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = FilterClose(*self);
        }
    }
}
impl Default for HFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HFILTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HFILTER_INSTANCE(pub isize);
impl HFILTER_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HFILTER_INSTANCE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = FilterInstanceClose(*self);
        }
    }
}
impl Default for HFILTER_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HFILTER_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0,
}
impl windows_core::TypeKind for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1,
    pub LegacyFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0,
}
impl windows_core::TypeKind for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
impl windows_core::TypeKind for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub FrameID: u32,
    pub VolumeFileSystemType: FLT_FILESYSTEM_TYPE,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
impl windows_core::TypeKind for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INSTANCE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
}
impl windows_core::TypeKind for INSTANCE_BASIC_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for INSTANCE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INSTANCE_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
impl windows_core::TypeKind for INSTANCE_FULL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for INSTANCE_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INSTANCE_PARTIAL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
}
impl windows_core::TypeKind for INSTANCE_PARTIAL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for INSTANCE_PARTIAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
