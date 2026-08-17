windows_core::imp::define_interface!(IDirectMusic, IDirectMusic_Vtbl, 0x6536115a_7b2d_11d2_ba18_0000f875ac12);
impl std::ops::Deref for IDirectMusic {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusic, windows_core::IUnknown);
impl IDirectMusic {
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumPort)(windows_core::Interface::as_raw(self), dwindex, pportcaps).ok()
    }
    pub unsafe fn CreateMusicBuffer<P0>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut Option<IDirectMusicBuffer>, punkouter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateMusicBuffer)(windows_core::Interface::as_raw(self), pbufferdesc, core::mem::transmute(ppbuffer), punkouter.param().abi()).ok()
    }
    pub unsafe fn CreatePort<P0>(&self, rclsidport: *const windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut Option<IDirectMusicPort>, punkouter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreatePort)(windows_core::Interface::as_raw(self), rclsidport, pportparams, core::mem::transmute(ppport), punkouter.param().abi()).ok()
    }
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumMasterClock)(windows_core::Interface::as_raw(self), dwindex, lpclockinfo).ok()
    }
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut windows_core::GUID, ppreferenceclock: *mut Option<super::super::IReferenceClock>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMasterClock)(windows_core::Interface::as_raw(self), pguidclock, core::mem::transmute(ppreferenceclock)).ok()
    }
    pub unsafe fn SetMasterClock(&self, rguidclock: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMasterClock)(windows_core::Interface::as_raw(self), rguidclock).ok()
    }
    pub unsafe fn Activate<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultPort)(windows_core::Interface::as_raw(self), pguidport).ok()
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<P0, P1>(&self, pdirectsound: P0, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectSound::IDirectSound>,
        P1: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetDirectSound)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), hwnd.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectMusic_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumPort: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMUS_PORTCAPS) -> windows_core::HRESULT,
    pub CreateMusicBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_BUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePort: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DMUS_PORTPARAMS8, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DMUS_CLOCKINFO8) -> windows_core::HRESULT,
    pub GetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDefaultPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
}
windows_core::imp::define_interface!(IDirectMusic8, IDirectMusic8_Vtbl, 0x2d3629f7_813d_4939_8508_f05c6b75fd97);
impl std::ops::Deref for IDirectMusic8 {
    type Target = IDirectMusic;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusic8, windows_core::IUnknown, IDirectMusic);
impl IDirectMusic8 {
    pub unsafe fn SetExternalMasterClock<P0>(&self, pclock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::IReferenceClock>,
    {
        (windows_core::Interface::vtable(self).SetExternalMasterClock)(windows_core::Interface::as_raw(self), pclock.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectMusic8_Vtbl {
    pub base__: IDirectMusic_Vtbl,
    pub SetExternalMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicBuffer, IDirectMusicBuffer_Vtbl, 0xd2ac2878_b39b_11d1_8704_00600893b1bd);
impl std::ops::Deref for IDirectMusicBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicBuffer, windows_core::IUnknown);
impl IDirectMusicBuffer {
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TotalTime(&self, prttime: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TotalTime)(windows_core::Interface::as_raw(self), prttime).ok()
    }
    pub unsafe fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PackStructured)(windows_core::Interface::as_raw(self), rt, dwchannelgroup, dwchannelmessage).ok()
    }
    pub unsafe fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PackUnstructured)(windows_core::Interface::as_raw(self), rt, dwchannelgroup, cb, lpb).ok()
    }
    pub unsafe fn ResetReadPtr(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetReadPtr)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextEvent)(windows_core::Interface::as_raw(self), prt, pdwchannelgroup, pdwlength, ppdata).ok()
    }
    pub unsafe fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRawBufferPtr)(windows_core::Interface::as_raw(self), ppdata).ok()
    }
    pub unsafe fn GetStartTime(&self, prt: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStartTime)(windows_core::Interface::as_raw(self), prt).ok()
    }
    pub unsafe fn GetUsedBytes(&self, pcb: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUsedBytes)(windows_core::Interface::as_raw(self), pcb).ok()
    }
    pub unsafe fn GetMaxBytes(&self, pcb: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMaxBytes)(windows_core::Interface::as_raw(self), pcb).ok()
    }
    pub unsafe fn GetBufferFormat(&self, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBufferFormat)(windows_core::Interface::as_raw(self), pguidformat).ok()
    }
    pub unsafe fn SetStartTime(&self, rt: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), rt).ok()
    }
    pub unsafe fn SetUsedBytes(&self, cb: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUsedBytes)(windows_core::Interface::as_raw(self), cb).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub PackStructured: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, u32) -> windows_core::HRESULT,
    pub PackUnstructured: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, u32, *mut u8) -> windows_core::HRESULT,
    pub ResetReadPtr: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64, *mut u32, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetRawBufferPtr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub GetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetUsedBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBufferFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetUsedBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicCollection, IDirectMusicCollection_Vtbl, 0xd2ac287c_b39b_11d1_8704_00600893b1bd);
impl std::ops::Deref for IDirectMusicCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicCollection, windows_core::IUnknown);
impl IDirectMusicCollection {
    pub unsafe fn GetInstrument(&self, dwpatch: u32) -> windows_core::Result<IDirectMusicInstrument> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstrument)(windows_core::Interface::as_raw(self), dwpatch, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumInstrument<P0>(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: P0, dwnamelen: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EnumInstrument)(windows_core::Interface::as_raw(self), dwindex, pdwpatch, pwszname.param().abi(), dwnamelen).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicDownload, IDirectMusicDownload_Vtbl, 0xd2ac287b_b39b_11d1_8704_00600893b1bd);
impl std::ops::Deref for IDirectMusicDownload {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicDownload, windows_core::IUnknown);
impl IDirectMusicDownload {
    pub unsafe fn GetBuffer(&self, ppvbuffer: *mut *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), ppvbuffer, pdwsize).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicDownload_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicDownloadedInstrument, IDirectMusicDownloadedInstrument_Vtbl, 0xd2ac287e_b39b_11d1_8704_00600893b1bd);
impl std::ops::Deref for IDirectMusicDownloadedInstrument {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicDownloadedInstrument, windows_core::IUnknown);
impl IDirectMusicDownloadedInstrument {}
#[repr(C)]
pub struct IDirectMusicDownloadedInstrument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IDirectMusicInstrument, IDirectMusicInstrument_Vtbl, 0xd2ac287d_b39b_11d1_8704_00600893b1bd);
impl std::ops::Deref for IDirectMusicInstrument {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicInstrument, windows_core::IUnknown);
impl IDirectMusicInstrument {
    pub unsafe fn GetPatch(&self, pdwpatch: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPatch)(windows_core::Interface::as_raw(self), pdwpatch).ok()
    }
    pub unsafe fn SetPatch(&self, dwpatch: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPatch)(windows_core::Interface::as_raw(self), dwpatch).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicInstrument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicPort, IDirectMusicPort_Vtbl, 0x08f2d8c9_37c2_11d2_b9f9_0000f875ac12);
impl std::ops::Deref for IDirectMusicPort {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicPort, windows_core::IUnknown);
impl IDirectMusicPort {
    pub unsafe fn PlayBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicBuffer>,
    {
        (windows_core::Interface::vtable(self).PlayBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok()
    }
    pub unsafe fn SetReadNotificationHandle<P0>(&self, hevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetReadNotificationHandle)(windows_core::Interface::as_raw(self), hevent.param().abi()).ok()
    }
    pub unsafe fn Read<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicBuffer>,
    {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok()
    }
    pub unsafe fn DownloadInstrument<P0>(&self, pinstrument: P0, ppdownloadedinstrument: *mut Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicInstrument>,
    {
        (windows_core::Interface::vtable(self).DownloadInstrument)(windows_core::Interface::as_raw(self), pinstrument.param().abi(), core::mem::transmute(ppdownloadedinstrument), pnoteranges, dwnumnoteranges).ok()
    }
    pub unsafe fn UnloadInstrument<P0>(&self, pdownloadedinstrument: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicDownloadedInstrument>,
    {
        (windows_core::Interface::vtable(self).UnloadInstrument)(windows_core::Interface::as_raw(self), pdownloadedinstrument.param().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLatencyClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRunningStats)(windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn Compact(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pportcaps).ok()
    }
    #[cfg(feature = "Win32_System_IO")]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeviceIoControl)(windows_core::Interface::as_raw(self), dwiocontrolcode, lpinbuffer, ninbuffersize, lpoutbuffer, noutbuffersize, lpbytesreturned, lpoverlapped).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNumChannelGroups)(windows_core::Interface::as_raw(self), dwchannelgroups).ok()
    }
    pub unsafe fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNumChannelGroups)(windows_core::Interface::as_raw(self), pdwchannelgroups).ok()
    }
    pub unsafe fn Activate<P0>(&self, factive: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), factive.param().abi()).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, pdwpriority).ok()
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<P0, P1>(&self, pdirectsound: P0, pdirectsoundbuffer: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectSound::IDirectSound>,
        P1: windows_core::Param<super::DirectSound::IDirectSoundBuffer>,
    {
        (windows_core::Interface::vtable(self).SetDirectSound)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), pdirectsoundbuffer.param().abi()).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwaveformatex, pdwwaveformatexsize, pdwbuffersize).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicPort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PlayBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReadNotificationHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DownloadInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut DMUS_NOTERANGE, u32) -> windows_core::HRESULT,
    pub UnloadInstrument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRunningStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_PORTCAPS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_IO")]
    pub DeviceIoControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32, *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_IO"))]
    DeviceIoControl: usize,
    pub SetNumChannelGroups: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNumChannelGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WAVEFORMATEX, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicPortDownload, IDirectMusicPortDownload_Vtbl, 0xd2ac287a_b39b_11d1_8704_00600893b1bd);
impl std::ops::Deref for IDirectMusicPortDownload {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicPortDownload, windows_core::IUnknown);
impl IDirectMusicPortDownload {
    pub unsafe fn GetBuffer(&self, dwdlid: u32) -> windows_core::Result<IDirectMusicDownload> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), dwdlid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AllocateBuffer(&self, dwsize: u32) -> windows_core::Result<IDirectMusicDownload> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).AllocateBuffer)(windows_core::Interface::as_raw(self), dwsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDLId)(windows_core::Interface::as_raw(self), pdwstartdlid, dwcount).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAppend)(windows_core::Interface::as_raw(self), pdwappend).ok()
    }
    pub unsafe fn Download<P0>(&self, pidmdownload: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicDownload>,
    {
        (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), pidmdownload.param().abi()).ok()
    }
    pub unsafe fn Unload<P0>(&self, pidmdownload: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicDownload>,
    {
        (windows_core::Interface::vtable(self).Unload)(windows_core::Interface::as_raw(self), pidmdownload.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicPortDownload_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDLId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub GetAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicSynth, IDirectMusicSynth_Vtbl, 0x09823661_5c85_11d2_afa6_00aa0024d8b6);
impl std::ops::Deref for IDirectMusicSynth {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicSynth, windows_core::IUnknown);
impl IDirectMusicSynth {
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pportparams).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNumChannelGroups)(windows_core::Interface::as_raw(self), dwgroups).ok()
    }
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut core::ffi::c_void, pbfree: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), phdownload, pvdata, pbfree).ok()
    }
    pub unsafe fn Unload<P0, P1>(&self, hdownload: P0, lpfreehandle: isize, huserdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).Unload)(windows_core::Interface::as_raw(self), hdownload.param().abi(), lpfreehandle, huserdata.param().abi()).ok()
    }
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PlayBuffer)(windows_core::Interface::as_raw(self), rt, pbbuffer, cbbuffer).ok()
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRunningStats)(windows_core::Interface::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPortCaps)(windows_core::Interface::as_raw(self), pcaps).ok()
    }
    pub unsafe fn SetMasterClock<P0>(&self, pclock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::IReferenceClock>,
    {
        (windows_core::Interface::vtable(self).SetMasterClock)(windows_core::Interface::as_raw(self), pclock.param().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLatencyClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Activate<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    pub unsafe fn SetSynthSink<P0>(&self, psynthsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicSynthSink>,
    {
        (windows_core::Interface::vtable(self).SetSynthSink)(windows_core::Interface::as_raw(self), psynthsink.param().abi()).ok()
    }
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), pbuffer, dwlength, llposition).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChannelPriority)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, pdwpriority).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwaveformatex, pdwwaveformatexsize).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAppend)(windows_core::Interface::as_raw(self), pdwappend).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicSynth_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_PORTPARAMS8) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNumChannelGroups: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HANDLE, *mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Unload: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HANDLE, isize, super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub PlayBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut u8, u32) -> windows_core::HRESULT,
    pub GetRunningStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT,
    pub GetPortCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DMUS_PORTCAPS) -> windows_core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetSynthSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16, u32, i64) -> windows_core::HRESULT,
    pub SetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub GetChannelPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    pub GetAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicSynth8, IDirectMusicSynth8_Vtbl, 0x53cab625_2711_4c9f_9de7_1b7f925f6fc8);
impl std::ops::Deref for IDirectMusicSynth8 {
    type Target = IDirectMusicSynth;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicSynth8, windows_core::IUnknown, IDirectMusicSynth);
impl IDirectMusicSynth8 {
    pub unsafe fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PlayVoice)(windows_core::Interface::as_raw(self), rt, dwvoiceid, dwchannelgroup, dwchannel, dwdlid, prpitch, vrvolume, stvoicestart, stloopstart, stloopend).ok()
    }
    pub unsafe fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopVoice)(windows_core::Interface::as_raw(self), rt, dwvoiceid).ok()
    }
    pub unsafe fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVoiceState)(windows_core::Interface::as_raw(self), dwvoice, cbvoice, dwvoicestate).ok()
    }
    pub unsafe fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), dwdownloadid, dwflags).ok()
    }
    pub unsafe fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssignChannelToBuses)(windows_core::Interface::as_raw(self), dwchannelgroup, dwchannel, pdwbuses, cbuses).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicSynth8_Vtbl {
    pub base__: IDirectMusicSynth_Vtbl,
    pub PlayVoice: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, u32, u32, u32, i32, i32, u64, u64, u64) -> windows_core::HRESULT,
    pub StopVoice: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32) -> windows_core::HRESULT,
    pub GetVoiceState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut DMUS_VOICE_STATE) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub AssignChannelToBuses: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicSynthSink, IDirectMusicSynthSink_Vtbl, 0x09823663_5c85_11d2_afa6_00aa0024d8b6);
impl std::ops::Deref for IDirectMusicSynthSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicSynthSink, windows_core::IUnknown);
impl IDirectMusicSynthSink {
    pub unsafe fn Init<P0>(&self, psynth: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicSynth>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), psynth.param().abi()).ok()
    }
    pub unsafe fn SetMasterClock<P0>(&self, pclock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::IReferenceClock>,
    {
        (windows_core::Interface::vtable(self).SetMasterClock)(windows_core::Interface::as_raw(self), pclock.param().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLatencyClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Activate<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    pub unsafe fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SampleToRefTime)(windows_core::Interface::as_raw(self), llsampletime, prftime).ok()
    }
    pub unsafe fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefTimeToSample)(windows_core::Interface::as_raw(self), rftime, pllsampletime).ok()
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<P0, P1>(&self, pdirectsound: P0, pdirectsoundbuffer: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectSound::IDirectSound>,
        P1: windows_core::Param<super::DirectSound::IDirectSoundBuffer>,
    {
        (windows_core::Interface::vtable(self).SetDirectSound)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), pdirectsoundbuffer.param().abi()).ok()
    }
    pub unsafe fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesiredBufferSize)(windows_core::Interface::as_raw(self), pdwbuffersizeinsamples).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicSynthSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SampleToRefTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut i64) -> windows_core::HRESULT,
    pub RefTimeToSample: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut i64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
    pub GetDesiredBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectMusicThru, IDirectMusicThru_Vtbl, 0xced153e7_3606_11d2_b9f9_0000f875ac12);
impl std::ops::Deref for IDirectMusicThru {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectMusicThru, windows_core::IUnknown);
impl IDirectMusicThru {
    pub unsafe fn ThruChannel<P0>(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectMusicPort>,
    {
        (windows_core::Interface::vtable(self).ThruChannel)(windows_core::Interface::as_raw(self), dwsourcechannelgroup, dwsourcechannel, dwdestinationchannelgroup, dwdestinationchannel, pdestinationport.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectMusicThru_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThruChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const CLSID_DirectMusic: windows_core::GUID = windows_core::GUID::from_u128(0x636b9f10_0c7d_11d1_95b2_0020afdc7421);
pub const CLSID_DirectMusicCollection: windows_core::GUID = windows_core::GUID::from_u128(0x480ff4b0_28b2_11d1_bef7_00c04fbf8fef);
pub const CLSID_DirectMusicSynth: windows_core::GUID = windows_core::GUID::from_u128(0x58c2b4d0_46e7_11d1_89ac_00a0c9054129);
pub const CLSID_DirectMusicSynthSink: windows_core::GUID = windows_core::GUID::from_u128(0xaec17ce3_a514_11d1_afa6_00aa0024d8b6);
pub const CLSID_DirectSoundPrivate: windows_core::GUID = windows_core::GUID::from_u128(0x11ab3ec0_25ec_11d1_a4d8_00c04fc28aca);
pub const CONN_DST_ATTENUATION: u32 = 1u32;
pub const CONN_DST_CENTER: u32 = 18u32;
pub const CONN_DST_CHORUS: u32 = 128u32;
pub const CONN_DST_EG1_ATTACKTIME: u32 = 518u32;
pub const CONN_DST_EG1_DECAYTIME: u32 = 519u32;
pub const CONN_DST_EG1_DELAYTIME: u32 = 523u32;
pub const CONN_DST_EG1_HOLDTIME: u32 = 524u32;
pub const CONN_DST_EG1_RELEASETIME: u32 = 521u32;
pub const CONN_DST_EG1_SHUTDOWNTIME: u32 = 525u32;
pub const CONN_DST_EG1_SUSTAINLEVEL: u32 = 522u32;
pub const CONN_DST_EG2_ATTACKTIME: u32 = 778u32;
pub const CONN_DST_EG2_DECAYTIME: u32 = 779u32;
pub const CONN_DST_EG2_DELAYTIME: u32 = 783u32;
pub const CONN_DST_EG2_HOLDTIME: u32 = 784u32;
pub const CONN_DST_EG2_RELEASETIME: u32 = 781u32;
pub const CONN_DST_EG2_SUSTAINLEVEL: u32 = 782u32;
pub const CONN_DST_FILTER_CUTOFF: u32 = 1280u32;
pub const CONN_DST_FILTER_Q: u32 = 1281u32;
pub const CONN_DST_GAIN: u32 = 1u32;
pub const CONN_DST_KEYNUMBER: u32 = 5u32;
pub const CONN_DST_LEFT: u32 = 16u32;
pub const CONN_DST_LEFTREAR: u32 = 19u32;
pub const CONN_DST_LFE_CHANNEL: u32 = 21u32;
pub const CONN_DST_LFO_FREQUENCY: u32 = 260u32;
pub const CONN_DST_LFO_STARTDELAY: u32 = 261u32;
pub const CONN_DST_NONE: u32 = 0u32;
pub const CONN_DST_PAN: u32 = 4u32;
pub const CONN_DST_PITCH: u32 = 3u32;
pub const CONN_DST_REVERB: u32 = 129u32;
pub const CONN_DST_RIGHT: u32 = 17u32;
pub const CONN_DST_RIGHTREAR: u32 = 20u32;
pub const CONN_DST_VIB_FREQUENCY: u32 = 276u32;
pub const CONN_DST_VIB_STARTDELAY: u32 = 277u32;
pub const CONN_SRC_CC1: u32 = 129u32;
pub const CONN_SRC_CC10: u32 = 138u32;
pub const CONN_SRC_CC11: u32 = 139u32;
pub const CONN_SRC_CC7: u32 = 135u32;
pub const CONN_SRC_CC91: u32 = 219u32;
pub const CONN_SRC_CC93: u32 = 221u32;
pub const CONN_SRC_CHANNELPRESSURE: u32 = 8u32;
pub const CONN_SRC_EG1: u32 = 4u32;
pub const CONN_SRC_EG2: u32 = 5u32;
pub const CONN_SRC_KEYNUMBER: u32 = 3u32;
pub const CONN_SRC_KEYONVELOCITY: u32 = 2u32;
pub const CONN_SRC_LFO: u32 = 1u32;
pub const CONN_SRC_MONOPRESSURE: u32 = 10u32;
pub const CONN_SRC_NONE: u32 = 0u32;
pub const CONN_SRC_PITCHWHEEL: u32 = 6u32;
pub const CONN_SRC_POLYPRESSURE: u32 = 7u32;
pub const CONN_SRC_VIBRATO: u32 = 9u32;
pub const CONN_TRN_CONCAVE: u32 = 1u32;
pub const CONN_TRN_CONVEX: u32 = 2u32;
pub const CONN_TRN_NONE: u32 = 0u32;
pub const CONN_TRN_SWITCH: u32 = 3u32;
pub const DAUD_CHAN10_VOICE_PRIORITY_OFFSET: u32 = 15u32;
pub const DAUD_CHAN11_VOICE_PRIORITY_OFFSET: u32 = 5u32;
pub const DAUD_CHAN12_VOICE_PRIORITY_OFFSET: u32 = 4u32;
pub const DAUD_CHAN13_VOICE_PRIORITY_OFFSET: u32 = 3u32;
pub const DAUD_CHAN14_VOICE_PRIORITY_OFFSET: u32 = 2u32;
pub const DAUD_CHAN15_VOICE_PRIORITY_OFFSET: u32 = 1u32;
pub const DAUD_CHAN16_VOICE_PRIORITY_OFFSET: u32 = 0u32;
pub const DAUD_CHAN1_VOICE_PRIORITY_OFFSET: u32 = 14u32;
pub const DAUD_CHAN2_VOICE_PRIORITY_OFFSET: u32 = 13u32;
pub const DAUD_CHAN3_VOICE_PRIORITY_OFFSET: u32 = 12u32;
pub const DAUD_CHAN4_VOICE_PRIORITY_OFFSET: u32 = 11u32;
pub const DAUD_CHAN5_VOICE_PRIORITY_OFFSET: u32 = 10u32;
pub const DAUD_CHAN6_VOICE_PRIORITY_OFFSET: u32 = 9u32;
pub const DAUD_CHAN7_VOICE_PRIORITY_OFFSET: u32 = 8u32;
pub const DAUD_CHAN8_VOICE_PRIORITY_OFFSET: u32 = 7u32;
pub const DAUD_CHAN9_VOICE_PRIORITY_OFFSET: u32 = 6u32;
pub const DAUD_CRITICAL_VOICE_PRIORITY: u32 = 4026531840u32;
pub const DAUD_HIGH_VOICE_PRIORITY: u32 = 3221225472u32;
pub const DAUD_LOW_VOICE_PRIORITY: u32 = 1073741824u32;
pub const DAUD_PERSIST_VOICE_PRIORITY: u32 = 268435456u32;
pub const DAUD_STANDARD_VOICE_PRIORITY: u32 = 2147483648u32;
pub const DIRECTSOUNDDEVICE_DATAFLOW_CAPTURE: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(1i32);
pub const DIRECTSOUNDDEVICE_DATAFLOW_RENDER: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(0i32);
pub const DIRECTSOUNDDEVICE_TYPE_EMULATED: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(0i32);
pub const DIRECTSOUNDDEVICE_TYPE_VXD: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(1i32);
pub const DIRECTSOUNDDEVICE_TYPE_WDM: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(2i32);
pub const DLSID_GMInHardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f24_c364_11d1_a760_0000f875ac12);
pub const DLSID_GSInHardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f25_c364_11d1_a760_0000f875ac12);
pub const DLSID_ManufacturersID: windows_core::GUID = windows_core::GUID::from_u128(0xb03e1181_8095_11d2_a1ef_00600833dbd8);
pub const DLSID_ProductID: windows_core::GUID = windows_core::GUID::from_u128(0xb03e1182_8095_11d2_a1ef_00600833dbd8);
pub const DLSID_SampleMemorySize: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const DLSID_SamplePlaybackRate: windows_core::GUID = windows_core::GUID::from_u128(0x2a91f713_a4bf_11d2_bbdf_00600833dbd8);
pub const DLSID_SupportsDLS1: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f27_c364_11d1_a760_0000f875ac12);
pub const DLSID_SupportsDLS2: windows_core::GUID = windows_core::GUID::from_u128(0xf14599e5_4689_11d2_afa6_00aa0024d8b6);
pub const DLSID_XGInHardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f26_c364_11d1_a760_0000f875ac12);
pub const DLS_CDL_ADD: u32 = 4u32;
pub const DLS_CDL_AND: u32 = 1u32;
pub const DLS_CDL_CONST: u32 = 16u32;
pub const DLS_CDL_DIVIDE: u32 = 7u32;
pub const DLS_CDL_EQ: u32 = 14u32;
pub const DLS_CDL_GE: u32 = 13u32;
pub const DLS_CDL_GT: u32 = 12u32;
pub const DLS_CDL_LE: u32 = 11u32;
pub const DLS_CDL_LOGICAL_AND: u32 = 8u32;
pub const DLS_CDL_LOGICAL_OR: u32 = 9u32;
pub const DLS_CDL_LT: u32 = 10u32;
pub const DLS_CDL_MULTIPLY: u32 = 6u32;
pub const DLS_CDL_NOT: u32 = 15u32;
pub const DLS_CDL_OR: u32 = 2u32;
pub const DLS_CDL_QUERY: u32 = 17u32;
pub const DLS_CDL_QUERYSUPPORTED: u32 = 18u32;
pub const DLS_CDL_SUBTRACT: u32 = 5u32;
pub const DLS_CDL_XOR: u32 = 3u32;
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
pub const DMUS_CLOCK_SYSTEM: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(0i32);
pub const DMUS_CLOCK_WAVE: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(1i32);
pub const DMUS_DEFAULT_SIZE_OFFSETTABLE: u32 = 1u32;
pub const DMUS_DOWNLOADINFO_INSTRUMENT: u32 = 1u32;
pub const DMUS_DOWNLOADINFO_INSTRUMENT2: u32 = 3u32;
pub const DMUS_DOWNLOADINFO_ONESHOTWAVE: u32 = 6u32;
pub const DMUS_DOWNLOADINFO_STREAMINGWAVE: u32 = 5u32;
pub const DMUS_DOWNLOADINFO_WAVE: u32 = 2u32;
pub const DMUS_DOWNLOADINFO_WAVEARTICULATION: u32 = 4u32;
pub const DMUS_EFFECT_CHORUS: u32 = 2u32;
pub const DMUS_EFFECT_DELAY: u32 = 4u32;
pub const DMUS_EFFECT_NONE: u32 = 0u32;
pub const DMUS_EFFECT_REVERB: u32 = 1u32;
pub const DMUS_EVENT_STRUCTURED: u32 = 1u32;
pub const DMUS_INSTRUMENT_GM_INSTRUMENT: u32 = 1u32;
pub const DMUS_MAX_DESCRIPTION: u32 = 128u32;
pub const DMUS_MAX_DRIVER: u32 = 128u32;
pub const DMUS_MIN_DATA_SIZE: u32 = 4u32;
pub const DMUS_PC_AUDIOPATH: u32 = 1024u32;
pub const DMUS_PC_DIRECTSOUND: u32 = 128u32;
pub const DMUS_PC_DLS: u32 = 1u32;
pub const DMUS_PC_DLS2: u32 = 512u32;
pub const DMUS_PC_EXTERNAL: u32 = 2u32;
pub const DMUS_PC_GMINHARDWARE: u32 = 16u32;
pub const DMUS_PC_GSINHARDWARE: u32 = 32u32;
pub const DMUS_PC_INPUTCLASS: u32 = 0u32;
pub const DMUS_PC_MEMORYSIZEFIXED: u32 = 8u32;
pub const DMUS_PC_OUTPUTCLASS: u32 = 1u32;
pub const DMUS_PC_SHAREABLE: u32 = 256u32;
pub const DMUS_PC_SOFTWARESYNTH: u32 = 4u32;
pub const DMUS_PC_SYSTEMMEMORY: u32 = 2147483647u32;
pub const DMUS_PC_WAVE: u32 = 2048u32;
pub const DMUS_PC_XGINHARDWARE: u32 = 64u32;
pub const DMUS_PORTPARAMS_AUDIOCHANNELS: u32 = 4u32;
pub const DMUS_PORTPARAMS_CHANNELGROUPS: u32 = 2u32;
pub const DMUS_PORTPARAMS_EFFECTS: u32 = 32u32;
pub const DMUS_PORTPARAMS_FEATURES: u32 = 128u32;
pub const DMUS_PORTPARAMS_SAMPLERATE: u32 = 8u32;
pub const DMUS_PORTPARAMS_SHARE: u32 = 64u32;
pub const DMUS_PORTPARAMS_VOICES: u32 = 1u32;
pub const DMUS_PORT_FEATURE_AUDIOPATH: u32 = 1u32;
pub const DMUS_PORT_FEATURE_STREAMING: u32 = 2u32;
pub const DMUS_PORT_KERNEL_MODE: u32 = 2u32;
pub const DMUS_PORT_USER_MODE_SYNTH: u32 = 1u32;
pub const DMUS_PORT_WINMM_DRIVER: u32 = 0u32;
pub const DMUS_SYNTHSTATS_CPU_PER_VOICE: u32 = 4u32;
pub const DMUS_SYNTHSTATS_FREE_MEMORY: u32 = 32u32;
pub const DMUS_SYNTHSTATS_LOST_NOTES: u32 = 8u32;
pub const DMUS_SYNTHSTATS_PEAK_VOLUME: u32 = 16u32;
pub const DMUS_SYNTHSTATS_SYSTEMMEMORY: u32 = 2147483647u32;
pub const DMUS_SYNTHSTATS_TOTAL_CPU: u32 = 2u32;
pub const DMUS_SYNTHSTATS_VOICES: u32 = 1u32;
pub const DMUS_VOLUME_MAX: u32 = 2000u32;
pub const DMUS_VOLUME_MIN: i32 = -20000i32;
pub const DSBUSID_BACK_CENTER: u32 = 8u32;
pub const DSBUSID_BACK_LEFT: u32 = 4u32;
pub const DSBUSID_BACK_RIGHT: u32 = 5u32;
pub const DSBUSID_CHORUS_SEND: u32 = 65u32;
pub const DSBUSID_DYNAMIC_0: u32 = 512u32;
pub const DSBUSID_FIRST_SPKR_LOC: u32 = 0u32;
pub const DSBUSID_FRONT_CENTER: u32 = 2u32;
pub const DSBUSID_FRONT_LEFT: u32 = 0u32;
pub const DSBUSID_FRONT_LEFT_OF_CENTER: u32 = 6u32;
pub const DSBUSID_FRONT_RIGHT: u32 = 1u32;
pub const DSBUSID_FRONT_RIGHT_OF_CENTER: u32 = 7u32;
pub const DSBUSID_LAST_SPKR_LOC: u32 = 17u32;
pub const DSBUSID_LEFT: u32 = 0u32;
pub const DSBUSID_LOW_FREQUENCY: u32 = 3u32;
pub const DSBUSID_NULL: u32 = 4294967295u32;
pub const DSBUSID_REVERB_SEND: u32 = 64u32;
pub const DSBUSID_RIGHT: u32 = 1u32;
pub const DSBUSID_SIDE_LEFT: u32 = 9u32;
pub const DSBUSID_SIDE_RIGHT: u32 = 10u32;
pub const DSBUSID_TOP_BACK_CENTER: u32 = 16u32;
pub const DSBUSID_TOP_BACK_LEFT: u32 = 15u32;
pub const DSBUSID_TOP_BACK_RIGHT: u32 = 17u32;
pub const DSBUSID_TOP_CENTER: u32 = 11u32;
pub const DSBUSID_TOP_FRONT_CENTER: u32 = 13u32;
pub const DSBUSID_TOP_FRONT_LEFT: u32 = 12u32;
pub const DSBUSID_TOP_FRONT_RIGHT: u32 = 14u32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(2i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(5i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(6i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(3i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(7i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(8i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(1i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(4i32);
pub const DSPROPSETID_DirectSoundDevice: windows_core::GUID = windows_core::GUID::from_u128(0x84624f82_25ec_11d1_a4d8_00c04fc28aca);
pub const DV_AUDIOMODE: u32 = 3840u32;
pub const DV_AUDIOQU: u32 = 117440512u32;
pub const DV_AUDIOSMP: u32 = 939524096u32;
pub const DV_CAP_AUD12Bits: u32 = 1u32;
pub const DV_CAP_AUD16Bits: u32 = 0u32;
pub const DV_DVSD_NTSC_FRAMESIZE: i32 = 120000i32;
pub const DV_DVSD_PAL_FRAMESIZE: i32 = 144000i32;
pub const DV_HD: u32 = 1u32;
pub const DV_NTSC: u32 = 0u32;
pub const DV_NTSCPAL: u32 = 2097152u32;
pub const DV_PAL: u32 = 1u32;
pub const DV_SD: u32 = 0u32;
pub const DV_SL: u32 = 2u32;
pub const DV_SMCHN: u32 = 57344u32;
pub const DV_STYPE: u32 = 2031616u32;
pub const F_INSTRUMENT_DRUMS: u32 = 2147483648u32;
pub const F_RGN_OPTION_SELFNONEXCLUSIVE: u32 = 1u32;
pub const F_WAVELINK_MULTICHANNEL: u32 = 2u32;
pub const F_WAVELINK_PHASE_MASTER: u32 = 1u32;
pub const F_WSMP_NO_COMPRESSION: i32 = 2i32;
pub const F_WSMP_NO_TRUNCATION: i32 = 1i32;
pub const GUID_DMUS_PROP_DLS1: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f27_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_DLS2: windows_core::GUID = windows_core::GUID::from_u128(0xf14599e5_4689_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_Effects: windows_core::GUID = windows_core::GUID::from_u128(0xcda8d611_684a_11d2_871e_00600893b1bd);
pub const GUID_DMUS_PROP_GM_Hardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f24_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_GS_Capable: windows_core::GUID = windows_core::GUID::from_u128(0x6496aba2_61b0_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_GS_Hardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f25_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_INSTRUMENT2: windows_core::GUID = windows_core::GUID::from_u128(0x865fd372_9f67_11d2_872a_00600893b1bd);
pub const GUID_DMUS_PROP_LegacyCaps: windows_core::GUID = windows_core::GUID::from_u128(0xcfa7cdc2_00a1_11d2_aad5_0000f875ac12);
pub const GUID_DMUS_PROP_MemorySize: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_SampleMemorySize: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_SamplePlaybackRate: windows_core::GUID = windows_core::GUID::from_u128(0x2a91f713_a4bf_11d2_bbdf_00600833dbd8);
pub const GUID_DMUS_PROP_SetSynthSink: windows_core::GUID = windows_core::GUID::from_u128(0x0a3a5ba5_37b6_11d2_b9f9_0000f875ac12);
pub const GUID_DMUS_PROP_SinkUsesDSound: windows_core::GUID = windows_core::GUID::from_u128(0xbe208857_8952_11d2_ba1c_0000f875ac12);
pub const GUID_DMUS_PROP_SynthSink_DSOUND: windows_core::GUID = windows_core::GUID::from_u128(0x0aa97844_c877_11d1_870c_00600893b1bd);
pub const GUID_DMUS_PROP_SynthSink_WAVE: windows_core::GUID = windows_core::GUID::from_u128(0x0aa97845_c877_11d1_870c_00600893b1bd);
pub const GUID_DMUS_PROP_Volume: windows_core::GUID = windows_core::GUID::from_u128(0xfedfae25_e46e_11d1_aace_0000f875ac12);
pub const GUID_DMUS_PROP_WavesReverb: windows_core::GUID = windows_core::GUID::from_u128(0x04cb5622_32e5_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_WriteLatency: windows_core::GUID = windows_core::GUID::from_u128(0x268a0fa0_60f2_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_WritePeriod: windows_core::GUID = windows_core::GUID::from_u128(0x268a0fa1_60f2_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_XG_Capable: windows_core::GUID = windows_core::GUID::from_u128(0x6496aba1_61b0_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_XG_Hardware: windows_core::GUID = windows_core::GUID::from_u128(0x178f2f26_c364_11d1_a760_0000f875ac12);
pub const POOL_CUE_NULL: i32 = -1i32;
pub const REFRESH_F_LASTBUFFER: u32 = 1u32;
pub const REGSTR_PATH_SOFTWARESYNTHS: windows_core::PCSTR = windows_core::s!("Software\\Microsoft\\DirectMusic\\SoftwareSynths");
pub const SIZE_DVINFO: u32 = 32u32;
pub const WAVELINK_CHANNEL_LEFT: i32 = 1i32;
pub const WAVELINK_CHANNEL_RIGHT: i32 = 2i32;
pub const WLOOP_TYPE_FORWARD: u32 = 0u32;
pub const WLOOP_TYPE_RELEASE: u32 = 2u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTSOUNDDEVICE_DATAFLOW(pub i32);
impl windows_core::TypeKind for DIRECTSOUNDDEVICE_DATAFLOW {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTSOUNDDEVICE_DATAFLOW {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTSOUNDDEVICE_DATAFLOW").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTSOUNDDEVICE_TYPE(pub i32);
impl windows_core::TypeKind for DIRECTSOUNDDEVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTSOUNDDEVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTSOUNDDEVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DMUS_CLOCKTYPE(pub i32);
impl windows_core::TypeKind for DMUS_CLOCKTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DMUS_CLOCKTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DMUS_CLOCKTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE(pub i32);
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DSPROPERTY_DIRECTSOUNDDEVICE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CONNECTION {
    pub usSource: u16,
    pub usControl: u16,
    pub usDestination: u16,
    pub usTransform: u16,
    pub lScale: i32,
}
impl Copy for CONNECTION {}
impl Clone for CONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CONNECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CONNECTION").field("usSource", &self.usSource).field("usControl", &self.usControl).field("usDestination", &self.usDestination).field("usTransform", &self.usTransform).field("lScale", &self.lScale).finish()
    }
}
impl windows_core::TypeKind for CONNECTION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.usSource == other.usSource && self.usControl == other.usControl && self.usDestination == other.usDestination && self.usTransform == other.usTransform && self.lScale == other.lScale
    }
}
impl Eq for CONNECTION {}
impl Default for CONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CONNECTIONLIST {
    pub cbSize: u32,
    pub cConnections: u32,
}
impl Copy for CONNECTIONLIST {}
impl Clone for CONNECTIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CONNECTIONLIST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CONNECTIONLIST").field("cbSize", &self.cbSize).field("cConnections", &self.cConnections).finish()
    }
}
impl windows_core::TypeKind for CONNECTIONLIST {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CONNECTIONLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cConnections == other.cConnections
    }
}
impl Eq for CONNECTIONLIST {}
impl Default for CONNECTIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DLSHEADER {
    pub cInstruments: u32,
}
impl Copy for DLSHEADER {}
impl Clone for DLSHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DLSHEADER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DLSHEADER").field("cInstruments", &self.cInstruments).finish()
    }
}
impl windows_core::TypeKind for DLSHEADER {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DLSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cInstruments == other.cInstruments
    }
}
impl Eq for DLSHEADER {}
impl Default for DLSHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DLSID {
    pub ulData1: u32,
    pub usData2: u16,
    pub usData3: u16,
    pub abData4: [u8; 8],
}
impl Copy for DLSID {}
impl Clone for DLSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DLSID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DLSID").field("ulData1", &self.ulData1).field("usData2", &self.usData2).field("usData3", &self.usData3).field("abData4", &self.abData4).finish()
    }
}
impl windows_core::TypeKind for DLSID {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DLSID {
    fn eq(&self, other: &Self) -> bool {
        self.ulData1 == other.ulData1 && self.usData2 == other.usData2 && self.usData3 == other.usData3 && self.abData4 == other.abData4
    }
}
impl Eq for DLSID {}
impl Default for DLSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DLSVERSION {
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
}
impl Copy for DLSVERSION {}
impl Clone for DLSVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DLSVERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DLSVERSION").field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).finish()
    }
}
impl windows_core::TypeKind for DLSVERSION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DLSVERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersionMS == other.dwVersionMS && self.dwVersionLS == other.dwVersionLS
    }
}
impl Eq for DLSVERSION {}
impl Default for DLSVERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_ARTICPARAMS {
    pub LFO: DMUS_LFOPARAMS,
    pub VolEG: DMUS_VEGPARAMS,
    pub PitchEG: DMUS_PEGPARAMS,
    pub Misc: DMUS_MSCPARAMS,
}
impl Copy for DMUS_ARTICPARAMS {}
impl Clone for DMUS_ARTICPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_ARTICPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_ARTICPARAMS").field("LFO", &self.LFO).field("VolEG", &self.VolEG).field("PitchEG", &self.PitchEG).field("Misc", &self.Misc).finish()
    }
}
impl windows_core::TypeKind for DMUS_ARTICPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_ARTICPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.LFO == other.LFO && self.VolEG == other.VolEG && self.PitchEG == other.PitchEG && self.Misc == other.Misc
    }
}
impl Eq for DMUS_ARTICPARAMS {}
impl Default for DMUS_ARTICPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_ARTICULATION {
    pub ulArt1Idx: u32,
    pub ulFirstExtCkIdx: u32,
}
impl Copy for DMUS_ARTICULATION {}
impl Clone for DMUS_ARTICULATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_ARTICULATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_ARTICULATION").field("ulArt1Idx", &self.ulArt1Idx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).finish()
    }
}
impl windows_core::TypeKind for DMUS_ARTICULATION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_ARTICULATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulArt1Idx == other.ulArt1Idx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx
    }
}
impl Eq for DMUS_ARTICULATION {}
impl Default for DMUS_ARTICULATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_ARTICULATION2 {
    pub ulArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulNextArtIdx: u32,
}
impl Copy for DMUS_ARTICULATION2 {}
impl Clone for DMUS_ARTICULATION2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_ARTICULATION2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_ARTICULATION2").field("ulArtIdx", &self.ulArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulNextArtIdx", &self.ulNextArtIdx).finish()
    }
}
impl windows_core::TypeKind for DMUS_ARTICULATION2 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_ARTICULATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.ulArtIdx == other.ulArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulNextArtIdx == other.ulNextArtIdx
    }
}
impl Eq for DMUS_ARTICULATION2 {}
impl Default for DMUS_ARTICULATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_BUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidBufferFormat: windows_core::GUID,
    pub cbBuffer: u32,
}
impl Copy for DMUS_BUFFERDESC {}
impl Clone for DMUS_BUFFERDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_BUFFERDESC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_BUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidBufferFormat", &self.guidBufferFormat).field("cbBuffer", &self.cbBuffer).finish()
    }
}
impl windows_core::TypeKind for DMUS_BUFFERDESC {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_BUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidBufferFormat == other.guidBufferFormat && self.cbBuffer == other.cbBuffer
    }
}
impl Eq for DMUS_BUFFERDESC {}
impl Default for DMUS_BUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_CLOCKINFO7 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: windows_core::GUID,
    pub wszDescription: [u16; 128],
}
impl Copy for DMUS_CLOCKINFO7 {}
impl Clone for DMUS_CLOCKINFO7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_CLOCKINFO7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_CLOCKINFO7").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).finish()
    }
}
impl windows_core::TypeKind for DMUS_CLOCKINFO7 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_CLOCKINFO7 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription
    }
}
impl Eq for DMUS_CLOCKINFO7 {}
impl Default for DMUS_CLOCKINFO7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_CLOCKINFO8 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: windows_core::GUID,
    pub wszDescription: [u16; 128],
    pub dwFlags: u32,
}
impl Copy for DMUS_CLOCKINFO8 {}
impl Clone for DMUS_CLOCKINFO8 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_CLOCKINFO8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_CLOCKINFO8").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).field("dwFlags", &self.dwFlags).finish()
    }
}
impl windows_core::TypeKind for DMUS_CLOCKINFO8 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_CLOCKINFO8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription && self.dwFlags == other.dwFlags
    }
}
impl Eq for DMUS_CLOCKINFO8 {}
impl Default for DMUS_CLOCKINFO8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_COPYRIGHT {
    pub cbSize: u32,
    pub byCopyright: [u8; 4],
}
impl Copy for DMUS_COPYRIGHT {}
impl Clone for DMUS_COPYRIGHT {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_COPYRIGHT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_COPYRIGHT").field("cbSize", &self.cbSize).field("byCopyright", &self.byCopyright).finish()
    }
}
impl windows_core::TypeKind for DMUS_COPYRIGHT {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_COPYRIGHT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byCopyright == other.byCopyright
    }
}
impl Eq for DMUS_COPYRIGHT {}
impl Default for DMUS_COPYRIGHT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_DOWNLOADINFO {
    pub dwDLType: u32,
    pub dwDLId: u32,
    pub dwNumOffsetTableEntries: u32,
    pub cbSize: u32,
}
impl Copy for DMUS_DOWNLOADINFO {}
impl Clone for DMUS_DOWNLOADINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_DOWNLOADINFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_DOWNLOADINFO").field("dwDLType", &self.dwDLType).field("dwDLId", &self.dwDLId).field("dwNumOffsetTableEntries", &self.dwNumOffsetTableEntries).field("cbSize", &self.cbSize).finish()
    }
}
impl windows_core::TypeKind for DMUS_DOWNLOADINFO {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_DOWNLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDLType == other.dwDLType && self.dwDLId == other.dwDLId && self.dwNumOffsetTableEntries == other.dwNumOffsetTableEntries && self.cbSize == other.cbSize
    }
}
impl Eq for DMUS_DOWNLOADINFO {}
impl Default for DMUS_DOWNLOADINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
pub struct DMUS_EVENTHEADER {
    pub cbEvent: u32,
    pub dwChannelGroup: u32,
    pub rtDelta: i64,
    pub dwFlags: u32,
}
impl Copy for DMUS_EVENTHEADER {}
impl Clone for DMUS_EVENTHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DMUS_EVENTHEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DMUS_EVENTHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_EXTENSIONCHUNK {
    pub cbSize: u32,
    pub ulNextExtCkIdx: u32,
    pub ExtCkID: u32,
    pub byExtCk: [u8; 4],
}
impl Copy for DMUS_EXTENSIONCHUNK {}
impl Clone for DMUS_EXTENSIONCHUNK {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_EXTENSIONCHUNK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_EXTENSIONCHUNK").field("cbSize", &self.cbSize).field("ulNextExtCkIdx", &self.ulNextExtCkIdx).field("ExtCkID", &self.ExtCkID).field("byExtCk", &self.byExtCk).finish()
    }
}
impl windows_core::TypeKind for DMUS_EXTENSIONCHUNK {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_EXTENSIONCHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulNextExtCkIdx == other.ulNextExtCkIdx && self.ExtCkID == other.ExtCkID && self.byExtCk == other.byExtCk
    }
}
impl Eq for DMUS_EXTENSIONCHUNK {}
impl Default for DMUS_EXTENSIONCHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_INSTRUMENT {
    pub ulPatch: u32,
    pub ulFirstRegionIdx: u32,
    pub ulGlobalArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulFlags: u32,
}
impl Copy for DMUS_INSTRUMENT {}
impl Clone for DMUS_INSTRUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_INSTRUMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_INSTRUMENT").field("ulPatch", &self.ulPatch).field("ulFirstRegionIdx", &self.ulFirstRegionIdx).field("ulGlobalArtIdx", &self.ulGlobalArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulCopyrightIdx", &self.ulCopyrightIdx).field("ulFlags", &self.ulFlags).finish()
    }
}
impl windows_core::TypeKind for DMUS_INSTRUMENT {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_INSTRUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ulPatch == other.ulPatch && self.ulFirstRegionIdx == other.ulFirstRegionIdx && self.ulGlobalArtIdx == other.ulGlobalArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulCopyrightIdx == other.ulCopyrightIdx && self.ulFlags == other.ulFlags
    }
}
impl Eq for DMUS_INSTRUMENT {}
impl Default for DMUS_INSTRUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_LFOPARAMS {
    pub pcFrequency: i32,
    pub tcDelay: i32,
    pub gcVolumeScale: i32,
    pub pcPitchScale: i32,
    pub gcMWToVolume: i32,
    pub pcMWToPitch: i32,
}
impl Copy for DMUS_LFOPARAMS {}
impl Clone for DMUS_LFOPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_LFOPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_LFOPARAMS").field("pcFrequency", &self.pcFrequency).field("tcDelay", &self.tcDelay).field("gcVolumeScale", &self.gcVolumeScale).field("pcPitchScale", &self.pcPitchScale).field("gcMWToVolume", &self.gcMWToVolume).field("pcMWToPitch", &self.pcMWToPitch).finish()
    }
}
impl windows_core::TypeKind for DMUS_LFOPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_LFOPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.pcFrequency == other.pcFrequency && self.tcDelay == other.tcDelay && self.gcVolumeScale == other.gcVolumeScale && self.pcPitchScale == other.pcPitchScale && self.gcMWToVolume == other.gcMWToVolume && self.pcMWToPitch == other.pcMWToPitch
    }
}
impl Eq for DMUS_LFOPARAMS {}
impl Default for DMUS_LFOPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_MSCPARAMS {
    pub ptDefaultPan: i32,
}
impl Copy for DMUS_MSCPARAMS {}
impl Clone for DMUS_MSCPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_MSCPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_MSCPARAMS").field("ptDefaultPan", &self.ptDefaultPan).finish()
    }
}
impl windows_core::TypeKind for DMUS_MSCPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_MSCPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ptDefaultPan == other.ptDefaultPan
    }
}
impl Eq for DMUS_MSCPARAMS {}
impl Default for DMUS_MSCPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_NOTERANGE {
    pub dwLowNote: u32,
    pub dwHighNote: u32,
}
impl Copy for DMUS_NOTERANGE {}
impl Clone for DMUS_NOTERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_NOTERANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_NOTERANGE").field("dwLowNote", &self.dwLowNote).field("dwHighNote", &self.dwHighNote).finish()
    }
}
impl windows_core::TypeKind for DMUS_NOTERANGE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_NOTERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowNote == other.dwLowNote && self.dwHighNote == other.dwHighNote
    }
}
impl Eq for DMUS_NOTERANGE {}
impl Default for DMUS_NOTERANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_OFFSETTABLE {
    pub ulOffsetTable: [u32; 1],
}
impl Copy for DMUS_OFFSETTABLE {}
impl Clone for DMUS_OFFSETTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_OFFSETTABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_OFFSETTABLE").field("ulOffsetTable", &self.ulOffsetTable).finish()
    }
}
impl windows_core::TypeKind for DMUS_OFFSETTABLE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_OFFSETTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffsetTable == other.ulOffsetTable
    }
}
impl Eq for DMUS_OFFSETTABLE {}
impl Default for DMUS_OFFSETTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_PEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
    pub pcRange: i32,
}
impl Copy for DMUS_PEGPARAMS {}
impl Clone for DMUS_PEGPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_PEGPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_PEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).field("pcRange", &self.pcRange).finish()
    }
}
impl windows_core::TypeKind for DMUS_PEGPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_PEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay && self.pcRange == other.pcRange
    }
}
impl Eq for DMUS_PEGPARAMS {}
impl Default for DMUS_PEGPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_PORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidPort: windows_core::GUID,
    pub dwClass: u32,
    pub dwType: u32,
    pub dwMemorySize: u32,
    pub dwMaxChannelGroups: u32,
    pub dwMaxVoices: u32,
    pub dwMaxAudioChannels: u32,
    pub dwEffectFlags: u32,
    pub wszDescription: [u16; 128],
}
impl Copy for DMUS_PORTCAPS {}
impl Clone for DMUS_PORTCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_PORTCAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_PORTCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidPort", &self.guidPort).field("dwClass", &self.dwClass).field("dwType", &self.dwType).field("dwMemorySize", &self.dwMemorySize).field("dwMaxChannelGroups", &self.dwMaxChannelGroups).field("dwMaxVoices", &self.dwMaxVoices).field("dwMaxAudioChannels", &self.dwMaxAudioChannels).field("dwEffectFlags", &self.dwEffectFlags).field("wszDescription", &self.wszDescription).finish()
    }
}
impl windows_core::TypeKind for DMUS_PORTCAPS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_PORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidPort == other.guidPort && self.dwClass == other.dwClass && self.dwType == other.dwType && self.dwMemorySize == other.dwMemorySize && self.dwMaxChannelGroups == other.dwMaxChannelGroups && self.dwMaxVoices == other.dwMaxVoices && self.dwMaxAudioChannels == other.dwMaxAudioChannels && self.dwEffectFlags == other.dwEffectFlags && self.wszDescription == other.wszDescription
    }
}
impl Eq for DMUS_PORTCAPS {}
impl Default for DMUS_PORTCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_PORTPARAMS7 {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: super::super::super::Foundation::BOOL,
}
impl Copy for DMUS_PORTPARAMS7 {}
impl Clone for DMUS_PORTPARAMS7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_PORTPARAMS7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_PORTPARAMS7").field("dwSize", &self.dwSize).field("dwValidParams", &self.dwValidParams).field("dwVoices", &self.dwVoices).field("dwChannelGroups", &self.dwChannelGroups).field("dwAudioChannels", &self.dwAudioChannels).field("dwSampleRate", &self.dwSampleRate).field("dwEffectFlags", &self.dwEffectFlags).field("fShare", &self.fShare).finish()
    }
}
impl windows_core::TypeKind for DMUS_PORTPARAMS7 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_PORTPARAMS7 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare
    }
}
impl Eq for DMUS_PORTPARAMS7 {}
impl Default for DMUS_PORTPARAMS7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_PORTPARAMS8 {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: super::super::super::Foundation::BOOL,
    pub dwFeatures: u32,
}
impl Copy for DMUS_PORTPARAMS8 {}
impl Clone for DMUS_PORTPARAMS8 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_PORTPARAMS8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_PORTPARAMS8").field("dwSize", &self.dwSize).field("dwValidParams", &self.dwValidParams).field("dwVoices", &self.dwVoices).field("dwChannelGroups", &self.dwChannelGroups).field("dwAudioChannels", &self.dwAudioChannels).field("dwSampleRate", &self.dwSampleRate).field("dwEffectFlags", &self.dwEffectFlags).field("fShare", &self.fShare).field("dwFeatures", &self.dwFeatures).finish()
    }
}
impl windows_core::TypeKind for DMUS_PORTPARAMS8 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_PORTPARAMS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare && self.dwFeatures == other.dwFeatures
    }
}
impl Eq for DMUS_PORTPARAMS8 {}
impl Default for DMUS_PORTPARAMS8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_REGION {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
    pub ulRegionArtIdx: u32,
    pub ulNextRegionIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub WaveLink: WAVELINK,
    pub WSMP: WSMPL,
    pub WLOOP: [WLOOP; 1],
}
impl Copy for DMUS_REGION {}
impl Clone for DMUS_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_REGION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_REGION").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).field("ulRegionArtIdx", &self.ulRegionArtIdx).field("ulNextRegionIdx", &self.ulNextRegionIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("WaveLink", &self.WaveLink).field("WSMP", &self.WSMP).field("WLOOP", &self.WLOOP).finish()
    }
}
impl windows_core::TypeKind for DMUS_REGION {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup && self.ulRegionArtIdx == other.ulRegionArtIdx && self.ulNextRegionIdx == other.ulNextRegionIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.WaveLink == other.WaveLink && self.WSMP == other.WSMP && self.WLOOP == other.WLOOP
    }
}
impl Eq for DMUS_REGION {}
impl Default for DMUS_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_SYNTHSTATS {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
}
impl Copy for DMUS_SYNTHSTATS {}
impl Clone for DMUS_SYNTHSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_SYNTHSTATS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_SYNTHSTATS").field("dwSize", &self.dwSize).field("dwValidStats", &self.dwValidStats).field("dwVoices", &self.dwVoices).field("dwTotalCPU", &self.dwTotalCPU).field("dwCPUPerVoice", &self.dwCPUPerVoice).field("dwLostNotes", &self.dwLostNotes).field("dwFreeMemory", &self.dwFreeMemory).field("lPeakVolume", &self.lPeakVolume).finish()
    }
}
impl windows_core::TypeKind for DMUS_SYNTHSTATS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_SYNTHSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume
    }
}
impl Eq for DMUS_SYNTHSTATS {}
impl Default for DMUS_SYNTHSTATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_SYNTHSTATS8 {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
    pub dwSynthMemUse: u32,
}
impl Copy for DMUS_SYNTHSTATS8 {}
impl Clone for DMUS_SYNTHSTATS8 {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_SYNTHSTATS8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_SYNTHSTATS8").field("dwSize", &self.dwSize).field("dwValidStats", &self.dwValidStats).field("dwVoices", &self.dwVoices).field("dwTotalCPU", &self.dwTotalCPU).field("dwCPUPerVoice", &self.dwCPUPerVoice).field("dwLostNotes", &self.dwLostNotes).field("dwFreeMemory", &self.dwFreeMemory).field("lPeakVolume", &self.lPeakVolume).field("dwSynthMemUse", &self.dwSynthMemUse).finish()
    }
}
impl windows_core::TypeKind for DMUS_SYNTHSTATS8 {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_SYNTHSTATS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume && self.dwSynthMemUse == other.dwSynthMemUse
    }
}
impl Eq for DMUS_SYNTHSTATS8 {}
impl Default for DMUS_SYNTHSTATS8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_VEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
}
impl Copy for DMUS_VEGPARAMS {}
impl Clone for DMUS_VEGPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_VEGPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_VEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).finish()
    }
}
impl windows_core::TypeKind for DMUS_VEGPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_VEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay
    }
}
impl Eq for DMUS_VEGPARAMS {}
impl Default for DMUS_VEGPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_VOICE_STATE {
    pub bExists: super::super::super::Foundation::BOOL,
    pub spPosition: u64,
}
impl Copy for DMUS_VOICE_STATE {}
impl Clone for DMUS_VOICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_VOICE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_VOICE_STATE").field("bExists", &self.bExists).field("spPosition", &self.spPosition).finish()
    }
}
impl windows_core::TypeKind for DMUS_VOICE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_VOICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.bExists == other.bExists && self.spPosition == other.spPosition
    }
}
impl Eq for DMUS_VOICE_STATE {}
impl Default for DMUS_VOICE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_WAVE {
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulWaveDataIdx: u32,
    pub WaveformatEx: super::WAVEFORMATEX,
}
impl Copy for DMUS_WAVE {}
impl Clone for DMUS_WAVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DMUS_WAVE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DMUS_WAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_WAVEARTDL {
    pub ulDownloadIdIdx: u32,
    pub ulBus: u32,
    pub ulBuffers: u32,
    pub ulMasterDLId: u32,
    pub usOptions: u16,
}
impl Copy for DMUS_WAVEARTDL {}
impl Clone for DMUS_WAVEARTDL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_WAVEARTDL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_WAVEARTDL").field("ulDownloadIdIdx", &self.ulDownloadIdIdx).field("ulBus", &self.ulBus).field("ulBuffers", &self.ulBuffers).field("ulMasterDLId", &self.ulMasterDLId).field("usOptions", &self.usOptions).finish()
    }
}
impl windows_core::TypeKind for DMUS_WAVEARTDL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_WAVEARTDL {
    fn eq(&self, other: &Self) -> bool {
        self.ulDownloadIdIdx == other.ulDownloadIdIdx && self.ulBus == other.ulBus && self.ulBuffers == other.ulBuffers && self.ulMasterDLId == other.ulMasterDLId && self.usOptions == other.usOptions
    }
}
impl Eq for DMUS_WAVEARTDL {}
impl Default for DMUS_WAVEARTDL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_WAVEDATA {
    pub cbSize: u32,
    pub byData: [u8; 4],
}
impl Copy for DMUS_WAVEDATA {}
impl Clone for DMUS_WAVEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_WAVEDATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_WAVEDATA").field("cbSize", &self.cbSize).field("byData", &self.byData).finish()
    }
}
impl windows_core::TypeKind for DMUS_WAVEDATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_WAVEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byData == other.byData
    }
}
impl Eq for DMUS_WAVEDATA {}
impl Default for DMUS_WAVEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_WAVEDL {
    pub cbWaveData: u32,
}
impl Copy for DMUS_WAVEDL {}
impl Clone for DMUS_WAVEDL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_WAVEDL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_WAVEDL").field("cbWaveData", &self.cbWaveData).finish()
    }
}
impl windows_core::TypeKind for DMUS_WAVEDL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_WAVEDL {
    fn eq(&self, other: &Self) -> bool {
        self.cbWaveData == other.cbWaveData
    }
}
impl Eq for DMUS_WAVEDL {}
impl Default for DMUS_WAVEDL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_WAVES_REVERB_PARAMS {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl Copy for DMUS_WAVES_REVERB_PARAMS {}
impl Clone for DMUS_WAVES_REVERB_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DMUS_WAVES_REVERB_PARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DMUS_WAVES_REVERB_PARAMS").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
    }
}
impl windows_core::TypeKind for DMUS_WAVES_REVERB_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DMUS_WAVES_REVERB_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.fInGain == other.fInGain && self.fReverbMix == other.fReverbMix && self.fReverbTime == other.fReverbTime && self.fHighFreqRTRatio == other.fHighFreqRTRatio
    }
}
impl Eq for DMUS_WAVES_REVERB_PARAMS {}
impl Default for DMUS_WAVES_REVERB_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    pub DeviceId: windows_core::GUID,
    pub DescriptionA: [i8; 256],
    pub DescriptionW: [u16; 256],
    pub ModuleA: [i8; 260],
    pub ModuleW: [u16; 260],
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub WaveDeviceId: u32,
    pub Devnode: u32,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA").field("DeviceId", &self.DeviceId).field("DescriptionA", &self.DescriptionA).field("DescriptionW", &self.DescriptionW).field("ModuleA", &self.ModuleA).field("ModuleW", &self.ModuleW).field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("WaveDeviceId", &self.WaveDeviceId).field("Devnode", &self.Devnode).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceId == other.DeviceId && self.DescriptionA == other.DescriptionA && self.DescriptionW == other.DescriptionW && self.ModuleA == other.ModuleA && self.ModuleW == other.ModuleW && self.Type == other.Type && self.DataFlow == other.DataFlow && self.WaveDeviceId == other.WaveDeviceId && self.Devnode == other.Devnode
    }
}
impl Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
    pub Description: windows_core::PSTR,
    pub Module: windows_core::PSTR,
    pub Interface: windows_core::PSTR,
    pub WaveDeviceId: u32,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA").field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).field("Description", &self.Description).field("Module", &self.Module).field("Interface", &self.Interface).field("WaveDeviceId", &self.WaveDeviceId).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
impl Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
    pub Description: windows_core::PWSTR,
    pub Module: windows_core::PWSTR,
    pub Interface: windows_core::PWSTR,
    pub WaveDeviceId: u32,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA").field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).field("Description", &self.Description).field("Module", &self.Module).field("Interface", &self.Interface).field("WaveDeviceId", &self.WaveDeviceId).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
impl Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1,
    pub Context: *mut core::ffi::c_void,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA").field("Context", &self.Context).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA,
    pub Context: *mut core::ffi::c_void,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA").field("Context", &self.Context).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW,
    pub Context: *mut core::ffi::c_void,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA").field("Context", &self.Context).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    pub DeviceName: windows_core::PSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
impl Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    pub DeviceName: windows_core::PWSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: windows_core::GUID,
}
impl Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
impl Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
impl windows_core::TypeKind for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
impl Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
impl Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DVAudInfo {
    pub bAudStyle: [u8; 2],
    pub bAudQu: [u8; 2],
    pub bNumAudPin: u8,
    pub wAvgSamplesPerPinPerFrm: [u16; 2],
    pub wBlkMode: u16,
    pub wDIFMode: u16,
    pub wBlkDiv: u16,
}
impl Copy for DVAudInfo {}
impl Clone for DVAudInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DVAudInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DVAudInfo").field("bAudStyle", &self.bAudStyle).field("bAudQu", &self.bAudQu).field("bNumAudPin", &self.bNumAudPin).field("wAvgSamplesPerPinPerFrm", &self.wAvgSamplesPerPinPerFrm).field("wBlkMode", &self.wBlkMode).field("wDIFMode", &self.wDIFMode).field("wBlkDiv", &self.wBlkDiv).finish()
    }
}
impl windows_core::TypeKind for DVAudInfo {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for DVAudInfo {
    fn eq(&self, other: &Self) -> bool {
        self.bAudStyle == other.bAudStyle && self.bAudQu == other.bAudQu && self.bNumAudPin == other.bNumAudPin && self.wAvgSamplesPerPinPerFrm == other.wAvgSamplesPerPinPerFrm && self.wBlkMode == other.wBlkMode && self.wDIFMode == other.wDIFMode && self.wBlkDiv == other.wBlkDiv
    }
}
impl Eq for DVAudInfo {}
impl Default for DVAudInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INSTHEADER {
    pub cRegions: u32,
    pub Locale: MIDILOCALE,
}
impl Copy for INSTHEADER {}
impl Clone for INSTHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for INSTHEADER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("INSTHEADER").field("cRegions", &self.cRegions).field("Locale", &self.Locale).finish()
    }
}
impl windows_core::TypeKind for INSTHEADER {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for INSTHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cRegions == other.cRegions && self.Locale == other.Locale
    }
}
impl Eq for INSTHEADER {}
impl Default for INSTHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct MDEVICECAPSEX {
    pub cbSize: u32,
    pub pCaps: *mut core::ffi::c_void,
}
impl Copy for MDEVICECAPSEX {}
impl Clone for MDEVICECAPSEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for MDEVICECAPSEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for MDEVICECAPSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MIDILOCALE {
    pub ulBank: u32,
    pub ulInstrument: u32,
}
impl Copy for MIDILOCALE {}
impl Clone for MIDILOCALE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MIDILOCALE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MIDILOCALE").field("ulBank", &self.ulBank).field("ulInstrument", &self.ulInstrument).finish()
    }
}
impl windows_core::TypeKind for MIDILOCALE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MIDILOCALE {
    fn eq(&self, other: &Self) -> bool {
        self.ulBank == other.ulBank && self.ulInstrument == other.ulInstrument
    }
}
impl Eq for MIDILOCALE {}
impl Default for MIDILOCALE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Multimedia")]
pub struct MIDIOPENDESC {
    pub hMidi: super::HMIDI,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
    pub cIds: u32,
    pub rgIds: [super::super::Multimedia::MIDIOPENSTRMID; 1],
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl Copy for MIDIOPENDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl Clone for MIDIOPENDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl windows_core::TypeKind for MIDIOPENDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl Default for MIDIOPENDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct POOLCUE {
    pub ulOffset: u32,
}
impl Copy for POOLCUE {}
impl Clone for POOLCUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for POOLCUE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("POOLCUE").field("ulOffset", &self.ulOffset).finish()
    }
}
impl windows_core::TypeKind for POOLCUE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for POOLCUE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffset == other.ulOffset
    }
}
impl Eq for POOLCUE {}
impl Default for POOLCUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct POOLTABLE {
    pub cbSize: u32,
    pub cCues: u32,
}
impl Copy for POOLTABLE {}
impl Clone for POOLTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for POOLTABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("POOLTABLE").field("cbSize", &self.cbSize).field("cCues", &self.cCues).finish()
    }
}
impl windows_core::TypeKind for POOLTABLE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for POOLTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cCues == other.cCues
    }
}
impl Eq for POOLTABLE {}
impl Default for POOLTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RGNHEADER {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
}
impl Copy for RGNHEADER {}
impl Clone for RGNHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for RGNHEADER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RGNHEADER").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).finish()
    }
}
impl windows_core::TypeKind for RGNHEADER {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for RGNHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup
    }
}
impl Eq for RGNHEADER {}
impl Default for RGNHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RGNRANGE {
    pub usLow: u16,
    pub usHigh: u16,
}
impl Copy for RGNRANGE {}
impl Clone for RGNRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for RGNRANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RGNRANGE").field("usLow", &self.usLow).field("usHigh", &self.usHigh).finish()
    }
}
impl windows_core::TypeKind for RGNRANGE {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for RGNRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.usLow == other.usLow && self.usHigh == other.usHigh
    }
}
impl Eq for RGNRANGE {}
impl Default for RGNRANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WAVELINK {
    pub fusOptions: u16,
    pub usPhaseGroup: u16,
    pub ulChannel: u32,
    pub ulTableIndex: u32,
}
impl Copy for WAVELINK {}
impl Clone for WAVELINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for WAVELINK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WAVELINK").field("fusOptions", &self.fusOptions).field("usPhaseGroup", &self.usPhaseGroup).field("ulChannel", &self.ulChannel).field("ulTableIndex", &self.ulTableIndex).finish()
    }
}
impl windows_core::TypeKind for WAVELINK {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for WAVELINK {
    fn eq(&self, other: &Self) -> bool {
        self.fusOptions == other.fusOptions && self.usPhaseGroup == other.usPhaseGroup && self.ulChannel == other.ulChannel && self.ulTableIndex == other.ulTableIndex
    }
}
impl Eq for WAVELINK {}
impl Default for WAVELINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLOOP {
    pub cbSize: u32,
    pub ulType: u32,
    pub ulStart: u32,
    pub ulLength: u32,
}
impl Copy for WLOOP {}
impl Clone for WLOOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for WLOOP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WLOOP").field("cbSize", &self.cbSize).field("ulType", &self.ulType).field("ulStart", &self.ulStart).field("ulLength", &self.ulLength).finish()
    }
}
impl windows_core::TypeKind for WLOOP {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for WLOOP {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulType == other.ulType && self.ulStart == other.ulStart && self.ulLength == other.ulLength
    }
}
impl Eq for WLOOP {}
impl Default for WLOOP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WSMPL {
    pub cbSize: u32,
    pub usUnityNote: u16,
    pub sFineTune: i16,
    pub lAttenuation: i32,
    pub fulOptions: u32,
    pub cSampleLoops: u32,
}
impl Copy for WSMPL {}
impl Clone for WSMPL {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for WSMPL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WSMPL").field("cbSize", &self.cbSize).field("usUnityNote", &self.usUnityNote).field("sFineTune", &self.sFineTune).field("lAttenuation", &self.lAttenuation).field("fulOptions", &self.fulOptions).field("cSampleLoops", &self.cSampleLoops).finish()
    }
}
impl windows_core::TypeKind for WSMPL {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for WSMPL {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.usUnityNote == other.usUnityNote && self.sFineTune == other.sFineTune && self.lAttenuation == other.lAttenuation && self.fulOptions == other.fulOptions && self.cSampleLoops == other.cSampleLoops
    }
}
impl Eq for WSMPL {}
impl Default for WSMPL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1 = Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA, param1: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA = Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA, param1: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW = Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA, param1: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
