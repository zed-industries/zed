pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const ENUM_MODE_METADATA_VIEWS: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(2i32);
pub const ENUM_MODE_RAW: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(0i32);
pub const ENUM_MODE_USE_DEVICE_PREF: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(1i32);
pub const EVENT_WMDM_CONTENT_TRANSFER: windows_core::GUID = windows_core::GUID::from_u128(0x339c9bf4_bcfe_4ed8_94df_eaf8c26ab61b);
windows_core::imp::define_interface!(IComponentAuthenticate, IComponentAuthenticate_Vtbl, 0xa9889c00_6d2b_11d3_8496_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IComponentAuthenticate, windows_core::IUnknown);
impl IComponentAuthenticate {
    pub unsafe fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: &[u8], ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SACAuth)(windows_core::Interface::as_raw(self), dwprotocolid, dwpass, core::mem::transmute(pbdatain.as_ptr()), pbdatain.len().try_into().unwrap(), ppbdataout as _, pdwdataoutlen as _).ok() }
    }
    pub unsafe fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SACGetProtocols)(windows_core::Interface::as_raw(self), ppdwprotocols as _, pdwprotocolcount as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentAuthenticate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SACAuth: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SACGetProtocols: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IComponentAuthenticate_Impl: windows_core::IUnknownImpl {
    fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> windows_core::Result<()>;
    fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> windows_core::Result<()>;
}
impl IComponentAuthenticate_Vtbl {
    pub const fn new<Identity: IComponentAuthenticate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SACAuth<Identity: IComponentAuthenticate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentAuthenticate_Impl::SACAuth(this, core::mem::transmute_copy(&dwprotocolid), core::mem::transmute_copy(&dwpass), core::mem::transmute_copy(&pbdatain), core::mem::transmute_copy(&dwdatainlen), core::mem::transmute_copy(&ppbdataout), core::mem::transmute_copy(&pdwdataoutlen)).into()
            }
        }
        unsafe extern "system" fn SACGetProtocols<Identity: IComponentAuthenticate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentAuthenticate_Impl::SACGetProtocols(this, core::mem::transmute_copy(&ppdwprotocols), core::mem::transmute_copy(&pdwprotocolcount)).into()
            }
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
impl windows_core::RuntimeName for IComponentAuthenticate {}
windows_core::imp::define_interface!(IMDSPDevice, IMDSPDevice_Vtbl, 0x1dcb3a12_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPDevice, windows_core::IUnknown);
impl IMDSPDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetManufacturer)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnumber as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPowerSource)(windows_core::Interface::as_raw(self), pdwpowersource as _, pdwpercentremaining as _).ok() }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceIcon(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceIcon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IMDSPEnumStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFormatSupport)(windows_core::Interface::as_raw(self), pformatex as _, pnformatcount as _, pppwszmimetype as _, pnmimetypecount as _).ok() }
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDMID, *mut u8) -> windows_core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Audio::WAVEFORMATEX, *mut u32, *mut *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormatSupport: usize,
    pub SendOpaqueCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDevice_Impl: windows_core::IUnknownImpl {
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
impl IMDSPDevice_Vtbl {
    pub const fn new<Identity: IMDSPDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetManufacturer<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice_Impl::GetManufacturer(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetVersion<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        pdwversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice_Impl::GetType(this) {
                    Ok(ok__) => {
                        pdwtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnumber), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn GetPowerSource<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice_Impl::GetPowerSource(this, core::mem::transmute_copy(&pdwpowersource), core::mem::transmute_copy(&pdwpercentremaining)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice_Impl::GetDeviceIcon(this) {
                    Ok(ok__) => {
                        hicon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice_Impl::EnumStorage(this) {
                    Ok(ok__) => {
                        ppenumstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatSupport<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice_Impl::GetFormatSupport(this, core::mem::transmute_copy(&pformatex), core::mem::transmute_copy(&pnformatcount), core::mem::transmute_copy(&pppwszmimetype), core::mem::transmute_copy(&pnmimetypecount)).into()
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
            }
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
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IMDSPDevice {}
windows_core::imp::define_interface!(IMDSPDevice2, IMDSPDevice2_Vtbl, 0x420d16ad_c97d_4e00_82aa_00e9f4335ddd);
impl core::ops::Deref for IMDSPDevice2 {
    type Target = IMDSPDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPDevice2, windows_core::IUnknown, IMDSPDevice);
impl IMDSPDevice2 {
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFormatSupport2)(windows_core::Interface::as_raw(self), dwflags, ppaudioformatex as _, pnaudioformatcount as _, ppvideoformatex as _, pnvideoformatcount as _, ppfiletype as _, pnfiletypecount as _).ok() }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSpecifyPropertyPages)(windows_core::Interface::as_raw(self), core::mem::transmute(ppspecifyproppages), pppunknowns as _, pcunks as _).ok() }
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCanonicalName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice2_Vtbl {
    pub base__: IMDSPDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::Audio::WAVEFORMATEX, *mut u32, *mut *mut super::MediaFoundation::VIDEOINFOHEADER, *mut u32, *mut *mut WMFILECAPABILITIES, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice2_Impl: IMDSPDevice_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
    fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()>;
    fn GetSpecifyPropertyPages(&self, ppspecifyproppages: windows_core::OutRef<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()>;
    fn GetCanonicalName(&self, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IMDSPDevice2_Vtbl {
    pub const fn new<Identity: IMDSPDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStorage<Identity: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice2_Impl::GetFormatSupport2(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppaudioformatex), core::mem::transmute_copy(&pnaudioformatcount), core::mem::transmute_copy(&ppvideoformatex), core::mem::transmute_copy(&pnvideoformatcount), core::mem::transmute_copy(&ppfiletype), core::mem::transmute_copy(&pnfiletypecount)).into()
            }
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppspecifyproppages: *mut *mut core::ffi::c_void, pppunknowns: *mut *mut *mut core::ffi::c_void, pcunks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice2_Impl::GetSpecifyPropertyPages(this, core::mem::transmute_copy(&ppspecifyproppages), core::mem::transmute_copy(&pppunknowns), core::mem::transmute_copy(&pcunks)).into()
            }
        }
        unsafe extern "system" fn GetCanonicalName<Identity: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice2_Impl::GetCanonicalName(this, core::mem::transmute_copy(&pwszpnpname), core::mem::transmute_copy(&nmaxchars)).into()
            }
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
impl windows_core::RuntimeName for IMDSPDevice2 {}
windows_core::imp::define_interface!(IMDSPDevice3, IMDSPDevice3_Vtbl, 0x1a839845_fc55_487c_976f_ee38ac0e8c4e);
impl core::ops::Deref for IMDSPDevice3 {
    type Target = IMDSPDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPDevice3, windows_core::IUnknown, IMDSPDevice, IMDSPDevice2);
impl IMDSPDevice3 {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), core::mem::transmute(pvalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatCapability)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceIoControl)(windows_core::Interface::as_raw(self), dwiocontrolcode, core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len().try_into().unwrap(), lpoutbuffer as _, pnoutbuffersize as _).ok() }
    }
    pub unsafe fn FindStorage<P1>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P1) -> windows_core::Result<IMDSPStorage>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice3_Vtbl {
    pub base__: IMDSPDevice2_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetFormatCapability: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FORMATCODE, *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMDSPDevice3_Impl: IMDSPDevice2_Impl {
    fn GetProperty(&self, pwszpropname: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&self, pwszpropname: &windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMDSPDevice3_Vtbl {
    pub const fn new<Identity: IMDSPDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice3_Impl::GetProperty(this, core::mem::transmute(&pwszpropname)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice3_Impl::SetProperty(this, core::mem::transmute(&pwszpropname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetFormatCapability<Identity: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice3_Impl::GetFormatCapability(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pformatsupport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceIoControl<Identity: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDevice3_Impl::DeviceIoControl(this, core::mem::transmute_copy(&dwiocontrolcode), core::mem::transmute_copy(&lpinbuffer), core::mem::transmute_copy(&ninbuffersize), core::mem::transmute_copy(&lpoutbuffer), core::mem::transmute_copy(&pnoutbuffersize)).into()
            }
        }
        unsafe extern "system" fn FindStorage<Identity: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDevice3_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMDSPDevice3 {}
windows_core::imp::define_interface!(IMDSPDeviceControl, IMDSPDeviceControl_Vtbl, 0x1dcb3a14_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPDeviceControl, windows_core::IUnknown);
impl IMDSPDeviceControl {
    pub unsafe fn GetDCStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDCStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Play(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Record)(windows_core::Interface::as_raw(self), pformat).ok() }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), fumode, noffset).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDeviceControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDCStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub Record: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    Record: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDeviceControl_Impl: windows_core::IUnknownImpl {
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
impl IMDSPDeviceControl_Vtbl {
    pub const fn new<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDCStatus<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDeviceControl_Impl::GetDCStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDeviceControl_Impl::GetCapabilities(this) {
                    Ok(ok__) => {
                        pdwcapabilitiesmask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Play<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDeviceControl_Impl::Play(this).into()
            }
        }
        unsafe extern "system" fn Record<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDeviceControl_Impl::Record(this, core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDeviceControl_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDeviceControl_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDeviceControl_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, noffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPDeviceControl_Impl::Seek(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&noffset)).into()
            }
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
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IMDSPDeviceControl {}
windows_core::imp::define_interface!(IMDSPDirectTransfer, IMDSPDirectTransfer_Vtbl, 0xc2fe57a8_9304_478c_9ee4_47e397b912d7);
windows_core::imp::interface_hierarchy!(IMDSPDirectTransfer, windows_core::IUnknown);
impl IMDSPDirectTransfer {
    pub unsafe fn TransferToDevice<P0, P1, P3, P4, P5>(&self, pwszsourcefilepath: P0, psourceoperation: P1, fuflags: u32, pwszdestinationname: P3, psourcemetadata: P4, ptransferprogress: P5) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMOperation>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<IWMDMMetaData>,
        P5: windows_core::Param<IWMDMProgress>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransferToDevice)(windows_core::Interface::as_raw(self), pwszsourcefilepath.param().abi(), psourceoperation.param().abi(), fuflags, pwszdestinationname.param().abi(), psourcemetadata.param().abi(), ptransferprogress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDirectTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransferToDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMDSPDirectTransfer_Impl: windows_core::IUnknownImpl {
    fn TransferToDevice(&self, pwszsourcefilepath: &windows_core::PCWSTR, psourceoperation: windows_core::Ref<IWMDMOperation>, fuflags: u32, pwszdestinationname: &windows_core::PCWSTR, psourcemetadata: windows_core::Ref<IWMDMMetaData>, ptransferprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<IMDSPStorage>;
}
impl IMDSPDirectTransfer_Vtbl {
    pub const fn new<Identity: IMDSPDirectTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferToDevice<Identity: IMDSPDirectTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsourcefilepath: windows_core::PCWSTR, psourceoperation: *mut core::ffi::c_void, fuflags: u32, pwszdestinationname: windows_core::PCWSTR, psourcemetadata: *mut core::ffi::c_void, ptransferprogress: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPDirectTransfer_Impl::TransferToDevice(this, core::mem::transmute(&pwszsourcefilepath), core::mem::transmute_copy(&psourceoperation), core::mem::transmute_copy(&fuflags), core::mem::transmute(&pwszdestinationname), core::mem::transmute_copy(&psourcemetadata), core::mem::transmute_copy(&ptransferprogress)) {
                    Ok(ok__) => {
                        ppnewobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TransferToDevice: TransferToDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPDirectTransfer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMDSPDirectTransfer {}
windows_core::imp::define_interface!(IMDSPEnumDevice, IMDSPEnumDevice_Vtbl, 0x1dcb3a11_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPEnumDevice, windows_core::IUnknown);
impl IMDSPEnumDevice {
    pub unsafe fn Next(&self, ppdevice: &mut [Option<IMDSPDevice>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppdevice.len().try_into().unwrap(), core::mem::transmute(ppdevice.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IMDSPEnumDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMDSPEnumDevice_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppdevice: *mut Option<IMDSPDevice>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMDSPEnumDevice>;
}
impl IMDSPEnumDevice_Vtbl {
    pub const fn new<Identity: IMDSPEnumDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppdevice: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPEnumDevice_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppdevice), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPEnumDevice_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                    Ok(ok__) => {
                        pceltfetched.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPEnumDevice_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPEnumDevice_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IMDSPEnumDevice {}
windows_core::imp::define_interface!(IMDSPEnumStorage, IMDSPEnumStorage_Vtbl, 0x1dcb3a15_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPEnumStorage, windows_core::IUnknown);
impl IMDSPEnumStorage {
    pub unsafe fn Next(&self, ppstorage: &mut [Option<IMDSPStorage>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppstorage.len().try_into().unwrap(), core::mem::transmute(ppstorage.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IMDSPEnumStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMDSPEnumStorage_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppstorage: *mut Option<IMDSPStorage>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMDSPEnumStorage>;
}
impl IMDSPEnumStorage_Vtbl {
    pub const fn new<Identity: IMDSPEnumStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppstorage: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPEnumStorage_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppstorage), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPEnumStorage_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                    Ok(ok__) => {
                        pceltfetched.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPEnumStorage_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPEnumStorage_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IMDSPEnumStorage {}
windows_core::imp::define_interface!(IMDSPObject, IMDSPObject_Vtbl, 0x1dcb3a18_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPObject, windows_core::IUnknown);
impl IMDSPObject {
    pub unsafe fn Open(&self, fumode: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), fumode).ok() }
    }
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pdata as _, pdwsize as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pdata, pdwsize as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn Delete<P1>(&self, fumode: u32, pprogress: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWMDMProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok() }
    }
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), fuflags, dwoffset).ok() }
    }
    pub unsafe fn Rename<P0, P1>(&self, pwsznewname: P0, pprogress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Rename)(windows_core::Interface::as_raw(self), pwsznewname.param().abi(), pprogress.param().abi()).ok() }
    }
    pub unsafe fn Move<P1, P2>(&self, fumode: u32, pprogress: P1, ptarget: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWMDMProgress>,
        P2: windows_core::Param<IMDSPStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi(), ptarget.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMDSPObject_Impl: windows_core::IUnknownImpl {
    fn Open(&self, fumode: u32) -> windows_core::Result<()>;
    fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn Delete(&self, fumode: u32, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<()>;
    fn Seek(&self, fuflags: u32, dwoffset: u32) -> windows_core::Result<()>;
    fn Rename(&self, pwsznewname: &windows_core::PCWSTR, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<()>;
    fn Move(&self, fumode: u32, pprogress: windows_core::Ref<IWMDMProgress>, ptarget: windows_core::Ref<IMDSPStorage>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IMDSPObject_Vtbl {
    pub const fn new<Identity: IMDSPObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Open(this, core::mem::transmute_copy(&fumode)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Read(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Write(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Delete(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&pprogress)).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, dwoffset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Seek(this, core::mem::transmute_copy(&fuflags), core::mem::transmute_copy(&dwoffset)).into()
            }
        }
        unsafe extern "system" fn Rename<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznewname: windows_core::PCWSTR, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Rename(this, core::mem::transmute(&pwsznewname), core::mem::transmute_copy(&pprogress)).into()
            }
        }
        unsafe extern "system" fn Move<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Move(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&pprogress), core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IMDSPObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject_Impl::Close(this).into()
            }
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
impl windows_core::RuntimeName for IMDSPObject {}
windows_core::imp::define_interface!(IMDSPObject2, IMDSPObject2_Vtbl, 0x3f34cd3e_5907_4341_9af9_97f4187c3aa5);
impl core::ops::Deref for IMDSPObject2 {
    type Target = IMDSPObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPObject2, windows_core::IUnknown, IMDSPObject);
impl IMDSPObject2 {
    pub unsafe fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadOnClearChannel)(windows_core::Interface::as_raw(self), pdata as _, pdwsize as _).ok() }
    }
    pub unsafe fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteOnClearChannel)(windows_core::Interface::as_raw(self), pdata, pdwsize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject2_Vtbl {
    pub base__: IMDSPObject_Vtbl,
    pub ReadOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub WriteOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IMDSPObject2_Impl: IMDSPObject_Impl {
    fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> windows_core::Result<()>;
}
impl IMDSPObject2_Vtbl {
    pub const fn new<Identity: IMDSPObject2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadOnClearChannel<Identity: IMDSPObject2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject2_Impl::ReadOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        unsafe extern "system" fn WriteOnClearChannel<Identity: IMDSPObject2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObject2_Impl::WriteOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
            }
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
impl windows_core::RuntimeName for IMDSPObject2 {}
windows_core::imp::define_interface!(IMDSPObjectInfo, IMDSPObjectInfo_Vtbl, 0x1dcb3a19_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPObjectInfo, windows_core::IUnknown);
impl IMDSPObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlayLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlayLength)(windows_core::Interface::as_raw(self), dwlength).ok() }
    }
    pub unsafe fn GetPlayOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlayOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlayOffset)(windows_core::Interface::as_raw(self), dwoffset).ok() }
    }
    pub unsafe fn GetTotalLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLastPlayPosition(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLongestPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObjectInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPlayLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMDSPObjectInfo_Impl: windows_core::IUnknownImpl {
    fn GetPlayLength(&self) -> windows_core::Result<u32>;
    fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()>;
    fn GetPlayOffset(&self) -> windows_core::Result<u32>;
    fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()>;
    fn GetTotalLength(&self) -> windows_core::Result<u32>;
    fn GetLastPlayPosition(&self) -> windows_core::Result<u32>;
    fn GetLongestPlayPosition(&self) -> windows_core::Result<u32>;
}
impl IMDSPObjectInfo_Vtbl {
    pub const fn new<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPlayLength<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPObjectInfo_Impl::GetPlayLength(this) {
                    Ok(ok__) => {
                        pdwlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlayLength<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObjectInfo_Impl::SetPlayLength(this, core::mem::transmute_copy(&dwlength)).into()
            }
        }
        unsafe extern "system" fn GetPlayOffset<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoffset: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPObjectInfo_Impl::GetPlayOffset(this) {
                    Ok(ok__) => {
                        pdwoffset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlayOffset<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPObjectInfo_Impl::SetPlayOffset(this, core::mem::transmute_copy(&dwoffset)).into()
            }
        }
        unsafe extern "system" fn GetTotalLength<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPObjectInfo_Impl::GetTotalLength(this) {
                    Ok(ok__) => {
                        pdwlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastpos: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPObjectInfo_Impl::GetLastPlayPosition(this) {
                    Ok(ok__) => {
                        pdwlastpos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlongestpos: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPObjectInfo_Impl::GetLongestPlayPosition(this) {
                    Ok(ok__) => {
                        pdwlongestpos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IMDSPObjectInfo {}
windows_core::imp::define_interface!(IMDSPRevoked, IMDSPRevoked_Vtbl, 0xa4e8f2d4_3f31_464d_b53d_4fc335998184);
windows_core::imp::interface_hierarchy!(IMDSPRevoked, windows_core::IUnknown);
impl IMDSPRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRevocationURL)(windows_core::Interface::as_raw(self), ppwszrevocationurl as _, pdwbufferlen as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPRevoked_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IMDSPRevoked_Impl: windows_core::IUnknownImpl {
    fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32) -> windows_core::Result<()>;
}
impl IMDSPRevoked_Vtbl {
    pub const fn new<Identity: IMDSPRevoked_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRevocationURL<Identity: IMDSPRevoked_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPRevoked_Impl::GetRevocationURL(this, core::mem::transmute_copy(&ppwszrevocationurl), core::mem::transmute_copy(&pdwbufferlen)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRevocationURL: GetRevocationURL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDSPRevoked as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMDSPRevoked {}
windows_core::imp::define_interface!(IMDSPStorage, IMDSPStorage_Vtbl, 0x1dcb3a16_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPStorage, windows_core::IUnknown);
impl IMDSPStorage {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttributes)(windows_core::Interface::as_raw(self), dwattributes, pformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetStorageGlobals(&self) -> windows_core::Result<IMDSPStorageGlobals> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorageGlobals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: Option<*mut super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), pdwattributes as _, pformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetDate(&self) -> windows_core::Result<WMDMDATETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), pdwsizelow as _, pdwsizehigh as _).ok() }
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRights)(windows_core::Interface::as_raw(self), pprights as _, pnrightscount as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P2>(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>, pwszname: P2) -> windows_core::Result<IMDSPStorage>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStorage)(windows_core::Interface::as_raw(self), dwattributes, pformat.unwrap_or(core::mem::zeroed()) as _, pwszname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IMDSPEnumStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAttributes: usize,
    pub GetStorageGlobals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAttributes: usize,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDMDATETIME) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetRights: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub CreateStorage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Audio::WAVEFORMATEX, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    CreateStorage: usize,
    pub EnumStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPStorage_Impl: windows_core::IUnknownImpl {
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
impl IMDSPStorage_Vtbl {
    pub const fn new<Identity: IMDSPStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAttributes<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage_Impl::SetAttributes(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorageglobals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage_Impl::GetStorageGlobals(this) {
                    Ok(ok__) => {
                        ppstorageglobals.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage_Impl::GetAttributes(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetDate<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage_Impl::GetDate(this) {
                    Ok(ok__) => {
                        pdatetimeutc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage_Impl::GetSize(this, core::mem::transmute_copy(&pdwsizelow), core::mem::transmute_copy(&pdwsizehigh)).into()
            }
        }
        unsafe extern "system" fn GetRights<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage_Impl::GetRights(this, core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn CreateStorage<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: windows_core::PCWSTR, ppnewstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage_Impl::CreateStorage(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat), core::mem::transmute(&pwszname)) {
                    Ok(ok__) => {
                        ppnewstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage_Impl::EnumStorage(this) {
                    Ok(ok__) => {
                        ppenumstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
            }
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
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IMDSPStorage {}
windows_core::imp::define_interface!(IMDSPStorage2, IMDSPStorage2_Vtbl, 0x0a5e07a5_6454_4451_9c36_1c6ae7e2b1d6);
impl core::ops::Deref for IMDSPStorage2 {
    type Target = IMDSPStorage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPStorage2, windows_core::IUnknown, IMDSPStorage);
impl IMDSPStorage2 {
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P4>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P4, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>
    where
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStorage2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, paudioformat.unwrap_or(core::mem::zeroed()) as _, pvideoformat.unwrap_or(core::mem::zeroed()) as _, pwszname.param().abi(), qwfilesize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttributes2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, paudioformat.unwrap_or(core::mem::zeroed()) as _, pvideoformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAttributes2)(windows_core::Interface::as_raw(self), pdwattributes as _, pdwattributesex as _, paudioformat.unwrap_or(core::mem::zeroed()) as _, pvideoformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage2_Vtbl {
    pub base__: IMDSPStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub CreateStorage2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::Audio::WAVEFORMATEX, *const super::MediaFoundation::VIDEOINFOHEADER, windows_core::PCWSTR, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    CreateStorage2: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetAttributes2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::Audio::WAVEFORMATEX, *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetAttributes2: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetAttributes2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut super::Audio::WAVEFORMATEX, *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetAttributes2: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage2_Impl: IMDSPStorage_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
    fn CreateStorage2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: &windows_core::PCWSTR, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>;
    fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
    fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage2_Vtbl {
    pub const fn new<Identity: IMDSPStorage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStorage<Identity: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStorage2<Identity: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: windows_core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage2_Impl::CreateStorage2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat), core::mem::transmute(&pwszname), core::mem::transmute_copy(&qwfilesize)) {
                    Ok(ok__) => {
                        ppnewstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttributes2<Identity: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage2_Impl::SetAttributes2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
            }
        }
        unsafe extern "system" fn GetAttributes2<Identity: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage2_Impl::GetAttributes2(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pdwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
            }
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
impl windows_core::RuntimeName for IMDSPStorage2 {}
windows_core::imp::define_interface!(IMDSPStorage3, IMDSPStorage3_Vtbl, 0x6c669867_97ed_4a67_9706_1c5529d2a414);
impl core::ops::Deref for IMDSPStorage3 {
    type Target = IMDSPStorage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPStorage3, windows_core::IUnknown, IMDSPStorage, IMDSPStorage2);
impl IMDSPStorage3 {
    pub unsafe fn GetMetadata<P0>(&self, pmetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMMetaData>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMetadata)(windows_core::Interface::as_raw(self), pmetadata.param().abi()).ok() }
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMMetaData>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMetadata)(windows_core::Interface::as_raw(self), pmetadata.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage3_Vtbl {
    pub base__: IMDSPStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage3_Impl: IMDSPStorage2_Impl {
    fn GetMetadata(&self, pmetadata: windows_core::Ref<IWMDMMetaData>) -> windows_core::Result<()>;
    fn SetMetadata(&self, pmetadata: windows_core::Ref<IWMDMMetaData>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage3_Vtbl {
    pub const fn new<Identity: IMDSPStorage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadata<Identity: IMDSPStorage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage3_Impl::GetMetadata(this, core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        unsafe extern "system" fn SetMetadata<Identity: IMDSPStorage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage3_Impl::SetMetadata(this, core::mem::transmute_copy(&pmetadata)).into()
            }
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
impl windows_core::RuntimeName for IMDSPStorage3 {}
windows_core::imp::define_interface!(IMDSPStorage4, IMDSPStorage4_Vtbl, 0x3133b2c4_515c_481b_b1ce_39327ecb4f74);
impl core::ops::Deref for IMDSPStorage4 {
    type Target = IMDSPStorage3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPStorage4, windows_core::IUnknown, IMDSPStorage, IMDSPStorage2, IMDSPStorage3);
impl IMDSPStorage4 {
    pub unsafe fn SetReferences(&self, ppispstorage: Option<&[Option<IMDSPStorage>]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReferences)(windows_core::Interface::as_raw(self), ppispstorage.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppispstorage.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut Option<IMDSPStorage>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetReferences)(windows_core::Interface::as_raw(self), pdwrefs as _, pppispstorage as _).ok() }
    }
    pub unsafe fn CreateStorageWithMetadata<P1, P2>(&self, dwattributes: u32, pwszname: P1, pmetadata: P2, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWMDMMetaData>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStorageWithMetadata)(windows_core::Interface::as_raw(self), dwattributes, pwszname.param().abi(), pmetadata.param().abi(), qwfilesize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSpecifiedMetadata<P2>(&self, ppwszpropnames: &[windows_core::PCWSTR], pmetadata: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWMDMMetaData>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetSpecifiedMetadata)(windows_core::Interface::as_raw(self), ppwszpropnames.len().try_into().unwrap(), core::mem::transmute(ppwszpropnames.as_ptr()), pmetadata.param().abi()).ok() }
    }
    pub unsafe fn FindStorage<P1>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P1) -> windows_core::Result<IMDSPStorage>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<IMDSPStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage4_Vtbl {
    pub base__: IMDSPStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStorageWithMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage4_Impl: IMDSPStorage3_Impl {
    fn SetReferences(&self, dwrefs: u32, ppispstorage: *const Option<IMDSPStorage>) -> windows_core::Result<()>;
    fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut Option<IMDSPStorage>) -> windows_core::Result<()>;
    fn CreateStorageWithMetadata(&self, dwattributes: u32, pwszname: &windows_core::PCWSTR, pmetadata: windows_core::Ref<IWMDMMetaData>, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>;
    fn GetSpecifiedMetadata(&self, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR, pmetadata: windows_core::Ref<IWMDMMetaData>) -> windows_core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IMDSPStorage>;
    fn GetParent(&self) -> windows_core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage4_Vtbl {
    pub const fn new<Identity: IMDSPStorage4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetReferences<Identity: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrefs: u32, ppispstorage: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage4_Impl::SetReferences(this, core::mem::transmute_copy(&dwrefs), core::mem::transmute_copy(&ppispstorage)).into()
            }
        }
        unsafe extern "system" fn GetReferences<Identity: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage4_Impl::GetReferences(this, core::mem::transmute_copy(&pdwrefs), core::mem::transmute_copy(&pppispstorage)).into()
            }
        }
        unsafe extern "system" fn CreateStorageWithMetadata<Identity: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pwszname: windows_core::PCWSTR, pmetadata: *mut core::ffi::c_void, qwfilesize: u64, ppnewstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage4_Impl::CreateStorageWithMetadata(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute(&pwszname), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&qwfilesize)) {
                    Ok(ok__) => {
                        ppnewstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorage4_Impl::GetSpecifiedMetadata(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppwszpropnames), core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        unsafe extern "system" fn FindStorage<Identity: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage4_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParent<Identity: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorage4_Impl::GetParent(this) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IMDSPStorage4 {}
windows_core::imp::define_interface!(IMDSPStorageGlobals, IMDSPStorageGlobals_Vtbl, 0x1dcb3a17_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDSPStorageGlobals, windows_core::IUnknown);
impl IMDSPStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnum as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTotalSize)(windows_core::Interface::as_raw(self), pdwtotalsizelow as _, pdwtotalsizehigh as _).ok() }
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTotalFree)(windows_core::Interface::as_raw(self), pdwfreelow as _, pdwfreehigh as _).ok() }
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTotalBad)(windows_core::Interface::as_raw(self), pdwbadlow as _, pdwbadhigh as _).ok() }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize<P1>(&self, fumode: u32, pprogress: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWMDMProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok() }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IMDSPDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootStorage(&self) -> windows_core::Result<IMDSPStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorageGlobals_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDMID, *mut u8) -> windows_core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMDSPStorageGlobals_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self) -> windows_core::Result<u32>;
    fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, fumode: u32, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<()>;
    fn GetDevice(&self) -> windows_core::Result<IMDSPDevice>;
    fn GetRootStorage(&self) -> windows_core::Result<IMDSPStorage>;
}
impl IMDSPStorageGlobals_Vtbl {
    pub const fn new<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorageGlobals_Impl::GetCapabilities(this) {
                    Ok(ok__) => {
                        pdwcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorageGlobals_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnum), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn GetTotalSize<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorageGlobals_Impl::GetTotalSize(this, core::mem::transmute_copy(&pdwtotalsizelow), core::mem::transmute_copy(&pdwtotalsizehigh)).into()
            }
        }
        unsafe extern "system" fn GetTotalFree<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorageGlobals_Impl::GetTotalFree(this, core::mem::transmute_copy(&pdwfreelow), core::mem::transmute_copy(&pdwfreehigh)).into()
            }
        }
        unsafe extern "system" fn GetTotalBad<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorageGlobals_Impl::GetTotalBad(this, core::mem::transmute_copy(&pdwbadlow), core::mem::transmute_copy(&pdwbadhigh)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorageGlobals_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDSPStorageGlobals_Impl::Initialize(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&pprogress)).into()
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorageGlobals_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootStorage<Identity: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDSPStorageGlobals_Impl::GetRootStorage(this) {
                    Ok(ok__) => {
                        pproot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IMDSPStorageGlobals {}
windows_core::imp::define_interface!(IMDServiceProvider, IMDServiceProvider_Vtbl, 0x1dcb3a10_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IMDServiceProvider, windows_core::IUnknown);
impl IMDServiceProvider {
    pub unsafe fn GetDeviceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumDevices(&self) -> windows_core::Result<IMDSPEnumDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMDServiceProvider_Impl: windows_core::IUnknownImpl {
    fn GetDeviceCount(&self) -> windows_core::Result<u32>;
    fn EnumDevices(&self) -> windows_core::Result<IMDSPEnumDevice>;
}
impl IMDServiceProvider_Vtbl {
    pub const fn new<Identity: IMDServiceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceCount<Identity: IMDServiceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDServiceProvider_Impl::GetDeviceCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: IMDServiceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMDServiceProvider_Impl::EnumDevices(this) {
                    Ok(ok__) => {
                        ppenumdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IMDServiceProvider {}
windows_core::imp::define_interface!(IMDServiceProvider2, IMDServiceProvider2_Vtbl, 0xb2fa24b7_cda3_4694_9862_413ae1a34819);
impl core::ops::Deref for IMDServiceProvider2 {
    type Target = IMDServiceProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDServiceProvider2, windows_core::IUnknown, IMDServiceProvider);
impl IMDServiceProvider2 {
    pub unsafe fn CreateDevice<P0>(&self, pwszdevicepath: P0, pdwcount: *mut u32, pppdevicearray: *mut *mut Option<IMDSPDevice>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), pwszdevicepath.param().abi(), pdwcount as _, pppdevicearray as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider2_Vtbl {
    pub base__: IMDServiceProvider_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMDServiceProvider2_Impl: IMDServiceProvider_Impl {
    fn CreateDevice(&self, pwszdevicepath: &windows_core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut Option<IMDSPDevice>) -> windows_core::Result<()>;
}
impl IMDServiceProvider2_Vtbl {
    pub const fn new<Identity: IMDServiceProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: IMDServiceProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicepath: windows_core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDServiceProvider2_Impl::CreateDevice(this, core::mem::transmute(&pwszdevicepath), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pppdevicearray)).into()
            }
        }
        Self { base__: IMDServiceProvider_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDServiceProvider2 as windows_core::Interface>::IID || iid == &<IMDServiceProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMDServiceProvider2 {}
windows_core::imp::define_interface!(IMDServiceProvider3, IMDServiceProvider3_Vtbl, 0x4ed13ef3_a971_4d19_9f51_0e1826b2da57);
impl core::ops::Deref for IMDServiceProvider3 {
    type Target = IMDServiceProvider2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDServiceProvider3, windows_core::IUnknown, IMDServiceProvider, IMDServiceProvider2);
impl IMDServiceProvider3 {
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeviceEnumPreference)(windows_core::Interface::as_raw(self), dwenumpref).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider3_Vtbl {
    pub base__: IMDServiceProvider2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IMDServiceProvider3_Impl: IMDServiceProvider2_Impl {
    fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> windows_core::Result<()>;
}
impl IMDServiceProvider3_Vtbl {
    pub const fn new<Identity: IMDServiceProvider3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: IMDServiceProvider3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenumpref: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMDServiceProvider3_Impl::SetDeviceEnumPreference(this, core::mem::transmute_copy(&dwenumpref)).into()
            }
        }
        Self { base__: IMDServiceProvider2_Vtbl::new::<Identity, OFFSET>(), SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDServiceProvider3 as windows_core::Interface>::IID || iid == &<IMDServiceProvider as windows_core::Interface>::IID || iid == &<IMDServiceProvider2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMDServiceProvider3 {}
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
windows_core::imp::define_interface!(ISCPSecureAuthenticate, ISCPSecureAuthenticate_Vtbl, 0x1dcb3a0f_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(ISCPSecureAuthenticate, windows_core::IUnknown);
impl ISCPSecureAuthenticate {
    pub unsafe fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecureQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSecureQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISCPSecureAuthenticate_Impl: windows_core::IUnknownImpl {
    fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery>;
}
impl ISCPSecureAuthenticate_Vtbl {
    pub const fn new<Identity: ISCPSecureAuthenticate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSecureQuery<Identity: ISCPSecureAuthenticate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCPSecureAuthenticate_Impl::GetSecureQuery(this) {
                    Ok(ok__) => {
                        ppsecurequery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSecureQuery: GetSecureQuery::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISCPSecureAuthenticate {}
windows_core::imp::define_interface!(ISCPSecureAuthenticate2, ISCPSecureAuthenticate2_Vtbl, 0xb580cfae_1672_47e2_acaa_44bbecbcae5b);
impl core::ops::Deref for ISCPSecureAuthenticate2 {
    type Target = ISCPSecureAuthenticate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureAuthenticate2, windows_core::IUnknown, ISCPSecureAuthenticate);
impl ISCPSecureAuthenticate2 {
    pub unsafe fn GetSCPSession(&self) -> windows_core::Result<ISCPSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSCPSession)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate2_Vtbl {
    pub base__: ISCPSecureAuthenticate_Vtbl,
    pub GetSCPSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISCPSecureAuthenticate2_Impl: ISCPSecureAuthenticate_Impl {
    fn GetSCPSession(&self) -> windows_core::Result<ISCPSession>;
}
impl ISCPSecureAuthenticate2_Vtbl {
    pub const fn new<Identity: ISCPSecureAuthenticate2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSCPSession<Identity: ISCPSecureAuthenticate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscpsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCPSecureAuthenticate2_Impl::GetSCPSession(this) {
                    Ok(ok__) => {
                        ppscpsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISCPSecureAuthenticate_Vtbl::new::<Identity, OFFSET>(), GetSCPSession: GetSCPSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate2 as windows_core::Interface>::IID || iid == &<ISCPSecureAuthenticate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISCPSecureAuthenticate2 {}
windows_core::imp::define_interface!(ISCPSecureExchange, ISCPSecureExchange_Vtbl, 0x1dcb3a0e_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(ISCPSecureExchange, windows_core::IUnknown);
impl ISCPSecureExchange {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransferContainerData)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pfureadyflags as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ObjectData)(windows_core::Interface::as_raw(self), pdata as _, pdwsize as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn TransferComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransferComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransferContainerData: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub ObjectData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub TransferComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISCPSecureExchange_Impl: windows_core::IUnknownImpl {
    fn TransferContainerData(&self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn TransferComplete(&self) -> windows_core::Result<()>;
}
impl ISCPSecureExchange_Vtbl {
    pub const fn new<Identity: ISCPSecureExchange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferContainerData<Identity: ISCPSecureExchange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureExchange_Impl::TransferContainerData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pfureadyflags), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn ObjectData<Identity: ISCPSecureExchange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureExchange_Impl::ObjectData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn TransferComplete<Identity: ISCPSecureExchange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureExchange_Impl::TransferComplete(this).into()
            }
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
impl windows_core::RuntimeName for ISCPSecureExchange {}
windows_core::imp::define_interface!(ISCPSecureExchange2, ISCPSecureExchange2_Vtbl, 0x6c62fc7b_2690_483f_9d44_0a20cb35577c);
impl core::ops::Deref for ISCPSecureExchange2 {
    type Target = ISCPSecureExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureExchange2, windows_core::IUnknown, ISCPSecureExchange);
impl ISCPSecureExchange2 {
    pub unsafe fn TransferContainerData2<P2>(&self, pdata: &[u8], pprogresscallback: P2, pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWMDMProgress3>,
    {
        unsafe { (windows_core::Interface::vtable(self).TransferContainerData2)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pprogresscallback.param().abi(), pfureadyflags as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange2_Vtbl {
    pub base__: ISCPSecureExchange_Vtbl,
    pub TransferContainerData2: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
}
pub trait ISCPSecureExchange2_Impl: ISCPSecureExchange_Impl {
    fn TransferContainerData2(&self, pdata: *const u8, dwsize: u32, pprogresscallback: windows_core::Ref<IWMDMProgress3>, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
}
impl ISCPSecureExchange2_Vtbl {
    pub const fn new<Identity: ISCPSecureExchange2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferContainerData2<Identity: ISCPSecureExchange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut core::ffi::c_void, pfureadyflags: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureExchange2_Impl::TransferContainerData2(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pprogresscallback), core::mem::transmute_copy(&pfureadyflags), core::mem::transmute_copy(&abmac)).into()
            }
        }
        Self { base__: ISCPSecureExchange_Vtbl::new::<Identity, OFFSET>(), TransferContainerData2: TransferContainerData2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureExchange2 as windows_core::Interface>::IID || iid == &<ISCPSecureExchange as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISCPSecureExchange2 {}
windows_core::imp::define_interface!(ISCPSecureExchange3, ISCPSecureExchange3_Vtbl, 0xab4e77e4_8908_4b17_bd2a_b1dbe6dd69e1);
impl core::ops::Deref for ISCPSecureExchange3 {
    type Target = ISCPSecureExchange2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureExchange3, windows_core::IUnknown, ISCPSecureExchange, ISCPSecureExchange2);
impl ISCPSecureExchange3 {
    pub unsafe fn TransferContainerDataOnClearChannel<P0, P3>(&self, pdevice: P0, pdata: &[u8], pprogresscallback: P3) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IMDSPDevice>,
        P3: windows_core::Param<IWMDMProgress3>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransferContainerDataOnClearChannel)(windows_core::Interface::as_raw(self), pdevice.param().abi(), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pprogresscallback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObjectDataOnClearChannel<P0>(&self, pdevice: P0, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObjectDataOnClearChannel)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdata as _, pdwsize as _).ok() }
    }
    pub unsafe fn TransferCompleteForDevice<P0>(&self, pdevice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).TransferCompleteForDevice)(windows_core::Interface::as_raw(self), pdevice.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange3_Vtbl {
    pub base__: ISCPSecureExchange2_Vtbl,
    pub TransferContainerDataOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetObjectDataOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub TransferCompleteForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISCPSecureExchange3_Impl: ISCPSecureExchange2_Impl {
    fn TransferContainerDataOnClearChannel(&self, pdevice: windows_core::Ref<IMDSPDevice>, pdata: *const u8, dwsize: u32, pprogresscallback: windows_core::Ref<IWMDMProgress3>) -> windows_core::Result<u32>;
    fn GetObjectDataOnClearChannel(&self, pdevice: windows_core::Ref<IMDSPDevice>, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
    fn TransferCompleteForDevice(&self, pdevice: windows_core::Ref<IMDSPDevice>) -> windows_core::Result<()>;
}
impl ISCPSecureExchange3_Vtbl {
    pub const fn new<Identity: ISCPSecureExchange3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferContainerDataOnClearChannel<Identity: ISCPSecureExchange3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut core::ffi::c_void, pfureadyflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCPSecureExchange3_Impl::TransferContainerDataOnClearChannel(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pprogresscallback)) {
                    Ok(ok__) => {
                        pfureadyflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectDataOnClearChannel<Identity: ISCPSecureExchange3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureExchange3_Impl::GetObjectDataOnClearChannel(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        unsafe extern "system" fn TransferCompleteForDevice<Identity: ISCPSecureExchange3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureExchange3_Impl::TransferCompleteForDevice(this, core::mem::transmute_copy(&pdevice)).into()
            }
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
impl windows_core::RuntimeName for ISCPSecureExchange3 {}
windows_core::imp::define_interface!(ISCPSecureQuery, ISCPSecureQuery_Vtbl, 0x1dcb3a0d_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(ISCPSecureQuery, windows_core::IUnknown);
impl ISCPSecureQuery {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDataDemands)(windows_core::Interface::as_raw(self), pfuflags as _, pdwminrightsdata as _, pdwminexaminedata as _, pdwmindecidedata as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn ExamineData<P1>(&self, fuflags: u32, pwszextension: P1, pdata: &[u8], abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExamineData)(windows_core::Interface::as_raw(self), fuflags, pwszextension.param().abi(), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn MakeDecision<P6>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P6, ppexchange: *mut Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P6: windows_core::Param<IMDSPStorageGlobals>,
    {
        unsafe { (windows_core::Interface::vtable(self).MakeDecision)(windows_core::Interface::as_raw(self), fuflags, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), dwappsec, core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstorageglobals.param().abi(), core::mem::transmute(ppexchange), core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn GetRights<P4>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P4, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IMDSPStorageGlobals>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRights)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.param().abi(), pprights as _, pnrightscount as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataDemands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub ExamineData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    pub MakeDecision: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub GetRights: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32, *mut u8) -> windows_core::HRESULT,
}
pub trait ISCPSecureQuery_Impl: windows_core::IUnknownImpl {
    fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn ExamineData(&self, fuflags: u32, pwszextension: &windows_core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn MakeDecision(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: windows_core::Ref<IMDSPStorageGlobals>, ppexchange: windows_core::OutRef<ISCPSecureExchange>, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetRights(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: windows_core::Ref<IMDSPStorageGlobals>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
}
impl ISCPSecureQuery_Vtbl {
    pub const fn new<Identity: ISCPSecureQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDataDemands<Identity: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureQuery_Impl::GetDataDemands(this, core::mem::transmute_copy(&pfuflags), core::mem::transmute_copy(&pdwminrightsdata), core::mem::transmute_copy(&pdwminexaminedata), core::mem::transmute_copy(&pdwmindecidedata), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn ExamineData<Identity: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pwszextension: windows_core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureQuery_Impl::ExamineData(this, core::mem::transmute_copy(&fuflags), core::mem::transmute(&pwszextension), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn MakeDecision<Identity: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut core::ffi::c_void, ppexchange: *mut *mut core::ffi::c_void, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureQuery_Impl::MakeDecision(this, core::mem::transmute_copy(&fuflags), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwappsec), core::mem::transmute_copy(&pbspsessionkey), core::mem::transmute_copy(&dwsessionkeylen), core::mem::transmute_copy(&pstorageglobals), core::mem::transmute_copy(&ppexchange), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn GetRights<Identity: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureQuery_Impl::GetRights(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pbspsessionkey), core::mem::transmute_copy(&dwsessionkeylen), core::mem::transmute_copy(&pstgglobals), core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount), core::mem::transmute_copy(&abmac)).into()
            }
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
impl windows_core::RuntimeName for ISCPSecureQuery {}
windows_core::imp::define_interface!(ISCPSecureQuery2, ISCPSecureQuery2_Vtbl, 0xebe17e25_4fd7_4632_af46_6d93d4fcc72e);
impl core::ops::Deref for ISCPSecureQuery2 {
    type Target = ISCPSecureQuery;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureQuery2, windows_core::IUnknown, ISCPSecureQuery);
impl ISCPSecureQuery2 {
    pub unsafe fn MakeDecision2<P6, P15>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P6, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: Option<*mut u64>, punknown: P15, ppexchange: *mut Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P6: windows_core::Param<IMDSPStorageGlobals>,
        P15: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MakeDecision2)(
                windows_core::Interface::as_raw(self),
                fuflags,
                core::mem::transmute(pdata.as_ptr()),
                pdata.len().try_into().unwrap(),
                dwappsec,
                core::mem::transmute(pbspsessionkey.as_ptr()),
                pbspsessionkey.len().try_into().unwrap(),
                pstorageglobals.param().abi(),
                core::mem::transmute(pappcertapp.as_ptr()),
                pappcertapp.len().try_into().unwrap(),
                core::mem::transmute(pappcertsp.as_ptr()),
                pappcertsp.len().try_into().unwrap(),
                pszrevocationurl as _,
                pdwrevocationurllen as _,
                pdwrevocationbitflag as _,
                pqwfilesize.unwrap_or(core::mem::zeroed()) as _,
                punknown.param().abi(),
                core::mem::transmute(ppexchange),
                core::mem::transmute(abmac.as_ptr()),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery2_Vtbl {
    pub base__: ISCPSecureQuery_Vtbl,
    pub MakeDecision2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8, u32, *mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut windows_core::PWSTR, *mut u32, *mut u32, *mut u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
pub trait ISCPSecureQuery2_Impl: ISCPSecureQuery_Impl {
    fn MakeDecision2(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: windows_core::Ref<IMDSPStorageGlobals>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: windows_core::Ref<windows_core::IUnknown>, ppexchange: windows_core::OutRef<ISCPSecureExchange>, abmac: *mut u8) -> windows_core::Result<()>;
}
impl ISCPSecureQuery2_Vtbl {
    pub const fn new<Identity: ISCPSecureQuery2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MakeDecision2<Identity: ISCPSecureQuery2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut core::ffi::c_void, ppexchange: *mut *mut core::ffi::c_void, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureQuery2_Impl::MakeDecision2(
                    this,
                    core::mem::transmute_copy(&fuflags),
                    core::mem::transmute_copy(&pdata),
                    core::mem::transmute_copy(&dwsize),
                    core::mem::transmute_copy(&dwappsec),
                    core::mem::transmute_copy(&pbspsessionkey),
                    core::mem::transmute_copy(&dwsessionkeylen),
                    core::mem::transmute_copy(&pstorageglobals),
                    core::mem::transmute_copy(&pappcertapp),
                    core::mem::transmute_copy(&dwappcertapplen),
                    core::mem::transmute_copy(&pappcertsp),
                    core::mem::transmute_copy(&dwappcertsplen),
                    core::mem::transmute_copy(&pszrevocationurl),
                    core::mem::transmute_copy(&pdwrevocationurllen),
                    core::mem::transmute_copy(&pdwrevocationbitflag),
                    core::mem::transmute_copy(&pqwfilesize),
                    core::mem::transmute_copy(&punknown),
                    core::mem::transmute_copy(&ppexchange),
                    core::mem::transmute_copy(&abmac),
                )
                .into()
            }
        }
        Self { base__: ISCPSecureQuery_Vtbl::new::<Identity, OFFSET>(), MakeDecision2: MakeDecision2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCPSecureQuery2 as windows_core::Interface>::IID || iid == &<ISCPSecureQuery as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISCPSecureQuery2 {}
windows_core::imp::define_interface!(ISCPSecureQuery3, ISCPSecureQuery3_Vtbl, 0xb7edd1a2_4dab_484b_b3c5_ad39b8b4c0b1);
impl core::ops::Deref for ISCPSecureQuery3 {
    type Target = ISCPSecureQuery2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureQuery3, windows_core::IUnknown, ISCPSecureQuery, ISCPSecureQuery2);
impl ISCPSecureQuery3 {
    pub unsafe fn GetRightsOnClearChannel<P4, P5>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P4, pprogresscallback: P5, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IMDSPStorageGlobals>,
        P5: windows_core::Param<IWMDMProgress3>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRightsOnClearChannel)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.param().abi(), pprogresscallback.param().abi(), pprights as _, pnrightscount as _).ok() }
    }
    pub unsafe fn MakeDecisionOnClearChannel<P6, P7, P16>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P6, pprogresscallback: P7, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: Option<*mut u64>, punknown: P16, ppexchange: *mut Option<ISCPSecureExchange>) -> windows_core::Result<()>
    where
        P6: windows_core::Param<IMDSPStorageGlobals>,
        P7: windows_core::Param<IWMDMProgress3>,
        P16: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MakeDecisionOnClearChannel)(
                windows_core::Interface::as_raw(self),
                fuflags,
                core::mem::transmute(pdata.as_ptr()),
                pdata.len().try_into().unwrap(),
                dwappsec,
                core::mem::transmute(pbspsessionkey.as_ptr()),
                pbspsessionkey.len().try_into().unwrap(),
                pstorageglobals.param().abi(),
                pprogresscallback.param().abi(),
                core::mem::transmute(pappcertapp.as_ptr()),
                pappcertapp.len().try_into().unwrap(),
                core::mem::transmute(pappcertsp.as_ptr()),
                pappcertsp.len().try_into().unwrap(),
                pszrevocationurl as _,
                pdwrevocationurllen as _,
                pdwrevocationbitflag as _,
                pqwfilesize.unwrap_or(core::mem::zeroed()) as _,
                punknown.param().abi(),
                core::mem::transmute(ppexchange),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery3_Vtbl {
    pub base__: ISCPSecureQuery2_Vtbl,
    pub GetRightsOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32) -> windows_core::HRESULT,
    pub MakeDecisionOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut windows_core::PWSTR, *mut u32, *mut u32, *mut u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISCPSecureQuery3_Impl: ISCPSecureQuery2_Impl {
    fn GetRightsOnClearChannel(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: windows_core::Ref<IMDSPStorageGlobals>, pprogresscallback: windows_core::Ref<IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>;
    fn MakeDecisionOnClearChannel(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: windows_core::Ref<IMDSPStorageGlobals>, pprogresscallback: windows_core::Ref<IWMDMProgress3>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: windows_core::Ref<windows_core::IUnknown>, ppexchange: windows_core::OutRef<ISCPSecureExchange>) -> windows_core::Result<()>;
}
impl ISCPSecureQuery3_Vtbl {
    pub const fn new<Identity: ISCPSecureQuery3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRightsOnClearChannel<Identity: ISCPSecureQuery3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut core::ffi::c_void, pprogresscallback: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureQuery3_Impl::GetRightsOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pbspsessionkey), core::mem::transmute_copy(&dwsessionkeylen), core::mem::transmute_copy(&pstgglobals), core::mem::transmute_copy(&pprogresscallback), core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount)).into()
            }
        }
        unsafe extern "system" fn MakeDecisionOnClearChannel<Identity: ISCPSecureQuery3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut core::ffi::c_void, pprogresscallback: *mut core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut core::ffi::c_void, ppexchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSecureQuery3_Impl::MakeDecisionOnClearChannel(
                    this,
                    core::mem::transmute_copy(&fuflags),
                    core::mem::transmute_copy(&pdata),
                    core::mem::transmute_copy(&dwsize),
                    core::mem::transmute_copy(&dwappsec),
                    core::mem::transmute_copy(&pbspsessionkey),
                    core::mem::transmute_copy(&dwsessionkeylen),
                    core::mem::transmute_copy(&pstorageglobals),
                    core::mem::transmute_copy(&pprogresscallback),
                    core::mem::transmute_copy(&pappcertapp),
                    core::mem::transmute_copy(&dwappcertapplen),
                    core::mem::transmute_copy(&pappcertsp),
                    core::mem::transmute_copy(&dwappcertsplen),
                    core::mem::transmute_copy(&pszrevocationurl),
                    core::mem::transmute_copy(&pdwrevocationurllen),
                    core::mem::transmute_copy(&pdwrevocationbitflag),
                    core::mem::transmute_copy(&pqwfilesize),
                    core::mem::transmute_copy(&punknown),
                    core::mem::transmute_copy(&ppexchange),
                )
                .into()
            }
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
impl windows_core::RuntimeName for ISCPSecureQuery3 {}
windows_core::imp::define_interface!(ISCPSession, ISCPSession_Vtbl, 0x88a3e6ed_eee4_4619_bbb3_fd4fb62715d1);
windows_core::imp::interface_hierarchy!(ISCPSession, windows_core::IUnknown);
impl ISCPSession {
    pub unsafe fn BeginSession<P0>(&self, pidevice: P0, pctx: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginSession)(windows_core::Interface::as_raw(self), pidevice.param().abi(), core::mem::transmute(pctx.as_ptr()), pctx.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn EndSession(&self, pctx: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self), core::mem::transmute(pctx.as_ptr()), pctx.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecureQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub GetSecureQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISCPSession_Impl: windows_core::IUnknownImpl {
    fn BeginSession(&self, pidevice: windows_core::Ref<IMDSPDevice>, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
    fn EndSession(&self, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
    fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery>;
}
impl ISCPSession_Vtbl {
    pub const fn new<Identity: ISCPSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginSession<Identity: ISCPSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidevice: *mut core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSession_Impl::BeginSession(this, core::mem::transmute_copy(&pidevice), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
            }
        }
        unsafe extern "system" fn EndSession<Identity: ISCPSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCPSession_Impl::EndSession(this, core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
            }
        }
        unsafe extern "system" fn GetSecureQuery<Identity: ISCPSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCPSession_Impl::GetSecureQuery(this) {
                    Ok(ok__) => {
                        ppsecurequery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISCPSession {}
windows_core::imp::define_interface!(IWMDMDevice, IWMDMDevice_Vtbl, 0x1dcb3a02_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMDevice, windows_core::IUnknown);
impl IWMDMDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetManufacturer)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnumber as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPowerSource)(windows_core::Interface::as_raw(self), pdwpowersource as _, pdwpercentremaining as _).ok() }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceIcon(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceIcon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IWMDMEnumStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFormatSupport)(windows_core::Interface::as_raw(self), ppformatex as _, pnformatcount as _, pppwszmimetype as _, pnmimetypecount as _).ok() }
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDMID, *mut u8) -> windows_core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Audio::WAVEFORMATEX, *mut u32, *mut *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormatSupport: usize,
    pub SendOpaqueCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDevice_Impl: windows_core::IUnknownImpl {
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
impl IWMDMDevice_Vtbl {
    pub const fn new<Identity: IWMDMDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetManufacturer<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice_Impl::GetManufacturer(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetVersion<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        pdwversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice_Impl::GetType(this) {
                    Ok(ok__) => {
                        pdwtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnumber), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn GetPowerSource<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice_Impl::GetPowerSource(this, core::mem::transmute_copy(&pdwpowersource), core::mem::transmute_copy(&pdwpercentremaining)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice_Impl::GetDeviceIcon(this) {
                    Ok(ok__) => {
                        hicon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice_Impl::EnumStorage(this) {
                    Ok(ok__) => {
                        ppenumstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatSupport<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice_Impl::GetFormatSupport(this, core::mem::transmute_copy(&ppformatex), core::mem::transmute_copy(&pnformatcount), core::mem::transmute_copy(&pppwszmimetype), core::mem::transmute_copy(&pnmimetypecount)).into()
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
            }
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
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMDevice {}
windows_core::imp::define_interface!(IWMDMDevice2, IWMDMDevice2_Vtbl, 0xe34f3d37_9d67_4fc1_9252_62d28b2f8b55);
impl core::ops::Deref for IWMDMDevice2 {
    type Target = IWMDMDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMDevice2, windows_core::IUnknown, IWMDMDevice);
impl IWMDMDevice2 {
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> windows_core::Result<IWMDMStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFormatSupport2)(windows_core::Interface::as_raw(self), dwflags, ppaudioformatex as _, pnaudioformatcount as _, ppvideoformatex as _, pnvideoformatcount as _, ppfiletype as _, pnfiletypecount as _).ok() }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSpecifyPropertyPages)(windows_core::Interface::as_raw(self), core::mem::transmute(ppspecifyproppages), pppunknowns as _, pcunks as _).ok() }
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCanonicalName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice2_Vtbl {
    pub base__: IWMDMDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::Audio::WAVEFORMATEX, *mut u32, *mut *mut super::MediaFoundation::VIDEOINFOHEADER, *mut u32, *mut *mut WMFILECAPABILITIES, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice2_Impl: IWMDMDevice_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
    fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()>;
    fn GetSpecifyPropertyPages(&self, ppspecifyproppages: windows_core::OutRef<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()>;
    fn GetCanonicalName(&self, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IWMDMDevice2_Vtbl {
    pub const fn new<Identity: IWMDMDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStorage<Identity: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice2_Impl::GetFormatSupport2(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppaudioformatex), core::mem::transmute_copy(&pnaudioformatcount), core::mem::transmute_copy(&ppvideoformatex), core::mem::transmute_copy(&pnvideoformatcount), core::mem::transmute_copy(&ppfiletype), core::mem::transmute_copy(&pnfiletypecount)).into()
            }
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppspecifyproppages: *mut *mut core::ffi::c_void, pppunknowns: *mut *mut *mut core::ffi::c_void, pcunks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice2_Impl::GetSpecifyPropertyPages(this, core::mem::transmute_copy(&ppspecifyproppages), core::mem::transmute_copy(&pppunknowns), core::mem::transmute_copy(&pcunks)).into()
            }
        }
        unsafe extern "system" fn GetCanonicalName<Identity: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpnpname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice2_Impl::GetCanonicalName(this, core::mem::transmute_copy(&pwszpnpname), core::mem::transmute_copy(&nmaxchars)).into()
            }
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
impl windows_core::RuntimeName for IWMDMDevice2 {}
windows_core::imp::define_interface!(IWMDMDevice3, IWMDMDevice3_Vtbl, 0x6c03e4fe_05db_4dda_9e3c_06233a6d5d65);
impl core::ops::Deref for IWMDMDevice3 {
    type Target = IWMDMDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMDevice3, windows_core::IUnknown, IWMDMDevice, IWMDMDevice2);
impl IWMDMDevice3 {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), core::mem::transmute(pvalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormatCapability)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceIoControl)(windows_core::Interface::as_raw(self), dwiocontrolcode, core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len().try_into().unwrap(), lpoutbuffer as _, pnoutbuffersize as _).ok() }
    }
    pub unsafe fn FindStorage<P1>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P1) -> windows_core::Result<IWMDMStorage>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice3_Vtbl {
    pub base__: IWMDMDevice2_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetFormatCapability: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FORMATCODE, *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMDMDevice3_Impl: IWMDMDevice2_Impl {
    fn GetProperty(&self, pwszpropname: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&self, pwszpropname: &windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWMDMDevice3_Vtbl {
    pub const fn new<Identity: IWMDMDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice3_Impl::GetProperty(this, core::mem::transmute(&pwszpropname)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpropname: windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice3_Impl::SetProperty(this, core::mem::transmute(&pwszpropname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetFormatCapability<Identity: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice3_Impl::GetFormatCapability(this, core::mem::transmute_copy(&format)) {
                    Ok(ok__) => {
                        pformatsupport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceIoControl<Identity: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDevice3_Impl::DeviceIoControl(this, core::mem::transmute_copy(&dwiocontrolcode), core::mem::transmute_copy(&lpinbuffer), core::mem::transmute_copy(&ninbuffersize), core::mem::transmute_copy(&lpoutbuffer), core::mem::transmute_copy(&pnoutbuffersize)).into()
            }
        }
        unsafe extern "system" fn FindStorage<Identity: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDevice3_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWMDMDevice3 {}
windows_core::imp::define_interface!(IWMDMDeviceControl, IWMDMDeviceControl_Vtbl, 0x1dcb3a04_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMDeviceControl, windows_core::IUnknown);
impl IWMDMDeviceControl {
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Play(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Record)(windows_core::Interface::as_raw(self), pformat).ok() }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), fumode, noffset).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub Record: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    Record: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDeviceControl_Impl: windows_core::IUnknownImpl {
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
impl IWMDMDeviceControl_Vtbl {
    pub const fn new<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDeviceControl_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMDeviceControl_Impl::GetCapabilities(this) {
                    Ok(ok__) => {
                        pdwcapabilitiesmask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Play<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceControl_Impl::Play(this).into()
            }
        }
        unsafe extern "system" fn Record<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceControl_Impl::Record(this, core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceControl_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceControl_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceControl_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, noffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceControl_Impl::Seek(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&noffset)).into()
            }
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
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMDeviceControl {}
windows_core::imp::define_interface!(IWMDMDeviceSession, IWMDMDeviceSession_Vtbl, 0x82af0a65_9d96_412c_83e5_3c43e4b06cc7);
windows_core::imp::interface_hierarchy!(IWMDMDeviceSession, windows_core::IUnknown);
impl IWMDMDeviceSession {
    pub unsafe fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: Option<&[u8]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginSession)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pctx.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: Option<&[u8]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pctx.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_SESSION_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_SESSION_TYPE, *const u8, u32) -> windows_core::HRESULT,
}
pub trait IWMDMDeviceSession_Impl: windows_core::IUnknownImpl {
    fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
    fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::Result<()>;
}
impl IWMDMDeviceSession_Vtbl {
    pub const fn new<Identity: IWMDMDeviceSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginSession<Identity: IWMDMDeviceSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceSession_Impl::BeginSession(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
            }
        }
        unsafe extern "system" fn EndSession<Identity: IWMDMDeviceSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMDeviceSession_Impl::EndSession(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&dwsizectx)).into()
            }
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
impl windows_core::RuntimeName for IWMDMDeviceSession {}
windows_core::imp::define_interface!(IWMDMEnumDevice, IWMDMEnumDevice_Vtbl, 0x1dcb3a01_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMEnumDevice, windows_core::IUnknown);
impl IWMDMEnumDevice {
    pub unsafe fn Next(&self, ppdevice: &mut [Option<IWMDMDevice>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppdevice.len().try_into().unwrap(), core::mem::transmute(ppdevice.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWMDMEnumDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDMEnumDevice_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppdevice: *mut Option<IWMDMDevice>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWMDMEnumDevice>;
}
impl IWMDMEnumDevice_Vtbl {
    pub const fn new<Identity: IWMDMEnumDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppdevice: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMEnumDevice_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppdevice), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMEnumDevice_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                    Ok(ok__) => {
                        pceltfetched.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMEnumDevice_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMEnumDevice_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IWMDMEnumDevice {}
windows_core::imp::define_interface!(IWMDMEnumStorage, IWMDMEnumStorage_Vtbl, 0x1dcb3a05_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMEnumStorage, windows_core::IUnknown);
impl IWMDMEnumStorage {
    pub unsafe fn Next(&self, ppstorage: &mut [Option<IWMDMStorage>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppstorage.len().try_into().unwrap(), core::mem::transmute(ppstorage.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWMDMEnumStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDMEnumStorage_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppstorage: *mut Option<IWMDMStorage>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWMDMEnumStorage>;
}
impl IWMDMEnumStorage_Vtbl {
    pub const fn new<Identity: IWMDMEnumStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppstorage: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMEnumStorage_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppstorage), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMEnumStorage_Impl::Skip(this, core::mem::transmute_copy(&celt)) {
                    Ok(ok__) => {
                        pceltfetched.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMEnumStorage_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMEnumStorage_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IWMDMEnumStorage {}
windows_core::imp::define_interface!(IWMDMLogger, IWMDMLogger_Vtbl, 0x110a3200_5a79_11d3_8d78_444553540000);
windows_core::imp::interface_hierarchy!(IWMDMLogger, windows_core::IUnknown);
impl IWMDMLogger {
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Enable(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
    pub unsafe fn GetLogFileName(&self, pszfilename: windows_core::PSTR, nmaxchars: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLogFileName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszfilename), nmaxchars).ok() }
    }
    pub unsafe fn SetLogFileName<P0>(&self, pszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLogFileName)(windows_core::Interface::as_raw(self), pszfilename.param().abi()).ok() }
    }
    pub unsafe fn LogString<P1, P2>(&self, dwflags: u32, pszsrcname: P1, pszlog: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LogString)(windows_core::Interface::as_raw(self), dwflags, pszsrcname.param().abi(), pszlog.param().abi()).ok() }
    }
    pub unsafe fn LogDword<P1, P2>(&self, dwflags: u32, pszsrcname: P1, pszlogformat: P2, dwlog: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LogDword)(windows_core::Interface::as_raw(self), dwflags, pszsrcname.param().abi(), pszlogformat.param().abi(), dwlog).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSizeParams)(windows_core::Interface::as_raw(self), pdwmaxsize as _, pdwshrinktosize as _).ok() }
    }
    pub unsafe fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSizeParams)(windows_core::Interface::as_raw(self), dwmaxsize, dwshrinktosize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMLogger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, u32) -> windows_core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> windows_core::HRESULT,
    pub LogString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCSTR, windows_core::PCSTR) -> windows_core::HRESULT,
    pub LogDword: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCSTR, windows_core::PCSTR, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSizeParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetSizeParams: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IWMDMLogger_Impl: windows_core::IUnknownImpl {
    fn IsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Enable(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLogFileName(&self, pszfilename: windows_core::PSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn SetLogFileName(&self, pszfilename: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn LogString(&self, dwflags: u32, pszsrcname: &windows_core::PCSTR, pszlog: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn LogDword(&self, dwflags: u32, pszsrcname: &windows_core::PCSTR, pszlogformat: &windows_core::PCSTR, dwlog: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> windows_core::Result<()>;
    fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> windows_core::Result<()>;
}
impl IWMDMLogger_Vtbl {
    pub const fn new<Identity: IWMDMLogger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsEnabled<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMLogger_Impl::IsEnabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enable<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::Enable(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn GetLogFileName<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::GetLogFileName(this, core::mem::transmute_copy(&pszfilename), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn SetLogFileName<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::SetLogFileName(this, core::mem::transmute(&pszfilename)).into()
            }
        }
        unsafe extern "system" fn LogString<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszsrcname: windows_core::PCSTR, pszlog: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::LogString(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszsrcname), core::mem::transmute(&pszlog)).into()
            }
        }
        unsafe extern "system" fn LogDword<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszsrcname: windows_core::PCSTR, pszlogformat: windows_core::PCSTR, dwlog: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::LogDword(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszsrcname), core::mem::transmute(&pszlogformat), core::mem::transmute_copy(&dwlog)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn GetSizeParams<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::GetSizeParams(this, core::mem::transmute_copy(&pdwmaxsize), core::mem::transmute_copy(&pdwshrinktosize)).into()
            }
        }
        unsafe extern "system" fn SetSizeParams<Identity: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMLogger_Impl::SetSizeParams(this, core::mem::transmute_copy(&dwmaxsize), core::mem::transmute_copy(&dwshrinktosize)).into()
            }
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
impl windows_core::RuntimeName for IWMDMLogger {}
windows_core::imp::define_interface!(IWMDMMetaData, IWMDMMetaData_Vtbl, 0xec3b0663_0951_460a_9a80_0dceed3c043c);
windows_core::imp::interface_hierarchy!(IWMDMMetaData, windows_core::IUnknown);
impl IWMDMMetaData {
    pub unsafe fn AddItem<P1>(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: P1, pvalue: Option<&[u8]>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), r#type, pwsztagname.param().abi(), core::mem::transmute(pvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn QueryByName<P0>(&self, pwsztagname: P0, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryByName)(windows_core::Interface::as_raw(self), pwsztagname.param().abi(), ptype as _, pvalue as _, pcblength as _).ok() }
    }
    pub unsafe fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryByIndex)(windows_core::Interface::as_raw(self), iindex, ppwszname as _, ptype as _, ppvalue as _, pcblength as _).ok() }
    }
    pub unsafe fn GetItemCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMMetaData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_TAG_DATATYPE, windows_core::PCWSTR, *const u8, u32) -> windows_core::HRESULT,
    pub QueryByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMDM_TAG_DATATYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub QueryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut WMDM_TAG_DATATYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IWMDMMetaData_Impl: windows_core::IUnknownImpl {
    fn AddItem(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: &windows_core::PCWSTR, pvalue: *const u8, ilength: u32) -> windows_core::Result<()>;
    fn QueryByName(&self, pwsztagname: &windows_core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>;
    fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>;
    fn GetItemCount(&self) -> windows_core::Result<u32>;
}
impl IWMDMMetaData_Vtbl {
    pub const fn new<Identity: IWMDMMetaData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddItem<Identity: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: windows_core::PCWSTR, pvalue: *const u8, ilength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMMetaData_Impl::AddItem(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&pwsztagname), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&ilength)).into()
            }
        }
        unsafe extern "system" fn QueryByName<Identity: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztagname: windows_core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMMetaData_Impl::QueryByName(this, core::mem::transmute(&pwsztagname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn QueryByIndex<Identity: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMMetaData_Impl::QueryByIndex(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&ppwszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&ppvalue), core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn GetItemCount<Identity: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMMetaData_Impl::GetItemCount(this) {
                    Ok(ok__) => {
                        icount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IWMDMMetaData {}
windows_core::imp::define_interface!(IWMDMNotification, IWMDMNotification_Vtbl, 0x3f5e95c0_0f43_4ed4_93d2_c89a45d59b81);
windows_core::imp::interface_hierarchy!(IWMDMNotification, windows_core::IUnknown);
impl IWMDMNotification {
    pub unsafe fn WMDMMessage<P1>(&self, dwmessagetype: u32, pwszcanonicalname: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WMDMMessage)(windows_core::Interface::as_raw(self), dwmessagetype, pwszcanonicalname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WMDMMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWMDMNotification_Impl: windows_core::IUnknownImpl {
    fn WMDMMessage(&self, dwmessagetype: u32, pwszcanonicalname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWMDMNotification_Vtbl {
    pub const fn new<Identity: IWMDMNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WMDMMessage<Identity: IWMDMNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMNotification_Impl::WMDMMessage(this, core::mem::transmute_copy(&dwmessagetype), core::mem::transmute(&pwszcanonicalname)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), WMDMMessage: WMDMMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMNotification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDMNotification {}
windows_core::imp::define_interface!(IWMDMObjectInfo, IWMDMObjectInfo_Vtbl, 0x1dcb3a09_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMObjectInfo, windows_core::IUnknown);
impl IWMDMObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlayLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlayLength)(windows_core::Interface::as_raw(self), dwlength).ok() }
    }
    pub unsafe fn GetPlayOffset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlayOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlayOffset)(windows_core::Interface::as_raw(self), dwoffset).ok() }
    }
    pub unsafe fn GetTotalLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLastPlayPosition(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLongestPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMObjectInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPlayLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IWMDMObjectInfo_Impl: windows_core::IUnknownImpl {
    fn GetPlayLength(&self) -> windows_core::Result<u32>;
    fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()>;
    fn GetPlayOffset(&self) -> windows_core::Result<u32>;
    fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()>;
    fn GetTotalLength(&self) -> windows_core::Result<u32>;
    fn GetLastPlayPosition(&self) -> windows_core::Result<u32>;
    fn GetLongestPlayPosition(&self) -> windows_core::Result<u32>;
}
impl IWMDMObjectInfo_Vtbl {
    pub const fn new<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPlayLength<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMObjectInfo_Impl::GetPlayLength(this) {
                    Ok(ok__) => {
                        pdwlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlayLength<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMObjectInfo_Impl::SetPlayLength(this, core::mem::transmute_copy(&dwlength)).into()
            }
        }
        unsafe extern "system" fn GetPlayOffset<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoffset: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMObjectInfo_Impl::GetPlayOffset(this) {
                    Ok(ok__) => {
                        pdwoffset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlayOffset<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMObjectInfo_Impl::SetPlayOffset(this, core::mem::transmute_copy(&dwoffset)).into()
            }
        }
        unsafe extern "system" fn GetTotalLength<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMObjectInfo_Impl::GetTotalLength(this) {
                    Ok(ok__) => {
                        pdwlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlastpos: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMObjectInfo_Impl::GetLastPlayPosition(this) {
                    Ok(ok__) => {
                        pdwlastpos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlongestpos: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMObjectInfo_Impl::GetLongestPlayPosition(this) {
                    Ok(ok__) => {
                        pdwlongestpos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IWMDMObjectInfo {}
windows_core::imp::define_interface!(IWMDMOperation, IWMDMOperation_Vtbl, 0x1dcb3a0b_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMOperation, windows_core::IUnknown);
impl IWMDMOperation {
    pub unsafe fn BeginRead(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginRead)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BeginWrite(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginWrite)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: Option<*mut super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectAttributes)(windows_core::Interface::as_raw(self), pdwattributes as _, pformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetObjectAttributes)(windows_core::Interface::as_raw(self), dwattributes, pformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectTotalSize)(windows_core::Interface::as_raw(self), pdwsize as _, pdwsizehigh as _).ok() }
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetObjectTotalSize)(windows_core::Interface::as_raw(self), dwsize, dwsizehigh).ok() }
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransferObjectData)(windows_core::Interface::as_raw(self), pdata as _, pdwsize as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn End<P1>(&self, phcompletioncode: *const windows_core::HRESULT, pnewobject: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), phcompletioncode, pnewobject.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginRead: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetObjectAttributes: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetObjectAttributes: usize,
    pub GetObjectTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetObjectTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub TransferObjectData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation_Impl: windows_core::IUnknownImpl {
    fn BeginRead(&self) -> windows_core::Result<()>;
    fn BeginWrite(&self) -> windows_core::Result<()>;
    fn GetObjectName(&self, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn SetObjectName(&self, pwszname: &windows_core::PCWSTR, nmaxchars: u32) -> windows_core::Result<()>;
    fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()>;
    fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> windows_core::Result<()>;
    fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::Result<()>;
    fn End(&self, phcompletioncode: *const windows_core::HRESULT, pnewobject: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMOperation_Vtbl {
    pub const fn new<Identity: IWMDMOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginRead<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::BeginRead(this).into()
            }
        }
        unsafe extern "system" fn BeginWrite<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::BeginWrite(this).into()
            }
        }
        unsafe extern "system" fn GetObjectName<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::GetObjectName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::SetObjectName(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::GetObjectAttributes(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::SetObjectAttributes(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn GetObjectTotalSize<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::GetObjectTotalSize(this, core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&pdwsizehigh)).into()
            }
        }
        unsafe extern "system" fn SetObjectTotalSize<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::SetObjectTotalSize(this, core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwsizehigh)).into()
            }
        }
        unsafe extern "system" fn TransferObjectData<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::TransferObjectData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn End<Identity: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phcompletioncode: *const windows_core::HRESULT, pnewobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation_Impl::End(this, core::mem::transmute_copy(&phcompletioncode), core::mem::transmute_copy(&pnewobject)).into()
            }
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
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMOperation {}
windows_core::imp::define_interface!(IWMDMOperation2, IWMDMOperation2_Vtbl, 0x33445b48_7df7_425c_ad8f_0fc6d82f9f75);
impl core::ops::Deref for IWMDMOperation2 {
    type Target = IWMDMOperation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMOperation2, windows_core::IUnknown, IWMDMOperation);
impl IWMDMOperation2 {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetObjectAttributes2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, pformat.unwrap_or(core::mem::zeroed()) as _, pvideoformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectAttributes2)(windows_core::Interface::as_raw(self), pdwattributes as _, pdwattributesex as _, paudioformat.unwrap_or(core::mem::zeroed()) as _, pvideoformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation2_Vtbl {
    pub base__: IWMDMOperation_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetObjectAttributes2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::Audio::WAVEFORMATEX, *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetObjectAttributes2: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetObjectAttributes2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut super::Audio::WAVEFORMATEX, *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetObjectAttributes2: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMOperation2_Impl: IWMDMOperation_Impl {
    fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
    fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMOperation2_Vtbl {
    pub const fn new<Identity: IWMDMOperation2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetObjectAttributes2<Identity: IWMDMOperation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation2_Impl::SetObjectAttributes2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pvideoformat)).into()
            }
        }
        unsafe extern "system" fn GetObjectAttributes2<Identity: IWMDMOperation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation2_Impl::GetObjectAttributes2(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pdwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
            }
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IWMDMOperation2 {}
windows_core::imp::define_interface!(IWMDMOperation3, IWMDMOperation3_Vtbl, 0xd1f9b46a_9ca8_46d8_9d0f_1ec9bae54919);
impl core::ops::Deref for IWMDMOperation3 {
    type Target = IWMDMOperation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMOperation3, windows_core::IUnknown, IWMDMOperation);
impl IWMDMOperation3 {
    pub unsafe fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransferObjectDataOnClearChannel)(windows_core::Interface::as_raw(self), pdata as _, pdwsize as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation3_Vtbl {
    pub base__: IWMDMOperation_Vtbl,
    pub TransferObjectDataOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation3_Impl: IWMDMOperation_Impl {
    fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMOperation3_Vtbl {
    pub const fn new<Identity: IWMDMOperation3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferObjectDataOnClearChannel<Identity: IWMDMOperation3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMOperation3_Impl::TransferObjectDataOnClearChannel(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        Self { base__: IWMDMOperation_Vtbl::new::<Identity, OFFSET>(), TransferObjectDataOnClearChannel: TransferObjectDataOnClearChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMOperation3 as windows_core::Interface>::IID || iid == &<IWMDMOperation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMOperation3 {}
windows_core::imp::define_interface!(IWMDMProgress, IWMDMProgress_Vtbl, 0x1dcb3a0c_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMProgress, windows_core::IUnknown);
impl IWMDMProgress {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), dwestimatedticks).ok() }
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), dwtranspiredticks).ok() }
    }
    pub unsafe fn End(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDMProgress_Impl: windows_core::IUnknownImpl {
    fn Begin(&self, dwestimatedticks: u32) -> windows_core::Result<()>;
    fn Progress(&self, dwtranspiredticks: u32) -> windows_core::Result<()>;
    fn End(&self) -> windows_core::Result<()>;
}
impl IWMDMProgress_Vtbl {
    pub const fn new<Identity: IWMDMProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin<Identity: IWMDMProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwestimatedticks: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMProgress_Impl::Begin(this, core::mem::transmute_copy(&dwestimatedticks)).into()
            }
        }
        unsafe extern "system" fn Progress<Identity: IWMDMProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtranspiredticks: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMProgress_Impl::Progress(this, core::mem::transmute_copy(&dwtranspiredticks)).into()
            }
        }
        unsafe extern "system" fn End<Identity: IWMDMProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMProgress_Impl::End(this).into()
            }
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
impl windows_core::RuntimeName for IWMDMProgress {}
windows_core::imp::define_interface!(IWMDMProgress2, IWMDMProgress2_Vtbl, 0x3a43f550_b383_4e92_b04a_e6bbc660fefc);
impl core::ops::Deref for IWMDMProgress2 {
    type Target = IWMDMProgress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMProgress2, windows_core::IUnknown, IWMDMProgress);
impl IWMDMProgress2 {
    pub unsafe fn End2(&self, hrcompletioncode: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).End2)(windows_core::Interface::as_raw(self), hrcompletioncode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress2_Vtbl {
    pub base__: IWMDMProgress_Vtbl,
    pub End2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IWMDMProgress2_Impl: IWMDMProgress_Impl {
    fn End2(&self, hrcompletioncode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IWMDMProgress2_Vtbl {
    pub const fn new<Identity: IWMDMProgress2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn End2<Identity: IWMDMProgress2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrcompletioncode: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMProgress2_Impl::End2(this, core::mem::transmute_copy(&hrcompletioncode)).into()
            }
        }
        Self { base__: IWMDMProgress_Vtbl::new::<Identity, OFFSET>(), End2: End2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMProgress2 as windows_core::Interface>::IID || iid == &<IWMDMProgress as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDMProgress2 {}
windows_core::imp::define_interface!(IWMDMProgress3, IWMDMProgress3_Vtbl, 0x21de01cb_3bb4_4929_b21a_17af3f80f658);
impl core::ops::Deref for IWMDMProgress3 {
    type Target = IWMDMProgress2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMProgress3, windows_core::IUnknown, IWMDMProgress, IWMDMProgress2);
impl IWMDMProgress3 {
    pub unsafe fn Begin3(&self, eventid: windows_core::GUID, dwestimatedticks: u32, pcontext: Option<*mut OPAQUECOMMAND>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin3)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid), dwestimatedticks, pcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Progress3(&self, eventid: windows_core::GUID, dwtranspiredticks: u32, pcontext: Option<*mut OPAQUECOMMAND>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Progress3)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid), dwtranspiredticks, pcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn End3(&self, eventid: windows_core::GUID, hrcompletioncode: windows_core::HRESULT, pcontext: Option<*mut OPAQUECOMMAND>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).End3)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid), hrcompletioncode, pcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress3_Vtbl {
    pub base__: IWMDMProgress2_Vtbl,
    pub Begin3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
    pub Progress3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
    pub End3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::HRESULT, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
}
pub trait IWMDMProgress3_Impl: IWMDMProgress2_Impl {
    fn Begin3(&self, eventid: &windows_core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
    fn Progress3(&self, eventid: &windows_core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
    fn End3(&self, eventid: &windows_core::GUID, hrcompletioncode: windows_core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> windows_core::Result<()>;
}
impl IWMDMProgress3_Vtbl {
    pub const fn new<Identity: IWMDMProgress3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin3<Identity: IWMDMProgress3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMProgress3_Impl::Begin3(this, core::mem::transmute(&eventid), core::mem::transmute_copy(&dwestimatedticks), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn Progress3<Identity: IWMDMProgress3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMProgress3_Impl::Progress3(this, core::mem::transmute(&eventid), core::mem::transmute_copy(&dwtranspiredticks), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn End3<Identity: IWMDMProgress3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: windows_core::GUID, hrcompletioncode: windows_core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMProgress3_Impl::End3(this, core::mem::transmute(&eventid), core::mem::transmute_copy(&hrcompletioncode), core::mem::transmute_copy(&pcontext)).into()
            }
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
impl windows_core::RuntimeName for IWMDMProgress3 {}
windows_core::imp::define_interface!(IWMDMRevoked, IWMDMRevoked_Vtbl, 0xebeccedb_88ee_4e55_b6a4_8d9f07d696aa);
windows_core::imp::interface_hierarchy!(IWMDMRevoked, windows_core::IUnknown);
impl IWMDMRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRevocationURL)(windows_core::Interface::as_raw(self), ppwszrevocationurl as _, pdwbufferlen as _, pdwrevokedbitflag as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMRevoked_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IWMDMRevoked_Impl: windows_core::IUnknownImpl {
    fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> windows_core::Result<()>;
}
impl IWMDMRevoked_Vtbl {
    pub const fn new<Identity: IWMDMRevoked_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRevocationURL<Identity: IWMDMRevoked_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMRevoked_Impl::GetRevocationURL(this, core::mem::transmute_copy(&ppwszrevocationurl), core::mem::transmute_copy(&pdwbufferlen), core::mem::transmute_copy(&pdwrevokedbitflag)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRevocationURL: GetRevocationURL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMRevoked as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDMRevoked {}
windows_core::imp::define_interface!(IWMDMStorage, IWMDMStorage_Vtbl, 0x1dcb3a06_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMStorage, windows_core::IUnknown);
impl IWMDMStorage {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttributes)(windows_core::Interface::as_raw(self), dwattributes, pformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetStorageGlobals(&self) -> windows_core::Result<IWMDMStorageGlobals> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorageGlobals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: Option<*mut super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), pdwattributes as _, pformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn GetDate(&self) -> windows_core::Result<WMDMDATETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), pdwsizelow as _, pdwsizehigh as _).ok() }
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRights)(windows_core::Interface::as_raw(self), pprights as _, pnrightscount as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IWMDMEnumStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAttributes: usize,
    pub GetStorageGlobals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAttributes: usize,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDMDATETIME) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetRights: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMStorage_Impl: windows_core::IUnknownImpl {
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
impl IWMDMStorage_Vtbl {
    pub const fn new<Identity: IWMDMStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAttributes<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage_Impl::SetAttributes(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorageglobals: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage_Impl::GetStorageGlobals(this) {
                    Ok(ok__) => {
                        ppstorageglobals.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage_Impl::GetAttributes(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pformat)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PWSTR, nmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage_Impl::GetName(this, core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&nmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetDate<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage_Impl::GetDate(this) {
                    Ok(ok__) => {
                        pdatetimeutc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage_Impl::GetSize(this, core::mem::transmute_copy(&pdwsizelow), core::mem::transmute_copy(&pdwsizehigh)).into()
            }
        }
        unsafe extern "system" fn GetRights<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage_Impl::GetRights(this, core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage_Impl::EnumStorage(this) {
                    Ok(ok__) => {
                        penumstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage_Impl::SendOpaqueCommand(this, core::mem::transmute_copy(&pcommand)).into()
            }
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
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IWMDMStorage {}
windows_core::imp::define_interface!(IWMDMStorage2, IWMDMStorage2_Vtbl, 0x1ed5a144_5cd5_4683_9eff_72cbdb2d9533);
impl core::ops::Deref for IWMDMStorage2 {
    type Target = IWMDMStorage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorage2, windows_core::IUnknown, IWMDMStorage);
impl IWMDMStorage2 {
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> windows_core::Result<IWMDMStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttributes2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, pformat.unwrap_or(core::mem::zeroed()) as _, pvideoformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAttributes2)(windows_core::Interface::as_raw(self), pdwattributes as _, pdwattributesex as _, paudioformat.unwrap_or(core::mem::zeroed()) as _, pvideoformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage2_Vtbl {
    pub base__: IWMDMStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetAttributes2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::Audio::WAVEFORMATEX, *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetAttributes2: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetAttributes2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut super::Audio::WAVEFORMATEX, *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetAttributes2: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage2_Impl: IWMDMStorage_Impl {
    fn GetStorage(&self, pszstoragename: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
    fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
    fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage2_Vtbl {
    pub const fn new<Identity: IWMDMStorage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStorage<Identity: IWMDMStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstoragename: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage2_Impl::GetStorage(this, core::mem::transmute(&pszstoragename)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttributes2<Identity: IWMDMStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage2_Impl::SetAttributes2(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&dwattributesex), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pvideoformat)).into()
            }
        }
        unsafe extern "system" fn GetAttributes2<Identity: IWMDMStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage2_Impl::GetAttributes2(this, core::mem::transmute_copy(&pdwattributes), core::mem::transmute_copy(&pdwattributesex), core::mem::transmute_copy(&paudioformat), core::mem::transmute_copy(&pvideoformat)).into()
            }
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
impl windows_core::RuntimeName for IWMDMStorage2 {}
windows_core::imp::define_interface!(IWMDMStorage3, IWMDMStorage3_Vtbl, 0x97717eea_926a_464e_96a4_247b0216026e);
impl core::ops::Deref for IWMDMStorage3 {
    type Target = IWMDMStorage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorage3, windows_core::IUnknown, IWMDMStorage, IWMDMStorage2);
impl IWMDMStorage3 {
    pub unsafe fn GetMetadata(&self) -> windows_core::Result<IWMDMMetaData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMMetaData>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMetadata)(windows_core::Interface::as_raw(self), pmetadata.param().abi()).ok() }
    }
    pub unsafe fn CreateEmptyMetadataObject(&self) -> windows_core::Result<IWMDMMetaData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEmptyMetadataObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: Option<&[WMDMMetadataView]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnumPreference)(windows_core::Interface::as_raw(self), pmode as _, pviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage3_Vtbl {
    pub base__: IWMDMStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEmptyMetadataObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEnumPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDM_STORAGE_ENUM_MODE, u32, *const WMDMMetadataView) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage3_Impl: IWMDMStorage2_Impl {
    fn GetMetadata(&self) -> windows_core::Result<IWMDMMetaData>;
    fn SetMetadata(&self, pmetadata: windows_core::Ref<IWMDMMetaData>) -> windows_core::Result<()>;
    fn CreateEmptyMetadataObject(&self) -> windows_core::Result<IWMDMMetaData>;
    fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage3_Vtbl {
    pub const fn new<Identity: IWMDMStorage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadata<Identity: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage3_Impl::GetMetadata(this) {
                    Ok(ok__) => {
                        ppmetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMetadata<Identity: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage3_Impl::SetMetadata(this, core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        unsafe extern "system" fn CreateEmptyMetadataObject<Identity: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage3_Impl::CreateEmptyMetadataObject(this) {
                    Ok(ok__) => {
                        ppmetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnumPreference<Identity: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage3_Impl::SetEnumPreference(this, core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&nviews), core::mem::transmute_copy(&pviews)).into()
            }
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
impl windows_core::RuntimeName for IWMDMStorage3 {}
windows_core::imp::define_interface!(IWMDMStorage4, IWMDMStorage4_Vtbl, 0xc225bac5_a03a_40b8_9a23_91cf478c64a6);
impl core::ops::Deref for IWMDMStorage4 {
    type Target = IWMDMStorage3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorage4, windows_core::IUnknown, IWMDMStorage, IWMDMStorage2, IWMDMStorage3);
impl IWMDMStorage4 {
    pub unsafe fn SetReferences(&self, ppiwmdmstorage: Option<&[Option<IWMDMStorage>]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReferences)(windows_core::Interface::as_raw(self), ppiwmdmstorage.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppiwmdmstorage.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut Option<IWMDMStorage>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetReferences)(windows_core::Interface::as_raw(self), pdwrefs as _, pppiwmdmstorage as _).ok() }
    }
    pub unsafe fn GetRightsWithProgress<P0>(&self, piprogresscallback: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress3>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRightsWithProgress)(windows_core::Interface::as_raw(self), piprogresscallback.param().abi(), pprights as _, pnrightscount as _).ok() }
    }
    pub unsafe fn GetSpecifiedMetadata(&self, ppwszpropnames: &[windows_core::PCWSTR]) -> windows_core::Result<IWMDMMetaData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpecifiedMetadata)(windows_core::Interface::as_raw(self), ppwszpropnames.len().try_into().unwrap(), core::mem::transmute(ppwszpropnames.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindStorage<P1>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P1) -> windows_core::Result<IWMDMStorage>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<IWMDMStorage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage4_Vtbl {
    pub base__: IWMDMStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRightsWithProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32) -> windows_core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage4_Impl: IWMDMStorage3_Impl {
    fn SetReferences(&self, dwrefs: u32, ppiwmdmstorage: *const Option<IWMDMStorage>) -> windows_core::Result<()>;
    fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut Option<IWMDMStorage>) -> windows_core::Result<()>;
    fn GetRightsWithProgress(&self, piprogresscallback: windows_core::Ref<IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>;
    fn GetSpecifiedMetadata(&self, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR) -> windows_core::Result<IWMDMMetaData>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &windows_core::PCWSTR) -> windows_core::Result<IWMDMStorage>;
    fn GetParent(&self) -> windows_core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage4_Vtbl {
    pub const fn new<Identity: IWMDMStorage4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetReferences<Identity: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage4_Impl::SetReferences(this, core::mem::transmute_copy(&dwrefs), core::mem::transmute_copy(&ppiwmdmstorage)).into()
            }
        }
        unsafe extern "system" fn GetReferences<Identity: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage4_Impl::GetReferences(this, core::mem::transmute_copy(&pdwrefs), core::mem::transmute_copy(&pppiwmdmstorage)).into()
            }
        }
        unsafe extern "system" fn GetRightsWithProgress<Identity: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piprogresscallback: *mut core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorage4_Impl::GetRightsWithProgress(this, core::mem::transmute_copy(&piprogresscallback), core::mem::transmute_copy(&pprights), core::mem::transmute_copy(&pnrightscount)).into()
            }
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproperties: u32, ppwszpropnames: *const windows_core::PCWSTR, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage4_Impl::GetSpecifiedMetadata(this, core::mem::transmute_copy(&cproperties), core::mem::transmute_copy(&ppwszpropnames)) {
                    Ok(ok__) => {
                        ppmetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindStorage<Identity: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: windows_core::PCWSTR, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage4_Impl::FindStorage(this, core::mem::transmute_copy(&findscope), core::mem::transmute(&pwszuniqueid)) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParent<Identity: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorage4_Impl::GetParent(this) {
                    Ok(ok__) => {
                        ppstorage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IWMDMStorage4 {}
windows_core::imp::define_interface!(IWMDMStorageControl, IWMDMStorageControl_Vtbl, 0x1dcb3a08_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMStorageControl, windows_core::IUnknown);
impl IWMDMStorageControl {
    pub unsafe fn Insert<P1, P2, P3>(&self, fumode: u32, pwszfile: P1, poperation: P2, pprogress: P3) -> windows_core::Result<IWMDMStorage>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWMDMOperation>,
        P3: windows_core::Param<IWMDMProgress>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), fumode, pwszfile.param().abi(), poperation.param().abi(), pprogress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P1>(&self, fumode: u32, pprogress: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWMDMProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok() }
    }
    pub unsafe fn Rename<P1, P2>(&self, fumode: u32, pwsznewname: P1, pprogress: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWMDMProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Rename)(windows_core::Interface::as_raw(self), fumode, pwsznewname.param().abi(), pprogress.param().abi()).ok() }
    }
    pub unsafe fn Read<P1, P2, P3>(&self, fumode: u32, pwszfile: P1, pprogress: P2, poperation: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWMDMProgress>,
        P3: windows_core::Param<IWMDMOperation>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), fumode, pwszfile.param().abi(), pprogress.param().abi(), poperation.param().abi()).ok() }
    }
    pub unsafe fn Move<P1, P2>(&self, fumode: u32, ptargetobject: P1, pprogress: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWMDMStorage>,
        P2: windows_core::Param<IWMDMProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), fumode, ptargetobject.param().abi(), pprogress.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDMStorageControl_Impl: windows_core::IUnknownImpl {
    fn Insert(&self, fumode: u32, pwszfile: &windows_core::PCWSTR, poperation: windows_core::Ref<IWMDMOperation>, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<IWMDMStorage>;
    fn Delete(&self, fumode: u32, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<()>;
    fn Rename(&self, fumode: u32, pwsznewname: &windows_core::PCWSTR, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<()>;
    fn Read(&self, fumode: u32, pwszfile: &windows_core::PCWSTR, pprogress: windows_core::Ref<IWMDMProgress>, poperation: windows_core::Ref<IWMDMOperation>) -> windows_core::Result<()>;
    fn Move(&self, fumode: u32, ptargetobject: windows_core::Ref<IWMDMStorage>, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<()>;
}
impl IWMDMStorageControl_Vtbl {
    pub const fn new<Identity: IWMDMStorageControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Insert<Identity: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwszfile: windows_core::PCWSTR, poperation: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorageControl_Impl::Insert(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwszfile), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&pprogress)) {
                    Ok(ok__) => {
                        ppnewobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageControl_Impl::Delete(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&pprogress)).into()
            }
        }
        unsafe extern "system" fn Rename<Identity: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwsznewname: windows_core::PCWSTR, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageControl_Impl::Rename(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwsznewname), core::mem::transmute_copy(&pprogress)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwszfile: windows_core::PCWSTR, pprogress: *mut core::ffi::c_void, poperation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageControl_Impl::Read(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwszfile), core::mem::transmute_copy(&pprogress), core::mem::transmute_copy(&poperation)).into()
            }
        }
        unsafe extern "system" fn Move<Identity: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, ptargetobject: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageControl_Impl::Move(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&ptargetobject), core::mem::transmute_copy(&pprogress)).into()
            }
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
impl windows_core::RuntimeName for IWMDMStorageControl {}
windows_core::imp::define_interface!(IWMDMStorageControl2, IWMDMStorageControl2_Vtbl, 0x972c2e88_bd6c_4125_8e09_84f837e637b6);
impl core::ops::Deref for IWMDMStorageControl2 {
    type Target = IWMDMStorageControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorageControl2, windows_core::IUnknown, IWMDMStorageControl);
impl IWMDMStorageControl2 {
    pub unsafe fn Insert2<P1, P2, P3, P4, P5>(&self, fumode: u32, pwszfilesource: P1, pwszfiledest: P2, poperation: P3, pprogress: P4, punknown: P5, ppnewobject: Option<*mut Option<IWMDMStorage>>) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWMDMOperation>,
        P4: windows_core::Param<IWMDMProgress>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Insert2)(windows_core::Interface::as_raw(self), fumode, pwszfilesource.param().abi(), pwszfiledest.param().abi(), poperation.param().abi(), pprogress.param().abi(), punknown.param().abi(), ppnewobject.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl2_Vtbl {
    pub base__: IWMDMStorageControl_Vtbl,
    pub Insert2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDMStorageControl2_Impl: IWMDMStorageControl_Impl {
    fn Insert2(&self, fumode: u32, pwszfilesource: &windows_core::PCWSTR, pwszfiledest: &windows_core::PCWSTR, poperation: windows_core::Ref<IWMDMOperation>, pprogress: windows_core::Ref<IWMDMProgress>, punknown: windows_core::Ref<windows_core::IUnknown>, ppnewobject: windows_core::OutRef<IWMDMStorage>) -> windows_core::Result<()>;
}
impl IWMDMStorageControl2_Vtbl {
    pub const fn new<Identity: IWMDMStorageControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Insert2<Identity: IWMDMStorageControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pwszfilesource: windows_core::PCWSTR, pwszfiledest: windows_core::PCWSTR, poperation: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void, punknown: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageControl2_Impl::Insert2(this, core::mem::transmute_copy(&fumode), core::mem::transmute(&pwszfilesource), core::mem::transmute(&pwszfiledest), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&pprogress), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&ppnewobject)).into()
            }
        }
        Self { base__: IWMDMStorageControl_Vtbl::new::<Identity, OFFSET>(), Insert2: Insert2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorageControl2 as windows_core::Interface>::IID || iid == &<IWMDMStorageControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDMStorageControl2 {}
windows_core::imp::define_interface!(IWMDMStorageControl3, IWMDMStorageControl3_Vtbl, 0xb3266365_d4f3_4696_8d53_bd27ec60993a);
impl core::ops::Deref for IWMDMStorageControl3 {
    type Target = IWMDMStorageControl2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorageControl3, windows_core::IUnknown, IWMDMStorageControl, IWMDMStorageControl2);
impl IWMDMStorageControl3 {
    pub unsafe fn Insert3<P2, P3, P4, P5, P6, P7>(&self, fumode: u32, futype: u32, pwszfilesource: P2, pwszfiledest: P3, poperation: P4, pprogress: P5, pmetadata: P6, punknown: P7, ppnewobject: Option<*mut Option<IWMDMStorage>>) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<IWMDMOperation>,
        P5: windows_core::Param<IWMDMProgress>,
        P6: windows_core::Param<IWMDMMetaData>,
        P7: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Insert3)(windows_core::Interface::as_raw(self), fumode, futype, pwszfilesource.param().abi(), pwszfiledest.param().abi(), poperation.param().abi(), pprogress.param().abi(), pmetadata.param().abi(), punknown.param().abi(), ppnewobject.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl3_Vtbl {
    pub base__: IWMDMStorageControl2_Vtbl,
    pub Insert3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDMStorageControl3_Impl: IWMDMStorageControl2_Impl {
    fn Insert3(&self, fumode: u32, futype: u32, pwszfilesource: &windows_core::PCWSTR, pwszfiledest: &windows_core::PCWSTR, poperation: windows_core::Ref<IWMDMOperation>, pprogress: windows_core::Ref<IWMDMProgress>, pmetadata: windows_core::Ref<IWMDMMetaData>, punknown: windows_core::Ref<windows_core::IUnknown>, ppnewobject: windows_core::OutRef<IWMDMStorage>) -> windows_core::Result<()>;
}
impl IWMDMStorageControl3_Vtbl {
    pub const fn new<Identity: IWMDMStorageControl3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Insert3<Identity: IWMDMStorageControl3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: windows_core::PCWSTR, pwszfiledest: windows_core::PCWSTR, poperation: *mut core::ffi::c_void, pprogress: *mut core::ffi::c_void, pmetadata: *mut core::ffi::c_void, punknown: *mut core::ffi::c_void, ppnewobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageControl3_Impl::Insert3(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&futype), core::mem::transmute(&pwszfilesource), core::mem::transmute(&pwszfiledest), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&pprogress), core::mem::transmute_copy(&pmetadata), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&ppnewobject)).into()
            }
        }
        Self { base__: IWMDMStorageControl2_Vtbl::new::<Identity, OFFSET>(), Insert3: Insert3::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorageControl3 as windows_core::Interface>::IID || iid == &<IWMDMStorageControl as windows_core::Interface>::IID || iid == &<IWMDMStorageControl2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDMStorageControl3 {}
windows_core::imp::define_interface!(IWMDMStorageGlobals, IWMDMStorageGlobals_Vtbl, 0x1dcb3a07_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDMStorageGlobals, windows_core::IUnknown);
impl IWMDMStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnum as _, core::mem::transmute(abmac.as_ptr())).ok() }
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTotalSize)(windows_core::Interface::as_raw(self), pdwtotalsizelow as _, pdwtotalsizehigh as _).ok() }
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTotalFree)(windows_core::Interface::as_raw(self), pdwfreelow as _, pdwfreehigh as _).ok() }
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTotalBad)(windows_core::Interface::as_raw(self), pdwbadlow as _, pdwbadhigh as _).ok() }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize<P1>(&self, fumode: u32, pprogress: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWMDMProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageGlobals_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDMID, *mut u8) -> windows_core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDMStorageGlobals_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self) -> windows_core::Result<u32>;
    fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::Result<()>;
    fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()>;
    fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, fumode: u32, pprogress: windows_core::Ref<IWMDMProgress>) -> windows_core::Result<()>;
}
impl IWMDMStorageGlobals_Vtbl {
    pub const fn new<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorageGlobals_Impl::GetCapabilities(this) {
                    Ok(ok__) => {
                        pdwcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageGlobals_Impl::GetSerialNumber(this, core::mem::transmute_copy(&pserialnum), core::mem::transmute_copy(&abmac)).into()
            }
        }
        unsafe extern "system" fn GetTotalSize<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageGlobals_Impl::GetTotalSize(this, core::mem::transmute_copy(&pdwtotalsizelow), core::mem::transmute_copy(&pdwtotalsizehigh)).into()
            }
        }
        unsafe extern "system" fn GetTotalFree<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageGlobals_Impl::GetTotalFree(this, core::mem::transmute_copy(&pdwfreelow), core::mem::transmute_copy(&pdwfreehigh)).into()
            }
        }
        unsafe extern "system" fn GetTotalBad<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageGlobals_Impl::GetTotalBad(this, core::mem::transmute_copy(&pdwbadlow), core::mem::transmute_copy(&pdwbadhigh)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDMStorageGlobals_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fumode: u32, pprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDMStorageGlobals_Impl::Initialize(this, core::mem::transmute_copy(&fumode), core::mem::transmute_copy(&pprogress)).into()
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDMStorageGlobals as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDMStorageGlobals {}
windows_core::imp::define_interface!(IWMDeviceManager, IWMDeviceManager_Vtbl, 0x1dcb3a00_33ed_11d3_8470_00c04f79dbc0);
windows_core::imp::interface_hierarchy!(IWMDeviceManager, windows_core::IUnknown);
impl IWMDeviceManager {
    pub unsafe fn GetRevision(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumDevices(&self) -> windows_core::Result<IWMDMEnumDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDeviceManager_Impl: windows_core::IUnknownImpl {
    fn GetRevision(&self) -> windows_core::Result<u32>;
    fn GetDeviceCount(&self) -> windows_core::Result<u32>;
    fn EnumDevices(&self) -> windows_core::Result<IWMDMEnumDevice>;
}
impl IWMDeviceManager_Vtbl {
    pub const fn new<Identity: IWMDeviceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRevision<Identity: IWMDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrevision: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceManager_Impl::GetRevision(this) {
                    Ok(ok__) => {
                        pdwrevision.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceCount<Identity: IWMDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceManager_Impl::GetDeviceCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: IWMDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceManager_Impl::EnumDevices(this) {
                    Ok(ok__) => {
                        ppenumdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IWMDeviceManager {}
windows_core::imp::define_interface!(IWMDeviceManager2, IWMDeviceManager2_Vtbl, 0x923e5249_8731_4c5b_9b1c_b8b60b6e46af);
impl core::ops::Deref for IWMDeviceManager2 {
    type Target = IWMDeviceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDeviceManager2, windows_core::IUnknown, IWMDeviceManager);
impl IWMDeviceManager2 {
    pub unsafe fn GetDeviceFromCanonicalName<P0>(&self, pwszcanonicalname: P0) -> windows_core::Result<IWMDMDevice>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceFromCanonicalName)(windows_core::Interface::as_raw(self), pwszcanonicalname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumDevices2(&self) -> windows_core::Result<IWMDMEnumDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDevices2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Reinitialize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reinitialize)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager2_Vtbl {
    pub base__: IWMDeviceManager_Vtbl,
    pub GetDeviceFromCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reinitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMDeviceManager2_Impl: IWMDeviceManager_Impl {
    fn GetDeviceFromCanonicalName(&self, pwszcanonicalname: &windows_core::PCWSTR) -> windows_core::Result<IWMDMDevice>;
    fn EnumDevices2(&self) -> windows_core::Result<IWMDMEnumDevice>;
    fn Reinitialize(&self) -> windows_core::Result<()>;
}
impl IWMDeviceManager2_Vtbl {
    pub const fn new<Identity: IWMDeviceManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceFromCanonicalName<Identity: IWMDeviceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcanonicalname: windows_core::PCWSTR, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceManager2_Impl::GetDeviceFromCanonicalName(this, core::mem::transmute(&pwszcanonicalname)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDevices2<Identity: IWMDeviceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMDeviceManager2_Impl::EnumDevices2(this) {
                    Ok(ok__) => {
                        ppenumdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reinitialize<Identity: IWMDeviceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDeviceManager2_Impl::Reinitialize(this).into()
            }
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
impl windows_core::RuntimeName for IWMDeviceManager2 {}
windows_core::imp::define_interface!(IWMDeviceManager3, IWMDeviceManager3_Vtbl, 0xaf185c41_100d_46ed_be2e_9ce8c44594ef);
impl core::ops::Deref for IWMDeviceManager3 {
    type Target = IWMDeviceManager2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDeviceManager3, windows_core::IUnknown, IWMDeviceManager, IWMDeviceManager2);
impl IWMDeviceManager3 {
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeviceEnumPreference)(windows_core::Interface::as_raw(self), dwenumpref).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager3_Vtbl {
    pub base__: IWMDeviceManager2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWMDeviceManager3_Impl: IWMDeviceManager2_Impl {
    fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> windows_core::Result<()>;
}
impl IWMDeviceManager3_Vtbl {
    pub const fn new<Identity: IWMDeviceManager3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: IWMDeviceManager3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwenumpref: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMDeviceManager3_Impl::SetDeviceEnumPreference(this, core::mem::transmute_copy(&dwenumpref)).into()
            }
        }
        Self { base__: IWMDeviceManager2_Vtbl::new::<Identity, OFFSET>(), SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMDeviceManager3 as windows_core::Interface>::IID || iid == &<IWMDeviceManager as windows_core::Interface>::IID || iid == &<IWMDeviceManager2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMDeviceManager3 {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MACINFO {
    pub fUsed: windows_core::BOOL,
    pub abMacState: [u8; 36],
}
impl Default for MACINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MDSP_READ: u32 = 1u32;
pub const MDSP_SEEK_BOF: u32 = 1u32;
pub const MDSP_SEEK_CUR: u32 = 2u32;
pub const MDSP_SEEK_EOF: u32 = 4u32;
pub const MDSP_WRITE: u32 = 2u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MTP_COMMAND_DATA_IN {
    pub OpCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub NextPhase: u32,
    pub CommandWriteDataSize: u32,
    pub CommandWriteData: [u8; 1],
}
impl Default for MTP_COMMAND_DATA_IN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MTP_COMMAND_DATA_OUT {
    pub ResponseCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub CommandReadDataSize: u32,
    pub CommandReadData: [u8; 1],
}
impl Default for MTP_COMMAND_DATA_OUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
pub const MTP_RESPONSE_OK: u16 = 8193u16;
pub const MediaDevMgr: windows_core::GUID = windows_core::GUID::from_u128(0x25baad81_3560_11d3_8471_00c04f79dbc0);
pub const MediaDevMgrClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0x50040c1d_bdbf_4924_b873_f14d6c5bfd66);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPAQUECOMMAND {
    pub guidCommand: windows_core::GUID,
    pub dwDataLen: u32,
    pub pData: *mut u8,
    pub abMAC: [u8; 20],
}
impl Default for OPAQUECOMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RSA_KEY_LEN: u32 = 64u32;
pub const SAC_CERT_V1: u32 = 2u32;
pub const SAC_CERT_X509: u32 = 1u32;
pub const SAC_MAC_LEN: u32 = 8u32;
pub const SAC_PROTOCOL_V1: u32 = 2u32;
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
pub const SCP_EVENTID_ACQSECURECLOCK: windows_core::GUID = windows_core::GUID::from_u128(0x86248cc9_4a59_43e2_9146_48a7f3f4140c);
pub const SCP_EVENTID_DRMINFO: windows_core::GUID = windows_core::GUID::from_u128(0x213dd287_41d2_432b_9e3f_3b4f7b3581dd);
pub const SCP_EVENTID_NEEDTOINDIV: windows_core::GUID = windows_core::GUID::from_u128(0x87a507c7_b469_4386_b976_d5d1ce538a6f);
pub const SCP_PARAMID_DRMVERSION: windows_core::GUID = windows_core::GUID::from_u128(0x41d0155d_7cc7_4217_ada9_005074624da4);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WMDMDATETIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WMDMDetermineMaxPropStringLen {
    pub sz001: [u16; 27],
    pub sz002: [u16; 31],
    pub sz003: [u16; 14],
    pub sz004: [u16; 16],
    pub sz005: [u16; 22],
    pub sz006: [u16; 14],
    pub sz007: [u16; 20],
    pub sz008: [u16; 20],
    pub sz009: [u16; 22],
    pub sz010: [u16; 11],
    pub sz011: [u16; 12],
    pub sz012: [u16; 17],
    pub sz013: [u16; 17],
    pub sz014: [u16; 16],
    pub sz015: [u16; 17],
    pub sz016: [u16; 11],
    pub sz017: [u16; 11],
    pub sz018: [u16; 15],
    pub sz019: [u16; 22],
    pub sz020: [u16; 20],
    pub sz021: [u16; 22],
    pub sz022: [u16; 21],
    pub sz023: [u16; 24],
    pub sz024: [u16; 20],
    pub sz025: [u16; 10],
    pub sz026: [u16; 14],
    pub sz027: [u16; 11],
    pub sz028: [u16; 11],
    pub sz029: [u16; 13],
    pub sz030: [u16; 17],
    pub sz031: [u16; 16],
    pub sz032: [u16; 17],
    pub sz033: [u16; 20],
    pub sz034: [u16; 19],
    pub sz035: [u16; 18],
    pub sz036: [u16; 18],
    pub sz037: [u16; 15],
    pub sz041: [u16; 14],
    pub sz043: [u16; 22],
    pub sz044: [u16; 16],
    pub sz045: [u16; 20],
    pub sz046: [u16; 14],
    pub sz047: [u16; 14],
    pub sz048: [u16; 12],
    pub sz049: [u16; 25],
    pub sz050: [u16; 26],
    pub sz051: [u16; 25],
    pub sz052: [u16; 16],
    pub sz053: [u16; 24],
    pub sz054: [u16; 15],
    pub sz055: [u16; 21],
    pub sz056: [u16; 16],
    pub sz057: [u16; 22],
    pub sz058: [u16; 14],
    pub sz059: [u16; 25],
    pub sz060: [u16; 18],
    pub sz061: [u16; 22],
    pub sz062: [u16; 26],
    pub sz063: [u16; 36],
    pub sz064: [u16; 23],
    pub sz065: [u16; 12],
    pub sz066: [u16; 24],
    pub sz067: [u16; 11],
    pub sz068: [u16; 12],
    pub sz069: [u16; 14],
    pub sz070: [u16; 20],
    pub sz071: [u16; 15],
    pub sz072: [u16; 14],
    pub sz073: [u16; 31],
    pub sz074: [u16; 24],
    pub sz075: [u16; 22],
    pub sz076: [u16; 24],
    pub sz077: [u16; 21],
    pub sz078: [u16; 27],
    pub sz079: [u16; 27],
    pub sz080: [u16; 20],
    pub sz081: [u16; 33],
    pub sz082: [u16; 21],
    pub sz083: [u16; 32],
    pub sz084: [u16; 26],
    pub sz085: [u16; 18],
    pub sz086: [u16; 30],
}
impl Default for WMDMDetermineMaxPropStringLen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDMDevice: windows_core::GUID = windows_core::GUID::from_u128(0x807b3cdf_357a_11d3_8471_00c04f79dbc0);
pub const WMDMDeviceEnum: windows_core::GUID = windows_core::GUID::from_u128(0x430e35af_3971_11d3_8474_00c04f79dbc0);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WMDMID {
    pub cbSize: u32,
    pub dwVendorID: u32,
    pub pID: [u8; 128],
    pub SerialNumberLength: u32,
}
impl Default for WMDMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDMID_LENGTH: u32 = 128u32;
pub const WMDMLogger: windows_core::GUID = windows_core::GUID::from_u128(0x110a3202_5a79_11d3_8d78_444553540000);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDMMessage(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WMDMMetadataView {
    pub pwszViewName: windows_core::PWSTR,
    pub nDepth: u32,
    pub ppwszTags: *mut *mut u16,
}
impl Default for WMDMMetadataView {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WMDMRIGHTS {
    pub cbSize: u32,
    pub dwContentType: u32,
    pub fuFlags: u32,
    pub fuRights: u32,
    pub dwAppSec: u32,
    pub dwPlaybackCount: u32,
    pub ExpirationDate: WMDMDATETIME,
}
pub const WMDMStorage: windows_core::GUID = windows_core::GUID::from_u128(0x807b3ce0_357a_11d3_8471_00c04f79dbc0);
pub const WMDMStorageEnum: windows_core::GUID = windows_core::GUID::from_u128(0xeb401a3b_3af7_11d3_8474_00c04f79dbc0);
pub const WMDMStorageGlobal: windows_core::GUID = windows_core::GUID::from_u128(0x807b3ce1_357a_11d3_8471_00c04f79dbc0);
pub const WMDM_APP_REVOKED: u32 = 2u32;
pub const WMDM_CONTENT_FILE: u32 = 4u32;
pub const WMDM_CONTENT_FOLDER: u32 = 8u32;
pub const WMDM_CONTENT_OPERATIONINTERFACE: u32 = 16u32;
pub const WMDM_DEVICECAP_CANPAUSE: u32 = 16u32;
pub const WMDM_DEVICECAP_CANPLAY: u32 = 1u32;
pub const WMDM_DEVICECAP_CANRECORD: u32 = 4u32;
pub const WMDM_DEVICECAP_CANRESUME: u32 = 32u32;
pub const WMDM_DEVICECAP_CANSEEK: u32 = 128u32;
pub const WMDM_DEVICECAP_CANSTOP: u32 = 64u32;
pub const WMDM_DEVICECAP_CANSTREAMPLAY: u32 = 2u32;
pub const WMDM_DEVICECAP_CANSTREAMRECORD: u32 = 8u32;
pub const WMDM_DEVICECAP_HASSECURECLOCK: u32 = 256u32;
pub const WMDM_DEVICE_PROTOCOL_MSC: windows_core::GUID = windows_core::GUID::from_u128(0xa4d2c26c_a881_44bb_bd5d_1f703c71f7a9);
pub const WMDM_DEVICE_PROTOCOL_MTP: windows_core::GUID = windows_core::GUID::from_u128(0x979e54e5_0afc_4604_8d93_dc798a4bcf45);
pub const WMDM_DEVICE_PROTOCOL_RAPI: windows_core::GUID = windows_core::GUID::from_u128(0x2a11ed91_8c8f_41e4_82d1_8386e003561c);
pub const WMDM_DEVICE_TYPE_DECODE: u32 = 4u32;
pub const WMDM_DEVICE_TYPE_ENCODE: u32 = 8u32;
pub const WMDM_DEVICE_TYPE_FILELISTRESYNC: u32 = 512u32;
pub const WMDM_DEVICE_TYPE_NONREENTRANT: u32 = 256u32;
pub const WMDM_DEVICE_TYPE_NONSDMI: u32 = 128u32;
pub const WMDM_DEVICE_TYPE_PLAYBACK: u32 = 1u32;
pub const WMDM_DEVICE_TYPE_RECORD: u32 = 2u32;
pub const WMDM_DEVICE_TYPE_SDMI: u32 = 64u32;
pub const WMDM_DEVICE_TYPE_STORAGE: u32 = 16u32;
pub const WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW: u32 = 1024u32;
pub const WMDM_DEVICE_TYPE_VIRTUAL: u32 = 32u32;
pub const WMDM_ENUM_PROP_VALID_VALUES_ANY: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(0i32);
pub const WMDM_ENUM_PROP_VALID_VALUES_ENUM: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDM_ENUM_PROP_VALID_VALUES_FORM(pub i32);
pub const WMDM_ENUM_PROP_VALID_VALUES_RANGE: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(1i32);
pub const WMDM_E_BUFFERTOOSMALL: i32 = -2147201016i32;
pub const WMDM_E_BUSY: i32 = -2147201024i32;
pub const WMDM_E_CALL_OUT_OF_SEQUENCE: i32 = -2147201017i32;
pub const WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE: i32 = -2147201005i32;
pub const WMDM_E_INCORRECT_APPSEC: i32 = -2147201008i32;
pub const WMDM_E_INCORRECT_RIGHTS: i32 = -2147201007i32;
pub const WMDM_E_INTERFACEDEAD: i32 = -2147201023i32;
pub const WMDM_E_INVALIDTYPE: i32 = -2147201022i32;
pub const WMDM_E_LICENSE_EXPIRED: i32 = -2147201006i32;
pub const WMDM_E_LICENSE_NOTEXIST: i32 = -2147201009i32;
pub const WMDM_E_MAC_CHECK_FAILED: i32 = -2147201014i32;
pub const WMDM_E_MOREDATA: i32 = -2147201015i32;
pub const WMDM_E_NORIGHTS: i32 = -2147201018i32;
pub const WMDM_E_NOTCERTIFIED: i32 = -2147201019i32;
pub const WMDM_E_NOTSUPPORTED: i32 = -2147201020i32;
pub const WMDM_E_PROCESSFAILED: i32 = -2147201021i32;
pub const WMDM_E_REVOKED: i32 = -2147201010i32;
pub const WMDM_E_SDMI_NOMORECOPIES: i32 = -2147201011i32;
pub const WMDM_E_SDMI_TRIGGER: i32 = -2147201012i32;
pub const WMDM_E_TOO_MANY_SESSIONS: i32 = -2147201005i32;
pub const WMDM_E_USER_CANCELLED: i32 = -2147201013i32;
pub const WMDM_FILE_ATTR_AUDIO: u32 = 4096u32;
pub const WMDM_FILE_ATTR_AUDIOBOOK: u32 = 2097152u32;
pub const WMDM_FILE_ATTR_CANDELETE: u32 = 32768u32;
pub const WMDM_FILE_ATTR_CANMOVE: u32 = 65536u32;
pub const WMDM_FILE_ATTR_CANPLAY: u32 = 16384u32;
pub const WMDM_FILE_ATTR_CANREAD: u32 = 262144u32;
pub const WMDM_FILE_ATTR_CANRENAME: u32 = 131072u32;
pub const WMDM_FILE_ATTR_DATA: u32 = 8192u32;
pub const WMDM_FILE_ATTR_FILE: u32 = 32u32;
pub const WMDM_FILE_ATTR_FOLDER: u32 = 8u32;
pub const WMDM_FILE_ATTR_HIDDEN: u32 = 4194304u32;
pub const WMDM_FILE_ATTR_LINK: u32 = 16u32;
pub const WMDM_FILE_ATTR_MUSIC: u32 = 524288u32;
pub const WMDM_FILE_ATTR_READONLY: u32 = 16777216u32;
pub const WMDM_FILE_ATTR_SYSTEM: u32 = 8388608u32;
pub const WMDM_FILE_ATTR_VIDEO: u32 = 64u32;
pub const WMDM_FILE_CREATE_OVERWRITE: u32 = 1048576u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDM_FIND_SCOPE(pub i32);
pub const WMDM_FIND_SCOPE_GLOBAL: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(0i32);
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDM_FORMATCODE(pub i32);
pub const WMDM_FORMATCODE_3G2: WMDM_FORMATCODE = WMDM_FORMATCODE(47493i32);
pub const WMDM_FORMATCODE_3G2A: WMDM_FORMATCODE = WMDM_FORMATCODE(860303937i32);
pub const WMDM_FORMATCODE_3GP: WMDM_FORMATCODE = WMDM_FORMATCODE(47492i32);
pub const WMDM_FORMATCODE_3GPA: WMDM_FORMATCODE = WMDM_FORMATCODE(860311617i32);
pub const WMDM_FORMATCODE_AAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47363i32);
pub const WMDM_FORMATCODE_ABSTRACTAUDIOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47619i32);
pub const WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47621i32);
pub const WMDM_FORMATCODE_ABSTRACTCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48641i32);
pub const WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47624i32);
pub const WMDM_FORMATCODE_ABSTRACTCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48001i32);
pub const WMDM_FORMATCODE_ABSTRACTCONTACTGROUP: WMDM_FORMATCODE = WMDM_FORMATCODE(47622i32);
pub const WMDM_FORMATCODE_ABSTRACTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47745i32);
pub const WMDM_FORMATCODE_ABSTRACTIMAGEALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47618i32);
pub const WMDM_FORMATCODE_ABSTRACTMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47873i32);
pub const WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER: WMDM_FORMATCODE = WMDM_FORMATCODE(47623i32);
pub const WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47617i32);
pub const WMDM_FORMATCODE_ABSTRACTVIDEOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47620i32);
pub const WMDM_FORMATCODE_AIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(12295i32);
pub const WMDM_FORMATCODE_ALLIMAGES: WMDM_FORMATCODE = WMDM_FORMATCODE(-1i32);
pub const WMDM_FORMATCODE_AMR: WMDM_FORMATCODE = WMDM_FORMATCODE(47368i32);
pub const WMDM_FORMATCODE_ASF: WMDM_FORMATCODE = WMDM_FORMATCODE(12300i32);
pub const WMDM_FORMATCODE_ASSOCIATION: WMDM_FORMATCODE = WMDM_FORMATCODE(12289i32);
pub const WMDM_FORMATCODE_ASXPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47635i32);
pub const WMDM_FORMATCODE_ATSCTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47495i32);
pub const WMDM_FORMATCODE_AUDIBLE: WMDM_FORMATCODE = WMDM_FORMATCODE(47364i32);
pub const WMDM_FORMATCODE_AVCHD: WMDM_FORMATCODE = WMDM_FORMATCODE(47494i32);
pub const WMDM_FORMATCODE_AVI: WMDM_FORMATCODE = WMDM_FORMATCODE(12298i32);
pub const WMDM_FORMATCODE_DPOF: WMDM_FORMATCODE = WMDM_FORMATCODE(12294i32);
pub const WMDM_FORMATCODE_DVBTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47496i32);
pub const WMDM_FORMATCODE_EXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(12291i32);
pub const WMDM_FORMATCODE_FLAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47366i32);
pub const WMDM_FORMATCODE_HTML: WMDM_FORMATCODE = WMDM_FORMATCODE(12293i32);
pub const WMDM_FORMATCODE_IMAGE_BMP: WMDM_FORMATCODE = WMDM_FORMATCODE(14340i32);
pub const WMDM_FORMATCODE_IMAGE_CIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14341i32);
pub const WMDM_FORMATCODE_IMAGE_EXIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14337i32);
pub const WMDM_FORMATCODE_IMAGE_FLASHPIX: WMDM_FORMATCODE = WMDM_FORMATCODE(14339i32);
pub const WMDM_FORMATCODE_IMAGE_GIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14343i32);
pub const WMDM_FORMATCODE_IMAGE_JFIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14344i32);
pub const WMDM_FORMATCODE_IMAGE_JP2: WMDM_FORMATCODE = WMDM_FORMATCODE(14351i32);
pub const WMDM_FORMATCODE_IMAGE_JPX: WMDM_FORMATCODE = WMDM_FORMATCODE(14352i32);
pub const WMDM_FORMATCODE_IMAGE_PCD: WMDM_FORMATCODE = WMDM_FORMATCODE(14345i32);
pub const WMDM_FORMATCODE_IMAGE_PICT: WMDM_FORMATCODE = WMDM_FORMATCODE(14346i32);
pub const WMDM_FORMATCODE_IMAGE_PNG: WMDM_FORMATCODE = WMDM_FORMATCODE(14347i32);
pub const WMDM_FORMATCODE_IMAGE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(14353i32);
pub const WMDM_FORMATCODE_IMAGE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(16383i32);
pub const WMDM_FORMATCODE_IMAGE_TIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14349i32);
pub const WMDM_FORMATCODE_IMAGE_TIFFEP: WMDM_FORMATCODE = WMDM_FORMATCODE(14338i32);
pub const WMDM_FORMATCODE_IMAGE_TIFFIT: WMDM_FORMATCODE = WMDM_FORMATCODE(14350i32);
pub const WMDM_FORMATCODE_IMAGE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(14336i32);
pub const WMDM_FORMATCODE_JPEGXR: WMDM_FORMATCODE = WMDM_FORMATCODE(47108i32);
pub const WMDM_FORMATCODE_M3UPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47633i32);
pub const WMDM_FORMATCODE_M4A: WMDM_FORMATCODE = WMDM_FORMATCODE(1297101889i32);
pub const WMDM_FORMATCODE_MEDIA_CAST: WMDM_FORMATCODE = WMDM_FORMATCODE(47627i32);
pub const WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47748i32);
pub const WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET: WMDM_FORMATCODE = WMDM_FORMATCODE(47749i32);
pub const WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47750i32);
pub const WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47747i32);
pub const WMDM_FORMATCODE_MK3D: WMDM_FORMATCODE = WMDM_FORMATCODE(47499i32);
pub const WMDM_FORMATCODE_MKA: WMDM_FORMATCODE = WMDM_FORMATCODE(47498i32);
pub const WMDM_FORMATCODE_MKV: WMDM_FORMATCODE = WMDM_FORMATCODE(47497i32);
pub const WMDM_FORMATCODE_MP2: WMDM_FORMATCODE = WMDM_FORMATCODE(47491i32);
pub const WMDM_FORMATCODE_MP3: WMDM_FORMATCODE = WMDM_FORMATCODE(12297i32);
pub const WMDM_FORMATCODE_MP4: WMDM_FORMATCODE = WMDM_FORMATCODE(47490i32);
pub const WMDM_FORMATCODE_MPEG: WMDM_FORMATCODE = WMDM_FORMATCODE(12299i32);
pub const WMDM_FORMATCODE_MPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47634i32);
pub const WMDM_FORMATCODE_NOTUSED: WMDM_FORMATCODE = WMDM_FORMATCODE(0i32);
pub const WMDM_FORMATCODE_OGG: WMDM_FORMATCODE = WMDM_FORMATCODE(47362i32);
pub const WMDM_FORMATCODE_PLSPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47636i32);
pub const WMDM_FORMATCODE_QCELP: WMDM_FORMATCODE = WMDM_FORMATCODE(47367i32);
pub const WMDM_FORMATCODE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(12301i32);
pub const WMDM_FORMATCODE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(14335i32);
pub const WMDM_FORMATCODE_SCRIPT: WMDM_FORMATCODE = WMDM_FORMATCODE(12290i32);
pub const WMDM_FORMATCODE_SECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(48770i32);
pub const WMDM_FORMATCODE_TEXT: WMDM_FORMATCODE = WMDM_FORMATCODE(12292i32);
pub const WMDM_FORMATCODE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(12288i32);
pub const WMDM_FORMATCODE_UNDEFINEDAUDIO: WMDM_FORMATCODE = WMDM_FORMATCODE(47360i32);
pub const WMDM_FORMATCODE_UNDEFINEDCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48640i32);
pub const WMDM_FORMATCODE_UNDEFINEDCOLLECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47616i32);
pub const WMDM_FORMATCODE_UNDEFINEDCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48000i32);
pub const WMDM_FORMATCODE_UNDEFINEDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47744i32);
pub const WMDM_FORMATCODE_UNDEFINEDFIRMWARE: WMDM_FORMATCODE = WMDM_FORMATCODE(47106i32);
pub const WMDM_FORMATCODE_UNDEFINEDMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47872i32);
pub const WMDM_FORMATCODE_UNDEFINEDVIDEO: WMDM_FORMATCODE = WMDM_FORMATCODE(47488i32);
pub const WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(48768i32);
pub const WMDM_FORMATCODE_VCALENDAR1: WMDM_FORMATCODE = WMDM_FORMATCODE(48642i32);
pub const WMDM_FORMATCODE_VCALENDAR2: WMDM_FORMATCODE = WMDM_FORMATCODE(48643i32);
pub const WMDM_FORMATCODE_VCARD2: WMDM_FORMATCODE = WMDM_FORMATCODE(48002i32);
pub const WMDM_FORMATCODE_VCARD3: WMDM_FORMATCODE = WMDM_FORMATCODE(48003i32);
pub const WMDM_FORMATCODE_WAVE: WMDM_FORMATCODE = WMDM_FORMATCODE(12296i32);
pub const WMDM_FORMATCODE_WBMP: WMDM_FORMATCODE = WMDM_FORMATCODE(47107i32);
pub const WMDM_FORMATCODE_WINDOWSIMAGEFORMAT: WMDM_FORMATCODE = WMDM_FORMATCODE(47233i32);
pub const WMDM_FORMATCODE_WMA: WMDM_FORMATCODE = WMDM_FORMATCODE(47361i32);
pub const WMDM_FORMATCODE_WMV: WMDM_FORMATCODE = WMDM_FORMATCODE(47489i32);
pub const WMDM_FORMATCODE_WPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47632i32);
pub const WMDM_FORMATCODE_XMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47746i32);
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for WMDM_FORMAT_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDM_GET_FORMAT_SUPPORT_AUDIO: u32 = 1u32;
pub const WMDM_GET_FORMAT_SUPPORT_FILE: u32 = 4u32;
pub const WMDM_GET_FORMAT_SUPPORT_VIDEO: u32 = 2u32;
pub const WMDM_LOG_NOTIMESTAMP: u32 = 16u32;
pub const WMDM_LOG_SEV_ERROR: u32 = 4u32;
pub const WMDM_LOG_SEV_INFO: u32 = 1u32;
pub const WMDM_LOG_SEV_WARN: u32 = 2u32;
pub const WMDM_MAC_LENGTH: u32 = 8u32;
pub const WMDM_MODE_BLOCK: u32 = 1u32;
pub const WMDM_MODE_PROGRESS: u32 = 64u32;
pub const WMDM_MODE_QUERY: u32 = 32u32;
pub const WMDM_MODE_RECURSIVE: u32 = 4096u32;
pub const WMDM_MODE_THREAD: u32 = 2u32;
pub const WMDM_MODE_TRANSFER_PROTECTED: u32 = 128u32;
pub const WMDM_MODE_TRANSFER_UNPROTECTED: u32 = 256u32;
pub const WMDM_MSG_DEVICE_ARRIVAL: WMDMMessage = WMDMMessage(0i32);
pub const WMDM_MSG_DEVICE_REMOVAL: WMDMMessage = WMDMMessage(1i32);
pub const WMDM_MSG_MEDIA_ARRIVAL: WMDMMessage = WMDMMessage(2i32);
pub const WMDM_MSG_MEDIA_REMOVAL: WMDMMessage = WMDMMessage(3i32);
pub const WMDM_POWER_CAP_BATTERY: u32 = 1u32;
pub const WMDM_POWER_CAP_EXTERNAL: u32 = 2u32;
pub const WMDM_POWER_IS_BATTERY: u32 = 4u32;
pub const WMDM_POWER_IS_EXTERNAL: u32 = 8u32;
pub const WMDM_POWER_PERCENT_AVAILABLE: u32 = 16u32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for WMDM_PROP_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: windows_core::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for WMDM_PROP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: core::mem::ManuallyDrop<WMDM_PROP_VALUES_RANGE>,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for WMDM_PROP_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for WMDM_PROP_VALUES_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeMax: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeStep: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for WMDM_PROP_VALUES_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDM_RIGHTS_COPY_TO_CD: u32 = 8u32;
pub const WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE: u32 = 2u32;
pub const WMDM_RIGHTS_COPY_TO_SDMI_DEVICE: u32 = 16u32;
pub const WMDM_RIGHTS_EXPIRATIONDATE: u32 = 2u32;
pub const WMDM_RIGHTS_FREESERIALIDS: u32 = 8u32;
pub const WMDM_RIGHTS_GROUPID: u32 = 4u32;
pub const WMDM_RIGHTS_NAMEDSERIALIDS: u32 = 16u32;
pub const WMDM_RIGHTS_PLAYBACKCOUNT: u32 = 1u32;
pub const WMDM_RIGHTS_PLAY_ON_PC: u32 = 1u32;
pub const WMDM_SCP_DECIDE_DATA: i32 = 8i32;
pub const WMDM_SCP_DRMINFO_NOT_DRMPROTECTED: i32 = 0i32;
pub const WMDM_SCP_DRMINFO_V1HEADER: i32 = 1i32;
pub const WMDM_SCP_DRMINFO_V2HEADER: i32 = 2i32;
pub const WMDM_SCP_EXAMINE_DATA: i32 = 2i32;
pub const WMDM_SCP_EXAMINE_EXTENSION: i32 = 1i32;
pub const WMDM_SCP_NO_MORE_CHANGES: i32 = 64i32;
pub const WMDM_SCP_PROTECTED_OUTPUT: i32 = 16i32;
pub const WMDM_SCP_REVOKED: u32 = 8u32;
pub const WMDM_SCP_RIGHTS_DATA: i32 = 64i32;
pub const WMDM_SCP_TRANSFER_OBJECTDATA: i32 = 32i32;
pub const WMDM_SCP_UNPROTECTED_OUTPUT: i32 = 32i32;
pub const WMDM_SEEK_BEGIN: u32 = 1u32;
pub const WMDM_SEEK_CURRENT: u32 = 2u32;
pub const WMDM_SEEK_END: u32 = 8u32;
pub const WMDM_SEEK_REMOTECONTROL: u32 = 1u32;
pub const WMDM_SEEK_STREAMINGAUDIO: u32 = 2u32;
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: windows_core::GUID = windows_core::GUID::from_u128(0x7de8686d_78ee_43ea_a496_c625ac91cc5d);
pub const WMDM_SESSION_CUSTOM: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(4096i32);
pub const WMDM_SESSION_DELETE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(256i32);
pub const WMDM_SESSION_NONE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(0i32);
pub const WMDM_SESSION_TRANSFER_FROM_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(16i32);
pub const WMDM_SESSION_TRANSFER_TO_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDM_SESSION_TYPE(pub i32);
pub const WMDM_SP_REVOKED: u32 = 4u32;
pub const WMDM_STATUS_BUSY: u32 = 2u32;
pub const WMDM_STATUS_DEVICECONTROL_PAUSED: u32 = 32u32;
pub const WMDM_STATUS_DEVICECONTROL_PLAYING: u32 = 8u32;
pub const WMDM_STATUS_DEVICECONTROL_RECORDING: u32 = 16u32;
pub const WMDM_STATUS_DEVICECONTROL_REMOTE: u32 = 64u32;
pub const WMDM_STATUS_DEVICECONTROL_STREAM: u32 = 128u32;
pub const WMDM_STATUS_DEVICE_NOTPRESENT: u32 = 4u32;
pub const WMDM_STATUS_READY: u32 = 1u32;
pub const WMDM_STATUS_STORAGECONTROL_APPENDING: u32 = 32768u32;
pub const WMDM_STATUS_STORAGECONTROL_DELETING: u32 = 16384u32;
pub const WMDM_STATUS_STORAGECONTROL_INSERTING: u32 = 8192u32;
pub const WMDM_STATUS_STORAGECONTROL_MOVING: u32 = 65536u32;
pub const WMDM_STATUS_STORAGECONTROL_READING: u32 = 131072u32;
pub const WMDM_STATUS_STORAGE_BROKEN: u32 = 1024u32;
pub const WMDM_STATUS_STORAGE_INITIALIZING: u32 = 512u32;
pub const WMDM_STATUS_STORAGE_NOTPRESENT: u32 = 256u32;
pub const WMDM_STATUS_STORAGE_NOTSUPPORTED: u32 = 2048u32;
pub const WMDM_STATUS_STORAGE_UNFORMATTED: u32 = 4096u32;
pub const WMDM_STORAGECAP_FILELIMITEXISTS: u32 = 32u32;
pub const WMDM_STORAGECAP_FILESINFOLDERS: u32 = 8u32;
pub const WMDM_STORAGECAP_FILESINROOT: u32 = 2u32;
pub const WMDM_STORAGECAP_FOLDERLIMITEXISTS: u32 = 16u32;
pub const WMDM_STORAGECAP_FOLDERSINFOLDERS: u32 = 4u32;
pub const WMDM_STORAGECAP_FOLDERSINROOT: u32 = 1u32;
pub const WMDM_STORAGECAP_NOT_INITIALIZABLE: u32 = 64u32;
pub const WMDM_STORAGECONTROL_INSERTAFTER: u32 = 1024u32;
pub const WMDM_STORAGECONTROL_INSERTBEFORE: u32 = 512u32;
pub const WMDM_STORAGECONTROL_INSERTINTO: u32 = 2048u32;
pub const WMDM_STORAGE_ATTR_CANEDITMETADATA: u32 = 128u32;
pub const WMDM_STORAGE_ATTR_FILESYSTEM: u32 = 1u32;
pub const WMDM_STORAGE_ATTR_FOLDERS: u32 = 256u32;
pub const WMDM_STORAGE_ATTR_HAS_FILES: u32 = 67108864u32;
pub const WMDM_STORAGE_ATTR_HAS_FOLDERS: u32 = 33554432u32;
pub const WMDM_STORAGE_ATTR_NONREMOVABLE: u32 = 4u32;
pub const WMDM_STORAGE_ATTR_REMOVABLE: u32 = 2u32;
pub const WMDM_STORAGE_ATTR_VIRTUAL: u32 = 536870912u32;
pub const WMDM_STORAGE_CONTAINS_DEFAULT: u32 = 268435456u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDM_STORAGE_ENUM_MODE(pub i32);
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WMDM_TAG_DATATYPE(pub i32);
pub const WMDM_TYPE_BINARY: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(2i32);
pub const WMDM_TYPE_BOOL: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(3i32);
pub const WMDM_TYPE_DATE: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(7i32);
pub const WMDM_TYPE_DWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(0i32);
pub const WMDM_TYPE_GUID: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(6i32);
pub const WMDM_TYPE_QWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(4i32);
pub const WMDM_TYPE_STRING: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(1i32);
pub const WMDM_TYPE_WORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(5i32);
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: windows_core::PWSTR,
    pub dwReserved: u32,
}
pub const g_wszAudioWAVECodec: windows_core::PCWSTR = windows_core::w!("WMDM/AudioWAVECodec");
pub const g_wszVideoFourCCCodec: windows_core::PCWSTR = windows_core::w!("WMDM/VideoFourCCCodec");
pub const g_wszWMDMAlbumArt: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumArt");
pub const g_wszWMDMAlbumArtist: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumArtist");
pub const g_wszWMDMAlbumCoverData: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumCoverData");
pub const g_wszWMDMAlbumCoverDuration: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumCoverDuration");
pub const g_wszWMDMAlbumCoverFormat: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumCoverFormat");
pub const g_wszWMDMAlbumCoverHeight: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumCoverHeight");
pub const g_wszWMDMAlbumCoverSize: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumCoverSize");
pub const g_wszWMDMAlbumCoverWidth: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumCoverWidth");
pub const g_wszWMDMAlbumTitle: windows_core::PCWSTR = windows_core::w!("WMDM/AlbumTitle");
pub const g_wszWMDMAudioBitDepth: windows_core::PCWSTR = windows_core::w!("WMDM/AudioBitDepth");
pub const g_wszWMDMAuthor: windows_core::PCWSTR = windows_core::w!("WMDM/Author");
pub const g_wszWMDMAuthorDate: windows_core::PCWSTR = windows_core::w!("WMDM/AuthorDate");
pub const g_wszWMDMBitRateType: windows_core::PCWSTR = windows_core::w!("WMDM/BitRateType");
pub const g_wszWMDMBitrate: windows_core::PCWSTR = windows_core::w!("WMDM/Bitrate");
pub const g_wszWMDMBlockAlignment: windows_core::PCWSTR = windows_core::w!("WMDM/BlockAlignment");
pub const g_wszWMDMBufferSize: windows_core::PCWSTR = windows_core::w!("WMDM/BufferSize");
pub const g_wszWMDMBuyNow: windows_core::PCWSTR = windows_core::w!("WMDM/BuyNow");
pub const g_wszWMDMByteBookmark: windows_core::PCWSTR = windows_core::w!("WMDM/ByteBookmark");
pub const g_wszWMDMCategory: windows_core::PCWSTR = windows_core::w!("WMDM/Category");
pub const g_wszWMDMCodec: windows_core::PCWSTR = windows_core::w!("WMDM/Codec");
pub const g_wszWMDMCollectionID: windows_core::PCWSTR = windows_core::w!("WMDM/CollectionID");
pub const g_wszWMDMComposer: windows_core::PCWSTR = windows_core::w!("WMDM/Composer");
pub const g_wszWMDMDRMId: windows_core::PCWSTR = windows_core::w!("WMDM/DRMId");
pub const g_wszWMDMDataLength: windows_core::PCWSTR = windows_core::w!("WMDM/DataLength");
pub const g_wszWMDMDataOffset: windows_core::PCWSTR = windows_core::w!("WMDM/DataOffset");
pub const g_wszWMDMDataUnits: windows_core::PCWSTR = windows_core::w!("WMDM/DataUnits");
pub const g_wszWMDMDescription: windows_core::PCWSTR = windows_core::w!("WMDM/Description");
pub const g_wszWMDMDestinationURL: windows_core::PCWSTR = windows_core::w!("WMDM/DestinationURL");
pub const g_wszWMDMDeviceFirmwareVersion: windows_core::PCWSTR = windows_core::w!("WMDM/DeviceFirmwareVersion");
pub const g_wszWMDMDeviceFriendlyName: windows_core::PCWSTR = windows_core::w!("WMDM/DeviceFriendlyName");
pub const g_wszWMDMDeviceModelName: windows_core::PCWSTR = windows_core::w!("WMDM/DeviceModelName");
pub const g_wszWMDMDevicePlayCount: windows_core::PCWSTR = windows_core::w!("WMDM/DevicePlayCount");
pub const g_wszWMDMDeviceProtocol: windows_core::PCWSTR = windows_core::w!("WMDM/DeviceProtocol");
pub const g_wszWMDMDeviceRevocationInfo: windows_core::PCWSTR = windows_core::w!("WMDM/DeviceRevocationInfo");
pub const g_wszWMDMDeviceServiceProviderVendor: windows_core::PCWSTR = windows_core::w!("WMDM/DeviceServiceProviderVendor");
pub const g_wszWMDMDeviceVendorExtension: windows_core::PCWSTR = windows_core::w!("WMDM/DeviceVendorExtension");
pub const g_wszWMDMDuration: windows_core::PCWSTR = windows_core::w!("WMDM/Duration");
pub const g_wszWMDMEditor: windows_core::PCWSTR = windows_core::w!("WMDM/Editor");
pub const g_wszWMDMEncodingProfile: windows_core::PCWSTR = windows_core::w!("WMDM/EncodingProfile");
pub const g_wszWMDMFileAttributes: windows_core::PCWSTR = windows_core::w!("WMDM/FileAttributes");
pub const g_wszWMDMFileCreationDate: windows_core::PCWSTR = windows_core::w!("WMDM/FileCreationDate");
pub const g_wszWMDMFileName: windows_core::PCWSTR = windows_core::w!("WMDM/FileName");
pub const g_wszWMDMFileSize: windows_core::PCWSTR = windows_core::w!("WMDM/FileSize");
pub const g_wszWMDMFormatCode: windows_core::PCWSTR = windows_core::w!("WMDM/FormatCode");
pub const g_wszWMDMFormatsSupported: windows_core::PCWSTR = windows_core::w!("WMDM/FormatsSupported");
pub const g_wszWMDMFormatsSupportedAreOrdered: windows_core::PCWSTR = windows_core::w!("WMDM/FormatsSupportedAreOrdered");
pub const g_wszWMDMFrameRate: windows_core::PCWSTR = windows_core::w!("WMDM/FrameRate");
pub const g_wszWMDMGenre: windows_core::PCWSTR = windows_core::w!("WMDM/Genre");
pub const g_wszWMDMHeight: windows_core::PCWSTR = windows_core::w!("WMDM/Height");
pub const g_wszWMDMIsProtected: windows_core::PCWSTR = windows_core::w!("WMDM/IsProtected");
pub const g_wszWMDMIsRepeat: windows_core::PCWSTR = windows_core::w!("WMDM/IsRepeat");
pub const g_wszWMDMKeyFrameDistance: windows_core::PCWSTR = windows_core::w!("WMDM/KeyFrameDistance");
pub const g_wszWMDMLastModifiedDate: windows_core::PCWSTR = windows_core::w!("WMDM/LastModifiedDate");
pub const g_wszWMDMMediaClassSecondaryID: windows_core::PCWSTR = windows_core::w!("WMDM/MediaClassSecondaryID");
pub const g_wszWMDMMediaCredits: windows_core::PCWSTR = windows_core::w!("WMDM/MediaCredits");
pub const g_wszWMDMMediaGuid: windows_core::PCWSTR = windows_core::w!("WMDM/MediaGuid");
pub const g_wszWMDMMediaOriginalBroadcastDateTime: windows_core::PCWSTR = windows_core::w!("WMDM/MediaOriginalBroadcastDateTime");
pub const g_wszWMDMMediaOriginalChannel: windows_core::PCWSTR = windows_core::w!("WMDM/MediaOriginalChannel");
pub const g_wszWMDMMediaStationName: windows_core::PCWSTR = windows_core::w!("WMDM/MediaStationName");
pub const g_wszWMDMMetaGenre: windows_core::PCWSTR = windows_core::w!("WMDM/MetaGenre");
pub const g_wszWMDMNonConsumable: windows_core::PCWSTR = windows_core::w!("WMDM/NonConsumable");
pub const g_wszWMDMNumChannels: windows_core::PCWSTR = windows_core::w!("WMDM/NumChannels");
pub const g_wszWMDMObjectBookmark: windows_core::PCWSTR = windows_core::w!("WMDM/ObjectBookmark");
pub const g_wszWMDMOwner: windows_core::PCWSTR = windows_core::w!("WMDM/Owner");
pub const g_wszWMDMParentalRating: windows_core::PCWSTR = windows_core::w!("WMDM/ParentalRating");
pub const g_wszWMDMPersistentUniqueID: windows_core::PCWSTR = windows_core::w!("WMDM/PersistentUniqueID");
pub const g_wszWMDMPlayCount: windows_core::PCWSTR = windows_core::w!("WMDM/PlayCount");
pub const g_wszWMDMProviderCopyright: windows_core::PCWSTR = windows_core::w!("WMDM/ProviderCopyright");
pub const g_wszWMDMQualitySetting: windows_core::PCWSTR = windows_core::w!("WMDM/QualitySetting");
pub const g_wszWMDMSampleRate: windows_core::PCWSTR = windows_core::w!("WMDM/SampleRate");
pub const g_wszWMDMScanType: windows_core::PCWSTR = windows_core::w!("WMDM/ScanType");
pub const g_wszWMDMSourceURL: windows_core::PCWSTR = windows_core::w!("WMDM/SourceURL");
pub const g_wszWMDMSubTitle: windows_core::PCWSTR = windows_core::w!("WMDM/SubTitle");
pub const g_wszWMDMSubTitleDescription: windows_core::PCWSTR = windows_core::w!("WMDM/SubTitleDescription");
pub const g_wszWMDMSupportedDeviceProperties: windows_core::PCWSTR = windows_core::w!("WMDM/SupportedDeviceProperties");
pub const g_wszWMDMSyncID: windows_core::PCWSTR = windows_core::w!("WMDM/SyncID");
pub const g_wszWMDMSyncRelationshipID: windows_core::PCWSTR = windows_core::w!("WMDM/SyncRelationshipID");
pub const g_wszWMDMSyncTime: windows_core::PCWSTR = windows_core::w!("WMDM/SyncTime");
pub const g_wszWMDMTimeBookmark: windows_core::PCWSTR = windows_core::w!("WMDM/TimeBookmark");
pub const g_wszWMDMTimeToLive: windows_core::PCWSTR = windows_core::w!("WMDM/TimeToLive");
pub const g_wszWMDMTitle: windows_core::PCWSTR = windows_core::w!("WMDM/Title");
pub const g_wszWMDMTotalBitrate: windows_core::PCWSTR = windows_core::w!("WMDM/TotalBitrate");
pub const g_wszWMDMTrack: windows_core::PCWSTR = windows_core::w!("WMDM/Track");
pub const g_wszWMDMTrackMood: windows_core::PCWSTR = windows_core::w!("WMDM/TrackMood");
pub const g_wszWMDMUserEffectiveRating: windows_core::PCWSTR = windows_core::w!("WMDM/UserEffectiveRating");
pub const g_wszWMDMUserLastPlayTime: windows_core::PCWSTR = windows_core::w!("WMDM/UserLastPlayTime");
pub const g_wszWMDMUserRating: windows_core::PCWSTR = windows_core::w!("WMDM/UserRating");
pub const g_wszWMDMUserRatingOnDevice: windows_core::PCWSTR = windows_core::w!("WMDM/UserRatingOnDevice");
pub const g_wszWMDMVideoBitrate: windows_core::PCWSTR = windows_core::w!("WMDM/VideoBitrate");
pub const g_wszWMDMWebmaster: windows_core::PCWSTR = windows_core::w!("WMDM/Webmaster");
pub const g_wszWMDMWidth: windows_core::PCWSTR = windows_core::w!("WMDM/Width");
pub const g_wszWMDMYear: windows_core::PCWSTR = windows_core::w!("WMDM/Year");
pub const g_wszWMDMediaClassPrimaryID: windows_core::PCWSTR = windows_core::w!("WMDM/MediaClassPrimaryID");
pub const g_wszWPDPassthroughPropertyValues: windows_core::PCWSTR = windows_core::w!("WPD/PassthroughPropertyValues");
