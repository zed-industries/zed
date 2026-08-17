windows_core::imp::define_interface!(IComponentAuthenticate, IComponentAuthenticate_Vtbl, 0xa9889c00_6d2b_11d3_8496_00c04f79dbc0);
impl core::ops::Deref for IComponentAuthenticate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComponentAuthenticate, windows_core::IUnknown);
impl IComponentAuthenticate {
    pub unsafe fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: &[u8], ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SACAuth)(windows_core::Interface::as_raw(self), dwprotocolid, dwpass, core::mem::transmute(pbdatain.as_ptr()), pbdatain.len().try_into().unwrap(), ppbdataout, pdwdataoutlen).ok()
    }
    pub unsafe fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SACGetProtocols)(windows_core::Interface::as_raw(self), ppdwprotocols, pdwprotocolcount).ok()
    }
}
#[repr(C)]
pub struct IComponentAuthenticate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SACAuth: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SACGetProtocols: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPDevice, IMDSPDevice_Vtbl, 0x1dcb3a12_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPDevice, windows_core::IUnknown);
impl IMDSPDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetManufacturer)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnumber, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPowerSource)(windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceIcon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormatSupport)(windows_core::Interface::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormatSupport2)(windows_core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSpecifyPropertyPages)(windows_core::Interface::as_raw(self), core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCanonicalName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IMDSPDevice2_Vtbl {
    pub base__: IMDSPDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::Audio::WAVEFORMATEX, *mut u32, *mut *mut super::MediaFoundation::VIDEOINFOHEADER, *mut u32, *mut *mut WMFILECAPABILITIES, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut Option<windows_core::IUnknown>, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPDevice3, IMDSPDevice3_Vtbl, 0x1a839845_fc55_487c_976f_ee38ac0e8c4e);
impl core::ops::Deref for IMDSPDevice3 {
    type Target = IMDSPDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPDevice3, windows_core::IUnknown, IMDSPDevice, IMDSPDevice2);
impl IMDSPDevice3 {
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> windows_core::Result<windows_core::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormatCapability)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeviceIoControl)(windows_core::Interface::as_raw(self), dwiocontrolcode, core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len().try_into().unwrap(), lpoutbuffer, pnoutbuffersize).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMDSPDevice3_Vtbl {
    pub base__: IMDSPDevice2_Vtbl,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetFormatCapability: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FORMATCODE, *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT,
    pub DeviceIoControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPDeviceControl, IMDSPDeviceControl_Vtbl, 0x1dcb3a14_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPDeviceControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPDeviceControl, windows_core::IUnknown);
impl IMDSPDeviceControl {
    pub unsafe fn GetDCStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDCStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Play(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Record)(windows_core::Interface::as_raw(self), pformat).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), fumode, noffset).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMDSPDirectTransfer, IMDSPDirectTransfer_Vtbl, 0xc2fe57a8_9304_478c_9ee4_47e397b912d7);
impl core::ops::Deref for IMDSPDirectTransfer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPDirectTransfer, windows_core::IUnknown);
impl IMDSPDirectTransfer {
    pub unsafe fn TransferToDevice<P0, P1, P2, P3, P4>(&self, pwszsourcefilepath: P0, psourceoperation: P1, fuflags: u32, pwszdestinationname: P2, psourcemetadata: P3, ptransferprogress: P4) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMOperation>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IWMDMMetaData>,
        P4: windows_core::Param<IWMDMProgress>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransferToDevice)(windows_core::Interface::as_raw(self), pwszsourcefilepath.param().abi(), psourceoperation.param().abi(), fuflags, pwszdestinationname.param().abi(), psourcemetadata.param().abi(), ptransferprogress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMDSPDirectTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransferToDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPEnumDevice, IMDSPEnumDevice_Vtbl, 0x1dcb3a11_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPEnumDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPEnumDevice, windows_core::IUnknown);
impl IMDSPEnumDevice {
    pub unsafe fn Next(&self, ppdevice: &mut [Option<IMDSPDevice>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppdevice.len().try_into().unwrap(), core::mem::transmute(ppdevice.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IMDSPEnumDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMDSPEnumDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPEnumStorage, IMDSPEnumStorage_Vtbl, 0x1dcb3a15_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPEnumStorage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPEnumStorage, windows_core::IUnknown);
impl IMDSPEnumStorage {
    pub unsafe fn Next(&self, ppstorage: &mut [Option<IMDSPStorage>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppstorage.len().try_into().unwrap(), core::mem::transmute(ppstorage.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMDSPEnumStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPObject, IMDSPObject_Vtbl, 0x1dcb3a18_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPObject, windows_core::IUnknown);
impl IMDSPObject {
    pub unsafe fn Open(&self, fumode: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), fumode).ok()
    }
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pdata, pdwsize, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pdata, pdwsize, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok()
    }
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), fuflags, dwoffset).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, pwsznewname: P0, pprogress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMProgress>,
    {
        (windows_core::Interface::vtable(self).Rename)(windows_core::Interface::as_raw(self), pwsznewname.param().abi(), pprogress.param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, pprogress: P0, ptarget: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress>,
        P1: windows_core::Param<IMDSPStorage>,
    {
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi(), ptarget.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).ReadOnClearChannel)(windows_core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
    pub unsafe fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteOnClearChannel)(windows_core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
}
#[repr(C)]
pub struct IMDSPObject2_Vtbl {
    pub base__: IMDSPObject_Vtbl,
    pub ReadOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub WriteOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPObjectInfo, IMDSPObjectInfo_Vtbl, 0x1dcb3a19_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPObjectInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPObjectInfo, windows_core::IUnknown);
impl IMDSPObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPlayLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPlayLength)(windows_core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetPlayOffset(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPlayOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPlayOffset)(windows_core::Interface::as_raw(self), dwoffset).ok()
    }
    pub unsafe fn GetTotalLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTotalLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLastPlayPosition(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLongestPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMDSPRevoked, IMDSPRevoked_Vtbl, 0xa4e8f2d4_3f31_464d_b53d_4fc335998184);
impl core::ops::Deref for IMDSPRevoked {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPRevoked, windows_core::IUnknown);
impl IMDSPRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRevocationURL)(windows_core::Interface::as_raw(self), ppwszrevocationurl, pdwbufferlen).ok()
    }
}
#[repr(C)]
pub struct IMDSPRevoked_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPStorage, IMDSPStorage_Vtbl, 0x1dcb3a16_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPStorage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPStorage, windows_core::IUnknown);
impl IMDSPStorage {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttributes)(windows_core::Interface::as_raw(self), dwattributes, core::mem::transmute(pformat.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> windows_core::Result<IMDSPStorageGlobals> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorageGlobals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: Option<*mut super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), pdwattributes, core::mem::transmute(pformat.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> windows_core::Result<WMDMDATETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRights)(windows_core::Interface::as_raw(self), pprights, pnrightscount, core::mem::transmute(abmac.as_ptr())).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStorage)(windows_core::Interface::as_raw(self), dwattributes, core::mem::transmute(pformat.unwrap_or(std::ptr::null())), pwszname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStorage2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, core::mem::transmute(paudioformat.unwrap_or(std::ptr::null())), core::mem::transmute(pvideoformat.unwrap_or(std::ptr::null())), pwszname.param().abi(), qwfilesize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttributes2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, core::mem::transmute(paudioformat.unwrap_or(std::ptr::null())), core::mem::transmute(pvideoformat.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributes2)(windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, core::mem::transmute(paudioformat.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvideoformat.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).GetMetadata)(windows_core::Interface::as_raw(self), pmetadata.param().abi()).ok()
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMMetaData>,
    {
        (windows_core::Interface::vtable(self).SetMetadata)(windows_core::Interface::as_raw(self), pmetadata.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMDSPStorage3_Vtbl {
    pub base__: IMDSPStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetReferences)(windows_core::Interface::as_raw(self), ppispstorage.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppispstorage.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut Option<IMDSPStorage>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReferences)(windows_core::Interface::as_raw(self), pdwrefs, pppispstorage).ok()
    }
    pub unsafe fn CreateStorageWithMetadata<P0, P1>(&self, dwattributes: u32, pwszname: P0, pmetadata: P1, qwfilesize: u64) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMMetaData>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStorageWithMetadata)(windows_core::Interface::as_raw(self), dwattributes, pwszname.param().abi(), pmetadata.param().abi(), qwfilesize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSpecifiedMetadata<P0>(&self, ppwszpropnames: &[windows_core::PCWSTR], pmetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMMetaData>,
    {
        (windows_core::Interface::vtable(self).GetSpecifiedMetadata)(windows_core::Interface::as_raw(self), ppwszpropnames.len().try_into().unwrap(), core::mem::transmute(ppwszpropnames.as_ptr()), pmetadata.param().abi()).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> windows_core::Result<IMDSPStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<IMDSPStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMDSPStorage4_Vtbl {
    pub base__: IMDSPStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Option<IMDSPStorage>) -> windows_core::HRESULT,
    pub CreateStorageWithMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMDSPStorageGlobals, IMDSPStorageGlobals_Vtbl, 0x1dcb3a17_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDSPStorageGlobals {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDSPStorageGlobals, windows_core::IUnknown);
impl IMDSPStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnum, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTotalSize)(windows_core::Interface::as_raw(self), pdwtotalsizelow, pdwtotalsizehigh).ok()
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTotalFree)(windows_core::Interface::as_raw(self), pdwfreelow, pdwfreehigh).ok()
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTotalBad)(windows_core::Interface::as_raw(self), pdwbadlow, pdwbadhigh).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Initialize<P0>(&self, fumode: u32, pprogress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok()
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IMDSPDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRootStorage(&self) -> windows_core::Result<IMDSPStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRootStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMDServiceProvider, IMDServiceProvider_Vtbl, 0x1dcb3a10_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IMDServiceProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMDServiceProvider, windows_core::IUnknown);
impl IMDServiceProvider {
    pub unsafe fn GetDeviceCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumDevices(&self) -> windows_core::Result<IMDSPEnumDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMDServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), pwszdevicepath.param().abi(), pdwcount, pppdevicearray).ok()
    }
}
#[repr(C)]
pub struct IMDServiceProvider2_Vtbl {
    pub base__: IMDServiceProvider_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut Option<IMDSPDevice>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetDeviceEnumPreference)(windows_core::Interface::as_raw(self), dwenumpref).ok()
    }
}
#[repr(C)]
pub struct IMDServiceProvider3_Vtbl {
    pub base__: IMDServiceProvider2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSecureAuthenticate, ISCPSecureAuthenticate_Vtbl, 0x1dcb3a0f_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for ISCPSecureAuthenticate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureAuthenticate, windows_core::IUnknown);
impl ISCPSecureAuthenticate {
    pub unsafe fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecureQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISCPSecureAuthenticate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSecureQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSCPSession)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISCPSecureAuthenticate2_Vtbl {
    pub base__: ISCPSecureAuthenticate_Vtbl,
    pub GetSCPSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSecureExchange, ISCPSecureExchange_Vtbl, 0x1dcb3a0e_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for ISCPSecureExchange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureExchange, windows_core::IUnknown);
impl ISCPSecureExchange {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransferContainerData)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pfureadyflags, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ObjectData)(windows_core::Interface::as_raw(self), pdata, pdwsize, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn TransferComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransferComplete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISCPSecureExchange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransferContainerData: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub ObjectData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub TransferComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSecureExchange2, ISCPSecureExchange2_Vtbl, 0x6c62fc7b_2690_483f_9d44_0a20cb35577c);
impl core::ops::Deref for ISCPSecureExchange2 {
    type Target = ISCPSecureExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureExchange2, windows_core::IUnknown, ISCPSecureExchange);
impl ISCPSecureExchange2 {
    pub unsafe fn TransferContainerData2<P0>(&self, pdata: &[u8], pprogresscallback: P0, pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress3>,
    {
        (windows_core::Interface::vtable(self).TransferContainerData2)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pprogresscallback.param().abi(), pfureadyflags, core::mem::transmute(abmac.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ISCPSecureExchange2_Vtbl {
    pub base__: ISCPSecureExchange_Vtbl,
    pub TransferContainerData2: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut core::ffi::c_void, *mut u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSecureExchange3, ISCPSecureExchange3_Vtbl, 0xab4e77e4_8908_4b17_bd2a_b1dbe6dd69e1);
impl core::ops::Deref for ISCPSecureExchange3 {
    type Target = ISCPSecureExchange2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureExchange3, windows_core::IUnknown, ISCPSecureExchange, ISCPSecureExchange2);
impl ISCPSecureExchange3 {
    pub unsafe fn TransferContainerDataOnClearChannel<P0, P1>(&self, pdevice: P0, pdata: &[u8], pprogresscallback: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IMDSPDevice>,
        P1: windows_core::Param<IWMDMProgress3>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TransferContainerDataOnClearChannel)(windows_core::Interface::as_raw(self), pdevice.param().abi(), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pprogresscallback.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetObjectDataOnClearChannel<P0>(&self, pdevice: P0, pdata: *mut u8, pdwsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPDevice>,
    {
        (windows_core::Interface::vtable(self).GetObjectDataOnClearChannel)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdata, pdwsize).ok()
    }
    pub unsafe fn TransferCompleteForDevice<P0>(&self, pdevice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPDevice>,
    {
        (windows_core::Interface::vtable(self).TransferCompleteForDevice)(windows_core::Interface::as_raw(self), pdevice.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISCPSecureExchange3_Vtbl {
    pub base__: ISCPSecureExchange2_Vtbl,
    pub TransferContainerDataOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetObjectDataOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub TransferCompleteForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSecureQuery, ISCPSecureQuery_Vtbl, 0x1dcb3a0d_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for ISCPSecureQuery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureQuery, windows_core::IUnknown);
impl ISCPSecureQuery {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDataDemands)(windows_core::Interface::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ExamineData)(windows_core::Interface::as_raw(self), fuflags, pwszextension.param().abi(), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPStorageGlobals>,
    {
        (windows_core::Interface::vtable(self).MakeDecision)(windows_core::Interface::as_raw(self), fuflags, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), dwappsec, core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstorageglobals.param().abi(), core::mem::transmute(ppexchange), core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPStorageGlobals>,
    {
        (windows_core::Interface::vtable(self).GetRights)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.param().abi(), pprights, pnrightscount, core::mem::transmute(abmac.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ISCPSecureQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataDemands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub ExamineData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    pub MakeDecision: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub GetRights: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSecureQuery2, ISCPSecureQuery2_Vtbl, 0xebe17e25_4fd7_4632_af46_6d93d4fcc72e);
impl core::ops::Deref for ISCPSecureQuery2 {
    type Target = ISCPSecureQuery;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureQuery2, windows_core::IUnknown, ISCPSecureQuery);
impl ISCPSecureQuery2 {
    pub unsafe fn MakeDecision2<P0, P1>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: Option<*mut u64>, punknown: P1, ppexchange: *mut Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPStorageGlobals>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
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
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            core::mem::transmute(pqwfilesize.unwrap_or(std::ptr::null_mut())),
            punknown.param().abi(),
            core::mem::transmute(ppexchange),
            core::mem::transmute(abmac.as_ptr()),
        )
        .ok()
    }
}
#[repr(C)]
pub struct ISCPSecureQuery2_Vtbl {
    pub base__: ISCPSecureQuery_Vtbl,
    pub MakeDecision2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8, u32, *mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut windows_core::PWSTR, *mut u32, *mut u32, *mut u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSecureQuery3, ISCPSecureQuery3_Vtbl, 0xb7edd1a2_4dab_484b_b3c5_ad39b8b4c0b1);
impl core::ops::Deref for ISCPSecureQuery3 {
    type Target = ISCPSecureQuery2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSecureQuery3, windows_core::IUnknown, ISCPSecureQuery, ISCPSecureQuery2);
impl ISCPSecureQuery3 {
    pub unsafe fn GetRightsOnClearChannel<P0, P1>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprogresscallback: P1, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPStorageGlobals>,
        P1: windows_core::Param<IWMDMProgress3>,
    {
        (windows_core::Interface::vtable(self).GetRightsOnClearChannel)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.param().abi(), pprogresscallback.param().abi(), pprights, pnrightscount).ok()
    }
    pub unsafe fn MakeDecisionOnClearChannel<P0, P1, P2>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pprogresscallback: P1, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: Option<*mut u64>, punknown: P2, ppexchange: *mut Option<ISCPSecureExchange>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPStorageGlobals>,
        P1: windows_core::Param<IWMDMProgress3>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
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
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            core::mem::transmute(pqwfilesize.unwrap_or(std::ptr::null_mut())),
            punknown.param().abi(),
            core::mem::transmute(ppexchange),
        )
        .ok()
    }
}
#[repr(C)]
pub struct ISCPSecureQuery3_Vtbl {
    pub base__: ISCPSecureQuery2_Vtbl,
    pub GetRightsOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32) -> windows_core::HRESULT,
    pub MakeDecisionOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut windows_core::PWSTR, *mut u32, *mut u32, *mut u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISCPSession, ISCPSession_Vtbl, 0x88a3e6ed_eee4_4619_bbb3_fd4fb62715d1);
impl core::ops::Deref for ISCPSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISCPSession, windows_core::IUnknown);
impl ISCPSession {
    pub unsafe fn BeginSession<P0>(&self, pidevice: P0, pctx: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMDSPDevice>,
    {
        (windows_core::Interface::vtable(self).BeginSession)(windows_core::Interface::as_raw(self), pidevice.param().abi(), core::mem::transmute(pctx.as_ptr()), pctx.len().try_into().unwrap()).ok()
    }
    pub unsafe fn EndSession(&self, pctx: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self), core::mem::transmute(pctx.as_ptr()), pctx.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetSecureQuery(&self) -> windows_core::Result<ISCPSecureQuery> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecureQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISCPSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub GetSecureQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMDevice, IWMDMDevice_Vtbl, 0x1dcb3a02_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMDevice, windows_core::IUnknown);
impl IWMDMDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetManufacturer)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnumber, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPowerSource)(windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceIcon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut windows_core::PWSTR, pnmimetypecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormatSupport)(windows_core::Interface::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormatSupport2)(windows_core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut Option<windows_core::IUnknown>, pcunks: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSpecifyPropertyPages)(windows_core::Interface::as_raw(self), core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCanonicalName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IWMDMDevice2_Vtbl {
    pub base__: IWMDMDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::Audio::WAVEFORMATEX, *mut u32, *mut *mut super::MediaFoundation::VIDEOINFOHEADER, *mut u32, *mut *mut WMFILECAPABILITIES, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut Option<windows_core::IUnknown>, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMDevice3, IWMDMDevice3_Vtbl, 0x6c03e4fe_05db_4dda_9e3c_06233a6d5d65);
impl core::ops::Deref for IWMDMDevice3 {
    type Target = IWMDMDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMDevice3, windows_core::IUnknown, IWMDMDevice, IWMDMDevice2);
impl IWMDMDevice3 {
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> windows_core::Result<windows_core::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pwszpropname.param().abi(), core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> windows_core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormatCapability)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeviceIoControl)(windows_core::Interface::as_raw(self), dwiocontrolcode, core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len().try_into().unwrap(), lpoutbuffer, pnoutbuffersize).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> windows_core::Result<IWMDMStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMDMDevice3_Vtbl {
    pub base__: IWMDMDevice2_Vtbl,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub GetFormatCapability: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FORMATCODE, *mut WMDM_FORMAT_CAPABILITY) -> windows_core::HRESULT,
    pub DeviceIoControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMDeviceControl, IWMDMDeviceControl_Vtbl, 0x1dcb3a04_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMDeviceControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMDeviceControl, windows_core::IUnknown);
impl IWMDMDeviceControl {
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Play(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Record)(windows_core::Interface::as_raw(self), pformat).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), fumode, noffset).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IWMDMDeviceSession, IWMDMDeviceSession_Vtbl, 0x82af0a65_9d96_412c_83e5_3c43e4b06cc7);
impl core::ops::Deref for IWMDMDeviceSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMDeviceSession, windows_core::IUnknown);
impl IWMDMDeviceSession {
    pub unsafe fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: Option<&[u8]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginSession)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pctx.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: Option<&[u8]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(pctx.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
}
#[repr(C)]
pub struct IWMDMDeviceSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_SESSION_TYPE, *const u8, u32) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_SESSION_TYPE, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMEnumDevice, IWMDMEnumDevice_Vtbl, 0x1dcb3a01_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMEnumDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMEnumDevice, windows_core::IUnknown);
impl IWMDMEnumDevice {
    pub unsafe fn Next(&self, ppdevice: &mut [Option<IWMDMDevice>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppdevice.len().try_into().unwrap(), core::mem::transmute(ppdevice.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMDMEnumDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMEnumStorage, IWMDMEnumStorage_Vtbl, 0x1dcb3a05_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMEnumStorage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMEnumStorage, windows_core::IUnknown);
impl IWMDMEnumStorage {
    pub unsafe fn Next(&self, ppstorage: &mut [Option<IWMDMStorage>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppstorage.len().try_into().unwrap(), core::mem::transmute(ppstorage.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt, &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMDMEnumStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMLogger, IWMDMLogger_Vtbl, 0x110a3200_5a79_11d3_8d78_444553540000);
impl core::ops::Deref for IWMDMLogger {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMLogger, windows_core::IUnknown);
impl IWMDMLogger {
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    pub unsafe fn GetLogFileName(&self, pszfilename: windows_core::PSTR, nmaxchars: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLogFileName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszfilename), nmaxchars).ok()
    }
    pub unsafe fn SetLogFileName<P0>(&self, pszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).SetLogFileName)(windows_core::Interface::as_raw(self), pszfilename.param().abi()).ok()
    }
    pub unsafe fn LogString<P0, P1>(&self, dwflags: u32, pszsrcname: P0, pszlog: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).LogString)(windows_core::Interface::as_raw(self), dwflags, pszsrcname.param().abi(), pszlog.param().abi()).ok()
    }
    pub unsafe fn LogDword<P0, P1>(&self, dwflags: u32, pszsrcname: P0, pszlogformat: P1, dwlog: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).LogDword)(windows_core::Interface::as_raw(self), dwflags, pszsrcname.param().abi(), pszlogformat.param().abi(), dwlog).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSizeParams)(windows_core::Interface::as_raw(self), pdwmaxsize, pdwshrinktosize).ok()
    }
    pub unsafe fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSizeParams)(windows_core::Interface::as_raw(self), dwmaxsize, dwshrinktosize).ok()
    }
}
#[repr(C)]
pub struct IWMDMLogger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, u32) -> windows_core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> windows_core::HRESULT,
    pub LogString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCSTR, windows_core::PCSTR) -> windows_core::HRESULT,
    pub LogDword: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCSTR, windows_core::PCSTR, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSizeParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetSizeParams: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMMetaData, IWMDMMetaData_Vtbl, 0xec3b0663_0951_460a_9a80_0dceed3c043c);
impl core::ops::Deref for IWMDMMetaData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMMetaData, windows_core::IUnknown);
impl IWMDMMetaData {
    pub unsafe fn AddItem<P0>(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: P0, pvalue: Option<&[u8]>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), r#type, pwsztagname.param().abi(), core::mem::transmute(pvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn QueryByName<P0>(&self, pwsztagname: P0, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).QueryByName)(windows_core::Interface::as_raw(self), pwsztagname.param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryByIndex)(windows_core::Interface::as_raw(self), iindex, ppwszname, ptype, ppvalue, pcblength).ok()
    }
    pub unsafe fn GetItemCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWMDMMetaData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_TAG_DATATYPE, windows_core::PCWSTR, *const u8, u32) -> windows_core::HRESULT,
    pub QueryByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WMDM_TAG_DATATYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub QueryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut WMDM_TAG_DATATYPE, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMNotification, IWMDMNotification_Vtbl, 0x3f5e95c0_0f43_4ed4_93d2_c89a45d59b81);
impl core::ops::Deref for IWMDMNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMNotification, windows_core::IUnknown);
impl IWMDMNotification {
    pub unsafe fn WMDMMessage<P0>(&self, dwmessagetype: u32, pwszcanonicalname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WMDMMessage)(windows_core::Interface::as_raw(self), dwmessagetype, pwszcanonicalname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMDMNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WMDMMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMObjectInfo, IWMDMObjectInfo_Vtbl, 0x1dcb3a09_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMObjectInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMObjectInfo, windows_core::IUnknown);
impl IWMDMObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPlayLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPlayLength)(windows_core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetPlayOffset(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPlayOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPlayOffset)(windows_core::Interface::as_raw(self), dwoffset).ok()
    }
    pub unsafe fn GetTotalLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTotalLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLastPlayPosition(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLongestPlayPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IWMDMOperation, IWMDMOperation_Vtbl, 0x1dcb3a0b_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMOperation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMOperation, windows_core::IUnknown);
impl IWMDMOperation {
    pub unsafe fn BeginRead(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginRead)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginWrite)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: Option<*mut super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectAttributes)(windows_core::Interface::as_raw(self), pdwattributes, core::mem::transmute(pformat.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetObjectAttributes)(windows_core::Interface::as_raw(self), dwattributes, core::mem::transmute(pformat.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectTotalSize)(windows_core::Interface::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetObjectTotalSize)(windows_core::Interface::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransferObjectData)(windows_core::Interface::as_raw(self), pdata, pdwsize, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const windows_core::HRESULT, pnewobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), phcompletioncode, pnewobject.param().abi()).ok()
    }
}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).SetObjectAttributes2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, core::mem::transmute(pformat.unwrap_or(std::ptr::null())), core::mem::transmute(pvideoformat.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectAttributes2)(windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, core::mem::transmute(paudioformat.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvideoformat.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).TransferObjectDataOnClearChannel)(windows_core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
}
#[repr(C)]
pub struct IWMDMOperation3_Vtbl {
    pub base__: IWMDMOperation_Vtbl,
    pub TransferObjectDataOnClearChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMProgress, IWMDMProgress_Vtbl, 0x1dcb3a0c_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMProgress {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMProgress, windows_core::IUnknown);
impl IWMDMProgress {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMDMProgress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).End2)(windows_core::Interface::as_raw(self), hrcompletioncode).ok()
    }
}
#[repr(C)]
pub struct IWMDMProgress2_Vtbl {
    pub base__: IWMDMProgress_Vtbl,
    pub End2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Begin3)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid), dwestimatedticks, core::mem::transmute(pcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Progress3(&self, eventid: windows_core::GUID, dwtranspiredticks: u32, pcontext: Option<*mut OPAQUECOMMAND>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Progress3)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid), dwtranspiredticks, core::mem::transmute(pcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn End3(&self, eventid: windows_core::GUID, hrcompletioncode: windows_core::HRESULT, pcontext: Option<*mut OPAQUECOMMAND>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).End3)(windows_core::Interface::as_raw(self), core::mem::transmute(eventid), hrcompletioncode, core::mem::transmute(pcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IWMDMProgress3_Vtbl {
    pub base__: IWMDMProgress2_Vtbl,
    pub Begin3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
    pub Progress3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
    pub End3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::HRESULT, *mut OPAQUECOMMAND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMRevoked, IWMDMRevoked_Vtbl, 0xebeccedb_88ee_4e55_b6a4_8d9f07d696aa);
impl core::ops::Deref for IWMDMRevoked {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMRevoked, windows_core::IUnknown);
impl IWMDMRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRevocationURL)(windows_core::Interface::as_raw(self), ppwszrevocationurl, pdwbufferlen, pdwrevokedbitflag).ok()
    }
}
#[repr(C)]
pub struct IWMDMRevoked_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMStorage, IWMDMStorage_Vtbl, 0x1dcb3a06_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMStorage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorage, windows_core::IUnknown);
impl IWMDMStorage {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttributes)(windows_core::Interface::as_raw(self), dwattributes, core::mem::transmute(pformat.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> windows_core::Result<IWMDMStorageGlobals> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorageGlobals)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: Option<*mut super::Audio::WAVEFORMATEX>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), pdwattributes, core::mem::transmute(pformat.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> windows_core::Result<WMDMDATETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRights)(windows_core::Interface::as_raw(self), pprights, pnrightscount, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn EnumStorage(&self) -> windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumStorage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendOpaqueCommand)(windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorage)(windows_core::Interface::as_raw(self), pszstoragename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttributes2)(windows_core::Interface::as_raw(self), dwattributes, dwattributesex, core::mem::transmute(pformat.unwrap_or(std::ptr::null())), core::mem::transmute(pvideoformat.unwrap_or(std::ptr::null()))).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttributes2)(windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, core::mem::transmute(paudioformat.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvideoformat.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMMetaData>,
    {
        (windows_core::Interface::vtable(self).SetMetadata)(windows_core::Interface::as_raw(self), pmetadata.param().abi()).ok()
    }
    pub unsafe fn CreateEmptyMetadataObject(&self) -> windows_core::Result<IWMDMMetaData> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEmptyMetadataObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: Option<&[WMDMMetadataView]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnumPreference)(windows_core::Interface::as_raw(self), pmode, pviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pviews.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
#[repr(C)]
pub struct IWMDMStorage3_Vtbl {
    pub base__: IWMDMStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEmptyMetadataObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEnumPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMDM_STORAGE_ENUM_MODE, u32, *const WMDMMetadataView) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetReferences)(windows_core::Interface::as_raw(self), ppiwmdmstorage.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppiwmdmstorage.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut Option<IWMDMStorage>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReferences)(windows_core::Interface::as_raw(self), pdwrefs, pppiwmdmstorage).ok()
    }
    pub unsafe fn GetRightsWithProgress<P0>(&self, piprogresscallback: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress3>,
    {
        (windows_core::Interface::vtable(self).GetRightsWithProgress)(windows_core::Interface::as_raw(self), piprogresscallback.param().abi(), pprights, pnrightscount).ok()
    }
    pub unsafe fn GetSpecifiedMetadata(&self, ppwszpropnames: &[windows_core::PCWSTR]) -> windows_core::Result<IWMDMMetaData> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpecifiedMetadata)(windows_core::Interface::as_raw(self), ppwszpropnames.len().try_into().unwrap(), core::mem::transmute(ppwszpropnames.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> windows_core::Result<IWMDMStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindStorage)(windows_core::Interface::as_raw(self), findscope, pwszuniqueid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<IWMDMStorage> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMDMStorage4_Vtbl {
    pub base__: IWMDMStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Option<IWMDMStorage>) -> windows_core::HRESULT,
    pub GetRightsWithProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut WMDMRIGHTS, *mut u32) -> windows_core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(*mut core::ffi::c_void, WMDM_FIND_SCOPE, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMStorageControl, IWMDMStorageControl_Vtbl, 0x1dcb3a08_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMStorageControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorageControl, windows_core::IUnknown);
impl IWMDMStorageControl {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> windows_core::Result<IWMDMStorage>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMOperation>,
        P2: windows_core::Param<IWMDMProgress>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), fumode, pwszfile.param().abi(), poperation.param().abi(), pprogress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMProgress>,
    {
        (windows_core::Interface::vtable(self).Rename)(windows_core::Interface::as_raw(self), fumode, pwsznewname.param().abi(), pprogress.param().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWMDMProgress>,
        P2: windows_core::Param<IWMDMOperation>,
    {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), fumode, pwszfile.param().abi(), pprogress.param().abi(), poperation.param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMStorage>,
        P1: windows_core::Param<IWMDMProgress>,
    {
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), fumode, ptargetobject.param().abi(), pprogress.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWMDMStorageControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMStorageControl2, IWMDMStorageControl2_Vtbl, 0x972c2e88_bd6c_4125_8e09_84f837e637b6);
impl core::ops::Deref for IWMDMStorageControl2 {
    type Target = IWMDMStorageControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorageControl2, windows_core::IUnknown, IWMDMStorageControl);
impl IWMDMStorageControl2 {
    pub unsafe fn Insert2<P0, P1, P2, P3, P4>(&self, fumode: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, punknown: P4, ppnewobject: Option<*mut Option<IWMDMStorage>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWMDMOperation>,
        P3: windows_core::Param<IWMDMProgress>,
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Insert2)(windows_core::Interface::as_raw(self), fumode, pwszfilesource.param().abi(), pwszfiledest.param().abi(), poperation.param().abi(), pprogress.param().abi(), punknown.param().abi(), core::mem::transmute(ppnewobject.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IWMDMStorageControl2_Vtbl {
    pub base__: IWMDMStorageControl_Vtbl,
    pub Insert2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMStorageControl3, IWMDMStorageControl3_Vtbl, 0xb3266365_d4f3_4696_8d53_bd27ec60993a);
impl core::ops::Deref for IWMDMStorageControl3 {
    type Target = IWMDMStorageControl2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorageControl3, windows_core::IUnknown, IWMDMStorageControl, IWMDMStorageControl2);
impl IWMDMStorageControl3 {
    pub unsafe fn Insert3<P0, P1, P2, P3, P4, P5>(&self, fumode: u32, futype: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, pmetadata: P4, punknown: P5, ppnewobject: Option<*mut Option<IWMDMStorage>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IWMDMOperation>,
        P3: windows_core::Param<IWMDMProgress>,
        P4: windows_core::Param<IWMDMMetaData>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Insert3)(windows_core::Interface::as_raw(self), fumode, futype, pwszfilesource.param().abi(), pwszfiledest.param().abi(), poperation.param().abi(), pprogress.param().abi(), pmetadata.param().abi(), punknown.param().abi(), core::mem::transmute(ppnewobject.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IWMDMStorageControl3_Vtbl {
    pub base__: IWMDMStorageControl2_Vtbl,
    pub Insert3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWMDMStorageGlobals, IWMDMStorageGlobals_Vtbl, 0x1dcb3a07_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDMStorageGlobals {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDMStorageGlobals, windows_core::IUnknown);
impl IWMDMStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: &mut [u8; 8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSerialNumber)(windows_core::Interface::as_raw(self), pserialnum, core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTotalSize)(windows_core::Interface::as_raw(self), pdwtotalsizelow, pdwtotalsizehigh).ok()
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTotalFree)(windows_core::Interface::as_raw(self), pdwfreelow, pdwfreehigh).ok()
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTotalBad)(windows_core::Interface::as_raw(self), pdwbadlow, pdwbadhigh).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Initialize<P0>(&self, fumode: u32, pprogress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWMDMProgress>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), fumode, pprogress.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IWMDeviceManager, IWMDeviceManager_Vtbl, 0x1dcb3a00_33ed_11d3_8470_00c04f79dbc0);
impl core::ops::Deref for IWMDeviceManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMDeviceManager, windows_core::IUnknown);
impl IWMDeviceManager {
    pub unsafe fn GetRevision(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumDevices(&self) -> windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWMDeviceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDeviceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceFromCanonicalName)(windows_core::Interface::as_raw(self), pwszcanonicalname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumDevices2(&self) -> windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDevices2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reinitialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reinitialize)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWMDeviceManager2_Vtbl {
    pub base__: IWMDeviceManager_Vtbl,
    pub GetDeviceFromCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reinitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetDeviceEnumPreference)(windows_core::Interface::as_raw(self), dwenumpref).ok()
    }
}
#[repr(C)]
pub struct IWMDeviceManager3_Vtbl {
    pub base__: IWMDeviceManager2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const ENUM_MODE_METADATA_VIEWS: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(2i32);
pub const ENUM_MODE_RAW: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(0i32);
pub const ENUM_MODE_USE_DEVICE_PREF: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(1i32);
pub const EVENT_WMDM_CONTENT_TRANSFER: windows_core::GUID = windows_core::GUID::from_u128(0x339c9bf4_bcfe_4ed8_94df_eaf8c26ab61b);
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
pub const MDSP_READ: u32 = 1u32;
pub const MDSP_SEEK_BOF: u32 = 1u32;
pub const MDSP_SEEK_CUR: u32 = 2u32;
pub const MDSP_SEEK_EOF: u32 = 4u32;
pub const MDSP_WRITE: u32 = 2u32;
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
pub const MTP_RESPONSE_OK: u16 = 8193u16;
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
pub const WMDMID_LENGTH: u32 = 128u32;
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
pub const WMDM_FIND_SCOPE_GLOBAL: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(0i32);
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(1i32);
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
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
pub const WMDM_TYPE_BINARY: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(2i32);
pub const WMDM_TYPE_BOOL: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(3i32);
pub const WMDM_TYPE_DATE: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(7i32);
pub const WMDM_TYPE_DWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(0i32);
pub const WMDM_TYPE_GUID: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(6i32);
pub const WMDM_TYPE_QWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(4i32);
pub const WMDM_TYPE_STRING: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(1i32);
pub const WMDM_TYPE_WORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(5i32);
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMDMMessage(pub i32);
impl windows_core::TypeKind for WMDMMessage {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMDMMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMDMMessage").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMDM_ENUM_PROP_VALID_VALUES_FORM(pub i32);
impl windows_core::TypeKind for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMDM_ENUM_PROP_VALID_VALUES_FORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMDM_FIND_SCOPE(pub i32);
impl windows_core::TypeKind for WMDM_FIND_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMDM_FIND_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMDM_FIND_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMDM_FORMATCODE(pub i32);
impl windows_core::TypeKind for WMDM_FORMATCODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMDM_FORMATCODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMDM_FORMATCODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMDM_SESSION_TYPE(pub i32);
impl windows_core::TypeKind for WMDM_SESSION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMDM_SESSION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMDM_SESSION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMDM_STORAGE_ENUM_MODE(pub i32);
impl windows_core::TypeKind for WMDM_STORAGE_ENUM_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMDM_STORAGE_ENUM_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMDM_STORAGE_ENUM_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WMDM_TAG_DATATYPE(pub i32);
impl windows_core::TypeKind for WMDM_TAG_DATATYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WMDM_TAG_DATATYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WMDM_TAG_DATATYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MACINFO {
    pub fUsed: super::super::Foundation::BOOL,
    pub abMacState: [u8; 36],
}
impl windows_core::TypeKind for MACINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MACINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for MTP_COMMAND_DATA_IN {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MTP_COMMAND_DATA_OUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MTP_COMMAND_DATA_OUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MediaDevMgr: windows_core::GUID = windows_core::GUID::from_u128(0x25baad81_3560_11d3_8471_00c04f79dbc0);
pub const MediaDevMgrClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0x50040c1d_bdbf_4924_b873_f14d6c5bfd66);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPAQUECOMMAND {
    pub guidCommand: windows_core::GUID,
    pub dwDataLen: u32,
    pub pData: *mut u8,
    pub abMAC: [u8; 20],
}
impl windows_core::TypeKind for OPAQUECOMMAND {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPAQUECOMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDMDATETIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
}
impl windows_core::TypeKind for WMDMDATETIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDMDATETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for WMDMDetermineMaxPropStringLen {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDMDetermineMaxPropStringLen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDMDevice: windows_core::GUID = windows_core::GUID::from_u128(0x807b3cdf_357a_11d3_8471_00c04f79dbc0);
pub const WMDMDeviceEnum: windows_core::GUID = windows_core::GUID::from_u128(0x430e35af_3971_11d3_8474_00c04f79dbc0);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDMID {
    pub cbSize: u32,
    pub dwVendorID: u32,
    pub pID: [u8; 128],
    pub SerialNumberLength: u32,
}
impl windows_core::TypeKind for WMDMID {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDMLogger: windows_core::GUID = windows_core::GUID::from_u128(0x110a3202_5a79_11d3_8d78_444553540000);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDMMetadataView {
    pub pwszViewName: windows_core::PWSTR,
    pub nDepth: u32,
    pub ppwszTags: *mut *mut u16,
}
impl windows_core::TypeKind for WMDMMetadataView {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDMMetadataView {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDMRIGHTS {
    pub cbSize: u32,
    pub dwContentType: u32,
    pub fuFlags: u32,
    pub fuRights: u32,
    pub dwAppSec: u32,
    pub dwPlaybackCount: u32,
    pub ExpirationDate: WMDMDATETIME,
}
impl windows_core::TypeKind for WMDMRIGHTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDMRIGHTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMDMStorage: windows_core::GUID = windows_core::GUID::from_u128(0x807b3ce0_357a_11d3_8471_00c04f79dbc0);
pub const WMDMStorageEnum: windows_core::GUID = windows_core::GUID::from_u128(0xeb401a3b_3af7_11d3_8474_00c04f79dbc0);
pub const WMDMStorageGlobal: windows_core::GUID = windows_core::GUID::from_u128(0x807b3ce1_357a_11d3_8471_00c04f79dbc0);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
impl windows_core::TypeKind for WMDM_FORMAT_CAPABILITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDM_FORMAT_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
impl windows_core::TypeKind for WMDM_PROP_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDM_PROP_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: windows_core::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
impl Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WMDM_PROP_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDM_PROP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: core::mem::ManuallyDrop<WMDM_PROP_VALUES_RANGE>,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
impl Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WMDM_PROP_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDM_PROP_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut windows_core::PROPVARIANT,
}
impl windows_core::TypeKind for WMDM_PROP_VALUES_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDM_PROP_VALUES_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: core::mem::ManuallyDrop<windows_core::PROPVARIANT>,
    pub rangeMax: core::mem::ManuallyDrop<windows_core::PROPVARIANT>,
    pub rangeStep: core::mem::ManuallyDrop<windows_core::PROPVARIANT>,
}
impl Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for WMDM_PROP_VALUES_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMDM_PROP_VALUES_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: windows_core::PWSTR,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for WMFILECAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for WMFILECAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
