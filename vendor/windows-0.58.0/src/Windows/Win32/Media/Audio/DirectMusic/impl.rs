#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub trait IDirectMusic_Impl: Sized {
    fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()>;
    fn CreateMusicBuffer(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut Option<IDirectMusicBuffer>, punkouter: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePort(&self, rclsidport: *const windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut Option<IDirectMusicPort>, punkouter: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> windows_core::Result<()>;
    fn GetMasterClock(&self, pguidclock: *mut windows_core::GUID, ppreferenceclock: *mut Option<super::super::IReferenceClock>) -> windows_core::Result<()>;
    fn SetMasterClock(&self, rguidclock: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Activate(&self, fenable: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDefaultPort(&self, pguidport: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: Option<&super::DirectSound::IDirectSound>, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl windows_core::RuntimeName for IDirectMusic {}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl IDirectMusic_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusic_Vtbl
    where
        Identity: IDirectMusic_Impl,
    {
        unsafe extern "system" fn EnumPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::EnumPort(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pportcaps)).into()
        }
        unsafe extern "system" fn CreateMusicBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::CreateMusicBuffer(this, core::mem::transmute_copy(&pbufferdesc), core::mem::transmute_copy(&ppbuffer), windows_core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn CreatePort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsidport: *const windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::CreatePort(this, core::mem::transmute_copy(&rclsidport), core::mem::transmute_copy(&pportparams), core::mem::transmute_copy(&ppport), windows_core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn EnumMasterClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::EnumMasterClock(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&lpclockinfo)).into()
        }
        unsafe extern "system" fn GetMasterClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidclock: *mut windows_core::GUID, ppreferenceclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::GetMasterClock(this, core::mem::transmute_copy(&pguidclock), core::mem::transmute_copy(&ppreferenceclock)).into()
        }
        unsafe extern "system" fn SetMasterClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidclock: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::SetMasterClock(this, core::mem::transmute_copy(&rguidclock)).into()
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::Activate(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetDefaultPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidport: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::GetDefaultPort(this, core::mem::transmute_copy(&pguidport)).into()
        }
        unsafe extern "system" fn SetDirectSound<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDirectMusic_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic_Impl::SetDirectSound(this, windows_core::from_raw_borrowed(&pdirectsound), core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumPort: EnumPort::<Identity, OFFSET>,
            CreateMusicBuffer: CreateMusicBuffer::<Identity, OFFSET>,
            CreatePort: CreatePort::<Identity, OFFSET>,
            EnumMasterClock: EnumMasterClock::<Identity, OFFSET>,
            GetMasterClock: GetMasterClock::<Identity, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            GetDefaultPort: GetDefaultPort::<Identity, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusic as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub trait IDirectMusic8_Impl: Sized + IDirectMusic_Impl {
    fn SetExternalMasterClock(&self, pclock: Option<&super::super::IReferenceClock>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl windows_core::RuntimeName for IDirectMusic8 {}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl IDirectMusic8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusic8_Vtbl
    where
        Identity: IDirectMusic8_Impl,
    {
        unsafe extern "system" fn SetExternalMasterClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclock: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusic8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusic8_Impl::SetExternalMasterClock(this, windows_core::from_raw_borrowed(&pclock)).into()
        }
        Self { base__: IDirectMusic_Vtbl::new::<Identity, OFFSET>(), SetExternalMasterClock: SetExternalMasterClock::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusic8 as windows_core::Interface>::IID || iid == &<IDirectMusic as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicBuffer_Impl: Sized {
    fn Flush(&self) -> windows_core::Result<()>;
    fn TotalTime(&self, prttime: *mut i64) -> windows_core::Result<()>;
    fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> windows_core::Result<()>;
    fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> windows_core::Result<()>;
    fn ResetReadPtr(&self) -> windows_core::Result<()>;
    fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetStartTime(&self, prt: *mut i64) -> windows_core::Result<()>;
    fn GetUsedBytes(&self, pcb: *mut u32) -> windows_core::Result<()>;
    fn GetMaxBytes(&self, pcb: *mut u32) -> windows_core::Result<()>;
    fn GetBufferFormat(&self, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetStartTime(&self, rt: i64) -> windows_core::Result<()>;
    fn SetUsedBytes(&self, cb: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicBuffer {}
impl IDirectMusicBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicBuffer_Vtbl
    where
        Identity: IDirectMusicBuffer_Impl,
    {
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::Flush(this).into()
        }
        unsafe extern "system" fn TotalTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prttime: *mut i64) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::TotalTime(this, core::mem::transmute_copy(&prttime)).into()
        }
        unsafe extern "system" fn PackStructured<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::PackStructured(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannelmessage)).into()
        }
        unsafe extern "system" fn PackUnstructured<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::PackUnstructured(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&lpb)).into()
        }
        unsafe extern "system" fn ResetReadPtr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::ResetReadPtr(this).into()
        }
        unsafe extern "system" fn GetNextEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::GetNextEvent(this, core::mem::transmute_copy(&prt), core::mem::transmute_copy(&pdwchannelgroup), core::mem::transmute_copy(&pdwlength), core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetRawBufferPtr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::GetRawBufferPtr(this, core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prt: *mut i64) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::GetStartTime(this, core::mem::transmute_copy(&prt)).into()
        }
        unsafe extern "system" fn GetUsedBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::GetUsedBytes(this, core::mem::transmute_copy(&pcb)).into()
        }
        unsafe extern "system" fn GetMaxBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::GetMaxBytes(this, core::mem::transmute_copy(&pcb)).into()
        }
        unsafe extern "system" fn GetBufferFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::GetBufferFormat(this, core::mem::transmute_copy(&pguidformat)).into()
        }
        unsafe extern "system" fn SetStartTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::SetStartTime(this, core::mem::transmute_copy(&rt)).into()
        }
        unsafe extern "system" fn SetUsedBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicBuffer_Impl::SetUsedBytes(this, core::mem::transmute_copy(&cb)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Flush: Flush::<Identity, OFFSET>,
            TotalTime: TotalTime::<Identity, OFFSET>,
            PackStructured: PackStructured::<Identity, OFFSET>,
            PackUnstructured: PackUnstructured::<Identity, OFFSET>,
            ResetReadPtr: ResetReadPtr::<Identity, OFFSET>,
            GetNextEvent: GetNextEvent::<Identity, OFFSET>,
            GetRawBufferPtr: GetRawBufferPtr::<Identity, OFFSET>,
            GetStartTime: GetStartTime::<Identity, OFFSET>,
            GetUsedBytes: GetUsedBytes::<Identity, OFFSET>,
            GetMaxBytes: GetMaxBytes::<Identity, OFFSET>,
            GetBufferFormat: GetBufferFormat::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            SetUsedBytes: SetUsedBytes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicBuffer as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicCollection_Impl: Sized {
    fn GetInstrument(&self, dwpatch: u32) -> windows_core::Result<IDirectMusicInstrument>;
    fn EnumInstrument(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: &windows_core::PCWSTR, dwnamelen: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicCollection {}
impl IDirectMusicCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicCollection_Vtbl
    where
        Identity: IDirectMusicCollection_Impl,
    {
        unsafe extern "system" fn GetInstrument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpatch: u32, ppinstrument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectMusicCollection_Impl::GetInstrument(this, core::mem::transmute_copy(&dwpatch)) {
                Ok(ok__) => {
                    ppinstrument.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumInstrument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: windows_core::PCWSTR, dwnamelen: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicCollection_Impl::EnumInstrument(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pdwpatch), core::mem::transmute(&pwszname), core::mem::transmute_copy(&dwnamelen)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInstrument: GetInstrument::<Identity, OFFSET>,
            EnumInstrument: EnumInstrument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicCollection as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicDownload_Impl: Sized {
    fn GetBuffer(&self, ppvbuffer: *mut *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicDownload {}
impl IDirectMusicDownload_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicDownload_Vtbl
    where
        Identity: IDirectMusicDownload_Impl,
    {
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvbuffer: *mut *mut core::ffi::c_void, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicDownload_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicDownload_Impl::GetBuffer(this, core::mem::transmute_copy(&ppvbuffer), core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBuffer: GetBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicDownload as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicDownloadedInstrument_Impl: Sized {}
impl windows_core::RuntimeName for IDirectMusicDownloadedInstrument {}
impl IDirectMusicDownloadedInstrument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicDownloadedInstrument_Vtbl
    where
        Identity: IDirectMusicDownloadedInstrument_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicDownloadedInstrument as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicInstrument_Impl: Sized {
    fn GetPatch(&self, pdwpatch: *mut u32) -> windows_core::Result<()>;
    fn SetPatch(&self, dwpatch: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicInstrument {}
impl IDirectMusicInstrument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicInstrument_Vtbl
    where
        Identity: IDirectMusicInstrument_Impl,
    {
        unsafe extern "system" fn GetPatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpatch: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicInstrument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicInstrument_Impl::GetPatch(this, core::mem::transmute_copy(&pdwpatch)).into()
        }
        unsafe extern "system" fn SetPatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpatch: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicInstrument_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicInstrument_Impl::SetPatch(this, core::mem::transmute_copy(&dwpatch)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPatch: GetPatch::<Identity, OFFSET>, SetPatch: SetPatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicInstrument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
pub trait IDirectMusicPort_Impl: Sized {
    fn PlayBuffer(&self, pbuffer: Option<&IDirectMusicBuffer>) -> windows_core::Result<()>;
    fn SetReadNotificationHandle(&self, hevent: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn Read(&self, pbuffer: Option<&IDirectMusicBuffer>) -> windows_core::Result<()>;
    fn DownloadInstrument(&self, pinstrument: Option<&IDirectMusicInstrument>, ppdownloadedinstrument: *mut Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> windows_core::Result<()>;
    fn UnloadInstrument(&self, pdownloadedinstrument: Option<&IDirectMusicDownloadedInstrument>) -> windows_core::Result<()>;
    fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock>;
    fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()>;
    fn Compact(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::Result<()>;
    fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> windows_core::Result<()>;
    fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> windows_core::Result<()>;
    fn Activate(&self, factive: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()>;
    fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: Option<&super::DirectSound::IDirectSound>, pdirectsoundbuffer: Option<&super::DirectSound::IDirectSoundBuffer>) -> windows_core::Result<()>;
    fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl windows_core::RuntimeName for IDirectMusicPort {}
#[cfg(all(feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl IDirectMusicPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicPort_Vtbl
    where
        Identity: IDirectMusicPort_Impl,
    {
        unsafe extern "system" fn PlayBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::PlayBuffer(this, windows_core::from_raw_borrowed(&pbuffer)).into()
        }
        unsafe extern "system" fn SetReadNotificationHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::SetReadNotificationHandle(this, core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::Read(this, windows_core::from_raw_borrowed(&pbuffer)).into()
        }
        unsafe extern "system" fn DownloadInstrument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstrument: *mut core::ffi::c_void, ppdownloadedinstrument: *mut *mut core::ffi::c_void, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::DownloadInstrument(this, windows_core::from_raw_borrowed(&pinstrument), core::mem::transmute_copy(&ppdownloadedinstrument), core::mem::transmute_copy(&pnoteranges), core::mem::transmute_copy(&dwnumnoteranges)).into()
        }
        unsafe extern "system" fn UnloadInstrument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdownloadedinstrument: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::UnloadInstrument(this, windows_core::from_raw_borrowed(&pdownloadedinstrument)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectMusicPort_Impl::GetLatencyClock(this) {
                Ok(ok__) => {
                    ppclock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::GetRunningStats(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn Compact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::Compact(this).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::GetCaps(this, core::mem::transmute_copy(&pportcaps)).into()
        }
        unsafe extern "system" fn DeviceIoControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::DeviceIoControl(this, core::mem::transmute_copy(&dwiocontrolcode), core::mem::transmute_copy(&lpinbuffer), core::mem::transmute_copy(&ninbuffersize), core::mem::transmute_copy(&lpoutbuffer), core::mem::transmute_copy(&noutbuffersize), core::mem::transmute_copy(&lpbytesreturned), core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroups: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::SetNumChannelGroups(this, core::mem::transmute_copy(&dwchannelgroups)).into()
        }
        unsafe extern "system" fn GetNumChannelGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwchannelgroups: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::GetNumChannelGroups(this, core::mem::transmute_copy(&pdwchannelgroups)).into()
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factive: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::Activate(this, core::mem::transmute_copy(&factive)).into()
        }
        unsafe extern "system" fn SetChannelPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::SetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetChannelPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::GetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&pdwpriority)).into()
        }
        unsafe extern "system" fn SetDirectSound<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, pdirectsoundbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::SetDirectSound(this, windows_core::from_raw_borrowed(&pdirectsound), windows_core::from_raw_borrowed(&pdirectsoundbuffer)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPort_Impl::GetFormat(this, core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&pdwwaveformatexsize), core::mem::transmute_copy(&pdwbuffersize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PlayBuffer: PlayBuffer::<Identity, OFFSET>,
            SetReadNotificationHandle: SetReadNotificationHandle::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            DownloadInstrument: DownloadInstrument::<Identity, OFFSET>,
            UnloadInstrument: UnloadInstrument::<Identity, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, OFFSET>,
            Compact: Compact::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, OFFSET>,
            GetNumChannelGroups: GetNumChannelGroups::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicPort as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicPortDownload_Impl: Sized {
    fn GetBuffer(&self, dwdlid: u32) -> windows_core::Result<IDirectMusicDownload>;
    fn AllocateBuffer(&self, dwsize: u32) -> windows_core::Result<IDirectMusicDownload>;
    fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> windows_core::Result<()>;
    fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()>;
    fn Download(&self, pidmdownload: Option<&IDirectMusicDownload>) -> windows_core::Result<()>;
    fn Unload(&self, pidmdownload: Option<&IDirectMusicDownload>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicPortDownload {}
impl IDirectMusicPortDownload_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicPortDownload_Vtbl
    where
        Identity: IDirectMusicPortDownload_Impl,
    {
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPortDownload_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectMusicPortDownload_Impl::GetBuffer(this, core::mem::transmute_copy(&dwdlid)) {
                Ok(ok__) => {
                    ppidmdownload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsize: u32, ppidmdownload: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPortDownload_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectMusicPortDownload_Impl::AllocateBuffer(this, core::mem::transmute_copy(&dwsize)) {
                Ok(ok__) => {
                    ppidmdownload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDLId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPortDownload_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPortDownload_Impl::GetDLId(this, core::mem::transmute_copy(&pdwstartdlid), core::mem::transmute_copy(&dwcount)).into()
        }
        unsafe extern "system" fn GetAppend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwappend: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPortDownload_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPortDownload_Impl::GetAppend(this, core::mem::transmute_copy(&pdwappend)).into()
        }
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidmdownload: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPortDownload_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPortDownload_Impl::Download(this, windows_core::from_raw_borrowed(&pidmdownload)).into()
        }
        unsafe extern "system" fn Unload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidmdownload: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicPortDownload_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicPortDownload_Impl::Unload(this, windows_core::from_raw_borrowed(&pidmdownload)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, OFFSET>,
            GetDLId: GetDLId::<Identity, OFFSET>,
            GetAppend: GetAppend::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            Unload: Unload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicPortDownload as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicSynth_Impl: Sized {
    fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn SetNumChannelGroups(&self, dwgroups: u32) -> windows_core::Result<()>;
    fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut core::ffi::c_void, pbfree: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Unload(&self, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> windows_core::Result<()>;
    fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::Result<()>;
    fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> windows_core::Result<()>;
    fn SetMasterClock(&self, pclock: Option<&super::super::IReferenceClock>) -> windows_core::Result<()>;
    fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock>;
    fn Activate(&self, fenable: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetSynthSink(&self, psynthsink: Option<&IDirectMusicSynthSink>) -> windows_core::Result<()>;
    fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> windows_core::Result<()>;
    fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::Result<()>;
    fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::Result<()>;
    fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> windows_core::Result<()>;
    fn GetAppend(&self, pdwappend: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicSynth {}
impl IDirectMusicSynth_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicSynth_Vtbl
    where
        Identity: IDirectMusicSynth_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::Open(this, core::mem::transmute_copy(&pportparams)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::Close(this).into()
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwgroups: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::SetNumChannelGroups(this, core::mem::transmute_copy(&dwgroups)).into()
        }
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut core::ffi::c_void, pbfree: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::Download(this, core::mem::transmute_copy(&phdownload), core::mem::transmute_copy(&pvdata), core::mem::transmute_copy(&pbfree)).into()
        }
        unsafe extern "system" fn Unload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::Unload(this, core::mem::transmute_copy(&hdownload), core::mem::transmute_copy(&lpfreehandle), core::mem::transmute_copy(&huserdata)).into()
        }
        unsafe extern "system" fn PlayBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::PlayBuffer(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&cbbuffer)).into()
        }
        unsafe extern "system" fn GetRunningStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::GetRunningStats(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetPortCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::GetPortCaps(this, core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn SetMasterClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclock: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::SetMasterClock(this, windows_core::from_raw_borrowed(&pclock)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectMusicSynth_Impl::GetLatencyClock(this) {
                Ok(ok__) => {
                    ppclock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::Activate(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetSynthSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psynthsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::SetSynthSink(this, windows_core::from_raw_borrowed(&psynthsink)).into()
        }
        unsafe extern "system" fn Render<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::Render(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&llposition)).into()
        }
        unsafe extern "system" fn SetChannelPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::SetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetChannelPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::GetChannelPriority(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&pdwpriority)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::GetFormat(this, core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&pdwwaveformatexsize)).into()
        }
        unsafe extern "system" fn GetAppend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwappend: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth_Impl::GetAppend(this, core::mem::transmute_copy(&pdwappend)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            Unload: Unload::<Identity, OFFSET>,
            PlayBuffer: PlayBuffer::<Identity, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, OFFSET>,
            GetPortCaps: GetPortCaps::<Identity, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            SetSynthSink: SetSynthSink::<Identity, OFFSET>,
            Render: Render::<Identity, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            GetAppend: GetAppend::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicSynth as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicSynth8_Impl: Sized + IDirectMusicSynth_Impl {
    fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> windows_core::Result<()>;
    fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> windows_core::Result<()>;
    fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> windows_core::Result<()>;
    fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> windows_core::Result<()>;
    fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicSynth8 {}
impl IDirectMusicSynth8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicSynth8_Vtbl
    where
        Identity: IDirectMusicSynth8_Impl,
    {
        unsafe extern "system" fn PlayVoice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth8_Impl::PlayVoice(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwvoiceid), core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&dwdlid), core::mem::transmute_copy(&prpitch), core::mem::transmute_copy(&vrvolume), core::mem::transmute_copy(&stvoicestart), core::mem::transmute_copy(&stloopstart), core::mem::transmute_copy(&stloopend)).into()
        }
        unsafe extern "system" fn StopVoice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rt: i64, dwvoiceid: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth8_Impl::StopVoice(this, core::mem::transmute_copy(&rt), core::mem::transmute_copy(&dwvoiceid)).into()
        }
        unsafe extern "system" fn GetVoiceState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth8_Impl::GetVoiceState(this, core::mem::transmute_copy(&dwvoice), core::mem::transmute_copy(&cbvoice), core::mem::transmute_copy(&dwvoicestate)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth8_Impl::Refresh(this, core::mem::transmute_copy(&dwdownloadid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn AssignChannelToBuses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynth8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynth8_Impl::AssignChannelToBuses(this, core::mem::transmute_copy(&dwchannelgroup), core::mem::transmute_copy(&dwchannel), core::mem::transmute_copy(&pdwbuses), core::mem::transmute_copy(&cbuses)).into()
        }
        Self {
            base__: IDirectMusicSynth_Vtbl::new::<Identity, OFFSET>(),
            PlayVoice: PlayVoice::<Identity, OFFSET>,
            StopVoice: StopVoice::<Identity, OFFSET>,
            GetVoiceState: GetVoiceState::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            AssignChannelToBuses: AssignChannelToBuses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicSynth8 as windows_core::Interface>::IID || iid == &<IDirectMusicSynth as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub trait IDirectMusicSynthSink_Impl: Sized {
    fn Init(&self, psynth: Option<&IDirectMusicSynth>) -> windows_core::Result<()>;
    fn SetMasterClock(&self, pclock: Option<&super::super::IReferenceClock>) -> windows_core::Result<()>;
    fn GetLatencyClock(&self) -> windows_core::Result<super::super::IReferenceClock>;
    fn Activate(&self, fenable: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> windows_core::Result<()>;
    fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> windows_core::Result<()>;
    fn SetDirectSound(&self, pdirectsound: Option<&super::DirectSound::IDirectSound>, pdirectsoundbuffer: Option<&super::DirectSound::IDirectSoundBuffer>) -> windows_core::Result<()>;
    fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl windows_core::RuntimeName for IDirectMusicSynthSink {}
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
impl IDirectMusicSynthSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicSynthSink_Vtbl
    where
        Identity: IDirectMusicSynthSink_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psynth: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynthSink_Impl::Init(this, windows_core::from_raw_borrowed(&psynth)).into()
        }
        unsafe extern "system" fn SetMasterClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclock: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynthSink_Impl::SetMasterClock(this, windows_core::from_raw_borrowed(&pclock)).into()
        }
        unsafe extern "system" fn GetLatencyClock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectMusicSynthSink_Impl::GetLatencyClock(this) {
                Ok(ok__) => {
                    ppclock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynthSink_Impl::Activate(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SampleToRefTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynthSink_Impl::SampleToRefTime(this, core::mem::transmute_copy(&llsampletime), core::mem::transmute_copy(&prftime)).into()
        }
        unsafe extern "system" fn RefTimeToSample<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynthSink_Impl::RefTimeToSample(this, core::mem::transmute_copy(&rftime), core::mem::transmute_copy(&pllsampletime)).into()
        }
        unsafe extern "system" fn SetDirectSound<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, pdirectsoundbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynthSink_Impl::SetDirectSound(this, windows_core::from_raw_borrowed(&pdirectsound), windows_core::from_raw_borrowed(&pdirectsoundbuffer)).into()
        }
        unsafe extern "system" fn GetDesiredBufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectMusicSynthSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicSynthSink_Impl::GetDesiredBufferSize(this, core::mem::transmute_copy(&pdwbuffersizeinsamples)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            SampleToRefTime: SampleToRefTime::<Identity, OFFSET>,
            RefTimeToSample: RefTimeToSample::<Identity, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, OFFSET>,
            GetDesiredBufferSize: GetDesiredBufferSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicSynthSink as windows_core::Interface>::IID
    }
}
pub trait IDirectMusicThru_Impl: Sized {
    fn ThruChannel(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: Option<&IDirectMusicPort>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectMusicThru {}
impl IDirectMusicThru_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectMusicThru_Vtbl
    where
        Identity: IDirectMusicThru_Impl,
    {
        unsafe extern "system" fn ThruChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectMusicThru_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectMusicThru_Impl::ThruChannel(this, core::mem::transmute_copy(&dwsourcechannelgroup), core::mem::transmute_copy(&dwsourcechannel), core::mem::transmute_copy(&dwdestinationchannelgroup), core::mem::transmute_copy(&dwdestinationchannel), windows_core::from_raw_borrowed(&pdestinationport)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ThruChannel: ThruChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectMusicThru as windows_core::Interface>::IID
    }
}
