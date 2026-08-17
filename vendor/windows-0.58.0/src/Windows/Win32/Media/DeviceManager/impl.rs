pub trait IComponentAuthenticate_Impl: Sized {
    fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> windows_core::Result<()>;
    fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComponentAuthenticate {}
impl IComponentAuthenticate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComponentAuthenticate_Vtbl
    where
        Identity: IComponentAuthenticate_Impl,
    {
        unsafe extern "system" fn SACAuth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComponentAuthenticate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComponentAuthenticate_Impl::SACAuth(this, core::mem::transmute_copy(&dwprotocolid), core::mem::transmute_copy(&dwpass), core::mem::transmute_copy(&pbdatain), core::mem::transmute_copy(&dwdatainlen), core::mem::transmute_copy(&ppbdataout), core::mem::transmute_copy(&pdwdataoutlen)).into()
        }
        unsafe extern "system" fn SACGetProtocols<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComponentAuthenticate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComponentAuthenticate_Impl::SACGetProtocols(this, core::mem::transmute_copy(&ppdwprotocols), core::mem::transmute_copy(&pdwprotocolcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SACAuth: SACAuth::<Identity, OFFSET>,
            SACGetProtocols: SACGetProtocols::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComponentAuthenticate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDevice_Impl: Sized {
    fn GetName(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetManufacturer(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetVersion(&self) -> windows_core::Result<u32>;
    fn GetType(&self) -> windows_core::Result<u32>;
    fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn GetDeviceIcon(&self) -> windows_core::Result<u32>;
    fn EnumStorage(&self) -> windows_core::Result<IMDSPEnumStorage>;
    fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::Result<()>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IMDSPDevice {}
#[cfg(feature = "Win32_Media_Audio")]
impl IMDSPDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPDevice_Vtbl
    where
        Identity: IMDSPDevice_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetManufacturer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice_Impl::GetManufacturer(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice_Impl::GetVersion(this) {
                Ok(ok__) => {
                    pdwversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice_Impl::GetType(this) {
                Ok(ok__) => {
                    pdwtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnumber), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetPowerSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice_Impl::GetPowerSource(this, core::mem::transmute_copy(&pdwpowersource), core::mem::transmute_copy(&pdwpercentremaining)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice_Impl::GetDeviceIcon(this) {
                Ok(ok__) => {
                    hicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice_Impl::EnumStorage(this) {
                Ok(ok__) => {
                    ppenumstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice_Impl::GetFormatSupport(this, core::mem::transmute_copy(&pformatex), core::mem::transmute_copy(&pnformatcount), core::mem::transmute_copy(&pppwszmimetype), core::mem::transmute_copy(&pnmimetypecount)).into()
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetManufacturer: GetManufacturer::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, OFFSET>,
            GetPowerSource: GetPowerSource::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, OFFSET>,
            EnumStorage: EnumStorage::<Identity, OFFSET>,
            GetFormatSupport: GetFormatSupport::<Identity, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice2_Impl: Sized + IMDSPDevice_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
    fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()>;
    fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()>;
    fn GetCanonicalName(&self, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IMDSPDevice2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IMDSPDevice2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPDevice2_Vtbl
    where
        Identity: IMDSPDevice2_Impl,
    {
        unsafe extern "system" fn GetStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice2_Impl::GetFormatSupport2(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppaudioformatex), core::mem::transmute_copy(&pnaudioformatcount), core::mem::transmute_copy(&ppvideoformatex), core::mem::transmute_copy(&pnvideoformatcount), core::mem::transmute_copy(&ppfiletype), core::mem::transmute_copy(&pnfiletypecount)).into()
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppspecifyproppages: *mut *mut core::ffi::c_void, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice2_Impl::GetSpecifyPropertyPages(this, core::mem::transmute_copy(&ppspecifyproppages), core::mem::transmute_copy(&pppunknowns), core::mem::transmute_copy(&pcunks)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice2_Impl::GetCanonicalName(this, core::mem::transmute_copy(&pwszpnpname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        Self {
            base__: IMDSPDevice_Vtbl::new::<Identity, OFFSET>(),
            GetStorage: GetStorage::<Identity, OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Identity, OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Identity, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPDevice2 as windows_core::Interface>::IID || iid == &<IMDSPDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice3_Impl: Sized + IMDSPDevice2_Impl {
    fn GetProperty(&self, pwszpropname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetProperty(&self, pwszpropname: &windows_core::PCWSTR, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IMDSPDevice3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IMDSPDevice3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPDevice3_Vtbl
    where
        Identity: IMDSPDevice3_Impl,
    {
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice3_Impl::GetProperty(this, core::mem::transmute(&pwszpropname)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice3_Impl::SetProperty(this, core::mem::transmute(&pwszpropname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetFormatCapability<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice3_Impl::GetFormatCapability(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    pformatsupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDevice3_Impl::DeviceIoControl(this, core::mem::transmute_copy(&dwiocontrolcode), core::mem::transmute_copy(&lpinbuffer), core::mem::transmute_copy(&ninbuffersize), core::mem::transmute_copy(&lpoutbuffer), core::mem::transmute_copy(&pnoutbuffersize)).into()
        }
        unsafe extern "system" fn FindStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDevice3_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMDSPDevice2_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetFormatCapability: GetFormatCapability::<Identity, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, OFFSET>,
            FindStorage: FindStorage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPDevice3 as windows_core::Interface>::IID || iid == &<IMDSPDevice as windows_core::Interface>::IID || iid == &<IMDSPDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDeviceControl_Impl: Sized {
    fn GetDCStatus(&self) -> windows_core::Result<u32>;
    fn GetCapabilities(&self) -> windows_core::Result<u32>;
    fn Play(&self) -> windows_core::Result<()>;
    fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Seek(&self, fumode: u32, noffset: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IMDSPDeviceControl {}
#[cfg(feature = "Win32_Media_Audio")]
impl IMDSPDeviceControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPDeviceControl_Vtbl
    where
        Identity: IMDSPDeviceControl_Impl,
    {
        unsafe extern "system" fn GetDCStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDeviceControl_Impl::GetDCStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDeviceControl_Impl::GetCapabilities(this) {
                Ok(ok__) => {
                    pdwcapabilitiesmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDeviceControl_Impl::Play(this).into()
        }
        unsafe extern "system" fn Record<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDeviceControl_Impl::Record(this, core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDeviceControl_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDeviceControl_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDeviceControl_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, noffset: i32) -> windows_core::HRESULT
        where
            Identity: IMDSPDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPDeviceControl_Impl::Seek(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&noffset)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDCStatus: GetDCStatus::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            Record: Record::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPDeviceControl as windows_core::Interface>::IID
    }
}
pub trait IMDSPDirectTransfer_Impl: Sized {
    fn TransferToDevice(&self, pwszsourcefilepath: &windows_core::PCWSTR, psourceoperation: Option<&IWMDMOperation>, fuflags: u32, pwszdestinationname: &windows_core::PCWSTR, psourcemetadata: Option<&IWMDMMetaData>, ptransferprogress: Option<&IWMDMProgress>) -> windows_core::Result<IMDSPStorage>;
}
impl windows_core::RuntimeName for IMDSPDirectTransfer {}
impl IMDSPDirectTransfer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPDirectTransfer_Vtbl
    where
        Identity: IMDSPDirectTransfer_Impl,
    {
        unsafe extern "system" fn TransferToDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsourcefilepath: windows_core::PCWSTR, psourceoperation: *mut core::ffi::c_void, fuflags: u32, pwszdestinationname: windows_core::PCWSTR, psourcemetadata: *mut core::ffi::c_void, ptransferprogress: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPDirectTransfer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPDirectTransfer_Impl::TransferToDevice(this, core::mem::transmute(&pwszsourcefilepath), windows_core::from_raw_borrowed(&psourceoperation), core::mem::transmute_copy(&fuflags), core::mem::transmute(&pwszdestinationname), windows_core::from_raw_borrowed(&psourcemetadata), windows_core::from_raw_borrowed(&ptransferprogress)) {
                Ok(ok__) => {
                    ppnewobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TransferToDevice: TransferToDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPDirectTransfer as windows_core::Interface>::IID
    }
}
pub trait IMDSPEnumDevice_Impl: Sized {
    fn Next(&self, celt: u32, ppdevice: *mut Option<IMDSPDevice>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMDSPEnumDevice>;
}
impl windows_core::RuntimeName for IMDSPEnumDevice {}
impl IMDSPEnumDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPEnumDevice_Vtbl
    where
        Identity: IMDSPEnumDevice_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppdevice: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPEnumDevice_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppdevice), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPEnumDevice_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                Ok(ok__) => {
                    pceltfetched.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPEnumDevice_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPEnumDevice_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenumdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPEnumDevice as windows_core::Interface>::IID
    }
}
pub trait IMDSPEnumStorage_Impl: Sized {
    fn Next(&self, celt: u32, ppstorage: *mut Option<IMDSPStorage>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMDSPEnumStorage>;
}
impl windows_core::RuntimeName for IMDSPEnumStorage {}
impl IMDSPEnumStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPEnumStorage_Vtbl
    where
        Identity: IMDSPEnumStorage_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppstorage: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPEnumStorage_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppstorage), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPEnumStorage_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                Ok(ok__) => {
                    pceltfetched.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPEnumStorage_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPEnumStorage_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenumstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPEnumStorage as windows_core::Interface>::IID
    }
}
pub trait IMDSPObject_Impl: Sized {
    fn Open(&self, fumode: u32) -> windows_core::Result<()>;
    fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn Delete(&self, fumode: u32, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<()>;
    fn Seek(&self, fuflags: u32, dwoffset: u32) -> windows_core::Result<()>;
    fn Rename(&self, pwsznewname: &windows_core::PCWSTR, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<()>;
    fn Move(&self, fumode: u32, pprogress: Option<&IWMDMProgress>, ptarget: Option<&IMDSPStorage>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMDSPObject {}
impl IMDSPObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPObject_Vtbl
    where
        Identity: IMDSPObject_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Open(this, core::mem::transmute_copy(&fumode)).into()
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Read(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Write(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Delete(this, core::mem::transmute_copy(&fumode), windows_core::from_raw_borrowed(&pprogress)).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, dwoffset: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Seek(this, core::mem::transmute_copy(&fuflags), core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznewname: windows_core::PCWSTR, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Rename(this, core::mem::transmute(&pwsznewname), windows_core::from_raw_borrowed(&pprogress)).into()
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Move(this, core::mem::transmute_copy(&fumode), windows_core::from_raw_borrowed(&pprogress), windows_core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPObject as windows_core::Interface>::IID
    }
}
pub trait IMDSPObject2_Impl: Sized + IMDSPObject_Impl {
    fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMDSPObject2 {}
impl IMDSPObject2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPObject2_Vtbl
    where
        Identity: IMDSPObject2_Impl,
    {
        unsafe extern "system" fn ReadOnClearChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObject2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject2_Impl::ReadOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn WriteOnClearChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObject2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObject2_Impl::WriteOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base__: IMDSPObject_Vtbl::new::<Identity, OFFSET>(),
            ReadOnClearChannel: ReadOnClearChannel::<Identity, OFFSET>,
            WriteOnClearChannel: WriteOnClearChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPObject2 as windows_core::Interface>::IID || iid == &<IMDSPObject as windows_core::Interface>::IID
    }
}
pub trait IMDSPObjectInfo_Impl: Sized {
    fn GetPlayLength(&self) -> windows_core::Result<u32>;
    fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()>;
    fn GetPlayOffset(&self) -> windows_core::Result<u32>;
    fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()>;
    fn GetTotalLength(&self) -> windows_core::Result<u32>;
    fn GetLastPlayPosition(&self) -> windows_core::Result<u32>;
    fn GetLongestPlayPosition(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMDSPObjectInfo {}
impl IMDSPObjectInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPObjectInfo_Vtbl
    where
        Identity: IMDSPObjectInfo_Impl,
    {
        unsafe extern "system" fn GetPlayLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPObjectInfo_Impl::GetPlayLength(this) {
                Ok(ok__) => {
                    pdwlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlength: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObjectInfo_Impl::SetPlayLength(this, core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetPlayOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoffset: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPObjectInfo_Impl::GetPlayOffset(this) {
                Ok(ok__) => {
                    pdwoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPObjectInfo_Impl::SetPlayOffset(this, core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn GetTotalLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPObjectInfo_Impl::GetTotalLength(this) {
                Ok(ok__) => {
                    pdwlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastpos: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPObjectInfo_Impl::GetLastPlayPosition(this) {
                Ok(ok__) => {
                    pdwlastpos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlongestpos: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPObjectInfo_Impl::GetLongestPlayPosition(this) {
                Ok(ok__) => {
                    pdwlongestpos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPlayLength: GetPlayLength::<Identity, OFFSET>,
            SetPlayLength: SetPlayLength::<Identity, OFFSET>,
            GetPlayOffset: GetPlayOffset::<Identity, OFFSET>,
            SetPlayOffset: SetPlayOffset::<Identity, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Identity, OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPObjectInfo as windows_core::Interface>::IID
    }
}
pub trait IMDSPRevoked_Impl: Sized {
    fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMDSPRevoked {}
impl IMDSPRevoked_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPRevoked_Vtbl
    where
        Identity: IMDSPRevoked_Impl,
    {
        unsafe extern "system" fn GetRevocationURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPRevoked_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPRevoked_Impl::GetRevocationURL(this, core::mem::transmute_copy(&ppwszrevocationurl), core::mem::transmute_copy(&pdwbufferlen)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRevocationURL: GetRevocationURL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPRevoked as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPStorage_Impl: Sized {
    fn SetAttributes(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetStorageGlobals(&self) -> windows_core::Result<IMDSPStorageGlobals>;
    fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetName(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetDate(&self) -> windows_core::Result<WMDMDATETIME>;
    fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()>;
    fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn CreateStorage(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
    fn EnumStorage(&self) -> windows_core::Result<IMDSPEnumStorage>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IMDSPStorage {}
#[cfg(feature = "Win32_Media_Audio")]
impl IMDSPStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPStorage_Vtbl
    where
        Identity: IMDSPStorage_Impl,
    {
        unsafe extern "system" fn SetAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage_Impl::SetAttributes(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorageglobals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage_Impl::GetStorageGlobals(this) {
                Ok(ok__) => {
                    ppstorageglobals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage_Impl::GetAttributes(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage_Impl::GetDate(this) {
                Ok(ok__) => {
                    pdatetimeutc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage_Impl::GetSize(this, core::mem::transmute_copy(&pdwsizelow), core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn GetRights<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage_Impl::GetRights(this, core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn CreateStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: windows_core::PCWSTR, ppnewstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage_Impl::CreateStorage(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat), core::mem::transmute(&pwszname)) {
                Ok(ok__) => {
                    ppnewstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage_Impl::EnumStorage(this) {
                Ok(ok__) => {
                    ppenumstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAttributes: SetAttributes::<Identity, OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetDate: GetDate::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetRights: GetRights::<Identity, OFFSET>,
            CreateStorage: CreateStorage::<Identity, OFFSET>,
            EnumStorage: EnumStorage::<Identity, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage2_Impl: Sized + IMDSPStorage_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
    fn CreateStorage2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: &windows_core::PCWSTR, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>;
    fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
    fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IMDSPStorage2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPStorage2_Vtbl
    where
        Identity: IMDSPStorage2_Impl,
    {
        unsafe extern "system" fn GetStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorage2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: windows_core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage2_Impl::CreateStorage2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat), core::mem::transmute(&pwszname), core::mem::transmute_copy(&qwfilesize)) {
                Ok(ok__) => {
                    ppnewstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage2_Impl::SetAttributes2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetAttributes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage2_Impl::GetAttributes2(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pdwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base__: IMDSPStorage_Vtbl::new::<Identity, OFFSET>(),
            GetStorage: GetStorage::<Identity, OFFSET>,
            CreateStorage2: CreateStorage2::<Identity, OFFSET>,
            SetAttributes2: SetAttributes2::<Identity, OFFSET>,
            GetAttributes2: GetAttributes2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPStorage2 as windows_core::Interface>::IID || iid == &<IMDSPStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage3_Impl: Sized + IMDSPStorage2_Impl {
    fn GetMetadata(&self, pmetadata: Option<&IWMDMMetaData>) -> windows_core::Result<()>;
    fn SetMetadata(&self, pmetadata: Option<&IWMDMMetaData>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IMDSPStorage3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPStorage3_Vtbl
    where
        Identity: IMDSPStorage3_Impl,
    {
        unsafe extern "system" fn GetMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage3_Impl::GetMetadata(this, windows_core::from_raw_borrowed(&pmetadata)).into()
        }
        unsafe extern "system" fn SetMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage3_Impl::SetMetadata(this, windows_core::from_raw_borrowed(&pmetadata)).into()
        }
        Self {
            base__: IMDSPStorage2_Vtbl::new::<Identity, OFFSET>(),
            GetMetadata: GetMetadata::<Identity, OFFSET>,
            SetMetadata: SetMetadata::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPStorage3 as windows_core::Interface>::IID || iid == &<IMDSPStorage as windows_core::Interface>::IID || iid == &<IMDSPStorage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage4_Impl: Sized + IMDSPStorage3_Impl {
    fn SetReferences(&self, dwrefs: u32, ppispstorage: *const Option<IMDSPStorage>) -> windows_core::Result<()>;
    fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut Option<IMDSPStorage>) -> windows_core::Result<()>;
    fn CreateStorageWithMetadata(&self, dwattributes: u32, pwszname: &windows_core::PCWSTR, pmetadata: Option<&IWMDMMetaData>, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>;
    fn GetSpecifiedMetadata(&self, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR, pmetadata: Option<&IWMDMMetaData>) -> windows_core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
    fn GetParent(&self) -> windows_core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IMDSPStorage4 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPStorage4_Vtbl
    where
        Identity: IMDSPStorage4_Impl,
    {
        unsafe extern "system" fn SetReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrefs: u32, ppispstorage: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage4_Impl::SetReferences(this, core::mem::transmute_copy(&dwrefs), core::mem::transmute_copy(&ppispstorage)).into()
        }
        unsafe extern "system" fn GetReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut Option<IMDSPStorage>) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage4_Impl::GetReferences(this, core::mem::transmute_copy(&pdwrefs), core::mem::transmute_copy(&pppispstorage)).into()
        }
        unsafe extern "system" fn CreateStorageWithMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pwszname: windows_core::PCWSTR, pmetadata: *mut core::ffi::c_void, qwfilesize: u64, ppnewstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage4_Impl::CreateStorageWithMetadata(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute(&pwszname), windows_core::from_raw_borrowed(&pmetadata), core::mem::transmute_copy(&qwfilesize)) {
                Ok(ok__) => {
                    ppnewstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorage4_Impl::GetSpecifiedMetadata(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppwszpropnames), windows_core::from_raw_borrowed(&pmetadata)).into()
        }
        unsafe extern "system" fn FindStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage4_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorage4_Impl::GetParent(this) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMDSPStorage3_Vtbl::new::<Identity, OFFSET>(),
            SetReferences: SetReferences::<Identity, OFFSET>,
            GetReferences: GetReferences::<Identity, OFFSET>,
            CreateStorageWithMetadata: CreateStorageWithMetadata::<Identity, OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Identity, OFFSET>,
            FindStorage: FindStorage::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPStorage4 as windows_core::Interface>::IID || iid == &<IMDSPStorage as windows_core::Interface>::IID || iid == &<IMDSPStorage2 as windows_core::Interface>::IID || iid == &<IMDSPStorage3 as windows_core::Interface>::IID
    }
}
pub trait IMDSPStorageGlobals_Impl: Sized {
    fn GetCapabilities(&self) -> windows_core::Result<u32>;
    fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, fumode: u32, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<()>;
    fn GetDevice(&self) -> windows_core::Result<IMDSPDevice>;
    fn GetRootStorage(&self) -> windows_core::Result<IMDSPStorage>;
}
impl windows_core::RuntimeName for IMDSPStorageGlobals {}
impl IMDSPStorageGlobals_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDSPStorageGlobals_Vtbl
    where
        Identity: IMDSPStorageGlobals_Impl,
    {
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorageGlobals_Impl::GetCapabilities(this) {
                Ok(ok__) => {
                    pdwcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorageGlobals_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnum), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetTotalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorageGlobals_Impl::GetTotalSize(this, core::mem::transmute_copy(&pdwtotalsizelow), core::mem::transmute_copy(&pdwtotalsizehigh)).into()
        }
        unsafe extern "system" fn GetTotalFree<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorageGlobals_Impl::GetTotalFree(this, core::mem::transmute_copy(&pdwfreelow), core::mem::transmute_copy(&pdwfreehigh)).into()
        }
        unsafe extern "system" fn GetTotalBad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorageGlobals_Impl::GetTotalBad(this, core::mem::transmute_copy(&pdwbadlow), core::mem::transmute_copy(&pdwbadhigh)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorageGlobals_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDSPStorageGlobals_Impl::Initialize(this, core::mem::transmute_copy(&fumode), windows_core::from_raw_borrowed(&pprogress)).into()
        }
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorageGlobals_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDSPStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDSPStorageGlobals_Impl::GetRootStorage(this) {
                Ok(ok__) => {
                    pproot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, OFFSET>,
            GetTotalSize: GetTotalSize::<Identity, OFFSET>,
            GetTotalFree: GetTotalFree::<Identity, OFFSET>,
            GetTotalBad: GetTotalBad::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetRootStorage: GetRootStorage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPStorageGlobals as windows_core::Interface>::IID
    }
}
pub trait IMDServiceProvider_Impl: Sized {
    fn GetDeviceCount(&self) -> windows_core::Result<u32>;
    fn EnumDevices(&self) -> windows_core::Result<IMDSPEnumDevice>;
}
impl windows_core::RuntimeName for IMDServiceProvider {}
impl IMDServiceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDServiceProvider_Vtbl
    where
        Identity: IMDServiceProvider_Impl,
    {
        unsafe extern "system" fn GetDeviceCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDServiceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDServiceProvider_Impl::GetDeviceCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDServiceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDServiceProvider_Impl::EnumDevices(this) {
                Ok(ok__) => {
                    ppenumdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceCount: GetDeviceCount::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDServiceProvider as windows_core::Interface>::IID
    }
}
pub trait IMDServiceProvider2_Impl: Sized + IMDServiceProvider_Impl {
    fn CreateDevice(&self, pwszdevicepath: &windows_core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut Option<IMDSPDevice>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMDServiceProvider2 {}
impl IMDServiceProvider2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDServiceProvider2_Vtbl
    where
        Identity: IMDServiceProvider2_Impl,
    {
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicepath: windows_core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut Option<IMDSPDevice>) -> windows_core::HRESULT
        where
            Identity: IMDServiceProvider2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDServiceProvider2_Impl::CreateDevice(this, core::mem::transmute(&pwszdevicepath), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pppdevicearray)).into()
        }
        Self { base__: IMDServiceProvider_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDServiceProvider2 as windows_core::Interface>::IID || iid == &<IMDServiceProvider as windows_core::Interface>::IID
    }
}
pub trait IMDServiceProvider3_Impl: Sized + IMDServiceProvider2_Impl {
    fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMDServiceProvider3 {}
impl IMDServiceProvider3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDServiceProvider3_Vtbl
    where
        Identity: IMDServiceProvider3_Impl,
    {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenumpref: u32) -> windows_core::HRESULT
        where
            Identity: IMDServiceProvider3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDServiceProvider3_Impl::SetDeviceEnumPreference(this, core::mem::transmute_copy(&dwenumpref)).into()
        }
        Self { base__: IMDServiceProvider2_Vtbl::new::<Identity, OFFSET>(), SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDServiceProvider3 as windows_core::Interface>::IID || iid == &<IMDServiceProvider as windows_core::Interface>::IID || iid == &<IMDServiceProvider2 as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureAuthenticate_Impl: Sized {
    fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery>;
}
impl windows_core::RuntimeName for ISCPSecureAuthenticate {}
impl ISCPSecureAuthenticate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureAuthenticate_Vtbl
    where
        Identity: ISCPSecureAuthenticate_Impl,
    {
        unsafe extern "system" fn GetSecureQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCPSecureAuthenticate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCPSecureAuthenticate_Impl::GetSecureQuery(this) {
                Ok(ok__) => {
                    ppsecurequery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSecureQuery: GetSecureQuery::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureAuthenticate2_Impl: Sized + ISCPSecureAuthenticate_Impl {
    fn GetSCPSession(&self) -> windows_core::Result<ISCPSession>;
}
impl windows_core::RuntimeName for ISCPSecureAuthenticate2 {}
impl ISCPSecureAuthenticate2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureAuthenticate2_Vtbl
    where
        Identity: ISCPSecureAuthenticate2_Impl,
    {
        unsafe extern "system" fn GetSCPSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscpsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCPSecureAuthenticate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCPSecureAuthenticate2_Impl::GetSCPSession(this) {
                Ok(ok__) => {
                    ppscpsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISCPSecureAuthenticate_Vtbl::new::<Identity, OFFSET>(), GetSCPSession: GetSCPSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate2 as windows_core::Interface>::IID || iid == &<ISCPSecureAuthenticate as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureExchange_Impl: Sized {
    fn TransferContainerData(&self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn TransferComplete(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISCPSecureExchange {}
impl ISCPSecureExchange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureExchange_Vtbl
    where
        Identity: ISCPSecureExchange_Impl,
    {
        unsafe extern "system" fn TransferContainerData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureExchange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureExchange_Impl::TransferContainerData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pfureadyflags), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn ObjectData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureExchange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureExchange_Impl::ObjectData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn TransferComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCPSecureExchange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureExchange_Impl::TransferComplete(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransferContainerData: TransferContainerData::<Identity, OFFSET>,
            ObjectData: ObjectData::<Identity, OFFSET>,
            TransferComplete: TransferComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureExchange as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureExchange2_Impl: Sized + ISCPSecureExchange_Impl {
    fn TransferContainerData2(&self, pdata: *const u8, dwsize: u32, pprogresscallback: Option<&IWMDMProgress3>, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISCPSecureExchange2 {}
impl ISCPSecureExchange2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureExchange2_Vtbl
    where
        Identity: ISCPSecureExchange2_Impl,
    {
        unsafe extern "system" fn TransferContainerData2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut core::ffi::c_void, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureExchange2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureExchange2_Impl::TransferContainerData2(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), windows_core::from_raw_borrowed(&pprogresscallback), core::mem::transmute_copy(&pfureadyflags), core::mem::transmute_copy(&abmac)).into()
        }
        Self { base__: ISCPSecureExchange_Vtbl::new::<Identity, OFFSET>(), TransferContainerData2: TransferContainerData2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureExchange2 as windows_core::Interface>::IID || iid == &<ISCPSecureExchange as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureExchange3_Impl: Sized + ISCPSecureExchange2_Impl {
    fn TransferContainerDataOnClearChannel(&self, pdevice: Option<&IMDSPDevice>, pdata: *const u8, dwsize: u32, pprogresscallback: Option<&IWMDMProgress3>) -> windows_core::Result<u32>;
    fn GetObjectDataOnClearChannel(&self, pdevice: Option<&IMDSPDevice>, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn TransferCompleteForDevice(&self, pdevice: Option<&IMDSPDevice>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISCPSecureExchange3 {}
impl ISCPSecureExchange3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureExchange3_Vtbl
    where
        Identity: ISCPSecureExchange3_Impl,
    {
        unsafe extern "system" fn TransferContainerDataOnClearChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut core::ffi::c_void, pfureadyflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISCPSecureExchange3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCPSecureExchange3_Impl::TransferContainerDataOnClearChannel(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), windows_core::from_raw_borrowed(&pprogresscallback)) {
                Ok(ok__) => {
                    pfureadyflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectDataOnClearChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISCPSecureExchange3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureExchange3_Impl::GetObjectDataOnClearChannel(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn TransferCompleteForDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCPSecureExchange3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureExchange3_Impl::TransferCompleteForDevice(this, windows_core::from_raw_borrowed(&pdevice)).into()
        }
        Self {
            base__: ISCPSecureExchange2_Vtbl::new::<Identity, OFFSET>(),
            TransferContainerDataOnClearChannel: TransferContainerDataOnClearChannel::<Identity, OFFSET>,
            GetObjectDataOnClearChannel: GetObjectDataOnClearChannel::<Identity, OFFSET>,
            TransferCompleteForDevice: TransferCompleteForDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureExchange3 as windows_core::Interface>::IID || iid == &<ISCPSecureExchange as windows_core::Interface>::IID || iid == &<ISCPSecureExchange2 as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureQuery_Impl: Sized {
    fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn ExamineData(&self, fuflags: u32, pwszextension: &windows_core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn MakeDecision(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: Option<&IMDSPStorageGlobals>, ppexchange: *mut Option<ISCPSecureExchange>, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetRights(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: Option<&IMDSPStorageGlobals>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISCPSecureQuery {}
impl ISCPSecureQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureQuery_Vtbl
    where
        Identity: ISCPSecureQuery_Impl,
    {
        unsafe extern "system" fn GetDataDemands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureQuery_Impl::GetDataDemands(this, core::mem::transmute_copy(&pfuflags), core::mem::transmute_copy(&pdwminrightsdata), core::mem::transmute_copy(&pdwminexaminedata), core::mem::transmute_copy(&pdwmindecidedata), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn ExamineData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pwszextension: windows_core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureQuery_Impl::ExamineData(this, core::mem::transmute_copy(&fuflags), core::mem::transmute(&pwszextension), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn MakeDecision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut core::ffi::c_void, ppexchange: *mut *mut core::ffi::c_void, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureQuery_Impl::MakeDecision(this, core::mem::transmute_copy(&fuflags), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwappsec), core::mem::transmute_copy(&pbspsessionkey), core::mem::transmute_copy(&dwsessionkeylen), windows_core::from_raw_borrowed(&pstorageglobals), core::mem::transmute_copy(&ppexchange), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetRights<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureQuery_Impl::GetRights(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pbspsessionkey), core::mem::transmute_copy(&dwsessionkeylen), windows_core::from_raw_borrowed(&pstgglobals), core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount), core::mem::transmute_copy(&abmac)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDataDemands: GetDataDemands::<Identity, OFFSET>,
            ExamineData: ExamineData::<Identity, OFFSET>,
            MakeDecision: MakeDecision::<Identity, OFFSET>,
            GetRights: GetRights::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureQuery as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureQuery2_Impl: Sized + ISCPSecureQuery_Impl {
    fn MakeDecision2(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: Option<&IMDSPStorageGlobals>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: Option<&windows_core::IUnknown>, ppexchange: *mut Option<ISCPSecureExchange>, abmac: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISCPSecureQuery2 {}
impl ISCPSecureQuery2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureQuery2_Vtbl
    where
        Identity: ISCPSecureQuery2_Impl,
    {
        unsafe extern "system" fn MakeDecision2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut core::ffi::c_void, ppexchange: *mut *mut core::ffi::c_void, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISCPSecureQuery2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureQuery2_Impl::MakeDecision2(
                this,
                core::mem::transmute_copy(&fuflags),
                core::mem::transmute_copy(&pdata),
                core::mem::transmute_copy(&dwsize),
                core::mem::transmute_copy(&dwappsec),
                core::mem::transmute_copy(&pbspsessionkey),
                core::mem::transmute_copy(&dwsessionkeylen),
                windows_core::from_raw_borrowed(&pstorageglobals),
                core::mem::transmute_copy(&pappcertapp),
                core::mem::transmute_copy(&dwappcertapplen),
                core::mem::transmute_copy(&pappcertsp),
                core::mem::transmute_copy(&dwappcertsplen),
                core::mem::transmute_copy(&pszrevocationurl),
                core::mem::transmute_copy(&pdwrevocationurllen),
                core::mem::transmute_copy(&pdwrevocationbitflag),
                core::mem::transmute_copy(&pqwfilesize),
                windows_core::from_raw_borrowed(&punknown),
                core::mem::transmute_copy(&ppexchange),
                core::mem::transmute_copy(&abmac),
            )
            .into()
        }
        Self { base__: ISCPSecureQuery_Vtbl::new::<Identity, OFFSET>(), MakeDecision2: MakeDecision2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureQuery2 as windows_core::Interface>::IID || iid == &<ISCPSecureQuery as windows_core::Interface>::IID
    }
}
pub trait ISCPSecureQuery3_Impl: Sized + ISCPSecureQuery2_Impl {
    fn GetRightsOnClearChannel(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: Option<&IMDSPStorageGlobals>, pprogresscallback: Option<&IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>;
    fn MakeDecisionOnClearChannel(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: Option<&IMDSPStorageGlobals>, pprogresscallback: Option<&IWMDMProgress3>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: Option<&windows_core::IUnknown>, ppexchange: *mut Option<ISCPSecureExchange>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISCPSecureQuery3 {}
impl ISCPSecureQuery3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSecureQuery3_Vtbl
    where
        Identity: ISCPSecureQuery3_Impl,
    {
        unsafe extern "system" fn GetRightsOnClearChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut core::ffi::c_void, pprogresscallback: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISCPSecureQuery3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureQuery3_Impl::GetRightsOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pbspsessionkey), core::mem::transmute_copy(&dwsessionkeylen), windows_core::from_raw_borrowed(&pstgglobals), windows_core::from_raw_borrowed(&pprogresscallback), core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount)).into()
        }
        unsafe extern "system" fn MakeDecisionOnClearChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut core::ffi::c_void, pprogresscallback: *mut core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut core::ffi::c_void, ppexchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCPSecureQuery3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSecureQuery3_Impl::MakeDecisionOnClearChannel(
                this,
                core::mem::transmute_copy(&fuflags),
                core::mem::transmute_copy(&pdata),
                core::mem::transmute_copy(&dwsize),
                core::mem::transmute_copy(&dwappsec),
                core::mem::transmute_copy(&pbspsessionkey),
                core::mem::transmute_copy(&dwsessionkeylen),
                windows_core::from_raw_borrowed(&pstorageglobals),
                windows_core::from_raw_borrowed(&pprogresscallback),
                core::mem::transmute_copy(&pappcertapp),
                core::mem::transmute_copy(&dwappcertapplen),
                core::mem::transmute_copy(&pappcertsp),
                core::mem::transmute_copy(&dwappcertsplen),
                core::mem::transmute_copy(&pszrevocationurl),
                core::mem::transmute_copy(&pdwrevocationurllen),
                core::mem::transmute_copy(&pdwrevocationbitflag),
                core::mem::transmute_copy(&pqwfilesize),
                windows_core::from_raw_borrowed(&punknown),
                core::mem::transmute_copy(&ppexchange),
            )
            .into()
        }
        Self {
            base__: ISCPSecureQuery2_Vtbl::new::<Identity, OFFSET>(),
            GetRightsOnClearChannel: GetRightsOnClearChannel::<Identity, OFFSET>,
            MakeDecisionOnClearChannel: MakeDecisionOnClearChannel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureQuery3 as windows_core::Interface>::IID || iid == &<ISCPSecureQuery as windows_core::Interface>::IID || iid == &<ISCPSecureQuery2 as windows_core::Interface>::IID
    }
}
pub trait ISCPSession_Impl: Sized {
    fn BeginSession(&self, pidevice: Option<&IMDSPDevice>, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
    fn EndSession(&self, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
    fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery>;
}
impl windows_core::RuntimeName for ISCPSession {}
impl ISCPSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCPSession_Vtbl
    where
        Identity: ISCPSession_Impl,
    {
        unsafe extern "system" fn BeginSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidevice: *mut core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT
        where
            Identity: ISCPSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSession_Impl::BeginSession(this, windows_core::from_raw_borrowed(&pidevice), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn EndSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT
        where
            Identity: ISCPSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCPSession_Impl::EndSession(this, core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn GetSecureQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCPSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCPSession_Impl::GetSecureQuery(this) {
                Ok(ok__) => {
                    ppsecurequery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginSession: BeginSession::<Identity, OFFSET>,
            EndSession: EndSession::<Identity, OFFSET>,
            GetSecureQuery: GetSecureQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDevice_Impl: Sized {
    fn GetName(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetManufacturer(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetVersion(&self) -> windows_core::Result<u32>;
    fn GetType(&self) -> windows_core::Result<u32>;
    fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn GetDeviceIcon(&self) -> windows_core::Result<u32>;
    fn EnumStorage(&self) -> windows_core::Result<IWMDMEnumStorage>;
    fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::Result<()>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMDevice {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMDevice_Vtbl
    where
        Identity: IWMDMDevice_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetManufacturer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice_Impl::GetManufacturer(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice_Impl::GetVersion(this) {
                Ok(ok__) => {
                    pdwversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice_Impl::GetType(this) {
                Ok(ok__) => {
                    pdwtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnumber), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetPowerSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice_Impl::GetPowerSource(this, core::mem::transmute_copy(&pdwpowersource), core::mem::transmute_copy(&pdwpercentremaining)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice_Impl::GetDeviceIcon(this) {
                Ok(ok__) => {
                    hicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice_Impl::EnumStorage(this) {
                Ok(ok__) => {
                    ppenumstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice_Impl::GetFormatSupport(this, core::mem::transmute_copy(&ppformatex), core::mem::transmute_copy(&pnformatcount), core::mem::transmute_copy(&pppwszmimetype), core::mem::transmute_copy(&pnmimetypecount)).into()
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetManufacturer: GetManufacturer::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, OFFSET>,
            GetPowerSource: GetPowerSource::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, OFFSET>,
            EnumStorage: EnumStorage::<Identity, OFFSET>,
            GetFormatSupport: GetFormatSupport::<Identity, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice2_Impl: Sized + IWMDMDevice_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
    fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()>;
    fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()>;
    fn GetCanonicalName(&self, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IWMDMDevice2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IWMDMDevice2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMDevice2_Vtbl
    where
        Identity: IWMDMDevice2_Impl,
    {
        unsafe extern "system" fn GetStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice2_Impl::GetFormatSupport2(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppaudioformatex), core::mem::transmute_copy(&pnaudioformatcount), core::mem::transmute_copy(&ppvideoformatex), core::mem::transmute_copy(&pnvideoformatcount), core::mem::transmute_copy(&ppfiletype), core::mem::transmute_copy(&pnfiletypecount)).into()
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppspecifyproppages: *mut *mut core::ffi::c_void, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice2_Impl::GetSpecifyPropertyPages(this, core::mem::transmute_copy(&ppspecifyproppages), core::mem::transmute_copy(&pppunknowns), core::mem::transmute_copy(&pcunks)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice2_Impl::GetCanonicalName(this, core::mem::transmute_copy(&pwszpnpname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        Self {
            base__: IWMDMDevice_Vtbl::new::<Identity, OFFSET>(),
            GetStorage: GetStorage::<Identity, OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Identity, OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Identity, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMDevice2 as windows_core::Interface>::IID || iid == &<IWMDMDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice3_Impl: Sized + IWMDMDevice2_Impl {
    fn GetProperty(&self, pwszpropname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetProperty(&self, pwszpropname: &windows_core::PCWSTR, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IWMDMDevice3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IWMDMDevice3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMDevice3_Vtbl
    where
        Identity: IWMDMDevice3_Impl,
    {
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice3_Impl::GetProperty(this, core::mem::transmute(&pwszpropname)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice3_Impl::SetProperty(this, core::mem::transmute(&pwszpropname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetFormatCapability<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice3_Impl::GetFormatCapability(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    pformatsupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDevice3_Impl::DeviceIoControl(this, core::mem::transmute_copy(&dwiocontrolcode), core::mem::transmute_copy(&lpinbuffer), core::mem::transmute_copy(&ninbuffersize), core::mem::transmute_copy(&lpoutbuffer), core::mem::transmute_copy(&pnoutbuffersize)).into()
        }
        unsafe extern "system" fn FindStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDevice3_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMDMDevice2_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetFormatCapability: GetFormatCapability::<Identity, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, OFFSET>,
            FindStorage: FindStorage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMDevice3 as windows_core::Interface>::IID || iid == &<IWMDMDevice as windows_core::Interface>::IID || iid == &<IWMDMDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDeviceControl_Impl: Sized {
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn GetCapabilities(&self) -> windows_core::Result<u32>;
    fn Play(&self) -> windows_core::Result<()>;
    fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Seek(&self, fumode: u32, noffset: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMDeviceControl {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMDeviceControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMDeviceControl_Vtbl
    where
        Identity: IWMDMDeviceControl_Impl,
    {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDeviceControl_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMDeviceControl_Impl::GetCapabilities(this) {
                Ok(ok__) => {
                    pdwcapabilitiesmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceControl_Impl::Play(this).into()
        }
        unsafe extern "system" fn Record<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceControl_Impl::Record(this, core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceControl_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceControl_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceControl_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, noffset: i32) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceControl_Impl::Seek(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&noffset)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            Record: Record::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMDeviceControl as windows_core::Interface>::IID
    }
}
pub trait IWMDMDeviceSession_Impl: Sized {
    fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
    fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMDeviceSession {}
impl IWMDMDeviceSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMDeviceSession_Vtbl
    where
        Identity: IWMDMDeviceSession_Impl,
    {
        unsafe extern "system" fn BeginSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceSession_Impl::BeginSession(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn EndSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMDeviceSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMDeviceSession_Impl::EndSession(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginSession: BeginSession::<Identity, OFFSET>,
            EndSession: EndSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMDeviceSession as windows_core::Interface>::IID
    }
}
pub trait IWMDMEnumDevice_Impl: Sized {
    fn Next(&self, celt: u32, ppdevice: *mut Option<IWMDMDevice>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWMDMEnumDevice>;
}
impl windows_core::RuntimeName for IWMDMEnumDevice {}
impl IWMDMEnumDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMEnumDevice_Vtbl
    where
        Identity: IWMDMEnumDevice_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppdevice: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMEnumDevice_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppdevice), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMEnumDevice_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                Ok(ok__) => {
                    pceltfetched.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMEnumDevice_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMEnumDevice_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenumdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMEnumDevice as windows_core::Interface>::IID
    }
}
pub trait IWMDMEnumStorage_Impl: Sized {
    fn Next(&self, celt: u32, ppstorage: *mut Option<IWMDMStorage>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWMDMEnumStorage>;
}
impl windows_core::RuntimeName for IWMDMEnumStorage {}
impl IWMDMEnumStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMEnumStorage_Vtbl
    where
        Identity: IWMDMEnumStorage_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppstorage: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMEnumStorage_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppstorage), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMEnumStorage_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                Ok(ok__) => {
                    pceltfetched.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMEnumStorage_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMEnumStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMEnumStorage_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenumstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMEnumStorage as windows_core::Interface>::IID
    }
}
pub trait IWMDMLogger_Impl: Sized {
    fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Enable(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLogFileName(&self, pszfilename: windows_core::PSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn SetLogFileName(&self, pszfilename: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn LogString(&self, dwflags: u32, pszsrcname: &windows_core::PCSTR, pszlog: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn LogDword(&self, dwflags: u32, pszsrcname: &windows_core::PCSTR, pszlogformat: &windows_core::PCSTR, dwlog: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> windows_core::Result<()>;
    fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMLogger {}
impl IWMDMLogger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMLogger_Vtbl
    where
        Identity: IWMDMLogger_Impl,
    {
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMLogger_Impl::IsEnabled(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::Enable(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetLogFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::GetLogFileName(this, core::mem::transmute_copy(&pszfilename), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn SetLogFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::SetLogFileName(this, core::mem::transmute(&pszfilename)).into()
        }
        unsafe extern "system" fn LogString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszsrcname: windows_core::PCSTR, pszlog: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::LogString(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszsrcname), core::mem::transmute(&pszlog)).into()
        }
        unsafe extern "system" fn LogDword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszsrcname: windows_core::PCSTR, pszlogformat: windows_core::PCSTR, dwlog: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::LogDword(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszsrcname), core::mem::transmute(&pszlogformat), core::mem::transmute_copy(&dwlog)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::Reset(this).into()
        }
        unsafe extern "system" fn GetSizeParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::GetSizeParams(this, core::mem::transmute_copy(&pdwmaxsize), core::mem::transmute_copy(&pdwshrinktosize)).into()
        }
        unsafe extern "system" fn SetSizeParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMLogger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMLogger_Impl::SetSizeParams(this, core::mem::transmute_copy(&dwmaxsize), core::mem::transmute_copy(&dwshrinktosize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            GetLogFileName: GetLogFileName::<Identity, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, OFFSET>,
            LogString: LogString::<Identity, OFFSET>,
            LogDword: LogDword::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetSizeParams: GetSizeParams::<Identity, OFFSET>,
            SetSizeParams: SetSizeParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMLogger as windows_core::Interface>::IID
    }
}
pub trait IWMDMMetaData_Impl: Sized {
    fn AddItem(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: &windows_core::PCWSTR, pvalue: *const u8, ilength: u32) -> windows_core::Result<()>;
    fn QueryByName(&self, pwsztagname: &windows_core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>;
    fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>;
    fn GetItemCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWMDMMetaData {}
impl IWMDMMetaData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMMetaData_Vtbl
    where
        Identity: IWMDMMetaData_Impl,
    {
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: windows_core::PCWSTR, pvalue: *const u8, ilength: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMMetaData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMMetaData_Impl::AddItem(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&pwsztagname), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&ilength)).into()
        }
        unsafe extern "system" fn QueryByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztagname: windows_core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMMetaData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMMetaData_Impl::QueryByName(this, core::mem::transmute(&pwsztagname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn QueryByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMMetaData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMMetaData_Impl::QueryByIndex(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&ppwszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&ppvalue), core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMMetaData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMMetaData_Impl::GetItemCount(this) {
                Ok(ok__) => {
                    icount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddItem: AddItem::<Identity, OFFSET>,
            QueryByName: QueryByName::<Identity, OFFSET>,
            QueryByIndex: QueryByIndex::<Identity, OFFSET>,
            GetItemCount: GetItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMMetaData as windows_core::Interface>::IID
    }
}
pub trait IWMDMNotification_Impl: Sized {
    fn WMDMMessage(&self, dwmessagetype: u32, pwszcanonicalname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMNotification {}
impl IWMDMNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMNotification_Vtbl
    where
        Identity: IWMDMNotification_Impl,
    {
        unsafe extern "system" fn WMDMMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IWMDMNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMNotification_Impl::WMDMMessage(this, core::mem::transmute_copy(&dwmessagetype), core::mem::transmute(&pwszcanonicalname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), WMDMMessage: WMDMMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMNotification as windows_core::Interface>::IID
    }
}
pub trait IWMDMObjectInfo_Impl: Sized {
    fn GetPlayLength(&self) -> windows_core::Result<u32>;
    fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()>;
    fn GetPlayOffset(&self) -> windows_core::Result<u32>;
    fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()>;
    fn GetTotalLength(&self) -> windows_core::Result<u32>;
    fn GetLastPlayPosition(&self) -> windows_core::Result<u32>;
    fn GetLongestPlayPosition(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWMDMObjectInfo {}
impl IWMDMObjectInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMObjectInfo_Vtbl
    where
        Identity: IWMDMObjectInfo_Impl,
    {
        unsafe extern "system" fn GetPlayLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMObjectInfo_Impl::GetPlayLength(this) {
                Ok(ok__) => {
                    pdwlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlength: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMObjectInfo_Impl::SetPlayLength(this, core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetPlayOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoffset: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMObjectInfo_Impl::GetPlayOffset(this) {
                Ok(ok__) => {
                    pdwoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMObjectInfo_Impl::SetPlayOffset(this, core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn GetTotalLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMObjectInfo_Impl::GetTotalLength(this) {
                Ok(ok__) => {
                    pdwlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastpos: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMObjectInfo_Impl::GetLastPlayPosition(this) {
                Ok(ok__) => {
                    pdwlastpos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlongestpos: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMObjectInfo_Impl::GetLongestPlayPosition(this) {
                Ok(ok__) => {
                    pdwlongestpos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPlayLength: GetPlayLength::<Identity, OFFSET>,
            SetPlayLength: SetPlayLength::<Identity, OFFSET>,
            GetPlayOffset: GetPlayOffset::<Identity, OFFSET>,
            SetPlayOffset: SetPlayOffset::<Identity, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Identity, OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMObjectInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation_Impl: Sized {
    fn BeginRead(&self) -> windows_core::Result<()>;
    fn BeginWrite(&self) -> windows_core::Result<()>;
    fn GetObjectName(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn SetObjectName(&self, pwszname: &windows_core::PCWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()>;
    fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> windows_core::Result<()>;
    fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn End(&self, phcompletioncode: *const windows_core::HRESULT, pnewobject: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMOperation {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMOperation_Vtbl
    where
        Identity: IWMDMOperation_Impl,
    {
        unsafe extern "system" fn BeginRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::BeginRead(this).into()
        }
        unsafe extern "system" fn BeginWrite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::BeginWrite(this).into()
        }
        unsafe extern "system" fn GetObjectName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::GetObjectName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn SetObjectName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::SetObjectName(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::GetObjectAttributes(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::SetObjectAttributes(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetObjectTotalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::GetObjectTotalSize(this, core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn SetObjectTotalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::SetObjectTotalSize(this, core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwsizehigh)).into()
        }
        unsafe extern "system" fn TransferObjectData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::TransferObjectData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn End<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phcompletioncode: *const windows_core::HRESULT, pnewobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation_Impl::End(this, core::mem::transmute_copy(&phcompletioncode), windows_core::from_raw_borrowed(&pnewobject)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginRead: BeginRead::<Identity, OFFSET>,
            BeginWrite: BeginWrite::<Identity, OFFSET>,
            GetObjectName: GetObjectName::<Identity, OFFSET>,
            SetObjectName: SetObjectName::<Identity, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, OFFSET>,
            GetObjectTotalSize: GetObjectTotalSize::<Identity, OFFSET>,
            SetObjectTotalSize: SetObjectTotalSize::<Identity, OFFSET>,
            TransferObjectData: TransferObjectData::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMOperation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMOperation2_Impl: Sized + IWMDMOperation_Impl {
    fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
    fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IWMDMOperation2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMOperation2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMOperation2_Vtbl
    where
        Identity: IWMDMOperation2_Impl,
    {
        unsafe extern "system" fn SetObjectAttributes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation2_Impl::SetObjectAttributes2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetObjectAttributes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation2_Impl::GetObjectAttributes2(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pdwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base__: IWMDMOperation_Vtbl::new::<Identity, OFFSET>(),
            SetObjectAttributes2: SetObjectAttributes2::<Identity, OFFSET>,
            GetObjectAttributes2: GetObjectAttributes2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMOperation2 as windows_core::Interface>::IID || iid == &<IWMDMOperation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation3_Impl: Sized + IWMDMOperation_Impl {
    fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMOperation3 {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMOperation3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMOperation3_Vtbl
    where
        Identity: IWMDMOperation3_Impl,
    {
        unsafe extern "system" fn TransferObjectDataOnClearChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMOperation3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMOperation3_Impl::TransferObjectDataOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base__: IWMDMOperation_Vtbl::new::<Identity, OFFSET>(), TransferObjectDataOnClearChannel: TransferObjectDataOnClearChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMOperation3 as windows_core::Interface>::IID || iid == &<IWMDMOperation as windows_core::Interface>::IID
    }
}
pub trait IWMDMProgress_Impl: Sized {
    fn Begin(&self, dwestimatedticks: u32) -> windows_core::Result<()>;
    fn Progress(&self, dwtranspiredticks: u32) -> windows_core::Result<()>;
    fn End(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMProgress {}
impl IWMDMProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMProgress_Vtbl
    where
        Identity: IWMDMProgress_Impl,
    {
        unsafe extern "system" fn Begin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwestimatedticks: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMProgress_Impl::Begin(this, core::mem::transmute_copy(&dwestimatedticks)).into()
        }
        unsafe extern "system" fn Progress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtranspiredticks: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMProgress_Impl::Progress(this, core::mem::transmute_copy(&dwtranspiredticks)).into()
        }
        unsafe extern "system" fn End<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMProgress_Impl::End(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, OFFSET>,
            Progress: Progress::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMProgress as windows_core::Interface>::IID
    }
}
pub trait IWMDMProgress2_Impl: Sized + IWMDMProgress_Impl {
    fn End2(&self, hrcompletioncode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMProgress2 {}
impl IWMDMProgress2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMProgress2_Vtbl
    where
        Identity: IWMDMProgress2_Impl,
    {
        unsafe extern "system" fn End2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrcompletioncode: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMDMProgress2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMProgress2_Impl::End2(this, core::mem::transmute_copy(&hrcompletioncode)).into()
        }
        Self { base__: IWMDMProgress_Vtbl::new::<Identity, OFFSET>(), End2: End2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMProgress2 as windows_core::Interface>::IID || iid == &<IWMDMProgress as windows_core::Interface>::IID
    }
}
pub trait IWMDMProgress3_Impl: Sized + IWMDMProgress2_Impl {
    fn Begin3(&self, eventid: &windows_core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
    fn Progress3(&self, eventid: &windows_core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
    fn End3(&self, eventid: &windows_core::GUID, hrcompletioncode: windows_core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMProgress3 {}
impl IWMDMProgress3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMProgress3_Vtbl
    where
        Identity: IWMDMProgress3_Impl,
    {
        unsafe extern "system" fn Begin3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::HRESULT
        where
            Identity: IWMDMProgress3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMProgress3_Impl::Begin3(this, core::mem::transmute(&eventid), core::mem::transmute_copy(&dwestimatedticks), core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Progress3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::HRESULT
        where
            Identity: IWMDMProgress3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMProgress3_Impl::Progress3(this, core::mem::transmute(&eventid), core::mem::transmute_copy(&dwtranspiredticks), core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn End3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID, hrcompletioncode: windows_core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> windows_core::HRESULT
        where
            Identity: IWMDMProgress3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMProgress3_Impl::End3(this, core::mem::transmute(&eventid), core::mem::transmute_copy(&hrcompletioncode), core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: IWMDMProgress2_Vtbl::new::<Identity, OFFSET>(),
            Begin3: Begin3::<Identity, OFFSET>,
            Progress3: Progress3::<Identity, OFFSET>,
            End3: End3::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMProgress3 as windows_core::Interface>::IID || iid == &<IWMDMProgress as windows_core::Interface>::IID || iid == &<IWMDMProgress2 as windows_core::Interface>::IID
    }
}
pub trait IWMDMRevoked_Impl: Sized {
    fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMRevoked {}
impl IWMDMRevoked_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMRevoked_Vtbl
    where
        Identity: IWMDMRevoked_Impl,
    {
        unsafe extern "system" fn GetRevocationURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMRevoked_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMRevoked_Impl::GetRevocationURL(this, core::mem::transmute_copy(&ppwszrevocationurl), core::mem::transmute_copy(&pdwbufferlen), core::mem::transmute_copy(&pdwrevokedbitflag)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRevocationURL: GetRevocationURL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMRevoked as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMStorage_Impl: Sized {
    fn SetAttributes(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetStorageGlobals(&self) -> windows_core::Result<IWMDMStorageGlobals>;
    fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetName(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetDate(&self) -> windows_core::Result<WMDMDATETIME>;
    fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()>;
    fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn EnumStorage(&self) -> windows_core::Result<IWMDMEnumStorage>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMStorage {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMStorage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorage_Vtbl
    where
        Identity: IWMDMStorage_Impl,
    {
        unsafe extern "system" fn SetAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage_Impl::SetAttributes(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorageglobals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage_Impl::GetStorageGlobals(this) {
                Ok(ok__) => {
                    ppstorageglobals.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage_Impl::GetAttributes(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage_Impl::GetDate(this) {
                Ok(ok__) => {
                    pdatetimeutc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage_Impl::GetSize(this, core::mem::transmute_copy(&pdwsizelow), core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn GetRights<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage_Impl::GetRights(this, core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn EnumStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage_Impl::EnumStorage(this) {
                Ok(ok__) => {
                    penumstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAttributes: SetAttributes::<Identity, OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetDate: GetDate::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetRights: GetRights::<Identity, OFFSET>,
            EnumStorage: EnumStorage::<Identity, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage2_Impl: Sized + IWMDMStorage_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
    fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
    fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IWMDMStorage2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorage2_Vtbl
    where
        Identity: IWMDMStorage2_Impl,
    {
        unsafe extern "system" fn GetStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage2_Impl::SetAttributes2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetAttributes2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage2_Impl::GetAttributes2(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pdwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base__: IWMDMStorage_Vtbl::new::<Identity, OFFSET>(),
            GetStorage: GetStorage::<Identity, OFFSET>,
            SetAttributes2: SetAttributes2::<Identity, OFFSET>,
            GetAttributes2: GetAttributes2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorage2 as windows_core::Interface>::IID || iid == &<IWMDMStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage3_Impl: Sized + IWMDMStorage2_Impl {
    fn GetMetadata(&self) -> windows_core::Result<IWMDMMetaData>;
    fn SetMetadata(&self, pmetadata: Option<&IWMDMMetaData>) -> windows_core::Result<()>;
    fn CreateEmptyMetadataObject(&self) -> windows_core::Result<IWMDMMetaData>;
    fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IWMDMStorage3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorage3_Vtbl
    where
        Identity: IWMDMStorage3_Impl,
    {
        unsafe extern "system" fn GetMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage3_Impl::GetMetadata(this) {
                Ok(ok__) => {
                    ppmetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage3_Impl::SetMetadata(this, windows_core::from_raw_borrowed(&pmetadata)).into()
        }
        unsafe extern "system" fn CreateEmptyMetadataObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage3_Impl::CreateEmptyMetadataObject(this) {
                Ok(ok__) => {
                    ppmetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumPreference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage3_Impl::SetEnumPreference(this, core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&nviews), core::mem::transmute_copy(&pviews)).into()
        }
        Self {
            base__: IWMDMStorage2_Vtbl::new::<Identity, OFFSET>(),
            GetMetadata: GetMetadata::<Identity, OFFSET>,
            SetMetadata: SetMetadata::<Identity, OFFSET>,
            CreateEmptyMetadataObject: CreateEmptyMetadataObject::<Identity, OFFSET>,
            SetEnumPreference: SetEnumPreference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorage3 as windows_core::Interface>::IID || iid == &<IWMDMStorage as windows_core::Interface>::IID || iid == &<IWMDMStorage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage4_Impl: Sized + IWMDMStorage3_Impl {
    fn SetReferences(&self, dwrefs: u32, ppiwmdmstorage: *const Option<IWMDMStorage>) -> windows_core::Result<()>;
    fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut Option<IWMDMStorage>) -> windows_core::Result<()>;
    fn GetRightsWithProgress(&self, piprogresscallback: Option<&IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>;
    fn GetSpecifiedMetadata(&self, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR) -> windows_core::Result<IWMDMMetaData>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
    fn GetParent(&self) -> windows_core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IWMDMStorage4 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorage4_Vtbl
    where
        Identity: IWMDMStorage4_Impl,
    {
        unsafe extern "system" fn SetReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage4_Impl::SetReferences(this, core::mem::transmute_copy(&dwrefs), core::mem::transmute_copy(&ppiwmdmstorage)).into()
        }
        unsafe extern "system" fn GetReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut Option<IWMDMStorage>) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage4_Impl::GetReferences(this, core::mem::transmute_copy(&pdwrefs), core::mem::transmute_copy(&pppiwmdmstorage)).into()
        }
        unsafe extern "system" fn GetRightsWithProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piprogresscallback: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorage4_Impl::GetRightsWithProgress(this, windows_core::from_raw_borrowed(&piprogresscallback), core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount)).into()
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage4_Impl::GetSpecifiedMetadata(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppwszpropnames)) {
                Ok(ok__) => {
                    ppmetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage4_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorage4_Impl::GetParent(this) {
                Ok(ok__) => {
                    ppstorage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMDMStorage3_Vtbl::new::<Identity, OFFSET>(),
            SetReferences: SetReferences::<Identity, OFFSET>,
            GetReferences: GetReferences::<Identity, OFFSET>,
            GetRightsWithProgress: GetRightsWithProgress::<Identity, OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Identity, OFFSET>,
            FindStorage: FindStorage::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorage4 as windows_core::Interface>::IID || iid == &<IWMDMStorage as windows_core::Interface>::IID || iid == &<IWMDMStorage2 as windows_core::Interface>::IID || iid == &<IWMDMStorage3 as windows_core::Interface>::IID
    }
}
pub trait IWMDMStorageControl_Impl: Sized {
    fn Insert(&self, fumode: u32, pwszfile: &windows_core::PCWSTR, poperation: Option<&IWMDMOperation>, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<IWMDMStorage>;
    fn Delete(&self, fumode: u32, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<()>;
    fn Rename(&self, fumode: u32, pwsznewname: &windows_core::PCWSTR, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<()>;
    fn Read(&self, fumode: u32, pwszfile: &windows_core::PCWSTR, pprogress: Option<&IWMDMProgress>, poperation: Option<&IWMDMOperation>) -> windows_core::Result<()>;
    fn Move(&self, fumode: u32, ptargetobject: Option<&IWMDMStorage>, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMStorageControl {}
impl IWMDMStorageControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorageControl_Vtbl
    where
        Identity: IWMDMStorageControl_Impl,
    {
        unsafe extern "system" fn Insert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwszfile: windows_core::PCWSTR, poperation: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorageControl_Impl::Insert(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwszfile), windows_core::from_raw_borrowed(&poperation), windows_core::from_raw_borrowed(&pprogress)) {
                Ok(ok__) => {
                    ppnewobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageControl_Impl::Delete(this, core::mem::transmute_copy(&fumode), windows_core::from_raw_borrowed(&pprogress)).into()
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwsznewname: windows_core::PCWSTR, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageControl_Impl::Rename(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwsznewname), windows_core::from_raw_borrowed(&pprogress)).into()
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwszfile: windows_core::PCWSTR, pprogress: *mut core::ffi::c_void, poperation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageControl_Impl::Read(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwszfile), windows_core::from_raw_borrowed(&pprogress), windows_core::from_raw_borrowed(&poperation)).into()
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, ptargetobject: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageControl_Impl::Move(this, core::mem::transmute_copy(&fumode), windows_core::from_raw_borrowed(&ptargetobject), windows_core::from_raw_borrowed(&pprogress)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Insert: Insert::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorageControl as windows_core::Interface>::IID
    }
}
pub trait IWMDMStorageControl2_Impl: Sized + IWMDMStorageControl_Impl {
    fn Insert2(&self, fumode: u32, pwszfilesource: &windows_core::PCWSTR, pwszfiledest: &windows_core::PCWSTR, poperation: Option<&IWMDMOperation>, pprogress: Option<&IWMDMProgress>, punknown: Option<&windows_core::IUnknown>, ppnewobject: *mut Option<IWMDMStorage>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMStorageControl2 {}
impl IWMDMStorageControl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorageControl2_Vtbl
    where
        Identity: IWMDMStorageControl2_Impl,
    {
        unsafe extern "system" fn Insert2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwszfilesource: windows_core::PCWSTR, pwszfiledest: windows_core::PCWSTR, poperation: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void, punknown: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageControl2_Impl::Insert2(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwszfilesource), core::mem::transmute(&pwszfiledest), windows_core::from_raw_borrowed(&poperation), windows_core::from_raw_borrowed(&pprogress), windows_core::from_raw_borrowed(&punknown), core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base__: IWMDMStorageControl_Vtbl::new::<Identity, OFFSET>(), Insert2: Insert2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorageControl2 as windows_core::Interface>::IID || iid == &<IWMDMStorageControl as windows_core::Interface>::IID
    }
}
pub trait IWMDMStorageControl3_Impl: Sized + IWMDMStorageControl2_Impl {
    fn Insert3(&self, fumode: u32, futype: u32, pwszfilesource: &windows_core::PCWSTR, pwszfiledest: &windows_core::PCWSTR, poperation: Option<&IWMDMOperation>, pprogress: Option<&IWMDMProgress>, pmetadata: Option<&IWMDMMetaData>, punknown: Option<&windows_core::IUnknown>, ppnewobject: *mut Option<IWMDMStorage>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMStorageControl3 {}
impl IWMDMStorageControl3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorageControl3_Vtbl
    where
        Identity: IWMDMStorageControl3_Impl,
    {
        unsafe extern "system" fn Insert3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: windows_core::PCWSTR, pwszfiledest: windows_core::PCWSTR, poperation: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void, punknown: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageControl3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageControl3_Impl::Insert3(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&futype), core::mem::transmute(&pwszfilesource), core::mem::transmute(&pwszfiledest), windows_core::from_raw_borrowed(&poperation), windows_core::from_raw_borrowed(&pprogress), windows_core::from_raw_borrowed(&pmetadata), windows_core::from_raw_borrowed(&punknown), core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base__: IWMDMStorageControl2_Vtbl::new::<Identity, OFFSET>(), Insert3: Insert3::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorageControl3 as windows_core::Interface>::IID || iid == &<IWMDMStorageControl as windows_core::Interface>::IID || iid == &<IWMDMStorageControl2 as windows_core::Interface>::IID
    }
}
pub trait IWMDMStorageGlobals_Impl: Sized {
    fn GetCapabilities(&self) -> windows_core::Result<u32>;
    fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, fumode: u32, pprogress: Option<&IWMDMProgress>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDMStorageGlobals {}
impl IWMDMStorageGlobals_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDMStorageGlobals_Vtbl
    where
        Identity: IWMDMStorageGlobals_Impl,
    {
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorageGlobals_Impl::GetCapabilities(this) {
                Ok(ok__) => {
                    pdwcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageGlobals_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnum), core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetTotalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageGlobals_Impl::GetTotalSize(this, core::mem::transmute_copy(&pdwtotalsizelow), core::mem::transmute_copy(&pdwtotalsizehigh)).into()
        }
        unsafe extern "system" fn GetTotalFree<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageGlobals_Impl::GetTotalFree(this, core::mem::transmute_copy(&pdwfreelow), core::mem::transmute_copy(&pdwfreehigh)).into()
        }
        unsafe extern "system" fn GetTotalBad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageGlobals_Impl::GetTotalBad(this, core::mem::transmute_copy(&pdwbadlow), core::mem::transmute_copy(&pdwbadhigh)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDMStorageGlobals_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDMStorageGlobals_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDMStorageGlobals_Impl::Initialize(this, core::mem::transmute_copy(&fumode), windows_core::from_raw_borrowed(&pprogress)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, OFFSET>,
            GetTotalSize: GetTotalSize::<Identity, OFFSET>,
            GetTotalFree: GetTotalFree::<Identity, OFFSET>,
            GetTotalBad: GetTotalBad::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorageGlobals as windows_core::Interface>::IID
    }
}
pub trait IWMDeviceManager_Impl: Sized {
    fn GetRevision(&self) -> windows_core::Result<u32>;
    fn GetDeviceCount(&self) -> windows_core::Result<u32>;
    fn EnumDevices(&self) -> windows_core::Result<IWMDMEnumDevice>;
}
impl windows_core::RuntimeName for IWMDeviceManager {}
impl IWMDeviceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDeviceManager_Vtbl
    where
        Identity: IWMDeviceManager_Impl,
    {
        unsafe extern "system" fn GetRevision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrevision: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceManager_Impl::GetRevision(this) {
                Ok(ok__) => {
                    pdwrevision.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceManager_Impl::GetDeviceCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceManager_Impl::EnumDevices(this) {
                Ok(ok__) => {
                    ppenumdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRevision: GetRevision::<Identity, OFFSET>,
            GetDeviceCount: GetDeviceCount::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDeviceManager as windows_core::Interface>::IID
    }
}
pub trait IWMDeviceManager2_Impl: Sized + IWMDeviceManager_Impl {
    fn GetDeviceFromCanonicalName(&self, pwszcanonicalname: &windows_core::PCWSTR) -> windows_core::Result<IWMDMDevice>;
    fn EnumDevices2(&self) -> windows_core::Result<IWMDMEnumDevice>;
    fn Reinitialize(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDeviceManager2 {}
impl IWMDeviceManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDeviceManager2_Vtbl
    where
        Identity: IWMDeviceManager2_Impl,
    {
        unsafe extern "system" fn GetDeviceFromCanonicalName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcanonicalname: windows_core::PCWSTR, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceManager2_Impl::GetDeviceFromCanonicalName(this, core::mem::transmute(&pwszcanonicalname)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMDeviceManager2_Impl::EnumDevices2(this) {
                Ok(ok__) => {
                    ppenumdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reinitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMDeviceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDeviceManager2_Impl::Reinitialize(this).into()
        }
        Self {
            base__: IWMDeviceManager_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceFromCanonicalName: GetDeviceFromCanonicalName::<Identity, OFFSET>,
            EnumDevices2: EnumDevices2::<Identity, OFFSET>,
            Reinitialize: Reinitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDeviceManager2 as windows_core::Interface>::IID || iid == &<IWMDeviceManager as windows_core::Interface>::IID
    }
}
pub trait IWMDeviceManager3_Impl: Sized + IWMDeviceManager2_Impl {
    fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMDeviceManager3 {}
impl IWMDeviceManager3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMDeviceManager3_Vtbl
    where
        Identity: IWMDeviceManager3_Impl,
    {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenumpref: u32) -> windows_core::HRESULT
        where
            Identity: IWMDeviceManager3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMDeviceManager3_Impl::SetDeviceEnumPreference(this, core::mem::transmute_copy(&dwenumpref)).into()
        }
        Self { base__: IWMDeviceManager2_Vtbl::new::<Identity, OFFSET>(), SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDeviceManager3 as windows_core::Interface>::IID || iid == &<IWMDeviceManager as windows_core::Interface>::IID || iid == &<IWMDeviceManager2 as windows_core::Interface>::IID
    }
}
